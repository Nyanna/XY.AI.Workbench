package xy.ai.conscious.connector.claude;

import java.net.http.HttpRequest;
import java.util.List;
import java.util.Random;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.TextNode;
import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.connector.IAIConnector;
import xy.ai.conscious.connector.harness.FrozenConfig;
import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.connector.harness.SessionAnswerBuilder;
import xy.ai.conscious.connector.harness.SessionCallbacks;
import xy.ai.conscious.connector.harness.SessionProcessor;
import xy.ai.conscious.connector.mcp.MCPClient;
import xy.ai.conscious.connector.openapi.anthropic.V1ClientImpl;
import xy.ai.conscious.connector.openapi.anthropic.components.AnyOfModel;
import xy.ai.conscious.connector.openapi.anthropic.components.AnyOfProperties;
import xy.ai.conscious.connector.openapi.anthropic.components.AnyOfRequired;
import xy.ai.conscious.connector.openapi.anthropic.components.ElementParamsThinkingTypeEnum;
import xy.ai.conscious.connector.openapi.anthropic.components.InputSchema;
import xy.ai.conscious.connector.openapi.anthropic.components.InputSchemaTypeEnum;
import xy.ai.conscious.connector.openapi.anthropic.components.OneOfElement;
import xy.ai.conscious.connector.openapi.anthropic.components.RequestTextBlock;
import xy.ai.conscious.connector.openapi.anthropic.components.SystemList;
import xy.ai.conscious.connector.openapi.anthropic.components.ThinkingConfigEnabled;
import xy.ai.conscious.connector.openapi.anthropic.components.Tool;
import xy.ai.conscious.connector.openapi.anthropic.enums.ElementTypeEnum;
import xy.ai.conscious.connector.openapi.anthropic.lists.RequiredList;
import xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation;
import xy.ai.conscious.connector.openapi.anthropic.components.AnyOfSystem;
import xy.ai.conscious.connector.openapi.anthropic.components.AnyOfOutputTokensDetails;
import xy.ai.conscious.connector.openapi.anthropic.components.Usage;
import xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheCreationInputTokens;
import xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams;
import xy.ai.conscious.connector.openapi.anthropic.components.Message;
import xy.ai.conscious.connector.openapi.anthropic.components.MessageContentList;
import xy.ai.conscious.connector.openapi.anthropic.components.MessagesList;
import xy.ai.conscious.connector.openapi.anthropic.components.Metadata;
import xy.ai.conscious.connector.openapi.anthropic.components.OneOfMessageContentElement;
import xy.ai.conscious.connector.openapi.anthropic.components.OneOfThinking;
import xy.ai.conscious.connector.openapi.anthropic.components.ResponseToolUseBlock;
import xy.ai.conscious.connector.openapi.anthropic.components.ToolsList;
import xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.V1MessagesResponse;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.Reasoning;
import xy.ai.conscious.model.Model.KeyPattern;

public class ClaudeConnector implements IAIConnector<ClaudeRequest, ClaudeResponse> {
  private static final String API_URL = "https://api.anthropic.com";
  private static final String API_VERSION = "2023-06-01";
  private final MCPClient mcpClient;
  private final ObjectMapper mapper = new ObjectMapper();
  private final SessionProcessor<Void> sessionProcessor;

  public ClaudeConnector(MCPClient mcpClient, SessionProcessor<Void> sessionProcessor) {
    this.mcpClient = mcpClient;
    this.sessionProcessor = sessionProcessor;
  }

  @Override
  public KeyPattern getSupportedKeyPattern() {
    return KeyPattern.Claude;
  }

  @Override
  public ClaudeRequest createRequest(Prompt prompt, IProgress mon) {
    IProgress sub = mon.setTask("BuildRequest", 1);
    FrozenConfig fc = prompt.config;
    CreateMessageParams params = new CreateMessageParams();
    Metadata metadata = new Metadata();
    metadata.setUserId(new AnyOfExplanation(TextNode.valueOf(new Random().nextInt(Integer.MAX_VALUE) + "")));
    params.setMetadata(metadata);
    if (Reasoning.Disabled.equals(fc.reasoning)) {
      params.setTemperature(fc.temperature);
      params.setTopP(fc.topP);
    } else {
      ThinkingConfigEnabled thinking = new ThinkingConfigEnabled();
      thinking.setType(ElementParamsThinkingTypeEnum.ENABLED);
      thinking.setBudgetTokens(fc.reasoningBudget.longValue());
      params.setThinking(new OneOfThinking(thinking.node()));
    }
    params.setModel(new AnyOfModel(TextNode.valueOf(fc.model.apiName)));
    params.setMaxTokens(fc.maxOutputTokens);
    SystemList system = new SystemList();
    if (fc.systemPrompt != null && !fc.systemPrompt.isBlank())
      system.add(textBlock(fc.systemPrompt));
    if (prompt.inputs != null && !prompt.inputs.isEmpty()) {
      // Session text (markers/includes) is parsed deterministically; unmarked plain
      // text still ends up as one system message per input, same as before.
      SessionCallbacks<Void> cb = (role, text) -> {
        system.add(textBlock(text));
        return null;
      };
      sessionProcessor.process(prompt.inputs, prompt.arg.processorEnabled, cb);
    }
    if (system.size() > 0)
      params.setSystem(new AnyOfSystem(system.node()));
    params.setMessages(new MessagesList());
    if (fc.tools != null && !fc.tools.isEmpty())
      appendTools(params, fc.tools);
    sub.worked();
    return new ClaudeRequest(params);
  }

