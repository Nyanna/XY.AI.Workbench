Verbessere das `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java`.

- Das Panel soll einen Sash und einen Detailbereich auf der rechten Seite Bekommen.
	- Der Detailbereich soll ungefähr die letzte Spalte überdecken ohne das diese aus dem Log entfernt wird
	- Der Cursor wird stets dem Logende nachgeführt und der Detailbereich zeigt aufgearbeitete Details der Zeile in der der Cursor steht
		- Wenn der Cursor versetzt wird kann können also die Details einer anderen Zeile betrachtet werden
	- Der Detailbereich soll über einen Button in der ActionBar togglebar sein (Default eingeblendet)
	- Der Detailbereich ist ebenfalls ein Textbereich. Er zeigt zuerst einen potenziellen Reason (Text bricht um)
	- Ist es ein Text oder Thinking Eintrag wird dieser Gezeigt
	- Ist es ein Tool Call Eintrag so folgt dem Reason mit einer Leerzeile das JSON in Pretty angezeigt
	- Ist es ein Tool Result Eintrag so wird das JSON innerhalb von "structuredContent.results" in Pretty nach einer Leerzeile angezeigt
	
- Der zusätzliche Button mit der Aufschrift "Filter" kann entfernt werden. Das Filtermenü bleibt und ist alleine ausreichend.
	- Die Filter Option "No Action" ändert ihr Default zu "Am"
	

