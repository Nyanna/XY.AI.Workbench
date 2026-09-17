package xy.ai.workbench.connector.openai;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Random;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;

import com.openai.client.OpenAIClient;
import com.openai.client.okhttp.OpenAIOkHttpClient;
import com.openai.core.JsonValue;
import com.openai.core.http.HttpResponseFor;
import com.openai.models.ChatModel;
import com.openai.models.Reasoning;
import com.openai.models.ReasoningEffort;
import com.openai.models.responses.FunctionTool;
import com.openai.models.responses.Response;
import com.openai.models.responses.Response.Instructions;
import com.openai.models.responses.ResponseCreateParams;
import com.openai.models.responses.ResponseCreateParams.Builder;
import com.openai.models.responses.ResponseCreateParams.Truncation;
import com.openai.models.responses.ResponseError;
import com.openai.models.responses.ResponseInputItem;
import com.openai.models.responses.ResponseInputText;
import com.openai.models.responses.ResponseStatus;
import com.openai.models.responses.ResponseUsage;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.LOG;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connector.IAIConnector;
import xy.ai.workbench.connector.harness.FrozenConfig;
import xy.ai.workbench.connector.harness.Prompt;
import xy.ai.workbench.connector.harness.SessionAnswerBuilder;
import xy.ai.workbench.connector.harness.SessionCallbacks;
import xy.ai.workbench.connector.harness.SessionProcessor;
import xy.ai.workbench.connector.mcp.MCPClient;
import xy.ai.workbench.models.AIAnswer;

public class OpenAIConnector implements IAIConnector<OpenAIRequest, OpenAIResponse> {
	private OpenAIClient client;
	private final MCPClient mcpClient;
	private final ObjectMapper mapper = new ObjectMapper();
	private final SessionProcessor sessionProcessor;

	public OpenAIConnector(ConfigManager cfg, MCPClient mcpClient, SessionProcessor sessionProcessor) {
		this.mcpClient = mcpClient;
		this.sessionProcessor = sessionProcessor;
		cfg.addKeyObs(k -> {
			if (getSupportedKeyPattern().matches(k))
				this.client = OpenAIOkHttpClient.builder().apiKey(k).build();
		}, true);
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.OpenAI;
	}

	@SuppressWarnings("deprecation")
	@Override
	public OpenAIRequest createRequest(Prompt prompt, IProgressMonitor mon) {
		boolean isBackground = false;
		SubMonitor sub = SubMonitor.convert(mon, "BuildRequest", 1);
		FrozenConfig fc = prompt.config;

		Builder builder = ResponseCreateParams.builder() //
				.maxOutputTokens(fc.maxOutputTokens)
				.safetyIdentifier(new Random().nextInt(Integer.MAX_VALUE) + "") //
				.truncation(Truncation.DISABLED) //
				.maxToolCalls(0)//
				.background(isBackground)//
				.instructions(fc.systemPrompt)//
				.parallelToolCalls(false)//
				.reasoning( //
						Reasoning.builder()//
								.effort(ReasoningEffort.of(fc.reasoning.name())) //
								.summary(Reasoning.Summary.AUTO)//
								.build())
				.model(ChatModel.of(fc.model.apiName)); //

		if (fc.model.cap.isSupportTemperature())
			builder.temperature(fc.temperature);

		if (fc.model.cap.isSupportTopP())
			builder.topP(fc.topP);

		if (prompt.inputs != null && !prompt.inputs.isEmpty()) {
			List<ResponseInputItem> respInputs = new ArrayList<>();
			SessionCallbacks<Void> cb = (role, text) -> {
				ResponseInputText inputText = ResponseInputText.builder().text(text).build();
				respInputs.add(ResponseInputItem.ofMessage(ResponseInputItem.Message.builder() //
						.role(ResponseInputItem.Message.Role.DEVELOPER)//
						.addContent(inputText).build()));
				return null;
			};
			sessionProcessor.process(prompt.inputs, prompt.processorEnabled, cb);
			if (!respInputs.isEmpty())
				builder.inputOfResponse(respInputs);
		}

		if (fc.tools != null && !fc.tools.isEmpty())
			appendTools(builder, fc.tools);

		ResponseCreateParams params = builder.build();
		sub.worked(1);
		return new OpenAIRequest(params);
	}

