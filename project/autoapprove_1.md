Verschaffe dir einen Überblick und analysiere die Implementierung eines Auto-Approve.

- Zielpackage: `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove`

Der Approver sitzt in xy.ai.conscious.connector.harness.PromptHandler.PromptRun.run(IProgress) nachdem der Tag ausgetauscht wurde und erhält die `AIAnswer`.
Der Approver verarbeitet "Control Request" von MCP gebundenen Connectoren als auch Tool-Calls von Tool Loops direkter Modelle.
Er kennt vier Ergebnisse:

1. `Pass` - die Answer ist entweder kein Tool-Call, kein Tool-Result oder aber der Approver trifft keine Entscheidung über `Deny` oder `Allow`/`Call`.
2. `Deny` - der Approver erkennt ein Tool-Call, ein Tool-Result, lehnt es aber ab. Mit oder ohne optionalen Grund.
3. `Allow` - ein Tool-Call oder Tool-Result für ein "Control Request" wurde genehmigt. Mit oder ohne Hinweis Grund.
4. `Call` - ein Tool-Call wurde erkannt und genehmigt (Nicht MCP Variante ohne "Control Request").

Der Approver lässt also die Antwort immer passieren aber führt mödlicherweise direkt die Folgeaktion aus -> Neuen Prompt Job bei `Deny`, `Allow`,`Call`

- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java`
- Es ist möglich, das eine Antwort mehrere Toolcalls beinhaltet
- Der Approver kann die Antwort verändern oder anreichern.
- Der Approver kann auf den Stati der Session reagieren

Vom Approver erkannte und auch verwendete Commands:

- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ToolCommand.java`
- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ControlRequestCommand.java`
- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java`
- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallCommand.java`
- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java`

## Beispiele

- Der Approver erkennt ein "Control Request" für einen Bash Aufruf. CWD ist per Regel ausgeschlossen. Der Approver lehnt den Request mit einer Begründung ab die den Agenten darauf Hinweis in einem anderen Verzeichnis zu starten,
- Der Approver erkennt einen Search Aufruf, prüft die Argumente genehmigt die Ausführung.
- Der Approver erkennt in einem Tool-Loop das ein Grep Kommando ausgeführt werden soll. Er genehmigt den Aufruf und stößt die Toolausführung an.
- Der Approver erkennt ein Tool-Result eines "Control Request". Die Zeichenanzahl übersteigt ein Limit. Der Approver lässt die Antwort ohne Aktion passieren. Der Operator wird dies entscheiden.
- Der Approver erkennt, dass es Turn 10 ist aber kein Cache-Read in den Stats gezählt wird. Der Approver Reichert die Antwort um eine Warnung an ("//warning:...") und lässt sie passieren.
- Der Approver erkennt das die Idle-Time der Session einen Threshold übersteigt. Er sendet daraufhin selbst eine definierte Anweisung, als Prompt an das Modell die Aufgabe fortzusetzen.
- Der Approver verwendet einen zukünftigen Mechanismus, um die Antwort auszuwerten (Rekursives LLM Call). Er stellt fest das noch offene Punkte übrig sind und weißt das Modell an diese Punkte zu erledigen.

## Zweck

Der Approver dient der Unterstützung eines menschlichen Operators. Der Operator pflegt einen Bestand von Regeln zur Auswertung. Der Approver wertet diese Regeln aus und übernimmt anteilig die Kontrolle bei eindeutigen Entscheidungen.

- Aktionen und Stati des Approvers werden später abgegriffen und dem Operator angezeigt.

## Regel

- Es wird ein Interface angelegt das Regeln implementieren
- Regeln werden im Package `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules` abgelegt.
- Der Approver hält eine Registry von Regeln in einer Reihenfolge
	- Regeln werden in der Registry registriert
	- Jede Regeln wird gegen den Input evaluiert
- Eine Regel erhält alle notwendigen Input und Outputparameter wie Session, Awnser und Prompt über entscheiden zu können.
- Alle Regeln werden durchlaufen und bestimmen ihre Anwendbarkeit selbst (Voter)
	- Regeln können mit `Deny`, `Allow` oder `Abstein` (wird ignoriert) antworten.
	- Regeln können Metainformationen hinzufügen wie einen Ablehnungsgrund (Reason) oder Hinweis (Hint)
- Die Regeln entscheiden nur und der Approver setzt die Resultate der Votings um.
SystemInit:  id=123c3b3a-c226-4b4d-8683-f62d56bab1c9, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich verschaffe mir zuerst einen Überblick über das Zielpackage und die genannten Dateien.
Control Request:
```yaml
id: 1359-1
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: .
    include:
    - '*'
    limit: 50
    intent: survey
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands
    pattern: ^(public |final |abstract )*(class|interface|enum|record)
    include:
    - '*.java'
    limit: 50
  reason: Bestandsaufnahme des Zielpackages autoapprove und der Command-Klassen
```
/answer 1359-1 allow
ReasoningToken: 16
Control Request:
```yaml
id: 1359-2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands
      matches:
      - path: ToolCommand.java
        matches:
        - "public class ToolCommand extends Command {"
      - path: CommandRegistry.java
        matches:
        - "public class CommandRegistry {"
      - path: Command.java
        matches:
        - "public abstract class Command {"
      - path: AnswerCommand.java
        matches:
        - "public class AnswerCommand extends Command {"
      - path: ExitCommand.java
        matches:
        - "public class ExitCommand extends Command {"
      - path: CallCommand.java
        matches:
        - "public class CallCommand extends Command {"
      - path: ResumeCommand.java
        matches:
        - "public class ResumeCommand extends Command {"
      - path: ControlRequestCommand.java
        matches:
        - "public class ControlRequestCommand extends Command {"
      - path: CallEditCommand.java
        matches:
        - "public class CallEditCommand extends Command {"
    errors:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      pattern: .
      error: Directory not found or not a directory.
```
/answer 1359-2 allow
Control Request:
```yaml
id: 1359-3
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ToolCommand.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ControlRequestCommand.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallCommand.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
  reason: Struktur von PromptHandler und den fünf genannten Commands lesen
```
/answer 1359-3 allow
Control Request:
```yaml
id: 1359-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
      nodes:
      - id: NZKTrV|g5OLnZ
        signature: import java.util.ArrayList;
      - id: PromptHandler
        signature: "/**  * Prompt-specific orchestration extracted from {"
        docstring: "Prompt-specific orchestration extracted from {@code AISessionManager}: drives"
        children:
        - id: PromptHandler.QEglBi|s8XuCw
          signature: private final AdaptingConnector connector;
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, IActiveEditor editorListener, AdaptingC…"
        - id: PromptHandler.addAnswerObs
          signature: public void addAnswerObs(Consumer<AIAnswer> obs)
        - id: PromptHandler.execute
          signature: public void execute(IExecutor exec)
        - id: PromptHandler.PromptRun
          signature: "private class PromptRun implements IMonitoredRunnable {"
          children:
          - id: PromptHandler.PromptRun.dBu9kx|851iHn
            signature: private final IExecutor exec;
          - id: PromptHandler.PromptRun.PromptRun
            signature: private PromptRun(IExecutor exec)
          - id: PromptHandler.PromptRun.run
            signature: public void run(IProgress mon)
        - id: PromptHandler.queueAsync
          signature: public void queueAsync(IExecutor exec)
        - id: PromptHandler.queueSync
          signature: "private void queueSync(IExecutor exec, IProgress mon)"
        - id: PromptHandler.queueAndSubmit
          signature: public void queueAndSubmit(IExecutor exec)
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(IExecutor exec, boolean batchFix, IProgres…"
        - id: PromptHandler.preprocessPrompt
          signature: private void preprocessPrompt(Prompt prompt)
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(IExecutor exec, IModelRequest req, IProgress mon)"
        - id: PromptHandler.checkGitSupport
          signature: "public void checkGitSupport(IExecutor exec, IModelRequest req)"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ToolCommand.java
      nodes:
      - id: ToolCommand
        signature: "public class ToolCommand extends Command {"
        children:
        - id: ToolCommand.18LEdl|nBKSDD
          signature: public static final String CMD_TOOL = "/tool";
        - id: ToolCommand.ToolCommand
          signature: public ToolCommand()
        - id: ToolCommand.ToolCommand_2
          signature: public ToolCommand(String toolId)
        - id: ToolCommand.prefix
          signature: public String prefix()
        - id: ToolCommand.matches
          signature: public boolean matches(String line)
        - id: ToolCommand.parse
          signature: public Command parse(String line)
        - id: ToolCommand.processorAction
          signature: public ProcessorAction processorAction()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ControlRequestCommand.java
      nodes:
      - id: Rud7VC|2ABMxb
        signature: import xy.ai.conscious.core.IEditorInterface;
      - id: ControlRequestCommand
        signature: "public class ControlRequestCommand extends Command {"
        children:
        - id: ControlRequestCommand.ControlRequestCommand
          signature: public ControlRequestCommand()
        - id: ControlRequestCommand.prefix
          signature: public String prefix()
        - id: ControlRequestCommand.parse
          signature: public Command parse(String line)
        - id: ControlRequestCommand.processorAction
          signature: public ProcessorAction processorAction()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java
      nodes:
      - id: uUL6Qs|J3MY6H
        signature: import java.util.regex.Matcher;
      - id: CallEditCommand
        signature: /**  * Detected when the whole input consists solely of a single YAML block  */…
        docstring: Detected when the whole input consists solely of a single YAML block
        children:
        - id: CallEditCommand.rU7BIw|BKuAVj
          signature: private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)\\R?…
        - id: CallEditCommand.CallEditCommand
          signature: public CallEditCommand()
        - id: CallEditCommand.CallEditCommand_2
          signature: private CallEditCommand(String yaml)
        - id: CallEditCommand.prefix
          signature: public String prefix()
        - id: CallEditCommand.matches
          signature: public boolean matches(String line)
        - id: CallEditCommand.parse
          signature: public Command parse(String line)
        - id: CallEditCommand.processorAction
          signature: public ProcessorAction processorAction()
        - id: CallEditCommand.yaml
          signature: public String yaml()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallCommand.java
      nodes:
      - id: CallCommand
        signature: "public class CallCommand extends Command {"
        children:
        - id: CallCommand.SX50AH|QuqZES
          signature: public static final String CMD_CALL = "/call";
        - id: CallCommand.CallCommand
          signature: public CallCommand()
        - id: CallCommand.prefix
          signature: public String prefix()
        - id: CallCommand.matches
          signature: public boolean matches(String line)
        - id: CallCommand.parse
          signature: public Command parse(String line)
        - id: CallCommand.processorAction
          signature: public ProcessorAction processorAction()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
      nodes:
      - id: AnswerCommand
        signature: "public class AnswerCommand extends Command {"
        children:
        - id: AnswerCommand.LN3qky|soKy4k
          signature: public static final String CMD_ANSWER = "/answer";
        - id: AnswerCommand.AnswerCommand
          signature: public AnswerCommand()
        - id: AnswerCommand.AnswerCommand_2
          signature: "public AnswerCommand(String id, String action, String reason)"
        - id: AnswerCommand.prefix
          signature: public String prefix()
        - id: AnswerCommand.matches
          signature: public boolean matches(String line)
        - id: AnswerCommand.parse
          signature: public Command parse(String line)
        - id: AnswerCommand.processorAction
          signature: public ProcessorAction processorAction()
        - id: AnswerCommand.Action
          signature: "public enum Action {"
        - id: AnswerCommand.BlockType
          signature: /**  * Distinguishes a pending tool call (arguments) from a pending tool result…
          docstring: Distinguishes a pending tool call (arguments) from a pending tool result (resul…
        - id: AnswerCommand.np6Lp5|nApalK
          signature: private String yaml;
        - id: AnswerCommand.setYaml
          signature: public void setYaml(String yaml)
          docstring: "Sets the raw YAML block this answer refers to, used by {@link #blockType()}."
        - id: AnswerCommand.blockType
          signature: public BlockType blockType()
        - id: AnswerCommand.action
          signature: public Action action()
```
/answer 1359-4 allow
Text:
 Das Package `autoapprove` existiert noch nicht. Jetzt lese ich die Details.
