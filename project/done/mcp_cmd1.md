Löse das `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java` Objekt auf. Die Funktion soll in den regulären Kommando-Stack `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java` und `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java` überführt werde.
Der `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java` unterstützte dabei nur 3 Kommandos: Exit, Tool und Call, kein Prompt.
Es ist dabei unnötig einen eigenen Kommandotyp oder Enum zu implementieren.
Die `preprocess` Logik ist unabhängig vom `MCPConnector` und in `ToolCommand`, `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java` und `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java` durch `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java` zu überführen.
Der `PromptHandler` soll semantisches Parsing aller Kommandos, das umfasst die Erfassung eines vor oder nachgelagerten YAML Script Blocks aus Input oder Editor, wie beispielsweise im Fall von `CallCommand`.
SystemInit:  id=09dd07ab-54fa-41d8-a447-ff8162ce71a4, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 47e1-23
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: Überblick über die zu bearbeitenden Dateien verschaffen
```
/answer 47e1-23 allow
Control Request:
```yaml
id: 47e1-24
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
      nodes:
      - id: 3zBTmJ|TGrMqF
      - id: MCPCommand
        signature: "public class MCPCommand {"
        children:
        - id: MCPCommand.PJNTO9|QpkmbA
        - id: MCPCommand.MCPCommand
          signature: "public MCPCommand(MCPCommandType type, String... parameters) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
      nodes:
      - id: fau272|XRuDTB
      - id: ToolCommand
        signature: "public class ToolCommand extends Command {"
        children:
        - id: ToolCommand.18LEdl|4RBjUa
        - id: ToolCommand.ToolCommand
          signature: "public ToolCommand() {"
        - id: ToolCommand.ToolCommand_1
          signature: "public ToolCommand(String toolId) {"
        - id: ToolCommand.prefix
          signature: "public String prefix() {"
        - id: ToolCommand.matches
          signature: "public boolean matches(String line) {"
        - id: ToolCommand.parse
          signature: "public Command parse(String line) {"
        - id: ToolCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: gYFgZy|l8eqO0
      - id: uzcHjn|BNLYpl
      - id: MCPConnector
        signature: "public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {"
        children:
        - id: MCPConnector.hOPGQb|xV4uSy
        - id: MCPConnector.MCPConnector
          signature: "public MCPConnector(ConfigManager cfg, MCPClient client) {"
        - id: MCPConnector.getSupportedKeyPattern
          signature: "public KeyPattern getSupportedKeyPattern() {"
        - id: MCPConnector.createRequest
          signature: "public MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
        - id: MCPConnector.executeRequest
          signature: "public MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job…"
        - id: MCPConnector.invokeCall
          signature: "private MCPResponse invokeCall(String id, JsonNode call) {"
        - id: MCPConnector.convertResponse
          signature: "public AIAnswer convertResponse(MCPResponse resp, IProgressMonitor mon) {"
        - id: MCPConnector.preprocess
          signature: "private MCPCommand preprocess(List<String> inputs) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
      nodes:
      - id: fau272|XRuDTB
      - id: ExitCommand
        signature: "public class ExitCommand extends Command {"
        children:
        - id: ExitCommand.Qvx8ey|ppwqjr
        - id: ExitCommand.ExitCommand
          signature: "public ExitCommand() {"
        - id: ExitCommand.prefix
          signature: "public String prefix() {"
        - id: ExitCommand.matches
          signature: "public boolean matches(String line) {"
        - id: ExitCommand.parse
          signature: "public Command parse(String line) {"
        - id: ExitCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: jyVQdv|ey0ixs
      - id: bXTkJR|w6qdUr
      - id: sEQQ8c|82A0Td
      - id: KEc50E|BnEeaS
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
          signature: "private boolean applyDetected(Optional<Command> cmd, IDocument doc, int lineInd…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.Q5D2ne|VNsv72
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
```
/answer 47e1-24 allow
ReasoningToken: 14
Control Request:
```yaml
id: 47e1-25
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
    ids:
    - 3zBTmJ|TGrMqF
    - MCPCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
    ids:
    - fau272|XRuDTB
    - ToolCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
    ids:
    - fau272|XRuDTB
    - CallCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - gYFgZy|l8eqO0
    - uzcHjn|BNLYpl
    - MCPConnector
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
    ids:
    - fau272|XRuDTB
    - ExitCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - PromptHandler
  reason: Vollständigen Quellcode der relevanten Dateien lesen für Refactoring
