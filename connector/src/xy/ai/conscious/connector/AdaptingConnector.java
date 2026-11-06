package xy.ai.conscious.connector;

import java.util.Collection;
import java.util.List;

import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.api.ext.IMCPCLauncher;
import xy.ai.conscious.batch.NewBatch;
import xy.ai.conscious.connector.claude.ClaudeBatch;
import xy.ai.conscious.connector.claude.ClaudeBatchConnector;
import xy.ai.conscious.connector.claude.ClaudeConnector;
import xy.ai.conscious.connector.claude.ClaudeRequest;
import xy.ai.conscious.connector.claude.ClaudeResponse;
import xy.ai.conscious.connector.claudecode.CCConnector;
import xy.ai.conscious.connector.claudecode.CCRequest;
import xy.ai.conscious.connector.claudecode.CCResponse;
import xy.ai.conscious.connector.claudecode.CCSessionManager;
import xy.ai.conscious.connector.deepseek.DeepSeekConnector;
import xy.ai.conscious.connector.deepseek.DeepSeekRequest;
import xy.ai.conscious.connector.deepseek.DeepSeekResponse;
import xy.ai.conscious.connector.google.GeminiBatch;
import xy.ai.conscious.connector.google.GeminiBatchConnector;
import xy.ai.conscious.connector.google.GeminiConnector;
import xy.ai.conscious.connector.google.GeminiRequest;
import xy.ai.conscious.connector.google.GeminiResponse;
import xy.ai.conscious.connector.harness.IIncludeAdapter;
import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.connector.harness.SessionProcessor;
import xy.ai.conscious.connector.mcp.MCPClient;
import xy.ai.conscious.connector.mcp.MCPConnector;
import xy.ai.conscious.connector.mcp.MCPRequest;
import xy.ai.conscious.connector.mcp.MCPResponse;
import xy.ai.conscious.connector.openai.OpenAIBatch;
import xy.ai.conscious.connector.openai.OpenAIBatchConnector;
import xy.ai.conscious.connector.openai.OpenAIConnector;
import xy.ai.conscious.connector.openai.OpenAIRequest;
import xy.ai.conscious.connector.openai.OpenAIResponse;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.IModelRequest;
import xy.ai.conscious.model.IModelResponse;
import xy.ai.conscious.model.Model;
import xy.ai.conscious.model.Model.KeyPattern;

public class AdaptingConnector implements IAIConnector<IModelRequest, IModelResponse>, IAIBatchConnector {
  private ConfigManager cfg;
  private OpenAIConnector chad;
  private IAIBatchConnector batchChad;
  private GeminiConnector gemini;
  private IAIBatchConnector batchGemini;
  private ClaudeConnector claude;
  private IAIBatchConnector batchClaude;
  private CCConnector claudeCode;
  private DeepSeekConnector deepseek;
  private MCPConnector mcp;
  private IAIBatchConnector newBatch;
  @SuppressWarnings("rawtypes")
  private SessionProcessor sessionProcessor;

  @SuppressWarnings("unchecked")
  public AdaptingConnector(ConfigManager cfg, CCSessionManager sessionManager, MCPClient mcpClient, IIncludeAdapter includeAdapter, IMCPCLauncher mcpc) {
    this.cfg = cfg;
    sessionProcessor = new SessionProcessor<>(includeAdapter);
    batchChad = new OpenAIBatchConnector(cfg, chad = new OpenAIConnector(mcpClient, sessionProcessor));
    batchGemini = new GeminiBatchConnector(cfg, gemini = new GeminiConnector(mcpClient, sessionProcessor));
    batchClaude = new ClaudeBatchConnector(cfg, claude = new ClaudeConnector(mcpClient, sessionProcessor));
    deepseek = new DeepSeekConnector(mcpClient, sessionProcessor);
    claudeCode = new CCConnector(sessionManager, includeAdapter, mcpc);
    mcp = new MCPConnector(mcpClient);
    newBatch = new NewBatchConnector();
  }

  @Override
  public KeyPattern getSupportedKeyPattern() {
    return KeyPattern.None;
  }

  private IAIConnector<? extends IModelRequest, ? extends IModelResponse> getConnector(Model model) {
    switch(model.cap.getKeyPattern()) {
      case OpenAI:
        return chad;
      case Gemini:
        return gemini;
      case Claude:
        return claude;
      case Deepseek:
        return deepseek;
      case ClaudeCode:
        return claudeCode;
      case Misc:
        return mcp;
      default:
    }
    throw new IllegalArgumentException("Model unsupported: " + model.displayName);
  }

  public IAIBatchConnector getBatchConnector(Model model) {
    switch(model.cap.getKeyPattern()) {
      case OpenAI:
        return batchChad;
      case Gemini:
        return batchGemini;
      case Claude:
        return batchClaude;
      default:
    }
    throw new IllegalArgumentException("Model unsupported: " + model.displayName);
  }

  @SuppressWarnings("rawtypes")
  public IAIConnector getConnector(IModelRequest request) {
    if (request instanceof GeminiRequest)
      return gemini;
    else if (request instanceof OpenAIRequest)
      return chad;
    else if (request instanceof ClaudeRequest)
      return claude;
    else if (request instanceof DeepSeekRequest)
      return deepseek;
    else if (request instanceof CCRequest)
      return claudeCode;
    else if (request instanceof MCPRequest)
      return mcp;
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
    else if (response instanceof DeepSeekResponse)
      return deepseek;
    else if (response instanceof CCResponse)
      return claudeCode;
    else if (response instanceof MCPResponse)
      return mcp;
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
  public IModelRequest createRequest(Prompt prompt, IProgress mon) {
    return getConnector(prompt.config.model).createRequest(prompt, mon);
  }

  @Override
  public List<IAIBatch> updateBatches(IProgress mon) {
    return getBatchConnector(cfg.getModel()).updateBatches(mon);
  }

  @Override
  public IAIBatch submitBatch(NewBatch entry, IProgress mon) {
    return getBatchConnector(cfg.getModel()).submitBatch(entry, mon);
  }

  @SuppressWarnings("unchecked")
  @Override
  public IModelResponse executeRequest(IModelRequest request, IProgress mon) {
    return getConnector(request).executeRequest(request, mon);
  }

  @Override
  public String requestsToJson(Collection<IModelRequest> reqs) {
    return getBatchConnector(reqs.iterator().next()).requestsToJson(reqs);
  }

  @SuppressWarnings("unchecked")
  @Override
  public AIAnswer convertResponse(IModelResponse response, IProgress mon) {
    return getConnector(response).convertResponse(response, mon);
  }

  @Override
  public IAIBatch cancelBatch(IAIBatch entry, IProgress mon) {
    return getBatchConnector(entry).cancelBatch(entry, mon);
  }

  @Override
  public void loadBatch(IAIBatch entry, IProgress mon) {
    getBatchConnector(entry).loadBatch(entry, mon);
  }

  @Override
  public void convertAnswers(IAIBatch obj, IProgress mon) {
    getBatchConnector(obj).convertAnswers(obj, mon);
  }
}
