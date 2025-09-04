package xy.ai.workbench.connectors;

import java.util.Collection;
import java.util.List;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.SessionConfig.Model;
import xy.ai.workbench.connectors.google.GeminiBatch;
import xy.ai.workbench.connectors.google.GeminiBatchConnector;
import xy.ai.workbench.connectors.google.GeminiConnector;
import xy.ai.workbench.connectors.google.GeminiModelRequest;
import xy.ai.workbench.connectors.google.GeminiModelResponse;
import xy.ai.workbench.connectors.openai.OpenAIBatch;
import xy.ai.workbench.connectors.openai.OpenAIBatchConnector;
import xy.ai.workbench.connectors.openai.OpenAIConnector;
import xy.ai.workbench.connectors.openai.OpenAIModelRequest;
import xy.ai.workbench.connectors.openai.OpenAIModelResponse;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class AdaptingConnector implements IAIConnector, IAIBatchConnector {

	private ConfigManager cfg;
	private IAIConnector chad;
	private IAIBatchConnector batchChad;
	private IAIConnector gemini;
	private IAIBatchConnector batchGemini;

	public AdaptingConnector(ConfigManager cfg) {
		this.cfg = cfg;
		chad = new OpenAIConnector(cfg);
		batchChad = new OpenAIBatchConnector(cfg);
		gemini = new GeminiConnector(cfg);
		batchGemini = new GeminiBatchConnector(cfg);
	}

	private IAIConnector getConnector(Model model) {
		switch (model) {
		case GPT_5:
		case GPT_5_MINI:
		case GPT_5_NANO:
			return chad;
		case GEMINI_25_PRO:
		case GEMINI_25_FLASH:
		case GEMINI_25_LIGHT:
			return gemini;
		default:
		}
		throw new IllegalArgumentException("Model unsupported");
	}

	public IAIBatchConnector getBatchConnector(Model model) {
		switch (model) {
		case GPT_5:
		case GPT_5_MINI:
		case GPT_5_NANO:
			return batchChad;
		case GEMINI_25_PRO:
		case GEMINI_25_FLASH:
		case GEMINI_25_LIGHT:
			return batchGemini;
		default:
		}
		throw new IllegalArgumentException("Model unsupported");
	}

	private IAIConnector getConnector(IModelRequest request) {
		if (request instanceof GeminiModelRequest)
			return gemini;
		else if (request instanceof OpenAIModelRequest)
			return chad;
		throw new IllegalArgumentException("Model unsupported");
	}

	private IAIConnector getConnector(IModelResponse response) {
		if (response instanceof GeminiModelResponse)
			return gemini;
		else if (response instanceof OpenAIModelResponse)
			return chad;
		throw new IllegalArgumentException("Model unsupported");
	}

	private IAIBatchConnector getBatchConnector(IModelRequest request) {
		if (request instanceof GeminiModelRequest)
			return batchGemini;
		else if (request instanceof OpenAIModelRequest)
			return batchChad;
		throw new IllegalArgumentException("Model unsupported");
	}

	private IAIBatchConnector getBatchConnector(IAIBatch entry) {
		if (entry instanceof GeminiBatch)
			return batchGemini;
		else if (entry instanceof OpenAIBatch)
			return batchChad;
		throw new IllegalArgumentException("Model unsupported");
	}

	public IModelRequest createRequest(String input, String systemPrompt, List<String> tools) {
		return getConnector(cfg.getModel()).createRequest(input, systemPrompt, tools);
	}

	public List<IAIBatch> updateBatches() {
		return getBatchConnector(cfg.getModel()).updateBatches();
	}

	public IAIBatch submitBatch(String json, Collection<String> reqIds) {
		return getBatchConnector(cfg.getModel()).submitBatch(json, reqIds);
	}

	public IModelResponse executeRequest(IModelRequest request) {
		return getConnector(request).executeRequest(request);
	}

	public String requestsToJson(Collection<IModelRequest> reqs) {
		return getBatchConnector(reqs.iterator().next()).requestsToJson(reqs);
	}

	public AIAnswer convertResponse(IModelResponse response) {
		return getConnector(response).convertResponse(response);
	}

	public AIAnswer convertToAnswer(String bodyJson) {
		if (bodyJson.contains("GPT_5"))
			return chad.convertToAnswer(bodyJson);
		return gemini.convertToAnswer(bodyJson);
	}

	public IAIBatch cancelBatch(IAIBatch entry) {
		return getBatchConnector(entry).cancelBatch(entry);
	}

	public void loadBatch(IAIBatch entry) {
		getBatchConnector(entry).loadBatch(entry);
	}

}
