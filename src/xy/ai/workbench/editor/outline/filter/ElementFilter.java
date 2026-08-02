package xy.ai.workbench.editor.outline.filter;

import org.eclipse.jface.viewers.Viewer;
import org.eclipse.jface.viewers.ViewerFilter;

import xy.ai.workbench.editor.mdast.nodes.AbstractNode;
import xy.ai.workbench.editor.mdast.nodes.Node;

public abstract class ElementFilter extends ViewerFilter {

	@Override
	public boolean select(Viewer viewer, Object parentElement, Object element) {
		return !(element instanceof Node node) || !matches(node.instance);
	}

	protected abstract boolean matches(AbstractNode instance);

	@Override
	public boolean equals(Object obj) {
		return obj != null && obj.getClass() == getClass();
	}

	@Override
	public int hashCode() {
		return getClass().hashCode();
	}
}
