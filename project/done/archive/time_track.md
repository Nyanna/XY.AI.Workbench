Ich brauche für Performance Messung ein Time Tracking analog zu TokenStats in "Result Stats".
Gemessen werde soll:

- Startzeitpunkt des letzten Prompts
- Endzeitpunkt, Anzeige in Laufzeit in Millisekunden
- Komulative Laufzeit aller Prompts in der Session
- Extra gemessen werden soll die reine IO-Zeit, sowie Anzahl gesendeter und empfangener Nachrichten
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java` soll die Zeit für den Control Client nicht enthalten und separat gemessen werden. 

`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TokenStats.java`
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java`
SystemInit:  id=03f10352-6ac0-4ee4-9ff1-33d37fab23ce, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 8fc6-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TokenStats.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  reason: Understand structure of the stats classes and control client
```
/answer 8fc6-1 allow
Control Request:
```yaml
id: 8fc6-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TokenStats.java
      nodes:
      - id: uUL6Qs|J3MY6H
        signature: import java.util.regex.Matcher;
      - id: TokenStats
        signature: "public class TokenStats {"
        children:
        - id: TokenStats.tV0ZpA|aTVvOy
          signature: public long inputToken;
        - id: TokenStats.toString
          signature: public String toString()
        - id: TokenStats.jxwqEb|DF2IZE
          signature: private static final Pattern PATTERN = Pattern
        - id: TokenStats.print
          signature: public String print()
        - id: TokenStats.fromString
          signature: public static TokenStats fromString(String s)
          docstring: "Parses stats previously produced by {@link #print()}, or {@code null} when"
        - id: TokenStats.add
          signature: public void add(TokenStats stats)
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
      nodes:
      - id: zOnqYA|j8mszO
        signature: import java.util.regex.Matcher;
      - id: AIAnswer
        signature: "public class AIAnswer {"
        children:
        - id: AIAnswer.j6EUkY|f7JitM
          signature: "public static final String RESULT = \"Result Stats: \";"
        - id: AIAnswer.AIAnswer
          signature: public AIAnswer(String id)
        - id: AIAnswer.toString
          signature: public String toString()
        - id: AIAnswer.print
          signature: public String print()
        - id: AIAnswer.fromString
          signature: public static AIAnswer fromString(String line)
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
/answer 8fc6-2 allow
Control Request:
```yaml
id: 8fc6-3
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: TokenStats
    include:
    - '*.java'
    limit: 50
  reason: Find usages of TokenStats
```
/answer 8fc6-3 allow
Control Request:
```yaml
id: 8fc6-4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/models/TokenStats.java
        matches:
        - lineno: 6
          match: "public class TokenStats {"
        - lineno: 17
          match: "\"TokenStats [inputToken=%s, outputToken=%s, reasoningToken=%s, totalInToken=%s, cacheRead=%s, cacheCreate=%s]\","
        - lineno: 33
          match: "public static TokenStats fromString(String s) {"
        - lineno: 39
          match: TokenStats stats = new TokenStats();
        - lineno: 49
          match: "public void add(TokenStats stats) {"
      - path: xy/ai/workbench/models/AIAnswer.java
        matches:
        - lineno: 15
          match: public final TokenStats stats = new TokenStats();
        - lineno: 44
          match: TokenStats stats = TokenStats.fromString(line.substring(idMatcher.end()));
      - path: xy/ai/workbench/editor/outline/filter/TokenStatsFilter.java
        matches:
        - lineno: 7
          match: "public class TokenStatsFilter extends ElementFilter {"
      - path: xy/ai/workbench/connector/mcp/MCPResponse.java
        matches:
        - lineno: 4
          match: import xy.ai.workbench.models.TokenStats;
        - lineno: 9
          match: public final TokenStats stats = new TokenStats();
      - path: xy/ai/workbench/connector/claudecode/CCSession.java
        matches:
        - lineno: 16
          match: import xy.ai.workbench.models.TokenStats;
        - lineno: 40
          match: public final TokenStats stats = new TokenStats();
      - path: xy/ai/workbench/connector/claudecode/CCResponse.java
        matches:
        - lineno: 6
          match: import xy.ai.workbench.models.TokenStats;
        - lineno: 22
          match: public final TokenStats stats = new TokenStats();
      - path: xy/ai/workbench/connector/claudecode/ProtocolParser.java
        matches:
        - lineno: 18
          match: import xy.ai.workbench.models.TokenStats;
        - lineno: 114
          match: TokenStats stats = new TokenStats();
```
/answer 8fc6-4 allow
Control Request:
```yaml
id: 8fc6-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TokenStats.java
    ids:
    - TokenStats
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
    ids:
    - AIAnswer
  reason: Read both classes
```
/answer 8fc6-5 allow
Control Request:
```yaml
id: 8fc6-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TokenStats.java
      nodes:
      - id: TokenStats
        type: ClassOrInterfaceDeclaration
        signature: "public class TokenStats {"
        children:
        - id: TokenStats.tV0ZpA|aTVvOy
          type: StatementGroup
          code: "\tpublic long inputToken;\n\tpublic long outputToken;\n\tpublic long reasoningToken;\n\tpublic long totalToken;\n\tpublic long cacheRead;\n\tpublic long cacheCreate;\n"
        - id: TokenStats.toString
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String toString() {\n\t\treturn String.format(\n\t\t\t\t\"TokenStats [inputToken=%s, outputToken=%s, reasoningToken=%s, totalInToken=%s, cacheRead=%s, cacheCreate=%s]\",\n\t\t\t\tinputToken, outputToken, reasoningToken, totalToken, cacheRead, cacheCreate);\n\t}\n"
        - id: TokenStats.jxwqEb|DF2IZE
          type: StatementGroup
          code: "\tprivate static final Pattern PATTERN = Pattern\n\t\t\t.compile(\"total: (\\\\d+), in: (\\\\d+), out: (\\\\d+), reason: (\\\\d+), read: (\\\\d+), write: (\\\\d+)\");\n"
        - id: TokenStats.print
          type: MethodDeclaration
          code: "\tpublic String print() {\n\t\treturn String.format(\"total: %s, in: %s, out: %s, reason: %s, read: %s, write: %s\", totalToken, inputToken,\n\t\t\t\toutputToken, reasoningToken, cacheRead, cacheCreate);\n\t}\n"
        - id: TokenStats.fromString
          type: MethodDeclaration
          code: "\tpublic static TokenStats fromString(String s) {\n\t\tif (s == null)\n\t\t\treturn null;\n\t\tMatcher m = PATTERN.matcher(s);\n\t\tif (!m.find())\n\t\t\treturn null;\n\t\tTokenStats stats = new TokenStats();\n\t\tstats.totalToken = Long.parseLong(m.group(1));\n\t\tstats.inputToken = Long.parseLong(m.group(2));\n\t\tstats.outputToken = Long.parseLong(m.group(3));\n\t\tstats.reasoningToken = Long.parseLong(m.group(4));\n\t\tstats.cacheRead = Long.parseLong(m.group(5));\n\t\tstats.cacheCreate = Long.parseLong(m.group(6));\n\t\treturn stats;\n\t}\n"
        - id: TokenStats.add
          type: MethodDeclaration
          code: "\tpublic void add(TokenStats stats) {\n\t\tinputToken += stats.inputToken;\n\t\toutputToken += stats.outputToken;\n\t\treasoningToken += stats.reasoningToken;\n\t\ttotalToken += stats.totalToken;\n\t\tcacheRead += stats.cacheRead;\n\t\tcacheCreate += stats.cacheCreate;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
      nodes:
      - id: AIAnswer
        type: ClassOrInterfaceDeclaration
        signature: "public class AIAnswer {"
        children:
        - id: AIAnswer.j6EUkY|f7JitM
          type: StatementGroup
          code: "\tpublic static final String RESULT = \"Result Stats: \";\n\n\tprivate static final Pattern ID_PATTERN = Pattern.compile(\"id=([^,]*), \");\n\n\tpublic final String id;\n\tpublic String sessionId;\n\tpublic final TokenStats stats = new TokenStats();\n\tpublic String answer = \"\";\n\tpublic String instructions = \"\";\n\tpublic boolean showStats = true;\n\t/**\n\t * Prompt that produced this answer; null once no longer available\n\t */\n\tpublic Prompt prompt;\n"
        - id: AIAnswer.AIAnswer
          type: ConstructorDeclaration
          code: "\tpublic AIAnswer(String id) {\n\t\tthis.id = id;\n\t\tthis.sessionId = id;\n\t}\n"
        - id: AIAnswer.toString
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String toString() {\n\t\treturn String.format(\"AIAnswer [stats=%s]\", stats);\n\t}\n"
        - id: AIAnswer.print
          type: MethodDeclaration
          code: "\tpublic String print() {\n\t\treturn String.format(\"%sid=%s, %s\", RESULT, sessionId, stats.print());\n\t}\n"
        - id: AIAnswer.fromString
          type: MethodDeclaration
          code: "\tpublic static AIAnswer fromString(String line) {\n\t\tif (line == null)\n\t\t\treturn null;\n\t\tMatcher idMatcher = ID_PATTERN.matcher(line);\n\t\tif (!idMatcher.find())\n\t\t\treturn null;\n\t\tTokenStats stats = TokenStats.fromString(line.substring(idMatcher.end()));\n\t\tif (stats == null)\n\t\t\treturn null;\n\t\tAIAnswer ans = new AIAnswer(idMatcher.group(1));\n\t\tans.stats.add(stats);\n\t\treturn ans;\n\t}\n"
```
/answer 8fc6-6 allow
Control Request:
```yaml
id: 8fc6-7
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
    pattern: stats|AIAnswer|RESULT
    include:
    - '*.java'
    limit: 50
  reason: Find where stats are populated and printed
```
/answer 8fc6-7 allow
Control Request:
```yaml
id: 8fc6-8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      matches:
      - path: mcp/MCPResponse.java
        matches:
        - lineno: 9
          match: public final TokenStats stats = new TokenStats();
      - path: mcp/MCPConnector.java
        matches:
        - lineno: 20
          match: import xy.ai.workbench.models.AIAnswer;
        - lineno: 81
          match: "return new MCPResponse(id, EditorInterface.TOOLRESULT + \"\\n\" + control.prettyToolResult(toolCallId, payload));"
        - lineno: 85
          match: "public AIAnswer convertResponse(MCPResponse resp, IProgressMonitor mon) {"
        - lineno: 86
          match: AIAnswer answer = new AIAnswer(resp.id);
      - path: claudecode/CCSession.java
        matches:
        - lineno: 40
          match: public final TokenStats stats = new TokenStats();
      - path: claudecode/CCResponse.java
        matches:
        - lineno: 22
          match: public final TokenStats stats = new TokenStats();
      - path: claudecode/ProtocolParser.java
        matches:
        - lineno: 114
          match: TokenStats stats = new TokenStats();
        - lineno: 117
          match: stats.inputToken += usage.path("inputTokens").asLong(0);
        - lineno: 118
          match: stats.outputToken += usage.path("outputTokens").asLong(0);
        - lineno: 119
          match: stats.cacheRead += usage.path("cacheReadInputTokens").asLong(0);
        - lineno: 120
          match: stats.cacheCreate += usage.path("cacheCreationInputTokens").asLong(0);
        - lineno: 122
          match: resp.stats.add(stats);
        - lineno: 202
          match: resp.stats.reasoningToken += thinkingTokens;
      - path: claudecode/CCConnector.java
        matches:
        - lineno: 28
          match: import xy.ai.workbench.models.AIAnswer;
        - lineno: 178
          match: session.stats.add(resp.stats);
        - lineno: 179
          match: session.stats.totalToken = session.stats.inputToken + session.stats.cacheCreate;
        - lineno: 195
          match: "public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {"
        - lineno: 196
          match: AIAnswer answer = new AIAnswer(resp.id);
        - lineno: 199
          match: answer.stats.inputToken = resp.stats.inputToken;
        - lineno: 200
          match: answer.stats.outputToken = resp.stats.outputToken;
        - lineno: 201
          match: answer.stats.reasoningToken = resp.stats.reasoningToken;
        - lineno: 202
          match: answer.stats.cacheRead = resp.stats.cacheRead;
        - lineno: 203
          match: answer.stats.cacheCreate = resp.stats.cacheCreate;
        - lineno: 204
          match: answer.stats.totalToken = answer.stats.inputToken + answer.stats.cacheCreate + answer.stats.outputToken;
      - path: google/GeminiConnector.java
        matches:
        - lineno: 41
          match: import xy.ai.workbench.models.AIAnswer;
        - lineno: 158
          match: "public AIAnswer convertResponse(GeminiResponse resp, IProgressMonitor mon) {"
        - lineno: 162
          match: AIAnswer res = new AIAnswer(resp.id);
        - lineno: 178
          match: res.stats.inputToken = usage.promptTokenCount().orElse(-1).intValue();
        - lineno: 179
          match: res.stats.reasoningToken = usage.thoughtsTokenCount().orElse(-1).intValue();
        - lineno: 180
          match: res.stats.totalToken = usage.totalTokenCount().orElse(-1).intValue();
      - path: google/GeminiBatchConnector.java
        matches:
        - lineno: 35
          match: import xy.ai.workbench.models.AIAnswer;
        - lineno: 113
          match: List<AIAnswer> answers = new ArrayList<>();
        - lineno: 124
          match: "AIAnswer ans = connector.convertResponse(new GeminiResponse(ids[i], r), sub);"
        - lineno: 198
          match: List<AIAnswer> answ = new ArrayList<>(entry.getAnswers());
        - lineno: 212
          match: AIAnswer eans = new AIAnswer("batch_error");
        - lineno: 220
          match: "private AIAnswer convertToAnswer(String responseJson, String id, IProgressMonitor mon) {"
        - lineno: 223
          match: "AIAnswer answer = connector.convertResponse(new GeminiResponse(id, resp), mon);"
      - path: google/GeminiBatch.java
        matches:
        - lineno: 13
          match: import xy.ai.workbench.models.AIAnswer;
        - lineno: 24
          match: private List<AIAnswer> answers = Collections.emptyList();
        - lineno: 36
          match: "public void setAnswers(List<AIAnswer> answ) {"
        - lineno: 41
          match: "public Collection<AIAnswer> getAnswers() {"
      - path: IAIConnector.java
        matches:
        - lineno: 7
          match: import xy.ai.workbench.models.AIAnswer;
        - lineno: 18
          match: "AIAnswer convertResponse(RESP resp, IProgressMonitor mon);"
      - path: IAIBatch.java
        matches:
        - lineno: 8
          match: import xy.ai.workbench.models.AIAnswer;
        - lineno: 38
          match: Collection<AIAnswer> getAnswers();
        - lineno: 40
          match: void setAnswers(List<AIAnswer> answ);
      - path: claude/ClaudeBatchConnector.java
        matches:
        - lineno: 29
          match: import xy.ai.workbench.models.AIAnswer;
        - lineno: 85
          match: AIAnswer an = new AIAnswer(res.customId());
      - path: claude/ClaudeBatch.java
        matches:
        - lineno: 12
          match: import xy.ai.workbench.models.AIAnswer;
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 8fc6-8 allow
Control Request:
```yaml
id: 8fc6-9
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids: []
  reason: placeholder
