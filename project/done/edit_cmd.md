In `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java` soll das Kommando Parsing in `submitEdit` vom Kommandosystem im `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java` übernommen werden. Dazu muss ein neues "CallEditCommand" in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands` angelegt und in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java` registriert werden.
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java` soll entsprechend das Kommando weiterreichen.
Kommando Parsing soll vom Kommando System übernommen werden.
Das "CallEditCommand" besteht aus allein aus einem selektierten YAML Skript block.
Das `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java` soll nun auch unterstützt werden. Dieses bezieht sich auf einen vorgelagerten YAML Block.
SystemInit:  id=af74611e-53e1-4aeb-8ec8-2208a01a6298, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 47e1-6d
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
  reason: Struktur der relevanten Dateien und Commands-Verzeichnis erkunden
```
/answer 47e1-6d allow
Control Request:
```yaml
id: 47e1-6e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: EdXLLM|WHD6Rc
      - id: 9zVoqr|vnzPrh
      - id: Gv3B9R|AQWMTb
      - id: CCControlClient
        signature: "public class CCControlClient {"
        children:
        - id: CCControlClient.pWyigk|ktvRxK
        - id: CCControlClient.checkControlEndpoint
          signature: "public void checkControlEndpoint(CCResponse resp) {"
        - id: CCControlClient.toYaml
          signature: "public String toYaml(JsonNode node) {"
        - id: CCControlClient.fromYaml
          signature: "public JsonNode fromYaml(String text) throws JsonProcessingException {"
        - id: CCControlClient.isMCPCAvailable
          signature: "public boolean isMCPCAvailable() {"
        - id: CCControlClient.poll
          signature: "private ArrayNode poll() {"
        - id: CCControlClient.approve
          signature: "public ArrayNode approve(String id) {"
        - id: CCControlClient.approve_1
          signature: "public ArrayNode approve(String id, String hint) {"
        - id: CCControlClient.deny
          signature: "public ArrayNode deny(String id, String reason) {"
        - id: CCControlClient.submitEdit
          signature: "public boolean submitEdit(String rawText) {"
        - id: CCControlClient.extractYamlBlock
          signature: "private String extractYamlBlock(String text) {"
        - id: CCControlClient.approvalNode
          signature: "private ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result,…"
        - id: CCControlClient.submit
          signature: "private ArrayNode submit(ObjectNode approval) {"
        - id: CCControlClient.post
          signature: "private ArrayNode post(ObjectNode body) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: RLo8qh|nFvk0T
      - id: 4UXeuu|NTMCrV
      - id: KEc50E|0JLMrF
      - id: fPHG2Y|yRFIp4
      - id: PromptHandler
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.dE1Gmz|tJO7uC
        - id: PromptHandler.iAEUAf|LdjI0C
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector,"
        - id: PromptHandler.clearObserver
          signature: "public void clearObserver() {"
        - id: PromptHandler.addInputStatObs
          signature: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {"
        - id: PromptHandler.addAnswerObs
          signature: "public void addAnswerObs(Consumer<AIAnswer> obs) {"
        - id: PromptHandler.updateInputStat
          signature: "public void updateInputStat(InputMode mode) {"
        - id: PromptHandler.initializeInputs
          signature: "public void initializeInputs() {"
        - id: PromptHandler.getInput
          signature: "private String getInput(InputMode mode) {"
        - id: PromptHandler.removeCommentLines
          signature: "private String removeCommentLines(String input) {"
        - id: PromptHandler.execute
          signature: "public void execute(Display display) {"
        - id: PromptHandler.PromptJob
          signature: "private class PromptJob extends Job {"
          children:
          - id: PromptHandler.PromptJob.zOXTQk|KFkQPW
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display, AIBatchManager batch) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, AIBatchManager batch, IProgressMonitor …"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display, AIBatchManager batch) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.maceLN|sqs7wQ
        - id: PromptHandler.buildPrompt
          signature: "private Prompt buildPrompt(Display display, boolean batch) {"
        - id: PromptHandler.ohnvjH|nhSFvk
        - id: PromptHandler.detectSelection
          signature: "private Prompt.InputKind detectSelection(ITextEditor textEditor, List<String> i…"
        - id: PromptHandler.Txg3oX|3Xw56U
        - id: PromptHandler.detectFullFile
          signature: "private Prompt.InputKind detectFullFile(ITextEditor textEditor, List<String> in…"
        - id: PromptHandler.applyDetected
          signature: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Comman…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.Q5D2ne|VNsv72
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: x5J3oI|C5FWzw
      - id: K5C4w9|X2Xlry
      - id: uzcHjn|mD6r4Z
      - id: CCConnector
        signature: "public class CCConnector implements IAIConnector<CCRequest, CCResponse> {"
        children:
        - id: CCConnector.51iiPA|yBdfkV
        - id: CCConnector.CCConnector
          signature: "public CCConnector(CCSessionManager sessionManager) {"
        - id: CCConnector.getSupportedKeyPattern
          signature: "public KeyPattern getSupportedKeyPattern() {"
        - id: CCConnector.createRequest
          signature: "public CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
        - id: CCConnector.executeRequest
          signature: "public CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {"
        - id: CCConnector.readUntilResult
          signature: "private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressM…"
        - id: CCConnector.abbreviateForStatus
          signature: "private static String abbreviateForStatus(String s) {"
        - id: CCConnector.convertResponse
          signature: "public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {"
        - id: CCConnector.getEditorLocation
          signature: "private EditorLocation getEditorLocation() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
      nodes:
      - id: b6Xn8Z|PmHdMh
      - id: CommandHandler
        signature: "public class CommandHandler {"
        children:
        - id: CommandHandler.yMrxSu|gq2BES
        - id: CommandHandler.CommandHandler
          signature: "private CommandHandler() {"
        - id: CommandHandler.detect
          signature: "public static Command detect(String line) {"
        - id: CommandHandler.list
          signature: "public static List<Command> list() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
      nodes:
      - id: fau272|XRuDTB
      - id: CallCommand
        signature: "public class CallCommand extends Command {"
        children:
        - id: CallCommand.SX50AH|tAIjYQ
        - id: CallCommand.CallCommand
          signature: "public CallCommand() {"
        - id: CallCommand.prefix
          signature: "public String prefix() {"
        - id: CallCommand.matches
          signature: "public boolean matches(String line) {"
        - id: CallCommand.parse
          signature: "public Command parse(String line) {"
        - id: CallCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
      error: Not a regular file.
```
/answer 47e1-6e allow
ReasoningToken: 14
Control Request:
```yaml
id: 47e1-6f
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
  reason: Base Command class ansehen
```
/answer 47e1-6f allow
Control Request:
```yaml
id: 47e1-70
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
      nodes:
      - id: fau272|XRuDTB
      - id: Command
        signature: "public abstract class Command {"
        children:
        - id: Command.ProcessorAction
          signature: "public enum ProcessorAction {"
        - id: Command.iKqF62|lJcluI
        - id: Command.Command
          signature: "protected Command(String... parameters) {"
        - id: Command.prefix
          signature: public abstract String prefix();
        - id: Command.matches
          signature: "public boolean matches(String line) {"
        - id: Command.parse
          signature: public abstract Command parse(String line);
        - id: Command.processorAction
          signature: public abstract ProcessorAction processorAction();
        - id: Command.parameter
          signature: "public String parameter(int i) {"
```
/answer 47e1-70 allow
Control Request:
```yaml
id: 47e1-71
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
    pattern: .
    include:
    - '*.java'
    limit: 50
  reason: Liste aller Dateien im commands-Verzeichnis
```
/answer 47e1-71 allow
Control Request:
```yaml
id: 47e1-72
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
      matches:
      - path: ToolCommand.java
        matches:
        - lineno: 1
          match: package xy.ai.workbench.commands;
        - lineno: 3
          match: "public class ToolCommand extends Command {"
        - lineno: 5
          match: public static final String CMD_TOOL   = "/tool";
        - lineno: 7
          match: "public ToolCommand() {"
        - lineno: 8
          match: this("");
        - lineno: 9
          match: "}"
        - lineno: 11
          match: "public ToolCommand(String toolId) {"
        - lineno: 12
          match: super(toolId);
        - lineno: 13
          match: "}"
        - lineno: 15
          match: '@Override'
        - lineno: 16
          match: "public String prefix() {"
        - lineno: 17
          match: return ToolCommand.CMD_TOOL;
        - lineno: 18
          match: "}"
        - lineno: 20
          match: '@Override'
        - lineno: 21
          match: "public boolean matches(String line) {"
        - lineno: 22
          match: return line != null && line.strip().matches("(?i)" + prefix() + "\\s+\\S+");
        - lineno: 23
          match: "}"
        - lineno: 25
          match: '@Override'
        - lineno: 26
          match: "public Command parse(String line) {"
        - lineno: 27
          match: "return new ToolCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());"
        - lineno: 28
          match: "}"
        - lineno: 30
          match: '@Override'
        - lineno: 31
          match: "public ProcessorAction processorAction() {"
        - lineno: 32
          match: return ProcessorAction.REMOVE;
        - lineno: 33
          match: "}"
        - lineno: 34
          match: "}"
      - path: Command.java
        matches:
        - lineno: 1
          match: package xy.ai.workbench.commands;
        - lineno: 3
          match: "public abstract class Command {"
        - lineno: 5
          match: "public enum ProcessorAction {"
        - lineno: 6
          match: "REMOVE, TRANSFORM, IGNORE_BLOCK, NONE"
        - lineno: 7
          match: "}"
        - lineno: 9
          match: "protected final String[] parameters;"
        - lineno: 11
          match: "protected Command(String... parameters) {"
        - lineno: 12
          match: this.parameters = parameters;
        - lineno: 13
          match: "}"
        - lineno: 15
          match: public abstract String prefix();
        - lineno: 17
          match: "public boolean matches(String line) {"
        - lineno: 18
          match: return line != null && line.strip().startsWith(prefix());
        - lineno: 19
          match: "}"
        - lineno: 21
          match: public abstract Command parse(String line);
        - lineno: 23
          match: public abstract ProcessorAction processorAction();
        - lineno: 25
          match: "public String parameter(int i) {"
        - lineno: 26
          match: "return parameters[i];"
        - lineno: 27
          match: "}"
        - lineno: 28
          match: "}"
      - path: AnswerCommand.java
        matches:
        - lineno: 1
          match: package xy.ai.workbench.commands;
        - lineno: 3
          match: "public class AnswerCommand extends Command {"
        - lineno: 5
          match: /** Command prefixes recognized by connector.CommandHandler / SessionProcessor. */
        - lineno: 6
          match: public static final String CMD_ANSWER = "/answer";
        - lineno: 8
          match: "public AnswerCommand() {"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 47e1-72 allow
Control Request:
```yaml
id: 47e1-73
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
    ids:
    - CallCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
    ids:
    - CommandHandler
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
    ids:
    - AnswerCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
    ids:
    - Command
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - CCControlClient.submitEdit
    - CCControlClient.extractYamlBlock
    - CCControlClient
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - PromptHandler.captureYamlBlock
    - PromptHandler.applyDetected
    - PromptHandler.detectSelection
    - PromptHandler.detectFullFile
    - PromptHandler.lineText
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - CCConnector
  reason: Details der relevanten Klassen lesen um Kommando-Handling und YAML-Extraktion zu verstehen
