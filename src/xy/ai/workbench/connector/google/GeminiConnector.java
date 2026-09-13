package xy.ai.workbench.connector.google;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;


import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;

import com.google.genai.Client;
import com.google.genai.types.Content;
import com.google.genai.types.FunctionCall;
import com.google.genai.types.FunctionDeclaration;
import com.google.genai.types.GenerateContentConfig;
import com.google.genai.types.GenerateContentConfig.Builder;
import com.google.genai.types.Tool;
import com.google.genai.types.GenerateContentResponse;
import com.google.genai.types.GenerateContentResponseUsageMetadata;
import com.google.genai.types.HarmBlockThreshold;
import com.google.genai.types.HarmCategory;
import com.google.genai.types.ModalityTokenCount;
import com.google.genai.types.Part;
import com.google.genai.types.SafetySetting;
import com.google.genai.types.ThinkingConfig;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

import xy.ai.workbench.connector.IAIConnector;
import xy.ai.workbench.connector.mcp.MCPClient;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.models.AIAnswer;

public class GeminiConnector implements IAIConnector<GeminiRequest, GeminiResponse> {
	private ConfigManager cfg;
	private Client client;
	private final MCPClient mcpClient;
	private final ObjectMapper mapper = new ObjectMapper();

	public GeminiConnector(ConfigManager cfg, MCPClient mcpClient) {
		this.cfg = cfg;
		this.mcpClient = mcpClient;
		cfg.addKeyObs(k -> {
			if (getSupportedKeyPattern().matches(k))
				this.client = Client.builder()//
						.apiKey(k)//
						.build();
		}, true);
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.Gemini;
	}

	@Override
	public GeminiRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,
			IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "BuildRequest", 1);

		int id = new Random().nextInt(Integer.MAX_VALUE);
		List<SafetySetting> safetySettings = List.of(//
				SafetySetting.builder()//
						.category(HarmCategory.Known.HARM_CATEGORY_HATE_SPEECH)//
						.threshold(HarmBlockThreshold.Known.BLOCK_NONE).build(),
				SafetySetting.builder()//
						.category(HarmCategory.Known.HARM_CATEGORY_DANGEROUS_CONTENT)//
						.threshold(HarmBlockThreshold.Known.BLOCK_NONE).build());

		// not supported
		Map<String, String> labels = new HashMap<>();
		labels.put(GeminiRequest.CUSTOM_ID, id + "");

		Builder config = GenerateContentConfig.builder()//
				.seed(id)//
				.thinkingConfig(ThinkingConfig.builder()//
						.thinkingBudget(getThinkingBudget(cfg.getReasoning(), cfg)))//
				.candidateCount(1) //
				.temperature(cfg.getTemperature().floatValue())//
				.topP(cfg.getTopP().floatValue()) //
				// .labels(labels) // not supported
				.maxOutputTokens(cfg.getMaxOutputTokens().intValue());
		if (!batchFix)
			config.safetySettings(safetySettings);

		List<Content> proccessedInputs = new ArrayList<>();
		if (systemPrompt != null && !systemPrompt.isBlank()) {
			Content systemInstruction = Content.fromParts(Part.fromText(systemPrompt));
			if (!batchFix)
				config.systemInstruction(systemInstruction);
			else
				proccessedInputs.add(systemInstruction.toBuilder().role("model").build());
		}

		if (inputs != null && !inputs.isEmpty())
			for (String input : inputs)
				if (input != null && !input.isBlank())
					proccessedInputs.add(Content.builder().parts(Part.fromText(input)).role("model").build());

		if (tools != null && !tools.isEmpty())
			appendTools(config, tools);

		GenerateContentConfig contentConfig = config.build();
		sub.worked(1);
		return new GeminiRequest(cfg.getModel(), proccessedInputs, contentConfig, id + "");
	}

	private void appendTools(Builder config, List<String> tools) {
		List<FunctionDeclaration> declarations = new ArrayList<>();
		for (String toolName : tools) {
			JsonNode tool = mcpClient.findTool(toolName);
			if (tool == null)
				continue;
			JsonNode inputSchema = tool.path("inputSchema");
			Object schema = inputSchema.isObject() ? mapper.convertValue(inputSchema, Object.class) : Map.of();
			declarations.add(FunctionDeclaration.builder()//
					.name(tool.path("name").asText(toolName))//
					.description(tool.path("description").asText(""))//
					.parametersJsonSchema(schema)//
					.build());
		}
		if (!declarations.isEmpty())
			config.tools(List.of(Tool.builder().functionDeclarations(declarations).build()));
	}

	private Integer getThinkingBudget(Reasoning reasoning, ConfigManager cfg2) {
		switch (reasoning) {
		case Budget:
			return cfg.getReasoningBudget();
		case Unlimited:
			return -1;
		case Disabled:
			return 0;
		default:
		}
		throw new IllegalArgumentException("Unsupported reasoning setting");
	}

	@Override
	public GeminiResponse executeRequest(GeminiRequest req, IProgressMonitor mon, Job job) {
		GenerateContentResponse res = client.models.generateContent( //
				req.model.apiName, //
				req.prompt, //
				req.config);
		return new GeminiResponse(req.getID(), res);
	}

	@Override
	public AIAnswer convertResponse(GeminiResponse resp, IProgressMonitor mon) {
		GenerateContentResponse cresp = resp.response;
		SubMonitor sub = SubMonitor.convert(mon, "Convert Respone", 1);

		AIAnswer res = new AIAnswer(resp.id);
		res.answer = cresp.text();

		List<FunctionCall> calls = cresp.functionCalls();
		if (calls != null)
			for (FunctionCall call : calls) {
				JsonNode args = call.args().isPresent() ? mapper.valueToTree(call.args().get()) : mapper.createObjectNode();
				if (res.answer != null && !res.answer.isEmpty())
					res.answer += "\n\n";
				res.answer += mcpClient.renderToolCall(call.name().orElse("unknown"), args);
			}

		if (cresp.usageMetadata().isPresent()) {
			GenerateContentResponseUsageMetadata usage = cresp.usageMetadata().get();

			res.stats.inputToken = usage.promptTokenCount().orElse(-1).intValue();
			res.stats.reasoningToken = usage.thoughtsTokenCount().orElse(-1).intValue();
			res.stats.totalToken = usage.totalTokenCount().orElse(-1).intValue();
			if (usage.promptTokensDetails().isPresent()) {
				List<ModalityTokenCount> details = usage.promptTokensDetails().get();
				details.isEmpty();
			}
		}

		switch (cresp.finishReason().knownEnum()) {
		case STOP:
			break; // no error
		case BLOCKLIST:
		case FINISH_REASON_UNSPECIFIED:
		case IMAGE_SAFETY:
		case LANGUAGE:
		case MALFORMED_FUNCTION_CALL:
		case MAX_TOKENS:
		case OTHER:
		case PROHIBITED_CONTENT:
		case RECITATION:
		case SAFETY:
		case SPII:
		case UNEXPECTED_TOOL_CALL:
		default:
			res.answer += "Error: " + cresp.finishReason().knownEnum().name();
		}
		sub.worked(1);
		return res;
	}
}
