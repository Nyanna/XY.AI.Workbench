Wenn parallel in einem anderen Editor mit aktivierten Autoapprover gearbeitet wird, erscheint der Fehler:
"Cannot process command: no active Claude Code session exists"

- Der Auto-Approver sollte Parallel in mehreren Prompts verwendet werden können.
- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java`

Es passiert vermutlich direkt, nachdem ein Result oder Tool Call allowed wird. Ich nehme an, es gibt eine Lücke und der PromptHandler (xy.ai.conscious.connector.harness.PromptHandler) greift irgend trotz der eigentlich selbstbezüglichen Schleife auf den Editor zu. 
SystemInit:  id=3673575c-0943-4875-9ad8-d6b1f50cdca6, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst den AutoApprover und den PromptHandler an.
Control Request:
```yaml
id: 06c1-140-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  reason: Struktur des AutoApprover verstehen
```
/answer 06c1-140-0 allow
Control Request:
```yaml
id: 06c1-141-140
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
/answer 06c1-141-140 allow
ReasoningToken: 19
Control Request:
```yaml
id: 06c1-142-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: no active Claude Code session
    include:
    - '*.java'
    exclude:
    - '**/target/**'
    - '**/bin/**'
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class PromptHandler
    include:
    - '*.java'
    exclude:
    - '**/target/**'
    - '**/bin/**'
  reason: Fehlermeldung und PromptHandler lokalisieren
```
/answer 06c1-142-0 allow
Control Request:
```yaml
id: 06c1-143-142
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
        matches:
        - "throw new IllegalStateException(\"Cannot process command: no active Claude Code session exists\");"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - "public class PromptHandler {"
```
/answer 06c1-143-142 allow
Control Request:
```yaml
id: 06c1-144-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover.evaluate
    - AutoApprover.onIdle
    - AutoApprover.forget
    - AutoApprover.followUpFor
    - AutoApprover.u5j4A4|wS5sLU
  reason: Kernlogik des AutoApprover lesen
```
/answer 06c1-144-0 allow
Control Request:
```yaml
id: 06c1-145-144
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
      - id: AutoApprover.forget
        type: MethodDeclaration
        code: |2
            public void forget(String sessionId) {
              sessions.forget(sessionId);
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
      - id: AutoApprover.u5j4A4|wS5sLU
        type: StatementGroup
        code: |2
            private final RuleRegistry registry;
            private final SessionTracker sessions = new SessionTracker();
            private final List<Consumer<Decision>> listeners = new CopyOnWriteArrayList<>();
            private volatile String idleInstruction = "Continue with the task.";
            private volatile boolean enabled = true;
```
/answer 06c1-145-144 allow
Control Request:
```yaml
id: 06c1-146-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
  reason: PromptHandler und SessionManager Struktur
```
/answer 06c1-146-0 allow
Control Request:
```yaml
id: 06c1-147-146
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
      nodes:
      - id: 4MFHdZ|HIydwL
        excert: import java.util.ArrayList;
      - id: bIIjix|2Uec6f
        excert: import xy.ai.conscious.model.Model;
      - id: PromptHandler
        excert: "/**  * Prompt-specific orchestration extracted from {"
        docstring: "Prompt-specific orchestration extracted from {@code AISessionManager}: drives"
        children:
        - id: PromptHandler.QEglBi|3VfDOv
          excert: private final AdaptingConnector connector;
        - id: PromptHandler.PromptHandler
          excert: "public PromptHandler(ConfigManager cfg, IActiveEditor editorListener, AdaptingC…"
        - id: PromptHandler.approver
          excert: public AutoApprover approver()
        - id: PromptHandler.addAnswerObs
          excert: public void addAnswerObs(Consumer<AIAnswer> obs)
        - id: PromptHandler.execute
          excert: public void execute(IExecutor exec)
        - id: PromptHandler.PromptRun
          excert: "private class PromptRun implements IMonitoredRunnable {"
          children:
          - id: PromptHandler.PromptRun.EeONka|ZoEVO9
            excert: private final IExecutor exec;
          - id: PromptHandler.PromptRun.PromptRun
            excert: private PromptRun(IExecutor exec)
          - id: PromptHandler.PromptRun.PromptRun_2
            excert: "private PromptRun(IExecutor exec, Prompt preset, int depth)"
          - id: PromptHandler.PromptRun.run
            excert: public void run(IProgress mon)
        - id: PromptHandler.queueAsync
          excert: public void queueAsync(IExecutor exec)
        - id: PromptHandler.queueSync
          excert: "private void queueSync(IExecutor exec, IProgress mon)"
        - id: PromptHandler.queueAndSubmit
          excert: public void queueAndSubmit(IExecutor exec)
        - id: PromptHandler.prepareRequest
          excert: "private IModelRequest prepareRequest(IExecutor exec, boolean batchFix, IProgres…"
        - id: PromptHandler.prepareRequest_2
          excert: "private IModelRequest prepareRequest(IExecutor exec, boolean batchFix, Prompt p…"
        - id: PromptHandler.preprocessPrompt
          excert: private void preprocessPrompt(Prompt prompt)
        - id: PromptHandler.executeInner
          excert: "private AIAnswer executeInner(IExecutor exec, IModelRequest req, IProgress mon)"
        - id: PromptHandler.onIdle
          excert: "public void onIdle(IExecutor exec, String sessionId)"
          docstring: Entry point for an external idle trigger.
        - id: PromptHandler.scheduleFollowUp
          excert: "private void scheduleFollowUp(IExecutor exec, FollowUp followUp, int depth)"
        - id: PromptHandler.evaluate
          excert: "private Decision evaluate(AIAnswer ans, int depth)"
        - id: PromptHandler.checkGitSupport
          excert: "public void checkGitSupport(IExecutor exec, IModelRequest req)"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
      nodes:
      - id: k0tyZU|ioWIOl
        excert: import java.util.ArrayList;
      - id: CCSessionManager
        excert: "public class CCSessionManager {"
        children:
        - id: CCSessionManager.BqTTt6|umAa9G
          excert: public final static String CREATE_NEW_MARKER = "CREATE_NEW_MARKER";
        - id: CCSessionManager.requestSession
          excert: "public synchronized CCSession requestSession(String selectedUuid, CCSessionPara…"
        - id: CCSessionManager.getSession
          excert: "public synchronized CCSession getSession(String selectedUuid, CCSessionParamete…"
        - id: CCSessionManager.addSession
          excert: private CCSession addSession(CCSession session)
        - id: CCSessionManager.importSession
          excert: "public synchronized CCSession importSession(String uuid, CCSessionParameters pa…"
        - id: CCSessionManager.terminateSessions
          excert: public synchronized void terminateSessions(List<String> toTerminate)
        - id: CCSessionManager.terminateSession
          excert: public synchronized void terminateSession(String uuidOrHash)
        - id: CCSessionManager.getSessions
          excert: public synchronized List<CCSession> getSessions()
        - id: CCSessionManager.addChangeListener
          excert: public void addChangeListener(Consumer<List<CCSession>> listener)
        - id: CCSessionManager.removeChangeListener
          excert: public void removeChangeListener(Consumer<List<CCSession>> listener)
        - id: CCSessionManager.getSelectedSessionUuid
          excert: public String getSelectedSessionUuid()
        - id: CCSessionManager.setSelectedSessionUuid
          excert: public void setSelectedSessionUuid(String uuid)
        - id: CCSessionManager.onSessionChanged
          excert: void onSessionChanged(CCSession session)
        - id: CCSessionManager.cleanupInvalidTerminated
          excert: private void cleanupInvalidTerminated()
        - id: CCSessionManager.findByUuid
          excert: private CCSession findByUuid(String uuid)
        - id: CCSessionManager.findByHash
          excert: private CCSession findByHash(String hash)
        - id: CCSessionManager.fireChanged
          excert: private void fireChanged()
        - id: CCSessionManager.fireChanged_2
          excert: private void fireChanged(List<Consumer<List<CCSession>>> changeListeners)
```
/answer 06c1-147-146 allow
Control Request:
```yaml
id: 06c1-148-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
    ids:
    - PromptHandler.QEglBi|3VfDOv
    - PromptHandler.PromptHandler
    - PromptHandler.execute
    - PromptHandler.PromptRun
    - PromptHandler.queueAsync
    - PromptHandler.queueSync
    - PromptHandler.queueAndSubmit
    - PromptHandler.prepareRequest
    - PromptHandler.prepareRequest_2
    - PromptHandler.preprocessPrompt
    - PromptHandler.executeInner
    - PromptHandler.onIdle
    - PromptHandler.scheduleFollowUp
    - PromptHandler.evaluate
  reason: "Ablauf von Prompt, Evaluate und FollowUp nachvollziehen, um den Editor-Zugriff zu finden"
```
/answer 06c1-148-0 allow
Control Request:
```yaml
id: 06c1-149-148
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler.QEglBi|3VfDOv
        type: StatementGroup
        code: |2
            private final AdaptingConnector connector;
            private final IEditorInterface editIfc;
            private final AIBatchManager batch;
            private final PromptInputHandler input;
            private final IGitSupport gitSupport;
            private final AutoApprover approver = new AutoApprover();
            private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();
      - id: PromptHandler.PromptHandler
        type: ConstructorDeclaration
        code: |2
            public PromptHandler(ConfigManager cfg, IActiveEditor editorListener, AdaptingConnector connector, AIBatchManager batch, IEditorInterface editIfc, IGitSupport gitSupport) {
              this.connector = connector;
              this.editIfc = editIfc;
              this.batch = batch;
              this.input = new PromptInputHandler(cfg, editorListener);
              this.gitSupport = gitSupport;
            }
      - id: PromptHandler.execute
        type: MethodDeclaration
        code: |2
            public void execute(IExecutor exec) {
              exec.schedule("Starting Prompt", new PromptRun(exec));
            }
      - id: PromptHandler.PromptRun
        type: ClassOrInterfaceDeclaration
        excert: "private class PromptRun implements IMonitoredRunnable {"
        children:
        - id: PromptHandler.PromptRun.EeONka|ZoEVO9
          type: StatementGroup
          code: |2
                private final IExecutor exec;
                private final Prompt preset;
                private final int turnDepth;
        - id: PromptHandler.PromptRun.PromptRun
          type: ConstructorDeclaration
          code: |2
                private PromptRun(IExecutor exec) {
                  this(exec, null, 0);
                }
        - id: PromptHandler.PromptRun.PromptRun_2
          type: ConstructorDeclaration
          code: |2
                private PromptRun(IExecutor exec, Prompt preset, int depth) {
                  this.exec = exec;
                  this.preset = preset;
                  this.turnDepth = depth;
                }
        - id: PromptHandler.PromptRun.run
          type: MethodDeclaration
          code: |2
                @Override
                public void run(IProgress mon) {
                  IProgress sub = mon.setTask("Executing prompt", 4);
                  String reqId = null;
                  Decision decision = null;
                  try {
                    sub.setTaskName("Prepare inputs");
                    var req = prepareRequest(exec, false, preset, sub);
                    sub.worked();
                    sub.setTaskName("Insert Tag");
                    editIfc.insertTag(exec, req, sub);
                    reqId = req.getID();
                    mon.worked();
                    sub.setTaskName("Execute prompt");
                    AIAnswer ans = executeInner(exec, req, sub);
                    mon.worked();
                    sub.setTaskName("Process Answer");
                    Decision evaluated = evaluate(ans, turnDepth);
                    editIfc.replaceTag(ans, sub);
                    decision = evaluated;
                    mon.worked();
                  } catch (Exception e) {
                    LOG.error(e.getMessage(), e);
                    // uncatched error case
                    if (reqId != null) {
                      AIAnswer error = new AIAnswer(reqId);
                      error.answer = e.getMessage();
                      editIfc.replaceTag(error, sub);
                    }
                  } finally {
                    mon.done();
                  }
                  if (decision != null && decision.followUp() != null)
                    scheduleFollowUp(exec, decision.followUp(), turnDepth + 1);
                }
      - id: PromptHandler.queueAsync
        type: MethodDeclaration
        code: |2
            public void queueAsync(IExecutor exec) {
              exec.schedule("Enqueue Prompt", mon -> {
                queueSync(exec, mon);
              });
            }
      - id: PromptHandler.queueSync
        type: MethodDeclaration
        code: |2
            private void queueSync(IExecutor exec, IProgress mon) {
              IProgress sub = mon.setTask("Enqueue batch prompt", 3);
              sub.setTaskName("Prepare inputs");
              var req = prepareRequest(exec, true, sub.subProgress());
              sub.setTaskName("Insert Tag");
              editIfc.insertTag(exec, req, sub.subProgress());
              sub.setTaskName("Enqueue prompt");
              batch.enqueue(req, sub.subProgress());
            }
      - id: PromptHandler.queueAndSubmit
        type: MethodDeclaration
        code: |2
            public void queueAndSubmit(IExecutor exec) {
              exec.schedule("Enqueue Prompt", (mon) -> {
                queueSync(exec, mon);
                batch.submitBatches(mon);
              });
            }
      - id: PromptHandler.prepareRequest
        type: MethodDeclaration
        code: |2
            private IModelRequest prepareRequest(IExecutor exec, boolean batchFix, IProgress mon) {
              return prepareRequest(exec, batchFix, null, mon);
            }
      - id: PromptHandler.prepareRequest_2
        type: MethodDeclaration
        code: |2
            private IModelRequest prepareRequest(IExecutor exec, boolean batchFix, Prompt preset, IProgress mon) {
              IProgress sub = mon.setTask("Preparing Call", 1);
              sub.setTaskName("Preparing Call");
              Prompt prompt = preset != null ? preset : input.buildPrompt(exec, batchFix);
              preprocessPrompt(prompt);
              IModelRequest req = connector.createRequest(prompt, sub);
              req.setPrompt(prompt);
              sub.worked();
              return req;
            }
      - id: PromptHandler.preprocessPrompt
        type: MethodDeclaration
        code: |2
            private void preprocessPrompt(Prompt prompt) {
              for (String key : prompt.config.keys.split(",")) if (prompt.config.model.cap.acceptsKey(key))
                prompt.config.keys = key;
              if (prompt.arg.command instanceof CallCommand || prompt.arg.command instanceof ToolCommand)
                prompt.config.model = Model.MCP_TOOLS;
            }
      - id: PromptHandler.executeInner
        type: MethodDeclaration
        code: |2
            private AIAnswer executeInner(IExecutor exec, IModelRequest req, IProgress mon) {
              exec.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));
              IModelResponse resp = connector.executeRequest(req, mon);
              AIAnswer res = connector.convertResponse(resp, mon);
              res.prompt = req.getPrompt();
              exec.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));
              checkGitSupport(exec, req);
              return res;
            }
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
```
/answer 06c1-149-148 allow
Control Request:
```yaml
id: 06c1-14a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: no active Claude Code session|getSelectedSessionUuid|getActiveEditor|activeEditor|editorListener
    include:
    - '*.java'
    exclude:
    - '**/target/**'
    - '**/bin/**'
    limit: 50
  reason: "Alle Stellen finden, an denen auf den aktiven Editor / die ausgewählte Session zugegriffen wird"
```
/answer 06c1-14a-0 allow
Control Request:
```yaml
id: 06c1-14b-14a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/connector/claudecode/CCConnector.java
        matches:
        - "session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);"
        - "session = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);"
      - path: xy/ai/conscious/connector/claudecode/CCSessionManager.java
        matches:
        - "throw new IllegalStateException(\"Cannot process command: no active Claude Code session exists\");"
        - "public String getSelectedSessionUuid() {"
      - path: xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - "public PromptHandler(ConfigManager cfg, IActiveEditor editorListener, AdaptingConnector connector, AIBatchManager batch, IEditorInterface editIfc, IGitSupport gitSupport) {"
        - "this.input = new PromptInputHandler(cfg, editorListener);"
      - path: xy/ai/conscious/connector/harness/PromptInputHandler.java
        matches:
        - private final IActiveEditor editorListener;
        - "public PromptInputHandler(ConfigManager cfg, IActiveEditor editorListener) {"
        - this.editorListener = editorListener;
        - arg.setEditor(editorListener.getLastTextEditor());
        - arg.absoluteFilePath = editorListener.resolveAbsoluteFilePath();
        - arg.project = editorListener.resolveProjectPath();
        - "detectSelection(editorListener.getSelection(), arg);"
        - "detectFullFile(editorListener.getFileContent(), arg);"
        - Selection full = editorListener.getFileContent();
```
/answer 06c1-14b-14a allow
Control Request:
```yaml
id: 06c1-14c-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
  reason: followUp-Methode und Session-Auswahl im Connector finden
```
/answer 06c1-14c-0 allow
Control Request:
```yaml
id: 06c1-14d-14c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
      nodes:
      - id: U3FBWT|pBPOgv
        excert: import java.util.regex.Matcher;
      - id: PromptInputHandler
        excert: "/**  * Input handling extracted from {"
        docstring: "Input handling extracted from {@link PromptHandler}: aggregates editor/config"
        children:
        - id: PromptInputHandler.3vFeve|ODKP5L
          excert: private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)^```…
        - id: PromptInputHandler.PromptInputHandler
          excert: "public PromptInputHandler(ConfigManager cfg, IActiveEditor editorListener)"
        - id: PromptInputHandler.buildPrompt
          excert: "public Prompt buildPrompt(IExecutor exec, boolean batch)"
        - id: PromptInputHandler.followUp
          excert: "public Prompt followUp(Prompt origin, Selection sel)"
          docstring: "Builds the prompt an operator would have triggered with the given selection,"
        - id: PromptInputHandler.detectSelection
          excert: "private void detectSelection(Selection sel, PromptArguments arg)"
        - id: PromptInputHandler.detectFullFile
          excert: "private void detectFullFile(Selection content, PromptArguments arg)"
        - id: PromptInputHandler.detectedCmd
          excert: "private static boolean detectedCmd(String line, String[] lines, int lineIndex, …"
        - id: PromptInputHandler.captureYamlBlock
          excert: "private static String captureYamlBlock(String[] lines, int lineIndex)"
        - id: PromptInputHandler.ParsedCommand
          excert: "public record ParsedCommand(Command command, String yaml) {"
        - id: PromptInputHandler.parseTrailingCommand
          excert: public static ParsedCommand parseTrailingCommand(String text)
          docstring: Detects the command on the last line of the text together with the YAML block p…
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
      nodes:
      - id: K2G7xy|dFqaMs
        excert: import java.io.IOException;
      - id: Z8xFy6|XvwUDR
        excert: import xy.ai.conscious.model.AIAnswer;
      - id: CCConnector
        excert: "public class CCConnector implements IAIConnector<CCRequest, CCResponse> {"
        children:
        - id: CCConnector.gou8f5|51qlkr
          excert: private static final int DEAD_TIMEOUT = 180_000;
        - id: CCConnector.CCConnector
          excert: "public CCConnector(CCSessionManager sessionManager, IIncludeAdapter includeAdap…"
        - id: CCConnector.getSupportedKeyPattern
          excert: public KeyPattern getSupportedKeyPattern()
        - id: CCConnector.createRequest
          excert: "public CCRequest createRequest(Prompt prompt, IProgress mon)"
        - id: CCConnector.SessionRequestCallbacks
          excert: private static class SessionRequestCallbacks implements SessionCallbacks<String…
          children:
          - id: CCConnector.SessionRequestCallbacks.message
            excert: "public String message(Role role, String text)"
        - id: CCConnector.executeRequest
          excert: "public CCResponse executeRequest(CCRequest req, IProgress mon)"
        - id: CCConnector.readUntilResult
          excert: "private CCResponse readUntilResult(CCRequest req, CCSession session, IProgress …"
        - id: CCConnector.abbreviateForStatus
          excert: private static String abbreviateForStatus(String s)
        - id: CCConnector.convertResponse
          excert: "public AIAnswer convertResponse(CCResponse resp, IProgress mon)"