Control Request:
```yaml
id: 1359-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
    ids:
    - PromptHandler.PromptRun
    - PromptHandler.queueAsync
    - PromptHandler.queueSync
    - PromptHandler.queueAndSubmit
    - PromptHandler.executeInner
    - PromptHandler.PromptHandler
    - PromptHandler.execute
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ToolCommand.java
    ids:
    - ToolCommand
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ControlRequestCommand.java
    ids:
    - ControlRequestCommand
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java
    ids:
    - CallEditCommand
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallCommand.java
    ids:
    - CallCommand
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
    ids:
    - AnswerCommand
  reason: "Quellcode von PromptRun und den Commands lesen, um Integrationspunkt und Erkennungslogik zu verstehen"
```
/answer 1359-5 allow
Control Request:
```yaml
id: 1359-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler.PromptRun
        type: ClassOrInterfaceDeclaration
        signature: "private class PromptRun implements IMonitoredRunnable {"
        children:
        - id: PromptHandler.PromptRun.dBu9kx|851iHn
          type: StatementGroup
          code: "\t\tprivate final IExecutor exec;\n"
        - id: PromptHandler.PromptRun.PromptRun
          type: ConstructorDeclaration
          code: "\t\tprivate PromptRun(IExecutor exec) {\n\t\t\tthis.exec = exec;\n\t\t}\n"
        - id: PromptHandler.PromptRun.run
          type: MethodDeclaration
          code: "\t\t@Override\n\t\tpublic void run(IProgress mon) {\n\t\t\tIProgress sub = mon.setTask(\"Executing prompt\", 4);\n\t\t\tString reqId = null;\n\t\t\ttry {\n\t\t\t\tsub.setTaskName(\"Prepare inputs\");\n\t\t\t\tvar req = prepareRequest(exec, false, sub);\n\t\t\t\tsub.worked();\n\t\t\t\tsub.setTaskName(\"Insert Tag\");\n\t\t\t\teditIfc.insertTag(exec, req, sub);\n\t\t\t\treqId = req.getID();\n\t\t\t\tmon.worked();\n\n\t\t\t\tsub.setTaskName(\"Execute prompt\");\n\t\t\t\tAIAnswer ans = executeInner(exec, req, sub);\n\t\t\t\tmon.worked();\n\t\t\t\tsub.setTaskName(\"Process Answer\");\n\t\t\t\teditIfc.replaceTag(ans, sub);\n\t\t\t\tmon.worked();\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t// uncatched error case\n\t\t\t\tif (reqId != null) {\n\t\t\t\t\tAIAnswer error = new AIAnswer(reqId);\n\t\t\t\t\terror.answer = e.getMessage();\n\t\t\t\t\teditIfc.replaceTag(error, sub);\n\t\t\t\t}\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t}\n"
      - id: PromptHandler.queueAsync
        type: MethodDeclaration
        code: "\tpublic void queueAsync(IExecutor exec) {\n\t\texec.schedule(\"Enqueue Prompt\", mon -> {\n\t\t\tqueueSync(exec, mon);\n\t\t});\n\t}\n"
      - id: PromptHandler.queueSync
        type: MethodDeclaration
        code: "\tprivate void queueSync(IExecutor exec, IProgress mon) {\n\t\tIProgress sub = mon.setTask(\"Enqueue batch prompt\", 3);\n\t\tsub.setTaskName(\"Prepare inputs\");\n\t\tvar req = prepareRequest(exec, true, sub.subProgress());\n\t\tsub.setTaskName(\"Insert Tag\");\n\t\teditIfc.insertTag(exec, req, sub.subProgress());\n\t\tsub.setTaskName(\"Enqueue prompt\");\n\t\tbatch.enqueue(req, sub.subProgress());\n\t}\n"
      - id: PromptHandler.queueAndSubmit
        type: MethodDeclaration
        code: "\tpublic void queueAndSubmit(IExecutor exec) {\n\t\texec.schedule(\"Enqueue Prompt\", (mon) -> {\n\t\t\tqueueSync(exec, mon);\n\t\t\tbatch.submitBatches(mon);\n\t\t});\n\t}\n"
      - id: PromptHandler.executeInner
        type: MethodDeclaration
        code: "\tprivate AIAnswer executeInner(IExecutor exec, IModelRequest req, IProgress mon) {\n\t\texec.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));\n\t\tIModelResponse resp = connector.executeRequest(req, mon);\n\t\tAIAnswer res = connector.convertResponse(resp, mon);\n\t\tres.prompt = req.getPrompt();\n\t\texec.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));\n\t\tcheckGitSupport(exec, req);\n\t\treturn res;\n\t}\n"
      - id: PromptHandler.PromptHandler
        type: ConstructorDeclaration
        code: "\tpublic PromptHandler(ConfigManager cfg, IActiveEditor editorListener, AdaptingConnector connector,\n\t\t\tAIBatchManager batch, IEditorInterface editIfc, IGitSupport gitSupport) {\n\t\tthis.connector = connector;\n\t\tthis.editIfc = editIfc;\n\t\tthis.batch = batch;\n\t\tthis.input = new PromptInputHandler(cfg, editorListener);\n\t\tthis.gitSupport = gitSupport;\n\t}\n"
      - id: PromptHandler.execute
        type: MethodDeclaration
        code: "\tpublic void execute(IExecutor exec) {\n\t\texec.schedule(\"Starting Prompt\", new PromptRun(exec));\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ToolCommand.java
      nodes:
      - id: ToolCommand
        type: ClassOrInterfaceDeclaration
        signature: "public class ToolCommand extends Command {"
        children:
        - id: ToolCommand.18LEdl|nBKSDD
          type: StatementGroup
          code: "\tpublic static final String CMD_TOOL = \"/tool\";\n"
        - id: ToolCommand.ToolCommand
          type: ConstructorDeclaration
          code: "\tpublic ToolCommand() {\n\t\tthis(\"\");\n\t}\n"
        - id: ToolCommand.ToolCommand_2
          type: ConstructorDeclaration
          code: "\tpublic ToolCommand(String toolId) {\n\t\tsuper(toolId);\n\t}\n"
        - id: ToolCommand.prefix
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String prefix() {\n\t\treturn ToolCommand.CMD_TOOL;\n\t}\n"
        - id: ToolCommand.matches
          type: MethodDeclaration
          code: "\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\");\n\t}\n"
        - id: ToolCommand.parse
          type: MethodDeclaration
          code: "\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new ToolCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());\n\t}\n"
        - id: ToolCommand.processorAction
          type: MethodDeclaration
          code: "\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/ControlRequestCommand.java
      nodes:
      - id: ControlRequestCommand
        type: ClassOrInterfaceDeclaration
        signature: "public class ControlRequestCommand extends Command {"
        children:
        - id: ControlRequestCommand.ControlRequestCommand
          type: ConstructorDeclaration
          code: "\tpublic ControlRequestCommand() {\n\t\tsuper();\n\t}\n"
        - id: ControlRequestCommand.prefix
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String prefix() {\n\t\treturn IEditorInterface.CONTROL_REQUEST;\n\t}\n"
        - id: ControlRequestCommand.parse
          type: MethodDeclaration
          code: "\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new ControlRequestCommand();\n\t}\n"
        - id: ControlRequestCommand.processorAction
          type: MethodDeclaration
          code: "\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.IGNORE_BLOCK;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallEditCommand.java
      nodes:
      - id: CallEditCommand
        type: ClassOrInterfaceDeclaration
        signature: /**  * Detected when the whole input consists solely of a single YAML block  */…
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CallCommand.java
      nodes:
      - id: CallCommand
        type: ClassOrInterfaceDeclaration
        signature: "public class CallCommand extends Command {"
        children:
        - id: CallCommand.SX50AH|QuqZES
          type: StatementGroup
          code: "\tpublic static final String CMD_CALL = \"/call\";\n"
        - id: CallCommand.CallCommand
          type: ConstructorDeclaration
          code: "\tpublic CallCommand() {\n\t\tsuper();\n\t}\n"
        - id: CallCommand.prefix
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String prefix() {\n\t\treturn CallCommand.CMD_CALL;\n\t}\n"
        - id: CallCommand.matches
          type: MethodDeclaration
          code: "\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().startsWith(prefix());\n\t}\n"
        - id: CallCommand.parse
          type: MethodDeclaration
          code: "\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new CallCommand();\n\t}\n"
        - id: CallCommand.processorAction
          type: MethodDeclaration
          code: "\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
      nodes:
      - id: AnswerCommand
        type: ClassOrInterfaceDeclaration
        signature: "public class AnswerCommand extends Command {"
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
```
/answer 1359-6 allow
Control Request:
```yaml
id: 1359-7
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/Command.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CommandRegistry.java
  reason: Basisklasse Command und CommandRegistry verstehen
```
/answer 1359-7 allow
Control Request:
```yaml
id: 1359-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/Command.java
      nodes:
      - id: Command
        signature: "public abstract class Command {"
        children:
        - id: Command.ProcessorAction
          signature: "public enum ProcessorAction {"
        - id: Command.iKqF62|lJcluI
          signature: "protected final String[] parameters;"
        - id: Command.Command
          signature: protected Command(String... parameters)
        - id: Command.prefix
          signature: public abstract String prefix()
        - id: Command.matches
          signature: public boolean matches(String line)
        - id: Command.parse
          signature: public abstract Command parse(String line)
        - id: Command.processorAction
          signature: public abstract ProcessorAction processorAction()
        - id: Command.parameter
          signature: public String parameter(int i)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CommandRegistry.java
      nodes:
      - id: 9mr2w6|HCtNZ6
        signature: import java.util.List;
      - id: CommandRegistry
        signature: "public class CommandRegistry {"
        children:
        - id: CommandRegistry.X4aQ5i|48dF4Q
          signature: private static final List<Command> REGISTRY = List.of( //
        - id: CommandRegistry.detect
          signature: public static Command detect(String line)
```
/answer 1359-8 allow
Text:
 Als Nächstes schaue ich mir `AIAnswer`, `IEditorInterface`, die Session und die Tool-/Control-Request-Erzeugung an.