```
/answer 47e1-25 allow
Control Request:
```yaml
id: 47e1-26
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
      nodes:
      - id: 3zBTmJ|TGrMqF
        type: imports
        code: |-
          package xy.ai.workbench.connector.mcp;

          import java.util.Objects;
      - id: MCPCommand
        type: class_declaration
        signature: "public class MCPCommand {"
        children:
        - id: MCPCommand.PJNTO9|QpkmbA
          type: statements
          code: "public final MCPCommandType type;\n\tpublic final String parameter;\n\tpublic final String[] parameters;"
        - id: MCPCommand.MCPCommand
          type: constructor_declaration
          code: "public MCPCommand(MCPCommandType type, String... parameters) {\n\t\tObjects.requireNonNull(type, \"Type can't be null\");\n\t\tthis.type = type;\n\t\tthis.parameter = parameters[0];\n\t\tthis.parameters = parameters;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
      nodes:
      - id: fau272|XRuDTB
        type: imports
        code: package xy.ai.workbench.commands;
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
      nodes:
      - id: fau272|XRuDTB
        type: imports
        code: package xy.ai.workbench.commands;
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: gYFgZy|l8eqO0
        type: imports
        code: |-
          package xy.ai.workbench.connector.mcp;

          import java.util.List;
          import java.util.UUID;

          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.jobs.Job;

          import com.fasterxml.jackson.databind.JsonNode;

          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.Model.KeyPattern;
          import xy.ai.workbench.commands.CommandHandler;
          import xy.ai.workbench.commands.ExitCommand;
          import xy.ai.workbench.commands.ToolCommand;
          import xy.ai.workbench.connector.IAIConnector;
      - id: uzcHjn|BNLYpl
        type: imports
        code: |-
          import xy.ai.workbench.connector.harness.Prompt;
          import xy.ai.workbench.models.AIAnswer;
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
          code: "@Override\n\tpublic MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tMCPCommand cmd = prompt.command == Prompt.Command.CALL ? new MCPCommand(MCPCommandType.Call, prompt.yamlBlock)\n\t\t\t\t: preprocess(prompt.inputs);\n\t\treturn new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools, cmd);\n\t}"
        - id: MCPConnector.executeRequest
          type: method_declaration
          code: "@Override\n\tpublic MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job) {\n\t\tswitch (req.cmd.type) {\n\t\tcase Exit:\n\t\t\tclient.close();\n\t\t\treturn new MCPResponse(req.id, \"MCP connection closed\");\n\t\tcase Tool: {\n\t\t\tJsonNode tool = client.findTool(req.cmd.parameter);\n\t\t\tif (tool == null)\n\t\t\t\tthrow new IllegalArgumentException(\"Unknown tool: \" + req.cmd.parameter);\n\t\t\treturn new MCPResponse(req.id, control.renderSchema(tool));\n\t\t}\n\t\tcase Call: {\n\t\t\tif (req.cmd.parameter == null || req.cmd.parameter.isBlank())\n\t\t\t\tthrow new IllegalArgumentException(\"No preceding ```yaml tool call block found before /call\");\n\t\t\treturn invokeCall(req.id, control.parseYaml(req.cmd.parameter));\n\t\t}\n\t\tcase Prompt: {\n\t\t\tString block = control.extractYamlBlock(req.cmd.parameter);\n\t\t\tif (block == null)\n\t\t\t\tthrow new IllegalArgumentException(\n\t\t\t\t\t\t\"No ```yaml tool call block found. Use \\\"/tool <id>\\\" to get a template.\");\n\t\t\treturn invokeCall(req.id, control.parseYaml(block));\n\t\t}\n\t\tdefault:\n\t\t\tthrow new UnsupportedOperationException(\"Unsupported command: \" + req.cmd.type);\n\t\t}\n\t}"
        - id: MCPConnector.invokeCall
          type: method_declaration
          code: "private MCPResponse invokeCall(String id, JsonNode call) {\n\t\tString name = call.path(\"tool\").asText(null);\n\t\tif (name == null || name.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"Tool call is missing the 'tool' field\");\n\t\tJsonNode tool = client.findTool(name);\n\t\tJsonNode arguments = control.fillReason(tool, call.path(\"arguments\"));\n\t\tJsonNode result = client.callTool(name, arguments);\n\t\treturn new MCPResponse(id, control.prettyResult(result));\n\t}"
        - id: MCPConnector.convertResponse
          type: method_declaration
          code: "@Override\n\tpublic AIAnswer convertResponse(MCPResponse resp, IProgressMonitor mon) {\n\t\tAIAnswer answer = new AIAnswer(resp.id);\n\t\tanswer.answer = resp.resultText;\n\t\treturn answer;\n\t}"
        - id: MCPConnector.preprocess
          type: method_declaration
          code: "private MCPCommand preprocess(List<String> inputs) {\n\t\tStringBuilder merged = null;\n\t\tfor (String input : inputs) {\n\t\t\tString clean = input != null ? input.strip() : \"\";\n\t\t\tif (clean.isBlank())\n\t\t\t\tcontinue;\n\t\t\tvar detected = CommandHandler.detect(clean);\n\t\t\tif (detected.isPresent() && detected.get() instanceof ExitCommand)\n\t\t\t\treturn new MCPCommand(MCPCommandType.Exit, \"\");\n\t\t\tif (detected.isPresent() && detected.get() instanceof ToolCommand tc)\n\t\t\t\treturn new MCPCommand(MCPCommandType.Tool, tc.parameter(0));\n\t\t\tif (merged == null)\n\t\t\t\tmerged = new StringBuilder();\n\t\t\telse\n\t\t\t\tmerged.append(\"\\n\");\n\t\t\tmerged.append(clean);\n\t\t}\n\t\tif (merged == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn new MCPCommand(MCPCommandType.Prompt, merged.toString());\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
      nodes:
      - id: fau272|XRuDTB
        type: imports
        code: package xy.ai.workbench.commands;
      - id: ExitCommand
        type: class_declaration
        signature: "public class ExitCommand extends Command {"
        children:
        - id: ExitCommand.Qvx8ey|ppwqjr
          type: statements
          code: public static final String CMD_EXIT   = "/exit";
        - id: ExitCommand.ExitCommand
          type: constructor_declaration
          code: "public ExitCommand() {\n\t\tsuper();\n\t}"
        - id: ExitCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn ExitCommand.CMD_EXIT;\n\t}"
        - id: ExitCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && prefix().equalsIgnoreCase(line.strip());\n\t}"
        - id: ExitCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ExitCommand();\n\t}"
        - id: ExitCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
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
          code: "private Prompt buildPrompt(Display display, boolean batch) {\n\t\tList<String> inputs = new ArrayList<>();\n\t\tString[] absoluteFilePath = new String[1];\n\t\tPrompt.InputKind[] kind = new Prompt.InputKind[1];\n\t\tCommand[] detected = new Command[1];\n\t\tString[] yamlBlock = new String[1];\n\n\t\tdisplay.syncExec(() -> {\n\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\tabsoluteFilePath[0] = resolveAbsoluteFilePath(textEditor);\n\n\t\t\tif (cfg.isInputEnabled(InputMode.Selection))\n\t\t\t\tkind[0] = detectSelection(textEditor, inputs, detected, yamlBlock);\n\t\t\telse if (cfg.isInputEnabled(InputMode.Converter))\n\t\t\t\tkind[0] = detectFullFile(textEditor, inputs, detected, yamlBlock);\n\t\t});\n\n\t\tFrozenConfig frozen = FrozenConfig.from(cfg);\n\n\t\tif (inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank()))\n\t\t\tthrow new IllegalArgumentException(\"Input and System Prompt Empty\");\n\n\t\t// TODO maybe store result editor ref in prompt\n\t\tif (editorListener.getLastTextEditor() == null && !batch)\n\t\t\tthrow new IllegalArgumentException(\"Result editor unset\");\n\n\t\tPrompt.Builder builder = Prompt.builder()//\n\t\t\t\t.inputs(inputs)//\n\t\t\t\t.kind(kind[0])//\n\t\t\t\t.batch(batch)//\n\t\t\t\t.config(frozen)//\n\t\t\t\t.processorEnabled(cfg.isInputEnabled(InputMode.Converter))//\n\t\t\t\t.absoluteFilePath(absoluteFilePath[0]);\n\n\t\tif (detected[0] instanceof CallCommand)\n\t\t\tbuilder.command(Prompt.Command.CALL).yamlBlock(yamlBlock[0]);\n\t\telse\n\t\t\tbuilder.command(Prompt.Command.NONE);\n\n\t\treturn builder.build();\n\t}"
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
          code: "private boolean applyDetected(Optional<Command> cmd, IDocument doc, int lineIndex, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (cmd.isEmpty())\n\t\t\treturn false;\n\t\tdetected[0] = cmd.get();\n\t\tif (detected[0] instanceof CallCommand)\n\t\t\tyamlBlock[0] = captureYamlBlock(doc, lineIndex);\n\t\treturn true;\n\t}"
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
```
/answer 47e1-26 allow
Control Request:
```yaml
id: 47e1-27
toolName: grep
arguments:
  items:
  - pattern: class Command
    directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
    include:
    - '*.java'
  - pattern: class Prompt
    directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
    include:
    - '*.java'
  - pattern: MCPCommandType
    directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp
    include:
    - '*.java'
  reason: Basisklassen und verbundene Typen finden
