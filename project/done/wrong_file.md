Es gibt noch einen Bug bei der parallelen Verwendung des Autoapprovers und dem wechsel des Editor.
Der Tag landet beim Promptende nicht im richtigen Editor.

Tag Replacement soll natürlich in der Datei geschehen, in der der Prompt gestartet wurde.
`/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java`
SystemInit:  id=7374f60d-efa7-4948-b20e-3bbb5dcfe178, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: cd56-1-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  reason: Understand structure of AutoApprover to find tag replacement logic
```
/answer cd56-1-0 allow
Control Request:
```yaml
id: cd56-2-1
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: Zpzs7d|sxLd1L
        excert: import java.util.ArrayList;
      - id: AutoApprover
        excert: "public class AutoApprover {"
        children:
        - id: AutoApprover.u5j4A4|wS5sLU
          excert: private final RuleRegistry registry;
        - id: AutoApprover.AutoApprover
          excert: public AutoApprover()
        - id: AutoApprover.AutoApprover_2
          excert: public AutoApprover(RuleRegistry registry)
        - id: AutoApprover.rules
          excert: public RuleRegistry rules()
        - id: AutoApprover.addListener
          excert: public void addListener(Consumer<Decision> listener)
        - id: AutoApprover.removeListener
          excert: public void removeListener(Consumer<Decision> listener)
        - id: AutoApprover.isEnabled
          excert: public boolean isEnabled()
        - id: AutoApprover.setEnabled
          excert: public void setEnabled(boolean enabled)
          docstring: "Globally switches the processing on or off; while off, answers pass untouched."
        - id: AutoApprover.setIdleInstruction
          excert: public void setIdleInstruction(String instruction)
        - id: AutoApprover.forget
          excert: public void forget(String sessionId)
        - id: AutoApprover.evaluate
          excert: "public Decision evaluate(AIAnswer ans, int depth)"
        - id: AutoApprover.omitDenied
          excert: "private static void omitDenied(AIAnswer ans, Decision decision)"
        - id: AutoApprover.onIdle
          excert: public Optional<FollowUp> onIdle(String sessionId)
        - id: AutoApprover.collect
          excert: private List<RuleVote> collect(ApprovalContext ctx)
        - id: AutoApprover.decide
          excert: "private Decision decide(ApprovalContext ctx, List<RuleVote> votes)"
        - id: AutoApprover.resultFor
          excert: "private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolea…"
        - id: AutoApprover.followUpFor
          excert: "private FollowUp followUpFor(ApprovalContext ctx, ApprovalResult result, String…"
        - id: AutoApprover.withBlock
          excert: "private static List<String> withBlock(ApprovalItem item, String commandLine)"
        - id: AutoApprover.withBlock_2
          excert: "private static List<String> withBlock(String yaml, String commandLine)"
        - id: AutoApprover.deniedYaml
          excert: private static String deniedYaml(ApprovalItem item)
        - id: AutoApprover.denial
          excert: "private static String denial(ApprovalItem item, String reason)"
        - id: AutoApprover.command
          excert: "private static String command(String base, String extra)"
        - id: AutoApprover.oneLine
          excert: private static String oneLine(String s)
        - id: AutoApprover.selection
          excert: private static Selection selection(List<String> lines)
        - id: AutoApprover.addNonBlank
          excert: "private static void addNonBlank(List<String> list, String text)"
        - id: AutoApprover.enrich
          excert: "private static void enrich(AIAnswer ans, List<String> warnings)"
          docstring: Warnings go in front so the command line stays the last line of the answer.
        - id: AutoApprover.publish
          excert: private void publish(Decision decision)
```
/answer cd56-2-1 allow
Control Request:
```yaml
id: cd56-3-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover.evaluate
    - AutoApprover.onIdle
    - AutoApprover.followUpFor
  reason: Read evaluate/onIdle/followUpFor to see how tags are handled
```
/answer cd56-3-0 allow
Control Request:
```yaml
id: cd56-4-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover.evaluate
        type: MethodDeclaration
        code: |2
            public Decision evaluate(AIAnswer ans, int depth) {
              if (!enabled)
                return Decision.pass(ans.sessionId);
              SessionSnapshot session = sessions.record(ans);
              List<ApprovalItem> items = AnswerParser.parse(ans.answer);
              ApprovalItem item = items.size() == 1 ? items.get(0) : null;
              sessions.track(session.id(), items);
              ApprovalItem call = sessions.callFor(session.id(), item);
              ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, call, session, registry, depth);
              Decision decision = decide(ctx, collect(ctx));
              omitDenied(ans, decision);
              enrich(ans, decision.warnings());
              publish(decision);
              return decision;
            }
      - id: AutoApprover.onIdle
        type: MethodDeclaration
        code: |2
            public Optional<FollowUp> onIdle(String sessionId) {
              if (!enabled)
                return Optional.empty();
              Prompt origin = sessions.lastPrompt(sessionId);
              String instruction = idleInstruction;
              if (origin == null || instruction.isBlank())
                return Optional.empty();
              FollowUp followUp = new FollowUp(origin, selection(instruction.lines().toList()));
              publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp, 0));
              return Optional.of(followUp);
            }
      - id: AutoApprover.followUpFor
        type: MethodDeclaration
        code: |2
            private FollowUp followUpFor(ApprovalContext ctx, ApprovalResult result, String reason, String hint, String instruction) {
              Prompt origin = ctx.prompt();
              ApprovalItem item = ctx.item();
              if (origin == null)
                return null;
              List<String> lines = switch(result) {
                case Pass ->
                  instruction.isBlank() ? null : instruction.lines().toList();
                case Allow ->
                  withBlock(item, command(AnswerCommand.CMD_ANSWER + " " + item.id() + " allow", hint));
                case Deny ->
                  item.isControl() ? withBlock(deniedYaml(item), command(AnswerCommand.CMD_ANSWER + " " + item.id() + " deny", reason)) : List.of(denial(item, reason));
                case Call ->
                  withBlock(item, CallCommand.CMD_CALL + " " + item.id());
              };
              return lines == null ? null : new FollowUp(origin, selection(lines));
            }
```
/answer cd56-4-3 allow
ReasoningToken: 17
Control Request:
```yaml
id: cd56-5-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: "[Tt]ag[A-Za-z]*(Replace|replace)|replaceTag|TagReplace"
    include:
    - '*.java'
    exclude:
    - '**/target/**'
    - '**/bin/**'
    limit: 30
  reason: Locate tag replacement code
