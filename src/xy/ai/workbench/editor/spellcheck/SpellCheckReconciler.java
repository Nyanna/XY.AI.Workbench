package xy.ai.workbench.editor.spellcheck;

import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.source.ISourceViewer;

import xy.ai.workbench.editor.EditorManager;
import xy.ai.workbench.editor.ISpellChecker;
import xy.ai.workbench.editor.mdast.nodes.Node;

public class SpellCheckReconciler implements ISpellChecker {

	private final SpellingStrategy strategy;

	public SpellCheckReconciler(ISourceViewer sourceViewer, EditorManager manager) {
		this.strategy = new SpellingStrategy(sourceViewer);
		manager.setSpellChecker(this);
	}

	@Override
	public void onDocumentChanged(IDocument document) {
		strategy.setDocument(document);
	}

	@Override
	public void reconcile(Node node) {
		reconcileLeaves(node);
	}

	private void reconcileLeaves(Node node) {
		if (!node.enableSpellcheck)
			strategy.clear(node);
		else if (node.children.isEmpty())
			strategy.reconcile(node);
		else
			for (Node child : node.children)
				reconcileLeaves(child);
	}
}
