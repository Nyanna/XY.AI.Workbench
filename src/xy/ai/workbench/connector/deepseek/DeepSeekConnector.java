package xy.ai.workbench.connector.deepseek;

import java.net.http.HttpRequest;
import java.util.List;
import java.util.Random;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.DoubleNode;
import com.fasterxml.jackson.databind.node.LongNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.LOG;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connector.IAIConnector;
import xy.ai.workbench.connector.mcp.MCPClient;
import xy.ai.workbench.connector.openapi.deepseek.ResponsesClientImpl;
import xy.ai.workbench.connector.openapi.deepseek.components.AnyOfBodyModel;
import xy.ai.workbench.connector.openapi.deepseek.components.FunctionTool;
import xy.ai.workbench.connector.openapi.deepseek.components.FunctionToolCall;
import xy.ai.workbench.connector.openapi.deepseek.components.OneOfToolsElement;
import xy.ai.workbench.connector.openapi.deepseek.components.ToolsList;
import xy.ai.workbench.connector.openapi.deepseek.enums.BodyToolChoiceTypeEnum;
import xy.ai.workbench.connector.openapi.deepseek.operators.AnyOfParameters;
import xy.ai.workbench.connector.openapi.deepseek.components.InputElementContentList;
import xy.ai.workbench.connector.openapi.deepseek.components.ModelResponseProperties;
import xy.ai.workbench.connector.openapi.deepseek.components.OneOfContentElement;
import xy.ai.workbench.connector.openapi.deepseek.components.OutputMessage;
import xy.ai.workbench.connector.openapi.deepseek.components.ReasoningItem;
import xy.ai.workbench.connector.openapi.deepseek.components.SummaryList;
import xy.ai.workbench.connector.openapi.deepseek.components.SummaryTextContent;
import xy.ai.workbench.connector.openapi.deepseek.enums.TypeEnum;
import xy.ai.workbench.connector.openapi.deepseek.lists.EasyInputMessage;
import xy.ai.workbench.connector.openapi.deepseek.lists.OneOfContent;
import xy.ai.workbench.connector.openapi.deepseek.lists.RoleEnum;
import xy.ai.workbench.connector.openapi.deepseek.operators.AnyOfEffort;
import xy.ai.workbench.connector.openapi.deepseek.operators.AnyOfMaxOutputTokens;
import xy.ai.workbench.connector.openapi.deepseek.operators.AnyOfReasoning;
import xy.ai.workbench.connector.openapi.deepseek.operators.AnyOfTemperature;
import xy.ai.workbench.connector.openapi.deepseek.operators.EffortEnum;
import xy.ai.workbench.connector.openapi.deepseek.request.responses.post.json.AllOfBody;
import xy.ai.workbench.connector.openapi.deepseek.request.responses.post.json.CreateResponseAllOfPart;
import xy.ai.workbench.connector.openapi.deepseek.request.responses.post.json.OneOfInput;
import xy.ai.workbench.connector.openapi.deepseek.responses.Responses;
import xy.ai.workbench.connector.openapi.deepseek.responses.code200.json.AnyOfError;
import xy.ai.workbench.connector.openapi.deepseek.responses.code200.json.AnyOfInstructions;
import xy.ai.workbench.connector.openapi.deepseek.responses.code200.json.OneOfElement;
import xy.ai.workbench.connector.openapi.deepseek.responses.code200.json.OutputList;
import xy.ai.workbench.connector.openapi.deepseek.responses.code200.json.ResponseAllOfPart;
import xy.ai.workbench.connector.openapi.deepseek.responses.code200.json.ResponseErrorAnyOfPart;
import xy.ai.workbench.connector.openapi.deepseek.responses.code200.json.ResponseUsage;
import xy.ai.workbench.connector.openapi.deepseek.responses.code200.json.ResponsesCode200Json;
import xy.ai.workbench.connector.harness.Role;
import xy.ai.workbench.connector.harness.SessionAnswerBuilder;
import xy.ai.workbench.connector.harness.SessionCallbacks;
import xy.ai.workbench.connector.harness.SessionProcessor;
import xy.ai.workbench.connector.harness.ToolCall;
import xy.ai.workbench.connector.harness.ToolResult;
import xy.ai.workbench.connector.openapi.deepseek.components.FunctionCallEnum;
import xy.ai.workbench.connector.openapi.deepseek.enums.InputElementTypeEnum;
import xy.ai.workbench.connector.openapi.deepseek.lists.FunctionCallOutputItemParam;
import xy.ai.workbench.connector.openapi.deepseek.lists.OneOfOutput;
import xy.ai.workbench.models.AIAnswer;

