Ich würde gerne die beiden Commands vereinheitlichen. Bei einer Answer wird ein Toolausführung bestätigt. Bei einem Call Edit wird die Ausführung zusätzlich abgewandelt. Ich würde gerne erreichen das ein Answer Command selbst erkennen kann ob der Vorlaufende Tool Call verändert wurde. Verschaffe dir eine Übersicht und prüfe das.

`/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java`
`/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java`
SystemInit:  id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 6a5a-68-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java
  reason: Überblick über Struktur beider Commands gewinnen
```
/answer 6a5a-68-0 allow
Control Request:
```yaml
id: 6a5a-69-68
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
      nodes:
      - id: AnswerCommand
        excert: "public class AnswerCommand extends Command {"
        children:
        - id: AnswerCommand.LN3qky|soKy4k
          excert: public static final String CMD_ANSWER = "/answer";
        - id: AnswerCommand.AnswerCommand
          excert: public AnswerCommand()
        - id: AnswerCommand.AnswerCommand_2
          excert: "public AnswerCommand(String id, String action, String reason)"
        - id: AnswerCommand.prefix
          excert: public String prefix()
        - id: AnswerCommand.matches
          excert: public boolean matches(String line)
        - id: AnswerCommand.parse
          excert: public Command parse(String line)
        - id: AnswerCommand.processorAction
          excert: public ProcessorAction processorAction()
        - id: AnswerCommand.Action
          excert: "public enum Action {"
        - id: AnswerCommand.BlockType
          excert: /**  * Distinguishes a pending tool call (arguments) from a pending tool result…
          docstring: Distinguishes a pending tool call (arguments) from a pending tool result (resul…
        - id: AnswerCommand.np6Lp5|nApalK
          excert: private String yaml;
        - id: AnswerCommand.setYaml
          excert: public void setYaml(String yaml)
          docstring: "Sets the raw YAML block this answer refers to, used by {@link #blockType()}."
        - id: AnswerCommand.blockType
          excert: public BlockType blockType()
        - id: AnswerCommand.action
          excert: public Action action()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java
      nodes:
      - id: uUL6Qs|J3MY6H
        excert: import java.util.regex.Matcher;
      - id: CallEditCommand
        excert: /**  * Detected when the whole input consists solely of a single YAML block  */…
        docstring: Detected when the whole input consists solely of a single YAML block
        children:
        - id: CallEditCommand.rU7BIw|BKuAVj
          excert: private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)\\R?…
        - id: CallEditCommand.CallEditCommand
          excert: public CallEditCommand()
        - id: CallEditCommand.CallEditCommand_2
          excert: private CallEditCommand(String yaml)
        - id: CallEditCommand.prefix
          excert: public String prefix()
        - id: CallEditCommand.matches
          excert: public boolean matches(String line)
        - id: CallEditCommand.parse
          excert: public Command parse(String line)
        - id: CallEditCommand.processorAction
          excert: public ProcessorAction processorAction()
        - id: CallEditCommand.yaml
          excert: public String yaml()
```
/answer 6a5a-69-68 allow
Control Request:
```yaml
id: 6a5a-6a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
    ids:
    - AnswerCommand
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java
    ids:
    - CallEditCommand
  reason: Vollständigen Quelltext beider Klassen lesen
