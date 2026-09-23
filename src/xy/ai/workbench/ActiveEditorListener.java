package xy.ai.workbench;

import java.io.File;
import java.lang.ref.WeakReference;
import java.net.URI;
import java.nio.file.Path;
import java.nio.file.Paths;

import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IProject;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.NullProgressMonitor;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.DocumentEvent;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IDocumentListener;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextListener;
import org.eclipse.jface.text.ITextOperationTarget;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.text.ITextViewer;
import org.eclipse.jface.text.TextEvent;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.ISelectionChangedListener;
import org.eclipse.jface.viewers.ISelectionProvider;
import org.eclipse.jface.viewers.SelectionChangedEvent;
import org.eclipse.swt.custom.CaretEvent;
import org.eclipse.swt.custom.CaretListener;
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.IPartListener2;
import org.eclipse.ui.IURIEditorInput;
import org.eclipse.ui.IWorkbenchPart;
import org.eclipse.ui.IWorkbenchPartReference;
import org.eclipse.ui.texteditor.AbstractTextEditor;
import org.eclipse.ui.texteditor.IDocumentProvider;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.editor.AISessionEditor;

public class ActiveEditorListener implements IPartListener2 {
	private final ConfigManager cfg;
	private final EditorChangeListener editorListener = new EditorChangeListener();

	private WeakReference<ITextEditor> lastTextEditor;
	private InputStatObserver obs;

	public ActiveEditorListener(ConfigManager cfg) {
		this.cfg = cfg;
	}

	public void setInputObserver(InputStatObserver obs) {
		if (this.obs != null)
			throw new IllegalStateException("Allready egistered");
		this.obs = obs;
	}

	public ITextEditor getLastTextEditor() {
		return lastTextEditor != null ? lastTextEditor.get() : null;
	}

	public IContainer baseContainer() {
		ITextEditor textEditor = getLastTextEditor();
		if (textEditor != null) {
			IEditorInput input = textEditor.getEditorInput();
			if (input instanceof IFileEditorInput)
				return ((IFileEditorInput) input).getFile().getParent();
		}
		return ResourcesPlugin.getWorkspace().getRoot();
	}

	public record Selection(String[] selection, Integer cursorOffset) {
	}

	public Selection getSelection() {
		var textEditor = getLastTextEditor();
		if (textEditor != null) {
			ISelectionProvider selPrv = textEditor.getSelectionProvider();
			if (selPrv != null) {
				ISelection sel = selPrv.getSelection();
				ITextSelection tsel = sel instanceof ITextSelection ? (ITextSelection) sel : null;
				if (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1)
					return new Selection(tsel.getText().split("\n"), null);

				if (tsel != null) {
					int line = tsel.getEndLine();
					IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
					try {
						IRegion lineInfo = doc.getLineInformation(line);
						return new Selection(doc.get(lineInfo.getOffset(), lineInfo.getLength()).split("\n"), null);
					} catch (BadLocationException e1) {
						LOG.error("Exception", e1);
					}
				}
			}
		}
		return null;
	}

	public String resolveAbsoluteFilePath() {
		var textEditor = getLastTextEditor();
		if (textEditor == null)
			return null;
		IEditorInput input = textEditor.getEditorInput();
		if (input instanceof IFileEditorInput)
			return ((IFileEditorInput) input).getFile().getLocation().toFile().getAbsolutePath();
		if (input instanceof IURIEditorInput)
			return new File(((IURIEditorInput) input).getURI()).getAbsolutePath();
		return null;
	}