```
/answer 47e1-73 allow
Control Request:
```yaml
id: 47e1-74
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
      nodes:
      - id: CallCommand
        type: class_declaration
        signature: "public class CallCommand extends Command {"
        children:
        - id: CallCommand.SX50AH|tAIjYQ
          type: statements
          code: public static final String CMD_CALL   = "/call";
        - id: CallCommand.CallCommand
          type: constructor_declaration
          code: "public CallCommand() {\n\t\tsuper();\n\t}"
        - id: CallCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn CallCommand.CMD_CALL;\n\t}"
        - id: CallCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && prefix().equalsIgnoreCase(line.strip());\n\t}"
        - id: CallCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\treturn new CallCommand();\n\t}"
        - id: CallCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
      nodes:
      - id: CommandHandler
        type: class_declaration
        signature: "public class CommandHandler {"
        children:
        - id: CommandHandler.yMrxSu|gq2BES
          type: statements
          code: "// TODO think about if it should stay like this\n\tprivate static final List<Command> REGISTRY = List.of(new AnswerCommand(), new ControlRequestCommand(),\n\t\t\tnew ResumeCommand(), new ExitCommand(), new ToolCommand(), new CallCommand());"
        - id: CommandHandler.CommandHandler
          type: constructor_declaration
          code: "private CommandHandler() {\n\t}"
        - id: CommandHandler.detect
          type: method_declaration
          code: "public static Command detect(String line) {\n\t\tfor (Command prototype : REGISTRY)\n\t\t\tif (prototype.matches(line))\n\t\t\t\treturn prototype.parse(line);\n\t\treturn null;\n\t}"
        - id: CommandHandler.list
          type: method_declaration
          code: "public static List<Command> list() {\n\t\treturn REGISTRY;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
      nodes:
      - id: AnswerCommand
        type: class_declaration
        signature: "public class AnswerCommand extends Command {"
        children:
        - id: AnswerCommand.yzeuY3|Jf8A1X
          type: statements
          code: "/** Command prefixes recognized by connector.CommandHandler / SessionProcessor. */\n\tpublic static final String CMD_ANSWER = \"/answer\";"
        - id: AnswerCommand.AnswerCommand
          type: constructor_declaration
          code: "public AnswerCommand() {\n\t\tthis(\"\", \"\", \"\");\n\t}"
        - id: AnswerCommand.AnswerCommand_1
          type: constructor_declaration
          code: "public AnswerCommand(String id, String action, String reason) {\n\t\tsuper(id, action, reason);\n\t}"
        - id: AnswerCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn AnswerCommand.CMD_ANSWER;\n\t}"
        - id: AnswerCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\\\\s+(allow|deny)(\\\\s+.*)?\");\n\t}"
        - id: AnswerCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\tString[] parts = line.strip().split(\"\\\\s+\", 4);\n\t\tString id = parts[1];\n\t\tString action = parts[2].toLowerCase();\n\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\treturn new AnswerCommand(id, action, reason);\n\t}"
        - id: AnswerCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.TRANSFORM;\n\t}"
        - id: AnswerCommand.Action
          type: enum_declaration
          code: "public enum Action {\n\t\tAllow, Deny\n\t}"
        - id: AnswerCommand.action
          type: method_declaration
          code: "public Action action() {\n\t\treturn \"allow\".equalsIgnoreCase(parameter(1)) ? Action.Allow : Action.Deny;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
      nodes:
      - id: Command
        type: class_declaration
        signature: "public abstract class Command {"
        children:
        - id: Command.ProcessorAction
          type: enum_declaration
          code: "public enum ProcessorAction {\n\t\tREMOVE, TRANSFORM, IGNORE_BLOCK, NONE\n\t}"
        - id: Command.iKqF62|lJcluI
          type: statements
          code: "protected final String[] parameters;"
        - id: Command.Command
          type: constructor_declaration
          code: "protected Command(String... parameters) {\n\t\tthis.parameters = parameters;\n\t}"
        - id: Command.prefix
          type: method_declaration
          code: public abstract String prefix();
        - id: Command.matches
          type: method_declaration
          code: "public boolean matches(String line) {\n\t\treturn line != null && line.strip().startsWith(prefix());\n\t}"
        - id: Command.parse
          type: method_declaration
          code: public abstract Command parse(String line);
        - id: Command.processorAction
          type: method_declaration
          code: public abstract ProcessorAction processorAction();
        - id: Command.parameter
          type: method_declaration
          code: "public String parameter(int i) {\n\t\treturn parameters[i];\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: CCControlClient.submitEdit
        type: method_declaration
        code: "public boolean submitEdit(String rawText) {\n\t\tif (rawText == null)\n\t\t\treturn false;\n\t\tString block = extractYamlBlock(rawText.strip());\n\t\tif (block == null)\n\t\t\treturn false;\n\n\t\tJsonNode edited;\n\t\ttry {\n\t\t\tedited = fromYaml(block);\n\t\t} catch (Exception e) {\n\t\t\tthrow new IllegalArgumentException(\"YAML Error\", e);\n\t\t}\n\t\tif (edited == null || !edited.isObject() || !edited.hasNonNull(\"id\"))\n\t\t\treturn false;\n\t\tsubmit((ObjectNode) edited);\n\t\treturn true;\n\t}"
      - id: CCControlClient.extractYamlBlock
        type: method_declaration
        code: "private String extractYamlBlock(String text) {\n\t\tint start = text.indexOf(\"```yaml\");\n\t\tif (start != 0)\n\t\t\treturn null;\n\t\tint contentStart = start + \"```yaml\".length();\n\t\tint end = text.indexOf(\"```\", contentStart);\n\t\tif (end == -1)\n\t\t\treturn null;\n\t\treturn text.substring(contentStart, end).strip();\n\t}"
      - id: CCControlClient
        type: class_declaration
        signature: "public class CCControlClient {"
        children:
        - id: CCControlClient.pWyigk|ktvRxK
          type: statements
          code: "private static final String CONTROL_URL = \"http://localhost:9093/control/tool\";\n\tprivate static final Duration TIMEOUT = Duration.ofSeconds(5);\n\n\tprivate final ObjectMapper mapper = JsonUtil.mapper();\n\tprivate YamlRenderer yaml = new YamlRenderer();\n\tprivate final HttpClient http = HttpClient.newBuilder().connectTimeout(TIMEOUT).build();"
        - id: CCControlClient.checkControlEndpoint
          type: method_declaration
          code: "public void checkControlEndpoint(CCResponse resp) {\n\t\tJsonNode pending = poll();\n\t\tif (pending.isEmpty())\n\t\t\treturn;\n\t\tStringBuilder res = new StringBuilder();\n\t\tProtocolParser.appendEvents(resp.events, res);\n\n\t\tJsonNode first = pending.get(0);\n\t\tres.append(String.format(\"%s\\n```yaml\\n%s\\n```\\n%s %s allow\", EditorInterface.CONTROL_REQUEST, toYaml(first), AnswerCommand.CMD_ANSWER,\n\t\t\t\tfirst.path(\"id\").asText()));\n\t\tresp.resultText = res.toString();\n\t}"
        - id: CCControlClient.toYaml
          type: method_declaration
          code: "public String toYaml(JsonNode node) {\n\t\treturn yaml.toYaml(node);\n\t}"
        - id: CCControlClient.fromYaml
          type: method_declaration
          code: "public JsonNode fromYaml(String text) throws JsonProcessingException {\n\t\treturn yaml.readTree(text);\n\t}"
        - id: CCControlClient.isMCPCAvailable
          type: method_declaration
          code: "public boolean isMCPCAvailable() {\n\t\treturn poll() != null; // never null, just check exceptions\n\t}"
        - id: CCControlClient.poll
          type: method_declaration
          code: "private ArrayNode poll() {\n\t\treturn post(mapper.createObjectNode());\n\t}"
        - id: CCControlClient.approve
          type: method_declaration
          code: "public ArrayNode approve(String id) {\n\t\treturn approve(id, null);\n\t}"
        - id: CCControlClient.approve_1
          type: method_declaration
          code: "public ArrayNode approve(String id, String hint) {\n\t\treturn submit(approvalNode(id, null, null, null, hint == null || hint.isBlank() ? null : hint));\n\t}"
        - id: CCControlClient.deny
          type: method_declaration
          code: "public ArrayNode deny(String id, String reason) {\n\t\treturn submit(approvalNode(id, null, null, reason == null ? \"\" : reason, null));\n\t}"
        - id: CCControlClient.submitEdit
          type: method_declaration
          code: "public boolean submitEdit(String rawText) {\n\t\tif (rawText == null)\n\t\t\treturn false;\n\t\tString block = extractYamlBlock(rawText.strip());\n\t\tif (block == null)\n\t\t\treturn false;\n\n\t\tJsonNode edited;\n\t\ttry {\n\t\t\tedited = fromYaml(block);\n\t\t} catch (Exception e) {\n\t\t\tthrow new IllegalArgumentException(\"YAML Error\", e);\n\t\t}\n\t\tif (edited == null || !edited.isObject() || !edited.hasNonNull(\"id\"))\n\t\t\treturn false;\n\t\tsubmit((ObjectNode) edited);\n\t\treturn true;\n\t}"
        - id: CCControlClient.extractYamlBlock
          type: method_declaration
          code: "private String extractYamlBlock(String text) {\n\t\tint start = text.indexOf(\"```yaml\");\n\t\tif (start != 0)\n\t\t\treturn null;\n\t\tint contentStart = start + \"```yaml\".length();\n\t\tint end = text.indexOf(\"```\", contentStart);\n\t\tif (end == -1)\n\t\t\treturn null;\n\t\treturn text.substring(contentStart, end).strip();\n\t}"
        - id: CCControlClient.approvalNode
          type: method_declaration
          code: "private ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result, String rejectReason,\n\t\t\tString hint) {\n\t\tObjectNode approval = mapper.createObjectNode();\n\t\tapproval.put(\"id\", id);\n\t\tif (arguments != null)\n\t\t\tapproval.set(\"arguments\", arguments);\n\t\tif (result != null)\n\t\t\tapproval.set(\"result\", result);\n\t\tif (rejectReason != null) {\n\t\t\tapproval.put(\"rejected\", true);\n\t\t\tapproval.put(\"reason\", rejectReason);\n\t\t}\n\t\tif (hint != null)\n\t\t\tapproval.put(\"hint\", hint);\n\t\treturn approval;\n\t}"
        - id: CCControlClient.submit
          type: method_declaration
          code: "private ArrayNode submit(ObjectNode approval) {\n\t\tObjectNode body = mapper.createObjectNode();\n\t\tbody.putArray(\"approvals\").add(approval);\n\t\treturn post(body);\n\t}"
        - id: CCControlClient.post
          type: method_declaration
          code: "private ArrayNode post(ObjectNode body) {\n\t\ttry {\n\t\t\tString json = JsonUtil.write(body);\n\t\t\tHttpRequest request = HttpRequest.newBuilder().uri(URI.create(CONTROL_URL)).timeout(TIMEOUT)\n\t\t\t\t\t.header(\"Content-Type\", \"application/json\").POST(BodyPublishers.ofString(json)).build();\n\t\t\tHttpResponse<String> response = http.send(request, BodyHandlers.ofString());\n\t\t\tif (response.statusCode() / 100 != 2) {\n\t\t\t\tLOG.error(\"control endpoint returned status \" + response.statusCode());\n\t\t\t\treturn mapper.createArrayNode();\n\t\t\t}\n\t\t\tString responseBody = response.body();\n\t\t\tif (responseBody == null || responseBody.isBlank()) {\n\t\t\t\tLOG.error(\"control endpoint returned an empty body\");\n\t\t\t\treturn mapper.createArrayNode();\n\t\t\t}\n\t\t\tJsonNode root = JsonUtil.readTree(responseBody);\n\t\t\tJsonNode pending = root.path(\"pending\");\n\t\t\treturn pending.isArray() ? (ArrayNode) pending : mapper.createArrayNode();\n\t\t} catch (IOException | InterruptedException e) {\n\t\t\tLOG.error(\"Control endpoint unreachable\", e);\n\t\t\tif (e instanceof InterruptedException)\n\t\t\t\tThread.currentThread().interrupt();\n\t\t\tthrow new IllegalStateException(\"Error on control endpoint\", e);\n\t\t}\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler.captureYamlBlock
        type: method_declaration
        code: "private String captureYamlBlock(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tString prefix = doc.get(0, doc.getLineOffset(lineIndex));\n\t\t\tMatcher m = YAML_BLOCK.matcher(prefix);\n\t\t\tString last = null;\n\t\t\twhile (m.find())\n\t\t\t\tlast = m.group(1);\n\t\t\treturn last;\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn null;\n\t\t}\n\t}"
      - id: PromptHandler.applyDetected
        type: method_declaration
        code: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (cmd == null)\n\t\t\treturn false;\n\t\tdetected[0] = cmd;\n\t\tyamlBlock[0] = captureYamlBlock(doc, lineIndex);\n\t\treturn true;\n\t}"
      - id: PromptHandler.detectSelection
        type: method_declaration
        code: "private Prompt.InputKind detectSelection(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn Prompt.InputKind.LineInput;\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn Prompt.InputKind.LineInput;\n\n\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1) {\n\t\t\tinputs.add(removeCommentLines(tsel.getText()));\n\t\t\t// Block start (multi-line command) takes precedence over a trailing command line.\n\t\t\tif (!applyDetected(CommandHandler.detect(lineText(doc, tsel.getStartLine())), doc, tsel.getStartLine(),\n\t\t\t\t\tdetected, yamlBlock))\n\t\t\t\tapplyDetected(CommandHandler.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(), detected,\n\t\t\t\t\t\tyamlBlock);\n\t\t\treturn Prompt.InputKind.BlockSelection;\n\t\t}\n\n\t\tif (tsel != null) {\n\t\t\ttry {\n\t\t\t\tIRegion lineInfo = doc.getLineInformation(tsel.getEndLine());\n\t\t\t\tinputs.add(doc.get(lineInfo.getOffset(), lineInfo.getLength()));\n\t\t\t} catch (BadLocationException e) {\n\t\t\t\tLOG.error(\"Exception\", e);\n\t\t\t}\n\t\t\tapplyDetected(CommandHandler.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(), detected,\n\t\t\t\t\tyamlBlock);\n\t\t}\n\t\treturn Prompt.InputKind.LineInput;\n\t}"
      - id: PromptHandler.detectFullFile
        type: method_declaration
        code: "private Prompt.InputKind detectFullFile(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn Prompt.InputKind.FullFile;\n\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn Prompt.InputKind.FullFile;\n\t\tinputs.add(doc.get());\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tif (tsel != null\n\t\t\t\t&& applyDetected(CommandHandler.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(),\n\t\t\t\t\t\tdetected, yamlBlock))\n\t\t\treturn Prompt.InputKind.FullFile;\n\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\tapplyDetected(CommandHandler.detect(lineText(doc, lastLine)), doc, lastLine, detected, yamlBlock);\n\t\treturn Prompt.InputKind.FullFile;\n\t}"
      - id: PromptHandler.lineText
        type: method_declaration
        code: "private String lineText(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tIRegion info = doc.getLineInformation(lineIndex);\n\t\t\treturn doc.get(info.getOffset(), info.getLength());\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn \"\";\n\t\t}\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector
        type: class_declaration
        signature: "public class CCConnector implements IAIConnector<CCRequest, CCResponse> {"
        children:
        - id: CCConnector.51iiPA|yBdfkV
          type: statements
          code: "private final CCRequestBuilder requestBuilder = new CCRequestBuilder();\n\tprivate final ProtocolParser jsonParser = new ProtocolParser();\n\tprivate final CCControlClient controlClient = new CCControlClient();\n\tprivate final CCSessionManager sessionManager;"
        - id: CCConnector.CCConnector
          type: constructor_declaration
          code: "public CCConnector(CCSessionManager sessionManager) {\n\t\tthis.sessionManager = sessionManager;\n\t}"
        - id: CCConnector.getSupportedKeyPattern
          type: method_declaration
          code: "@Override\n\tpublic KeyPattern getSupportedKeyPattern() {\n\t\treturn KeyPattern.ClaudeCode;\n\t}"
        - id: CCConnector.createRequest
          type: method_declaration
          code: "@Override\n\tpublic CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Create request\", 1);\n\t\tsub.subTask(\"Detect command\");\n\n\t\tCommand command = prompt.command;\n\t\tif (command == null) {\n\t\t\tString text = prompt.inputs.stream().map(String::strip).filter(s -> !s.isBlank())\n\t\t\t\t\t.collect(Collectors.joining(\"\\n\"));\n\t\t\tif (controlClient.submitEdit(text))\n\t\t\t\tcommand = new ModificationCommand();\n\t\t\telse if (!text.isEmpty())\n\t\t\t\tcommand = new PromptCommand(text);\n\t\t}\n\t\tif (command == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\tsub.worked(1);\n\n\t\tString title = command instanceof PromptCommand pc ? pc.title() : null;\n\t\treturn new CCRequest(UUID.randomUUID().toString(), title, prompt.config, command);\n\t}"
        - id: CCConnector.executeRequest
          type: method_declaration
          code: "@Override\n\tpublic CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Executing prompt\", 2);\n\t\tCCSession session = null;\n\n\t\tEditorLocation loc = getEditorLocation();\n\t\tFrozenConfig fc = req.config;\n\t\tClaudeSessionParameters params = new ClaudeSessionParameters(loc.projectPath, fc.systemPrompt, fc.tools,\n\t\t\t\tfc.model, fc.reasoning, fc.profile, fc.cliProfile, fc.cacheMode, loc.relativeFilePath);\n\t\tparams.setTitle(req.title);\n\n\t\tCommand command = req.command;\n\t\tif (command instanceof ResumeCommand rc) {\n\t\t\tsub.subTask(\"Importing session\");\n\t\t\tsessionManager.importSession(rc.parameter(0), params);\n\t\t\treturn new CCResponse(req.id, \"Session created\");\n\t\t}\n\t\tif (command instanceof ExitCommand) {\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t\tsub.subTask(\"Terminating CLI process\");\n\t\t\tsession.terminate();\n\t\t\treturn new CCResponse(req.id, \"Session closed!\");\n\t\t}\n\t\tif (command instanceof AnswerCommand ac) {\n\t\t\tif (ac.action() == AnswerCommand.Action.Allow)\n\t\t\t\tcontrolClient.approve(ac.parameter(0), ac.parameter(2));\n\t\t\telse\n\t\t\t\tcontrolClient.deny(ac.parameter(0), ac.parameter(2));\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t} else if (command instanceof ModificationCommand) {\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params); // already sent\n\t\t} else if (command instanceof PromptCommand) {\n\t\t\tsub.subTask(\"Acquiring session\");\n\t\t\tsession = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t} else {\n\t\t\tthrow new UnsupportedOperationException(\"Unsupported command: \" + command.prefix());\n\t\t}\n\n\t\ttry {\n\t\t\tsession.setInPrompt(true);\n\t\t\tif (command instanceof PromptCommand pc) {\n\t\t\t\tif (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !controlClient.isMCPCAvailable())\n\t\t\t\t\tthrow new IllegalStateException(\"MCPC not reachable for AgentProfile\");\n\n\t\t\t\tsub.subTask(\"Sending prompt\");\n\t\t\t\tsession.writeLine(requestBuilder.buildPromptJson(pc.parameter(0)));\n\t\t\t}\n\n\t\t\tsub.subTask(\"Waiting for answer\");\n\t\t\treturn readUntilResult(req, session, sub.split(1), job);\n\n\t\t} catch (IOException e) {\n\t\t\tthrow new IllegalStateException(\"Claude Code CLI error\", e);\n\t\t} finally {\n\t\t\tsession.setInPrompt(false);\n\t\t}\n\t}"
        - id: CCConnector.readUntilResult
          type: method_declaration
          code: "private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressMonitor mon, Job job) throws IOException {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Reading Claude output\", 10000);\n\t\tCCResponse resp = new CCResponse(req.id);\n\n\t\tString line;\n\t\twhile (true) {\n\t\t\t// alternate read sources undtil answer\n\t\t\ttry {\n\t\t\t\t// wait 300 ms\n\t\t\t\tif ((line = session.readLine()) != null) {\n\t\t\t\t\tsession.setLastRawLine(line);\n\t\t\t\t\tjob.setName(abbreviateForStatus(line));\n\t\t\t\t\tmon.worked(1);\n\t\t\t\t\tjsonParser.parseLine(resp, session, sub, line);\n\n\t\t\t\t}\n\t\t\t} catch (Exception ex) {\n\t\t\t\tLOG.error(\"Error reading line: \", ex);\n\t\t\t\ttry {\n\t\t\t\t\twhile ((line = session.readError()) != null)\n\t\t\t\t\t\tLOG.error(\"CLI stderr: \" + line);\n\t\t\t\t} catch (Exception ex2) {\n\t\t\t\t\tLOG.error(\"Error reading stderr: \", ex2);\n\t\t\t\t}\n\t\t\t\tresp.resultText = ex.getMessage();\n\t\t\t}\n\n\t\t\tif (!resp.isReady())\n\t\t\t\tcontrolClient.checkControlEndpoint(resp);\n\n\t\t\tif (resp.isReady()) {\n\t\t\t\tsession.stats.add(resp.stats);\n\t\t\t\tsession.stats.totalToken = session.stats.inputToken + session.stats.cacheCreate;\n\t\t\t\treturn resp;\n\t\t\t}\n\t\t\tsessionManager.onSessionChanged(session);\n\t\t}\n\t}"
        - id: CCConnector.abbreviateForStatus
          type: method_declaration
          code: "private static String abbreviateForStatus(String s) {\n\t\tif (s == null)\n\t\t\treturn \"null\";\n\t\tCRC32 crc = new CRC32();\n\t\tcrc.update(s.getBytes());\n\t\treturn String.format(\"Prompting... (%d/%d)\", s.length(), crc.getValue());\n\t}"
        - id: CCConnector.convertResponse
          type: method_declaration
          code: "@Override\n\tpublic AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {\n\t\tAIAnswer answer = new AIAnswer(resp.id);\n\t\tanswer.stats.inputToken = resp.stats.inputToken;\n\t\tanswer.stats.outputToken = resp.stats.outputToken;\n\t\tanswer.stats.reasoningToken = resp.stats.reasoningToken;\n\t\tanswer.stats.cacheRead = resp.stats.cacheRead;\n\t\tanswer.stats.cacheCreate = resp.stats.cacheCreate;\n\t\tanswer.stats.totalToken = answer.stats.inputToken + answer.stats.cacheCreate + answer.stats.outputToken;\n\t\tanswer.answer = resp.resultText;\n\t\treturn answer;\n\t}"
        - id: CCConnector.getEditorLocation
          type: method_declaration
          code: "private EditorLocation getEditorLocation() {\n\t\tEditorLocation result = new EditorLocation();\n\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\ttry {\n\t\t\t\tIWorkbenchWindow window = PlatformUI.getWorkbench().getActiveWorkbenchWindow();\n\t\t\t\tif (window == null)\n\t\t\t\t\treturn;\n\t\t\t\tIWorkbenchPage page = window.getActivePage();\n\t\t\t\tif (page == null)\n\t\t\t\t\treturn;\n\t\t\t\tIEditorPart editor = page.getActiveEditor();\n\t\t\t\tif (editor == null)\n\t\t\t\t\treturn;\n\t\t\t\tIEditorInput editorInput = editor.getEditorInput();\n\t\t\t\tif (!(editorInput instanceof IFileEditorInput))\n\t\t\t\t\tthrow new IllegalArgumentException(\"Connector does not support external files\");\n\n\t\t\t\tIFileEditorInput fileInput = (IFileEditorInput) editorInput;\n\t\t\t\tIProject project = fileInput.getFile().getProject();\n\t\t\t\tPath projectPath = Paths.get(project.getLocation().toOSString());\n\t\t\t\tString relativeFilePath = fileInput.getFile().getProjectRelativePath().toString();\n\t\t\t\tresult.set(projectPath, relativeFilePath);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(\"Failed to resolve editor paths\", e);\n\t\t\t}\n\t\t});\n\t\tif (result.projectPath == null)\n\t\t\tthrow new IllegalStateException(\"Failed to resolve editor paths\");\n\t\treturn result;\n\t}"
