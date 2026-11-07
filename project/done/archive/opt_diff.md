Das "DiffPanel" soll optimiert werden.

Das Panel zeigt beim Editor wechsel den letzten Diff des aktuellen Projektes anzeigt (vorhanden).
Es soll einen Sync Button bekommen (default disabled) mit dem diese Funktion an und abgeschaltet werden kann.

Relevante Dateien:
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java`
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java` (Sync Funktion)
SystemInit:  id=31a70d53-458b-401b-8109-ad23c3a3aa82, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 9242-2b
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  reason: "Struktur der beiden Dateien verstehen, um Sync-Button in DiffPanel zu ergänzen"
```
/answer 9242-2b allow
Control Request:
```yaml
id: 9242-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
      nodes:
      - id: YSzFy5|r0Jc26
        signature: package xy.ai.workbench.view.diff;
      - id: QLb7yZ|xiW4BF
        signature: import org.eclipse.jface.text.IDocument;
      - id: XWI4bt|5Ch9H6
        signature: import org.eclipse.swt.layout.GridLayout;
      - id: DiffPanel
        signature: "public class DiffPanel extends ViewPart {"
        children:
        - id: DiffPanel.3wi7eh|97Qrkw
          signature: public static final String ID = "xy.ai.workbench.view.diff.DiffPanel";
        - id: DiffPanel.createPartControl
          signature: "public void createPartControl(Composite parent) {"
        - id: DiffPanel.onEditorChange
          signature: "public void onEditorChange() {"
        - id: DiffPanel.onSnapshot
          signature: "public void onSnapshot(SnapshotResult result, Repository repo) {"
        - id: DiffPanel.setFocus
          signature: "public void setFocus() {"
        - id: DiffPanel.Formatter
          signature: "private class Formatter extends DiffRegionFormatter {"
          children:
          - id: DiffPanel.Formatter.Formatter
            signature: "private Formatter(IDocument document) {"
          - id: DiffPanel.Formatter.format
            signature: "public void format(FileHeader h, RawText a, RawText b, DiffDriver d) throws IOE…"
          - id: DiffPanel.Formatter.writeHunkHeader
            signature: "protected void writeHunkHeader(int aStartLine, int aEndLine, int bStartLine, in…"
          - id: DiffPanel.Formatter.formatGitDiffFirstHeaderLine
            signature: "protected void formatGitDiffFirstHeaderLine(ByteArrayOutputStream o, ChangeType…"
          - id: DiffPanel.Formatter.formatIndexLine
            signature: "protected void formatIndexLine(OutputStream o, DiffEntry ent) throws IOExceptio…"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
      nodes:
      - id: 4vwuq9|q8EEkX
        signature: package xy.ai.workbench.view;
      - id: pR0JEI|q5FGay
        signature: import org.eclipse.jface.viewers.IStructuredSelection;
      - id: mBChEJ|MpiRdV
        signature: import org.eclipse.ui.ISharedImages;
      - id: gdGpeM|g4HAfn
        signature: import xy.ai.workbench.connector.claudecode.ClaudeSessionParameters;
      - id: 9YsexQ|taievi
        signature: /**
      - id: ClaudeCodeSessionView
        signature: "public class ClaudeCodeSessionView extends ViewPart {"
        children:
        - id: ClaudeCodeSessionView.OPYzLN|gegS3r
          signature: /** The ID used in plugin.xml. */
        - id: ClaudeCodeSessionView.q0zV8G|GwW0Yw
          signature: private static final CCSession CNEW_LAUDE_CODE_SESSION = new CCSession(CCSessio…
        - id: ClaudeCodeSessionView.e0oAsC|4lJi1W
          signature: private ActionDescription syncAction;
        - id: ClaudeCodeSessionView.createPartControl
          signature: "public void createPartControl(Composite parent) {"
        - id: ClaudeCodeSessionView.dispose
          signature: "public void dispose() {"
        - id: ClaudeCodeSessionView.setFocus
          signature: "public void setFocus() {"
        - id: ClaudeCodeSessionView.makeActions
          signature: "private void makeActions() {"
        - id: ClaudeCodeSessionView.updateCurrentEditor
          signature: "private void updateCurrentEditor() {"
        - id: ClaudeCodeSessionView.findAssociatedSession
          signature: "private CCSession findAssociatedSession(List<CCSession> sessions) {"
        - id: ClaudeCodeSessionView.selectSession
          signature: "private void selectSession(CCSession session) {"
        - id: ClaudeCodeSessionView.syncSelectionToCurrentFile
          signature: "private void syncSelectionToCurrentFile() {"
        - id: ClaudeCodeSessionView.idLabel
          signature: "private String idLabel(CCSession s) {"
        - id: ClaudeCodeSessionView.stateLabel
          signature: "private String stateLabel(CCSession s) {"
        - id: ClaudeCodeSessionView.formatMMSS
          signature: "private static String formatMMSS(long totalSeconds) {"
        - id: ClaudeCodeSessionView.detailLabel
          signature: "private String detailLabel(CCSession s) {"
        - id: ClaudeCodeSessionView.abbreviateForDisplay
          signature: "private static String abbreviateForDisplay(String s) {"
        - id: ClaudeCodeSessionView.fileNameOf
          signature: "private static String fileNameOf(String path) {"
        - id: ClaudeCodeSessionView.refreshAsync
          signature: "private void refreshAsync() {"
        - id: ClaudeCodeSessionView.refreshTable
          signature: "private void refreshTable() {"
        - id: ClaudeCodeSessionView.createColumn
          signature: "private TableViewerColumn createColumn(String label, int weight) {"
```
/answer 9242-2c allow
Control Request:
```yaml
id: 9242-2d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    ids:
    - ClaudeCodeSessionView.e0oAsC|4lJi1W
    - ClaudeCodeSessionView.createPartControl
    - ClaudeCodeSessionView.makeActions
    - ClaudeCodeSessionView.updateCurrentEditor
    - ClaudeCodeSessionView.syncSelectionToCurrentFile
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
    ids:
    - DiffPanel.createPartControl
    - DiffPanel.onEditorChange
    - DiffPanel.3wi7eh|97Qrkw
  reason: "Details der Sync-Funktion und des DiffPanels lesen, um Implementierung zu planen"
```
/answer 9242-2d allow
Control Request:
```yaml
id: 9242-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
      nodes:
      - id: ClaudeCodeSessionView.e0oAsC|4lJi1W
        type: statements
        code: "private ActionDescription syncAction;\n\tprivate CCSessionManager sessionManager;\n\tprivate ConfigManager cfg;\n\n\tprivate final java.util.function.Consumer<List<CCSession>> changeListener = sessions -> refreshAsync();\n\n\tprivate Runnable ttlRefreshRunnable;\n\tprivate boolean disposed = false;\n\n\tprivate Path currentProjectPath;\n\tprivate String currentRelativeFilePath;"
      - id: ClaudeCodeSessionView.createPartControl
        type: method_declaration
        code: "@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tsessionManager = Activator.getDefault().ccSessionManager;\n\t\tcfg = Activator.getDefault().cfg;\n\n\t\tparent.setLayout(new FillLayout());\n\n\t\t// Table composite\n\t\tComposite tableComp = new Composite(parent, SWT.NONE);\n\t\ttableComp.setLayout(tableLayout = new TableColumnLayout());\n\n\t\tviewer = new TableViewer(tableComp, SWT.SINGLE | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER | SWT.FULL_SELECTION);\n\t\tTable table = viewer.getTable();\n\t\ttable.setHeaderVisible(true);\n\t\ttable.setLinesVisible(true);\n\n\t\t{\n\t\t\tcreateColumn(\"ID\", 10)\n\t\t\t\t\t.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> idLabel((CCSession) e)));\n\n\t\t\tcreateColumn(\"State\", 35)\n\t\t\t\t\t.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> stateLabel((CCSession) e)));\n\n\t\t\tcreateColumn(\"Detail\", 55)\n\t\t\t\t\t.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> detailLabel((CCSession) e)));\n\t\t}\n\n\t\tviewer.setContentProvider(ArrayContentProvider.getInstance());\n\t\tviewer.setInput(new ArrayList<CCSession>());\n\n\t\tviewer.addSelectionChangedListener(event -> {\n\t\t\tIStructuredSelection sel = viewer.getStructuredSelection();\n\t\t\tif (sel.isEmpty()) {\n\t\t\t\tsessionManager.setSelectedSessionUuid(null);\n\t\t\t} else {\n\t\t\t\tCCSession s = (CCSession) sel.getFirstElement();\n\t\t\t\tsessionManager.setSelectedSessionUuid(s.getSessionUuid());\n\t\t\t}\n\t\t});\n\n\t\tviewer.addDoubleClickListener(event -> {\n\t\t\tIStructuredSelection sel = (IStructuredSelection) event.getSelection();\n\t\t\tif (!sel.isEmpty() && sel.getFirstElement() instanceof CCSession) {\n\t\t\t\tCCSession s = (CCSession) sel.getFirstElement();\n\t\t\t\tif (s != CNEW_LAUDE_CODE_SESSION)\n\t\t\t\t\tnew SessionDetailDialog(viewer.getControl().getShell(), s).open();\n\t\t\t}\n\t\t});\n\n\t\tsessionManager.addChangeListener(changeListener);\n\n\t\t// Toolbar\n\t\tmakeActions();\n\n\t\tIActionBars bars = getViewSite().getActionBars();\n\t\tact.fillLocalToolBar(bars.getToolBarManager());\n\t\tact.fillLocalPullDown(bars.getMenuManager());\n\t\tbars.updateActionBars();\n\n\t\tActivator.getDefault().editorListener.addTextEditorObserver(e -> updateCurrentEditor());\n\t\tupdateCurrentEditor();\n\n\t\tttlRefreshRunnable = new Runnable() {\n\t\t\t@Override\n\t\t\tpublic void run() {\n\t\t\t\tif (disposed)\n\t\t\t\t\treturn;\n\t\t\t\trefreshTable();\n\t\t\t\tDisplay.getCurrent().timerExec(TTL_REFRESH_INTERVAL_MS, this);\n\t\t\t}\n\t\t};\n\t\tDisplay.getDefault().timerExec(TTL_REFRESH_INTERVAL_MS, ttlRefreshRunnable);\n\t}"
      - id: ClaudeCodeSessionView.makeActions
        type: method_declaration
        code: "private void makeActions() {\n\t\tsyncAction = act.create().toolbar().text(\"Sync\", \"Link session selection to the focused editor\")\n\t\t\t\t.image(ISharedImages.IMG_ELCL_SYNCED).runnable(() -> {\n\t\t\t\t\tif (syncAction.isChecked())\n\t\t\t\t\t\tsyncSelectionToCurrentFile();\n\t\t\t\t});\n\t\tsyncAction.done();\n\t\tsyncAction.setChecked(true);\n\n\t\tact.create().text(\"Terminate Session\", \"Terminates the selected CLI session\")\n\t\t\t\t.image(ISharedImages.IMG_TOOL_DELETE).toolbar().pullDown()\n\t\t\t\t.selection(viewer, CCSession.class, session -> {\n\t\t\t\t\tsessionManager.terminateSessions(java.util.List.of(session.getID()));\n\t\t\t\t}).done();\n\t}"
      - id: ClaudeCodeSessionView.updateCurrentEditor
        type: method_declaration
        code: "private void updateCurrentEditor() {\n\t\tActiveEditorListener edtlst = Activator.getDefault().editorListener;\n\t\tcurrentProjectPath = edtlst.resolveProjectPath();\n\t\tIFile editorFile = edtlst.getLastEditorFile();\n\t\tcurrentRelativeFilePath = editorFile != null ? editorFile.getProjectRelativePath().toString() : null;\n\n\t\tif (syncAction.isChecked())\n\t\t\tsyncSelectionToCurrentFile();\n\t}"
      - id: ClaudeCodeSessionView.syncSelectionToCurrentFile
        type: method_declaration
        code: "private void syncSelectionToCurrentFile() {\n\t\tif (viewer == null || viewer.getControl().isDisposed() || sessionManager == null)\n\t\t\treturn;\n\t\tselectSession(findAssociatedSession(sessionManager.getSessions()));\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
      nodes:
      - id: DiffPanel.createPartControl
        type: method_declaration
        code: "@SuppressWarnings(\"restriction\")\n\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tINSTANCE = this;\n\t\tActivator.getDefault().editorListener.addTextEditorObserver(e -> onEditorChange());\n\t\tparent.setLayout(new GridLayout(1, false));\n\n\t\tstatusLabel = new Label(parent, SWT.NONE);\n\t\tstatusLabel.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));\n\t\tstatusLabel.setText(\"Empty\");\n\n\t\tComposite diffSection = new Composite(parent, SWT.NONE);\n\t\tdiffSection.setLayout(new FillLayout());\n\t\tdiffViewer = new DiffViewer(parent, null, SWT.V_SCROLL | SWT.H_SCROLL);\n\t\tdiffViewer.getControl().setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));\n\t\tdiffViewer.setDocument(new Document(\"Empty\"));\n\t}"
      - id: DiffPanel.onEditorChange
        type: method_declaration
        code: "public void onEditorChange() {\n\t\tActiveEditorListener editLst = Activator.getDefault().editorListener;\n\t\tPath path = editLst.resolveProjectPath();\n\t\tif (path == null)\n\t\t\treturn;\n\t\tFile projectPath = path.toFile();\n\t\ttry {\n\t\t\tOpSnapshotter snap = new OpSnapshotter(projectPath);\n\t\t\tSnapshotResult result = snap.findLatest();\n\t\t\tif (result != null)\n\t\t\t\tonSnapshot(result, snap.getRepository());\n\t\t} catch (IOException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}"
      - id: DiffPanel.3wi7eh|97Qrkw
        type: statements
        code: "public static final String ID = \"xy.ai.workbench.view.diff.DiffPanel\";\n\n\tpublic static DiffPanel INSTANCE;\n\n\tprivate Label statusLabel;\n\t@SuppressWarnings(\"restriction\")\n\tprivate DiffViewer diffViewer;"
```
/answer 9242-2e allow
ReasoningToken: 87
Control Request:
```yaml
id: 9242-2f
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
    pattern: "ActionDescription|private.*act[^ion]| act[.;=]"
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: class ActionDescription|class Act
    include:
    - '*.java'
  reason: Herkunft der ActionDescription/act Utility-Klasse finden für konsistente Sync-Button-Implementierung im DiffPanel
```
/answer 9242-2f allow
Control Request:
```yaml
id: 9242-30
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
      matches:
      - path: explorer/GitIgnoreRule.java
        matches:
        - lineno: 17
          match: private final Pattern exactPattern;
        - lineno: 22
          match: "private GitIgnoreRule(Pattern exactPattern, Pattern nestedPattern, boolean negate, boolean dirOnly) {"
      - path: AISessionView.java
        matches:
        - lineno: 46
          match: ConfigManager cfg = act.cfg;
        - lineno: 47
          match: PromptHandler prompt = act.promptHandler;
      - path: AIBatchView.java
        matches:
        - lineno: 50
          match: import xy.ai.workbench.view.ActionManager.ActionDescription;
        - lineno: 64
          match: private ActionManager act = new ActionManager();
        - lineno: 232
          match: ActionDescription autoUpd = act.create();
      - path: ActionManager.java
        matches:
        - lineno: 20
          match: private List<ActionDescription> actions = new ArrayList<>();
        - lineno: 22
          match: "public void add(ActionDescription actionDescription) {"
        - lineno: 26
          match: "public ActionDescription create() {"
        - lineno: 27
          match: return new ActionDescription(this);
        - lineno: 42
          match: "public static class ActionDescription extends Action {"
        - lineno: 52
          match: "public ActionDescription(ActionManager manager) {"
        - lineno: 72
          match: "public ActionDescription pullDown() {"
        - lineno: 77
          match: "public ActionDescription contextMenu() {"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/ActionManager.java
        matches:
        - lineno: 42
          match: "public static class ActionDescription extends Action {"
```
/answer 9242-30 allow
ReasoningToken: 99
Control Request:
```yaml
id: 9242-31
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ActionManager.java
  reason: "API der ActionManager/ActionDescription Klasse verstehen, um Toggle-Button korrekt zu erstellen"
```
/answer 9242-31 allow
Control Request:
```yaml
id: 9242-32
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ActionManager.java
      nodes:
      - id: JSIYgG|7LI9hU
        signature: package xy.ai.workbench.view;
      - id: ienT4L|ZCO97Q
        signature: import org.eclipse.jface.viewers.StructuredViewer;
      - id: ActionManager
        signature: "public class ActionManager {"
        children:
        - id: ActionManager.zN9t5L|i5SE1F
          signature: private List<ActionDescription> actions = new ArrayList<>();
        - id: ActionManager.add
          signature: "public void add(ActionDescription actionDescription) {"
        - id: ActionManager.create
          signature: "public ActionDescription create() {"
        - id: ActionManager.fillLocalPullDown
          signature: "public void fillLocalPullDown(IMenuManager manager) {"
        - id: ActionManager.fillContextMenu
          signature: "public void fillContextMenu(IMenuManager manager) {"
        - id: ActionManager.fillLocalToolBar
          signature: "public void fillLocalToolBar(IToolBarManager manager) {"
        - id: ActionManager.ActionDescription
          signature: "public static class ActionDescription extends Action {"
          children:
          - id: ActionManager.ActionDescription.Wo4DCk|IyF4Uc
            signature: private boolean isPullDown = false;
          - id: ActionManager.ActionDescription.ActionDescription
            signature: "public ActionDescription(ActionManager manager) {"
          - id: ActionManager.ActionDescription.isPullDown
            signature: "public boolean isPullDown() {"
          - id: ActionManager.ActionDescription.isContextMenu
            signature: "public boolean isContextMenu() {"
          - id: ActionManager.ActionDescription.isToolbar
            signature: "public boolean isToolbar() {"
          - id: ActionManager.ActionDescription.done
            signature: "public void done() {"
          - id: ActionManager.ActionDescription.pullDown
            signature: "public ActionDescription pullDown() {"
          - id: ActionManager.ActionDescription.contextMenu
            signature: "public ActionDescription contextMenu() {"
          - id: ActionManager.ActionDescription.toolbar
            signature: "public ActionDescription toolbar() {"
          - id: ActionManager.ActionDescription.text
            signature: "public ActionDescription text(String text, String tooltip) {"
          - id: ActionManager.ActionDescription.image
            signature: "public ActionDescription image(String image) {"
          - id: ActionManager.ActionDescription.runnable
            signature: "public ActionDescription runnable(Runnable run) {"
          - id: ActionManager.ActionDescription.selection
            signature: "public <C> ActionDescription selection(StructuredViewer viewer, Class<C> clazz,…"
          - id: ActionManager.ActionDescription.selection_1
            signature: "public ActionDescription selection(StructuredViewer viewer, Consumer<IStructure…"
          - id: ActionManager.ActionDescription.job
            signature: "public ActionDescription job(Consumer<IProgressMonitor> run) {"
          - id: ActionManager.ActionDescription.jobFunc
            signature: "public ActionDescription jobFunc(IJobFunction job) {"
          - id: ActionManager.ActionDescription.display
            signature: "public ActionDescription display(Runnable run) {"
          - id: ActionManager.ActionDescription.run
            signature: "public void run() {"
```
/answer 9242-32 allow
ReasoningToken: 48
Control Request:
```yaml
id: 9242-33
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ActionManager.java
    ids:
    - ActionManager.ActionDescription.ActionDescription
    - ActionManager.ActionDescription.done
    - ActionManager.ActionDescription.text
    - ActionManager.ActionDescription.image
  reason: "Details der ActionDescription-Konstruktion prüfen, um Checkbox-Verhalten korrekt zu setzen"
```
/answer 9242-33 allow
Control Request:
```yaml
id: 9242-34
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ActionManager.java
      nodes:
      - id: ActionManager.ActionDescription.ActionDescription
        type: constructor_declaration
        code: "public ActionDescription(ActionManager manager) {\n\t\t\tthis.manager = manager;\n\t\t}"
      - id: ActionManager.ActionDescription.done
        type: method_declaration
        code: "public void done() {\n\t\t\tmanager.add(this);\n\t\t}"
      - id: ActionManager.ActionDescription.text
        type: method_declaration
        code: "public ActionDescription text(String text, String tooltip) {\n\t\t\tsetText(text);\n\t\t\tsetToolTipText(tooltip);\n\t\t\treturn this;\n\t\t}"
      - id: ActionManager.ActionDescription.image
        type: method_declaration
        code: "public ActionDescription image(String image) {\n\t\t\tsetImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(image));\n\t\t\treturn this;\n\t\t}"
```
/answer 9242-34 allow
ReasoningToken: 48
Control Request:
```yaml
id: 9242-35
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
  reason: Vollständige Imports der Datei prüfen für nötige neue Imports
```
/answer 9242-35 allow Keine Checkbox, ein Toggle Button wie bei den anderen auch
Control Request:
```yaml
id: 9242-36
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
      nodes:
      - id: YSzFy5|r0Jc26
        signature: package xy.ai.workbench.view.diff;
      - id: QLb7yZ|xiW4BF
        signature: import org.eclipse.jface.text.IDocument;
      - id: XWI4bt|5Ch9H6
        signature: import org.eclipse.swt.layout.GridLayout;
      - id: DiffPanel
        signature: "public class DiffPanel extends ViewPart {"
        children:
        - id: DiffPanel.3wi7eh|97Qrkw
          signature: public static final String ID = "xy.ai.workbench.view.diff.DiffPanel";
        - id: DiffPanel.createPartControl
          signature: "public void createPartControl(Composite parent) {"
        - id: DiffPanel.onEditorChange
          signature: "public void onEditorChange() {"
        - id: DiffPanel.onSnapshot
          signature: "public void onSnapshot(SnapshotResult result, Repository repo) {"
        - id: DiffPanel.setFocus
          signature: "public void setFocus() {"
        - id: DiffPanel.Formatter
          signature: "private class Formatter extends DiffRegionFormatter {"
          children:
          - id: DiffPanel.Formatter.Formatter
            signature: "private Formatter(IDocument document) {"
          - id: DiffPanel.Formatter.format
            signature: "public void format(FileHeader h, RawText a, RawText b, DiffDriver d) throws IOE…"
          - id: DiffPanel.Formatter.writeHunkHeader
            signature: "protected void writeHunkHeader(int aStartLine, int aEndLine, int bStartLine, in…"
          - id: DiffPanel.Formatter.formatGitDiffFirstHeaderLine
            signature: "protected void formatGitDiffFirstHeaderLine(ByteArrayOutputStream o, ChangeType…"
          - id: DiffPanel.Formatter.formatIndexLine
            signature: "protected void formatIndexLine(OutputStream o, DiffEntry ent) throws IOExceptio…"
```
/answer 9242-36 allow
ReasoningToken: 572
Control Request:
```yaml
id: 9242-37
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
  node_type: import_declaration
  reason: Alle Imports der DiffPanel.java auflisten
```
/answer 9242-37 allow
Control Request:
```yaml
id: 9242-38
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
      nodes: []
```
/answer 9242-38 allow
ReasoningToken: 289
Control Request:
```yaml
id: 9242-39
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
    ids:
    - QLb7yZ|xiW4BF
    - XWI4bt|5Ch9H6
  reason: Vollständigen Import-Block der DiffPanel.java lesen
```
/answer 9242-39 allow
Control Request:
```yaml
id: 9242-3a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
      nodes:
      - id: QLb7yZ|xiW4BF
        type: imports
        code: |-
          import org.eclipse.jface.text.IDocument;
          import org.eclipse.jgit.diff.DiffDriver;
          import org.eclipse.jgit.diff.DiffEntry;
          import org.eclipse.jgit.diff.DiffEntry.ChangeType;
          import org.eclipse.jgit.diff.RawText;
          import org.eclipse.jgit.lib.Repository;
          import org.eclipse.jgit.patch.FileHeader;
          import org.eclipse.jgit.revwalk.RevCommit;
          import org.eclipse.jgit.revwalk.RevWalk;
          import org.eclipse.swt.SWT;
          import org.eclipse.swt.layout.FillLayout;
          import org.eclipse.swt.layout.GridData;
      - id: XWI4bt|5Ch9H6
        type: imports
        code: |-
          import org.eclipse.swt.layout.GridLayout;
          import org.eclipse.swt.widgets.Composite;
          import org.eclipse.swt.widgets.Label;
          import org.eclipse.ui.editors.text.EditorsUI;
          import org.eclipse.ui.part.ViewPart;

          import xy.ai.workbench.Activator;
          import xy.ai.workbench.ActiveEditorListener;
          import xy.ai.workbench.LOG;
```
/answer 9242-3a allow
Text:
 Jetzt implementiere ich Sync-Button in DiffPanel.
Control Request:
```yaml
id: 9242-3b
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
    id: XWI4bt|5Ch9H6
    old_text: |-
      import org.eclipse.ui.editors.text.EditorsUI;
      import org.eclipse.ui.part.ViewPart;

      import xy.ai.workbench.Activator;
      import xy.ai.workbench.ActiveEditorListener;
      import xy.ai.workbench.LOG;
    new_text: |-
      import org.eclipse.ui.IActionBars;
      import org.eclipse.ui.ISharedImages;
      import org.eclipse.ui.editors.text.EditorsUI;
      import org.eclipse.ui.part.ViewPart;

      import xy.ai.workbench.Activator;
      import xy.ai.workbench.ActiveEditorListener;
      import xy.ai.workbench.LOG;
      import xy.ai.workbench.view.ActionManager;
      import xy.ai.workbench.view.ActionManager.ActionDescription;
  reason: Benötigte Imports für ActionManager/ActionDescription und Toolbar ergänzen
```
/answer 9242-3b allow
Control Request:
```yaml
id: 9242-3c
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
    id: DiffPanel.3wi7eh|97Qrkw
    old_text: "private Label statusLabel;\n\t@SuppressWarnings(\"restriction\")\n\tprivate DiffViewer diffViewer;"
    new_text: "private Label statusLabel;\n\t@SuppressWarnings(\"restriction\")\n\tprivate DiffViewer diffViewer;\n\n\tprivate ActionManager act = new ActionManager();\n\tprivate ActionDescription syncAction;"
  reason: Felder für ActionManager und Sync-Toggle-Action hinzufügen
```
/answer 9242-3c allow
Text:
 Jetzt `createPartControl` und `onEditorChange` anpassen sowie `makeActions` hinzufügen:
Control Request:
```yaml
id: 9242-3d
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
    id: DiffPanel.createPartControl
    old_text: "diffViewer.setDocument(new Document(\"Empty\"));\n\t}"
    new_text: "diffViewer.setDocument(new Document(\"Empty\"));\n\n\t\tmakeActions();\n\t\tIActionBars bars = getViewSite().getActionBars();\n\t\tact.fillLocalToolBar(bars.getToolBarManager());\n\t\tbars.updateActionBars();\n\t}\n\n\tprivate void makeActions() {\n\t\tsyncAction = act.create().toolbar().text(\"Sync\", \"Automatically show the latest diff on editor change\")\n\t\t\t\t.image(ISharedImages.IMG_ELCL_SYNCED).runnable(() -> {\n\t\t\t\t\tif (syncAction.isChecked())\n\t\t\t\t\t\tonEditorChange();\n\t\t\t\t});\n\t\tsyncAction.done();\n\t\tsyncAction.setChecked(false);\n\t}"
  reason: Toolbar-Registrierung und makeActions mit standardmäßig deaktiviertem Sync-Toggle ergänzen
```
/answer 9242-3d allow
Control Request:
```yaml
id: 9242-3e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
    id: DiffPanel.onEditorChange
    old_text: "public void onEditorChange() {\n\t\tActiveEditorListener editLst"
    new_text: "public void onEditorChange() {\n\t\tif (syncAction != null && !syncAction.isChecked())\n\t\t\treturn;\n\t\tActiveEditorListener editLst"
  reason: "onEditorChange nur ausführen, wenn Sync aktiviert ist"
```
/answer 9242-3e allow
Control Request:
```yaml
id: 9242-3f
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
  reason: Kompilierbarkeit der Änderungen prüfen
```
/answer 9242-3f allow
Result Stats: id=31a70d53-458b-401b-8109-ad23c3a3aa82, total: 25728, in: 159, out: 4933, reason: 0, read: 199837, write: 20636
Fertig. Der Sync-Toggle-Button (default deaktiviert) wurde im `DiffPanel` analog zum Muster in `ClaudeCodeSessionView` ergänzt: `onEditorChange()` aktualisiert den Diff nur noch, wenn `syncAction.isChecked()` ist.
