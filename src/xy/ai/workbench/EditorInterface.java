package xy.ai.workbench;

import java.io.ByteArrayInputStream;
import java.io.InputStream;
import java.io.UnsupportedEncodingException;
import java.net.URI;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.Path;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.IURIEditorInput;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.ide.IDE;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connectors.AdaptingConnector;
import xy.ai.workbench.marker.MarkerRessourceScanner;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;

public class EditorInterface {
	public static final String USER = "User:";
	public static final String AGENT = "Agent:";

	private final ActiveEditorListener editorListener;
	private final AdaptingConnector connector;
	private final ConfigManager cfg;

	public EditorInterface(ActiveEditorListener editorListener, AdaptingConnector connector, ConfigManager cfg) {
		this.editorListener = editorListener;
		this.connector = connector;
		this.cfg = cfg;
	}

	public void insertTag(Display display, IModelRequest req, IProgressMonitor mon) {
		display.syncExec(() -> {
			ITextEditor textEditor = editorListener.getLastTextEditor();
			if (OutputMode.New_File.equals(cfg.getOuputMode())) {

				IEditorInput editorInput = textEditor.getEditorInput();
				IFile currentFile;
				if (editorInput instanceof IFileEditorInput)
					currentFile = ((IFileEditorInput) editorInput).getFile();
				else if (editorInput instanceof IURIEditorInput) {
					URI uri = ((IURIEditorInput) editorInput).getURI();
					String fileName = new Path(uri.getPath()).lastSegment();
					currentFile = ResourcesPlugin.getWorkspace().getRoot().getProject("ExternalFiles")
							.getFile(fileName);

					if (!currentFile.exists())
						try {
							currentFile.createLink(uri, IResource.ALLOW_MISSING_LOCAL, mon);
						} catch (CoreException e) {
							throw new IllegalStateException("Could not link external file", e);
						}
				} else
					throw new IllegalArgumentException("Editor type not supported for new file output mode");

				IContainer parent = currentFile.getParent();

				String timestamp = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyyMMdd.HHmmss"));
				IFile newFile = parent.getFile(new Path(timestamp + ".md"));
				String tag = generateTag(req);
				try {
					InputStream source = new ByteArrayInputStream(tag.getBytes("UTF-8"));

					if (!newFile.exists()) {
						newFile.create(source, true, null);
					} else {
						newFile.setContents(source, true, true, null);
					}
					newFile.touch(null);

					IWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();
					IDE.openEditor(page, newFile);
				} catch (PartInitException e) {
					LOG.info("Error opening new editor file");
				} catch (CoreException e) {
					LOG.info("Error writting file");
				} catch (UnsupportedEncodingException e) {
					LOG.info("Error unsupported encoding");
				}

			} else {

				IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
				ISelection selection = textEditor.getSelectionProvider().getSelection();
				ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;

				try {
					String tag = generateTag(req);
					switch (cfg.getOuputMode()) {
					case Chat:
						String replace = String.format("\n%s\n%s\n%s\n", AGENT, tag, USER);
						doc.replace(doc.getLength(), 0, replace);
						textEditor.selectAndReveal(doc.getLength(), 0);
						break;
					case Append:
						doc.replace(doc.getLength(), 0, "\n" + tag);
						textEditor.selectAndReveal(doc.getLength(), 0);
						break;
					case Replace:
						if (tsel != null)
							doc.replace(tsel.getOffset(), tsel.getLength(), tag);
						break;
					case Cursor:
						if (tsel != null)
							doc.replace(tsel.getOffset(), 0, tag);
						break;
					case New_File:
						throw new UnsupportedOperationException();
					}
					textEditor.doSave(mon);
				} catch (BadLocationException e) {
					LOG.info("Error adding text");
				}
			}
		});
	}

	private String generateTag(IModelRequest req) {
		KeyPattern pattern = connector.getConnector(req).getSupportedKeyPattern();
		return MarkerRessourceScanner.getPromptTag(pattern.name(), req.getID());
	}

	public void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {
		if (!Activator.getDefault().markerScanner.findAndReplaceMarkers(ans))
			LOG.info("Error: wasn't able to replace prompt marker with answer:\n" + ans.answer);
	}
}
