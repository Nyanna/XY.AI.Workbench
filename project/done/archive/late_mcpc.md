Der Start von `startMcpcLaunch` in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java` kommt zu spät.
Er muss nach `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java` verlagert werden und in `isMCPCAvailable` nach dem ersten Fehlschlag greifen.

Erst wenn "MCPC" extern nicht erreicht wird soll initial einmalig der Launch Config zweig aufgerufen werden.
SystemInit:  id=8000783d-f5f5-4f77-b296-f252e25f859a, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: ffd4-1
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: startMcpcLaunch|isMCPCAvailable
    include:
    - '*.java'
    intent: read_lines
  reason: Locate usages of startMcpcLaunch and isMCPCAvailable
```
/answer ffd4-1 allow
Control Request:
```yaml
id: ffd4-2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      matches:
      - path: CCControlClient.java
        matches:
        - lineno: 60
          match: "public boolean isMCPCAvailable() {"
      - path: CCConnector.java
        matches:
        - lineno: 89
          match: "private boolean startMcpcLaunch(TimeStats ts) {"
        - lineno: 118
          match: if (ts.control(() -> controlClient.isMCPCAvailable()))
        - lineno: 178
          match: if (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !ts.control(() -> controlClient.isMCPCAvailable()) && !startMcpcLaunch(ts))
```
/answer ffd4-2 allow
Control Request:
```yaml
id: ffd4-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  reason: Read connector
```
/answer ffd4-3 allow
Control Request:
```yaml
id: ffd4-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: j77HfY|S3QA7e
        signature: import xy.ai.workbench.models.TimeStats;
      - id: 42oSWM|HQqmfJ
        signature: import org.eclipse.core.runtime.CoreException;
      - id: CCConnector
        signature: "public class CCConnector implements IAIConnector<CCRequest, CCResponse> {"
        children:
        - id: CCConnector.gZ1jsL|ajaBEH
          signature: private static final long CONTROL_CHECK_INTERVAL_MS = 300;
        - id: CCConnector.CCConnector
          signature: "public CCConnector(CCSessionManager sessionManager, IIncludeAdapter includeAdap…"
        - id: CCConnector.getSupportedKeyPattern
          signature: public KeyPattern getSupportedKeyPattern()
        - id: CCConnector.createRequest
          signature: "public CCRequest createRequest(Prompt prompt, IProgressMonitor mon)"
        - id: CCConnector.SessionRequestCallbacks
          signature: private static class SessionRequestCallbacks implements SessionCallbacks<String…
          children:
          - id: CCConnector.SessionRequestCallbacks.message
            signature: "public String message(Role role, String text)"
        - id: CCConnector.startMcpcLaunch
          signature: private boolean startMcpcLaunch(TimeStats ts)
        - id: CCConnector.executeRequest
          signature: "public CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job)"
        - id: CCConnector.readUntilResult
          signature: "private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressM…"
        - id: CCConnector.abbreviateForStatus
          signature: private static String abbreviateForStatus(String s)
        - id: CCConnector.convertResponse
          signature: "public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon)"
```
/answer ffd4-4 allow
Control Request:
```yaml
id: ffd4-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  reason: Read control client
```
/answer ffd4-5 allow
Control Request:
```yaml
id: ffd4-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: 7CTFqh|rOlOra
        signature: import java.io.IOException;
      - id: CCControlClient
        signature: /**  * Minimal HTTP client for the MCPC human-in-the-loop tool-control endpoint…
        docstring: Minimal HTTP client for the MCPC human-in-the-loop tool-control endpoint
        children:
        - id: CCControlClient.pWyigk|uaZQu4
          signature: private static final String CONTROL_URL = "http://localhost:9093/control/tool";
        - id: CCControlClient.checkControlEndpoint
          signature: "public void checkControlEndpoint(CCResponse resp, String sessionId)"
        - id: CCControlClient.toYaml
          signature: public String toYaml(JsonNode node)
        - id: CCControlClient.fromYaml
          signature: public JsonNode fromYaml(String text) throws JsonProcessingException
        - id: CCControlClient.isMCPCAvailable
          signature: public boolean isMCPCAvailable()
        - id: CCControlClient.poll
          signature: private ArrayNode poll(String sessionId)
        - id: CCControlClient.approve
          signature: public ArrayNode approve(String id)
        - id: CCControlClient.approve_2
          signature: "public ArrayNode approve(String id, String hint)"
        - id: CCControlClient.exec
          signature: "public ArrayNode exec(String id, String hint)"
        - id: CCControlClient.deny
          signature: "public ArrayNode deny(String id, String reason)"
        - id: CCControlClient.submitEdit
          signature: public boolean submitEdit(String yamlBlock)
        - id: CCControlClient.approvalNode
          signature: "private ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result,…"
        - id: CCControlClient.submit
          signature: private ArrayNode submit(ObjectNode approval)
        - id: CCControlClient.post
          signature: "private ArrayNode post(ObjectNode body, String sessionId)"
```
/answer ffd4-6 allow
Text:
 Bodies weren't shown; read by line.
