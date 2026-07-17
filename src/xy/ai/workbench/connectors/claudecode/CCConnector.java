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
import xy.ai.workbench.connectors.claudecode.CCRequest.Command;
import xy.ai.workbench.models.AIAnswer;

public class CCConnector implements IAIConnector<CCRequest, CCResponse> {

	private final CCRequestBuilder requestBuilder = new CCRequestBuilder();
	private final ProtocolParser jsonParser = new ProtocolParser();
	private final CCControlClient controlClient = new CCControlClient();
	private final CCSessionManager sessionManager;

	private ConfigManager cfg;

	public CCConnector(ConfigManager cfg, CCSessionManager sessionManager) {
		this.cfg = cfg;
		this.sessionManager = sessionManager;
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.ClaudeCode;
	}

	@Override
	public CCRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,
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

		return new CCRequest(id, title, systemPrompt, tools, command);
	}

	@Override
	public CCResponse executeRequest(CCRequest req, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 2);
		CCSession session = null;

		EditorLocation loc = getEditorLocation();
		SessionParameters params = new SessionParameters(loc.projectPath, req.systemPrompt, req.tools, cfg.getModel(),
				cfg.getReasoning(), cfg.getProfile(), cfg.getKeys(), cfg.getCacheMode(), loc.relativeFilePath);
		params.setTitle(req.title);

		switch (req.cmd.type) {
		case Resume:
			sub.subTask("Importing session");
			sessionManager.importSession(req.cmd.parameter, params);
			return new CCResponse(req.id, "Session created");
		case Exit:
			session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
			sub.subTask("Terminating CLI process");
			session.terminate();
			return new CCResponse(req.id, "Session closed!");
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

	private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressMonitor mon) throws IOException {
		SubMonitor sub = SubMonitor.convert(mon, "Reading Claude output", IProgressMonitor.UNKNOWN);
		CCResponse resp = new CCResponse(req.id);

		String line;
		while (true) {
			// alternate read sources undtil answer
			controlClient.checkControlEndpoint(resp);

			if (!resp.isReady())
				try {
					// wait 300 ms
					if ((line = session.readLine()) != null) {
						session.setLastRawLine(line);
						jsonParser.parseLine(resp, session, sub, line);

					}
				} catch (Exception ex) {
					while ((line = session.readError()) != null)
						LOG.error("CLI stderr: " + line);
					throw ex;
				}

			if (resp.isReady()) {
				session.stats.add(resp.stats);
				session.stats.totalinToken = session.stats.inputToken + session.stats.cacheCreate;
				return resp;
			}
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
				else if (clean.matches("(?i)" + CCControlClient.ANSWER + "\\s+\\S+\\s+(allow|deny)(\\s+.*)?")) {
					String[] parts = clean.split("\\s+", 4);
					String id = parts[1];
					String action = parts[2].toLowerCase();
					String reason = parts.length > 3 ? parts[3].strip() : "";
					if ("allow".equals(action))
						commands.add(new Command(CommandType.Allow, id));
					else
						commands.add(new Command(CommandType.Deny, id, reason));
				} else if (controlClient.submitEdit(clean))
					commands.add(new Command(CommandType.Modification, ""));
				else
					commands.add(new Command(CommandType.Prompt, clean));
		if (commands.isEmpty())
			throw new IllegalStateException("No commands in inputs");
		return commands;
	}

	@Override
	public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {
		AIAnswer answer = new AIAnswer(resp.id);
		answer.stats.inputToken = resp.stats.inputToken + resp.stats.cacheCreate;
		answer.stats.outputToken = resp.stats.outputToken;
		answer.stats.reasoningToken = resp.stats.reasoningToken;
		answer.stats.totalinToken = answer.stats.inputToken + answer.stats.outputToken;
		answer.stats.cacheRead = resp.stats.cacheRead;
		answer.stats.cacheCreate = resp.stats.cacheCreate;
		answer.answer = resp.resultText;
		return answer;
	}

	private EditorLocation getEditorLocation() {
		EditorLocation result = new EditorLocation();
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
				Path projectPath = Paths.get(project.getLocation().toOSString());
				String relativeFilePath = fileInput.getFile().getProjectRelativePath().toString();
				result.set(projectPath, relativeFilePath);
			} catch (Exception e) {
				LOG.error("Failed to resolve editor paths", e);
			}
		});
		if (result.projectPath == null)
			throw new IllegalStateException("Failed to resolve editor paths");
		return result;
	}
}