```
/answer 47e1-74 allow
Control Request:
```yaml
id: 47e1-75
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - PromptHandler
    - 4UXeuu|NTMCrV
    - fPHG2Y|yRFIp4
  reason: "Vollständigen PromptHandler-Code inkl. buildPrompt, Imports, Felder lesen"
```
/answer 47e1-75 allow
Control Request:
```yaml
id: 47e1-76
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler
        type: class_declaration
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.dE1Gmz|tJO7uC
          type: statements
          code: "// Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block before a command line.\n\tprivate static final Pattern YAML_BLOCK = Pattern.compile(\"^```yaml\\\\R(.*?)^```$\", Pattern.MULTILINE | Pattern.DOTALL);\n\n\tprivate final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final EditorInterface editIfc;\n\tprivate final IncludeAdapter includeAdapter;\n\n\tprivate int[] inputStats = new int[InputMode.values().length];"
        - id: PromptHandler.iAEUAf|LdjI0C
          type: statements
          code: "private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();\n\tprivate List<Consumer<int[]>> inputStatObs = new ArrayList<>();"
        - id: PromptHandler.PromptHandler
          type: constructor_declaration
          code: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector,\n\t\t\tActiveEditorListener editorListener, EditorInterface editIfc, IncludeAdapter includeAdapter) {\n\t\tthis.cfg = cfg;\n\t\tthis.connector = connector;\n\t\tthis.editorListener = editorListener;\n\t\tthis.editIfc = editIfc;\n\t\tthis.includeAdapter = includeAdapter;\n\t}"
        - id: PromptHandler.clearObserver
          type: method_declaration
          code: "public void clearObserver() {\n\t\tanswerObs.clear();\n\t\tinputStatObs.clear();\n\t}"
        - id: PromptHandler.addInputStatObs
          type: method_declaration
          code: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {\n\t\tinputStatObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(inputStats);\n\t}"
        - id: PromptHandler.addAnswerObs
          type: method_declaration
          code: "public void addAnswerObs(Consumer<AIAnswer> obs) {\n\t\tanswerObs.add(obs);\n\t}"
        - id: PromptHandler.updateInputStat
          type: method_declaration
          code: "public void updateInputStat(InputMode mode) {\n\t\tif (mode == InputMode.Tools) {\n\t\t\tString[] tools = cfg.getTools();\n\t\t\tinputStats[mode.ordinal()] = tools != null ? tools.length : 0;\n\t\t} else {\n\t\t\tString input = getInput(mode);\n\t\t\tinputStats[mode.ordinal()] = input != null ? input.length() : -1;\n\t\t}\n\t\tinputStatObs.forEach(c -> c.accept(inputStats));\n\t}"
        - id: PromptHandler.initializeInputs
          type: method_declaration
          code: "public void initializeInputs() {\n\t\tfor (var mode : InputMode.values())\n\t\t\tupdateInputStat(mode);\n\t\tincludeAdapter.initializeInputs();\n\t}"
        - id: PromptHandler.getInput
          type: method_declaration
          code: "private String getInput(InputMode mode) {\n\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\n\t\tswitch (mode) {\n\t\tcase SystemPrompt:\n\t\t\tStringBuffer systemPrompt = new StringBuffer();\n\t\t\tArrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith(\"#\"))\n\t\t\t\t\t.forEach(e -> systemPrompt.append(\"* \").append(e).append(\".\\n\"));\n\t\t\tString freeText = cfg.getFreeText();\n\t\t\tif (freeText != null && !freeText.isBlank())\n\t\t\t\tsystemPrompt.append(\".\\n\").append(cfg.getFreeText()).append(\".\\n\");\n\t\t\tString prompttext = systemPrompt.toString();\n\t\t\treturn prompttext.length() > 0 && !prompttext.isBlank() ? prompttext : null;\n\t\tcase Selection:\n\t\t\tif (textEditor != null) {\n\t\t\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\t\t\tif (selectionProvider != null) {\n\t\t\t\t\tISelection selection = selectionProvider.getSelection();\n\t\t\t\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\t\t\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1)\n\t\t\t\t\t\treturn removeCommentLines(tsel.getText());\n\n\t\t\t\t\tif (tsel != null) {\n\t\t\t\t\t\tint line = tsel.getEndLine();\n\t\t\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\tIRegion lineInfo = doc.getLineInformation(line);\n\t\t\t\t\t\t\treturn doc.get(lineInfo.getOffset(), lineInfo.getLength());\n\t\t\t\t\t\t} catch (BadLocationException e1) {\n\t\t\t\t\t\t\tLOG.error(\"Exception\", e1);\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t\tbreak;\n\t\tcase Converter:\n\t\t\tif (textEditor != null) {\n\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\tif (doc != null)\n\t\t\t\t\treturn doc.get();\n\t\t\t}\n\t\t\tbreak;\n\t\tcase Tools:\n\t\t\tthrow new UnsupportedOperationException();\n\t\t}\n\t\treturn null;\n\t}"
        - id: PromptHandler.removeCommentLines
          type: method_declaration
          code: "private String removeCommentLines(String input) {\n\t\tif (input == null || input.isEmpty())\n\t\t\treturn input;\n\n\t\tStringBuffer result = new StringBuffer();\n\t\tString[] lines = input.split(\"\\\\R\");\n\n\t\tfor (String line : lines)\n\t\t\tif (!line.trim().startsWith(AbstractRule.LINE_COMMENT))\n\t\t\t\tresult.append(line).append(System.lineSeparator());\n\n\t\treturn result.toString();\n\t}"
        - id: PromptHandler.execute
          type: method_declaration
          code: "public void execute(Display display) {\n\t\tnew PromptJob(\"Starting Prompt\", display).schedule();\n\t}"
        - id: PromptHandler.PromptJob
          type: class_declaration
          signature: "private class PromptJob extends Job {"
          children:
          - id: PromptHandler.PromptJob.zOXTQk|KFkQPW
            type: statements
            code: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            type: constructor_declaration
            code: "private PromptJob(String name, Display display) {\n\t\t\tsuper(name);\n\t\t\tthis.display = display;\n\t\t}"
          - id: PromptHandler.PromptJob.run
            type: method_declaration
            code: "@Override\n\t\tprotected IStatus run(IProgressMonitor mon) {\n\t\t\tSubMonitor sub = SubMonitor.convert(mon, \"Executing prompt\", 4);\n\t\t\tString reqId = null;\n\t\t\ttry {\n\t\t\t\tsub.subTask(\"Prepare inputs\");\n\t\t\t\tvar req = prepareRequest(display, false, sub);\n\t\t\t\tsub.worked(1);\n\t\t\t\tsub.subTask(\"Insert Tag\");\n\t\t\t\teditIfc.insertTag(display, req, sub);\n\t\t\t\treqId = req.getID();\n\t\t\t\tmon.worked(1);\n\n\t\t\t\tsub.subTask(\"Execute prompt\");\n\t\t\t\tvar ans = executeInner(display, req, sub, this);\n\t\t\t\tmon.worked(1);\n\t\t\t\tsub.subTask(\"Process Answer\");\n\t\t\t\teditIfc.replaceTag(display, ans, sub);\n\t\t\t\tmon.worked(1);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t// uncatched error case\n\t\t\t\tif (reqId != null) {\n\t\t\t\t\tAIAnswer error = new AIAnswer(reqId);\n\t\t\t\t\terror.answer = e.getMessage();\n\t\t\t\t\teditIfc.replaceTag(display, error, sub);\n\t\t\t\t}\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}"
        - id: PromptHandler.queueAsync
          type: method_declaration
          code: "public void queueAsync(Display display, AIBatchManager batch) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, batch, mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.queueSync
          type: method_declaration
          code: "private void queueSync(Display display, AIBatchManager batch, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Enqueue batch prompt\", 3);\n\t\tsub.subTask(\"Prepare inputs\");\n\t\tvar req = prepareRequest(display, true, sub.split(1));\n\t\tsub.subTask(\"Insert Tag\");\n\t\teditIfc.insertTag(display, req, sub.split(1));\n\t\tsub.subTask(\"Enqueue prompt\");\n\t\tbatch.enqueue(req, sub.split(1));\n\t}"
        - id: PromptHandler.queueAndSubmit
          type: method_declaration
          code: "public void queueAndSubmit(Display display, AIBatchManager batch) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, batch, mon);\n\t\t\t\tbatch.submitBatches(mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.prepareRequest
          type: method_declaration
          code: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Preparing Call\", 1);\n\t\tsub.subTask(\"Preparing Call\");\n\t\tPrompt prompt = buildPrompt(display, batchFix);\n\t\tIModelRequest req = connector.createRequest(prompt, sub);\n\t\tsub.worked(1);\n\t\treturn req;\n\t}"
        - id: PromptHandler.maceLN|sqs7wQ
          type: statements
          code: "/** Freezes the current editor/config state into a {@link Prompt}; runs command detection (see class doc). */"
        - id: PromptHandler.buildPrompt
          type: method_declaration
          code: "private Prompt buildPrompt(Display display, boolean batch) {\n\t\tList<String> inputs = new ArrayList<>();\n\t\tString[] absoluteFilePath = new String[1];\n\t\tPrompt.InputKind[] kind = new Prompt.InputKind[1];\n\t\tCommand[] detected = new Command[1];\n\t\tString[] yamlBlock = new String[1];\n\n\t\tdisplay.syncExec(() -> {\n\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\tabsoluteFilePath[0] = resolveAbsoluteFilePath(textEditor);\n\n\t\t\tif (cfg.isInputEnabled(InputMode.Selection))\n\t\t\t\tkind[0] = detectSelection(textEditor, inputs, detected, yamlBlock);\n\t\t\telse if (cfg.isInputEnabled(InputMode.Converter))\n\t\t\t\tkind[0] = detectFullFile(textEditor, inputs, detected, yamlBlock);\n\t\t});\n\n\t\tFrozenConfig frozen = FrozenConfig.from(cfg);\n\n\t\tif (inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank()))\n\t\t\tthrow new IllegalArgumentException(\"Input and System Prompt Empty\");\n\n\t\t// TODO maybe store result editor ref in prompt\n\t\tif (editorListener.getLastTextEditor() == null && !batch)\n\t\t\tthrow new IllegalArgumentException(\"Result editor unset\");\n\n\t\tPrompt.Builder builder = Prompt.builder()//\n\t\t\t\t.inputs(inputs)//\n\t\t\t\t.kind(kind[0])//\n\t\t\t\t.batch(batch)//\n\t\t\t\t.config(frozen)//\n\t\t\t\t.processorEnabled(cfg.isInputEnabled(InputMode.Converter))//\n\t\t\t\t.absoluteFilePath(absoluteFilePath[0]);\n\n\t\tbuilder.command(detected[0]).yamlBlock(yamlBlock[0]);\n\n\t\treturn builder.build();\n\t}"
        - id: PromptHandler.ohnvjH|nhSFvk
          type: statements
          code: "// Selection mode: a real (multi-char) selection is a BlockSelection, an empty/caret selection\n\t// falls back to the current cursor line (LineInput)."
        - id: PromptHandler.detectSelection
          type: method_declaration
          code: "private Prompt.InputKind detectSelection(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn Prompt.InputKind.LineInput;\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn Prompt.InputKind.LineInput;\n\n\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1) {\n\t\t\tinputs.add(removeCommentLines(tsel.getText()));\n\t\t\t// Block start (multi-line command) takes precedence over a trailing command line.\n\t\t\tif (!applyDetected(CommandHandler.detect(lineText(doc, tsel.getStartLine())), doc, tsel.getStartLine(),\n\t\t\t\t\tdetected, yamlBlock))\n\t\t\t\tapplyDetected(CommandHandler.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(), detected,\n\t\t\t\t\t\tyamlBlock);\n\t\t\treturn Prompt.InputKind.BlockSelection;\n\t\t}\n\n\t\tif (tsel != null) {\n\t\t\ttry {\n\t\t\t\tIRegion lineInfo = doc.getLineInformation(tsel.getEndLine());\n\t\t\t\tinputs.add(doc.get(lineInfo.getOffset(), lineInfo.getLength()));\n\t\t\t} catch (BadLocationException e) {\n\t\t\t\tLOG.error(\"Exception\", e);\n\t\t\t}\n\t\t\tapplyDetected(CommandHandler.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(), detected,\n\t\t\t\t\tyamlBlock);\n\t\t}\n\t\treturn Prompt.InputKind.LineInput;\n\t}"
        - id: PromptHandler.Txg3oX|3Xw56U
          type: statements
          code: "// Processor (full file) mode: the caret line is checked first, then the last line of the file,\n\t// matching a command appended after the generated content."
        - id: PromptHandler.detectFullFile
          type: method_declaration
          code: "private Prompt.InputKind detectFullFile(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn Prompt.InputKind.FullFile;\n\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn Prompt.InputKind.FullFile;\n\t\tinputs.add(doc.get());\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tif (tsel != null\n\t\t\t\t&& applyDetected(CommandHandler.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(),\n\t\t\t\t\t\tdetected, yamlBlock))\n\t\t\treturn Prompt.InputKind.FullFile;\n\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\tapplyDetected(CommandHandler.detect(lineText(doc, lastLine)), doc, lastLine, detected, yamlBlock);\n\t\treturn Prompt.InputKind.FullFile;\n\t}"
        - id: PromptHandler.applyDetected
          type: method_declaration
          code: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (cmd == null)\n\t\t\treturn false;\n\t\tdetected[0] = cmd;\n\t\tyamlBlock[0] = captureYamlBlock(doc, lineIndex);\n\t\treturn true;\n\t}"
        - id: PromptHandler.lineText
          type: method_declaration
          code: "private String lineText(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tIRegion info = doc.getLineInformation(lineIndex);\n\t\t\treturn doc.get(info.getOffset(), info.getLength());\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn \"\";\n\t\t}\n\t}"
        - id: PromptHandler.Q5D2ne|VNsv72
          type: statements
          code: // Walks backwards from the command line and keeps the closest preceding ```yaml block.
        - id: PromptHandler.captureYamlBlock
          type: method_declaration
          code: "private String captureYamlBlock(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tString prefix = doc.get(0, doc.getLineOffset(lineIndex));\n\t\t\tMatcher m = YAML_BLOCK.matcher(prefix);\n\t\t\tString last = null;\n\t\t\twhile (m.find())\n\t\t\t\tlast = m.group(1);\n\t\t\treturn last;\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn null;\n\t\t}\n\t}"
        - id: PromptHandler.resolveAbsoluteFilePath
          type: method_declaration
          code: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {\n\t\tif (textEditor == null)\n\t\t\treturn null;\n\t\tIEditorInput input = textEditor.getEditorInput();\n\t\tif (input instanceof IFileEditorInput)\n\t\t\treturn ((IFileEditorInput) input).getFile().getLocation().toFile().getAbsolutePath();\n\t\tif (input instanceof IURIEditorInput)\n\t\t\treturn new File(((IURIEditorInput) input).getURI()).getAbsolutePath();\n\t\treturn null;\n\t}"
        - id: PromptHandler.executeInner
          type: method_declaration
          code: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon, PromptJob job) {\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));\n\t\tIModelResponse resp = connector.executeRequest(req, mon, job);\n\t\tAIAnswer res = connector.convertResponse(resp, mon);\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));\n\t\treturn res;\n\t}"
      - id: 4UXeuu|NTMCrV
        type: imports
        code: |-
          import org.eclipse.jface.text.IDocument;
          import org.eclipse.jface.text.IRegion;
          import org.eclipse.jface.text.ITextSelection;
          import org.eclipse.jface.viewers.ISelection;
          import org.eclipse.jface.viewers.ISelectionProvider;
          import org.eclipse.swt.widgets.Display;
          import org.eclipse.ui.IEditorInput;
          import org.eclipse.ui.IFileEditorInput;
          import org.eclipse.ui.IURIEditorInput;
          import org.eclipse.ui.texteditor.ITextEditor;

          import xy.ai.workbench.ActiveEditorListener;
          import xy.ai.workbench.ConfigManager;
      - id: fPHG2Y|yRFIp4
        type: statements
        code: |-
          /**
           * Prompt-specific orchestration extracted from {@code AISessionManager}: builds a frozen
           * {@link Prompt} from the active editor/selection, runs command detection and drives the
           * prepare/insertTag/execute/replaceTag job pipeline.
           */
