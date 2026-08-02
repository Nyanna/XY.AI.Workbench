package xy.ai.workbench.editor.outline;

import org.eclipse.jface.viewers.ITreeContentProvider;

import xy.ai.workbench.editor.mdast.MarkdownDocument;
import xy.ai.workbench.editor.mdast.nodes.Node;

public class MarkdownNavigatorContentProvider implements ITreeContentProvider {

	private static final Object[] EMPTY = new Object[0];

	@Override
	public Object[] getElements(Object input) {
		return getChildren(input);
	}

	@Override
	public Object[] getChildren(Object element) {
		if (element instanceof MarkdownDocument doc)
			return doc.getRoot() != null ? doc.getRoot().children.toArray() : EMPTY;
		if (element instanceof Node node)
			return node.children.toArray();
		return EMPTY;
	}

	@Override
	public Object getParent(Object element) {
		return element instanceof Node node ? node.parent : null;
	}

	@Override
	public boolean hasChildren(Object element) {
		return getChildren(element).length > 0;
	}
}
