Der `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java` soll sich Tool-Calls aus "Control Requests" und direkte, pro session merken um sie mit einem folgenden Result verknüpfen zu können.

- Er braucht sich nicht mehr als die letzten 10 Tool-Calls zu merken
- Regeln sollen die Information bekommen, zu welchen Tool-Call ein Result gehört
- Die Tool-Call und Tool-Result der "Control Requests" sind nach dem Schema `<hash>-<nummer>-<ref>` konstruiert. Ein Tool-Call hat eine "<ref>" von 0 und ein Result hat als "<ref>" die "<nummer>" des Tool-Calls zu dem sie gehört.

Implementiere dann auch eine Regel analog zu `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java` das Tool-Result erlaubt die weniger als 3000 Zeichen haben und zu laut `xy.ai.conscious.autoapprove.rules.AllowedToolsRule.ALLOWED_TOOLS` abenfalls automatisch erlaubt werden. Größere Tool-Results soll der Operator selbst genehmigen.
SystemInit:  id=afa1dafc-7a6e-4f03-b242-d09fab790969, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst die betroffenen Dateien an.
Control Request:
```yaml
id: dabf-1-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
  reason: Struktur von AutoApprover und AllowedToolsRule verstehen
```
/answer dabf-1-0 allow
ReasoningToken: 13
Control Request:
```yaml
id: dabf-2-1
result:
  structuredContent:
    results:
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
          docstring: Evaluates the rules for the answer. Must be called before the prompt tag is
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
      nodes:
      - id: z2FhVL|9B9F9F
        signature: import java.util.Set;
      - id: AllowedToolsRule
        signature: "/**  * Allows every call of the tools listed in {"
        docstring: "Allows every call of the tools listed in {@link #ALLOWED_TOOLS}, regardless"
        children:
        - id: AllowedToolsRule.9ooLSz|6h9AKo
          signature: "public static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\" };"
        - id: AllowedToolsRule.AllowedToolsRule
          signature: public AllowedToolsRule()
        - id: AllowedToolsRule.AllowedToolsRule_2
          signature: public AllowedToolsRule(String... toolNames)
        - id: AllowedToolsRule.vote
          signature: public Vote vote(ApprovalContext ctx)
```
/answer dabf-2-1 allow
Control Request:
```yaml
id: dabf-3-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: .*
    include:
    - '*.java'
    limit: 50
  reason: Alle Dateien im autoapprove-Paket auflisten
```
/answer dabf-3-0 allow
Control Request:
```yaml
id: dabf-4-3
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: RuleRegistry.java
        matches:
        - package xy.ai.conscious.autoapprove;
        - ""
        - import java.util.List;
        - import java.util.Objects;
        - import java.util.concurrent.CopyOnWriteArrayList;
        - import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
        - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
        - /** Ordered registry of rules. */
        - "public class RuleRegistry {"
        - private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayList<>();
        - "public RuleRegistry() {"
        - register(new AllowedToolsRule());
        - "}"
        - "public RuleRegistry(List<IApprovalRule> rules) {"
        - this.rules.addAll(rules);
        - "public void register(IApprovalRule rule) {"
        - rules.addIfAbsent(Objects.requireNonNull(rule));
        - "public boolean unregister(IApprovalRule rule) {"
        - return rules.remove(rule);
        - "public List<IApprovalRule> rules() {"
        - return rules;
      - path: AutoApprover.java
        matches:
        - package xy.ai.conscious.autoapprove;
        - ""
        - import java.util.ArrayList;
        - import java.util.List;
        - import java.util.Optional;
        - import java.util.concurrent.CopyOnWriteArrayList;
        - import java.util.function.Consumer;
        - import xy.ai.conscious.api.IActiveEditor.Selection;
        - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
        - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
        - import xy.ai.conscious.autoapprove.rules.Vote;
        - import xy.ai.conscious.commands.AnswerCommand;
        - import xy.ai.conscious.commands.CallCommand;
        - import xy.ai.conscious.connector.harness.Prompt;
        - import xy.ai.conscious.core.LOG;
        - import xy.ai.conscious.model.AIAnswer;
      warning: "Limit of 50 matches reached (total 610); narrow pattern, directory or include/exclude filters."
```
/answer dabf-4-3 allow
Control Request:
```yaml
id: dabf-5-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
  reason: Vollständigen Quelltext lesen
```
/answer dabf-5-0 allow
Control Request:
```yaml
id: dabf-6-5
result:
  structuredContent:
    results:
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
          docstring: Evaluates the rules for the answer. Must be called before the prompt tag is
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
      nodes:
      - id: z2FhVL|9B9F9F
        signature: import java.util.Set;
      - id: AllowedToolsRule
        signature: "/**  * Allows every call of the tools listed in {"
        docstring: "Allows every call of the tools listed in {@link #ALLOWED_TOOLS}, regardless"
        children:
        - id: AllowedToolsRule.9ooLSz|6h9AKo
          signature: "public static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\" };"
        - id: AllowedToolsRule.AllowedToolsRule
          signature: public AllowedToolsRule()
        - id: AllowedToolsRule.AllowedToolsRule_2
          signature: public AllowedToolsRule(String... toolNames)
        - id: AllowedToolsRule.vote
          signature: public Vote vote(ApprovalContext ctx)
```
/answer dabf-6-5 allow
Control Request:
```yaml
id: dabf-7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ^(public |final )?(abstract )?(class|interface|record|enum)
    include:
    - '*.java'
    limit: 50
  reason: "Typen im Paket (ApprovalContext, ApprovalItem, ...) finden"
```
/answer dabf-7-0 deny
Text:
 Die Suche wurde abgelehnt. Ich lese deshalb gezielt die Knoten, die ich brauche.
