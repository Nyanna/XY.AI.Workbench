Das `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java` soll in seiner Action Bar einen Toggle bekommen (rotes Kreuz), um global die Verarbeitung des AutoApprovers zu deaktivieren (Default: Verarbeitung ist an.).

 `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java` gehalten in `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java`
 
 Zudem soll die Action bar einen Toggle bekommen um die automatische Aktivierung bei Inhaltsänderung ein und ausschalten zu können (Default an).
 Die automatische Aktivierung soll analog der Eclipse Console funktionieren. Das heißt, sobald neue Aktionen das Log füllen soll ein potenziell ausgeblendet Panel aufpoppen (Analog der Funktion für STDOUT und STD Error in der Eclipse Console).
SystemInit:  id=7b7a76da-b2f2-4ee0-841b-85f3a1362a08, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 88b5-3-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
  reason: Understand structure of the three files
```
/answer 88b5-3-0 allow
Control Request:
```yaml
id: 88b5-4-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
      nodes:
      - id: MGpBD7|9gbchz
        signature: import java.util.ArrayDeque;
      - id: AutoApprovePanel
        signature: /**  * Console like log of the decisions of the auto approver for all sessions.…
        docstring: Console like log of the decisions of the auto approver for all sessions.
        children:
        - id: AutoApprovePanel.uxnwF9|tMhmxr
          signature: public static final String ID = "xy.ai.workbench.view.approver.AutoApprovePanel…
        - id: AutoApprovePanel.Filter
          signature: "private enum Filter {"
          children:
          - id: AutoApprovePanel.Filter.Cf4hGc|MDDbQP
            signature: "Allow(\"Allow\"), Denied(\"Denied\"), NoAction(\"No Action\");"
          - id: AutoApprovePanel.Filter.VLRnLu|dnJOEd
            signature: private final String label;
          - id: AutoApprovePanel.Filter.Filter
            signature: Filter(String label)
        - id: AutoApprovePanel.Entry
          signature: "private record Entry(Filter filter, String line) {"
        - id: AutoApprovePanel.6OCwuw|TNYxIu
          signature: private final Deque<Entry> buffer = new ArrayDeque<>();
        - id: AutoApprovePanel.createPartControl
          signature: public void createPartControl(Composite parent)
        - id: AutoApprovePanel.createFilterAction
          signature: private Action createFilterAction()
        - id: AutoApprovePanel.fillMenu
          signature: private void fillMenu(Menu menu)
        - id: AutoApprovePanel.onDecision
          signature: private void onDecision(Decision decision)
        - id: AutoApprovePanel.append
          signature: private void append(Entry entry)
        - id: AutoApprovePanel.rebuild
          signature: private void rebuild()
        - id: AutoApprovePanel.scrollToEnd
          signature: private void scrollToEnd()
        - id: AutoApprovePanel.toEntry
          signature: private static Entry toEntry(Decision d)
        - id: AutoApprovePanel.text
          signature: private static String text(JsonNode node)
        - id: AutoApprovePanel.oneLine
          signature: private static String oneLine(String s)
        - id: AutoApprovePanel.shorten
          signature: "private static String shorten(String s, int max)"
        - id: AutoApprovePanel.fit
          signature: "private static String fit(String s, int width)"
        - id: AutoApprovePanel.pad
          signature: "private static String pad(String s, int width)"
        - id: AutoApprovePanel.dispose
          signature: public void dispose()
        - id: AutoApprovePanel.setFocus
          signature: public void setFocus()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: Zpzs7d|sxLd1L
        signature: import java.util.ArrayList;
      - id: AutoApprover
        signature: /**  * Evaluates the registered rules against an answer and derives the follow-…
        docstring: Evaluates the registered rules against an answer and derives the follow-up
        children:
        - id: AutoApprover.tK23pE|KMoPOJ
          signature: public static final int DEFAULT_MAX_CHAIN = 20;
        - id: AutoApprover.AutoApprover
          signature: public AutoApprover()
        - id: AutoApprover.AutoApprover_2
          signature: public AutoApprover(RuleRegistry registry)
        - id: AutoApprover.rules
          signature: public RuleRegistry rules()
        - id: AutoApprover.addListener
          signature: public void addListener(Consumer<Decision> listener)
        - id: AutoApprover.removeListener
          signature: public void removeListener(Consumer<Decision> listener)
        - id: AutoApprover.setMaxChain
          signature: public void setMaxChain(int maxChain)
          docstring: Maximum number of consecutive automatic follow-up prompts.
        - id: AutoApprover.setIdleInstruction
          signature: public void setIdleInstruction(String instruction)
        - id: AutoApprover.forget
          signature: public void forget(String sessionId)
        - id: AutoApprover.evaluate
          signature: "public Decision evaluate(AIAnswer ans, int depth)"
        - id: AutoApprover.onIdle
          signature: public Optional<FollowUp> onIdle(String sessionId)
        - id: AutoApprover.collect
          signature: private List<RuleVote> collect(ApprovalContext ctx)
        - id: AutoApprover.decide
          signature: "private Decision decide(ApprovalContext ctx, List<RuleVote> votes, int depth)"
        - id: AutoApprover.resultFor
          signature: "private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolea…"
        - id: AutoApprover.followUpFor
          signature: "private FollowUp followUpFor(ApprovalContext ctx, ApprovalResult result, String…"
        - id: AutoApprover.withBlock
          signature: "private static List<String> withBlock(ApprovalItem item, String commandLine)"
          docstring: "Yaml block followed by the command line, as the operator would select it."
        - id: AutoApprover.denial
          signature: "private static String denial(ApprovalItem item, String reason)"
        - id: AutoApprover.command
          signature: "private static String command(String base, String extra)"
        - id: AutoApprover.oneLine
          signature: private static String oneLine(String s)
        - id: AutoApprover.selection
          signature: private static Selection selection(List<String> lines)
        - id: AutoApprover.addNonBlank
          signature: "private static void addNonBlank(List<String> list, String text)"
        - id: AutoApprover.enrich
          signature: "private static void enrich(AIAnswer ans, List<String> warnings)"
          docstring: Warnings go in front so the command line stays the last line of the answer.
        - id: AutoApprover.publish
          signature: private void publish(Decision decision)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
      nodes:
      - id: 4MFHdZ|HIydwL
        signature: import java.util.ArrayList;
      - id: bIIjix|2Uec6f
        signature: import xy.ai.conscious.model.Model;
      - id: PromptHandler
        signature: "/**  * Prompt-specific orchestration extracted from {"
        docstring: "Prompt-specific orchestration extracted from {@code AISessionManager}: drives"
        children:
        - id: PromptHandler.QEglBi|3VfDOv
          signature: private final AdaptingConnector connector;
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, IActiveEditor editorListener, AdaptingC…"
        - id: PromptHandler.approver
          signature: public AutoApprover approver()
        - id: PromptHandler.addAnswerObs
          signature: public void addAnswerObs(Consumer<AIAnswer> obs)
        - id: PromptHandler.execute
          signature: public void execute(IExecutor exec)
        - id: PromptHandler.PromptRun
          signature: "private class PromptRun implements IMonitoredRunnable {"
          children:
          - id: PromptHandler.PromptRun.EeONka|ZoEVO9
            signature: private final IExecutor exec;
          - id: PromptHandler.PromptRun.PromptRun
            signature: private PromptRun(IExecutor exec)
          - id: PromptHandler.PromptRun.PromptRun_2
            signature: "private PromptRun(IExecutor exec, Prompt preset, int depth)"
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
        - id: PromptHandler.prepareRequest_2
          signature: "private IModelRequest prepareRequest(IExecutor exec, boolean batchFix, Prompt p…"
        - id: PromptHandler.preprocessPrompt
          signature: private void preprocessPrompt(Prompt prompt)
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(IExecutor exec, IModelRequest req, IProgress mon)"
        - id: PromptHandler.onIdle
          signature: "public void onIdle(IExecutor exec, String sessionId)"
          docstring: Entry point for an external idle trigger.
        - id: PromptHandler.scheduleFollowUp
          signature: "private void scheduleFollowUp(IExecutor exec, FollowUp followUp, int depth)"
        - id: PromptHandler.evaluate
          signature: "private Decision evaluate(AIAnswer ans, int depth)"
        - id: PromptHandler.checkGitSupport
          signature: "public void checkGitSupport(IExecutor exec, IModelRequest req)"
```
/answer 88b5-4-3 allow
Control Request:
```yaml
id: 88b5-5-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    ids:
    - AutoApprovePanel.createPartControl
    - AutoApprovePanel.createFilterAction
    - AutoApprovePanel.onDecision
    - AutoApprovePanel.append
    - AutoApprovePanel.dispose
    - AutoApprovePanel.6OCwuw|TNYxIu
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover.evaluate
    - AutoApprover.onIdle
    - AutoApprover.setMaxChain
    - AutoApprover.AutoApprover_2
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
    ids:
    - PromptHandler.onIdle
    - PromptHandler.scheduleFollowUp
    - PromptHandler.evaluate
    - PromptHandler.approver
  reason: Read relevant code
```
/answer 88b5-5-0 allow
Control Request:
```yaml
id: 88b5-6-5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
      nodes:
      - id: AutoApprovePanel.createPartControl
        type: MethodDeclaration
        code: |2
            @Override
            public void createPartControl(Composite parent) {
              display = parent.getDisplay();
              console = new StyledText(parent, SWT.READ_ONLY | SWT.V_SCROLL | SWT.H_SCROLL);
              console.setFont(JFaceResources.getTextFont());
              console.setWordWrap(false);
              IActionBars bars = getViewSite().getActionBars();
              bars.getToolBarManager().add(createFilterAction());
              bars.updateActionBars();
              approver = Activator.getDefault().promptHandler.approver();
              approver.addListener(observer);
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
        code: |2
            private void append(Entry entry) {
              buffer.addLast(entry);
              if (buffer.size() > MAX_LINES && shown.contains(buffer.removeFirst().filter()))
                console.replaceTextRange(0, console.getOffsetAtLine(1), "");
              if (shown.contains(entry.filter())) {
                console.append(entry.line() + "\n");
                scrollToEnd();
              }
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
      - id: AutoApprovePanel.6OCwuw|TNYxIu
        type: StatementGroup
        code: |2
            private final Deque<Entry> buffer = new ArrayDeque<>();
            private final Set<Filter> shown = EnumSet.of(Filter.Allow, Filter.Denied);
            private final Consumer<Decision> observer = this::onDecision;
            private Display display;
            private StyledText console;
            private AutoApprover approver;
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover.evaluate
        type: MethodDeclaration
        code: |2
            public Decision evaluate(AIAnswer ans, int depth) {
              SessionSnapshot session = sessions.record(ans);
              List<ApprovalItem> items = AnswerParser.parse(ans.answer);
              ApprovalItem item = items.size() == 1 ? items.get(0) : null;
              sessions.track(session.id(), items);
              ApprovalItem call = sessions.callFor(session.id(), item);
              ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, call, session);
              Decision decision = decide(ctx, collect(ctx), depth);
              enrich(ans, decision.warnings());
              publish(decision);
              return decision;
            }
      - id: AutoApprover.onIdle
        type: MethodDeclaration
        code: |2
            public Optional<FollowUp> onIdle(String sessionId) {
              Prompt origin = sessions.lastPrompt(sessionId);
              String instruction = idleInstruction;
              if (origin == null || instruction.isBlank())
                return Optional.empty();
              FollowUp followUp = new FollowUp(origin, selection(instruction.lines().toList()));
              publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp, 0));
              return Optional.of(followUp);
            }
      - id: AutoApprover.setMaxChain
        type: MethodDeclaration
        code: |2
            public void setMaxChain(int maxChain) {
              this.maxChain = maxChain;
            }
      - id: AutoApprover.AutoApprover_2
        type: ConstructorDeclaration
        code: |2
            public AutoApprover(RuleRegistry registry) {
              this.registry = registry;
            }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler.onIdle
        type: MethodDeclaration
        code: |2
            public void onIdle(IExecutor exec, String sessionId) {
              approver.onIdle(sessionId).ifPresent(f -> scheduleFollowUp(exec, f, 1));
            }
      - id: PromptHandler.scheduleFollowUp
        type: MethodDeclaration
        code: |2
            private void scheduleFollowUp(IExecutor exec, FollowUp followUp, int depth) {
              try {
                Prompt prompt = input.followUp(followUp.origin(), followUp.selection());
                exec.schedule("Auto Approve", new PromptRun(exec, prompt, depth));
              } catch (RuntimeException e) {
                LOG.error(e.getMessage(), e);
              }
            }
      - id: PromptHandler.evaluate
        type: MethodDeclaration
        code: |2
            private Decision evaluate(AIAnswer ans, int depth) {
              try {
                return approver.evaluate(ans, depth);
              } catch (RuntimeException e) {
                LOG.error(e.getMessage(), e);
                return Decision.pass(ans.sessionId);
              }
            }
      - id: PromptHandler.approver
        type: MethodDeclaration
        code: "  public AutoApprover approver() {\n\treturn approver;\n  }\n"
```
/answer 88b5-6-5 allow
Control Request:
```yaml
id: 88b5-7-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  node_type: Import|import.*
  reason: See imports
```
/answer 88b5-7-0 allow
Control Request:
```yaml
id: 88b5-8-7
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
      nodes:
      - id: MGpBD7|9gbchz
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
```
/answer 88b5-8-7 allow
Control Request:
```yaml
id: 88b5-9-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.evaluate
    old_text: '    SessionSnapshot session = sessions.record(ans);'
    new_text: |2-
          if (!enabled)
            return Decision.pass(ans.sessionId);
          SessionSnapshot session = sessions.record(ans);
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.onIdle
    old_text: '    Prompt origin = sessions.lastPrompt(sessionId);'
    new_text: |2-
          if (!enabled)
            return Optional.empty();
          Prompt origin = sessions.lastPrompt(sessionId);
  reason: Skip processing when disabled
```
/answer 88b5-9-0 allow
ReasoningToken: 688
Control Request:
```yaml
id: 88b5-a-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.setMaxChain
    position: before
    source: |2+
        public boolean isEnabled() {
          return enabled;
        }

        /** Globally switches the processing on or off; while off, answers pass untouched. */
        public void setEnabled(boolean enabled) {
          this.enabled = enabled;
        }

  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.tK23pE|KMoPOJ
    position: after
    source: |2-

        private volatile boolean enabled = true;
  reason: Add enabled flag and accessors
