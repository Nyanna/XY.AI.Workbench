package xy.ai.workbench.connectors.claudecode;

import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Iterator;
import java.util.LinkedHashMap;
import java.util.Map.Entry;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

import xy.ai.workbench.LOG;

/**
 * Parses JSON structures from Claude Code API responses. Handles extraction and
 * processing of results, tool uses, events, and rate limits.
 */
public class ClaudeCodeJsonParser {
	public static final String THINKING = "Thinking:";
	public static final String TEXT = "Text:";
	public static final String TOOLUSE = "Tool:";
	private static final int TOOL_INPUT_MAX_LENGTH = 120;

    private final ObjectMapper mapper = new ObjectMapper();
    private final ResultPostProcessor resultPostProcessor = new ResultPostProcessor();
	private boolean recordText = false;

	/**
	 * Parses a result event from the Claude Code API response. Combines
	 * thinking/text events and model usage information into a ClaudeCodeResponse.
	 *
	 * @param id                   request ID
	 * @param node                 the result JSON node
	 * @param assistantEvents      collected assistant events to prepend
	 * @param totalReasoningTokens accumulated reasoning tokens
	 * @return ClaudeCodeResponse with parsed result
	 */
	public ClaudeCodeResponse parseResult(String id, JsonNode node, LinkedHashMap<String, String> assistantEvents,
			long totalReasoningTokens) {
		boolean isError = node.path("is_error").asBoolean(false) || "error".equals(node.path("subtype").asText());
		String resultText = resultPostProcessor.process(node.path("result").asText(""));

		// Prepend collected thinking/text events as markdown lines
		if (!assistantEvents.isEmpty()) {
			StringBuilder prefix = new StringBuilder();
			for (String line : assistantEvents.values()) {
				prefix.append(line).append("\n");
			}
			prefix.append("\n");
			resultText = commented(prefix.toString()) + "\n" + resultText;
		}

		ClaudeCodeResponse resp = new ClaudeCodeResponse(id, resultText, isError);

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

		// Set the accumulated reasoning tokens
		resp.reasoningTokens = totalReasoningTokens;

		return resp;
	}

	/**
	 * Parses a tool_use event from the Claude Code API response.
	 *
	 * @param requestId the request ID
	 * @param node      the tool_use JSON node
	 * @return ClaudeCodeResponse with tool use information
	 */
	public ClaudeCodeResponse parseToolUse(String requestId, JsonNode node) {
		String toolName = node.path("name").asText("");
		String toolUseId = node.path("id").asText("");
		JsonNode input = node.path("input");

		String inputStr;
		if (input.isObject() && input.size() == 1) {
			@SuppressWarnings("deprecation")
			String val = input.fields().next().getValue().asText();
			inputStr = "`" + val + "`";
		} else {
			try {
				inputStr = mapper.writeValueAsString(input);
			} catch (Exception e) {
				inputStr = input.toString();
			}
		}

		String markdown = "Tool: " + toolName + "\nInput: " + inputStr + "\nID: " + toolUseId;
		return new ClaudeCodeResponse(requestId, commented(markdown), toolUseId);
	}

	/**
	 * Extracts "thinking" and "text" content blocks from an assistant event
	 * snapshot and stores them in the ordered dedup map. Blocks already seen (same
	 * type + content) are silently ignored so that repeated snapshots do not
	 * produce duplicates.
	 *
	 * @param node            the assistant event JSON node
	 * @param assistantEvents map to collect events
	 * @param mon
	 */
	public void collectAssistantEvents(JsonNode node, LinkedHashMap<String, String> assistantEvents,
			IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Received Claude message", 1);

		JsonNode content = node.path("message").path("content");
		if (!content.isArray())
			return;
		for (JsonNode block : content) {
			String blockType = block.path("type").asText();
			if ("thinking".equals(blockType)) {
				// Some versions use "thinking" field, others fall back to "text"
				String text = block.path("thinking").asText("");
				if (text.isEmpty())
					text = block.path("text").asText("");
				if (!text.isEmpty())
					assistantEvents.putIfAbsent("thinking\0" + text, THINKING + "\n" + text);
				sub.subTask("Claude is thinking");
			} else if (recordText && "text".equals(blockType)) {
				String text = block.path("text").asText("");
				if (!text.isEmpty())
					assistantEvents.putIfAbsent("text\0" + text, TEXT + "\n" + text);
			} else if ("tool_use".equals(blockType)) {
				String toolName = block.path("name").asText("");
				String text = " " + toolName + "\n";
				JsonNode inputs = block.path("input");
				if (inputs.isObject()) {
					var inputNames = inputs.fieldNames();
					while (inputNames.hasNext()) {
						String inputName = inputNames.next();
						String value = inputs.path(inputName).toString();
						if (value.length() > TOOL_INPUT_MAX_LENGTH)
							value = value.substring(0, TOOL_INPUT_MAX_LENGTH) + "…";
						text += inputName + ": " + value.replace('\n', ' ') + "\n";
					}
				}
				if (!text.isEmpty()) {
					assistantEvents.putIfAbsent("tool\0" + text, TOOLUSE + text);
					sub.subTask("Claude uses tool: " + toolName);
				}
			}
		}
		sub.worked(1);
	}