```
/answer cd56-5-0 allow
Control Request:
```yaml
id: cd56-6-5
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/EditorInterface.java
        matches:
        - "public void replaceTag(AIAnswer ans, IProgress mon) {"
      - path: src/xy/ai/workbench/view/AIBatchView.java
        matches:
        - "Activator.getDefault().editorInterface.replaceTag((AIAnswer) obj, new EclipseProgress(mon));"
      - path: connector/src/xy/ai/conscious/core/IEditorInterface.java
        matches:
        - "public void replaceTag(AIAnswer ans, IProgress mon);"
      - path: connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - "* the prepare/insertTag/execute/replaceTag job pipeline, delegating input"
        - "editIfc.replaceTag(ans, sub);"
        - "editIfc.replaceTag(error, sub);"
```
/answer cd56-6-5 allow
Control Request:
```yaml
id: cd56-7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: replaceTag|insertTag|getActiveEditor|activeEditor|getActivePage|getActiveWorkbench
    include:
    - EditorInterface.java
    limit: 40
  reason: Find how EditorInterface resolves the editor
```
/answer cd56-7-0 allow
Control Request:
```yaml
id: cd56-8-7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: EditorInterface.java
        matches:
        - "public void insertTag(IExecutor exec, IModelRequest req, IProgress mon) {"
        - IWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();
        - "public void replaceTag(AIAnswer ans, IProgress mon) {"
```
/answer cd56-8-7 allow
Control Request:
```yaml
id: cd56-9-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  reason: Get node ids
```
/answer cd56-9-0 allow
Control Request:
```yaml
id: cd56-a-9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: 1beUBs|3Q9CR6
        excert: import java.io.ByteArrayInputStream;
      - id: bHiiRy|rWc6m1
        excert: import xy.ai.conscious.core.ConfigManager;
      - id: EditorInterface
        excert: "public class EditorInterface implements IEditorInterface {"
        children:
        - id: EditorInterface.fZITGq|vDMTwT
          excert: public static final String USER = IEditorInterface.USER;
        - id: EditorInterface.EditorInterface
          excert: "public EditorInterface(ConfigManager cfg, ActiveEditorListener editorListener, …"
        - id: EditorInterface.register
          excert: public void register(BundleContext context) throws Exception
        - id: EditorInterface.dispose
          excert: public void dispose(BundleContext context)
        - id: EditorInterface.insertTag
          excert: "public void insertTag(IExecutor exec, IModelRequest req, IProgress mon)"
        - id: EditorInterface.generateTag
          excert: private String generateTag(IModelRequest req)
        - id: EditorInterface.replaceTag
          excert: "public void replaceTag(AIAnswer ans, IProgress mon)"
```
/answer cd56-a-9 allow
Control Request:
```yaml
id: cd56-b-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    ids:
    - EditorInterface.insertTag
    - EditorInterface.replaceTag
    - EditorInterface.EditorInterface
  reason: Read tag insert/replace logic
```
/answer cd56-b-0 allow
Control Request:
```yaml
id: cd56-c-b
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
      - id: EditorInterface.EditorInterface
        type: ConstructorDeclaration
        code: "\tpublic EditorInterface(ConfigManager cfg, ActiveEditorListener editorListener, AdaptingConnector connector) {\n\t\tthis.cfg = cfg;\n\t\tthis.editorListener = editorListener;\n\t\tthis.connector = connector;\n\t}\n"
```
/answer cd56-c-b allow
Control Request:
```yaml
id: cd56-d-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: findAndReplaceMarkers|class .*MarkerScanner
    include:
    - '*.java'
    exclude:
    - '**/target/**'
    - '**/bin/**'
    limit: 10
    intent: editing
  reason: Locate marker scanner that picks the editor for replacement
```
/answer cd56-d-0 allow
Control Request:
```yaml
id: cd56-e-d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/EditorInterface.java
        nodes:
        - id: EditorInterface.replaceTag
          matches:
          - if (!markerScanner.findAndReplaceMarkers(ans))
      - path: src/xy/ai/workbench/marker/MarkerRessourceScanner.java
        nodes:
        - id: MarkerRessourceScanner.findAndReplaceMarkers
          matches:
          - "public boolean findAndReplaceMarkers(AIAnswer ans) {"