```
/answer 88b5-a-0 allow
Control Request:
```yaml
id: 88b5-b-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.createPartControl
    start_marker: '    IActionBars bars ='
    end_marker: '    approver.addListener(observer);'
    content: |2-
          approver = Activator.getDefault().promptHandler.approver();
          IActionBars bars = getViewSite().getActionBars();
          bars.getToolBarManager().add(createDisableAction());
          bars.getToolBarManager().add(createAutoShowAction());
          bars.getToolBarManager().add(createFilterAction());
          bars.updateActionBars();
          approver.addListener(observer);
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.append
    start_marker: '      console.append(entry.line()'
    end_marker: '      scrollToEnd();'
    content: |2-
            console.append(entry.line() + "\n");
            scrollToEnd();
            if (showOnChange)
              getSite().getPage().bringToTop(this);
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.6OCwuw|TNYxIu
    start_marker: '  private AutoApprover approver;'
    end_marker: '  private AutoApprover approver;'
    content: |2-
        private AutoApprover approver;
        private boolean showOnChange = true;
  reason: "Wire actions, auto-show and field"
```
/answer 88b5-b-0 allow
ReasoningToken: 29
Control Request:
```yaml
id: 88b5-c-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.createFilterAction
    position: before
    source: |2+
        private Action createDisableAction() {
          Action action = new Action("Disable Auto Approver", IAction.AS_CHECK_BOX) {
            @Override
            public void run() {
              approver.setEnabled(!isChecked());
            }
          };
          action.setToolTipText("Disable the processing of the auto approver");
          action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages()
              .getImageDescriptor(ISharedImages.IMG_ELCL_REMOVE));
          action.setChecked(!approver.isEnabled());
          return action;
        }

        private Action createAutoShowAction() {
          Action action = new Action("Show When Content Changes", IAction.AS_CHECK_BOX) {
            @Override
            public void run() {
              showOnChange = isChecked();
            }
          };
          action.setToolTipText("Show this view when new entries are logged");
          action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages()
              .getImageDescriptor(ISharedImages.IMG_ETOOL_DEF_PERSPECTIVE));
          action.setChecked(showOnChange);
          return action;
        }

  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: MGpBD7|9gbchz
    position: after
    source: |
      import org.eclipse.ui.ISharedImages;
      import org.eclipse.ui.PlatformUI;
  reason: Add action factories and imports