```
/answer 47e1-76 allow
ReasoningToken: 121
ReasoningToken: 132
Control Request:
```yaml
id: 47e1-77
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: CallCommand|yamlBlock|YamlBlock|CMD_CALL
    include:
    - '*.java'
    limit: 50
  reason: Alle Verwendungen von CallCommand und yamlBlock im Projekt finden
```
/answer 47e1-77 allow
Control Request:
```yaml
id: 47e1-78
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/mcp/MCPClient.java
        matches:
        - lineno: 69
          match: return yaml.toYamlBlock(call);
      - path: xy/ai/workbench/connector/mcp/MCPControlClient.java
        matches:
        - lineno: 13
          match: import xy.ai.workbench.commands.CallCommand;
        - lineno: 56
          match: sb.append(CallCommand.CMD_CALL);
        - lineno: 61
          match: "public String extractYamlBlock(String text) {"
      - path: xy/ai/workbench/connector/mcp/MCPRequest.java
        matches:
        - lineno: 15
          match: public final String yamlBlock;
        - lineno: 17
          match: "public MCPRequest(String id, String systemPrompt, List<String> tools, Command command, String yamlBlock) {"
        - lineno: 23
          match: this.yamlBlock = yamlBlock;
      - path: xy/ai/workbench/connector/mcp/MCPConnector.java
        matches:
        - lineno: 12
          match: import xy.ai.workbench.commands.CallCommand;
        - lineno: 41
          match: '&& !(command instanceof CallCommand))'
        - lineno: 45
          match: prompt.yamlBlock);
        - lineno: 60
          match: "if (req.command instanceof CallCommand) {"
        - lineno: 61
          match: if (req.yamlBlock == null || req.yamlBlock.isBlank())
        - lineno: 63
          match: "return invokeCall(req.id, control.parseYaml(req.yamlBlock));"
      - path: xy/ai/workbench/connector/claudecode/YamlRenderer.java
        matches:
        - lineno: 55
          match: "public String toYamlBlock(JsonNode node) {"
      - path: xy/ai/workbench/connector/claudecode/CCControlClient.java
        matches:
        - lineno: 85
          match: String block = extractYamlBlock(rawText.strip());
        - lineno: 101
          match: "private String extractYamlBlock(String text) {"
      - path: xy/ai/workbench/connector/harness/SessionRenderer.java
        matches:
        - lineno: 37
          match: return EditorInterface.TOOLUSE + "\n" + YAML.toYamlBlock(node);
        - lineno: 44
          match: return EditorInterface.TOOLRESULT + "\n" + YAML.toYamlBlock(node);
      - path: xy/ai/workbench/connector/harness/PromptHandler.java
        matches:
        - lineno: 267
          match: "String[] yamlBlock = new String[1];"
        - lineno: 274
          match: "kind[0] = detectSelection(textEditor, inputs, detected, yamlBlock);"
        - lineno: 276
          match: "kind[0] = detectFullFile(textEditor, inputs, detected, yamlBlock);"
        - lineno: 296
          match: "builder.command(detected[0]).yamlBlock(yamlBlock[0]);"
        - lineno: 304
          match: "String[] yamlBlock) {"
        - lineno: 319
          match: "detected, yamlBlock))"
        - lineno: 321
          match: yamlBlock);
        - lineno: 333
          match: yamlBlock);
        - lineno: 341
          match: "String[] yamlBlock) {"
        - lineno: 355
          match: "detected, yamlBlock))"
        - lineno: 359
          match: "applyDetected(CommandHandler.detect(lineText(doc, lastLine)), doc, lastLine, detected, yamlBlock);"
        - lineno: 364
          match: "String[] yamlBlock) {"
        - lineno: 368
          match: "yamlBlock[0] = captureYamlBlock(doc, lineIndex);"
        - lineno: 383
          match: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
      - path: xy/ai/workbench/connector/harness/Prompt.java
        matches:
        - lineno: 31
          match: public final String yamlBlock;
        - lineno: 36
          match: "String absoluteFilePath, Command command, String yamlBlock) {"
        - lineno: 48
          match: this.yamlBlock = yamlBlock;
        - lineno: 104
          match: private String yamlBlock;
        - lineno: 144
          match: "public Builder yamlBlock(String yamlBlock) {"
        - lineno: 145
          match: this.yamlBlock = yamlBlock;
        - lineno: 150
          match: "return new Prompt(inputs, kind, batch, config, processorEnabled, absoluteFilePath, command, yamlBlock);"
      - path: xy/ai/workbench/commands/CallCommand.java
        matches:
        - lineno: 3
          match: "public class CallCommand extends Command {"
        - lineno: 5
          match: public static final String CMD_CALL   = "/call";
        - lineno: 7
          match: "public CallCommand() {"
        - lineno: 13
          match: return CallCommand.CMD_CALL;
        - lineno: 23
          match: return new CallCommand();
      - path: xy/ai/workbench/commands/CommandHandler.java
        matches:
        - lineno: 9
          match: "new ResumeCommand(), new ExitCommand(), new ToolCommand(), new CallCommand());"
