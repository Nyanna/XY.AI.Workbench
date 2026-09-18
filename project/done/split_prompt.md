Der `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java` soll in seine zwei Apekte aufgeteilt werden.
1. Koordination und Ausführung soll im PromptHandler bleiben.
2. Input Handling, Aggregation und Preprocessing (Kommandosubstitution) -> soll ausgelagert werden nach "PromptInputHandler"
SystemInit:  id=83bf3270-9b48-4f1b-9fae-477ea1238bbd, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 21e8-12
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: "Struktur der Klasse analysieren, um Aufteilung zu planen"
```
/answer 21e8-12 allow
Control Request:
```yaml
id: 21e8-13
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: jyVQdv|I3qhgN
        signature: package xy.ai.workbench.connector.harness;
      - id: GEmxmM|8HTEOr
        signature: import org.eclipse.core.resources.IProject;
      - id: DaYIWN|qhXhJ9
        signature: import org.eclipse.ui.texteditor.ITextEditor;
      - id: KEc50E|MeSef9
        signature: import xy.ai.workbench.commands.CommandRegistry;
      - id: fPHG2Y|URMgPQ
        signature: /**
      - id: PromptHandler
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.ljM8Ja|2m2HTZ
          signature: // Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block
        - id: PromptHandler.LSyBuh|FgyeYv
          signature: "private int[] inputStats = new int[InputMode.values().length];"
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEdit…"
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
            signature: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, IProgressMonitor mon) {"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.o8lrZ3|vOa7RL
          signature: /**
        - id: PromptHandler.buildPrompt
          signature: "private Prompt buildPrompt(Display display, boolean batch) {"
        - id: PromptHandler.ohnvjH|PwL7la
          signature: "// Selection mode: a real (multi-char) selection is a BlockSelection, an"
        - id: PromptHandler.detectSelection
          signature: "private void detectSelection(ITextEditor textEditor, List<String> inputs, Comma…"
        - id: PromptHandler.Txg3oX|CbRnfT
          signature: "// Processor (full file) mode: the caret line is checked first, then the last"
        - id: PromptHandler.detectFullFile
          signature: "private void detectFullFile(ITextEditor textEditor, List<String> inputs, Comman…"
        - id: PromptHandler.applyDetected
          signature: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Comman…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.WEwxAI|MZVZI2
          signature: // Walks backwards from the command line and keeps the closest preceding ```yaml
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.resolveProjectPath
          signature: "private Path resolveProjectPath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
```
/answer 21e8-13 allow
ReasoningToken: 11
Control Request:
```yaml
id: 21e8-14
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - PromptHandler
    - jyVQdv|I3qhgN
    - GEmxmM|8HTEOr
    - DaYIWN|qhXhJ9
    - KEc50E|MeSef9
    - fPHG2Y|URMgPQ
  reason: "Vollständigen Quellcode der Klasse und Imports lesen, um die Aufteilung durchzuführen"
```
/answer 21e8-14 allow
Control Request:
```yaml
id: 21e8-15
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler
        type: class_declaration
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.ljM8Ja|2m2HTZ
          type: statements
          code: "// Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block\n\t// before a command line.\n\tprivate static final Pattern YAML_BLOCK = Pattern.compile(\"^```yaml\\\\R(.*?)^```$\",\n\t\t\tPattern.MULTILINE | Pattern.DOTALL);\n\n\tprivate final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final EditorInterface editIfc;\n\tprivate final IncludeAdapter includeAdapter;\n\tprivate final AIBatchManager batch;"
        - id: PromptHandler.LSyBuh|FgyeYv
          type: statements
          code: "private int[] inputStats = new int[InputMode.values().length];\n\tprivate List<Consumer<AIAnswer>> answerObs = new ArrayList<>();\n\tprivate List<Consumer<int[]>> inputStatObs = new ArrayList<>();"
        - id: PromptHandler.PromptHandler
          type: constructor_declaration
          code: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEditorListener editorListener,\n\t\t\tEditorInterface editIfc, IncludeAdapter includeAdapter) {\n\t\tthis.cfg = cfg;\n\t\tthis.connector = connector;\n\t\tthis.editorListener = editorListener;\n\t\tthis.editIfc = editIfc;\n\t\tthis.includeAdapter = includeAdapter;\n\t\tthis.batch = Activator.getDefault().batch;\n\t}"
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
          code: "public void queueAsync(Display display) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.queueSync
          type: method_declaration
          code: "private void queueSync(Display display, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Enqueue batch prompt\", 3);\n\t\tsub.subTask(\"Prepare inputs\");\n\t\tvar req = prepareRequest(display, true, sub.split(1));\n\t\tsub.subTask(\"Insert Tag\");\n\t\teditIfc.insertTag(display, req, sub.split(1));\n\t\tsub.subTask(\"Enqueue prompt\");\n\t\tbatch.enqueue(req, sub.split(1));\n\t}"
        - id: PromptHandler.queueAndSubmit
          type: method_declaration
          code: "public void queueAndSubmit(Display display) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, mon);\n\t\t\t\tbatch.submitBatches(mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.prepareRequest
          type: method_declaration
          code: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Preparing Call\", 1);\n\t\tsub.subTask(\"Preparing Call\");\n\t\tPrompt prompt = buildPrompt(display, batchFix);\n\t\tIModelRequest req = connector.createRequest(prompt, sub);\n\t\tsub.worked(1);\n\t\treturn req;\n\t}"
        - id: PromptHandler.o8lrZ3|vOa7RL
          type: statements
          code: "/**\n\t * Freezes the current editor/config state into a {@link Prompt}; runs command\n\t * detection (see class doc).\n\t */"
        - id: PromptHandler.buildPrompt
          type: method_declaration
          code: "private Prompt buildPrompt(Display display, boolean batch) {\n\t\tList<String> inputs = new ArrayList<>();\n\t\tString[] absoluteFilePath = new String[1];\n\t\tPath[] projectPath = new Path[1];\n\t\tCommand[] detected = new Command[1];\n\t\tString[] yamlBlock = new String[1];\n\n\t\tdisplay.syncExec(() -> {\n\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\tabsoluteFilePath[0] = resolveAbsoluteFilePath(textEditor);\n\t\t\tprojectPath[0] = resolveProjectPath(textEditor);\n\n\t\t\tif (cfg.isInputEnabled(InputMode.Selection))\n\t\t\t\tdetectSelection(textEditor, inputs, detected, yamlBlock);\n\t\t\telse if (cfg.isInputEnabled(InputMode.Converter))\n\t\t\t\tdetectFullFile(textEditor, inputs, detected, yamlBlock);\n\t\t});\n\n\t\tFrozenConfig frozen = FrozenConfig.from(cfg);\n\n\t\tif (inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank()))\n\t\t\tthrow new IllegalArgumentException(\"Input and System Prompt Empty\");\n\n\t\tif (editorListener.getLastTextEditor() == null && !batch)\n\t\t\tthrow new IllegalArgumentException(\"Result editor unset\");\n\n\t\treturn new Prompt(inputs, batch, frozen, cfg.isInputEnabled(InputMode.Converter), absoluteFilePath[0],\n\t\t\t\tprojectPath[0], detected[0], yamlBlock[0]);\n\t}"
        - id: PromptHandler.ohnvjH|PwL7la
          type: statements
          code: "// Selection mode: a real (multi-char) selection is a BlockSelection, an\n\t// empty/caret selection\n\t// falls back to the current cursor line (LineInput)."
        - id: PromptHandler.detectSelection
          type: method_declaration
          code: "private void detectSelection(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn;\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn;\n\n\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1) {\n\t\t\t// A selection consisting solely of a ```yaml block is itself a command\n\t\t\t// (CallEditCommand).\n\t\t\tCommand block = CommandRegistry.detect(tsel.getText());\n\t\t\tif (block instanceof CallEditCommand) {\n\t\t\t\tdetected[0] = block;\n\t\t\t\treturn;\n\t\t\t}\n\n\t\t\tinputs.add(removeCommentLines(tsel.getText()));\n\t\t\t// An /answer command starting at the very beginning of the block spans the\n\t\t\t// whole\n\t\t\t// selection, allowing a multi-line (better formatted) reason/hint for allow and\n\t\t\t// deny alike.\n\t\t\tif (block instanceof AnswerCommand) {\n\t\t\t\tdetected[0] = block;\n\t\t\t\tyamlBlock[0] = captureYamlBlock(doc, tsel.getStartLine());\n\t\t\t}\n\t\t\t// Block start (multi-line command) takes precedence over a trailing command\n\t\t\t// line.\n\t\t\telse if (!applyDetected(CommandRegistry.detect(lineText(doc, tsel.getStartLine())), doc,\n\t\t\t\t\ttsel.getStartLine(), detected, yamlBlock))\n\t\t\t\tapplyDetected(CommandRegistry.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(),\n\t\t\t\t\t\tdetected, yamlBlock);\n\t\t\treturn;\n\t\t}\n\n\t\tif (tsel != null) {\n\t\t\ttry {\n\t\t\t\tIRegion lineInfo = doc.getLineInformation(tsel.getEndLine());\n\t\t\t\tinputs.add(doc.get(lineInfo.getOffset(), lineInfo.getLength()));\n\t\t\t} catch (BadLocationException e) {\n\t\t\t\tLOG.error(\"Exception\", e);\n\t\t\t}\n\t\t\tapplyDetected(CommandRegistry.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(), detected,\n\t\t\t\t\tyamlBlock);\n\t\t}\n\t}"
        - id: PromptHandler.Txg3oX|CbRnfT
          type: statements
          code: "// Processor (full file) mode: the caret line is checked first, then the last\n\t// line of the file,\n\t// matching a command appended after the generated content."
        - id: PromptHandler.detectFullFile
          type: method_declaration
          code: "private void detectFullFile(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn;\n\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn;\n\t\tinputs.add(doc.get());\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tif (tsel != null && applyDetected(CommandRegistry.detect(lineText(doc, tsel.getEndLine())), doc,\n\t\t\t\ttsel.getEndLine(), detected, yamlBlock))\n\t\t\treturn;\n\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\tapplyDetected(CommandRegistry.detect(lineText(doc, lastLine)), doc, lastLine, detected, yamlBlock);\n\t\treturn;\n\t}"
        - id: PromptHandler.applyDetected
          type: method_declaration
          code: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Command[] detected, String[] yamlBlock) {\n\t\tif (cmd == null)\n\t\t\treturn false;\n\t\tdetected[0] = cmd;\n\t\tyamlBlock[0] = captureYamlBlock(doc, lineIndex);\n\t\treturn true;\n\t}"
        - id: PromptHandler.lineText
          type: method_declaration
          code: "private String lineText(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tIRegion info = doc.getLineInformation(lineIndex);\n\t\t\treturn doc.get(info.getOffset(), info.getLength());\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn \"\";\n\t\t}\n\t}"
        - id: PromptHandler.WEwxAI|MZVZI2
          type: statements
          code: "// Walks backwards from the command line and keeps the closest preceding ```yaml\n\t// block."
        - id: PromptHandler.captureYamlBlock
          type: method_declaration
          code: "private String captureYamlBlock(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tString prefix = doc.get(0, doc.getLineOffset(lineIndex));\n\t\t\tMatcher m = YAML_BLOCK.matcher(prefix);\n\t\t\tString last = null;\n\t\t\twhile (m.find())\n\t\t\t\tlast = m.group(1);\n\t\t\treturn last;\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn null;\n\t\t}\n\t}"
        - id: PromptHandler.resolveAbsoluteFilePath
          type: method_declaration
          code: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {\n\t\tif (textEditor == null)\n\t\t\treturn null;\n\t\tIEditorInput input = textEditor.getEditorInput();\n\t\tif (input instanceof IFileEditorInput)\n\t\t\treturn ((IFileEditorInput) input).getFile().getLocation().toFile().getAbsolutePath();\n\t\tif (input instanceof IURIEditorInput)\n\t\t\treturn new File(((IURIEditorInput) input).getURI()).getAbsolutePath();\n\t\treturn null;\n\t}"
        - id: PromptHandler.resolveProjectPath
          type: method_declaration
          code: "private Path resolveProjectPath(ITextEditor textEditor) {\n\t\tif (textEditor == null)\n\t\t\treturn null;\n\t\tIEditorInput input = textEditor.getEditorInput();\n\t\tif (!(input instanceof IFileEditorInput))\n\t\t\treturn null;\n\t\tIProject project = ((IFileEditorInput) input).getFile().getProject();\n\t\treturn Paths.get(project.getLocation().toOSString());\n\t}"
        - id: PromptHandler.executeInner
          type: method_declaration
          code: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon, PromptJob job) {\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));\n\t\tIModelResponse resp = connector.executeRequest(req, mon, job);\n\t\tAIAnswer res = connector.convertResponse(resp, mon);\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));\n\t\treturn res;\n\t}"
      - id: jyVQdv|I3qhgN
        type: imports
        code: |-
          package xy.ai.workbench.connector.harness;

          import java.io.File;
          import java.nio.file.Path;
          import java.nio.file.Paths;
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
      - id: GEmxmM|8HTEOr
        type: imports
        code: |-
          import org.eclipse.core.resources.IProject;
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
      - id: DaYIWN|qhXhJ9
        type: imports
        code: |-
          import org.eclipse.ui.texteditor.ITextEditor;

          import xy.ai.workbench.Activator;
          import xy.ai.workbench.ActiveEditorListener;
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.EditorInterface;
          import xy.ai.workbench.IncludeAdapter;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.batch.AIBatchManager;
          import xy.ai.workbench.commands.AnswerCommand;
          import xy.ai.workbench.commands.CallEditCommand;
          import xy.ai.workbench.commands.Command;
      - id: KEc50E|MeSef9
        type: imports
        code: |-
          import xy.ai.workbench.commands.CommandRegistry;
          import xy.ai.workbench.connector.AdaptingConnector;
          import xy.ai.workbench.editor.md.AbstractRule;
          import xy.ai.workbench.models.AIAnswer;
          import xy.ai.workbench.models.IModelRequest;
          import xy.ai.workbench.models.IModelResponse;
      - id: fPHG2Y|URMgPQ
        type: statements
        code: |-
          /**
           * Prompt-specific orchestration extracted from {@code AISessionManager}: builds
           * a frozen {@link Prompt} from the active editor/selection, runs command
           * detection and drives the prepare/insertTag/execute/replaceTag job pipeline.
           */
