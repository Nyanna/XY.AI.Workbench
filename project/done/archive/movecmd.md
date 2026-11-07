Die Methode `preprocessInput` in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java` soll vom Kommandosystem in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java` übernommen werden. Diese verfügt bereits über eine Logik, die ein Kommando im Input priorisiert und lokalisiert basierend auf der Selektion (Zeile, Block, Datei).

Ein eigener `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java` wird nicht benötigt.

Die Kommandotypisierend wird bereits implementiert:
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/PromptCommand.java`
	- Title ermittelung soll mit in das `PromptCommand` wandern
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java`
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java`
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java`
	- `AnswerCommand` kann Allow, Deny als interne Enum übernehmen
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ModificationCommand.java`

Der `CCConnector` soll also nicht selbst Kommando Parsing implementieren. Diverse Kommandos werden zwischen verschiedenen geteilt.
SystemInit:  id=fb96ed27-5e56-43b6-8335-4c0e2eeeed43, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 47e1-4a
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/PromptCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ModificationCommand.java
  reason: "Aktuellen Stand der beteiligten Dateien verstehen, um preprocessInput-Logik zu migrieren."
```
/answer 47e1-4a allow
Control Request:
```yaml
id: 47e1-4b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: OsZtmS|YZNdWp
      - id: c3l42g|pOv9wK
      - id: uzcHjn|8HwufD
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
        - id: CCConnector.preprocessInput
          signature: "private List<Command> preprocessInput(List<String> inputs) {"
        - id: CCConnector.convertResponse
          signature: "public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {"
        - id: CCConnector.getEditorLocation
          signature: "private EditorLocation getEditorLocation() {"
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
      nodes:
      - id: Q6eIx0|8zE170
      - id: CommandType
        signature: "public enum CommandType {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/PromptCommand.java
      nodes:
      - id: fau272|XRuDTB
      - id: PromptCommand
        signature: "public class PromptCommand extends Command {"
        children:
        - id: PromptCommand.PromptCommand
          signature: "public PromptCommand() {"
        - id: PromptCommand.PromptCommand_1
          signature: "public PromptCommand(String text) {"
        - id: PromptCommand.prefix
          signature: "public String prefix() {"
        - id: PromptCommand.matches
          signature: "public boolean matches(String line) {"
        - id: PromptCommand.parse
          signature: "public Command parse(String line) {"
        - id: PromptCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java
      nodes:
      - id: fau272|XRuDTB
      - id: ResumeCommand
        signature: "public class ResumeCommand extends Command {"
        children:
        - id: ResumeCommand.M0PdNg|ksR6XK
        - id: ResumeCommand.ResumeCommand
          signature: "public ResumeCommand() {"
        - id: ResumeCommand.ResumeCommand_1
          signature: "public ResumeCommand(String sessionId) {"
        - id: ResumeCommand.prefix
          signature: "public String prefix() {"
        - id: ResumeCommand.matches
          signature: "public boolean matches(String line) {"
        - id: ResumeCommand.parse
          signature: "public Command parse(String line) {"
        - id: ResumeCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
      nodes:
      - id: fau272|XRuDTB
      - id: AnswerCommand
        signature: "public class AnswerCommand extends Command {"
        children:
        - id: AnswerCommand.yzeuY3|Jf8A1X
        - id: AnswerCommand.AnswerCommand
          signature: "public AnswerCommand() {"
        - id: AnswerCommand.AnswerCommand_1
          signature: "public AnswerCommand(String id, String action, String reason) {"
        - id: AnswerCommand.prefix
          signature: "public String prefix() {"
        - id: AnswerCommand.matches
          signature: "public boolean matches(String line) {"
        - id: AnswerCommand.parse
          signature: "public Command parse(String line) {"
        - id: AnswerCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ModificationCommand.java
      nodes:
      - id: fau272|XRuDTB
      - id: ModificationCommand
        signature: "public class ModificationCommand extends Command {"
        children:
        - id: ModificationCommand.ModificationCommand
          signature: "public ModificationCommand() {"
        - id: ModificationCommand.prefix
          signature: "public String prefix() {"
        - id: ModificationCommand.matches
          signature: "public boolean matches(String line) {"
        - id: ModificationCommand.parse
          signature: "public Command parse(String line) {"
        - id: ModificationCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
```
/answer 47e1-4b allow
ReasoningToken: 18
Text:
 Ich lese die vollständigen Quellen der relevanten Dateien.