```
/answer 47e1-78 allow
ReasoningToken: 1456
Control Request:
```yaml
id: 47e1-79
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - MCPConnector
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
    ids:
    - MCPControlClient
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    ids:
    - MCPRequest
  reason: MCP-Referenzimplementierung von CallCommand-Unterstützung vollständig lesen als Vorlage
```
/answer 47e1-79 allow
Control Request:
```yaml
id: 47e1-7a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: MCPConnector
        type: class_declaration
        signature: "public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {"
        children:
        - id: MCPConnector.hOPGQb|xV4uSy
          type: statements
          code: "private final MCPClient client;\n\tprivate final MCPControlClient control = new MCPControlClient();\n\n\t@SuppressWarnings(\"unused\")\n\tprivate final ConfigManager cfg;"
        - id: MCPConnector.MCPConnector
          type: constructor_declaration
          code: "public MCPConnector(ConfigManager cfg, MCPClient client) {\n\t\tthis.cfg = cfg;\n\t\tthis.client = client;\n\t}"
        - id: MCPConnector.getSupportedKeyPattern
          type: method_declaration
          code: "@Override\n\tpublic KeyPattern getSupportedKeyPattern() {\n\t\treturn KeyPattern.Misc;\n\t}"
        - id: MCPConnector.createRequest
          type: method_declaration
          code: "@Override\n\tpublic MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tCommand command = prompt.command;\n\t\tif (!(command instanceof ExitCommand) && !(command instanceof ToolCommand)\n\t\t\t\t&& !(command instanceof CallCommand))\n\t\t\tthrow new IllegalArgumentException(\n\t\t\t\t\t\"No command detected. Use \\\"/exit\\\", \\\"/tool <id>\\\" or \\\"/call\\\".\");\n\t\treturn new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools, command,\n\t\t\t\tprompt.yamlBlock);\n\t}"
        - id: MCPConnector.executeRequest
          type: method_declaration
          code: "@Override\n\tpublic MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job) {\n\t\tif (req.command instanceof ExitCommand) {\n\t\t\tclient.close();\n\t\t\treturn new MCPResponse(req.id, \"MCP connection closed\");\n\t\t}\n\t\tif (req.command instanceof ToolCommand tc) {\n\t\t\tJsonNode tool = client.findTool(tc.parameter(0));\n\t\t\tif (tool == null)\n\t\t\t\tthrow new IllegalArgumentException(\"Unknown tool: \" + tc.parameter(0));\n\t\t\treturn new MCPResponse(req.id, control.renderSchema(tool));\n\t\t}\n\t\tif (req.command instanceof CallCommand) {\n\t\t\tif (req.yamlBlock == null || req.yamlBlock.isBlank())\n\t\t\t\tthrow new IllegalArgumentException(\"No preceding ```yaml tool call block found before /call\");\n\t\t\treturn invokeCall(req.id, control.parseYaml(req.yamlBlock));\n\t\t}\n\t\tthrow new UnsupportedOperationException(\"Unsupported command: \" + req.command.prefix());\n\t}"
        - id: MCPConnector.invokeCall
          type: method_declaration
          code: "private MCPResponse invokeCall(String id, JsonNode call) {\n\t\tString name = call.path(\"tool\").asText(null);\n\t\tif (name == null || name.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"Tool call is missing the 'tool' field\");\n\t\tJsonNode tool = client.findTool(name);\n\t\tJsonNode arguments = control.fillReason(tool, call.path(\"arguments\"));\n\t\tJsonNode result = client.callTool(name, arguments);\n\t\treturn new MCPResponse(id, control.prettyResult(result));\n\t}"
        - id: MCPConnector.convertResponse
          type: method_declaration
          code: "@Override\n\tpublic AIAnswer convertResponse(MCPResponse resp, IProgressMonitor mon) {\n\t\tAIAnswer answer = new AIAnswer(resp.id);\n\t\tanswer.answer = resp.resultText;\n\t\treturn answer;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
      nodes:
      - id: MCPControlClient
        type: class_declaration
        signature: "public class MCPControlClient {"
        children:
        - id: MCPControlClient.JloL0D|yKZBpv
          type: statements
          code: "private final YamlRenderer yaml = new YamlRenderer();\n\n\t/**\n\t * Renders a tool's input schema as a YAML template (fenced block). Everything\n\t * required for the call is bare YAML; descriptions and constraints are\n\t * comments; defaults are pre-filled.\n\t */"
        - id: MCPControlClient.renderSchema
          type: method_declaration
          code: "@SuppressWarnings(\"deprecation\")\n\tpublic String renderSchema(JsonNode tool) {\n\t\tStringBuilder sb = new StringBuilder();\n\t\tsb.append(EditorInterface.TOOLUSE).append(\"\\n\");\n\t\tsb.append(\"```yaml\\n\");\n\t\tsb.append(\"tool: \").append(tool.path(\"name\").asText()).append(\"\\n\");\n\n\t\tJsonNode schema = tool.path(\"inputSchema\");\n\t\tJsonNode props = schema.path(\"properties\");\n\t\tSet<String> required = requiredSet(schema);\n\t\tStringBuilder argsBody = new StringBuilder();\n\t\tif (props.isObject()) {\n\t\t\tIterator<Map.Entry<String, JsonNode>> it = props.fields();\n\t\t\twhile (it.hasNext()) {\n\t\t\t\tMap.Entry<String, JsonNode> e = it.next();\n\t\t\t\tif (\"reason\".equals(e.getKey()))\n\t\t\t\t\tcontinue;\n\t\t\t\tappendProperty(argsBody, e.getKey(), e.getValue(), required.contains(e.getKey()));\n\t\t\t}\n\t\t}\n\t\tif (argsBody.length() > 0) {\n\t\t\tsb.append(\"arguments:\\n\").append(argsBody);\n\t\t} else {\n\t\t\tsb.append(\"arguments: {}\\n\");\n\t\t}\n\t\tsb.append(\"```\\n\");\n\t\tsb.append(CallCommand.CMD_CALL);\n\t\treturn sb.toString();\n\t}"
        - id: MCPControlClient.dKZb74|tHZasV
          type: statements
          code: "/** Extracts the content of a leading/embedded ```yaml block, or null. */"
        - id: MCPControlClient.extractYamlBlock
          type: method_declaration
          code: "public String extractYamlBlock(String text) {\n\t\tif (text == null)\n\t\t\treturn null;\n\t\tString t = text.strip();\n\t\tint start = t.indexOf(\"```yaml\");\n\t\tif (start < 0)\n\t\t\treturn null;\n\t\tint contentStart = start + \"```yaml\".length();\n\t\tint end = t.indexOf(\"```\", contentStart);\n\t\tif (end == -1)\n\t\t\treturn null;\n\t\treturn t.substring(contentStart, end).strip();\n\t}"
        - id: MCPControlClient.parseYaml
          type: method_declaration
          code: "public JsonNode parseYaml(String block) {\n\t\ttry {\n\t\t\tJsonNode node = yaml.readTree(block);\n\t\t\tif (node == null || !node.isObject())\n\t\t\t\tthrow new IllegalArgumentException(\"Tool call YAML must be a mapping\");\n\t\t\treturn node;\n\t\t} catch (JsonProcessingException e) {\n\t\t\tthrow new IllegalArgumentException(\"Invalid YAML tool call: \" + e.getMessage(), e);\n\t\t}\n\t}"
        - id: MCPControlClient.prettyResult
          type: method_declaration
          code: "public String prettyResult(JsonNode result) {\n\t\tJsonNode structured = result.path(\"structuredContent\");\n\t\tif (!structured.isMissingNode() && !structured.isNull())\n\t\t\treturn \"```json\\n\" + JsonUtil.pretty(structured) + \"\\n```\";\n\n\t\tJsonNode content = result.path(\"content\");\n\t\tif (content.isArray())\n\t\t\treturn extractText(content);\n\n\t\treturn \"```json\\n\" + JsonUtil.pretty(result) + \"\\n```\";\n\t}"
        - id: MCPControlClient.khCRJ6|sVRL8A
          type: statements
          code: "/** Concatenates the \"text\" entries of an MCP \"content\" array (plain, unfenced). */"
        - id: MCPControlClient.extractText
          type: method_declaration
          code: "public String extractText(JsonNode content) {\n\t\tStringBuilder sb = new StringBuilder();\n\t\tfor (JsonNode c : content) {\n\t\t\tif (sb.length() > 0)\n\t\t\t\tsb.append(\"\\n\");\n\t\t\tsb.append(c.path(\"text\").asText(\"\"));\n\t\t}\n\t\treturn sb.toString();\n\t}"
        - id: MCPControlClient.82vyIy|L43zbT
          type: statements
          code: "/**\n\t * Ensures a \"reason\" argument is present when the tool's schema declares one,\n\t * since it is filtered out of the rendered template and never filled in by\n\t * the user.\n\t */"
        - id: MCPControlClient.fillReason
          type: method_declaration
          code: "public JsonNode fillReason(JsonNode tool, JsonNode arguments) {\n\t\tObjectNode args = arguments != null && arguments.isObject() ? (ObjectNode) arguments\n\t\t\t\t: JsonUtil.mapper().createObjectNode();\n\t\tJsonNode props = tool == null ? null : tool.path(\"inputSchema\").path(\"properties\");\n\t\tif (props != null && props.has(\"reason\") && !args.has(\"reason\"))\n\t\t\targs.put(\"reason\", \"\");\n\t\treturn args;\n\t}"
        - id: MCPControlClient.appendProperty
          type: method_declaration
          code: "private void appendProperty(StringBuilder sb, String key, JsonNode prop, boolean required) {\n\t\tString indent = \"  \";\n\t\tappendComment(sb, indent, prop.path(\"description\").asText(\"\"));\n\t\tString type = prop.path(\"type\").asText(null);\n\t\tString value = renderValue(prop, type, indent);\n\t\tsb.append(indent).append(key).append(\":\");\n\t\tif (value.startsWith(\"\\n\"))\n\t\t\tsb.append(value).append(\"\\n\");\n\t\telse\n\t\t\tsb.append(\" \").append(value).append(\"\\n\");\n\t}"
        - id: MCPControlClient.renderValue
          type: method_declaration
          code: "private String renderValue(JsonNode prop, String type, String indent) {\n\t\tJsonNode def = prop.get(\"default\");\n\t\tif (def != null && !def.isNull())\n\t\t\treturn JsonUtil.compact(def);\n\t\tif (type == null)\n\t\t\treturn \"null\";\n\t\tswitch (type) {\n\t\tcase \"string\":\n\t\t\treturn \"\\\"\\\"\";\n\t\tcase \"integer\":\n\t\tcase \"number\":\n\t\t\treturn \"0\";\n\t\tcase \"boolean\":\n\t\t\treturn \"false\";\n\t\tcase \"array\":\n\t\t\treturn renderArrayValue(prop, indent);\n\t\tcase \"object\":\n\t\t\treturn \"{}\";\n\t\tdefault:\n\t\t\treturn \"null\";\n\t\t}\n\t}"
        - id: MCPControlClient.Mwqdao|cn5YCc
          type: statements
          code: "/**\n\t * Renders an array value. Generically pre-fills a single example entry when\n\t * the array's item schema is an object with a \"path\" property, since that is\n\t * by far the most common list-of-paths shape (e.g. \"- path: /path\").\n\t */\n\t/**\n\t * Renders an array value. Generically pre-fills a single example entry from\n\t * the array's item schema when it is an object with properties, instead of\n\t * just an empty \"[]\" — so the user sees the expected item shape directly.\n\t */"
        - id: MCPControlClient.renderArrayValue
          type: method_declaration
          code: "@SuppressWarnings(\"deprecation\")\n\tprivate String renderArrayValue(JsonNode prop, String indent) {\n\t\tJsonNode itemProps = prop.path(\"items\").path(\"properties\");\n\t\tif (!itemProps.isObject() || itemProps.size() == 0)\n\t\t\treturn \"[]\";\n\n\t\tString itemIndent = indent + \"  \";\n\t\tStringBuilder sb = new StringBuilder();\n\t\tIterator<Map.Entry<String, JsonNode>> it = itemProps.fields();\n\t\tboolean first = true;\n\t\twhile (it.hasNext()) {\n\t\t\tMap.Entry<String, JsonNode> e = it.next();\n\t\t\tString childType = e.getValue().path(\"type\").asText(null);\n\t\t\tString value = renderValue(e.getValue(), childType, itemIndent + \"  \");\n\t\t\tsb.append(\"\\n\").append(itemIndent).append(first ? \"- \" : \"  \").append(e.getKey()).append(\": \")\n\t\t\t\t\t.append(value);\n\t\t\tfirst = false;\n\t\t}\n\t\treturn sb.toString();\n\t}"
        - id: MCPControlClient.requiredSet
          type: method_declaration
          code: "private Set<String> requiredSet(JsonNode schema) {\n\t\tSet<String> req = new HashSet<>();\n\t\tJsonNode r = schema.path(\"required\");\n\t\tif (r.isArray())\n\t\t\tfor (JsonNode n : r)\n\t\t\t\treq.add(n.asText());\n\t\treturn req;\n\t}"
        - id: MCPControlClient.appendComment
          type: method_declaration
          code: "private void appendComment(StringBuilder sb, String indent, String text) {\n\t\tif (text == null || text.isBlank())\n\t\t\treturn;\n\t\tfor (String line : text.strip().split(\"\\n\"))\n\t\t\tsb.append(indent).append(\"# \").append(line.stripTrailing()).append(\"\\n\");\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
      nodes:
      - id: MCPRequest
        type: class_declaration
        signature: "public class MCPRequest implements IModelRequest {"
        children:
        - id: MCPRequest.71UuDv|OldiGB
          type: statements
          code: "public final String id;\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final Command command;\n\tpublic final String yamlBlock;"
        - id: MCPRequest.MCPRequest
          type: constructor_declaration
          code: "public MCPRequest(String id, String systemPrompt, List<String> tools, Command command, String yamlBlock) {\n\t\tObjects.requireNonNull(command, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.systemPrompt = systemPrompt;\n\t\tthis.tools = tools != null ? tools : Collections.emptyList();\n\t\tthis.command = command;\n\t\tthis.yamlBlock = yamlBlock;\n\t}"
        - id: MCPRequest.getID
          type: method_declaration
          code: "@Override\n\tpublic String getID() {\n\t\treturn id;\n\t}"