```
/answer 06c1-14d-14c allow
ReasoningToken: 32
Control Request:
```yaml
id: 06c1-14e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: no active Claude Code session
    include:
    - CCSessionManager.java
    intent: editing
  reason: Wurfstelle im SessionManager
```
/answer 06c1-14e-0 allow
Control Request:
```yaml
id: 06c1-14f-14e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/connector/claudecode/CCSessionManager.java
        nodes:
        - id: CCSessionManager.getSession
          matches:
          - "throw new IllegalStateException(\"Cannot process command: no active Claude Code session exists\");"
```
/answer 06c1-14f-14e allow
Control Request:
```yaml
id: 06c1-150-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
    ids:
    - PromptInputHandler.buildPrompt
    - PromptInputHandler.followUp
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
    ids:
    - CCConnector.createRequest
    - CCConnector.executeRequest
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
    ids:
    - CCSessionManager.requestSession
    - CCSessionManager.getSession
    - CCSessionManager.getSelectedSessionUuid
    - CCSessionManager.setSelectedSessionUuid
  reason: "Prüfen, wo die Session-Auswahl (globaler Editor-Zustand) in den FollowUp-Pfad hineinläuft"
```
/answer 06c1-150-0 allow
Control Request:
```yaml
id: 06c1-151-150
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
      - id: PromptInputHandler.followUp
        type: MethodDeclaration
        code: |2
            public Prompt followUp(Prompt origin, Selection sel) {
              PromptArguments arg = new PromptArguments();
              arg.setEditor(origin.arg.getEditor());
              arg.absoluteFilePath = origin.arg.absoluteFilePath;
              arg.project = origin.arg.project;
              arg.processorEnabled = origin.arg.processorEnabled;
              detectSelection(sel, arg);
              return new Prompt(arg.inputs, origin.batch, origin.config.copy(), arg);
            }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.createRequest
        type: MethodDeclaration
        code: |2
            @Override
            public CCRequest createRequest(Prompt prompt, IProgress mon) {
              IProgress sub = mon.setTask("Create request", 1);
              Command command = prompt.arg.command;
              String promptText = null;
              if (command == null) {
                List<String> out = new ArrayList<>();
                for (String input : prompt.inputs) {
                  if (input != null && !input.isBlank())
                    includeProcessor.processText(input, List.of(), callbacks, out, includeProcessor);
                }
                promptText = out.stream().map(String::strip).filter(s -> !s.isBlank()).collect(Collectors.joining("\n"));
                if (promptText.isEmpty())
                  throw new IllegalStateException("Prompt text is empty and no command");
              }
              sub.worked();
              String title = promptText == null ? null : promptText.substring(0, Math.min(100, promptText.length())).replace('\n', ' ');
              return new CCRequest(UUID.randomUUID().toString(), title, prompt.config, promptText, prompt.arg);
            }
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
      nodes:
      - id: CCSessionManager.requestSession
        type: MethodDeclaration
        code: "\tpublic synchronized CCSession requestSession(String selectedUuid, CCSessionParameters params) {\n\t\tcleanupInvalidTerminated();\n\n\t\tCCSession session = null;\n\t\tif (CREATE_NEW_MARKER.equals(selectedUuid)) {\n\t\t\tsession = addSession(new CCSession(this, params));\n//\t\t\tLOG.info(\"New session created, hash=\" + params.getHash());\n\t\t} else if (selectedUuid != null) {\n\t\t\tif ((session = findByUuid(selectedUuid)) == null)\n\t\t\t\tthrow new IllegalStateException(\"Selected session not found: \" + selectedUuid);\n\t\t\telse if (!params.getHash().equals(session.getParameters().getHash()))\n\t\t\t\tthrow new IllegalStateException(\n\t\t\t\t\t\t\"Selected session parameters are incompatible with the current configuration\");\n\t\t\tif (session.isExpired())\n\t\t\t\tthrow new IllegalStateException(\"Session has expired\");\n\t\t} else if ((session = findByHash(params.getHash())) != null) {\n//\t\t\tLOG.info(\"Use param hash session, hash=\" + params.getHash());\n\t\t\tif (session.isExpired())\n\t\t\t\tthrow new IllegalStateException(\"Session has expired\");\n\t\t} else {\n\t\t\tsession = addSession(new CCSession(this, params));\n//\t\t\tLOG.info(\"New session created, hash=\" + params.getHash());\n\t\t}\n\n\t\treturn session;\n\t}\n"
      - id: CCSessionManager.getSession
        type: MethodDeclaration
        code: "\tpublic synchronized CCSession getSession(String selectedUuid, CCSessionParameters params) {\n\t\tcleanupInvalidTerminated();\n\t\tCCSession res = null;\n\n\t\tif (!CREATE_NEW_MARKER.equals(selectedUuid))\n\t\t\tres = selectedUuid != null ? findByUuid(selectedUuid) : findByHash(params.getHash());\n\n\t\tif (res == null)\n\t\t\tthrow new IllegalStateException(\"Cannot process command: no active Claude Code session exists\");\n\t\treturn res;\n\t}\n"
      - id: CCSessionManager.getSelectedSessionUuid
        type: MethodDeclaration
        code: "\tpublic String getSelectedSessionUuid() {\n\t\treturn selectedSessionUuid;\n\t}\n"
      - id: CCSessionManager.setSelectedSessionUuid
        type: MethodDeclaration
        code: "\tpublic void setSelectedSessionUuid(String uuid) {\n\t\tthis.selectedSessionUuid = uuid;\n\t}\n"
