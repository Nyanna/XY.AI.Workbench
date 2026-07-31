package xy.ai.workbench.connector;

import java.util.Collection;
import java.util.List;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.jobs.Job;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.batch.NewBatch;
import xy.ai.workbench.connector.claude.ClaudeBatch;
import xy.ai.workbench.connector.claude.ClaudeBatchConnector;
import xy.ai.workbench.connector.claude.ClaudeConnector;
import xy.ai.workbench.connector.claude.ClaudeRequest;
import xy.ai.workbench.connector.claude.ClaudeResponse;
import xy.ai.workbench.connector.claudecode.CCConnector;
import xy.ai.workbench.connector.claudecode.CCRequest;
import xy.ai.workbench.connector.claudecode.CCResponse;
import xy.ai.workbench.connector.claudecode.CCSessionManager;
import xy.ai.workbench.connector.google.GeminiBatch;
import xy.ai.workbench.connector.google.GeminiBatchConnector;
import xy.ai.workbench.connector.google.GeminiConnector;
import xy.ai.workbench.connector.google.GeminiRequest;
import xy.ai.workbench.connector.google.GeminiResponse;
import xy.ai.workbench.connector.openai.OpenAIBatch;
import xy.ai.workbench.connector.openai.OpenAIBatchConnector;
import xy.ai.workbench.connector.openai.OpenAIConnector;
import xy.ai.workbench.connector.openai.OpenAIRequest;
import xy.ai.workbench.connector.openai.OpenAIResponse;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class AdaptingConnector implements IAIConnector<IModelRequest, IModelResponse>, IAIBatchConnector {

	private ConfigManager cfg;
	private OpenAIConnector chad;
	private IAIBatchConnector batchChad;
	private GeminiConnector gemini;
	private IAIBatchConnector batchGemini;
	private ClaudeConnector claude;
	private IAIBatchConnector batchClaude;
	private CCConnector claudeCode;
	private IAIBatchConnector newBatch;

	public AdaptingConnector(ConfigManager cfg, CCSessionManager sessionManager) {
		this.cfg = cfg;
		batchChad = new OpenAIBatchConnector(cfg, chad = new OpenAIConnector(cfg));
		batchGemini = new GeminiBatchConnector(cfg, gemini = new GeminiConnector(cfg));
		batchClaude = new ClaudeBatchConnector(cfg, claude = new ClaudeConnector(cfg));
		claudeCode = new CCConnector(cfg, sessionManager);
		newBatch = new NewBatchConnector();
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.None;
	}

	private IAIConnector<? extends IModelRequest, ? extends IModelResponse> getConnector(Model model) {
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
		case CC_HAIKU:
		case CC_SONNET:
		case CC_OPUS:
		case CC_MCPC_HAIKU:
		case CC_MCPC_SONNET:
		case CC_MCPC_OPUS:
			return claudeCode;
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

	@SuppressWarnings("rawtypes")
	public IAIConnector getConnector(IModelRequest request) {
		if (request instanceof GeminiRequest)
			return gemini;
		else if (request instanceof OpenAIRequest)
			return chad;
		else if (request instanceof ClaudeRequest)
			return claude;
		else if (request instanceof CCRequest)
			return claudeCode;
		throw new IllegalArgumentException("Model unsupported");
	}

	@SuppressWarnings("rawtypes")
	private IAIConnector getConnector(IModelResponse response) {
		if (response instanceof GeminiResponse)
			return gemini;
		else if (response instanceof OpenAIResponse)
			return chad;
		else if (response instanceof ClaudeResponse)
			return claude;
		else if (response instanceof CCResponse)
			return claudeCode;
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
	public IModelRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,
			IProgressMonitor mon) {
		return getConnector(cfg.getModel()).createRequest(inputs, systemPrompt, tools, batchFix, mon);
	}

	@Override
	public List<IAIBatch> updateBatches(IProgressMonitor mon) {
		return getBatchConnector(cfg.getModel()).updateBatches(mon);
	}

	@Override
	public IAIBatch submitBatch(NewBatch entry, IProgressMonitor mon) {
		return getBatchConnector(cfg.getModel()).submitBatch(entry, mon);
	}

	@SuppressWarnings("unchecked")
	@Override
	public IModelResponse executeRequest(IModelRequest request, IProgressMonitor mon, Job job) {
		return getConnector(request).executeRequest(request, mon, job);
	}

	@Override
	public String requestsToJson(Collection<IModelRequest> reqs) {
		return getBatchConnector(reqs.iterator().next()).requestsToJson(reqs);
	}

	@SuppressWarnings("unchecked")
	@Override
	public AIAnswer convertResponse(IModelResponse response, IProgressMonitor mon) {
		return getConnector(response).convertResponse(response, mon);
	}

	@Override
	public IAIBatch cancelBatch(IAIBatch entry, IProgressMonitor mon) {
		return getBatchConnector(entry).cancelBatch(entry, mon);
	}

	@Override
	public void loadBatch(IAIBatch entry, IProgressMonitor mon) {
		getBatchConnector(entry).loadBatch(entry, mon);
	}

	@Override
	public void convertAnswers(IAIBatch obj, IProgressMonitor mon) {
		getBatchConnector(obj).convertAnswers(obj, mon);
	}

}
