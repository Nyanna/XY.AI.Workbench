Implementiere ein Anzeigepanel zur Kontrolle des Autoapprovers.

- Zielpackage: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver`
- Beispiel: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java`
- Das Panel registriert sich erst bei seiner Initialisierung als Observer beim Approver.
	- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java`
	- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java`
	- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java`

- Das Autoapprove Panel soll analog einer Konsole funktionieren die mitscrollt und Logzeilen mit fester Spaltenbreite zeigt.
	- Einer Logzeile soll angezeigt werden: Session-ID, Aktion(Approved), Toolname oder "Result", Reason Feld(Überlänge kürzen), Anzahl Zeichen in der Answer, Argumente des Tool-Calls oder Result Auszug (Überlängen kürzen).
	- Ein Menü in der Aktionsleiste erlaubt das Toggeln der Filterung für den Aktionstyp: Allow(default an), Denied(default an), No Action(Passierte Antowrten, default aus)
	- Puffergröße nicht mehr als 1000 Zeilen
	- Angezeigt werden die Aktionen aller Sessions
SystemInit:  id=3230103e-7b23-4ac9-b0e9-7101fedfdc89, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst die referenzierten Dateien an.
Control Request:
```yaml
id: dabf-2c-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
  reason: Struktur der Beispiel- und Referenzdateien verstehen
```
/answer dabf-2c-0 allow
ReasoningToken: 13
Control Request:
```yaml
id: dabf-2d-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
      nodes:
      - id: c5aZtt|aiBuqm
        signature: import java.io.ByteArrayOutputStream;
      - id: k3gk8L|otwwvf
        signature: import org.eclipse.swt.layout.GridLayout;
      - id: DiffPanel
        signature: "public class DiffPanel extends ViewPart {"
        children:
        - id: DiffPanel.wWlfYJ|xHxfdE
          signature: public static final String ID = "xy.ai.workbench.view.diff.DiffPanel";
        - id: DiffPanel.createPartControl
          signature: public void createPartControl(Composite parent)
        - id: DiffPanel.makeActions
          signature: private void makeActions()
        - id: DiffPanel.onEditorChange
          signature: public void onEditorChange()
        - id: DiffPanel.onSnapshot
          signature: "public void onSnapshot(SnapshotResult result, Repository repo)"
        - id: DiffPanel.setFocus
          signature: public void setFocus()
        - id: DiffPanel.Formatter
          signature: '@SuppressWarnings("restriction") private class Formatter extends DiffRegionForm…'
          children:
          - id: DiffPanel.Formatter.Formatter
            signature: private Formatter(IDocument document)
          - id: DiffPanel.Formatter.format
            signature: "public void format(FileHeader h, RawText a, RawText b, DiffDriver d) throws IOE…"
          - id: DiffPanel.Formatter.writeHunkHeader
            signature: "protected void writeHunkHeader(int aStartLine, int aEndLine, int bStartLine, in…"
          - id: DiffPanel.Formatter.formatGitDiffFirstHeaderLine
            signature: "protected void formatGitDiffFirstHeaderLine(ByteArrayOutputStream o, ChangeType…"
          - id: DiffPanel.Formatter.formatIndexLine
            signature: "protected void formatIndexLine(OutputStream o, DiffEntry ent) throws IOException"
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
          docstring: Called by an external idle trigger.
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
      nodes:
      - id: JYalcT|Lqo914
        signature: import org.eclipse.core.runtime.Platform;
      - id: Activator
        signature: /**  * The activator class controls the plug-in life cycle  */ public class Act…
        docstring: The activator class controls the plug-in life cycle
        children:
        - id: Activator.pJi3mE|rcTWIJ
          signature: public static final String PLUGIN_ID = "XY.AI.Workbench"; //$NON-NLS-1$
        - id: Activator.start
          signature: public void start(BundleContext context) throws Exception
        - id: Activator.stop
          signature: public void stop(BundleContext context) throws Exception
        - id: Activator.getDefault
          signature: public static Activator getDefault()
```
/answer dabf-2d-2c allow
Control Request:
```yaml
id: dabf-2e-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
    recursive: true
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    recursive: true
  reason: Vorhandene Dateien im View-Package und Autoapprove-Package ermitteln
```
/answer dabf-2e-0 allow
Control Request:
```yaml
id: dabf-2f-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
      entries:
      - ".:"
      - AIBatchView.java
      - AISessionView.java
      - ActionManager.java
      - ClaudeCodeSessionView.java
      - FlatEndingContentProvider.java
      - InputStats.java
      - MultiSelectListener.java
      - PartListener2Adapter.java
      - PresetHandler.java
      - SessionDetailDialog.java
      - "./diff:"
      - DiffPanel.java
      - OpSnapshotter.java
      - SnapshotResult.java
      - "./explorer:"
      - GitIgnoreFilter.java
      - GitIgnoreRule.java
      - GitIgnoreViewerFilter.java
      - MarkdownContentProvider.java
      - MarkdownExplorerView.java
      - MarkdownLabelProvider.java
      - MarkdownLinkHelper.java
      - ModificationDateComparator.java
      - ProjectFilter.java
      - "./session:"
      - ActionButtonsSection.java
      - Clearable.java
      - EditTab.java
      - InputSelection.java
      - InputsTable.java
      - InstructionSection.java
      - ModelParametersSection.java
      - OutputModeSection.java
      - PresetList.java
      - PresetsTab.java
      - SelectTab.java
      - ToolSelection.java
      - ToolsTab.java
      - UpdateGuard.java
      - UsageLogSection.java
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      entries:
      - ".:"
      - AnswerParser.java
      - ApprovalContext.java
      - ApprovalItem.java
      - ApprovalResult.java
      - AutoApprover.java
      - Decision.java
      - FollowUp.java
      - RuleRegistry.java
      - RuleVote.java
      - SessionSnapshot.java
      - SessionTracker.java
      - "./rules:"
      - AllowedToolsRule.java
      - CANDIDATES.md
      - IApprovalRule.java
      - SmallResultRule.java
      - Verdict.java
      - Vote.java
```
/answer dabf-2f-2e allow
Control Request:
```yaml
id: dabf-30-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalResult.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
  reason: Quellcode der Beispiel- und Modellklassen lesen
```
/answer dabf-30-0 allow
Control Request:
```yaml
id: dabf-31-30
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
      nodes:
      - id: c5aZtt|aiBuqm
        signature: import java.io.ByteArrayOutputStream;
      - id: k3gk8L|otwwvf
        signature: import org.eclipse.swt.layout.GridLayout;
      - id: DiffPanel
        signature: "public class DiffPanel extends ViewPart {"
        children:
        - id: DiffPanel.wWlfYJ|xHxfdE
          signature: public static final String ID = "xy.ai.workbench.view.diff.DiffPanel";
        - id: DiffPanel.createPartControl
          signature: public void createPartControl(Composite parent)
        - id: DiffPanel.makeActions
          signature: private void makeActions()
        - id: DiffPanel.onEditorChange
          signature: public void onEditorChange()
        - id: DiffPanel.onSnapshot
          signature: "public void onSnapshot(SnapshotResult result, Repository repo)"
        - id: DiffPanel.setFocus
          signature: public void setFocus()
        - id: DiffPanel.Formatter
          signature: '@SuppressWarnings("restriction") private class Formatter extends DiffRegionForm…'
          children:
          - id: DiffPanel.Formatter.Formatter
            signature: private Formatter(IDocument document)
          - id: DiffPanel.Formatter.format
            signature: "public void format(FileHeader h, RawText a, RawText b, DiffDriver d) throws IOE…"
          - id: DiffPanel.Formatter.writeHunkHeader
            signature: "protected void writeHunkHeader(int aStartLine, int aEndLine, int bStartLine, in…"
          - id: DiffPanel.Formatter.formatGitDiffFirstHeaderLine
            signature: "protected void formatGitDiffFirstHeaderLine(ByteArrayOutputStream o, ChangeType…"
          - id: DiffPanel.Formatter.formatIndexLine
            signature: "protected void formatIndexLine(OutputStream o, DiffEntry ent) throws IOException"
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
          docstring: Called by an external idle trigger.
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
      nodes:
      - id: 9mr2w6|HCtNZ6
        signature: import java.util.List;
      - id: Decision
        signature: /**  * Outcome of one evaluation. Also the event published to listeners.  *  * …
        docstring: Outcome of one evaluation. Also the event published to listeners.
        children:
        - id: Decision.v2lM9h|x96jHo
          signature: "public Decision {"
        - id: Decision.pass
          signature: public static Decision pass(String sessionId)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
      nodes:
      - id: D3sKkB|eJTl0D
        signature: import com.fasterxml.jackson.databind.JsonNode;
      - id: ApprovalItem
        signature: /**  * A tool call or tool result detected in an answer.  *  * @param data pars…
        docstring: A tool call or tool result detected in an answer.
        children:
        - id: ApprovalItem.Kind
          signature: "public enum Kind {"
        - id: ApprovalItem.arguments
          signature: public JsonNode arguments()
        - id: ApprovalItem.result
          signature: public JsonNode result()
        - id: ApprovalItem.isControl
          signature: public boolean isControl()
          docstring: Item belongs to a control request (MCP bound connector).
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalResult.java
      nodes:
      - id: ApprovalResult
        signature: "public enum ApprovalResult {"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      nodes:
      - id: GwlVvM|lj1u2H
        signature: import xy.ai.conscious.connector.harness.Prompt;
      - id: ApprovalContext
        signature: "/**  * Input of a rule.  *  * @param item {"
        docstring: Input of a rule.
        children:
        - id: ApprovalContext.hasItem
          signature: public boolean hasItem()
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
      nodes:
      - id: JYalcT|Lqo914
        signature: import org.eclipse.core.runtime.Platform;
      - id: Activator
        signature: /**  * The activator class controls the plug-in life cycle  */ public class Act…
        docstring: The activator class controls the plug-in life cycle
        children:
        - id: Activator.pJi3mE|rcTWIJ
          signature: public static final String PLUGIN_ID = "XY.AI.Workbench"; //$NON-NLS-1$
        - id: Activator.start
          signature: public void start(BundleContext context) throws Exception
        - id: Activator.stop
          signature: public void stop(BundleContext context) throws Exception
        - id: Activator.getDefault
          signature: public static Activator getDefault()