```
/answer 8fc6-9 allow
Control Request:
```yaml
id: 8fc6-a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: K9s6LE|NFCMYw
        signature: import java.io.IOException;
      - id: gqhGpR|GrHQlC
        signature: import xy.ai.workbench.connector.harness.IIncludeAdapter;
      - id: CCConnector
        signature: "public class CCConnector implements IAIConnector<CCRequest, CCResponse> {"
        children:
        - id: CCConnector.gmpwNc|XMcbVZ
          signature: private final CCRequestBuilder requestBuilder = new CCRequestBuilder();
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
        - id: CCConnector.executeRequest
          signature: "public CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job)"
        - id: CCConnector.readUntilResult
          signature: "private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressM…"
        - id: CCConnector.abbreviateForStatus
          signature: private static String abbreviateForStatus(String s)
        - id: CCConnector.convertResponse
          signature: "public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon)"
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 8fc6-a allow
Control Request:
```yaml
id: 8fc6-b
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  lineno: 100
  end_lineno: 215
  reason: Read request flow and convertResponse
```
/answer 8fc6-b allow
Control Request:
```yaml
id: 8fc6-c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes: []
```
/answer 8fc6-c allow
Control Request:
```yaml
id: 8fc6-d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - CCConnector.executeRequest
    - CCConnector.readUntilResult
    - CCConnector.convertResponse
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - CCControlClient.checkControlEndpoint
    - CCControlClient.post
  reason: Read flow to place measurements
```
/answer 8fc6-d allow
Control Request:
```yaml
id: 8fc6-e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.executeRequest
        type: MethodDeclaration
        code: |2
            @Override
            public CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {
              SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 2);
              CCSession session = null;
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
                  controlClient.approve(ac.parameter(0), ac.parameter(2));
                else if (ac.action() == AnswerCommand.Action.Exec)
                  controlClient.exec(ac.parameter(0), ac.parameter(2));
                else
                  controlClient.deny(ac.parameter(0), ac.parameter(2));
                session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
              } else if (command instanceof CallEditCommand cec) {
                if (!controlClient.submitEdit(cec.yaml()))
                  throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
                session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
              } else if (command instanceof CallCommand) {
                if (req.arg.yaml == null || req.arg.yaml.isBlank())
                  throw new IllegalArgumentException("No preceding ```yaml block found before /call");
                if (!controlClient.submitEdit(req.arg.yaml))
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
      - id: CCConnector.readUntilResult
        type: MethodDeclaration
        code: |2
            private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressMonitor mon, Job job) throws IOException {
              SubMonitor sub = SubMonitor.convert(mon, "Reading Claude output", 10000);
              CCResponse resp = new CCResponse(req.id);
              String line;
              long lastChangeAt = System.currentTimeMillis();
              while (true) {
                boolean changed = false;
                // alternate read sources undtil answer
                try {
                  // wait 300 ms
                  if ((line = session.readLine()) != null) {
                    session.setLastRawLine(line);
                    job.setName(abbreviateForStatus(line));
                    mon.worked(1);
                    jsonParser.parseLine(resp, session, sub, line);
                    changed = true;
                  }
                } catch (Exception ex) {
                  LOG.error("Error reading line: ", ex);
                  resp.resultText = ex.getMessage();
                  changed = true;
                }
                try {
                  String errLine = session.readError();
                  if (errLine != null) {
                    LOG.error("CLI stderr: " + errLine);
                    resp.resultText = (resp.resultText == null ? "" : resp.resultText + "\n") + errLine;
                    changed = true;
                  }
                } catch (Exception ex2) {
                  LOG.error("Error reading stderr: ", ex2);
                }
                if (changed)
                  lastChangeAt = System.currentTimeMillis();
                else if (System.currentTimeMillis() - lastChangeAt > 30000)
                  // Stuck detection: neither STDOUT nor STDERR produced anything for 30s
                  throw new IOException("Stuck detection: no STDOUT/STDERR output for 30 seconds (session " + session.getID() + ")");
                if (!resp.isReady())
                  controlClient.checkControlEndpoint(resp, session.getID());
                if (resp.isReady()) {
                  resp.sessionId = session.getID();
                  session.stats.add(resp.stats);
                  session.stats.totalToken = session.stats.inputToken + session.stats.cacheCreate;
                  return resp;
                }
                sessionManager.onSessionChanged(session);
              }
            }
      - id: CCConnector.convertResponse
        type: MethodDeclaration
        code: |2
            @Override
            public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {
              AIAnswer answer = new AIAnswer(resp.id);
              answer.showStats = resp.showStats;
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
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
/answer 8fc6-e allow
Control Request:
```yaml
id: 8fc6-f
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
  reason: Inspect session/response
```
/answer 8fc6-f allow
Control Request:
```yaml
id: 8fc6-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
      nodes:
      - id: bpHcRg|JYkZcB
        signature: import java.io.File;
      - id: CCSession
        signature: "public class CCSession {"
        children:
        - id: CCSession.PrvPLS|gRlRu2
          signature: private final String uuid;
        - id: CCSession.CCSession
          signature: "public CCSession(CCSessionManager manager, ClaudeSessionParameters parameters)"
        - id: CCSession.CCSession_2
          signature: "public CCSession(String sessionUuid, CCSessionManager manager, ClaudeSessionPar…"
        - id: CCSession.CCSession_3
          signature: "private CCSession(String sessionUuid, boolean resume, CCSessionManager manager,…"
        - id: CCSession.getRemainingTtlMinutes
          signature: public long getRemainingTtlMinutes()
        - id: CCSession.getState
          signature: public SessionState getState()
        - id: CCSession.getRemainingTtlSeconds
          signature: public long getRemainingTtlSeconds()
          docstring: "@return the number of whole seconds until this session's TTL runs out, or"
        - id: CCSession.isValid
          signature: public boolean isValid()
          docstring: "@return {@code true} unless the session is {@link SessionState#Expired}. Used"
        - id: CCSession.start
          signature: private synchronized void start() throws IOException
        - id: CCSession.terminate
          signature: public synchronized void terminate()
        - id: CCSession.writeLine
          signature: public synchronized void writeLine(String jsonLine) throws IOException
        - id: CCSession.mirrorLine
          signature: private void mirrorLine(String jsonLine) throws IOException
        - id: CCSession.readLine
          signature: public String readLine()
        - id: CCSession.readError
          signature: public String readError()
        - id: CCSession.readLine_2
          signature: "private String readLine(TimedLineReader reader, String channel)"
        - id: CCSession.openMirrorIfNeeded
          signature: private void openMirrorIfNeeded()
        - id: CCSession.getSessionUuid
          signature: public String getSessionUuid()
        - id: CCSession.getLastSentAt
          signature: public Instant getLastSentAt()
        - id: CCSession.getLastParsedMessage
          signature: public String getLastParsedMessage()
        - id: CCSession.setLastParsedMessage
          signature: public void setLastParsedMessage(String msg)
        - id: CCSession.setLastRawLine
          signature: public void setLastRawLine(String line)
        - id: CCSession.getLastRawLine
          signature: public String getLastRawLine()
        - id: CCSession.isLastRawLineProcessed
          signature: public boolean isLastRawLineProcessed()
        - id: CCSession.getLastReceivedAt
          signature: public Instant getLastReceivedAt()
        - id: CCSession.isExpired
          signature: public boolean isExpired()
        - id: CCSession.isProcessAlive
          signature: private boolean isProcessAlive()
        - id: CCSession.setInPrompt
          signature: public void setInPrompt(boolean inPrompt)
        - id: CCSession.getParameters
          signature: public ClaudeSessionParameters getParameters()
        - id: CCSession.notifyChanged
          signature: private void notifyChanged()
        - id: CCSession.getID
          signature: public String getID()
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
      nodes:
      - id: GmYxyr|se6MkO
        signature: import java.util.LinkedHashMap;
      - id: CCResponse
        signature: "public class CCResponse implements IModelResponse {"
        children:
        - id: CCResponse.B0vmpT|2GN6aA
          signature: public final String id;
        - id: CCResponse.CCResponse
          signature: public CCResponse(String id)
        - id: CCResponse.CCResponse_2
          signature: "public CCResponse(String id, String resultText)"
        - id: CCResponse.setToolUse
          signature: "public void setToolUse(String resultText, String toolUseId)"
        - id: CCResponse.isReady
          signature: public boolean isReady()
```
/answer 8fc6-10 allow
Control Request:
```yaml
id: 8fc6-11
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
  text: TokenStats stats
  reason: Get node id for stats field
```
/answer 8fc6-11 allow
Control Request:
```yaml
id: 8fc6-12
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
      nodes:
      - id: CCResponse.B0vmpT|2GN6aA
        code: "\tpublic final String id;\n\tpublic String sessionId;\n\tpublic String resultText;\n\tpublic boolean isError;\n\t/**\n\t * True when this response represents a pending tool_use request rather than a\n\t * final result.\n\t */\n\tpublic boolean isToolRequest;\n\t/** The tool_use id when isToolRequest is true, otherwise null. */\n\tpublic String toolUseId;\n\n\tpublic final TokenStats stats = new TokenStats();\n\tpublic final LinkedHashMap<String, String> events = new LinkedHashMap<>();\n\tpublic boolean showStats = true;\n"
```
/answer 8fc6-12 allow
Control Request:
```yaml
id: 8fc6-13
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
  text: TokenStats stats
  reason: Get node id for session stats field
```
/answer 8fc6-13 allow
Control Request:
```yaml
id: 8fc6-14
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
      nodes:
      - id: CCSession.PrvPLS|gRlRu2
        code: "\tprivate final String uuid;\n\tprivate final ClaudeSessionParameters parameters;\n\n\tprivate Process process;\n\tprivate PrintWriter stdin;\n\tprivate TimedLineReader stdout;\n\tprivate TimedLineReader stderr;\n\tprivate Writer mirror;\n\n\t@SuppressWarnings(\"unused\")\n\tprivate final Instant createdAt = Instant.now();\n\t@SuppressWarnings(\"unused\")\n\tprivate Instant startedAt;\n\t/** The last time a prompt was sent to STDIN. Determines TTL. */\n\tprivate Instant lastSentAt;\n\tprivate volatile Instant lastReceivedAt;\n\n\tpublic final TokenStats stats = new TokenStats();\n\tprivate volatile boolean inPrompt;\n\tprivate volatile String lastParsedMessage;\n\tprivate volatile String lastRawLine;\n\tprivate volatile boolean lastRawLineProcessed;\n\tprivate boolean resume;\n\n\tprivate final CCSessionManager manager;\n"
```
/answer 8fc6-14 allow
Control Request:
```yaml
id: 8fc6-15
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TimeStats.java
    source: "package xy.ai.workbench.models;\n\nimport java.util.regex.Matcher;\nimport java.util.regex.Pattern;\n\npublic class TimeStats {\n\t/** Epoch millis of the last prompt */\n\tpublic long startAt;\n\tpublic long endAt;\n\t/** Runtime of the last prompt in ms, excluding control client time */\n\tpublic long durationMs;\n\t/** Cumulative durationMs of all prompts in the session */\n\tpublic long totalMs;\n\tpublic long ioMs;\n\tpublic long controlMs;\n\tpublic long sent;\n\tpublic long received;\n\n\tprivate static final Pattern PATTERN = Pattern.compile(\n\t\t\t\"start: (\\\\d+), end: (\\\\d+), time: (\\\\d+), total: (\\\\d+), io: (\\\\d+), control: (\\\\d+), sent: (\\\\d+), recv: (\\\\d+)\");\n\n\tpublic void start() {\n\t\tstartAt = System.currentTimeMillis();\n\t}\n\n\t/** Ends the measurement; control time is excluded from the duration. */\n\tpublic void stop() {\n\t\tendAt = System.currentTimeMillis();\n\t\tdurationMs = Math.max(0, endAt - startAt - controlMs);\n\t}\n\n\tpublic <T> T control(java.util.function.Supplier<T> call) {\n\t\tlong t = System.nanoTime();\n\t\ttry {\n\t\t\treturn call.get();\n\t\t} finally {\n\t\t\tcontrolMs += (System.nanoTime() - t) / 1_000_000;\n\t\t}\n\t}\n\n\tpublic <T> T io(java.util.function.Supplier<T> call) {\n\t\tlong t = System.nanoTime();\n\t\ttry {\n\t\t\treturn call.get();\n\t\t} finally {\n\t\t\tioMs += (System.nanoTime() - t) / 1_000_000;\n\t\t}\n\t}\n\n\t@Override\n\tpublic String toString() {\n\t\treturn String.format(\n\t\t\t\t\"TimeStats [startAt=%s, endAt=%s, durationMs=%s, totalMs=%s, ioMs=%s, controlMs=%s, sent=%s, received=%s]\",\n\t\t\t\tstartAt, endAt, durationMs, totalMs, ioMs, controlMs, sent, received);\n\t}\n\n\tpublic String print() {\n\t\treturn String.format(\"start: %s, end: %s, time: %s, total: %s, io: %s, control: %s, sent: %s, recv: %s\", startAt,\n\t\t\t\tendAt, durationMs, totalMs, ioMs, controlMs, sent, received);\n\t}\n\n\tpublic static TimeStats fromString(String s) {\n\t\tif (s == null)\n\t\t\treturn null;\n\t\tMatcher m = PATTERN.matcher(s);\n\t\tif (!m.find())\n\t\t\treturn null;\n\t\tTimeStats t = new TimeStats();\n\t\tt.startAt = Long.parseLong(m.group(1));\n\t\tt.endAt = Long.parseLong(m.group(2));\n\t\tt.durationMs = Long.parseLong(m.group(3));\n\t\tt.totalMs = Long.parseLong(m.group(4));\n\t\tt.ioMs = Long.parseLong(m.group(5));\n\t\tt.controlMs = Long.parseLong(m.group(6));\n\t\tt.sent = Long.parseLong(m.group(7));\n\t\tt.received = Long.parseLong(m.group(8));\n\t\treturn t;\n\t}\n\n\t/** Accumulates a finished prompt into this (session) total. */\n\tpublic void add(TimeStats t) {\n\t\tstartAt = t.startAt;\n\t\tendAt = t.endAt;\n\t\tdurationMs = t.durationMs;\n\t\ttotalMs += t.durationMs;\n\t\tioMs += t.ioMs;\n\t\tcontrolMs += t.controlMs;\n\t\tsent += t.sent;\n\t\treceived += t.received;\n\t}\n}\n"
  reason: New TimeStats model analogous to TokenStats
```
/answer 8fc6-15 allow
Control Request:
```yaml
id: 8fc6-16
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
    id: AIAnswer.j6EUkY|f7JitM
    old_text: public final TokenStats stats = new TokenStats();
    new_text: "public final TokenStats stats = new TokenStats();\n\tpublic final TimeStats time = new TimeStats();"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
    id: CCResponse.B0vmpT|2GN6aA
    old_text: public final TokenStats stats = new TokenStats();
    new_text: "public final TokenStats stats = new TokenStats();\n\tpublic final xy.ai.workbench.models.TimeStats time = new xy.ai.workbench.models.TimeStats();"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
    id: CCSession.PrvPLS|gRlRu2
    old_text: public final TokenStats stats = new TokenStats();
    new_text: "public final TokenStats stats = new TokenStats();\n\tpublic final xy.ai.workbench.models.TimeStats time = new xy.ai.workbench.models.TimeStats();"
  reason: Add time fields
```
/answer 8fc6-16 allow
Control Request:
```yaml
id: 8fc6-17
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
    id: AIAnswer.print
    source: "\tpublic String print() {\n\t\treturn String.format(\"%sid=%s, %s, %s\", RESULT, sessionId, stats.print(), time.print());\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
    id: AIAnswer.fromString
    source: "\tpublic static AIAnswer fromString(String line) {\n\t\tif (line == null)\n\t\t\treturn null;\n\t\tMatcher idMatcher = ID_PATTERN.matcher(line);\n\t\tif (!idMatcher.find())\n\t\t\treturn null;\n\t\tString rest = line.substring(idMatcher.end());\n\t\tTokenStats stats = TokenStats.fromString(rest);\n\t\tif (stats == null)\n\t\t\treturn null;\n\t\tAIAnswer ans = new AIAnswer(idMatcher.group(1));\n\t\tans.stats.add(stats);\n\t\tTimeStats t = TimeStats.fromString(rest);\n\t\tif (t != null)\n\t\t\tans.time.add(t);\n\t\treturn ans;\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.convertResponse
    source: |2
        @Override
        public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {
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
  reason: Print/parse/convert time stats
```
/answer 8fc6-17 allow
Control Request:
```yaml
id: 8fc6-18
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: |2
          CCSession session = null;
    new_text: |2
          CCSession session = null;
          TimeStats ts = new TimeStats();
          ts.start();
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: "        controlClient.approve(ac.parameter(0), ac.parameter(2));"
    new_text: "        ts.control(() -> controlClient.approve(ac.parameter(0), ac.parameter(2)));"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: "        controlClient.exec(ac.parameter(0), ac.parameter(2));"
    new_text: "        ts.control(() -> controlClient.exec(ac.parameter(0), ac.parameter(2)));"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: "        controlClient.deny(ac.parameter(0), ac.parameter(2));"
    new_text: "        ts.control(() -> controlClient.deny(ac.parameter(0), ac.parameter(2)));"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: if (!controlClient.submitEdit(cec.yaml()))
    new_text: if (!ts.control(() -> controlClient.submitEdit(cec.yaml())))
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: if (!controlClient.submitEdit(req.arg.yaml))
    new_text: if (!ts.control(() -> controlClient.submitEdit(req.arg.yaml)))
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: '&& !controlClient.isMCPCAvailable())'
    new_text: '&& !ts.control(() -> controlClient.isMCPCAvailable()))'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: '        session.writeLine(requestBuilder.buildPromptJson(req.promptText));'
    new_text: |2-
              String promptJson = requestBuilder.buildPromptJson(req.promptText);
              long t0 = System.nanoTime();
              session.writeLine(promptJson);
              ts.ioMs += (System.nanoTime() - t0) / 1_000_000;
              ts.sent++;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: "readUntilResult(req, session, sub.split(1), job);"
    new_text: "readUntilResult(req, session, sub.split(1), job, ts);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.readUntilResult
    old_text: "IProgressMonitor mon, Job job) throws IOException {"
    new_text: "IProgressMonitor mon, Job job, TimeStats ts) throws IOException {"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.readUntilResult
    old_text: "if ((line = session.readLine()) != null) {"
    new_text: |-
      if ((line = ts.io(() -> session.readLine())) != null) {
                ts.received++;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.readUntilResult
    old_text: String errLine = session.readError();
    new_text: String errLine = ts.io(() -> session.readError());
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.readUntilResult
    old_text: "controlClient.checkControlEndpoint(resp, session.getID());"
    new_text: |-
      ts.control(() -> {
                controlClient.checkControlEndpoint(resp, session.getID());
                return null;
              });
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.readUntilResult
    old_text: '        session.stats.add(resp.stats);'
    new_text: |2-
              ts.stop();
              session.time.add(ts);
              resp.time.add(ts);
              resp.time.totalMs = session.time.totalMs;
              session.stats.add(resp.stats);
  reason: Instrument connector flow
```
/answer 8fc6-18 allow
Control Request:
```yaml
id: 8fc6-19
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: K9s6LE|NFCMYw
    position: before
    source: |
      import xy.ai.workbench.models.TimeStats;
  reason: Import TimeStats
```
/answer 8fc6-19 allow
Control Request:
```yaml
id: 8fc6-1a
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TimeStats.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
  reason: Syntax check
```
/answer 8fc6-1a allow
Result Stats: id=03f10352-6ac0-4ee4-9ff1-33d37fab23ce, total: 0, in: 0, out: 0, reason: 1458, read: 0, write: 0
Cannot read STDOUT: the CLI process is not started (uuid=03f10352-6ac0-4ee4-9ff1-33d37fab23ce, processAlive=false). A prompt must be sent (writeLine) before reading.