ReasoningToken: 90
Control Request:
```yaml
id: dabf-8-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
    ids:
    - AllowedToolsRule
  reason: Quelltext der Klassen AutoApprover und AllowedToolsRule per Node-ID lesen
```
/answer dabf-8-0 allow
Control Request:
```yaml
id: dabf-9-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover
        type: ClassOrInterfaceDeclaration
        signature: /**  * Evaluates the registered rules against an answer and derives the follow-…
        docstring: Evaluates the registered rules against an answer and derives the follow-up
        children:
        - id: AutoApprover.tK23pE|KMoPOJ
          type: StatementGroup
          code: |2
              public static final int DEFAULT_MAX_CHAIN = 20;
              private final RuleRegistry registry;
              private final SessionTracker sessions = new SessionTracker();
              private final List<Consumer<Decision>> listeners = new CopyOnWriteArrayList<>();
              private volatile int maxChain = DEFAULT_MAX_CHAIN;
              private volatile String idleInstruction = "Continue with the task.";
        - id: AutoApprover.AutoApprover
          type: ConstructorDeclaration
          code: |2
              public AutoApprover() {
                this(new RuleRegistry());
              }
        - id: AutoApprover.AutoApprover_2
          type: ConstructorDeclaration
          code: |2
              public AutoApprover(RuleRegistry registry) {
                this.registry = registry;
              }
        - id: AutoApprover.rules
          type: MethodDeclaration
          code: |2
              public RuleRegistry rules() {
                return registry;
              }
        - id: AutoApprover.addListener
          type: MethodDeclaration
          code: |2
              public void addListener(Consumer<Decision> listener) {
                listeners.add(listener);
              }
        - id: AutoApprover.setMaxChain
          type: MethodDeclaration
          code: |2
              public void setMaxChain(int maxChain) {
                this.maxChain = maxChain;
              }
        - id: AutoApprover.setIdleInstruction
          type: MethodDeclaration
          code: |2
              public void setIdleInstruction(String instruction) {
                this.idleInstruction = instruction == null ? "" : instruction;
              }
        - id: AutoApprover.forget
          type: MethodDeclaration
          code: |2
              public void forget(String sessionId) {
                sessions.forget(sessionId);
              }
        - id: AutoApprover.evaluate
          type: MethodDeclaration
          code: |2
              public Decision evaluate(AIAnswer ans, int depth) {
                SessionSnapshot session = sessions.record(ans);
                List<ApprovalItem> items = AnswerParser.parse(ans.answer);
                ApprovalItem item = items.size() == 1 ? items.get(0) : null;
                ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, session);
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
                publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp));
                return Optional.of(followUp);
              }
        - id: AutoApprover.collect
          type: MethodDeclaration
          code: |2
              private List<RuleVote> collect(ApprovalContext ctx) {
                List<RuleVote> votes = new ArrayList<>();
                for (IApprovalRule rule : registry.rules()) {
                  try {
                    Vote vote = rule.vote(ctx);
                    votes.add(new RuleVote(rule.id(), vote == null ? Vote.abstain() : vote));
                  } catch (RuntimeException e) {
                    LOG.error("Approval rule failed: " + rule.id(), e);
                  }
                }
                return votes;
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
                    item.isControl() ? withBlock(item, command(AnswerCommand.CMD_ANSWER + " " + item.id() + " deny", reason)) : List.of(denial(item, reason));
                  case Call ->
                    withBlock(item, CallCommand.CMD_CALL + " " + item.id());
                };
                return lines == null ? null : new FollowUp(origin, selection(lines));
              }
        - id: AutoApprover.withBlock
          type: MethodDeclaration
          code: |2
              private static List<String> withBlock(ApprovalItem item, String commandLine) {
                List<String> lines = new ArrayList<>();
                lines.add("```yaml");
                item.yaml().lines().forEach(lines::add);
                lines.add("```");
                lines.add(commandLine);
                return lines;
              }
        - id: AutoApprover.denial
          type: MethodDeclaration
          code: |2
              private static String denial(ApprovalItem item, String reason) {
                String tool = item.tool().isBlank() ? "" : " (" + item.tool() + ")";
                String why = oneLine(reason);
                return "Tool call " + item.id() + tool + " was denied" + (why.isEmpty() ? "." : ": " + why);
              }
        - id: AutoApprover.command
          type: MethodDeclaration
          code: |2
              private static String command(String base, String extra) {
                String text = oneLine(extra);
                return text.isEmpty() ? base : base + " " + text;
              }
        - id: AutoApprover.oneLine
          type: MethodDeclaration
          code: |2
              private static String oneLine(String s) {
                return s == null ? "" : s.replaceAll("\\s+", " ").strip();
              }
        - id: AutoApprover.selection
          type: MethodDeclaration
          code: |2
              private static Selection selection(List<String> lines) {
                return new Selection(lines.toArray(String[]::new), null);
              }
        - id: AutoApprover.addNonBlank
          type: MethodDeclaration
          code: |2
              private static void addNonBlank(List<String> list, String text) {
                if (text != null && !text.isBlank())
                  list.add(text);
              }
        - id: AutoApprover.enrich
          type: MethodDeclaration
          code: |2
              private static void enrich(AIAnswer ans, List<String> warnings) {
                if (warnings.isEmpty())
                  return;
                StringBuilder sb = new StringBuilder();
                for (String w : warnings) sb.append("//warning: ").append(oneLine(w)).append('\n');
                ans.answer = sb + (ans.answer == null ? "" : ans.answer);
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
      nodes:
      - id: AllowedToolsRule
        type: ClassOrInterfaceDeclaration
        signature: "/**  * Allows every call of the tools listed in {"
        docstring: "Allows every call of the tools listed in {@link #ALLOWED_TOOLS}, regardless"
        children:
        - id: AllowedToolsRule.9ooLSz|6h9AKo
          type: StatementGroup
          code: "\tpublic static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\" };\n\n\tprivate final Set<String> allowed;\n"
        - id: AllowedToolsRule.AllowedToolsRule
          type: ConstructorDeclaration
          code: "\tpublic AllowedToolsRule() {\n\t\tthis(ALLOWED_TOOLS);\n\t}\n"
        - id: AllowedToolsRule.AllowedToolsRule_2
          type: ConstructorDeclaration
          code: "\tpublic AllowedToolsRule(String... toolNames) {\n\t\tthis.allowed = Set.of(toolNames);\n\t}\n"
        - id: AllowedToolsRule.vote
          type: MethodDeclaration
          code: "\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\treturn allowed.contains(item.tool()) ? Vote.allow() : Vote.abstain();\n\t}\n"
```
/answer dabf-9-8 allow
Text:
 `resultFor` gibt für `Kind.ToolResult` immer `Pass` zurück. Das muss ich beim Umbau berücksichtigen. Als Nächstes lese ich die zugehörigen Typen.
ReasoningToken: 112
Control Request:
```yaml
id: dabf-a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ^package
    include:
    - ApprovalContext.java
    - ApprovalItem.java
    - SessionTracker.java
    - SessionSnapshot.java
    - AnswerParser.java
    - Decision.java
    - ApprovalResult.java
    limit: 20
  reason: "Prüfen, welche der benötigten Typen im Paket existieren"
```
/answer dabf-a-0 allow
Control Request:
```yaml
id: dabf-b-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: Decision.java
        matches:
        - package xy.ai.conscious.autoapprove;
      - path: ApprovalContext.java
        matches:
        - package xy.ai.conscious.autoapprove;
      - path: SessionTracker.java
        matches:
        - package xy.ai.conscious.autoapprove;
      - path: AnswerParser.java
        matches:
        - package xy.ai.conscious.autoapprove;
      - path: ApprovalResult.java
        matches:
        - package xy.ai.conscious.autoapprove;
      - path: SessionSnapshot.java
        matches:
        - package xy.ai.conscious.autoapprove;
      - path: ApprovalItem.java
        matches:
        - package xy.ai.conscious.autoapprove;
```
/answer dabf-b-a allow
Control Request:
```yaml
id: dabf-c-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionSnapshot.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
  reason: Strukturen der Hilfstypen prüfen
```
/answer dabf-c-0 allow
Control Request:
```yaml
id: dabf-d-c
result:
  structuredContent:
    results:
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
      nodes:
      - id: u5AEf0|3azJeX
        signature: import java.time.Instant;
      - id: SessionTracker
        signature: "final class SessionTracker {"
        children:
        - id: SessionTracker.State
          signature: "private static final class State {"
        - id: SessionTracker.KU3d9P|xtMvk7
          signature: "private final Map<String, State> sessions = new ConcurrentHashMap<>();"
        - id: SessionTracker.keyOf
          signature: static String keyOf(AIAnswer ans)
          docstring: "Connectors without own session id leave {@code sessionId == id}; fall back to t…"
        - id: SessionTracker.record
          signature: SessionSnapshot record(AIAnswer ans)
        - id: SessionTracker.lastPrompt
          signature: Prompt lastPrompt(String key)
        - id: SessionTracker.forget
          signature: void forget(String key)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionSnapshot.java
      nodes:
      - id: u5AEf0|FmRYFF
        signature: import java.time.Instant;
      - id: SessionSnapshot
        signature: /**  * Session state as seen by the rules. Includes the answer under evaluation…
        docstring: Session state as seen by the rules. Includes the answer under evaluation.
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
      nodes:
      - id: 7jZU6l|In6FUr
        signature: import java.util.ArrayList;
      - id: AnswerParser
        signature: /**  * Extracts tool calls and tool results from the rendered answer text.  */ …
        docstring: Extracts tool calls and tool results from the rendered answer text.
        children:
        - id: AnswerParser.vUPH7s|pV9dhl
          signature: private static final Pattern BLOCK = Pattern.compile(
        - id: AnswerParser.parse
          signature: static List<ApprovalItem> parse(String text)
        - id: AnswerParser.toItem
          signature: "private static ApprovalItem toItem(String marker, String yaml)"
```
/answer dabf-d-c allow
Control Request:
```yaml
id: dabf-e-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    ids:
    - ApprovalContext
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
    ids:
    - ApprovalItem
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
    ids:
    - SessionTracker
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionSnapshot.java
    ids:
    - SessionSnapshot
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
    ids:
    - AnswerParser
  reason: "Quelltext der Hilfstypen lesen, um Tracking und Regel-Kontext zu erweitern"
```
/answer dabf-e-0 allow
Control Request:
```yaml
id: dabf-f-e
result:
  structuredContent:
    results:
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
      nodes:
      - id: SessionTracker
        type: ClassOrInterfaceDeclaration
        signature: "final class SessionTracker {"
        children:
        - id: SessionTracker.State
          type: ClassOrInterfaceDeclaration
          code: "\tprivate static final class State {\n\t\tint turn;\n\t\tfinal TokenStats tokens = new TokenStats();\n\t\tInstant lastActivity;\n\t\tPrompt lastPrompt;\n\t}\n"
        - id: SessionTracker.KU3d9P|xtMvk7
          type: StatementGroup
          code: "\tprivate final Map<String, State> sessions = new ConcurrentHashMap<>();\n"
        - id: SessionTracker.keyOf
          type: MethodDeclaration
          code: "\tstatic String keyOf(AIAnswer ans) {\n\t\tif (ans.sessionId != null && !ans.sessionId.equals(ans.id))\n\t\t\treturn ans.sessionId;\n\t\treturn ans.prompt != null ? ans.prompt.sessionId() : ans.id;\n\t}\n"
        - id: SessionTracker.record
          type: MethodDeclaration
          code: "\tSessionSnapshot record(AIAnswer ans) {\n\t\tString key = keyOf(ans);\n\t\tState s = sessions.computeIfAbsent(key, k -> new State());\n\t\tsynchronized (s) {\n\t\t\tif (ans.showStats)\n\t\t\t\ts.turn++;\n\t\t\ts.tokens.add(ans.stats);\n\t\t\ts.lastActivity = Instant.now();\n\t\t\tif (ans.prompt != null)\n\t\t\t\ts.lastPrompt = ans.prompt;\n\t\t\tTokenStats copy = new TokenStats();\n\t\t\tcopy.add(s.tokens);\n\t\t\treturn new SessionSnapshot(key, s.turn, copy, s.lastActivity);\n\t\t}\n\t}\n"
        - id: SessionTracker.lastPrompt
          type: MethodDeclaration
          code: "\tPrompt lastPrompt(String key) {\n\t\tState s = sessions.get(key);\n\t\tif (s == null)\n\t\t\treturn null;\n\t\tsynchronized (s) {\n\t\t\treturn s.lastPrompt;\n\t\t}\n\t}\n"
        - id: SessionTracker.forget
          type: MethodDeclaration
          code: "\tvoid forget(String key) {\n\t\tsessions.remove(key);\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionSnapshot.java
      nodes:
      - id: SessionSnapshot
        type: RecordDeclaration
        code: |
          public record SessionSnapshot(String id, int turn, TokenStats tokens, Instant lastActivity) {
          }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
      nodes:
      - id: AnswerParser
        type: ClassOrInterfaceDeclaration
        signature: /**  * Extracts tool calls and tool results from the rendered answer text.  */ …
        docstring: Extracts tool calls and tool results from the rendered answer text.
        children:
        - id: AnswerParser.vUPH7s|pV9dhl
          type: StatementGroup
          code: "\tprivate static final Pattern BLOCK = Pattern.compile(\n\t\t\t\"^(\" + Pattern.quote(IEditorInterface.CONTROL_REQUEST) + \"|\" + Pattern.quote(IEditorInterface.TOOLUSE) + \"|\"\n\t\t\t\t\t+ Pattern.quote(IEditorInterface.TOOLRESULT) + \")[ \\\\t]*\\\\R```yaml\\\\R(.*?)^```[ \\\\t]*$\",\n\t\t\tPattern.MULTILINE | Pattern.DOTALL);\n\tprivate static final YAMLMapper MAPPER = new YAMLMapper();\n"
        - id: AnswerParser.parse
          type: MethodDeclaration
          code: "\tstatic List<ApprovalItem> parse(String text) {\n\t\tList<ApprovalItem> items = new ArrayList<>();\n\t\tif (text == null || text.isEmpty())\n\t\t\treturn items;\n\t\tMatcher m = BLOCK.matcher(text);\n\t\twhile (m.find()) {\n\t\t\tApprovalItem item = toItem(m.group(1), m.group(2));\n\t\t\tif (item != null)\n\t\t\t\titems.add(item);\n\t\t}\n\t\treturn items;\n\t}\n"
        - id: AnswerParser.toItem
          type: MethodDeclaration
          code: "\tprivate static ApprovalItem toItem(String marker, String yaml) {\n\t\tJsonNode node;\n\t\ttry {\n\t\t\tnode = MAPPER.readTree(yaml);\n\t\t} catch (JsonProcessingException e) {\n\t\t\treturn null;\n\t\t}\n\t\tif (node == null || !node.isObject())\n\t\t\treturn null;\n\t\tKind kind;\n\t\tif (IEditorInterface.CONTROL_REQUEST.equals(marker))\n\t\t\tkind = node.has(\"result\") ? Kind.ControlResult : Kind.ControlCall;\n\t\telse\n\t\t\tkind = IEditorInterface.TOOLUSE.equals(marker) ? Kind.ToolCall : Kind.ToolResult;\n\t\tString tool = node.path(\"tool\").asText(node.path(\"name\").asText(\"\"));\n\t\treturn new ApprovalItem(kind, node.path(\"id\").asText(\"\"), tool, node, yaml);\n\t}\n"
```
/answer dabf-f-e allow
Control Request:
```yaml
id: dabf-10-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
  node_type: RecordDeclaration
  reason: Record-Komponenten und Javadoc von ApprovalContext und ApprovalItem lesen
```
/answer dabf-10-0 allow
ReasoningToken: 79
Control Request:
```yaml
id: dabf-11-10
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      nodes:
      - id: ApprovalContext
        type: RecordDeclaration
        code: "public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, SessionSnapshot session) {\n\n\tpublic boolean hasItem() {\n\t\treturn item != null;\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
      nodes:
      - id: ApprovalItem
        type: RecordDeclaration
        code: "public record ApprovalItem(Kind kind, String id, String tool, JsonNode data, String yaml) {\n\n\tpublic enum Kind {\n\t\tControlCall, ControlResult, ToolCall, ToolResult\n\t}\n\n\tpublic JsonNode arguments() {\n\t\treturn data.path(\"arguments\");\n\t}\n\n\tpublic JsonNode result() {\n\t\treturn data.path(\"result\");\n\t}\n\n\t/** Item belongs to a control request (MCP bound connector). */\n\tpublic boolean isControl() {\n\t\treturn kind == Kind.ControlCall || kind == Kind.ControlResult;\n\t}\n}\n"
```
/answer dabf-11-10 allow
Control Request:
```yaml
id: dabf-12-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  node_type: ImportDeclaration|PackageDeclaration
  reason: Imports und Header der Dateien als Vorlage für die neue Regel
```
/answer dabf-12-0 allow
Control Request:
```yaml
id: dabf-13-12
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes: []
```
/answer dabf-13-12 allow
ReasoningToken: 34
Control Request:
```yaml
id: dabf-14-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules
    pattern: ^(package|import|public|\s+(default|String|Vote)|\s+(public )?static)
    include:
    - AllowedToolsRule.java
    - IApprovalRule.java
    - Vote.java
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ToolResult|ControlResult|new ApprovalContext|new SessionSnapshot|\.record\(
    include:
    - '*.java'
    limit: 30
  reason: Imports und Interface von Regeln sowie Verwendungsstellen der zu ändernden Typen prüfen
```
/answer dabf-14-0 allow
Control Request:
```yaml
id: dabf-15-14
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules
      matches:
      - path: Vote.java
        matches:
        - package xy.ai.conscious.autoapprove.rules;
        - import java.util.ArrayList;
        - import java.util.List;
        - "public record Vote(Verdict verdict, String reason, String hint, List<String> warnings, String instruction) {"
        - "public static Vote abstain() {"
        - "public static Vote allow() {"
        - "public static Vote allow(String hint) {"
        - "public static Vote deny(String reason) {"
        - "public static Vote warning(String text) {"
        - "public static Vote instruction(String text) {"
      - path: IApprovalRule.java
        matches:
        - package xy.ai.conscious.autoapprove.rules;
        - import xy.ai.conscious.autoapprove.ApprovalContext;
        - "public interface IApprovalRule {"
        - "default String id() {"
        - Vote vote(ApprovalContext ctx);
      - path: AllowedToolsRule.java
        matches:
        - package xy.ai.conscious.autoapprove.rules;
        - import java.util.Set;
        - import xy.ai.conscious.autoapprove.ApprovalContext;
        - import xy.ai.conscious.autoapprove.ApprovalItem;
        - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
        - "public class AllowedToolsRule implements IApprovalRule {"
        - "public static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\" };"
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: AutoApprover.java
        matches:
        - SessionSnapshot session = sessions.record(ans);
        - "ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, session);"
        - if (item == null || item.id().isBlank() || item.kind() == Kind.ToolResult)
      - path: SessionTracker.java
        matches:
        - "return new SessionSnapshot(key, s.turn, copy, s.lastActivity);"
      - path: AnswerParser.java
        matches:
        - "kind = node.has(\"result\") ? Kind.ControlResult : Kind.ControlCall;"
        - "kind = IEditorInterface.TOOLUSE.equals(marker) ? Kind.ToolCall : Kind.ToolResult;"
      - path: ApprovalItem.java
        matches:
        - "ControlCall, ControlResult, ToolCall, ToolResult"
        - return kind == Kind.ControlCall || kind == Kind.ControlResult;
```
/answer dabf-15-14 allow
ReasoningToken: 30
Control Request:
```yaml
id: dabf-16-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
    pattern: new ApprovalContext\(|new SessionSnapshot\(
    include:
    - '*.java'
    exclude:
    - '**/target/**'
    - '**/bin/**'
    limit: 20
  reason: "Aufrufer der Record-Konstruktoren im gesamten Workspace finden, bevor ApprovalContext erweitert wird"
```
/answer dabf-16-0 allow
Control Request:
```yaml
id: dabf-17-16
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
      matches:
      - path: autoapprove/AutoApprover.java
        matches:
        - "ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, session);"
      - path: autoapprove/SessionTracker.java
        matches:
        - "return new SessionSnapshot(key, s.turn, copy, s.lastActivity);"
```
/answer dabf-17-16 allow
Text:
 Der Konstruktor von `ApprovalContext` wird nur in `AutoApprover` aufgerufen, ich kann ihn also gefahrlos erweitern. Davor lese ich noch das Javadoc des Records und die Imports von `SessionTracker`.
ReasoningToken: 2059
Control Request:
```yaml
id: dabf-18-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ^(import|/\*\*| \*|\s*\*/)
    include:
    - ApprovalContext.java
    - SessionTracker.java
    intent: editing
    limit: 40
  reason: Javadoc von ApprovalContext und Imports von SessionTracker vor dem Bearbeiten ansehen
```
/answer dabf-18-0 allow
Control Request:
```yaml
id: dabf-19-18
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: ApprovalContext.java
        nodes:
        - id: GwlVvM|lj1u2H
          matches:
          - import xy.ai.conscious.connector.harness.Prompt;
          - import xy.ai.conscious.model.AIAnswer;
        matches:
        - lineno: 6
          match: /**
        - lineno: 7
          match: '* Input of a rule.'
        - lineno: 8
          match: '*'
        - lineno: 9
          match: "* @param item {@code null} if the answer contains no (or more than one) tool call/result"
        - lineno: 10
          match: '*/'
      - path: SessionTracker.java
        nodes:
        - id: u5AEf0|3azJeX
          matches:
          - import java.time.Instant;
          - import java.util.Map;
          - import java.util.concurrent.ConcurrentHashMap;
          - import xy.ai.conscious.connector.harness.Prompt;
          - import xy.ai.conscious.model.AIAnswer;
          - import xy.ai.conscious.model.TokenStats;
```
/answer dabf-19-18 allow
Text:
 Ich passe jetzt `ApprovalContext` und `SessionTracker` an und lege die neue Regel an.
Control Request:
```yaml
id: dabf-1a-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    id: ApprovalContext
    old_text: "ApprovalItem item, SessionSnapshot session)"
    new_text: "ApprovalItem item, ApprovalItem call, SessionSnapshot session)"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    id: ApprovalContext
    old_text: more than one) tool call/result
    new_text: |-
      more than one) tool call/result
       * @param call the remembered tool call {@code item} is the result of; {@code null} if {@code item} is no result or
       *        the call is unknown (e.g. older than the remembered calls)
  reason: ApprovalContext um den zugehörigen Tool-Call erweitern
```
/answer dabf-1a-0 allow
ReasoningToken: 99
Control Request:
```yaml
id: dabf-1b-1a
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      error: |
        invalid type declaration: Encountered unexpected token: "record" "record"
            at line 1, column 8.

        Was expecting one of:

            "@"
            "class"
            "enum"
            "interface"
      id: ApprovalContext
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      error: Text not found in node.
      id: ApprovalContext
      reason: not_found
      next_step: reread_node
```
/answer dabf-1b-1a allow
Control Request:
```yaml
id: dabf-1c-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
    id: SessionTracker.State
    source: "\tprivate static final class State {\n\t\tint turn;\n\t\tfinal TokenStats tokens = new TokenStats();\n\t\tInstant lastActivity;\n\t\tPrompt lastPrompt;\n\t\tfinal Deque<ApprovalItem> calls = new ArrayDeque<>();\n\t}\n"
  reason: Pro Session einen begrenzten Speicher für Tool-Calls ergänzen
```
/answer dabf-1c-0 allow
Control Request:
```yaml
id: dabf-1d-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
    id: SessionTracker.lastPrompt
    position: before
    source: "\t/** Remembers the tool calls of the items; only the {@link #MAX_CALLS} most recent ones are kept. */\n\tvoid track(String key, List<ApprovalItem> items) {\n\t\tState s = sessions.computeIfAbsent(key, k -> new State());\n\t\tsynchronized (s) {\n\t\t\tfor (ApprovalItem item : items) {\n\t\t\t\tif (!isCall(item) || item.id().isBlank())\n\t\t\t\t\tcontinue;\n\t\t\t\ts.calls.removeIf(c -> c.id().equals(item.id()));\n\t\t\t\ts.calls.addLast(item);\n\t\t\t\twhile (s.calls.size() > MAX_CALLS)\n\t\t\t\t\ts.calls.removeFirst();\n\t\t\t}\n\t\t}\n\t}\n\n\t/** The remembered tool call the result belongs to, or {@code null}. */\n\tApprovalItem callFor(String key, ApprovalItem result) {\n\t\tif (result == null || (result.kind() != Kind.ControlResult && result.kind() != Kind.ToolResult))\n\t\t\treturn null;\n\t\tState s = sessions.get(key);\n\t\tString callId = callIdOf(result);\n\t\tif (s == null || callId == null)\n\t\t\treturn null;\n\t\tsynchronized (s) {\n\t\t\tfor (ApprovalItem c : s.calls) {\n\t\t\t\tif (c.id().equals(callId) && c.isControl() == result.isControl())\n\t\t\t\t\treturn c;\n\t\t\t}\n\t\t\treturn null;\n\t\t}\n\t}\n\n\t/**\n\t * Control request ids are {@code <hash>-<number>-<ref>}: a call has ref 0, a result carries the number of its call\n\t * as ref. Other results use the id of their call.\n\t */\n\tstatic String callIdOf(ApprovalItem result) {\n\t\tif (!result.isControl())\n\t\t\treturn result.id().isBlank() ? null : result.id();\n\t\tMatcher m = CONTROL_ID.matcher(result.id());\n\t\tif (!m.matches() || m.group(3).equals(\"0\"))\n\t\t\treturn null;\n\t\treturn m.group(1) + \"-\" + m.group(3) + \"-0\";\n\t}\n\n\tprivate static boolean isCall(ApprovalItem item) {\n\t\treturn item.kind() == Kind.ControlCall || item.kind() == Kind.ToolCall;\n\t}\n\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
    id: SessionTracker.KU3d9P|xtMvk7
    position: before
    source: "\tstatic final int MAX_CALLS = 10;\n\tprivate static final Pattern CONTROL_ID = Pattern.compile(\"(.+)-(\\\\d+)-(\\\\d+)\");\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
    id: u5AEf0|3azJeX
    position: before
    source: |
      import java.util.ArrayDeque;
      import java.util.Deque;
      import java.util.List;
      import java.util.regex.Matcher;
      import java.util.regex.Pattern;
      import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
  reason: "Methoden track/callFor/callIdOf, Konstanten und Imports in SessionTracker ergänzen"
```
/answer dabf-1d-0 allow
Control Request:
```yaml
id: dabf-1e-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.util.Set;\nimport com.fasterxml.jackson.databind.JsonNode;\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\n/**\n * Allows results shorter than {@link #MAX_LENGTH} characters if the tool call they belong to is listed in\n * {@link AllowedToolsRule#ALLOWED_TOOLS}. Larger results are left to the operator.\n */\npublic class SmallResultRule implements IApprovalRule {\n\n\tpublic static final int MAX_LENGTH = 3000;\n\n\tprivate final Set<String> allowed;\n\n\tpublic SmallResultRule() {\n\t\tthis(AllowedToolsRule.ALLOWED_TOOLS);\n\t}\n\n\tpublic SmallResultRule(String... toolNames) {\n\t\tthis.allowed = Set.of(toolNames);\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tApprovalItem call = ctx.call();\n\t\tif (item == null || call == null || call.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlResult && item.kind() != Kind.ToolResult)\n\t\t\treturn Vote.abstain();\n\t\tif (!allowed.contains(call.tool()))\n\t\t\treturn Vote.abstain();\n\t\treturn length(item) < MAX_LENGTH ? Vote.allow() : Vote.abstain();\n\t}\n\n\tprivate static int length(ApprovalItem item) {\n\t\tJsonNode result = item.result();\n\t\tif (result.isMissingNode())\n\t\t\treturn item.yaml().length();\n\t\treturn result.isTextual() ? result.asText().length() : result.toString().length();\n\t}\n}\n"
  reason: Neue Regel SmallResultRule analog zu AllowedToolsRule anlegen
```
/answer dabf-1e-0 allow
Text:
 Die Änderung an `ApprovalContext` ist fehlgeschlagen, weil das Werkzeug den Record nicht über Textersetzung bearbeiten kann. Ich ersetze deshalb den ganzen Knoten.
ReasoningToken: 126
Control Request:
```yaml
id: dabf-1f-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    id: ApprovalContext
    source: "/**\n * Input of a rule.\n *\n * @param item {@code null} if the answer contains no (or more than one) tool call/result\n * @param call the remembered tool call {@code item} is the result of; {@code null} if {@code item} is no result or\n *        the call is unknown (e.g. older than the remembered calls)\n */\npublic record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call,\n\t\tSessionSnapshot session) {\n\n\tpublic boolean hasItem() {\n\t\treturn item != null;\n\t}\n}\n"
  reason: ApprovalContext-Record durch Version mit Komponente 'call' ersetzen
```
/answer dabf-1f-0 allow
Control Request:
```yaml
id: dabf-20-1f
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      error: |
        invalid type declaration: Encountered unexpected token: "record" "record"
            at line 8, column 8.

        Was expecting one of:

            "@"
            "class"
            "enum"
            "interface"
      id: ApprovalContext
```
/answer dabf-20-1f allow
Text:
 Der Parser des Werkzeugs unterstützt keine Records. Die Datei ist klein, deshalb schreibe ich sie komplett neu. Package, Imports und Javadoc habe ich vorher geprüft.
Control Request:
```yaml
id: dabf-21-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    overwrite: true
    source: "package xy.ai.conscious.autoapprove;\n\nimport xy.ai.conscious.connector.harness.Prompt;\nimport xy.ai.conscious.model.AIAnswer;\n\n/**\n * Input of a rule.\n *\n * @param item {@code null} if the answer contains no (or more than one) tool call/result\n * @param call the remembered tool call {@code item} is the result of; {@code null} if {@code item} is no result or\n *        the call is unknown (e.g. older than the remembered calls)\n */\npublic record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call,\n\t\tSessionSnapshot session) {\n\n\tpublic boolean hasItem() {\n\t\treturn item != null;\n\t}\n}\n"
  reason: ApprovalContext mit neuer Komponente 'call' neu schreiben (Record-Parser des Werkzeugs nicht unterstützt)
```
/answer dabf-21-0 allow
ReasoningToken: 157
Control Request:
```yaml
id: dabf-22-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.evaluate
    old_text: "    ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, session);"
    new_text: |2-
          sessions.track(session.id(), items);
          ApprovalItem call = sessions.callFor(session.id(), item);
          ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, call, session);
  reason: Tool-Calls in evaluate merken und den zum Result gehörenden Call in den Kontext geben
```
/answer dabf-22-0 allow
Control Request:
```yaml
id: dabf-23-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: RuleRegistry.RuleRegistry
    old_text: register(new AllowedToolsRule());
    new_text: "register(new AllowedToolsRule());\n\t\tregister(new SmallResultRule());"
  reason: Neue Regel standardmäßig registrieren
```
/answer dabf-23-0 allow
Text:
 Jetzt fehlt noch der Import in `RuleRegistry`. Danach prüfe ich, ob alles kompiliert.
Control Request:
```yaml
id: dabf-24-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: RuleRegistry
    old_text: import xy.ai.conscious.autoapprove.rules.IApprovalRule;
    new_text: |-
      import xy.ai.conscious.autoapprove.rules.IApprovalRule;
      import xy.ai.conscious.autoapprove.rules.SmallResultRule;
  reason: Import für SmallResultRule in RuleRegistry
```
/answer dabf-24-0 allow
Control Request:
```yaml
id: dabf-25-24
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      error: Text not found in node.
      id: RuleRegistry
      reason: not_found
      next_step: reread_node
```
/answer dabf-25-24 allow
Text:
 Imports liegen in einem eigenen Knoten, deshalb suche ich dessen ID.
Control Request:
```yaml
id: dabf-26-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  reason: ID des Import-Knotens in RuleRegistry ermitteln
```
/answer dabf-26-0 allow
Control Request:
```yaml
id: dabf-27-26
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: KNcHmE|O8u2I5
        signature: import java.util.List;
      - id: RuleRegistry
        signature: "/**  * Ordered registry of rules.  */ public class RuleRegistry {"
        docstring: Ordered registry of rules.
        children:
        - id: RuleRegistry.VUx1sY|HlFjwk
          signature: private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayL…
        - id: RuleRegistry.RuleRegistry
          signature: public RuleRegistry()
        - id: RuleRegistry.RuleRegistry_2
          signature: public RuleRegistry(List<IApprovalRule> rules)
        - id: RuleRegistry.register
          signature: public void register(IApprovalRule rule)
        - id: RuleRegistry.unregister
          signature: public boolean unregister(IApprovalRule rule)
        - id: RuleRegistry.rules
          signature: public List<IApprovalRule> rules()
```
/answer dabf-27-26 allow
Control Request:
```yaml
id: dabf-28-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: KNcHmE|O8u2I5
    old_text: import xy.ai.conscious.autoapprove.rules.IApprovalRule;
    new_text: |-
      import xy.ai.conscious.autoapprove.rules.IApprovalRule;
      import xy.ai.conscious.autoapprove.rules.SmallResultRule;
  reason: Import für SmallResultRule in RuleRegistry ergänzen
```
/answer dabf-28-0 allow
Control Request:
```yaml
id: dabf-29-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  reason: Geänderte Dateien auf Syntaxfehler prüfen
```
/answer dabf-29-0 allow
Control Request:
```yaml
id: dabf-2a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ^import .*(Kind|ApprovalItem);
    include:
    - SessionTracker.java
    - AutoApprover.java
    limit: 10
  reason: "Prüfen, dass ApprovalItem in AutoApprover (gleiches Paket) und Kind in SessionTracker importiert sind"
```
/answer dabf-2a-0 allow
Control Request:
```yaml
id: dabf-2b-2a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: AutoApprover.java
        matches:
        - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
      - path: SessionTracker.java
        matches:
        - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
```
/answer dabf-2b-2a allow
Result Stats: id=afa1dafc-7a6e-4f03-b242-d09fab790969, total: 51243, in: 36, out: 13591, reason: 51, read: 410795, write: 37616, start: 1794073634409, end: 1794073641759, time: 7284, total: 92674, io: 0, control: 66, sent: 0, recv: 189
ReasoningToken: 51
`AutoApprover` merkt sich jetzt pro Session die letzten 10 Tool-Calls und gibt Regeln den zu einem Result gehörenden Call. Die neue Regel `SmallResultRule` erlaubt kleine Results. Ich habe nur die Syntax geprüft und nicht kompiliert. Es gibt auch keine Tests dafür.

**Änderungen**
- **`SessionTracker`:** Jede Session hält jetzt höchstens 10 Calls, ältere fallen heraus. Eine neue Methode `track` merkt sich `ControlCall` und `ToolCall`, die in einer Antwort vorkommen. `callFor` liefert zu einem Result den passenden Call.
- **Zuordnung bei Control Requests:** Die ID hat das Schema `<hash>-<nummer>-<ref>`. Ein Result `h-6-5` gehört zum Call `h-5-0`.
- **Zuordnung bei direkten Tool-Calls:** Hier nehme ich an, dass das Result dieselbe `id` wie der Call hat. Das habe ich nicht gegen echte Daten geprüft.
- **`ApprovalContext`:** Neues Feld `call` mit dem zugehörigen Tool-Call, `null` wenn er unbekannt oder schon vergessen ist. Der Record ist jetzt `ApprovalContext(answer, prompt, item, call, session)`. Ich habe die Datei komplett neu geschrieben, weil das Werkzeug keine Records bearbeiten kann. Der Konstruktor wird nur in `AutoApprover` aufgerufen.
- **`AutoApprover.evaluate`:** Die Methode merkt die Calls und füllt `call` im Kontext.
- **`SmallResultRule`:** Sie stimmt für ein Result mit `Allow`, wenn das Tool seines Calls in `AllowedToolsRule.ALLOWED_TOOLS` steht und das Result kürzer als 3000 Zeichen ist. Sonst enthält sie sich, und der Operator entscheidet selbst.
- **`RuleRegistry`:** Die Regel ist neben `AllowedToolsRule` standardmäßig registriert.

**Hinweise**
- Die Länge zählt vom Feld `result`: bei Text die Zeichen, bei einer Struktur die JSON-Darstellung. Fehlt das Feld, zählt das ganze YAML.
- `AutoApprover.resultFor` gibt für `Kind.ToolResult` weiterhin `Pass` zurück. Die Regel wirkt deshalb nur bei Results aus Control Requests, die der Operator sonst bestätigen müsste. Bei direkten Results hat ihr Votum keine Folge.
