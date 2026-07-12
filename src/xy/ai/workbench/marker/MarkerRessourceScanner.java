package xy.ai.workbench.marker;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.eclipse.core.filebuffers.FileBuffers;
import org.eclipse.core.filebuffers.ITextFileBuffer;
import org.eclipse.core.filebuffers.ITextFileBufferManager;
import org.eclipse.core.filebuffers.LocationKind;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IMarker;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.resources.IResourceChangeEvent;
import org.eclipse.core.resources.IResourceChangeListener;
import org.eclipse.core.resources.IResourceDelta;
import org.eclipse.core.resources.IResourceDeltaVisitor;
import org.eclipse.core.resources.IWorkspaceRoot;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IEditorReference;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.texteditor.ITextEditor;
import org.osgi.framework.BundleContext;

import xy.ai.workbench.Activator;
import xy.ai.workbench.LOG;
import xy.ai.workbench.OutputMode;
import xy.ai.workbench.models.AIAnswer;

public class MarkerRessourceScanner implements IResourceChangeListener, IResourceDeltaVisitor {
	private static final String AIREQ_PREFIX = "xy.ai.req";
	private static final String MARKER_ID = "xy.ai.workbench.promptmarker";
	private static final String MARKER_REQ_ID_ATTR = "requestId";
	private static final String MARKER_OFF_ID_ATTR = "offset";
	private static final String MARKER_LEN_ID_ATTR = "length";
	private final Pattern pattern;

	public MarkerRessourceScanner(BundleContext context) {
		ResourcesPlugin.getWorkspace().addResourceChangeListener(this, IResourceChangeEvent.POST_CHANGE);
		pattern = Pattern.compile("\\[" + AIREQ_PREFIX + ":(.*):(.*)\\]", Pattern.CASE_INSENSITIVE);
	}

	public void dispose(BundleContext context) {
		ResourcesPlugin.getWorkspace().removeResourceChangeListener(this);
	}

	@Override
	public boolean visit(IResourceDelta delta) throws CoreException {
		IResource rsc = delta.getResource();
		if (rsc.getType() == IResource.FILE //
				&& rsc.getName().endsWith(".md")//
				&& delta.getKind() == IResourceDelta.CHANGED //
				&& rsc instanceof IFile //
				&& (delta.getFlags() & IResourceDelta.CONTENT) != 0)
			Job.create("Scanning file for markers", mon -> {
				rescannFile((IFile) rsc);
				return Status.OK_STATUS;
			}).schedule();
		return true;
	}

	private void rescannFile(IFile file) {
		int lineNumber = 1;
		int totaloff = 0;

		try (BufferedReader reader = new BufferedReader(new InputStreamReader(file.getContents()))) {
			file.deleteMarkers(MARKER_ID, false, IResource.DEPTH_ZERO);

			String line;
			while ((line = reader.readLine()) != null) {
				Matcher m = pattern.matcher(line);
				while (m.find()) {
					IMarker marker = file.createMarker(MARKER_ID);
					marker.setAttribute(IMarker.MESSAGE, m.group(1) + ": " + m.group(2));
					marker.setAttribute(IMarker.LINE_NUMBER, lineNumber);
					marker.setAttribute(MARKER_OFF_ID_ATTR, totaloff + m.start());
					marker.setAttribute(MARKER_LEN_ID_ATTR, m.end() - m.start());
					marker.setAttribute(IMarker.PRIORITY, IMarker.PRIORITY_NORMAL);
					marker.setAttribute(MARKER_REQ_ID_ATTR, m.group(2));
				}
				lineNumber++;
				totaloff += line.length() + 1;
			}
		} catch (IOException | CoreException e) {
			LOG.error(e.getMessage(), e);
		}
	}

	@Override
	public void resourceChanged(IResourceChangeEvent event) {
		if (event.getType() == IResourceChangeEvent.POST_CHANGE) {
			try {
				event.getDelta().accept(this);
			} catch (CoreException e) {
				LOG.error(e.getMessage(), e);
			}
		}
	}

	/**
	 * @param ans
	 * @return true when at least one marker was displayed and the AI answer is
	 *         persisted
	 */
	public boolean findAndReplaceMarkers(AIAnswer ans) {
		IWorkspaceRoot root = ResourcesPlugin.getWorkspace().getRoot();
		boolean res = false;
		try {
			IMarker[] markers = root.findMarkers(MARKER_ID, true, IResource.DEPTH_INFINITE);
			for (IMarker marker : markers)
				if (ans.id.equals(marker.getAttribute(MARKER_REQ_ID_ATTR)))
					res |= replaceMarker(ans, marker);
		} catch (CoreException e) {
			LOG.error(e.getMessage(), e);
		}

		// Fallback: the marker's stored position is outdated
		if (!res)
			res = findAndReplaceInOpenEditors(ans);

		return res;
	}