Control Request:
```yaml
id: ffd4-7
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - CCConnector.startMcpcLaunch
    - CCConnector.executeRequest
    - CCConnector.CCConnector
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - CCControlClient.isMCPCAvailable
    - CCControlClient.checkControlEndpoint
  reason: Read relevant methods
```
/answer ffd4-7 allow
Control Request:
```yaml
id: ffd4-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.startMcpcLaunch
        type: MethodDeclaration
        code: |2
            private boolean startMcpcLaunch(TimeStats ts) {
              try {
                ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();
                for (ILaunch l : lm.getLaunches()) {
                  ILaunchConfiguration c = l.getLaunchConfiguration();
                  if (c != null && MCPC_LAUNCH_NAME.equals(c.getName()) && !l.isTerminated())
                    return false;
                }
                ILaunchConfiguration cfg = null;
                for (ILaunchConfiguration c : lm.getLaunchConfigurations()) {
                  if (MCPC_LAUNCH_NAME.equals(c.getName())) {
                    cfg = c;
                    break;
                  }
                }
                if (cfg == null) {
                  ILaunchConfigurationType type = lm.getLaunchConfigurationType("org.eclipse.ui.externaltools.ProgramLaunchConfigurationType");
                  ILaunchConfigurationWorkingCopy wc = type.newInstance(null, MCPC_LAUNCH_NAME);
                  String dir = "/home/user/xyan/xy.ai.workbench/mcpc";
                  wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LOCATION", "/usr/bin/bash");
                  wc.setAttribute("org.eclipse.ui.externaltools.ATTR_TOOL_ARGUMENTS", "-lc \"" + dir + "/start.sh\"");
                  wc.setAttribute("org.eclipse.ui.externaltools.ATTR_WORKING_DIRECTORY", dir);
                  wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LAUNCH_CONFIGURATION_BUILD_SCOPE", "${none}");
                  wc.setAttribute("org.eclipse.debug.core.environmentVariables", java.util.Map.of("TERM", "xterm-256color"));
                  cfg = wc.doSave();
                }
                cfg.launch(ILaunchManager.RUN_MODE, new NullProgressMonitor());
                long end = System.currentTimeMillis() + 3000;
                while (System.currentTimeMillis() < end) {
                  if (ts.control(() -> controlClient.isMCPCAvailable()))
                    return true;
                  Thread.sleep(250);
                }
              } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
              } catch (CoreException e) {
                LOG.error("Failed to start MCPC run configuration", e);
              }
              return false;
            }
      - id: CCConnector.executeRequest
        type: MethodDeclaration
        code: |2
            @Override
            public CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {
              SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 2);
              CCSession session = null;
              TimeStats ts = new TimeStats();
              ts.start();
              String relativeFilePath = req.arg.project.relativize(Paths.get(req.arg.absoluteFilePath)).toString();
              FrozenConfig fc = req.config;
              ClaudeSessionParameters params = new ClaudeSessionParameters(req.arg.project, fc.systemPrompt, fc.tools, fc.model, fc.reasoning, fc.profile, fc.keys, fc.cacheMode, relativeFilePath);
              params.setTitle(req.title);
              Command command = req.arg.command;
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
                  ts.control(() -> controlClient.approve(ac.parameter(0), ac.parameter(2)));
                else if (ac.action() == AnswerCommand.Action.Exec)
                  ts.control(() -> controlClient.exec(ac.parameter(0), ac.parameter(2)));
                else
                  ts.control(() -> controlClient.deny(ac.parameter(0), ac.parameter(2)));
                session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
              } else if (command instanceof CallEditCommand cec) {
                if (!ts.control(() -> controlClient.submitEdit(cec.yaml())))
                  throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
                session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
              } else if (command instanceof CallCommand) {
                if (req.arg.yaml == null || req.arg.yaml.isBlank())
                  throw new IllegalArgumentException("No preceding ```yaml block found before /call");
                if (!ts.control(() -> controlClient.submitEdit(req.arg.yaml)))
                  throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
                session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
              } else if (command == null) {
                sub.subTask("Acquiring session");
                session = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);
              } else
                throw new UnsupportedOperationException("Unsupported command: " + command.prefix());
              try {
                session.setInPrompt(true);
                if (command == null) {
                  if (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !ts.control(() -> controlClient.isMCPCAvailable()) && !startMcpcLaunch(ts))
                    throw new IllegalStateException("MCPC not reachable for AgentProfile");
                  sub.subTask("Sending prompt");
                  String promptJson = requestBuilder.buildPromptJson(req.promptText);
                  long t0 = System.nanoTime();
                  session.writeLine(promptJson);
                  ts.ioMs += (System.nanoTime() - t0) / 1_000_000;
                  ts.sent++;
                }
                sub.subTask("Waiting for answer");
                return readUntilResult(req, session, sub.split(1), job, ts);
              } catch (IOException e) {
                throw new IllegalStateException("Claude Code CLI error", e);
              } finally {
                session.setInPrompt(false);
              }
            }
      - id: CCConnector.CCConnector
        type: ConstructorDeclaration
        code: |2
            public CCConnector(CCSessionManager sessionManager, IIncludeAdapter includeAdapter) {
              this.sessionManager = sessionManager;
              this.includeProcessor = new IncludeProcessor<>(includeAdapter);
            }
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: CCControlClient.isMCPCAvailable
        type: MethodDeclaration
        code: |2
            public boolean isMCPCAvailable() {
              // never null, just check exceptions
              return poll(null) != null;
            }
      - id: CCControlClient.checkControlEndpoint
        type: MethodDeclaration
        code: |2
            public void checkControlEndpoint(CCResponse resp, String sessionId) {
              JsonNode pending = poll(sessionId);
              if (pending.isEmpty())
                return;
              StringBuilder res = new StringBuilder();
              ProtocolParser.appendEvents(resp.events, res);
              JsonNode first = pending.get(0);
              res.append(String.format("%s\n```yaml\n%s\n```\n%s %s allow", EditorInterface.CONTROL_REQUEST, toYaml(first), AnswerCommand.CMD_ANSWER, first.path("id").asText()));
              resp.resultText = res.toString();
              resp.showStats = false;
            }