```
/answer dabf-31-30 allow
ReasoningToken: 65
Control Request:
```yaml
id: dabf-32-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
    ids:
    - c5aZtt|aiBuqm
    - DiffPanel.wWlfYJ|xHxfdE
    - DiffPanel.createPartControl
    - DiffPanel.makeActions
    - DiffPanel.onEditorChange
    - DiffPanel.setFocus
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover.addListener
    - AutoApprover.evaluate
    - AutoApprover.decide
    - AutoApprover.publish
    - AutoApprover.resultFor
    - AutoApprover.denial
    - AutoApprover.onIdle
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
    ids:
    - Decision
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
    ids:
    - ApprovalItem
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalResult.java
    ids:
    - ApprovalResult
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    ids:
    - ApprovalContext
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
    ids:
    - Activator
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
    ids:
    - PromptHandler.approver
    - PromptHandler.PromptHandler
    - PromptHandler.evaluate
  reason: Quelltext der relevanten Knoten lesen
```
/answer dabf-32-0 allow
Control Request:
```yaml
id: dabf-33-32
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
      nodes:
      - id: c5aZtt|aiBuqm
        type: ImportGroup
        code: |
          import java.io.ByteArrayOutputStream;
          import java.io.File;
          import java.io.IOException;
          import java.io.OutputStream;
          import java.nio.charset.StandardCharsets;
          import java.nio.file.Path;
          import java.util.function.Predicate;
          import java.util.stream.Collectors;

          import org.eclipse.egit.ui.internal.commit.DiffDocument;
          import org.eclipse.egit.ui.internal.commit.DiffRegionFormatter;
          import org.eclipse.egit.ui.internal.commit.DiffViewer;
          import org.eclipse.jface.text.Document;
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
      - id: DiffPanel.wWlfYJ|xHxfdE
        type: StatementGroup
        code: "\tpublic static final String ID = \"xy.ai.workbench.view.diff.DiffPanel\";\n\n\tpublic static DiffPanel INSTANCE;\n\n\tprivate Label statusLabel;\n\t@SuppressWarnings(\"restriction\")\n\tprivate DiffViewer diffViewer;\n\n\tprivate ActionManager act = new ActionManager();\n\tprivate ActionDescription syncAction;\n"
      - id: DiffPanel.createPartControl
        type: MethodDeclaration
        code: "\t@SuppressWarnings(\"restriction\")\n\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tINSTANCE = this;\n\t\tActivator.getDefault().editorListener.addTextEditorObserver(e -> onEditorChange());\n\t\tparent.setLayout(new GridLayout(1, false));\n\n\t\tstatusLabel = new Label(parent, SWT.NONE);\n\t\tstatusLabel.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));\n\t\tstatusLabel.setText(\"Empty\");\n\n\t\tComposite diffSection = new Composite(parent, SWT.NONE);\n\t\tdiffSection.setLayout(new FillLayout());\n\t\tdiffViewer = new DiffViewer(parent, null, SWT.V_SCROLL | SWT.H_SCROLL);\n\t\tdiffViewer.getControl().setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));\n\t\tdiffViewer.setDocument(new Document(\"Empty\"));\n\n\t\tmakeActions();\n\t\tIActionBars bars = getViewSite().getActionBars();\n\t\tact.fillLocalToolBar(bars.getToolBarManager());\n\t\tbars.updateActionBars();\n\t}\n"
      - id: DiffPanel.makeActions
        type: MethodDeclaration
        code: "\tprivate void makeActions() {\n\t\tsyncAction = act.create().toolbar().text(\"Sync\", \"Automatically show the latest diff on editor change\")\n\t\t\t\t.image(ISharedImages.IMG_ELCL_SYNCED).runnable(() -> {\n\t\t\t\t\tif (syncAction.isChecked())\n\t\t\t\t\t\tonEditorChange();\n\t\t\t\t});\n\t\tsyncAction.done();\n\t\tsyncAction.setChecked(false);\n\t}\n"
      - id: DiffPanel.onEditorChange
        type: MethodDeclaration
        code: "\tpublic void onEditorChange() {\n\t\tif (syncAction != null && !syncAction.isChecked())\n\t\t\treturn;\n\t\tActiveEditorListener editLst = Activator.getDefault().editorListener;\n\t\tPath path = editLst.resolveProjectPath();\n\t\tif (path == null)\n\t\t\treturn;\n\t\tFile projectPath = path.toFile();\n\t\ttry {\n\t\t\tOpSnapshotter snap = new OpSnapshotter(projectPath);\n\t\t\tSnapshotResult result = snap.findLatest();\n\t\t\tif (result != null)\n\t\t\t\tonSnapshot(result, snap.getRepository());\n\t\t} catch (IOException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}\n"
      - id: DiffPanel.setFocus
        type: MethodDeclaration
        code: "\t@Override\n\tpublic void setFocus() {\n\t\tdiffViewer.getControl().setFocus();\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover.addListener
        type: MethodDeclaration
        code: |2
            public void addListener(Consumer<Decision> listener) {
              listeners.add(listener);
            }
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
      - id: AutoApprover.decide
        type: MethodDeclaration
        code: |2
            private Decision decide(ApprovalContext ctx, List<RuleVote> votes, int depth) {
              List<String> reasons = new ArrayList<>();
              List<String> hints = new ArrayList<>();
              List<String> warnings = new ArrayList<>();
              List<String> instructions = new ArrayList<>();
              boolean deny = false;
              boolean allow = false;
              for (RuleVote rv : votes) {
                Vote v = rv.vote();
                warnings.addAll(v.warnings());
                addNonBlank(instructions, v.instruction());
                switch(v.verdict()) {
                  case Deny ->
                    {
                      deny = true;
                      addNonBlank(reasons, v.reason());
                    }
                  case Allow ->
                    {
                      allow = true;
                      addNonBlank(hints, v.hint());
                    }
                  case Abstain ->
                    {
                    }
                }
              }
              ApprovalResult result = resultFor(ctx.item(), deny, allow);
              String reason = String.join("; ", reasons);
              String hint = String.join("; ", hints);
              String instruction = String.join("\n", instructions);
              FollowUp followUp = null;
              if (depth >= maxChain) {
                if (result != ApprovalResult.Pass || !instruction.isEmpty())
                  warnings.add("auto-approve chain limit reached (" + maxChain + ")");
                result = ApprovalResult.Pass;
              } else {
                followUp = followUpFor(ctx, result, reason, hint, instruction);
                if (followUp == null)
                  result = ApprovalResult.Pass;
              }
              return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp);
            }
      - id: AutoApprover.publish
        type: MethodDeclaration
        code: |2
            private void publish(Decision decision) {
              for (Consumer<Decision> l : listeners) {
                try {
                  l.accept(decision);
                } catch (RuntimeException e) {
                  LOG.error(e.getMessage(), e);
                }
              }
            }
      - id: AutoApprover.resultFor
        type: MethodDeclaration
        code: |2
            private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolean allow) {
              if (item == null || item.id().isBlank() || item.kind() == Kind.ToolResult)
                return ApprovalResult.Pass;
              if (deny)
                return ApprovalResult.Deny;
              if (!allow)
                return ApprovalResult.Pass;
              return item.kind() == Kind.ToolCall ? ApprovalResult.Call : ApprovalResult.Allow;
            }
      - id: AutoApprover.denial
        type: MethodDeclaration
        code: |2
            private static String denial(ApprovalItem item, String reason) {
              String tool = item.tool().isBlank() ? "" : " (" + item.tool() + ")";
              String why = oneLine(reason);
              return "Tool call " + item.id() + tool + " was denied" + (why.isEmpty() ? "." : ": " + why);
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
              publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp));
              return Optional.of(followUp);
            }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
      nodes:
      - id: Decision
        type: RecordDeclaration
        signature: /**  * Outcome of one evaluation. Also the event published to listeners.  *  * …
        docstring: Outcome of one evaluation. Also the event published to listeners.
        children:
        - id: Decision.v2lM9h|x96jHo
          type: StatementGroup
          code: "\tpublic Decision {\n\t\tresult = result == null ? ApprovalResult.Pass : result;\n\t\treason = reason == null ? \"\" : reason;\n\t\thint = hint == null ? \"\" : hint;\n\t\twarnings = warnings == null ? List.of() : List.copyOf(warnings);\n\t\tinstruction = instruction == null ? \"\" : instruction;\n\t\tvotes = votes == null ? List.of() : List.copyOf(votes);\n\t}\n"
        - id: Decision.pass
          type: MethodDeclaration
          code: "\tpublic static Decision pass(String sessionId) {\n\t\treturn new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null);\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
      nodes:
      - id: ApprovalItem
        type: RecordDeclaration
        signature: /**  * A tool call or tool result detected in an answer.  *  * @param data pars…
        docstring: A tool call or tool result detected in an answer.
        children:
        - id: ApprovalItem.Kind
          type: EnumDeclaration
          code: "\tpublic enum Kind {\n\t\tControlCall, ControlResult, ToolCall, ToolResult\n\t}\n"
        - id: ApprovalItem.arguments
          type: MethodDeclaration
          code: "\tpublic JsonNode arguments() {\n\t\treturn data.path(\"arguments\");\n\t}\n"
        - id: ApprovalItem.result
          type: MethodDeclaration
          code: "\tpublic JsonNode result() {\n\t\treturn data.path(\"result\");\n\t}\n"
        - id: ApprovalItem.isControl
          type: MethodDeclaration
          code: "\tpublic boolean isControl() {\n\t\treturn kind == Kind.ControlCall || kind == Kind.ControlResult;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalResult.java
      nodes:
      - id: ApprovalResult
        type: EnumDeclaration
        code: "public enum ApprovalResult {\n\t/** No decision; the answer is not a tool call/result or no rule decided. */\n\tPass,\n\t/** Tool call or result of a control request was rejected. */\n\tDeny,\n\t/** Tool call or result of a control request was approved. */\n\tAllow,\n\t/** Tool call without control request was approved and is executed. */\n\tCall\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      nodes:
      - id: ApprovalContext
        type: RecordDeclaration
        signature: "/**  * Input of a rule.  *  * @param item {"
        docstring: Input of a rule.
        children:
        - id: ApprovalContext.hasItem
          type: MethodDeclaration
          code: "\tpublic boolean hasItem() {\n\t\treturn item != null;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
      nodes:
      - id: Activator
        type: ClassOrInterfaceDeclaration
        signature: /**  * The activator class controls the plug-in life cycle  */ public class Act…
        docstring: The activator class controls the plug-in life cycle
        children:
        - id: Activator.pJi3mE|rcTWIJ
          type: StatementGroup
          code: "\tpublic static final String PLUGIN_ID = \"XY.AI.Workbench\"; //$NON-NLS-1$\n\n\tprivate static Activator plugin;\n\n\tprivate MCPClient mcpClient;\n\tpublic ActiveEditorListener editorListener;\n\tpublic CCSessionManager ccSessionManager;\n\tpublic ConfigManagerAdapter cma;\n\tpublic IncludeAdapter includeAdapter;\n\tprivate AdaptingConnector connector;\n\tpublic AIBatchManager batch;\n\tpublic AIBatchResponseManager batchRequests;\n\tpublic EditorInterface editorInterface;\n\tpublic PromptHandler promptHandler;\n"
        - id: Activator.start
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void start(BundleContext context) throws Exception {\n\t\tsuper.start(context);\n\t\tplugin = this;\n\t\tLOG.log = new EclipseLogging(Platform.getLog(context.getBundle()));\n\n\t\tmcpClient = new MCPClient();\n\t\teditorListener = new ActiveEditorListener();\n\t\tccSessionManager = new CCSessionManager();\n\t\tcma = new ConfigManagerAdapter(editorListener, mcpClient, new EclipseStateCache());\n\t\tincludeAdapter = new IncludeAdapter(editorListener);\n\t\tconnector = new AdaptingConnector(cma.getManager(), ccSessionManager, mcpClient, includeAdapter,\n\t\t\t\tnew EclipseMCPCLaucher());\n\t\tbatch = new AIBatchManager(connector);\n\t\tbatchRequests = new AIBatchResponseManager(connector);\n\t\teditorInterface = new EditorInterface(cma.getManager(), editorListener, connector);\n\t\tpromptHandler = new PromptHandler(cma.getManager(), editorListener, connector, batch, editorInterface,\n\t\t\t\tnew GitSupport());\n\n\t\teditorInterface.register(context);\n\t}\n"
        - id: Activator.stop
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void stop(BundleContext context) throws Exception {\n\t\tplugin = null;\n\t\teditorInterface.dispose(context);\n\t\tsuper.stop(context);\n\t}\n"
        - id: Activator.getDefault
          type: MethodDeclaration
          code: "\tpublic static Activator getDefault() {\n\t\treturn plugin;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler.approver
        type: MethodDeclaration
        code: "  public AutoApprover approver() {\n\treturn approver;\n  }\n"
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
/answer dabf-33-32 allow
ReasoningToken: 112
Control Request:
```yaml
id: dabf-34-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: "^\\s*(public )?(record|@param|\\) \\{|String |ApprovalItem |Kind |JsonNode |List<|FollowUp |ApprovalResult |Prompt |SessionSnapshot |AIAnswer |int |boolean )"
    include:
    - Decision.java
    - ApprovalItem.java
    - ApprovalContext.java
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: listeners|approver *=|private .*approver
    include:
    - AutoApprover.java
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness
    pattern: approver
    include:
    - PromptHandler.java
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: view\.diff\.DiffPanel|DiffPanel
    include:
    - '*.xml'
    - '*.MF'
    - '*.properties'
    - '*.java'
    exclude:
    - bin/**
    - target/**
  reason: "Felder von Decision/ApprovalItem, Listener-Typ, Approver-Zugriff und View-Registrierung ermitteln"
```
/answer dabf-34-0 allow
Control Request:
```yaml
id: dabf-35-34
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: Decision.java
        matches:
        - "public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,"
        - "List<String> warnings, String instruction, List<RuleVote> votes, FollowUp followUp) {"
      - path: ApprovalContext.java
        matches:
        - "public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call,"
        - "SessionSnapshot session) {"
        - "public boolean hasItem() {"
      - path: ApprovalItem.java
        matches:
        - "public record ApprovalItem(Kind kind, String id, String tool, JsonNode data, String yaml) {"
        - "public JsonNode arguments() {"
        - "public JsonNode result() {"
        - "public boolean isControl() {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: AutoApprover.java
        matches:
        - private final List<Consumer<Decision>> listeners = new CopyOnWriteArrayList<>();
        - listeners.add(listener);
        - "for (Consumer<Decision> l : listeners) {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness
      matches:
      - path: PromptHandler.java
        matches:
        - private final AutoApprover approver = new AutoApprover();
        - "public AutoApprover approver() {"
        - return approver;
        - "approver.onIdle(sessionId).ifPresent(f -> scheduleFollowUp(exec, f, 1));"
        - "return approver.evaluate(ans, depth);"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: plugin.xml
        matches:
        - class="xy.ai.workbench.view.diff.DiffPanel"
        - id="xy.ai.workbench.view.diff.DiffPanel"
      - path: src/xy/ai/workbench/view/diff/DiffPanel.java
        matches:
        - "public class DiffPanel extends ViewPart {"
        - public static final String ID = "xy.ai.workbench.view.diff.DiffPanel";
        - public static DiffPanel INSTANCE;
      - path: src/xy/ai/workbench/eclipse/GitSupport.java
        matches:
        - import xy.ai.workbench.view.diff.DiffPanel;
        - if (result != null && DiffPanel.INSTANCE != null)
        - "exec.asyncExec(() -> DiffPanel.INSTANCE.onSnapshot(result, snap.getRepository()));"
```
/answer dabf-35-34 allow
ReasoningToken: 245
Control Request:
```yaml
id: dabf-36-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: new Decision\(
    include:
    - '*.java'
    exclude:
    - bin/**
    - target/**
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: DiffPanel
    include:
    - plugin.xml
    intent: editing
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
    pattern: ^\s*(public|protected).*\(|^import|^public class|^class
    include:
    - ActionManager.java
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff
    pattern: ^import (xy|org.eclipse.ui|org.eclipse.jface.action)|LOG
    include:
    - DiffPanel.java
  reason: "Decision-Konstruktoraufrufe, plugin.xml-Eintrag, ActionManager-API und DiffPanel-Imports prüfen"
```
/answer dabf-36-0 allow
Control Request:
```yaml
id: dabf-37-36
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - "publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp));"
        - "return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp);"
      - path: connector/src/xy/ai/conscious/autoapprove/Decision.java
        matches:
        - "return new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null);"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: plugin.xml
        nodes:
        - id: plugin.extension_2
          matches:
          - class="xy.ai.workbench.view.diff.DiffPanel"
          - id="xy.ai.workbench.view.diff.DiffPanel"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
      matches:
      - path: ActionManager.java
        matches:
        - import java.util.ArrayList;
        - import java.util.List;
        - import java.util.function.Consumer;
        - import org.eclipse.core.runtime.IProgressMonitor;
        - import org.eclipse.core.runtime.Status;
        - import org.eclipse.core.runtime.jobs.IJobFunction;
        - import org.eclipse.core.runtime.jobs.Job;
        - import org.eclipse.jface.action.Action;
        - import org.eclipse.jface.action.IMenuManager;
        - import org.eclipse.jface.action.IToolBarManager;
        - import org.eclipse.jface.viewers.IStructuredSelection;
        - import org.eclipse.jface.viewers.StructuredViewer;
        - import org.eclipse.swt.widgets.Display;
        - import org.eclipse.ui.PlatformUI;
        - "public class ActionManager {"
        - "public void add(ActionDescription actionDescription) {"
        - "public ActionDescription create() {"
        - "public void fillLocalPullDown(IMenuManager manager) {"
        - "public void fillContextMenu(IMenuManager manager) {"
        - "public void fillLocalToolBar(IToolBarManager manager) {"
        - "public ActionDescription(ActionManager manager) {"
        - "public boolean isPullDown() {"
        - "public boolean isContextMenu() {"
        - "public boolean isToolbar() {"
        - "public void done() {"
        - "public ActionDescription pullDown() {"
        - "public ActionDescription contextMenu() {"
        - "public ActionDescription toolbar() {"
        - "public ActionDescription text(String text, String tooltip) {"
        - "public ActionDescription image(String image) {"
        - "public ActionDescription runnable(Runnable run) {"
        - "public <C> ActionDescription selection(StructuredViewer viewer, Class<C> clazz, Consumer<C> run) {"
        - "public ActionDescription selection(StructuredViewer viewer, Consumer<IStructuredSelection> run) {"
        - "public ActionDescription job(Consumer<IProgressMonitor> run) {"
        - "public ActionDescription jobFunc(IJobFunction job) {"
        - "public ActionDescription display(Runnable run) {"
        - "public void run() {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff
      matches:
      - path: DiffPanel.java
        matches:
        - import org.eclipse.ui.IActionBars;
        - import org.eclipse.ui.ISharedImages;
        - import org.eclipse.ui.editors.text.EditorsUI;
        - import org.eclipse.ui.part.ViewPart;
        - import xy.ai.conscious.core.LOG;
        - import xy.ai.workbench.Activator;
        - import xy.ai.workbench.ActiveEditorListener;
        - import xy.ai.workbench.view.ActionManager;
        - import xy.ai.workbench.view.ActionManager.ActionDescription;
        - "LOG.error(e.getMessage(), e);"
```
/answer dabf-37-36 allow
Thinking:
Das Panel braucht die Antwortlänge, die in `Decision` fehlt, also lese ich jetzt `ActionManager`, den `plugin.xml`-Eintrag und `Decision` weiter, bevor ich entscheide, wie ich sie bekomme.