```
/answer 06c1-151-150 allow
ReasoningToken: 338
Control Request:
```yaml
id: 06c1-152-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: sessionId|SelectedSessionUuid|selectedSessionUuid
    include:
    - '*.java'
    exclude:
    - '**/target/**'
    - '**/bin/**'
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class (PromptArguments|Prompt|CCRequest|AIAnswer|CCResponse|FollowUp)\b|record FollowUp
    include:
    - '*.java'
    exclude:
    - '**/target/**'
    - '**/bin/**'
  reason: Herkunft von sessionId und der global gewählten Session nachverfolgen
```
/answer 06c1-152-0 allow
Control Request:
```yaml
id: 06c1-153-152
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/view/ClaudeCodeSessionView.java
        matches:
        - sessionManager.setSelectedSessionUuid(null);
        - sessionManager.setSelectedSessionUuid(s.getSessionUuid());
      - path: src/xy/ai/workbench/view/approver/AutoApprovePanel.java
        matches:
        - "String line = fit(d.sessionId(), SESSION_WIDTH) + \" | \" + fit(action, ACTION_WIDTH) + \" | \" + fit(tool, TOOL_WIDTH) + \" | \" + pad(Integer.toString(d.answerLength()), CHARS_WIDTH);"
      - path: connector/src/xy/ai/conscious/model/AIAnswer.java
        matches:
        - public String sessionId;
        - this.sessionId = id;
        - "return String.format(\"%sid=%s, %s, %s\", RESULT, sessionId, stats.print(), time.print());"
      - path: connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - "public void forget(String sessionId) {"
        - sessions.forget(sessionId);
        - return Decision.pass(ans.sessionId);
        - "public Optional<FollowUp> onIdle(String sessionId) {"
        - Prompt origin = sessions.lastPrompt(sessionId);
        - "publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp, 0));"
      - path: connector/src/xy/ai/conscious/autoapprove/Decision.java
        matches:
        - "public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,"
        - "public static Decision pass(String sessionId) {"
        - "return new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null, 0);"
      - path: connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
        matches:
        - "* Connectors without own session id leave {@code sessionId == id}; fall back to the prompt's session."
        - if (ans.sessionId != null && !ans.sessionId.equals(ans.id))
        - return ans.sessionId;
        - "return ans.prompt != null ? ans.prompt.sessionId() : ans.id;"
      - path: connector/src/xy/ai/conscious/connector/mcp/MCPClient.java
        matches:
        - private final String sessionId = UUID.randomUUID().toString();
        - ".header(\"X-MCPC-SESSION-ID\", sessionId);"
      - path: connector/src/xy/ai/conscious/connector/claudecode/CCResponse.java
        matches:
        - public String sessionId;
      - path: connector/src/xy/ai/conscious/connector/claudecode/ProtocolParser.java
        matches:
        - String sessionId = node.path("session_id").asText("");
        - "String metadata = String.format(\"%s id=%s, cwd=%s, model=%s\", SYSTEM_INIT, sessionId, cwd, model);"
      - path: connector/src/xy/ai/conscious/connector/claudecode/CCControlClient.java
        matches:
        - "public void checkControlEndpoint(CCResponse resp, String sessionId) {"
        - JsonNode pending = poll(sessionId);
        - "private ArrayNode poll(String sessionId) {"
        - "return post(mapper.createObjectNode(), sessionId);"
        - "private ArrayNode post(ObjectNode body, String sessionId) {"
        - "String url = sessionId == null ? CONTROL_URL : CONTROL_URL + \"?session=\" + URLEncoder.encode(sessionId, StandardCharsets.UTF_8);"
      - path: connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
        matches:
        - "session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);"
        - "session = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);"
        - resp.sessionId = session.getID();
        - answer.sessionId = resp.sessionId;
      - path: connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
        matches:
        - private volatile String selectedSessionUuid;
        - "public String getSelectedSessionUuid() {"
        - return selectedSessionUuid;
        - "public void setSelectedSessionUuid(String uuid) {"
        - this.selectedSessionUuid = uuid;
      - path: connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - "public void onIdle(IExecutor exec, String sessionId) {"
        - "approver.onIdle(sessionId).ifPresent(f -> scheduleFollowUp(exec, f, 1));"
        - return Decision.pass(ans.sessionId);
      - path: connector/src/xy/ai/conscious/connector/harness/Prompt.java
        matches:
        - private String sessionId;
        - "public String sessionId() {"
        - if (sessionId == null)
        - sessionId = computeSessionId();
        - return sessionId;
      - path: connector/src/xy/ai/conscious/commands/ResumeCommand.java
        matches:
        - "public ResumeCommand(String sessionId) {"
      warning: "Limit of 50 matches reached (total 51); narrow pattern, directory or include/exclude filters."
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: connector/src/xy/ai/conscious/model/AIAnswer.java
        matches:
        - "public class AIAnswer {"
      - path: connector/src/xy/ai/conscious/autoapprove/FollowUp.java
        matches:
        - "public record FollowUp(Prompt origin, Selection selection) {"
      - path: connector/src/xy/ai/conscious/connector/claudecode/CCResponse.java
        matches:
        - "public class CCResponse implements IModelResponse {"
      - path: connector/src/xy/ai/conscious/connector/claudecode/CCRequest.java
        matches:
        - "public class CCRequest extends AbstractModelRequest {"
      - path: connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
        matches:
        - "public class PromptArguments {"
      - path: connector/src/xy/ai/conscious/connector/harness/Prompt.java
        matches:
        - "public class Prompt {"
