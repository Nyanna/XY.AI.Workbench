package xy.ai.workbench.connectors.claudecode;

import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.UUID;

import org.eclipse.core.resources.IProject;
import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.PlatformUI;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.LOG;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class ClaudeCodeConnector implements IAIConnector {

	private final ObjectMapper mapper = new ObjectMapper();
	private final ClaudeCodeRequestBuilder requestBuilder = new ClaudeCodeRequestBuilder();
	private final ClaudeCodeJsonParser jsonParser = new ClaudeCodeJsonParser();
	private final ClaudeCodeSessionManager sessionManager;

	private ConfigManager cfg;

	public ClaudeCodeConnector(ConfigManager cfg, ClaudeCodeSessionManager sessionManager) {
		this.cfg = cfg;
		this.sessionManager = sessionManager;
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.ClaudeCode;
	}

	@Override
	public IModelRequest createRequest(String input, String systemPrompt, List<String> tools, boolean batchFix,
			IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Create request", 2);
		String id = UUID.randomUUID().toString();

		// Preprocessing: extract Allow/Deny/exit/resume lines
		sub.subTask("Preprocess input");
		boolean[] exitFlag = { false };
		String[] resumeUuidHolder = { null };
		String processedInput = preprocessInput(input, exitFlag, resumeUuidHolder);
		String title = input.substring(0, Math.min(100, input.length() - 1));

		// Combine system prompt, tools, and input into one text block
		StringBuilder text = new StringBuilder();
		if (systemPrompt != null && !systemPrompt.isBlank())
			text.append(systemPrompt).append("\n\n");
		if (tools != null)
			for (String tool : tools)
				if (tool != null && !tool.isBlank())
					text.append(tool).append("\n\n");
		if (processedInput != null && !processedInput.isBlank())
			text.append(processedInput);
		sub.worked(1);

		sub.subTask("Build prompt");
		String trimmed = text.toString().trim();
		String promptJson = trimmed.isEmpty() ? null : requestBuilder.buildPromptJson(trimmed);
		sub.worked(1);

		return new ClaudeCodeRequest(id, title, promptJson, exitFlag[0], resumeUuidHolder[0]);
	}

	@Override
	public IModelResponse executeRequest(IModelRequest request, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 2);
		ClaudeCodeRequest req = (ClaudeCodeRequest) request;
		ClaudeCodeSession session = null;

		try {
			SessionParameters params = new SessionParameters(getEditorFilePath(), cfg.getModel(), cfg.getReasoning(),
					cfg.getProfile(), cfg.getKeys());
			params.setTitle(req.title);

			if (req.resumeUuid != null) {
				sub.subTask("Importing session");
				sessionManager.importSession(req.resumeUuid, params);
				waitFor();
				return new ClaudeCodeResponse(req.id, "Session created", false);
			}

			sub.subTask("Acquiring session");
			session = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);
			session.setInPrompt(true);
			sub.worked(1);

			// /exit with no prompt: terminate and return
			if (req.exitAfterResult && req.promptJson == null) {
				sub.subTask("Terminating CLI process");
				session.terminate();
				ClaudeCodeResponse resp = new ClaudeCodeResponse(req.id, "Session closed!", false);
				resp.isExited = true;
				waitFor();
				return resp;
			}

			if (req.promptJson != null) {
				sub.subTask("Sending prompt");
				session.writeLine(req.promptJson);
			}

			sub.subTask("Waiting for answer");
			return readUntilResult(req, session, sub.split(1));

		} catch (IOException e) {
			throw new IllegalStateException("Claude Code CLI error", e);
		} finally {
			if (session != null)
				session.setInPrompt(false);
		}
	}

	private void waitFor() {
		try {
			Thread.sleep(1000); // brief delay before saving marker
		} catch (InterruptedException ignored) {
		}
	}

	private ClaudeCodeResponse readUntilResult(ClaudeCodeRequest req, ClaudeCodeSession session, IProgressMonitor mon)
			throws IOException {
		SubMonitor sub = SubMonitor.convert(mon, "Reading Claude output", IProgressMonitor.UNKNOWN);

		// Ordered map: key = "type\0content" for dedup, value = formatted markdown line
		LinkedHashMap<String, String> assistantEvents = new LinkedHashMap<>();
		long totalReasoningTokens = 0;

		String line;
		while ((line = session.readLine()) != null) {
			try {
				JsonNode node = mapper.readTree(line);
				String type = node.path("type").asText();

				if ("result".equals(type)) {
					sub.subTask("Received final result");
					return jsonParser.parseResult(req.id, node, assistantEvents, totalReasoningTokens);
				}
				if ("tool_use".equals(type)) {
					sub.subTask("Received tool use request");
					return jsonParser.parseToolUse(req.id, node);
				}

				if ("stream_event".equals(type)) {
					String eventType = node.path("event").path("type").asText();
					if ("message_delta".equals(eventType)) {
						totalReasoningTokens += jsonParser.collectMessageDeltaEvent(node, assistantEvents);
						updateLastParsedMessage(session, assistantEvents);
					}
				} else if ("rate_limit_event".equals(type)) {
					jsonParser.processRateLimitEvent(node);
				} else if ("assistant".equals(type)) {
					jsonParser.collectAssistantEvents(node, assistantEvents, sub.split(1));
					updateLastParsedMessage(session, assistantEvents);
				}

			} catch (Exception ignored) {
				// Non-JSON or unrecognised line — continue reading
			}
		}

		throw new IllegalStateException("Claude Code process ended without a result event");
	}

	private void updateLastParsedMessage(ClaudeCodeSession session, LinkedHashMap<String, String> assistantEvents) {
		if (!assistantEvents.isEmpty()) {
			String last = null;
			for (String v : assistantEvents.values())
				last = v;
			session.setLastParsedMessage(last);
		}
	}

	private String preprocessInput(String input, boolean[] exitFlag, String[] resumeUuidHolder) {
		if (input == null)
			return null;
		for (String line : input.split("\n", -1)) {
			String trimmed = line.strip();
			if (trimmed.matches("/(?i)allow\\s+\\S+")) {
				String toolUseId = trimmed.split("\\s+", 2)[1];
				return requestBuilder.buildApproveJson(toolUseId);
			} else if (trimmed.matches("/(?i)deny\\s+\\S+")) {
				String toolUseId = trimmed.split("\\s+", 2)[1];
				return requestBuilder.buildDenyJson(toolUseId);
			} else if ("/exit".equalsIgnoreCase(trimmed)) {
				exitFlag[0] = true;
				return trimmed;
			} else if (trimmed.matches("/(?i)resume\\s+\\S+")) {
				resumeUuidHolder[0] = trimmed.split("\\s+", 2)[1];
				return trimmed;
			}
		}
		String result = input.strip();
		return result.isEmpty() ? null : result;
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

	private Path getEditorFilePath() {
		Path[] paths = new Path[1];
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
					throw new IllegalArgumentException("Connector does not support external files");

				IFileEditorInput fileInput = (IFileEditorInput) editorInput;
				IProject project = fileInput.getFile().getProject();
				paths[0] = Paths.get(project.getLocation().toOSString());
			} catch (Exception e) {
				LOG.error("ClaudeCodeConnector: failed to resolve editor paths", e);
			}
		});
		if (paths[0] == null)
			throw new IllegalStateException("Failed to resolve editor paths");
		return paths[0];
	}
}