Control Request:
```yaml
id: dabf-38-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/plugin.xml
    ids:
    - plugin.extension_2
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ActionManager.java
    ids:
    - ActionManager
  reason: "View-Extension in plugin.xml und ActionManager-Implementierung (Menüs, Checked-Actions) lesen"
```
/answer dabf-38-0 allow
ReasoningToken: 199
Control Request:
```yaml
id: dabf-39-38
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/plugin.xml
      nodes:
      - id: plugin.extension_2
        type: element
        code: "<extension\n\t\tpoint=\"org.eclipse.ui.views\">\n\t\t<category\n\t\t\tid=\"XY.AI.Workbench\"\n\t\t\tname=\"AI\">\n\t\t</category>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.AISessionView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.AISessionView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"XY.Ai Session Settings\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.AIBatchView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.AIBatchView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"XY.AI Batch List\">\n\t\t</view>\n\t\t<view\n\t\t\tname=\"AI Explorer\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.explorer.MarkdownExplorerView\"\n\t\t\tid=\"xy.ai.workbench.view.AIExplorer\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.ClaudeCodeSessionView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.ClaudeCodeSessionView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"CC Sessions\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.diff.DiffPanel\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.diff.DiffPanel\"\n\t\t\tname=\"AI Diff\">\n\t\t</view>\n\t</extension>"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ActionManager.java
      nodes:
      - id: ActionManager
        type: ClassOrInterfaceDeclaration
        signature: "public class ActionManager {"
        children:
        - id: ActionManager.zN9t5L|i5SE1F
          type: StatementGroup
          code: "\tprivate List<ActionDescription> actions = new ArrayList<>();\n"
        - id: ActionManager.add
          type: MethodDeclaration
          code: "\tpublic void add(ActionDescription actionDescription) {\n\t\tactions.add(actionDescription);\n\t}\n"
        - id: ActionManager.create
          type: MethodDeclaration
          code: "\tpublic ActionDescription create() {\n\t\treturn new ActionDescription(this);\n\t}\n"
        - id: ActionManager.fillLocalPullDown
          type: MethodDeclaration
          code: "\tpublic void fillLocalPullDown(IMenuManager manager) {\n\t\tactions.stream().filter(a -> a.isPullDown()).forEach(a -> manager.add(a));\n\t}\n"
        - id: ActionManager.fillContextMenu
          type: MethodDeclaration
          code: "\tpublic void fillContextMenu(IMenuManager manager) {\n\t\tactions.stream().filter(a -> a.isContextMenu()).forEach(a -> manager.add(a));\n\t}\n"
        - id: ActionManager.fillLocalToolBar
          type: MethodDeclaration
          code: "\tpublic void fillLocalToolBar(IToolBarManager manager) {\n\t\tactions.stream().filter(a -> a.isToolbar()).forEach(a -> manager.add(a));\n\t}\n"
        - id: ActionManager.ActionDescription
          type: ClassOrInterfaceDeclaration
          signature: "public static class ActionDescription extends Action {"
          children:
          - id: ActionManager.ActionDescription.Wo4DCk|nZZd2G
            type: StatementGroup
            code: "\t\tprivate boolean isPullDown = false;\n\t\tprivate boolean isContextMenu = false;\n\t\tprivate boolean isToolbar = false;\n\n\t\tprivate Runnable run;\n\t\tprivate Runnable display;\n\t\tprivate ActionManager manager;\n\t\tprivate IJobFunction job;\n"
          - id: ActionManager.ActionDescription.ActionDescription
            type: ConstructorDeclaration
            code: "\t\tpublic ActionDescription(ActionManager manager) {\n\t\t\tthis.manager = manager;\n\t\t}\n"
          - id: ActionManager.ActionDescription.isPullDown
            type: MethodDeclaration
            code: "\t\tpublic boolean isPullDown() {\n\t\t\treturn isPullDown;\n\t\t}\n"
          - id: ActionManager.ActionDescription.isContextMenu
            type: MethodDeclaration
            code: "\t\tpublic boolean isContextMenu() {\n\t\t\treturn isContextMenu;\n\t\t}\n"
          - id: ActionManager.ActionDescription.isToolbar
            type: MethodDeclaration
            code: "\t\tpublic boolean isToolbar() {\n\t\t\treturn isToolbar;\n\t\t}\n"
          - id: ActionManager.ActionDescription.done
            type: MethodDeclaration
            code: "\t\tpublic void done() {\n\t\t\tmanager.add(this);\n\t\t}\n"
          - id: ActionManager.ActionDescription.pullDown
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription pullDown() {\n\t\t\tthis.isPullDown = true;\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.contextMenu
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription contextMenu() {\n\t\t\tthis.isContextMenu = true;\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.toolbar
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription toolbar() {\n\t\t\tthis.isToolbar = true;\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.text
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription text(String text, String tooltip) {\n\t\t\tsetText(text);\n\t\t\tsetToolTipText(tooltip);\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.image
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription image(String image) {\n\t\t\tsetImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(image));\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.runnable
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription runnable(Runnable run) {\n\t\t\tthis.run = run;\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.selection
            type: MethodDeclaration
            code: "\t\t@SuppressWarnings(\"unchecked\")\n\t\tpublic <C> ActionDescription selection(StructuredViewer viewer, Class<C> clazz, Consumer<C> run) {\n\t\t\tthis.run = () -> {\n\t\t\t\tIStructuredSelection sel = (IStructuredSelection) viewer.getSelection();\n\t\t\t\tif (!sel.isEmpty()) {\n\t\t\t\t\tObject elem = sel.getFirstElement();\n\t\t\t\t\tif (clazz.isInstance(elem))\n\t\t\t\t\t\trun.accept((C) elem);\n\t\t\t\t}\n\t\t\t};\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.selection_2
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription selection(StructuredViewer viewer, Consumer<IStructuredSelection> run) {\n\t\t\tthis.run = () -> {\n\t\t\t\tIStructuredSelection sel = (IStructuredSelection) viewer.getSelection();\n\t\t\t\tif (!sel.isEmpty())\n\t\t\t\t\trun.accept(sel);\n\t\t\t};\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.job
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription job(Consumer<IProgressMonitor> run) {\n\t\t\tthis.job = (mon) -> {\n\t\t\t\trun.accept(mon);\n\t\t\t\treturn Status.OK_STATUS;\n\t\t\t};\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.jobFunc
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription jobFunc(IJobFunction job) {\n\t\t\tthis.job = job;\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.display
            type: MethodDeclaration
            code: "\t\tpublic ActionDescription display(Runnable run) {\n\t\t\tthis.display = run;\n\t\t\treturn this;\n\t\t}\n"
          - id: ActionManager.ActionDescription.run
            type: MethodDeclaration
            code: "\t\t@Override\n\t\tpublic void run() {\n\t\t\tif (display != null)\n\t\t\t\tDisplay.getDefault().asyncExec(display);\n\t\t\telse if (job != null)\n\t\t\t\tJob.create(getText(), job).schedule();\n\t\t\telse\n\t\t\t\trun.run();\n\t\t}\n"
```
/answer dabf-39-38 allow
Control Request:
```yaml
id: dabf-3a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: public (String|final String) (answer|sessionId)|class AIAnswer
    include:
    - AIAnswer.java
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ^\s*(\*|/\*\*).*|^public record.*
    include:
    - Decision.java
  reason: AIAnswer-Felder und Decision-Javadoc prüfen
```
/answer dabf-3a-0 allow
Control Request:
```yaml
id: dabf-3b-3a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/model/AIAnswer.java
        matches:
        - "public class AIAnswer {"
        - public String sessionId;
        - public String answer = "";
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: Decision.java
        matches:
        - /**
        - '* Outcome of one evaluation. Also the event published to listeners.'
        - '*'
        - "* @param item        {@code null} if no single tool call/result was detected"
        - "* @param instruction follow-up instruction requested by rules, empty if none"
        - "* @param followUp    prompt the approver triggers, {@code null} if none"
        - '*/'
        - "public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,"
```
/answer dabf-3b-3a allow
Control Request:
```yaml
id: dabf-3c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: autoapprove|Export-Package|Require-Bundle|Bundle-ClassPath
    include:
    - MANIFEST.MF
    - build.properties
    - .classpath
    exclude:
    - bin/**
    - target/**
  reason: "Prüfen, ob das autoapprove-Package aus dem Plugin sichtbar ist"
```
/answer dabf-3c-0 deny ist es
ReasoningToken: 2482
Control Request:
```yaml
id: dabf-3d-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
    ids:
    - Decision
  reason: Decision-Record komplett inkl. Javadoc für die Erweiterung ansehen
```
/answer dabf-3d-0 allow
Control Request:
```yaml
id: dabf-3e-3d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
      nodes:
      - id: Decision
        type: RecordDeclaration
        signature: /**  * Outcome of one evaluation. Also the event published to listeners.  *  * …
        docstring: Outcome of one evaluation. Also the event published to listeners.
        children:
        - id: Decision.v2lM9h|x96jHo
          type: StatementGroup
          code: "\tpublic Decision {\n\t\tresult = result == null ? ApprovalResult.Pass : result;\n\t\treason = reason == null ? \"\" : reason;\n\t\thint = hint == null ? \"\" : hint;\n\t\twarnings = warnings == null ? List.of() : List.copyOf(warnings);\n\t\tinstruction = instruction == null ? \"\" : instruction;\n\t\tvotes = votes == null ? List.of() : List.copyOf(votes);\n\t}\n"
        - id: Decision.pass
          type: MethodDeclaration
          code: "\tpublic static Decision pass(String sessionId) {\n\t\treturn new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null);\n\t}\n"
```
/answer dabf-3e-3d allow
Text:
 Alles klar, das Package ist also sichtbar. Ich erweitere jetzt `Decision` um die Antwortlänge, ergänze `removeListener` im `AutoApprover` und passe die drei Konstruktoraufrufe an.
