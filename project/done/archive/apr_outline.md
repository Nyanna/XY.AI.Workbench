Implementiere eine Regel `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java` und registriere sie, die auf Basis einer Array Konstante ein Liste von Tool-Calls ("toolName") pauschal erlaubt.

Beispiel:
Control Request:
```yaml
id: 506e-6
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/docs/scripts/build_html.py
```

- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java`
SystemInit:  id=904fc321-bcd6-496e-b161-58df0d241e34, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 3186-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
  reason: Understand the AutoApprover and the IApprovalRule interface
```
/answer 3186-1 allow
Control Request:
```yaml
id: 3186-2
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
      nodes:
      - id: H4HWeP|XqIL50
        signature: import xy.ai.conscious.autoapprove.ApprovalContext;
      - id: IApprovalRule
        signature: /**  * A voter. Every registered rule is evaluated for every answer and decides…
        docstring: A voter. Every registered rule is evaluated for every answer and decides on
        children:
        - id: IApprovalRule.id
          signature: String id()
        - id: IApprovalRule.vote
          signature: abstract Vote vote(ApprovalContext ctx)
```
/answer 3186-2 allow
Control Request:
```yaml
id: 3186-3
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: IApprovalRule
    include:
    - '*.java'
  reason: Find existing rule implementations
```
/answer 3186-3 allow
Control Request:
```yaml
id: 3186-4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: RuleRegistry.java
        matches:
        - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
        - private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayList<>();
        - "public RuleRegistry register(IApprovalRule rule) {"
        - "public boolean unregister(IApprovalRule rule) {"
        - "public List<IApprovalRule> rules() {"
      - path: AutoApprover.java
        matches:
        - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
        - "for (IApprovalRule rule : registry.rules()) {"
      - path: rules/IApprovalRule.java
        matches:
        - "public interface IApprovalRule {"
```
/answer 3186-4 allow
Control Request:
```yaml
id: 3186-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover.AutoApprover
    - AutoApprover.AutoApprover_2
    - AutoApprover.collect
  reason: Read interface and AutoApprover constructors
```
/answer 3186-5 allow
ReasoningToken: 60
Control Request:
```yaml
id: 3186-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
      nodes:
      - id: H4HWeP|XqIL50
        signature: import xy.ai.conscious.autoapprove.ApprovalContext;
      - id: IApprovalRule
        signature: /**  * A voter. Every registered rule is evaluated for every answer and decides…
        docstring: A voter. Every registered rule is evaluated for every answer and decides on
        children:
        - id: IApprovalRule.id
          signature: String id()
        - id: IApprovalRule.vote
          signature: abstract Vote vote(ApprovalContext ctx)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover.AutoApprover
        type: ConstructorDeclaration
        code: "\tpublic AutoApprover() {\n\t\tthis(new RuleRegistry());\n\t}\n"
      - id: AutoApprover.AutoApprover_2
        type: ConstructorDeclaration
        code: "\tpublic AutoApprover(RuleRegistry registry) {\n\t\tthis.registry = registry;\n\t}\n"
      - id: AutoApprover.collect
        type: MethodDeclaration
        code: "\tprivate List<RuleVote> collect(ApprovalContext ctx) {\n\t\tList<RuleVote> votes = new ArrayList<>();\n\t\tfor (IApprovalRule rule : registry.rules()) {\n\t\t\ttry {\n\t\t\t\tVote vote = rule.vote(ctx);\n\t\t\t\tvotes.add(new RuleVote(rule.id(), vote == null ? Vote.abstain() : vote));\n\t\t\t} catch (RuntimeException e) {\n\t\t\t\tLOG.error(\"Approval rule failed: \" + rule.id(), e);\n\t\t\t}\n\t\t}\n\t\treturn votes;\n\t}\n"
```
/answer 3186-6 allow
Control Request:
```yaml
id: 3186-7
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: .*
    include:
    - '*.java'
    limit: 5
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: new AutoApprover\(|\.register\(
    include:
    - '*.java'
  reason: Find files in the package and registration sites
```
/answer 3186-7 allow
Control Request:
```yaml
id: 3186-8
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
      warning: "Limit of 5 matches reached (total 560); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - private final AutoApprover approver = new AutoApprover();
```
/answer 3186-8 allow
ReasoningToken: 51
Control Request:
```yaml
id: 3186-9
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ^(public |final )*(class|interface|record|enum)
    include:
    - '*.java'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness
    pattern: approver|AutoApprover
    include:
    - PromptHandler.java
  reason: List types in package and usage of approver in PromptHandler
```
/answer 3186-9 allow
Control Request:
```yaml
id: 3186-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: RuleRegistry.java
        matches:
        - "public class RuleRegistry {"
      - path: AutoApprover.java
        matches:
        - "public class AutoApprover {"
      - path: Decision.java
        matches:
        - "public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,"
      - path: ApprovalContext.java
        matches:
        - "public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, SessionSnapshot session) {"
      - path: SessionTracker.java
        matches:
        - "final class SessionTracker {"
      - path: AnswerParser.java
        matches:
        - "public class AnswerParser {"
      - path: FollowUp.java
        matches:
        - "public record FollowUp(Prompt origin, Selection selection) {"
      - path: ApprovalResult.java
        matches:
        - "public enum ApprovalResult {"
      - path: rules/Vote.java
        matches:
        - "public record Vote(Verdict verdict, String reason, String hint, List<String> warnings, String instruction) {"
      - path: rules/IApprovalRule.java
        matches:
        - "public interface IApprovalRule {"
      - path: rules/Verdict.java
        matches:
        - "public enum Verdict {"
      - path: SessionSnapshot.java
        matches:
        - "public record SessionSnapshot(String id, int turn, TokenStats tokens, Instant lastActivity) {"
      - path: RuleVote.java
        matches:
        - "public record RuleVote(String ruleId, Vote vote) {"
      - path: ApprovalItem.java
        matches:
        - "public record ApprovalItem(Kind kind, String id, String tool, JsonNode data, String yaml) {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness
      matches:
      - path: PromptHandler.java
        matches:
        - import xy.ai.conscious.autoapprove.AutoApprover;
        - private final AutoApprover approver = new AutoApprover();
        - "public AutoApprover approver() {"
        - return approver;
        - "approver.onIdle(sessionId).ifPresent(f -> scheduleFollowUp(exec, f, 1));"
        - "return approver.evaluate(ans, depth);"
```
/answer 3186-a allow
Control Request:
```yaml
id: 3186-b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Verdict.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
  reason: "Read Vote, Verdict, ApprovalItem, RuleRegistry and interface fully"
```
/answer 3186-b allow
ReasoningToken: 28
Control Request:
```yaml
id: 3186-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
      nodes:
      - id: UbJ2xB|ENVO48
        signature: import java.util.ArrayList;
      - id: Vote
        signature: /**  * Result of a single rule. Rules only decide; the approver executes. Besid…
        docstring: Result of a single rule. Rules only decide; the approver executes. Besides the
        children:
        - id: Vote.2F7xfC|B2J1EK
          signature: "private static final Vote ABSTAIN = new Vote(Verdict.Abstain, null, null, null,…"
        - id: Vote.abstain
          signature: public static Vote abstain()
        - id: Vote.allow
          signature: public static Vote allow()
        - id: Vote.allow_2
          signature: public static Vote allow(String hint)
        - id: Vote.deny
          signature: public static Vote deny(String reason)
        - id: Vote.warning
          signature: public static Vote warning(String text)
          docstring: Abstains but adds a warning to the answer.
        - id: Vote.instruction
          signature: public static Vote instruction(String text)
          docstring: Abstains but requests a follow-up prompt with the given instruction.
        - id: Vote.withWarning
          signature: public Vote withWarning(String text)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Verdict.java
      nodes:
      - id: Verdict
        signature: "public enum Verdict {"
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: KNcHmE|GPQi5U
        signature: import java.util.List;
      - id: RuleRegistry
        signature: "/**  * Ordered registry of rules.  */ public class RuleRegistry {"
        docstring: Ordered registry of rules.
        children:
        - id: RuleRegistry.VUx1sY|HlFjwk
          signature: private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayL…
        - id: RuleRegistry.register
          signature: public RuleRegistry register(IApprovalRule rule)
        - id: RuleRegistry.unregister
          signature: public boolean unregister(IApprovalRule rule)
        - id: RuleRegistry.rules
          signature: public List<IApprovalRule> rules()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
      nodes:
      - id: H4HWeP|XqIL50
        signature: import xy.ai.conscious.autoapprove.ApprovalContext;
      - id: IApprovalRule
        signature: /**  * A voter. Every registered rule is evaluated for every answer and decides…
        docstring: A voter. Every registered rule is evaluated for every answer and decides on
        children:
        - id: IApprovalRule.id
          signature: String id()
        - id: IApprovalRule.vote
          signature: abstract Vote vote(ApprovalContext ctx)
```
/answer 3186-c allow
Control Request:
```yaml
id: 3186-d
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: approver\(\)|rules\(\)\.register|RuleRegistry
    include:
    - '*.java'
    exclude:
    - '**/target/**'
  reason: Find where rules are registered
```
/answer 3186-d allow
Control Request:
```yaml
id: 3186-e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
        matches:
        - "public class RuleRegistry {"
        - "public RuleRegistry register(IApprovalRule rule) {"
      - path: connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - private final RuleRegistry registry;
        - this(new RuleRegistry());
        - "public AutoApprover(RuleRegistry registry) {"
        - "public RuleRegistry rules() {"
      - path: connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - "public AutoApprover approver() {"
```
/answer 3186-e allow
Control Request:
```yaml
id: 3186-f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
    ids:
    - IApprovalRule
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
    ids:
    - ApprovalItem
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover.rules
    - AutoApprover.decide
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
    ids:
    - Vote.allow_2
  reason: "Read source of interface, ApprovalItem, decide, and Vote.allow"
```
/answer 3186-f allow
ReasoningToken: 47
Control Request:
```yaml
id: 3186-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
      nodes:
      - id: IApprovalRule
        type: ClassOrInterfaceDeclaration
        signature: /**  * A voter. Every registered rule is evaluated for every answer and decides…
        docstring: A voter. Every registered rule is evaluated for every answer and decides on
        children:
        - id: IApprovalRule.id
          type: MethodDeclaration
          code: "\tdefault String id() {\n\t\treturn getClass().getSimpleName();\n\t}\n"
        - id: IApprovalRule.vote
          type: MethodDeclaration
          code: "\tVote vote(ApprovalContext ctx);\n"
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover.rules
        type: MethodDeclaration
        code: "\tpublic RuleRegistry rules() {\n\t\treturn registry;\n\t}\n"
      - id: AutoApprover.decide
        type: MethodDeclaration
        code: "\tprivate Decision decide(ApprovalContext ctx, List<RuleVote> votes, int depth) {\n\t\tList<String> reasons = new ArrayList<>();\n\t\tList<String> hints = new ArrayList<>();\n\t\tList<String> warnings = new ArrayList<>();\n\t\tList<String> instructions = new ArrayList<>();\n\t\tboolean deny = false;\n\t\tboolean allow = false;\n\t\tfor (RuleVote rv : votes) {\n\t\t\tVote v = rv.vote();\n\t\t\twarnings.addAll(v.warnings());\n\t\t\taddNonBlank(instructions, v.instruction());\n\t\t\tswitch (v.verdict()) {\n\t\t\tcase Deny -> {\n\t\t\t\tdeny = true;\n\t\t\t\taddNonBlank(reasons, v.reason());\n\t\t\t}\n\t\t\tcase Allow -> {\n\t\t\t\tallow = true;\n\t\t\t\taddNonBlank(hints, v.hint());\n\t\t\t}\n\t\t\tcase Abstain -> {\n\t\t\t}\n\t\t\t}\n\t\t}\n\t\tApprovalResult result = resultFor(ctx.item(), deny, allow);\n\t\tString reason = String.join(\"; \", reasons);\n\t\tString hint = String.join(\"; \", hints);\n\t\tString instruction = String.join(\"\\n\", instructions);\n\n\t\tFollowUp followUp = null;\n\t\tif (depth >= maxChain) {\n\t\t\tif (result != ApprovalResult.Pass || !instruction.isEmpty())\n\t\t\t\twarnings.add(\"auto-approve chain limit reached (\" + maxChain + \")\");\n\t\t\tresult = ApprovalResult.Pass;\n\t\t} else {\n\t\t\tfollowUp = followUpFor(ctx, result, reason, hint, instruction);\n\t\t\tif (followUp == null)\n\t\t\t\tresult = ApprovalResult.Pass;\n\t\t}\n\t\treturn new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes,\n\t\t\t\tfollowUp);\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
      nodes:
      - id: Vote.allow_2
        type: MethodDeclaration
        code: "\tpublic static Vote allow(String hint) {\n\t\treturn new Vote(Verdict.Allow, null, hint, null, null);\n\t}\n"
```
/answer 3186-10 allow
Control Request:
```yaml
id: 3186-11
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness
    pattern: PromptHandler\(
    include:
    - PromptHandler.java
    intent: editing
  reason: Find constructor of PromptHandler for registration
```
/answer 3186-11 deny Registriere es ind er Default registry im Approver
Control Request:
```yaml
id: 3186-12
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.util.Set;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\n/**\n * Allows every call of the tools listed in {@link #ALLOWED_TOOLS}, regardless\n * of their arguments.\n */\npublic class AllowedToolsRule implements IApprovalRule {\n\n\tpublic static final String[] ALLOWED_TOOLS = { \"read_file\" };\n\n\tprivate final Set<String> allowed;\n\n\tpublic AllowedToolsRule() {\n\t\tthis(ALLOWED_TOOLS);\n\t}\n\n\tpublic AllowedToolsRule(String... toolNames) {\n\t\tthis.allowed = Set.of(toolNames);\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\treturn allowed.contains(item.tool()) ? Vote.allow() : Vote.abstain();\n\t}\n}\n"
  reason: Create the new rule implementing IApprovalRule
```
/answer 3186-12 allow
ReasoningToken: 320
Control Request:
```yaml
id: 3186-13
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  node_type: ImportDeclaration
  text: RuleRegistry
  reason: Check imports of AutoApprover (same package as RuleRegistry; need rule import)
```
/answer 3186-13 allow
Control Request:
```yaml
id: 3186-14
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes: []
```
/answer 3186-14 allow
Text:
 Now register it in the default constructor and add the import.
