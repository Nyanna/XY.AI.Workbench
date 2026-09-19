package xy.ai.workbench.connector.claude;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Random;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;

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

import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connector.IAIConnector;
import xy.ai.workbench.connector.harness.FrozenConfig;
import xy.ai.workbench.connector.harness.Prompt;
import xy.ai.workbench.connector.mcp.MCPClient;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connector.harness.SessionAnswerBuilder;
import xy.ai.workbench.connector.harness.SessionCallbacks;
import xy.ai.workbench.connector.harness.SessionProcessor;
import xy.ai.workbench.models.AIAnswer;

public class ClaudeConnector implements IAIConnector<ClaudeRequest, ClaudeResponse> {
	private final MCPClient mcpClient;
	private final ObjectMapper mapper = new ObjectMapper();
	private final SessionProcessor sessionProcessor;

	public ClaudeConnector(MCPClient mcpClient, SessionProcessor sessionProcessor) {
		this.mcpClient = mcpClient;
		this.sessionProcessor = sessionProcessor;
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.Claude;
	}

	@SuppressWarnings("deprecation")
	@Override
	public ClaudeRequest createRequest(Prompt prompt, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "BuildRequest", 1);
		FrozenConfig fc = prompt.config;

		Builder builder = MessageCreateParams.builder();

		builder.metadata(Metadata.builder().userId(new Random().nextInt(Integer.MAX_VALUE) + "").build());

		if (Reasoning.Disabled.equals(fc.reasoning)) {
			builder.temperature(fc.temperature);
			builder.topP(fc.topP);
		} else
			builder.thinking(ThinkingConfigEnabled.builder()//
					.budgetTokens(fc.reasoningBudget).build());

		builder.model(fc.model.apiName);
		builder.maxTokens(fc.maxOutputTokens);

		if (fc.systemPrompt != null && !fc.systemPrompt.isBlank())
			builder.system(fc.systemPrompt);

		if (prompt.inputs != null && !prompt.inputs.isEmpty()) {
			// Session text (markers/includes) is parsed deterministically; unmarked plain
			// text still ends up as one system message per input, same as before.
			SessionCallbacks<Void> cb = (role, text) -> {
				builder.addSystemMessage(text);
				return null;
			};
			sessionProcessor.process(prompt.inputs, prompt.arg.processorEnabled, cb);
		}

		if (fc.tools != null && !fc.tools.isEmpty())
			appendTools(builder, fc.tools);

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
		var client = AnthropicOkHttpClient.builder().apiKey(req.getPrompt().config.keys).build();
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

		SessionAnswerBuilder answer = new SessionAnswerBuilder();
		for (ContentBlock content : msg.content())
			if (content.isText())
				answer.text(content.asText().text());
			else if (content.isToolUse()) {
				ToolUseBlock toolUse = content.asToolUse();
				JsonNode args;
				try {
					args = mapper.readTree(toolUse._input().toString());
				} catch (Exception e) {
					args = mapper.createObjectNode();
				}
				answer.toolCall(toolUse.id(), toolUse.name(), args);
			}

		res.answer = answer.toString();
		sub.worked(1);
		return res;
	}
}
