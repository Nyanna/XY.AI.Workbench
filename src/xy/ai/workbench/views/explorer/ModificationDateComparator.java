package xy.ai.workbench.views.explorer;

import org.eclipse.core.resources.IResource;
import org.eclipse.jface.viewers.*;

public class ModificationDateComparator extends ViewerComparator {

	@Override
	public int compare(Viewer viewer, Object e1, Object e2) {
		if (!(e1 instanceof IResource) || !(e2 instanceof IResource))
			return super.compare(viewer, e1, e2);

		IResource r1 = (IResource) e1;
		IResource r2 = (IResource) e2;

		if (r1.getType() != r2.getType())
			return Integer.compare(r1.getType(), r2.getType());

		return Long.compare(r2.getLocalTimeStamp(), r1.getLocalTimeStamp());
	}
}