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
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.ui.editors.text.TextEditor;
import org.eclipse.ui.views.contentoutline.IContentOutlinePage;

import xy.ai.workbench.editors.spellcheck.SpellCheckInstaller;
import xy.ai.workbench.mdast.MarkdownDocument;
import xy.ai.workbench.mdast.nodes.Node;

public class AITextEditor extends TextEditor {
	private static final int LIMIT = 512 * 1024;
	private boolean rulerVisible = true;
	private CompositeRuler ruler;
	private List<IVerticalRulerColumn> decorators = new ArrayList<>();

	private MarkdownDocument ast;
	private DocumentBuffer astBuffer;
	private int pendingRemoved;

	private MarkdownOutlinePage outlinePage;

	private final IDocumentListener docListener = new IDocumentListener() {
		@Override
		public void documentChanged(DocumentEvent evt) {
			updateRulerVisibility(evt.getDocument());
			updateLineNumbers(evt.getDocument());
			updateAst(evt);
			refreshOutline();
		}

		@Override
		public void documentAboutToBeChanged(DocumentEvent evt) {
			pendingRemoved = evt.getLength();
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
					buildAst(newInput);
					refreshOutline();
				} else {
					ast = null;
					astBuffer = null;
					refreshOutline();
				}
			}
		});

		return sourceViewer;
	}

	@Override
	public void createPartControl(Composite parent) {
		super.createPartControl(parent);
		SpellCheckInstaller.installPainter(getSourceViewer());

		if (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)
			widget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));
	}

	private void handleCaretMoved(int offset) {
		if (outlinePage != null)
			outlinePage.selectNodeForOffset(offset);
	}

	@Override
	public <T> T getAdapter(Class<T> adapter) {
		if (IContentOutlinePage.class.equals(adapter)) {
			if (outlinePage == null)
				outlinePage = new MarkdownOutlinePage(this);
			return adapter.cast(outlinePage);
		}
		return super.getAdapter(adapter);
	}

	private void refreshOutline() {
		if (outlinePage != null)
			outlinePage.refresh();
	}

	public void selectAndRevealNode(Node node) {
		if (node == null)
			return;
		selectAndReveal(node.getOffset(), node.length());
	}

	public IDocument getMarkdownDocument() {
		return astBuffer != null ? astBuffer.document() : null;
	}

	@Override
	protected boolean getInitialWordWrapStatus() {
		return true;
	}

	private void buildAst(IDocument document) {
		astBuffer = new DocumentBuffer(document);
		ast = new MarkdownDocument(astBuffer);
		ast.update(0, 0, astBuffer.length());
	}

	private void updateAst(DocumentEvent evt) {
		if (ast == null || astBuffer == null || astBuffer.document() != evt.getDocument())
			return;
		String text = evt.getText();
		int inserted = text == null ? 0 : text.length();
		ast.update(evt.getOffset(), pendingRemoved, inserted);
	}

	public MarkdownDocument getMarkdownAst() {
		return ast;
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

		if (size > LIMIT && it.hasNext()) {
			while (it.hasNext() && (d = it.next()) != null)
				decorators.add(d);
			for (var dec : decorators)
				ruler.removeDecorator(dec);
		} else if (size < LIMIT && !it.hasNext() && !decorators.isEmpty()) {
			for (var i = 0; i < decorators.size(); i++)
				ruler.addDecorator(i, decorators.get(i));
			decorators.clear();
		}
	}
}