```
/answer 47e1-7a allow
Control Request:
```yaml
id: 47e1-7b
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
  reason: Struktur von CCRequest/CCResponse ansehen für Anpassung
```
/answer 47e1-7b allow
Control Request:
```yaml
id: 47e1-7c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: VAgrYc|ZeBb7W
      - id: CCRequest
        signature: "public class CCRequest implements IModelRequest {"
        children:
        - id: CCRequest.YzeF3A|FDyZsY
        - id: CCRequest.CCRequest
          signature: "public CCRequest(String id, String title, FrozenConfig config, Command command)…"
        - id: CCRequest.getID
          signature: "public String getID() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
      nodes:
      - id: sw4CNG|HeBXVy
      - id: CCResponse
        signature: "public class CCResponse implements IModelResponse {"
        children:
        - id: CCResponse.oarthD|TRluTO
        - id: CCResponse.CCResponse
          signature: "public CCResponse(String id) {"
        - id: CCResponse.CCResponse_1
          signature: "public CCResponse(String id, String resultText) {"
        - id: CCResponse.setToolUse
          signature: "public void setToolUse(String resultText, String toolUseId) {"
        - id: CCResponse.isReady
          signature: "public boolean isReady() {"
```
/answer 47e1-7c allow
ReasoningToken: 194
Control Request:
```yaml
id: 47e1-7d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: ModificationCommand
    include:
    - '*.java'
    limit: 50
  reason: "Alle Verwendungen von ModificationCommand finden, um zu verstehen ob sie noch benötigt wird"
