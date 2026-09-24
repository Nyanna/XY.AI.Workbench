package xy.ai.workbench;

import java.io.File;
import java.net.URI;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.function.Consumer;

import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.NullProgressMonitor;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.ISelectionProvider;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.IPartListener2;
import org.eclipse.ui.IURIEditorInput;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchPart;
import org.eclipse.ui.IWorkbenchPartReference;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.texteditor.IDocumentProvider;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.editor.AISessionEditor;
import xy.ai.workbench.editor.EditorChangeListener;
import xy.ai.workbench.view.PartListener2Adapter;

public class ActiveEditorListener extends PartListener2Adapter implements IPartListener2 {
	private final EditorChangeListener editorListener = new EditorChangeListener();
	private boolean initialized;

	public void addInputObserver(Consumer<InputMode> obs) {
		checkInitBindings();
		editorListener.addInputObserver(obs);
	}

	public void addTextEditorObserver(Consumer<ITextEditor> obs) {
		checkInitBindings();
		editorListener.addTextEditorObserver(obs);
	}

	private void checkInitBindings() {
		if (initialized)
			return;
		IWorkbenchWindow window = PlatformUI.getWorkbench().getActiveWorkbenchWindow();
		IWorkbenchPage activePage = window != null ? window.getActivePage() : null;
		if (activePage != null)
			activePage.addPartListener(this);
		initialized = true;
	}

	public ITextEditor getLastTextEditor() {
		checkInitBindings();
		return editorListener.getLastTextEditor();
	}

	private IEditorInput getLastEditorInput() {
		ITextEditor edit = getLastTextEditor();
		return edit != null ? edit.getEditorInput() : null;
	}

	private ITextSelection getLastEditorSelection() {
		ITextEditor edit = getLastTextEditor();
		ISelectionProvider selPrv = edit != null ? edit.getSelectionProvider() : null;
		ISelection sel = selPrv != null ? selPrv.getSelection() : null;
		return sel instanceof ITextSelection ? (ITextSelection) sel : null;
	}

	public IFile getLastEditorFile() {
		IEditorInput input = getLastEditorInput();
		if (input == null)
			return null;
		if (input instanceof IFileEditorInput)
			return ((IFileEditorInput) input).getFile();
		else if (input instanceof IURIEditorInput) {
			URI uri = ((IURIEditorInput) input).getURI();
			String fileName = new org.eclipse.core.runtime.Path(uri.getPath()).lastSegment();
			IFile file = ResourcesPlugin.getWorkspace().getRoot().getProject("ExternalFiles").getFile(fileName);

			if (!file.exists())
				try {
					file.createLink(uri, IResource.ALLOW_MISSING_LOCAL, new NullProgressMonitor());
				} catch (CoreException e) {
					throw new IllegalStateException("Could not link external file", e);
				}
			return file;
		}
		return null;
	}

	public IContainer getLastContainer() {
		IFile file = getLastEditorFile();
		return file != null ? file.getParent() : ResourcesPlugin.getWorkspace().getRoot();
	}

	public record Selection(String[] selection, Integer cursorOffset) {
	}

	public Selection getSelection() {
		ITextSelection tsel = getLastEditorSelection();
		if (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1)
			return new Selection(tsel.getText().split("\n"), null);

		if (tsel != null) {
			int line = tsel.getEndLine();
			IDocument doc = getLastTextEditor().getDocumentProvider().getDocument(getLastTextEditor().getEditorInput());
			try {
				IRegion lineInfo = doc.getLineInformation(line);
				return new Selection(doc.get(lineInfo.getOffset(), lineInfo.getLength()).split("\n"), null);
			} catch (BadLocationException e1) {
				LOG.error("Exception", e1);
			}
		}
		return null;
	}

	public String resolveAbsoluteFilePath() {
		IFile file = getLastEditorFile();
		if (file != null)
			return file.getLocation().toFile().getAbsolutePath();
		IEditorInput input = getLastEditorInput();
		if (input instanceof IURIEditorInput)
			return new File(((IURIEditorInput) input).getURI()).getAbsolutePath();
		return null;
	}

	public Path resolveProjectPath() {
		IFile file = getLastEditorFile();
		return file != null ? Paths.get(file.getProject().getLocation().toOSString()) : null;
	}

	public Selection getFileContent() {
		ITextEditor textEditor = getLastTextEditor();
		IDocumentProvider provider;
		IDocument doc;
		String content;
		if (textEditor != null && (provider = textEditor.getDocumentProvider()) != null
				&& (doc = provider.getDocument(textEditor.getEditorInput())) != null && (content = doc.get()) != null) {
			ISelectionProvider prv = textEditor.getSelectionProvider();
			ISelection sel = prv != null ? prv.getSelection() : null;
			ITextSelection tsel = sel instanceof ITextSelection ? (ITextSelection) sel : null;
			return new Selection(content.split("\n"), tsel != null ? tsel.getEndLine() : null);
		}
		return null;
	}

	@Override
	public void partActivated(IWorkbenchPartReference partRef) {
		IEditorPart editor = null;
		IWorkbenchPart part = partRef.getPart(false);
		if (part instanceof AISessionEditor)
			editor = ((AISessionEditor) part).getEditor();
		else if (part instanceof IEditorPart)
			editor = (IEditorPart) part;

		editorListener.editorChanged(editor instanceof ITextEditor ? (ITextEditor) editor : null);
	}
}