package xy.ai.workbench.handlers;

import org.eclipse.core.commands.AbstractHandler;
import org.eclipse.core.commands.ExecutionEvent;
import org.eclipse.core.commands.ExecutionException;

import xy.ai.workbench.Activator;
import xy.ai.workbench.eclipse.EclipseExecutor;

public class SessionExecuteHandler extends AbstractHandler {

	@Override
	public Object execute(ExecutionEvent event) throws ExecutionException {
		Activator.getDefault().promptHandler.execute(new EclipseExecutor());
		return null;
	}
}