```
/answer 47e1-7d allow
Control Request:
```yaml
id: 47e1-7e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/claudecode/CCConnector.java
        matches:
        - lineno: 28
          match: import xy.ai.workbench.commands.ModificationCommand;
        - lineno: 62
          match: command = new ModificationCommand();
        - lineno: 103
          match: "} else if (command instanceof ModificationCommand) {"
      - path: xy/ai/workbench/commands/ModificationCommand.java
        matches:
        - lineno: 3
          match: "public class ModificationCommand extends Command {"
        - lineno: 5
          match: "public ModificationCommand() {"
        - lineno: 21
          match: return new ModificationCommand();
```
/answer 47e1-7e allow
Control Request:
```yaml
id: 47e1-7f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    ids:
    - CCRequest
  reason: CCRequest Konstruktor und Felder vollständig sehen
```
/answer 47e1-7f allow
Control Request:
```yaml
id: 47e1-80
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: CCRequest
        type: class_declaration
        signature: "public class CCRequest implements IModelRequest {"
        children:
        - id: CCRequest.YzeF3A|FDyZsY
          type: statements
          code: "public final String id;\n\tpublic final String title;\n\n\tpublic final FrozenConfig config;\n\tpublic final Command command;"
        - id: CCRequest.CCRequest
          type: constructor_declaration
          code: "public CCRequest(String id, String title, FrozenConfig config, Command command) {\n\t\tObjects.requireNonNull(command, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.title = title;\n\t\tthis.config = config;\n\t\tthis.command = command;\n\t}"
        - id: CCRequest.getID
          type: method_declaration
          code: "@Override\n\tpublic String getID() {\n\t\treturn id;\n\t}"
```
/answer 47e1-80 allow
ReasoningToken: 6320
Control Request:
```yaml
id: 47e1-81
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ModificationCommand.java
    ids:
    - ModificationCommand
  reason: ModificationCommand vollständig ansehen
```
/answer 47e1-81 allow
Control Request:
```yaml
id: 47e1-82
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ModificationCommand.java
      nodes:
      - id: ModificationCommand
        type: class_declaration
        signature: "public class ModificationCommand extends Command {"
        children:
        - id: ModificationCommand.ModificationCommand
          type: constructor_declaration
          code: "public ModificationCommand() {\n\t\tsuper();\n\t}"
        - id: ModificationCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn \"\";\n\t}"
        - id: ModificationCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\treturn false;\n\t}"
        - id: ModificationCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ModificationCommand();\n\t}"
        - id: ModificationCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.NONE;\n\t}"
```
/answer 47e1-82 allow
ReasoningToken: 95
Control Request:
```yaml
id: 47e1-83
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - x5J3oI|C5FWzw
    - K5C4w9|X2Xlry
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - RLo8qh|nFvk0T
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
    ids:
    - ToolCommand
  reason: Vollständige Imports von CCConnector und PromptHandler sowie ToolCommand als Vorlage lesen
```
/answer 47e1-83 allow
Control Request:
```yaml
id: 47e1-84
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: x5J3oI|C5FWzw
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import java.io.IOException;
          import java.nio.file.Path;
          import java.nio.file.Paths;
          import java.util.UUID;
          import java.util.stream.Collectors;
          import java.util.zip.CRC32;

          import org.eclipse.core.resources.IProject;
          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.SubMonitor;
          import org.eclipse.core.runtime.jobs.Job;
          import org.eclipse.swt.widgets.Display;
          import org.eclipse.ui.IEditorInput;
          import org.eclipse.ui.IEditorPart;
      - id: K5C4w9|X2Xlry
        type: imports
        code: |-
          import org.eclipse.ui.IFileEditorInput;
          import org.eclipse.ui.IWorkbenchPage;
          import org.eclipse.ui.IWorkbenchWindow;
          import org.eclipse.ui.PlatformUI;

          import xy.ai.workbench.AgentProfile;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model.KeyPattern;
          import xy.ai.workbench.commands.AnswerCommand;
          import xy.ai.workbench.commands.Command;
          import xy.ai.workbench.commands.ExitCommand;
          import xy.ai.workbench.commands.ModificationCommand;
          import xy.ai.workbench.commands.PromptCommand;
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: RLo8qh|nFvk0T
        type: imports
        code: |-
          package xy.ai.workbench.connector.harness;

          import java.io.File;
          import java.util.ArrayList;
          import java.util.Arrays;
          import java.util.List;
          import java.util.function.Consumer;
          import java.util.regex.Matcher;
          import java.util.regex.Pattern;

          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.IStatus;
          import org.eclipse.core.runtime.Status;
          import org.eclipse.core.runtime.SubMonitor;
          import org.eclipse.core.runtime.jobs.Job;
          import org.eclipse.jface.text.BadLocationException;
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
      nodes:
      - id: ToolCommand
        type: class_declaration
        signature: "public class ToolCommand extends Command {"
        children:
        - id: ToolCommand.18LEdl|4RBjUa
          type: statements
          code: public static final String CMD_TOOL   = "/tool";
        - id: ToolCommand.ToolCommand
          type: constructor_declaration
          code: "public ToolCommand() {\n\t\tthis(\"\");\n\t}"
        - id: ToolCommand.ToolCommand_1
          type: constructor_declaration
          code: "public ToolCommand(String toolId) {\n\t\tsuper(toolId);\n\t}"
        - id: ToolCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn ToolCommand.CMD_TOOL;\n\t}"
        - id: ToolCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\");\n\t}"
        - id: ToolCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ToolCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());\n\t}"
        - id: ToolCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
```
/answer 47e1-84 allow
ReasoningToken: 5654
Control Request:
```yaml
id: 47e1-85
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
    ids:
    - b6Xn8Z|PmHdMh
  reason: Imports von CommandHandler ansehen um CallEditCommand-Import korrekt hinzuzufügen
```
/answer 47e1-85 allow
Control Request:
```yaml
id: 47e1-86
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
      nodes:
      - id: b6Xn8Z|PmHdMh
        type: imports
        code: |-
          package xy.ai.workbench.commands;

          import java.util.List;
```
/answer 47e1-86 allow
ReasoningToken: 20
Text:
 Same package, no explicit import needed. Now implementing all changes.
Control Request:
```yaml
id: 47e1-87
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallEditCommand.java
    source: "package xy.ai.workbench.commands;\n\nimport java.util.regex.Matcher;\nimport java.util.regex.Pattern;\n\n/** Detected when the whole input consists solely of a single fenced ```yaml ...``` block (a pending edit/approval). */\npublic class CallEditCommand extends Command {\n\n\tprivate static final Pattern YAML_BLOCK = Pattern.compile(\"^```yaml\\\\R(.*?)\\\\R?```$\", Pattern.DOTALL);\n\n\tpublic CallEditCommand() {\n\t\tsuper();\n\t}\n\n\tprivate CallEditCommand(String yaml) {\n\t\tsuper(yaml);\n\t}\n\n\t@Override\n\tpublic String prefix() {\n\t\treturn \"```yaml\";\n\t}\n\n\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && YAML_BLOCK.matcher(line.strip()).matches();\n\t}\n\n\t@Override\n\tpublic Command parse(String line) {\n\t\tMatcher m = YAML_BLOCK.matcher(line.strip());\n\t\tm.matches();\n\t\treturn new CallEditCommand(m.group(1).strip());\n\t}\n\n\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}\n\n\tpublic String yaml() {\n\t\treturn parameter(0);\n\t}\n}\n"
  reason: "Neues CallEditCommand anlegen, das einen allein selektierten YAML-Block erkennt"