Control Request:
```yaml
id: 47e1-4c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - OsZtmS|YZNdWp
    - c3l42g|pOv9wK
    - uzcHjn|8HwufD
    - CCConnector
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - RLo8qh|nFvk0T
    - 4UXeuu|NTMCrV
    - KEc50E|0JLMrF
    - fPHG2Y|yRFIp4
    - PromptHandler
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
    ids:
    - Q6eIx0|8zE170
    - CommandType
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/PromptCommand.java
    ids:
    - fau272|XRuDTB
    - PromptCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
    ids:
    - ExitCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java
    ids:
    - ResumeCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
    ids:
    - AnswerCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ModificationCommand.java
    ids:
    - ModificationCommand
  reason: "Vollständigen Code laden, um Migration von preprocessInput nach PromptHandler und Command-Klassen korrekt umzusetzen."
```
/answer 47e1-4c allow
Control Request:
```yaml
id: 47e1-4d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: OsZtmS|YZNdWp
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import java.io.IOException;
          import java.nio.file.Path;
          import java.nio.file.Paths;
          import java.util.ArrayList;
          import java.util.List;
          import java.util.UUID;
          import java.util.zip.CRC32;

          import org.eclipse.core.resources.IProject;
          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.SubMonitor;
          import org.eclipse.core.runtime.jobs.Job;
          import org.eclipse.swt.widgets.Display;
          import org.eclipse.ui.IEditorInput;
      - id: c3l42g|pOv9wK
        type: imports
        code: |-
          import org.eclipse.ui.IEditorPart;
          import org.eclipse.ui.IFileEditorInput;
          import org.eclipse.ui.IWorkbenchPage;
          import org.eclipse.ui.IWorkbenchWindow;
          import org.eclipse.ui.PlatformUI;

          import xy.ai.workbench.AgentProfile;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model.KeyPattern;
          import xy.ai.workbench.commands.AnswerCommand;
          import xy.ai.workbench.commands.CommandHandler;
          import xy.ai.workbench.commands.ExitCommand;
          import xy.ai.workbench.commands.ResumeCommand;
      - id: uzcHjn|8HwufD
        type: imports
        code: |-
          import xy.ai.workbench.connector.IAIConnector;
          import xy.ai.workbench.connector.harness.FrozenConfig;
          import xy.ai.workbench.connector.harness.Prompt;
          import xy.ai.workbench.connector.claudecode.CCRequest.Command;
          import xy.ai.workbench.models.AIAnswer;
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
          code: "@Override\n\tpublic CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Create request\", 2);\n\t\tString id = UUID.randomUUID().toString();\n\n\t\t// Preprocessing: extract Allow/Deny/exit/resume lines\n\t\tString title = null;\n\t\tCommand command = null;\n\t\tStringBuilder merged = null;\n\n\t\t{ // Preprocess\n\t\t\tsub.subTask(\"Preprocess input\");\n\t\t\tfor (Command cmd : preprocessInput(prompt.inputs))\n\t\t\t\tif (CommandType.Prompt.equals(cmd.type)) {\n\t\t\t\t\tif (title == null)\n\t\t\t\t\t\ttitle = cmd.parameter.substring(0, Math.min(100, cmd.parameter.length())).replace('\\n', ' ');\n\n\t\t\t\t\tif (merged != null)\n\t\t\t\t\t\tmerged.append(\"\\n\");\n\t\t\t\t\telse\n\t\t\t\t\t\tmerged = new StringBuilder();\n\t\t\t\t\tmerged.append(cmd.parameter);\n\t\t\t\t} else {\n\t\t\t\t\tcommand = cmd;\n\t\t\t\t\tbreak;\n\t\t\t\t}\n\t\t\tsub.worked(1);\n\t\t}\n\n\t\tif (command == null && merged != null) {\n\t\t\tsub.subTask(\"Build prompt\");\n\t\t\tcommand = new Command(CommandType.Prompt, requestBuilder.buildPromptJson(merged.toString().trim()));\n\t\t\tsub.worked(1);\n\t\t}\n\n\t\treturn new CCRequest(id, title, prompt.config, command);\n\t}"
        - id: CCConnector.executeRequest
          type: method_declaration
          code: "@Override\n\tpublic CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Executing prompt\", 2);\n\t\tCCSession session = null;\n\n\t\tEditorLocation loc = getEditorLocation();\n\t\tFrozenConfig fc = req.config;\n\t\tClaudeSessionParameters params = new ClaudeSessionParameters(loc.projectPath, fc.systemPrompt, fc.tools,\n\t\t\t\tfc.model, fc.reasoning, fc.profile, fc.cliProfile, fc.cacheMode, loc.relativeFilePath);\n\t\tparams.setTitle(req.title);\n\n\t\tswitch (req.cmd.type) {\n\t\tcase Resume:\n\t\t\tsub.subTask(\"Importing session\");\n\t\t\tsessionManager.importSession(req.cmd.parameter, params);\n\t\t\treturn new CCResponse(req.id, \"Session created\");\n\t\tcase Exit:\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t\tsub.subTask(\"Terminating CLI process\");\n\t\t\tsession.terminate();\n\t\t\treturn new CCResponse(req.id, \"Session closed!\");\n\t\tcase Allow:\n\t\tcase Deny:\n\t\tcase Modification:\n\t\t\tswitch (req.cmd.type) {\n\t\t\tcase Allow:\n\t\t\t\tcontrolClient.approve(req.cmd.parameters[0],\n\t\t\t\t\t\treq.cmd.parameters.length > 1 ? req.cmd.parameters[1] : null);\n\t\t\t\tbreak;\n\t\t\tcase Deny:\n\t\t\t\tcontrolClient.deny(req.cmd.parameters[0], req.cmd.parameters[1]);\n\t\t\t\tbreak;\n\t\t\tcase Modification:\n\t\t\t\tbreak; // allready sent\n\t\t\tdefault:\n\t\t\t\tthrow new UnsupportedOperationException();\n\t\t\t}\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t\tbreak;\n\t\tcase Prompt:\n\t\t\tsub.subTask(\"Acquiring session\");\n\t\t\tsession = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t\tbreak;\n\t\t}\n\n\t\ttry {\n\t\t\tsession.setInPrompt(true);\n\t\t\tif (CommandType.Prompt.equals(req.cmd.type)) {\n\t\t\t\tif (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !controlClient.isMCPCAvailable())\n\t\t\t\t\tthrow new IllegalStateException(\"MCPC not reachable for AgentProfile\");\n\n\t\t\t\tsub.subTask(\"Sending prompt\");\n\t\t\t\tsession.writeLine(req.cmd.parameter);\n\t\t\t}\n\n\t\t\tsub.subTask(\"Waiting for answer\");\n\t\t\treturn readUntilResult(req, session, sub.split(1), job);\n\n\t\t} catch (IOException e) {\n\t\t\tthrow new IllegalStateException(\"Claude Code CLI error\", e);\n\t\t} finally {\n\t\t\tsession.setInPrompt(false);\n\t\t}\n\t}"
        - id: CCConnector.readUntilResult
          type: method_declaration
          code: "private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressMonitor mon, Job job) throws IOException {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Reading Claude output\", 10000);\n\t\tCCResponse resp = new CCResponse(req.id);\n\n\t\tString line;\n\t\twhile (true) {\n\t\t\t// alternate read sources undtil answer\n\t\t\ttry {\n\t\t\t\t// wait 300 ms\n\t\t\t\tif ((line = session.readLine()) != null) {\n\t\t\t\t\tsession.setLastRawLine(line);\n\t\t\t\t\tjob.setName(abbreviateForStatus(line));\n\t\t\t\t\tmon.worked(1);\n\t\t\t\t\tjsonParser.parseLine(resp, session, sub, line);\n\n\t\t\t\t}\n\t\t\t} catch (Exception ex) {\n\t\t\t\tLOG.error(\"Error reading line: \", ex);\n\t\t\t\ttry {\n\t\t\t\t\twhile ((line = session.readError()) != null)\n\t\t\t\t\t\tLOG.error(\"CLI stderr: \" + line);\n\t\t\t\t} catch (Exception ex2) {\n\t\t\t\t\tLOG.error(\"Error reading stderr: \", ex2);\n\t\t\t\t}\n\t\t\t\tresp.resultText = ex.getMessage();\n\t\t\t}\n\n\t\t\tif (!resp.isReady())\n\t\t\t\tcontrolClient.checkControlEndpoint(resp);\n\n\t\t\tif (resp.isReady()) {\n\t\t\t\tsession.stats.add(resp.stats);\n\t\t\t\tsession.stats.totalToken = session.stats.inputToken + session.stats.cacheCreate;\n\t\t\t\treturn resp;\n\t\t\t}\n\t\t\tsessionManager.onSessionChanged(session);\n\t\t}\n\t}"
        - id: CCConnector.abbreviateForStatus
          type: method_declaration
          code: "private static String abbreviateForStatus(String s) {\n\t\tif (s == null)\n\t\t\treturn \"null\";\n\t\tCRC32 crc = new CRC32();\n\t\tcrc.update(s.getBytes());\n\t\treturn String.format(\"Prompting... (%d/%d)\", s.length(), crc.getValue());\n\t}"
        - id: CCConnector.preprocessInput
          type: method_declaration
          code: "private List<Command> preprocessInput(List<String> inputs) {\n\t\tList<Command> commands = new ArrayList<Command>();\n\t\tString clean;\n\t\tfor (String input : inputs)\n\t\t\tif (!(clean = input != null ? input.strip() : \"\").isBlank()) {\n\t\t\t\tvar detected = CommandHandler.detect(clean);\n\t\t\t\tif (detected instanceof ExitCommand)\n\t\t\t\t\tcommands.add(new Command(CommandType.Exit, \"\"));\n\t\t\t\telse if (detected instanceof ResumeCommand rc)\n\t\t\t\t\tcommands.add(new Command(CommandType.Resume, rc.parameter(0)));\n\t\t\t\telse if (detected instanceof AnswerCommand ac) {\n\t\t\t\t\tString id = ac.parameter(0);\n\t\t\t\t\tString action = ac.parameter(1);\n\t\t\t\t\tString reason = ac.parameter(2);\n\t\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id, reason));\n\t\t\t\t\telse\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));\n\t\t\t\t} else if (controlClient.submitEdit(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Modification, \"\"));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Prompt, clean));\n\t\t\t}\n\t\tif (commands.isEmpty())\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn commands;\n\t}"
        - id: CCConnector.convertResponse
          type: method_declaration
          code: "@Override\n\tpublic AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {\n\t\tAIAnswer answer = new AIAnswer(resp.id);\n\t\tanswer.stats.inputToken = resp.stats.inputToken;\n\t\tanswer.stats.outputToken = resp.stats.outputToken;\n\t\tanswer.stats.reasoningToken = resp.stats.reasoningToken;\n\t\tanswer.stats.cacheRead = resp.stats.cacheRead;\n\t\tanswer.stats.cacheCreate = resp.stats.cacheCreate;\n\t\tanswer.stats.totalToken = answer.stats.inputToken + answer.stats.cacheCreate + answer.stats.outputToken;\n\t\tanswer.answer = resp.resultText;\n\t\treturn answer;\n\t}"
        - id: CCConnector.getEditorLocation
          type: method_declaration
          code: "private EditorLocation getEditorLocation() {\n\t\tEditorLocation result = new EditorLocation();\n\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\ttry {\n\t\t\t\tIWorkbenchWindow window = PlatformUI.getWorkbench().getActiveWorkbenchWindow();\n\t\t\t\tif (window == null)\n\t\t\t\t\treturn;\n\t\t\t\tIWorkbenchPage page = window.getActivePage();\n\t\t\t\tif (page == null)\n\t\t\t\t\treturn;\n\t\t\t\tIEditorPart editor = page.getActiveEditor();\n\t\t\t\tif (editor == null)\n\t\t\t\t\treturn;\n\t\t\t\tIEditorInput editorInput = editor.getEditorInput();\n\t\t\t\tif (!(editorInput instanceof IFileEditorInput))\n\t\t\t\t\tthrow new IllegalArgumentException(\"Connector does not support external files\");\n\n\t\t\t\tIFileEditorInput fileInput = (IFileEditorInput) editorInput;\n\t\t\t\tIProject project = fileInput.getFile().getProject();\n\t\t\t\tPath projectPath = Paths.get(project.getLocation().toOSString());\n\t\t\t\tString relativeFilePath = fileInput.getFile().getProjectRelativePath().toString();\n\t\t\t\tresult.set(projectPath, relativeFilePath);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(\"Failed to resolve editor paths\", e);\n\t\t\t}\n\t\t});\n\t\tif (result.projectPath == null)\n\t\t\tthrow new IllegalStateException(\"Failed to resolve editor paths\");\n\t\treturn result;\n\t}"
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
      - id: KEc50E|0JLMrF
        type: imports
        code: |-
          import xy.ai.workbench.EditorInterface;
          import xy.ai.workbench.IncludeAdapter;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.batch.AIBatchManager;
          import xy.ai.workbench.commands.Command;
          import xy.ai.workbench.commands.CommandHandler;
          import xy.ai.workbench.connector.AdaptingConnector;
          import xy.ai.workbench.editor.md.AbstractRule;
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
      nodes:
      - id: Q6eIx0|8zE170
        type: imports
        code: package xy.ai.workbench.connector.claudecode;
      - id: CommandType
        type: enum_declaration
        code: "public enum CommandType {\n\tPrompt, Exit, Resume, Allow, Deny, Modification;\n}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/PromptCommand.java
      nodes:
      - id: fau272|XRuDTB
        type: imports
        code: package xy.ai.workbench.commands;
      - id: PromptCommand
        type: class_declaration
        signature: "public class PromptCommand extends Command {"
        children:
        - id: PromptCommand.PromptCommand
          type: constructor_declaration
          code: "public PromptCommand() {\n\t\tthis(\"\");\n\t}"
        - id: PromptCommand.PromptCommand_1
          type: constructor_declaration
          code: "public PromptCommand(String text) {\n\t\tsuper(text);\n\t}"
        - id: PromptCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn \"\";\n\t}"
        - id: PromptCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\treturn true;\n\t}"
        - id: PromptCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\treturn new PromptCommand(line.strip());\n\t}"
        - id: PromptCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.NONE;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
      nodes:
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java
      nodes:
      - id: ResumeCommand
        type: class_declaration
        signature: "public class ResumeCommand extends Command {"
        children:
        - id: ResumeCommand.M0PdNg|ksR6XK
          type: statements
          code: public static final String CMD_RESUME = "/resume";
        - id: ResumeCommand.ResumeCommand
          type: constructor_declaration
          code: "public ResumeCommand() {\n\t\tthis(\"\");\n\t}"
        - id: ResumeCommand.ResumeCommand_1
          type: constructor_declaration
          code: "public ResumeCommand(String sessionId) {\n\t\tsuper(sessionId);\n\t}"
        - id: ResumeCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn ResumeCommand.CMD_RESUME;\n\t}"
        - id: ResumeCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\");\n\t}"
        - id: ResumeCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ResumeCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());\n\t}"
        - id: ResumeCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
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
/answer 47e1-4d allow
Control Request:
```yaml
id: 47e1-4e
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequestBuilder.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  reason: "Basisklassen für Commands, CCRequest und Prompt verstehen, um Migration korrekt umzusetzen."
```
/answer 47e1-4e allow
Control Request:
```yaml
id: 47e1-4f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: VAgrYc|ijdZfc
      - id: CCRequest
        signature: "public class CCRequest implements IModelRequest {"
        children:
        - id: CCRequest.Tvcbr3|MUSzxw
        - id: CCRequest.CCRequest
          signature: "public CCRequest(String id, String title, FrozenConfig config, Command cmd) {"
        - id: CCRequest.getID
          signature: "public String getID() {"
        - id: CCRequest.Command
          signature: "public static class Command {"
          children:
          - id: CCRequest.Command.yQ1tid|e89M8i
          - id: CCRequest.Command.Command
            signature: "public Command(CommandType type, String... parameters) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequestBuilder.java
      nodes:
      - id: xQlkgT|KjjfaQ
      - id: CCRequestBuilder
        signature: "public class CCRequestBuilder {"
        children:
        - id: CCRequestBuilder.buildPromptJson
          signature: "public String buildPromptJson(String text) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
      nodes:
      - id: 7n1FzW|zXfU3F
      - id: T3A86v|qTHHcx
      - id: Prompt
        signature: "public class Prompt {"
        children:
        - id: Prompt.InputKind
          signature: "public enum InputKind {"
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
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
      error: Not a regular file.
