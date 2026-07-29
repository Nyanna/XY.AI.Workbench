package xy.ai.workbench.editor;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.source.CompositeRuler;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.text.source.IVerticalRuler;
import org.eclipse.jface.text.source.IVerticalRulerColumn;
import org.eclipse.jface.text.source.SourceViewer;
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.editors.text.TextEditor;
import org.eclipse.ui.views.contentoutline.IContentOutlinePage;
import org.eclipse.jface.text.ITextViewerExtension2;
import org.eclipse.jface.text.source.AnnotationPainter;
import org.eclipse.swt.SWT;
import org.eclipse.ui.texteditor.DefaultMarkerAnnotationAccess;

import xy.ai.workbench.editor.mdast.nodes.Node;
import xy.ai.workbench.editor.spellcheck.SpellCheckReconciler;
import xy.ai.workbench.editor.spellcheck.SpellingAnnotation;

public class AITextEditor extends TextEditor {
	private static final int LIMIT = 512 * 1024;
	private boolean rulerVisible = true;
	private CompositeRuler ruler;
	private List<IVerticalRulerColumn> decorators = new ArrayList<>();

	private final EditorManager manager = new EditorManager();

	private MarkdownOutlinePage outlinePage;

	public AITextEditor() {
		super();
		setSourceViewerConfiguration(new AISourceViewerConfiguration(manager));
	}

	@Override
	protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {
		ISourceViewer sourceViewer = super.createSourceViewer(parent, ruler, styles);

		if (ruler instanceof CompositeRuler)
			this.ruler = (CompositeRuler) ruler;

		manager.addListener(new ManagerListener());
		manager.install(sourceViewer);
		new SpellCheckReconciler(sourceViewer, manager);

		return sourceViewer;
	}

	@Override
	public void createPartControl(Composite parent) {
		super.createPartControl(parent);
		installPainter(getSourceViewer());

		if (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)
			widget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));
	}

	private void installPainter(ISourceViewer sourceViewer) {
		Display display = sourceViewer.getTextWidget().getDisplay();

		AnnotationPainter painter = new AnnotationPainter(sourceViewer, new DefaultMarkerAnnotationAccess());
		painter.addTextStyleStrategy(SpellingAnnotation.TYPE,
				new AnnotationPainter.UnderlineStrategy(SWT.UNDERLINE_SQUIGGLE));
		painter.addAnnotationType(SpellingAnnotation.TYPE, SpellingAnnotation.TYPE);
		painter.setAnnotationTypeColor(SpellingAnnotation.TYPE, display.getSystemColor(SWT.COLOR_RED));

		// addTextStyleStrategy works through ITextPresentationListener – register
		// explicitly,
		// because addPainter() alone does NOT do this registration.
		// addTextPresentationListener is only on the concrete SourceViewer class, not
		// on ISourceViewer.
		if (sourceViewer instanceof SourceViewer)
			((SourceViewer) sourceViewer).addTextPresentationListener(painter);
		((ITextViewerExtension2) sourceViewer).addPainter(painter);
	}

	@Override
	public void dispose() {
		manager.uninstall();
		super.dispose();
	}

	private void handleCaretMoved(int offset) {
		if (outlinePage != null) {
			var selection = getSourceViewer().getSelectedRange();
			if (selection.y == 0)
				Display.getDefault().asyncExec(() -> outlinePage.selectNodeForOffset(offset));
		}
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

	@Override
	protected boolean getInitialWordWrapStatus() {
		return true;
	}

	public EditorManager getUpdateManager() {
		return manager;
	}

	private class ManagerListener implements IManagerListener {
		@Override
		public void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {
			if (newDocument == null)
				refreshOutline(); // clears the outline; onAstUpdated covers the non-null case
		}

		@Override
		public void onAstUpdated(Node node) {
			IDocument doc = manager.getDocument();
			if (doc == null)
				return;
			updateRulerVisibility(doc);
			updateLineNumbers(doc);
			refreshOutline();
		}
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
