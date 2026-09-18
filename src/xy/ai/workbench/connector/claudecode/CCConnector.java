package xy.ai.workbench.connector.claudecode;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.UUID;
import java.util.stream.Collectors;
import java.util.zip.CRC32;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.LOG;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.commands.AnswerCommand;
import xy.ai.workbench.commands.CallCommand;
import xy.ai.workbench.commands.CallEditCommand;
import xy.ai.workbench.commands.Command;
import xy.ai.workbench.commands.ExitCommand;
import xy.ai.workbench.commands.ResumeCommand;
import xy.ai.workbench.connector.IAIConnector;
import xy.ai.workbench.connector.harness.FrozenConfig;
import xy.ai.workbench.connector.harness.Prompt;
import xy.ai.workbench.models.AIAnswer;

public class CCConnector implements IAIConnector<CCRequest, CCResponse> {

	private final CCRequestBuilder requestBuilder = new CCRequestBuilder();
	private final ProtocolParser jsonParser = new ProtocolParser();
	private final CCControlClient controlClient = new CCControlClient();
	private final CCSessionManager sessionManager;

	public CCConnector(CCSessionManager sessionManager) {
		this.sessionManager = sessionManager;
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.ClaudeCode;
	}

	@Override
	public CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Create request", 1);

		Command command = prompt.command;
		String promptText = null;
		if (command == null) {
			promptText = prompt.inputs.stream().map(String::strip).filter(s -> !s.isBlank())
					.collect(Collectors.joining("\n"));
			if (promptText.isEmpty())
				throw new IllegalStateException("No commands in inputs");
		}
		sub.worked(1);

		String title = promptText == null ? null : promptText.substring(0, Math.min(100, promptText.length())).replace('\n', ' ');
		return new CCRequest(UUID.randomUUID().toString(), title, prompt.config, command, promptText, prompt.yamlBlock,
				prompt.absoluteFilePath, prompt.projectPath);
	}

	@Override
	public CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {
		SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 2);
		CCSession session = null;

		String relativeFilePath = req.projectPath.relativize(Paths.get(req.absoluteFilePath)).toString();
		FrozenConfig fc = req.config;
		ClaudeSessionParameters params = new ClaudeSessionParameters(req.projectPath, fc.systemPrompt, fc.tools,
				fc.model, fc.reasoning, fc.profile, fc.cliProfile, fc.cacheMode, relativeFilePath);
		params.setTitle(req.title);

		Command command = req.command;
		if (command instanceof ResumeCommand rc) {
			sub.subTask("Importing session");
			sessionManager.importSession(rc.parameter(0), params);
			return new CCResponse(req.id, "Session created");
		}
		if (command instanceof ExitCommand) {
			session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
			sub.subTask("Terminating CLI process");
			session.terminate();
			return new CCResponse(req.id, "Session closed!");
		}
		if (command instanceof AnswerCommand ac) {
			if (ac.action() == AnswerCommand.Action.Allow)
				controlClient.approve(ac.parameter(0), ac.parameter(2));
			else
				controlClient.deny(ac.parameter(0), ac.parameter(2));
			session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
		} else if (command instanceof CallEditCommand cec) {
			if (!controlClient.submitEdit(cec.yaml()))
				throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
			session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
		} else if (command instanceof CallCommand) {
			if (req.yamlBlock == null || req.yamlBlock.isBlank())
				throw new IllegalArgumentException("No preceding ```yaml block found before /call");
			if (!controlClient.submitEdit(req.yamlBlock))
				throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
			session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
		} else if (command == null) {
			sub.subTask("Acquiring session");
			session = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);
		} else {
			throw new UnsupportedOperationException("Unsupported command: " + command.prefix());
		}

		try {
			session.setInPrompt(true);
			if (command == null) {
				if (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !controlClient.isMCPCAvailable())
					throw new IllegalStateException("MCPC not reachable for AgentProfile");

				sub.subTask("Sending prompt");
				session.writeLine(requestBuilder.buildPromptJson(req.promptText));
			}

			sub.subTask("Waiting for answer");
			return readUntilResult(req, session, sub.split(1), job);

		} catch (IOException e) {
			throw new IllegalStateException("Claude Code CLI error", e);
		} finally {
			session.setInPrompt(false);
		}
	}

	private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressMonitor mon, Job job) throws IOException {
		SubMonitor sub = SubMonitor.convert(mon, "Reading Claude output", 10000);
		CCResponse resp = new CCResponse(req.id);

		String line;
		while (true) {
			// alternate read sources undtil answer
			try {
				// wait 300 ms
				if ((line = session.readLine()) != null) {
					session.setLastRawLine(line);
					job.setName(abbreviateForStatus(line));
					mon.worked(1);
					jsonParser.parseLine(resp, session, sub, line);

				}
			} catch (Exception ex) {
				LOG.error("Error reading line: ", ex);
				try {
					while ((line = session.readError()) != null)
						LOG.error("CLI stderr: " + line);
				} catch (Exception ex2) {
					LOG.error("Error reading stderr: ", ex2);
				}
				resp.resultText = ex.getMessage();
			}

			if (!resp.isReady())
				controlClient.checkControlEndpoint(resp);

			if (resp.isReady()) {
				resp.sessionId = session.getID();
				session.stats.add(resp.stats);
				session.stats.totalToken = session.stats.inputToken + session.stats.cacheCreate;
				return resp;
			}
			sessionManager.onSessionChanged(session);
		}
	}

	private static String abbreviateForStatus(String s) {
		if (s == null)
			return "null";
		CRC32 crc = new CRC32();
		crc.update(s.getBytes());
		return String.format("Prompting... (%d/%d)", s.length(), crc.getValue());
	}

	

	@Override
	public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {
		AIAnswer answer = new AIAnswer(resp.id);
		answer.sessionId = resp.sessionId;
		answer.stats.inputToken = resp.stats.inputToken;
		answer.stats.outputToken = resp.stats.outputToken;
		answer.stats.reasoningToken = resp.stats.reasoningToken;
		answer.stats.cacheRead = resp.stats.cacheRead;
		answer.stats.cacheCreate = resp.stats.cacheCreate;
		answer.stats.totalToken = answer.stats.inputToken + answer.stats.cacheCreate + answer.stats.outputToken;
		answer.answer = resp.resultText;
		return answer;
	}

	
}
