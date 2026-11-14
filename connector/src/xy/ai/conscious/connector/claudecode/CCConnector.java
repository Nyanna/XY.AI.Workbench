package xy.ai.conscious.connector.claudecode;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;
import java.util.stream.Collectors;
import java.util.zip.CRC32;
import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.api.ext.IMCPCLauncher;
import xy.ai.conscious.commands.AnswerCommand;
import xy.ai.conscious.commands.CallCommand;
import xy.ai.conscious.commands.Command;
import xy.ai.conscious.commands.ExitCommand;
import xy.ai.conscious.commands.ResumeCommand;
import xy.ai.conscious.connector.IAIConnector;
import xy.ai.conscious.connector.harness.FrozenConfig;
import xy.ai.conscious.connector.harness.IIncludeAdapter;
import xy.ai.conscious.connector.harness.IncludeProcessor;
import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.connector.harness.Role;
import xy.ai.conscious.connector.harness.SessionCallbacks;
import xy.ai.conscious.core.AgentProfile;
import xy.ai.conscious.core.LOG;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.TimeStats;
import xy.ai.conscious.model.Model.KeyPattern;

public class CCConnector implements IAIConnector<CCRequest, CCResponse> {
  private static final int DEAD_TIMEOUT = 180_000;
  private static final long CONTROL_CHECK_INTERVAL_MS = 300;
  private static final long IDLE_WAIT_MS = 50;
  private final CCRequestBuilder requestBuilder = new CCRequestBuilder();
  private final ProtocolParser jsonParser = new ProtocolParser();
  private final CCControlClient controlClient;
  private final SessionCallbacks<String> callbacks = new SessionRequestCallbacks();
  private final CCSessionManager sessionManager;
  private final IncludeProcessor<String> includeProcessor;

  public CCConnector(CCSessionManager sessionManager, IIncludeAdapter includeAdapter, IMCPCLauncher mcpc) {
    this.sessionManager = sessionManager;
    this.includeProcessor = new IncludeProcessor<>(includeAdapter);
    this.controlClient = new CCControlClient(mcpc);
  }

  @Override
  public KeyPattern getSupportedKeyPattern() {
    return KeyPattern.ClaudeCode;
  }

  @Override
  public CCRequest createRequest(Prompt prompt, IProgress mon) {
    IProgress sub = mon.setTask("Create request", 1);
    Command command = prompt.arg.command;
    String promptText = null;
    if (command == null) {
      List<String> out = new ArrayList<>();
      for (String input : prompt.inputs) {
        if (input != null && !input.isBlank())
          includeProcessor.processText(input, List.of(), callbacks, out, includeProcessor);
      }
      promptText = out.stream().map(String::strip).filter(s -> !s.isBlank()).collect(Collectors.joining("\n"));
      if (promptText.isEmpty())
        throw new IllegalStateException("Prompt text is empty and no command");
    }
    sub.worked();
    String title = promptText == null ? null : promptText.substring(0, Math.min(100, promptText.length())).replace('\n', ' ');
    return new CCRequest(UUID.randomUUID().toString(), title, prompt.config, promptText, prompt.arg);
  }

  private static class SessionRequestCallbacks implements SessionCallbacks<String> {

    @Override
    public String message(Role role, String text) {
      return text;
    }
  }