```
/answer 47e1-27 allow
Control Request:
```yaml
id: 47e1-28
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
      matches:
      - path: Command.java
        matches:
        - lineno: 3
          match: "public abstract class Command {"
      - path: CommandHandler.java
        matches:
        - lineno: 6
          match: "public class CommandHandler {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
      matches:
      - path: PromptHandler.java
        matches:
        - lineno: 50
          match: "public final class PromptHandler {"
        - lineno: 172
          match: "private class PromptJob extends Job {"
      - path: Prompt.java
        matches:
        - lineno: 17
          match: "public class Prompt {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp
      matches:
      - path: MCPCommandType.java
        matches:
        - lineno: 3
          match: "public enum MCPCommandType {"
      - path: MCPCommand.java
        matches:
        - lineno: 6
          match: public final MCPCommandType type;
        - lineno: 10
          match: "public MCPCommand(MCPCommandType type, String... parameters) {"
      - path: MCPConnector.java
        matches:
        - lineno: 39
          match: "MCPCommand cmd = prompt.command == Prompt.Command.CALL ? new MCPCommand(MCPCommandType.Call, prompt.yamlBlock)"
        - lineno: 98
          match: "return new MCPCommand(MCPCommandType.Exit, \"\");"
        - lineno: 100
          match: "return new MCPCommand(MCPCommandType.Tool, tc.parameter(0));"
        - lineno: 109
          match: "return new MCPCommand(MCPCommandType.Prompt, merged.toString());"
```
/answer 47e1-28 allow
ReasoningToken: 16
Control Request:
```yaml
id: 47e1-29
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    ids:
    - '*'
  reason: Basisklassen und Datenstrukturen lesen für Refactoring-Plan
