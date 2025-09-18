package xy.ai.workbench.handlers;

import java.io.ByteArrayInputStream;
import java.io.InputStream;

import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.runtime.Path;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.ui.IWorkbench;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.dialogs.WizardNewFileCreationPage;
import org.eclipse.ui.ide.IDE;
import org.eclipse.ui.wizards.newresource.BasicNewFileResourceWizard;

public abstract class AbstractNewFileWizard extends BasicNewFileResourceWizard {
	private WizardNewFileCreationPage mainPage;

	protected abstract String getFileName();

	protected InputStream getFileContent() {
		return new ByteArrayInputStream(new byte[0]);
	}

	@Override
	public void init(IWorkbench workbench, IStructuredSelection selection) {
		super.init(workbench, selection);
		setWindowTitle("New " + getFileName() + " File");
	}

	@Override
	public void addPages() {
		super.addPages();
		mainPage = (WizardNewFileCreationPage) getPage("newFilePage1");
		if (mainPage != null) {
			mainPage.setTitle("Create a " + getFileName() + " File");
			mainPage.setFileName(getFileName());
		}
		boolean success = performFinish();

		if (success && getShell() != null)
			getShell().getDisplay().asyncExec(() -> getShell().close());
	}

	@Override
	public boolean performFinish() {
		Object firstElement = selection.getFirstElement();
		if (!(firstElement instanceof IContainer))
			return false;

		IContainer container = (IContainer) firstElement;
		IFile file = container.getFile(new Path(getFileName()));

		if (!file.exists())
			try (InputStream stream = getFileContent()) {
				file.create(stream, true, null);
			} catch (Exception e) {
				throw new IllegalArgumentException(e);
			}

		try {
			IWorkbenchPage page = getWorkbench().getActiveWorkbenchWindow().getActivePage();
			IDE.openEditor(page, file, true);
		} catch (PartInitException e) {
			throw new IllegalArgumentException(e);
		}
		return true;
	}
}