package xy.ai.workbench;

import java.util.ArrayList;
import java.util.Collection;
import java.util.LinkedHashSet;
import java.util.List;

import com.fasterxml.jackson.databind.JsonNode;

import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.connector.harness.PromptHandler;
import xy.ai.workbench.connector.harness.SessionProcessor;
import xy.ai.workbench.connector.mcp.MCPClient;

public class AISessionManager {
	public static final String CONTEXT_PROMPT_TXT = "context.prompt.txt";

	private final ActiveEditorListener editorListener;
	private final IncludeAdapter includeAdapter;
	private final MCPClient mcpClient;
	public final EditorInterface editIfc;
	private final PromptHandler prompt;

	public AISessionManager(ConfigManager cfg, AdaptingConnector connector, AIBatchManager batch, MCPClient mcpClient,
			SessionProcessor sessionProcessor) {
		this.mcpClient = mcpClient;
		editorListener = new ActiveEditorListener();
		editIfc = new EditorInterface(editorListener, connector);
		includeAdapter = new IncludeAdapter(editorListener);
		prompt = new PromptHandler(cfg, connector, batch, editIfc, includeAdapter);
		editorListener.setPrompt(prompt);
		sessionProcessor.setAdapter(includeAdapter);
		cfg.addInputModeObs(i -> prompt.updateInputStat(i));
		cfg.addEnabledToolsObs(t -> prompt.updateInputStat(InputMode.Tools), false);
		cfg.addModelObs(this::discoverTools, false);
	}

	private void discoverTools(Model model) {
		if (model == null)
			return;
		List<String> discovered = new ArrayList<>();
		for (JsonNode tool : mcpClient.listTools())
			discovered.add(tool.path("name").asText());
		model.cap.tools(sortByToolOrder(discovered));
	}

	private String[] sortByToolOrder(Collection<String> names) {
		LinkedHashSet<String> remaining = new LinkedHashSet<>(names);
		List<String> ordered = new ArrayList<>();
		for (String known : Tools.ALL)
			if (remaining.remove(known))
				ordered.add(known);
		ordered.addAll(remaining);
		return ordered.toArray(new String[0]);
	}

	public PromptHandler getPromptHandler() {
		return prompt;
	}
}