Beispiellog:
```text
0b562e81-… | Pass     | bash             |    1002 | Rewrite test files using alias maps from parent files at HEAD:   {"items":[{"cwd":"/home/user/xyan/xy.ai.workbench/rag/src/resident/_d/a","script":["cat > /tmp/claude-1000/s/untest.py <<'EOF'\nimport re,sys,subprocess\nfor p,par in zip(sys.argv[1::2],sys.argv[2::2…
0b562e81-… | Pass     | Result           |     162 |                {"structuredContent":{"results":[{"index":0,"stdout":""}]}}
0b562e81-… | Approved | run_command      |     193 | Re-run tests:   {"items":[{"cwd":"/home/user/xyan/xy.ai.workbench/rag","id":"runTests"}],"reason":"Re-run tests"}
0b562e81-… | Approved | Result           |     171 |                {"structuredContent":{"results":[{"index":0,"stdout":"OK: 2662 tests passed\n"}]}}
```
SystemInit:  id=696ec05a-4fd4-47c8-b551-17fe0acd5a78, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 4843-81-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  reason: Datei lesen
```
/answer 4843-81-0 allow
Control Request:
```yaml
id: 4843-82-81
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
        nodes:
        - id: L0iCkq|MgjHi3
          excert: import java.util.ArrayDeque;
        - id: Nwa7TR|4NjytU
          excert: import org.eclipse.ui.PlatformUI;
        - id: AutoApprovePanel
          excert: /**  * Console like log of the decisions of the auto approver for all sessions.…
          docstring: Console like log of the decisions of the auto approver for all sessions.
          children:
          - id: AutoApprovePanel.uxnwF9|tMhmxr
            excert: public static final String ID = "xy.ai.workbench.view.approver.AutoApprovePanel…
          - id: AutoApprovePanel.Filter
            excert: "private enum Filter {"
            children:
            - id: AutoApprovePanel.Filter.Cf4hGc|MDDbQP
              excert: "Allow(\"Allow\"), Denied(\"Denied\"), NoAction(\"No Action\");"
            - id: AutoApprovePanel.Filter.VLRnLu|dnJOEd
              excert: private final String label;
            - id: AutoApprovePanel.Filter.Filter
              excert: Filter(String label)
          - id: AutoApprovePanel.Entry
            excert: "private record Entry(Filter filter, String line) {"
          - id: AutoApprovePanel.fauStF|jd84rM
            excert: private final Deque<Entry> buffer = new ArrayDeque<>();
          - id: AutoApprovePanel.createPartControl
            excert: public void createPartControl(Composite parent)
          - id: AutoApprovePanel.createDisableAction
            excert: private Action createDisableAction()
          - id: AutoApprovePanel.createAutoShowAction
            excert: private Action createAutoShowAction()
          - id: AutoApprovePanel.createFilterAction
            excert: private Action createFilterAction()
          - id: AutoApprovePanel.fillMenu
            excert: private void fillMenu(Menu menu)
          - id: AutoApprovePanel.onDecision
            excert: private void onDecision(Decision decision)
          - id: AutoApprovePanel.append
            excert: private void append(Entry entry)
          - id: AutoApprovePanel.rebuild
            excert: private void rebuild()
          - id: AutoApprovePanel.scrollToEnd
            excert: private void scrollToEnd()
          - id: AutoApprovePanel.toEntry
            excert: private static Entry toEntry(Decision d)
          - id: AutoApprovePanel.text
            excert: private static String text(JsonNode node)
          - id: AutoApprovePanel.oneLine
            excert: private static String oneLine(String s)
          - id: AutoApprovePanel.shorten
            excert: "private static String shorten(String s, int max)"
          - id: AutoApprovePanel.fit
            excert: "private static String fit(String s, int width)"
          - id: AutoApprovePanel.pad
            excert: "private static String pad(String s, int width)"
          - id: AutoApprovePanel.dispose
            excert: public void dispose()
          - id: AutoApprovePanel.setFocus
            excert: public void setFocus()
```
/answer 4843-82-81 allow
Control Request:
```yaml
id: 4843-83-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  reason: Quelltext lesen
```
/answer 4843-83-0 allow
Control Request:
```yaml
id: 4843-84-83
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
        nodes:
        - id: L0iCkq|MgjHi3
          excert: import java.util.ArrayDeque;
        - id: Nwa7TR|4NjytU
          excert: import org.eclipse.ui.PlatformUI;
        - id: AutoApprovePanel
          excert: /**  * Console like log of the decisions of the auto approver for all sessions.…
          docstring: Console like log of the decisions of the auto approver for all sessions.
          children:
          - id: AutoApprovePanel.uxnwF9|tMhmxr
            excert: public static final String ID = "xy.ai.workbench.view.approver.AutoApprovePanel…
          - id: AutoApprovePanel.Filter
            excert: "private enum Filter {"
            children:
            - id: AutoApprovePanel.Filter.Cf4hGc|MDDbQP
              excert: "Allow(\"Allow\"), Denied(\"Denied\"), NoAction(\"No Action\");"
            - id: AutoApprovePanel.Filter.VLRnLu|dnJOEd
              excert: private final String label;
            - id: AutoApprovePanel.Filter.Filter
              excert: Filter(String label)
          - id: AutoApprovePanel.Entry
            excert: "private record Entry(Filter filter, String line) {"
          - id: AutoApprovePanel.fauStF|jd84rM
            excert: private final Deque<Entry> buffer = new ArrayDeque<>();
          - id: AutoApprovePanel.createPartControl
            excert: public void createPartControl(Composite parent)
          - id: AutoApprovePanel.createDisableAction
            excert: private Action createDisableAction()
          - id: AutoApprovePanel.createAutoShowAction
            excert: private Action createAutoShowAction()
          - id: AutoApprovePanel.createFilterAction
            excert: private Action createFilterAction()
          - id: AutoApprovePanel.fillMenu
            excert: private void fillMenu(Menu menu)
          - id: AutoApprovePanel.onDecision
            excert: private void onDecision(Decision decision)
          - id: AutoApprovePanel.append
            excert: private void append(Entry entry)
          - id: AutoApprovePanel.rebuild
            excert: private void rebuild()
          - id: AutoApprovePanel.scrollToEnd
            excert: private void scrollToEnd()
          - id: AutoApprovePanel.toEntry
            excert: private static Entry toEntry(Decision d)
          - id: AutoApprovePanel.text
            excert: private static String text(JsonNode node)
          - id: AutoApprovePanel.oneLine
            excert: private static String oneLine(String s)
          - id: AutoApprovePanel.shorten
            excert: "private static String shorten(String s, int max)"
          - id: AutoApprovePanel.fit
            excert: "private static String fit(String s, int width)"
          - id: AutoApprovePanel.pad
            excert: "private static String pad(String s, int width)"
          - id: AutoApprovePanel.dispose
            excert: public void dispose()
          - id: AutoApprovePanel.setFocus
            excert: public void setFocus()
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java instead."
```
/answer 4843-84-83 allow
Text:
 Ich lese die Knoten per ID.
Control Request:
```yaml
id: 4843-85-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    ids:
    - L0iCkq|MgjHi3
    - Nwa7TR|4NjytU
    - AutoApprovePanel
  reason: Quelltext der Klasse lesen
```
/answer 4843-85-0 allow
Control Request:
```yaml
id: 4843-86-85
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
        nodes:
        - id: L0iCkq|MgjHi3
          type: ImportGroup
          code: |
            import java.util.ArrayDeque;
            import java.util.Deque;
            import java.util.EnumSet;
            import java.util.Set;
            import java.util.function.Consumer;
            import org.eclipse.jface.action.Action;
            import org.eclipse.jface.action.IAction;
            import org.eclipse.jface.action.IMenuCreator;
            import org.eclipse.jface.resource.JFaceResources;
            import org.eclipse.swt.SWT;
            import org.eclipse.swt.custom.StyledText;
            import org.eclipse.swt.widgets.Control;
            import org.eclipse.swt.widgets.Display;
            import org.eclipse.swt.widgets.Menu;
            import org.eclipse.swt.widgets.MenuItem;
            import org.eclipse.ui.IActionBars;
            import org.eclipse.ui.part.ViewPart;
            import org.eclipse.swt.widgets.Composite;
            import com.fasterxml.jackson.databind.JsonNode;
            import xy.ai.conscious.autoapprove.ApprovalItem;
            import xy.ai.conscious.autoapprove.AutoApprover;
            import xy.ai.conscious.autoapprove.Decision;
            import xy.ai.workbench.Activator;
            import org.eclipse.ui.ISharedImages;
            import org.eclipse.ui.IWorkbenchPage;
            import org.eclipse.ui.PartInitException;
        - id: Nwa7TR|4NjytU
          type: ImportGroup
          code: |
            import org.eclipse.ui.PlatformUI;
        - id: AutoApprovePanel
          type: ClassOrInterfaceDeclaration
          excert: /**  * Console like log of the decisions of the auto approver for all sessions.…
          docstring: Console like log of the decisions of the auto approver for all sessions.
          children:
          - id: AutoApprovePanel.uxnwF9|tMhmxr
            type: StatementGroup
            code: |2
                public static final String ID = "xy.ai.workbench.view.approver.AutoApprovePanel";
                private static final int MAX_LINES = 1000;
                private static final int SESSION_WIDTH = 10;
                private static final int ACTION_WIDTH = 8;
                private static final int TOOL_WIDTH = 16;
                private static final int CHARS_WIDTH = 7;
                private static final int DETAIL_MAX = 200;
          - id: AutoApprovePanel.Filter
            type: EnumDeclaration
            excert: "private enum Filter {"
            children:
            - id: AutoApprovePanel.Filter.Cf4hGc|MDDbQP
              type: StatementGroup
              code: |2
                    Allow("Allow"), Denied("Denied"), NoAction("No Action");
            - id: AutoApprovePanel.Filter.VLRnLu|dnJOEd
              type: StatementGroup
              code: |2
                    private final String label;
            - id: AutoApprovePanel.Filter.Filter
              type: ConstructorDeclaration
              code: |2
                    Filter(String label) {
                      this.label = label;
                    }
          - id: AutoApprovePanel.Entry
            type: RecordDeclaration
            code: |2
                private record Entry(Filter filter, String line) {
                }
          - id: AutoApprovePanel.fauStF|jd84rM
            type: StatementGroup
            code: |2
                private final Deque<Entry> buffer = new ArrayDeque<>();
                private final Set<Filter> shown = EnumSet.of(Filter.Allow, Filter.Denied);
                private final Consumer<Decision> observer = this::onDecision;
                private Display display;
                private StyledText console;
                private AutoApprover approver;
                private boolean showOnChange = true;
          - id: AutoApprovePanel.createPartControl
            type: MethodDeclaration
            code: |2
                @Override
                public void createPartControl(Composite parent) {
                  display = parent.getDisplay();
                  console = new StyledText(parent, SWT.READ_ONLY | SWT.V_SCROLL | SWT.H_SCROLL);
                  console.setFont(JFaceResources.getTextFont());
                  console.setWordWrap(false);
                  approver = Activator.getDefault().promptHandler.approver();
                  IActionBars bars = getViewSite().getActionBars();
                  bars.getToolBarManager().add(createDisableAction());
                  bars.getToolBarManager().add(createAutoShowAction());
                  bars.getToolBarManager().add(createFilterAction());
                  bars.updateActionBars();
                  approver.addListener(observer);
                }
          - id: AutoApprovePanel.createDisableAction
            type: MethodDeclaration
            code: |2
                private Action createDisableAction() {
                  Action action = new Action("Disable Auto Approver", IAction.AS_CHECK_BOX) {

                    @Override
                    public void run() {
                      approver.setEnabled(!isChecked());
                    }
                  };
                  action.setToolTipText("Disable the processing of the auto approver");
                  action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_ELCL_REMOVE));
                  action.setChecked(!approver.isEnabled());
                  return action;
                }
          - id: AutoApprovePanel.createAutoShowAction
            type: MethodDeclaration
            code: |2
                private Action createAutoShowAction() {
                  Action action = new Action("Show When Content Changes", IAction.AS_CHECK_BOX) {

                    @Override
                    public void run() {
                      showOnChange = isChecked();
                    }
                  };
                  action.setToolTipText("Show this view when new entries are logged");
                  action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_ETOOL_DEF_PERSPECTIVE));
                  action.setChecked(showOnChange);
                  return action;
                }
          - id: AutoApprovePanel.createFilterAction
            type: MethodDeclaration
            code: |2
                private Action createFilterAction() {
                  Action action = new Action("Filter", IAction.AS_DROP_DOWN_MENU) {
                  };
                  action.setToolTipText("Select the action types to show");
                  action.setMenuCreator(new IMenuCreator() {
                    private Menu menu;

                    @Override
                    public Menu getMenu(Control parent) {
                      dispose();
                      menu = new Menu(parent);
                      fillMenu(menu);
                      return menu;
                    }

                    @Override
                    public Menu getMenu(Menu parent) {
                      dispose();
                      menu = new Menu(parent);
                      fillMenu(menu);
                      return menu;
                    }

                    @Override
                    public void dispose() {
                      if (menu != null && !menu.isDisposed())
                        menu.dispose();
                      menu = null;
                    }
                  });
                  return action;
                }
          - id: AutoApprovePanel.fillMenu
            type: MethodDeclaration
            code: |2
                private void fillMenu(Menu menu) {
                  for (Filter filter : Filter.values()) {
                    MenuItem item = new MenuItem(menu, SWT.CHECK);
                    item.setText(filter.label);
                    item.setSelection(shown.contains(filter));
                    item.addListener(SWT.Selection, e -> {
                      if (item.getSelection())
                        shown.add(filter);
                      else
                        shown.remove(filter);
                      rebuild();
                    });
                  }
                }
          - id: AutoApprovePanel.onDecision
            type: MethodDeclaration
            code: |2
                private void onDecision(Decision decision) {
                  Entry entry = toEntry(decision);
                  display.asyncExec(() -> {
                    if (!console.isDisposed())
                      append(entry);
                  });
                }
          - id: AutoApprovePanel.append
            type: MethodDeclaration
            code: "  private void append(Entry entry) {\n    buffer.addLast(entry);\n    if (buffer.size() > MAX_LINES && shown.contains(buffer.removeFirst().filter()))\n      console.replaceTextRange(0, console.getOffsetAtLine(1), \"\");\n    if (shown.contains(entry.filter())) {\n      console.append(entry.line() + \"\\n\");\n      scrollToEnd();\n      if (showOnChange)\n\t\ttry {\n\t\t\tgetSite().getPage().showView(ID, null, IWorkbenchPage.VIEW_VISIBLE);\n\t\t} catch (PartInitException e) {\n\t\t\t// ignore not present\n\t\t}\n    }\n  }\n"
          - id: AutoApprovePanel.rebuild
            type: MethodDeclaration
            code: |2
                private void rebuild() {
                  StringBuilder text = new StringBuilder();
                  for (Entry entry : buffer) {
                    if (shown.contains(entry.filter()))
                      text.append(entry.line()).append('\n');
                  }
                  console.setText(text.toString());
                  scrollToEnd();
                }
          - id: AutoApprovePanel.scrollToEnd
            type: MethodDeclaration
            code: |2
                private void scrollToEnd() {
                  console.setCaretOffset(console.getCharCount());
                  console.showSelection();
                }
          - id: AutoApprovePanel.toEntry
            type: MethodDeclaration
            code: |2
                private static Entry toEntry(Decision d) {
                  ApprovalItem item = d.item();
                  boolean isResult = item != null && (item.kind() == ApprovalItem.Kind.ToolResult || item.kind() == ApprovalItem.Kind.ControlResult);
                  Filter filter = switch(d.result()) {
                    case Allow, Call ->
                      Filter.Allow;
                    case Deny ->
                      Filter.Denied;
                    case Pass ->
                      Filter.NoAction;
                  };
                  String action = switch(filter) {
                    case Allow ->
                      "Approved";
                    case Denied ->
                      "Denied";
                    case NoAction ->
                      "Pass";
                  };
                  String tool = item == null ? "-" : isResult ? "Result" : item.tool();
                  String detail = item == null ? "" : text(isResult ? item.result() : item.arguments());
                  String reason = item == null || isResult ? "" : oneLine(item.arguments().path("reason").asText(""));
                  String line = fit(d.sessionId(), SESSION_WIDTH) + " | " + fit(action, ACTION_WIDTH) + " | " + fit(tool, TOOL_WIDTH) + " | " + pad(Integer.toString(d.answerLength()), CHARS_WIDTH);
                  if (reason.length() > 0)
                    line += " | " + reason + ":   " + shorten(detail, DETAIL_MAX);
                  else
                    line += " | " + pad("", 15) + shorten(detail, DETAIL_MAX);
                  return new Entry(filter, line);
                }
          - id: AutoApprovePanel.text
            type: MethodDeclaration
            code: |2
                private static String text(JsonNode node) {
                  return node.isTextual() ? node.asText() : node.toString();
                }
          - id: AutoApprovePanel.oneLine
            type: MethodDeclaration
            code: |2
                private static String oneLine(String s) {
                  return s == null ? "" : s.replaceAll("\\s+", " ").strip();
                }
          - id: AutoApprovePanel.shorten
            type: MethodDeclaration
            code: |2
                private static String shorten(String s, int max) {
                  String line = oneLine(s);
                  return line.length() <= max ? line : line.substring(0, max - 1) + "…";
                }
          - id: AutoApprovePanel.fit
            type: MethodDeclaration
            code: |2
                private static String fit(String s, int width) {
                  return String.format("%-" + width + "s", shorten(s, width));
                }
          - id: AutoApprovePanel.pad
            type: MethodDeclaration
            code: |2
                private static String pad(String s, int width) {
                  return String.format("%" + width + "s", s);
                }
          - id: AutoApprovePanel.dispose
            type: MethodDeclaration
            code: |2
                @Override
                public void dispose() {
                  if (approver != null)
                    approver.removeListener(observer);
                  super.dispose();
                }
          - id: AutoApprovePanel.setFocus
            type: MethodDeclaration
            code: |2
                @Override
                public void setFocus() {
                  // disabled
                }
```
/answer 4843-86-85 allow
ReasoningToken: 36
Control Request:
```yaml
id: 4843-87-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: (enum Kind|record ApprovalItem|class ApprovalItem)
    include:
    - ApprovalItem.java
    exclude:
    - '**/target/**'
  reason: ApprovalItem Struktur prüfen
```
/answer 4843-87-0 allow
Control Request:
```yaml
id: 4843-88-87
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
          matches:
          - "public record ApprovalItem(Kind kind, String id, String tool, JsonNode data, String yaml) {"
          - "public enum Kind {"
```
/answer 4843-88-87 allow
Control Request:
```yaml
id: 4843-89-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
    node_type: RecordDeclaration
  reason: ApprovalItem lesen
```
/answer 4843-89-0 allow
Control Request:
```yaml
id: 4843-8a-89
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
        nodes:
        - id: ApprovalItem
          type: RecordDeclaration
          code: "public record ApprovalItem(Kind kind, String id, String tool, JsonNode data, String yaml) {\n\n\tpublic enum Kind {\n\t\tControlCall, ControlResult, ToolCall, ToolResult\n\t}\n\n\tpublic JsonNode arguments() {\n\t\treturn data.path(\"arguments\");\n\t}\n\n\tpublic JsonNode result() {\n\t\treturn data.path(\"result\");\n\t}\n\n\t/** Item belongs to a control request (MCP bound connector). */\n\tpublic boolean isControl() {\n\t\treturn kind == Kind.ControlCall || kind == Kind.ControlResult;\n\t}\n}\n"
