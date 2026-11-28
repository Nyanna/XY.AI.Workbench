package xy.ai.workbench.editor.navigate;

import java.nio.file.Path;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.IStatus;
import org.eclipse.core.runtime.Status;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.hyperlink.IHyperlink;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.ide.IDE;
import org.eclipse.ui.statushandlers.StatusManager;

class FileHyperlink implements IHyperlink {
  private final IRegion region;
  private final Path file;

  FileHyperlink(IRegion region, Path file) {
    this.region = region;
    this.file = file;
  }

  @Override
  public IRegion getHyperlinkRegion() {
    return region;
  }

  @Override
  public String getTypeLabel() {
    return "Open File";
  }

  @Override
  public String getHyperlinkText() {
    return "Open " + file.getFileName();
  }

  @Override
  public void open() {
    try {
      IWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();
      IFile[] files = ResourcesPlugin.getWorkspace().getRoot().findFilesForLocationURI(file.toUri());
      if (files.length > 0)
        IDE.openEditor(page, files[0]);
      else
        IDE.openEditor(page, file.toUri(), IDE.getEditorDescriptor(file.getFileName().toString(), true, false).getId(), true);
    } catch (CoreException e) {
      StatusManager.getManager().handle(new Status(IStatus.ERROR, "xy.ai.workbench", "Cannot open " + file, e), StatusManager.SHOW);
    }
  }
}