```
/answer 47e1-87 allow
Control Request:
```yaml
id: 47e1-88
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
    old_text: "new ResumeCommand(), new ExitCommand(), new ToolCommand(), new CallCommand());"
    new_text: "new ResumeCommand(), new ExitCommand(), new ToolCommand(), new CallCommand(), new CallEditCommand());"
  reason: CallEditCommand in der Kommando-Registry registrieren
```
/answer 47e1-88 allow
Control Request:
```yaml
id: 47e1-89
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ModificationCommand.java
  reason: ModificationCommand ist nach der Umstellung obsolet und wird entfernt
```
/answer 47e1-89 allow
Control Request:
```yaml
id: 47e1-8a
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    old_text: "\tpublic final FrozenConfig config;\n\tpublic final Command command;"
    new_text: "\tpublic final FrozenConfig config;\n\tpublic final Command command;\n\tpublic final String yamlBlock;"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    old_text: "public CCRequest(String id, String title, FrozenConfig config, Command command) {\n\t\tObjects.requireNonNull(command, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.title = title;\n\t\tthis.config = config;\n\t\tthis.command = command;\n\t}"
    new_text: "public CCRequest(String id, String title, FrozenConfig config, Command command, String yamlBlock) {\n\t\tObjects.requireNonNull(command, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.title = title;\n\t\tthis.config = config;\n\t\tthis.command = command;\n\t\tthis.yamlBlock = yamlBlock;\n\t}"
  reason: "CCRequest um yamlBlock erweitern, analog zu MCPRequest, für CallCommand-Unterstützung"
```
/answer 47e1-8a allow
Control Request:
```yaml
id: 47e1-8b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      result: success
      new_id: CCRequest.71UuDv|notoLy
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      error: "Text found in node but rejected: the edit would merge/split lines or break syntax there."
      reason: guard_rejected
```
/answer 47e1-8b allow
Text:
 Now update `CCRequest.java` to carry the yaml block:
Control Request:
```yaml
id: 47e1-8c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    ids:
    - CCRequest
  reason: Aktuellen Stand von CCRequest nach erstem Edit prüfen
```
/answer 47e1-8c allow
Control Request:
```yaml
id: 47e1-8d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: CCRequest
        type: class_declaration
        signature: "public class CCRequest implements IModelRequest {"
        children:
        - id: CCRequest.71UuDv|notoLy
          type: statements
          code: "public final String id;\n\tpublic final String title;\n\n\tpublic final FrozenConfig config;\n\tpublic final Command command;\n\tpublic final String yamlBlock;"
        - id: CCRequest.CCRequest
          type: constructor_declaration
          code: "public CCRequest(String id, String title, FrozenConfig config, Command command) {\n\t\tObjects.requireNonNull(command, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.title = title;\n\t\tthis.config = config;\n\t\tthis.command = command;\n\t}"
        - id: CCRequest.getID
          type: method_declaration
          code: "@Override\n\tpublic String getID() {\n\t\treturn id;\n\t}"
```
/answer 47e1-8d allow
Control Request:
```yaml
id: 47e1-8e
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    id: CCRequest.CCRequest
    source: "public CCRequest(String id, String title, FrozenConfig config, Command command, String yamlBlock) {\n\t\tObjects.requireNonNull(command, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.title = title;\n\t\tthis.config = config;\n\t\tthis.command = command;\n\t\tthis.yamlBlock = yamlBlock;\n\t}"
  reason: CCRequest-Konstruktor um yamlBlock-Parameter erweitern
```
/answer 47e1-8e allow
ReasoningToken: 22
Text:
 Now update `CCControlClient.submitEdit`/remove `extractYamlBlock`:
Control Request:
```yaml
id: 47e1-8f
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.submitEdit
    source: "public boolean submitEdit(String yamlBlock) {\n\t\tif (yamlBlock == null || yamlBlock.isBlank())\n\t\t\treturn false;\n\n\t\tJsonNode edited;\n\t\ttry {\n\t\t\tedited = fromYaml(yamlBlock);\n\t\t} catch (Exception e) {\n\t\t\tthrow new IllegalArgumentException(\"YAML Error\", e);\n\t\t}\n\t\tif (edited == null || !edited.isObject() || !edited.hasNonNull(\"id\"))\n\t\t\treturn false;\n\t\tsubmit((ObjectNode) edited);\n\t\treturn true;\n\t}"
  reason: "submitEdit vereinfachen, da die YAML-Block-Extraktion nun vom Kommandosystem übernommen wird"
```
/answer 47e1-8f allow
Control Request:
```yaml
id: 47e1-90
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.extractYamlBlock
  reason: "extractYamlBlock ist obsolet, da Kommando-Parsing jetzt vom Kommandosystem übernommen wird"
```
/answer 47e1-90 allow
Control Request:
```yaml
id: 47e1-91
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    old_text: |-
      import xy.ai.workbench.commands.AnswerCommand;
      import xy.ai.workbench.commands.Command;
      import xy.ai.workbench.commands.ExitCommand;
      import xy.ai.workbench.commands.ModificationCommand;
      import xy.ai.workbench.commands.PromptCommand;
    new_text: |-
      import xy.ai.workbench.commands.AnswerCommand;
      import xy.ai.workbench.commands.CallCommand;
      import xy.ai.workbench.commands.CallEditCommand;
      import xy.ai.workbench.commands.Command;
      import xy.ai.workbench.commands.ExitCommand;
      import xy.ai.workbench.commands.PromptCommand;
  reason: "Imports für CallCommand/CallEditCommand ergänzen, ModificationCommand-Import entfernen"
```
/answer 47e1-91 allow
Text:
 Now update `CCConnector.java`: imports, `createRequest`, `executeRequest`.
Control Request:
```yaml
id: 47e1-92
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.createRequest
    source: "@Override\n\tpublic CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Create request\", 1);\n\t\tsub.subTask(\"Detect command\");\n\n\t\tCommand command = prompt.command;\n\t\tif (command == null) {\n\t\t\tString text = prompt.inputs.stream().map(String::strip).filter(s -> !s.isBlank())\n\t\t\t\t\t.collect(Collectors.joining(\"\\n\"));\n\t\t\tif (!text.isEmpty())\n\t\t\t\tcommand = new PromptCommand(text);\n\t\t}\n\t\tif (command == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\tsub.worked(1);\n\n\t\tString title = command instanceof PromptCommand pc ? pc.title() : null;\n\t\treturn new CCRequest(UUID.randomUUID().toString(), title, prompt.config, command, prompt.yamlBlock);\n\t}"
  reason: "createRequest: submitEdit-basierte Ad-hoc-Erkennung entfernen, da CallEditCommand/CallCommand dies über das Kommandosystem übernehmen"
```
/answer 47e1-92 allow
Control Request:
```yaml
id: 47e1-93
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    old_text: "\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t} else if (command instanceof ModificationCommand) {\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params); // already sent\n\t\t} else if (command instanceof PromptCommand) {"
    new_text: "\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t} else if (command instanceof CallEditCommand cec) {\n\t\t\tif (!controlClient.submitEdit(cec.yaml()))\n\t\t\t\tthrow new IllegalArgumentException(\"Invalid or incomplete edit YAML block\");\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t} else if (command instanceof CallCommand) {\n\t\t\tif (req.yamlBlock == null || req.yamlBlock.isBlank())\n\t\t\t\tthrow new IllegalArgumentException(\"No preceding ```yaml block found before /call\");\n\t\t\tif (!controlClient.submitEdit(req.yamlBlock))\n\t\t\t\tthrow new IllegalArgumentException(\"Invalid or incomplete edit YAML block\");\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t} else if (command instanceof PromptCommand) {"
  reason: "executeRequest: CallEditCommand und CallCommand statt ModificationCommand behandeln, Kommando wird jetzt korrekt weitergereicht"
```
/answer 47e1-93 allow
Control Request:
```yaml
id: 47e1-94
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    old_text: "\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1) {\n\t\t\tinputs.add(removeCommentLines(tsel.getText()));\n\t\t\t// Block start (multi-line command) takes precedence over a trailing command line."
    new_text: "\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1) {\n\t\t\t// A selection consisting solely of a ```yaml block is itself a command (CallEditCommand).\n\t\t\tCommand block = CommandHandler.detect(tsel.getText());\n\t\t\tif (block instanceof CallEditCommand) {\n\t\t\t\tdetected[0] = block;\n\t\t\t\treturn Prompt.InputKind.BlockSelection;\n\t\t\t}\n\n\t\t\tinputs.add(removeCommentLines(tsel.getText()));\n\t\t\t// Block start (multi-line command) takes precedence over a trailing command line."
  reason: "detectSelection: eine ausschließlich aus einem YAML-Block bestehende Auswahl als CallEditCommand erkennen"
```
/answer 47e1-94 allow
ReasoningToken: 27
Text:
 Now update `PromptHandler.java` to detect a selection that is solely a YAML block as `CallEditCommand`:
Control Request:
```yaml
id: 47e1-95
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
    pattern: import xy.ai.workbench.commands
    include:
    - PromptHandler.java
  reason: Bestehende commands-Imports in PromptHandler finden
```
/answer 47e1-95 allow
Control Request:
```yaml
id: 47e1-96
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
      matches:
      - path: PromptHandler.java
        matches:
        - lineno: 35
          match: import xy.ai.workbench.commands.Command;
        - lineno: 36
          match: import xy.ai.workbench.commands.CommandHandler;
```
/answer 47e1-96 allow
Control Request:
```yaml
id: 47e1-97
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    old_text: |-
      import xy.ai.workbench.commands.Command;
      import xy.ai.workbench.commands.CommandHandler;
    new_text: |-
      import xy.ai.workbench.commands.CallEditCommand;
      import xy.ai.workbench.commands.Command;
      import xy.ai.workbench.commands.CommandHandler;
  reason: Import von CallEditCommand in PromptHandler ergänzen
```
/answer 47e1-97 allow
Text:
 Now add the `CallEditCommand` import to PromptHandler:
Control Request:
```yaml
id: 47e1-98
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallEditCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: Alle geänderten Dateien auf Kompilierbarkeit prüfen
```
/answer 47e1-98 allow
Text:
 Now validate all changed files.
Control Request:
```yaml
id: 47e1-99
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: new CCRequest\(
    include:
    - '*.java'
  reason: "Alle Stellen finden, die CCRequest instanziieren, um Konstruktoraufrufe zu prüfen"
```
/answer 47e1-99 allow
Control Request:
```yaml
id: 47e1-9a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/claudecode/CCConnector.java
        matches:
        - lineno: 70
          match: "return new CCRequest(UUID.randomUUID().toString(), title, prompt.config, command, prompt.yamlBlock);"
```
/answer 47e1-9a allow
ReasoningToken: 76
Result Stats:  id=af74611e-53e1-4aeb-8ec8-2208a01a6298, total: 0, in: 181, out: 23843, reason: 0, read: 1254294, write: 70123
Only one caller, already updated. All changes are consistent and validated.
