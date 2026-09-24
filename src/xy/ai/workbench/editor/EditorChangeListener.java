package xy.ai.workbench.editor;

import java.lang.ref.WeakReference;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

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
import org.eclipse.ui.texteditor.AbstractTextEditor;
import org.eclipse.ui.texteditor.IDocumentProvider;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.InputMode;

public class EditorChangeListener {
	private SelectionListener selectionListener = new SelectionListener();
	private DocumentListener documentListener = new DocumentListener();
	private TextChangeListener textListener = new TextChangeListener();
	private CaretListener caretListener = new EditorCaretListener();
	private List<Consumer<InputMode>> inputStatObs = new ArrayList<>();
	private List<Consumer<ITextEditor>> editorObs = new ArrayList<>();
	private WeakReference<ITextEditor> lastTextEditor;
	private ITextEditor textEditor;

	public void addInputObserver(Consumer<InputMode> obs) {
		inputStatObs.add(obs);
	}

	public void addTextEditorObserver(Consumer<ITextEditor> obs) {
		editorObs.add(obs);
	}

	public ITextEditor getLastTextEditor() {
		return lastTextEditor != null ? lastTextEditor.get() : null;
	}

	private void setTextEditor(ITextEditor textEditor) {
		this.textEditor = textEditor;
		ITextEditor old = lastTextEditor != null ? lastTextEditor.get() : null;
		if (textEditor != null && textEditor != old) {
			lastTextEditor = new WeakReference<>(textEditor);
			editorObs.forEach(c -> c.accept(textEditor));
		}
	}

	private ITextEditor getTextEditor() {
		return textEditor;
	}

	public void editorChanged(ITextEditor editor) {
		removeListener();

		Job.create("Update Input Stats", (mon) -> {
			Display.getDefault().asyncExec(() -> {
				inputStatObs.forEach(c -> c.accept(InputMode.Selection));
				inputStatObs.forEach(c -> c.accept(InputMode.Converter));
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

	public class EditorCaretListener implements CaretListener {
		@Override
		public void caretMoved(CaretEvent event) {
			Display.getDefault().asyncExec(() -> {
				inputStatObs.forEach(c -> c.accept(InputMode.Selection));
			});
		}
	}

	public class DocumentListener implements IDocumentListener {
		@Override
		public void documentChanged(DocumentEvent event) {
			Job.create("Update Input Stats", (mon) -> {
				Display.getDefault().asyncExec(() -> {
					inputStatObs.forEach(c -> c.accept(InputMode.Selection));
					inputStatObs.forEach(c -> c.accept(InputMode.Converter));
				});
			}).schedule(1000);

		}

		@Override
		public void documentAboutToBeChanged(DocumentEvent event) {
		}
	}

	public class SelectionListener implements ISelectionChangedListener {
		@Override
		public void selectionChanged(SelectionChangedEvent event) {
			Display.getDefault().asyncExec(() -> inputStatObs.forEach(c -> c.accept(InputMode.Selection)));
		}
	}

	public class TextChangeListener implements ITextListener {
		@Override
		public void textChanged(TextEvent event) {
			Display.getDefault().asyncExec(() -> {
				inputStatObs.forEach(c -> c.accept(InputMode.Selection));
				inputStatObs.forEach(c -> c.accept(InputMode.Converter));
			});
		}
	}
}