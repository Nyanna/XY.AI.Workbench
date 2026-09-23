package xy.ai.workbench.handlers;

import org.eclipse.core.commands.AbstractHandler;
import org.eclipse.core.commands.ExecutionEvent;
import org.eclipse.core.commands.ExecutionException;
import org.eclipse.swt.widgets.Display;

import xy.ai.workbench.Activator;

public class SessionExecuteHandler extends AbstractHandler {

	@Override
	public Object execute(ExecutionEvent event) throws ExecutionException {
		Activator.getDefault().promptHandler.execute(Display.getCurrent());
		return null;
	}
}