	@Override
	public OpenAIResponse executeRequest(OpenAIRequest req, IProgressMonitor mon, Job job) {
		ResponseCreateParams params = ((OpenAIRequest) req).reqquest;
		boolean isBackground = params.background().orElse(Boolean.FALSE);

		HttpResponseFor<Response> rwResponse = client.responses().withRawResponse().create(params);
		Response resp = rwResponse.parse();

		// for background mode
		if (isBackground && resp.status().isPresent()) {
			ResponseStatus status;
			do {
				try {
					Thread.sleep(500);
				} catch (InterruptedException e) {
				}
				status = resp.status().get();
			} while (status.equals(ResponseStatus.QUEUED) || status.equals(ResponseStatus.IN_PROGRESS));
		}
		return new OpenAIResponse(resp);
	}

	@Override
	public AIAnswer convertResponse(OpenAIResponse response, IProgressMonitor mon) {
		Response resp = ((OpenAIResponse) response).response;
		SubMonitor sub = SubMonitor.convert(mon, "Convert Respone", 1);

		AIAnswer res = new AIAnswer(resp.safetyIdentifier().orElse("none"));
		if (resp.usage().isPresent()) {
			ResponseUsage usage = resp.usage().get();
			res.stats.inputToken = usage.inputTokens();
			res.stats.outputToken = usage.outputTokens();
			res.stats.totalToken = usage.totalTokens();
			if (usage.outputTokensDetails() != null)
				res.stats.reasoningToken = usage.outputTokensDetails().reasoningTokens();
		}

		if (resp.instructions().isPresent()) {
			Instructions ins = resp.instructions().get();
			res.instructions = ins.asString();
		}

		if (resp.incompleteDetails().isPresent()) {
			var incomplete = resp.incompleteDetails().get();
			LOG.error("Incomplete: " + incomplete.reason().get().toString() + " ");
		}
		if (resp.error().isPresent()) {
			ResponseError error = resp.error().get();
			res.answer = error.code() + ": " + error.message();
			LOG.error("Error: " + error.code() + " " + error.message());

		} else {
			SessionAnswerBuilder answer = new SessionAnswerBuilder();
			for (var out : resp.output()) {
				if (out.isMessage()) {
					var msg = out.message().get();
					StringBuilder text = new StringBuilder();
					for (var cnt : msg.content()) {
						if (cnt.isOutputText())
							text.append(cnt.asOutputText().text());
						else if (cnt.isRefusal())
							LOG.error("Refusal: " + cnt.asRefusal().refusal());
					}
					answer.text(text.toString());
				} else if (out.isFunctionCall()) {
					var fc = out.functionCall().get();
					JsonNode args;
					try {
						args = mapper.readTree(fc.arguments());
					} catch (Exception e) {
						args = mapper.createObjectNode();
					}
					answer.toolCall(fc.callId(), fc.name(), args);
				} else if (out.isReasoning()) {
					for (var cnt : out.asReasoning().summary())
						answer.reasoning(cnt.text());
					if (out.asReasoning().content().isPresent())
						for (var cnt : out.asReasoning().content().get())
							answer.reasoning(cnt.text());
				} else {
					LOG.info("Other output!");
				}
			}
			res.answer = answer.toString();
		}
		sub.worked(1);
		return res;
	}

	private Builder appendTools(Builder builder, List<String> tools) {
		for (String toolName : tools) {
			JsonNode tool = mcpClient.findTool(toolName);
			if (tool == null)
				continue;
			JsonNode schema = tool.path("inputSchema");
			JsonNode propsNode = schema.path("properties");
			Object properties = propsNode.isObject() ? mapper.convertValue(propsNode, Object.class) : Map.of();
			List<String> required = new ArrayList<>();
			schema.path("required").forEach(r -> required.add(r.asText()));
			FunctionTool functionTool = FunctionTool.builder()//
					.name(tool.path("name").asText(toolName))//
					.description(tool.path("description").asText(""))//
					.parameters(FunctionTool.Parameters.builder()//
							.putAdditionalProperty("type", JsonValue.from("object"))//
							.putAdditionalProperty("properties", JsonValue.from(properties))//
							.putAdditionalProperty("required", JsonValue.from(required))//
							.build())//
					.strict(false)//
					.build();
			builder.addTool(functionTool);
		}
		return builder;
	}
}