```
/answer 21e8-15 allow
ReasoningToken: 14
Control Request:
```yaml
id: 21e8-16
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: new PromptHandler\(|PromptHandler\.
    include:
    - '*.java'
  reason: "Prüfen, wo PromptHandler instanziiert wird, um öffentliche API-Kompatibilität sicherzustellen"
```
/answer 21e8-16 allow
Control Request:
```yaml
id: 21e8-17
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/AISessionManager.java
        matches:
        - lineno: 30
          match: "prompt = new PromptHandler(cfg, connector, editorListener, editIfc, includeAdapter);"
      - path: xy/ai/workbench/connector/harness/Prompt.java
        matches:
        - lineno: 14
          match: "* exclusively via {@link Builder}, driven by the PromptHandler."
```
/answer 21e8-17 allow
ReasoningToken: 1039
Control Request:
```yaml
id: 21e8-18
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: \.addInputStatObs\(|\.updateInputStat\(|\.initializeInputs\(
    include:
    - '*.java'
  reason: "Aufrufer der Input-Stat-Methoden finden, um Delegation korrekt zu gestalten"
```
/answer 21e8-18 allow
Control Request:
```yaml
id: 21e8-19
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/AISessionManager.java
        matches:
        - lineno: 33
          match: cfg.addInputModeObs(i -> prompt.updateInputStat(i));
        - lineno: 34
          match: "cfg.addEnabledToolsObs(t -> prompt.updateInputStat(InputMode.Tools), false);"
      - path: xy/ai/workbench/ActiveEditorListener.java
        matches:
        - lineno: 78
          match: prompt.updateInputStat(InputMode.Selection);
        - lineno: 79
          match: prompt.updateInputStat(InputMode.Converter);
        - lineno: 152
          match: prompt.updateInputStat(InputMode.Selection);
        - lineno: 162
          match: prompt.updateInputStat(InputMode.Selection);
        - lineno: 163
          match: prompt.updateInputStat(InputMode.Converter);
        - lineno: 173
          match: Display.getDefault().asyncExec(() -> prompt.updateInputStat(InputMode.Selection));
        - lineno: 181
          match: prompt.updateInputStat(InputMode.Selection);
        - lineno: 182
          match: prompt.updateInputStat(InputMode.Converter);
      - path: xy/ai/workbench/view/session/InputsTable.java
        matches:
        - lineno: 44
          match: "prompt.addInputStatObs(is -> {"
      - path: xy/ai/workbench/view/AISessionView.java
        matches:
        - lineno: 76
          match: prompt.initializeInputs();
      - path: xy/ai/workbench/connector/harness/PromptHandler.java
        matches:
        - lineno: 107
          match: includeAdapter.initializeInputs();
```
/answer 21e8-19 allow
ReasoningToken: 81
Control Request:
```yaml
id: 21e8-1a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
    source: "package xy.ai.workbench.connector.harness;\n\nimport java.io.File;\nimport java.nio.file.Path;\nimport java.nio.file.Paths;\nimport java.util.ArrayList;\nimport java.util.Arrays;\nimport java.util.List;\nimport java.util.function.Consumer;\nimport java.util.regex.Matcher;\nimport java.util.regex.Pattern;\n\nimport org.eclipse.core.resources.IProject;\nimport org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IRegion;\nimport org.eclipse.jface.text.ITextSelection;\nimport org.eclipse.jface.viewers.ISelection;\nimport org.eclipse.jface.viewers.ISelectionProvider;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.IEditorInput;\nimport org.eclipse.ui.IFileEditorInput;\nimport org.eclipse.ui.IURIEditorInput;\nimport org.eclipse.ui.texteditor.ITextEditor;\n\nimport xy.ai.workbench.ActiveEditorListener;\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.IncludeAdapter;\nimport xy.ai.workbench.InputMode;\nimport xy.ai.workbench.LOG;\nimport xy.ai.workbench.commands.AnswerCommand;\nimport xy.ai.workbench.commands.CallEditCommand;\nimport xy.ai.workbench.commands.Command;\nimport xy.ai.workbench.commands.CommandRegistry;\nimport xy.ai.workbench.editor.md.AbstractRule;\n\n/**\n * Input handling extracted from {@link PromptHandler}: aggregates editor/config\n * inputs, tracks input-size stats and runs command preprocessing (command\n * substitution / detection) needed to build a {@link Prompt}.\n */\nfinal class PromptInputHandler {\n\n\t// Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block\n\t// before a command line.\n\tprivate static final Pattern YAML_BLOCK = Pattern.compile(\"^```yaml\\\\R(.*?)^```$\",\n\t\t\tPattern.MULTILINE | Pattern.DOTALL);\n\n\tprivate final ConfigManager cfg;\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final IncludeAdapter includeAdapter;\n\n\tprivate int[] inputStats = new int[InputMode.values().length];\n\tprivate List<Consumer<int[]>> inputStatObs = new ArrayList<>();\n\n\tPromptInputHandler(ConfigManager cfg, ActiveEditorListener editorListener, IncludeAdapter includeAdapter) {\n\t\tthis.cfg = cfg;\n\t\tthis.editorListener = editorListener;\n\t\tthis.includeAdapter = includeAdapter;\n\t}\n\n\tvoid addInputStatObs(Consumer<int[]> obs, boolean initialize) {\n\t\tinputStatObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(inputStats);\n\t}\n\n\tvoid updateInputStat(InputMode mode) {\n\t\tif (mode == InputMode.Tools) {\n\t\t\tString[] tools = cfg.getTools();\n\t\t\tinputStats[mode.ordinal()] = tools != null ? tools.length : 0;\n\t\t} else {\n\t\t\tString input = getInput(mode);\n\t\t\tinputStats[mode.ordinal()] = input != null ? input.length() : -1;\n\t\t}\n\t\tinputStatObs.forEach(c -> c.accept(inputStats));\n\t}\n\n\tvoid initializeInputs() {\n\t\tfor (var mode : InputMode.values())\n\t\t\tupdateInputStat(mode);\n\t\tincludeAdapter.initializeInputs();\n\t}\n\n\tprivate String getInput(InputMode mode) {\n\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\n\t\tswitch (mode) {\n\t\tcase SystemPrompt:\n\t\t\tStringBuffer systemPrompt = new StringBuffer();\n\t\t\tArrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith(\"#\"))\n\t\t\t\t\t.forEach(e -> systemPrompt.append(\"* \").append(e).append(\".\\n\"));\n\t\t\tString freeText = cfg.getFreeText();\n\t\t\tif (freeText != null && !freeText.isBlank())\n\t\t\t\tsystemPrompt.append(\".\\n\").append(cfg.getFreeText()).append(\".\\n\");\n\t\t\tString prompttext = systemPrompt.toString();\n\t\t\treturn prompttext.length() > 0 && !prompttext.isBlank() ? prompttext : null;\n\t\tcase Selection:\n\t\t\tif (textEditor != null) {\n\t\t\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\t\t\tif (selectionProvider != null) {\n\t\t\t\t\tISelection selection = selectionProvider.getSelection();\n\t\t\t\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\t\t\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1)\n\t\t\t\t\t\treturn removeCommentLines(tsel.getText());\n\n\t\t\t\t\tif (tsel != null) {\n\t\t\t\t\t\tint line = tsel.getEndLine();\n\t\t\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\tIRegion lineInfo = doc.getLineInformation(line);\n\t\t\t\t\t\t\treturn doc.get(lineInfo.getOffset(), lineInfo.getLength());\n\t\t\t\t\t\t} catch (BadLocationException e1) {\n\t\t\t\t\t\t\tLOG.error(\"Exception\", e1);\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t\tbreak;\n\t\tcase Converter:\n\t\t\tif (textEditor != null) {\n\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\tif (doc != null)\n\t\t\t\t\treturn doc.get();\n\t\t\t}\n\t\t\tbreak;\n\t\tcase Tools:\n\t\t\tthrow new UnsupportedOperationException();\n\t\t}\n\t\treturn null;\n\t}\n\n\tprivate String removeCommentLines(String input) {\n\t\tif (input == null || input.isEmpty())\n\t\t\treturn input;\n\n\t\tStringBuffer result = new StringBuffer();\n\t\tString[] lines = input.split(\"\\\\R\");\n\n\t\tfor (String line : lines)\n\t\t\tif (!line.trim().startsWith(AbstractRule.LINE_COMMENT))\n\t\t\t\tresult.append(line).append(System.lineSeparator());\n\n\t\treturn result.toString();\n\t}\n\n\t/**\n\t * Freezes the current editor/config state into a {@link Prompt}; runs command\n\t * detection (see class doc).\n\t */\n\tPrompt buildPrompt(Display display, boolean batch) {\n\t\tList<String> inputs = new ArrayList<>();\n\t\tString[] absoluteFilePath = new String[1];\n\t\tPath[] projectPath = new Path[1];\n\t\tCommand[] detected = new Command[1];\n\t\tString[] yamlBlock = new String[1];\n\n\t\tdisplay.syncExec(() -> {\n\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\tabsoluteFilePath[0] = resolveAbsoluteFilePath(textEditor);\n\t\t\tprojectPath[0] = resolveProjectPath(textEditor);\n\n\t\t\tif (cfg.isInputEnabled(InputMode.Selection))\n\t\t\t\tdetectSelection(textEditor, inputs, detected, yamlBlock);\n\t\t\telse if (cfg.isInputEnabled(InputMode.Converter))\n\t\t\t\tdetectFullFile(textEditor, inputs, detected, yamlBlock);\n\t\t});\n\n\t\tFrozenConfig frozen = FrozenConfig.from(cfg);\n\n\t\tif (inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank()))\n\t\t\tthrow new IllegalArgumentException(\"Input and System Prompt Empty\");\n\n\t\tif (editorListener.getLastTextEditor() == null && !batch)\n\t\t\tthrow new IllegalArgumentException(\"Result editor unset\");\n\n\t\treturn new Prompt(inputs, batch, frozen, cfg.isInputEnabled(InputMode.Converter), absoluteFilePath[0],\n\t\t\t\tprojectPath[0], detected[0], yamlBlock[0]);\n\t}\n\n\t// Selection mode: a real (multi-char) selection is a BlockSelection, an\n\t// empty/caret selection\n\t// falls back to the current cursor line (LineInput).\n\tprivate void detectSelection(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn;\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn;\n\n\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1) {\n\t\t\t// A selection consisting solely of a ```yaml block is itself a command\n\t\t\t// (CallEditCommand).\n\t\t\tCommand block = CommandRegistry.detect(tsel.getText());\n\t\t\tif (block instanceof CallEditCommand) {\n\t\t\t\tdetected[0] = block;\n\t\t\t\treturn;\n\t\t\t}\n\n\t\t\tinputs.add(removeCommentLines(tsel.getText()));\n\t\t\t// An /answer command starting at the very beginning of the block spans the\n\t\t\t// whole\n\t\t\t// selection, allowing a multi-line (better formatted) reason/hint for allow and\n\t\t\t// deny alike.\n\t\t\tif (block instanceof AnswerCommand) {\n\t\t\t\tdetected[0] = block;\n\t\t\t\tyamlBlock[0] = captureYamlBlock(doc, tsel.getStartLine());\n\t\t\t}\n\t\t\t// Block start (multi-line command) takes precedence over a trailing command\n\t\t\t// line.\n\t\t\telse if (!applyDetected(CommandRegistry.detect(lineText(doc, tsel.getStartLine())), doc,\n\t\t\t\t\ttsel.getStartLine(), detected, yamlBlock))\n\t\t\t\tapplyDetected(CommandRegistry.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(),\n\t\t\t\t\t\tdetected, yamlBlock);\n\t\t\treturn;\n\t\t}\n\n\t\tif (tsel != null) {\n\t\t\ttry {\n\t\t\t\tIRegion lineInfo = doc.getLineInformation(tsel.getEndLine());\n\t\t\t\tinputs.add(doc.get(lineInfo.getOffset(), lineInfo.getLength()));\n\t\t\t} catch (BadLocationException e) {\n\t\t\t\tLOG.error(\"Exception\", e);\n\t\t\t}\n\t\t\tapplyDetected(CommandRegistry.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(), detected,\n\t\t\t\t\tyamlBlock);\n\t\t}\n\t}\n\n\t// Processor (full file) mode: the caret line is checked first, then the last\n\t// line of the file,\n\t// matching a command appended after the generated content.\n\tprivate void detectFullFile(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn;\n\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn;\n\t\tinputs.add(doc.get());\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tif (tsel != null && applyDetected(CommandRegistry.detect(lineText(doc, tsel.getEndLine())), doc,\n\t\t\t\ttsel.getEndLine(), detected, yamlBlock))\n\t\t\treturn;\n\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\tapplyDetected(CommandRegistry.detect(lineText(doc, lastLine)), doc, lastLine, detected, yamlBlock);\n\t\treturn;\n\t}\n\n\tprivate boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Command[] detected, String[] yamlBlock) {\n\t\tif (cmd == null)\n\t\t\treturn false;\n\t\tdetected[0] = cmd;\n\t\tyamlBlock[0] = captureYamlBlock(doc, lineIndex);\n\t\treturn true;\n\t}\n\n\tprivate String lineText(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tIRegion info = doc.getLineInformation(lineIndex);\n\t\t\treturn doc.get(info.getOffset(), info.getLength());\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn \"\";\n\t\t}\n\t}\n\n\t// Walks backwards from the command line and keeps the closest preceding ```yaml\n\t// block.\n\tprivate String captureYamlBlock(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tString prefix = doc.get(0, doc.getLineOffset(lineIndex));\n\t\t\tMatcher m = YAML_BLOCK.matcher(prefix);\n\t\t\tString last = null;\n\t\t\twhile (m.find())\n\t\t\t\tlast = m.group(1);\n\t\t\treturn last;\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn null;\n\t\t}\n\t}\n\n\tprivate String resolveAbsoluteFilePath(ITextEditor textEditor) {\n\t\tif (textEditor == null)\n\t\t\treturn null;\n\t\tIEditorInput input = textEditor.getEditorInput();\n\t\tif (input instanceof IFileEditorInput)\n\t\t\treturn ((IFileEditorInput) input).getFile().getLocation().toFile().getAbsolutePath();\n\t\tif (input instanceof IURIEditorInput)\n\t\t\treturn new File(((IURIEditorInput) input).getURI()).getAbsolutePath();\n\t\treturn null;\n\t}\n\n\tprivate Path resolveProjectPath(ITextEditor textEditor) {\n\t\tif (textEditor == null)\n\t\t\treturn null;\n\t\tIEditorInput input = textEditor.getEditorInput();\n\t\tif (!(input instanceof IFileEditorInput))\n\t\t\treturn null;\n\t\tIProject project = ((IFileEditorInput) input).getFile().getProject();\n\t\treturn Paths.get(project.getLocation().toOSString());\n\t}\n}\n"
  reason: "Neue Klasse PromptInputHandler mit Input Handling, Aggregation und Preprocessing anlegen"
```
/answer 21e8-1a allow
Control Request:
```yaml
id: 21e8-1b
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: fPHG2Y|URMgPQ
    source: |-
      /**
       * Prompt-specific orchestration extracted from {@code AISessionManager}: drives
       * the prepare/insertTag/execute/replaceTag job pipeline, delegating input
       * aggregation/preprocessing to {@link PromptInputHandler}.
       */
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: jyVQdv|I3qhgN
    source: |-
      package xy.ai.workbench.connector.harness;

      import java.util.ArrayList;
      import java.util.List;
      import java.util.function.Consumer;

      import org.eclipse.core.runtime.IProgressMonitor;
      import org.eclipse.core.runtime.IStatus;
      import org.eclipse.core.runtime.Status;
      import org.eclipse.core.runtime.SubMonitor;
      import org.eclipse.core.runtime.jobs.Job;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: GEmxmM|8HTEOr
    source: import org.eclipse.swt.widgets.Display;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: DaYIWN|qhXhJ9
    source: |-
      import xy.ai.workbench.Activator;
      import xy.ai.workbench.ActiveEditorListener;
      import xy.ai.workbench.ConfigManager;
      import xy.ai.workbench.EditorInterface;
      import xy.ai.workbench.IncludeAdapter;
      import xy.ai.workbench.InputMode;
      import xy.ai.workbench.LOG;
      import xy.ai.workbench.batch.AIBatchManager;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: KEc50E|MeSef9
    source: |-
      import xy.ai.workbench.connector.AdaptingConnector;
      import xy.ai.workbench.models.AIAnswer;
      import xy.ai.workbench.models.IModelRequest;
      import xy.ai.workbench.models.IModelResponse;
  reason: Imports von PromptHandler bereinigen (nicht mehr benötigte entfernen)
```
/answer 21e8-1b allow
Control Request:
```yaml
id: 21e8-1c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: fPHG2Y|URMgPQ
      new_id: hMGSgH|1FhRB8
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: jyVQdv|I3qhgN
      new_id: lNlMUf|plxNbT
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: KEc50E|MeSef9
      new_id: KEc50E|Y0sZte
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      error: No node matched the selector.
      id: GEmxmM|8HTEOr
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      error: No node matched the selector.
      id: DaYIWN|qhXhJ9
```
/answer 21e8-1c allow
Control Request:
```yaml
id: 21e8-1d
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: Aktuellen Zustand der Datei nach teilweisem Edit prüfen
```
/answer 21e8-1d allow
Control Request:
```yaml
id: 21e8-1e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: lNlMUf|plxNbT
        signature: package xy.ai.workbench.connector.harness;
      - id: 4UXeuu|5xcut7
        signature: import org.eclipse.jface.text.IRegion;
      - id: uzcHjn|XyLKjl
        signature: import xy.ai.workbench.EditorInterface;
      - id: KEc50E|Y0sZte
        signature: import xy.ai.workbench.connector.AdaptingConnector;
      - id: hMGSgH|1FhRB8
        signature: /**
      - id: PromptHandler
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.ljM8Ja|2m2HTZ
          signature: // Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block
        - id: PromptHandler.LSyBuh|FgyeYv
          signature: "private int[] inputStats = new int[InputMode.values().length];"
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEdit…"
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
            signature: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, IProgressMonitor mon) {"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.o8lrZ3|vOa7RL
          signature: /**
        - id: PromptHandler.buildPrompt
          signature: "private Prompt buildPrompt(Display display, boolean batch) {"
        - id: PromptHandler.ohnvjH|PwL7la
          signature: "// Selection mode: a real (multi-char) selection is a BlockSelection, an"
        - id: PromptHandler.detectSelection
          signature: "private void detectSelection(ITextEditor textEditor, List<String> inputs, Comma…"
        - id: PromptHandler.Txg3oX|CbRnfT
          signature: "// Processor (full file) mode: the caret line is checked first, then the last"
        - id: PromptHandler.detectFullFile
          signature: "private void detectFullFile(ITextEditor textEditor, List<String> inputs, Comman…"
        - id: PromptHandler.applyDetected
          signature: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Comman…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.WEwxAI|MZVZI2
          signature: // Walks backwards from the command line and keeps the closest preceding ```yaml
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.resolveProjectPath
          signature: "private Path resolveProjectPath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
```
/answer 21e8-1e allow
Control Request:
```yaml
id: 21e8-1f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - 4UXeuu|5xcut7
    - uzcHjn|XyLKjl
  reason: "Inhalt der verbleibenden Import-Blöcke lesen, um sie korrekt zu bereinigen"
```
/answer 21e8-1f allow
Control Request:
```yaml
id: 21e8-20
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: 4UXeuu|5xcut7
        type: imports
        code: |-
          import org.eclipse.jface.text.IRegion;
          import org.eclipse.jface.text.ITextSelection;
          import org.eclipse.jface.viewers.ISelection;
          import org.eclipse.jface.viewers.ISelectionProvider;
          import org.eclipse.swt.widgets.Display;
          import org.eclipse.ui.IEditorInput;
          import org.eclipse.ui.IFileEditorInput;
          import org.eclipse.ui.IURIEditorInput;
          import org.eclipse.ui.texteditor.ITextEditor;

          import xy.ai.workbench.Activator;
          import xy.ai.workbench.ActiveEditorListener;
          import xy.ai.workbench.ConfigManager;
      - id: uzcHjn|XyLKjl
        type: imports
        code: |-
          import xy.ai.workbench.EditorInterface;
          import xy.ai.workbench.IncludeAdapter;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.batch.AIBatchManager;
          import xy.ai.workbench.commands.AnswerCommand;
          import xy.ai.workbench.commands.CallEditCommand;
          import xy.ai.workbench.commands.Command;
          import xy.ai.workbench.commands.CommandRegistry;
          import xy.ai.workbench.connector.AdaptingConnector;
          import xy.ai.workbench.editor.md.AbstractRule;
          import xy.ai.workbench.models.AIAnswer;
```
/answer 21e8-20 allow
Control Request:
```yaml
id: 21e8-21
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - KEc50E|Y0sZte
  reason: "Restlichen Import-Block prüfen, um AIAnswer/IModelRequest/IModelResponse korrekt zu erhalten"
```
/answer 21e8-21 allow
Control Request:
```yaml
id: 21e8-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: KEc50E|Y0sZte
        type: imports
        code: |-
          import xy.ai.workbench.connector.AdaptingConnector;
          import xy.ai.workbench.models.AIAnswer;
          import xy.ai.workbench.models.IModelRequest;
          import xy.ai.workbench.models.IModelResponse;
```
/answer 21e8-22 allow
Control Request:
```yaml
id: 21e8-23
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: 4UXeuu|5xcut7
    source: |-
      import org.eclipse.swt.widgets.Display;

      import xy.ai.workbench.Activator;
      import xy.ai.workbench.ActiveEditorListener;
      import xy.ai.workbench.ConfigManager;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: uzcHjn|XyLKjl
    source: |-
      import xy.ai.workbench.EditorInterface;
      import xy.ai.workbench.IncludeAdapter;
      import xy.ai.workbench.InputMode;
      import xy.ai.workbench.LOG;
      import xy.ai.workbench.batch.AIBatchManager;
  reason: Verbleibende Import-Blöcke von PromptHandler auf tatsächlich benötigte Typen reduzieren
```
/answer 21e8-23 allow
Control Request:
```yaml
id: 21e8-24
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: 4UXeuu|5xcut7
      new_id: MbJyjk|d7OcEV
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      error: No node matched the selector.
      id: uzcHjn|XyLKjl
```
/answer 21e8-24 allow
Control Request:
```yaml
id: 21e8-25
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: Aktuellen Zustand der Imports nach Edit prüfen
```
/answer 21e8-25 allow
Control Request:
```yaml
id: 21e8-26
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: lNlMUf|plxNbT
        signature: package xy.ai.workbench.connector.harness;
      - id: MbJyjk|d7OcEV
        signature: import org.eclipse.swt.widgets.Display;
      - id: KEc50E|2cMnpj
        signature: import xy.ai.workbench.commands.CommandRegistry;
      - id: hMGSgH|1FhRB8
        signature: /**
      - id: PromptHandler
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.ljM8Ja|2m2HTZ
          signature: // Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block
        - id: PromptHandler.LSyBuh|FgyeYv
          signature: "private int[] inputStats = new int[InputMode.values().length];"
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEdit…"
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
            signature: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, IProgressMonitor mon) {"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.o8lrZ3|vOa7RL
          signature: /**
        - id: PromptHandler.buildPrompt
          signature: "private Prompt buildPrompt(Display display, boolean batch) {"
        - id: PromptHandler.ohnvjH|PwL7la
          signature: "// Selection mode: a real (multi-char) selection is a BlockSelection, an"
        - id: PromptHandler.detectSelection
          signature: "private void detectSelection(ITextEditor textEditor, List<String> inputs, Comma…"
        - id: PromptHandler.Txg3oX|CbRnfT
          signature: "// Processor (full file) mode: the caret line is checked first, then the last"
        - id: PromptHandler.detectFullFile
          signature: "private void detectFullFile(ITextEditor textEditor, List<String> inputs, Comman…"
        - id: PromptHandler.applyDetected
          signature: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Comman…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.WEwxAI|MZVZI2
          signature: // Walks backwards from the command line and keeps the closest preceding ```yaml
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.resolveProjectPath
          signature: "private Path resolveProjectPath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
```
/answer 21e8-26 allow
Control Request:
```yaml
id: 21e8-27
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - lNlMUf|plxNbT
    - MbJyjk|d7OcEV
    - KEc50E|2cMnpj
  reason: Vollständigen aktuellen Import-Bereich lesen
```
/answer 21e8-27 allow
Control Request:
```yaml
id: 21e8-28
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: lNlMUf|plxNbT
        type: imports
        code: |-
          package xy.ai.workbench.connector.harness;

          import java.util.ArrayList;
          import java.util.List;
          import java.util.function.Consumer;

          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.IStatus;
          import org.eclipse.core.runtime.Status;
          import org.eclipse.core.runtime.SubMonitor;
          import org.eclipse.core.runtime.jobs.Job;
          import org.eclipse.core.resources.IProject;
          import org.eclipse.jface.text.BadLocationException;
          import org.eclipse.jface.text.IDocument;
      - id: MbJyjk|d7OcEV
        type: imports
        code: |-
          import org.eclipse.swt.widgets.Display;

          import xy.ai.workbench.Activator;
          import xy.ai.workbench.ActiveEditorListener;
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.EditorInterface;
          import xy.ai.workbench.IncludeAdapter;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.batch.AIBatchManager;
          import xy.ai.workbench.commands.AnswerCommand;
          import xy.ai.workbench.commands.CallEditCommand;
          import xy.ai.workbench.commands.Command;
      - id: KEc50E|2cMnpj
        type: imports
        code: |-
          import xy.ai.workbench.commands.CommandRegistry;
          import xy.ai.workbench.connector.AdaptingConnector;
          import xy.ai.workbench.editor.md.AbstractRule;
          import xy.ai.workbench.models.AIAnswer;
          import xy.ai.workbench.connector.AdaptingConnector;
          import xy.ai.workbench.models.AIAnswer;
          import xy.ai.workbench.models.IModelRequest;
          import xy.ai.workbench.models.IModelResponse;
```
/answer 21e8-28 allow
Control Request:
```yaml
id: 21e8-29
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: lNlMUf|plxNbT
    source: |-
      package xy.ai.workbench.connector.harness;

      import java.util.ArrayList;
      import java.util.List;
      import java.util.function.Consumer;

      import org.eclipse.core.runtime.IProgressMonitor;
      import org.eclipse.core.runtime.IStatus;
      import org.eclipse.core.runtime.Status;
      import org.eclipse.core.runtime.SubMonitor;
      import org.eclipse.core.runtime.jobs.Job;
      import org.eclipse.swt.widgets.Display;

      import xy.ai.workbench.Activator;
      import xy.ai.workbench.ActiveEditorListener;
      import xy.ai.workbench.ConfigManager;
      import xy.ai.workbench.EditorInterface;
      import xy.ai.workbench.IncludeAdapter;
      import xy.ai.workbench.InputMode;
      import xy.ai.workbench.LOG;
      import xy.ai.workbench.batch.AIBatchManager;
      import xy.ai.workbench.connector.AdaptingConnector;
      import xy.ai.workbench.models.AIAnswer;
      import xy.ai.workbench.models.IModelRequest;
      import xy.ai.workbench.models.IModelResponse;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: MbJyjk|d7OcEV
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: KEc50E|2cMnpj
    source: ""
  reason: Import-Bereich vollständig konsolidieren und Duplikate entfernen
```
/answer 21e8-29 allow
Control Request:
```yaml
id: 21e8-2a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: lNlMUf|plxNbT
      new_id: z9X5SW|dfu0iF
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: KEc50E|2cMnpj
      new_id: KEc50E|x4YbAP
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      error: No node matched the selector.
      id: MbJyjk|d7OcEV
```
/answer 21e8-2a allow
Control Request:
```yaml
id: 21e8-2b
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: Aktuellen Import-Zustand nochmals prüfen
```
/answer 21e8-2b allow
Control Request:
```yaml
id: 21e8-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: z9X5SW|dfu0iF
        signature: package xy.ai.workbench.connector.harness;
      - id: 5wdHBZ|59odD5
        signature: import xy.ai.workbench.EditorInterface;
      - id: sEQQ8c|KfxDYW
        signature: import xy.ai.workbench.ConfigManager;
      - id: hMGSgH|1FhRB8
        signature: /**
      - id: PromptHandler
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.ljM8Ja|2m2HTZ
          signature: // Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block
        - id: PromptHandler.LSyBuh|FgyeYv
          signature: "private int[] inputStats = new int[InputMode.values().length];"
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEdit…"
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
            signature: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, IProgressMonitor mon) {"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.o8lrZ3|vOa7RL
          signature: /**
        - id: PromptHandler.buildPrompt
          signature: "private Prompt buildPrompt(Display display, boolean batch) {"
        - id: PromptHandler.ohnvjH|PwL7la
          signature: "// Selection mode: a real (multi-char) selection is a BlockSelection, an"
        - id: PromptHandler.detectSelection
          signature: "private void detectSelection(ITextEditor textEditor, List<String> inputs, Comma…"
        - id: PromptHandler.Txg3oX|CbRnfT
          signature: "// Processor (full file) mode: the caret line is checked first, then the last"
        - id: PromptHandler.detectFullFile
          signature: "private void detectFullFile(ITextEditor textEditor, List<String> inputs, Comman…"
        - id: PromptHandler.applyDetected
          signature: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Comman…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.WEwxAI|MZVZI2
          signature: // Walks backwards from the command line and keeps the closest preceding ```yaml
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.resolveProjectPath
          signature: "private Path resolveProjectPath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
```
/answer 21e8-2c allow
Control Request:
```yaml
id: 21e8-2d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - z9X5SW|dfu0iF
    - 5wdHBZ|59odD5
    - sEQQ8c|KfxDYW
  reason: Verbleibende Import-Duplikate identifizieren
```
/answer 21e8-2d allow
Control Request:
```yaml
id: 21e8-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: z9X5SW|dfu0iF
        type: imports
        code: |-
          package xy.ai.workbench.connector.harness;

          import java.util.ArrayList;
          import java.util.List;
          import java.util.function.Consumer;

          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.IStatus;
          import org.eclipse.core.runtime.Status;
          import org.eclipse.core.runtime.SubMonitor;
          import org.eclipse.core.runtime.jobs.Job;
          import org.eclipse.swt.widgets.Display;

          import xy.ai.workbench.Activator;
          import xy.ai.workbench.ActiveEditorListener;
          import xy.ai.workbench.ConfigManager;
      - id: 5wdHBZ|59odD5
        type: imports
        code: |-
          import xy.ai.workbench.EditorInterface;
          import xy.ai.workbench.IncludeAdapter;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.batch.AIBatchManager;
          import xy.ai.workbench.connector.AdaptingConnector;
          import xy.ai.workbench.models.AIAnswer;
          import xy.ai.workbench.models.IModelRequest;
          import xy.ai.workbench.models.IModelResponse;
          import org.eclipse.swt.widgets.Display;

          import xy.ai.workbench.Activator;
          import xy.ai.workbench.ActiveEditorListener;
      - id: sEQQ8c|KfxDYW
        type: imports
        code: |-
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.EditorInterface;
          import xy.ai.workbench.IncludeAdapter;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.batch.AIBatchManager;
          import xy.ai.workbench.commands.AnswerCommand;
          import xy.ai.workbench.commands.CallEditCommand;
          import xy.ai.workbench.commands.Command;
          import xy.ai.workbench.commands.CommandRegistry;
          import xy.ai.workbench.connector.AdaptingConnector;
          import xy.ai.workbench.editor.md.AbstractRule;
```
/answer 21e8-2e allow
Control Request:
```yaml
id: 21e8-2f
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: 5wdHBZ|59odD5
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: sEQQ8c|KfxDYW
    source: ""
  reason: Verbleibende duplizierte Import-Blöcke leeren
```
/answer 21e8-2f allow
Control Request:
```yaml
id: 21e8-30
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: Prüfen ob Import-Bereich jetzt sauber ist
```
/answer 21e8-30 allow
Control Request:
```yaml
id: 21e8-31
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: z9X5SW|dfu0iF
        signature: package xy.ai.workbench.connector.harness;
      - id: hMGSgH|1FhRB8
        signature: /**
      - id: PromptHandler
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.ljM8Ja|2m2HTZ
          signature: // Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block
        - id: PromptHandler.LSyBuh|FgyeYv
          signature: "private int[] inputStats = new int[InputMode.values().length];"
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEdit…"
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
            signature: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, IProgressMonitor mon) {"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.o8lrZ3|vOa7RL
          signature: /**
        - id: PromptHandler.buildPrompt
          signature: "private Prompt buildPrompt(Display display, boolean batch) {"
        - id: PromptHandler.ohnvjH|PwL7la
          signature: "// Selection mode: a real (multi-char) selection is a BlockSelection, an"
        - id: PromptHandler.detectSelection
          signature: "private void detectSelection(ITextEditor textEditor, List<String> inputs, Comma…"
        - id: PromptHandler.Txg3oX|CbRnfT
          signature: "// Processor (full file) mode: the caret line is checked first, then the last"
        - id: PromptHandler.detectFullFile
          signature: "private void detectFullFile(ITextEditor textEditor, List<String> inputs, Comman…"
        - id: PromptHandler.applyDetected
          signature: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Comman…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.WEwxAI|MZVZI2
          signature: // Walks backwards from the command line and keeps the closest preceding ```yaml
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.resolveProjectPath
          signature: "private Path resolveProjectPath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
```
/answer 21e8-31 allow
Control Request:
```yaml
id: 21e8-32
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.ljM8Ja|2m2HTZ
    source: "private final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\tprivate final EditorInterface editIfc;\n\tprivate final AIBatchManager batch;\n\tprivate final PromptInputHandler input;"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.LSyBuh|FgyeYv
    source: private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.PromptHandler
    source: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEditorListener editorListener,\n\t\t\tEditorInterface editIfc, IncludeAdapter includeAdapter) {\n\t\tthis.cfg = cfg;\n\t\tthis.connector = connector;\n\t\tthis.editIfc = editIfc;\n\t\tthis.batch = Activator.getDefault().batch;\n\t\tthis.input = new PromptInputHandler(cfg, editorListener, includeAdapter);\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.addInputStatObs
    source: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {\n\t\tinput.addInputStatObs(obs, initialize);\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.updateInputStat
    source: "public void updateInputStat(InputMode mode) {\n\t\tinput.updateInputStat(mode);\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.initializeInputs
    source: "public void initializeInputs() {\n\t\tinput.initializeInputs();\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.buildPrompt
    source: "private Prompt buildPrompt(Display display, boolean batch) {\n\t\treturn input.buildPrompt(display, batch);\n\t}"
  reason: "Felder, Konstruktor und Input-Delegationsmethoden in PromptHandler anpassen"
```
/answer 21e8-32 allow
Control Request:
```yaml
id: 21e8-33
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: PromptHandler.ljM8Ja|2m2HTZ
      new_id: PromptHandler.0bratw|rh9zqI
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: PromptHandler.PromptHandler
      new_id: PromptHandler.PromptHandler
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: PromptHandler.addInputStatObs
      new_id: PromptHandler.addInputStatObs
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: PromptHandler.updateInputStat
      new_id: PromptHandler.updateInputStat
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: PromptHandler.initializeInputs
      new_id: PromptHandler.initializeInputs
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      result: success
      id: PromptHandler.buildPrompt
      new_id: PromptHandler.buildPrompt
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      error: No node matched the selector.
      id: PromptHandler.LSyBuh|FgyeYv
```
/answer 21e8-33 allow
Control Request:
```yaml
id: 21e8-34
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: Aktuelle Node-IDs nach Edits erneut abrufen
```
/answer 21e8-34 allow
Control Request:
```yaml
id: 21e8-35
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: z9X5SW|dfu0iF
        signature: package xy.ai.workbench.connector.harness;
      - id: hMGSgH|1FhRB8
        signature: /**
      - id: PromptHandler
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.0bratw|rh9zqI
          signature: private final ConfigManager cfg;
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEdit…"
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
            signature: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, IProgressMonitor mon) {"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.o8lrZ3|vOa7RL
          signature: /**
        - id: PromptHandler.buildPrompt
          signature: "private Prompt buildPrompt(Display display, boolean batch) {"
        - id: PromptHandler.ohnvjH|PwL7la
          signature: "// Selection mode: a real (multi-char) selection is a BlockSelection, an"
        - id: PromptHandler.detectSelection
          signature: "private void detectSelection(ITextEditor textEditor, List<String> inputs, Comma…"
        - id: PromptHandler.Txg3oX|CbRnfT
          signature: "// Processor (full file) mode: the caret line is checked first, then the last"
        - id: PromptHandler.detectFullFile
          signature: "private void detectFullFile(ITextEditor textEditor, List<String> inputs, Comman…"
        - id: PromptHandler.applyDetected
          signature: "private boolean applyDetected(Command cmd, IDocument doc, int lineIndex, Comman…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.WEwxAI|MZVZI2
          signature: // Walks backwards from the command line and keeps the closest preceding ```yaml
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.resolveProjectPath
          signature: "private Path resolveProjectPath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
```
/answer 21e8-35 allow
ReasoningToken: 2247
Control Request:
```yaml
id: 21e8-36
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - PromptHandler.0bratw|rh9zqI
  reason: "Aktuellen Feldblock lesen, um answerObs Feld korrekt hinzuzufügen"
```
/answer 21e8-36 allow
Control Request:
```yaml
id: 21e8-37
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler.0bratw|rh9zqI
        type: statements
        code: "private final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\tprivate final EditorInterface editIfc;\n\tprivate final AIBatchManager batch;\n\tprivate final PromptInputHandler input;\n\n\tprivate int[] inputStats = new int[InputMode.values().length];\n\tprivate List<Consumer<AIAnswer>> answerObs = new ArrayList<>();\n\tprivate List<Consumer<int[]>> inputStatObs = new ArrayList<>();"
```
/answer 21e8-37 allow
Text:
 Jetzt PromptHandler.java umbauen: Felder/Methoden entfernen, Delegation an PromptInputHandler einbauen, Imports bereinigen.
Control Request:
```yaml
id: 21e8-38
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.0bratw|rh9zqI
    old_text: "private int[] inputStats = new int[InputMode.values().length];\n\tprivate List<Consumer<AIAnswer>> answerObs = new ArrayList<>();\n\tprivate List<Consumer<int[]>> inputStatObs = new ArrayList<>();"
    new_text: private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();
  reason: Nicht mehr benötigte Input-Stat-Felder aus PromptHandler entfernen
```
/answer 21e8-38 allow
Control Request:
```yaml
id: 21e8-39
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.getInput
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.removeCommentLines
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.ohnvjH|PwL7la
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.detectSelection
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.Txg3oX|CbRnfT
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.detectFullFile
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.applyDetected
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.lineText
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.WEwxAI|MZVZI2
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.captureYamlBlock
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.resolveAbsoluteFilePath
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.resolveProjectPath
  reason: Nach PromptInputHandler ausgelagerte Methoden aus PromptHandler entfernen
```
/answer 21e8-39 allow
ReasoningToken: 29
Control Request:
```yaml
id: 21e8-3a
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
  reason: Kompilierbarkeit der aufgeteilten Dateien prüfen
```
/answer 21e8-3a allow
ReasoningToken: 192
ReasoningToken: 362
Control Request:
```yaml
id: 21e8-3b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - PromptHandler
  reason: Finalen Zustand der PromptHandler-Klasse vollständig prüfen
```
/answer 21e8-3b allow
Control Request:
```yaml
id: 21e8-3c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler
        type: class_declaration
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.0bratw|wFt8oN
          type: statements
          code: "private final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\tprivate final EditorInterface editIfc;\n\tprivate final AIBatchManager batch;\n\tprivate final PromptInputHandler input;\n\n\tprivate List<Consumer<AIAnswer>> answerObs = new ArrayList<>();"
        - id: PromptHandler.PromptHandler
          type: constructor_declaration
          code: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, ActiveEditorListener editorListener,\n\t\t\tEditorInterface editIfc, IncludeAdapter includeAdapter) {\n\t\tthis.cfg = cfg;\n\t\tthis.connector = connector;\n\t\tthis.editIfc = editIfc;\n\t\tthis.batch = Activator.getDefault().batch;\n\t\tthis.input = new PromptInputHandler(cfg, editorListener, includeAdapter);\n\t}"
        - id: PromptHandler.addInputStatObs
          type: method_declaration
          code: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {\n\t\tinput.addInputStatObs(obs, initialize);\n\t}"
        - id: PromptHandler.addAnswerObs
          type: method_declaration
          code: "public void addAnswerObs(Consumer<AIAnswer> obs) {\n\t\tanswerObs.add(obs);\n\t}"
        - id: PromptHandler.updateInputStat
          type: method_declaration
          code: "public void updateInputStat(InputMode mode) {\n\t\tinput.updateInputStat(mode);\n\t}"
        - id: PromptHandler.initializeInputs
          type: method_declaration
          code: "public void initializeInputs() {\n\t\tinput.initializeInputs();\n\t}"
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
          code: "public void queueAsync(Display display) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.queueSync
          type: method_declaration
          code: "private void queueSync(Display display, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Enqueue batch prompt\", 3);\n\t\tsub.subTask(\"Prepare inputs\");\n\t\tvar req = prepareRequest(display, true, sub.split(1));\n\t\tsub.subTask(\"Insert Tag\");\n\t\teditIfc.insertTag(display, req, sub.split(1));\n\t\tsub.subTask(\"Enqueue prompt\");\n\t\tbatch.enqueue(req, sub.split(1));\n\t}"
        - id: PromptHandler.queueAndSubmit
          type: method_declaration
          code: "public void queueAndSubmit(Display display) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, mon);\n\t\t\t\tbatch.submitBatches(mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.prepareRequest
          type: method_declaration
          code: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Preparing Call\", 1);\n\t\tsub.subTask(\"Preparing Call\");\n\t\tPrompt prompt = buildPrompt(display, batchFix);\n\t\tIModelRequest req = connector.createRequest(prompt, sub);\n\t\tsub.worked(1);\n\t\treturn req;\n\t}"
        - id: PromptHandler.o8lrZ3|vOa7RL
          type: statements
          code: "/**\n\t * Freezes the current editor/config state into a {@link Prompt}; runs command\n\t * detection (see class doc).\n\t */"
        - id: PromptHandler.buildPrompt
          type: method_declaration
          code: "private Prompt buildPrompt(Display display, boolean batch) {\n\t\treturn input.buildPrompt(display, batch);\n\t}"
        - id: PromptHandler.executeInner
          type: method_declaration
          code: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon, PromptJob job) {\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));\n\t\tIModelResponse resp = connector.executeRequest(req, mon, job);\n\t\tAIAnswer res = connector.convertResponse(resp, mon);\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));\n\t\treturn res;\n\t}"
