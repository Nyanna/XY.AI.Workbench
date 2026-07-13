package xy.ai.workbench.editors;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

import org.eclipse.jface.text.DocumentEvent;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IDocumentListener;
import org.eclipse.jface.text.ITextInputListener;
import org.eclipse.jface.text.source.CompositeRuler;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.text.source.IVerticalRuler;
import org.eclipse.jface.text.source.IVerticalRulerColumn;
import org.eclipse.jface.text.source.SourceViewer;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.ui.editors.text.TextEditor;

import xy.ai.workbench.editors.spellcheck.SpellCheckInstaller;

public class AITextEditor extends TextEditor {
	private static final int LIMIT = 512 * 1024;
	private static final int SEGMENT_THRESHOLD = 250;
	private boolean rulerVisible = true;
	private CompositeRuler ruler;
	private List<IVerticalRulerColumn> decorators = new ArrayList<>();

	private final IDocumentListener docListener = new IDocumentListener() {
		@Override
		public void documentChanged(DocumentEvent evt) {
			updateRulerVisibility(evt.getDocument());
			updateLineNumbers(evt.getDocument());
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

		if (ruler instanceof CompositeRuler)
			this.ruler = (CompositeRuler) ruler;

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
					updateLineNumbers(newInput);
				}
			}
		});
		sourceViewer.getTextWidget().addBidiSegmentListener(event -> {
			int length = event.lineText.length();
			if (length <= SEGMENT_THRESHOLD)
				return;

			int segmentSize = 200;
			int segmentCount = length / segmentSize;
			int[] segments = new int[segmentCount + 2];
			segments[0] = 0;
			for (int i = 1; i <= segmentCount; i++)
				segments[i] = i * segmentSize;
			segments[segments.length - 1] = length;
			event.segments = segments;
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

	private void updateLineNumbers(IDocument document) {
		if (ruler == null)
			return;

		long size = document.getLength();
		Iterator<IVerticalRulerColumn> it = ruler.getDecoratorIterator();
		IVerticalRulerColumn d;

		if (it.hasNext() && size > LIMIT) {
			while (it.hasNext() && (d = it.next()) != null)
				decorators.add(d);
			for (var dec : decorators)
				ruler.removeDecorator(dec);
		} else if (!it.hasNext() && !decorators.isEmpty()) {
			for (var i = 0; i < decorators.size(); i++)
				ruler.addDecorator(i, decorators.get(i));
			decorators.clear();
		}
	}
}