```
/answer 88b5-c-0 allow
Control Request:
```yaml
id: 88b5-d-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  reason: Check syntax
```
/answer 88b5-d-0 allow
Result Stats: id=7b7a76da-b2f2-4ee0-841b-85f3a1362a08, total: 23155, in: 12, out: 4773, reason: 0, read: 63893, write: 18370, start: 1794255470376, end: 1794255476488, time: 6044, total: 28950, io: 0, control: 68, sent: 0, recv: 159
Die Action Bar des `AutoApprovePanel` hat jetzt zwei neue Toggles. Kompiliert oder getestet habe ich nichts, nur die Syntax beider Dateien geprüft.

**`AutoApprover`**
- Neues Flag `enabled`, Default `true`, mit `isEnabled()` und `setEnabled(boolean)`.
- Ist es aus, gibt `evaluate()` direkt `Decision.pass(...)` zurück. Es wird dann nichts aufgezeichnet, ausgewertet oder veröffentlicht, und es entsteht kein Follow-up.
- Ist es aus, liefert `onIdle()` ein leeres `Optional`.
- `PromptHandler` musste ich nicht ändern. Er hält den `AutoApprover` bereits und ruft `evaluate()` und `onIdle()` darauf auf, also greift das Flag dort automatisch.

**`AutoApprovePanel`**
- **Deaktivieren-Toggle:** Er trägt das rote Kreuz (`IMG_ELCL_REMOVE`). Ist er gedrückt, ist der AutoApprover aus. Der Anfangszustand kommt aus `approver.isEnabled()`.
- **Auto-Anzeige-Toggle:** "Show When Content Changes", Default an. Ist er an, ruft `append()` nach jedem sichtbar angehängten Eintrag `getSite().getPage().bringToTop(this)` auf. Das entspricht dem Verhalten der Eclipse Console, wenn das Panel hinter einem anderen Tab liegt. Einträge, die der Filter ausblendet, lösen das nicht aus.
- Für den Auto-Anzeige-Toggle gibt es kein passendes Standardicon. Ich habe `IMG_ETOOL_DEF_PERSPECTIVE` als Platzhalter genommen. Sag Bescheid, wenn du ein eigenes Icon willst.

Wenn das Panel ganz geschlossen ist, hört es nicht auf den Approver, weil der Listener erst in `createPartControl` registriert wird. Dann poppt auch nichts auf. Ein Aufpoppen eines geschlossenen Panels würde einen dauerhaften Listener plus `showView(ID, null, VIEW_VISIBLE)` brauchen. Das kann ich ergänzen.
