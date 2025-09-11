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
import org.eclipse.jface.text.IRegion;
import org.eclipse.swt.widgets.Display;
import org.osgi.framework.BundleContext;

import xy.ai.workbench.models.AIAnswer;

public class MarkerRessourceScanner implements IResourceChangeListener, IResourceDeltaVisitor {
	private static final String AIREQ_PREFIX = "xy.ai.req";
	private static final String MARKER_ID = "xy.ai.workbench.promptmarker";
	private static final String MARKER_REQ_ID_ATTR = "requestId";
	private final Pattern pattern;

	public MarkerRessourceScanner(BundleContext context) {
		ResourcesPlugin.getWorkspace().addResourceChangeListener(this, IResourceChangeEvent.POST_CHANGE);
		pattern = Pattern.compile("\\[" + AIREQ_PREFIX + ":(.*)\\]", Pattern.CASE_INSENSITIVE);
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

		try (BufferedReader reader = new BufferedReader(new InputStreamReader(file.getContents()))) {
			file.deleteMarkers(MARKER_ID, false, IResource.DEPTH_ZERO);

			String line;
			while ((line = reader.readLine()) != null) {
				Matcher m = pattern.matcher(line);
				while (m.find()) {
					IMarker marker = file.createMarker(MARKER_ID);
					marker.setAttribute(IMarker.MESSAGE, "AI Prompt: " + m.group());
					marker.setAttribute(IMarker.LINE_NUMBER, lineNumber);
					marker.setAttribute(IMarker.PRIORITY, IMarker.PRIORITY_NORMAL);
					marker.setAttribute(MARKER_REQ_ID_ATTR, m.group(1));
				}
				lineNumber++;
			}
		} catch (IOException | CoreException e) {
			e.printStackTrace();
		}
	}

	@Override
	public void resourceChanged(IResourceChangeEvent event) {
		if (event.getType() == IResourceChangeEvent.POST_CHANGE) {
			try {
				event.getDelta().accept(this);
			} catch (CoreException e) {
				e.printStackTrace();
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
			e.printStackTrace();
		}
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
		try {
			bm.connect(file.getFullPath(), LocationKind.IFILE, null);
			ITextFileBuffer tb = bm.getTextFileBuffer(file.getFullPath(), LocationKind.IFILE);
			String expr = getPromptTag(ans.id);

			Display.getDefault().syncExec(() -> {
				try {
					IDocument doc = tb.getDocument();

					IRegion linf = doc.getLineInformation(line - 1);
					String old = doc.get(linf.getOffset(), linf.getLength());
					String newl = old.replace(expr, ans.answer);
					doc.replace(linf.getOffset(), linf.getLength(), newl);
				} catch (BadLocationException e) {
					e.printStackTrace();
				}
			});

			tb.commit(null, false);
			marker.delete();
			return true;

		} catch (CoreException e) {
			e.printStackTrace();
		} finally {
			try {
				bm.disconnect(file.getFullPath(), LocationKind.IFILE, null);
			} catch (CoreException e) {
				e.printStackTrace();
			}
		}
		return false;
	}

	public static String getPromptTag(String id) {
		return String.format("[%s:%s]", AIREQ_PREFIX, id);
	}
}
