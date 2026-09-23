package xy.ai.workbench;

import java.io.ByteArrayInputStream;
import java.io.InputStream;
import java.io.UnsupportedEncodingException;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.Path;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.ide.IDE;
import org.eclipse.ui.texteditor.ITextEditor;
import org.osgi.framework.BundleContext;

import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.marker.MarkerRessourceScanner;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;

public class EditorInterface {
	public static final String USER = "User:";
	public static final String AGENT = "Agent:";

	public static final String TEXT = "Text:";
	public static final String THINKING = "Thinking:";
	public static final String THINKING_META = "Thinking Meta:";
	public static final String TOOLUSE = "Tool:";
	public static final String TOOLRESULT = "ToolResult:";
	public static final String CONTROL_REQUEST = "Control Request:";

	private final ConfigManager cfg;
	private final ActiveEditorListener editorListener;
	private final AdaptingConnector connector;
	private MarkerRessourceScanner markerScanner;

	public EditorInterface(ConfigManager cfg, ActiveEditorListener editorListener, AdaptingConnector connector) {
		this.cfg = cfg;
		this.editorListener = editorListener;
		this.connector = connector;
	}

	public void register(BundleContext context) throws Exception {
		if (markerScanner != null)
			dispose(context);
		markerScanner = new MarkerRessourceScanner(cfg, context);
	}

	public void dispose(BundleContext context) {
		if (markerScanner != null)
			markerScanner.dispose(context);
		markerScanner = null;
	}

	public void insertTag(Display display, IModelRequest req, IProgressMonitor mon) {
		display.syncExec(() -> {
			OutputMode outputMode = req.getPrompt().config.outputMode;
			if (OutputMode.New_File.equals(outputMode)) {
				IContainer parent = editorListener.getCurrentFile().getParent();

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
					LOG.info("Error opening new editor file", e);
				} catch (CoreException e) {
					LOG.info("Error writting file", e);
				} catch (UnsupportedEncodingException e) {
					LOG.info("Error unsupported encoding", e);
				}

			} else {

				ITextEditor textEditor = editorListener.getLastTextEditor();
				IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
				ISelection selection = textEditor.getSelectionProvider().getSelection();
				ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;

				try {
					String tag = generateTag(req);
					switch (outputMode) {
					case Chat:
						String replace = String.format("\n%s\n%s\n%s\n", AGENT, tag, USER);
						doc.replace(doc.getLength(), 0, replace);
						display.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));
						break;
					case Append:
						doc.replace(doc.getLength(), 0, "\n" + tag);
						display.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));
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
					LOG.info("Error adding text", e);
				}
			}
		});
	}

	private String generateTag(IModelRequest req) {
		KeyPattern pattern = connector.getConnector(req).getSupportedKeyPattern();
		return MarkerRessourceScanner.getPromptTag(pattern.name(), req.getID());
	}

	public void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {
		OutputMode mode = ans.prompt != null ? ans.prompt.config.outputMode : null;
		if (ans.showStats && (OutputMode.Chat.equals(mode) || OutputMode.Append.equals(mode)))
			ans.answer = ans.print() + "\n" + ans.answer;
		if (!markerScanner.findAndReplaceMarkers(ans))
			LOG.info("Error: wasn't able to replace prompt marker with answer:\n" + ans.answer);
	}
}