```
/answer 47e1-4f allow
ReasoningToken: 309
Control Request:
```yaml
id: 47e1-50
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
    pattern: .*
    include:
    - '*.java'
    limit: 50
  reason: Alle Dateien im commands-Package auflisten.
```
/answer 47e1-50 allow
Control Request:
```yaml
id: 47e1-51
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
        - lineno: 2
          match: ""
        - lineno: 3
          match: "public class ToolCommand extends Command {"
        - lineno: 4
          match: ""
        - lineno: 5
          match: public static final String CMD_TOOL   = "/tool";
        - lineno: 6
          match: ""
        - lineno: 7
          match: "public ToolCommand() {"
        - lineno: 8
          match: this("");
        - lineno: 9
          match: "}"
        - lineno: 10
          match: ""
        - lineno: 11
          match: "public ToolCommand(String toolId) {"
        - lineno: 12
          match: super(toolId);
        - lineno: 13
          match: "}"
        - lineno: 14
          match: ""
        - lineno: 15
          match: '@Override'
        - lineno: 16
          match: "public String prefix() {"
        - lineno: 17
          match: return ToolCommand.CMD_TOOL;
        - lineno: 18
          match: "}"
        - lineno: 19
          match: ""
        - lineno: 20
          match: '@Override'
        - lineno: 21
          match: "public boolean matches(String line) {"
        - lineno: 22
          match: return line != null && line.strip().matches("(?i)" + prefix() + "\\s+\\S+");
        - lineno: 23
          match: "}"
        - lineno: 24
          match: ""
        - lineno: 25
          match: '@Override'
        - lineno: 26
          match: "public Command parse(String line) {"
        - lineno: 27
          match: "return new ToolCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());"
        - lineno: 28
          match: "}"
        - lineno: 29
          match: ""
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
        - lineno: 2
          match: ""
        - lineno: 3
          match: "public abstract class Command {"
        - lineno: 4
          match: ""
        - lineno: 5
          match: "public enum ProcessorAction {"
        - lineno: 6
          match: "REMOVE, TRANSFORM, IGNORE_BLOCK, NONE"
        - lineno: 7
          match: "}"
        - lineno: 8
          match: ""
        - lineno: 9
          match: "protected final String[] parameters;"
        - lineno: 10
          match: ""
        - lineno: 11
          match: "protected Command(String... parameters) {"
        - lineno: 12
          match: this.parameters = parameters;
        - lineno: 13
          match: "}"
        - lineno: 14
          match: ""
        - lineno: 15
          match: public abstract String prefix();
        - lineno: 16
          match: ""
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 47e1-51 allow
ReasoningToken: 28
Control Request:
```yaml
id: 47e1-52
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
  reason: Basis-Command-Klasse und CommandHandler verstehen für Migration.
```
/answer 47e1-52 allow
Control Request:
```yaml
id: 47e1-53
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
```
/answer 47e1-53 allow
Control Request:
```yaml
id: 47e1-54
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
    ids:
    - Command
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
    ids:
    - b6Xn8Z|PmHdMh
    - CommandHandler
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    ids:
    - VAgrYc|ijdZfc
    - CCRequest
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
    ids:
    - 7n1FzW|zXfU3F
    - T3A86v|qTHHcx
    - Prompt
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - CCControlClient.submitEdit
    - CCControlClient.extractYamlBlock
  reason: Vollständigen Code der Basisklassen lesen.
```
/answer 47e1-54 allow
Control Request:
```yaml
id: 47e1-55
result:
  structuredContent:
    results:
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandHandler.java
      nodes:
      - id: b6Xn8Z|PmHdMh
        type: imports
        code: |-
          package xy.ai.workbench.commands;

          import java.util.List;
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: VAgrYc|ijdZfc
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import java.util.Objects;

          import xy.ai.workbench.connector.harness.FrozenConfig;
          import xy.ai.workbench.models.IModelRequest;
      - id: CCRequest
        type: class_declaration
        signature: "public class CCRequest implements IModelRequest {"
        children:
        - id: CCRequest.Tvcbr3|MUSzxw
          type: statements
          code: "public final String id;\n\tpublic final String title;\n\n\tpublic final FrozenConfig config;\n\tpublic final Command cmd;"
        - id: CCRequest.CCRequest
          type: constructor_declaration
          code: "public CCRequest(String id, String title, FrozenConfig config, Command cmd) {\n\t\tObjects.requireNonNull(cmd, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.title = title;\n\t\tthis.config = config;\n\t\tthis.cmd = cmd;\n\t}"
        - id: CCRequest.getID
          type: method_declaration
          code: "@Override\n\tpublic String getID() {\n\t\treturn id;\n\t}"
        - id: CCRequest.Command
          type: class_declaration
          signature: "public static class Command {"
          children:
          - id: CCRequest.Command.yQ1tid|e89M8i
            type: statements
            code: "public final CommandType type;\n\t\tpublic final String parameter;\n\t\tpublic final String[] parameters;"
          - id: CCRequest.Command.Command
            type: constructor_declaration
            code: "public Command(CommandType type, String... parameters) {\n\t\t\tObjects.requireNonNull(type, \"Type can't be null\");\n\t\t\tthis.type = type;\n\t\t\tthis.parameter = parameters[0];\n\t\t\tthis.parameters = parameters;\n\t\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
      nodes:
      - id: 7n1FzW|zXfU3F
        type: imports
        code: |-
          package xy.ai.workbench.connector.harness;

          import java.nio.charset.StandardCharsets;
          import java.nio.file.Path;
          import java.security.MessageDigest;
          import java.security.NoSuchAlgorithmException;
          import java.util.List;

          import xy.ai.workbench.commands.Command;
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
        - id: Prompt.uUxVT7|GekI8D
          type: statements
          code: "public final List<String> inputs;\n\tpublic final InputKind kind;\n\tpublic final boolean batch;\n\tpublic final FrozenConfig config;\n\tpublic final boolean processorEnabled;\n\tpublic final String absoluteFilePath;\n\tpublic final Command command;\n\tpublic final String yamlBlock;\n\n\tprivate String sessionId;"
        - id: Prompt.Prompt
          type: constructor_declaration
          code: "private Prompt(List<String> inputs, InputKind kind, boolean batch, FrozenConfig config, boolean processorEnabled,\n\t\t\tString absoluteFilePath, Command command, String yamlBlock) {\n\t\tif (config == null)\n\t\t\tthrow new IllegalArgumentException(\"config must not be null\");\n\t\tif (absoluteFilePath == null || absoluteFilePath.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"absoluteFilePath must not be blank\");\n\t\tthis.inputs = inputs != null ? List.copyOf(inputs) : List.of();\n\t\tthis.kind = kind;\n\t\tthis.batch = batch;\n\t\tthis.config = config;\n\t\tthis.processorEnabled = processorEnabled;\n\t\tthis.absoluteFilePath = absoluteFilePath;\n\t\tthis.command = command;\n\t\tthis.yamlBlock = yamlBlock;\n\t}"
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: CCControlClient.submitEdit
        type: method_declaration
        code: "public boolean submitEdit(String rawText) {\n\t\tif (rawText == null)\n\t\t\treturn false;\n\t\tString block = extractYamlBlock(rawText.strip());\n\t\tif (block == null)\n\t\t\treturn false;\n\n\t\tJsonNode edited;\n\t\ttry {\n\t\t\tedited = fromYaml(block);\n\t\t} catch (Exception e) {\n\t\t\tthrow new IllegalArgumentException(\"YAML Error\", e);\n\t\t}\n\t\tif (edited == null || !edited.isObject() || !edited.hasNonNull(\"id\"))\n\t\t\treturn false;\n\t\tsubmit((ObjectNode) edited);\n\t\treturn true;\n\t}"
      - id: CCControlClient.extractYamlBlock
        type: method_declaration
        code: "private String extractYamlBlock(String text) {\n\t\tint start = text.indexOf(\"```yaml\");\n\t\tif (start != 0)\n\t\t\treturn null;\n\t\tint contentStart = start + \"```yaml\".length();\n\t\tint end = text.indexOf(\"```\", contentStart);\n\t\tif (end == -1)\n\t\t\treturn null;\n\t\treturn text.substring(contentStart, end).strip();\n\t}"