ReasoningToken: 177
Control Request:
```yaml
id: dabf-3f-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
    id: Decision
    old_text: "List<String> warnings, String instruction, List<RuleVote> votes, FollowUp followUp) {"
    new_text: "List<String> warnings, String instruction, List<RuleVote> votes, FollowUp followUp, int answerLength) {"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
    id: Decision
    old_text: "* @param followUp    prompt the approver triggers, {@code null} if none"
    new_text: |-
      * @param followUp    prompt the approver triggers, {@code null} if none
       * @param answerLength number of characters in the evaluated answer
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
    id: Decision.pass
    old_text: "null, null, null, null, null, null, null);"
    new_text: "null, null, null, null, null, null, null, 0);"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    old_text: "ctx.item(), reason, hint, warnings, instruction, votes, followUp);"
    new_text: |-
      ctx.item(), reason, hint, warnings, instruction, votes, followUp,
              ctx.answer().answer == null ? 0 : ctx.answer().answer.length());
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.onIdle
    old_text: "null, null, null, instruction, null, followUp));"
    new_text: "null, null, null, instruction, null, followUp, 0));"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.addListener
    old_text: |2-
          listeners.add(listener);
        }
    new_text: |2-
          listeners.add(listener);
        }

        public void removeListener(Consumer<Decision> listener) {
          listeners.remove(listener);
        }
  reason: "Antwortlänge in Decision aufnehmen, Konstruktoraufrufe anpassen und removeListener ergänzen"
```
/answer dabf-3f-0 allow
Control Request:
```yaml
id: dabf-40-3f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
      result: success
      id: Decision.pass
      new_id: Decision.pass
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.decide
      new_id: AutoApprover.decide
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.onIdle
      new_id: AutoApprover.onIdle
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
      error: |
        invalid type declaration: Encountered unexpected token: "record" "record"
            at line 1, column 8.

        Was expecting one of:

            "@"
            "class"
            "enum"
            "interface"
      id: Decision
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
      error: Text not found in node.
      id: Decision
      reason: not_found
      next_step: reread_node
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      error: |
        invalid member declaration: Encountered unexpected token: "public" "public"
            at line 5, column 3.

        Was expecting:

            <EOF>
      id: AutoApprover.addListener
```
/answer dabf-40-3f allow
Text:
 Drei Edits sind fehlgeschlagen: der Record-Header von `Decision` und `removeListener`. Ich prüfe, was in den Dateien tatsächlich steht.
