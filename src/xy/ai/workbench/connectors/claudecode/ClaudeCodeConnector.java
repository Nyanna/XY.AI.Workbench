package xy.ai.workbench.connectors.claudecode;

import java.io.BufferedReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
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

		// Combine systemPrompt, tools, and input into a single text block
		StringBuilder text = new StringBuilder();
		if (systemPrompt != null && !systemPrompt.isBlank())
			text.append(systemPrompt).append("\n\n");
		if (tools != null)
			for (String tool : tools)
				if (tool != null && !tool.isBlank())
					text.append(tool).append("\n\n");
		if (input != null && !input.isBlank())
			text.append(input);

		String promptJson = buildPromptJson(text.toString().trim());

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

		return new ClaudeCodeRequest(id, promptJson, paths[0], paths[1]);
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
			stdin.println(req.promptJson);
			stdin.flush();
			return readUntilResult(req);
		} catch (IOException e) {
			throw new IllegalStateException("Claude Code CLI error", e);
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
		cmd.add("--profile");
		cmd.add(profile);
		cmd.add("--agent");
		cmd.add("default");
		cmd.add("--verbose");
		cmd.add("--include-hook-events");
		cmd.add("--include-partial-messages");
		cmd.add("--input-format");
		cmd.add("stream-json");
		cmd.add("--output-format");
		cmd.add("stream-json");
		cmd.add("--replay-user-messages");
		cmd.add("--effort");
		cmd.add(cfg.getReasoning().name().toLowerCase());
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

		try {
			String line;
			while ((line = stdout.readLine()) != null) {
				// Mirror every line to the output JSON file
				if (mirror != null) {
					mirror.write(line);
					mirror.write(System.lineSeparator());
					mirror.flush();
				}

				// Check for result event
				try {
					JsonNode node = mapper.readTree(line);
					if ("result".equals(node.path("type").asText()))
						return parseResult(req.id, node);
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

	private ClaudeCodeResponse parseResult(String id, JsonNode node) {
		boolean isError = node.path("is_error").asBoolean(false)
				|| "error".equals(node.path("subtype").asText());
		String resultText = node.path("result").asText("");

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

		return resp;
	}

	@Override
	public AIAnswer convertResponse(IModelResponse response, IProgressMonitor mon) {
		ClaudeCodeResponse resp = (ClaudeCodeResponse) response;
		AIAnswer answer = new AIAnswer(resp.id);
		answer.inputToken = resp.inputTokens;
		answer.outputToken = resp.outputTokens;
		answer.totalToken = answer.inputToken + answer.outputToken;
		answer.cacheRead = resp.cacheReadInputTokens;
		answer.cacheCreate = resp.cacheCreationInputTokens;
		answer.answer = resp.resultText;
		return answer;
	}
}