  private RequestTextBlock textBlock(String text) {
    RequestTextBlock block = new RequestTextBlock();
    block.setType(ElementTypeEnum.TEXT);
    block.setText(text);
    return block;
  }

  private void appendTools(CreateMessageParams params, List<String> toolNames) {
    ToolsList tools = new ToolsList();
    for (String toolName : toolNames) {
      JsonNode mcpTool = mcpClient.findTool(toolName);
      if (mcpTool == null)
        continue;
      JsonNode schema = mcpTool.path("inputSchema");
      JsonNode props = schema.path("properties");
      InputSchema inputSchema = new InputSchema();
      inputSchema.setType(InputSchemaTypeEnum.OBJECT);
      if (props.isObject())
        inputSchema.setProperties(new AnyOfProperties(props));
      if (schema.path("required").isArray()) {
        RequiredList required = new RequiredList();
        schema.path("required").forEach(r -> required.add(r.asText()));
        inputSchema.setRequired(new AnyOfRequired(required.node()));
      }
      Tool tool = new Tool();
      tool.setName(mcpTool.path("name").asText(toolName));
      tool.setDescription(mcpTool.path("title").asText("") + "\n" + mcpTool.path("description").asText(""));
      tool.setInputSchema(inputSchema);
      tools.add(new OneOfElement(tool.node()));
    }
    if (tools.size() > 0)
      params.setTools(tools);
  }

  static V1ClientImpl createClient(String apiKey) {
    return new V1ClientImpl(API_URL) {

      @Override
      protected void customizeRequest(HttpRequest.Builder builder) {
        builder.header("x-api-key", apiKey);
        builder.header("anthropic-version", API_VERSION);
      }
    };
  }

  @Override
  public ClaudeResponse executeRequest(ClaudeRequest req, IProgress mon) {
    V1MessagesResponse httpResp = createClient(req.getPrompt().config.keys).messages_post(req.params);
    if (httpResp.getCode200() == null)
      throw new IllegalStateException("Anthropic request failed (" + httpResp.statusCode() + "): " + httpResp.body());
    return new ClaudeResponse(httpResp.getCode200().getJson(), req.getID());
  }

  private static long tokenValue(AnyOfCacheCreationInputTokens value) {
    return value != null && value.isInteger() && value.getInteger() != null ? value.getInteger() : 0;
  }

  @Override
  public AIAnswer convertResponse(ClaudeResponse resp, IProgress mon) {
    Message msg = resp.response;
    IProgress sub = mon.setTask("Convert Respone", 1);
    AIAnswer res = new AIAnswer(resp.id);
    Usage usage = msg.getUsage();
    res.stats.inputToken = usage.getInputTokens();
    res.stats.outputToken = usage.getOutputTokens();
    res.stats.cacheRead = tokenValue(usage.getCacheReadInputTokens());
    res.stats.cacheCreate = tokenValue(usage.getCacheCreationInputTokens());
    AnyOfOutputTokensDetails details = usage.getOutputTokensDetails();
    if (details != null && details.isOutputTokensDetails() && details.getOutputTokensDetails().getThinkingTokens() != null)
      res.stats.reasoningToken = details.getOutputTokensDetails().getThinkingTokens();
    res.stats.totalToken = res.stats.inputToken + res.stats.cacheCreate + res.stats.outputToken;
    SessionAnswerBuilder answer = new SessionAnswerBuilder();
    MessageContentList contents = msg.getContent();
    for (int i = 0; i < contents.size(); i++) {
      OneOfMessageContentElement content = contents.get(i);
      if (content.isResponseToolUseBlock()) {
        ResponseToolUseBlock toolUse = content.getResponseToolUseBlock();
        JsonNode args = toolUse.getInput();
        answer.toolCall(toolUse.getId(), toolUse.getName(), args != null && !args.isNull() ? args : mapper.createObjectNode());
      } else if (content.isResponseTextBlock())
        answer.text(content.getResponseTextBlock().getText());
    }
    res.answer = answer.toString();
    sub.worked();
    return res;
  }
}