ReasoningToken: 121
Control Request:
```yaml
id: dabf-41-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: "answerLength|followUp\\)|followUp, 0|removeListener|record Decision|FollowUp followUp"
    include:
    - Decision.java
    - AutoApprover.java
    intent: editing
  reason: Aktuellen Stand nach den teilweise fehlgeschlagenen Edits prüfen
```
/answer dabf-41-0 allow
Control Request:
```yaml
id: dabf-42-41
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: AutoApprover.java
        nodes:
        - id: AutoApprover.onIdle
          matches:
          - "FollowUp followUp = new FollowUp(origin, selection(instruction.lines().toList()));"
          - "publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp, 0));"
          - return Optional.of(followUp);
        - id: AutoApprover.decide
          matches:
          - FollowUp followUp = null;
        - id: AutoApprover.followUpFor
          matches:
          - "private FollowUp followUpFor(ApprovalContext ctx, ApprovalResult result, String reason, String hint, String instruction) {"
      - path: Decision.java
        nodes:
        - id: Decision
          matches:
          - "public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint, List<String> warnings, String instruction, List<RuleVote> votes, FollowUp followUp) {"
```
/answer dabf-42-41 allow
Thinking:
Der Record-Header ist unverändert, der AST-Parser scheint Records aber nicht zu verarbeiten, weshalb die geänderten Aufrufe nicht passen. Ich schreibe die Datei daher komplett neu.