```
/answer 47e1-29 allow
Control Request:
```yaml
id: 47e1-2a
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
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
      nodes:
      - id: 2nSWaI|GD8TFr
      - id: CommandHandler
        signature: "public class CommandHandler {"
        children:
        - id: CommandHandler.yMrxSu|gq2BES
        - id: CommandHandler.CommandHandler
          signature: "private CommandHandler() {"
        - id: CommandHandler.detect
          signature: "public static Optional<Command> detect(String line) {"
        - id: CommandHandler.list
          signature: "public static List<Command> list() {"
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
      nodes:
      - id: 7n1FzW|AZ2luy
      - id: T3A86v|qTHHcx
      - id: Prompt
        signature: "public class Prompt {"
        children:
        - id: Prompt.InputKind
          signature: "public enum InputKind {"
        - id: Prompt.Command
          signature: "public enum Command {"
        - id: Prompt.uUxVT7|GekI8D
        - id: Prompt.Prompt
          signature: "private Prompt(List<String> inputs, InputKind kind, boolean batch, FrozenConfig…"
        - id: Prompt.Xua3u0|1iYvhx
        - id: Prompt.sessionId
          signature: "public String sessionId() {"
        - id: Prompt.computeSessionId
          signature: "private String computeSessionId() {"
        - id: Prompt.G0QOxV|j22yeX
        - id: Prompt.toSessionParameters
          signature: "public static SessionParameters toSessionParameters(Prompt prompt, Path cwd) {"
        - id: Prompt.luEJel|M0HBGM
        - id: Prompt.toClaudeSessionParameters
          signature: "public static ClaudeSessionParameters toClaudeSessionParameters(Prompt prompt, …"
        - id: Prompt.builder
          signature: "public static Builder builder() {"
        - id: Prompt.4K9Ahw|LWbHnr
        - id: Prompt.Builder
          signature: "public static final class Builder {"
          children:
          - id: Prompt.Builder.vzZSXJ|xcGm6d
          - id: Prompt.Builder.Builder
            signature: "private Builder() {"
          - id: Prompt.Builder.inputs
            signature: "public Builder inputs(List<String> inputs) {"
          - id: Prompt.Builder.kind
            signature: "public Builder kind(InputKind kind) {"
          - id: Prompt.Builder.batch
            signature: "public Builder batch(boolean batch) {"
          - id: Prompt.Builder.config
            signature: "public Builder config(FrozenConfig config) {"
          - id: Prompt.Builder.processorEnabled
            signature: "public Builder processorEnabled(boolean processorEnabled) {"
          - id: Prompt.Builder.absoluteFilePath
            signature: "public Builder absoluteFilePath(String absoluteFilePath) {"
          - id: Prompt.Builder.command
            signature: "public Builder command(Command command) {"
          - id: Prompt.Builder.yamlBlock
            signature: "public Builder yamlBlock(String yamlBlock) {"
          - id: Prompt.Builder.build
            signature: "public Prompt build() {"
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
      nodes:
      - id: H2WnQb|QmRDpJ
      - id: MCPCommandType
        signature: "public enum MCPCommandType {"
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
      nodes:
      - id: VAgrYc|svzMxJ
      - id: MCPRequest
        signature: "public class MCPRequest implements IModelRequest {"
        children:
        - id: MCPRequest.LLeok1|41cj72
        - id: MCPRequest.MCPRequest
          signature: "public MCPRequest(String id, String systemPrompt, List<String> tools, MCPComman…"
        - id: MCPRequest.getID
          signature: "public String getID() {"
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 47e1-2a allow
ReasoningToken: 126
Control Request:
```yaml
id: 47e1-2b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
    ids:
    - fau272|XRuDTB
    - Command
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
    ids:
    - 2nSWaI|GD8TFr
    - CommandHandler
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
    ids:
    - 7n1FzW|AZ2luy
    - T3A86v|qTHHcx
    - Prompt
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    ids:
    - H2WnQb|QmRDpJ
    - MCPCommandType
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    ids:
    - VAgrYc|svzMxJ
    - MCPRequest
  reason: Vollständigen Code lesen für Refactoring
```
/answer 47e1-2b allow
Control Request:
```yaml
id: 47e1-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
      nodes:
      - id: fau272|XRuDTB
        type: imports
        code: package xy.ai.workbench.commands;
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
      nodes:
      - id: 2nSWaI|GD8TFr
        type: imports
        code: |-
          package xy.ai.workbench.commands;

          import java.util.List;
          import java.util.Optional;
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
          code: "public static Optional<Command> detect(String line) {\n\t\tfor (Command prototype : REGISTRY)\n\t\t\tif (prototype.matches(line))\n\t\t\t\treturn Optional.of(prototype.parse(line));\n\t\treturn Optional.empty();\n\t}"
        - id: CommandHandler.list
          type: method_declaration
          code: "public static List<Command> list() {\n\t\treturn REGISTRY;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
      nodes:
      - id: 7n1FzW|AZ2luy
        type: imports
        code: |-
          package xy.ai.workbench.connector.harness;

          import java.nio.charset.StandardCharsets;
          import java.nio.file.Path;
          import java.security.MessageDigest;
          import java.security.NoSuchAlgorithmException;
          import java.util.List;

          import xy.ai.workbench.connector.SessionParameters;
          import xy.ai.workbench.connector.claudecode.ClaudeSessionParameters;
      - id: T3A86v|qTHHcx
        type: statements
        code: |-
          /**
           * Immutable, frozen prompt: semantically separated Inputs / Batch / Config,
           * plus the activation state for the SessionProcessor. Construction happens
           * exclusively via {@link Builder}, driven by the PromptHandler.
           */
      - id: Prompt
        type: class_declaration
        signature: "public class Prompt {"
        children:
        - id: Prompt.InputKind
          type: enum_declaration
          code: "public enum InputKind {\n\t\tLineInput, BlockSelection, FullFile\n\t}"
        - id: Prompt.Command
          type: enum_declaration
          code: "public enum Command {\n\t\tNONE, CALL\n\t}"
        - id: Prompt.uUxVT7|GekI8D
          type: statements
          code: "public final List<String> inputs;\n\tpublic final InputKind kind;\n\tpublic final boolean batch;\n\tpublic final FrozenConfig config;\n\tpublic final boolean processorEnabled;\n\tpublic final String absoluteFilePath;\n\tpublic final Command command;\n\tpublic final String yamlBlock;\n\n\tprivate String sessionId;"
        - id: Prompt.Prompt
          type: constructor_declaration
          code: "private Prompt(List<String> inputs, InputKind kind, boolean batch, FrozenConfig config, boolean processorEnabled,\n\t\t\tString absoluteFilePath, Command command, String yamlBlock) {\n\t\tif (config == null)\n\t\t\tthrow new IllegalArgumentException(\"config must not be null\");\n\t\tif (absoluteFilePath == null || absoluteFilePath.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"absoluteFilePath must not be blank\");\n\t\tthis.inputs = inputs != null ? List.copyOf(inputs) : List.of();\n\t\tthis.kind = kind;\n\t\tthis.batch = batch;\n\t\tthis.config = config;\n\t\tthis.processorEnabled = processorEnabled;\n\t\tthis.absoluteFilePath = absoluteFilePath;\n\t\tthis.command = command != null ? command : Command.NONE;\n\t\tthis.yamlBlock = yamlBlock;\n\t}"
        - id: Prompt.Xua3u0|1iYvhx
          type: statements
          code: "/** Deterministic session id: hash(absoluteFilePath, batch, config.getHash()). */"
        - id: Prompt.sessionId
          type: method_declaration
          code: "public String sessionId() {\n\t\tif (sessionId == null)\n\t\t\tsessionId = computeSessionId();\n\t\treturn sessionId;\n\t}"
        - id: Prompt.computeSessionId
          type: method_declaration
          code: "private String computeSessionId() {\n\t\tString input = absoluteFilePath + \"|\" + batch + \"|\" + config.getHash();\n\t\ttry {\n\t\t\tMessageDigest md = MessageDigest.getInstance(\"MD5\");\n\t\t\tbyte[] bytes = md.digest(input.getBytes(StandardCharsets.UTF_8));\n\t\t\tStringBuilder sb = new StringBuilder();\n\t\t\tfor (byte b : bytes)\n\t\t\t\tsb.append(String.format(\"%02x\", b));\n\t\t\treturn sb.substring(0, 8);\n\t\t} catch (NoSuchAlgorithmException e) {\n\t\t\t// Stable fallback (no external dependency)\n\t\t\tlong h = 0;\n\t\t\tfor (char c : input.toCharArray())\n\t\t\t\th = h * 31L + c;\n\t\t\treturn String.format(\"%08x\", h & 0xFFFFFFFFL);\n\t\t}\n\t}"
        - id: Prompt.G0QOxV|j22yeX
          type: statements
          code: "/** Derives generic SessionParameters from {@code prompt.config} (replaces Step 07's fromConfig(cfg, ...)). */"
        - id: Prompt.toSessionParameters
          type: method_declaration
          code: "public static SessionParameters toSessionParameters(Prompt prompt, Path cwd) {\n\t\tFrozenConfig c = prompt.config;\n\t\treturn new SessionParameters(cwd, c.systemPrompt, c.tools, c.model, c.reasoning, c.profile, c.cliProfile,\n\t\t\t\tc.cacheMode, prompt.absoluteFilePath, c.topP, c.temperature,\n\t\t\t\tc.maxOutputTokens != null ? c.maxOutputTokens.intValue() : null);\n\t}"
        - id: Prompt.luEJel|M0HBGM
          type: statements
          code: "/** Derives Claude-specific SessionParameters from {@code prompt.config}. */"
        - id: Prompt.toClaudeSessionParameters
          type: method_declaration
          code: "public static ClaudeSessionParameters toClaudeSessionParameters(Prompt prompt, Path cwd) {\n\t\tFrozenConfig c = prompt.config;\n\t\treturn new ClaudeSessionParameters(cwd, c.systemPrompt, c.tools, c.model, c.reasoning, c.profile, c.cliProfile,\n\t\t\t\tc.cacheMode, prompt.absoluteFilePath);\n\t}"
        - id: Prompt.builder
          type: method_declaration
          code: "public static Builder builder() {\n\t\treturn new Builder();\n\t}"
        - id: Prompt.4K9Ahw|LWbHnr
          type: statements
          code: "/** Builder for {@link Prompt}; intended for use by the PromptHandler (Step 06). */"
        - id: Prompt.Builder
          type: class_declaration
          signature: "public static final class Builder {"
          children:
          - id: Prompt.Builder.vzZSXJ|xcGm6d
            type: statements
            code: "private List<String> inputs;\n\t\tprivate InputKind kind;\n\t\tprivate boolean batch;\n\t\tprivate FrozenConfig config;\n\t\tprivate boolean processorEnabled;\n\t\tprivate String absoluteFilePath;\n\t\tprivate Command command;\n\t\tprivate String yamlBlock;"
          - id: Prompt.Builder.Builder
            type: constructor_declaration
            code: "private Builder() {\n\t\t}"
          - id: Prompt.Builder.inputs
            type: method_declaration
            code: "public Builder inputs(List<String> inputs) {\n\t\t\tthis.inputs = inputs;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.kind
            type: method_declaration
            code: "public Builder kind(InputKind kind) {\n\t\t\tthis.kind = kind;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.batch
            type: method_declaration
            code: "public Builder batch(boolean batch) {\n\t\t\tthis.batch = batch;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.config
            type: method_declaration
            code: "public Builder config(FrozenConfig config) {\n\t\t\tthis.config = config;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.processorEnabled
            type: method_declaration
            code: "public Builder processorEnabled(boolean processorEnabled) {\n\t\t\tthis.processorEnabled = processorEnabled;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.absoluteFilePath
            type: method_declaration
            code: "public Builder absoluteFilePath(String absoluteFilePath) {\n\t\t\tthis.absoluteFilePath = absoluteFilePath;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.command
            type: method_declaration
            code: "public Builder command(Command command) {\n\t\t\tthis.command = command;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.yamlBlock
            type: method_declaration
            code: "public Builder yamlBlock(String yamlBlock) {\n\t\t\tthis.yamlBlock = yamlBlock;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.build
            type: method_declaration
            code: "public Prompt build() {\n\t\t\treturn new Prompt(inputs, kind, batch, config, processorEnabled, absoluteFilePath, command, yamlBlock);\n\t\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
      nodes:
      - id: H2WnQb|QmRDpJ
        type: imports
        code: package xy.ai.workbench.connector.mcp;
      - id: MCPCommandType
        type: enum_declaration
        code: "public enum MCPCommandType {\n\tPrompt, Tool, Exit, Call;\n}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
      nodes:
      - id: VAgrYc|svzMxJ
        type: imports
        code: |-
          package xy.ai.workbench.connector.mcp;

          import java.util.Collections;
          import java.util.List;
          import java.util.Objects;

          import xy.ai.workbench.models.IModelRequest;
      - id: MCPRequest
        type: class_declaration
        signature: "public class MCPRequest implements IModelRequest {"
        children:
        - id: MCPRequest.LLeok1|41cj72
          type: statements
          code: "public final String id;\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final MCPCommand cmd;"
        - id: MCPRequest.MCPRequest
          type: constructor_declaration
          code: "public MCPRequest(String id, String systemPrompt, List<String> tools, MCPCommand cmd) {\n\t\tObjects.requireNonNull(cmd, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.systemPrompt = systemPrompt;\n\t\tthis.tools = tools != null ? tools : Collections.emptyList();\n\t\tthis.cmd = cmd;\n\t}"
        - id: MCPRequest.getID
          type: method_declaration
          code: "@Override\n\tpublic String getID() {\n\t\treturn id;\n\t}"
```
/answer 47e1-2c allow
ReasoningToken: 23
Control Request:
```yaml
id: 47e1-2d
toolName: grep
arguments:
  items:
  - pattern: Prompt\.builder\(\)
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
  - pattern: MCPConnector|createRequest\(
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
  - pattern: prompt\.command|Prompt\.Command
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
  reason: "Alle Aufrufer von Prompt.builder und MCPConnector finden, um Kontext für den Chat-/Editor-Flow zu verstehen"
```
/answer 47e1-2d allow
Control Request:
```yaml
id: 47e1-2e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/harness/PromptHandler.java
        matches:
        - lineno: 290
          match: Prompt.Builder builder = Prompt.builder()//
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/mcp/MCPConnector.java
        matches:
        - lineno: 20
          match: "public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {"
        - lineno: 27
          match: "public MCPConnector(ConfigManager cfg, MCPClient client) {"
        - lineno: 38
          match: "public MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: xy/ai/workbench/connector/claudecode/CCConnector.java
        matches:
        - lineno: 53
          match: "public CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: xy/ai/workbench/connector/google/GeminiConnector.java
        matches:
        - lineno: 68
          match: "public GeminiRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: xy/ai/workbench/connector/IAIConnector.java
        matches:
        - lineno: 14
          match: "REQ createRequest(Prompt prompt, IProgressMonitor mon);"
      - path: xy/ai/workbench/connector/claude/ClaudeConnector.java
        matches:
        - lineno: 60
          match: "public ClaudeRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: xy/ai/workbench/connector/openai/OpenAIConnector.java
        matches:
        - lineno: 67
          match: "public OpenAIRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: xy/ai/workbench/connector/AdaptingConnector.java
        matches:
        - lineno: 25
          match: import xy.ai.workbench.connector.mcp.MCPConnector;
        - lineno: 56
          match: private MCPConnector mcp;
        - lineno: 67
          match: "mcp = new MCPConnector(cfg, mcpClient);"
        - lineno: 165
          match: "public IModelRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
        - lineno: 166
          match: "return getConnector(prompt.config.model).createRequest(prompt, mon);"
      - path: xy/ai/workbench/connector/deepseek/DeepSeekConnector.java
        matches:
        - lineno: 110
          match: "public DeepSeekRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: xy/ai/workbench/connector/harness/PromptHandler.java
        matches:
        - lineno: 258
          match: "IModelRequest req = connector.createRequest(prompt, sub);"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/mcp/MCPConnector.java
        matches:
        - lineno: 39
          match: "MCPCommand cmd = prompt.command == Prompt.Command.CALL ? new MCPCommand(MCPCommandType.Call, prompt.yamlBlock)"
      - path: xy/ai/workbench/connector/harness/PromptHandler.java
        matches:
        - lineno: 299
          match: "builder.command(Prompt.Command.CALL).yamlBlock(yamlBlock[0]);"
        - lineno: 301
          match: builder.command(Prompt.Command.NONE);
```
/answer 47e1-2e allow
ReasoningToken: 4008
Control Request:
```yaml
id: 47e1-2f
toolName: grep
arguments:
  items:
  - pattern: prompt\.command|prompt\.yamlBlock|\.command\(|yamlBlock
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
    limit: 50
  reason: Alle Verwendungen von Prompt.command und yamlBlock in der Codebasis finden
```
/answer 47e1-2f allow
Control Request:
```yaml
id: 47e1-30
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/mcp/MCPConnector.java
        matches:
        - lineno: 39
          match: "MCPCommand cmd = prompt.command == Prompt.Command.CALL ? new MCPCommand(MCPCommandType.Call, prompt.yamlBlock)"
      - path: xy/ai/workbench/connector/harness/PromptHandler.java
        matches:
        - lineno: 269
          match: "String[] yamlBlock = new String[1];"
        - lineno: 276
          match: "kind[0] = detectSelection(textEditor, inputs, detected, yamlBlock);"
        - lineno: 278
          match: "kind[0] = detectFullFile(textEditor, inputs, detected, yamlBlock);"
        - lineno: 299
          match: "builder.command(Prompt.Command.CALL).yamlBlock(yamlBlock[0]);"
        - lineno: 301
          match: builder.command(Prompt.Command.NONE);
        - lineno: 309
          match: "String[] yamlBlock) {"
        - lineno: 324
          match: "detected, yamlBlock))"
        - lineno: 326
          match: yamlBlock);
        - lineno: 338
          match: yamlBlock);
        - lineno: 346
          match: "String[] yamlBlock) {"
        - lineno: 360
          match: "detected, yamlBlock))"
        - lineno: 364
          match: "applyDetected(CommandHandler.detect(lineText(doc, lastLine)), doc, lastLine, detected, yamlBlock);"
        - lineno: 369
          match: "String[] yamlBlock) {"
        - lineno: 374
          match: "yamlBlock[0] = captureYamlBlock(doc, lineIndex);"
      - path: xy/ai/workbench/connector/harness/Prompt.java
        matches:
        - lineno: 34
          match: public final String yamlBlock;
        - lineno: 39
          match: "String absoluteFilePath, Command command, String yamlBlock) {"
        - lineno: 51
          match: this.yamlBlock = yamlBlock;
        - lineno: 107
          match: private String yamlBlock;
        - lineno: 147
          match: "public Builder yamlBlock(String yamlBlock) {"
        - lineno: 148
          match: this.yamlBlock = yamlBlock;
        - lineno: 153
          match: "return new Prompt(inputs, kind, batch, config, processorEnabled, absoluteFilePath, command, yamlBlock);"
```
/answer 47e1-30 allow
ReasoningToken: 2108
Control Request:
```yaml
id: 47e1-31
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: Imports von PromptHandler.java einsehen
```
/answer 47e1-31 allow
Control Request:
```yaml
id: 47e1-32
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: jyVQdv|ey0ixs
      - id: bXTkJR|w6qdUr
      - id: sEQQ8c|82A0Td
      - id: KEc50E|BnEeaS
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
          signature: "private boolean applyDetected(Optional<Command> cmd, IDocument doc, int lineInd…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.Q5D2ne|VNsv72
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
```
/answer 47e1-32 allow
ReasoningToken: 117
Control Request:
```yaml
id: 47e1-33
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - jyVQdv|ey0ixs
    - bXTkJR|w6qdUr
    - sEQQ8c|82A0Td
    - KEc50E|BnEeaS
    - fPHG2Y|yRFIp4
  reason: Kopfkommentare und Imports von PromptHandler lesen
```
/answer 47e1-33 allow
Control Request:
```yaml
id: 47e1-34
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: jyVQdv|ey0ixs
        type: imports
        code: |-
          package xy.ai.workbench.connector.harness;

          import java.io.File;
          import java.util.ArrayList;
          import java.util.Arrays;
          import java.util.List;
          import java.util.Optional;
          import java.util.function.Consumer;
          import java.util.regex.Matcher;
          import java.util.regex.Pattern;

          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.IStatus;
          import org.eclipse.core.runtime.Status;
          import org.eclipse.core.runtime.SubMonitor;
          import org.eclipse.core.runtime.jobs.Job;
      - id: bXTkJR|w6qdUr
        type: imports
        code: |-
          import org.eclipse.jface.text.BadLocationException;
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
      - id: sEQQ8c|82A0Td
        type: imports
        code: |-
          import xy.ai.workbench.ActiveEditorListener;
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.EditorInterface;
          import xy.ai.workbench.IncludeAdapter;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.batch.AIBatchManager;
          import xy.ai.workbench.commands.CallCommand;
          import xy.ai.workbench.commands.Command;
          import xy.ai.workbench.commands.CommandHandler;
          import xy.ai.workbench.connector.AdaptingConnector;
          import xy.ai.workbench.editor.md.AbstractRule;
      - id: KEc50E|BnEeaS
        type: imports
        code: |-
          import xy.ai.workbench.models.AIAnswer;
          import xy.ai.workbench.models.IModelRequest;
          import xy.ai.workbench.models.IModelResponse;
      - id: fPHG2Y|yRFIp4
        type: statements
        code: |-
          /**
           * Prompt-specific orchestration extracted from {@code AISessionManager}: builds a frozen
           * {@link Prompt} from the active editor/selection, runs command detection and drives the
           * prepare/insertTag/execute/replaceTag job pipeline.
           */
```
/answer 47e1-34 allow
ReasoningToken: 140
Control Request:
```yaml
id: 47e1-35
toolName: grep
arguments:
  items:
  - pattern: MCPCommand|req\.cmd|\.cmd\.
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
  reason: "Alle Verwendungen von MCPCommand und req.cmd finden, um vollständige Anpassung sicherzustellen"
```
/answer 47e1-35 allow
Control Request:
```yaml
id: 47e1-36
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/mcp/MCPRequest.java
        matches:
        - lineno: 13
          match: public final MCPCommand cmd;
        - lineno: 15
          match: "public MCPRequest(String id, String systemPrompt, List<String> tools, MCPCommand cmd) {"
      - path: xy/ai/workbench/connector/mcp/MCPCommandType.java
        matches:
        - lineno: 3
          match: "public enum MCPCommandType {"
      - path: xy/ai/workbench/connector/mcp/MCPCommand.java
        matches:
        - lineno: 5
          match: "public class MCPCommand {"
        - lineno: 6
          match: public final MCPCommandType type;
        - lineno: 10
          match: "public MCPCommand(MCPCommandType type, String... parameters) {"
      - path: xy/ai/workbench/connector/mcp/MCPConnector.java
        matches:
        - lineno: 39
          match: "MCPCommand cmd = prompt.command == Prompt.Command.CALL ? new MCPCommand(MCPCommandType.Call, prompt.yamlBlock)"
        - lineno: 46
          match: "switch (req.cmd.type) {"
        - lineno: 51
          match: JsonNode tool = client.findTool(req.cmd.parameter);
        - lineno: 53
          match: "throw new IllegalArgumentException(\"Unknown tool: \" + req.cmd.parameter);"
        - lineno: 57
          match: if (req.cmd.parameter == null || req.cmd.parameter.isBlank())
        - lineno: 59
          match: "return invokeCall(req.id, control.parseYaml(req.cmd.parameter));"
        - lineno: 62
          match: String block = control.extractYamlBlock(req.cmd.parameter);
        - lineno: 69
          match: "throw new UnsupportedOperationException(\"Unsupported command: \" + req.cmd.type);"
        - lineno: 90
          match: "private MCPCommand preprocess(List<String> inputs) {"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 47e1-36 allow
ReasoningToken: 339
Text:
 Jetzt setze ich die Änderungen um.
Control Request:
```yaml
id: 47e1-37
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
    id: Prompt
    start_marker: "public enum Command {\n\t\tNONE, CALL\n\t}\n\n"
    end_marker: "\tprivate String sessionId;"
    content: "public final List<String> inputs;\n\tpublic final InputKind kind;\n\tpublic final boolean batch;\n\tpublic final FrozenConfig config;\n\tpublic final boolean processorEnabled;\n\tpublic final String absoluteFilePath;\n\tpublic final Command command;\n\tpublic final String yamlBlock;\n\n\tprivate String sessionId;"
  reason: "Entfernen des Prompt.Command-Enums, Feld command nutzt jetzt xy.ai.workbench.commands.Command direkt"