/**
 * Connector for Deepseek's OpenAI-Responses-compatible API, based on the
 * generated SDK in {@code connector.openapi.deepseek}. Deepseek has no
 * safetyIdentifier field, so - analogous to {@code ClaudeConnector} - a
 * random id is generated per request and threaded through to the response
 * instead of relying on an echoed server-side identifier.
 */
public class DeepSeekConnector implements IAIConnector<DeepSeekRequest, DeepSeekResponse> {

	private static final String BASE_URL = "https://api.deepseek.com";

	private ConfigManager cfg;
	private ResponsesClientImpl client;
	private final MCPClient mcpClient;
	private final SessionProcessor sessionProcessor;

	public DeepSeekConnector(ConfigManager cfg, MCPClient mcpClient, SessionProcessor sessionProcessor) {
		this.cfg = cfg;
		this.mcpClient = mcpClient;
		this.sessionProcessor = sessionProcessor;
		cfg.addKeyObs(k -> {
			if (getSupportedKeyPattern().matches(k))
				this.client = new ResponsesClientImpl(BASE_URL) {
					@Override
					protected void customizeRequest(HttpRequest.Builder builder) {
						builder.header("Authorization", "Bearer " + k);
					}
				};
		}, true);
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.Deepseek;
	}

	@Override
	public DeepSeekRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools,
			boolean batchFix, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "BuildRequest", 1);

		ObjectMapper mapper = new ObjectMapper();
		ObjectNode root = mapper.createObjectNode();
		AllOfBody requestBody = new AllOfBody(root);

		requestBody.getResponseProperties().setModel(new AnyOfBodyModel(TextNode.valueOf(cfg.getModel().apiName)));

		CreateResponseAllOfPart part = requestBody.getCreateResponseAllOfPart();
		part.setMaxOutputTokens(new AnyOfMaxOutputTokens(LongNode.valueOf(cfg.getMaxOutputTokens())));

		if (systemPrompt != null && !systemPrompt.isBlank())
			part.setInstructions(new xy.ai.workbench.connector.openapi.deepseek.operators.AnyOfInstructions(
					TextNode.valueOf(systemPrompt)));

		ModelResponseProperties modelProps = requestBody.getCreateModelResponsePropertiesAllOf()
				.getModelResponseProperties();
		String reqId = Integer.toString(new Random().nextInt(Integer.MAX_VALUE));
		// don't set User ID, segmentation prevents caching
		//modelProps.setUser(userId);

		if (cfg.getCapabilities().isSupportTemperature())
			// ignored when thinking, 0-2
			modelProps.setTemperature(new AnyOfTemperature(DoubleNode.valueOf(cfg.getTemperature())));

		if (cfg.getCapabilities().isSupportTopP())
			// onl yused when thinking 0.95-1
			modelProps.setTopP(new AnyOfTemperature(DoubleNode.valueOf(cfg.getTopP())));

		EffortEnum effort = toEffort(cfg.getReasoning());
		if (effort != null) {
			ObjectNode reasoningNode = mapper.createObjectNode();
			new xy.ai.workbench.connector.openapi.deepseek.operators.Reasoning(reasoningNode)
					.setEffort(new AnyOfEffort(TextNode.valueOf(effort.rawValue())));
			part.setReasoning(new AnyOfReasoning(reasoningNode));
		}

		ArrayNode input = mapper.createArrayNode();
		if (inputs != null)
			for (ObjectNode item : sessionProcessor.process(inputs, new SessionRequestCallbacks(mapper)))
				input.add(item);

		if (tools != null && !tools.isEmpty())
			appendTools(mapper, requestBody, tools);

		if (input.size() > 0)
			part.setInput(new OneOfInput(input));

		sub.worked(1);
		return new DeepSeekRequest(requestBody, reqId);
	}

	private void appendTools(ObjectMapper mapper, AllOfBody requestBody, List<String> tools) {
		ToolsList toolsList = new ToolsList(mapper.createArrayNode());
		for (String toolName : tools) {
			JsonNode tool = mcpClient.findTool(toolName);
			if (tool == null)
				continue;
			ObjectNode toolNode = mapper.createObjectNode();
			FunctionTool ft = new FunctionTool(toolNode);
			ft.setType(BodyToolChoiceTypeEnum.FUNCTION);
			ft.setName(tool.path("name").asText(toolName));
			ft.setDescription(new xy.ai.workbench.connector.openapi.deepseek.operators.AnyOfInstructions(
					TextNode.valueOf(tool.path("description").asText(""))));
			ft.setParameters(new AnyOfParameters(tool.path("inputSchema")));
			toolsList.add(new OneOfToolsElement(toolNode));
		}
		if (toolsList.size() > 0)
			requestBody.getResponseProperties().setTools(toolsList);
	}

	private ObjectNode createMessage(ObjectMapper mapper, RoleEnum role, String text) {
		ObjectNode node = mapper.createObjectNode();
		EasyInputMessage msg = new EasyInputMessage(node);
		msg.setType(TypeEnum.MESSAGE);
		msg.setRole(role);
		msg.setContent(new OneOfContent(TextNode.valueOf(text)));
		return node;
	}