```
/answer 06c1-153-152 allow
Thinking:
Der Fehler stammt aus `CCSessionManager.getSession(...)`, weil Follow-ups den global gesetzten Session-Wert der UI verwenden statt der Session, aus der die Antwort kam. Ich prüfe jetzt, wie sich die Session-ID an den Prompt binden lässt.


Control Request:
```yaml
id: 06c1-154-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCRequest.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
  reason: "Prompt, PromptArguments, CCRequest und SessionTracker komplett lesen, um zu sehen, wo die Session-Bindung ergänzt werden kann"
```
/answer 06c1-154-0 allow
Control Request:
```yaml
id: 06c1-155-154
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
      nodes:
      - id: W9TUuR|WySTfY
        excert: import java.util.List;
      - id: Prompt
        excert: "/**  * Immutable, frozen prompt: semantically separated Inputs / Batch / Config…"
        docstring: "Immutable, frozen prompt: semantically separated Inputs / Batch / Config /"
        children:
        - id: Prompt.uUxVT7|RYVfcb
          excert: public final List<String> inputs;
        - id: Prompt.Prompt
          excert: "public Prompt(List<String> inputs, boolean batch, FrozenConfig config, PromptAr…"
        - id: Prompt.sessionId
          excert: public String sessionId()
          docstring: "Deterministic session id: hash(absoluteFilePath, batch, config.getHash())."
        - id: Prompt.computeSessionId
          excert: private String computeSessionId()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
      nodes:
      - id: TLGErr|Ew8ehZ
        excert: import java.lang.ref.WeakReference;
      - id: PromptArguments
        excert: "public class PromptArguments {"
        children:
        - id: PromptArguments.gK6na9|d9fNBh
          excert: public List<String> inputs = new ArrayList<>();
        - id: PromptArguments.setEditor
          excert: public void setEditor(Object editor)
        - id: PromptArguments.getEditor
          excert: public Object getEditor()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCRequest.java
      nodes:
      - id: 9xbrVR|OkrKah
        excert: import xy.ai.conscious.connector.harness.FrozenConfig;
      - id: CCRequest
        excert: "public class CCRequest extends AbstractModelRequest {"
        children:
        - id: CCRequest.Uvoqj8|bPFO6Z
          excert: public final String id;
        - id: CCRequest.CCRequest
          excert: "public CCRequest(String id, String title, FrozenConfig config, String promptTex…"
        - id: CCRequest.getID
          excert: public String getID()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
      nodes:
      - id: cdBDk8|TOkdih
        excert: import java.util.ArrayDeque;
      - id: SessionTracker
        excert: "final class SessionTracker {"
        children:
        - id: SessionTracker.State
          excert: "private static final class State {"
        - id: SessionTracker.U6jdLZ|v7Yqoi
          excert: static final int MAX_CALLS = 10;
        - id: SessionTracker.keyOf
          excert: static String keyOf(AIAnswer ans)
          docstring: "Connectors without own session id leave {@code sessionId == id}; fall back to t…"
        - id: SessionTracker.record
          excert: SessionSnapshot record(AIAnswer ans)
        - id: SessionTracker.track
          excert: "void track(String key, List<ApprovalItem> items)"
          docstring: "Remembers the tool calls of the items; only the {@link #MAX_CALLS} most recent …"
        - id: SessionTracker.callFor
          excert: "ApprovalItem callFor(String key, ApprovalItem result)"
          docstring: "The remembered tool call the result belongs to, or {@code null}."
        - id: SessionTracker.callIdOf
          excert: static String callIdOf(ApprovalItem result)
          docstring: "Control request ids are {@code <hash>-<number>-<ref>}: a call has ref 0, a resu…"
        - id: SessionTracker.isCall
          excert: private static boolean isCall(ApprovalItem item)
        - id: SessionTracker.lastPrompt
          excert: Prompt lastPrompt(String key)
        - id: SessionTracker.forget
          excert: void forget(String key)