```
/answer ffd4-8 allow
ReasoningToken: 43
Control Request:
```yaml
id: ffd4-9
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: MCPC_LAUNCH_NAME|LOG\b.*=|^import .*(debug|NullProgress|CoreException|Logger|LoggerFactory)|DebugPlugin|ILaunch|NullProgressMonitor
    include:
    - CCConnector.java
    - CCControlClient.java
    intent: read_lines
  reason: Find imports/constants to move
```
/answer ffd4-9 allow
Control Request:
```yaml
id: ffd4-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      matches:
      - path: CCConnector.java
        matches:
        - lineno: 27
          match: import org.eclipse.core.runtime.CoreException;
        - lineno: 28
          match: import org.eclipse.core.runtime.NullProgressMonitor;
        - lineno: 29
          match: import org.eclipse.debug.core.DebugPlugin;
        - lineno: 30
          match: import org.eclipse.debug.core.ILaunch;
        - lineno: 31
          match: import org.eclipse.debug.core.ILaunchConfiguration;
        - lineno: 32
          match: import org.eclipse.debug.core.ILaunchConfigurationType;
        - lineno: 33
          match: import org.eclipse.debug.core.ILaunchConfigurationWorkingCopy;
        - lineno: 34
          match: import org.eclipse.debug.core.ILaunchManager;
        - lineno: 43
          match: private static final String MCPC_LAUNCH_NAME = "MCPC";
        - lineno: 91
          match: ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();
        - lineno: 92
          match: "for (ILaunch l : lm.getLaunches()) {"
        - lineno: 93
          match: ILaunchConfiguration c = l.getLaunchConfiguration();
        - lineno: 94
          match: if (c != null && MCPC_LAUNCH_NAME.equals(c.getName()) && !l.isTerminated())
        - lineno: 97
          match: ILaunchConfiguration cfg = null;
        - lineno: 98
          match: "for (ILaunchConfiguration c : lm.getLaunchConfigurations()) {"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer ffd4-a allow