Control Request:
```yaml
id: 3186-15
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.AutoApprover
    old_text: this(new RuleRegistry());
    new_text: this(new RuleRegistry().register(new AllowedToolsRule()));
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: H4HWeP|XqIL50
    old_text: import xy.ai.conscious.autoapprove.rules.IApprovalRule;
    new_text: |-
      import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
      import xy.ai.conscious.autoapprove.rules.IApprovalRule;
  reason: Register rule in default registry
```
/answer 3186-15 allow
Control Request:
```yaml
id: 3186-16
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.AutoApprover
      new_id: AutoApprover.AutoApprover
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: H4HWeP|XqIL50
```
/answer 3186-16 allow
ReasoningToken: 27
Control Request:
```yaml
id: 3186-17
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  text: import xy.ai.conscious.autoapprove.rules.IApprovalRule
  reason: Get node id of the IApprovalRule import
```
/answer 3186-17 allow
Control Request:
```yaml
id: 3186-18
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: Zpzs7d|sxLd1L
        code: |
          import java.util.ArrayList;
          import java.util.List;
          import java.util.Optional;
          import java.util.concurrent.CopyOnWriteArrayList;
          import java.util.function.Consumer;
          import xy.ai.conscious.api.IActiveEditor.Selection;
          import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
          import xy.ai.conscious.autoapprove.rules.IApprovalRule;
          import xy.ai.conscious.autoapprove.rules.Vote;
          import xy.ai.conscious.commands.AnswerCommand;
          import xy.ai.conscious.commands.CallCommand;
          import xy.ai.conscious.connector.harness.Prompt;
          import xy.ai.conscious.core.LOG;
          import xy.ai.conscious.model.AIAnswer;
```
/answer 3186-18 allow
Control Request:
```yaml
id: 3186-19
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: Zpzs7d|sxLd1L
    old_text: import xy.ai.conscious.autoapprove.rules.IApprovalRule;
    new_text: |-
      import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
      import xy.ai.conscious.autoapprove.rules.IApprovalRule;
  reason: Add import for the new rule
```
/answer 3186-19 allow
Control Request:
```yaml
id: 3186-1a
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
  reason: Verify both files compile
```
/answer 3186-1a allow
Result Stats: id=904fc321-bcd6-496e-b161-58df0d241e34, total: 25222, in: 22, out: 5401, reason: 0, read: 139351, write: 19799, start: 1794070994611, end: 1794070999564, time: 4889, total: 56034, io: 0, control: 64, sent: 0, recv: 122
Ich habe die Regel angelegt und in der Default-Registry des `AutoApprover` registriert. Beide Dateien haben die Syntaxprüfung bestanden. Kompiliert oder getestet habe ich nicht.

