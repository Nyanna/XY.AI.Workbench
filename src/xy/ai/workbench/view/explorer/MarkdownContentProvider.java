package xy.ai.workbench.view.explorer;

import org.eclipse.core.resources.IContainer;

import xy.ai.workbench.view.FlatEndingContentProvider;

public class MarkdownContentProvider extends FlatEndingContentProvider {
	public MarkdownContentProvider() {
		super(".md");
	}

	@Override
	public Object getParent(Object element) {
		if (element instanceof IContainer)
			return null;
		return super.getParent(element);
	}
}
