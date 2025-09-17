package xy.ai.workbench.handlers;

import xy.ai.workbench.AISessionManager;

public class NewPromptContextWizard extends AbstractNewFileWizard {
	@Override
	protected String getFileName() {
		return AISessionManager.CONTEXT_PROMPT_TXT;
	}
}