```
/answer 4843-8a-89 allow
ReasoningToken: 42
Control Request:
```yaml
id: 4843-8b-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: record Decision|new Decision\(
    include:
    - '*.java'
    limit: 15
  reason: Decision lesen
```
/answer 4843-8b-0 allow
Control Request:
```yaml
id: 4843-8c-8b
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
        matches:
        - path: AutoApprover.java
          matches:
          - "publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp, 0));"
          - "return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());"
        - path: Decision.java
          matches:
          - "public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,"
          - "return new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null, 0);"
```
/answer 4843-8c-8b allow
ReasoningToken: 41
Control Request:
```yaml
id: 4843-8d-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
    node_type: RecordDeclaration
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    text: publish(new Decision
  reason: Decision und Publish-Kontext
```
/answer 4843-8d-0 allow
Control Request:
```yaml
id: 4843-8e-8d
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
        nodes:
        - id: Decision
          type: RecordDeclaration
          code: "public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,\n\t\tList<String> warnings, String instruction, List<RuleVote> votes, FollowUp followUp, int answerLength) {\n\n\tpublic Decision {\n\t\tresult = result == null ? ApprovalResult.Pass : result;\n\t\treason = reason == null ? \"\" : reason;\n\t\thint = hint == null ? \"\" : hint;\n\t\twarnings = warnings == null ? List.of() : List.copyOf(warnings);\n\t\tinstruction = instruction == null ? \"\" : instruction;\n\t\tvotes = votes == null ? List.of() : List.copyOf(votes);\n\t}\n\n\tpublic static Decision pass(String sessionId) {\n\t\treturn new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null, 0);\n\t}\n}\n"
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
        nodes:
        - id: AutoApprover.onIdle
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
```
/answer 4843-8e-8d allow
ReasoningToken: 126
Control Request:
```yaml
id: 4843-8f-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: new ApprovalItem\(|"thinking"|"text"
    include:
    - '*.java'
    limit: 20
  reason: Text/Thinking Items finden
```
/answer 4843-8f-0 allow
Control Request:
```yaml
id: 4843-90-8f
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src
        matches:
        - path: xy/ai/conscious/model/resolver/AnthropicModelResolver.java
          matches:
          - boolean thinking = capNode.path("thinking").path("supported").asBoolean(false);
        - path: xy/ai/conscious/model/resolver/GeminiModelResolver.java
          matches:
          - boolean thinking = entry.path("thinking").asBoolean(false);
        - path: xy/ai/conscious/autoapprove/AnswerParser.java
          matches:
          - "return new ApprovalItem(kind, node.path(\"id\").asText(\"\"), tool, node, yaml);"
        - path: xy/ai/conscious/autoapprove/rules/DenyOwnPromptFileRule.java
          matches:
          - "public static final String[] PATH_FIELDS = { \"path\", \"paths\", \"directory\", \"file\", \"file_path\", \"filePath\", \"command\", \"pattern\", \"text\", \"content\" };"
        - path: xy/ai/conscious/connector/mcp/MCPClient.java
          matches:
          - sb.append(c.path("text").asText(""));
        - path: xy/ai/conscious/connector/mcp/MCPControlClient.java
          matches:
          - "* Concatenates the \"text\" entries of an MCP \"content\" array (plain, unfenced)."
          - sb.append(c.path("text").asText(""));
        - path: xy/ai/conscious/connector/claudecode/ProtocolParser.java
          matches:
          - "if (\"thinking\".equals(blockType)) {"
          - "// Some versions use \"thinking\" field, others fall back to \"text\""
          - String text = block.path("thinking").asText("");
          - text = block.path("text").asText("");
          - "} else if (recordText && \"text\".equals(blockType)) {"
          - String text = block.path("text").asText("");
        - path: xy/ai/conscious/connector/claudecode/CCRequestBuilder.java
          matches:
          - "textNode.put(\"type\", \"text\");"
          - "textNode.put(\"text\", text);"
        - path: xy/ai/conscious/connector/harness/IncludeProcessor.java
          matches:
          - "node.put(\"text\", match.text);"
```
/answer 4843-90-8f allow
ReasoningToken: 2767
ReasoningToken: 114
Control Request:
```yaml
id: 4843-91-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.Entry
    addImport:
    - import java.util.ArrayList;
    - import java.util.List;
    - import org.eclipse.swt.custom.SashForm;
    source: |2
        private record Entry(Filter filter, String line, String detail) {
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.fauStF|jd84rM
    source: |2
        private final Deque<Entry> buffer = new ArrayDeque<>();
        private final List<Entry> visible = new ArrayList<>();
        private final Set<Filter> shown = EnumSet.allOf(Filter.class);
        private final Consumer<Decision> observer = this::onDecision;
        private Display display;
        private SashForm sash;
        private StyledText console;
        private StyledText details;
        private AutoApprover approver;
        private boolean showOnChange = true;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.createPartControl
    source: |2
        @Override
        public void createPartControl(Composite parent) {
          display = parent.getDisplay();
          sash = new SashForm(parent, SWT.HORIZONTAL);
          console = new StyledText(sash, SWT.READ_ONLY | SWT.V_SCROLL | SWT.H_SCROLL);
          console.setFont(JFaceResources.getTextFont());
          console.setWordWrap(false);
          console.addCaretListener(e -> showDetails(console.getLineAtOffset(e.caretOffset)));
          details = new StyledText(sash, SWT.READ_ONLY | SWT.V_SCROLL | SWT.H_SCROLL);
          details.setFont(JFaceResources.getTextFont());
          details.setWordWrap(true);
          sash.setWeights(new int[] { 60, 40 });
          approver = Activator.getDefault().promptHandler.approver();
          IActionBars bars = getViewSite().getActionBars();
          bars.getToolBarManager().add(createDisableAction());
          bars.getToolBarManager().add(createAutoShowAction());
          bars.getToolBarManager().add(createDetailsAction());
          bars.getToolBarManager().add(createFilterAction());
          bars.updateActionBars();
          approver.addListener(observer);
        }

        private Action createDetailsAction() {
          Action action = new Action("Show Details", IAction.AS_CHECK_BOX) {

            @Override
            public void run() {
              sash.setMaximizedControl(isChecked() ? null : console);
            }
          };
          action.setToolTipText("Show the details of the line at the cursor");
          action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_OBJ_FILE));
          action.setChecked(true);
          return action;
        }

        private void showDetails(int line) {
          if (details == null || details.isDisposed())
            return;
          details.setText(line >= 0 && line < visible.size() ? visible.get(line).detail() : "");
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.createFilterAction
    source: |2
        private Action createFilterAction() {
          Action action = new Action("Filter", IAction.AS_DROP_DOWN_MENU) {
          };
          action.setToolTipText("Select the action types to show");
          action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_ELCL_SYNCED));
          action.setMenuCreator(new IMenuCreator() {
            private Menu menu;

            @Override
            public Menu getMenu(Control parent) {
              dispose();
              menu = new Menu(parent);
              fillMenu(menu);
              return menu;
            }

            @Override
            public Menu getMenu(Menu parent) {
              dispose();
              menu = new Menu(parent);
              fillMenu(menu);
              return menu;
            }

            @Override
            public void dispose() {
              if (menu != null && !menu.isDisposed())
                menu.dispose();
              menu = null;
            }
          });
          return action;
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.append
    source: |2
        private void append(Entry entry) {
          buffer.addLast(entry);
          if (buffer.size() > MAX_LINES && shown.contains(buffer.removeFirst().filter())) {
            visible.remove(0);
            console.replaceTextRange(0, console.getOffsetAtLine(1), "");
          }
          if (shown.contains(entry.filter())) {
            visible.add(entry);
            console.append(entry.line() + "\n");
            scrollToEnd();
            if (showOnChange)
              try {
                getSite().getPage().showView(ID, null, IWorkbenchPage.VIEW_VISIBLE);
              } catch (PartInitException e) {
                // ignore not present
              }
          }
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.rebuild
    source: |2
        private void rebuild() {
          StringBuilder text = new StringBuilder();
          visible.clear();
          for (Entry entry : buffer) {
            if (shown.contains(entry.filter())) {
              visible.add(entry);
              text.append(entry.line()).append('\n');
            }
          }
          console.setText(text.toString());
          scrollToEnd();
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.scrollToEnd
    source: |2
        private void scrollToEnd() {
          int last = visible.size() - 1;
          console.setCaretOffset(last < 0 ? 0 : console.getOffsetAtLine(last));
          console.showSelection();
          showDetails(last);
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.toEntry
    source: |2
        private static Entry toEntry(Decision d) {
          ApprovalItem item = d.item();
          boolean isResult = item != null && (item.kind() == ApprovalItem.Kind.ToolResult || item.kind() == ApprovalItem.Kind.ControlResult);
          Filter filter = switch(d.result()) {
            case Allow, Call ->
              Filter.Allow;
            case Deny ->
              Filter.Denied;
            case Pass ->
              Filter.NoAction;
          };
          String action = switch(filter) {
            case Allow ->
              "Approved";
            case Denied ->
              "Denied";
            case NoAction ->
              "Pass";
          };
          String tool = item == null ? "-" : isResult ? "Result" : item.tool();
          String detail = item == null ? "" : text(isResult ? item.result() : item.arguments());
          String reason = item == null || isResult ? "" : oneLine(item.arguments().path("reason").asText(""));
          String line = fit(d.sessionId(), SESSION_WIDTH) + " | " + fit(action, ACTION_WIDTH) + " | " + fit(tool, TOOL_WIDTH) + " | " + pad(Integer.toString(d.answerLength()), CHARS_WIDTH);
          if (reason.length() > 0)
            line += " | " + reason + ":   " + shorten(detail, DETAIL_MAX);
          else
            line += " | " + pad("", 15) + shorten(detail, DETAIL_MAX);
          return new Entry(filter, line, details(d, isResult));
        }

        private static String details(Decision d, boolean isResult) {
          ApprovalItem item = d.item();
          if (item == null)
            return d.instruction().isBlank() ? d.reason() : d.instruction();
          JsonNode data = item.data();
          JsonNode content = data.path("text").isMissingNode() ? data.path("thinking") : data.path("text");
          if (!isResult && !content.isMissingNode())
            return text(content);
          if (isResult) {
            JsonNode results = item.result().path("structuredContent").path("results");
            return join(d.reason(), pretty(results.isMissingNode() ? item.result() : results));
          }
          String reason = item.arguments().path("reason").asText("");
          return join(reason.isBlank() ? d.reason() : reason, pretty(item.arguments()));
        }

        private static String join(String reason, String body) {
          return reason.isBlank() ? body : reason.strip() + "\n\n" + body;
        }

        private static String pretty(JsonNode node) {
          return node.isTextual() ? node.asText() : node.toPrettyString();
        }
  reason: "Detailbereich, Sash, Toggle-Action und Filter-Anpassungen implementieren"
```
/answer 4843-91-0 allow
Result Stats: id=696ec05a-4fd4-47c8-b551-17fe0acd5a78, total: 42676, in: 22, out: 13031, reason: 0, read: 146686, write: 29623, start: 1795681495528, end: 1795681504134, time: 8537, total: 89928, io: 0, control: 69, sent: 0, recv: 216
Ich habe `AutoApprovePanel.java` angepasst. Kompiliert oder getestet habe ich nichts.

**Layout**
- Das Panel ist jetzt ein `SashForm`: links das Log, rechts der Detailbereich. Die Gewichtung ist 60/40, damit der Detailbereich ungefähr die letzte Spalte überdeckt. Das Log bleibt unverändert.
- Der Detailbereich ist ein schreibgeschützter `StyledText` mit Zeilenumbruch.
- In der ActionBar gibt es einen neuen Toggle-Button „Show Details“, standardmäßig eingeblendet. Beim Ausblenden wird das Log maximiert.

**Cursor und Details**
- Beim Anhängen und beim Neuaufbau des Logs springt der Cursor auf die letzte Logzeile. Der Cursor steht damit am Anfang dieser Zeile statt hinter dem letzten Zeilenumbruch.
- Ein Caret-Listener zeigt die Details der Zeile unter dem Cursor an. Dafür gibt es eine Liste der sichtbaren Einträge, die zu den Logzeilen parallel läuft.
- Wenn du den Cursor versetzt, siehst du die Details einer anderen Zeile. Sobald ein neuer Eintrag kommt, springt der Cursor wieder ans Ende.

**Inhalt der Details**
- **Tool Call:** Zuerst kommt der Reason aus `arguments.reason`, ersatzweise `Decision.reason`. Nach einer Leerzeile folgt `arguments` als Pretty-JSON.
- **Tool Result:** Zuerst kommt ein eventueller Reason. Nach einer Leerzeile folgt `structuredContent.results` als Pretty-JSON. Fehlt dieser Pfad, nehme ich das ganze `result`.
- **Text oder Thinking:** Ich konnte diese Einträge im Code nicht eindeutig erkennen, denn `ApprovalItem.Kind` kennt nur `ControlCall`, `ControlResult`, `ToolCall` und `ToolResult`. Ich habe deshalb geraten: Ein Item mit `text`- oder `thinking`-Feld in den Daten zeigt diesen Text. Bei Einträgen ohne Item (z. B. Idle-Pass) zeigt das Panel die `instruction`, ersatzweise den Reason. Falls Text und Thinking anders ankommen, muss ich `details()` anpassen.

**Filter**
- Der Filter-Button hatte als reine Textaktion die Aufschrift „Filter“. Ich habe ihm ein Icon gegeben (`IMG_ELCL_SYNCED`). Die Aufschrift ist damit weg, und das Dropdown-Menü bleibt. Das Icon ist nur ein Platzhalter, es gibt kein passendes Filter-Symbol unter den Shared Images.
- „No Action“ ist jetzt standardmäßig an. Alle drei Filter sind zu Beginn aktiv.
