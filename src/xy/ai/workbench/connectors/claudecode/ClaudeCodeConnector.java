package xy.ai.workbench.connectors.claudecode;

import java.io.IOException;
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

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.LOG;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.connectors.claudecode.ClaudeCodeRequest.Command;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class ClaudeCodeConnector implements IAIConnector {

	private final ClaudeCodeRequestBuilder requestBuilder = new ClaudeCodeRequestBuilder();
	private final ClaudeCodeJsonParser jsonParser = new ClaudeCodeJsonParser();
	private final ClaudeCodeControlClient controlClient = new ClaudeCodeControlClient();
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
	public IModelRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,
			IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Create request", 2);
		String id = UUID.randomUUID().toString();

		// Preprocessing: extract Allow/Deny/exit/resume lines
		String title = null;
		Command command = null;
		StringBuilder merged = null;

		{ // Preprocess
			sub.subTask("Preprocess input");
			for (Command cmd : preprocessInput(inputs))
				if (CommandType.Prompt.equals(cmd.type)) {
					if (title == null)
						title = cmd.parameter.substring(0, Math.min(100, cmd.parameter.length())).replace('\n', ' ');

					if (merged != null)
						merged.append("\n");
					else
						merged = new StringBuilder();
					merged.append(cmd.parameter);
				} else {
					command = cmd;
					break;
				}
			sub.worked(1);
		}

		if (command == null && merged != null) {
			sub.subTask("Build prompt");
			command = new Command(CommandType.Prompt, requestBuilder.buildPromptJson(merged.toString().trim()));
			sub.worked(1);
		}

		return new ClaudeCodeRequest(id, title, systemPrompt, tools, command);
	}

	@Override
	public IModelResponse executeRequest(IModelRequest request, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 2);
		ClaudeCodeRequest req = (ClaudeCodeRequest) request;
		ClaudeCodeSession session = null;

		SessionParameters params = new SessionParameters(getEditorFilePath(), req.systemPrompt, req.tools,
				cfg.getModel(), cfg.getReasoning(), cfg.getProfile(), cfg.getKeys());
		params.setTitle(req.title);

		switch (req.cmd.type) {
		case Resume:
			sub.subTask("Importing session");
			sessionManager.importSession(req.cmd.parameter, params);
			return new ClaudeCodeResponse(req.id, "Session created", false);
		case Exit:
			session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
			sub.subTask("Terminating CLI process");
			session.terminate();
			return new ClaudeCodeResponse(req.id, "Session closed!", false);
		case Allow:
		case Deny:
		case Modification:
			switch (req.cmd.type) {
			case Allow:
				controlClient.approve(req.cmd.parameter);
				break;
			case Deny:
				controlClient.deny(req.cmd.parameters[0], req.cmd.parameters[1]);
				break;
			case Modification:
				break; // allready sent
			default:
				throw new UnsupportedOperationException();
			}
			session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
			break;
		case Prompt:
			sub.subTask("Acquiring session");
			session = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);
			break;
		}

		try {
			session.setInPrompt(true);
			if (CommandType.Prompt.equals(req.cmd.type)) {
				if (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !controlClient.isMCPCAvailable())
					throw new IllegalStateException("MCPC not reachable for AgentProfile");

				sub.subTask("Sending prompt");
				session.writeLine(req.cmd.parameter);
			}

			sub.subTask("Waiting for answer");
			return readUntilResult(req, session, sub.split(1));

		} catch (IOException e) {
			throw new IllegalStateException("Claude Code CLI error", e);
		} finally {
			session.setInPrompt(false);
		}
	}

	private ClaudeCodeResponse readUntilResult(ClaudeCodeRequest req, ClaudeCodeSession session, IProgressMonitor mon)
			throws IOException {
		SubMonitor sub = SubMonitor.convert(mon, "Reading Claude output", IProgressMonitor.UNKNOWN);

		// Ordered map: key = "type\0content" for dedup, value = formatted markdown line
		LinkedHashMap<String, String> assistantEvents = new LinkedHashMap<>();
		long totalReasoningTokens = 0;

		String line;
		while (true) {
			ClaudeCodeResponse pendingResponse = controlClient.checkControlEndpoint(req);
			if (pendingResponse != null)
				return pendingResponse;

			if ((line = session.readLine()) == null)
				break;

			// Step 1: parse. A malformed line is a genuine problem (e.g. a large web
			// research result truncated by the transport) and must be logged, never
			// silently swallowed — swallowing it is why long results never appeared.
			JsonNode node;
			try {
				node = JsonUtil.readTree(line);
			} catch (JsonProcessingException parseError) {
				LOG.error("ClaudeCodeConnector: could not parse CLI line as JSON (length=" + line.length() + "): "
						+ JsonUtil.abbreviate(line), parseError);
				continue;
			}

			// Step 2: dispatch. A failure here is a bug in our handling of a valid
			// event; log it with context (type + length) and keep reading so a single
			// bad event cannot strand the whole response.
			String type = JsonUtil.plainText(node.path("type"));
			try {
				if ("result".equals(type)) {
					sub.subTask("Received final result");
					return jsonParser.parseResult(req.id, node, assistantEvents, totalReasoningTokens);
				}
				if ("tool_use".equals(type)) {
					sub.subTask("Received tool use request");
					return jsonParser.parseToolUse(req.id, node);
				}

				if ("system".equals(type)) {
					String subtype = JsonUtil.plainText(node.path("subtype"));
					if ("init".equals(subtype)) {
						sub.subTask("Received system init metadata");
						jsonParser.parseSystemInitEvent(node, assistantEvents);
						updateLastParsedMessage(session, assistantEvents);
					}
				} else if ("stream_event".equals(type)) {
					String eventType = JsonUtil.plainText(node.path("event").path("type"));
					if ("message_delta".equals(eventType)) {
						totalReasoningTokens += jsonParser.collectMessageDeltaEvent(node, assistantEvents);
						updateLastParsedMessage(session, assistantEvents);
					}
				} else if ("rate_limit_event".equals(type)) {
					jsonParser.processRateLimitEvent(node);
				} else if ("assistant".equals(type)) {
					boolean recordToolUse = !AgentProfile.MCPC.equals(session.getParameters().agentProfile);
					jsonParser.collectAssistantEvents(node, assistantEvents, false, recordToolUse, sub.split(1));
					updateLastParsedMessage(session, assistantEvents);
				}
			} catch (Exception processingError) {
				LOG.error("ClaudeCodeConnector: failed to process CLI event (type=" + type + ", length=" + line.length()
						+ "): " + JsonUtil.abbreviate(line), processingError);
			}
		}

		// STDOUT closed without a result — surface anything the CLI left on STDERR.
		while ((line = session.readError()) != null)
			LOG.error("ClaudeCodeConnector: CLI stderr: " + line);

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

	private List<Command> preprocessInput(List<String> inputs) {
		List<Command> commands = new ArrayList<Command>();
		String clean;
		for (String input : inputs)
			if (!(clean = input != null ? input.strip() : "").isBlank())
				if ("/exit".equalsIgnoreCase(clean))
					commands.add(new Command(CommandType.Exit, ""));
				else if (clean.matches("(?i)/resume\\s+\\S+"))
					commands.add(new Command(CommandType.Resume, clean.split("\\s+", 2)[1].strip()));
				else if (clean.matches("(?i)/allow\\s+\\S+"))
					commands.add(new Command(CommandType.Allow, clean.split("\\s+", 2)[1].strip()));
				else if (clean.matches("(?i)/deny\\s+\\S+(\\s+.*)?")) {
					String[] parts = clean.split("\\s+", 2)[1].strip().split("\\s+", 2);
					commands.add(
							new Command(CommandType.Deny, parts[0].strip(), parts.length > 1 ? parts[1].strip() : ""));
				} else if (controlClient.submitEdit(clean))
					commands.add(new Command(CommandType.Modification, ""));
				else
					commands.add(new Command(CommandType.Prompt, clean));
		if (commands.isEmpty())
			throw new IllegalStateException("No commands in inputs");
		return commands;
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