```
/answer cd56-e-d allow
Control Request:
```yaml
id: cd56-f-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  reason: See scanner structure
```
/answer cd56-f-0 allow
Control Request:
```yaml
id: cd56-10-f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      nodes:
      - id: EnIoex|zidzZW
        excert: import java.io.BufferedReader;
      - id: RgSv9x|9C3gUk
        excert: import org.eclipse.jface.text.BadLocationException;
      - id: wRD3M9|xFNZQY
        excert: import xy.ai.conscious.model.AIAnswer;
      - id: MarkerRessourceScanner
        excert: "public class MarkerRessourceScanner implements IResourceChangeListener, IResour…"
        children:
        - id: MarkerRessourceScanner.1KGB34|BKcKaJ
          excert: private static final String MARKER_ID = "xy.ai.workbench.promptmarker";
        - id: MarkerRessourceScanner.MarkerRessourceScanner
          excert: "public MarkerRessourceScanner(ConfigManager cfg, BundleContext context)"
        - id: MarkerRessourceScanner.dispose
          excert: public void dispose(BundleContext context)
        - id: MarkerRessourceScanner.visit
          excert: public boolean visit(IResourceDelta delta) throws CoreException
        - id: MarkerRessourceScanner.rescannFile
          excert: private void rescannFile(IFile file)
        - id: MarkerRessourceScanner.resourceChanged
          excert: public void resourceChanged(IResourceChangeEvent event)
        - id: MarkerRessourceScanner.findAndReplaceMarkers
          excert: public boolean findAndReplaceMarkers(AIAnswer ans)
          docstring: '@param ans'
        - id: MarkerRessourceScanner.replaceMarker
          excert: "private boolean replaceMarker(AIAnswer ans, IMarker marker)"
        - id: MarkerRessourceScanner.resolveTagRange
          excert: "private int[] resolveTagRange(IDocument doc, String requestId, int off, int len)"
          docstring: Verifies that the given offset/length still points at the tag belonging to
        - id: MarkerRessourceScanner.findTagInDocument
          excert: "private int[] findTagInDocument(IDocument doc, String requestId)"
          docstring: Scans the full doc content for the tag belonging to the given request id.
        - id: MarkerRessourceScanner.tryReplaceInEditor
          excert: "private boolean tryReplaceInEditor(AIAnswer ans, ITextEditor editor)"
          docstring: "Fallback used when the marker based replacement failed, e.g. because the"
        - id: MarkerRessourceScanner.replaceInDocument
          excert: "private boolean replaceInDocument(AIAnswer ans, ITextEditor editor, IDocument d…"
        - id: MarkerRessourceScanner.findAndReplaceInOpenEditors
          excert: private boolean findAndReplaceInOpenEditors(AIAnswer ans)
        - id: MarkerRessourceScanner.findOpenEditorFor
          excert: private ITextEditor findOpenEditorFor(IFile file)
        - id: MarkerRessourceScanner.isAutoFollowModeEnabled
          excert: private boolean isAutoFollowModeEnabled()
        - id: MarkerRessourceScanner.shouldAutoFollow
          excert: "private boolean shouldAutoFollow(ITextEditor editor, IDocument doc)"
        - id: MarkerRessourceScanner.AutoFollowState
          excert: "/**  * Holds, per doc, the auto-follow {"
          docstring: "Holds, per doc, the auto-follow {@link IDocumentListener} together with the"
          children:
          - id: MarkerRessourceScanner.AutoFollowState.hrG5hD|T7P2Br
            excert: final ITextEditor editor;
          - id: MarkerRessourceScanner.AutoFollowState.AutoFollowState
            excert: "AutoFollowState(ITextEditor editor, IDocumentListener listener)"
        - id: MarkerRessourceScanner.QY71kA|WDmgDC
          excert: "private final Map<IDocument, AutoFollowState> autoFollowListeners = new HashMap…"
        - id: MarkerRessourceScanner.moveCursorToLastLineStart
          excert: "private void moveCursorToLastLineStart(ITextEditor editor, IDocument doc)"
          docstring: "Moves the cursor to the end of the doc (start of the last line, i.e. an empty"
        - id: MarkerRessourceScanner.ensureAutoFollowListener
          excert: "private void ensureAutoFollowListener(ITextEditor editor, IDocument doc)"
        - id: MarkerRessourceScanner.ensurePartCloseCleanupRegistered
          excert: private void ensurePartCloseCleanupRegistered()
        - id: MarkerRessourceScanner.unwrapTextEditor
          excert: private ITextEditor unwrapTextEditor(IWorkbenchPart part)
        - id: MarkerRessourceScanner.removeAutoFollowListenerFor
          excert: private void removeAutoFollowListenerFor(ITextEditor editor)
        - id: MarkerRessourceScanner.tryMoveCursorToLastLineStart
          excert: "private boolean tryMoveCursorToLastLineStart(ITextEditor editor, IDocument doc)"
          docstring: Attempts to place the cursor at the end of the doc and verifies that the
        - id: MarkerRessourceScanner.getOpenTextEditors
          excert: private java.util.List<ITextEditor> getOpenTextEditors()
```
/answer cd56-10-f allow
Control Request:
```yaml
id: cd56-11-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    ids:
    - MarkerRessourceScanner.findAndReplaceMarkers
    - MarkerRessourceScanner.replaceMarker
    - MarkerRessourceScanner.resolveTagRange
    - MarkerRessourceScanner.findTagInDocument
    - MarkerRessourceScanner.tryReplaceInEditor
    - MarkerRessourceScanner.replaceInDocument
    - MarkerRessourceScanner.findAndReplaceInOpenEditors
    - MarkerRessourceScanner.findOpenEditorFor
  reason: Read replacement logic to find where the wrong editor is chosen