```
/answer 47e1-37 allow
Control Request:
```yaml
id: 47e1-38
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
    id: Prompt.Prompt
    old_text: "this.command = command != null ? command : Command.NONE;"
    new_text: this.command = command;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
    id: 7n1FzW|AZ2luy
    old_text: import xy.ai.workbench.connector.SessionParameters;
    new_text: |-
      import xy.ai.workbench.commands.Command;
      import xy.ai.workbench.connector.SessionParameters;
  reason: Konstruktor anpassen und Import für Command hinzufügen
```
/answer 47e1-38 allow
Control Request:
```yaml
id: 47e1-39
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
      result: success
      id: 7n1FzW|AZ2luy
      new_id: 7n1FzW|zXfU3F
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
      error: "Text found in node but rejected: the edit would merge/split lines or break syntax there."
      id: Prompt.Prompt
      reason: guard_rejected
```
/answer 47e1-39 allow
Control Request:
```yaml
id: 47e1-3a
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
  id: Prompt.Prompt
  reason: Aktuellen Konstruktor-Code prüfen
```
/answer 47e1-3a allow
Control Request:
```yaml
id: 47e1-3b
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
      nodes:
      - id: Prompt.Prompt
        code: "private Prompt(List<String> inputs, InputKind kind, boolean batch, FrozenConfig config, boolean processorEnabled,\n\t\t\tString absoluteFilePath, Command command, String yamlBlock) {\n\t\tif (config == null)\n\t\t\tthrow new IllegalArgumentException(\"config must not be null\");\n\t\tif (absoluteFilePath == null || absoluteFilePath.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"absoluteFilePath must not be blank\");\n\t\tthis.inputs = inputs != null ? List.copyOf(inputs) : List.of();\n\t\tthis.kind = kind;\n\t\tthis.batch = batch;\n\t\tthis.config = config;\n\t\tthis.processorEnabled = processorEnabled;\n\t\tthis.absoluteFilePath = absoluteFilePath;\n\t\tthis.command = command != null ? command : Command.NONE;\n\t\tthis.yamlBlock = yamlBlock;\n\t}"
