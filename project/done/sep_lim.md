Lagere die Funktion von `DEFAULT_MAX_CHAIN` in `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java` in eine eigene konfigurierbare Regel aus wie `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java`
SystemInit:  id=a1cab16b-c523-4125-9037-1c32c77eff57, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 5f0f-45-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
  reason: Struktur beider Dateien ansehen
```
/answer 5f0f-45-0 allow
Control Request:
```yaml
id: 5f0f-46-45
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
        - id: AutoApprover.TaT2M1|RdFmS2
          excert: public static final int DEFAULT_MAX_CHAIN = 20;
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
        - id: AutoApprover.setMaxChain
          excert: public void setMaxChain(int maxChain)
          docstring: Maximum number of consecutive automatic follow-up prompts.
        - id: AutoApprover.setIdleInstruction
          excert: public void setIdleInstruction(String instruction)
        - id: AutoApprover.forget
          excert: public void forget(String sessionId)
        - id: AutoApprover.evaluate
          excert: "public Decision evaluate(AIAnswer ans, int depth)"
        - id: AutoApprover.onIdle
          excert: public Optional<FollowUp> onIdle(String sessionId)
        - id: AutoApprover.collect
          excert: private List<RuleVote> collect(ApprovalContext ctx)
        - id: AutoApprover.decide
          excert: "private Decision decide(ApprovalContext ctx, List<RuleVote> votes, int depth)"
        - id: AutoApprover.resultFor
          excert: "private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolea…"
        - id: AutoApprover.followUpFor
          excert: "private FollowUp followUpFor(ApprovalContext ctx, ApprovalResult result, String…"
        - id: AutoApprover.withBlock
          excert: "private static List<String> withBlock(ApprovalItem item, String commandLine)"
          docstring: "Yaml block followed by the command line, as the operator would select it."
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
      nodes:
      - id: z2FhVL|v3y3zx
        excert: import java.util.Set;
      - id: SmallResultRule
        excert: "/**  * Allows small results of the tools allowed by {"
        docstring: "Allows small results of the tools allowed by {@link AllowedToolsRule}."
        children:
        - id: SmallResultRule.d86O60|CJR8lV
          excert: public static final String ID = SmallResultRule.class.getSimpleName();
        - id: SmallResultRule.SmallResultRule
          excert: public SmallResultRule()
        - id: SmallResultRule.SmallResultRule_2
          excert: public SmallResultRule(long maxLength)
        - id: SmallResultRule.id
          excert: public String id()
        - id: SmallResultRule.getDefaultSetting
          excert: public RuleSettings getDefaultSetting()
        - id: SmallResultRule.vote
          excert: public Vote vote(ApprovalContext ctx)
        - id: SmallResultRule.length
          excert: private static int length(ApprovalItem item)