```
/answer cd56-11-0 allow
Control Request:
```yaml
id: cd56-12-11
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      nodes:
      - id: MarkerRessourceScanner.findAndReplaceMarkers
        type: MethodDeclaration
        code: |2
            public boolean findAndReplaceMarkers(AIAnswer ans) {
              ITextEditor hint = ans.prompt != null ? (ITextEditor) ans.prompt.arg.getEditor() : null;
              if (hint != null && tryReplaceInEditor(ans, hint))
                return true;
              IWorkspaceRoot root = ResourcesPlugin.getWorkspace().getRoot();
              boolean res = false;
              try {
                IMarker[] markers = root.findMarkers(MARKER_ID, true, IResource.DEPTH_INFINITE);
                for (IMarker marker : markers) if (ans.id.equals(marker.getAttribute(MARKER_REQ_ID_ATTR)))
                  res |= replaceMarker(ans, marker);
              } catch (CoreException e) {
                LOG.error(e.getMessage(), e);
              }
              // Fallback: the marker's stored position is outdated
              if (!res)
                res = findAndReplaceInOpenEditors(ans);
              return res;
            }
      - id: MarkerRessourceScanner.replaceMarker
        type: MethodDeclaration
        code: |2
            private boolean replaceMarker(AIAnswer ans, IMarker marker) {
              IResource resource = marker.getResource();
              if (!(resource instanceof IFile))
                return false;
              IFile file = (IFile) resource;
              int line = marker.getAttribute(IMarker.LINE_NUMBER, -1);
              if (line <= 0)
                return false;
              ITextFileBufferManager bm = FileBuffers.getTextFileBufferManager();
              boolean[] replaced = { false };
              try {
                bm.connect(file.getFullPath(), LocationKind.IFILE, null);
                ITextFileBuffer tb = bm.getTextFileBuffer(file.getFullPath(), LocationKind.IFILE);
                Display.getDefault().syncExec(() -> {
                  try {
                    IDocument doc = tb.getDocument();
                    int off = marker.getAttribute(MARKER_OFF_ID_ATTR, -1);
                    int len = marker.getAttribute(MARKER_LEN_ID_ATTR, -1);
                    int[] range = resolveTagRange(doc, ans.id, off, len);
                    if (range == null)
                      // stored position no longer matches the live doc
                      return;
                    ITextEditor editor = null;
                    if (isAutoFollowModeEnabled())
                      editor = findOpenEditorFor(file);
                    boolean autoFollow = shouldAutoFollow(editor, doc);
                    doc.replace(range[0], range[1], ans.answer);
                    replaced[0] = true;
                    if (autoFollow)
                      moveCursorToLastLineStart(editor, doc);
                  } catch (BadLocationException e) {
                    LOG.error(e.getMessage(), e);
                  }
                });
                if (replaced[0])
                  try {
                    tb.commit(null, false);
                    marker.delete();
                    return true;
                  } catch (IllegalStateException e) {
                    // ignore when not found and fall back
                    return false;
                  }
              } catch (CoreException e) {
                LOG.error(e.getMessage(), e);
              } finally {
                try {
                  bm.disconnect(file.getFullPath(), LocationKind.IFILE, null);
                } catch (CoreException e) {
                  LOG.error(e.getMessage(), e);
                }
              }
              return false;
            }
      - id: MarkerRessourceScanner.resolveTagRange
        type: MethodDeclaration
        code: |2
            private int[] resolveTagRange(IDocument doc, String requestId, int off, int len) {
              if (off >= 0 && len >= 0 && off + len <= doc.getLength())
                try {
                  String candidate = doc.get(off, len);
                  Matcher m = pattern.matcher(candidate);
                  if (m.matches() && requestId.equals(m.group(2)))
                    return new int[] { off, len };
                } catch (BadLocationException e) {
                  // fall through to full-doc search
                }
              return findTagInDocument(doc, requestId);
            }
      - id: MarkerRessourceScanner.findTagInDocument
        type: MethodDeclaration
        code: |2
            private int[] findTagInDocument(IDocument doc, String requestId) {
              String content = doc.get();
              Matcher m = pattern.matcher(content);
              while (m.find()) if (requestId.equals(m.group(2)))
                return new int[] { m.start(), m.end() - m.start() };
              return null;
            }
      - id: MarkerRessourceScanner.tryReplaceInEditor
        type: MethodDeclaration
        code: |2
            private boolean tryReplaceInEditor(AIAnswer ans, ITextEditor editor) {
              boolean[] res = { false };
              Display.getDefault().syncExec(() -> {
                try {
                  IDocument doc = editor.getDocumentProvider() != null ? editor.getDocumentProvider().getDocument(editor.getEditorInput()) : null;
                  if (doc != null)
                    res[0] = replaceInDocument(ans, editor, doc);
                } catch (RuntimeException e) {
                  // hint editor may have been closed/disposed in the meantime
                }
              });
              return res[0];
            }
      - id: MarkerRessourceScanner.replaceInDocument
        type: MethodDeclaration
        code: |2
            private boolean replaceInDocument(AIAnswer ans, ITextEditor editor, IDocument doc) {
              int[] range = findTagInDocument(doc, ans.id);
              if (range == null)
                return false;
              try {
                boolean autoFollow = isAutoFollowModeEnabled() && shouldAutoFollow(editor, doc);
                doc.replace(range[0], range[1], ans.answer);
                if (autoFollow)
                  moveCursorToLastLineStart(editor, doc);
                return true;
              } catch (BadLocationException e) {
                LOG.error(e.getMessage(), e);
                return false;
              }
            }
      - id: MarkerRessourceScanner.findAndReplaceInOpenEditors
        type: MethodDeclaration
        code: |2
            private boolean findAndReplaceInOpenEditors(AIAnswer ans) {
              boolean[] res = { false };
              Display.getDefault().syncExec(() -> {
                for (ITextEditor editor : getOpenTextEditors()) {
                  IDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());
                  if (doc == null)
                    continue;
                  if (replaceInDocument(ans, editor, doc)) {
                    res[0] = true;
                    break;
                  }
                }
              });
              return res[0];
            }
      - id: MarkerRessourceScanner.findOpenEditorFor
        type: MethodDeclaration
        code: |2
            private ITextEditor findOpenEditorFor(IFile file) {
              for (ITextEditor editor : getOpenTextEditors()) {
                IEditorInput input = editor.getEditorInput();
                if (input instanceof IFileEditorInput && file.equals(((IFileEditorInput) input).getFile()))
                  return editor;
              }
              return null;
            }
