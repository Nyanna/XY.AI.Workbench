package xy.ai.workbench.editor;

import org.eclipse.core.resources.IResourceChangeEvent;
import org.eclipse.core.resources.IResourceChangeListener;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.jface.dialogs.ErrorDialog;
import org.eclipse.swt.custom.CTabFolder;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.part.FileEditorInput;
import org.eclipse.ui.part.MultiPageEditorPart;
import xy.ai.conscious.core.LOG;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.resources.IResourceDelta;
import org.eclipse.ui.IEditorInput;

public class AISessionEditor extends MultiPageEditorPart implements IResourceChangeListener {
  private AITextEditor editor;

  public AISessionEditor() {
    super();
    ResourcesPlugin.getWorkspace().addResourceChangeListener(this);
  }

  public AITextEditor getEditor() {
    return editor;
  }

  @Override
  protected void createPages() {
    try {
      editor = new AITextEditor();
      editor.setInitializationData(getConfigurationElement(), null, null);
      int index = addPage(editor, getEditorInput());
      setPartName(getEditorInput().getName());
      setPageText(index, editor.getTitle());
      var folder = ((CTabFolder) getContainer());
      folder.setTabHeight(0);
    } catch (PartInitException e) {
      ErrorDialog.openError(getSite().getShell(), "Error creating nested text editor", null, e.getStatus());
      LOG.error("Error initing part", e);
    }
  }

  @Override
  public void dispose() {
    ResourcesPlugin.getWorkspace().removeResourceChangeListener(this);
    super.dispose();
  }

  @Override
  public void doSave(IProgressMonitor mon) {
    SubMonitor sub = SubMonitor.convert(mon, "Save editor", 1);
    getEditor().doSave(sub.split(1));
  }

  @Override
  public void doSaveAs() {
    IEditorPart editor = getEditor();
    editor.doSaveAs();
    setInput(editor.getEditorInput());
    updateTitle();
  }

  @Override
  public boolean isSaveAsAllowed() {
    return true;
  }

  @Override
  public void resourceChanged(final IResourceChangeEvent event) {
    if (event.getType() == IResourceChangeEvent.PRE_CLOSE)
      Display.getDefault().asyncExec(() -> {
        IWorkbenchPage[] pages = getSite().getWorkbenchWindow().getPages();
        for (int i = 0; i < pages.length; i++) {
          if (((FileEditorInput) editor.getEditorInput()).getFile().getProject().equals(event.getResource())) {
            IEditorPart editorPart = pages[i].findEditor(editor.getEditorInput());
            pages[i].closeEditor(editorPart, true);
          }
        }
      });
    else if (event.getType() == IResourceChangeEvent.POST_CHANGE)
      handleMove(event);
  }

  /**
   * Aktualisiert Input und Titel, wenn die Datei (oder ein übergeordneter Ordner)
   * verschoben oder umbenannt wurde.
   */
  private void handleMove(final IResourceChangeEvent event) {
    if (!(getEditorInput() instanceof FileEditorInput fileInput))
      return;
    IResourceDelta rootDelta = event.getDelta();
    if (rootDelta == null)
      return;
    IResourceDelta delta = rootDelta.findMember(fileInput.getFile().getFullPath());
    if (delta == null || (delta.getFlags() & IResourceDelta.MOVED_TO) == 0 || (delta.getKind() & IResourceDelta.REMOVED) == 0)
      return;
    IResource newResource = ResourcesPlugin.getWorkspace().getRoot().findMember(delta.getMovedToPath());
    if (!(newResource instanceof IFile newFile))
      return;
    Display display = Display.getDefault();
    display.asyncExec(() -> {
      if (display.isDisposed() || getContainer() == null || getContainer().isDisposed())
        return;
      setInput(new FileEditorInput(newFile));
      updateTitle();
    });
  }

  private void updateTitle() {
    IEditorInput input = getEditorInput();
    setPartName(input.getName());
    setTitleToolTip(input.getToolTipText());
    if (editor != null && getPageCount() > 0)
      setPageText(0, editor.getTitle());
  }
}