	public Path resolveProjectPath() {
		var textEditor = getLastTextEditor();
		if (textEditor == null)
			return null;
		IEditorInput input = textEditor.getEditorInput();
		if (!(input instanceof IFileEditorInput))
			return null;
		IProject project = ((IFileEditorInput) input).getFile().getProject();
		return Paths.get(project.getLocation().toOSString());
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

	public IResource getCurrentFile() {
		var textEditor = getLastTextEditor();
		IEditorInput editorInput = textEditor.getEditorInput();
		IFile currentFile;
		if (editorInput instanceof IFileEditorInput)
			currentFile = ((IFileEditorInput) editorInput).getFile();
		else if (editorInput instanceof IURIEditorInput) {
			URI uri = ((IURIEditorInput) editorInput).getURI();
			String fileName = new org.eclipse.core.runtime.Path(uri.getPath()).lastSegment();
			currentFile = ResourcesPlugin.getWorkspace().getRoot().getProject("ExternalFiles").getFile(fileName);

			if (!currentFile.exists())
				try {
					currentFile.createLink(uri, IResource.ALLOW_MISSING_LOCAL, new NullProgressMonitor());
				} catch (CoreException e) {
					throw new IllegalStateException("Could not link external file", e);
				}
		} else
			throw new IllegalArgumentException("Editor type not supported for new file output mode");
		return currentFile;
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

		if (editor != null)
			cfg.activateEditor(editor);
	}

	public class EditorChangeListener {
		private SelectionListener selectionListener = new SelectionListener();
		private DocumentListener documentListener = new DocumentListener();
		private TextChangeListener textListener = new TextChangeListener();
		private CaretListener caretListener = new EditorCaretListener();
		private ITextEditor textEditor;

		private void setTextEditor(ITextEditor textEditor) {
			this.textEditor = textEditor;
			if (textEditor != null)
				lastTextEditor = new WeakReference<>(textEditor);
		}

		private ITextEditor getTextEditor() {
			return textEditor;
		}

		public void editorChanged(ITextEditor editor) {
			removeListener();

			Job.create("Update Input Stats", (mon) -> {
				Display.getDefault().asyncExec(() -> {
					obs.updateInputStat(InputMode.Selection);
					obs.updateInputStat(InputMode.Converter);
				});
			}).schedule(300);

			if (editor != null)
				registerListener(editor);
		}

		private void registerListener(ITextEditor editor) {
			setTextEditor(editor);
			editor.getSelectionProvider().addSelectionChangedListener(selectionListener);

			IDocumentProvider documentProvider = editor.getDocumentProvider();
			if (documentProvider != null) {
				IDocument doc = documentProvider.getDocument(editor.getEditorInput());
				if (doc != null)
					doc.addDocumentListener(documentListener);
			}

			if (editor instanceof AbstractTextEditor) {
				AbstractTextEditor abstractEditor = (AbstractTextEditor) editor;

				ITextViewer textViewer = abstractEditor.getAdapter(ITextViewer.class);
				if (textViewer != null)
					textViewer.addTextListener(textListener);

				ISourceViewer sourceViewer = (ISourceViewer) abstractEditor.getAdapter(ITextOperationTarget.class);
				if (sourceViewer != null) {
					StyledText textWidget = sourceViewer.getTextWidget();
					if (textWidget != null)
						textWidget.addCaretListener(caretListener);
				}
			}
		}

		private void removeListener() {
			ITextEditor editor = getTextEditor();
			if (editor != null) {
				ISelectionProvider selectionProvider = editor.getSelectionProvider();
				if (selectionProvider != null)
					selectionProvider.removeSelectionChangedListener(selectionListener);

				IDocumentProvider documentProvider = editor.getDocumentProvider();
				if (documentProvider != null) {
					IDocument doc = documentProvider.getDocument(editor.getEditorInput());
					if (doc != null)
						doc.removeDocumentListener(documentListener);
				}

				if (editor instanceof AbstractTextEditor) {
					AbstractTextEditor abstractEditor = (AbstractTextEditor) editor;

					ITextViewer textViewer = abstractEditor.getAdapter(ITextViewer.class);
					if (textViewer != null)
						textViewer.removeTextListener(textListener);

					ISourceViewer sourceViewer = (ISourceViewer) abstractEditor.getAdapter(ITextOperationTarget.class);
					if (sourceViewer != null) {
						StyledText textWidget = sourceViewer.getTextWidget();
						if (textWidget != null)
							textWidget.removeCaretListener(caretListener);
					}
				}

				setTextEditor(null);
			}
		}
	}

	public class EditorCaretListener implements CaretListener {
		@Override
		public void caretMoved(CaretEvent event) {
			Display.getDefault().asyncExec(() -> {
				obs.updateInputStat(InputMode.Selection);
			});
		}
	}

	public class DocumentListener extends AbstractDocumentListener {
		@Override
		public void documentChanged(DocumentEvent event) {
			Job.create("Update Input Stats", (mon) -> {
				Display.getDefault().asyncExec(() -> {
					obs.updateInputStat(InputMode.Selection);
					obs.updateInputStat(InputMode.Converter);
				});
			}).schedule(1000);

		}
	}

	public class SelectionListener implements ISelectionChangedListener {
		@Override
		public void selectionChanged(SelectionChangedEvent event) {
			Display.getDefault().asyncExec(() -> obs.updateInputStat(InputMode.Selection));
		}
	}

	public class TextChangeListener implements ITextListener {
		@Override
		public void textChanged(TextEvent event) {
			Display.getDefault().asyncExec(() -> {
				obs.updateInputStat(InputMode.Selection);
				obs.updateInputStat(InputMode.Converter);
			});
		}
	}

	public abstract class AbstractDocumentListener implements IDocumentListener {
		@Override
		public void documentAboutToBeChanged(DocumentEvent event) {
		}
	}

	@Override
	public void partBroughtToTop(IWorkbenchPartReference partRef) {
	}

	@Override
	public void partClosed(IWorkbenchPartReference partRef) {
	}

	@Override
	public void partDeactivated(IWorkbenchPartReference partRef) {
	}

	@Override
	public void partOpened(IWorkbenchPartReference partRef) {
	}

	@Override
	public void partHidden(IWorkbenchPartReference partRef) {
	}

	@Override
	public void partVisible(IWorkbenchPartReference partRef) {
	}

	@Override
	public void partInputChanged(IWorkbenchPartReference partRef) {
	}

	public static interface InputStatObserver {
		public void updateInputStat(InputMode mode);
	}
}