Control Request:
```yaml
id: 1359-9
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: CONTROL_REQUEST|class AIAnswer|interface IEditorInterface|class Prompt\b|class AISession\b|interface IExecutor
    include:
    - '*.java'
    limit: 40
    intent: survey
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: AnswerCommand|CallEditCommand|ControlRequestCommand|ToolCommand|CallCommand
    include:
    - '*.java'
    exclude:
    - '**/commands/*.java'
    limit: 50
    intent: survey
  reason: Kerntypen und Verwendungsstellen der Commands lokalisieren
```
/answer 1359-9 allow
ReasoningToken: 78
Control Request:
```yaml
id: 1359-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/api/IExecutor.java
        matches:
        - "public interface IExecutor {"
      - path: xy/ai/conscious/model/AIAnswer.java
        matches:
        - "public class AIAnswer {"
      - path: xy/ai/conscious/core/IEditorInterface.java
        matches:
        - "public interface IEditorInterface {"
        - public static final String CONTROL_REQUEST = "Control Request:";
      - path: xy/ai/conscious/connector/claudecode/CCControlClient.java
        matches:
        - "res.append(String.format(\"%s\\n```yaml\\n%s\\n```\\n%s %s allow\", IEditorInterface.CONTROL_REQUEST, toYaml(first), AnswerCommand.CMD_ANSWER, first.path(\"id\").asText()));"
      - path: xy/ai/conscious/connector/harness/Prompt.java
        matches:
        - "public class Prompt {"
      - path: xy/ai/conscious/commands/ControlRequestCommand.java
        matches:
        - return IEditorInterface.CONTROL_REQUEST;
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/connector/mcp/MCPControlClient.java
        matches:
        - import xy.ai.conscious.commands.CallCommand;
        - sb.append(CallCommand.CMD_CALL);
      - path: xy/ai/conscious/connector/mcp/MCPConnector.java
        matches:
        - import xy.ai.conscious.commands.CallCommand;
        - import xy.ai.conscious.commands.CallEditCommand;
        - import xy.ai.conscious.commands.ToolCommand;
        - if (!(command instanceof ExitCommand) && !(command instanceof ToolCommand) && !(command instanceof CallCommand)
        - '&& !(command instanceof CallEditCommand))'
        - "if (req.command instanceof ToolCommand tc) {"
        - "if (req.command instanceof CallCommand) {"
        - "if (req.command instanceof CallEditCommand) {"
      - path: xy/ai/conscious/connector/claudecode/CCControlClient.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - "res.append(String.format(\"%s\\n```yaml\\n%s\\n```\\n%s %s allow\", IEditorInterface.CONTROL_REQUEST, toYaml(first), AnswerCommand.CMD_ANSWER, first.path(\"id\").asText()));"
      - path: xy/ai/conscious/connector/claudecode/CCConnector.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - import xy.ai.conscious.commands.CallCommand;
        - import xy.ai.conscious.commands.CallEditCommand;
        - "if (command instanceof AnswerCommand ac) {"
        - if (ac.action() == AnswerCommand.Action.Allow)
        - else if (ac.action() == AnswerCommand.Action.Exec)
        - "} else if (command instanceof CallEditCommand cec) {"
        - "} else if (command instanceof CallCommand) {"
      - path: xy/ai/conscious/connector/harness/SessionRenderer.java
        matches:
        - import xy.ai.conscious.commands.CallCommand;
        - "return String.format(\"%s\\n%s\\n%s %s\", IEditorInterface.TOOLUSE, YAML.toYamlBlock(node), CallCommand.CMD_CALL,"
      - path: xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - import xy.ai.conscious.commands.CallCommand;
        - import xy.ai.conscious.commands.ToolCommand;
        - if (prompt.arg.command instanceof CallCommand || prompt.arg.command instanceof ToolCommand)
        - if (!(cmd instanceof AnswerCommand) && !(cmd instanceof CallCommand))
      - path: xy/ai/conscious/connector/harness/PromptInputHandler.java
        matches:
        - import xy.ai.conscious.commands.AnswerCommand;
        - import xy.ai.conscious.commands.CallEditCommand;
        - "if (blockCmd instanceof CallEditCommand) {"
        - "if (blockCmd instanceof AnswerCommand ac) {"
        - if (arg.command instanceof AnswerCommand ac)
        - if (cmd instanceof AnswerCommand ac)
      - path: xy/ai/conscious/commands/ToolCommand.java
        matches:
        - "public class ToolCommand extends Command {"
        - "public ToolCommand() {"
        - "public ToolCommand(String toolId) {"
        - return ToolCommand.CMD_TOOL;
        - "return new ToolCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());"
      - path: xy/ai/conscious/commands/CommandRegistry.java
        matches:
        - "new AnswerCommand(), //"
        - "new ControlRequestCommand(), //"
        - "new ToolCommand(), //"
        - "new CallCommand(), //"
        - new CallEditCommand());
      - path: xy/ai/conscious/commands/AnswerCommand.java
        matches:
        - "public class AnswerCommand extends Command {"
        - "public AnswerCommand() {"
        - "public AnswerCommand(String id, String action, String reason) {"
        - return AnswerCommand.CMD_ANSWER;
        - "return new AnswerCommand(id, action, reason);"
      - path: xy/ai/conscious/commands/CallCommand.java
        matches:
        - "public class CallCommand extends Command {"
        - "public CallCommand() {"
      warning: "Limit of 50 matches reached (total 59); narrow pattern, directory or include/exclude filters."
```
/answer 1359-a allow
Control Request:
```yaml
id: 1359-b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/Command.java
    ids:
    - Command
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CommandRegistry.java
    ids:
    - CommandRegistry
  reason: Command-Basis und Registry-Erkennungslogik lesen
```
/answer 1359-b allow
Control Request:
```yaml
id: 1359-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/Command.java
      nodes:
      - id: Command
        type: ClassOrInterfaceDeclaration
        signature: "public abstract class Command {"
        children:
        - id: Command.ProcessorAction
          type: EnumDeclaration
          code: "\tpublic enum ProcessorAction {\n\t\tREMOVE, TRANSFORM, IGNORE_BLOCK, NONE\n\t}\n"
        - id: Command.iKqF62|lJcluI
          type: StatementGroup
          code: "\tprotected final String[] parameters;\n"
        - id: Command.Command
          type: ConstructorDeclaration
          code: "\tprotected Command(String... parameters) {\n\t\tthis.parameters = parameters;\n\t}\n"
        - id: Command.prefix
          type: MethodDeclaration
          code: "\tpublic abstract String prefix();\n"
        - id: Command.matches
          type: MethodDeclaration
          code: "\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().startsWith(prefix());\n\t}\n"
        - id: Command.parse
          type: MethodDeclaration
          code: "\tpublic abstract Command parse(String line);\n"
        - id: Command.processorAction
          type: MethodDeclaration
          code: "\tpublic abstract ProcessorAction processorAction();\n"
        - id: Command.parameter
          type: MethodDeclaration
          code: "\tpublic String parameter(int i) {\n\t\treturn parameters.length > i ? parameters[i] : null;\n\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/CommandRegistry.java
      nodes:
      - id: CommandRegistry
        type: ClassOrInterfaceDeclaration
        signature: "public class CommandRegistry {"
        children:
        - id: CommandRegistry.X4aQ5i|48dF4Q
          type: StatementGroup
          code: "\tprivate static final List<Command> REGISTRY = List.of( //\n\t\t\tnew AnswerCommand(), //\n\t\t\tnew ControlRequestCommand(), //\n\t\t\tnew ResumeCommand(), //\n\t\t\tnew ExitCommand(), //\n\t\t\tnew ToolCommand(), //\n\t\t\tnew CallCommand(), //\n\t\t\tnew CallEditCommand());\n"
        - id: CommandRegistry.detect
          type: MethodDeclaration
          code: "\tpublic static Command detect(String line) {\n\t\tfor (Command prototype : REGISTRY)\n\t\t\tif (prototype.matches(line))\n\t\t\t\treturn prototype.parse(line);\n\t\treturn null;\n\t}\n"
```
/answer 1359-c allow
Control Request:
```yaml
id: 1359-d
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/AIAnswer.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/IEditorInterface.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/api/IExecutor.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/SessionRenderer.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
  reason: "Strukturen von AIAnswer, IEditorInterface, Prompt, IExecutor, SessionRenderer, CCControlClient erfassen"
```
/answer 1359-d allow
Control Request:
```yaml
id: 1359-e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/AIAnswer.java
      nodes:
      - id: zOnqYA|vAsn3G
        signature: import java.util.regex.Matcher;
      - id: AIAnswer
        signature: "public class AIAnswer {"
        children:
        - id: AIAnswer.j6EUkY|1Eboje
          signature: "public static final String RESULT = \"Result Stats: \";"
        - id: AIAnswer.AIAnswer
          signature: public AIAnswer(String id)
        - id: AIAnswer.toString
          signature: public String toString()
        - id: AIAnswer.print
          signature: public String print()
        - id: AIAnswer.fromString
          signature: public static AIAnswer fromString(String line)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/IEditorInterface.java
      nodes:
      - id: rklDWV|UUoZV5
        signature: import java.util.regex.Pattern;
      - id: IEditorInterface
        signature: "public interface IEditorInterface {"
        children:
        - id: IEditorInterface.gCM0q2|sDVECk
          signature: public static final String LINE_COMMENT = "#:";
        - id: IEditorInterface.insertTag
          signature: "public abstract void insertTag(IExecutor exec, IModelRequest req, IProgress mon)"
        - id: IEditorInterface.replaceTag
          signature: "public abstract void replaceTag(AIAnswer ans, IProgress mon)"
        - id: IEditorInterface.getPromptTag
          signature: "public static String getPromptTag(String meta, String id)"
        - id: IEditorInterface.containsPromptTag
          signature: public static boolean containsPromptTag(String line)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
      nodes:
      - id: W9TUuR|WySTfY
        signature: import java.util.List;
      - id: Prompt
        signature: "/**  * Immutable, frozen prompt: semantically separated Inputs / Batch / Config…"
        docstring: "Immutable, frozen prompt: semantically separated Inputs / Batch / Config /"
        children:
        - id: Prompt.uUxVT7|RYVfcb
          signature: public final List<String> inputs;
        - id: Prompt.Prompt
          signature: "public Prompt(List<String> inputs, boolean batch, FrozenConfig config, PromptAr…"
        - id: Prompt.sessionId
          signature: public String sessionId()
          docstring: "Deterministic session id: hash(absoluteFilePath, batch, config.getHash())."
        - id: Prompt.computeSessionId
          signature: private String computeSessionId()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/api/IExecutor.java
      nodes:
      - id: wmvHLq|98tFYi
        signature: import xy.ai.conscious.core.LOG;
      - id: IExecutor
        signature: "public interface IExecutor {"
        children:
        - id: IExecutor.asyncExec
          signature: abstract void asyncExec(Runnable run)
        - id: IExecutor.syncExec
          signature: abstract void syncExec(Runnable run)
        - id: IExecutor.schedule
          signature: "public void schedule(String name, IMonitoredRunnable run)"
        - id: IExecutor.schedule_2
          signature: "public abstract void schedule(String name, IMonitoredRunnable run, long delay)"
        - id: IExecutor.IMonitoredRunnable
          signature: "public interface IMonitoredRunnable {"
          children:
          - id: IExecutor.IMonitoredRunnable.run
            signature: abstract void run(IProgress mon) throws Exception
        - id: IExecutor.IProgress
          signature: "public interface IProgress {"
          children:
          - id: IExecutor.IProgress.subProgress
            signature: IProgress subProgress()
          - id: IExecutor.IProgress.setTask
            signature: "IProgress setTask(String taskName, int expectedWork)"
          - id: IExecutor.IProgress.setTaskName
            signature: void setTaskName(String name)
          - id: IExecutor.IProgress.worked
            signature: void worked()
          - id: IExecutor.IProgress.done
            signature: void done()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/SessionRenderer.java
      nodes:
      - id: 8N6SOO|DPRenr
        signature: import java.util.List;
      - id: SessionRenderer
        signature: "/**  * Renders {"
        docstring: "Renders {@link SessionCallbacks} turns back into the plain-text session"
        children:
        - id: SessionRenderer.97qJP9|Yi2Gjf
          signature: private static final ObjectMapper MAPPER = new ObjectMapper();
        - id: SessionRenderer.reasoningPlaceholder
          signature: public static String reasoningPlaceholder(int index)
        - id: SessionRenderer.text
          signature: public static String text(String text)
        - id: SessionRenderer.reasoning
          signature: "public static String reasoning(List<String> texts, String meta)"
        - id: SessionRenderer.toolCall
          signature: public static String toolCall(ToolCall call)
        - id: SessionRenderer.toolResult
          signature: public static String toolResult(ToolResult result)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
      nodes:
      - id: K2G7xy|HsdHqq
        signature: import java.io.IOException;
      - id: CCControlClient
        signature: /**  * Minimal HTTP client for the MCPC human-in-the-loop tool-control endpoint…
        docstring: Minimal HTTP client for the MCPC human-in-the-loop tool-control endpoint
        children:
        - id: CCControlClient.mBFSrf|Pegcal
          signature: private static final String CONTROL_URL = "http://localhost:9093/control/tool";
        - id: CCControlClient.CCControlClient
          signature: public CCControlClient(IMCPCLauncher mcpc)
        - id: CCControlClient.checkControlEndpoint
          signature: "public void checkControlEndpoint(CCResponse resp, String sessionId)"
        - id: CCControlClient.toYaml
          signature: public String toYaml(JsonNode node)
        - id: CCControlClient.fromYaml
          signature: public JsonNode fromYaml(String text) throws JsonProcessingException
        - id: CCControlClient.isMCPCAvailable
          signature: public boolean isMCPCAvailable()
        - id: CCControlClient.ping
          signature: private boolean ping()
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
/answer 1359-e allow
Control Request:
```yaml
id: 1359-f
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness
    pattern: AnswerCommand|CallCommand|ToolCommand|CallEditCommand
    include:
    - PromptHandler.java
    - PromptInputHandler.java
    intent: editing
    limit: 30
  reason: Kontext der Command-Verwendung in PromptHandler/PromptInputHandler mit Zeilennummern
```
/answer 1359-f allow
Control Request:
```yaml
id: 1359-10
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness
      matches:
      - path: PromptHandler.java
        nodes:
        - id: NZKTrV|g5OLnZ
          matches:
          - import xy.ai.conscious.commands.AnswerCommand;
          - import xy.ai.conscious.commands.CallCommand;
          - import xy.ai.conscious.commands.ToolCommand;
        - id: PromptHandler.preprocessPrompt
          matches:
          - if (prompt.arg.command instanceof CallCommand || prompt.arg.command instanceof ToolCommand)
        - id: PromptHandler.checkGitSupport
          matches:
          - if (!(cmd instanceof AnswerCommand) && !(cmd instanceof CallCommand))
      - path: PromptInputHandler.java
        nodes:
        - id: U3FBWT|pBPOgv
          matches:
          - import xy.ai.conscious.commands.AnswerCommand;
          - import xy.ai.conscious.commands.CallEditCommand;
        - id: PromptInputHandler.detectSelection
          matches:
          - "if (blockCmd instanceof CallEditCommand) {"
          - "if (blockCmd instanceof AnswerCommand ac) {"
          - if (arg.command instanceof AnswerCommand ac)
        - id: PromptInputHandler.detectedCmd
          matches:
          - if (cmd instanceof AnswerCommand ac)
```
/answer 1359-10 allow
Control Request:
```yaml
id: 1359-11
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/AIAnswer.java
    ids:
    - AIAnswer
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/IEditorInterface.java
    ids:
    - IEditorInterface
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
    ids:
    - Prompt
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/SessionRenderer.java
    ids:
    - SessionRenderer.toolCall
    - SessionRenderer.toolResult
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
    ids:
    - CCControlClient.checkControlEndpoint
    - CCControlClient.approve
    - CCControlClient.approve_2
    - CCControlClient.exec
    - CCControlClient.deny
    - CCControlClient.approvalNode
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
    ids:
    - PromptHandler.prepareRequest
    - PromptHandler.preprocessPrompt
    - PromptHandler.checkGitSupport
  reason: "Datenmodell der Answer, Tag-Schnittstelle, Prompt und die Control-Request-/Tool-Call-Formate lesen"
