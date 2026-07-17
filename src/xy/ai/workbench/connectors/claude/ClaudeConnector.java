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
import com.anthropic.models.messages.ThinkingConfigEnabled;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.models.AIAnswer;

public class ClaudeConnector implements IAIConnector<ClaudeRequest, ClaudeResponse> {
	private ConfigManager cfg;
	private AnthropicClient client;

	public ClaudeConnector(ConfigManager cfg) {
		this.cfg = cfg;
		cfg.addKeyObs(k -> {
			if (getSupportedKeyPattern().matches(k))
				this.client = AnthropicOkHttpClient.builder().apiKey(k).build();
		}, true);
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.Claude;
	}

	@Override
	public ClaudeRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,
			IProgressMonitor mon) {
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

		if (inputs != null && !inputs.isEmpty())
			for (String input : inputs)
				if (input != null && !input.isBlank())
					builder.addUserMessage(input);

		MessageCreateParams createParams = builder.build();
		sub.worked(1);
		return new ClaudeRequest(createParams);
	}

	@Override
	public ClaudeResponse executeRequest(ClaudeRequest req, IProgressMonitor mon) {
		Message message = client.messages().create(req.params);
		return new ClaudeResponse(message, req.getID());
	}

	@Override
	public AIAnswer convertResponse(ClaudeResponse resp, IProgressMonitor mon) {
		Message msg = resp.response;
		SubMonitor sub = SubMonitor.convert(mon, "Convert Respone", 1);

		AIAnswer res = new AIAnswer(resp.id);

		res.stats.inputToken = msg.usage().inputTokens();
		res.stats.outputToken = msg.usage().outputTokens();
//		resp.usage().cacheCreationInputTokens();
//		resp.usage().cacheReadInputTokens();
		res.stats.totalinToken = res.stats.inputToken + res.stats.outputToken;

		StringBuffer answer = new StringBuffer();
		for (ContentBlock content : msg.content())
			if (content.isText())
				answer.append(content.asText().text());

		res.answer = answer.toString();
		sub.worked(1);
		return res;
	}
}