```
/answer 6a5a-6a-0 allow
Control Request:
```yaml
id: 6a5a-6b-6a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
      nodes:
      - id: AnswerCommand
        type: ClassOrInterfaceDeclaration
        excert: "public class AnswerCommand extends Command {"
        children:
        - id: AnswerCommand.LN3qky|soKy4k
          type: StatementGroup
          code: "\tpublic static final String CMD_ANSWER = \"/answer\";\n"
        - id: AnswerCommand.AnswerCommand
          type: ConstructorDeclaration
          code: "\tpublic AnswerCommand() {\n\t\tthis(\"\", \"\", \"\");\n\t}\n"
        - id: AnswerCommand.AnswerCommand_2
          type: ConstructorDeclaration
          code: "\tpublic AnswerCommand(String id, String action, String reason) {\n\t\tsuper(id, action, reason);\n\t}\n"
        - id: AnswerCommand.prefix
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String prefix() {\n\t\treturn AnswerCommand.CMD_ANSWER;\n\t}\n"
        - id: AnswerCommand.matches
          type: MethodDeclaration
          code: "\t@Override\n\tpublic boolean matches(String line) {\n\t\t// \"s\" (DOTALL) allows a block-selected multi-line reason/hint after allow|deny.\n\t\treturn line != null && line.strip().matches(\"(?is)\" + prefix() + \"\\\\s+\\\\S+\\\\s+(allow|deny|exec)(\\\\s+.*)?\");\n\t}\n"
        - id: AnswerCommand.parse
          type: MethodDeclaration
          code: "\t@Override\n\tpublic Command parse(String line) {\n\t\tString[] parts = line.strip().split(\"\\\\s+\", 4);\n\t\tString id = parts[1];\n\t\tString action = parts[2].toLowerCase();\n\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\treturn new AnswerCommand(id, action, reason);\n\t}\n"
        - id: AnswerCommand.processorAction
          type: MethodDeclaration
          code: "\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.TRANSFORM;\n\t}\n"
        - id: AnswerCommand.Action
          type: EnumDeclaration
          code: "\tpublic enum Action {\n\t\tAllow, Deny, Exec\n\t}\n"
        - id: AnswerCommand.BlockType
          type: EnumDeclaration
          code: "\tpublic enum BlockType {\n\t\tCall, Result\n\t}\n"
        - id: AnswerCommand.np6Lp5|nApalK
          type: StatementGroup
          code: "\tprivate String yaml;\n"
        - id: AnswerCommand.setYaml
          type: MethodDeclaration
          code: "\tpublic void setYaml(String yaml) {\n\t\tthis.yaml = yaml;\n\t}\n"
        - id: AnswerCommand.blockType
          type: MethodDeclaration
          code: "\tpublic BlockType blockType() {\n\t\tif (yaml != null && yaml.lines().anyMatch(l -> l.startsWith(\"result:\")))\n\t\t\treturn BlockType.Result;\n\t\treturn BlockType.Call;\n\t}\n"
        - id: AnswerCommand.action
          type: MethodDeclaration
          code: "\tpublic Action action() {\n\t\tif (\"allow\".equalsIgnoreCase(parameter(1)))\n\t\t\treturn Action.Allow;\n\t\tif (\"exec\".equalsIgnoreCase(parameter(1)))\n\t\t\treturn Action.Exec;\n\t\treturn Action.Deny;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java
      nodes:
      - id: CallEditCommand
        type: ClassOrInterfaceDeclaration
        excert: /**  * Detected when the whole input consists solely of a single YAML block  */…
        docstring: Detected when the whole input consists solely of a single YAML block
        children:
        - id: CallEditCommand.rU7BIw|BKuAVj
          type: StatementGroup
          code: "\tprivate static final Pattern YAML_BLOCK = Pattern.compile(\"^```yaml\\\\R(.*?)\\\\R?```$\", Pattern.DOTALL);\n"
        - id: CallEditCommand.CallEditCommand
          type: ConstructorDeclaration
          code: "\tpublic CallEditCommand() {\n\t\tsuper();\n\t}\n"
        - id: CallEditCommand.CallEditCommand_2
          type: ConstructorDeclaration
          code: "\tprivate CallEditCommand(String yaml) {\n\t\tsuper(yaml);\n\t}\n"
        - id: CallEditCommand.prefix
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String prefix() {\n\t\treturn \"```yaml\";\n\t}\n"
        - id: CallEditCommand.matches
          type: MethodDeclaration
          code: "\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && YAML_BLOCK.matcher(line.strip()).matches();\n\t}\n"
        - id: CallEditCommand.parse
          type: MethodDeclaration
          code: "\t@Override\n\tpublic Command parse(String line) {\n\t\tMatcher m = YAML_BLOCK.matcher(line.strip());\n\t\tm.matches();\n\t\treturn new CallEditCommand(m.group(1).strip());\n\t}\n"
        - id: CallEditCommand.processorAction
          type: MethodDeclaration
          code: "\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}\n"
        - id: CallEditCommand.yaml
          type: MethodDeclaration
          code: "\tpublic String yaml() {\n\t\treturn parameter(0);\n\t}\n"
```
/answer 6a5a-6b-6a allow
Control Request:
```yaml
id: 6a5a-6c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: CallEditCommand|AnswerCommand
    include:
    - '*.java'
    exclude:
    - build/**
    - target/**
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: CallEditCommand|AnswerCommand
    include:
    - '*.java'
    exclude:
    - build/**
    - target/**
    limit: 50
  reason: Verwendungsstellen beider Commands finden
```
/answer 6a5a-6c-0 allow
Control Request:
```yaml
id: 6a5a-6d-6c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/editor/mdast/nodes/Elements.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - "public static final PrefixBlock ANSWER = new PrefixBlock(AnswerCommand.CMD_ANSWER, true);"
      - path: xy/ai/workbench/editor/AIRuleScanner.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - "register(Elements.Tools.ANSWER, new PrefixLineRule(AnswerCommand.CMD_ANSWER, commentDarkToken));"
      - path: xy/ai/workbench/eclipse/GitSupport.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - boolean revertToolResultDeny = cmd instanceof AnswerCommand ac && ac.action() == AnswerCommand.Action.Deny && ac.blockType() == AnswerCommand.BlockType.Result;
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - "withBlock(item, command(AnswerCommand.CMD_ANSWER + \" \" + item.id() + \" allow\", hint));"
        - "item.isControl() ? withBlock(item, command(AnswerCommand.CMD_ANSWER + \" \" + item.id() + \" deny\", reason)) : List.of(denial(item, reason));"
      - path: xy/ai/conscious/autoapprove/AnswerParser.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - import xy.ai.conscious.commands.AnswerCommand.BlockType;
        - if (command instanceof AnswerCommand ac)
      - path: xy/ai/conscious/connector/mcp/MCPConnector.java
        matches:
        - import xy.ai.conscious.commands.CallEditCommand;
        - '&& !(command instanceof CallEditCommand))'
        - "if (req.command instanceof CallEditCommand) {"
      - path: xy/ai/conscious/connector/claudecode/CCControlClient.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - "res.append(String.format(\"%s\\n```yaml\\n%s\\n```\\n%s %s allow\", IEditorInterface.CONTROL_REQUEST, toYaml(first), AnswerCommand.CMD_ANSWER, first.path(\"id\").asText()));"
      - path: xy/ai/conscious/connector/claudecode/CCConnector.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - import xy.ai.conscious.commands.CallEditCommand;
        - "if (command instanceof AnswerCommand ac) {"
        - if (ac.action() == AnswerCommand.Action.Allow)
        - else if (ac.action() == AnswerCommand.Action.Exec)
        - "} else if (command instanceof CallEditCommand cec) {"
      - path: xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - if (!(cmd instanceof AnswerCommand) && !(cmd instanceof CallCommand))
      - path: xy/ai/conscious/connector/harness/PromptInputHandler.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - import xy.ai.conscious.commands.CallEditCommand;
        - "if (blockCmd instanceof CallEditCommand) {"
        - "if (blockCmd instanceof AnswerCommand ac) {"
        - if (arg.command instanceof AnswerCommand ac)
        - if (cmd instanceof AnswerCommand ac)
      - path: xy/ai/conscious/commands/CommandRegistry.java
        matches:
        - "new AnswerCommand(), //"
        - new CallEditCommand());
      - path: xy/ai/conscious/commands/AnswerCommand.java
        matches:
        - "public class AnswerCommand extends Command {"
        - "public AnswerCommand() {"
        - "public AnswerCommand(String id, String action, String reason) {"
        - return AnswerCommand.CMD_ANSWER;
        - "return new AnswerCommand(id, action, reason);"
      - path: xy/ai/conscious/commands/CallEditCommand.java
        matches:
        - "public class CallEditCommand extends Command {"
        - "public CallEditCommand() {"
        - "private CallEditCommand(String yaml) {"
        - return new CallEditCommand(m.group(1).strip());
```
/answer 6a5a-6d-6c allow
ReasoningToken: 40
Control Request:
```yaml
id: 6a5a-6e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector
    pattern: CallEditCommand|AnswerCommand|setYaml|blockType|cec\.|ac\.
    include:
    - CCConnector.java
    - MCPConnector.java
    - PromptInputHandler.java
    - PromptHandler.java
    limit: 50
    intent: editing
  reason: Genaue Verwendungsstellen in den Connectoren und Handlern lokalisieren
```
/answer 6a5a-6e-0 allow
Control Request:
```yaml
id: 6a5a-6f-6e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector
      matches:
      - path: mcp/MCPConnector.java
        nodes:
        - id: pPj3JF|oyGHrQ
          matches:
          - import xy.ai.conscious.commands.CallEditCommand;
        - id: MCPConnector.createRequest
          matches:
          - '&& !(command instanceof CallEditCommand))'
        - id: MCPConnector.executeRequest
          matches:
          - "if (req.command instanceof CallEditCommand) {"
      - path: claudecode/CCConnector.java
        nodes:
        - id: qRSWAT|u9KtEh
          matches:
          - import xy.ai.conscious.commands.AnswerCommand;
          - import xy.ai.conscious.commands.CallEditCommand;
        - id: CCConnector.executeRequest
          matches:
          - "if (command instanceof AnswerCommand ac) {"
          - if (ac.action() == AnswerCommand.Action.Allow)
          - "ts.control(() -> controlClient.approve(ac.parameter(0), ac.parameter(2)));"
          - else if (ac.action() == AnswerCommand.Action.Exec)
          - "ts.control(() -> controlClient.exec(ac.parameter(0), ac.parameter(2)));"
          - "ts.control(() -> controlClient.deny(ac.parameter(0), ac.parameter(2)));"
          - "} else if (command instanceof CallEditCommand cec) {"
          - if (!ts.control(() -> controlClient.submitEdit(cec.yaml())))
      - path: harness/PromptHandler.java
        nodes:
        - id: 4MFHdZ|HIydwL
          matches:
          - import xy.ai.conscious.commands.AnswerCommand;
        - id: PromptHandler.checkGitSupport
          matches:
          - if (!(cmd instanceof AnswerCommand) && !(cmd instanceof CallCommand))
      - path: harness/PromptInputHandler.java
        nodes:
        - id: U3FBWT|pBPOgv
          matches:
          - import xy.ai.conscious.commands.AnswerCommand;
          - import xy.ai.conscious.commands.CallEditCommand;
        - id: PromptInputHandler.detectSelection
          matches:
          - "if (blockCmd instanceof CallEditCommand) {"
          - "if (blockCmd instanceof AnswerCommand ac) {"
          - ac.setYaml(arg.yaml);
          - if (arg.command instanceof AnswerCommand ac)
          - ac.setYaml(arg.yaml);
        - id: PromptInputHandler.detectedCmd
          matches:
          - if (cmd instanceof AnswerCommand ac)
          - ac.setYaml(arg.yaml);
```
/answer 6a5a-6f-6e allow
Control Request:
```yaml
id: 6a5a-70-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
    ids:
    - PromptInputHandler.detectSelection
    - PromptInputHandler.detectedCmd
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
    ids:
    - CCConnector.executeRequest
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/mcp/MCPConnector.java
    ids:
    - MCPConnector.createRequest
    - MCPConnector.executeRequest
  reason: "Ablauf lesen: wo yaml gesetzt wird und wie Answer/Edit ausgeführt werden"
```
/answer 6a5a-70-0 allow
Control Request:
```yaml
id: 6a5a-71-70
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
      nodes:
      - id: PromptInputHandler.detectSelection
        type: MethodDeclaration
        code: |2
            private void detectSelection(Selection sel, PromptArguments arg) {
              if (sel == null || sel.selection() == null || sel.selection().length == 0)
                return;
              String[] lines = sel.selection();
              if (lines.length > 1) {
                // selection solely of a YAML block
                String text = String.join(System.lineSeparator(), lines);
                Command blockCmd = CommandRegistry.detect(text);
                if (blockCmd instanceof CallEditCommand) {
                  arg.command = blockCmd;
                  return;
                }
                arg.inputs.add(IActiveEditor.removeCommentLines(sel));
                // An /answer command starting at the beginning of the block spans the
                // whole selection, allowing a multi-line reason/hint for
                // allow and deny alike.
                if (blockCmd instanceof AnswerCommand ac) {
                  arg.command = blockCmd;
                  arg.yaml = captureYamlBlock(lines, 0);
                  ac.setYaml(arg.yaml);
                } else // multi-line command takes precedence over a trailing command
                if (!detectedCmd(lines[0], lines, 0, arg))
                  detectedCmd(lines[lines.length - 1], lines, lines.length - 1, arg);
                return;
              }
              arg.inputs.add(lines[0]);
              // Cursor-only (no real selection): "lines" is just the current line, so a
              // preceding ```yaml block cannot be found within it. Fall back to the full
              // document to locate the block before the cursor line.
              if (detectedCmd(lines[0], lines, 0, arg) && arg.yaml == null) {
                Selection full = editorListener.getFileContent();
                if (full != null && full.selection() != null && full.cursorOffset() != null) {
                  arg.yaml = captureYamlBlock(full.selection(), full.cursorOffset());
                  if (arg.command instanceof AnswerCommand ac)
                    ac.setYaml(arg.yaml);
                }
              }
            }
      - id: PromptInputHandler.detectedCmd
        type: MethodDeclaration
        code: |2
            private static boolean detectedCmd(String line, String[] lines, int lineIndex, PromptArguments arg) {
              Command cmd = CommandRegistry.detect(line);
              if (cmd == null)
                return false;
              arg.command = cmd;
              arg.yaml = captureYamlBlock(lines, lineIndex);
              if (cmd instanceof AnswerCommand ac)
                ac.setYaml(arg.yaml);
              return true;
            }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.executeRequest
        type: MethodDeclaration
        code: "  @Override\n  public CCResponse executeRequest(CCRequest req, IProgress mon) {\n\tIProgress sub = mon.setTask(\"Executing prompt\", 2);\n    CCSession session = null;\n    TimeStats ts = new TimeStats();\n    ts.start();\n    String relativeFilePath = req.arg.project.relativize(Paths.get(req.arg.absoluteFilePath)).toString();\n    FrozenConfig fc = req.config;\n    CCSessionParameters params = new CCSessionParameters(req.arg.project, fc.systemPrompt, fc.tools, fc.model, fc.reasoning, fc.profile, fc.keys, fc.cacheMode, relativeFilePath);\n    params.setTitle(req.title);\n    Command command = req.arg.command;\n    if (command instanceof ResumeCommand rc) {\n      sub.setTaskName(\"Importing session\");\n      sessionManager.importSession(rc.parameter(0), params);\n      return new CCResponse(req.id, \"Session created\");\n    }\n    if (command instanceof ExitCommand) {\n      session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n      sub.setTaskName(\"Terminating CLI process\");\n      session.terminate();\n      return new CCResponse(req.id, \"Session closed!\");\n    }\n    if (command instanceof AnswerCommand ac) {\n      if (ac.action() == AnswerCommand.Action.Allow)\n        ts.control(() -> controlClient.approve(ac.parameter(0), ac.parameter(2)));\n      else if (ac.action() == AnswerCommand.Action.Exec)\n        ts.control(() -> controlClient.exec(ac.parameter(0), ac.parameter(2)));\n      else\n        ts.control(() -> controlClient.deny(ac.parameter(0), ac.parameter(2)));\n      session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n    } else if (command instanceof CallEditCommand cec) {\n      if (!ts.control(() -> controlClient.submitEdit(cec.yaml())))\n        throw new IllegalArgumentException(\"Invalid or incomplete edit YAML block\");\n      session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n    } else if (command instanceof CallCommand) {\n      if (req.arg.yaml == null || req.arg.yaml.isBlank())\n        throw new IllegalArgumentException(\"No preceding ```yaml block found before /call\");\n      if (!ts.control(() -> controlClient.submitEdit(req.arg.yaml)))\n        throw new IllegalArgumentException(\"Invalid or incomplete edit YAML block\");\n      session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n    } else if (command == null) {\n      sub.setTaskName(\"Acquiring session\");\n      session = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);\n    } else\n      throw new UnsupportedOperationException(\"Unsupported command: \" + command.prefix());\n    try {\n      session.setInPrompt(true);\n      if (command == null) {\n        if (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !ts.control(() -> controlClient.isMCPCAvailable()))\n          throw new IllegalStateException(\"MCPC not reachable for AgentProfile\");\n        sub.setTaskName(\"Sending prompt\");\n        String promptJson = requestBuilder.buildPromptJson(req.promptText);\n        long t0 = System.nanoTime();\n        session.writeLine(promptJson);\n        ts.ioMs += (System.nanoTime() - t0) / 1_000_000;\n        ts.sent++;\n      }\n      sub.setTaskName(\"Waiting for answer\");\n      return readUntilResult(req, session, sub.subProgress(), ts);\n    } catch (IOException e) {\n      throw new IllegalStateException(\"Claude Code CLI error\", e);\n    } finally {\n      session.setInPrompt(false);\n    }\n  }\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/mcp/MCPConnector.java
      nodes:
      - id: MCPConnector.createRequest
        type: MethodDeclaration
        code: "\t@Override\n\tpublic MCPRequest createRequest(Prompt prompt, IProgress mon) {\n\t\tCommand command = prompt.arg.command;\n\t\tif (!(command instanceof ExitCommand) && !(command instanceof ToolCommand) && !(command instanceof CallCommand)\n\t\t\t\t&& !(command instanceof CallEditCommand))\n\t\t\tthrow new IllegalArgumentException(String.format(\"No command detected. [%s]\",\n\t\t\t\t\tcommand == null ? \"none\" : command.getClass().getSimpleName()));\n\t\treturn new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools, command,\n\t\t\t\tprompt.arg.yaml);\n\t}\n"
      - id: MCPConnector.executeRequest
        type: MethodDeclaration
        code: "\t@Override\n\tpublic MCPResponse executeRequest(MCPRequest req, IProgress mon) {\n\t\tif (req.command instanceof ExitCommand) {\n\t\t\tclient.close();\n\t\t\treturn new MCPResponse(req.id, \"MCP connection closed\");\n\t\t}\n\t\tif (req.command instanceof ToolCommand tc) {\n\t\t\tJsonNode tool = client.findTool(tc.parameter(0));\n\t\t\tif (tool == null)\n\t\t\t\tthrow new IllegalArgumentException(\"Unknown tool: \" + tc.parameter(0));\n\t\t\treturn new MCPResponse(req.id, control.renderSchema(tool));\n\t\t}\n\t\tif (req.command instanceof CallCommand) {\n\t\t\tif (req.yamlBlock == null || req.yamlBlock.isBlank())\n\t\t\t\tthrow new IllegalArgumentException(\"No preceding ```yaml tool call block found before /call\");\n\t\t\treturn invokeCall(req.id, control.parseYaml(req.yamlBlock));\n\t\t}\n\t\tif (req.command instanceof CallEditCommand) {\n\t\t\tif (req.command.parameter(0) == null || req.command.parameter(0).isBlank())\n\t\t\t\tthrow new IllegalArgumentException(\"No ```yaml tool call block found\");\n\t\t\treturn invokeCall(req.id, control.parseYaml(req.command.parameter(0)));\n\t\t}\n\t\tthrow new UnsupportedOperationException(\"Unsupported command: \" + req.command.prefix());\n\t}\n"
```
/answer 6a5a-71-70 allow
ReasoningToken: 46
Control Request:
```yaml
id: 6a5a-72-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
  reason: Struktur des Control-Clients (approve/exec/deny/submitEdit)
```
/answer 6a5a-72-0 allow
Control Request:
```yaml
id: 6a5a-73-72
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
      nodes:
      - id: K2G7xy|HsdHqq
        excert: import java.io.IOException;
      - id: CCControlClient
        excert: /**  * Minimal HTTP client for the MCPC human-in-the-loop tool-control endpoint…
        docstring: Minimal HTTP client for the MCPC human-in-the-loop tool-control endpoint
        children:
        - id: CCControlClient.mBFSrf|Pegcal
          excert: private static final String CONTROL_URL = "http://localhost:9093/control/tool";
        - id: CCControlClient.CCControlClient
          excert: public CCControlClient(IMCPCLauncher mcpc)
        - id: CCControlClient.checkControlEndpoint
          excert: "public void checkControlEndpoint(CCResponse resp, String sessionId)"
        - id: CCControlClient.toYaml
          excert: public String toYaml(JsonNode node)
        - id: CCControlClient.fromYaml
          excert: public JsonNode fromYaml(String text) throws JsonProcessingException
        - id: CCControlClient.isMCPCAvailable
          excert: public boolean isMCPCAvailable()
        - id: CCControlClient.ping
          excert: private boolean ping()
        - id: CCControlClient.poll
          excert: private ArrayNode poll(String sessionId)
        - id: CCControlClient.approve
          excert: public ArrayNode approve(String id)
        - id: CCControlClient.approve_2
          excert: "public ArrayNode approve(String id, String hint)"
        - id: CCControlClient.exec
          excert: "public ArrayNode exec(String id, String hint)"
        - id: CCControlClient.deny
          excert: "public ArrayNode deny(String id, String reason)"
        - id: CCControlClient.submitEdit
          excert: public boolean submitEdit(String yamlBlock)
        - id: CCControlClient.approvalNode
          excert: "private ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result,…"
        - id: CCControlClient.submit
          excert: private ArrayNode submit(ObjectNode approval)
        - id: CCControlClient.post
          excert: "private ArrayNode post(ObjectNode body, String sessionId)"
```
/answer 6a5a-73-72 allow
Control Request:
```yaml
id: 6a5a-74-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
    ids:
    - CCControlClient.checkControlEndpoint
    - CCControlClient.poll
    - CCControlClient.approve
    - CCControlClient.approve_2
    - CCControlClient.exec
    - CCControlClient.deny
    - CCControlClient.submitEdit
    - CCControlClient.approvalNode
    - CCControlClient.submit
    - CCControlClient.post
  reason: "Prüfen, wie approve/exec/submitEdit den Tool Call bestätigen und ob das Original zum Vergleich verfügbar ist"
```
/answer 6a5a-74-0 allow
Control Request:
```yaml
id: 6a5a-75-74
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
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
              res.append(String.format("%s\n```yaml\n%s\n```\n%s %s allow", IEditorInterface.CONTROL_REQUEST, toYaml(first), AnswerCommand.CMD_ANSWER, first.path("id").asText()));
              resp.resultText = res.toString();
              resp.showStats = false;
            }
      - id: CCControlClient.poll
        type: MethodDeclaration
        code: |2
            private ArrayNode poll(String sessionId) {
              return post(mapper.createObjectNode(), sessionId);
            }
      - id: CCControlClient.approve
        type: MethodDeclaration
        code: |2
            public ArrayNode approve(String id) {
              return approve(id, null);
            }
      - id: CCControlClient.approve_2
        type: MethodDeclaration
        code: |2
            public ArrayNode approve(String id, String hint) {
              return submit(approvalNode(id, null, null, null, hint == null || hint.isBlank() ? null : hint, false));
            }
      - id: CCControlClient.exec
        type: MethodDeclaration
        code: |2
            public ArrayNode exec(String id, String hint) {
              return submit(approvalNode(id, null, null, null, hint == null || hint.isBlank() ? null : hint, true));
            }
      - id: CCControlClient.deny
        type: MethodDeclaration
        code: |2
            public ArrayNode deny(String id, String reason) {
              return submit(approvalNode(id, null, null, reason == null ? "" : reason, null, false));
            }
      - id: CCControlClient.submitEdit
        type: MethodDeclaration
        code: |2
            public boolean submitEdit(String yamlBlock) {
              if (yamlBlock == null || yamlBlock.isBlank())
                return false;
              JsonNode edited;
              try {
                edited = fromYaml(yamlBlock);
              } catch (Exception e) {
                throw new IllegalArgumentException("YAML Error", e);
              }
              if (edited == null || !edited.isObject() || !edited.hasNonNull("id"))
                return false;
              submit((ObjectNode) edited);
              return true;
            }
      - id: CCControlClient.approvalNode
        type: MethodDeclaration
        code: |2
            private ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result, String rejectReason, String hint, boolean exec) {
              ObjectNode approval = mapper.createObjectNode();
              approval.put("id", id);
              if (arguments != null)
                approval.set("arguments", arguments);
              if (result != null)
                approval.set("result", result);
              if (rejectReason != null) {
                approval.put("rejected", true);
                approval.put("reason", rejectReason);
              }
              if (hint != null)
                approval.put("hint", hint);
              if (exec)
                approval.put("exec", true);
              return approval;
            }
      - id: CCControlClient.submit
        type: MethodDeclaration
        code: |2
            private ArrayNode submit(ObjectNode approval) {
              ObjectNode body = mapper.createObjectNode();
              body.putArray("approvals").add(approval);
              return post(body, null);
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
/answer 6a5a-75-74 allow
Control Request:
```yaml
id: 6a5a-76-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
    pattern: captureYamlBlock|CONTROL_REQUEST|pendingCalls|lastPending
    include:
    - '*.java'
    limit: 20
    intent: editing
  reason: "captureYamlBlock und Stelle, wo das ursprünglich angezeigte Pending ggf. gehalten wird"
```
/answer 6a5a-76-0 allow
ReasoningToken: 345
Control Request:
```yaml
id: 6a5a-77-76
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
      matches:
      - path: core/IEditorInterface.java
        nodes:
        - id: IEditorInterface.gCM0q2|sDVECk
          matches:
          - public static final String CONTROL_REQUEST = "Control Request:";
      - path: connector/claudecode/CCControlClient.java
        nodes:
        - id: CCControlClient.checkControlEndpoint
          matches:
          - "res.append(String.format(\"%s\\n```yaml\\n%s\\n```\\n%s %s allow\", IEditorInterface.CONTROL_REQUEST, toYaml(first), AnswerCommand.CMD_ANSWER, first.path(\"id\").asText()));"
      - path: connector/harness/PromptInputHandler.java
        nodes:
        - id: PromptInputHandler.detectSelection
          matches:
          - "arg.yaml = captureYamlBlock(lines, 0);"
          - "arg.yaml = captureYamlBlock(full.selection(), full.cursorOffset());"
        - id: PromptInputHandler.detectedCmd
          matches:
          - "arg.yaml = captureYamlBlock(lines, lineIndex);"
        - id: PromptInputHandler.captureYamlBlock
          matches:
          - "private static String captureYamlBlock(String[] lines, int lineIndex) {"
      - path: commands/ControlRequestCommand.java
        nodes:
        - id: ControlRequestCommand.prefix
          matches:
          - return IEditorInterface.CONTROL_REQUEST;
```
/answer 6a5a-77-76 allow
Control Request:
```yaml
id: 6a5a-78-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
  reason: "Prüfen, wie AnswerParser den YAML-Block/BlockType nutzt"
```
/answer 6a5a-78-0 allow
Control Request:
```yaml
id: 6a5a-79-78
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
      nodes:
      - id: 63CXAz|RDbYNO
        excert: import java.util.ArrayList;
      - id: AnswerParser
        excert: /**  * Extracts tool calls and tool results from the rendered answer text.  */ …
        docstring: Extracts tool calls and tool results from the rendered answer text.
        children:
        - id: AnswerParser.e37lAi|DWomFI
          excert: private static final YAMLMapper MAPPER = new YAMLMapper();
        - id: AnswerParser.parse
          excert: static List<ApprovalItem> parse(String text)
        - id: AnswerParser.parseCommandItem
          excert: private static ApprovalItem parseCommandItem(String text)
        - id: AnswerParser.parseToolResult
          excert: private static ApprovalItem parseToolResult(String text)
        - id: AnswerParser.kindOf
          excert: private static Kind kindOf(Command command)
        - id: AnswerParser.toItem
          excert: "private static ApprovalItem toItem(Kind kind, String yaml)"
```
/answer 6a5a-79-78 allow
Control Request:
```yaml
id: 6a5a-7a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
    ids:
    - PromptInputHandler.captureYamlBlock
  reason: "Lesen, wie der YAML-Block vor dem /answer erfasst wird"
```
/answer 6a5a-7a-0 allow
Control Request:
```yaml
id: 6a5a-7b-7a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
      nodes:
      - id: PromptInputHandler.captureYamlBlock
        type: MethodDeclaration
        code: |2
            private static String captureYamlBlock(String[] lines, int lineIndex) {
              StringBuilder prefix = new StringBuilder();
              for (int i = 0; i < lineIndex && i < lines.length; i++) prefix.append(lines[i]).append("\n");
              Matcher m = YAML_BLOCK.matcher(prefix);
              String last = null;
              while (m.find()) last = m.group(1);
              return last;
            }
```
/answer 6a5a-7b-7a allow
Result Stats: id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, total: 28988, in: 18, out: 5023, reason: 710, read: 123665, write: 23947, start: 1794647912379, end: 1794647931724, time: 19241, total: 50013, io: 0, control: 104, sent: 0, recv: 352
ReasoningToken: 710
Ja, das ist machbar. Ich habe nichts geändert, das ist nur die Bestandsaufnahme.

## Aktueller Stand

**AnswerCommand** (`/answer <id> allow|deny|exec [hint]`)
- Der Command kennt über `setYaml(...)` bereits den YAML-Block vor der `/answer`-Zeile. `PromptInputHandler` setzt ihn an drei Stellen (`detectSelection`, `detectedCmd` und im Cursor-Fallback). Das ist der Block, den der Nutzer im Editor sieht, also schon der mögliche editierte Stand.
- Verwendet wird er bisher nur in `blockType()` (Call oder Result), und das nur für `GitSupport`.
- `CCConnector` ruft je nach Action `approve`, `exec` oder `deny` auf. Die Zusatzinfo ist nur `id` und `hint`, der YAML-Inhalt wird nicht übertragen.

**CallEditCommand** (die Eingabe besteht nur aus einem ` ```yaml `-Block)
- `parse` übernimmt den YAML-Text, und `CCConnector` schickt ihn per `submitEdit(yaml)` komplett als Approval an MCPC. Die `id` steckt im YAML selbst.
- Im `MCPConnector` wird derselbe Block ohne Vergleich direkt mit `invokeCall` ausgeführt.
- Der Command ist `REMOVE`, Answer ist `TRANSFORM`. Auch `/call` (`CallCommand`) nutzt `arg.yaml` und `submitEdit`, ist also fachlich der dritte Verwandte.