```
/answer 1359-11 allow
ReasoningToken: 95
Control Request:
```yaml
id: 1359-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/AIAnswer.java
      nodes:
      - id: AIAnswer
        type: ClassOrInterfaceDeclaration
        signature: "public class AIAnswer {"
        children:
        - id: AIAnswer.j6EUkY|1Eboje
          type: StatementGroup
          code: |2
              public static final String RESULT = "Result Stats: ";
              private static final Pattern ID_PATTERN = Pattern.compile("id=([^,]*), ");
              public final String id;
              public String sessionId;
              public final TokenStats stats = new TokenStats();
              public final TimeStats time = new TimeStats();
              public String answer = "";
              public String instructions = "";
              public boolean showStats = true;
              /**
               * Prompt that produced this answer; null once no longer available
               */
              public Prompt prompt;
        - id: AIAnswer.AIAnswer
          type: ConstructorDeclaration
          code: |2
              public AIAnswer(String id) {
                this.id = id;
                this.sessionId = id;
              }
        - id: AIAnswer.toString
          type: MethodDeclaration
          code: |2
              @Override
              public String toString() {
                return String.format("AIAnswer [stats=%s]", stats);
              }
        - id: AIAnswer.print
          type: MethodDeclaration
          code: |2
              public String print() {
                return String.format("%sid=%s, %s, %s", RESULT, sessionId, stats.print(), time.print());
              }
        - id: AIAnswer.fromString
          type: MethodDeclaration
          code: |2
              public static AIAnswer fromString(String line) {
                if (line == null)
                  return null;
                Matcher idMatcher = ID_PATTERN.matcher(line);
                if (!idMatcher.find())
                  return null;
                String rest = line.substring(idMatcher.end());
                TokenStats stats = TokenStats.fromString(rest);
                if (stats == null)
                  return null;
                AIAnswer ans = new AIAnswer(idMatcher.group(1));
                ans.stats.add(stats);
                TimeStats t = TimeStats.fromString(rest);
                if (t != null)
                  ans.time.add(t);
                return ans;
              }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/IEditorInterface.java
      nodes:
      - id: IEditorInterface
        type: ClassOrInterfaceDeclaration
        signature: "public interface IEditorInterface {"
        children:
        - id: IEditorInterface.gCM0q2|sDVECk
          type: StatementGroup
          code: "\tpublic static final String LINE_COMMENT = \"#:\";\n\tpublic static final String USER = \"User:\";\n\tpublic static final String AGENT = \"Agent:\";\n\n\tpublic static final String TEXT = \"Text:\";\n\tpublic static final String THINKING = \"Thinking:\";\n\tpublic static final String THINKING_META = \"Thinking Meta:\";\n\tpublic static final String TOOLUSE = \"Tool:\";\n\tpublic static final String TOOLRESULT = \"ToolResult:\";\n\tpublic static final String CONTROL_REQUEST = \"Control Request:\";\n\n\tpublic static final String AIREQ_PREFIX = \"xy.ai.req\";\n\tpublic static final Pattern PROMPT_TAG_PATTERN = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\", Pattern.CASE_INSENSITIVE);\n"
        - id: IEditorInterface.insertTag
          type: MethodDeclaration
          code: "\tpublic void insertTag(IExecutor exec, IModelRequest req, IProgress mon);\n"
        - id: IEditorInterface.replaceTag
          type: MethodDeclaration
          code: "\tpublic void replaceTag(AIAnswer ans, IProgress mon);\n"
        - id: IEditorInterface.getPromptTag
          type: MethodDeclaration
          code: "\tpublic static String getPromptTag(String meta, String id) {\n\t\treturn String.format(\"[%s:%s:%s]\", IEditorInterface.AIREQ_PREFIX, meta, id);\n\t}\n"
        - id: IEditorInterface.containsPromptTag
          type: MethodDeclaration
          code: "\tpublic static boolean containsPromptTag(String line) {\n\t\treturn line != null && IEditorInterface.PROMPT_TAG_PATTERN.matcher(line).find();\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
      nodes:
      - id: Prompt
        type: ClassOrInterfaceDeclaration
        signature: "/**  * Immutable, frozen prompt: semantically separated Inputs / Batch / Config…"
        docstring: "Immutable, frozen prompt: semantically separated Inputs / Batch / Config /"
        children:
        - id: Prompt.uUxVT7|RYVfcb
          type: StatementGroup
          code: "\tpublic final List<String> inputs;\n\tpublic final boolean batch;\n\tpublic final FrozenConfig config;\n\tpublic final PromptArguments arg;\n\n\tprivate String sessionId;\n"
        - id: Prompt.Prompt
          type: ConstructorDeclaration
          code: "\tpublic Prompt(List<String> inputs, boolean batch, FrozenConfig config, PromptArguments arg) {\n\t\tif (config == null)\n\t\t\tthrow new IllegalArgumentException(\"config must not be null\");\n\t\tif (arg.absoluteFilePath == null || arg.absoluteFilePath.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"absoluteFilePath must not be blank\");\n\t\tif (arg.project == null)\n\t\t\tthrow new IllegalArgumentException(\"projectPath must not be null\");\n\t\tthis.inputs = inputs != null ? List.copyOf(inputs) : List.of();\n\t\tthis.batch = batch;\n\t\tthis.config = config;\n\t\tthis.arg = arg;\n\t}\n"
        - id: Prompt.sessionId
          type: MethodDeclaration
          code: "\tpublic String sessionId() {\n\t\tif (sessionId == null)\n\t\t\tsessionId = computeSessionId();\n\t\treturn sessionId;\n\t}\n"
        - id: Prompt.computeSessionId
          type: MethodDeclaration
          code: "\tprivate String computeSessionId() {\n\t\tString input = arg.absoluteFilePath + \"|\" + batch + \"|\" + config.getHash();\n\t\treturn Hash.hash(input);\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/SessionRenderer.java
      nodes:
      - id: SessionRenderer.toolCall
        type: MethodDeclaration
        code: "\tpublic static String toolCall(ToolCall call) {\n\t\tObjectNode node = MAPPER.createObjectNode();\n\t\tnode.put(\"id\", call.id == null ? \"\" : call.id);\n\t\tnode.put(\"tool\", call.name == null ? \"\" : call.name);\n\t\tnode.set(\"arguments\",\n\t\t\t\tcall.arguments != null && call.arguments.isObject() ? call.arguments : MAPPER.createObjectNode());\n\t\treturn String.format(\"%s\\n%s\\n%s %s\", IEditorInterface.TOOLUSE, YAML.toYamlBlock(node), CallCommand.CMD_CALL,\n\t\t\t\tcall.id);\n\t}\n"
      - id: SessionRenderer.toolResult
        type: MethodDeclaration
        code: "\tpublic static String toolResult(ToolResult result) {\n\t\tObjectNode node = MAPPER.createObjectNode();\n\t\tnode.put(\"id\", result.id == null ? \"\" : result.id);\n\t\tnode.put(\"result\", result.content == null ? \"\" : result.content);\n\t\treturn IEditorInterface.TOOLRESULT + \"\\n\" + YAML.toYamlBlock(node);\n\t}\n"
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler.prepareRequest
        type: MethodDeclaration
        code: "\tprivate IModelRequest prepareRequest(IExecutor exec, boolean batchFix, IProgress mon) {\n\t\tIProgress sub = mon.setTask(\"Preparing Call\", 1);\n\t\tsub.setTaskName(\"Preparing Call\");\n\t\tPrompt prompt = input.buildPrompt(exec, batchFix);\n\t\tpreprocessPrompt(prompt);\n\t\tIModelRequest req = connector.createRequest(prompt, sub);\n\t\treq.setPrompt(prompt);\n\t\tsub.worked();\n\t\treturn req;\n\t}\n"
      - id: PromptHandler.preprocessPrompt
        type: MethodDeclaration
        code: "\tprivate void preprocessPrompt(Prompt prompt) {\n\t\tfor (String key : prompt.config.keys.split(\",\"))\n\t\t\tif (prompt.config.model.cap.acceptsKey(key))\n\t\t\t\tprompt.config.keys = key;\n\t\tif (prompt.arg.command instanceof CallCommand || prompt.arg.command instanceof ToolCommand)\n\t\t\tprompt.config.model = Model.MCP_TOOLS;\n\t}\n"
      - id: PromptHandler.checkGitSupport
        type: MethodDeclaration
        code: "\tpublic void checkGitSupport(IExecutor exec, IModelRequest req) {\n\t\tif (gitSupport == null)\n\t\t\treturn;\n\n\t\tPrompt prompt = req.getPrompt();\n\t\tif (prompt == null || prompt.arg == null)\n\t\t\treturn;\n\n\t\tCommand cmd = prompt.arg.command;\n\t\tif (!(cmd instanceof AnswerCommand) && !(cmd instanceof CallCommand))\n\t\t\treturn;\n\n\t\tgitSupport.triggerSnapshotIfNeeded(exec, req);\n\t}\n"
```
/answer 1359-12 allow
Control Request:
```yaml
id: 1359-13
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/mcp
    pattern: instanceof (AnswerCommand|CallCommand|CallEditCommand|ToolCommand)
    include:
    - CCConnector.java
    - MCPConnector.java
    intent: editing
    limit: 20
  reason: "Stellen in CCConnector/MCPConnector finden, die Commands in Folgeaktionen umsetzen"