```
/answer 47e1-3b allow
ReasoningToken: 27
Control Request:
```yaml
id: 47e1-3c
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
    id: Prompt.Prompt
    source: "private Prompt(List<String> inputs, InputKind kind, boolean batch, FrozenConfig config, boolean processorEnabled,\n\t\t\tString absoluteFilePath, Command command, String yamlBlock) {\n\t\tif (config == null)\n\t\t\tthrow new IllegalArgumentException(\"config must not be null\");\n\t\tif (absoluteFilePath == null || absoluteFilePath.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"absoluteFilePath must not be blank\");\n\t\tthis.inputs = inputs != null ? List.copyOf(inputs) : List.of();\n\t\tthis.kind = kind;\n\t\tthis.batch = batch;\n\t\tthis.config = config;\n\t\tthis.processorEnabled = processorEnabled;\n\t\tthis.absoluteFilePath = absoluteFilePath;\n\t\tthis.command = command;\n\t\tthis.yamlBlock = yamlBlock;\n\t}"
  reason: "Konstruktor anpassen: command wird nicht mehr auf ein Enum-NONE gemappt"
```
/answer 47e1-3c allow
Control Request:
```yaml
id: 47e1-3d
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.buildPrompt
    old_text: "if (detected[0] instanceof CallCommand)\n\t\t\tbuilder.command(Prompt.Command.CALL).yamlBlock(yamlBlock[0]);\n\t\telse\n\t\t\tbuilder.command(Prompt.Command.NONE);"
    new_text: "builder.command(detected[0]).yamlBlock(yamlBlock[0]);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.applyDetected
    old_text: "detected[0] = cmd.get();\n\t\tif (detected[0] instanceof CallCommand)\n\t\t\tyamlBlock[0] = captureYamlBlock(doc, lineIndex);\n\t\treturn true;"
    new_text: "detected[0] = cmd.get();\n\t\tyamlBlock[0] = captureYamlBlock(doc, lineIndex);\n\t\treturn true;"
  reason: "Kommandoerkennung generalisieren: jedes erkannte Command (Exit/Tool/Call) wird durchgereicht, YAML-Block wird generisch erfasst"
