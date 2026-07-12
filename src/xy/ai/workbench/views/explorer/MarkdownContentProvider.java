package xy.ai.workbench.views.explorer;

import java.util.ArrayList;
import java.util.List;

import org.eclipse.core.resources.IResource;

import xy.ai.workbench.views.FlatEndingContentProvider;

public class MarkdownContentProvider extends FlatEndingContentProvider {
	public MarkdownContentProvider() {
		super(".md");
	}

	@Override
	public Object[] getElements(Object root) {
		return filterIgnored(super.getElements(root));
	}

	@Override
	public Object[] getChildren(Object parentElement) {
		return filterIgnored(super.getChildren(parentElement));
	}

	/**
	 * Removes every element that is located inside a directory excluded by a
	 * ".gitignore" file (or that is itself excluded).
	 */
	private Object[] filterIgnored(Object[] elements) {
		List<Object> result = new ArrayList<>(elements.length);
		for (Object element : elements) {
			if (element instanceof IResource && GitIgnoreFilter.isIgnored((IResource) element))
				continue;
			result.add(element);
		}
		return result.toArray();
	}
}
