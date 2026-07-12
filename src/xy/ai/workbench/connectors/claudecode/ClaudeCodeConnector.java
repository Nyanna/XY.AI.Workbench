package xy.ai.workbench.connectors.claudecode;

import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
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

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.LOG;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.connectors.claudecode.ClaudeCodeRequest.Command;
import xy.ai.workbench.models.AIAnswer;

public class ClaudeCodeConnector implements IAIConnector<ClaudeCodeRequest, ClaudeCodeResponse> {

	private final ClaudeCodeRequestBuilder requestBuilder = new ClaudeCodeRequestBuilder();
	private final ClaudeCodeProtocol jsonParser = new ClaudeCodeProtocol();
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
	public ClaudeCodeRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools,
			boolean batchFix, IProgressMonitor mon) {
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
	public ClaudeCodeResponse executeRequest(ClaudeCodeRequest req, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 2);
		ClaudeCodeSession session = null;

		SessionParameters params = new SessionParameters(getEditorFilePath(), req.systemPrompt, req.tools,
				cfg.getModel(), cfg.getReasoning(), cfg.getProfile(), cfg.getKeys());
		params.setTitle(req.title);

		switch (req.cmd.type) {
		case Resume:
			sub.subTask("Importing session");
			sessionManager.importSession(req.cmd.parameter, params);
			return new ClaudeCodeResponse(req.id, "Session created");
		case Exit:
			session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
			sub.subTask("Terminating CLI process");
			session.terminate();
			return new ClaudeCodeResponse(req.id, "Session closed!");
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
		ClaudeCodeResponse resp = new ClaudeCodeResponse(req.id);

		String line;
		while (true) {
			// alternate read sources undtil answer
			controlClient.checkControlEndpoint(resp);

			if (!resp.isReady())
				try {
					// wait 300 ms
					if ((line = session.readLine()) != null) {
						jsonParser.parseLine(resp, session, sub, line);

					}
				} catch (Exception ex) {
					while ((line = session.readError()) != null)
						LOG.error("ClaudeCodeConnector: CLI stderr: " + line);
					throw ex;
				}

			if (resp.isReady())
				return resp;
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
	public AIAnswer convertResponse(ClaudeCodeResponse resp, IProgressMonitor mon) {
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