```
/answer 21e8-3c allow
Text:
 Gut, dieser Block ist bereits korrekt. Jetzt die beiden übrigen Blöcke bereinigen:
ReasoningToken: 33
Control Request:
```yaml
id: 21e8-3d
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.o8lrZ3|vOa7RL
  reason: "Veralteten, jetzt irreführenden Javadoc-Kommentar über der reinen Delegationsmethode buildPrompt entfernen"
```
/answer 21e8-3d allow
ReasoningToken: 224
Control Request:
```yaml
id: 21e8-3e
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
  reason: Finale Syntaxprüfung beider Dateien nach allen Änderungen
```
/answer 21e8-3e allow
ReasoningToken: 346
ReasoningToken: 38
ReasoningToken: 551
Text:
 Imports sind jetzt sauber. Jetzt die Klassenkörper-Methoden anpassen: Felder, Delegation, `buildPrompt`-Aufruf, und alle input-bezogenen Methoden entfernen.
ReasoningToken: 36
ReasoningToken: 151
Text:
 Ich muss `inputStats` und `inputStatObs` entfernen (jetzt in PromptInputHandler), `answerObs` bleibt.
Text:
 Jetzt die ausgelagerten Methoden (`getInput`, `removeCommentLines`, `detectSelection`, `detectFullFile`, `applyDetected`, `lineText`, `captureYamlBlock`, `resolveAbsoluteFilePath`, `resolveProjectPath`) sowie die zugehörigen Kommentare löschen.
Text:
 Jetzt validieren und den finalen Zustand prüfen.
ReasoningToken: 75
ReasoningToken: 568
Text:
 Abschließende Validierung:
Result Stats:  id=83bf3270-9b48-4f1b-9fae-477ea1238bbd, total: 0, in: 185, out: 21677, reason: 0, read: 969740, write: 57546
Die Aufteilung ist abgeschlossen:

- **`PromptHandler`**: Koordination/Ausführung (Konstruktor, `execute`, `PromptJob`, `queueAsync/queueSync/queueAndSubmit`, `prepareRequest`, `executeInner`, `answerObs`), delegiert Input-bezogene Aufrufe an `input`.
- **`PromptInputHandler`** (neu, package-private): Input-Aggregation (`getInput`, `buildPrompt`, `detectSelection`, `detectFullFile`), Stats (`inputStats`, `addInputStatObs`, `updateInputStat`, `initializeInputs`) und Preprocessing/Kommandosubstitution (`applyDetected`, `captureYamlBlock`, `removeCommentLines`, `lineText`, `resolveAbsoluteFilePath`, `resolveProjectPath`, `YAML_BLOCK`).

Öffentliche API von `PromptHandler` (Konstruktor, `addInputStatObs`, `updateInputStat`, `initializeInputs`) ist unverändert, externe Aufrufer benötigen keine Anpassung. Beide Dateien validiert.