	/**
	 * Extracts reasoning tokens from a message_delta event. The thinking_tokens are
	 * found at event.usage.output_tokens_details.thinking_tokens. If present,
	 * appends "ReasoningToken: <count>" to assistantEvents with key
	 * "reasoning\0<count>".
	 *
	 * @param node            the message_delta event JSON node
	 * @param assistantEvents map to collect events
	 * @return the thinking_tokens count extracted from this event, or 0 if not
	 *         present
	 */
	public long collectMessageDeltaEvent(JsonNode node, LinkedHashMap<String, String> assistantEvents) {
		long thinkingTokens = 0;
		JsonNode usage = node.path("event").path("usage");
		if (usage.isObject()) {
			JsonNode outputTokensDetails = usage.path("output_tokens_details");
			if (outputTokensDetails.isObject()) {
				thinkingTokens = outputTokensDetails.path("thinking_tokens").asLong(0);
				if (thinkingTokens > 0) {
					String key = "reasoning\0" + thinkingTokens;
					String value = "ReasoningToken: " + thinkingTokens;
					assistantEvents.putIfAbsent(key, value);
				}
			}
		}
		return thinkingTokens;
	}

	/**
	 * Processes a rate_limit_event from the Claude Code API. Extracts rate limit
	 * info (utilization, resets_at, status, etc.) for both five_hour and seven_day
	 * limits, and logs them in a human-readable format with timestamps converted to
	 * readable date/time.
	 *
	 * @param node the rate_limit_event JSON node
	 */
	public void processRateLimitEvent(JsonNode node) {
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

			// Convert resetsAt Unix timestamp to human-readable format
			String resetsAtReadable = resetsAt > 0
					? new SimpleDateFormat("yyyy-MM-dd HH:mm:ss").format(new Date(resetsAt * 1000))
					: "unknown";

			// Build log message
			StringBuilder logMsg = new StringBuilder();
			logMsg.append("Rate Limit Event [").append(rateLimitType).append("]: ");
			logMsg.append("status=").append(status);
			logMsg.append(" | utilization=").append(String.format("%.2f%%", utilization * 100));
			logMsg.append(" | resets_at=").append(resetsAtReadable);

			// Add optional fields only if they have meaningful values
			if (!errorCode.isEmpty()) {
				logMsg.append(" | errorCode=").append(errorCode);
			}
			if (canUserPurchaseCredits) {
				logMsg.append(" | canUserPurchaseCredits=").append(canUserPurchaseCredits);
			}
			if (hasChargeableSavedPaymentMethod) {
				logMsg.append(" | hasChargeableSavedPaymentMethod=").append(hasChargeableSavedPaymentMethod);
			}
			if (isUsingOverage) {
				logMsg.append(" | isUsingOverage=").append(isUsingOverage);
			}
			if (!overageStatus.isEmpty()) {
				logMsg.append(" | overageStatus=").append(overageStatus);
			}
			if (!overageDisabledReason.isEmpty()) {
				logMsg.append(" | overageDisabledReason=").append(overageDisabledReason);
			}

			LOG.info(logMsg.toString());
		} catch (Exception e) {
			LOG.error("ClaudeCodeConnector: failed to process rate limit event", e);
		}
	}

	/**
	 * Parses a system init event from the Claude Code API response.
	 * Extracts metadata: cwd, session_id, model, and plugin names (comma-separated).
	 * Stores the formatted metadata line in assistantEvents.
	 *
	 * @param node            the system init event JSON node
	 * @param assistantEvents map to collect events
	 */
	public void parseSystemInitEvent(JsonNode node, LinkedHashMap<String, String> assistantEvents) {
		try {
			String cwd = node.path("cwd").asText("");
			String sessionId = node.path("session_id").asText("");
			String model = node.path("model").asText("");

			// Extract plugin names from the plugins array
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

			// Format as a single line with all metadata
			String metadata = "SystemInit: cwd=" + cwd + " | session_id=" + sessionId + " | model=" + model
					+ " | plugins=" + pluginNames.toString();

			assistantEvents.putIfAbsent("system_init\0metadata", metadata);
		} catch (Exception e) {
			LOG.error("ClaudeCodeJsonParser: failed to parse system init event", e);
		}
	}

	/**
	 * Converts text to commented format (markdown-style comments). Removes
	 * duplicate blank lines and prepends "#: " prefix to each line.
	 *
	 * @param input the input text
	 * @return commented text
	 */
	public String commented(String input) {
		while (input.indexOf("\n\n") != -1)
			input = input.replace("\n\n", "\n");
		if (input.endsWith("\n"))
			input = input.substring(0, input.length() - 1);
		return "#: " + input.replace("\n", "\n#: ");
	}
}