Control Request:
```yaml
id: ffd4-b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - CCControlClient.poll
    - CCControlClient.pWyigk|uaZQu4
  reason: Read poll
```
/answer ffd4-b allow
Control Request:
```yaml
id: ffd4-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: CCControlClient.poll
        type: MethodDeclaration
        code: |2
            private ArrayNode poll(String sessionId) {
              return post(mapper.createObjectNode(), sessionId);
            }
      - id: CCControlClient.pWyigk|uaZQu4
        type: StatementGroup
        code: |2
            private static final String CONTROL_URL = "http://localhost:9093/control/tool";
            private static final Duration TIMEOUT = Duration.ofSeconds(5);
            private final ObjectMapper mapper = JsonUtil.mapper();
            private YamlRenderer yaml = new YamlRenderer();
            private final HttpClient http = HttpClient.newBuilder().connectTimeout(TIMEOUT).build();
```
/answer ffd4-c allow
ReasoningToken: 31
Control Request:
```yaml
id: ffd4-d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: \bLOG\b|MCPC_LAUNCH_NAME|NullProgressMonitor|CoreException|ILaunch|DebugPlugin
    include:
    - CCConnector.java
    limit: 50
    intent: read_lines
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: ^import|\bLOG\b|class CCControlClient
    include:
    - CCControlClient.java
    limit: 50
    intent: read_lines
  reason: Check LOG definition and imports
```
/answer ffd4-d allow
Control Request:
```yaml
id: ffd4-e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      matches:
      - path: CCConnector.java
        matches:
        - lineno: 14
          match: import xy.ai.workbench.LOG;
        - lineno: 27
          match: import org.eclipse.core.runtime.CoreException;
        - lineno: 28
          match: import org.eclipse.core.runtime.NullProgressMonitor;
        - lineno: 29
          match: import org.eclipse.debug.core.DebugPlugin;
        - lineno: 30
          match: import org.eclipse.debug.core.ILaunch;
        - lineno: 31
          match: import org.eclipse.debug.core.ILaunchConfiguration;
        - lineno: 32
          match: import org.eclipse.debug.core.ILaunchConfigurationType;
        - lineno: 33
          match: import org.eclipse.debug.core.ILaunchConfigurationWorkingCopy;
        - lineno: 34
          match: import org.eclipse.debug.core.ILaunchManager;
        - lineno: 43
          match: private static final String MCPC_LAUNCH_NAME = "MCPC";
        - lineno: 91
          match: ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();
        - lineno: 92
          match: "for (ILaunch l : lm.getLaunches()) {"
        - lineno: 93
          match: ILaunchConfiguration c = l.getLaunchConfiguration();
        - lineno: 94
          match: if (c != null && MCPC_LAUNCH_NAME.equals(c.getName()) && !l.isTerminated())
        - lineno: 97
          match: ILaunchConfiguration cfg = null;
        - lineno: 98
          match: "for (ILaunchConfiguration c : lm.getLaunchConfigurations()) {"
        - lineno: 99
          match: "if (MCPC_LAUNCH_NAME.equals(c.getName())) {"
        - lineno: 105
          match: ILaunchConfigurationType type = lm.getLaunchConfigurationType("org.eclipse.ui.externaltools.ProgramLaunchConfigurationType");
        - lineno: 106
          match: "ILaunchConfigurationWorkingCopy wc = type.newInstance(null, MCPC_LAUNCH_NAME);"
        - lineno: 115
          match: "cfg.launch(ILaunchManager.RUN_MODE, new NullProgressMonitor());"
        - lineno: 124
          match: "} catch (CoreException e) {"
        - lineno: 125
          match: "LOG.error(\"Failed to start MCPC run configuration\", e);"
        - lineno: 213
          match: "LOG.error(\"CLI stderr: \" + errLine);"
        - lineno: 218
          match: "LOG.error(\"Error reading stderr: \", ex2);"
        - lineno: 233
          match: "LOG.error(\"Error reading line: \", ex);"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      matches:
      - path: CCControlClient.java
        matches:
        - lineno: 3
          match: import java.io.IOException;
        - lineno: 4
          match: import java.net.URI;
        - lineno: 5
          match: import java.net.URLEncoder;
        - lineno: 6
          match: import java.net.http.HttpClient;
        - lineno: 7
          match: import java.net.http.HttpRequest;
        - lineno: 8
          match: import java.net.http.HttpRequest.BodyPublishers;
        - lineno: 9
          match: import java.net.http.HttpResponse;
        - lineno: 10
          match: import java.net.http.HttpResponse.BodyHandlers;
        - lineno: 11
          match: import java.nio.charset.StandardCharsets;
        - lineno: 12
          match: import java.time.Duration;
        - lineno: 13
          match: import com.fasterxml.jackson.core.JsonProcessingException;
        - lineno: 14
          match: import com.fasterxml.jackson.databind.JsonNode;
        - lineno: 15
          match: import com.fasterxml.jackson.databind.ObjectMapper;
        - lineno: 16
          match: import com.fasterxml.jackson.databind.node.ArrayNode;
        - lineno: 17
          match: import com.fasterxml.jackson.databind.node.ObjectNode;
        - lineno: 18
          match: import xy.ai.workbench.EditorInterface;
        - lineno: 19
          match: import xy.ai.workbench.LOG;
        - lineno: 20
          match: import xy.ai.workbench.commands.AnswerCommand;
        - lineno: 33
          match: "public class CCControlClient {"
        - lineno: 131
          match: LOG.error("control endpoint returned status " + response.statusCode());
        - lineno: 136
          match: LOG.error("control endpoint returned an empty body");
        - lineno: 143
          match: "LOG.error(\"Control endpoint unreachable\", e);"
```
/answer ffd4-e allow
ReasoningToken: 29
Control Request:
```yaml
id: ffd4-f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - CCControlClient.post
  reason: Understand failure behavior
```
/answer ffd4-f allow
Control Request:
```yaml
id: ffd4-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: CCControlClient.post
        type: MethodDeclaration
        code: |2
            private ArrayNode post(ObjectNode body, String sessionId) {
              try {
                String json = JsonUtil.write(body);
                String url = sessionId == null ? CONTROL_URL : CONTROL_URL + "?session=" + URLEncoder.encode(sessionId, StandardCharsets.UTF_8);
                HttpRequest request = HttpRequest.newBuilder().uri(URI.create(url)).timeout(TIMEOUT).header("Content-Type", "application/json").POST(BodyPublishers.ofString(json)).build();
                HttpResponse<String> response = http.send(request, BodyHandlers.ofString());
                if (response.statusCode() / 100 != 2) {
                  LOG.error("control endpoint returned status " + response.statusCode());
                  return mapper.createArrayNode();
                }
                String responseBody = response.body();
                if (responseBody == null || responseBody.isBlank()) {
                  LOG.error("control endpoint returned an empty body");
                  return mapper.createArrayNode();
                }
                JsonNode root = JsonUtil.readTree(responseBody);
                JsonNode pending = root.path("pending");
                return pending.isArray() ? (ArrayNode) pending : mapper.createArrayNode();
              } catch (IOException | InterruptedException e) {
                LOG.error("Control endpoint unreachable", e);
                if (e instanceof InterruptedException)
                  Thread.currentThread().interrupt();
                throw new IllegalStateException("Error on control endpoint", e);
              }
            }
```
/answer ffd4-10 allow
ReasoningToken: 308
Control Request:
```yaml
id: ffd4-11
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.isMCPCAvailable
    source: |2-
        public boolean isMCPCAvailable() {
          if (ping())
            return true;
          synchronized (this) {
            if (launchAttempted)
              return false;
            launchAttempted = true;
          }
          return startMcpcLaunch();
        }

        private boolean ping() {
          try {
            return poll(null) != null;
          } catch (IllegalStateException e) {
            return false;
          }
        }

        private boolean startMcpcLaunch() {
          try {
            ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();
            for (ILaunch l : lm.getLaunches()) {
              ILaunchConfiguration c = l.getLaunchConfiguration();
              if (c != null && MCPC_LAUNCH_NAME.equals(c.getName()) && !l.isTerminated())
                return false;
            }
            ILaunchConfiguration cfg = null;
            for (ILaunchConfiguration c : lm.getLaunchConfigurations()) {
              if (MCPC_LAUNCH_NAME.equals(c.getName())) {
                cfg = c;
                break;
              }
            }
            if (cfg == null) {
              ILaunchConfigurationType type = lm.getLaunchConfigurationType("org.eclipse.ui.externaltools.ProgramLaunchConfigurationType");
              ILaunchConfigurationWorkingCopy wc = type.newInstance(null, MCPC_LAUNCH_NAME);
              String dir = "/home/user/xyan/xy.ai.workbench/mcpc";
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LOCATION", "/usr/bin/bash");
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_TOOL_ARGUMENTS", "-lc \"" + dir + "/start.sh\"");
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_WORKING_DIRECTORY", dir);
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LAUNCH_CONFIGURATION_BUILD_SCOPE", "${none}");
              wc.setAttribute("org.eclipse.debug.core.environmentVariables", java.util.Map.of("TERM", "xterm-256color"));
              cfg = wc.doSave();
            }
            cfg.launch(ILaunchManager.RUN_MODE, new NullProgressMonitor());
            long end = System.currentTimeMillis() + 3000;
            while (System.currentTimeMillis() < end) {
              if (ping())
                return true;
              Thread.sleep(250);
            }
          } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
          } catch (CoreException e) {
            LOG.error("Failed to start MCPC run configuration", e);
          }
          return false;
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.pWyigk|uaZQu4
    source: |2-
        private static final String CONTROL_URL = "http://localhost:9093/control/tool";
        private static final String MCPC_LAUNCH_NAME = "MCPC";
        private static final Duration TIMEOUT = Duration.ofSeconds(5);
        private final ObjectMapper mapper = JsonUtil.mapper();
        private YamlRenderer yaml = new YamlRenderer();
        private final HttpClient http = HttpClient.newBuilder().connectTimeout(TIMEOUT).build();
        private boolean launchAttempted;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.startMcpcLaunch
    source: "  private static final class Removed {}"
  reason: Move launch logic into control client; remove from connector (placeholder removed next)
```
/answer ffd4-11 allow
Control Request:
```yaml
id: ffd4-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      result: success
      id: CCControlClient.pWyigk|uaZQu4
      new_id: CCControlClient.uR3OCX|fnJ3Z3
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      result: success
      id: CCConnector.startMcpcLaunch
      new_id: CCConnector.Removed
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      error: |
        invalid member declaration: Encountered unexpected token: "private" "private"
            at line 12, column 3.

        Was expecting:

            <EOF>
      id: CCControlClient.isMCPCAvailable
```
/answer ffd4-12 allow
Control Request:
```yaml
id: ffd4-13
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.Removed
  reason: Remove placeholder
```
/answer ffd4-13 allow
ReasoningToken: 57
Control Request:
```yaml
id: ffd4-14
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.isMCPCAvailable
    position: after
    source: |2-
        private boolean ping() {
          try {
            return poll(null) != null;
          } catch (IllegalStateException e) {
            return false;
          }
        }
  reason: Add ping
```
/answer ffd4-14 allow
Control Request:
```yaml
id: ffd4-15
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.ping
    position: after
    source: |2-
        private boolean startMcpcLaunch() {
          try {
            ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();
            for (ILaunch l : lm.getLaunches()) {
              ILaunchConfiguration c = l.getLaunchConfiguration();
              if (c != null && MCPC_LAUNCH_NAME.equals(c.getName()) && !l.isTerminated())
                return false;
            }
            ILaunchConfiguration cfg = null;
            for (ILaunchConfiguration c : lm.getLaunchConfigurations()) {
              if (MCPC_LAUNCH_NAME.equals(c.getName())) {
                cfg = c;
                break;
              }
            }
            if (cfg == null) {
              ILaunchConfigurationType type = lm.getLaunchConfigurationType("org.eclipse.ui.externaltools.ProgramLaunchConfigurationType");
              ILaunchConfigurationWorkingCopy wc = type.newInstance(null, MCPC_LAUNCH_NAME);
              String dir = "/home/user/xyan/xy.ai.workbench/mcpc";
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LOCATION", "/usr/bin/bash");
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_TOOL_ARGUMENTS", "-lc \"" + dir + "/start.sh\"");
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_WORKING_DIRECTORY", dir);
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LAUNCH_CONFIGURATION_BUILD_SCOPE", "${none}");
              wc.setAttribute("org.eclipse.debug.core.environmentVariables", java.util.Map.of("TERM", "xterm-256color"));
              cfg = wc.doSave();
            }
            cfg.launch(ILaunchManager.RUN_MODE, new NullProgressMonitor());
            long end = System.currentTimeMillis() + 3000;
            while (System.currentTimeMillis() < end) {
              if (ping())
                return true;
              Thread.sleep(250);
            }
          } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
          } catch (CoreException e) {
            LOG.error("Failed to start MCPC run configuration", e);
          }
          return false;
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.uR3OCX|fnJ3Z3
    position: after
    source: '  private static final String UNUSED_MARKER = "";'
  reason: Add launch method (second item is removed next)
```
/answer ffd4-15 allow
Control Request:
```yaml
id: ffd4-16
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.153g8X|1MyiBK
  reason: Remove accidental marker field
```
/answer ffd4-16 allow
Control Request:
```yaml
id: ffd4-17
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.isMCPCAvailable
    source: |2-
        public boolean isMCPCAvailable() {
          if (ping())
            return true;
          synchronized (this) {
            if (launchAttempted)
              return false;
            launchAttempted = true;
          }
          return startMcpcLaunch();
        }
  reason: Launch once after first failure
```
/answer ffd4-17 allow
ReasoningToken: 43
Control Request:
```yaml
id: ffd4-18
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: 7CTFqh|rOlOra
    position: before
    source: |-
      import org.eclipse.core.runtime.CoreException;
      import org.eclipse.core.runtime.NullProgressMonitor;
      import org.eclipse.debug.core.DebugPlugin;
      import org.eclipse.debug.core.ILaunch;
      import org.eclipse.debug.core.ILaunchConfiguration;
      import org.eclipse.debug.core.ILaunchConfigurationType;
      import org.eclipse.debug.core.ILaunchConfigurationWorkingCopy;
      import org.eclipse.debug.core.ILaunchManager;
  reason: Add imports
```
/answer ffd4-18 allow
Control Request:
```yaml
id: ffd4-19
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: '!ts.control(() -> controlClient.isMCPCAvailable()) && !startMcpcLaunch(ts))'
    new_text: '!ts.control(() -> controlClient.isMCPCAvailable()))'
  reason: Drop connector launch call
```
/answer ffd4-19 allow
Control Request:
```yaml
id: ffd4-1a
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  text: MCPC_LAUNCH_NAME
  reason: Find leftover constant
```
/answer ffd4-1a allow
Control Request:
```yaml
id: ffd4-1b
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.gZ1jsL|ajaBEH
        code: |2
            private static final long CONTROL_CHECK_INTERVAL_MS = 300;
            private static final long IDLE_WAIT_MS = 50;
            private static final String MCPC_LAUNCH_NAME = "MCPC";
            private final CCRequestBuilder requestBuilder = new CCRequestBuilder();
            private final ProtocolParser jsonParser = new ProtocolParser();
            private final CCControlClient controlClient = new CCControlClient();
            private final SessionCallbacks<String> callbacks = new SessionRequestCallbacks();
            private final CCSessionManager sessionManager;
            private final IncludeProcessor<String> includeProcessor;
```
/answer ffd4-1b allow
Control Request:
```yaml
id: ffd4-1c
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.gZ1jsL|ajaBEH
    old_text: |2
        private static final String MCPC_LAUNCH_NAME = "MCPC";
    new_text: ""
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: 42oSWM|HQqmfJ
    old_text: import org.eclipse.core.runtime.CoreException;
    new_text: ""
  reason: Remove unused constant and import
```
/answer ffd4-1c allow
Control Request:
```yaml
id: ffd4-1d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: NullProgressMonitor|DebugPlugin|ILaunch|CoreException
    include:
    - CCConnector.java
    intent: read_lines
  reason: Find unused imports in connector
```
/answer ffd4-1d allow
Control Request:
```yaml
id: ffd4-1e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      matches:
      - path: CCConnector.java
        matches:
        - lineno: 27
          match: import org.eclipse.core.runtime.NullProgressMonitor;
        - lineno: 28
          match: import org.eclipse.debug.core.DebugPlugin;
        - lineno: 29
          match: import org.eclipse.debug.core.ILaunch;
        - lineno: 30
          match: import org.eclipse.debug.core.ILaunchConfiguration;
        - lineno: 31
          match: import org.eclipse.debug.core.ILaunchConfigurationType;
        - lineno: 32
          match: import org.eclipse.debug.core.ILaunchConfigurationWorkingCopy;
        - lineno: 33
          match: import org.eclipse.debug.core.ILaunchManager;
```
/answer ffd4-1e allow
Text:
 Remove the now-unused imports.
