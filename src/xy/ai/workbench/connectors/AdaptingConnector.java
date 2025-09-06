package xy.ai.workbench.connectors;

import java.util.Collection;
import java.util.List;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model;
import xy.ai.workbench.batch.NewBatch;
import xy.ai.workbench.connectors.claude.ClaudeBatch;
import xy.ai.workbench.connectors.claude.ClaudeBatchConnector;
import xy.ai.workbench.connectors.claude.ClaudeConnector;
import xy.ai.workbench.connectors.claude.ClaudeRequest;
import xy.ai.workbench.connectors.claude.ClaudeResponse;
import xy.ai.workbench.connectors.google.GeminiBatch;
import xy.ai.workbench.connectors.google.GeminiBatchConnector;
import xy.ai.workbench.connectors.google.GeminiConnector;
import xy.ai.workbench.connectors.google.GeminiRequest;
import xy.ai.workbench.connectors.google.GeminiResponse;
import xy.ai.workbench.connectors.openai.OpenAIBatch;
import xy.ai.workbench.connectors.openai.OpenAIBatchConnector;
import xy.ai.workbench.connectors.openai.OpenAIConnector;
import xy.ai.workbench.connectors.openai.OpenAIRequest;
import xy.ai.workbench.connectors.openai.OpenAIResponse;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class AdaptingConnector implements IAIConnector, IAIBatchConnector {

	private ConfigManager cfg;
	private OpenAIConnector chad;
	private IAIBatchConnector batchChad;
	private GeminiConnector gemini;
	private IAIBatchConnector batchGemini;
	private ClaudeConnector claude;
	private IAIBatchConnector batchClaude;
	private IAIBatchConnector newBatch;

	public AdaptingConnector(ConfigManager cfg) {
		this.cfg = cfg;
		batchChad = new OpenAIBatchConnector(cfg, chad = new OpenAIConnector(cfg));
		batchGemini = new GeminiBatchConnector(cfg, gemini = new GeminiConnector(cfg));
		batchClaude = new ClaudeBatchConnector(cfg, claude = new ClaudeConnector(cfg));
		newBatch = new NewBatchConnector();
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
		case CLAUDE_OPUS:
		case CLAUDE_SONNET:
			return claude;
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
		case CLAUDE_OPUS:
		case CLAUDE_SONNET:
			return batchClaude;
		default:
		}
		throw new IllegalArgumentException("Model unsupported");
	}

	private IAIConnector getConnector(IModelRequest request) {
		if (request instanceof GeminiRequest)
			return gemini;
		else if (request instanceof OpenAIRequest)
			return chad;
		else if (request instanceof ClaudeRequest)
			return claude;
		throw new IllegalArgumentException("Model unsupported");
	}

	private IAIConnector getConnector(IModelResponse response) {
		if (response instanceof GeminiResponse)
			return gemini;
		else if (response instanceof OpenAIResponse)
			return chad;
		else if (response instanceof ClaudeResponse)
			return claude;
		throw new IllegalArgumentException("Model unsupported");
	}

	private IAIBatchConnector getBatchConnector(IModelRequest request) {
		if (request instanceof GeminiRequest)
			return batchGemini;
		else if (request instanceof OpenAIRequest)
			return batchChad;
		else if (request instanceof ClaudeRequest)
			return batchClaude;
		throw new IllegalArgumentException("Model unsupported");
	}

	private IAIBatchConnector getBatchConnector(IAIBatch entry) {
		if (entry instanceof GeminiBatch)
			return batchGemini;
		else if (entry instanceof OpenAIBatch)
			return batchChad;
		else if (entry instanceof ClaudeBatch)
			return batchClaude;
		else if (entry instanceof NewBatch)
			return newBatch;
		throw new IllegalArgumentException("Model unsupported");
	}

	@Override
	public IModelRequest createRequest(String input, String systemPrompt, List<String> tools, boolean batchFix) {
		return getConnector(cfg.getModel()).createRequest(input, systemPrompt, tools, batchFix);
	}

	@Override
	public List<IAIBatch> updateBatches() {
		return getBatchConnector(cfg.getModel()).updateBatches();
	}

	@Override
	public IAIBatch submitBatch(NewBatch entry) {
		return getBatchConnector(cfg.getModel()).submitBatch(entry);
	}

	@Override
	public IModelResponse executeRequest(IModelRequest request) {
		return getConnector(request).executeRequest(request);
	}

	@Override
	public String requestsToJson(Collection<IModelRequest> reqs) {
		return getBatchConnector(reqs.iterator().next()).requestsToJson(reqs);
	}

	@Override
	public AIAnswer convertResponse(IModelResponse response) {
		return getConnector(response).convertResponse(response);
	}

	@Override
	public IAIBatch cancelBatch(IAIBatch entry) {
		return getBatchConnector(entry).cancelBatch(entry);
	}

	@Override
	public void loadBatch(IAIBatch entry) {
		getBatchConnector(entry).loadBatch(entry);
	}

	@Override
	public void convertAnswers(IAIBatch obj) {
		getBatchConnector(obj).convertAnswers(obj);
	}

}
