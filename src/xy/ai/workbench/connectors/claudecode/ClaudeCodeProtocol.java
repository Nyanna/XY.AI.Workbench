package xy.ai.workbench.connectors.claudecode;

import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Iterator;
import java.util.Map;
import java.util.Map.Entry;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.LOG;

/**
 * Parses JSON structures from Claude Code API responses. Handles extraction and
 * processing of results, tool uses, events, and rate limits.
 */
public class ClaudeCodeProtocol {
	public static final String THINKING = "Thinking:";
	public static final String TEXT = "Text:";
	public static final String TOOLUSE = "Tool:";
	private static final int TOOL_INPUT_MAX_LENGTH = 120;

	private final ResultPostProcessor postProcessor = new ResultPostProcessor();

	public void parseLine(ClaudeCodeResponse resp, ClaudeCodeSession session, SubMonitor sub, String line) {
		JsonNode node;
		try {
			node = JsonUtil.readTree(line);
		} catch (JsonProcessingException parseError) {
			throw new IllegalStateException(
					"Could not parse CLI line as JSON (length=" + line.length() + "): " + JsonUtil.abbreviate(line),
					parseError);
		}

		String type = JsonUtil.plainText(node.path("type"));
		try {
			if ("result".equals(type)) {
				sub.subTask("Received final result");
				parseResult(resp, node);
			} else if ("tool_use".equals(type)) {
				sub.subTask("Received tool use request");
				parseToolUse(resp, node);
			} else if ("system".equals(type)) {
				String subtype = JsonUtil.plainText(node.path("subtype"));
				if ("init".equals(subtype)) {
					sub.subTask("Received system init metadata");
					parseSystemInitEvent(resp, node);
					updateLastParsedMessage(resp, session);
				}
			} else if ("stream_event".equals(type)) {
				String eventType = JsonUtil.plainText(node.path("event").path("type"));
				if ("message_delta".equals(eventType)) {
					collectMessageDeltaEvent(resp, node);
					updateLastParsedMessage(resp, session);
				}
			} else if ("rate_limit_event".equals(type)) {
				parseRateLimitEvent(node);
			} else if ("assistant".equals(type)) {
				boolean recordToolUse = !AgentProfile.MCPC.equals(session.getParameters().agentProfile);
				parseAssistantEvents(node, resp, false, recordToolUse, sub.split(1));
				updateLastParsedMessage(resp, session);
			}
		} catch (Exception ex) {
			LOG.error("ClaudeCodeConnector: failed to process CLI event (type=" + type + ", length=" + line.length()
					+ "): " + JsonUtil.abbreviate(line), ex);
			throw ex;
		}
	}

	private void updateLastParsedMessage(ClaudeCodeResponse resp, ClaudeCodeSession session) {
		if (!resp.events.isEmpty()) {
			String last = null;
			for (String v : resp.events.values())
				last = v;
			session.setLastParsedMessage(last);
		}
	}

	private void parseResult(ClaudeCodeResponse resp, JsonNode node) {
		boolean isError = node.path("is_error").asBoolean(false) || "error".equals(node.path("subtype").asText());
		// plainText yields the logical result value without JSON quoting/escaping
		// (and handles a structured result node), instead of a bare asText().
		StringBuilder res = new StringBuilder(postProcessor.process(JsonUtil.plainText(node.path("result"))));
		if (!res.isEmpty() && res.charAt(res.length() - 1) != '\n')
			res.append("\n");

		// Some subtypes (e.g. "error_during_execution") carry no "result" field but
		// report the failure(s) in an "errors" array instead.
		String errorsText = joinErrors(node.path("errors"));
		if (!errorsText.isEmpty())
			res.append(errorsText.strip()).append("\n");

		appendEvents(resp.events, res);

		resp.resultText = res.toString();
		resp.isError = isError;

		// Extract token usage information
		JsonNode modelUsage = node.path("modelUsage");
		if (modelUsage.isObject()) {
			@SuppressWarnings("deprecation")
			Iterator<Entry<String, JsonNode>> fields = modelUsage.fields();
			fields.forEachRemaining(entry -> {
				JsonNode usage = entry.getValue();
				resp.inputTokens += usage.path("inputTokens").asLong(0);
				resp.outputTokens += usage.path("outputTokens").asLong(0);
				resp.cacheReadInputTokens += usage.path("cacheReadInputTokens").asLong(0);
				resp.cacheCreationInputTokens += usage.path("cacheCreationInputTokens").asLong(0);
			});
		}
	}