```
/answer 06c1-155-154 allow
ReasoningToken: 299
Control Request:
```yaml
id: 06c1-156-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
  name: convertResponse
  reason: Wo answer.sessionId gesetzt wird
```
/answer 06c1-156-0 allow
Control Request:
```yaml
id: 06c1-157-156
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.convertResponse
        code: |2
            @Override
            public AIAnswer convertResponse(CCResponse resp, IProgress mon) {
              AIAnswer answer = new AIAnswer(resp.id);
              answer.showStats = resp.showStats;
              answer.sessionId = resp.sessionId;
              answer.stats.inputToken = resp.stats.inputToken;
              answer.stats.outputToken = resp.stats.outputToken;
              answer.stats.reasoningToken = resp.stats.reasoningToken;
              answer.stats.cacheRead = resp.stats.cacheRead;
              answer.stats.cacheCreate = resp.stats.cacheCreate;
              answer.stats.totalToken = answer.stats.inputToken + answer.stats.cacheCreate + answer.stats.outputToken;
              answer.time.add(resp.time);
              answer.time.totalMs = resp.time.totalMs;
              answer.answer = resp.resultText;
              return answer;
            }
```
/answer 06c1-157-156 allow
Control Request:
```yaml
id: 06c1-158-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
    ids:
    - PromptArguments.gK6na9|d9fNBh
    - PromptArguments.setEditor
    - PromptArguments.getEditor
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
    ids:
    - Prompt.Prompt
    - Prompt.sessionId
    - Prompt.computeSessionId
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
    ids:
    - SessionTracker.State
    - SessionTracker.keyOf
    - SessionTracker.record
    - SessionTracker.lastPrompt
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
    ids:
    - CCSessionManager.findByUuid
    - CCSessionManager.findByHash
    - CCSessionManager.BqTTt6|umAa9G
  reason: "Details zu Prompt-Session-Hash, SessionTracker.record und Session-Suche lesen"
