package xy.ai.workbench.handlers;

import java.util.List;
import java.util.stream.Collectors;
import org.eclipse.core.commands.AbstractHandler;
import org.eclipse.core.commands.ExecutionEvent;
import org.eclipse.core.commands.ExecutionException;
import org.eclipse.core.resources.IResource;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.swt.dnd.Clipboard;
import org.eclipse.swt.dnd.TextTransfer;
import org.eclipse.swt.dnd.Transfer;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.handlers.HandlerUtil;

/**
 * Copies the absolute file path(s) of the currently selected resource(s), each
 * wrapped in backticks ("`"), to the system clipboard. Falls back to the
 * active editor's input file if no resource selection is available (e.g. when
 * invoked from an editor's context menu).
 */
public class CopyPathHandler extends AbstractHandler {

	@Override
	public Object execute(ExecutionEvent event) throws ExecutionException {
		List<IResource> resources = resolveResources(event);
		if (resources.isEmpty())
			return null;

		String text = resources.stream() //
				.map(resource -> resource.getLocation()) //
				.filter(location -> location != null) //
				.map(location -> "`" + location.toOSString() + "`") //
				.collect(Collectors.joining(System.lineSeparator()));

		if (text.isEmpty())
			return null;

		Clipboard clipboard = new Clipboard(Display.getCurrent());
		try {
			clipboard.setContents(new Object[] { text }, new Transfer[] { TextTransfer.getInstance() });
		} finally {
			clipboard.dispose();
		}
		return null;
	}

	private List<IResource> resolveResources(ExecutionEvent event) {
		Object selectionObj = HandlerUtil.getCurrentSelection(event);
		if (selectionObj instanceof IStructuredSelection) {
			IStructuredSelection selection = (IStructuredSelection) selectionObj;
			List<IResource> fromSelection = selection.stream() //
					.map(this::toResource) //
					.filter(resource -> resource != null) //
					.collect(Collectors.toList());
			if (!fromSelection.isEmpty())
				return fromSelection;
		}

		// Fallback: no (resource) selection available, e.g. invoked from an
		// editor's context menu. Use the active editor's input file instead.
		IEditorPart activeEditor = HandlerUtil.getActiveEditor(event);
		if (activeEditor != null) {
			IEditorInput input = activeEditor.getEditorInput();
			IResource resource = toResource(input);
			if (resource != null)
				return List.of(resource);
		}

		return List.of();
	}

	private IResource toResource(Object element) {
		if (element instanceof IResource)
			return (IResource) element;
		if (element instanceof org.eclipse.core.runtime.IAdaptable)
			return ((org.eclipse.core.runtime.IAdaptable) element).getAdapter(IResource.class);
		return null;
	}
}
