package xy.ai.workbench.connectors.google;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;

import com.google.genai.Client;
import com.google.genai.types.Content;
import com.google.genai.types.GenerateContentConfig;
import com.google.genai.types.GenerateContentConfig.Builder;
import com.google.genai.types.GenerateContentResponse;
import com.google.genai.types.GenerateContentResponseUsageMetadata;
import com.google.genai.types.HarmBlockThreshold;
import com.google.genai.types.HarmCategory;
import com.google.genai.types.ModalityTokenCount;
import com.google.genai.types.Part;
import com.google.genai.types.SafetySetting;
import com.google.genai.types.ThinkingConfig;

import autovalue.shaded.com.google.common.collect.ImmutableList;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.models.AIAnswer;

public class GeminiConnector implements IAIConnector<GeminiRequest, GeminiResponse> {
	private ConfigManager cfg;
	private Client client;

	public GeminiConnector(ConfigManager cfg) {
		this.cfg = cfg;
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
		ImmutableList<SafetySetting> safetySettings = ImmutableList.of(//
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
				proccessedInputs.add(systemInstruction);
		}

		if (inputs != null && !inputs.isEmpty())
			for (String input : inputs)
				if (input != null && !input.isBlank())
					proccessedInputs.add(Content.fromParts(Part.fromText(input)));

		if (tools != null && !tools.isEmpty())
			for (String tool : tools)
				proccessedInputs.add(Content.fromParts(Part.fromText(tool)));

		GenerateContentConfig contentConfig = config.build();
		sub.worked(1);
		return new GeminiRequest(cfg.getModel(), proccessedInputs, contentConfig, id + "");
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
	public GeminiResponse executeRequest(GeminiRequest req, IProgressMonitor mon) {
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

		if (cresp.usageMetadata().isPresent()) {
			GenerateContentResponseUsageMetadata usage = cresp.usageMetadata().get();

			res.inputToken = usage.promptTokenCount().orElse(-1).intValue();
			res.reasoningToken = usage.thoughtsTokenCount().orElse(-1).intValue();
			res.totalToken = usage.totalTokenCount().orElse(-1).intValue();
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