```
/answer 06c1-158-0 allow
Control Request:
```yaml
id: 06c1-159-158
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
      nodes:
      - id: PromptArguments.gK6na9|d9fNBh
        type: StatementGroup
        code: "\tpublic List<String> inputs = new ArrayList<>();\n\tpublic String absoluteFilePath;\n\tpublic Path project;\n\tpublic Command command;\n\tpublic String yaml;\n\t// Editor active when the prompt was built; used as a hint for tag replacement.\n\tprivate WeakReference<Object> editor;\n\tpublic boolean processorEnabled;\n"
      - id: PromptArguments.setEditor
        type: MethodDeclaration
        code: "\tpublic void setEditor(Object editor) {\n\t\tthis.editor = editor != null ? new WeakReference<>(editor) : null;\n\t}\n"
      - id: PromptArguments.getEditor
        type: MethodDeclaration
        code: "\tpublic Object getEditor() {\n\t\treturn editor != null ? editor.get() : null;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
      nodes:
      - id: Prompt.Prompt
        type: ConstructorDeclaration
        code: "\tpublic Prompt(List<String> inputs, boolean batch, FrozenConfig config, PromptArguments arg) {\n\t\tif (config == null)\n\t\t\tthrow new IllegalArgumentException(\"config must not be null\");\n\t\tif (arg.absoluteFilePath == null || arg.absoluteFilePath.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"absoluteFilePath must not be blank\");\n\t\tif (arg.project == null)\n\t\t\tthrow new IllegalArgumentException(\"projectPath must not be null\");\n\t\tthis.inputs = inputs != null ? List.copyOf(inputs) : List.of();\n\t\tthis.batch = batch;\n\t\tthis.config = config;\n\t\tthis.arg = arg;\n\t}\n"
      - id: Prompt.sessionId
        type: MethodDeclaration
        code: "\tpublic String sessionId() {\n\t\tif (sessionId == null)\n\t\t\tsessionId = computeSessionId();\n\t\treturn sessionId;\n\t}\n"
      - id: Prompt.computeSessionId
        type: MethodDeclaration
        code: "\tprivate String computeSessionId() {\n\t\tString input = arg.absoluteFilePath + \"|\" + batch + \"|\" + config.getHash();\n\t\treturn Hash.hash(input);\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
      nodes:
      - id: SessionTracker.State
        type: ClassOrInterfaceDeclaration
        code: |2
            private static final class State {
              int turn;
              final TokenStats tokens = new TokenStats();
              Instant lastActivity;
              Prompt lastPrompt;
              final Deque<ApprovalItem> calls = new ArrayDeque<>();
            }
      - id: SessionTracker.keyOf
        type: MethodDeclaration
        code: |2
            static String keyOf(AIAnswer ans) {
              if (ans.sessionId != null && !ans.sessionId.equals(ans.id))
                return ans.sessionId;
              return ans.prompt != null ? ans.prompt.sessionId() : ans.id;
            }
      - id: SessionTracker.record
        type: MethodDeclaration
        code: |2
            SessionSnapshot record(AIAnswer ans) {
              String key = keyOf(ans);
              State s = sessions.computeIfAbsent(key, k -> new State());
              synchronized (s) {
                if (ans.showStats)
                  s.turn++;
                s.tokens.add(ans.stats);
                s.lastActivity = Instant.now();
                if (ans.prompt != null)
                  s.lastPrompt = ans.prompt;
                TokenStats copy = new TokenStats();
                copy.add(s.tokens);
                return new SessionSnapshot(key, s.turn, copy, s.lastActivity);
              }
            }
      - id: SessionTracker.lastPrompt
        type: MethodDeclaration
        code: |2
            Prompt lastPrompt(String key) {
              State s = sessions.get(key);
              if (s == null)
                return null;
              synchronized (s) {
                return s.lastPrompt;
              }
            }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCSessionManager.java
      nodes:
      - id: CCSessionManager.findByUuid
        type: MethodDeclaration
        code: "\tprivate CCSession findByUuid(String uuid) {\n\t\tif (uuid != null)\n\t\t\tfor (CCSession s : sessions)\n\t\t\t\tif (uuid.equals(s.getSessionUuid()))\n\t\t\t\t\treturn s;\n\t\treturn null;\n\t}\n"
      - id: CCSessionManager.findByHash
        type: MethodDeclaration
        code: "\tprivate CCSession findByHash(String hash) {\n\t\tfor (CCSession s : sessions)\n\t\t\tif (hash.equals(s.getParameters().getHash()))\n\t\t\t\treturn s;\n\t\treturn null;\n\t}\n"
      - id: CCSessionManager.BqTTt6|umAa9G
        type: StatementGroup
        code: "\tpublic final static String CREATE_NEW_MARKER = \"CREATE_NEW_MARKER\";\n\n\t/** Ordered list of all known sessions. Access must be {@code synchronized}. */\n\tprivate final List<CCSession> sessions = new ArrayList<>();\n\n\t/**\n\t * UUID of the session currently selected in the panel, or {@code null}. Updated\n\t * by the panel view on selection change.\n\t */\n\tprivate volatile String selectedSessionUuid;\n\n\tprivate final List<Consumer<List<CCSession>>> changeListeners = new ArrayList<>();\n"