  @Override
  public CCResponse executeRequest(CCRequest req, IProgress mon) {
    IProgress sub = mon.setTask("Executing prompt", 2);
    CCSession session = null;
    TimeStats ts = new TimeStats();
    ts.start();
    String relativeFilePath = req.arg.project.relativize(Paths.get(req.arg.absoluteFilePath)).toString();
    FrozenConfig fc = req.config;
    CCSessionParameters params = new CCSessionParameters(req.arg.project, fc.systemPrompt, fc.tools, fc.model, fc.reasoning, fc.profile, fc.keys, fc.cacheMode, relativeFilePath);
    params.setTitle(req.title);
    Command command = req.arg.command;
    if (command instanceof ResumeCommand rc) {
      sub.setTaskName("Importing session");
      sessionManager.importSession(rc.parameter(0), params);
      return new CCResponse(req.id, "Session created");
    }
    if (command instanceof ExitCommand) {
      session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
      sub.setTaskName("Terminating CLI process");
      session.terminate();
      return new CCResponse(req.id, "Session closed!");
    }
    if (command instanceof AnswerCommand ac) {
      ts.control(() -> controlClient.answer(ac.parameter(0), ac.action(), ac.parameter(2), ac.yaml()));
      session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
    } else if (command instanceof CallCommand) {
      if (req.arg.yaml == null || req.arg.yaml.isBlank())
        throw new IllegalArgumentException("No preceding ```yaml block found before /call");
      if (!ts.control(() -> controlClient.submitEdit(req.arg.yaml)))
        throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
      session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
    } else if (command == null) {
      sub.setTaskName("Acquiring session");
      session = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);
    } else
      throw new UnsupportedOperationException("Unsupported command: " + command.prefix());
    try {
      session.setInPrompt(true);
      if (command == null) {
        if (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !ts.control(() -> controlClient.isMCPCAvailable()))
          throw new IllegalStateException("MCPC not reachable for AgentProfile");
        sub.setTaskName("Sending prompt");
        String promptJson = requestBuilder.buildPromptJson(req.promptText);
        long t0 = System.nanoTime();
        session.writeLine(promptJson);
        ts.ioMs += (System.nanoTime() - t0) / 1_000_000;
        ts.sent++;
      }
      sub.setTaskName("Waiting for answer");
      return readUntilResult(req, session, sub.subProgress(), ts);
    } catch (IOException e) {
      throw new IllegalStateException("Claude Code CLI error", e);
    } finally {
      session.setInPrompt(false);
    }
  }

  private CCResponse readUntilResult(CCRequest req, CCSession session, IProgress mon, TimeStats ts) throws IOException {
    IProgress sub = mon.setTask("Reading Claude output", 10000);
    CCResponse resp = new CCResponse(req.id);
    String line;
    long lastDataAt = System.nanoTime();
    long lastControlAt = 0;
    boolean errDead = false;
    while (true) {
      long before = lastDataAt;
      // Sources are probed without blocking; a source is only read if it has data.
      // lastDataAt is set on receipt and after processing, so processing time never counts against the limit.
      if (!errDead)
        try {
          if (session.hasError()) {
            String errLine = ts.io(() -> session.readError());
            if (errLine != null) {
              lastDataAt = System.nanoTime();
              LOG.error("CLI stderr: " + errLine);
              resp.resultText = (resp.resultText == null ? "" : resp.resultText + "\n") + errLine;
            }
          }
        } catch (Exception ex2) {
          LOG.error("Error reading stderr: ", ex2);
          // avoid a hot loop on a broken stream
          errDead = true;
        }
      try {
        if (session.hasOutput() && (line = ts.io(() -> session.readLine())) != null) {
          lastDataAt = System.nanoTime();
          ts.received++;
          session.setLastRawLine(line);
          IProgress monInner = mon.setTask(abbreviateForStatus(line), 1);
          monInner.worked();
          jsonParser.parseLine(resp, session, sub, line);
          lastDataAt = System.nanoTime();
        }
      } catch (Exception ex) {
        LOG.error("Error reading line: ", ex);
        resp.resultText = ex.getMessage();
        lastDataAt = System.nanoTime();
      }
      boolean idle = lastDataAt == before;
      if (idle && !resp.isReady()) {
        // Check control endpoint (HTTP), rate limited to limit CPU spent on state lookups
        long now = System.currentTimeMillis();
        if (now - lastControlAt >= CONTROL_CHECK_INTERVAL_MS) {
          lastControlAt = now;
          ts.control(() -> {
            controlClient.checkControlEndpoint(resp, session.getID());
            return null;
          });
        }
        if (!resp.isReady()) {
          long silentMs = (System.nanoTime() - lastDataAt) / 1_000_000;
          if (silentMs > DEAD_TIMEOUT)
            throw new IOException("Stuck detection: no data from STDOUT/STDERR for " + silentMs / 1000 + " seconds (session " + session.getID() + ", received=" + ts.received + ", lastLine=" + abbreviateForStatus(String.valueOf(session.getLastRawLine())) + ")");
          try {
            Thread.sleep(IDLE_WAIT_MS);
          } catch (InterruptedException ie) {
            Thread.currentThread().interrupt();
            throw new IOException("Interrupted while waiting for Claude Code output (session " + session.getID() + ")", ie);
          }
        }
      }
      if (resp.isReady()) {
        resp.sessionId = session.getID();
        ts.stop();
        session.time.add(ts);
        resp.time.add(ts);
        resp.time.totalMs = session.time.totalMs;
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
  public AIAnswer convertResponse(CCResponse resp, IProgress mon) {
    AIAnswer answer = new AIAnswer(resp.id);
    answer.showStats = resp.showStats;
    answer.sessionId = resp.sessionId;
    answer.stats.inputToken = resp.stats.inputToken;
    answer.stats.outputToken = resp.stats.outputToken;
    answer.stats.reasoningToken = resp.stats.reasoningToken;
    answer.stats.cacheRead = resp.stats.cacheRead;
    answer.stats.cacheCreate = resp.stats.cacheCreate;
    answer.stats.totalToken = answer.stats.inputToken + answer.stats.cacheCreate + answer.stats.outputToken;
    answer.time.add(resp.time);
    answer.time.totalMs = resp.time.totalMs;
    answer.answer = resp.resultText;
    return answer;
  }
}
