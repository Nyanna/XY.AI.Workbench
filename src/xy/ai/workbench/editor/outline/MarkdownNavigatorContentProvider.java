package xy.ai.workbench.editor.outline;

import org.eclipse.jface.viewers.ITreeContentProvider;

public class MarkdownNavigatorContentProvider implements ITreeContentProvider {

	private static final Object[] EMPTY = new Object[0];

	@Override
	public Object[] getElements(Object input) {
		return getChildren(input);
	}

	@Override
	public Object[] getChildren(Object element) {
		if (element instanceof NodeElement ne)
			return ne.children();
		return EMPTY;
	}

	@Override
	public Object getParent(Object element) {
		if (element instanceof NodeElement ne)
			return ne.parent();
		return null;
	}

	@Override
	public boolean hasChildren(Object element) {
		if (element instanceof NodeElement ne)
			return ne.node().children.size() > 0;
		return false;
	}
}
