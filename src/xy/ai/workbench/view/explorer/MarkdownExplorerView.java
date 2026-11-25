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
import xy.ai.conscious.core.LOG;
import java.io.ByteArrayInputStream;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import org.eclipse.core.commands.AbstractHandler;
import org.eclipse.core.commands.ExecutionEvent;
import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFolder;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.Path;
import org.eclipse.jface.action.Action;
import org.eclipse.jface.dialogs.IInputValidator;
import org.eclipse.jface.dialogs.InputDialog;
import org.eclipse.jface.dialogs.MessageDialog;
import org.eclipse.jface.window.Window;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.handlers.IHandlerService;

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
    ResourcesPlugin.getWorkspace().addResourceChangeListener(resourceChangeListener, IResourceChangeEvent.POST_CHANGE);
    trackFocusedProject();
    createNewFileAction();
    registerRenameHandler();
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

  private void createNewFileAction() {
    Action newAction = new Action("New") {

      @Override
      public void run() {
        createNewMarkdownFile();
      }
    };
    newAction.setToolTipText("New Markdown file in the selected directory");
    newAction.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_TOOL_NEW_WIZARD));
    getViewSite().getActionBars().getToolBarManager().add(newAction);
    getViewSite().getActionBars().updateActionBars();
  }

  private IResource getSelectedResource() {
    ISelection selection = getCommonViewer().getSelection();
    if (selection instanceof IStructuredSelection) {
      Object element = ((IStructuredSelection) selection).getFirstElement();
      if (element instanceof IResource)
        return (IResource) element;
    }
    return null;
  }

  private void createNewMarkdownFile() {
    IResource selected = getSelectedResource();
    IContainer dir = null;
    if (selected instanceof IContainer)
      dir = (IContainer) selected;
    else if (selected != null)
      dir = selected.getParent();
    if (dir == null || !dir.exists()) {
      MessageDialog.openInformation(getSite().getShell(), "New Markdown file", "Please select a directory first.");
      return;
    }
    String name = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyMMdd.HHmmss")) + ".md";
    IFile file = dir.getFile(new Path(name));
    try {
      if (!file.exists())
        file.create(new ByteArrayInputStream(new byte[0]), true, null);
      getCommonViewer().refresh();
      getCommonViewer().setSelection(new org.eclipse.jface.viewers.StructuredSelection(file), true);
      openFile(file);
    } catch (CoreException e) {
      LOG.error(e.getMessage(), e);
    }
  }

  private void registerRenameHandler() {
    IHandlerService handlerService = getSite().getService(IHandlerService.class);
    if (handlerService == null)
      return;
    handlerService.activateHandler("org.eclipse.ui.edit.rename", new AbstractHandler() {

      @Override
      public Object execute(ExecutionEvent event) {
        renameSelected();
        return null;
      }
    });
  }

  private void renameSelected() {
    IResource resource = getSelectedResource();
    if (!(resource instanceof IFile) && !(resource instanceof IFolder))
      return;
    IContainer parent = resource.getParent();
    IInputValidator validator = newName -> {
      if (newName == null || newName.trim().isEmpty())
        return "Name must not be empty";
      if (newName.contains("/") || newName.contains("\\"))
        return "Name must not contain path separators";
      if (!newName.equals(resource.getName()) && parent.findMember(newName) != null)
        return "A resource with this name already exists";
      return null;
    };
    InputDialog dialog = new InputDialog(getSite().getShell(), "Rename", "New name for '" + resource.getName() + "':", resource.getName(), validator);
    if (dialog.open() != Window.OK)
      return;
    String newName = dialog.getValue().trim();
    if (newName.equals(resource.getName()))
      return;
    try {
      resource.move(parent.getFullPath().append(newName), IResource.KEEP_HISTORY, null);
      getCommonViewer().refresh();
    } catch (CoreException e) {
      LOG.error(e.getMessage(), e);
      MessageDialog.openError(getSite().getShell(), "Rename", e.getMessage());
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