/** Turns {@link SessionProcessor} callbacks into DeepSeek Responses-API input items (full SDK-based implementation). */
	private final class SessionRequestCallbacks implements SessionCallbacks<ObjectNode> {
		private final ObjectMapper mapper;

		private SessionRequestCallbacks(ObjectMapper mapper) {
			this.mapper = mapper;
		}

		@Override
		public ObjectNode message(Role role, String text) {
			return createMessage(mapper, role == Role.Agent ? RoleEnum.ASSISTANT : RoleEnum.USER, text);
		}

		@Override
		public ObjectNode toolCall(ToolCall call) {
			ObjectNode node = mapper.createObjectNode();
			FunctionToolCall ftc = new FunctionToolCall(node);
			ftc.setType(FunctionCallEnum.FUNCTION_CALL);
			ftc.setId(call.id);
			ftc.setCallId(call.id);
			ftc.setName(call.name);
			ftc.setArguments(call.arguments == null ? "{}" : call.arguments.toString());
			return node;
		}

		@Override
		public ObjectNode toolResult(ToolResult result) {
			ObjectNode node = mapper.createObjectNode();
			FunctionCallOutputItemParam out = new FunctionCallOutputItemParam(node);
			out.setType(InputElementTypeEnum.FUNCTION_CALL_OUTPUT);
			out.setCallId(new xy.ai.workbench.connector.openapi.deepseek.operators.AnyOfInstructions(
					TextNode.valueOf(result.id == null ? "" : result.id)));
			out.setOutput(new OneOfOutput(TextNode.valueOf(result.content == null ? "" : result.content)));
			return node;
		}
	}

	

	private EffortEnum toEffort(Reasoning reasoning) {
		if (reasoning == null)
			return null;
		switch (reasoning) {
		case Disabled:
			return EffortEnum.NONE;
		case low:
			return EffortEnum.LOW;
		case high:
			return EffortEnum.HIGH;
		case max:
			return EffortEnum.MAX;
		default:
			throw new UnsupportedOperationException("Unsupported reasoning");
		}
	}

	@Override
	public DeepSeekResponse executeRequest(DeepSeekRequest req, IProgressMonitor mon, Job job) {
		Responses resp = client.createResponse(req.request);
		return new DeepSeekResponse(resp, req.getID());
	}

	@Override
	public AIAnswer convertResponse(DeepSeekResponse response, IProgressMonitor mon) {
		Responses resp = response.response;
		SubMonitor sub = SubMonitor.convert(mon, "Convert Respone", 1);

		AIAnswer res = new AIAnswer(response.id);

		if (!"200".equals(resp.statusCode())) {
			LOG.error("Deepseek request failed: HTTP " + resp.statusCode());
			res.answer = "HTTP " + resp.statusCode();
			sub.worked(1);
			return res;
		}

		ResponsesCode200Json code200 = resp.getCode200();
		if (code200 == null || !code200.isJson()) {
			LOG.error("Unexpected Deepseek response content type: " + (code200 == null ? "none" : code200.contentType()));
			sub.worked(1);
			return res;
		}

		ResponseAllOfPart part = code200.getJson().getResponseAllOfPart();

		ResponseUsage usage = part.getUsage();
		if (usage != null) {
			if (usage.getInputTokens() != null)
				res.stats.inputToken = usage.getInputTokens();
			if (usage.getOutputTokens() != null)
				res.stats.outputToken = usage.getOutputTokens();
			if (usage.getTotalTokens() != null)
				res.stats.totalToken = usage.getTotalTokens();
		}

		AnyOfInstructions instructions = part.getInstructions();
		if (instructions != null && !instructions.isNull() && instructions.getOneOfInstructions() != null
				&& instructions.getOneOfInstructions().isString())
			res.instructions = instructions.getOneOfInstructions().getString();

		AnyOfError error = part.getError();
		if (error != null && !error.isNull() && error.isResponseErrorAnyOfPart()) {
			ResponseErrorAnyOfPart err = error.getResponseErrorAnyOfPart();
			res.answer = err.getCode().rawValue() + ": " + err.getMessage();
			LOG.error("Error: " + err.getCode().rawValue() + " " + err.getMessage());
		} else {
			SessionAnswerBuilder answer = new SessionAnswerBuilder();
			OutputList output = part.getOutput();
			if (output != null)
				for (int i = 0; i < output.size(); i++) {
					OneOfElement el = output.get(i);
					if (el.isOutputMessage()) {
						OutputMessage msg = el.getOutputMessage();
						InputElementContentList content = msg.getContent();
						StringBuilder text = new StringBuilder();
						for (int j = 0; content != null && j < content.size(); j++) {
							OneOfContentElement c = content.get(j);
							if (c.isOutputTextContent())
								text.append(c.getOutputTextContent().getText());
							else if (c.isRefusalContent())
								LOG.error("Refusal: " + c.getRefusalContent().getRefusal());
						}
						answer.text(text.toString());
					} else if (el.isFunctionToolCall()) {
						FunctionToolCall call = el.getFunctionToolCall();
						JsonNode args;
						try {
							args = new ObjectMapper().readTree(call.getArguments());
						} catch (Exception e) {
							args = new ObjectMapper().createObjectNode();
						}
						answer.toolCall(call.getCallId(), call.getName(), args);
					} else if (el.isReasoningItem()) {
						// Reasoning is emitted directly as text (round-trippable via the THINKING
						// marker); the API itself ignores replayed reasoning_content on the next turn.
						ReasoningItem reasoning = el.getReasoningItem();
						SummaryList summary = reasoning.getSummary();
						for (int j = 0; summary != null && j < summary.size(); j++) {
							SummaryTextContent txt = summary.get(j);
							answer.reasoning(txt.getText());
						}
					} else {
						LOG.info("Other output!");
					}
				}
			res.answer = answer.toString();
		}
		sub.worked(1);
		return res;
	}
}