## Was fehlt

Das **Original** des Tool Calls liegt nicht im Client. `CCControlClient.checkControlEndpoint` pollt MCPC, rendert `pending.get(0)` per `toYaml` in den Editor und merkt sich nichts. Ohne Original kann `AnswerCommand` nicht feststellen, ob der Block davor verändert wurde.

## Mögliche Umsetzung

1. **Original im Client cachen.** `checkControlEndpoint` speichert `id → JsonNode` des angezeigten Pendings, zum Beispiel in einer `ConcurrentHashMap` im `CCControlClient`.
2. **Vergleich in einer neuen Methode** `CCControlClient.answer(id, action, hint, yaml)`:
   - `yaml` leer: normales `approve`, `exec` oder `deny`.
   - `yaml` per `fromYaml` parsen und mit `JsonNode.equals` gegen das gecachte Original vergleichen. So spielen Formatierung und Kommentare keine Rolle.
   - Gleich: normales `approve`.
   - Verschieden: `submitEdit`-Pfad, aber mit `hint` und `exec` ergänzt, was heute verloren geht.
   - Bei `deny` wird der YAML-Stand ignoriert.
3. **`CCConnector`:** Der Zweig `AnswerCommand` ruft nur noch `answer(...)` auf. `CallEditCommand` und sein Zweig können dann entfallen, ebenso der Eintrag in `CommandRegistry` und die Sonderbehandlung in `PromptInputHandler.detectSelection` und `MCPConnector`. Ein reiner YAML-Block wäre dann ein Answer ohne Zeile, oder der Nutzer muss immer `/answer id allow` ergänzen. Das ist eine Designentscheidung.

