package xy.ai.workbench.editor.spellcheck;

import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextViewer;
import org.eclipse.jface.text.Region;
import org.eclipse.jface.text.reconciler.IReconciler;
import org.eclipse.jface.text.reconciler.IReconcilingStrategy;
import org.eclipse.jface.text.source.ISourceViewer;

import xy.ai.workbench.editor.mdast.TextRegion;
import xy.ai.workbench.editor.mdast.nodes.Node;
import xy.ai.workbench.editor.update.EditorManager;

public class SpellCheckReconciler implements IReconciler, EditorManager.Listener {

	private final EditorManager manager;
	private final SpellingStrategy strategy;

	public SpellCheckReconciler(ISourceViewer sourceViewer, EditorManager manager) {
		this.manager = manager;
		this.strategy = new SpellingStrategy(sourceViewer);
	}

	@Override
	public void install(ITextViewer textViewer) {
		manager.removeListener(this);
		manager.addListener(this);

		IDocument doc = manager.getDocument();
		if (doc != null && manager.getAst() != null) {
			strategy.setDocument(doc);
			manager.runAsync(() -> strategy.reconcile(new Region(0, doc.getLength())));
		}
	}

	@Override
	public void uninstall() {
		manager.removeListener(this);
	}

	@Override
	public IReconcilingStrategy getReconcilingStrategy(String contentType) {
		return IDocument.DEFAULT_CONTENT_TYPE.equals(contentType) ? strategy : null;
	}

	private void apply(Node node, IRegion region) {
		if (node != null && !node.enableSpellcheck)
			strategy.clear(region);
		else
			strategy.reconcile(region);
	}

	@Override
	public void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {
		strategy.setDocument(newDocument);
	}

	@Override
	public void onAstUpdated(TextRegion region) {
		if (region != null)
			manager.runAsync(() -> apply(region.n(), new Region(region.offset(), region.length())));
	}
}
