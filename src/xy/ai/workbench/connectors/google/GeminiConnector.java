package xy.ai.workbench.connectors.google;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;

import autovalue.shaded.com.google.common.collect.ImmutableList;

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

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class GeminiConnector implements IAIConnector {
	private ConfigManager cfg;
	private Client client;

	public GeminiConnector(ConfigManager cfg) {
		this.cfg = cfg;
		cfg.addKeyObs(k -> {
			if (KeyPattern.Gemini.matches(k))
				this.client = Client.builder()//
						.apiKey(k)//
						.build();
		}, true);
	}

	@Override
	public IModelRequest createRequest(String input, String systemPrompt, List<String> tools, boolean batchFix) {

		ImmutableList<SafetySetting> safetySettings = ImmutableList.of(//
				SafetySetting.builder()//
						.category(HarmCategory.Known.HARM_CATEGORY_HATE_SPEECH)//
						.threshold(HarmBlockThreshold.Known.BLOCK_NONE).build(),
				SafetySetting.builder()//
						.category(HarmCategory.Known.HARM_CATEGORY_DANGEROUS_CONTENT)//
						.threshold(HarmBlockThreshold.Known.BLOCK_NONE).build());

		Map<String, String> labels = new HashMap<>();
		labels.put(GeminiModelRequest.CUSTOM_ID, new Random().nextInt(Integer.MAX_VALUE) + "");

		Builder config = GenerateContentConfig.builder()
				// Sets the thinking budget to 0 to disable thinking mode
				.thinkingConfig(ThinkingConfig.builder()//
						.thinkingBudget(getThinkingBudget(cfg.getReasoning(), cfg)))//
				.candidateCount(1) //
				.temperature(cfg.getTemperature().floatValue())//
				.topP(cfg.getTopP().floatValue()) //
				// .labels(labels) // not supported
				.maxOutputTokens(cfg.getMaxOutputTokens().intValue());
		if (!batchFix)
			config = config.safetySettings(safetySettings);

		List<Content> inputs = new ArrayList<>();
		if (systemPrompt != null && !systemPrompt.isBlank()) {
			Content systemInstruction = Content.fromParts(Part.fromText(systemPrompt));
			if (!batchFix)
				config = config.systemInstruction(systemInstruction);
			else
				inputs.add(systemInstruction);
		}

		if (input != null && !input.isBlank())
			inputs.add(Content.fromParts(Part.fromText(input)));

		if (tools != null && !tools.isEmpty())
			for (String tool : tools)
				inputs.add(Content.fromParts(Part.fromText(tool)));

		return new GeminiModelRequest(cfg.getModel(), inputs, config.build());
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
	public IModelResponse executeRequest(IModelRequest request) {
		GeminiModelRequest params = ((GeminiModelRequest) request);

		GenerateContentResponse res = client.models.generateContent( //
				params.model.apiName, //
				params.prompt, //
				params.config);

		return new GeminiModelResponse(res);
	}

	@Override
	public AIAnswer convertResponse(IModelResponse response) {
		GenerateContentResponse resp = ((GeminiModelResponse) response).response;

		AIAnswer res = new AIAnswer();
		res.answer = resp.text();

		if (resp.usageMetadata().isPresent()) {
			GenerateContentResponseUsageMetadata usage = resp.usageMetadata().get();

			res.inputToken = usage.promptTokenCount().orElse(-1).intValue();
			res.reasoningToken = usage.thoughtsTokenCount().orElse(-1).intValue();
			res.totalToken = usage.totalTokenCount().orElse(-1).intValue();
			if (usage.promptTokensDetails().isPresent()) {
				List<ModalityTokenCount> details = usage.promptTokensDetails().get();
				details.isEmpty();
				// TODO parse modaility tokens get().getFirst().tokenCount()
			}
		}

//
//		if (resp.instructions().isPresent()) {
//			Instructions ins = resp.instructions().get();
//			res.instructions = ins.asString();
//		}
//		if (resp.error().isPresent()) {
//			ResponseError error = resp.error().get();
//			res.answer = error.code() + ": " + error.message();
//
		// TODO detailed reasoning output
		return res;
	}
}
