package xy.ai.workbench.connectors.claudecode;

import java.io.BufferedReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.UUID;

import org.eclipse.core.resources.IProject;
import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.PlatformUI;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.LOG;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class ClaudeCodeConnector implements IAIConnector {

	private static final String SCRIPT = System.getProperty("user.home")
			+ "/xyan/xy.ai.workbench/claude-code/claude-session.sh";

	private final ConfigManager cfg;
	private final ObjectMapper mapper = new ObjectMapper();
	private final ResultPostProcessor resultPostProcessor = new ResultPostProcessor();
	private boolean recordText = false;

	private Process process;
	private PrintWriter stdin;
	private BufferedReader stdout;
	private Path processWorkDir;
	private String profile;

	public ClaudeCodeConnector(ConfigManager cfg) {
		this.cfg = cfg;
		cfg.addKeyObs(k -> {
			if (getSupportedKeyPattern().matches(k))
				this.profile = k;
		}, true);
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.ClaudeCode;
	}

	@Override
	public IModelRequest createRequest(String input, String systemPrompt, List<String> tools, boolean batchFix,
			IProgressMonitor mon) {
		String id = UUID.randomUUID().toString();

		// --- Preprocessing: extract Allow/Deny lines and /exit command from input ---
		List<String> preMessages = new ArrayList<>();
		boolean[] exitFlag = { false };
		String processedInput = preprocessInput(input, preMessages, exitFlag);

		// Combine systemPrompt, tools, and remaining input into a single text block
		StringBuilder text = new StringBuilder();
		if (systemPrompt != null && !systemPrompt.isBlank())
			text.append(systemPrompt).append("\n\n");
		if (tools != null)
			for (String tool : tools)
				if (tool != null && !tool.isBlank())
					text.append(tool).append("\n\n");
		if (processedInput != null && !processedInput.isBlank())
			text.append(processedInput);

		String trimmed = text.toString().trim();
		String promptJson = trimmed.isEmpty() ? null : buildPromptJson(trimmed);

		// Resolve paths from the active editor on the UI thread
		Path[] paths = new Path[2]; // [0]=workDir, [1]=outputJsonFile
		Display.getDefault().syncExec(() -> {
			try {
				IWorkbenchWindow window = PlatformUI.getWorkbench().getActiveWorkbenchWindow();
				if (window == null)
					return;
				IWorkbenchPage page = window.getActivePage();
				if (page == null)
					return;
				IEditorPart editor = page.getActiveEditor();
				if (editor == null)
					return;
				IEditorInput editorInput = editor.getEditorInput();
				if (!(editorInput instanceof IFileEditorInput))
					throw new IllegalArgumentException("Connector don't supports external files");

				IFileEditorInput fileInput = (IFileEditorInput) editorInput;
				IProject project = fileInput.getFile().getProject();
				paths[0] = Paths.get(project.getLocation().toOSString());

				String filePath = fileInput.getFile().getLocation().toOSString();
				int dotIdx = filePath.lastIndexOf('.');
				String jsonPath = (dotIdx >= 0 ? filePath.substring(0, dotIdx) : filePath) + ".json";
				paths[1] = Paths.get(jsonPath);
			} catch (Exception e) {
				LOG.error("ClaudeCodeConnector: failed to resolve editor paths", e);
			}
		});

		return new ClaudeCodeRequest(id, preMessages, promptJson, paths[0], paths[1], exitFlag[0]);
	}

	private String buildPromptJson(String text) {
		try {
			ObjectNode root = mapper.createObjectNode();
			root.put("type", "user");
			ObjectNode message = root.putObject("message");
			message.put("role", "user");
			ArrayNode content = message.putArray("content");
			ObjectNode textNode = content.addObject();
			textNode.put("type", "text");
			textNode.put("text", text);
			return mapper.writeValueAsString(root);
		} catch (Exception e) {
			throw new IllegalStateException("Failed to build prompt JSON", e);
		}
	}

	@Override
	public IModelResponse executeRequest(IModelRequest request, IProgressMonitor mon) {
		ClaudeCodeRequest req = (ClaudeCodeRequest) request;
		try {
			ensureProcess(req);
			// Send approve/deny pre-messages first
			for (String msg : req.preMessages) {
				stdin.println(msg);
			}
			// /exit with no further prompt: terminate immediately, return control-flow response
			if (req.exitAfterResult && req.promptJson == null) {
				stdin.flush();
				terminateProcess();
				ClaudeCodeResponse resp = new ClaudeCodeResponse(req.id, "Session closed!", false);
				resp.isExited = true;
				try {
					Thread.sleep(1000); // die to delay for saving marker
				} catch (InterruptedException e) {
				}
				return resp;
			}
			// Send the prompt if present
			if (req.promptJson != null) {
				stdin.println(req.promptJson);
			}
			stdin.flush();
			ClaudeCodeResponse resp = readUntilResult(req);
			// /exit after result: terminate subprocess
			if (req.exitAfterResult) {
				terminateProcess();
				resp.isExited = true;
			}
			return resp;
		} catch (IOException e) {
			throw new IllegalStateException("Claude Code CLI error", e);
		}
	}

	private synchronized void terminateProcess() {
		if (process != null) {
			try {
				stdin.close();
			} catch (Exception ignored) {
			}
			process.destroy();
			process = null;
			stdin = null;
			stdout = null;
			LOG.info("Claude Code process terminated via /exit command");
		}
	}

	private synchronized void ensureProcess(ClaudeCodeRequest req) throws IOException {
		if (process != null && process.isAlive())
			return;

		// Determine working directory: fixed on first start, preserved across restarts
		if (processWorkDir == null)
			if (req.workDir != null)
				processWorkDir = req.workDir;
			else
				throw new IllegalStateException(
						"No active editor to determine working directory for Claude Code process");

		ProcessBuilder pb = new ProcessBuilder(buildCommand());
		pb.directory(processWorkDir.toFile());
		pb.redirectErrorStream(false);

		process = pb.start();
		stdin = new PrintWriter(process.getOutputStream());
		stdout = new BufferedReader(new InputStreamReader(process.getInputStream()));

		LOG.info("Claude Code process started in: " + processWorkDir);
	}

	private List<String> buildCommand() {
		List<String> cmd = new ArrayList<>();
		cmd.add(SCRIPT);
		cmd.add(cfg.getProfile().name); // Agent Definition
		cmd.add("--profile");
		cmd.add(profile);
		cmd.add("--verbose");
		cmd.add("--include-hook-events");
		cmd.add("--include-partial-messages");
		cmd.add("--input-format");
		cmd.add("stream-json");
		cmd.add("--output-format");
		cmd.add("stream-json");
		cmd.add("--replay-user-messages");
		cmd.add("--model");
		cmd.add(cfg.getModel().apiName);
		cmd.add("--effort");
		cmd.add(cfg.getReasoning().name().toLowerCase());
		cmd.add("--dangerously-skip-permissions"); //as long there is no permission prompt handling implemented
		LOG.info("Claude-CLI command: " + String.join(" ", cmd));
		return cmd;
	}

	private ClaudeCodeResponse readUntilResult(ClaudeCodeRequest req) throws IOException {
		FileWriter mirror = null;
		if (req.outputJsonFile != null) {
			try {
				mirror = new FileWriter(req.outputJsonFile.toFile(), false);
			} catch (IOException e) {
				LOG.error("ClaudeCodeConnector: cannot open mirror file: " + req.outputJsonFile, e);
			}
		}

		// Ordered map: key = "type\0content" for dedup, value = formatted markdown line
		LinkedHashMap<String, String> assistantEvents = new LinkedHashMap<>();
		// Accumulate reasoning tokens across all message_delta events
		long totalReasoningTokens = 0;

		try {
			String line;
			while ((line = stdout.readLine()) != null) {
				// Mirror every line to the output JSON file
				if (mirror != null) {
					mirror.write(line);
					mirror.write(System.lineSeparator());
					mirror.flush();
				}

				// Check for result, tool_use, stream_event, or assistant event
				try {
					JsonNode node = mapper.readTree(line);
					String type = node.path("type").asText();
					if ("result".equals(type))
						return parseResult(req.id, node, assistantEvents, totalReasoningTokens);
					if ("tool_use".equals(type))
						return parseToolUse(req.id, node);
					if ("stream_event".equals(type)) {
						String eventType = node.path("event").path("type").asText();
						if ("message_delta".equals(eventType)) {
							totalReasoningTokens += collectMessageDeltaEvent(node, assistantEvents);
						}
					}
					if ("rate_limit_event".equals(type)) {
						processRateLimitEvent(node);
					}
					if ("assistant".equals(type))
						collectAssistantEvents(node, assistantEvents);
				} catch (Exception ignored) {
					// Non-JSON or unrecognised line — continue reading
				}
			}
		} finally {
			if (mirror != null)
				try {
					mirror.close();
				} catch (IOException ignored) {
				}
		}

		throw new IllegalStateException("Claude Code process ended without a result event");
	}

	/**
	 * Extracts reasoning tokens from a message_delta event.
	 * The thinking_tokens are found at event.usage.output_tokens_details.thinking_tokens.
	 * If present, appends "ReasoningToken: <count>" to assistantEvents with key "reasoning\0<count>".
	 *
	 * @return the thinking_tokens count extracted from this event, or 0 if not present
	 */
	private long collectMessageDeltaEvent(JsonNode node, LinkedHashMap<String, String> assistantEvents) {
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
	 * Extracts "thinking" and "text" content blocks from an assistant event snapshot
	 * and stores them in the ordered dedup map. Blocks already seen (same type + content)
	 * are silently ignored so that repeated snapshots do not produce duplicates.
	 */
	private void collectAssistantEvents(JsonNode node, LinkedHashMap<String, String> assistantEvents) {
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
					assistantEvents.putIfAbsent("thinking\0" + text, "Thinking: " + text);
			} else if (recordText && "text".equals(blockType)) {
				String text = block.path("text").asText("");
				if (!text.isEmpty())
					assistantEvents.putIfAbsent("text\0" + text, "Text: " + text);
			}
		}
	}

	/**
	 * Scans {@code input} line by line.
	 * <ul>
	 *   <li>Lines that are solely "/allow &lt;id&gt;" or "/deny &lt;id&gt;" are extracted:
	 *       an approve/deny JSON is added to {@code preMessages} and the line is removed.</li>
	 *   <li>A line that is solely "/exit" sets {@code exitFlag[0] = true} and is removed.</li>
	 * </ul>
	 * The remaining lines (after trimming the whole result) are returned; returns null
	 * when nothing is left.
	 */
	private String preprocessInput(String input, List<String> preMessages, boolean[] exitFlag) {
		if (input == null)
			return null;
		String[] lines = input.split("\n", -1);
		List<String> remaining = new ArrayList<>();
		for (String line : lines) {
			String trimmed = line.strip();
			if (trimmed.matches("/(?i)allow\\s+\\S+")) {
				String toolUseId = trimmed.split("\\s+", 2)[1];
				preMessages.add(buildApproveJson(toolUseId));
			} else if (trimmed.matches("/(?i)deny\\s+\\S+")) {
				String toolUseId = trimmed.split("\\s+", 2)[1];
				preMessages.add(buildDenyJson(toolUseId));
			} else if ("/exit".equals(trimmed)) {
				exitFlag[0] = true;
			} else {
				remaining.add(line);
			}
		}
		String result = String.join("\n", remaining).strip();
		return result.isEmpty() ? null : result;
	}

	private String buildApproveJson(String toolUseId) {
		try {
			ObjectNode node = mapper.createObjectNode();
			node.put("type", "approve");
			node.put("tool_use_id", toolUseId);
			return mapper.writeValueAsString(node);
		} catch (Exception e) {
			throw new IllegalStateException("Failed to build approve JSON", e);
		}
	}

	private String buildDenyJson(String toolUseId) {
		try {
			ObjectNode node = mapper.createObjectNode();
			node.put("type", "deny");
			node.put("tool_use_id", toolUseId);
			return mapper.writeValueAsString(node);
		} catch (Exception e) {
			throw new IllegalStateException("Failed to build deny JSON", e);
		}
	}

	private ClaudeCodeResponse parseToolUse(String requestId, JsonNode node) {
		String toolName = node.path("name").asText("");
		String toolUseId = node.path("id").asText("");
		JsonNode input = node.path("input");

		String inputStr;
		if (input.isObject() && input.size() == 1) {
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

	private String commented(String input) {
		while (input.indexOf("\n\n") != -1)
			input = input.replace("\n\n", "\n");
		if(input.endsWith("\n"))
			input = input.substring(0, input.length() - 1);
		return "#: " + input.replace("\n", "\n#: ");
	}

	/**
	 * Processes a rate_limit_event from the SDK.
	 * Extracts rate limit info (utilization, resets_at, status, etc.) for both five_hour and seven_day limits,
	 * and logs them in a human-readable format with timestamps converted to readable date/time.
	 */
	private void processRateLimitEvent(JsonNode node) {
		try {
			JsonNode rateLimitInfo = node.path("rate_limit_info");
			String rateLimitType = rateLimitInfo.path("rateLimitType").asText();

			// Extract fields (use 0 for missing values)
			String status = rateLimitInfo.path("status").asText("unknown");
			long resetsAt = rateLimitInfo.path("resetsAt").asLong(0);
			double utilization = rateLimitInfo.path("utilization").asDouble(0.0);
			String errorCode = rateLimitInfo.path("errorCode").asText("");
			boolean canUserPurchaseCredits = rateLimitInfo.path("canUserPurchaseCredits").asBoolean(false);
			boolean hasChargeableSavedPaymentMethod = rateLimitInfo.path("hasChargeableSavedPaymentMethod").asBoolean(false);
			boolean isUsingOverage = rateLimitInfo.path("isUsingOverage").asBoolean(false);
			String overageStatus = rateLimitInfo.path("overageStatus").asText("");
			String overageDisabledReason = rateLimitInfo.path("overageDisabledReason").asText("");

			// Convert resetsAt Unix timestamp to human-readable format
			String resetsAtReadable = resetsAt > 0
				? new java.text.SimpleDateFormat("yyyy-MM-dd HH:mm:ss").format(new java.util.Date(resetsAt * 1000))
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

	private ClaudeCodeResponse parseResult(String id, JsonNode node, LinkedHashMap<String, String> assistantEvents, long totalReasoningTokens) {
		boolean isError = node.path("is_error").asBoolean(false)
				|| "error".equals(node.path("subtype").asText());
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

		JsonNode modelUsage = node.path("modelUsage");
		if (modelUsage.isObject()) {
			modelUsage.fields().forEachRemaining(entry -> {
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

	@Override
	public AIAnswer convertResponse(IModelResponse response, IProgressMonitor mon) {
		ClaudeCodeResponse resp = (ClaudeCodeResponse) response;
		AIAnswer answer = new AIAnswer(resp.id);
		answer.inputToken = resp.inputTokens + resp.cacheCreationInputTokens;
		answer.outputToken = resp.outputTokens;
		answer.reasoningToken = resp.reasoningTokens;
		answer.totalToken = answer.inputToken + answer.outputToken;
		answer.cacheRead = resp.cacheReadInputTokens;
		answer.cacheCreate = resp.cacheCreationInputTokens;
		answer.answer = resp.resultText;
		return answer;
	}
}
