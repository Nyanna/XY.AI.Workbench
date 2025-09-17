package xy.ai.workbench;

import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.jface.text.DocumentEvent;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IDocumentListener;
import org.eclipse.jface.text.ITextListener;
import org.eclipse.jface.text.ITextOperationTarget;
import org.eclipse.jface.text.ITextViewer;
import org.eclipse.jface.text.TextEvent;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.viewers.ISelectionChangedListener;
import org.eclipse.jface.viewers.ISelectionProvider;
import org.eclipse.jface.viewers.SelectionChangedEvent;
import org.eclipse.swt.custom.CaretEvent;
import org.eclipse.swt.custom.CaretListener;
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IPartListener2;
import org.eclipse.ui.IWorkbenchPart;
import org.eclipse.ui.IWorkbenchPartReference;
import org.eclipse.ui.texteditor.AbstractTextEditor;
import org.eclipse.ui.texteditor.IDocumentProvider;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.editors.AISessionEditor;

public class ActiveEditorListener implements IPartListener2 {
	private EditorChangeListener editorListener = new EditorChangeListener();

	private ITextEditor lastTextEditor;
	private AISessionManager manager;

	public ActiveEditorListener(AISessionManager manager) {
		this.manager = manager;
	}

	public ITextEditor getLastTextEditor() {
		return lastTextEditor;
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

	public class EditorChangeListener {
		private SelectionListener selectionListener = new SelectionListener();
		private DocumentListener documentListener = new DocumentListener();
		private TextChangeListener textListener = new TextChangeListener();
		private CaretListener caretListener = new EditorCaretListener();
		private ITextEditor textEditor;

		private void setTextEditor(ITextEditor textEditor) {
			this.textEditor = textEditor;
			if (textEditor != null)
				lastTextEditor = textEditor;
		}

		private ITextEditor getTextEditor() {
			return textEditor;
		}

		public void editorChanged(ITextEditor editor) {
			removeListener();

			Job.create("Update Input Stats", (mon) -> {
				Display.getDefault().asyncExec(() -> {
					manager.updateInputStat(InputMode.Editor);
					manager.updateInputStat(InputMode.Selection);
					manager.updateInputStat(InputMode.Current_line);
					manager.updateInputStat(InputMode.Context_prompt);
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
				manager.updateInputStat(InputMode.Current_line);
			});
		}
	}

	public class DocumentListener extends AbstractDocumentListener {
		@Override
		public void documentChanged(DocumentEvent event) {
			Job.create("Update Input Stats", (mon) -> {
				Display.getDefault().asyncExec(() -> {
					manager.updateInputStat(InputMode.Editor);
					manager.updateInputStat(InputMode.Selection);
					manager.updateInputStat(InputMode.Current_line);
				});
			}).schedule(1000);

		}
	}

	public class SelectionListener implements ISelectionChangedListener {
		@Override
		public void selectionChanged(SelectionChangedEvent event) {
			Display.getDefault().asyncExec(() -> {
				manager.updateInputStat(InputMode.Selection);
				manager.updateInputStat(InputMode.Current_line);
			});
		}
	}

	public class TextChangeListener implements ITextListener {
		@Override
		public void textChanged(TextEvent event) {
			Display.getDefault().asyncExec(() -> {
				manager.updateInputStat(InputMode.Editor);
				manager.updateInputStat(InputMode.Selection);
				manager.updateInputStat(InputMode.Current_line);
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
}