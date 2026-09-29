Der `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java` soll in `buildPrompt()` erkennen, wenn ein Prompt versehentlich, also in einer Zeile mit einem Prompt Tag ausgelöst wird (`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java`, `getPromptTag`) und dann eine Exception werfen.
SystemInit:  id=6ee053a3-4f36-471c-ba17-17e9fb9f6cef, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: cefd-2
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  reason: "Struktur beider Dateien prüfen, um buildPrompt() und getPromptTag() zu verstehen"
```
/answer cefd-2 allow
Control Request:
```yaml
id: cefd-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
      nodes:
      - id: njnPQV|SJ135r
        signature: import java.util.ArrayList;
      - id: sEQQ8c|onrBiT
        signature: import xy.ai.workbench.commands.Command;
      - id: PromptInputHandler
        signature: "/**  * Input handling extracted from {"
        docstring: "Input handling extracted from {@link PromptHandler}: aggregates editor/config"
        children:
        - id: PromptInputHandler.dcPuv5|G8OVuS
          signature: private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)^```…
        - id: PromptInputHandler.PromptInputHandler
          signature: "public PromptInputHandler(ConfigManager cfg, ActiveEditorListener editorListene…"
        - id: PromptInputHandler.addInputStatObs
          signature: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize)"
        - id: PromptInputHandler.updateInputStat
          signature: private void updateInputStat(InputMode mode)
        - id: PromptInputHandler.initializeInputs
          signature: public void initializeInputs()
        - id: PromptInputHandler.getInput
          signature: private String getInput(InputMode mode)
        - id: PromptInputHandler.removeCommentLines
          signature: private String removeCommentLines(Selection input)
        - id: PromptInputHandler.buildPrompt
          signature: "public Prompt buildPrompt(Display display, boolean batch)"
          docstring: "Freezes the current editor/config state into a {@link Prompt}; runs command"
        - id: PromptInputHandler.detectSelection
          signature: "private void detectSelection(Selection sel, PromptArguments arg)"
          docstring: "Selection mode: a real (multi-char) selection is a BlockSelection, an"
        - id: PromptInputHandler.detectFullFile
          signature: "private void detectFullFile(Selection content, PromptArguments arg)"
        - id: PromptInputHandler.detectedCmd
          signature: "private boolean detectedCmd(String line, String[] lines, int lineIndex, PromptA…"
        - id: PromptInputHandler.captureYamlBlock
          signature: "private String captureYamlBlock(String[] lines, int lineIndex)"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      nodes:
      - id: 8F4ogT|IZC9FH
        signature: import java.io.BufferedReader;
      - id: msPtFF|1JsNCE
        signature: import org.eclipse.core.resources.IMarker;
      - id: EpdK0Z|pyjvlq
        signature: import org.eclipse.core.runtime.jobs.Job;
      - id: nLRwJK|pRpe5d
        signature: import org.eclipse.ui.IFileEditorInput;
      - id: uzcHjn|DXLr0N
        signature: import xy.ai.workbench.OutputMode;
      - id: MarkerRessourceScanner
        signature: "public class MarkerRessourceScanner implements IResourceChangeListener, IResour…"
        children:
        - id: MarkerRessourceScanner.1KGB34|MrWChh
          signature: private static final String AIREQ_PREFIX = "xy.ai.req";
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
        - id: MarkerRessourceScanner.getPromptTag
          signature: "public static String getPromptTag(String meta, String id)"
```
/answer cefd-3 allow
ReasoningToken: 11
Control Request:
```yaml
id: cefd-4
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
    ids:
    - PromptInputHandler.buildPrompt
    - PromptInputHandler.detectSelection
    - PromptInputHandler.detectFullFile
    - PromptInputHandler.detectedCmd
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    ids:
    - MarkerRessourceScanner.getPromptTag
    - MarkerRessourceScanner.1KGB34|MrWChh
  reason: "Code von buildPrompt und getPromptTag lesen, um die Erkennungslogik zu implementieren"
```
/answer cefd-4 allow
Control Request:
```yaml
id: cefd-5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
      nodes:
      - id: PromptInputHandler.buildPrompt
        type: MethodDeclaration
        code: "\tpublic Prompt buildPrompt(Display display, boolean batch) {\n\t\tPromptArguments arg = new PromptArguments();\n\t\tdisplay.syncExec(() -> {\n\t\t\targ.setEditor(editorListener.getLastTextEditor());\n\t\t\targ.absoluteFilePath = editorListener.resolveAbsoluteFilePath();\n\t\t\targ.project = editorListener.resolveProjectPath();\n\n\t\t\tif (cfg.isInputEnabled(InputMode.Selection))\n\t\t\t\tdetectSelection(editorListener.getSelection(), arg);\n\t\t\telse if (cfg.isInputEnabled(InputMode.Converter))\n\t\t\t\tdetectFullFile(editorListener.getFileContent(), arg);\n\t\t});\n\t\tFrozenConfig frozen = FrozenConfig.from(cfg);\n\t\targ.processorEnabled = cfg.isInputEnabled(InputMode.Converter);\n\n\t\tif (arg.inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank())\n\t\t\t\t&& arg.command == null)\n\t\t\tthrow new IllegalArgumentException(\"Input and System Prompt and Command Empty\");\n\n\t\tif (arg.getEditor() == null && !batch)\n\t\t\tthrow new IllegalArgumentException(\"Result editor unset\");\n\n\t\treturn new Prompt(arg.inputs, batch, frozen, arg);\n\t}\n"
      - id: PromptInputHandler.detectSelection
        type: MethodDeclaration
        code: "\tprivate void detectSelection(Selection sel, PromptArguments arg) {\n\t\tif (sel == null || sel.selection() == null || sel.selection().length == 0)\n\t\t\treturn;\n\n\t\tString[] lines = sel.selection();\n\n\t\tif (lines.length > 1) {\n\t\t\t// selection solely of a YAML block\n\t\t\tString text = String.join(System.lineSeparator(), lines);\n\t\t\tCommand blockCmd = CommandRegistry.detect(text);\n\t\t\tif (blockCmd instanceof CallEditCommand) {\n\t\t\t\targ.command = blockCmd;\n\t\t\t\treturn;\n\t\t\t}\n\n\t\t\targ.inputs.add(removeCommentLines(sel));\n\t\t\t// An /answer command starting at the beginning of the block spans the\n\t\t\t// whole selection, allowing a multi-line reason/hint for\n\t\t\t// allow and deny alike.\n\t\t\tif (blockCmd instanceof AnswerCommand ac) {\n\t\t\t\targ.command = blockCmd;\n\t\t\t\targ.yaml = captureYamlBlock(lines, 0);\n\t\t\t\tac.setYaml(arg.yaml);\n\t\t\t}\n\t\t\t// multi-line command takes precedence over a trailing command\n\t\t\telse if (!detectedCmd(lines[0], lines, 0, arg))\n\t\t\t\tdetectedCmd(lines[lines.length - 1], lines, lines.length - 1, arg);\n\t\t\treturn;\n\t\t}\n\n\t\targ.inputs.add(lines[0]);\n\t\tdetectedCmd(lines[0], lines, 0, arg);\n\t}\n"
      - id: PromptInputHandler.detectFullFile
        type: MethodDeclaration
        code: "\tprivate void detectFullFile(Selection content, PromptArguments arg) {\n\t\tif (content == null || content.selection() == null || content.selection().length == 0)\n\t\t\treturn;\n\n\t\tString[] lines = content.selection();\n\t\targ.inputs.add(String.join(\"\\n\", lines));\n\n\t\tInteger cursorLine = content.cursorOffset();\n\t\tif (cursorLine != null && cursorLine >= 0 && cursorLine < lines.length\n\t\t\t\t&& detectedCmd(lines[cursorLine], lines, cursorLine, arg))\n\t\t\treturn;\n\n\t\tint lastLine = lines.length - 1;\n\t\tdetectedCmd(lines[lastLine], lines, lastLine, arg);\n\t}\n"
      - id: PromptInputHandler.detectedCmd
        type: MethodDeclaration
        code: "\tprivate boolean detectedCmd(String line, String[] lines, int lineIndex, PromptArguments arg) {\n\t\tCommand cmd = CommandRegistry.detect(line);\n\t\tif (cmd == null)\n\t\t\treturn false;\n\t\targ.command = cmd;\n\t\targ.yaml = captureYamlBlock(lines, lineIndex);\n\t\tif (cmd instanceof AnswerCommand ac)\n\t\t\tac.setYaml(arg.yaml);\n\t\treturn true;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      nodes:
      - id: MarkerRessourceScanner.getPromptTag
        type: MethodDeclaration
        code: "\tpublic static String getPromptTag(String meta, String id) {\n\t\treturn String.format(\"[%s:%s:%s]\", AIREQ_PREFIX, meta, id);\n\t}\n"
      - id: MarkerRessourceScanner.1KGB34|MrWChh
        type: StatementGroup
        code: "\tprivate static final String AIREQ_PREFIX = \"xy.ai.req\";\n\tprivate static final String MARKER_ID = \"xy.ai.workbench.promptmarker\";\n\tprivate static final String MARKER_REQ_ID_ATTR = \"requestId\";\n\tprivate static final String MARKER_OFF_ID_ATTR = \"offset\";\n\tprivate static final String MARKER_LEN_ID_ATTR = \"length\";\n\tprivate final Pattern pattern;\n\tprivate ConfigManager cfg;\n"
```
/answer cefd-5 allow
Control Request:
```yaml
id: cefd-6
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  regexp: pattern\s*=
  reason: "Prüfen, wie 'pattern' Feld initialisiert wird und ob es dem PromptTag-Format entspricht"
```
/answer cefd-6 allow
Control Request:
```yaml
id: cefd-7
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      nodes:
      - id: MarkerRessourceScanner.MarkerRessourceScanner
        code: "\tpublic MarkerRessourceScanner(ConfigManager cfg, BundleContext context) {\n\t\tthis.cfg = cfg;\n\t\tResourcesPlugin.getWorkspace().addResourceChangeListener(this, IResourceChangeEvent.POST_CHANGE);\n\t\tpattern = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\", Pattern.CASE_INSENSITIVE);\n\t}\n"
```
/answer cefd-7 allow
ReasoningToken: 400
Control Request:
```yaml
id: cefd-8
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
  node_type: ImportDeclaration
  reason: Imports der Datei prüfen für Pattern und MarkerRessourceScanner
```
/answer cefd-8 allow
Control Request:
```yaml
id: cefd-9
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
      nodes: []
```
/answer cefd-9 allow
ReasoningToken: 474
Control Request:
```yaml
id: cefd-a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
    ids:
    - njnPQV|SJ135r
    - sEQQ8c|onrBiT
  reason: Import-Block der Datei einsehen
```
/answer cefd-a allow
Control Request:
```yaml
id: cefd-b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
      nodes:
      - id: njnPQV|SJ135r
        type: ImportGroup
        code: |
          import java.util.ArrayList;
          import java.util.Arrays;
          import java.util.List;
          import java.util.function.Consumer;
          import java.util.regex.Matcher;
          import java.util.regex.Pattern;

          import org.eclipse.swt.widgets.Display;

          import xy.ai.workbench.ActiveEditorListener;
          import xy.ai.workbench.ActiveEditorListener.Selection;
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.commands.AnswerCommand;
          import xy.ai.workbench.commands.CallEditCommand;
      - id: sEQQ8c|onrBiT
        type: ImportGroup
        code: |
          import xy.ai.workbench.commands.Command;
          import xy.ai.workbench.commands.CommandRegistry;
          import xy.ai.workbench.editor.md.AbstractRule;
```
/answer cefd-b allow
Text:
 Pattern und Matcher sind schon importiert. Nun füge ich einen öffentlichen, statischen Pattern-Check in `MarkerRessourceScanner` hinzu und nutze ihn in `buildPrompt`.
