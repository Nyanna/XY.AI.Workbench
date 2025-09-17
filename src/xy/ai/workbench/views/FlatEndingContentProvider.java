package xy.ai.workbench.views;

import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.TreeSet;

import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IProject;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.resources.IResourceVisitor;
import org.eclipse.core.resources.IWorkspaceRoot;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.jface.viewers.ITreeContentProvider;

public class FlatEndingContentProvider implements ITreeContentProvider {
	private String ending;

	public FlatEndingContentProvider(String ending) {
		this.ending = ending;
	}

	@Override
	public Object[] getElements(Object root) {
		Object inputElement = ((IWorkspaceRoot) root).getProjects();
		Set<IContainer> dirs = new TreeSet<>((a, b) -> a.getName().compareTo(b.getName()));
		for (var proj : (Object[]) inputElement)
			try {
				IResourceVisitor visitor = resource -> {
					if (resource instanceof IFile) {
						IFile file = (IFile) resource;
						if (file.getName().endsWith(ending))
							dirs.add(file.getParent());
					}
					return true;
				};
				((IProject) proj).accept(visitor);
			} catch (CoreException e) {
			}
		return dirs.toArray();
	}

	@Override
	public Object[] getChildren(Object parentElement) {
		if (parentElement instanceof IContainer) {
			IContainer parent = (IContainer) parentElement;
			List<IFile> files = new ArrayList<>();
			try {
				IResourceVisitor visitor = resource -> {
					if (resource instanceof IFile) {
						IFile file = (IFile) resource;
						if (file.getName().endsWith(ending))
							files.add(file);
					}
					return true;
				};
				parent.accept(visitor, 1, IResource.NONE);
			} catch (CoreException e) {
			}
			return files.toArray();
		}
		return new Object[0];
	}

	@Override
	public boolean hasChildren(Object element) {
		return getChildren(element).length > 0;
	}

	@Override
	public Object getParent(Object element) {
		if (element instanceof IResource)
			return ((IResource) element).getParent();
		return null;
	}
}