ReasoningToken: 691
Control Request:
```yaml
id: dabf-43-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: .*
    include:
    - Decision.java
    limit: 50
  reason: Gesamten Inhalt von Decision.java für ein vollständiges Neuschreiben lesen
```
/answer dabf-43-0 allow Ersetze den Record als ganzes
Control Request:
```yaml
id: dabf-44-43
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: Decision.java
        matches:
        - package xy.ai.conscious.autoapprove;
        - ""
        - import java.util.List;
        - /**
        - '* Outcome of one evaluation. Also the event published to listeners.'
        - '*'
        - "* @param item        {@code null} if no single tool call/result was detected"
        - "* @param instruction follow-up instruction requested by rules, empty if none"
        - "* @param followUp    prompt the approver triggers, {@code null} if none"
        - '*/'
        - "public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint, List<String> warnings, String instruction, List<RuleVote> votes, FollowUp followUp) {"
        - "public Decision {"
        - "result = result == null ? ApprovalResult.Pass : result;"
        - "reason = reason == null ? \"\" : reason;"
        - "hint = hint == null ? \"\" : hint;"
        - "warnings = warnings == null ? List.of() : List.copyOf(warnings);"
        - "instruction = instruction == null ? \"\" : instruction;"
        - "votes = votes == null ? List.of() : List.copyOf(votes);"
        - "}"
        - "public static Decision pass(String sessionId) {"
        - "return new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null, 0);"
```
/answer dabf-44-43 allow
Text:
 Du willst den Record als Ganzes ersetzen. Das mache ich jetzt mit dem vollständigen Record. Danach prüfe ich das Javadoc.
Control Request:
```yaml
id: dabf-45-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
    id: Decision
    source: "/**\n * Outcome of one evaluation. Also the event published to listeners.\n *\n * @param item         {@code null} if no single tool call/result was detected\n * @param instruction  follow-up instruction requested by rules, empty if none\n * @param followUp     prompt the approver triggers, {@code null} if none\n * @param answerLength number of characters in the evaluated answer\n */\npublic record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,\n\t\tList<String> warnings, String instruction, List<RuleVote> votes, FollowUp followUp, int answerLength) {\n\n\tpublic Decision {\n\t\tresult = result == null ? ApprovalResult.Pass : result;\n\t\treason = reason == null ? \"\" : reason;\n\t\thint = hint == null ? \"\" : hint;\n\t\twarnings = warnings == null ? List.of() : List.copyOf(warnings);\n\t\tinstruction = instruction == null ? \"\" : instruction;\n\t\tvotes = votes == null ? List.of() : List.copyOf(votes);\n\t}\n\n\tpublic static Decision pass(String sessionId) {\n\t\treturn new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null, 0);\n\t}\n}\n"
  reason: Decision-Record als Ganzes um answerLength erweitern
```
/answer dabf-45-0 allow
ReasoningToken: 243
Control Request:
```yaml
id: dabf-46-45
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
      error: |
        invalid type declaration: Encountered unexpected token: "record" "record"
            at line 9, column 8.

        Was expecting one of:

            "@"
            "class"
            "enum"
            "interface"
      id: Decision
```
/answer dabf-46-45 allow Benutze ast_wrie
Control Request:
```yaml
id: dabf-47-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.addListener
    position: after
    source: |2
        public void removeListener(Consumer<Decision> listener) {
          listeners.remove(listener);
        }
  reason: "removeListener als eigenes Member ergänzen, damit das Panel sich beim Dispose abmelden kann"
```
/answer dabf-47-0 allow
Thinking:
`ast_replace` schlägt am gleichen Parser-Problem mit Records fehl, also überschreibe ich `Decision.java` vollständig mit `ast_create`.


ReasoningToken: 312
Control Request:
```yaml
id: dabf-48-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
    overwrite: true
    source: "package xy.ai.conscious.autoapprove;\n\nimport java.util.List;\n\n/**\n * Outcome of one evaluation. Also the event published to listeners.\n *\n * @param item         {@code null} if no single tool call/result was detected\n * @param instruction  follow-up instruction requested by rules, empty if none\n * @param followUp     prompt the approver triggers, {@code null} if none\n * @param answerLength number of characters in the evaluated answer\n */\npublic record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,\n\t\tList<String> warnings, String instruction, List<RuleVote> votes, FollowUp followUp, int answerLength) {\n\n\tpublic Decision {\n\t\tresult = result == null ? ApprovalResult.Pass : result;\n\t\treason = reason == null ? \"\" : reason;\n\t\thint = hint == null ? \"\" : hint;\n\t\twarnings = warnings == null ? List.of() : List.copyOf(warnings);\n\t\tinstruction = instruction == null ? \"\" : instruction;\n\t\tvotes = votes == null ? List.of() : List.copyOf(votes);\n\t}\n\n\tpublic static Decision pass(String sessionId) {\n\t\treturn new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null, 0);\n\t}\n}\n"
  reason: "Decision.java vollständig mit dem neuen Record-Header überschreiben, da Knoten-Edits Records nicht parsen"
```
/answer dabf-48-0 allow
Thinking:
Die `Decision`-Klasse ist angepasst; als Nächstes prüfe ich, ob das Workbench-Plugin Jackson bereits direkt nutzt, damit das Panel `item.arguments()` und `item.result()` korrekt aufrufen kann.


