package xy.ai.workbench.view.explorer;

import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IProject;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.resources.IResourceChangeEvent;
import org.eclipse.core.resources.IResourceChangeListener;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.jface.viewers.IOpenListener;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.OpenEvent;
import org.eclipse.jface.viewers.StructuredSelection;
import org.eclipse.jface.viewers.TreeViewer;
import org.eclipse.jface.viewers.Viewer;
import org.eclipse.jface.viewers.ViewerFilter;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IActionBars;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IPartListener2;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchPart;
import org.eclipse.ui.IWorkbenchPartReference;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.actions.ActionGroup;
import org.eclipse.ui.ide.IDE;
import org.eclipse.ui.ide.ResourceUtil;
import org.eclipse.ui.navigator.CommonNavigator;
import org.eclipse.ui.navigator.CommonViewer;

import xy.ai.workbench.LOG;
import xy.ai.workbench.view.ActionManager;
import xy.ai.workbench.view.ActionManager.ActionDescription;

public class MarkdownExplorerView extends CommonNavigator {
	private IResourceChangeListener resourceChangeListener;

	/** Builds the view local toolbar/menu, see {@link #createOwnActionBars()}. */
	private final ActionManager act = new ActionManager();

	/** Toggle: when checked, the file of the focused editor is selected. */
	private ActionDescription syncAction;

	/** Toggle (in the view's hamburger menu): when checked, the tree is limited to the focused project. */
	private ActionDescription filterToProjectAction;

	private ViewerFilter projectFilter;
	private IProject focusedProject;
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

		createOwnActionBars();
	}

	/**
	 * Overridden to suppress the default Common Navigator toolbar/menu actions
	 * (back/forward/up, collapse all, link with editor, select filters). This
	 * view provides its own, purpose built toolbar/menu, see
	 * {@link #createOwnActionBars()}.
	 */
	@Override
	protected ActionGroup createCommonActionGroup() {
		return new ActionGroup() {
			@Override
			public void fillActionBars(IActionBars actionBars) {
				// intentionally empty, see createOwnActionBars()
			}
		};
	}

	/**
	 * Builds the view's own toolbar (a "Sync" toggle) and the view's own
	 * drop-down/hamburger menu (further options, currently "Filter to
	 * Project").
	 * <p>
	 * While "Sync" is enabled, the file of the currently focused editor is
	 * selected in the tree. While "Filter to Project" is enabled, the tree is
	 * limited to the project containing the file of the currently focused
	 * editor.
	 */
	private void createOwnActionBars() {
		projectFilter = new ViewerFilter() {
			@Override
			public boolean select(Viewer viewer, Object parentElement, Object element) {
				if (focusedProject == null || !(element instanceof IResource))
					return true;
				return focusedProject.equals(((IResource) element).getProject());
			}
		};

		syncAction = act.create().toolbar()
				.text("Sync", "Select the file of the focused editor")
				.image(ISharedImages.IMG_ELCL_SYNCED)
				.runnable(this::handleSyncToggled);
		syncAction.done();
		syncAction.setChecked(false);

		filterToProjectAction = act.create().pullDown()
				.text("Filter to Project", "Limit the content to the project of the focused editor")
				.runnable(this::handleFilterToProjectToggled);
		filterToProjectAction.done();
		filterToProjectAction.setChecked(false);

		IActionBars actionBars = getViewSite().getActionBars();
		act.fillLocalToolBar(actionBars.getToolBarManager());
		act.fillLocalPullDown(actionBars.getMenuManager());
		actionBars.updateActionBars();

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

	private void handleSyncToggled() {
		if (!syncAction.isChecked())
			return;
		IEditorPart activeEditor = getSite().getPage().getActiveEditor();
		if (activeEditor != null)
			onEditorFocused(activeEditor);
	}

	private void handleFilterToProjectToggled() {
		CommonViewer viewer = getCommonViewer();
		if (filterToProjectAction.isChecked()) {
			IEditorPart activeEditor = getSite().getPage().getActiveEditor();
			IFile file = activeEditor != null ? ResourceUtil.getFile(activeEditor.getEditorInput()) : null;
			if (file != null)
				focusedProject = file.getProject();
			viewer.addFilter(projectFilter);
		} else {
			viewer.removeFilter(projectFilter);
		}
	}

	/**
	 * Reacts to a newly focused editor: updates the "Filter to Project" scope
	 * and, if "Sync" is enabled, selects the editor's file in the tree.
	 */
	private void onEditorFocused(IEditorPart editor) {
		IFile file = ResourceUtil.getFile(editor.getEditorInput());
		if (file == null)
			return;

		if (!file.getProject().equals(focusedProject)) {
			focusedProject = file.getProject();
			if (filterToProjectAction.isChecked())
				getCommonViewer().refresh();
		}

		if (syncAction.isChecked())
			getCommonViewer().setSelection(new StructuredSelection(file), true);
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