Control Request:
```yaml
id: cefd-c
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    id: MarkerRessourceScanner.getPromptTag
    old_text: "\tpublic static String getPromptTag(String meta, String id) {\n\t\treturn String.format(\"[%s:%s:%s]\", AIREQ_PREFIX, meta, id);\n\t}"
    new_text: "\tpublic static String getPromptTag(String meta, String id) {\n\t\treturn String.format(\"[%s:%s:%s]\", AIREQ_PREFIX, meta, id);\n\t}\n\n\tprivate static final Pattern PROMPT_TAG_PATTERN = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\",\n\t\t\tPattern.CASE_INSENSITIVE);\n\n\t/**\n\t * Whether the given line contains a prompt tag, i.e. the marker placeholder\n\t * inserted for an already running/pending request (see {@link #getPromptTag}).\n\t */\n\tpublic static boolean containsPromptTag(String line) {\n\t\treturn line != null && PROMPT_TAG_PATTERN.matcher(line).find();\n\t}"
  reason: Öffentliche Prüfmethode für PromptTag-Zeilen in MarkerRessourceScanner ergänzen
```
/answer cefd-c allow
Control Request:
```yaml
id: cefd-d
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      error: |
        invalid member declaration: Encountered unexpected token: "private" "private"
            at line 5, column 3.

        Was expecting:

            <EOF>
      id: MarkerRessourceScanner.getPromptTag
```
/answer cefd-d allow
Control Request:
```yaml
id: cefd-e
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    id: MarkerRessourceScanner.getPromptTag
    position: after
    source: "\n\tprivate static final Pattern PROMPT_TAG_PATTERN = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\",\n\t\t\tPattern.CASE_INSENSITIVE);\n\n\t/**\n\t * Whether the given line contains a prompt tag, i.e. the marker placeholder\n\t * inserted for an already running/pending request (see {@link #getPromptTag}).\n\t */\n\tpublic static boolean containsPromptTag(String line) {\n\t\treturn line != null && PROMPT_TAG_PATTERN.matcher(line).find();\n\t}\n"
  reason: Öffentliche Prüfmethode für PromptTag-Zeilen in MarkerRessourceScanner ergänzen
```
/answer cefd-e allow
Control Request:
```yaml
id: cefd-f
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      error: |
        invalid member declaration: Encountered unexpected token: "public" "public"
            at line 9, column 3.

        Was expecting:

            <EOF>
      id: MarkerRessourceScanner.getPromptTag
```
/answer cefd-f allow
Text:
 Ich nutze stattdessen ast_insert, um die neue Methode nach `getPromptTag` einzufügen.