	public static void appendEvents(Map<String, String> events, StringBuilder resultText) {
		if (!events.isEmpty())
			for (String line : events.values())
				resultText.append(commented(line)).append("\n");
	}

	private void parseToolUse(ClaudeCodeResponse resp, JsonNode node) {
		String toolName = node.path("name").asText("");
		String toolUseId = node.path("id").asText("");
		JsonNode input = node.path("input");

		String inputStr;
		if (input.isObject() && input.size() == 1) {
			@SuppressWarnings("deprecation")
			String val = JsonUtil.plainText(input.fields().next().getValue());
			inputStr = "`" + val + "`";
		} else
			inputStr = JsonUtil.compact(input);

		resp.setToolUse(commented("Tool: " + toolName + "\nInput: " + inputStr + "\nID: " + toolUseId), toolUseId);
	}

	private void parseAssistantEvents(JsonNode node, ClaudeCodeResponse resp, boolean recordText, boolean recordToolUse,
			IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Received Claude message", 1);

		JsonNode content = node.path("message").path("content");
		if (content.isArray())
			for (JsonNode block : content) {
				String blockType = block.path("type").asText();
				if ("thinking".equals(blockType)) {
					// Some versions use "thinking" field, others fall back to "text"
					String text = block.path("thinking").asText("");
					if (text.isEmpty())
						text = block.path("text").asText("");
					if (!text.isEmpty())
						resp.events.putIfAbsent("thinking\0" + text, THINKING + "\n" + text);
					sub.subTask("Claude is thinking");
				} else if (recordText && "text".equals(blockType)) {
					String text = block.path("text").asText("");
					if (!text.isEmpty())
						resp.events.putIfAbsent("text\0" + text, TEXT + "\n" + text);
				} else if (recordToolUse && "tool_use".equals(blockType)) {
					String toolName = block.path("name").asText("");
					String text = " " + toolName + "\n";
					JsonNode inputs = block.path("input");
					if (inputs.isObject()) {
						var inputNames = inputs.fieldNames();
						while (inputNames.hasNext()) {
							String inputName = inputNames.next();
							// plainText avoids the double-escaping seen with toString():
							// a value like [\s\S] must stay [\s\S], not become [\\s\\S].
							String value = JsonUtil.plainText(inputs.path(inputName));
							if (value.length() > TOOL_INPUT_MAX_LENGTH)
								value = value.substring(0, TOOL_INPUT_MAX_LENGTH) + "…";
							text += inputName + ": " + value.replace('\n', ' ') + "\n";
						}
					}
					if (!text.isEmpty()) {
						resp.events.putIfAbsent("tool\0" + text, TOOLUSE + text);
						sub.subTask("Claude uses tool: " + toolName);
					}
				}
			}
		sub.worked(1);
	}

	private void collectMessageDeltaEvent(ClaudeCodeResponse resp, JsonNode node) {
		long thinkingTokens = 0;
		JsonNode usage = node.path("event").path("usage");
		if (usage.isObject()) {
			JsonNode outputTokensDetails = usage.path("output_tokens_details");
			if (outputTokensDetails.isObject()) {
				thinkingTokens = outputTokensDetails.path("thinking_tokens").asLong(0);
				if (thinkingTokens > 0) {
					String key = "reasoning\0" + thinkingTokens;
					String value = "ReasoningToken: " + thinkingTokens;
					resp.events.putIfAbsent(key, value);
				}
			}
		}
		resp.totalReasoningTokens += thinkingTokens;
	}