```
/answer 47e1-55 allow
Control Request:
```yaml
id: 47e1-56
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: \.command\b|\.yamlBlock\b|prompt\.command|prompt\.yamlBlock
    include:
    - '*.java'
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: ModificationCommand|ControlRequestCommand|CallCommand
    include:
    - '*.java'
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: CommandType|preprocessInput|CCRequest\.Command
    include:
    - '*.java'
    limit: 40
  reason: "Verwendungsstellen von Prompt.command/yamlBlock, ModificationCommand, ControlRequestCommand, CallCommand und CommandType finden, um Abhängigkeiten zu klären."
```
/answer 47e1-56 allow
Control Request:
```yaml
id: 47e1-57
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/mcp/MCPRequest.java
        matches:
        - lineno: 22
          match: this.command = command;
        - lineno: 23
          match: this.yamlBlock = yamlBlock;
      - path: xy/ai/workbench/connector/mcp/MCPConnector.java
        matches:
        - lineno: 39
          match: Command command = prompt.command;
        - lineno: 45
          match: prompt.yamlBlock);
        - lineno: 50
          match: "if (req.command instanceof ExitCommand) {"
        - lineno: 54
          match: "if (req.command instanceof ToolCommand tc) {"
        - lineno: 60
          match: "if (req.command instanceof CallCommand) {"
        - lineno: 61
          match: if (req.yamlBlock == null || req.yamlBlock.isBlank())
        - lineno: 63
          match: "return invokeCall(req.id, control.parseYaml(req.yamlBlock));"
        - lineno: 65
          match: "throw new UnsupportedOperationException(\"Unsupported command: \" + req.command.prefix());"
      - path: xy/ai/workbench/connector/harness/PromptHandler.java
        matches:
        - lineno: 296
          match: "builder.command(detected[0]).yamlBlock(yamlBlock[0]);"
      - path: xy/ai/workbench/connector/harness/Prompt.java
        matches:
        - lineno: 47
          match: this.command = command;
        - lineno: 48
          match: this.yamlBlock = yamlBlock;
        - lineno: 140
          match: this.command = command;
        - lineno: 145
          match: this.yamlBlock = yamlBlock;
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/mcp/MCPControlClient.java
        matches:
        - lineno: 13
          match: import xy.ai.workbench.commands.CallCommand;
        - lineno: 56
          match: sb.append(CallCommand.CMD_CALL);
      - path: xy/ai/workbench/connector/mcp/MCPConnector.java
        matches:
        - lineno: 12
          match: import xy.ai.workbench.commands.CallCommand;
        - lineno: 41
          match: '&& !(command instanceof CallCommand))'
        - lineno: 60
          match: "if (req.command instanceof CallCommand) {"
      - path: xy/ai/workbench/commands/ModificationCommand.java
        matches:
        - lineno: 3
          match: "public class ModificationCommand extends Command {"
        - lineno: 5
          match: "public ModificationCommand() {"
        - lineno: 21
          match: return new ModificationCommand();
      - path: xy/ai/workbench/commands/CallCommand.java
        matches:
        - lineno: 3
          match: "public class CallCommand extends Command {"
        - lineno: 7
          match: "public CallCommand() {"
        - lineno: 13
          match: return CallCommand.CMD_CALL;
        - lineno: 23
          match: return new CallCommand();
      - path: xy/ai/workbench/commands/ControlRequestCommand.java
        matches:
        - lineno: 5
          match: "public class ControlRequestCommand extends Command {"
        - lineno: 7
          match: "public ControlRequestCommand() {"
        - lineno: 18
          match: return new ControlRequestCommand();
      - path: xy/ai/workbench/commands/CommandHandler.java
        matches:
        - lineno: 8
          match: "private static final List<Command> REGISTRY = List.of(new AnswerCommand(), new ControlRequestCommand(),"
        - lineno: 9
          match: "new ResumeCommand(), new ExitCommand(), new ToolCommand(), new CallCommand());"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/claudecode/CommandType.java
        matches:
        - lineno: 3
          match: "public enum CommandType {"
      - path: xy/ai/workbench/connector/claudecode/CCConnector.java
        matches:
        - lineno: 33
          match: import xy.ai.workbench.connector.claudecode.CCRequest.Command;
        - lineno: 64
          match: "for (Command cmd : preprocessInput(prompt.inputs))"
        - lineno: 65
          match: "if (CommandType.Prompt.equals(cmd.type)) {"
        - lineno: 83
          match: "command = new Command(CommandType.Prompt, requestBuilder.buildPromptJson(merged.toString().trim()));"
        - lineno: 137
          match: "if (CommandType.Prompt.equals(req.cmd.type)) {"
        - lineno: 202
          match: "private List<Command> preprocessInput(List<String> inputs) {"
        - lineno: 209
          match: "commands.add(new Command(CommandType.Exit, \"\"));"
        - lineno: 211
          match: "commands.add(new Command(CommandType.Resume, rc.parameter(0)));"
        - lineno: 217
          match: "commands.add(new Command(CommandType.Allow, id, reason));"
        - lineno: 219
          match: "commands.add(new Command(CommandType.Deny, id, reason));"
        - lineno: 221
          match: "commands.add(new Command(CommandType.Modification, \"\"));"
        - lineno: 223
          match: "commands.add(new Command(CommandType.Prompt, clean));"
      - path: xy/ai/workbench/connector/claudecode/CCRequest.java
        matches:
        - lineno: 30
          match: public final CommandType type;
        - lineno: 34
          match: "public Command(CommandType type, String... parameters) {"