```
/answer cd56-12-11 allow
ReasoningToken: 100
Control Request:
```yaml
id: cd56-13-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: \.arg\b|getEditor\(\)|setEditor\(|FollowUp\b.*(origin|prompt)|getLastTextEditor
    include:
    - '*.java'
    exclude:
    - '**/target/**'
    - '**/bin/**'
    limit: 50
  reason: Trace where prompt arg editor is set and how follow-ups derive prompts
```
/answer cd56-13-0 allow
Control Request:
```yaml
id: cd56-14-13
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/EditorInterface.java
        matches:
        - ITextEditor textEditor = editorListener.getLastTextEditor();
      - path: src/xy/ai/workbench/ActiveEditorListener.java
        matches:
        - "public ITextEditor getLastTextEditor() {"
        - return editorListener.getLastTextEditor();
        - ITextEditor edit = getLastTextEditor();
        - IDocument doc = getLastTextEditor().getDocumentProvider().getDocument(getLastTextEditor().getEditorInput());
        - ITextEditor textEditor = getLastTextEditor();
        - editor = ((AISessionEditor) part).getEditor();
      - path: src/xy/ai/workbench/editor/CommentHandler.java
        matches:
        - ITextEditor editor = ((AISessionEditor) HandlerUtil.getActiveEditorChecked(event)).getEditor();
      - path: src/xy/ai/workbench/editor/EditorChangeListener.java
        matches:
        - "public ITextEditor getLastTextEditor() {"
      - path: src/xy/ai/workbench/editor/AISessionEditor.java
        matches:
        - "public AITextEditor getEditor() {"
        - getEditor().doSave(sub.split(1));
        - IEditorPart editor = getEditor();
      - path: src/xy/ai/workbench/marker/MarkerRessourceScanner.java
        matches:
        - "ITextEditor hint = ans.prompt != null ? (ITextEditor) ans.prompt.arg.getEditor() : null;"
        - part = ((AISessionEditor) part).getEditor();
      - path: src/xy/ai/workbench/eclipse/GitSupport.java
        matches:
        - Command cmd = prompt.arg.command;
        - String opId = extractYamlId(prompt.arg.yaml);
        - OpSnapshotter snap = new OpSnapshotter(prompt.arg.project.toFile());
      - path: connector/src/xy/ai/conscious/api/IActiveEditor.java
        matches:
        - "default Object getLastTextEditor() {"
      - path: connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - "FollowUp followUp = new FollowUp(origin, selection(instruction.lines().toList()));"
        - "return lines == null ? null : new FollowUp(origin, selection(lines));"
      - path: connector/src/xy/ai/conscious/autoapprove/FollowUp.java
        matches:
        - "public record FollowUp(Prompt origin, Selection selection) {"
      - path: connector/src/xy/ai/conscious/autoapprove/rules/DenyOwnPromptFileRule.java
        matches:
        - if (item == null || prompt == null || prompt.arg.absoluteFilePath == null || prompt.arg.absoluteFilePath.isBlank())
        - file = Path.of(prompt.arg.absoluteFilePath).toAbsolutePath().normalize();
      - path: connector/src/xy/ai/conscious/connector/mcp/MCPConnector.java
        matches:
        - Command command = prompt.arg.command;
        - "return new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools, command, prompt.arg.yaml);"
      - path: connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
        matches:
        - Command command = prompt.arg.command;
        - "return new CCRequest(UUID.randomUUID().toString(), title, prompt.config, promptText, prompt.arg);"
        - String relativeFilePath = req.arg.project.relativize(Paths.get(req.arg.absoluteFilePath)).toString();
        - "CCSessionParameters params = new CCSessionParameters(req.arg.project, fc.systemPrompt, fc.tools, fc.model, fc.reasoning, fc.profile, fc.keys, fc.cacheMode, relativeFilePath);"
        - Command command = req.arg.command;
        - "String selectedUuid = req.arg.sessionUuid != null ? req.arg.sessionUuid : sessionManager.getSelectedSessionUuid();"
        - if (req.arg.yaml == null || req.arg.yaml.isBlank())
        - if (!ts.control(() -> controlClient.submitEdit(req.arg.yaml)))
        - if (req.arg.sessionUuid == null)
        - req.arg.sessionUuid = session.getSessionUuid();
      - path: connector/src/xy/ai/conscious/connector/claudecode/CCRequest.java
        matches:
        - this.arg = arg;
      - path: connector/src/xy/ai/conscious/connector/google/GeminiConnector.java
        matches:
        - "sessionProcessor.process(prompt.inputs, prompt.arg.processorEnabled, cb);"
      - path: connector/src/xy/ai/conscious/connector/claude/ClaudeConnector.java
        matches:
        - "sessionProcessor.process(prompt.inputs, prompt.arg.processorEnabled, cb);"
      - path: connector/src/xy/ai/conscious/connector/openai/OpenAIConnector.java
        matches:
        - "sessionProcessor.process(prompt.inputs, prompt.arg.processorEnabled, cb);"
      - path: connector/src/xy/ai/conscious/connector/deepseek/DeepSeekConnector.java
        matches:
        - "for (ObjectNode item : sessionProcessor.process(prompt.inputs, prompt.arg.processorEnabled,"
      - path: connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - if (prompt.arg.command instanceof CallCommand || prompt.arg.command instanceof ToolCommand)
        - if (prompt == null || prompt.arg == null)
        - Command cmd = prompt.arg.command;
      - path: connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
        matches:
        - arg.setEditor(editorListener.getLastTextEditor());
        - if (arg.getEditor() == null && !batch)
        - arg.setEditor(origin.arg.getEditor());
        - arg.absoluteFilePath = origin.arg.absoluteFilePath;
        - arg.project = origin.arg.project;
      warning: "Limit of 50 matches reached (total 55); narrow pattern, directory or include/exclude filters."
```
/answer cd56-14-13 allow
Control Request:
```yaml
id: cd56-15-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
  text: getEditor
  reason: Read code around editor assignment for follow-ups
```
/answer cd56-15-0 allow
Control Request:
```yaml
id: cd56-16-15
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
      nodes:
      - id: PromptInputHandler.buildPrompt
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
      - id: PromptInputHandler.followUp
        code: |2
            public Prompt followUp(Prompt origin, Selection sel) {
              PromptArguments arg = new PromptArguments();
              arg.setEditor(origin.arg.getEditor());
              arg.absoluteFilePath = origin.arg.absoluteFilePath;
              arg.project = origin.arg.project;
              arg.processorEnabled = origin.arg.processorEnabled;
              arg.sessionUuid = origin.arg.sessionUuid;
              detectSelection(sel, arg);
              return new Prompt(arg.inputs, origin.batch, origin.config.copy(), arg);
            }
```
/answer cd56-16-15 allow
ReasoningToken: 263
Control Request:
```yaml
id: cd56-17-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: (Object|ITextEditor) (getEditor|editor)\b|void setEditor
    include:
    - PromptArguments.java
    exclude:
    - '**/target/**'
    intent: editing
  reason: Check getEditor type in PromptArguments
```
/answer cd56-17-0 allow
Control Request:
```yaml
id: cd56-18-17
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
        nodes:
        - id: PromptArguments.setEditor
          matches:
          - "public void setEditor(Object editor) {"
        - id: PromptArguments.getEditor
          matches:
          - "public Object getEditor() {"
```
/answer cd56-18-17 allow
Control Request:
```yaml
id: cd56-19-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    id: EditorInterface.insertTag
    old_text: ITextEditor textEditor = editorListener.getLastTextEditor();
    new_text: "Object origin = req.getPrompt().arg != null ? req.getPrompt().arg.getEditor() : null;\n\t\t\t\tITextEditor textEditor = origin instanceof ITextEditor ? (ITextEditor) origin : editorListener.getLastTextEditor();"
  reason: "Insert the tag into the editor the prompt was started in, not the currently active one"
```
/answer cd56-19-0 allow
ReasoningToken: 66
Control Request:
```yaml
id: cd56-1a-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  reason: Verify compile
```
/answer cd56-1a-0 allow
Result Stats: id=7374f60d-efa7-4948-b20e-3bbb5dcfe178, total: 28367, in: 26, out: 3752, reason: 0, read: 179508, write: 24589, start: 1794858712134, end: 1794858718902, time: 6691, total: 53530, io: 0, control: 77, sent: 0, recv: 114
Ich habe den Fehler in `EditorInterface.insertTag` behoben, nicht in `AutoApprover.java`. Ich habe nur geprüft, dass die Datei kompiliert. Getestet habe ich nicht, auch nicht mit Autoapprover und Editorwechsel.

**Ursache:** `AutoApprover` erzeugt Follow-up-Prompts mit dem Editor des ursprünglichen Prompts. `PromptInputHandler.followUp` übernimmt `origin.arg.getEditor()`, und `MarkerRessourceScanner.findAndReplaceMarkers` nutzt diesen Editor als Hinweis für das Ersetzen. `insertTag` hat den Tag aber in `editorListener.getLastTextEditor()` geschrieben, also in den Editor, der gerade aktiv ist. Wechselte man den Editor, während der Autoapprover lief, landete der Tag des Follow-ups im falschen Editor. Beim Promptende fand der Hinweis-Editor den Tag dann nicht.

**Änderung:** `insertTag` nimmt jetzt `req.getPrompt().arg.getEditor()`, also den Editor, in dem der Prompt gestartet wurde. Nur wenn dort kein `ITextEditor` gesetzt ist, nimmt es wie bisher den zuletzt aktiven. Tag-Einfügen und Tag-Ersetzen gehen damit in dieselbe Datei.

Der Modus `New_File` ist unverändert. Er legt die neue Datei weiter im zuletzt aktiven Container an (`getLastContainer`).
Was auch möglich wäre ist alle offenen Editoren mit der Promptdatei abggleichen oder den Tag in die Datei ohne Editor schreiben.
Result Stats: id=7374f60d-efa7-4948-b20e-3bbb5dcfe178, total: 29749, in: 28, out: 4484, reason: 158, read: 204097, write: 25237, start: 1794858818958, end: 1794858825635, time: 6612, total: 60142, io: 0, control: 65, sent: 1, recv: 138
SystemInit:  id=7374f60d-efa7-4948-b20e-3bbb5dcfe178, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 158
Beides gibt es beim Ersetzen schon. Der Fehler lag beim Einfügen.

- **Alle offenen Editoren abgleichen:** `findAndReplaceMarkers` macht das als Fallback. `findAndReplaceInOpenEditors` sucht die Tag-ID in allen offenen Text-Editoren.
- **Ohne Editor in die Datei schreiben:** `replaceMarker` schreibt über den `ITextFileBuffer` direkt in die Datei. Das läuft über den Marker, der an der Datei hängt.

Die Ersetzung wäre also auch bei einem falsch geratenen Editor oft noch durchgekommen, weil sie den Tag per ID in den offenen Editoren sucht. Der Tag stand aber im falschen Editor. Deshalb hat der Autoapprover dort ersetzt und nicht in der Promptdatei.

Mein Fix löst das für den Normalfall. Offen bleibt der Fall, dass der Ursprungs-Editor während des Laufs geschlossen wurde. Dann ist `arg.getEditor()` ein disposed Editor, und `insertTag` würde beim Zugriff auf den Document-Provider fehlschlagen.

Dafür könnte ich `insertTag` robuster machen:
1. Zuerst den Editor aus `arg` nehmen, wenn er noch offen ist.
2. Sonst unter den offenen Editoren den mit `arg.absoluteFilePath` suchen.
3. Sonst den Tag über den `ITextFileBuffer` direkt in die Datei schreiben, ohne Editor.

Soll ich das umsetzen?
Ja und berücksichtige auch den initialen Fall, nämlich das der tag beim auslösen eines Promptes auch in die richtige Datei muss.
SystemInit:  id=7374f60d-efa7-4948-b20e-3bbb5dcfe178, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: cd56-1b-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    ids:
    - 1beUBs|3Q9CR6
    - bHiiRy|rWc6m1
    - EditorInterface.fZITGq|vDMTwT
    - EditorInterface.generateTag
  reason: Read imports and fields before refactoring insertTag
```
/answer cd56-1b-0 allow
ReasoningToken: 144
Control Request:
```yaml
id: cd56-1c-1b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: 1beUBs|3Q9CR6
        type: ImportGroup
        code: |
          import java.io.ByteArrayInputStream;
          import java.io.InputStream;
          import java.io.UnsupportedEncodingException;
          import java.time.LocalDateTime;
          import java.time.format.DateTimeFormatter;
          import org.eclipse.core.resources.IContainer;
          import org.eclipse.core.resources.IFile;
          import org.eclipse.core.runtime.CoreException;
          import org.eclipse.core.runtime.NullProgressMonitor;
          import org.eclipse.core.runtime.Path;
          import org.eclipse.jface.text.BadLocationException;
          import org.eclipse.jface.text.IDocument;
          import org.eclipse.jface.text.ITextSelection;
          import org.eclipse.jface.viewers.ISelection;
          import org.eclipse.ui.IWorkbenchPage;
          import org.eclipse.ui.PartInitException;
          import org.eclipse.ui.PlatformUI;
          import org.eclipse.ui.ide.IDE;
          import org.eclipse.ui.texteditor.ITextEditor;
          import org.osgi.framework.BundleContext;
          import xy.ai.conscious.api.IExecutor;
          import xy.ai.conscious.api.IExecutor.IProgress;
          import xy.ai.conscious.connector.AdaptingConnector;
      - id: bHiiRy|rWc6m1
        type: ImportGroup
        code: |
          import xy.ai.conscious.core.ConfigManager;
          import xy.ai.conscious.core.IEditorInterface;
          import xy.ai.conscious.core.LOG;
          import xy.ai.conscious.core.OutputMode;
          import xy.ai.conscious.model.AIAnswer;
          import xy.ai.conscious.model.IModelRequest;
          import xy.ai.conscious.model.Model.KeyPattern;
          import xy.ai.workbench.marker.MarkerRessourceScanner;
      - id: EditorInterface.fZITGq|vDMTwT
        type: StatementGroup
        code: |2
            public static final String USER = IEditorInterface.USER;
            public static final String AGENT = IEditorInterface.AGENT;
            public static final String TEXT = IEditorInterface.TEXT;
            public static final String THINKING = IEditorInterface.THINKING;
            public static final String THINKING_META = IEditorInterface.THINKING_META;
            public static final String TOOLUSE = IEditorInterface.TOOLUSE;
            public static final String TOOLRESULT = IEditorInterface.TOOLRESULT;
            public static final String CONTROL_REQUEST = IEditorInterface.CONTROL_REQUEST;
            private final ConfigManager cfg;
            private final ActiveEditorListener editorListener;
            private final AdaptingConnector connector;
            private MarkerRessourceScanner markerScanner;
      - id: EditorInterface.generateTag
        type: MethodDeclaration
        code: |2
            private String generateTag(IModelRequest req) {
              KeyPattern pattern = connector.getConnector(req).getSupportedKeyPattern();
              return IEditorInterface.getPromptTag(pattern.name(), req.getID());
            }
```
/answer cd56-1c-1b allow
Control Request:
```yaml
id: cd56-1d-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  name: getOpenTextEditors
  reason: See how open editors are enumerated
```
/answer cd56-1d-0 allow
Control Request:
```yaml
id: cd56-1e-1d
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      nodes:
      - id: MarkerRessourceScanner.getOpenTextEditors
        code: |2
            private java.util.List<ITextEditor> getOpenTextEditors() {
              java.util.List<ITextEditor> editors = new java.util.ArrayList<>();
              for (IWorkbenchWindow window : PlatformUI.getWorkbench().getWorkbenchWindows()) for (IWorkbenchPage page : window.getPages()) for (IEditorReference ref : page.getEditorReferences()) {
                IEditorPart part = ref.getEditor(false);
                if (part instanceof AISessionEditor)
                  part = ((AISessionEditor) part).getEditor();
                if (part instanceof ITextEditor) {
                  IEditorInput input = part.getEditorInput();
                  // prefer file based / URI based editors, but any ITextEditor works
                  if (input instanceof IFileEditorInput || input != null)
                    editors.add((ITextEditor) part);
                }
              }
              return editors;
            }
```
/answer cd56-1e-1d allow
ReasoningToken: 1169
Control Request:
```yaml
id: cd56-1f-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    id: MarkerRessourceScanner.getOpenTextEditors
    position: after
    source: |2
        /**
         * Resolves the editor a prompt was started in: the hint if it is still open,
         * otherwise any open editor showing the given file.
         */
        public ITextEditor findOpenEditor(Object hint, String absoluteFilePath) {
          java.util.List<ITextEditor> open = getOpenTextEditors();
          if (hint instanceof ITextEditor && open.contains(hint))
            return (ITextEditor) hint;
          if (absoluteFilePath == null || absoluteFilePath.isBlank())
            return null;
          java.nio.file.Path target = java.nio.file.Paths.get(absoluteFilePath).toAbsolutePath().normalize();
          for (ITextEditor editor : open) {
            java.nio.file.Path location = locationOf(editor.getEditorInput());
            if (target.equals(location))
              return editor;
          }
          return null;
        }

        private static java.nio.file.Path locationOf(IEditorInput input) {
          try {
            if (input instanceof IFileEditorInput) {
              org.eclipse.core.runtime.IPath loc = ((IFileEditorInput) input).getFile().getLocation();
              return loc != null ? loc.toFile().toPath().toAbsolutePath().normalize() : null;
            }
            if (input instanceof org.eclipse.ui.IURIEditorInput) {
              java.net.URI uri = ((org.eclipse.ui.IURIEditorInput) input).getURI();
              return "file".equals(uri.getScheme()) ? java.nio.file.Paths.get(uri).toAbsolutePath().normalize() : null;
            }
          } catch (RuntimeException e) {
            // unresolvable input
          }
          return null;
        }
  reason: Add public editor resolution by hint and prompt file path
```
/answer cd56-1f-0 allow
ReasoningToken: 846
Control Request:
```yaml
id: cd56-20-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    id: EditorInterface.insertTag
    start_marker: Object origin = req.getPrompt()
    end_marker: getLastTextEditor();
    content: "PromptArguments arg = req.getPrompt().arg;\n\t\t\t\tITextEditor textEditor = markerScanner.findOpenEditor(arg != null ? arg.getEditor() : null,\n\t\t\t\t\t\targ != null ? arg.absoluteFilePath : null);\n\t\t\t\tif (textEditor == null && (arg == null || arg.absoluteFilePath == null))\n\t\t\t\t\ttextEditor = editorListener.getLastTextEditor();\n\t\t\t\tif (textEditor == null) {\n\t\t\t\t\tinsertTagIntoFile(arg != null ? arg.absoluteFilePath : null, outputMode, generateTag(req));\n\t\t\t\t\treturn;\n\t\t\t\t}"
    exact: false
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    id: EditorInterface.insertTag
    start_marker: IContainer parent = editorListener.getLastContainer();
    end_marker: getLastContainer();
    content: IContainer parent = resolveContainer(req.getPrompt().arg);
  reason: "Resolve target editor by prompt editor/file, fall back to file buffer; resolve container for New_File"
```
/answer cd56-20-0 allow
Control Request:
```yaml
id: cd56-21-20
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      result: success
      id: EditorInterface.insertTag
      new_id: EditorInterface.insertTag
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      error: End marker must start after start marker ends.
      id: EditorInterface.insertTag
      reason: marker_order
```
/answer cd56-21-20 allow
Text:
 Der zweite Edit ist fehlgeschlagen. Ich ersetze die Zeile stattdessen mit einem Block-Edit und füge die Hilfsmethoden hinzu.
Control Request:
```yaml
id: cd56-22-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    id: EditorInterface.insertTag
    old_text: IContainer parent = editorListener.getLastContainer();
    new_text: IContainer parent = resolveContainer(req.getPrompt().arg);
  reason: Use the prompt's file container for New_File
```
/answer cd56-22-0 allow
Control Request:
```yaml
id: cd56-23-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    id: EditorInterface.generateTag
    position: before
    source: |2+
        private IContainer resolveContainer(PromptArguments arg) {
          if (arg != null && arg.absoluteFilePath != null) {
            IFile file = ResourcesPlugin.getWorkspace().getRoot().getFileForLocation(Path.fromOSString(arg.absoluteFilePath));
            if (file != null)
              return file.getParent();
          }
          return editorListener.getLastContainer();
        }

        private void insertTagIntoFile(String absoluteFilePath, OutputMode mode, String tag) {
          if (absoluteFilePath == null) {
            LOG.info("Error: no editor or file to insert prompt tag into");
            return;
          }
          String text;
          switch(mode) {
            case Chat:
              text = String.format("\n%s\n%s\n%s\n", AGENT, tag, USER);
              break;
            case Append:
              text = "\n" + tag;
              break;
            default:
              LOG.info("Error: output mode " + mode + " needs an open editor to insert the prompt tag");
              return;
          }
          IPath path = Path.fromOSString(absoluteFilePath);
          ITextFileBufferManager bm = FileBuffers.getTextFileBufferManager();
          try {
            bm.connect(path, LocationKind.LOCATION, null);
            try {
              ITextFileBuffer buffer = bm.getTextFileBuffer(path, LocationKind.LOCATION);
              IDocument doc = buffer.getDocument();
              doc.replace(doc.getLength(), 0, text);
              buffer.commit(null, true);
            } finally {
              bm.disconnect(path, LocationKind.LOCATION, null);
            }
          } catch (CoreException | BadLocationException e) {
            LOG.info("Error writing prompt tag to file", e);
          }
        }

    addImport:
    - import org.eclipse.core.resources.ResourcesPlugin;
    - import org.eclipse.core.runtime.IPath;
    - import org.eclipse.core.filebuffers.FileBuffers;
    - import org.eclipse.core.filebuffers.ITextFileBuffer;
    - import org.eclipse.core.filebuffers.ITextFileBufferManager;
    - import org.eclipse.core.filebuffers.LocationKind;
    - import xy.ai.conscious.harness.PromptArguments;
  reason: Add container resolution and editor-less file buffer tag insertion
```
/answer cd56-23-0 allow
Control Request:
```yaml
id: cd56-24-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  reason: Verify both files compile
```
/answer cd56-24-0 allow
ReasoningToken: 82
Control Request:
```yaml
id: cd56-25-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    ids:
    - EditorInterface.insertTag
  reason: Review the edited method
```
/answer cd56-25-0 allow
Control Request:
```yaml
id: cd56-26-25
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: EditorInterface.insertTag
        type: MethodDeclaration
        code: |2
            @Override
            public void insertTag(IExecutor exec, IModelRequest req, IProgress mon) {
              exec.syncExec(() -> {
                OutputMode outputMode = req.getPrompt().config.outputMode;
                if (OutputMode.New_File.equals(outputMode)) {
                  IContainer parent = resolveContainer(req.getPrompt().arg);
                  String timestamp = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyyMMdd.HHmmss"));
                  IFile newFile = parent.getFile(new Path(timestamp + ".md"));
                  String tag = generateTag(req);
                  try {
                    InputStream source = new ByteArrayInputStream(tag.getBytes("UTF-8"));
                    if (!newFile.exists())
                      newFile.create(source, true, null);
                    else
                      newFile.setContents(source, true, true, null);
                    newFile.touch(null);
                    IWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();
                    IDE.openEditor(page, newFile);
                  } catch (PartInitException e) {
                    LOG.info("Error opening new editor file", e);
                  } catch (CoreException e) {
                    LOG.info("Error writting file", e);
                  } catch (UnsupportedEncodingException e) {
                    LOG.info("Error unsupported encoding", e);
                  }
                } else {
                  PromptArguments arg = req.getPrompt().arg;
                  ITextEditor textEditor = markerScanner.findOpenEditor(arg != null ? arg.getEditor() : null, arg != null ? arg.absoluteFilePath : null);
                  if (textEditor == null && (arg == null || arg.absoluteFilePath == null))
                    textEditor = editorListener.getLastTextEditor();
                  if (textEditor == null) {
                    insertTagIntoFile(arg != null ? arg.absoluteFilePath : null, outputMode, generateTag(req));
                    return;
                  }
                  IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
                  ISelection selection = textEditor.getSelectionProvider().getSelection();
                  ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
                  try {
                    String tag = generateTag(req);
                    switch(outputMode) {
                      case Chat:
                        String replace = String.format("\n%s\n%s\n%s\n", AGENT, tag, USER);
                        doc.replace(doc.getLength(), 0, replace);
                        exec.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));
                        break;
                      case Append:
                        doc.replace(doc.getLength(), 0, "\n" + tag);
                        exec.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));
                        break;
                      case Replace:
                        if (tsel != null)
                          doc.replace(tsel.getOffset(), tsel.getLength(), tag);
                        break;
                      case Cursor:
                        if (tsel != null)
                          doc.replace(tsel.getOffset(), 0, tag);
                        break;
                      case New_File:
                        throw new UnsupportedOperationException();
                    }
                    textEditor.doSave(new NullProgressMonitor());
                  } catch (BadLocationException e) {
                    LOG.info("Error adding text", e);
                  }
                }
              });
            }
```
/answer cd56-26-25 allow
Text:
 `textEditor` wird neu zugewiesen, und die Lambdas weiter unten brauchen eine effektiv finale Variable. Das behebe ich.
ReasoningToken: 75
Control Request:
```yaml
id: cd56-27-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    id: EditorInterface.insertTag
    old_text: "ITextEditor textEditor = markerScanner.findOpenEditor(arg != null ? arg.getEditor() : null, arg != null ? arg.absoluteFilePath : null);"
    new_text: "ITextEditor resolved = markerScanner.findOpenEditor(arg != null ? arg.getEditor() : null, arg != null ? arg.absoluteFilePath : null);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    id: EditorInterface.insertTag
    old_text: |-
      if (textEditor == null && (arg == null || arg.absoluteFilePath == null))
                textEditor = editorListener.getLastTextEditor();
              if (textEditor == null) {
    new_text: |-
      if (resolved == null && (arg == null || arg.absoluteFilePath == null))
                resolved = editorListener.getLastTextEditor();
              final ITextEditor textEditor = resolved;
              if (textEditor == null) {
  reason: Make textEditor effectively final for the lambdas
```
/answer cd56-27-0 allow
Result Stats: id=7374f60d-efa7-4948-b20e-3bbb5dcfe178, total: 48636, in: 42, out: 11675, reason: 0, read: 416602, write: 36919, start: 1794859007682, end: 1794859035959, time: 28138, total: 140832, io: 0, control: 139, sent: 0, recv: 162
`insertTag` schreibt den Tag jetzt in die Datei, in der der Prompt gestartet wurde, auch beim initialen Auslösen. Ich konnte nur die Syntax prüfen, einen echten Compile habe ich nicht gemacht. Getestet ist es nicht, auch nicht mit Autoapprover und Editorwechsel.

**Neue Auflösung in `EditorInterface.insertTag`:**
1. Der Editor aus `prompt.arg` wird genommen, wenn er noch offen ist. Das gilt schon für den ersten Prompt, weil `buildPrompt` den Editor beim Auslösen festhält und nicht erst beim Einfügen des Tags.
2. Ist er geschlossen, wird unter den offenen Editoren einer mit `arg.absoluteFilePath` gesucht. Das macht die neue Methode `MarkerRessourceScanner.findOpenEditor`. Sie vergleicht über `IFileEditorInput` oder `IURIEditorInput`.
3. Ist die Datei in keinem Editor offen, wird der Tag über den `ITextFileBuffer` direkt in die Datei geschrieben (`insertTagIntoFile`). Das geht nur für `Chat` und `Append`. `Replace` und `Cursor` brauchen eine Selektion und bräuchten dafür einen offenen Editor. In dem Fall gibt es nur einen Log-Eintrag.
4. Nur wenn der Prompt gar keinen Dateipfad hat, wird der zuletzt aktive Editor genommen, wie bisher.

**Zusätzlich:** Im Modus `New_File` entsteht die neue Datei jetzt im Ordner der Promptdatei (`resolveContainer`). Vorher war es der Ordner des zuletzt aktiven Editors.

Beim Ersetzen musste ich nichts ändern. Das läuft weiter über Editor-Hinweis, Marker und offene Editoren. Der Marker-Pfad schreibt über den File-Buffer, funktioniert also auch ohne offenen Editor.

Beim Test würde ich den Fall ohne offenen Editor ansehen. Dort kommt der Marker erst nach dem Speichern der Datei zustande. Ob er rechtzeitig vor dem Promptende da ist, habe ich nicht geprüft.
