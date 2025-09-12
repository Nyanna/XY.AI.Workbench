package xy.ai.workbench.connectors.claude;

import java.util.List;
import java.util.Random;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;

import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.models.messages.ContentBlock;
import com.anthropic.models.messages.Message;
import com.anthropic.models.messages.MessageCreateParams;
import com.anthropic.models.messages.MessageCreateParams.Builder;
import com.anthropic.models.messages.Metadata;
import com.anthropic.models.messages.TextBlock;
import com.anthropic.models.messages.ThinkingConfigEnabled;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class ClaudeConnector implements IAIConnector {
	private ConfigManager cfg;
	private AnthropicClient client;

	public ClaudeConnector(ConfigManager cfg) {
		this.cfg = cfg;
		cfg.addKeyObs(k -> {
			if (KeyPattern.Claude.matches(k))
				this.client = AnthropicOkHttpClient.builder().apiKey(k).build();
		}, true);
	}

	@Override
	public IModelRequest createRequest(String input, String systemPrompt, List<String> tools, boolean batchFix, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "BuildRequest", 1);

		Builder builder = MessageCreateParams.builder();

		builder.metadata(Metadata.builder().userId(new Random().nextInt(Integer.MAX_VALUE) + "").build());

		if (Reasoning.Disabled.equals(cfg.getReasoning())) {
			builder.temperature(cfg.getTemperature());
			builder.topP(cfg.getTopP());
		} else
			builder.thinking(ThinkingConfigEnabled.builder()//
					.budgetTokens(cfg.getReasoningBudget()).build());

		builder.model(cfg.getModel().apiName);
		builder.maxTokens(cfg.getMaxOutputTokens());

		if (systemPrompt != null && !systemPrompt.isBlank())
			builder.system(systemPrompt);

		if (input != null && !input.isBlank())
			builder.addUserMessage(input);

		if (tools != null && !tools.isEmpty())
			for (String tool : tools)
				builder.addUserMessage(tool);

		MessageCreateParams createParams = builder.build();
		sub.done();
		return new ClaudeRequest(createParams);
	}

	@Override
	public IModelResponse executeRequest(IModelRequest request, IProgressMonitor mon) {
		ClaudeRequest params = ((ClaudeRequest) request);

		Message message = client.messages().create(params.params);
		return new ClaudeResponse(message);
	}

	@Override
	public AIAnswer convertResponse(IModelResponse response, IProgressMonitor mon) {
		Message resp = ((ClaudeResponse) response).response;
		SubMonitor sub = SubMonitor.convert(mon, "Convert Respone", 1);

		AIAnswer res = new AIAnswer(resp.id());

		res.inputToken = resp.usage().inputTokens();
		res.outputToken = resp.usage().outputTokens();
//		resp.usage().cacheCreationInputTokens();
//		resp.usage().cacheReadInputTokens();
		res.totalToken = res.inputToken + res.outputToken;

		StringBuffer answer = new StringBuffer();
		for (ContentBlock content : resp.content())
			if (content.isText())
				answer.append(content.asText().text());

		res.answer = answer.toString();
		sub.done();
		return res;
	}
}
