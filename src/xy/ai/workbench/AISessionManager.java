package xy.ai.workbench;

import java.util.ArrayList;
import java.util.Collection;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.function.Consumer;

import org.eclipse.swt.widgets.Display;

import com.fasterxml.jackson.databind.JsonNode;

import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.connector.AdaptingConnector;
import xy.ai.workbench.connector.harness.PromptHandler;
import xy.ai.workbench.connector.harness.SessionProcessor;
import xy.ai.workbench.connector.mcp.MCPClient;
import xy.ai.workbench.models.AIAnswer;


public class AISessionManager {
	public static final String CONTEXT_PROMPT_TXT = "context.prompt.txt";

	private ActiveEditorListener editorListener = new ActiveEditorListener(this);
	private IncludeAdapter includeAdapter;
	private final MCPClient mcpClient;
	public final EditorInterface editIfc;
	private final PromptHandler promptHandler;
	

	public AISessionManager(ConfigManager cfg, AdaptingConnector connector, MCPClient mcpClient,
			SessionProcessor sessionProcessor) {
		this.mcpClient = mcpClient;
		editIfc = new EditorInterface(editorListener, connector, cfg);
		includeAdapter = new IncludeAdapter(editorListener);
		promptHandler = new PromptHandler(cfg, connector, editorListener, editIfc, includeAdapter);
		sessionProcessor.setAdapter(includeAdapter);
		cfg.addInputModeObs(i -> updateInputStat(i));
		cfg.addEnabledToolsObs(t -> updateInputStat(InputMode.Tools), false);
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

	public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {
		promptHandler.addInputStatObs(obs, initialize);
	}

	public void addAnswerObs(Consumer<AIAnswer> obs) {
		promptHandler.addAnswerObs(obs);
	}

	public void updateInputStat(InputMode mode) {
		promptHandler.updateInputStat(mode);
	}

	public void initializeInputs() {
		promptHandler.initializeInputs();
	}

	public void execute(Display display) {
		promptHandler.execute(display);
	}

	public void queueAsync(Display display, AIBatchManager batch) {
		promptHandler.queueAsync(display, batch);
	}

	public void queueAndSubmit(Display display, AIBatchManager batch) {
		promptHandler.queueAndSubmit(display, batch);
	}
}
