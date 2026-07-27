package xy.ai.workbench.views.explorer;

import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.jface.viewers.*;

public class ModificationDateComparator extends ViewerComparator {

	@Override
	public int compare(Viewer viewer, Object e1, Object e2) {
		if (!(e1 instanceof IResource) || !(e2 instanceof IResource))
			return super.compare(viewer, e1, e2);

		IResource r1 = (IResource) e1;
		IResource r2 = (IResource) e2;

		return Long.compare(effectiveTimeStamp(r2), effectiveTimeStamp(r1));
	}

	private long effectiveTimeStamp(IResource resource) {
		if (resource instanceof IContainer) {
			long newestMarkdown = newestMarkdownTimeStamp((IContainer) resource);
			if (newestMarkdown >= 0)
				return newestMarkdown;
		}
		return resource.getLocalTimeStamp();
	}

	private long newestMarkdownTimeStamp(IContainer container) {
		long newest = -1;
		try {
			for (IResource member : container.members()) {
				if (member instanceof IFile && member.getName().endsWith(".md")) {
					long stamp = member.getLocalTimeStamp();
					if (stamp > newest)
						newest = stamp;
				}
			}
		} catch (CoreException e) {
		}
		return newest;
	}
}
