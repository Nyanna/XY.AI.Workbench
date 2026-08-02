package xy.ai.workbench.view.explorer;

import org.eclipse.core.resources.IResource;
import org.eclipse.jface.viewers.Viewer;
import org.eclipse.jface.viewers.ViewerFilter;

public class GitIgnoreViewerFilter extends ViewerFilter {

	@Override
	public boolean select(Viewer viewer, Object parentElement, Object element) {
		return !(element instanceof IResource) || !GitIgnoreFilter.isIgnored((IResource) element);
	}

	@Override
	public boolean equals(Object obj) {
		return obj instanceof GitIgnoreViewerFilter;
	}

	@Override
	public int hashCode() {
		return GitIgnoreViewerFilter.class.hashCode();
	}
}
