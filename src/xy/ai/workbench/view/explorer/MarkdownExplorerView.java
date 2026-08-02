package xy.ai.workbench.view.explorer;

import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IResourceChangeEvent;
import org.eclipse.core.resources.IResourceChangeListener;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.jface.viewers.IOpenListener;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.OpenEvent;
import org.eclipse.jface.viewers.TreeViewer;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IPartListener2;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchPart;
import org.eclipse.ui.IWorkbenchPartReference;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.ide.IDE;
import org.eclipse.ui.ide.ResourceUtil;
import org.eclipse.ui.navigator.CommonNavigator;
import org.eclipse.ui.navigator.CommonViewer;

import xy.ai.workbench.LOG;

public class MarkdownExplorerView extends CommonNavigator {
	private IResourceChangeListener resourceChangeListener;
	private IPartListener2 editorFocusListener;

	@Override
	public void createPartControl(Composite aParent) {
		super.createPartControl(aParent);
		CommonViewer viewer = getCommonViewer();
		viewer.setInput(ResourcesPlugin.getWorkspace().getRoot());
		viewer.setComparator(new ModificationDateComparator());

		resourceChangeListener = new IResourceChangeListener() {
			@Override
			public void resourceChanged(IResourceChangeEvent event) {
				Display.getDefault().asyncExec(() -> {
					if (getCommonViewer() != null && !getCommonViewer().getControl().isDisposed())
						getCommonViewer().refresh();
				});
			}
		};

		ResourcesPlugin.getWorkspace().addResourceChangeListener(resourceChangeListener,
				IResourceChangeEvent.POST_CHANGE);

		trackFocusedProject();
	}

	private void trackFocusedProject() {
		IWorkbenchPage page = getSite().getPage();
		editorFocusListener = new IPartListener2() {
			@Override
			public void partActivated(IWorkbenchPartReference partRef) {
				IWorkbenchPart part = partRef.getPart(false);
				if (part instanceof IEditorPart)
					onEditorFocused((IEditorPart) part);
			}
		};
		page.addPartListener(editorFocusListener);

		IEditorPart activeEditor = page.getActiveEditor();
		if (activeEditor != null)
			onEditorFocused(activeEditor);
	}

	private void onEditorFocused(IEditorPart editor) {
		IFile file = ResourceUtil.getFile(editor.getEditorInput());
		if (file == null)
			return;

		if (!file.getProject().equals(ProjectFilter.getFocusedProject())) {
			ProjectFilter.setFocusedProject(file.getProject());
			getCommonViewer().refresh();
		}
	}

	@Override
	protected void initListeners(TreeViewer viewer) {
		super.initListeners(viewer);

		viewer.addOpenListener(new IOpenListener() {
			@Override
			public void open(OpenEvent event) {
				ISelection selection = event.getSelection();
				if (selection instanceof IStructuredSelection) {
					Object element = ((IStructuredSelection) selection).getFirstElement();
					if (element instanceof IFile)
						openFile((IFile) element);
				}
			}
		});
	}

	private void openFile(IFile file) {
		IWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();
		try {
			IDE.openEditor(page, file);
		} catch (PartInitException e) {
			LOG.error(e.getMessage(), e);
		}
	}

	@Override
	public void dispose() {
		if (resourceChangeListener != null)
			ResourcesPlugin.getWorkspace().removeResourceChangeListener(resourceChangeListener);
		if (editorFocusListener != null && getSite() != null && getSite().getPage() != null)
			getSite().getPage().removePartListener(editorFocusListener);
		super.dispose();
	}
}