```
/answer 47e1-57 allow
ReasoningToken: 877
Control Request:
```yaml
id: 47e1-58
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - MCPConnector
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    ids:
    - MCPRequest
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
    ids:
    - CallCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ControlRequestCommand.java
    ids:
    - ControlRequestCommand
  reason: MCPConnector als Referenzmuster für die Migration von Command-basierter Verarbeitung studieren.
```
/answer 47e1-58 allow
Control Request:
```yaml
id: 47e1-59
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ControlRequestCommand.java
      nodes:
      - id: ControlRequestCommand
        type: class_declaration
        signature: "public class ControlRequestCommand extends Command {"
        children:
        - id: ControlRequestCommand.ControlRequestCommand
          type: constructor_declaration
          code: "public ControlRequestCommand() {\n\t\tsuper();\n\t}"
        - id: ControlRequestCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn EditorInterface.CONTROL_REQUEST;\n\t}"
        - id: ControlRequestCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ControlRequestCommand();\n\t}"
        - id: ControlRequestCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.IGNORE_BLOCK;\n\t}"
```
/answer 47e1-59 allow
ReasoningToken: 141
Control Request:
```yaml
id: 47e1-5a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequestBuilder.java
    ids:
    - xQlkgT|KjjfaQ
    - CCRequestBuilder
  reason: buildPromptJson-Implementierung prüfen für Titel-Extraktionsreihenfolge.
