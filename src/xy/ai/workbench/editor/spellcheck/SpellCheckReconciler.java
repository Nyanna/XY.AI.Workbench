package xy.ai.workbench.editor.spellcheck;

import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.source.ISourceViewer;

import xy.ai.workbench.editor.EditorManager;
import xy.ai.workbench.editor.ISpellChecker;
import xy.ai.workbench.editor.mdast.ModificationRange;
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
	public void reconcile(ModificationRange range) {
		reconcileLeaves(range.getNode(), range.getStart(), range.getEnd());
	}

	private void reconcileLeaves(Node node, int rangeStart, int rangeEnd) {
		int nodeStart = node.getOffset();
		int nodeEnd = node.getEndOffset();
		if (nodeEnd <= rangeStart || nodeStart >= rangeEnd)
			return;

		int start = Math.max(nodeStart, rangeStart);
		int end = Math.min(nodeEnd, rangeEnd);

		if (!node.enableSpellcheck)
			strategy.clear(node, start, end);
		else if (node.children.isEmpty())
			strategy.reconcile(node, start, end);
		else
			for (Node child : node.children)
				reconcileLeaves(child, rangeStart, rangeEnd);
	}
}