```
/answer 1359-13 allow
Control Request:
```yaml
id: 1359-14
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/mcp
      matches:
      - path: CCConnector.java
        nodes:
        - id: CCConnector.executeRequest
          matches:
          - "if (command instanceof AnswerCommand ac) {"
          - "} else if (command instanceof CallEditCommand cec) {"
          - "} else if (command instanceof CallCommand) {"
      - path: MCPConnector.java
        nodes:
        - id: MCPConnector.createRequest
          matches:
          - if (!(command instanceof ExitCommand) && !(command instanceof ToolCommand) && !(command instanceof CallCommand)
          - '&& !(command instanceof CallEditCommand))'
        - id: MCPConnector.executeRequest
          matches:
          - "if (req.command instanceof ToolCommand tc) {"
          - "if (req.command instanceof CallCommand) {"
          - "if (req.command instanceof CallEditCommand) {"
```
/answer 1359-14 allow
Control Request:
```yaml
id: 1359-15
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
    ids:
    - CCConnector.executeRequest
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/mcp/MCPConnector.java
    ids:
    - MCPConnector.createRequest
    - MCPConnector.executeRequest
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
    ids:
    - PromptInputHandler.detectSelection
    - PromptInputHandler.detectedCmd
  reason: Folgeaktionen (Allow/Deny/Exec/Call) in den Connectoren und Prompt-Erzeugung aus Commands nachvollziehen
```
/answer 1359-15 allow
ReasoningToken: 105
Control Request:
```yaml
id: 1359-16
result:
  structuredContent:
    results:
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
            private boolean detectedCmd(String line, String[] lines, int lineIndex, PromptArguments arg) {
              Command cmd = CommandRegistry.detect(line);
              if (cmd == null)
                return false;
              arg.command = cmd;
              arg.yaml = captureYamlBlock(lines, lineIndex);
              if (cmd instanceof AnswerCommand ac)
                ac.setYaml(arg.yaml);
              return true;
            }
```
/answer 1359-16 allow
Control Request:
```yaml
id: 1359-17
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: class (PromptArguments|TokenStats|TimeStats|ToolCall|ToolResult|SessionCallbacks|FrozenConfig|AdaptingConnector)\b|interface (SessionCallbacks|IModelRequest|ISession\w*)
    include:
    - '*.java'
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: idle|Idle|cacheRead|cache_read|turnCount|numTurns|num_turns
    include:
    - '*.java'
    limit: 40
  reason: "Typen für Session-Status (Turns, Cache-Read, Idle-Time) und Prompt-Argumente lokalisieren"
```
/answer 1359-17 allow
Control Request:
```yaml
id: 1359-18
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/model/TimeStats.java
        matches:
        - "public class TimeStats {"
      - path: xy/ai/conscious/model/IModelRequest.java
        matches:
        - "public interface IModelRequest {"
      - path: xy/ai/conscious/model/TokenStats.java
        matches:
        - "public class TokenStats {"
      - path: xy/ai/conscious/connector/AdaptingConnector.java
        matches:
        - "public class AdaptingConnector implements IAIConnector<IModelRequest, IModelResponse>, IAIBatchConnector {"
      - path: xy/ai/conscious/connector/harness/SessionCallbacks.java
        matches:
        - "public interface SessionCallbacks<M> {"
      - path: xy/ai/conscious/connector/harness/ISessionIncludeProcessor.java
        matches:
        - "public interface ISessionIncludeProcessor<M> {"
      - path: xy/ai/conscious/connector/harness/ToolResult.java
        matches:
        - "public final class ToolResult {"
      - path: xy/ai/conscious/connector/harness/FrozenConfig.java
        matches:
        - "public class FrozenConfig {"
      - path: xy/ai/conscious/connector/harness/ToolCall.java
        matches:
        - "public class ToolCall {"
      - path: xy/ai/conscious/connector/harness/PromptArguments.java
        matches:
        - "public class PromptArguments {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/model/TokenStats.java
        matches:
        - public long cacheRead;
        - "\"TokenStats [inputToken=%s, outputToken=%s, reasoningToken=%s, totalInToken=%s, cacheRead=%s, cacheCreate=%s]\","
        - "inputToken, outputToken, reasoningToken, totalToken, cacheRead, cacheCreate);"
        - "outputToken, reasoningToken, cacheRead, cacheCreate);"
        - stats.cacheRead = Long.parseLong(m.group(5));
        - cacheRead += stats.cacheRead;
      - path: xy/ai/conscious/connector/claudecode/TimedLineReader.java
        matches:
        - '* waiting for the idle timeout.'
      - path: xy/ai/conscious/connector/claudecode/ProtocolParser.java
        matches:
        - stats.cacheRead += usage.path("cacheReadInputTokens").asLong(0);
      - path: xy/ai/conscious/connector/claudecode/CCConnector.java
        matches:
        - boolean idle = lastDataAt == before;
        - "if (idle && !resp.isReady()) {"
        - answer.stats.cacheRead = resp.stats.cacheRead;
      - path: xy/ai/conscious/connector/claude/ClaudeConnector.java
        matches:
        - res.stats.cacheRead = tokenValue(usage.getCacheReadInputTokens());
      - path: xy/ai/conscious/connector/deepseek/DeepSeekConnector.java
        matches:
        - res.stats.cacheRead = Optional.ofNullable(ind.getCachedTokens()).orElse(0L);
        - res.stats.inputToken -= res.stats.cacheRead;
```
/answer 1359-18 allow
Control Request:
```yaml
id: 1359-19
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/TokenStats.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/TimeStats.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/AdaptingConnector.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/IModelRequest.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/ToolCall.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/ToolResult.java
  reason: "Datentypen, die Regeln als Input erhalten sollen, kennenlernen"
```
/answer 1359-19 allow
ReasoningToken: 129
Control Request:
```yaml
id: 1359-1a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
      nodes:
      - id: TLGErr|Ew8ehZ
        signature: import java.lang.ref.WeakReference;
      - id: PromptArguments
        signature: "public class PromptArguments {"
        children:
        - id: PromptArguments.gK6na9|d9fNBh
          signature: public List<String> inputs = new ArrayList<>();
        - id: PromptArguments.setEditor
          signature: public void setEditor(Object editor)
        - id: PromptArguments.getEditor
          signature: public Object getEditor()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/TokenStats.java
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/TimeStats.java
      nodes:
      - id: gON95s|vg6cm6
        signature: import java.util.function.Supplier;
      - id: TimeStats
        signature: "public class TimeStats {"
        children:
        - id: TimeStats.eK297o|j8VTKx
          signature: public long startAt;
        - id: TimeStats.start
          signature: public void start()
        - id: TimeStats.stop
          signature: public void stop()
          docstring: Ends the measurement; control time is excluded from the duration.
        - id: TimeStats.control
          signature: public T control(Supplier<T> call)
        - id: TimeStats.io
          signature: public T io(Supplier<T> call)
        - id: TimeStats.toString
          signature: public String toString()
        - id: TimeStats.print
          signature: public String print()
        - id: TimeStats.fromString
          signature: public static TimeStats fromString(String s)
        - id: TimeStats.add
          signature: public void add(TimeStats t)
          docstring: Accumulates a finished prompt into this (session) total.
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/AdaptingConnector.java
      nodes:
      - id: aXpqw4|P09Koy
        signature: import java.util.Collection;
      - id: mEYlBR|yhnNCb
        signature: import xy.ai.conscious.connector.google.GeminiConnector;
      - id: Z8xFy6|97Z7LM
        signature: import xy.ai.conscious.model.Model;
      - id: AdaptingConnector
        signature: "public class AdaptingConnector implements IAIConnector<IModelRequest, IModelRes…"
        children:
        - id: AdaptingConnector.VG9tFx|iQKf7Q
          signature: private ConfigManager cfg;
        - id: AdaptingConnector.AdaptingConnector
          signature: "public AdaptingConnector(ConfigManager cfg, CCSessionManager sessionManager, MC…"
        - id: AdaptingConnector.getSupportedKeyPattern
          signature: public KeyPattern getSupportedKeyPattern()
        - id: AdaptingConnector.getConnector
          signature: "private IAIConnector<? extends IModelRequest, ? extends IModelResponse> getConn…"
        - id: AdaptingConnector.getBatchConnector
          signature: public IAIBatchConnector getBatchConnector(Model model)
        - id: AdaptingConnector.getConnector_2
          signature: public IAIConnector getConnector(IModelRequest request)
        - id: AdaptingConnector.getConnector_3
          signature: private IAIConnector getConnector(IModelResponse response)
        - id: AdaptingConnector.getBatchConnector_2
          signature: private IAIBatchConnector getBatchConnector(IModelRequest request)
        - id: AdaptingConnector.getBatchConnector_3
          signature: private IAIBatchConnector getBatchConnector(IAIBatch entry)
        - id: AdaptingConnector.createRequest
          signature: "public IModelRequest createRequest(Prompt prompt, IProgress mon)"
        - id: AdaptingConnector.updateBatches
          signature: public List<IAIBatch> updateBatches(IProgress mon)
        - id: AdaptingConnector.submitBatch
          signature: "public IAIBatch submitBatch(NewBatch entry, IProgress mon)"
        - id: AdaptingConnector.executeRequest
          signature: "public IModelResponse executeRequest(IModelRequest request, IProgress mon)"
        - id: AdaptingConnector.requestsToJson
          signature: public String requestsToJson(Collection<IModelRequest> reqs)
        - id: AdaptingConnector.convertResponse
          signature: "public AIAnswer convertResponse(IModelResponse response, IProgress mon)"
        - id: AdaptingConnector.cancelBatch
          signature: "public IAIBatch cancelBatch(IAIBatch entry, IProgress mon)"
        - id: AdaptingConnector.loadBatch
          signature: "public void loadBatch(IAIBatch entry, IProgress mon)"
        - id: AdaptingConnector.convertAnswers
          signature: "public void convertAnswers(IAIBatch obj, IProgress mon)"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/IModelRequest.java
      nodes:
      - id: 5QfUGi|LkwiB3
        signature: import xy.ai.conscious.connector.harness.Prompt;
      - id: IModelRequest
        signature: "public interface IModelRequest {"
        children:
        - id: IModelRequest.getID
          signature: public abstract String getID()
        - id: IModelRequest.getPrompt
          signature: public abstract Prompt getPrompt()
        - id: IModelRequest.setPrompt
          signature: public abstract void setPrompt(Prompt prompt)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/ToolCall.java
      nodes:
      - id: D3sKkB|eJTl0D
        signature: import com.fasterxml.jackson.databind.JsonNode;
      - id: ToolCall
        signature: "public class ToolCall {"
        children:
        - id: ToolCall.WrmHAW|pLXvFi
          signature: public final String id;
        - id: ToolCall.ToolCall
          signature: "public ToolCall(String id, String name, JsonNode arguments)"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/ToolResult.java
      nodes:
      - id: ToolResult
        signature: "/**  * The (textual) result of a previously issued {"
        docstring: "The (textual) result of a previously issued {@link ToolCall}."
        children:
        - id: ToolResult.ipv24q|vttbiB
          signature: public final String id;
        - id: ToolResult.ToolResult
          signature: "public ToolResult(String id, String content)"
```
/answer 1359-1a allow
Control Request:
```yaml
id: 1359-1b
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
    pattern: class CC\w*Session\w*|getSelectedSessionUuid|setInPrompt|turn|Turn|lastActivity|lastDataAt
    include:
    - '*.java'
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: "implements IEditorInterface|IEditorInterface\\s*\\{"
    include:
    - '*.java'
    limit: 10
  reason: "Session-Zustand (Turns, Aktivität) und Implementierung der Editor-Schnittstelle finden"
```
/answer 1359-1b allow
Control Request:
```yaml
id: 1359-1c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
      matches:
      - path: TimedLineReader.java
        matches:
        - return true;
        - return delegate != null; // EOF is reported once as null
        - return delegate != null && delegate.ready();
        - return result;
        - "if (eof) { // return remaining"
        - // once return null
        - return null;
      - path: HookParser.java
        matches:
        - "* Applies all post-processing steps to {@code text} and returns the cleaned"
        - "* result. If {@code text} is {@code null} or blank it is returned as-is."
        - return text;
        - "* @return the hook output, or the original {@code text} if the pattern is not"
        - return text; // no hook output found — keep original
        - return hookOutput.toString();
      - path: CCSessionParameters.java
        matches:
        - "public class CCSessionParameters extends SessionParameters {"
        - return fc.keys.contains(cliProfile) //
        - return hash;
        - return Hash.hash(input);
        - return cmd;
        - return title;
      - path: CCSession.java
        matches:
        - "public class CCSession {"
        - return -1;
        - return 0l;
        - "return Math.max(0, 5 - elapsed);"
        - "return Math.max(0, 60 - elapsed);"
        - return SessionState.Prompting;
        - return SessionState.Expired;
        - return SessionState.Open;
        - return SessionState.Created;
        - "* @return the number of whole seconds until this session's TTL runs out, or"
        - return 0L;
        - "return Math.max(0, totalSeconds - elapsed);"
        - "* @return {@code true} unless the session is {@link SessionState#Expired}. Used"
        - return getState() != SessionState.Expired;
        - return;
      warning: "Limit of 40 matches reached (total 171); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/EditorInterface.java
        matches:
        - "public class EditorInterface implements IEditorInterface{"
      - path: connector/src/xy/ai/conscious/core/IEditorInterface.java
        matches:
        - "public interface IEditorInterface {"
```
/answer 1359-1c allow
Control Request:
```yaml
id: 1359-1d
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSession.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  reason: "Session-Status-API (State, TTL, Idle) und Tag-Austausch in der EditorInterface-Implementierung verstehen"
```
/answer 1359-1d allow
ReasoningToken: 111
Control Request:
```yaml
id: 1359-1e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSession.java
      nodes:
      - id: hLlb0k|JJgtFv
        signature: import java.io.File;
      - id: CCSession
        signature: "public class CCSession {"
        children:
        - id: CCSession.0wAS29|92BusB
          signature: private final String uuid;
        - id: CCSession.CCSession
          signature: "public CCSession(CCSessionManager manager, CCSessionParameters parameters)"
        - id: CCSession.CCSession_2
          signature: "public CCSession(String sessionUuid, CCSessionManager manager, CCSessionParamet…"
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
        - id: CCSession.hasOutput
          signature: public boolean hasOutput()
          docstring: "Non-blocking probe: is there anything to read on STDOUT?"
        - id: CCSession.hasError
          signature: public boolean hasError()
          docstring: "Non-blocking probe: is there anything to read on STDERR?"
        - id: CCSession.hasData
          signature: "private boolean hasData(TimedLineReader reader, String channel)"
        - id: CCSession.requireReader
          signature: "private void requireReader(TimedLineReader reader, String channel)"
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
          signature: public CCSessionParameters getParameters()
        - id: CCSession.notifyChanged
          signature: private void notifyChanged()
        - id: CCSession.getID
          signature: public String getID()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
      nodes:
      - id: k0tyZU|ioWIOl
        signature: import java.util.ArrayList;
      - id: CCSessionManager
        signature: "public class CCSessionManager {"
        children:
        - id: CCSessionManager.BqTTt6|umAa9G
          signature: public final static String CREATE_NEW_MARKER = "CREATE_NEW_MARKER";
        - id: CCSessionManager.requestSession
          signature: "public synchronized CCSession requestSession(String selectedUuid, CCSessionPara…"
        - id: CCSessionManager.getSession
          signature: "public synchronized CCSession getSession(String selectedUuid, CCSessionParamete…"
        - id: CCSessionManager.addSession
          signature: private CCSession addSession(CCSession session)
        - id: CCSessionManager.importSession
          signature: "public synchronized CCSession importSession(String uuid, CCSessionParameters pa…"
        - id: CCSessionManager.terminateSessions
          signature: public synchronized void terminateSessions(List<String> toTerminate)
        - id: CCSessionManager.terminateSession
          signature: public synchronized void terminateSession(String uuidOrHash)
        - id: CCSessionManager.getSessions
          signature: public synchronized List<CCSession> getSessions()
        - id: CCSessionManager.addChangeListener
          signature: public void addChangeListener(Consumer<List<CCSession>> listener)
        - id: CCSessionManager.removeChangeListener
          signature: public void removeChangeListener(Consumer<List<CCSession>> listener)
        - id: CCSessionManager.getSelectedSessionUuid
          signature: public String getSelectedSessionUuid()
        - id: CCSessionManager.setSelectedSessionUuid
          signature: public void setSelectedSessionUuid(String uuid)
        - id: CCSessionManager.onSessionChanged
          signature: void onSessionChanged(CCSession session)
        - id: CCSessionManager.cleanupInvalidTerminated
          signature: private void cleanupInvalidTerminated()
        - id: CCSessionManager.findByUuid
          signature: private CCSession findByUuid(String uuid)
        - id: CCSessionManager.findByHash
          signature: private CCSession findByHash(String hash)
        - id: CCSessionManager.fireChanged
          signature: private void fireChanged()
        - id: CCSessionManager.fireChanged_2
          signature: private void fireChanged(List<Consumer<List<CCSession>>> changeListeners)
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: 1beUBs|3Q9CR6
        signature: import java.io.ByteArrayInputStream;
      - id: bHiiRy|rWc6m1
        signature: import xy.ai.conscious.core.ConfigManager;
      - id: EditorInterface
        signature: "public class EditorInterface implements IEditorInterface {"
        children:
        - id: EditorInterface.fZITGq|vDMTwT
          signature: public static final String USER = IEditorInterface.USER;
        - id: EditorInterface.EditorInterface
          signature: "public EditorInterface(ConfigManager cfg, ActiveEditorListener editorListener, …"
        - id: EditorInterface.register
          signature: public void register(BundleContext context) throws Exception
        - id: EditorInterface.dispose
          signature: public void dispose(BundleContext context)
        - id: EditorInterface.insertTag
          signature: "public void insertTag(IExecutor exec, IModelRequest req, IProgress mon)"
        - id: EditorInterface.generateTag
          signature: private String generateTag(IModelRequest req)
        - id: EditorInterface.replaceTag
          signature: "public void replaceTag(AIAnswer ans, IProgress mon)"
```
/answer 1359-1e allow
Control Request:
```yaml
id: 1359-1f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
    ids:
    - PromptArguments
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/TokenStats.java
    ids:
    - TokenStats
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/TimeStats.java
    ids:
    - TimeStats
  reason: "Felder von PromptArguments, TokenStats, TimeStats im Detail lesen"
```
/answer 1359-1f allow
Control Request:
```yaml
id: 1359-20
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
      nodes:
      - id: PromptArguments
        type: ClassOrInterfaceDeclaration
        signature: "public class PromptArguments {"
        children:
        - id: PromptArguments.gK6na9|d9fNBh
          type: StatementGroup
          code: "\tpublic List<String> inputs = new ArrayList<>();\n\tpublic String absoluteFilePath;\n\tpublic Path project;\n\tpublic Command command;\n\tpublic String yaml;\n\t// Editor active when the prompt was built; used as a hint for tag replacement.\n\tprivate WeakReference<Object> editor;\n\tpublic boolean processorEnabled;\n"
        - id: PromptArguments.setEditor
          type: MethodDeclaration
          code: "\tpublic void setEditor(Object editor) {\n\t\tthis.editor = editor != null ? new WeakReference<>(editor) : null;\n\t}\n"
        - id: PromptArguments.getEditor
          type: MethodDeclaration
          code: "\tpublic Object getEditor() {\n\t\treturn editor != null ? editor.get() : null;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/TokenStats.java
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/TimeStats.java
      nodes:
      - id: TimeStats
        type: ClassOrInterfaceDeclaration
        signature: "public class TimeStats {"
        children:
        - id: TimeStats.eK297o|j8VTKx
          type: StatementGroup
          code: "\tpublic long startAt;\n\tpublic long endAt;\n\t/** Runtime of the last prompt in ms, excluding control client time */\n\tpublic long durationMs;\n\t/** Cumulative durationMs of all prompts in the session */\n\tpublic long totalMs;\n\tpublic long ioMs;\n\tpublic long controlMs;\n\tpublic long sent;\n\tpublic long received;\n\n\tprivate static final Pattern PATTERN = Pattern.compile(\n\t\t\t\"start: (\\\\d+), end: (\\\\d+), time: (\\\\d+), total: (\\\\d+), io: (\\\\d+), control: (\\\\d+), sent: (\\\\d+), recv: (\\\\d+)\");\n"
        - id: TimeStats.start
          type: MethodDeclaration
          code: "\tpublic void start() {\n\t\tstartAt = System.currentTimeMillis();\n\t}\n"
        - id: TimeStats.stop
          type: MethodDeclaration
          code: "\tpublic void stop() {\n\t\tendAt = System.currentTimeMillis();\n\t\tdurationMs = Math.max(0, endAt - startAt - controlMs);\n\t}\n"
        - id: TimeStats.control
          type: MethodDeclaration
          code: "\tpublic <T> T control(Supplier<T> call) {\n\t\tlong t = System.nanoTime();\n\t\ttry {\n\t\t\treturn call.get();\n\t\t} finally {\n\t\t\tcontrolMs += (System.nanoTime() - t) / 1_000_000;\n\t\t}\n\t}\n"
        - id: TimeStats.io
          type: MethodDeclaration
          code: "\tpublic <T> T io(Supplier<T> call) {\n\t\tlong t = System.nanoTime();\n\t\ttry {\n\t\t\treturn call.get();\n\t\t} finally {\n\t\t\tioMs += (System.nanoTime() - t) / 1_000_000;\n\t\t}\n\t}\n"
        - id: TimeStats.toString
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String toString() {\n\t\treturn String.format(\n\t\t\t\t\"TimeStats [startAt=%s, endAt=%s, durationMs=%s, totalMs=%s, ioMs=%s, controlMs=%s, sent=%s, received=%s]\",\n\t\t\t\tstartAt, endAt, durationMs, totalMs, ioMs, controlMs, sent, received);\n\t}\n"
        - id: TimeStats.print
          type: MethodDeclaration
          code: "\tpublic String print() {\n\t\treturn String.format(\"start: %s, end: %s, time: %s, total: %s, io: %s, control: %s, sent: %s, recv: %s\", startAt,\n\t\t\t\tendAt, durationMs, totalMs, ioMs, controlMs, sent, received);\n\t}\n"
        - id: TimeStats.fromString
          type: MethodDeclaration
          code: "\tpublic static TimeStats fromString(String s) {\n\t\tif (s == null)\n\t\t\treturn null;\n\t\tMatcher m = PATTERN.matcher(s);\n\t\tif (!m.find())\n\t\t\treturn null;\n\t\tTimeStats t = new TimeStats();\n\t\tt.startAt = Long.parseLong(m.group(1));\n\t\tt.endAt = Long.parseLong(m.group(2));\n\t\tt.durationMs = Long.parseLong(m.group(3));\n\t\tt.totalMs = Long.parseLong(m.group(4));\n\t\tt.ioMs = Long.parseLong(m.group(5));\n\t\tt.controlMs = Long.parseLong(m.group(6));\n\t\tt.sent = Long.parseLong(m.group(7));\n\t\tt.received = Long.parseLong(m.group(8));\n\t\treturn t;\n\t}\n"
        - id: TimeStats.add
          type: MethodDeclaration
          code: "\tpublic void add(TimeStats t) {\n\t\tstartAt = t.startAt;\n\t\tendAt = t.endAt;\n\t\tdurationMs = t.durationMs;\n\t\ttotalMs += t.durationMs;\n\t\tioMs += t.ioMs;\n\t\tcontrolMs += t.controlMs;\n\t\tsent += t.sent;\n\t\treceived += t.received;\n\t}\n"
```
/answer 1359-20 allow
Thinking:
Der Package `autoapprove` existiert noch nicht; ich habe Integrationspunkt, Commands, Control-Request-Format und Stats-Modell gelesen. Als Nächstes prüfe ich, wie ein Folge-Prompt in `PromptHandler.execute` ausgelöst wird.


Control Request:
```yaml
id: 1359-21
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
  reason: "buildPrompt und die Prompt-Konstruktion aus Editor-Zustand finden, um programmatische Folge-Prompts zu bewerten"
```
/answer 1359-21 allow
ReasoningToken: 462
Control Request:
```yaml
id: 1359-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
      nodes:
      - id: U3FBWT|pBPOgv
        signature: import java.util.regex.Matcher;
      - id: PromptInputHandler
        signature: "/**  * Input handling extracted from {"
        docstring: "Input handling extracted from {@link PromptHandler}: aggregates editor/config"
        children:
        - id: PromptInputHandler.3vFeve|ODKP5L
          signature: private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)^```…
        - id: PromptInputHandler.PromptInputHandler
          signature: "public PromptInputHandler(ConfigManager cfg, IActiveEditor editorListener)"
        - id: PromptInputHandler.buildPrompt
          signature: "public Prompt buildPrompt(IExecutor exec, boolean batch)"
        - id: PromptInputHandler.detectSelection
          signature: "private void detectSelection(Selection sel, PromptArguments arg)"
        - id: PromptInputHandler.detectFullFile
          signature: "private void detectFullFile(Selection content, PromptArguments arg)"
        - id: PromptInputHandler.detectedCmd
          signature: "private boolean detectedCmd(String line, String[] lines, int lineIndex, PromptA…"
        - id: PromptInputHandler.captureYamlBlock
          signature: "private String captureYamlBlock(String[] lines, int lineIndex)"
```
/answer 1359-22 allow
Control Request:
```yaml
id: 1359-23
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    ids:
    - EditorInterface.insertTag
    - EditorInterface.replaceTag
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/AdaptingConnector.java
    ids:
    - AdaptingConnector.convertResponse
  reason: "Wie Tag ersetzt und Antwort konvertiert wird, damit die Approver-Position und Answer-Veränderung klar sind"
```
/answer 1359-23 allow
Control Request:
```yaml
id: 1359-24
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: EditorInterface.insertTag
        type: MethodDeclaration
        code: "\t@Override\n\tpublic void insertTag(IExecutor exec, IModelRequest req, IProgress mon) {\n\t\texec.syncExec(() -> {\n\t\t\tOutputMode outputMode = req.getPrompt().config.outputMode;\n\t\t\tif (OutputMode.New_File.equals(outputMode)) {\n\t\t\t\tIContainer parent = editorListener.getLastContainer();\n\n\t\t\t\tString timestamp = LocalDateTime.now().format(DateTimeFormatter.ofPattern(\"yyyyMMdd.HHmmss\"));\n\t\t\t\tIFile newFile = parent.getFile(new Path(timestamp + \".md\"));\n\t\t\t\tString tag = generateTag(req);\n\t\t\t\ttry {\n\t\t\t\t\tInputStream source = new ByteArrayInputStream(tag.getBytes(\"UTF-8\"));\n\n\t\t\t\t\tif (!newFile.exists()) {\n\t\t\t\t\t\tnewFile.create(source, true, null);\n\t\t\t\t\t} else {\n\t\t\t\t\t\tnewFile.setContents(source, true, true, null);\n\t\t\t\t\t}\n\t\t\t\t\tnewFile.touch(null);\n\n\t\t\t\t\tIWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();\n\t\t\t\t\tIDE.openEditor(page, newFile);\n\t\t\t\t} catch (PartInitException e) {\n\t\t\t\t\tLOG.info(\"Error opening new editor file\", e);\n\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\tLOG.info(\"Error writting file\", e);\n\t\t\t\t} catch (UnsupportedEncodingException e) {\n\t\t\t\t\tLOG.info(\"Error unsupported encoding\", e);\n\t\t\t\t}\n\n\t\t\t} else {\n\n\t\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\tISelection selection = textEditor.getSelectionProvider().getSelection();\n\t\t\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\n\t\t\t\ttry {\n\t\t\t\t\tString tag = generateTag(req);\n\t\t\t\t\tswitch (outputMode) {\n\t\t\t\t\tcase Chat:\n\t\t\t\t\t\tString replace = String.format(\"\\n%s\\n%s\\n%s\\n\", AGENT, tag, USER);\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, replace);\n\t\t\t\t\t\texec.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Append:\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, \"\\n\" + tag);\n\t\t\t\t\t\texec.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Replace:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), tsel.getLength(), tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Cursor:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), 0, tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase New_File:\n\t\t\t\t\t\tthrow new UnsupportedOperationException();\n\t\t\t\t\t}\n\t\t\t\t\ttextEditor.doSave(new NullProgressMonitor());\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.info(\"Error adding text\", e);\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t}\n"
      - id: EditorInterface.replaceTag
        type: MethodDeclaration
        code: "\t@Override\n\tpublic void replaceTag(AIAnswer ans, IProgress mon) {\n\t\tOutputMode mode = ans.prompt != null ? ans.prompt.config.outputMode : null;\n\t\tif (ans.showStats && (OutputMode.Chat.equals(mode) || OutputMode.Append.equals(mode)))\n\t\t\tans.answer = ans.print() + \"\\n\" + ans.answer;\n\t\tif (!markerScanner.findAndReplaceMarkers(ans))\n\t\t\tLOG.info(\"Error: wasn't able to replace prompt marker with answer:\\n\" + ans.answer);\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/AdaptingConnector.java
      nodes:
      - id: AdaptingConnector.convertResponse
        type: MethodDeclaration
        code: |2
            @SuppressWarnings("unchecked")
            @Override
            public AIAnswer convertResponse(IModelResponse response, IProgress mon) {
              return getConnector(response).convertResponse(response, mon);
            }
```
/answer 1359-24 allow
Control Request:
```yaml
id: 1359-25
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector
    pattern: SessionRenderer\.(toolCall|toolResult)|ToolLoop|toolLoop
    include:
    - '*.java'
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: autoapprove|AutoApprove
    include:
    - '*.java'
    - '*.xml'
    - '*.MF'
    - '*.md'
    limit: 10
  reason: Tool-Loop-Implementierung der direkten Modelle und mögliche Vorarbeiten zu Auto-Approve finden
```
/answer 1359-25 allow
Control Request:
```yaml
id: 1359-26
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector
      matches:
      - path: harness/SessionCallbacks.java
        matches:
        - "return message(Role.Agent, SessionRenderer.toolCall(call));"
        - "return message(Role.User, SessionRenderer.toolResult(result));"
      - path: harness/SessionAnswerBuilder.java
        matches:
        - "return separate().appendRaw(SessionRenderer.toolCall(new ToolCall(id, name, arguments)));"
        - "return separate().appendRaw(SessionRenderer.toolResult(new ToolResult(id, content)));"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: project/autoapprove_1.md
        matches:
        - "- Zielpackage: `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove`"
        - '- Regeln werden im Package `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules` abgelegt.'
        - '- /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove'
        - "reason: Bestandsaufnahme des Zielpackages autoapprove und der Command-Klassen"
        - Das Package `autoapprove` existiert noch nicht. Jetzt lese ich die Details.
        - "Der Package `autoapprove` existiert noch nicht; ich habe Integrationspunkt, Commands, Control-Request-Format und Stats-Modell gelesen. Als Nächstes prüfe ich, wie ein Folge-Prompt in `PromptHandler.execute` ausgelöst wird."
        - "pattern: autoapprove|AutoApprove"
```
/answer 1359-26 allow
Control Request:
```yaml
id: 1359-27
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
    ids:
    - PromptInputHandler.buildPrompt
  reason: "Prüfen, wie Prompt/PromptArguments entstehen, um programmatische Folge-Prompts zu bewerten"
```
/answer 1359-27 allow
ReasoningToken: 607
Control Request:
```yaml
id: 1359-28
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
      nodes:
      - id: PromptInputHandler.buildPrompt
        type: MethodDeclaration
        code: |2
            public Prompt buildPrompt(IExecutor exec, boolean batch) {
              PromptArguments arg = new PromptArguments();
              exec.syncExec(() -> {
                arg.setEditor(editorListener.getLastTextEditor());
                arg.absoluteFilePath = editorListener.resolveAbsoluteFilePath();
                arg.project = editorListener.resolveProjectPath();
                if (cfg.isInputEnabled(InputMode.Selection))
                  detectSelection(editorListener.getSelection(), arg);
                else if (cfg.isInputEnabled(InputMode.Converter))
                  detectFullFile(editorListener.getFileContent(), arg);
              });
              for (String input : arg.inputs) if (IEditorInterface.containsPromptTag(input))
                throw new IllegalStateException("Prompt accidentally triggered on a line containing a prompt tag");
              FrozenConfig frozen = FrozenConfig.from(cfg);
              arg.processorEnabled = cfg.isInputEnabled(InputMode.Converter);
              if (arg.inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank()) && arg.command == null)
                throw new IllegalArgumentException("Input and System Prompt and Command Empty");
              if (arg.getEditor() == null && !batch)
                throw new IllegalArgumentException("Result editor unset");
              return new Prompt(arg.inputs, batch, frozen, arg);
            }
```
/answer 1359-28 allow
Control Request:
```yaml
id: 1359-29
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector
    pattern: new ToolCall\(|invokeCall|CMD_CALL|class \w*Loop\w*
    include:
    - '*.java'
    limit: 20
  reason: Ort der Tool-Loop-Ausführung für direkte Modelle und den MCP-invokeCall bestimmen
```
/answer 1359-29 allow
Control Request:
```yaml
id: 1359-2a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector
      matches:
      - path: mcp/MCPControlClient.java
        matches:
        - sb.append(CallCommand.CMD_CALL);
      - path: mcp/MCPConnector.java
        matches:
        - "return invokeCall(req.id, control.parseYaml(req.yamlBlock));"
        - "return invokeCall(req.id, control.parseYaml(req.command.parameter(0)));"
        - "private MCPResponse invokeCall(String id, JsonNode call) {"
      - path: harness/SessionRenderer.java
        matches:
        - "return String.format(\"%s\\n%s\\n%s %s\", IEditorInterface.TOOLUSE, YAML.toYamlBlock(node), CallCommand.CMD_CALL,"
      - path: harness/SessionProcessor.java
        matches:
        - "out.add(call.toolCall(new ToolCall(node.path(\"id\").asText(\"\"), node.path(\"tool\").asText(\"\"),"
      - path: harness/SessionAnswerBuilder.java
        matches:
        - "return separate().appendRaw(SessionRenderer.toolCall(new ToolCall(id, name, arguments)));"
```
/answer 1359-2a allow
Result Stats: id=123c3b3a-c226-4b4d-8683-f62d56bab1c9, total: 76044, in: 22, out: 17111, reason: 4457, read: 317532, write: 58911, start: 1794053292807, end: 1794053355818, time: 62740, total: 128353, io: 0, control: 271, sent: 0, recv: 837
ReasoningToken: 4457
# Analyse Auto-Approve

Das Package `autoapprove` gibt es noch nicht. Ich habe nur gelesen und nichts geändert. Gelesen habe ich `PromptHandler`, `PromptInputHandler`, die fünf Commands, `CommandRegistry`, `CCConnector`, `CCControlClient`, `MCPConnector`, `SessionRenderer`, `EditorInterface`, `AIAnswer`, `TokenStats`, `TimeStats`, `Prompt`, `PromptArguments` und `CCSession`/`CCSessionManager`.

## 1. Was der Approver sieht

Der Approver bekommt einen `AIAnswer` mit Freitext in `answer`. Die Tool-Aufrufe stecken als Textmuster darin:

| Fall | Muster in `answer` | Quelle |
|---|---|---|
| Control Request (Claude Code / MCPC) | `Control Request:` + ```` ```yaml ```` + `/answer <id> allow` | `CCControlClient.checkControlEndpoint` |
| Tool-Call (direktes Modell) | `Tool:` + ```` ```yaml {id, tool, arguments} ```` + `/call <id>` | `SessionRenderer.toolCall` |
| Tool-Result (direktes Modell) | `ToolResult:` + ```` ```yaml {id, result} ```` ohne Folgekommando | `SessionRenderer.toolResult` |
| Tool-Result eines Control Requests | Control-Request-Block, dessen YAML eine `result:`-Zeile hat | `AnswerCommand.blockType()` |

**Befund 1: Es gibt keinen Parser für Antworten.** `CommandRegistry.detect` arbeitet zeilenweise auf Editor-Text. `captureYamlBlock` in `PromptInputHandler` ist `private`. Der Approver braucht einen eigenen `AnswerParser`, der aus `answer` eine Liste von Items extrahiert. Der YAML-Capture sollte dafür herausgelöst werden.

**Befund 2: Ein Control Request enthält nur ein Item.** `checkControlEndpoint` rendert nur `pending.get(0)`. Mehrere Toolcalls in einer Antwort gibt es also nur bei direkten Modellen mit mehreren `Tool:`-Blöcken. Bei Claude Code laufen mehrere offene Requests als Kette einzelner Antworten.

**Befund 3: Control-Request-Antworten haben `showStats=false`.** Regeln wie "Turn 10 ohne Cache-Read" brauchen Stats, die in genau diesen Antworten fehlen können. Ob die Felder trotzdem befüllt sind, habe ich nicht geprüft.

## 2. Folgeaktionen

Die Ausführung gibt es schon, aber nur über den Editor-Pfad:

- `Allow`, `Deny` und `Exec` laufen über `AnswerCommand`. `CCConnector` ruft dafür `controlClient.approve`, `deny` oder `exec` auf.
- `Call` läuft über `CallCommand` plus `arg.yaml`. `MCPConnector.invokeCall` führt es aus, `CCConnector` ruft `submitEdit` auf.
- Ein geänderter Aufruf läuft über `CallEditCommand`, der das YAML direkt mitbringt.

**Befund 4: Ein Folge-Prompt lässt sich nicht aus dem Editor-Zustand bauen.** `PromptRun.run` ruft `prepareRequest` und damit `PromptInputHandler.buildPrompt` auf. Das liest Selektion und Cursor aus dem Editor. Der Approver muss stattdessen einen `Prompt` aus `ans.prompt` ableiten (Pfad, Projekt, Editor, Config) und `arg.command` sowie `arg.yaml` selbst setzen. `PromptRun` braucht dafür einen Konstruktor, der einen fertigen Prompt entgegennimmt.

Dabei gibt es mehrere Risiken:

- **Config wird mutiert.** `preprocessPrompt` überschreibt `prompt.config.model` und `prompt.config.keys`. Der Folge-Prompt braucht deshalb eine Kopie der `FrozenConfig`, sonst ändert er den Originalprompt.
- **Tag-Einfügung hängt am Editor.** `insertTag` nutzt Selektion und Cursor (Modi Replace und Cursor). Das passt für automatische Folgejobs nicht. In Chat und Append ist es unkritisch, dort verhält es sich wie ein manuelles `/answer`.
- **Endlosketten.** Jede Folgeantwort läuft wieder durch den Approver. Eine Allow-Kette bei Claude Code ist gewollt, braucht aber Tiefenlimit und Abbruch. Außerdem sollte per `exec.schedule` statt per Rekursion geplant werden.

## 3. Antwort verändern

**Befund 5: "Nach replaceTag" und "Antwort anreichern" widersprechen sich.** `replaceTag` ersetzt den Marker im Editor. Danach gibt es den Marker nicht mehr, und `IEditorInterface` hat keine API zum nachträglichen Ändern. Zwei Lösungen:

- (a) Der Approver wertet vor `replaceTag` aus (Anreichern) und handelt danach (Folgejobs).
- (b) `IEditorInterface` bekommt eine neue Methode zum Anhängen an die Antwort.

Ich würde (a) bevorzugen. Das ist eine Abweichung von der Vorgabe und braucht deine Entscheidung.

Außerdem fängt der `catch` in `run()` Fehler ab und ruft `replaceTag` mit der Fehlermeldung auf. Der Approver braucht darum einen eigenen `try/catch`, der im Fehlerfall `Pass` ergibt. Sonst würde ein Regelfehler eine bereits ersetzte Antwort überschreiben.

## 4. Session-Status

**Befund 6: Es gibt kein einheitliches Session-Objekt.** Vorhanden sind:

- `CCSession` mit State, TTL, `getLastSentAt` und `getLastReceivedAt`. Das gilt nur für Claude Code.
- `AIAnswer.sessionId` und `AIAnswer.stats` pro Antwort.
- `TimeStats.add`, dokumentiert als "Accumulates … into this (session) total". Wo diese Summe gehalten wird, habe ich nicht gefunden.

Einen Turn-Zähler und kumulierte Cache-Read-Werte habe ich nicht gefunden. Der Approver braucht ein eigenes `SessionSnapshot` (Turn, kumulierte Stats, letzte Aktivität, State, TTL). Er sollte es selbst pro `sessionId` führen und bei Claude Code zusätzlich aus `CCSession` füllen.

Die Idle-Time-Reaktion kann nicht nur auf Antworten reagieren, denn bei Stille kommt keine Antwort. Dafür bietet sich ein verzögerter Job an (`exec.schedule(name, run, delay)` existiert).

## 5. Vorschlag Regel-Design

```java
// rules/
public interface IApprovalRule {
    Vote vote(ApprovalContext ctx);   // Voter: entscheidet Anwendbarkeit selbst
}
public enum Verdict { ALLOW, DENY, ABSTAIN }
public record Vote(Verdict verdict, String reason, String hint,
                   List<String> warnings, String followUp) {}
```

- `ApprovalContext` enthält Session-Snapshot, `AIAnswer`, `Prompt` und das aktuelle `ApprovalItem`. Das Item hat Art (Control-Call, Control-Result, Tool-Call, Tool-Result), `id`, Tool und Argumente bzw. Result.
- `RuleRegistry` hält die Regeln in Reihenfolge, mit `register(...)`. Alle Regeln werden durchlaufen.
- Der Approver aggregiert und führt aus. Regeln entscheiden nur.
- Layout in `autoapprove/`: `AutoApprover`, `RuleRegistry`, `AnswerParser`, `ApprovalItem`, `ApprovalContext`, `SessionTracker`, `FollowUpDispatcher`, `ApprovalResult` (`Pass`/`Deny`/`Allow`/`Call`) und `ApprovalListener` für die spätere Operator-Anzeige. Die Regeln liegen in `autoapprove/rules/`.

## 6. Lücken in der Spezifikation

1. **Weitere Ergebnisse als `Deny`/`Allow`/`Abstain`.** Deine Beispiele 5 bis 7 (Warnung anreichern, Idle-Fortsetzung, rekursiver LLM-Call) sind keine Tool-Entscheidungen. Ich habe `warnings` und `followUp` als Daten im `Vote` vorgesehen, die der Approver umsetzt. Passt das?
2. **Konfliktauflösung.** Die Aggregation bei gemischten Votes ist nicht definiert. Ich schlage vor: `DENY` schlägt `ALLOW`, `ALLOW` schlägt `ABSTAIN`, Reasons und Hints werden zusammengeführt.
3. **Mehrere Toolcalls in einer Antwort.** Das Ergebnis ist dann pro Item, nicht pro Antwort. Bei gemischten Entscheidungen ist unklar, wie die Antwort zusammengefasst wird. Ebenso unklar ist, ob erst alle Items entschieden sein müssen, bevor ein Folgejob läuft.
4. **`Deny` bei direkten Tool-Calls.** Ohne Control Request gibt es kein `/answer deny`. Eine Ablehnung müsste als Prompt oder als `ToolResult`-Block mit dem Grund zurückgehen. Das habe ich im Code nicht gefunden und es ist zu klären.
5. **`Exec`.** Die Action `AnswerCommand.Action.Exec` (`controlClient.exec`) kommt in deinen vier Ergebnissen nicht vor. Soll der Approver sie nutzen können?
6. **Argumente ändern.** `CallEditCommand` erlaubt es, Argumente zu ersetzen, etwa um das CWD zu korrigieren statt abzulehnen. Soll eine Regel das vorschlagen dürfen?

Außerdem liegt unter `/home/user/xyan/xy.ai.workbench/project/autoapprove_1.md` eine Datei. Sie enthält deine Aufgabenbeschreibung und Bruchstücke meiner bisherigen Zwischenmeldungen aus dieser Session. Das sieht nach einem versehentlichen Mitschnitt aus. Ich habe sie nicht verändert.

Soll ich als Nächstes die Punkte 5.1 und 5.2 festlegen und mit dem Gerüst (Interface, Registry, Parser, `AutoApprover`) und der Anbindung in `PromptRun` beginnen? Die Entscheidung zu Befund 5 (vor oder nach `replaceTag`) brauche ich dafür zuerst.
Gib mir eine Minute
Ja du hast recht zu Punkt 3. der Approver muss vor dem Replace Tag ansetzen.
Es stimmt nicht das der Approver keinen Prompt erzeugen kann.
Die Ableitung aus der Prompt Datei ist die zentrale Schnittstelle. Der Approver handelt also, indem er potenziell selbst den Tag ersetzt und den folgeprompt so auslöst wie der Operator es getan hätte. Was er dabei benötigt ist hauptsächlich die Selection.
Tunr Anzahl könnte in den Stats noch ergänzt werden zur Session.
Der Trigger für die Idletime liefert ein externer Trigger.
Lass wir die multiplen Tool-Calls im gleichen Loop erstmal weg.
SystemInit:  id=123c3b3a-c226-4b4d-8683-f62d56bab1c9, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Verstanden. Das ändert das Design an zwei Stellen: Der Approver ersetzt den Tag selbst, und der Folgeprompt entsteht über die Selection wie beim Operator. Ich prüfe jetzt, wie Selection und Marker-Ersetzung technisch funktionieren.
ReasoningToken: 684
Control Request:
```yaml
id: 1359-2b
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: interface IActiveEditor|class ActiveEditorListener|class MarkerScanner|findAndReplaceMarkers|record Selection|class Selection
    include:
    - '*.java'
    limit: 20
  reason: Typen für Editor-Selection und Marker-Ersetzung lokalisieren
```
/answer 1359-2b allow
Control Request:
```yaml
id: 1359-2c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/EditorInterface.java
        matches:
        - if (!markerScanner.findAndReplaceMarkers(ans))
      - path: src/xy/ai/workbench/ActiveEditorListener.java
        matches:
        - "public class ActiveEditorListener extends PartListener2Adapter implements IPartListener2, IActiveEditor {"
      - path: src/xy/ai/workbench/editor/EditorChangeListener.java
        matches:
        - "public class SelectionListener implements ISelectionChangedListener {"
      - path: src/xy/ai/workbench/marker/MarkerRessourceScanner.java
        matches:
        - "public boolean findAndReplaceMarkers(AIAnswer ans) {"
      - path: connector/src/xy/ai/conscious/api/IActiveEditor.java
        matches:
        - "public interface IActiveEditor {"
        - "public record Selection(String[] selection, Integer cursorOffset) {"
```
/answer 1359-2c allow
Control Request:
```yaml
id: 1359-2d
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/api/IActiveEditor.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ActiveEditorListener.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  reason: API für Selection lesen/setzen und Marker-Ersetzung verstehen
```
/answer 1359-2d allow
Control Request:
```yaml
id: 1359-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/api/IActiveEditor.java
      nodes:
      - id: wMUZob|BtYDUa
        signature: import java.nio.file.Path;
      - id: IActiveEditor
        signature: "public interface IActiveEditor {"
        children:
        - id: IActiveEditor.getSelection
          signature: abstract Selection getSelection()
        - id: IActiveEditor.getFileContent
          signature: abstract Selection getFileContent()
        - id: IActiveEditor.resolveAbsoluteFilePath
          signature: abstract String resolveAbsoluteFilePath()
        - id: IActiveEditor.resolveProjectPath
          signature: abstract Path resolveProjectPath()
        - id: IActiveEditor.getLastTextEditor
          signature: Object getLastTextEditor()
        - id: IActiveEditor.removeCommentLines
          signature: public static String removeCommentLines(Selection input)
        - id: IActiveEditor.Selection
          signature: "public record Selection(String[] selection, Integer cursorOffset) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ActiveEditorListener.java
      nodes:
      - id: m8E7jQ|sMsvfQ
        signature: import java.io.File;
      - id: bPYsxK|hap3Sq
        signature: import org.eclipse.ui.IWorkbenchPartReference;
      - id: ActiveEditorListener
        signature: public class ActiveEditorListener extends PartListener2Adapter implements IPart…
        children:
        - id: ActiveEditorListener.7MTwkl|a9VMMU
          signature: private final EditorChangeListener editorListener = new EditorChangeListener();
        - id: ActiveEditorListener.addInputObserver
          signature: public void addInputObserver(Consumer<InputMode> obs)
        - id: ActiveEditorListener.addTextEditorObserver
          signature: public void addTextEditorObserver(Consumer<ITextEditor> obs)
        - id: ActiveEditorListener.checkInitBindings
          signature: private void checkInitBindings()
        - id: ActiveEditorListener.getLastTextEditor
          signature: public ITextEditor getLastTextEditor()
        - id: ActiveEditorListener.getLastEditorInput
          signature: private IEditorInput getLastEditorInput()
        - id: ActiveEditorListener.getLastEditorSelection
          signature: private ITextSelection getLastEditorSelection()
        - id: ActiveEditorListener.getLastEditorFile
          signature: public IFile getLastEditorFile()
        - id: ActiveEditorListener.getLastContainer
          signature: public IContainer getLastContainer()
        - id: ActiveEditorListener.getSelection
          signature: public Selection getSelection()
        - id: ActiveEditorListener.resolveAbsoluteFilePath
          signature: public String resolveAbsoluteFilePath()
        - id: ActiveEditorListener.resolveProjectPath
          signature: public Path resolveProjectPath()
        - id: ActiveEditorListener.getFileContent
          signature: public Selection getFileContent()
        - id: ActiveEditorListener.partActivated
          signature: public void partActivated(IWorkbenchPartReference partRef)
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      nodes:
      - id: EnIoex|zidzZW
        signature: import java.io.BufferedReader;
      - id: RgSv9x|9C3gUk
        signature: import org.eclipse.jface.text.BadLocationException;
      - id: wRD3M9|xFNZQY
        signature: import xy.ai.conscious.model.AIAnswer;
      - id: MarkerRessourceScanner
        signature: "public class MarkerRessourceScanner implements IResourceChangeListener, IResour…"
        children:
        - id: MarkerRessourceScanner.1KGB34|BKcKaJ
          signature: private static final String MARKER_ID = "xy.ai.workbench.promptmarker";
        - id: MarkerRessourceScanner.MarkerRessourceScanner
          signature: "public MarkerRessourceScanner(ConfigManager cfg, BundleContext context)"
        - id: MarkerRessourceScanner.dispose
          signature: public void dispose(BundleContext context)
        - id: MarkerRessourceScanner.visit
          signature: public boolean visit(IResourceDelta delta) throws CoreException
        - id: MarkerRessourceScanner.rescannFile
          signature: private void rescannFile(IFile file)
        - id: MarkerRessourceScanner.resourceChanged
          signature: public void resourceChanged(IResourceChangeEvent event)
        - id: MarkerRessourceScanner.findAndReplaceMarkers
          signature: public boolean findAndReplaceMarkers(AIAnswer ans)
          docstring: '@param ans'
        - id: MarkerRessourceScanner.replaceMarker
          signature: "private boolean replaceMarker(AIAnswer ans, IMarker marker)"
        - id: MarkerRessourceScanner.resolveTagRange
          signature: "private int[] resolveTagRange(IDocument doc, String requestId, int off, int len)"
          docstring: Verifies that the given offset/length still points at the tag belonging to
        - id: MarkerRessourceScanner.findTagInDocument
          signature: "private int[] findTagInDocument(IDocument doc, String requestId)"
          docstring: Scans the full doc content for the tag belonging to the given request id.
        - id: MarkerRessourceScanner.tryReplaceInEditor
          signature: "private boolean tryReplaceInEditor(AIAnswer ans, ITextEditor editor)"
          docstring: "Fallback used when the marker based replacement failed, e.g. because the"
        - id: MarkerRessourceScanner.replaceInDocument
          signature: "private boolean replaceInDocument(AIAnswer ans, ITextEditor editor, IDocument d…"
        - id: MarkerRessourceScanner.findAndReplaceInOpenEditors
          signature: private boolean findAndReplaceInOpenEditors(AIAnswer ans)
        - id: MarkerRessourceScanner.findOpenEditorFor
          signature: private ITextEditor findOpenEditorFor(IFile file)
        - id: MarkerRessourceScanner.isAutoFollowModeEnabled
          signature: private boolean isAutoFollowModeEnabled()
        - id: MarkerRessourceScanner.shouldAutoFollow
          signature: "private boolean shouldAutoFollow(ITextEditor editor, IDocument doc)"
        - id: MarkerRessourceScanner.AutoFollowState
          signature: "/**  * Holds, per doc, the auto-follow {"
          docstring: "Holds, per doc, the auto-follow {@link IDocumentListener} together with the"
          children:
          - id: MarkerRessourceScanner.AutoFollowState.hrG5hD|T7P2Br
            signature: final ITextEditor editor;
          - id: MarkerRessourceScanner.AutoFollowState.AutoFollowState
            signature: "AutoFollowState(ITextEditor editor, IDocumentListener listener)"
        - id: MarkerRessourceScanner.QY71kA|WDmgDC
          signature: "private final Map<IDocument, AutoFollowState> autoFollowListeners = new HashMap…"
        - id: MarkerRessourceScanner.moveCursorToLastLineStart
          signature: "private void moveCursorToLastLineStart(ITextEditor editor, IDocument doc)"
          docstring: "Moves the cursor to the end of the doc (start of the last line, i.e. an empty"
        - id: MarkerRessourceScanner.ensureAutoFollowListener
          signature: "private void ensureAutoFollowListener(ITextEditor editor, IDocument doc)"
        - id: MarkerRessourceScanner.ensurePartCloseCleanupRegistered
          signature: private void ensurePartCloseCleanupRegistered()
        - id: MarkerRessourceScanner.unwrapTextEditor
          signature: private ITextEditor unwrapTextEditor(IWorkbenchPart part)
        - id: MarkerRessourceScanner.removeAutoFollowListenerFor
          signature: private void removeAutoFollowListenerFor(ITextEditor editor)
        - id: MarkerRessourceScanner.tryMoveCursorToLastLineStart
          signature: "private boolean tryMoveCursorToLastLineStart(ITextEditor editor, IDocument doc)"
          docstring: Attempts to place the cursor at the end of the doc and verifies that the
        - id: MarkerRessourceScanner.getOpenTextEditors
          signature: private java.util.List<ITextEditor> getOpenTextEditors()
```
/answer 1359-2e allow
ReasoningToken: 556
Control Request:
```yaml
id: 1359-2f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ActiveEditorListener.java
    ids:
    - ActiveEditorListener.getSelection
    - ActiveEditorListener.getLastEditorSelection
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    ids:
    - MarkerRessourceScanner.replaceInDocument
    - MarkerRessourceScanner.moveCursorToLastLineStart
  reason: "Prüfen, wie die Selection bestimmt wird und wo der Cursor nach dem Tag-Replace landet"
```
/answer 1359-2f deny das ist nicht notwendig. Die IDdee ist eher das der Approver nicht den tatsächlichen Cursor ändert sondern die Selection für den nächsten Prompt ändert. Der Approver kann den tag setzen und anschließend die Parameter für den nächsten Prompt erstellen.
Result Stats: id=123c3b3a-c226-4b4d-8683-f62d56bab1c9, total: 95023, in: 32, out: 21180, reason: 503, read: 653685, write: 73811, start: 1794053931088, end: 1794053948880, time: 17688, total: 167948, io: 0, control: 104, sent: 0, recv: 290
ReasoningToken: 503
Verstanden, ich lasse die Dateien ungelesen. Der Cursor bleibt unberührt. Der Approver baut die Selection für den Folgeprompt selbst.

## Angepasster Ablauf in `PromptRun.run`

1. `executeInner` liefert `ans`.
2. `approver.evaluate(ans)` parst die Antwort und befragt die Regeln. Das Ergebnis ist ein `Decision` mit Outcome (`Pass`/`Deny`/`Allow`/`Call`), Reason/Hint und Warnungen. Warnungen reichern `ans.answer` an, die Antwort ist noch nicht im Editor.
3. Der Approver oder `PromptRun` ruft `editIfc.replaceTag(ans)` auf. Der Tag wird immer ersetzt, auch bei `Pass`.
4. `approver.act(decision)` baut für `Deny`/`Allow`/`Call` den Folgeprompt und plant ihn mit `exec.schedule`.

## Folgeprompt über injizierte Selection

`PromptInputHandler.detectSelection(Selection, PromptArguments)` nimmt die Selection schon als Parameter. Nur `buildPrompt` holt sie aus `editorListener`. Das reicht als Änderung:

- `buildPrompt(exec, batch, Selection override)` als Überladung. Ohne Override bleibt das Verhalten wie heute.
- `PromptRun` bekommt einen optionalen Override, den es an `prepareRequest` weiterreicht.
- Der Approver erzeugt die Selection aus den Zeilen der Antwort, nämlich dem YAML-Block plus der Kommandozeile:
  - `Allow`/`Deny`: `/answer <id> allow|deny [reason|hint]`.
  - `Call`: `/call <id>`.
- `detectSelection` erkennt Command und YAML dann wie beim Operator. `AnswerCommand`, `CallCommand` und die Connectoren bleiben unverändert.

Offen bleibt der Editor selbst. `buildPrompt` setzt weiter `arg.setEditor(editorListener.getLastTextEditor())` und den Dateipfad. Der Folgeprompt landet also im zuletzt aktiven Editor. Wenn der Operator zwischenzeitlich wechselt, wäre das der falsche. Besser wäre es, Editor und Pfad aus dem Ursprungs-Prompt (`ans.prompt.arg`) zu übernehmen. Das lässt sich in derselben Überladung lösen.

## Übernommen

- Turn-Anzahl kommt als Feld in die Session-Stats. Ein Zähler pro `sessionId` im Approver reicht für den Anfang, falls die Stats nicht erweitert werden.
- Der Idle-Trigger kommt von außen. Der Approver bekommt dafür eine Methode wie `onIdle(sessionId)`. Sie baut die Selection mit der definierten Fortsetzungs-Anweisung.
- Mehrere Toolcalls pro Antwort lasse ich weg. Der Parser nimmt das erste Item und ignoriert weitere. Bei mehreren Items ergibt das `Pass`, damit nichts halb entschieden wird.

## Als Nächstes

Ich würde mit dem Gerüst im Package `autoapprove` und `autoapprove/rules` beginnen:

- `IApprovalRule`, `Verdict`, `Vote`
- `RuleRegistry`
- `AnswerParser`
- `ApprovalContext`
- `Decision`
- `AutoApprover`

Dazu kommen die beiden kleinen Änderungen in `PromptInputHandler` und `PromptHandler`. Soll ich so starten, mit `DENY` vor `ALLOW` vor `ABSTAIN` als Aggregation?
