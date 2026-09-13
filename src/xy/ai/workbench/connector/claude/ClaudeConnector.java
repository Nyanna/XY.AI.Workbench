package xy.ai.workbench.connector.claude;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Random;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;

import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.core.JsonValue;
import com.anthropic.models.messages.ContentBlock;
import com.anthropic.models.messages.Message;
import com.anthropic.models.messages.MessageCreateParams;
import com.anthropic.models.messages.Tool;
import com.anthropic.models.messages.ToolUseBlock;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.anthropic.models.messages.MessageCreateParams.Builder;
import com.anthropic.models.messages.Metadata;
import com.anthropic.models.messages.ThinkingConfigEnabled;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connector.IAIConnector;
import xy.ai.workbench.connector.mcp.MCPClient;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.models.AIAnswer;

public class ClaudeConnector implements IAIConnector<ClaudeRequest, ClaudeResponse> {
	private ConfigManager cfg;
	private AnthropicClient client;
	private final MCPClient mcpClient;
	private final ObjectMapper mapper = new ObjectMapper();

	public ClaudeConnector(ConfigManager cfg, MCPClient mcpClient) {
		this.cfg = cfg;
		this.mcpClient = mcpClient;
		cfg.addKeyObs(k -> {
			if (getSupportedKeyPattern().matches(k))
				this.client = AnthropicOkHttpClient.builder().apiKey(k).build();
		}, true);
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.Claude;
	}

	@SuppressWarnings("deprecation")
	@Override
	public ClaudeRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,
			IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "BuildRequest", 1);

		Builder builder = MessageCreateParams.builder();

		builder.metadata(Metadata.builder().userId(new Random().nextInt(Integer.MAX_VALUE) + "").build());

		if (Reasoning.Disabled.equals(cfg.getReasoning())) {
			builder.temperature(cfg.getTemperature());
			builder.topP(cfg.getTopP());
		} else
			builder.thinking(ThinkingConfigEnabled.builder()//
					.budgetTokens(cfg.getReasoningBudget()).build());

		builder.model(cfg.getModel().apiName);
		builder.maxTokens(cfg.getMaxOutputTokens());

		if (systemPrompt != null && !systemPrompt.isBlank())
			builder.system(systemPrompt);

		if (inputs != null && !inputs.isEmpty())
			for (String input : inputs)
				if (input != null && !input.isBlank())
					builder.addSystemMessage(input);

		if (tools != null && !tools.isEmpty())
			appendTools(builder, tools);

		MessageCreateParams createParams = builder.build();
		sub.worked(1);
		return new ClaudeRequest(createParams);
	}

	private void appendTools(Builder builder, List<String> tools) {
		for (String toolName : tools) {
			JsonNode tool = mcpClient.findTool(toolName);
			if (tool == null)
				continue;
			JsonNode schema = tool.path("inputSchema");
			JsonNode props = schema.path("properties");
			@SuppressWarnings("unchecked")
			Tool.InputSchema.Builder schemaBuilder = Tool.InputSchema.builder()
					.properties(JsonValue.from(props.isObject() ? mapper.convertValue(props, Object.class) : Map.of()));
			if (schema.path("required").isArray()) {
				List<String> required = new ArrayList<>();
				schema.path("required").forEach(r -> required.add(r.asText()));
				schemaBuilder.required(required);
			}
			builder.addTool(Tool.builder()//
					.name(tool.path("name").asText(toolName))//
					.description(tool.path("description").asText(""))//
					.inputSchema(schemaBuilder.build())//
					.build());
		}
	}

	@Override
	public ClaudeResponse executeRequest(ClaudeRequest req, IProgressMonitor mon, Job job) {
		Message message = client.messages().create(req.params);
		return new ClaudeResponse(message, req.getID());
	}

	@Override
	public AIAnswer convertResponse(ClaudeResponse resp, IProgressMonitor mon) {
		Message msg = resp.response;
		SubMonitor sub = SubMonitor.convert(mon, "Convert Respone", 1);

		AIAnswer res = new AIAnswer(resp.id);

		res.stats.inputToken = msg.usage().inputTokens();
		res.stats.outputToken = msg.usage().outputTokens();
//		resp.usage().cacheCreationInputTokens();
//		resp.usage().cacheReadInputTokens();
		res.stats.totalToken = res.stats.inputToken + res.stats.outputToken;

		StringBuffer answer = new StringBuffer();
		for (ContentBlock content : msg.content())
			if (content.isText())
				answer.append(content.asText().text());
			else if (content.isToolUse()) {
				ToolUseBlock toolUse = content.asToolUse();
				JsonNode args;
				try {
					args = mapper.readTree(toolUse._input().toString());
				} catch (Exception e) {
					args = mapper.createObjectNode();
				}
				if (answer.length() > 0)
					answer.append("\n\n");
				answer.append(mcpClient.renderToolCall(toolUse.name(), args));
			}

		res.answer = answer.toString();
		sub.worked(1);
		return res;
	}
}