```
/answer 47e1-3d allow
ReasoningToken: 104
Text:
 Nun PromptHandler.java anpassen: generisches Command-/YAML-Parsing statt CallCommand-spezifisch.
Control Request:
```yaml
id: 47e1-3e
toolName: grep
arguments:
  items:
  - pattern: CallCommand
    directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
    include:
    - '*.java'
  reason: Prüfen ob CallCommand Import noch verwendet wird
```
/answer 47e1-3e allow
Control Request:
```yaml
id: 47e1-3f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
      matches:
      - path: PromptHandler.java
        matches:
        - lineno: 36
          match: import xy.ai.workbench.commands.CallCommand;
```
/answer 47e1-3f allow
Control Request:
```yaml
id: 47e1-40
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: sEQQ8c|82A0Td
    old_text: |-
      import xy.ai.workbench.commands.CallCommand;
      import xy.ai.workbench.commands.Command;
    new_text: import xy.ai.workbench.commands.Command;
  reason: Ungenutzten Import entfernen
```
/answer 47e1-40 allow
ReasoningToken: 54
Text:
 Import nicht mehr genutzt, entfernen.
Control Request:
```yaml
id: 47e1-41
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    id: MCPRequest.LLeok1|41cj72
    source: "public final String id;\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final Command command;\n\tpublic final String yamlBlock;"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    id: MCPRequest.MCPRequest
    source: "public MCPRequest(String id, String systemPrompt, List<String> tools, Command command, String yamlBlock) {\n\t\tObjects.requireNonNull(command, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.systemPrompt = systemPrompt;\n\t\tthis.tools = tools != null ? tools : Collections.emptyList();\n\t\tthis.command = command;\n\t\tthis.yamlBlock = yamlBlock;\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    id: VAgrYc|svzMxJ
    source: |-
      package xy.ai.workbench.connector.mcp;

      import java.util.Collections;
      import java.util.List;
      import java.util.Objects;

      import xy.ai.workbench.commands.Command;
      import xy.ai.workbench.models.IModelRequest;
  reason: MCPRequest auf regulären Command-Stack statt MCPCommand umstellen