```
/answer 47e1-5a allow
Control Request:
```yaml
id: 47e1-5b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequestBuilder.java
      nodes:
      - id: xQlkgT|KjjfaQ
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import com.fasterxml.jackson.databind.node.ArrayNode;
          import com.fasterxml.jackson.databind.node.ObjectNode;
      - id: CCRequestBuilder
        type: class_declaration
        signature: "public class CCRequestBuilder {"
        children:
        - id: CCRequestBuilder.buildPromptJson
          type: method_declaration
          code: "public String buildPromptJson(String text) {\n\t\ttry {\n\t\t\tObjectNode root = JsonUtil.mapper().createObjectNode();\n\t\t\troot.put(\"type\", \"user\");\n\t\t\tObjectNode message = root.putObject(\"message\");\n\t\t\tmessage.put(\"role\", \"user\");\n\t\t\tArrayNode content = message.putArray(\"content\");\n\t\t\tObjectNode textNode = content.addObject();\n\t\t\ttextNode.put(\"type\", \"text\");\n\t\t\ttextNode.put(\"text\", text);\n\t\t\treturn JsonUtil.mapper().writeValueAsString(root);\n\t\t} catch (Exception e) {\n\t\t\tthrow new IllegalStateException(\"Failed to build prompt JSON\", e);\n\t\t}\n\t}"
```
/answer 47e1-5b allow
ReasoningToken: 8041
Control Request:
```yaml
id: 47e1-5c
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: processorAction\(\)|ProcessorAction\.
    include:
    - '*.java'
    limit: 40
  reason: "Verstehen, wie ProcessorAction die Prompt-Inputs vor der Connector-Verarbeitung bereinigt."
```
/answer 47e1-5c allow
Control Request:
```yaml
id: 47e1-5d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/harness/SessionProcessor.java
        matches:
        - lineno: 113
          match: "switch (command.processorAction()) {"
      - path: xy/ai/workbench/commands/ToolCommand.java
        matches:
        - lineno: 31
          match: "public ProcessorAction processorAction() {"
        - lineno: 32
          match: return ProcessorAction.REMOVE;
      - path: xy/ai/workbench/commands/Command.java
        matches:
        - lineno: 23
          match: public abstract ProcessorAction processorAction();
      - path: xy/ai/workbench/commands/AnswerCommand.java
        matches:
        - lineno: 36
          match: "public ProcessorAction processorAction() {"
        - lineno: 37
          match: return ProcessorAction.TRANSFORM;
      - path: xy/ai/workbench/commands/PromptCommand.java
        matches:
        - lineno: 29
          match: "public ProcessorAction processorAction() {"
        - lineno: 30
          match: return ProcessorAction.NONE;
      - path: xy/ai/workbench/commands/ModificationCommand.java
        matches:
        - lineno: 25
          match: "public ProcessorAction processorAction() {"
        - lineno: 26
          match: return ProcessorAction.NONE;
      - path: xy/ai/workbench/commands/ExitCommand.java
        matches:
        - lineno: 27
          match: "public ProcessorAction processorAction() {"
        - lineno: 28
          match: return ProcessorAction.REMOVE;
      - path: xy/ai/workbench/commands/CallCommand.java
        matches:
        - lineno: 27
          match: "public ProcessorAction processorAction() {"
        - lineno: 28
          match: return ProcessorAction.REMOVE;
      - path: xy/ai/workbench/commands/ResumeCommand.java
        matches:
        - lineno: 31
          match: "public ProcessorAction processorAction() {"
        - lineno: 32
          match: return ProcessorAction.REMOVE;
      - path: xy/ai/workbench/commands/ControlRequestCommand.java
        matches:
        - lineno: 22
          match: "public ProcessorAction processorAction() {"
        - lineno: 23
          match: return ProcessorAction.IGNORE_BLOCK;
