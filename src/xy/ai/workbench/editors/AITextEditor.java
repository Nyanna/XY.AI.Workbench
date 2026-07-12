package xy.ai.workbench.editors;

import org.eclipse.jface.text.DocumentEvent;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IDocumentListener;
import org.eclipse.jface.text.ITextInputListener;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.text.source.IVerticalRuler;
import org.eclipse.jface.text.source.SourceViewer;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.ui.editors.text.TextEditor;

import xy.ai.workbench.editors.spellcheck.SpellCheckInstaller;

public class AITextEditor extends TextEditor {
	private static final int LIMIT = 512 * 1024;
	private boolean rulerVisible = true;

	private final IDocumentListener docListener = new IDocumentListener() {
		@Override
		public void documentChanged(DocumentEvent evt) {
			updateRulerVisibility(evt.getDocument());
		}

		@Override
		public void documentAboutToBeChanged(DocumentEvent evt) {
		}
	};

	public AITextEditor() {
		super();
		setSourceViewerConfiguration(new AISourceViewerConfiguration());
	}

	@Override
	protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {
		ISourceViewer sourceViewer = super.createSourceViewer(parent, ruler, styles);
		sourceViewer.addTextInputListener(new ITextInputListener() {
			@Override
			public void inputDocumentAboutToBeChanged(IDocument oldInput, IDocument newInput) {
				if (oldInput != null)
					oldInput.removeDocumentListener(docListener);
			}

			@Override
			public void inputDocumentChanged(IDocument oldInput, IDocument newInput) {
				if (newInput != null) {
					newInput.addDocumentListener(docListener);
					updateRulerVisibility(newInput); // initialer Check
				}
			}
		});

		return sourceViewer;
	}

	@Override
	public void createPartControl(Composite parent) {
		super.createPartControl(parent);
		SpellCheckInstaller.installPainter(getSourceViewer());
	}

	@Override
	protected boolean getInitialWordWrapStatus() {
		return true;
	}

	private void updateRulerVisibility(IDocument document) {
		boolean shouldShow = document.getLength() <= LIMIT;
		if (shouldShow != rulerVisible) {
			rulerVisible = shouldShow;
			if (getSourceViewer() instanceof SourceViewer sv) {
				sv.showAnnotations(shouldShow);
				sv.showAnnotationsOverview(shouldShow);
			}
		}
	}
}