	private boolean replaceMarker(AIAnswer ans, IMarker marker) {
		IResource resource = marker.getResource();
		if (!(resource instanceof IFile))
			return false;

		IFile file = (IFile) resource;
		int line = marker.getAttribute(IMarker.LINE_NUMBER, -1);
		if (line <= 0)
			return false;

		ITextFileBufferManager bm = FileBuffers.getTextFileBufferManager();
		boolean[] replaced = { false };
		try {
			bm.connect(file.getFullPath(), LocationKind.IFILE, null);
			ITextFileBuffer tb = bm.getTextFileBuffer(file.getFullPath(), LocationKind.IFILE);

			Display.getDefault().syncExec(() -> {
				try {
					IDocument doc = tb.getDocument();
					int off = marker.getAttribute(MARKER_OFF_ID_ATTR, -1);
					int len = marker.getAttribute(MARKER_LEN_ID_ATTR, -1);

					int[] range = resolveTagRange(doc, ans.id, off, len);
					if (range == null)
						return; // stored position no longer matches the live document

					ITextEditor editor = null;
					if (isAutoFollowModeEnabled())
						editor = findOpenEditorFor(file);
					boolean autoFollow = shouldAutoFollow(editor, doc);

					doc.replace(range[0], range[1], ans.answer);
					replaced[0] = true;

					if (autoFollow)
						moveCursorToLastLineStart(editor, doc);
				} catch (BadLocationException e) {
					LOG.error(e.getMessage(), e);
				}
			});

			if (replaced[0]) {
				tb.commit(null, false);
				marker.delete();
				return true;
			}

		} catch (CoreException e) {
			LOG.error(e.getMessage(), e);
		} finally {
			try {
				bm.disconnect(file.getFullPath(), LocationKind.IFILE, null);
			} catch (CoreException e) {
				LOG.error(e.getMessage(), e);
			}
		}
		return false;
	}

	/**
	 * Verifies that the given offset/length still points at the tag belonging to
	 * the given request id. If it does not (e.g. because the document was edited in
	 * the meantime and the marker position is stale) the whole document is searched
	 * for the tag instead.
	 *
	 * @return an {offset, length} pair pointing at the current location of the tag
	 *         in the document, or {@code null} if the tag can no longer be found.
	 */
	private int[] resolveTagRange(IDocument doc, String requestId, int off, int len) {
		if (off >= 0 && len >= 0 && off + len <= doc.getLength()) {
			try {
				String candidate = doc.get(off, len);
				Matcher m = pattern.matcher(candidate);
				if (m.matches() && requestId.equals(m.group(2)))
					return new int[] { off, len };
			} catch (BadLocationException e) {
				// fall through to full-document search
			}
		}
		return findTagInDocument(doc, requestId);
	}

	/**
	 * Scans the full document content for the tag belonging to the given request
	 * id.
	 *
	 * @return an {offset, length} pair, or {@code null} if not found.
	 */
	private int[] findTagInDocument(IDocument doc, String requestId) {
		String content = doc.get();
		Matcher m = pattern.matcher(content);
		while (m.find())
			if (requestId.equals(m.group(2)))
				return new int[] { m.start(), m.end() - m.start() };
		return null;
	}

	/**
	 * Fallback used when the marker based replacement failed, e.g. because the
	 * marker's stored offset is no longer in sync with the (still dirty) editor
	 * content, or no marker exists at all yet. Searches all currently open text
	 * editors for the tag belonging to the given request id and replaces it
	 * directly in the editor's document. The editor is intentionally not saved so
	 * that a parallel edit by the user is not disturbed.
	 */
	private boolean findAndReplaceInOpenEditors(AIAnswer ans) {
		boolean[] res = { false };
		Display.getDefault().syncExec(() -> {
			for (ITextEditor editor : getOpenTextEditors()) {
				IDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());
				if (doc == null)
					continue;

				int[] range = findTagInDocument(doc, ans.id);
				if (range == null)
					continue;

				try {
					boolean autoFollow = isAutoFollowModeEnabled() && shouldAutoFollow(editor, doc);
					doc.replace(range[0], range[1], ans.answer);
					res[0] = true;
					if (autoFollow)
						moveCursorToLastLineStart(editor, doc);
					break;
				} catch (BadLocationException e) {
					LOG.error(e.getMessage(), e);
				}
			}
		});
		return res[0];
	}

	private ITextEditor findOpenEditorFor(IFile file) {
		for (ITextEditor editor : getOpenTextEditors()) {
			IEditorInput input = editor.getEditorInput();
			if (input instanceof IFileEditorInput && file.equals(((IFileEditorInput) input).getFile()))
				return editor;
		}
		return null;
	}

	private boolean isAutoFollowModeEnabled() {
		OutputMode mode = Activator.getDefault().cfg.getOuputMode();
		return mode == OutputMode.Append || mode == OutputMode.Chat;
	}

	private boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {
		if (editor == null || editor.isDirty())
			return false;

		ISelection selection = editor.getSelectionProvider().getSelection();
		if (!(selection instanceof ITextSelection))
			return false;

		ITextSelection tsel = (ITextSelection) selection;
		int lastLine = doc.getNumberOfLines() - 1;
		return tsel.getStartLine() >= lastLine;
	}

	private void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {
		try {
			int lastLine = doc.getNumberOfLines() - 1;
			int offset = doc.getLineOffset(lastLine);
			editor.selectAndReveal(offset, 0);
		} catch (BadLocationException e) {
			LOG.error(e.getMessage(), e);
		}
	}

	private java.util.List<ITextEditor> getOpenTextEditors() {
		java.util.List<ITextEditor> editors = new java.util.ArrayList<>();
		for (IWorkbenchWindow window : PlatformUI.getWorkbench().getWorkbenchWindows())
			for (IWorkbenchPage page : window.getPages())
				for (IEditorReference ref : page.getEditorReferences()) {
					IEditorPart part = ref.getEditor(false);
					if (part instanceof ITextEditor) {
						IEditorInput input = part.getEditorInput();
						// prefer file based / URI based editors, but any ITextEditor works
						if (input instanceof IFileEditorInput || input != null)
							editors.add((ITextEditor) part);
					}
				}
		return editors;
	}

	public static String getPromptTag(String meta, String id) {
		return String.format("[%s:%s:%s]", AIREQ_PREFIX, meta, id);
	}
}
