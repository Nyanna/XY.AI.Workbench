package xy.ai.workbench;

import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.connector.harness.PromptHandler;
import xy.ai.workbench.connector.harness.SessionProcessor;

public class AISessionManager {
	public static final String CONTEXT_PROMPT_TXT = "context.prompt.txt";

	private final ActiveEditorListener editorListener;
	private final IncludeAdapter includeAdapter;
	public final EditorInterface editIfc;
	private final PromptHandler prompt;

	public AISessionManager(ConfigManager cfg, AdaptingConnector connector, AIBatchManager batch,
			SessionProcessor sessionProcessor) {
		editorListener = new ActiveEditorListener();
		editIfc = new EditorInterface(editorListener, connector);
		includeAdapter = new IncludeAdapter(editorListener);
		prompt = new PromptHandler(cfg, connector, batch, editIfc, includeAdapter);
		editorListener.setPrompt(prompt);
		sessionProcessor.setAdapter(includeAdapter);
		cfg.addInputModeObs(i -> prompt.updateInputStat(i));
		cfg.addEnabledToolsObs(t -> prompt.updateInputStat(InputMode.Tools), false);
	}
	
	public PromptHandler getPromptHandler() {
		return prompt;
	}
}