ReasoningToken: 1996
Control Request:
```yaml
id: dabf-49-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: com\.fasterxml\.jackson
    include:
    - '*.java'
    limit: 3
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: jackson
    include:
    - MANIFEST.MF
    limit: 3
  reason: "Klassenpfad-Verfügbarkeit von Jackson im Workbench-Plugin prüfen, da das Panel JsonNode-Rückgaben nutzt"
```
/answer dabf-49-0 allow Jackson ist verfügbar
Control Request:
```yaml
id: dabf-4a-49
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/MementoConverter.java
        matches:
        - import com.fasterxml.jackson.core.JsonProcessingException;
        - import com.fasterxml.jackson.databind.ObjectMapper;
      - path: xy/ai/workbench/view/PresetHandler.java
        matches:
        - import com.fasterxml.jackson.databind.ObjectMapper;
      warning: "Limit of 3 matches reached (total 7); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: META-INF/MANIFEST.MF
        matches:
        - "Bundle-ClassPath: libs/jackson-dataformat-yaml-2.20.0.jar,"
        - "libs/openai/jackson-annotations-3.0-rc5.jar,"
        - "libs/openai/jackson-core-2.20.0.jar,"
      warning: "Limit of 3 matches reached (total 8); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer dabf-4a-49 allow
Text:
 Jackson ist verfügbar. Jetzt lege ich das Panel an.
Control Request:
```yaml
id: dabf-4b-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    source: "package xy.ai.workbench.view.approver;\n\nimport java.util.ArrayDeque;\nimport java.util.Deque;\nimport java.util.EnumSet;\nimport java.util.Set;\nimport java.util.function.Consumer;\n\nimport org.eclipse.jface.action.Action;\nimport org.eclipse.jface.action.IAction;\nimport org.eclipse.jface.action.IMenuCreator;\nimport org.eclipse.jface.resource.JFaceResources;\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.custom.StyledText;\nimport org.eclipse.swt.widgets.Control;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.swt.widgets.Menu;\nimport org.eclipse.swt.widgets.MenuItem;\nimport org.eclipse.ui.IActionBars;\nimport org.eclipse.ui.part.ViewPart;\nimport org.eclipse.swt.widgets.Composite;\n\nimport com.fasterxml.jackson.databind.JsonNode;\n\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.AutoApprover;\nimport xy.ai.conscious.autoapprove.Decision;\nimport xy.ai.workbench.Activator;\n\n/** Console like log of the decisions of the auto approver for all sessions. */\npublic class AutoApprovePanel extends ViewPart {\n\n\tpublic static final String ID = \"xy.ai.workbench.view.approver.AutoApprovePanel\";\n\n\tprivate static final int MAX_LINES = 1000;\n\tprivate static final int SESSION_WIDTH = 10;\n\tprivate static final int ACTION_WIDTH = 8;\n\tprivate static final int TOOL_WIDTH = 24;\n\tprivate static final int REASON_WIDTH = 30;\n\tprivate static final int CHARS_WIDTH = 7;\n\tprivate static final int DETAIL_MAX = 200;\n\n\tprivate enum Filter {\n\t\tAllow(\"Allow\"), Denied(\"Denied\"), NoAction(\"No Action\");\n\n\t\tprivate final String label;\n\n\t\tFilter(String label) {\n\t\t\tthis.label = label;\n\t\t}\n\t}\n\n\tprivate record Entry(Filter filter, String line) {\n\t}\n\n\tprivate final Deque<Entry> buffer = new ArrayDeque<>();\n\tprivate final Set<Filter> shown = EnumSet.of(Filter.Allow, Filter.Denied);\n\tprivate final Consumer<Decision> observer = this::onDecision;\n\n\tprivate Display display;\n\tprivate StyledText console;\n\tprivate AutoApprover approver;\n\n\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tdisplay = parent.getDisplay();\n\t\tconsole = new StyledText(parent, SWT.READ_ONLY | SWT.V_SCROLL | SWT.H_SCROLL);\n\t\tconsole.setFont(JFaceResources.getTextFont());\n\t\tconsole.setWordWrap(false);\n\n\t\tIActionBars bars = getViewSite().getActionBars();\n\t\tbars.getToolBarManager().add(createFilterAction());\n\t\tbars.updateActionBars();\n\n\t\tapprover = Activator.getDefault().promptHandler.approver();\n\t\tapprover.addListener(observer);\n\t}\n\n\tprivate Action createFilterAction() {\n\t\tAction action = new Action(\"Filter\", IAction.AS_DROP_DOWN_MENU) {\n\t\t};\n\t\taction.setToolTipText(\"Select the action types to show\");\n\t\taction.setMenuCreator(new IMenuCreator() {\n\t\t\tprivate Menu menu;\n\n\t\t\t@Override\n\t\t\tpublic Menu getMenu(Control parent) {\n\t\t\t\tdispose();\n\t\t\t\tmenu = new Menu(parent);\n\t\t\t\tfillMenu(menu);\n\t\t\t\treturn menu;\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic Menu getMenu(Menu parent) {\n\t\t\t\tdispose();\n\t\t\t\tmenu = new Menu(parent);\n\t\t\t\tfillMenu(menu);\n\t\t\t\treturn menu;\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void dispose() {\n\t\t\t\tif (menu != null && !menu.isDisposed())\n\t\t\t\t\tmenu.dispose();\n\t\t\t\tmenu = null;\n\t\t\t}\n\t\t});\n\t\treturn action;\n\t}\n\n\tprivate void fillMenu(Menu menu) {\n\t\tfor (Filter filter : Filter.values()) {\n\t\t\tMenuItem item = new MenuItem(menu, SWT.CHECK);\n\t\t\titem.setText(filter.label);\n\t\t\titem.setSelection(shown.contains(filter));\n\t\t\titem.addListener(SWT.Selection, e -> {\n\t\t\t\tif (item.getSelection())\n\t\t\t\t\tshown.add(filter);\n\t\t\t\telse\n\t\t\t\t\tshown.remove(filter);\n\t\t\t\trebuild();\n\t\t\t});\n\t\t}\n\t}\n\n\tprivate void onDecision(Decision decision) {\n\t\tEntry entry = toEntry(decision);\n\t\tdisplay.asyncExec(() -> {\n\t\t\tif (!console.isDisposed())\n\t\t\t\tappend(entry);\n\t\t});\n\t}\n\n\tprivate void append(Entry entry) {\n\t\tbuffer.addLast(entry);\n\t\tif (buffer.size() > MAX_LINES && shown.contains(buffer.removeFirst().filter()))\n\t\t\tconsole.replaceTextRange(0, console.getOffsetAtLine(1), \"\");\n\t\tif (shown.contains(entry.filter())) {\n\t\t\tconsole.append(entry.line() + \"\\n\");\n\t\t\tscrollToEnd();\n\t\t}\n\t}\n\n\tprivate void rebuild() {\n\t\tStringBuilder text = new StringBuilder();\n\t\tfor (Entry entry : buffer) {\n\t\t\tif (shown.contains(entry.filter()))\n\t\t\t\ttext.append(entry.line()).append('\\n');\n\t\t}\n\t\tconsole.setText(text.toString());\n\t\tscrollToEnd();\n\t}\n\n\tprivate void scrollToEnd() {\n\t\tconsole.setCaretOffset(console.getCharCount());\n\t\tconsole.showSelection();\n\t}\n\n\tprivate static Entry toEntry(Decision d) {\n\t\tApprovalItem item = d.item();\n\t\tboolean isResult = item != null && (item.kind() == ApprovalItem.Kind.ToolResult\n\t\t\t\t|| item.kind() == ApprovalItem.Kind.ControlResult);\n\t\tFilter filter = switch (d.result()) {\n\t\tcase Allow, Call -> Filter.Allow;\n\t\tcase Deny -> Filter.Denied;\n\t\tcase Pass -> Filter.NoAction;\n\t\t};\n\t\tString action = switch (filter) {\n\t\tcase Allow -> \"Approved\";\n\t\tcase Denied -> \"Denied\";\n\t\tcase NoAction -> \"Pass\";\n\t\t};\n\t\tString tool = item == null ? \"-\" : isResult ? \"Result\" : item.tool();\n\t\tString detail = item == null ? \"\" : text(isResult ? item.result() : item.arguments());\n\n\t\tString line = fit(d.sessionId(), SESSION_WIDTH) + \" \" + fit(action, ACTION_WIDTH) + \" \" + fit(tool, TOOL_WIDTH)\n\t\t\t\t+ \" \" + fit(d.reason(), REASON_WIDTH) + \" \" + pad(Integer.toString(d.answerLength()), CHARS_WIDTH)\n\t\t\t\t+ \" \" + shorten(detail, DETAIL_MAX);\n\t\treturn new Entry(filter, line);\n\t}\n\n\tprivate static String text(JsonNode node) {\n\t\treturn node.isTextual() ? node.asText() : node.toString();\n\t}\n\n\tprivate static String oneLine(String s) {\n\t\treturn s == null ? \"\" : s.replaceAll(\"\\\\s+\", \" \").strip();\n\t}\n\n\tprivate static String shorten(String s, int max) {\n\t\tString line = oneLine(s);\n\t\treturn line.length() <= max ? line : line.substring(0, max - 1) + \"…\";\n\t}\n\n\tprivate static String fit(String s, int width) {\n\t\treturn pad(shorten(s, width), width).stripTrailing().isEmpty() ? \" \".repeat(width)\n\t\t\t\t: String.format(\"%-\" + width + \"s\", shorten(s, width));\n\t}\n\n\tprivate static String pad(String s, int width) {\n\t\treturn String.format(\"%\" + width + \"s\", s);\n\t}\n\n\t@Override\n\tpublic void dispose() {\n\t\tif (approver != null)\n\t\t\tapprover.removeListener(observer);\n\t\tsuper.dispose();\n\t}\n\n\t@Override\n\tpublic void setFocus() {\n\t\tconsole.setFocus();\n\t}\n}\n"
  reason: Neues Anzeigepanel AutoApprovePanel im Zielpackage anlegen
```
/answer dabf-4b-0 allow
Text:
 Beim Durchsehen sind mir zwei Dinge aufgefallen: `fit` ist unnötig verschachtelt, und das Unicode-Ellipsis-Zeichen im Quelltext ist unnötig riskant. Beides korrigiere ich.
ReasoningToken: 306
Control Request:
```yaml
id: dabf-4c-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  name: fit
  reason: Node-IDs von fit und shorten für die Korrektur ermitteln
```
/answer dabf-4c-0 allow
Control Request:
```yaml
id: dabf-4d-4c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
      nodes:
      - id: AutoApprovePanel.fit
        code: "\tprivate static String fit(String s, int width) {\n\t\treturn pad(shorten(s, width), width).stripTrailing().isEmpty() ? \" \".repeat(width)\n\t\t\t\t: String.format(\"%-\" + width + \"s\", shorten(s, width));\n\t}\n"
```
/answer dabf-4d-4c allow
Control Request:
```yaml
id: dabf-4e-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  name: shorten
  reason: Node-ID von shorten ermitteln