## Risiken

- **Cache geht verloren** bei Neustart oder bei Antworten auf alte Requests. Dann gibt es kein Original. Ich würde das als geändert behandeln, wenn ein YAML vorliegt, oder alternativ nur als normales Approve.
- **Mehrere Pendings:** `checkControlEndpoint` zeigt nur das erste an. Der Cache muss pro `id` geführt werden.
- **MCPConnector** (Pfad ohne MCPC-Control-Server) hat gar kein Original. Dort würde man weiterhin einfach das YAML ausführen.
- **Sauberste Variante:** MCPC vergleicht serverseitig, und der Client schickt immer das YAML mit. Das ist unabhängig vom Client-Cache, braucht aber eine Änderung am MCPC-Server, den ich hier nicht sehe.

Soll ich die Client-Variante (Cache plus `answer(...)`) umsetzen und `CallEditCommand` entfernen? Oder erst den MCPC-Server prüfen?
Es wäre auch denkbar immer die volle Antwort an den MCPC zu schicken und dieser kann die Änderung prüfen oder Ebend auch nicht der Interne zustand könnte vollständig ignioriert werden und bildet nur die initiale Eingabe.
SystemInit:  id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 181
Control Request:
```yaml
id: 6a5a-7c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan
    pattern: control/tool|"approvals"
    include:
    - '*.java'
    - '*.py'
    - '*.ts'
    - '*.js'
    - '*.go'
    - '*.kt'
    exclude:
    - node_modules/**
    - build/**
    - target/**
    - .git/**
    limit: 20
  reason: "Prüfen, ob der MCPC-Server-Code (Control-Endpoint) im Workspace liegt, um die Idee gegen die echte Server-Logik zu bewerten"
```
/answer 6a5a-7c-0 deny das liegt hier: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control`
Control Request:
```yaml
id: 6a5a-7d-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control
    pattern: approvals|rejected|"exec"|"hint"|arguments|pending
    include:
    - '*.java'
    limit: 50
  reason: "Server-Logik im vom Nutzer genannten Verzeichnis lesen: wie Approvals, Edits und der interne Pending-Zustand verarbeitet werden"