```
/answer 47e1-5d allow
ReasoningToken: 4176
Control Request:
```yaml
id: 47e1-5e
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: new AnswerCommand\(
    include:
    - '*.java'
    limit: 20
  reason: Alle Konstruktionsaufrufe von AnswerCommand prüfen für konsistente Groß-/Kleinschreibung der action.
```
/answer 47e1-5e allow
Control Request:
```yaml
id: 47e1-5f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/commands/AnswerCommand.java
        matches:
        - lineno: 32
          match: "return new AnswerCommand(id, action, reason);"
      - path: xy/ai/workbench/commands/CommandHandler.java
        matches:
        - lineno: 8
          match: "private static final List<Command> REGISTRY = List.of(new AnswerCommand(), new ControlRequestCommand(),"
```
/answer 47e1-5f allow
ReasoningToken: 8376
Control Request:
```yaml
id: 47e1-60
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/PromptCommand.java
    id: PromptCommand.processorAction
    old_text: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.NONE;\n\t}"
    new_text: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.NONE;\n\t}\n\n\t/** First 100 chars of the prompt text, single-lined; used as session/title label. */\n\tpublic String title() {\n\t\tString text = parameter(0);\n\t\treturn text.substring(0, Math.min(100, text.length())).replace('\\n', ' ');\n\t}"
  reason: Titelermittlung aus CCConnector in PromptCommand verschieben.
