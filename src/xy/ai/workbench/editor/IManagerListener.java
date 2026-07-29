package xy.ai.workbench.editor;

import org.eclipse.jface.text.IDocument;

import xy.ai.workbench.editor.mdast.nodes.Node;

public interface IManagerListener {

	default void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {
	}

	/**
	 * Fired once, on the UI thread, after the debounced AST reparse for a batch of
	 * edits has completed, with the resulting changed region.
	 */
	default void onAstUpdated(Node node) {
	}
}