package xy.ai.workbench.view.explorer;

import org.eclipse.core.resources.IProject;
import org.eclipse.core.resources.IResource;
import org.eclipse.jface.viewers.Viewer;
import org.eclipse.jface.viewers.ViewerFilter;

/**
 * Common Navigator Framework filter that limits the tree to the resources
 * of a single, "focused" project (usually the project of the currently
 * focused editor).
 * <p>
 * Contributed as a {@code commonFilter} (see {@code plugin.xml}) and
 * activated by default. {@link MarkdownExplorerView} additionally exposes a
 * dedicated "Filter to Project" toolbar action that lets the user toggle
 * this filter on/off and keeps {@link #setFocusedProject(IProject)} in
 * sync with the focused editor.
 * <p>
 * The Common Navigator Framework instantiates {@code commonFilter} classes
 * via reflection, so a plain instance field cannot be used to track the
 * focused project; it is instead kept in a static field. For the same
 * reason, {@link #equals(Object)}/{@link #hashCode()} are overridden to
 * treat every instance as equivalent, so that {@link MarkdownExplorerView}
 * can add/remove "the" filter on the viewer regardless of which particular
 * instance (its own, or the one created by the framework) is currently
 * installed.
 */
public class ProjectFilter extends ViewerFilter {

	private static volatile IProject focusedProject;

	public static void setFocusedProject(IProject project) {
		focusedProject = project;
	}

	public static IProject getFocusedProject() {
		return focusedProject;
	}

	@Override
	public boolean select(Viewer viewer, Object parentElement, Object element) {
		IProject project = focusedProject;
		if (project == null || !(element instanceof IResource))
			return true;
		return project.equals(((IResource) element).getProject());
	}

	@Override
	public boolean equals(Object obj) {
		return obj instanceof ProjectFilter;
	}

	@Override
	public int hashCode() {
		return ProjectFilter.class.hashCode();
	}
}
