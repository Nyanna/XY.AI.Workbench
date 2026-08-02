package xy.ai.workbench.view.explorer;

import org.eclipse.core.resources.IFile;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.StructuredSelection;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.ide.ResourceUtil;
import org.eclipse.ui.navigator.ILinkHelper;
import org.eclipse.ui.part.FileEditorInput;

public class MarkdownLinkHelper implements ILinkHelper {

	@Override
	public IStructuredSelection findSelection(IEditorInput anInput) {
		IFile file = ResourceUtil.getFile(anInput);
		if (file != null)
			return new StructuredSelection(file);
		return StructuredSelection.EMPTY;
	}

	@Override
	public void activateEditor(IWorkbenchPage aPage, IStructuredSelection aSelection) {
		if (aSelection == null || aSelection.isEmpty())
			return;
		Object element = aSelection.getFirstElement();
		if (element instanceof IFile) {
			IEditorInput fileInput = new FileEditorInput((IFile) element);
			IEditorPart editor = aPage.findEditor(fileInput);
			if (editor != null)
				aPage.bringToTop(editor);
		}
	}
}