Control Request:
```yaml
id: ffd4-1f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  regexp: import org\.eclipse\.(debug\.core|core\.runtime\.NullProgressMonitor)
  reason: Get import node ids
```
/answer ffd4-1f allow
Control Request:
```yaml
id: ffd4-20
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: 42oSWM|K6SOVU
        code: |
          import org.eclipse.core.runtime.NullProgressMonitor;
          import org.eclipse.debug.core.DebugPlugin;
          import org.eclipse.debug.core.ILaunch;
          import org.eclipse.debug.core.ILaunchConfiguration;
          import org.eclipse.debug.core.ILaunchConfigurationType;
          import org.eclipse.debug.core.ILaunchConfigurationWorkingCopy;
          import org.eclipse.debug.core.ILaunchManager;
          import xy.ai.workbench.connector.harness.IIncludeAdapter;
          import xy.ai.workbench.connector.harness.IncludeProcessor;
          import xy.ai.workbench.models.AIAnswer;
          import java.util.List;
```
/answer ffd4-20 allow
Control Request:
```yaml
id: ffd4-21
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: 42oSWM|K6SOVU
    start_marker: import org.eclipse.core.runtime.NullProgressMonitor;
    end_marker: import org.eclipse.debug.core.ILaunchManager;
    content: ""
  reason: Remove unused imports
```
/answer ffd4-21 allow
Control Request:
```yaml
id: ffd4-22
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  reason: Syntax check
```
/answer ffd4-22 allow
Result Stats: id=8000783d-f5f5-4f77-b296-f252e25f859a, total: 34517, in: 28, out: 8367, reason: 0, read: 213560, write: 26122, start: 1792778423550, end: 1792778428846, time: 5234, total: 101906, io: 0, control: 62, sent: 0, recv: 120
Der Launch-Start liegt jetzt in `CCControlClient`. Beide Dateien bestehen die Syntaxprüfung, übersetzt oder ausgeführt habe ich sie nicht.

- **`isMCPCAvailable()`:** Es prüft zuerst per `ping()`, ob der Endpunkt antwortet. Eine `IllegalStateException` zählt dabei als „nicht erreichbar“. Schlägt das fehl, wird einmalig `startMcpcLaunch()` aufgerufen. Das verhindert ein Flag `launchAttempted`, das gegen Parallelzugriff geschützt ist. Spätere Fehlschläge geben direkt `false` zurück.
- **`startMcpcLaunch()`:** Ich habe die Logik unverändert übernommen, auch die 3 Sekunden Wartezeit mit Polling alle 250 ms. Statt `controlClient.isMCPCAvailable()` ruft es jetzt `ping()` auf, sonst würde es sich selbst rekursiv aufrufen. `MCPC_LAUNCH_NAME` und die nötigen Imports sind mit umgezogen.
- **`CCConnector`:** `startMcpcLaunch`, die Konstante `MCPC_LAUNCH_NAME` und die nicht mehr benötigten Imports sind entfernt. Die Prüfung in `executeRequest` lautet jetzt nur noch `!ts.control(() -> controlClient.isMCPCAvailable())`.

Ein laufender, aber noch nicht erreichbarer MCPC-Launch führt weiterhin zu `false`. Das war schon vorher so.
