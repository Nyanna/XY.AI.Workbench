package xy.ai.workbench.handlers;

import xy.ai.workbench.IncludeAdapter;

public class NewPromptContextWizard extends AbstractNewFileWizard {
	@Override
	protected String getFileName() {
		return IncludeAdapter.CONTEXT_PROMPT_TXT;
	}
}
