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
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class ClaudeCodeConnector implements IAIConnector {

	private final ObjectMapper mapper = new ObjectMapper();
	private final ResultPostProcessor resultPostProcessor = new ResultPostProcessor();
	private final ClaudeCodeRequestBuilder requestBuilder;
	private final ClaudeCodeJsonParser jsonParser;
	private boolean recordText = false;

	private Process process;
	private PrintWriter stdin;
	private BufferedReader stdout;
	private Path processWorkDir;
	private String profile;
	private ConfigManager cfg;

	public ClaudeCodeConnector(ConfigManager cfg) {
		this.requestBuilder = new ClaudeCodeRequestBuilder(mapper, cfg);
		this.cfg = cfg;
		this.jsonParser = new ClaudeCodeJsonParser(mapper, resultPostProcessor);
		this.jsonParser.setRecordText(recordText);
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
		SubMonitor sub = SubMonitor.convert(mon, "Create request", 2);
		String id = UUID.randomUUID().toString();

		// --- Preprocessing: extract Allow/Deny lines and /exit command from input ---
		sub.subTask("Preproccess input");
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
		sub.worked(1);

		sub.subTask("Build Prompt");
		String trimmed = text.toString().trim();
		String promptJson = trimmed.isEmpty() ? null : requestBuilder.buildPromptJson(trimmed);

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
		sub.worked(1);

		return new ClaudeCodeRequest(id, preMessages, promptJson, paths[0], paths[1], exitFlag[0]);
	}

	@Override
	public IModelResponse executeRequest(IModelRequest request, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 2);
		ClaudeCodeRequest req = (ClaudeCodeRequest) request;
		try {
			ensureProcess(req, sub.split(1));
			// Send approve/deny pre-messages first
			for (String msg : req.preMessages) {
				stdin.println(msg);
			}
			// /exit with no further prompt: terminate immediately, return control-flow
			// response
			if (req.exitAfterResult && req.promptJson == null) {
				stdin.flush();
				sub.subTask("Terminate Claude CLI proccess");
				terminateProcess();
				ClaudeCodeResponse resp = new ClaudeCodeResponse(req.id, "Session closed!", false);
				resp.isExited = true;
				try {
					Thread.sleep(1000); // die to delay for saving marker
				} catch (InterruptedException e) {
				}
				return resp;
			}

			sub.subTask("Sending prompt");
			// Send the prompt if present
			if (req.promptJson != null) {
				stdin.println(req.promptJson);
			}
			stdin.flush();

			sub.subTask("Waiting for answer");
			ClaudeCodeResponse resp = readUntilResult(req, sub.split(1));
			// /exit after result: terminate subprocess
			if (req.exitAfterResult) {
				sub.subTask("Terminate Claude CLI proccess");
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

	private synchronized void ensureProcess(ClaudeCodeRequest req, IProgressMonitor mon) throws IOException {
		SubMonitor sub = SubMonitor.convert(mon, "Start CLI", 1);
		
		sub.subTask("Check Proccess");
		if (process != null && process.isAlive()) {
			sub.subTask("Proccess already running");
			return;
		}

		sub.subTask("Start Claude-Code-CLI...");
		// Determine working directory: fixed on first start, preserved across restarts
		if (processWorkDir == null)
			if (req.workDir != null)
				processWorkDir = req.workDir;
			else
				throw new IllegalStateException(
						"No active editor to determine working directory for Claude Code process");

		List<String> cmd = requestBuilder.buildCommand(profile);
		ProcessBuilder pb = new ProcessBuilder(cmd);
		pb.directory(processWorkDir.toFile());
		pb.redirectErrorStream(false);

		// Disable spell check: set environment variable for prompt hook
		pb.environment().put("CLAUDE_CODE_DISABLE_SPELLCHECK", "true");
		if (Reasoning.Disabled.equals(cfg.getReasoning())) {
			pb.environment().put("MAX_THINKING_TOKENS", "0");
		}

		process = pb.start();
		stdin = new PrintWriter(process.getOutputStream());
		stdout = new BufferedReader(new InputStreamReader(process.getInputStream()));

		LOG.info("Claude Code process started in: " + processWorkDir);
		LOG.info("Claude-CLI command: " + String.join(" ", cmd));
		sub.worked(1);;
	}

	private ClaudeCodeResponse readUntilResult(ClaudeCodeRequest req, IProgressMonitor mon) throws IOException {
		SubMonitor sub = SubMonitor.convert(mon, "Reading Claude output", IProgressMonitor.UNKNOWN);
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
					if ("result".equals(type)) {
						sub.subTask("Received final result");
						return jsonParser.parseResult(req.id, node, assistantEvents, totalReasoningTokens);
					}
					if ("tool_use".equals(type)) {
						sub.subTask("Received tool use requst");
						return jsonParser.parseToolUse(req.id, node);
					}

					if ("stream_event".equals(type)) {
						String eventType = node.path("event").path("type").asText();
						if ("message_delta".equals(eventType)) {
							totalReasoningTokens += jsonParser.collectMessageDeltaEvent(node, assistantEvents);
						}
					} else if ("rate_limit_event".equals(type))
						jsonParser.processRateLimitEvent(node);
					else if ("assistant".equals(type)) {
						jsonParser.collectAssistantEvents(node, assistantEvents, sub.split(1));
					}

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
	 * Scans {@code input} line by line.
	 * <ul>
	 * <li>Lines that are solely "/allow &lt;id&gt;" or "/deny &lt;id&gt;" are
	 * extracted: an approve/deny JSON is added to {@code preMessages} and the line
	 * is removed.</li>
	 * <li>A line that is solely "/exit" sets {@code exitFlag[0] = true} and is
	 * removed.</li>
	 * </ul>
	 * The remaining lines (after trimming the whole result) are returned; returns
	 * null when nothing is left.
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
				preMessages.add(requestBuilder.buildApproveJson(toolUseId));
			} else if (trimmed.matches("/(?i)deny\\s+\\S+")) {
				String toolUseId = trimmed.split("\\s+", 2)[1];
				preMessages.add(requestBuilder.buildDenyJson(toolUseId));
			} else if ("/exit".equals(trimmed)) {
				exitFlag[0] = true;
			} else {
				remaining.add(line);
			}
		}
		String result = String.join("\n", remaining).strip();
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
}