```
/answer 47e1-41 allow
ReasoningToken: 114
Text:
 Jetzt MCPConnector.java überarbeiten.
Control Request:
```yaml
id: 47e1-42
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    id: gYFgZy|l8eqO0
    source: |-
      package xy.ai.workbench.connector.mcp;

      import java.util.UUID;

      import org.eclipse.core.runtime.IProgressMonitor;
      import org.eclipse.core.runtime.jobs.Job;

      import com.fasterxml.jackson.databind.JsonNode;

      import xy.ai.workbench.ConfigManager;
      import xy.ai.workbench.Model.KeyPattern;
      import xy.ai.workbench.commands.CallCommand;
      import xy.ai.workbench.commands.Command;
      import xy.ai.workbench.commands.ExitCommand;
      import xy.ai.workbench.commands.ToolCommand;
      import xy.ai.workbench.connector.IAIConnector;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    id: MCPConnector.createRequest
    source: "@Override\n\tpublic MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tCommand command = prompt.command;\n\t\tif (!(command instanceof ExitCommand) && !(command instanceof ToolCommand)\n\t\t\t\t&& !(command instanceof CallCommand))\n\t\t\tthrow new IllegalArgumentException(\n\t\t\t\t\t\"No command detected. Use \\\"/exit\\\", \\\"/tool <id>\\\" or \\\"/call\\\".\");\n\t\treturn new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools, command,\n\t\t\t\tprompt.yamlBlock);\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    id: MCPConnector.executeRequest
    source: "@Override\n\tpublic MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job) {\n\t\tif (req.command instanceof ExitCommand) {\n\t\t\tclient.close();\n\t\t\treturn new MCPResponse(req.id, \"MCP connection closed\");\n\t\t}\n\t\tif (req.command instanceof ToolCommand tc) {\n\t\t\tJsonNode tool = client.findTool(tc.parameter(0));\n\t\t\tif (tool == null)\n\t\t\t\tthrow new IllegalArgumentException(\"Unknown tool: \" + tc.parameter(0));\n\t\t\treturn new MCPResponse(req.id, control.renderSchema(tool));\n\t\t}\n\t\tif (req.command instanceof CallCommand) {\n\t\t\tif (req.yamlBlock == null || req.yamlBlock.isBlank())\n\t\t\t\tthrow new IllegalArgumentException(\"No preceding ```yaml tool call block found before /call\");\n\t\t\treturn invokeCall(req.id, control.parseYaml(req.yamlBlock));\n\t\t}\n\t\tthrow new UnsupportedOperationException(\"Unsupported command: \" + req.command.prefix());\n\t}"
  reason: "createRequest/executeRequest auf regulären Command-Stack (Exit/Tool/Call) umstellen, Prompt-Fallback entfernen"
```
/answer 47e1-42 allow
Control Request:
```yaml
id: 47e1-43
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    id: MCPConnector.preprocess
  reason: "preprocess-Logik entfernen, da sie in PromptHandler übergeführt wurde"
```
/answer 47e1-43 allow
Control Request:
```yaml
id: 47e1-44
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
  reason: "MCPCommand und MCPCommandType Dateien entfernen, da die Funktion in den regulären Kommando-Stack überführt wurde"
```
/answer 47e1-44 allow
Control Request:
```yaml
id: 47e1-45
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
  reason: Alle geänderten Dateien auf Kompilierfähigkeit prüfen
```
/answer 47e1-45 allow
Control Request:
```yaml
id: 47e1-46
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - MCPConnector
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    ids:
    - MCPRequest
  reason: Finalen Code der geänderten Dateien überprüfen
```
/answer 47e1-46 allow
Control Request:
```yaml
id: 47e1-47
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
/answer 47e1-47 allow
Text:
 Jetzt die `preprocess`-Methode entfernen und `List`-Import prüfen.
Control Request:
```yaml
id: 47e1-48
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - uzcHjn|BNLYpl
  reason: "Zweiten Import-Block prüfen, ob Prompt-Import noch gebraucht wird"
```
/answer 47e1-48 allow
Control Request:
```yaml
id: 47e1-49
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: uzcHjn|BNLYpl
        type: imports
        code: |-
          import xy.ai.workbench.connector.harness.Prompt;
          import xy.ai.workbench.models.AIAnswer;
```
/answer 47e1-49 allow
Text:
 Jetzt MCPCommand.java und MCPCommandType.java löschen.
Text:
 Jetzt validieren.
ReasoningToken: 134
Text:
 Sieht gut aus. Kurze Prüfung der finalen `Prompt.java`-Datei und ungenutzter Imports (`uzcHjn|BNLYpl` in MCPConnector für `Prompt`).
Result Stats:  id=09dd07ab-54fa-41d8-a447-ff8162ce71a4, total: 0, in: 181, out: 16613, reason: 0, read: 1009201, write: 57890
Passt, `Prompt` wird weiterhin in `createRequest` verwendet. Refactoring abgeschlossen.