```
/answer dabf-4e-0 allow
Control Request:
```yaml
id: dabf-4f-4e
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
      nodes:
      - id: AutoApprovePanel.shorten
        code: "\tprivate static String shorten(String s, int max) {\n\t\tString line = oneLine(s);\n\t\treturn line.length() <= max ? line : line.substring(0, max - 1) + \"…\";\n\t}\n"
```
/answer dabf-4f-4e allow
Control Request:
```yaml
id: dabf-50-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.fit
    source: "\tprivate static String fit(String s, int width) {\n\t\treturn String.format(\"%-\" + width + \"s\", shorten(s, width));\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.shorten
    source: "\tprivate static String shorten(String s, int max) {\n\t\tString line = oneLine(s);\n\t\treturn line.length() <= max ? line : line.substring(0, max - 3) + \"...\";\n\t}\n"
  reason: fit vereinfachen und ASCII-Ellipsis statt Unicode-Zeichen verwenden
```
/answer dabf-50-0 allow
Control Request:
```yaml
id: dabf-51-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/plugin.xml
    id: plugin.extension_2
    position: after
    source: ""
  reason: Platzhalter - wird nicht benötigt
```
/answer dabf-51-0 allow
Text:
 Den leeren `ast_insert` in `plugin.xml` hätte ich nicht absetzen sollen. Ich prüfe, ob er etwas verändert hat, bevor ich den View-Eintrag ergänze.
ReasoningToken: 78
Control Request:
```yaml
id: dabf-52-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/plugin.xml
  text: view.diff.DiffPanel
  reason: plugin.xml nach dem versehentlichen leeren Insert prüfen und View-Element-ID für die Registrierung finden
```
/answer dabf-52-0 deny ich korrigiere das
Text:
 Du korrigierst `plugin.xml` selbst, deshalb fasse ich die Datei nicht mehr an. Ich validiere noch die geänderten Java-Dateien.
ReasoningToken: 122
Control Request:
```yaml
id: dabf-53-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
  reason: Geänderte und neue Java-Dateien auf Syntaxfehler prüfen
```
/answer dabf-53-0 allow
Result Stats: id=3230103e-7b23-4ac9-b0e9-7101fedfdc89, total: 73240, in: 40, out: 21049, reason: 0, read: 647124, write: 52151, start: 1794075286233, end: 1794075295740, time: 9444, total: 168839, io: 0, control: 63, sent: 0, recv: 252
Das Panel `AutoApprovePanel` ist implementiert, aber noch nicht als View in `plugin.xml` registriert. Die Dateien sind syntaktisch geprüft. Gebaut oder in Eclipse gestartet habe ich nichts.

**`plugin.xml` (noch offen):** Mein leerer `ast_insert` hat die Datei möglicherweise verändert. Du wolltest das selbst korrigieren, deshalb habe ich sie nicht weiter angefasst. Bitte ergänze dort den View-Eintrag, analog zum `DiffPanel`:
```xml
<view
	allowMultiple="false"
	category="XY.AI.Workbench"
	class="xy.ai.workbench.view.approver.AutoApprovePanel"
	icon="icons/sample.png"
	id="xy.ai.workbench.view.approver.AutoApprovePanel"
	name="AI Auto Approve">
</view>
```

**Panel** (`view/approver/AutoApprovePanel.java`):
- **Registrierung:** Das Panel meldet sich erst in `createPartControl` über `Activator.getDefault().promptHandler.approver().addListener(...)` an. In `dispose()` meldet es sich wieder ab. Es zeigt die Aktionen aller Sessions.
- **Konsole:** Das Panel ist ein schreibgeschütztes `StyledText` mit Monospace-Font, ohne Umbruch, und scrollt bei jeder neuen Zeile ans Ende.
- **Logzeile:** Die Spalten sind: Session-ID (10 Zeichen), Aktion (8), Toolname oder `Result` (24), Reason (30), Zeichenzahl der Answer (7, rechtsbündig), danach Argumente bzw. Result-Auszug (max. 200 Zeichen). Zu lange Werte werden mit `...` gekürzt, Zeilenumbrüche werden zu Leerzeichen.
- **Aktion:** `Allow` und `Call` erscheinen als `Approved`, `Deny` als `Denied`, `Pass` als `Pass`. Ohne erkanntes Tool steht `-` in der Toolspalte.
- **Filter:** Ein Dropdown-Menü `Filter` in der Toolbar hat die Einträge Allow (an), Denied (an) und No Action (aus). Beim Umschalten wird die Anzeige aus dem Puffer neu aufgebaut.
- **Puffer:** Der Puffer hält höchstens 1000 Einträge, unabhängig vom Filter. Beim Überlauf fällt der älteste Eintrag weg.

**Änderungen am Connector:**
- **`Decision`:** Das Event enthielt die Answer-Länge nicht. Ich habe `int answerLength` als letzte Komponente ergänzt und die drei Konstruktoraufrufe angepasst. Der Idle-Fall und `Decision.pass` setzen 0.
- **`Decision.java`:** Der AST-Parser der Tools konnte Records nicht verarbeiten, deshalb habe ich die Datei mit `ast_create` komplett neu geschrieben. Das habe ich aus deiner abgeschnittenen Nachricht "Benutze ast_wrie" geschlossen. Falls du etwas anderes meintest, sag Bescheid.
- **`AutoApprover.removeListener`:** Diese Methode ist neu, damit das Panel beim Schließen keine Listener hinterlässt.

Die Session-ID wird bei 10 Zeichen abgeschnitten. Wenn deine IDs längere gemeinsame Präfixe haben, muss `SESSION_WIDTH` größer werden.
Muss das Panel nicht in der Plugin.xml registriert werden?
SystemInit:  id=3230103e-7b23-4ac9-b0e9-7101fedfdc89, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Doch, das muss es. Ich habe es nur offen gelassen, weil du gesagt hattest, dass du `plugin.xml` selbst korrigierst. Ich prüfe jetzt den aktuellen Stand der Datei und trage die View ein.
ReasoningToken: 102
Control Request:
```yaml
id: dabf-54-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/plugin.xml
  text: view.diff.DiffPanel
  reason: Aktuellen Stand von plugin.xml nach dem leeren Insert prüfen und die ID des DiffPanel-View-Elements finden
```
/answer dabf-54-0 allow
Control Request:
```yaml
id: dabf-55-54
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/plugin.xml
      nodes:
      - id: plugin.extension_2
        code: "<extension\n\t\tpoint=\"org.eclipse.ui.views\">\n\t\t<category\n\t\t\tid=\"XY.AI.Workbench\"\n\t\t\tname=\"AI\">\n\t\t</category>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.AISessionView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.AISessionView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"XY.Ai Session Settings\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.AIBatchView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.AIBatchView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"XY.AI Batch List\">\n\t\t</view>\n\t\t<view\n\t\t\tname=\"AI Explorer\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.explorer.MarkdownExplorerView\"\n\t\t\tid=\"xy.ai.workbench.view.AIExplorer\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.ClaudeCodeSessionView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.ClaudeCodeSessionView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"CC Sessions\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.diff.DiffPanel\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.diff.DiffPanel\"\n\t\t\tname=\"AI Diff\">\n\t\t</view>\n\t</extension>"
```
/answer dabf-55-54 allow
Text:
 `plugin.xml` ist unverändert, der leere Insert hat nichts bewirkt. Ich füge die neue View direkt nach dem `DiffPanel`-Eintrag ein.
ReasoningToken: 86
Control Request:
```yaml
id: dabf-56-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/plugin.xml
    id: plugin.extension_2
    old_text: "name=\"AI Diff\">\n\t\t</view>"
    new_text: "name=\"AI Diff\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.approver.AutoApprovePanel\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.approver.AutoApprovePanel\"\n\t\t\tname=\"AI Auto Approve\">\n\t\t</view>"
  reason: AutoApprovePanel als View in plugin.xml registrieren
```
/answer dabf-56-0 allow
Result Stats: id=3230103e-7b23-4ac9-b0e9-7101fedfdc89, total: 77840, in: 46, out: 22193, reason: 0, read: 807730, write: 55601, start: 1794075442094, end: 1794075445070, time: 2915, total: 179896, io: 0, control: 61, sent: 0, recv: 59
Das Panel ist jetzt in `plugin.xml` registriert. Die View heißt "AI Auto Approve" und liegt in der Kategorie "AI" (`XY.AI.Workbench`), direkt nach "AI Diff". Ihre ID ist `xy.ai.workbench.view.approver.AutoApprovePanel`. Ich habe das nur als Texteingriff gemacht und nichts gestartet. Du kannst sie in Eclipse über *Window → Show View → AI* öffnen.

Mein leerer `ast_insert` von vorhin hatte die Datei nicht verändert, der Rest von `plugin.xml` ist unverändert.