```
/answer 6a5a-7d-0 allow
Control Request:
```yaml
id: 6a5a-7e-7d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control
```
/answer 6a5a-7e-7d allow
Control Request:
```yaml
id: 6a5a-7f-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control
    pattern: .
    exclude:
    - .git/**
    limit: 50
  reason: "Verzeichnis enthält offenbar keine .java-Treffer; ohne Dateifilter prüfen, welche Dateien dort liegen"
```
/answer 6a5a-7f-0 allow
Control Request:
```yaml
id: 6a5a-80-7f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control
      matches:
      - path: handler.py
        matches:
        - '"""HTTP handler for the human-in-the-loop control endpoint."""'
        - import logging
        - from http import HTTPStatus
        - "from typing import TYPE_CHECKING, Any"
        - "from urllib.parse import parse_qs, urlparse"
        - from xy.ai.mcpc.server import jsonrpc
        - "if TYPE_CHECKING:"
        - from xy.ai.mcpc.server.http_transport import StreamableHttpHandler
        - logger = logging.getLogger('xy.ai.mcpc.control')
        - "class ControlHandler:"
        - '"""Handles POST requests to the tool-control endpoint (``/control/tool``).'
        - Instantiate with the active :class:`StreamableHttpHandler` and call
        - ":meth:`matches` to check the request path, then :meth:`handle` to process"
        - it.
        - "Request body (JSON)::"
        - "{\"approvals\": [...]}"
        - "Each approval entry:"
        - "* ``{\"id\": \"…\"}``                              — simple approval"
        - "* ``{\"id\": \"…\", \"rejected\": true, \"reason\": \"…\"}``  — rejection"
        - "Response body (JSON)::"
        - "{\"pending\": [...]}"
        - '"""'
        - "def __init__(self, http: 'StreamableHttpHandler') -> None:"
        - self._http = http
        - "def matches(self) -> bool:"
        - '"""Return ``True`` when the request path equals ``config.control_path``."""'
        - return urlparse(self._http.path).path == self._http.config.control_path
        - "def handle(self) -> None:"
        - '"""Process a poll/approval request from the control client."""'
        - logger.debug('Control endpoint reached')
        - "'# type: ignore[attr-defined]'"
        - control = self._http.server.environment.control_manager
        - "if control is None:"
        - "logger.warning('Control: manager not enabled, returning 404')"
        - "self._http._send_http_error(HTTPStatus.NOT_FOUND, 'Tool control is not enabled')"
        - return
        - raw = self._http._read_body()
        - "logger.debug('Control: body read, length=%d', len(raw) if raw is not None else -1)"
        - "if raw is None:"
        - "if raw:"
        - "try:"
        - body = jsonrpc.parse_body(raw)
        - "except Exception as exc:"
        - "logger.warning('Control: invalid JSON body: %s', exc)"
        - "self._http._send_http_error(HTTPStatus.BAD_REQUEST, 'Invalid JSON body')"
        - "approvals = body.get('approvals', [])"
        - "if not isinstance(approvals, list):"
        - "logger.warning(\"Control: 'approvals' is not a list: %r\", approvals)"
      warning: "Limit of 50 matches reached (total 287); narrow pattern, directory or include/exclude filters."
```
/answer 6a5a-80-7f allow
ReasoningToken: 12
Control Request:
```yaml
id: 6a5a-81-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: class .*Control|def (submit|approve|resolve|apply|poll|wait|register|process)\w*|'arguments'|'result'|'rejected'|'exec'|'hint'
    include:
    - '*.py'
    limit: 50
  reason: Den ControlManager und die Verarbeitung von arguments/result/hint/exec im Server finden
```
/answer 6a5a-81-0 allow
Control Request:
```yaml
id: 6a5a-82-81
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: control/handler.py
        matches:
        - "class ControlHandler:"
      - path: control/manager.py
        matches:
        - "class ControlDecision:"
        - "item['arguments'] = _reason_last(self.arguments)"
        - "item['result'] = self.result"
        - "class ToolControlManager:"
        - "def submit_request(self, session: Session, tool_name: str, arguments: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:"
        - "def submit_result(self, session: Session, tool_name: str, result: dict[str, Any], *, request_number: int, auto_approve: bool=False) -> ControlDecision:"
        - "'result',"
        - "def process_approvals(self, approvals: list[dict[str, Any]]) -> None:"
        - "if approval.get('rejected'):"
        - "modified_arguments=approval.get('arguments'),"
        - "modified_result=approval.get('result'),"
        - "approval_hint=approval.get('hint'),"
        - approval.get('exec')))
      - path: server/http_transport.py
        matches:
        - "def apply_tools_header(config, comm_log, session_id: str, session, raw: str | None) -> None:"
        - "def apply_ccprofile_header(comm_log, session_id: str, session, raw: str | None) -> None:"
        - "def apply_control_header(comm_log, session_id: str, session, raw: str | None) -> None:"
      - path: server/jsonrpc.py
        matches:
        - "if 'result' in message or 'error' in message:"
        - "return {'jsonrpc': JSONRPC_VERSION, 'id': request_id, 'result': result}"
      - path: server/session.py
        matches:
        - "def register_agent_session(self, cli_session_id: str, *, model: str | None=None, profile: str | None=None) -> AgentSubSession:"
      - path: server/mcp_protocol.py
        matches:
        - "arguments = params.get('arguments', {})"
        - "label = f\"'{path}'\" if path else 'arguments'"
      - path: config.py
        matches:
        - "def resolved_ws_host(self) -> str:"
      - path: embedded/tools.py
        matches:
        - "def register_tool(tool: ToolDefinition | ToolProvider, registry: ToolRegistry, environment: AppEnvironment) -> None:"
      - path: cli/manager.py
        matches:
        - "def resolve(self, cli_session_id: str) -> CliSession | None:"
      - path: cli/session.py
        matches:
        - "def poll(self) -> int | None:"
        - "def wait(self, timeout: float | None=None) -> int:"
        - "elif kind == 'result':"
        - text = obj.get('result')
      - path: tools/mcp/github.py
        matches:
        - "def register_github_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:"
      - path: tools/mcp/client.py
        matches:
        - "'params': {'name': name, 'arguments': arguments}}, expect_response=True)"
        - "if isinstance(message, dict) and ('result' in message or 'error' in message):"
        - result = message.get('result')
      - path: tools/mcp/context7.py
        matches:
        - "def register_context7_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:"
      - path: tools/mcp/exa/fetch_results.py
        matches:
        - "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: tools/mcp/exa/fetch.py
        matches:
        - "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: tools/mcp/exa/search.py
        matches:
        - "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: tools/mcp/exa/__init__.py
        matches:
        - "def register_exa_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:"
      - path: tools/mcp/exa/search_results.py
        matches:
        - "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: tools/tool_usage/__init__.py
        matches:
        - "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: tools/ast/base.py
        matches:
        - "def resolve_by_prefix(located: list['Located'], target_id: str) -> 'Located | None':"
        - "def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> 'ImportOutcome':"
      - path: tools/ast/find.py
        matches:
        - "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: tools/ast/script.py
        matches:
        - "exec(compile(code, '<ast-script>', 'exec'), sandbox_globals, env)"
        - "if 'result' in env:"
        - "return AstScriptResult(result='success', value=repr(env['result']))"
        - description = "Run a restricted Python script against a file's AST when the single-operation ast_* tools (list/find/read/replace/insert/delete) are in… 'result' to return data; edits made via 'tree' are saved automatically. See the 'code' parameter for the full tree API and node-attribute reference."
        - "'description': \"Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limite…ne header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed.\"}},"
        - "structured: dict[str, Any] = {'result': result.result}"
        - "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      warning: "Limit of 50 matches reached (total 104); narrow pattern, directory or include/exclude filters."
      error: 2 match line(s) exceeded 300 characters and were shortened in the middle with an ellipsis (150 leading and trailing characters kept).
```
/answer 6a5a-82-81 allow
Control Request:
```yaml
id: 6a5a-83-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  reason: "Struktur des ToolControlManager lesen, um process_approvals und den internen Zustand zu verstehen"
```
/answer 6a5a-83-0 allow
Control Request:
```yaml
id: 6a5a-84-83
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
      nodes:
      - id: NXKGJd|ABmM9X
        excert: '''Human-in-the-loop tool control manager.\n\nImplements the Manager Pattern for …'
      - id: XNKbmy|OT0fAY
        excert: import logging
      - id: mGVkgb|UCEZKE
        excert: logger = logging.getLogger('xy.ai.mcpc.control')
      - id: ControlDecision
        excert: "@dataclass(slots=True) class ControlDecision:"
        docstring: "The outcome of a human review, produced by :meth:`ToolControlManager.process_ap…"
      - id: reason_last
        excert: "def _reason_last(arguments: dict[str, Any]) -> dict[str, Any]:"
        docstring: "Return *arguments* with ``reason``, if present, moved to the end so it is alway…"
      - id: PendingItem
        excert: "@dataclass class _PendingItem:"
        children:
        - id: PendingItem.QOjskF|HDKZo4
          excert: "id: str"
        - id: PendingItem.to_dict
          excert: "def to_dict(self) -> dict[str, Any]:"
      - id: ToolControlManager
        excert: "class ToolControlManager:"
        docstring: "Thread-safe manager for human-in-the-loop tool interception. Usage:: manager = …"
        children:
        - id: ToolControlManager.Jy5BQH|BflXLc
          excert: '''Thread-safe manager for human-in-the-loop tool interception.\n\n    Usage::\n\…'
        - id: ToolControlManager.init
          excert: "def __init__(self, timeout: float=_DEFAULT_TIMEOUT) -> None:"
        - id: ToolControlManager.submit_request
          excert: "def submit_request(self, session: Session, tool_name: str, arguments: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:"
          docstring: Block until the controller approves/rejects a tool-call *request*. Returns a :c…
        - id: ToolControlManager.submit_result
          excert: "def submit_result(self, session: Session, tool_name: str, result: dict[str, Any], *, request_number: int, auto_approve: bool=False) -> ControlDecision:"
          docstring: Block until the controller approves/replaces a tool-call *result*. Returns a :c…
        - id: ToolControlManager.get_pending
          excert: "def get_pending(self, session_id: str | None=None) -> list[dict[str, Any]]:"
          docstring: Return serialisable snapshots of items still awaiting a decision. When *session…
        - id: ToolControlManager.process_approvals
          excert: "def process_approvals(self, approvals: list[dict[str, Any]]) -> None:"
          docstring: Apply a batch of approval decisions from the control client. Each entry in *app…
        - id: ToolControlManager.cancel_session
          excert: "def cancel_session(self, session_id: str, reason: str='Connection closed') -> None:"
          docstring: Reject every pending item belonging to *session_id*. Called by a transport (HTT…
        - id: ToolControlManager.next_number
          excert: "def _next_number(self) -> int:"
        - id: ToolControlManager.enqueue
          excert: "def _enqueue(self, session: Session, phase: str, tool_name: str, arguments: dict[str, Any] | None, result: dict[str, Any] | None, *, number: int, ref: int) -> _PendingItem:"
        - id: ToolControlManager.wait
          excert: "def _wait(self, item: _PendingItem) -> ControlDecision:"
```
/answer 6a5a-84-83 allow
Control Request:
```yaml
id: 6a5a-85-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    ids:
    - ControlDecision
    - PendingItem
    - ToolControlManager.submit_request
    - ToolControlManager.submit_result
    - ToolControlManager.get_pending
    - ToolControlManager.process_approvals
  reason: "Server-Logik lesen: wie Edits (arguments/result), hint und exec heute verarbeitet werden und was der Pending-Zustand enthält"
```
/answer 6a5a-85-0 allow
Control Request:
```yaml
id: 6a5a-86-85
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
      nodes:
      - id: ControlDecision
        type: ClassDef
        code: |-
          @dataclass(slots=True)
          class ControlDecision:
              """The outcome of a human review, produced by :meth:`ToolControlManager.process_approvals`."""
              approved: bool
              'True when the call should proceed (possibly with modified data).'
              rejection_reason: str | None = None
              'Human-readable hint for the agent when ``approved`` is False.'
              approval_hint: str | None = None
              'Optional human-written hint attached to an *approval* (``/allow <id> <hint>``).\n\n    Unlike :attr:`rejection_reason`, this never aborts the call — it enriches\n    the eventual result instead. The same field is used for both the\n    ``request`` and ``result`` phase; when a hint is supplied at both phases\n    for the same call, the two are combined (see ``protocol.py``).\n    '
              modified_arguments: dict[str, Any] | None = None
              'Replacement arguments for the ``request`` phase (``None`` → keep originals).'
              modified_result: dict[str, Any] | None = None
              'Replacement result dict for the ``result`` phase (``None`` → keep original).'
              request_number: int = 0
              'Sequence number of the ``request`` phase item; set by :meth:`ToolControlManager.submit_request`.'
              force_result_review: bool = False
              "Set by an ``exec`` approval: behaves like ``allow`` for the request phase\n    but disables any tool-provided ``auto_approve`` for the subsequent result\n    phase, so a questionable command's output is always reviewed before use.\n    "
      - id: PendingItem
        type: ClassDef
        excert: "@dataclass class _PendingItem:"
        children:
        - id: PendingItem.QOjskF|HDKZo4
          type: statements
          code: |-
            id: str
            '# "request" | "result"'
            phase: str
            tool_name: str
            session_id: str
            '# populated in request phase'
            arguments: dict[str, Any] | None
            '# populated in result phase'
            result: dict[str, Any] | None
            _event: threading.Event = field(default_factory=threading.Event, repr=False)
            _decision: ControlDecision | None = field(default=None, repr=False)
        - id: PendingItem.to_dict
          type: FunctionDef
          code: |-
            def to_dict(self) -> dict[str, Any]:
                item: dict[str, Any] = {'id': self.id}
                if self.arguments is not None:
                    item['toolName'] = self.tool_name
                    item['arguments'] = _reason_last(self.arguments)
                if self.result is not None:
                    item['result'] = self.result
                return item
      - id: ToolControlManager.submit_request
        type: FunctionDef
        code: |-
          def submit_request(self, session: Session, tool_name: str, arguments: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:
              """Block until the controller approves/rejects a tool-call *request*.

                  Returns a :class:`ControlDecision`.  If the decision includes
                  ``modified_arguments``, the caller should substitute them before
                  invoking the tool handler.  ``ControlDecision.request_number`` must be
                  passed to :meth:`submit_result` to reference this call.
                  """
              number = self._next_number()
              if auto_approve or session.control_disabled:
                  logger.info('Auto-approving request for %s [%s] (tool-flagged or control disabled)', tool_name, session.id)
                  return ControlDecision(approved=True, request_number=number)
              item = self._enqueue(session, 'request', tool_name, arguments=arguments, result=None, number=number, ref=0)
              decision = self._wait(item)
              decision.request_number = number
              return decision
      - id: ToolControlManager.submit_result
        type: FunctionDef
        code: |-
          def submit_result(self, session: Session, tool_name: str, result: dict[str, Any], *, request_number: int, auto_approve: bool=False) -> ControlDecision:
              """Block until the controller approves/replaces a tool-call *result*.

                  Returns a :class:`ControlDecision`.  If the decision includes
                  ``modified_result``, the caller should use that instead of the
                  original result.

                  ``request_number`` is the number of the originating tool call
                  (:attr:`ControlDecision.request_number`); it forms the back reference
                  in the item id.

                  ``auto_approve`` is a hint set by the *tool itself* (via
                  ``ToolResult.auto_approve``), not derived from the shape of the
                  result.
                  """
              if auto_approve or session.control_disabled:
                  logger.info('Auto-approving result for %s [%s] (tool-flagged or control disabled)', tool_name, session.id)
                  return ControlDecision(approved=True)
              item = self._enqueue(
                  session,
                  'result',
                  tool_name,
                  arguments=None,
                  result=result,
                  number=self._next_number(),
                  ref=request_number)
              return self._wait(item)
      - id: ToolControlManager.get_pending
        type: FunctionDef
        code: |-
          def get_pending(self, session_id: str | None=None) -> list[dict[str, Any]]:
              """Return serialisable snapshots of items still awaiting a decision.

                  When *session_id* is given, only items belonging to that session are
                  returned, so parallel sessions don't see each other's pending
                  requests on the poll endpoint.
                  """
              with self._lock:
                  items = self._pending.values()
                  if session_id is not None:
                      items = (item for item in items if item.session_id == session_id)
                  return [item.to_dict() for item in items]
      - id: ToolControlManager.process_approvals
        type: FunctionDef
        code: |-
          def process_approvals(self, approvals: list[dict[str, Any]]) -> None:
              """Apply a batch of approval decisions from the control client.

                  Each entry in *approvals* must have at minimum an ``"id"`` key.
                  Accepted forms:

                  * ``{"id": "…"}`` — simple approval, keep original data.
                  * ``{"id": "…", "rejected": true, "reason": "…"}`` — rejection.
                  * ``{"id": "…", "arguments": {…}}`` — approve with modified arguments.
                  * ``{"id": "…", "result": {…}}`` — approve with replaced result.
                  * ``{"id": "…", "hint": "…"}`` — approve with a hint for the agent
                    (``/allow <id> <hint>``); does not reject the call, only enriches
                    the eventual result.
                  * ``{"id": "…", "exec": true}`` — approve like ``allow`` but disable
                    a possible tool-provided ``auto_approve`` for the result phase
                    (``/answer <id> exec``), forcing the result to be reviewed too.
                  """
              for approval in approvals:
                  item_id = approval.get('id')
                  if not isinstance(item_id, str):
                      logger.warning("Approval entry missing 'id', skipped: %s", approval)
                      continue
                  with self._lock:
                      item = self._pending.get(item_id)
                  if item is None:
                      logger.warning('Unknown approval id %s, skipped', item_id)
                      continue
                  if approval.get('rejected'):
                      decision = ControlDecision(approved=False, rejection_reason=approval.get(
                          'reason') or 'Reconsider your approach based on the requirements.')
                  else:
                      decision = ControlDecision(
                          approved=True,
                          modified_arguments=approval.get('arguments'),
                          modified_result=approval.get('result'),
                          approval_hint=approval.get('hint'),
                          force_result_review=bool(
                              approval.get('exec')))
                  item._decision = decision
                  with self._lock:
                      self._pending.pop(item_id, None)
                  logger.info('Dequeued control item %s [%s/%s]: approved=%s',
                              item.tool_name, item.phase, item_id, decision.approved)
                  item._event.set()
```
/answer 6a5a-86-85 allow
ReasoningToken: 163
Control Request:
```yaml
id: 6a5a-87-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: modified_arguments|modified_result|approval_hint
    include:
    - '*.py'
    exclude:
    - control/manager.py
    limit: 30
    intent: editing
  reason: "Prüfen, wie der Server modified_arguments/modified_result weiterverwendet und ob der Agent über Änderungen informiert wird"
```
/answer 6a5a-87-0 allow
Control Request:
```yaml
id: 6a5a-88-87
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: control/manager.py
        nodes:
        - id: ControlDecision.jPJsNr|RBrVk3
          matches:
          - "approval_hint: str | None = None"
        - id: ControlDecision.JDoXwg|iuwS7Z
          matches:
          - "modified_arguments: dict[str, Any] | None = None"
        - id: ControlDecision.HPV3vU|EULS6X
          matches:
          - "modified_result: dict[str, Any] | None = None"
        - id: ToolControlManager.submit_request.ZuqmEE|U5NIYz
          matches:
          - "``modified_arguments``, the caller should substitute them before"
        - id: ToolControlManager.submit_result.TJs5qr|rM36WU
          matches:
          - "``modified_result``, the caller should use that instead of the"
        - id: ToolControlManager.process_approvals.v0IhH7|7RG2Nh
          matches:
          - "modified_arguments=approval.get('arguments'),"
          - "modified_result=approval.get('result'),"
          - "approval_hint=approval.get('hint'),"
      - path: server/mcp_protocol.py
        nodes:
        - id: McpProtocol.handle_tools_call.2w8tpw|yuXXaJ
          matches:
          - "if decision.modified_arguments is not None:"
          - arguments = decision.modified_arguments
          - "if decision.approval_hint:"
          - "return ToolResult(structured_content={'answer': decision.approval_hint}).to_dict()"
          - request_hint = decision.approval_hint
        - id: McpProtocol.handle_tools_call.dDYvUB|3FtoJS
          matches:
          - "hint_parts = [h for h in (request_hint, decision.approval_hint) if h]"
          - "if decision.modified_result is not None:"
          - result_dict = dict(decision.modified_result)
          - "if combined_hint and decision.modified_result is not None:"
```
/answer 6a5a-88-87 allow
Result Stats: id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, total: 46512, in: 34, out: 9726, reason: 1028, read: 355284, write: 36752, start: 1794648217477, end: 1794648239694, time: 22112, total: 93840, io: 0, control: 105, sent: 0, recv: 367
ReasoningToken: 1028
Der Ansatz ist besser als mein Cache-Vorschlag. Das habe ich mir jetzt auch im Server angesehen (`mcpc/.../control/manager.py`, `server/mcp_protocol.py`).

## Was der Server schon kann

- `_PendingItem` hält `arguments` (Request-Phase) oder `result` (Result-Phase) für jede `id`. Das Original liegt also schon serverseitig, ein Client-Cache ist unnötig.
- `process_approvals` nimmt `arguments`, `result`, `hint`, `exec` und `rejected` **in einem Eintrag** an und baut daraus ein `ControlDecision`. Die Kombination aus Edit, Hint und Exec ist damit schon möglich.
- `mcp_protocol.handle_tools_call` ersetzt `arguments` bzw. `result`, sobald `modified_*` nicht `None` ist, **ohne zu vergleichen**. Der Agent erfährt nicht, dass der Nutzer etwas geändert hat. Bei unverändertem YAML wäre das Ergebnis dasselbe wie bei einem normalen Approve.

## Zu deiner Idee

Wenn der Client bei `/answer` immer das volle YAML mitschickt, wird `CallEditCommand` überflüssig. Dann gibt es nur noch eine Ausführungsart: **Answer = `{id, arguments|result aus dem YAML, hint, exec}`**.

Der Server hat dann zwei Möglichkeiten:

1. **Vergleichen:** Er prüft `approval.arguments == item.arguments`. Dict-Gleichheit ignoriert Reihenfolge und Formatierung, `_reason_last` stört also nicht. Bei Gleichheit setzt er `modified_*=None`. Bei Abweichung markiert er die Entscheidung als `modified` und kann den Agenten informieren, etwa mit „Arguments were modified by the user". Das ist heute nicht möglich.
2. **Internen Zustand ignorieren:** Das YAML des Clients gilt als Wahrheit. Das funktioniert technisch, ist aber schwächer, weil dann die Plausibilitätsprüfungen fehlen (siehe unten).

Ich würde Variante 1 nehmen. Der Server behält seinen Zustand als **Referenz**, der Client bleibt dumm und braucht keinen Cache. Neustarts und mehrere Pendings sind damit auch kein Problem mehr.

## Dinge, die ich absichern würde

- **Falscher Block:** `captureYamlBlock` nimmt den *letzten* YAML-Block vor der `/answer`-Zeile, ohne die `id` zu prüfen. Heute ist das harmlos, weil nur die `id` gesendet wird. Wenn das YAML immer mitgeschickt wird, muss `yaml.id == answer.id` gelten, sonst wird das YAML ignoriert oder es gibt einen Fehler.
- **Phase:** Ein `arguments`-Block an ein Result-Item (oder umgekehrt) muss serverseitig abgelehnt werden. `item.phase` ist dafür vorhanden. Der Client kennt die Phase schon über `blockType()`.
- **`toolName`:** Im YAML steht `toolName`, im Eintrag wird er nicht ausgewertet. Er sollte ignoriert werden, damit der Nutzer nicht versehentlich das Tool wechselt.
- **Deny:** Das YAML wird ignoriert, nur `reason` zählt.
- **Kaputtes YAML:** Wird heute in `submitEdit` als `IllegalArgumentException("YAML Error")` gemeldet. Das sollte bleiben. Es sollte nicht stillschweigend als unverändert gelten.

## Umfang

- **Client:** `CCControlClient.answer(id, action, hint, yaml)` baut genau einen Approval-Eintrag. `CCConnector` hat dann nur noch den Answer-Zweig. `CallEditCommand`, sein Eintrag in `CommandRegistry` und die Sonderbehandlung in `PromptInputHandler.detectSelection` entfallen.
- **Server:** In `process_approvals` kommen Vergleich, Phasenprüfung und ein `modified`-Flag dazu. Optional gibt `mcp_protocol` einen Hinweis an den Agenten.
- **Offen:** Ein reiner YAML-Block ohne `/answer`-Zeile wäre danach kein gültiger Befehl mehr. Das wäre eine Verhaltensänderung für den Nutzer. Außerdem nutzt der `MCPConnector` `CallEditCommand` für direkte Aufrufe ohne MCPC-Control. Dafür bräuchte es eine Entscheidung, ob er bleibt.

Soll ich beides umsetzen (Server und Client), und wie soll der Fall „nur YAML-Block ohne `/answer`" künftig behandelt werden?
Gib mir eine Minute
Result Stats: id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, total: 49369, in: 36, out: 9771, reason: 0, read: 392036, write: 39562, start: 1794648382531, end: 1794648384285, time: 1710, total: 95550, io: 0, control: 44, sent: 1, recv: 19
SystemInit:  id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Klar, nimm dir die Zeit. Ich warte auf deine Entscheidung und ändere bis dahin nichts.
Der User hat ohnehin die volle Kontrolle auch ein Toolwechsel wäre möglich. Entsprechend müssen Argumente auch nicht abgesichert werden. Der User kann auch ein Result beliebig anreichern oder verändern (Das müsste auch jetzt schon so sein. Approved wird beides Einzeln.)
Der ID Vergleich ist sinnvoll.
SystemInit:  id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 425
Control Request:
```yaml
id: 6a5a-89-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
    pattern: def handle_tools_call|submit_request|submit_result|tool_name|toolName
    include:
    - mcp_protocol.py
    limit: 30
    intent: editing
  reason: "Prüfen, wie aufwendig ein Toolwechsel durch den Nutzer in handle_tools_call wäre (toolName wird heute ignoriert)"
```
/answer 6a5a-89-0 allow
Control Request:
```yaml
id: 6a5a-8a-89
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
      matches:
      - path: mcp_protocol.py
        nodes:
        - id: McpProtocol.handle_tools_call.2w8tpw|yuXXaJ
          matches:
          - "decision = control.submit_request(session, name, arguments)"
        - id: McpProtocol.handle_tools_call.dDYvUB|3FtoJS
          matches:
          - decision = control.submit_result(
```
/answer 6a5a-8a-89 allow
Control Request:
```yaml
id: 6a5a-8b-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
  name: handle_tools_call
  reason: "Vollständige Methode lesen, um Toolwechsel und die Auswertung von modified_* zu bewerten"
```
/answer 6a5a-8b-0 allow
Control Request:
```yaml
id: 6a5a-8c-8b
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
      nodes: []
```
/answer 6a5a-8c-8b allow
Control Request:
```yaml
id: 6a5a-8d-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
  text: submit_request
  reason: Den Knoten mit submit_request/submit_result finden und im Kontext lesen
```
/answer 6a5a-8d-0 allow
Control Request:
```yaml
id: 6a5a-8e-8d
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
      nodes:
      - id: McpProtocol.handle_tools_call.2w8tpw|yuXXaJ
        code: |-
          if control is not None and (not skip_control):
              decision = control.submit_request(session, name, arguments)
              request_number = decision.request_number
              if not decision.approved:
                  reason = decision.rejection_reason
                  if name == TOOLNAME_ASK_USER:
                      return ToolResult(
                          structured_content={
                              'answer': reason or 'No answer given, anticipate and proceed'}).to_dict()
                  return ToolResult(
                      content=[
                          text_content(
                              f'Rejected by User: {reason}' if reason else 'Rejected by User. No reason was given; Reconsider your approach based on the requirements.')],
                      is_error=True).to_dict()
              if decision.modified_arguments is not None:
                  arguments = decision.modified_arguments
              force_result_review = decision.force_result_review
              if decision.approval_hint:
                  "# For ask_user, an approval hint *is* the human's answer"
                  if name == TOOLNAME_ASK_USER:
                      return ToolResult(structured_content={'answer': decision.approval_hint}).to_dict()
                  request_hint = decision.approval_hint
```
/answer 6a5a-8e-8d allow
ReasoningToken: 166
Control Request:
```yaml
id: 6a5a-8f-0
toolName: ask_user
arguments:
  question: "Soll ich jetzt umsetzen? Zwei Punkte sind noch offen: (1) Ein reiner ```yaml-Block ohne /answer-Zeile: soll er künftig nichts mehr auslösen (Answer ist Pflicht), oder soll er weiterhin wie ein Answer-Allow mit diesem YAML behandelt werden? (2) Toolwechsel: wird `toolName` im YAML heute ignoriert. Soll der Server ihn künftig wirklich übernehmen (name = geändertes Tool in handle_tools_call), oder reicht es, dass keine Absicherung erfolgt und toolName wie bisher wirkungslos bleibt?"
  reason: "Zwei Designentscheidungen fehlen vor der Umsetzung: Verhalten eines reinen YAML-Blocks ohne /answer und ob ein Toolwechsel serverseitig tatsächlich wirksam werden soll (erfordert Änderung in mcp_protocol.handle_tools_call)"
```
/answer 6a5a-8f-0 allow Der Tollwechsel braucht nicht gesichert werden. Prüfe: ein reiner YAML Block wird potentiell noch in einem reinen ToolCall verwendet aber ja, für für ein Call Edit ist es dann nicht mehr notwendig eine Selektion vorzunehmen.
ReasoningToken: 389
Control Request:
```yaml
id: 6a5a-90-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands
    pattern: CallCommand|CallEditCommand|ToolCommand|detect\(
    include:
    - '*.java'
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: CallEdit|Call Edit|call edit
    include:
    - '*.md'
    - '*.txt'
    - '*.xml'
    - '*.properties'
    - '*.yaml'
    - '*.yml'
    - '*.java'
    exclude:
    - build/**
    - target/**
    - .git/**
    limit: 30
  reason: "Prüfen, wo ein reiner YAML-Block noch als eigenständiger Tool Call dient (CallCommand, MCPConnector) und ob Doku/Konfiguration CallEdit erwähnt"
```
/answer 6a5a-90-0 allow
Control Request:
```yaml
id: 6a5a-91-90
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands
      matches:
      - path: ToolCommand.java
        matches:
        - "public class ToolCommand extends Command {"
        - "public ToolCommand() {"
        - "public ToolCommand(String toolId) {"
        - return ToolCommand.CMD_TOOL;
        - "return new ToolCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());"
      - path: CommandRegistry.java
        matches:
        - "new ToolCommand(), //"
        - "new CallCommand(), //"
        - new CallEditCommand());
        - "public static Command detect(String line) {"
      - path: CallCommand.java
        matches:
        - "public class CallCommand extends Command {"
        - "public CallCommand() {"
        - return CallCommand.CMD_CALL;
        - return new CallCommand();
      - path: CallEditCommand.java
        matches:
        - "public class CallEditCommand extends Command {"
        - "public CallEditCommand() {"
        - "private CallEditCommand(String yaml) {"
        - return new CallEditCommand(m.group(1).strip());
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: project/detect_change.md
        matches:
        - Ich würde gerne die beiden Commands vereinheitlichen. Bei einer Answer wird ein Toolausführung bestätigt. Bei einem Call Edit wird die Ausführung zusä…gerne erreichen das ein Answer Command selbst erkennen kann ob der Vorlaufende Tool Call verändert wurde. Verschaffe dir eine Übersicht und prüfe das.
        - '`/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java`'
        - '- /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java'
        - "- path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java"
        - "- id: CallEditCommand"
        - "- id: CallEditCommand.rU7BIw|BKuAVj"
        - "- id: CallEditCommand.CallEditCommand"
        - "excert: public CallEditCommand()"
        - "- id: CallEditCommand.CallEditCommand_2"
        - "excert: private CallEditCommand(String yaml)"
        - "- id: CallEditCommand.prefix"
        - "- id: CallEditCommand.matches"
        - "- id: CallEditCommand.parse"
        - "- id: CallEditCommand.processorAction"
        - "- id: CallEditCommand.yaml"
        - '- CallEditCommand'
        - "code: \"\\tpublic CallEditCommand() {\\n\\t\\tsuper();\\n\\t}\\n\""
        - "code: \"\\tprivate CallEditCommand(String yaml) {\\n\\t\\tsuper(yaml);\\n\\t}\\n\""
        - "code: \"\\t@Override\\n\\tpublic Command parse(String line) {\\n\\t\\tMatcher m = YAML_BLOCK.matcher(line.strip());\\n\\t\\tm.matches();\\n\\t\\treturn new CallEditCommand(m.group(1).strip());\\n\\t}\\n\""
      warning: "Limit of 30 matches reached (total 302); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
      error: 1 match line(s) exceeded 300 characters and were shortened in the middle with an ellipsis (150 leading and trailing characters kept).
```
/answer 6a5a-91-90 allow
Result Stats: id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, total: 58507, in: 48, out: 14004, reason: 596, read: 637052, write: 44455, start: 1794648662322, end: 1794648677020, time: 14616, total: 136092, io: 0, control: 82, sent: 0, recv: 254
ReasoningToken: 596
Ein reiner YAML-Block wird tatsächlich noch als eigener Tool Call gebraucht, aber nur in einem Connector. Ich habe noch nichts geändert.

## Wo ein reiner YAML-Block noch verwendet wird

| Stelle | Verhalten heute |
|---|---|
| **MCPConnector** | `CallEditCommand` führt den Block direkt aus (`invokeCall(parseYaml(...))`). Es gibt dort keinen MCPC-Control-Server und keinen Pending-Zustand, also auch nichts, was ein Answer bestätigen könnte. Das ist der reine Tool Call. |
| **CCConnector** | `CallEditCommand` ruft `submitEdit(yaml)` auf. Das lässt sich künftig durch ein Answer mit YAML ersetzen. |
| **`/call`** (`CallCommand`) | Beide Connectoren nutzen den YAML-Block vor der Zeile (`arg.yaml`). Das bleibt unverändert. |

`CallEditCommand` bleibt also für den `MCPConnector` bestehen. Aus dem `CCConnector` kann der Zweig entfallen.

## Zur Selektion

Die Selektion ist nur deshalb nötig, weil `CallEditCommand` nur erkannt wird, wenn der **gesamte Input** ein einzelner YAML-Block ist (`detectSelection`, Zweig `lines.length > 1`). Ein Answer braucht das nicht. Steht der Cursor auf der `/answer`-Zeile, holt der Cursor-Fallback den Block davor aus dem Dokument. Für das Bestätigen und Editieren in Claude Code wird die Selektion damit überflüssig.

## Plan

**MCPC-Server** (`control/manager.py`, `server/mcp_protocol.py`)
- `process_approvals` vergleicht `arguments` bzw. `result` mit dem Pending-Item. Gleiche Werte werden als unverändert behandelt. Das Ergebnis ist ein neues `modified`-Flag im `ControlDecision`.
- Keine Phasen- und Toolname-Prüfung, wie du es entschieden hast. `toolName` wird weiterhin ignoriert. Ein echter Toolwechsel bräuchte `name = ...` in `handle_tools_call`, das mache ich nur, wenn du es willst.
- Optional: `mcp_protocol` gibt dem Agenten einen Hinweis „modified by user". Soll ich das einbauen? Es ändert, was der Agent sieht.

**Client** (`CCControlClient`, `CCConnector`)
- Neue Methode `answer(id, action, hint, yaml)` baut genau einen Approval-Eintrag. Ist `yaml.id == answer.id`, wird `arguments` bzw. `result` übernommen, je nach `blockType()`. Sonst wird das YAML ignoriert. Bei `deny` wird es immer ignoriert.
- Der `CallEditCommand`-Zweig in `CCConnector` entfällt, `submitEdit` kann danach entfernt werden.
- Bei Allow und Exec wird das YAML immer mitgeschickt, und der Server erkennt selbst, ob es verändert wurde.

`CallEditCommand`, `CommandRegistry` und der `MCPConnector` bleiben, die Selektionslogik in `PromptInputHandler` kann für den CC-Fall entfallen.

Soll ich das so umsetzen, und soll der Agent einen Hinweis bei Änderungen bekommen?
Ein Hinweis ist noch nicht nötig. Setze das um.
SystemInit:  id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 6a5a-92-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
    id: AnswerCommand.setYaml
    position: after
    source: "\n\t/** The raw YAML block preceding this answer (possibly edited by the user), or {@code null}. */\n\tpublic String yaml() {\n\t\treturn yaml;\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
    id: CCControlClient.deny
    position: after
    source: |2

        /**
         * Unified answer to a pending item. For allow/exec the YAML block preceding the
         * answer (if it carries the same id) is always sent along as {@code arguments}
         * or {@code result}; the server detects whether it differs from the original.
         * A deny ignores the block.
         */
        public ArrayNode answer(String id, AnswerCommand.Action action, String text, String yamlBlock) {
          if (action == AnswerCommand.Action.Deny)
            return deny(id, text);
          JsonNode edited = editedNode(id, yamlBlock);
          JsonNode arguments = edited == null || edited.path("arguments").isMissingNode() || edited.get("arguments").isNull() ? null : edited.get("arguments");
          JsonNode result = edited == null || edited.path("result").isMissingNode() || edited.get("result").isNull() ? null : edited.get("result");
          return submit(approvalNode(id, arguments, result, null, text == null || text.isBlank() ? null : text, action == AnswerCommand.Action.Exec));
        }

        /** Parses the YAML block; returns it only if it is an object whose id equals {@code id}. */
        private JsonNode editedNode(String id, String yamlBlock) {
          if (yamlBlock == null || yamlBlock.isBlank())
            return null;
          JsonNode node;
          try {
            node = fromYaml(yamlBlock);
          } catch (Exception e) {
            throw new IllegalArgumentException("YAML Error", e);
          }
          return node != null && node.isObject() && id != null && id.equals(node.path("id").asText(null)) ? node : null;
        }
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    id: ToolControlManager
    position: before
    source: |+
      def _changed(submitted: dict[str, Any] | None, original: dict[str, Any] | None) -> dict[str, Any] | None:
          """Return *submitted* only if it differs from *original*; ``None`` means unchanged."""
          if submitted is None or submitted == original:
              return None
          return submitted


  reason: "Neue Methoden anlegen: AnswerCommand.yaml(), CCControlClient.answer(...) mit ID-Prüfung, und serverseitiger Änderungsvergleich-Helper"
```
/answer 6a5a-92-0 allow
ReasoningToken: 3183
Control Request:
```yaml
id: 6a5a-93-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    id: ToolControlManager.process_approvals
    start_marker: modified_arguments=approval.get(
    end_marker: "modified_result=approval.get('result'),"
    content: |-
      modified_arguments=_changed(approval.get('arguments'), item.arguments),
                      modified_result=_changed(approval.get('result'), item.result),
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    start_marker: "AnswerCommand ac) {"
    end_marker: "instanceof CallCommand) {"
    content: |-
      AnswerCommand ac) {
            ts.control(() -> controlClient.answer(ac.parameter(0), ac.action(), ac.parameter(2), ac.yaml()));
            session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
          } else if (command instanceof CallCommand) {
    removeImport:
    - import xy.ai.conscious.commands.CallEditCommand;
  reason: "Server: unveränderte Argumente/Result als unmodifiziert behandeln. CCConnector: Answer ruft die vereinheitlichte answer(...) auf, CallEditCommand-Zweig entfällt"
```
/answer 6a5a-93-0 allow
Control Request:
```yaml
id: 6a5a-94-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  reason: Kompilierbarkeit der geänderten Dateien prüfen
```
/answer 6a5a-94-0 allow
Control Request:
```yaml
id: 6a5a-95-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
    ids:
    - CCConnector.executeRequest
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    ids:
    - ToolControlManager.process_approvals
  reason: Ergebnis der Edits gegenlesen
```
/answer 6a5a-95-0 allow
Control Request:
```yaml
id: 6a5a-96-95
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.executeRequest
        type: MethodDeclaration
        code: |2
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
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
      nodes:
      - id: ToolControlManager.process_approvals
        type: FunctionDef
        code: |-
          def process_approvals(self, approvals: list[dict[str, Any]]) -> None:
              """Apply a batch of approval decisions from the control client.

                  Each entry in *approvals* must have at minimum an ``"id"`` key.
                  Accepted forms:

                  * ``{"id": "…"}`` — simple approval, keep original data.
                  * ``{"id": "…", "rejected": true, "reason": "…"}`` — rejection.
                  * ``{"id": "…", "arguments": {…}}`` — approve with modified arguments.
                  * ``{"id": "…", "result": {…}}`` — approve with replaced result.
                  * ``{"id": "…", "hint": "…"}`` — approve with a hint for the agent
                    (``/allow <id> <hint>``); does not reject the call, only enriches
                    the eventual result.
                  * ``{"id": "…", "exec": true}`` — approve like ``allow`` but disable
                    a possible tool-provided ``auto_approve`` for the result phase
                    (``/answer <id> exec``), forcing the result to be reviewed too.
                  """
              for approval in approvals:
                  item_id = approval.get('id')
                  if not isinstance(item_id, str):
                      logger.warning("Approval entry missing 'id', skipped: %s", approval)
                      continue
                  with self._lock:
                      item = self._pending.get(item_id)
                  if item is None:
                      logger.warning('Unknown approval id %s, skipped', item_id)
                      continue
                  if approval.get('rejected'):
                      decision = ControlDecision(approved=False, rejection_reason=approval.get(
                          'reason') or 'Reconsider your approach based on the requirements.')
                  else:
                      decision = ControlDecision(
                          approved=True, modified_arguments=_changed(
                              approval.get('arguments'), item.arguments), modified_result=_changed(
                                  approval.get('result'), item.result), approval_hint=approval.get('hint'), force_result_review=bool(
                                      approval.get('exec')))
                  item._decision = decision
                  with self._lock:
                      self._pending.pop(item_id, None)
                  logger.info('Dequeued control item %s [%s/%s]: approved=%s',
                              item.tool_name, item.phase, item_id, decision.approved)
                  item._event.set()
```
/answer 6a5a-96-95 allow
Result Stats: id=cc3badab-d8a6-4fb1-b7b2-35789e3b2f9e, total: 75778, in: 54, out: 20534, reason: 0, read: 779812, write: 55190, start: 1794648869590, end: 1794648878396, time: 8740, total: 180992, io: 0, control: 66, sent: 0, recv: 196
Beides ist umgesetzt, und die geänderten Dateien kompilieren laut `ast_validate`. Getestet habe ich nichts, weder Java noch Python.

## Server (MCPC)

In `control/manager.py` ist ein Helper `_changed(submitted, original)` neu. `process_approvals` setzt `modified_arguments` und `modified_result` nur noch, wenn das Eingereichte vom Pending-Item abweicht. Bei Gleichheit wirkt es wie ein normales Approve. Der Agent bekommt keinen Hinweis, so wie du es wolltest. Die Python-Datei ist beim Schreiben umformatiert worden (der Aufruf von `ControlDecision(...)` ist umbrochen), das ist nur Kosmetik.

## Client (Java)

- **`AnswerCommand`:** Es gibt jetzt `yaml()`, den Getter zum bisherigen `setYaml`.
- **`CCControlClient.answer(id, action, text, yaml)`:**
  - Bei Deny wird das YAML ignoriert und `deny(id, text)` aufgerufen.
  - Bei Allow und Exec wird das YAML geparst. Hat es dieselbe `id` wie die Antwort, gehen `arguments` bzw. `result` mit. Sonst wird es ignoriert.
  - Hint und Exec werden im selben Approval gesendet, das ging beim alten Edit-Pfad verloren.
  - Kaputtes YAML wirft weiterhin `IllegalArgumentException("YAML Error")`.
- **`CCConnector`:** Der Answer-Zweig ruft nur noch `answer(...)` auf. Der `CallEditCommand`-Zweig und sein Import sind entfernt.

## Unverändert

- `CallEditCommand`, `CommandRegistry` und der `MCPConnector` sind unverändert, denn der reine Tool Call läuft dort weiter über den YAML-Block.
- `submitEdit` bleibt, weil `/call` es noch nutzt.
- `PromptInputHandler` habe ich nicht angefasst. Die Selektionslogik für `CallEditCommand` ist dort noch drin, sie wird für Answer aber nicht mehr gebraucht.

## Zu beachten

- Ein reiner YAML-Block ohne `/answer` führt im `CCConnector` jetzt zu `UnsupportedOperationException("Unsupported command: ```yaml")`. Vorher wurde er per `submitEdit` bestätigt. Wenn dir das zu hart ist, kann ich dort eine verständlichere Fehlermeldung einbauen.
- Der Client schickt bei Allow und Exec jetzt immer den Block mit. Ist der letzte YAML-Block vor `/answer` kein Tool Call, wird er wegen der abweichenden `id` ignoriert. Ist er kein gültiges YAML, gibt es den Fehler `YAML Error`.