```
/answer 47e1-60 allow
ReasoningToken: 180
Control Request:
```yaml
id: 47e1-61
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
    id: AnswerCommand.processorAction
    old_text: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.TRANSFORM;\n\t}"
    new_text: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.TRANSFORM;\n\t}\n\n\tpublic enum Action {\n\t\tAllow, Deny\n\t}\n\n\tpublic Action action() {\n\t\treturn \"allow\".equalsIgnoreCase(parameter(1)) ? Action.Allow : Action.Deny;\n\t}"
  reason: Allow/Deny als interne Enum in AnswerCommand ergänzen.
```
/answer 47e1-61 allow
Text:
 Jetzt CCRequest.java umbauen:
Control Request:
```yaml
id: 47e1-62
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    id: VAgrYc|ijdZfc
    source: |-
      package xy.ai.workbench.connector.claudecode;

      import java.util.Objects;

      import xy.ai.workbench.commands.Command;
      import xy.ai.workbench.connector.harness.FrozenConfig;
      import xy.ai.workbench.models.IModelRequest;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    id: CCRequest
    source: "public class CCRequest implements IModelRequest {\n\n\tpublic final String id;\n\tpublic final String title;\n\n\tpublic final FrozenConfig config;\n\tpublic final Command command;\n\n\tpublic CCRequest(String id, String title, FrozenConfig config, Command command) {\n\t\tObjects.requireNonNull(command, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.title = title;\n\t\tthis.config = config;\n\t\tthis.command = command;\n\t}\n\n\t@Override\n\tpublic String getID() {\n\t\treturn id;\n\t}\n}"
  reason: CCRequest von eigener CommandType/Command-Kapselung auf das geteilte Command-System umstellen.
```
/answer 47e1-62 allow
Control Request:
```yaml
id: 47e1-63
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
  reason: "CommandType wird nicht mehr benötigt, da CCConnector das geteilte Command-System aus xy.ai.workbench.commands verwendet."
```
/answer 47e1-63 allow
Text:
 Jetzt CommandType.java löschen und CCConnector.java umbauen:
Control Request:
```yaml
id: 47e1-64
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: c3l42g|pOv9wK
    source: |-
      import org.eclipse.ui.IEditorPart;
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
      import xy.ai.workbench.commands.ResumeCommand;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: uzcHjn|8HwufD
    source: |-
      import xy.ai.workbench.connector.IAIConnector;
      import xy.ai.workbench.connector.harness.FrozenConfig;
      import xy.ai.workbench.connector.harness.Prompt;
      import xy.ai.workbench.models.AIAnswer;
  reason: "Imports von CCConnector auf das geteilte Command-System umstellen, CCRequest.Command und CommandHandler-Import entfernen."
```
/answer 47e1-64 allow
Control Request:
```yaml
id: 47e1-65
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: OsZtmS|YZNdWp
    old_text: |-
      import java.nio.file.Paths;
      import java.util.ArrayList;
      import java.util.List;
      import java.util.UUID;
    new_text: |-
      import java.nio.file.Paths;
      import java.util.UUID;
      import java.util.stream.Collectors;
  reason: Nicht mehr benötigte Imports entfernen und Collectors für die Text-Zusammenführung ergänzen.
```
/answer 47e1-65 allow
ReasoningToken: 39
ReasoningToken: 83
Text:
 Jetzt `createRequest`, `executeRequest` neu schreiben und `preprocessInput` löschen:
Control Request:
```yaml
id: 47e1-66
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.createRequest
    source: "@Override\n\tpublic CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Create request\", 1);\n\t\tsub.subTask(\"Detect command\");\n\n\t\t// prompt.command already prioritized/localized by PromptHandler (line/block/file).\n\t\tCommand command = prompt.command;\n\t\tif (command == null) {\n\t\t\tString text = prompt.inputs.stream().map(String::strip).filter(s -> !s.isBlank())\n\t\t\t\t\t.collect(Collectors.joining(\"\\n\"));\n\t\t\tif (controlClient.submitEdit(text))\n\t\t\t\tcommand = new ModificationCommand();\n\t\t\telse if (!text.isEmpty())\n\t\t\t\tcommand = new PromptCommand(text);\n\t\t}\n\t\tif (command == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\tsub.worked(1);\n\n\t\tString title = command instanceof PromptCommand pc ? pc.title() : null;\n\t\treturn new CCRequest(UUID.randomUUID().toString(), title, prompt.config, command);\n\t}"
  reason: "createRequest auf das geteilte Command-System von PromptHandler/commands-Paket umstellen, eigenes Parsing entfernen."
```
/answer 47e1-66 allow
Control Request:
```yaml
id: 47e1-67
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    source: "@Override\n\tpublic CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Executing prompt\", 2);\n\t\tCCSession session = null;\n\n\t\tEditorLocation loc = getEditorLocation();\n\t\tFrozenConfig fc = req.config;\n\t\tClaudeSessionParameters params = new ClaudeSessionParameters(loc.projectPath, fc.systemPrompt, fc.tools,\n\t\t\t\tfc.model, fc.reasoning, fc.profile, fc.cliProfile, fc.cacheMode, loc.relativeFilePath);\n\t\tparams.setTitle(req.title);\n\n\t\tCommand command = req.command;\n\t\tif (command instanceof ResumeCommand rc) {\n\t\t\tsub.subTask(\"Importing session\");\n\t\t\tsessionManager.importSession(rc.parameter(0), params);\n\t\t\treturn new CCResponse(req.id, \"Session created\");\n\t\t}\n\t\tif (command instanceof ExitCommand) {\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t\tsub.subTask(\"Terminating CLI process\");\n\t\t\tsession.terminate();\n\t\t\treturn new CCResponse(req.id, \"Session closed!\");\n\t\t}\n\t\tif (command instanceof AnswerCommand ac) {\n\t\t\tif (ac.action() == AnswerCommand.Action.Allow)\n\t\t\t\tcontrolClient.approve(ac.parameter(0), ac.parameter(2));\n\t\t\telse\n\t\t\t\tcontrolClient.deny(ac.parameter(0), ac.parameter(2));\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t} else if (command instanceof ModificationCommand) {\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params); // already sent\n\t\t} else if (command instanceof PromptCommand) {\n\t\t\tsub.subTask(\"Acquiring session\");\n\t\t\tsession = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t} else {\n\t\t\tthrow new UnsupportedOperationException(\"Unsupported command: \" + command.prefix());\n\t\t}\n\n\t\ttry {\n\t\t\tsession.setInPrompt(true);\n\t\t\tif (command instanceof PromptCommand pc) {\n\t\t\t\tif (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !controlClient.isMCPCAvailable())\n\t\t\t\t\tthrow new IllegalStateException(\"MCPC not reachable for AgentProfile\");\n\n\t\t\t\tsub.subTask(\"Sending prompt\");\n\t\t\t\tsession.writeLine(requestBuilder.buildPromptJson(pc.parameter(0)));\n\t\t\t}\n\n\t\t\tsub.subTask(\"Waiting for answer\");\n\t\t\treturn readUntilResult(req, session, sub.split(1), job);\n\n\t\t} catch (IOException e) {\n\t\t\tthrow new IllegalStateException(\"Claude Code CLI error\", e);\n\t\t} finally {\n\t\t\tsession.setInPrompt(false);\n\t\t}\n\t}"
  reason: executeRequest auf instanceof-Dispatch der geteilten Command-Typen umstellen statt CommandType-Enum.
```
/answer 47e1-67 allow
Control Request:
```yaml
id: 47e1-68
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.preprocessInput
  reason: preprocessInput ist ins geteilte Command-System der PromptHandler-Logik verschoben und in CCConnector nicht mehr nötig.
```
/answer 47e1-68 allow
Control Request:
```yaml
id: 47e1-69
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/PromptCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
  reason: "Sicherstellen, dass alle geänderten Java-Dateien syntaktisch korrekt sind."
```
/answer 47e1-69 allow
Control Request:
```yaml
id: 47e1-6a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - CCConnector
  reason: Endgültigen Code von CCConnector zur Kontrolle vollständig lesen.
```
/answer 47e1-6a allow
Control Request:
```yaml
id: 47e1-6b
result:
  structuredContent:
    results:
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
          code: "@Override\n\tpublic CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Create request\", 1);\n\t\tsub.subTask(\"Detect command\");\n\n\t\t// prompt.command already prioritized/localized by PromptHandler (line/block/file).\n\t\tCommand command = prompt.command;\n\t\tif (command == null) {\n\t\t\tString text = prompt.inputs.stream().map(String::strip).filter(s -> !s.isBlank())\n\t\t\t\t\t.collect(Collectors.joining(\"\\n\"));\n\t\t\tif (controlClient.submitEdit(text))\n\t\t\t\tcommand = new ModificationCommand();\n\t\t\telse if (!text.isEmpty())\n\t\t\t\tcommand = new PromptCommand(text);\n\t\t}\n\t\tif (command == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\tsub.worked(1);\n\n\t\tString title = command instanceof PromptCommand pc ? pc.title() : null;\n\t\treturn new CCRequest(UUID.randomUUID().toString(), title, prompt.config, command);\n\t}"
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
/answer 47e1-6b allow
Text:
 Nun `preprocessInput` entfernen:
Control Request:
```yaml
id: 47e1-6c
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ModificationCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  reason: "Sicherstellen, dass die unveränderten, aber logisch verbundenen Dateien weiterhin gültig sind."
```
/answer 47e1-6c allow
Text:
 Jetzt validieren und PromptHandler auf Modification-Block-Erkennung prüfen. Zunächst Validierung aller betroffenen Dateien:
ReasoningToken: 134
Text:
 Sieht korrekt aus. Nun teste ich, ob `PromptHandler.java` und die commands-Dateien noch korrekt sind (keine Änderungen dort nötig, aber prüfen wir zur Sicherheit die restlichen Dateien im commands-Paket):
Result Stats:  id=fb96ed27-5e56-43b6-8335-4c0e2eeeed43, total: 0, in: 179, out: 32660, reason: 0, read: 1303884, write: 83630
Alle Dateien sind syntaktisch korrekt. Migration abgeschlossen:

- **CCConnector**: `preprocessInput` entfernt; `createRequest`/`executeRequest` nutzen jetzt `prompt.command` (von `PromptHandler` priorisiert/lokalisiert) sowie `instanceof`-Dispatch auf die geteilten Command-Typen statt eigenem Enum-Parsing.
- **CommandType.java**: gelöscht.
- **CCRequest**: eigene innere `Command`/`CommandType`-Kapselung entfernt, nutzt jetzt `xy.ai.workbench.commands.Command`.
- **PromptCommand**: `title()`-Methode ergänzt (Titel-Ermittlung aus CCConnector übernommen).
- **AnswerCommand**: interne Enum `Action { Allow, Deny }` plus `action()` ergänzt.