- **Neue Regel:** `rules/AllowedToolsRule.java` implementiert `IApprovalRule`. `IApprovalRule.java` selbst ist das Interface, deshalb habe ich die Implementierung in eine neue Datei gelegt.
- **Konstante:** `ALLOWED_TOOLS` ist ein `String[]` und enthält vorerst nur `read_file`, wie in deinem Beispiel. Weitere Tool-Namen trägst du dort ein.
- **Verhalten:** Bei einem `ControlCall` oder `ToolCall` mit einem Tool-Namen aus der Liste stimmt die Regel mit `Vote.allow()` zu, unabhängig von den Argumenten. In allen anderen Fällen enthält sie sich, auch bei Results. Ein Veto einer anderen Regel überstimmt sie weiterhin, weil `AutoApprover.decide` jedes `Deny` höher gewichtet.
- **Registrierung:** Der No-Arg-Konstruktor von `AutoApprover` ruft jetzt `this(new RuleRegistry().register(new AllowedToolsRule()))` auf. `PromptHandler` nutzt diesen Konstruktor und bekommt die Regel deshalb automatisch. Wer `AutoApprover(RuleRegistry)` mit eigener Registry nutzt, bekommt sie nicht.
- **Zusätzlicher Konstruktor:** `AllowedToolsRule(String...)` erlaubt eine abweichende Tool-Liste, etwa für Tests.