Control Request:
```yaml
id: cefd-10
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    id: MarkerRessourceScanner.getPromptTag
    position: after
    source: "\tprivate static final Pattern PROMPT_TAG_PATTERN = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\", Pattern.CASE_INSENSITIVE);"
  reason: PROMPT_TAG_PATTERN-Feld separat einfügen
```
/answer cefd-10 allow
Control Request:
```yaml
id: cefd-11
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    id: MarkerRessourceScanner.pKYBlx|1bylSF
    position: after
    source: "\t/**\n\t * Whether the given line contains a prompt tag, i.e. the marker placeholder\n\t * inserted for an already running/pending request (see {@link #getPromptTag}).\n\t */\n\tpublic static boolean containsPromptTag(String line) {\n\t\treturn line != null && PROMPT_TAG_PATTERN.matcher(line).find();\n\t}"
  reason: containsPromptTag-Methode nach dem neuen Feld einfügen
```
/answer cefd-11 allow
ReasoningToken: 166
Control Request:
```yaml
id: cefd-12
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
    id: PromptInputHandler.buildPrompt
    old_text: "\t\tFrozenConfig frozen = FrozenConfig.from(cfg);\n\t\targ.processorEnabled = cfg.isInputEnabled(InputMode.Converter);"
    new_text: "\t\tfor (String input : arg.inputs)\n\t\t\tif (MarkerRessourceScanner.containsPromptTag(input))\n\t\t\t\tthrow new IllegalStateException(\"Prompt accidentally triggered on a line containing a prompt tag\");\n\n\t\tFrozenConfig frozen = FrozenConfig.from(cfg);\n\t\targ.processorEnabled = cfg.isInputEnabled(InputMode.Converter);"
  reason: Erkennung von versehentlich ausgelösten Prompts auf PromptTag-Zeilen in buildPrompt ergänzen
```
/answer cefd-12 allow
Control Request:
```yaml
id: cefd-13
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
    id: sEQQ8c|onrBiT
    old_text: import xy.ai.workbench.editor.md.AbstractRule;
    new_text: |-
      import xy.ai.workbench.editor.md.AbstractRule;
      import xy.ai.workbench.marker.MarkerRessourceScanner;
  reason: Import für MarkerRessourceScanner ergänzen
```
/answer cefd-13 allow
Text:
 Nun `buildPrompt()` erweitern, um die Erkennung durchzuführen. Ich prüfe die relevanten Input-Zeilen (Selection und Full-File) auf einen Prompt-Tag.
Control Request:
```yaml
id: cefd-14
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  reason: Kompilierbarkeit beider geänderten Dateien prüfen
```
/answer cefd-14 allow
Result Stats: id=6ee053a3-4f36-471c-ba17-17e9fb9f6cef, total: 23062, in: 157, out: 5145, reason: 0, read: 162304, write: 17760
Text:
 Jetzt den fehlenden Import ergänzen und validieren.
Beide Dateien kompilieren erfolgreich. Zusammenfassung der Änderung:

- `MarkerRessourceScanner`: neues `PROMPT_TAG_PATTERN` + `containsPromptTag(String line)` zur Erkennung von Zeilen mit PromptTag.
- `PromptInputHandler.buildPrompt()`: prüft alle gesammelten `arg.inputs` und wirft `IllegalStateException`, falls eine Zeile einen PromptTag enthält.