```
/answer 06c1-159-158 allow
ReasoningToken: 138
Control Request:
```yaml
id: 06c1-15a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
    pattern: getID\(\)|getSessionUuid\(\)|String uuid|getParameters\(\)
    include:
    - CCSession.java
    intent: survey
  reason: Unterschied zwischen CCSession.getID und getSessionUuid klären
```
/answer 06c1-15a-0 allow
Control Request:
```yaml
id: 06c1-15b-15a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
      matches:
      - path: CCSession.java
        matches:
        - private final String uuid;
        - "public String getSessionUuid() {"
        - "public CCSessionParameters getParameters() {"
        - "public String getID() {"
        - return getParameters().getHash();
```
/answer 06c1-15b-15a allow
ReasoningToken: 1455
Control Request:
```yaml
id: 06c1-15c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
    pattern: uuid
    include:
    - CCSession.java
    intent: survey
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
    pattern: public .* (answer|submitEdit)\(
    include:
    - CCControlClient.java
    intent: survey
  reason: "Prüfen, wann CCSession.uuid gesetzt wird und ob answer/submitEdit sessiongebunden sind"
```
/answer 06c1-15c-0 allow
Control Request:
```yaml
id: 06c1-15d-15c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
      matches:
      - path: CCSession.java
        matches:
        - private final String uuid;
        - this.uuid = sessionUuid;
        - cmd.add(uuid);
        - "pb.environment().put(\"MCPC_SESSION_ID\", uuid);"
        - "LOG.info(\"CLI started, uuid=\" + uuid + \", workDir=\" + parameters.cwd);"
        - "LOG.info(\"Terminated, uuid=\" + uuid);"
        - "throw new IllegalStateException(\"STDIN unavailable after start(); process=\" + process + \", uuid=\" + uuid);"
        - "throw new IllegalStateException(\"Failed to probe \" + channel + \" for session \" + uuid, e);"
        - "throw new IllegalStateException(\"Cannot read \" + channel + \": the CLI process is not started\" + \" (uuid=\" + uuid + \", processAlive=\" + isProcessAlive() + \").\" + \" A prompt must be sent (writeLine) before reading.\");"
        - "throw new IllegalStateException(\"Failed to read \" + channel + \" for session \" + uuid, e);"
        - if (uuid == null || uuid.isBlank())
        - filePath = di.resolve(uuid + ".json").toFile();
        - return uuid;
        - if (uuid != null)
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode
      matches:
      - path: CCControlClient.java
        matches:
        - "public ArrayNode answer(String id, AnswerCommand.Action action, String text, String yamlBlock) {"
        - "public boolean submitEdit(String yamlBlock) {"
```
/answer 06c1-15d-15c allow
Thinking:
Die Ursache liegt darin, dass das Follow-up die in der UI gewählte Session über einen prozessweiten Wert abfragt, statt die Session der Antwort zu nutzen. Ich binde den Follow-up deshalb an die Session, aus der die Antwort kam.


ReasoningToken: 2410
Control Request:
```yaml
id: 06c1-15e-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
    id: PromptArguments.gK6na9|d9fNBh
    old_text: public boolean processorEnabled;
    new_text: "public boolean processorEnabled;\n\t// UUID of the connector session this prompt ran in. Set by the connector on first use and inherited by\n\t// follow-ups, so they stay bound to their own session regardless of the (global) UI selection.\n\tpublic volatile String sessionUuid;"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
    id: PromptInputHandler.followUp
    old_text: arg.processorEnabled = origin.arg.processorEnabled;
    new_text: |-
      arg.processorEnabled = origin.arg.processorEnabled;
          arg.sessionUuid = origin.arg.sessionUuid;
  reason: Session-UUID im Prompt mitführen und an Follow-ups vererben
```
/answer 06c1-15e-0 allow
Text:
 Jetzt der Connector. Er nutzt die gebundene Session und löst sie vor dem `answer` oder `submitEdit` auf. Bisher wurde die Antwort abgeschickt, bevor klar war, ob es die Session überhaupt gibt.
Control Request:
```yaml
id: 06c1-15f-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    start_marker: instanceof ExitCommand
    end_marker: UnsupportedOperationException(
    content: |-
      instanceof ExitCommand) {
            session = sessionManager.getSession(selectedUuid, params);
            sub.setTaskName("Terminating CLI process");
            session.terminate();
            return new CCResponse(req.id, "Session closed!");
          }
          if (command instanceof AnswerCommand ac) {
            // resolve the session first: do not answer the control request if its session is gone
            session = sessionManager.getSession(selectedUuid, params);
            ts.control(() -> controlClient.answer(ac.parameter(0), ac.action(), ac.parameter(2), ac.yaml()));
          } else if (command instanceof CallCommand) {
            if (req.arg.yaml == null || req.arg.yaml.isBlank())
              throw new IllegalArgumentException("No preceding ```yaml block found before /call");
            session = sessionManager.getSession(selectedUuid, params);
            if (!ts.control(() -> controlClient.submitEdit(req.arg.yaml)))
              throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
          } else if (command == null) {
            sub.setTaskName("Acquiring session");
            session = sessionManager.requestSession(selectedUuid, params);
          } else
            throw new UnsupportedOperationException(
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    start_marker: Command command = req.arg.command;
    end_marker: Command command = req.arg.command;
    content: |-
      Command command = req.arg.command;
          // A follow-up (e.g. auto-approved answer) stays in the session it originates from; only a fresh,
          // user-triggered prompt falls back to the session currently selected in the UI.
          String selectedUuid = req.arg.sessionUuid != null ? req.arg.sessionUuid : sessionManager.getSelectedSessionUuid();
  reason: Gebundene Session statt globaler UI-Auswahl verwenden und Session vor dem Answer auflösen
```
/answer 06c1-15f-0 allow
Control Request:
```yaml
id: 06c1-160-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: session.setInPrompt(true);
    new_text: |-
      if (req.arg.sessionUuid == null)
              req.arg.sessionUuid = session.getSessionUuid();
            session.setInPrompt(true);
  reason: "Session nach der Auflösung an den Prompt binden, damit Follow-ups sie erben"
```
/answer 06c1-160-0 allow
ReasoningToken: 43
Control Request:
```yaml
id: 06c1-161-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptArguments.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptInputHandler.java
  reason: Kompilierbarkeit prüfen
```
/answer 06c1-161-0 allow
ReasoningToken: 17
Control Request:
```yaml
id: 06c1-162-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/claudecode/CCConnector.java
    ids:
    - CCConnector.executeRequest
  reason: Ergebnis der Edits gegenlesen
```
/answer 06c1-162-0 allow
Control Request:
```yaml
id: 06c1-163-162
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
              // A follow-up (e.g. auto-approved answer) stays in the session it originates from; only a fresh,
              // user-triggered prompt falls back to the session currently selected in the UI.
              String selectedUuid = req.arg.sessionUuid != null ? req.arg.sessionUuid : sessionManager.getSelectedSessionUuid();
              if (command instanceof ResumeCommand rc) {
                sub.setTaskName("Importing session");
                sessionManager.importSession(rc.parameter(0), params);
                return new CCResponse(req.id, "Session created");
              }
              if (command instanceof ExitCommand) {
                session = sessionManager.getSession(selectedUuid, params);
                sub.setTaskName("Terminating CLI process");
                session.terminate();
                return new CCResponse(req.id, "Session closed!");
              }
              if (command instanceof AnswerCommand ac) {
                // resolve the session first: do not answer the control request if its session is gone
                session = sessionManager.getSession(selectedUuid, params);
                ts.control(() -> controlClient.answer(ac.parameter(0), ac.action(), ac.parameter(2), ac.yaml()));
              } else if (command instanceof CallCommand) {
                if (req.arg.yaml == null || req.arg.yaml.isBlank())
                  throw new IllegalArgumentException("No preceding ```yaml block found before /call");
                session = sessionManager.getSession(selectedUuid, params);
                if (!ts.control(() -> controlClient.submitEdit(req.arg.yaml)))
                  throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
              } else if (command == null) {
                sub.setTaskName("Acquiring session");
                session = sessionManager.requestSession(selectedUuid, params);
              } else
                throw new UnsupportedOperationException("Unsupported command: " + command.prefix());
              try {
                if (req.arg.sessionUuid == null)
                  req.arg.sessionUuid = session.getSessionUuid();
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
```
/answer 06c1-163-162 allow
Result Stats: id=3673575c-0943-4875-9ad8-d6b1f50cdca6, total: 54964, in: 28, out: 12298, reason: 0, read: 333778, write: 42638, start: 1794757163592, end: 1794757172293, time: 8655, total: 90629, io: 0, control: 46, sent: 0, recv: 233
Ich habe den Fehler behoben. Das Projekt habe ich nicht gebaut und keinen Test mit zwei Editoren gefahren. `ast_validate` meldet die drei geänderten Dateien als in Ordnung.

## Ursache
Deine Vermutung stimmte nur zum Teil. Der Zugriff auf den Editor war nicht die Lücke. Das `PromptHandler`-Follow-up (`scheduleFollowUp` → `PromptRun`) liest den Editor nicht neu. Die Lücke lag im Connector:

- Die Antwort des Approvers läuft als normaler Prompt durch `CCConnector.executeRequest`. Das gilt für `answer … allow/deny`, `call` und die Idle-Anweisung.
- Dort wurde die Session über `sessionManager.getSelectedSessionUuid()` bestimmt. Das ist die Session, die gerade in der UI ausgewählt ist (`ClaudeCodeSessionView`), und der Wert gilt für den ganzen Prozess.
- Wenn du in einem anderen Editor arbeitest, ändert sich diese Auswahl. Sie kann auch `null` oder `CREATE_NEW_MARKER` sein. Dann findet `CCSessionManager.getSession` keine Session und wirft die Fehlermeldung.
- Hatte der andere Editor eine andere existierende Session gewählt, ging die Approver-Antwort an diese fremde Session. Das war vermutlich der gefährlichere Fall.
- Außerdem wurde `controlClient.answer(...)` schon abgeschickt, bevor die Session aufgelöst war.

## Änderung
- **`PromptArguments`:** Neues Feld `sessionUuid`. Es hält die Session, in der der Prompt lief.
- **`CCConnector.executeRequest`:**
  - Ein Prompt mit gebundener Session verwendet diese Session. Nur ein frischer Prompt, den du selbst auslöst, nimmt weiter die in der UI gewählte.
  - Nach dem Auflösen der Session wird deren UUID im Prompt gespeichert.
  - Bei `answer` und `call` wird die Session jetzt vor `answer` bzw. `submitEdit` aufgelöst. Fehlt sie, wird nichts mehr an einen Control-Request geschickt.
- **`PromptInputHandler.followUp`:** Der Follow-up übernimmt `sessionUuid` vom ursprünglichen Prompt. Jede Kette aus Antwort, Follow-up und nächster Antwort bleibt damit in ihrer eigenen Session. Das gilt auch für mehrere Prompts parallel.

## Offene Punkte
- Wird eine Session wirklich beendet, während ein Follow-up noch aussteht, kommt weiterhin die Fehlermeldung. Das ist dann gewollt.
- `ExitCommand` und `ResumeCommand` laufen unverändert. Ein vom Nutzer getipptes `/exit` bezieht sich weiter auf die in der UI gewählte Session.
- Bitte teste es mit zwei Editoren, in denen der Auto-Approver gleichzeitig läuft. Prüfe dabei auch, ob nach dem Wechsel der Session-Auswahl in einem Editor der andere weiterläuft.
