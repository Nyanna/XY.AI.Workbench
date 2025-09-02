package xy.ai.workbench.handlers;

import org.eclipse.core.commands.AbstractHandler;
import org.eclipse.core.commands.ExecutionEvent;
import org.eclipse.core.commands.ExecutionException;

import xy.ai.workbench.AISessionManager;
import xy.ai.workbench.Activator;
import xy.ai.workbench.views.AISessionView;

public class SessionExecuteHandler extends AbstractHandler {

	@Override
	public Object execute(ExecutionEvent event) throws ExecutionException {
		AISessionManager session = Activator.getDefault().session;
		session.execute(AISessionView.currentInstance.display);
		return null;
	}
}