	private void parseRateLimitEvent(JsonNode node) {
		try {
			JsonNode rateLimitInfo = node.path("rate_limit_info");
			String rateLimitType = rateLimitInfo.path("rateLimitType").asText();

			// Extract fields (use 0 for missing values)
			String status = rateLimitInfo.path("status").asText("unknown");
			long resetsAt = rateLimitInfo.path("resetsAt").asLong(0);
			double utilization = rateLimitInfo.path("utilization").asDouble(0.0);
			String errorCode = rateLimitInfo.path("errorCode").asText("");
			boolean canUserPurchaseCredits = rateLimitInfo.path("canUserPurchaseCredits").asBoolean(false);
			boolean hasChargeableSavedPaymentMethod = rateLimitInfo.path("hasChargeableSavedPaymentMethod")
					.asBoolean(false);
			boolean isUsingOverage = rateLimitInfo.path("isUsingOverage").asBoolean(false);
			String overageStatus = rateLimitInfo.path("overageStatus").asText("");
			String overageDisabledReason = rateLimitInfo.path("overageDisabledReason").asText("");

			String resetsAtReadable = resetsAt > 0
					? new SimpleDateFormat("yyyy-MM-dd HH:mm:ss").format(new Date(resetsAt * 1000))
					: "unknown";

			StringBuilder logMsg = new StringBuilder();
			logMsg.append("Rate Limit Event [").append(rateLimitType).append("]: ");
			logMsg.append("status=").append(status);
			logMsg.append(" | utilization=").append(String.format("%.2f%%", utilization * 100));
			logMsg.append(" | resets_at=").append(resetsAtReadable);

			if (!errorCode.isEmpty())
				logMsg.append(" | errorCode=").append(errorCode);
			if (canUserPurchaseCredits)
				logMsg.append(" | canUserPurchaseCredits=").append(canUserPurchaseCredits);
			if (hasChargeableSavedPaymentMethod)
				logMsg.append(" | hasChargeableSavedPaymentMethod=").append(hasChargeableSavedPaymentMethod);
			if (isUsingOverage)
				logMsg.append(" | isUsingOverage=").append(isUsingOverage);
			if (!overageStatus.isEmpty())
				logMsg.append(" | overageStatus=").append(overageStatus);
			if (!overageDisabledReason.isEmpty())
				logMsg.append(" | overageDisabledReason=").append(overageDisabledReason);

			LOG.info(logMsg.toString());
		} catch (Exception e) {
			LOG.error("ClaudeCodeConnector: failed to process rate limit event", e);
			throw e;
		}
	}

	private void parseSystemInitEvent(ClaudeCodeResponse resp, JsonNode node) {
		Map<String, String> assistantEvents = resp.events;
		try {
			String cwd = node.path("cwd").asText("");
			String sessionId = node.path("session_id").asText("");
			String model = node.path("model").asText("");

			StringBuilder pluginNames = new StringBuilder();
			JsonNode plugins = node.path("plugins");
			if (plugins.isArray()) {
				boolean first = true;
				for (JsonNode plugin : plugins) {
					String name = plugin.path("name").asText("");
					if (!name.isEmpty()) {
						if (!first)
							pluginNames.append(", ");
						pluginNames.append(name);
						first = false;
					}
				}
			}

			String metadata = "SystemInit: cwd=" + cwd + " | session_id=" + sessionId + " | model=" + model
					+ " | plugins=" + pluginNames.toString();
			assistantEvents.putIfAbsent("system_init\0metadata", metadata);
		} catch (Exception e) {
			LOG.error("ClaudeCodeJsonParser: failed to parse system init event", e);
			throw e;
		}
	}

	private static String joinErrors(JsonNode errors) {
		if (!errors.isArray() || errors.isEmpty())
			return "";
		StringBuilder sb = new StringBuilder();
		for (JsonNode error : errors) {
			String text = JsonUtil.plainText(error);
			if (text.isEmpty())
				continue;
			if (sb.length() > 0)
				sb.append("\n");
			sb.append(text);
		}
		return sb.toString();
	}

	public static String commented(String input) {
		while (input.indexOf("\n\n") != -1)
			input = input.replace("\n\n", "\n");
		if (input.endsWith("\n"))
			input = input.substring(0, input.length() - 1);
		return "#:" + input.replace("\n", "\n#:");
	}
}