```
/answer 5f0f-46-45 allow
Control Request:
```yaml
id: 5f0f-47-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: DEFAULT_MAX_CHAIN|SmallResultRule
    include:
    - '*.java'
    - '*.xml'
    - '*.properties'
    - '*.json'
    - '*.yaml'
    - '*.yml'
    exclude:
    - target/**
    - build/**
  reason: Alle Verwendungen finden
```
/answer 5f0f-47-0 allow
Control Request:
```yaml
id: 5f0f-48-47
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/RuleRegistry.java
        matches:
        - import xy.ai.conscious.autoapprove.rules.SmallResultRule;
        - register(new SmallResultRule());
      - path: xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - public static final int DEFAULT_MAX_CHAIN = 20;
        - private volatile int maxChain = DEFAULT_MAX_CHAIN;
      - path: xy/ai/conscious/autoapprove/rules/SmallResultRule.java
        matches:
        - "public class SmallResultRule implements IApprovalRule {"
        - public static final String ID = SmallResultRule.class.getSimpleName();
        - "public SmallResultRule() {"
        - "public SmallResultRule(long maxLength) {"
```
/answer 5f0f-48-47 allow
Control Request:
```yaml
id: 5f0f-49-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover
    - AutoApprover.collect
    - AutoApprover.decide
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  reason: Quellcode lesen
```
/answer 5f0f-49-0 allow
Control Request:
```yaml
id: 5f0f-4a-49
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
      nodes:
      - id: z2FhVL|v3y3zx
        excert: import java.util.Set;
      - id: SmallResultRule
        excert: "/**  * Allows small results of the tools allowed by {"
        docstring: "Allows small results of the tools allowed by {@link AllowedToolsRule}."
        children:
        - id: SmallResultRule.d86O60|CJR8lV
          excert: public static final String ID = SmallResultRule.class.getSimpleName();
        - id: SmallResultRule.SmallResultRule
          excert: public SmallResultRule()
        - id: SmallResultRule.SmallResultRule_2
          excert: public SmallResultRule(long maxLength)
        - id: SmallResultRule.id
          excert: public String id()
        - id: SmallResultRule.getDefaultSetting
          excert: public RuleSettings getDefaultSetting()
        - id: SmallResultRule.vote
          excert: public Vote vote(ApprovalContext ctx)
        - id: SmallResultRule.length
          excert: private static int length(ApprovalItem item)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover
        type: ClassOrInterfaceDeclaration
        excert: "public class AutoApprover {"
        children:
        - id: AutoApprover.TaT2M1|RdFmS2
          type: StatementGroup
          code: |2
              public static final int DEFAULT_MAX_CHAIN = 20;
              private final RuleRegistry registry;
              private final SessionTracker sessions = new SessionTracker();
              private final List<Consumer<Decision>> listeners = new CopyOnWriteArrayList<>();
              private volatile int maxChain = DEFAULT_MAX_CHAIN;
              private volatile String idleInstruction = "Continue with the task.";
              private volatile boolean enabled = true;
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
        - id: AutoApprover.removeListener
          type: MethodDeclaration
          code: |2
              public void removeListener(Consumer<Decision> listener) {
                listeners.remove(listener);
              }
        - id: AutoApprover.isEnabled
          type: MethodDeclaration
          code: |2
              public boolean isEnabled() {
                return enabled;
              }
        - id: AutoApprover.setEnabled
          type: MethodDeclaration
          code: |2
              public void setEnabled(boolean enabled) {
                this.enabled = enabled;
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
                if (!enabled)
                  return Decision.pass(ans.sessionId);
                SessionSnapshot session = sessions.record(ans);
                List<ApprovalItem> items = AnswerParser.parse(ans.answer);
                ApprovalItem item = items.size() == 1 ? items.get(0) : null;
                sessions.track(session.id(), items);
                ApprovalItem call = sessions.callFor(session.id(), item);
                ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, call, session, registry);
                Decision decision = decide(ctx, collect(ctx), depth);
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
        - id: AutoApprover.collect
          type: MethodDeclaration
          code: |2
              private List<RuleVote> collect(ApprovalContext ctx) {
                List<RuleVote> votes = new ArrayList<>();
                for (IApprovalRule rule : registry.rules()) {
                  if (!ctx.settings(rule.id()).isActive())
                    continue;
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
                boolean hold = false;
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
                    case Hold ->
                      {
                        hold = true;
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
                ApprovalResult result = resultFor(ctx.item(), deny, hold, allow);
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
                return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());
              }
        - id: AutoApprover.resultFor
          type: MethodDeclaration
          code: |2
              private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolean hold, boolean allow) {
                if (item == null || item.id().isBlank() || item.kind() == Kind.ToolResult)
                  return ApprovalResult.Pass;
                if (deny)
                  return ApprovalResult.Deny;
                if (hold || !allow)
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
      - id: AutoApprover.collect
        type: MethodDeclaration
        code: |2
            private List<RuleVote> collect(ApprovalContext ctx) {
              List<RuleVote> votes = new ArrayList<>();
              for (IApprovalRule rule : registry.rules()) {
                if (!ctx.settings(rule.id()).isActive())
                  continue;
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
              boolean hold = false;
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
                  case Hold ->
                    {
                      hold = true;
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
              ApprovalResult result = resultFor(ctx.item(), deny, hold, allow);
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
              return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());
            }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: 3sbhBT|DnCfAe
        excert: import java.util.List;
      - id: RuleRegistry
        excert: "/**  * Ordered registry of rules.  */ public class RuleRegistry {"
        docstring: Ordered registry of rules.
        children:
        - id: RuleRegistry.VUx1sY|HlFjwk
          excert: private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayL…
        - id: RuleRegistry.RuleRegistry
          excert: public RuleRegistry()
        - id: RuleRegistry.RuleRegistry_2
          excert: public RuleRegistry(List<IApprovalRule> rules)
        - id: RuleRegistry.register
          excert: public void register(IApprovalRule rule)
        - id: RuleRegistry.unregister
          excert: public boolean unregister(IApprovalRule rule)
        - id: RuleRegistry.rules
          excert: public List<IApprovalRule> rules()
        - id: RuleRegistry.find
          excert: public Optional<IApprovalRule> find(String id)
        - id: RuleRegistry.defaultSettings
          excert: public RuleSettings defaultSettings(String id)
          docstring: Default settings of the rule as an independent clone; empty settings for an
```
/answer 5f0f-4a-49 allow
Control Request:
```yaml
id: 5f0f-4b-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
    ids:
    - SmallResultRule
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    ids:
    - RuleRegistry.RuleRegistry
    - RuleRegistry.RuleRegistry_2
  reason: Code der Regel und Registry lesen
```
/answer 5f0f-4b-0 allow
ReasoningToken: 60
Control Request:
```yaml
id: 5f0f-4c-4b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
      nodes:
      - id: SmallResultRule
        type: ClassOrInterfaceDeclaration
        excert: "/**  * Allows small results of the tools allowed by {"
        docstring: "Allows small results of the tools allowed by {@link AllowedToolsRule}."
        children:
        - id: SmallResultRule.d86O60|CJR8lV
          type: StatementGroup
          code: "\tpublic static final String ID = SmallResultRule.class.getSimpleName();\n\tpublic static final String MAX_LENGTH_SETTING = \"maxLength\";\n\tpublic static final int MAX_LENGTH = 3000;\n\n\tprivate final RuleSettings defaults;\n"
        - id: SmallResultRule.SmallResultRule
          type: ConstructorDeclaration
          code: "\tpublic SmallResultRule() {\n\t\tthis(MAX_LENGTH);\n\t}\n"
        - id: SmallResultRule.SmallResultRule_2
          type: ConstructorDeclaration
          code: "\tpublic SmallResultRule(long maxLength) {\n\t\tthis.defaults = new RuleSettings().addNumber(MAX_LENGTH_SETTING, maxLength);\n\t}\n"
        - id: SmallResultRule.id
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n"
        - id: SmallResultRule.getDefaultSetting
          type: MethodDeclaration
          code: "\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n"
        - id: SmallResultRule.vote
          type: MethodDeclaration
          code: "\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tApprovalItem call = ctx.call();\n\t\tif (item == null || call == null || call.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlResult && item.kind() != Kind.ToolResult)\n\t\t\treturn Vote.abstain();\n\t\tSet<String> allowed = ctx.settings(AllowedToolsRule.ID).getStringSet(AllowedToolsRule.TOOLS);\n\t\tif (!allowed.contains(call.tool()))\n\t\t\treturn Vote.abstain();\n\t\treturn length(item) < ctx.settings(ID).getNumber(MAX_LENGTH_SETTING) ? Vote.allow() : Vote.abstain();\n\t}\n"
        - id: SmallResultRule.length
          type: MethodDeclaration
          code: "\tprivate static int length(ApprovalItem item) {\n\t\tJsonNode result = item.result();\n\t\tif (result.isMissingNode())\n\t\t\treturn item.yaml().length();\n\t\treturn result.isTextual() ? result.asText().length() : result.toString().length();\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: RuleRegistry.RuleRegistry
        type: ConstructorDeclaration
        code: |2
            public RuleRegistry() {
              register(new AllowedToolsRule());
              register(new SmallResultRule());
              register(new DeepPathReadRule());
              register(new DenyRootPathRule());
              register(new HoldMarkdownRule());
            }
      - id: RuleRegistry.RuleRegistry_2
        type: ConstructorDeclaration
        code: |2
            public RuleRegistry(List<IApprovalRule> rules) {
              this.rules.addAll(rules);
            }
```
/answer 5f0f-4c-4b allow
Control Request:
```yaml
id: 5f0f-4d-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: setMaxChain|maxChain|depth
    include:
    - '*.java'
    exclude:
    - target/**
    - build/**
    limit: 40
  reason: Verwendung von maxChain und depth finden
```
/answer 5f0f-4d-0 allow
Control Request:
```yaml
id: 5f0f-4e-4d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - private volatile int maxChain = DEFAULT_MAX_CHAIN;
        - "public void setMaxChain(int maxChain) {"
        - this.maxChain = maxChain;
        - "public Decision evaluate(AIAnswer ans, int depth) {"
        - "Decision decision = decide(ctx, collect(ctx), depth);"
        - "private Decision decide(ApprovalContext ctx, List<RuleVote> votes, int depth) {"
        - "if (depth >= maxChain) {"
        - warnings.add("auto-approve chain limit reached (" + maxChain + ")");
      - path: xy/ai/conscious/autoapprove/rules/PathArguments.java
        matches:
        - "/** Collects all string values (or string array elements) of fields named like {@code fields}, at any depth. */"
      - path: xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - "private PromptRun(IExecutor exec, Prompt preset, int depth) {"
        - this.turnDepth = depth;
        - "private void scheduleFollowUp(IExecutor exec, FollowUp followUp, int depth) {"
        - "exec.schedule(\"Auto Approve\", new PromptRun(exec, prompt, depth));"
        - "private Decision evaluate(AIAnswer ans, int depth) {"
        - "return approver.evaluate(ans, depth);"
```
/answer 5f0f-4e-4d allow
ReasoningToken: 215
Control Request:
```yaml
id: 5f0f-4f-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: new ApprovalContext\(|record ApprovalContext|class ApprovalContext|record Vote|class Vote|static Vote \w+\(
    include:
    - '*.java'
    exclude:
    - target/**
    intent: editing
  reason: ApprovalContext und Vote lokalisieren
```
/answer 5f0f-4f-0 allow
Control Request:
```yaml
id: 5f0f-50-4f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: AutoApprover.java
        nodes:
        - id: AutoApprover.evaluate
          matches:
          - "ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, call, session, registry);"
      - path: ApprovalContext.java
        nodes:
        - id: ApprovalContext
          matches:
          - "public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call, SessionSnapshot session, RuleRegistry registry) {"
      - path: rules/Vote.java
        nodes:
        - id: Vote
          matches:
          - "public record Vote(Verdict verdict, String reason, String hint, List<String> warnings, String instruction) {"
        - id: Vote.abstain
          matches:
          - "public static Vote abstain() {"
        - id: Vote.allow
          matches:
          - "public static Vote allow() {"
        - id: Vote.allow_2
          matches:
          - "public static Vote allow(String hint) {"
        - id: Vote.deny
          matches:
          - "public static Vote deny(String reason) {"
        - id: Vote.hold
          matches:
          - "public static Vote hold() {"
        - id: Vote.hold_2
          matches:
          - "public static Vote hold(String warning) {"
        - id: Vote.warning
          matches:
          - "public static Vote warning(String text) {"
        - id: Vote.instruction
          matches:
          - "public static Vote instruction(String text) {"
```
/answer 5f0f-50-4f allow
Control Request:
```yaml
id: 5f0f-51-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    ids:
    - ApprovalContext
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
    ids:
    - z2FhVL|v3y3zx
  reason: ApprovalContext und Imports lesen
```
/answer 5f0f-51-0 allow
Control Request:
```yaml
id: 5f0f-52-51
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      nodes:
      - id: ApprovalContext
        type: RecordDeclaration
        excert: "public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item…"
        children:
        - id: ApprovalContext.hasItem
          type: MethodDeclaration
          code: |2
              public boolean hasItem() {
                return item != null;
              }
        - id: ApprovalContext.settings
          type: MethodDeclaration
          code: |2
              public RuleSettings settings(String ruleId) {
                RuleSettings defaults = registry.defaultSettings(ruleId);
                RuleSettings override = prompt == null ? null : prompt.config.ruleSettings.get(ruleId);
                return defaults.overlay(override);
              }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
      nodes:
      - id: z2FhVL|v3y3zx
        type: ImportGroup
        code: |
          import java.util.Set;

          import com.fasterxml.jackson.databind.JsonNode;

          import xy.ai.conscious.autoapprove.ApprovalContext;
          import xy.ai.conscious.autoapprove.ApprovalItem;
          import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
```
/answer 5f0f-52-51 allow
Control Request:
```yaml
id: 5f0f-53-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: ApprovalContext\(|setMaxChain|HoldMarkdownRule
    include:
    - '*.java'
    exclude:
    - target/**
  reason: Weitere Konstruktoraufrufe prüfen
```
/answer 5f0f-53-0 allow
Control Request:
```yaml
id: 5f0f-54-53
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/RuleRegistry.java
        matches:
        - import xy.ai.conscious.autoapprove.rules.HoldMarkdownRule;
        - register(new HoldMarkdownRule());
      - path: xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - "public void setMaxChain(int maxChain) {"
        - "ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, call, session, registry);"
      - path: xy/ai/conscious/autoapprove/ApprovalContext.java
        matches:
        - "public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call, SessionSnapshot session, RuleRegistry registry) {"
      - path: xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
        matches:
        - "public class HoldMarkdownRule implements IApprovalRule {"
        - public static final String ID = HoldMarkdownRule.class.getSimpleName();
        - "public HoldMarkdownRule() {"
        - "public HoldMarkdownRule(Set<String> pathFields, Set<String> patternFields) {"
```
/answer 5f0f-54-53 allow
Control Request:
```yaml
id: 5f0f-55-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\n\n/**\n * Limits the number of consecutive automatic follow-up prompts. Once the chain depth reaches the\n * configured maximum, the rule holds and {@link xy.ai.conscious.autoapprove.AutoApprover} stops\n * producing follow-ups.\n */\npublic class ChainLimitRule implements IApprovalRule {\n\n\tpublic static final String ID = ChainLimitRule.class.getSimpleName();\n\tpublic static final String MAX_CHAIN_SETTING = \"maxChain\";\n\tpublic static final int MAX_CHAIN = 20;\n\n\tprivate final RuleSettings defaults;\n\n\tpublic ChainLimitRule() {\n\t\tthis(MAX_CHAIN);\n\t}\n\n\tpublic ChainLimitRule(long maxChain) {\n\t\tthis.defaults = new RuleSettings().addNumber(MAX_CHAIN_SETTING, maxChain);\n\t}\n\n\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n\n\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n\n\t/** The configured maximum chain length for the given context. */\n\tpublic static long maxChain(ApprovalContext ctx) {\n\t\treturn ctx.settings(ID).getNumber(MAX_CHAIN_SETTING).longValue();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\treturn ctx.depth() >= maxChain(ctx) ? Vote.hold() : Vote.abstain();\n\t}\n}\n"
  reason: Neue konfigurierbare Regel ChainLimitRule analog zu SmallResultRule anlegen
```
/answer 5f0f-55-0 allow
ReasoningToken: 599
Control Request:
```yaml
id: 5f0f-56-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    id: ApprovalContext
    old_text: "SessionSnapshot session, RuleRegistry registry) {"
    new_text: "SessionSnapshot session, RuleRegistry registry, int depth) {"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: RuleRegistry.RuleRegistry
    old_text: register(new SmallResultRule());
    new_text: |-
      register(new SmallResultRule());
          register(new ChainLimitRule());
    addImport:
    - import xy.ai.conscious.autoapprove.rules.ChainLimitRule;
  reason: Tiefe in den Kontext aufnehmen und Regel registrieren
```
/answer 5f0f-56-0 allow
Control Request:
```yaml
id: 5f0f-57-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.TaT2M1|RdFmS2
    old_text: |2
        public static final int DEFAULT_MAX_CHAIN = 20;
    new_text: ""
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.TaT2M1|RdFmS2
    old_text: |2
        private volatile int maxChain = DEFAULT_MAX_CHAIN;
    new_text: ""
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.evaluate
    old_text: "session, registry);"
    new_text: "session, registry, depth);"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    old_text: "    if (depth >= maxChain) {"
    new_text: "    if (chainLimited) {"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    old_text: (" + maxChain + ")"
    new_text: (" + ChainLimitRule.maxChain(ctx) + ")"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    old_text: |2
          boolean allow = false;
          for (RuleVote rv : votes) {
            Vote v = rv.vote();
    new_text: |2
          boolean allow = false;
          boolean chainLimited = false;
          for (RuleVote rv : votes) {
            Vote v = rv.vote();
            if (rv.ruleId().equals(ChainLimitRule.ID) && v.verdict() == Vote.Verdict.Hold)
              chainLimited = true;
    addImport:
    - import xy.ai.conscious.autoapprove.rules.ChainLimitRule;
  reason: AutoApprover auf die neue Regel umstellen
```
/answer 5f0f-57-0 allow
ReasoningToken: 94
Control Request:
```yaml
id: 5f0f-58-57
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.TaT2M1|RdFmS2
      new_id: AutoApprover.u5j4A4|Kz4hf9
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.evaluate
      new_id: AutoApprover.evaluate
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.decide
      new_id: AutoApprover.decide
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.decide
      new_id: AutoApprover.decide
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.decide
      new_id: AutoApprover.decide
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: AutoApprover.TaT2M1|RdFmS2
    notice: Imports/includes/use statements could not be verified. Check the affected files yourself.
```
/answer 5f0f-58-57 allow
Control Request:
```yaml
id: 5f0f-59-0
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.setMaxChain
  reason: setMaxChain wird nirgends verwendet und ist durch Regel-Einstellung ersetzt
```
/answer 5f0f-59-0 allow
Control Request:
```yaml
id: 5f0f-5a-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.u5j4A4|Kz4hf9
    old_text: |2
        private volatile int maxChain = DEFAULT_MAX_CHAIN;
    new_text: ""
  reason: Verbleibendes maxChain-Feld entfernen (vorheriger Versuch hatte veraltete ID)
```
/answer 5f0f-5a-0 allow
ReasoningToken: 32
Control Request:
```yaml
id: 5f0f-5b-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: record RuleVote|Verdict|getNumber
    include:
    - '*.java'
    exclude:
    - target/**
  reason: "RuleVote-Accessor, Verdict-Typ und getNumber-Rückgabetyp prüfen"
```
/answer 5f0f-5b-0 allow
Control Request:
```yaml
id: 5f0f-5c-5b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: AutoApprover.java
        matches:
        - if (rv.ruleId().equals(ChainLimitRule.ID) && v.verdict() == Vote.Verdict.Hold)
      - path: rules/Vote.java
        matches:
        - "public record Vote(Verdict verdict, String reason, String hint, List<String> warnings, String instruction) {"
        - "private static final Vote ABSTAIN = new Vote(Verdict.Abstain, null, null, null, null);"
        - "verdict = verdict == null ? Verdict.Abstain : verdict;"
        - "return new Vote(Verdict.Allow, null, hint, null, null);"
        - "return new Vote(Verdict.Deny, reason, null, null, null);"
        - "return new Vote(Verdict.Hold, null, null, null, null);"
        - "return new Vote(Verdict.Hold, null, null, List.of(warning), null);"
        - "return new Vote(Verdict.Abstain, null, null, List.of(text), null);"
        - "return new Vote(Verdict.Abstain, null, null, null, text);"
      - path: rules/SmallResultRule.java
        matches:
        - "return length(item) < ctx.settings(ID).getNumber(MAX_LENGTH_SETTING) ? Vote.allow() : Vote.abstain();"
      - path: rules/ChainLimitRule.java
        matches:
        - return ctx.settings(ID).getNumber(MAX_CHAIN_SETTING).longValue();
      - path: rules/DeepPathReadRule.java
        matches:
        - "return length(item) < settings.getNumber(MAX_LENGTH_SETTING) ? Vote.allow() : Vote.abstain();"
        - long minDepth = settings.getNumber(MIN_DEPTH_SETTING);
      - path: rules/Verdict.java
        matches:
        - "public enum Verdict {"
      warning: "Limit of 15 matches reached (total 17); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 5f0f-5c-5b allow
Control Request:
```yaml
id: 5f0f-5d-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
    id: ChainLimitRule
    old_text: getNumber(MAX_CHAIN_SETTING).longValue()
    new_text: getNumber(MAX_CHAIN_SETTING)
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    old_text: v.verdict() == Vote.Verdict.Hold
    new_text: v.verdict() == Verdict.Hold
    addImport:
    - import xy.ai.conscious.autoapprove.rules.Verdict;
  reason: Typfehler korrigieren
```
/answer 5f0f-5d-0 allow
ReasoningToken: 65
Control Request:
```yaml
id: 5f0f-5e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: record RuleVote|^import .*(Verdict|ChainLimitRule)
    include:
    - '*.java'
    exclude:
    - target/**
  reason: RuleVote-Felder und Imports prüfen
```
/answer 5f0f-5e-0 allow
Control Request:
```yaml
id: 5f0f-5f-5e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/RuleVote.java
        matches:
        - "public record RuleVote(String ruleId, Vote vote) {"
```
/answer 5f0f-5f-5e allow
ReasoningToken: 58
Control Request:
```yaml
id: 5f0f-60-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: import .*(Verdict|ChainLimitRule|SmallResultRule|rules\.)
    include:
    - AutoApprover.java
    - RuleRegistry.java
    intent: editing
  reason: "Prüfen, ob Imports gesetzt wurden"
```
/answer 5f0f-60-0 allow
Control Request:
```yaml
id: 5f0f-61-60
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: RuleRegistry.java
        nodes:
        - id: 3sbhBT|DnCfAe
          matches:
          - import xy.ai.conscious.autoapprove.rules.HoldMarkdownRule;
          - import xy.ai.conscious.autoapprove.rules.RuleSettings;
          - import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
          - import xy.ai.conscious.autoapprove.rules.DeepPathReadRule;
          - import xy.ai.conscious.autoapprove.rules.DenyRootPathRule;
          - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
          - import xy.ai.conscious.autoapprove.rules.SmallResultRule;
      - path: AutoApprover.java
        nodes:
        - id: Zpzs7d|sxLd1L
          matches:
          - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
          - import xy.ai.conscious.autoapprove.rules.Vote;
```
/answer 5f0f-61-60 allow
Text:
 Die Imports fehlen tatsächlich. Ich füge sie jetzt hinzu.
Control Request:
```yaml
id: 5f0f-62-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: 3sbhBT|DnCfAe
    position: after
    source: |
      import xy.ai.conscious.autoapprove.rules.ChainLimitRule;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: Zpzs7d|sxLd1L
    position: after
    source: |
      import xy.ai.conscious.autoapprove.rules.ChainLimitRule;
      import xy.ai.conscious.autoapprove.rules.Verdict;
  reason: Fehlende Imports ergänzen
```
/answer 5f0f-62-0 allow
Control Request:
```yaml
id: 5f0f-63-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
  reason: Kompilierbarkeit prüfen
```
/answer 5f0f-63-0 allow
Result Stats: id=a1cab16b-c523-4125-9037-1c32c77eff57, total: 37921, in: 24, out: 8078, reason: 0, read: 222076, write: 29819, start: 1794513113576, end: 1794513121667, time: 8037, total: 64726, io: 0, control: 54, sent: 0, recv: 172
Die Kettenbegrenzung ist jetzt eine eigene, konfigurierbare Regel `ChainLimitRule`. Alle vier geänderten Dateien bestehen `ast_validate`. Einen vollständigen Build und Tests habe ich nicht ausgeführt.

**Neu: `rules/ChainLimitRule.java`**, aufgebaut wie `SmallResultRule`:
- **Einstellung:** `maxChain` mit Standardwert 20, gesetzt über `RuleSettings().addNumber(...)`. Über die Rule-Settings des Prompts lässt sich der Wert pro Prompt überschreiben.
- **Konstruktoren:** Es gibt `ChainLimitRule()` und `ChainLimitRule(long maxChain)`.
- **Abstimmung:** `vote()` gibt `Vote.hold()` zurück, sobald `ctx.depth() >= maxChain`, sonst `abstain`.
- **Hilfsmethode:** `maxChain(ctx)` liefert den gültigen Wert für die Warnmeldung.

**Geändert:**
- **`ApprovalContext`:** Das Record hat ein neues Feld `int depth`, damit Regeln die Kettentiefe sehen.
- **`RuleRegistry`:** Die Regel wird im Standardkonstruktor direkt nach `SmallResultRule` registriert.
- **`AutoApprover`:**
  - `DEFAULT_MAX_CHAIN`, das Feld `maxChain` und `setMaxChain()` sind entfernt. Eine Suche im Quellcode fand keine anderen Aufrufer.
  - `evaluate()` übergibt `depth` an den Kontext.
  - `decide()` prüft nicht mehr `depth >= maxChain`. Es erkennt stattdessen ein Hold von `ChainLimitRule`. Dann erzwingt es wie vorher `Pass`, erzeugt keinen Follow-up und warnt weiterhin nur, wenn ein Ergebnis oder eine Anweisung unterdrückt wurde.

**Beachten:**
- Das Hold allein stoppt Follow-ups nicht, weil Idle-Anweisungen trotzdem eine Kette fortsetzen würden. Deshalb prüft `decide()` das Hold dieser Regel zusätzlich. Das entspricht dem bisherigen harten Limit.
- Wird die Regel deaktiviert, gibt es kein Kettenlimit mehr. Vorher galt immer eins.
Das Hold reicht, eine sonderbehandlung nur dieser Regel ist nicht notwendig
SystemInit:  id=a1cab16b-c523-4125-9037-1c32c77eff57, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 181
Control Request:
```yaml
id: 5f0f-64-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  name: decide
  reason: Aktuellen Stand von decide lesen
```
/answer 5f0f-64-0 allow
Control Request:
```yaml
id: 5f0f-65-64
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover.decide
        code: |2
            private Decision decide(ApprovalContext ctx, List<RuleVote> votes, int depth) {
              List<String> reasons = new ArrayList<>();
              List<String> hints = new ArrayList<>();
              List<String> warnings = new ArrayList<>();
              List<String> instructions = new ArrayList<>();
              boolean deny = false;
              boolean hold = false;
              boolean allow = false;
              boolean chainLimited = false;
              for (RuleVote rv : votes) {
                Vote v = rv.vote();
                if (rv.ruleId().equals(ChainLimitRule.ID) && v.verdict() == Verdict.Hold)
                  chainLimited = true;
                warnings.addAll(v.warnings());
                addNonBlank(instructions, v.instruction());
                switch(v.verdict()) {
                  case Deny ->
                    {
                      deny = true;
                      addNonBlank(reasons, v.reason());
                    }
                  case Hold ->
                    {
                      hold = true;
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
              ApprovalResult result = resultFor(ctx.item(), deny, hold, allow);
              String reason = String.join("; ", reasons);
              String hint = String.join("; ", hints);
              String instruction = String.join("\n", instructions);
              FollowUp followUp = null;
              if (chainLimited) {
                if (result != ApprovalResult.Pass || !instruction.isEmpty())
                  warnings.add("auto-approve chain limit reached (" + ChainLimitRule.maxChain(ctx) + ")");
                result = ApprovalResult.Pass;
              } else {
                followUp = followUpFor(ctx, result, reason, hint, instruction);
                if (followUp == null)
                  result = ApprovalResult.Pass;
              }
              return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());
            }
```
/answer 5f0f-65-64 allow
Control Request:
```yaml
id: 5f0f-66-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    start_marker: '    FollowUp followUp = null;'
    end_marker: |2-
              result = ApprovalResult.Pass;
          }
    content: |2-
          FollowUp followUp = followUpFor(ctx, result, reason, hint, instruction);
          if (followUp == null)
            result = ApprovalResult.Pass;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    start_marker: '    boolean chainLimited = false;'
    end_marker: '      warnings.addAll(v.warnings());'
    content: |2-
          for (RuleVote rv : votes) {
            Vote v = rv.vote();
            warnings.addAll(v.warnings());
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    start_marker: "List<RuleVote> votes, int depth)"
    end_marker: "List<RuleVote> votes, int depth)"
    content: List<RuleVote> votes)
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.evaluate
    start_marker: "collect(ctx), depth)"
    end_marker: "collect(ctx), depth)"
    content: collect(ctx))
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
    id: ChainLimitRule
    start_marker: "\t/** The configured maximum"
    end_marker: ": Vote.abstain();\n\t}"
    content: "\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tlong max = ctx.settings(ID).getNumber(MAX_CHAIN_SETTING);\n\t\treturn ctx.depth() >= max ? Vote.hold(\"auto-approve chain limit reached (\" + max + \")\") : Vote.abstain();\n\t}"
  reason: Sonderbehandlung entfernen; Warnung kommt aus der Regel
```
/answer 5f0f-66-0 allow
ReasoningToken: 63
Control Request:
```yaml
id: 5f0f-67-66
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.decide
      new_id: AutoApprover.decide
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.decide
      new_id: AutoApprover.decide
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.evaluate
      new_id: AutoApprover.evaluate
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      error: End marker is ambiguous – found 2 occurrences in node.
      id: AutoApprover.decide
      reason: ambiguous
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
      error: Start marker not found in node.
      id: ChainLimitRule
      reason: not_found
      position: start
      next_step: reread_node
```
/answer 5f0f-67-66 allow
Control Request:
```yaml
id: 5f0f-68-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  node_type: ImportDeclaration
  regexp: ChainLimitRule|Verdict
  reason: Nicht mehr benötigte Imports lokalisieren
```
/answer 5f0f-68-0 allow
Control Request:
```yaml
id: 5f0f-69-68
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes: []
```
/answer 5f0f-69-68 allow
Text:
 Zwei Edits sind fehlgeschlagen. Ich lese den aktuellen Stand und korrigiere.
Control Request:
```yaml
id: 5f0f-6a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: chainLimited|ChainLimitRule|Verdict|FollowUp followUp = |decide\(|maxChain
    include:
    - AutoApprover.java
    - ChainLimitRule.java
    intent: editing
  reason: Aktuellen Stand der betroffenen Stellen prüfen
```
/answer 5f0f-6a-0 allow
Control Request:
```yaml
id: 5f0f-6b-6a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: AutoApprover.java
        nodes:
        - id: 6RUKko|pKGyXf
          matches:
          - import xy.ai.conscious.autoapprove.rules.ChainLimitRule;
          - import xy.ai.conscious.autoapprove.rules.Verdict;
        - id: AutoApprover.evaluate
          matches:
          - "Decision decision = decide(ctx, collect(ctx));"
        - id: AutoApprover.onIdle
          matches:
          - "FollowUp followUp = new FollowUp(origin, selection(instruction.lines().toList()));"
        - id: AutoApprover.decide
          matches:
          - "private Decision decide(ApprovalContext ctx, List<RuleVote> votes) {"
          - FollowUp followUp = null;
          - "if (chainLimited) {"
          - warnings.add("auto-approve chain limit reached (" + ChainLimitRule.maxChain(ctx) + ")");
      - path: rules/ChainLimitRule.java
        nodes:
        - id: ChainLimitRule
          matches:
          - "public class ChainLimitRule implements IApprovalRule {"
        - id: ChainLimitRule.d86O60|3uMcpA
          matches:
          - public static final String ID = ChainLimitRule.class.getSimpleName();
          - public static final String MAX_CHAIN_SETTING = "maxChain";
        - id: ChainLimitRule.ChainLimitRule
          matches:
          - "public ChainLimitRule() {"
        - id: ChainLimitRule.ChainLimitRule_2
          matches:
          - "public ChainLimitRule(long maxChain) {"
          - "this.defaults = new RuleSettings().addNumber(MAX_CHAIN_SETTING, maxChain);"
        - id: ChainLimitRule.maxChain
          matches:
          - "public static long maxChain(ApprovalContext ctx) {"
      warning: "Limit of 15 matches reached (total 16); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 5f0f-6b-6a allow
ReasoningToken: 117
Control Request:
```yaml
id: 5f0f-6c-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    start_marker: FollowUp followUp = null;
    end_marker: |-
      result = ApprovalResult.Pass;
          }
          return
    content: |-
      FollowUp followUp = followUpFor(ctx, result, reason, hint, instruction);
          if (followUp == null)
            result = ApprovalResult.Pass;
          return
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
    id: ChainLimitRule.maxChain
    start_marker: public static long maxChain
    end_marker: "ctx) {"
    content: "public static long maxChain(ApprovalContext ctx) {"
  reason: Follow-up-Block vereinfachen
```
/answer 5f0f-6c-0 allow
Control Request:
```yaml
id: 5f0f-6d-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
    ids:
    - ChainLimitRule
  reason: Regel-Code neu lesen
```
/answer 5f0f-6d-0 allow
Control Request:
```yaml
id: 5f0f-6e-6d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
      nodes:
      - id: ChainLimitRule
        type: ClassOrInterfaceDeclaration
        excert: "public class ChainLimitRule implements IApprovalRule {"
        children:
        - id: ChainLimitRule.d86O60|3uMcpA
          type: StatementGroup
          code: |2
              public static final String ID = ChainLimitRule.class.getSimpleName();
              public static final String MAX_CHAIN_SETTING = "maxChain";
              public static final int MAX_CHAIN = 20;
              private final RuleSettings defaults;
        - id: ChainLimitRule.ChainLimitRule
          type: ConstructorDeclaration
          code: |2
              public ChainLimitRule() {
                this(MAX_CHAIN);
              }
        - id: ChainLimitRule.ChainLimitRule_2
          type: ConstructorDeclaration
          code: |2
              public ChainLimitRule(long maxChain) {
                this.defaults = new RuleSettings().addNumber(MAX_CHAIN_SETTING, maxChain);
              }
        - id: ChainLimitRule.id
          type: MethodDeclaration
          code: |2
              @Override
              public String id() {
                return ID;
              }
        - id: ChainLimitRule.getDefaultSetting
          type: MethodDeclaration
          code: |2
              @Override
              public RuleSettings getDefaultSetting() {
                return defaults.clone();
              }
        - id: ChainLimitRule.maxChain
          type: MethodDeclaration
          code: |2
              public static long maxChain(ApprovalContext ctx) {
                return ctx.settings(ID).getNumber(MAX_CHAIN_SETTING);
              }
        - id: ChainLimitRule.vote
          type: MethodDeclaration
          code: |2
              @Override
              public Vote vote(ApprovalContext ctx) {
                return ctx.depth() >= maxChain(ctx) ? Vote.hold() : Vote.abstain();
              }
```
/answer 5f0f-6e-6d allow
Text:
 Jetzt kommt die Warnung aus der Regel. Danach entferne ich die Hilfsmethode und die ungenutzten Imports.
Control Request:
```yaml
id: 5f0f-6f-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
    id: ChainLimitRule.vote
    source: |2
        @Override
        public Vote vote(ApprovalContext ctx) {
          long max = ctx.settings(ID).getNumber(MAX_CHAIN_SETTING);
          return ctx.depth() >= max ? Vote.hold("auto-approve chain limit reached (" + max + ")") : Vote.abstain();
        }
  reason: Warnung in die Regel verlagern
```
/answer 5f0f-6f-0 allow
Control Request:
```yaml
id: 5f0f-70-0
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
    id: ChainLimitRule.maxChain
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    removeImport:
    - import xy.ai.conscious.autoapprove.rules.ChainLimitRule;
    - import xy.ai.conscious.autoapprove.rules.Verdict;
    id: 6RUKko|pKGyXf
  reason: Hilfsmethode und ungenutzte Imports entfernen
```
/answer 5f0f-70-0 allow
Control Request:
```yaml
id: 5f0f-71-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/ChainLimitRule.java
  reason: Kompilierbarkeit prüfen
```
/answer 5f0f-71-0 allow
ReasoningToken: 65
Control Request:
```yaml
id: 5f0f-72-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ChainLimitRule|Verdict|chainLimited|maxChain
    include:
    - AutoApprover.java
  reason: Reste in AutoApprover prüfen
```
/answer 5f0f-72-0 allow
Control Request:
```yaml
id: 5f0f-73-72
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
```
/answer 5f0f-73-72 allow
Result Stats: id=a1cab16b-c523-4125-9037-1c32c77eff57, total: 50879, in: 38, out: 12268, reason: 0, read: 458247, write: 38573, start: 1794513278804, end: 1794513283581, time: 4724, total: 100769, io: 0, control: 53, sent: 0, recv: 101
Die Sonderbehandlung ist entfernt. `AutoApprover` kennt `ChainLimitRule` jetzt nicht mehr. `ast_validate` bestätigt, dass beide Dateien kompilieren. Einen vollständigen Build habe ich nicht ausgeführt.

- **`AutoApprover.decide()`:** Die Tiefenprüfung und der Sonderfall sind weg. Der Parameter `depth` fehlt jetzt auch bei `decide()` und seinem Aufruf in `evaluate()`. Ein Hold wird wie bei jeder anderen Regel behandelt. Die beiden ungenutzten Imports sind gelöscht.
- **`ChainLimitRule.vote()`:** Ab `depth >= maxChain` kommt `Vote.hold("auto-approve chain limit reached (N)")`, sonst `abstain`. Die Warnung stammt damit aus der Regel. Die Hilfsmethode `maxChain(ctx)` habe ich entfernt, weil nichts sie mehr aufruft.

Die Warnung erscheint jetzt immer, sobald das Limit erreicht ist. Vorher kam sie nur, wenn ein Ergebnis oder eine Anweisung unterdrückt wurde.

Außerdem unterdrückt das Hold keine Anweisungen mehr. Eine Idle-Anweisung kann also auch nach Erreichen des Limits noch einen Follow-up auslösen. Vorher stoppte das harte Limit auch das.
