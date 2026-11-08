Erstelle eine Regel Analog zu `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java`.
Diese soll Antworten passieren lassen, die wie in `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java` (Tools) erlaubt werden jedoch in Argumenten auf Markdown Dateien zielen.
Hierzu muss ein neues `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Verdict.java` einführt werden mit einen höheren Prio als Allow aber unterhalb von Deny.

Mögliche Feldnamen sind "path", "paths" (Liste), "include" (Regexp, verdacht/inhalt auf ".md" reicht).
Das Ziel ist das Abfangen von Tool Calls die zu Prompts und Dokumentationen führen.
SystemInit:  id=b7ac6d2e-184a-479b-9578-a3d807a1a63d, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 146e-1-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    recursive: true
  reason: Explore autoapprove package structure
```
/answer 146e-1-0 allow
Control Request:
```yaml
id: 146e-2-1
result:
  structuredContent:
    results:
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
      - DeepPathReadRule.java
      - DenyRootPathRule.java
      - IApprovalRule.java
      - PathArguments.java
      - SmallResultRule.java
      - Verdict.java
      - Vote.java
```
/answer 146e-2-1 allow
Control Request:
```yaml
id: 146e-3-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
  reason: Read the reference rules
```
/answer 146e-3-0 allow
Control Request:
```yaml
id: 146e-4-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
      nodes:
      - id: 09A9xJ|Ot0X0x
        signature: import java.nio.file.Path;
      - id: DenyRootPathRule
        signature: /**  * Denies calls of path-based tools if any path-like argument points at the…
        docstring: Denies calls of path-based tools if any path-like argument points at the root d…
        children:
        - id: DenyRootPathRule.KkcWlY|1v1nwE
          signature: public static final String REASON = "Don't use the root directory.";
        - id: DenyRootPathRule.DenyRootPathRule
          signature: public DenyRootPathRule()
        - id: DenyRootPathRule.DenyRootPathRule_2
          signature: "public DenyRootPathRule(Set<String> tools, Set<String> pathFields, Path cwd)"
        - id: DenyRootPathRule.vote
          signature: public Vote vote(ApprovalContext ctx)
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
      nodes:
      - id: z2FhVL|9B9F9F
        signature: import java.util.Set;
      - id: AllowedToolsRule
        signature: "/**  * Allows every call of the tools listed in {"
        docstring: "Allows every call of the tools listed in {@link #ALLOWED_TOOLS}, regardless"
        children:
        - id: AllowedToolsRule.9ooLSz|b44gaE
          signature: "public static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\", \"ast_…"
        - id: AllowedToolsRule.AllowedToolsRule
          signature: public AllowedToolsRule()
        - id: AllowedToolsRule.AllowedToolsRule_2
          signature: public AllowedToolsRule(String... toolNames)
        - id: AllowedToolsRule.vote
          signature: public Vote vote(ApprovalContext ctx)
```
/answer 146e-4-3 allow
Text:
 Verdict.java existiert bereits. Ich lese die relevanten Dateien.
Control Request:
```yaml
id: 146e-5-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Verdict.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/PathArguments.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
  regexp: .
  node_type: Module|CompilationUnit|class|enum|interface|record
  reason: Read full sources
```
/answer 146e-5-0 allow
Control Request:
```yaml
id: 146e-6-5
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
      nodes:
      - id: DenyRootPathRule
        type: ClassOrInterfaceDeclaration
        code: "public class DenyRootPathRule implements IApprovalRule {\n\n\tpublic static final String REASON = \"Don't use the root directory.\";\n\n\tprivate final Set<String> tools;\n\tprivate final Set<String> pathFields;\n\tprivate final Path cwd;\n\n\tpublic DenyRootPathRule() {\n\t\tthis(Set.of(DeepPathReadRule.TOOLS), Set.of(DeepPathReadRule.PATH_FIELDS),\n\t\t\t\tPath.of(\"\").toAbsolutePath().normalize());\n\t}\n\n\tpublic DenyRootPathRule(Set<String> tools, Set<String> pathFields, Path cwd) {\n\t\tthis.tools = Set.copyOf(tools);\n\t\tthis.pathFields = Set.copyOf(pathFields);\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null || !tools.contains(item.tool()))\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tfor (String value : PathArguments.collect(item.arguments(), pathFields)) {\n\t\t\tPath path = PathArguments.resolve(cwd, value);\n\t\t\tif (path != null && PathArguments.isRoot(path))\n\t\t\t\treturn Vote.deny(REASON);\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
      nodes:
      - id: AllowedToolsRule
        type: ClassOrInterfaceDeclaration
        code: "public class AllowedToolsRule implements IApprovalRule {\n\n\tpublic static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\", \"ast_validate\", \"run_command\", \"ast_find\" };\n\n\tprivate final Set<String> allowed;\n\n\tpublic AllowedToolsRule() {\n\t\tthis(ALLOWED_TOOLS);\n\t}\n\n\tpublic AllowedToolsRule(String... toolNames) {\n\t\tthis.allowed = Set.of(toolNames);\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\treturn allowed.contains(item.tool()) ? Vote.allow() : Vote.abstain();\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Verdict.java
      nodes:
      - id: Verdict
        type: EnumDeclaration
        code: "public enum Verdict {\n\tAllow, Deny, Abstain\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/PathArguments.java
      nodes:
      - id: PathArguments
        type: ClassOrInterfaceDeclaration
        code: "public class PathArguments {\n\n\tprivate PathArguments() {\n\t}\n\n\t/** Collects all string values (or string array elements) of fields named like {@code fields}, at any depth. */\n\tstatic List<String> collect(JsonNode node, Set<String> fields) {\n\t\tList<String> out = new ArrayList<>();\n\t\tcollect(node, fields, out);\n\t\treturn out;\n\t}\n\n\t@SuppressWarnings(\"deprecation\")\n\tprivate static void collect(JsonNode node, Set<String> fields, List<String> out) {\n\t\tif (node == null)\n\t\t\treturn;\n\t\tif (node.isObject()) {\n\t\t\tnode.fields().forEachRemaining(e -> {\n\t\t\t\tif (fields.contains(e.getKey()))\n\t\t\t\t\taddValues(e.getValue(), out);\n\t\t\t\tcollect(e.getValue(), fields, out);\n\t\t\t});\n\t\t} else if (node.isArray()) {\n\t\t\tnode.forEach(child -> collect(child, fields, out));\n\t\t}\n\t}\n\n\tprivate static void addValues(JsonNode value, List<String> out) {\n\t\tif (value.isTextual())\n\t\t\tout.add(value.asText());\n\t\telse if (value.isArray())\n\t\t\tvalue.forEach(v -> addValues(v, out));\n\t}\n\n\t/** Resolves against {@code cwd} and normalizes; null if the value is not a valid path. */\n\tstatic Path resolve(Path cwd, String value) {\n\t\ttry {\n\t\t\treturn cwd.resolve(value).normalize();\n\t\t} catch (InvalidPathException e) {\n\t\t\treturn null;\n\t\t}\n\t}\n\n\tstatic boolean isRoot(Path path) {\n\t\treturn path.isAbsolute() && path.getNameCount() == 0;\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
      nodes:
      - id: Vote
        type: RecordDeclaration
        code: "public record Vote(Verdict verdict, String reason, String hint, List<String> warnings, String instruction) {\n\n\tprivate static final Vote ABSTAIN = new Vote(Verdict.Abstain, null, null, null, null);\n\n\tpublic Vote {\n\t\tverdict = verdict == null ? Verdict.Abstain : verdict;\n\t\treason = reason == null ? \"\" : reason;\n\t\thint = hint == null ? \"\" : hint;\n\t\twarnings = warnings == null ? List.of() : List.copyOf(warnings);\n\t\tinstruction = instruction == null ? \"\" : instruction;\n\t}\n\n\tpublic static Vote abstain() {\n\t\treturn ABSTAIN;\n\t}\n\n\tpublic static Vote allow() {\n\t\treturn allow(null);\n\t}\n\n\tpublic static Vote allow(String hint) {\n\t\treturn new Vote(Verdict.Allow, null, hint, null, null);\n\t}\n\n\tpublic static Vote deny(String reason) {\n\t\treturn new Vote(Verdict.Deny, reason, null, null, null);\n\t}\n\n\t/** Abstains but adds a warning to the answer. */\n\tpublic static Vote warning(String text) {\n\t\treturn new Vote(Verdict.Abstain, null, null, List.of(text), null);\n\t}\n\n\t/** Abstains but requests a follow-up prompt with the given instruction. */\n\tpublic static Vote instruction(String text) {\n\t\treturn new Vote(Verdict.Abstain, null, null, null, text);\n\t}\n\n\tpublic Vote withWarning(String text) {\n\t\tList<String> all = new ArrayList<>(warnings);\n\t\tall.add(text);\n\t\treturn new Vote(verdict, reason, hint, all, instruction);\n\t}\n}\n"
```
/answer 146e-6-5 allow
Control Request:
```yaml
id: 146e-7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: Verdict\.|DenyRootPathRule
    include:
    - '*.java'
    exclude:
    - build/**
    limit: 50
  reason: Find usages of Verdict and registration of rules
```
/answer 146e-7-0 allow
Control Request:
```yaml
id: 146e-8-7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/RuleRegistry.java
        matches:
        - import xy.ai.conscious.autoapprove.rules.DenyRootPathRule;
        - register(new DenyRootPathRule());
      - path: xy/ai/conscious/autoapprove/rules/Vote.java
        matches:
        - "private static final Vote ABSTAIN = new Vote(Verdict.Abstain, null, null, null, null);"
        - "verdict = verdict == null ? Verdict.Abstain : verdict;"
        - "return new Vote(Verdict.Allow, null, hint, null, null);"
        - "return new Vote(Verdict.Deny, reason, null, null, null);"
        - "return new Vote(Verdict.Abstain, null, null, List.of(text), null);"
        - "return new Vote(Verdict.Abstain, null, null, null, text);"
      - path: xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
        matches:
        - "public class DenyRootPathRule implements IApprovalRule {"
        - "public DenyRootPathRule() {"
        - "public DenyRootPathRule(Set<String> tools, Set<String> pathFields, Path cwd) {"
```
/answer 146e-8-7 allow
Control Request:
```yaml
id: 146e-9-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbenchconnector/src
    pattern: Verdict|Deny\b|Abstain
    include:
    - '*.java'
    - '*.md'
    exclude:
    - rules/Vote.java
    - build/**
    - bin/**
    limit: 50
  reason: Find consumers of verdicts
```
/answer 146e-9-0 allow
ReasoningToken: 225
Control Request:
```yaml
id: 146e-a-9
result:
  structuredContent:
    errors:
    - directory:
      - /home/user/xyan/xy.ai.workbenchconnector/src
      pattern: Verdict|Deny\b|Abstain
      error: Directory not found or not a directory.
```
/answer 146e-a-9 deny Suche tiefer
Control Request:
```yaml
id: 146e-b-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
  reason: Read registry and DeepPathReadRule constants
```
/answer 146e-b-0 allow
Control Request:
```yaml
id: 146e-c-b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: 3sbhBT|qHscGq
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
      nodes:
      - id: AKHURJ|uCLMlL
        signature: import java.nio.file.Path;
      - id: DeepPathReadRule
        signature: "/**  * Allows calls of path-based read tools (e.g. search, list) and their resu…"
        docstring: "Allows calls of path-based read tools (e.g. search, list) and their results"
        children:
        - id: DeepPathReadRule.KkcWlY|PLZV8L
          signature: "public static final String[] TOOLS = { \"search\", \"list\" };"
        - id: DeepPathReadRule.DeepPathReadRule
          signature: public DeepPathReadRule()
        - id: DeepPathReadRule.DeepPathReadRule_2
          signature: "public DeepPathReadRule(Set<String> tools, Set<String> pathFields, int minDepth…"
        - id: DeepPathReadRule.vote
          signature: public Vote vote(ApprovalContext ctx)
        - id: DeepPathReadRule.allDeep
          signature: private boolean allDeep(ApprovalItem call)
        - id: DeepPathReadRule.length
          signature: private static int length(ApprovalItem item)
```
/answer 146e-c-b allow
ReasoningToken: 34
Control Request:
```yaml
id: 146e-d-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: Verdict|\.Deny|Abstain|PATH_FIELDS|DenyRootPathRule|AllowedToolsRule
    include:
    - '*.java'
    - '*.md'
    exclude:
    - Vote.java
    limit: 50
  reason: Deeper search for verdict consumers and rule registrations
```
/answer 146e-d-0 allow
Control Request:
```yaml
id: 146e-e-d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/RuleRegistry.java
        matches:
        - import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
        - import xy.ai.conscious.autoapprove.rules.DenyRootPathRule;
        - register(new AllowedToolsRule());
        - register(new DenyRootPathRule());
      - path: xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - "* Aggregation: Deny beats Allow beats Abstain. Reasons, hints, warnings and"
        - case Abstain ->
        - return ApprovalResult.Deny;
      - path: xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
        matches:
        - "public class DenyRootPathRule implements IApprovalRule {"
        - "public DenyRootPathRule() {"
        - "this(Set.of(DeepPathReadRule.TOOLS), Set.of(DeepPathReadRule.PATH_FIELDS),"
        - "public DenyRootPathRule(Set<String> tools, Set<String> pathFields, Path cwd) {"
      - path: xy/ai/conscious/autoapprove/rules/SmallResultRule.java
        matches:
        - "* {@link AllowedToolsRule#ALLOWED_TOOLS}. Larger results are left to the operator."
        - this(AllowedToolsRule.ALLOWED_TOOLS);
      - path: xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
        matches:
        - "public class AllowedToolsRule implements IApprovalRule {"
        - "public AllowedToolsRule() {"
        - "public AllowedToolsRule(String... toolNames) {"
      - path: xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
        matches:
        - "public static final String[] PATH_FIELDS = { \"path\", \"directory\" };"
        - "this(Set.of(TOOLS), Set.of(PATH_FIELDS), MIN_DEPTH, Path.of(\"\").toAbsolutePath().normalize());"
      - path: xy/ai/conscious/autoapprove/rules/Verdict.java
        matches:
        - "public enum Verdict {"
        - "Allow, Deny, Abstain"
      - path: xy/ai/conscious/commands/AnswerCommand.java
        matches:
        - return Action.Deny;
```
/answer 146e-e-d allow
Control Request:
```yaml
id: 146e-f-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalResult.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  regexp: .
  node_type: ClassOrInterfaceDeclaration|EnumDeclaration|RecordDeclaration
  reason: Read aggregation logic and result type
```
/answer 146e-f-0 allow
Control Request:
```yaml
id: 146e-10-f
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover
        type: ClassOrInterfaceDeclaration
        code: |
          public class AutoApprover {
            public static final int DEFAULT_MAX_CHAIN = 20;
            private final RuleRegistry registry;
            private final SessionTracker sessions = new SessionTracker();
            private final List<Consumer<Decision>> listeners = new CopyOnWriteArrayList<>();
            private volatile int maxChain = DEFAULT_MAX_CHAIN;
            private volatile String idleInstruction = "Continue with the task.";

            public AutoApprover() {
              this(new RuleRegistry());
            }

            public AutoApprover(RuleRegistry registry) {
              this.registry = registry;
            }

            public RuleRegistry rules() {
              return registry;
            }

            public void addListener(Consumer<Decision> listener) {
              listeners.add(listener);
            }

            public void removeListener(Consumer<Decision> listener) {
              listeners.remove(listener);
            }

            /**
             * Maximum number of consecutive automatic follow-up prompts.
             */
            public void setMaxChain(int maxChain) {
              this.maxChain = maxChain;
            }

            public void setIdleInstruction(String instruction) {
              this.idleInstruction = instruction == null ? "" : instruction;
            }

            public void forget(String sessionId) {
              sessions.forget(sessionId);
            }

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

            public Optional<FollowUp> onIdle(String sessionId) {
              Prompt origin = sessions.lastPrompt(sessionId);
              String instruction = idleInstruction;
              if (origin == null || instruction.isBlank())
                return Optional.empty();
              FollowUp followUp = new FollowUp(origin, selection(instruction.lines().toList()));
              publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp, 0));
              return Optional.of(followUp);
            }

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
              return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());
            }

            private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolean allow) {
              if (item == null || item.id().isBlank() || item.kind() == Kind.ToolResult)
                return ApprovalResult.Pass;
              if (deny)
                return ApprovalResult.Deny;
              if (!allow)
                return ApprovalResult.Pass;
              return item.kind() == Kind.ToolCall ? ApprovalResult.Call : ApprovalResult.Allow;
            }

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

            /**
             * Yaml block followed by the command line, as the operator would select it.
             */
            private static List<String> withBlock(ApprovalItem item, String commandLine) {
              List<String> lines = new ArrayList<>();
              lines.add("```yaml");
              item.yaml().lines().forEach(lines::add);
              lines.add("```");
              lines.add(commandLine);
              return lines;
            }

            private static String denial(ApprovalItem item, String reason) {
              String tool = item.tool().isBlank() ? "" : " (" + item.tool() + ")";
              String why = oneLine(reason);
              return "Tool call " + item.id() + tool + " was denied" + (why.isEmpty() ? "." : ": " + why);
            }

            private static String command(String base, String extra) {
              String text = oneLine(extra);
              return text.isEmpty() ? base : base + " " + text;
            }

            private static String oneLine(String s) {
              return s == null ? "" : s.replaceAll("\\s+", " ").strip();
            }

            private static Selection selection(List<String> lines) {
              return new Selection(lines.toArray(String[]::new), null);
            }

            private static void addNonBlank(List<String> list, String text) {
              if (text != null && !text.isBlank())
                list.add(text);
            }

            /**
             * Warnings go in front so the command line stays the last line of the answer.
             */
            private static void enrich(AIAnswer ans, List<String> warnings) {
              if (warnings.isEmpty())
                return;
              StringBuilder sb = new StringBuilder();
              for (String w : warnings) sb.append("//warning: ").append(oneLine(w)).append('\n');
              ans.answer = sb + (ans.answer == null ? "" : ans.answer);
            }

            private void publish(Decision decision) {
              for (Consumer<Decision> l : listeners) {
                try {
                  l.accept(decision);
                } catch (RuntimeException e) {
                  LOG.error(e.getMessage(), e);
                }
              }
            }
          }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalResult.java
      nodes:
      - id: ApprovalResult
        type: EnumDeclaration
        code: "public enum ApprovalResult {\n\t/** No decision; the answer is not a tool call/result or no rule decided. */\n\tPass,\n\t/** Tool call or result of a control request was rejected. */\n\tDeny,\n\t/** Tool call or result of a control request was approved. */\n\tAllow,\n\t/** Tool call without control request was approved and is executed. */\n\tCall\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: RuleRegistry
        type: ClassOrInterfaceDeclaration
        code: |
          public class RuleRegistry {
            private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayList<>();

            public RuleRegistry() {
              register(new AllowedToolsRule());
              register(new SmallResultRule());
              register(new DeepPathReadRule());
              register(new DenyRootPathRule());
            }

            public RuleRegistry(List<IApprovalRule> rules) {
              this.rules.addAll(rules);
            }

            public void register(IApprovalRule rule) {
              rules.addIfAbsent(Objects.requireNonNull(rule));
            }

            public boolean unregister(IApprovalRule rule) {
              return rules.remove(rule);
            }

            public List<IApprovalRule> rules() {
              return rules;
            }
          }
```
/answer 146e-10-f allow
ReasoningToken: 633
Control Request:
```yaml
id: 146e-11-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Verdict.java
    id: Verdict
    source: "public enum Verdict {\n\tAllow,\n\t/** Outranks {@link #Allow}, but is outranked by {@link #Deny}: the call is left to the operator. */\n\tHold, Deny, Abstain\n}"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
    id: Vote.deny
    source: "public static Vote deny(String reason) {\n\t\treturn new Vote(Verdict.Deny, reason, null, null, null);\n\t}\n\n\t/** Revokes automatic approval and adds a warning to the answer. */\n\tpublic static Vote hold(String warning) {\n\t\treturn new Vote(Verdict.Hold, null, null, List.of(warning), null);\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.decide
    source: |-
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
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover.resultFor
    source: |-
      private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolean hold, boolean allow) {
          if (item == null || item.id().isBlank() || item.kind() == Kind.ToolResult)
            return ApprovalResult.Pass;
          if (deny)
            return ApprovalResult.Deny;
          if (hold || !allow)
            return ApprovalResult.Pass;
          return item.kind() == Kind.ToolCall ? ApprovalResult.Call : ApprovalResult.Allow;
        }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
    id: ""
    source: ""
  reason: "Add Hold verdict, Vote.hold, and aggregation in AutoApprover"
```
/answer 146e-11-0 allow
Control Request:
```yaml
id: 146e-12-11
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Verdict.java
      result: success
      id: Verdict
      new_id: Verdict
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
      result: success
      id: Vote.deny
      new_id: Vote.deny
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.decide
      new_id: AutoApprover.decide
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      result: success
      id: AutoApprover.resultFor
      new_id: AutoApprover.resultFor
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
      error: File not found.
      id: ""
```
/answer 146e-12-11 allow
ReasoningToken: 44
Control Request:
```yaml
id: 146e-13-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
  node_type: Import|Package
  reason: Get exact imports and package
```
/answer 146e-13-0 allow
Control Request:
```yaml
id: 146e-14-13
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
      nodes:
      - id: 09A9xJ|Ot0X0x
        type: ImportGroup
        code: |
          import java.nio.file.Path;
          import java.util.Set;

          import xy.ai.conscious.autoapprove.ApprovalContext;
          import xy.ai.conscious.autoapprove.ApprovalItem;
          import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
```
/answer 146e-14-13 allow
Control Request:
```yaml
id: 146e-15-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.util.Locale;\nimport java.util.Set;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\n/**\n * Holds back calls of auto-approved tools that target markdown files (prompts, documentation), so they\n * are not approved automatically.\n */\npublic class HoldMarkdownRule implements IApprovalRule {\n\n\tpublic static final String WARNING = \"Markdown file targeted, not approved automatically.\";\n\tpublic static final String[] PATH_FIELDS = { \"path\", \"paths\" };\n\tpublic static final String[] PATTERN_FIELDS = { \"include\" };\n\n\tprivate static final String EXTENSION = \".md\";\n\n\tprivate final Set<String> tools;\n\tprivate final Set<String> pathFields;\n\tprivate final Set<String> patternFields;\n\n\tpublic HoldMarkdownRule() {\n\t\tthis(Set.of(AllowedToolsRule.ALLOWED_TOOLS), Set.of(PATH_FIELDS), Set.of(PATTERN_FIELDS));\n\t}\n\n\tpublic HoldMarkdownRule(Set<String> tools, Set<String> pathFields, Set<String> patternFields) {\n\t\tthis.tools = Set.copyOf(tools);\n\t\tthis.pathFields = Set.copyOf(pathFields);\n\t\tthis.patternFields = Set.copyOf(patternFields);\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null || !tools.contains(item.tool()))\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tfor (String value : PathArguments.collect(item.arguments(), pathFields)) {\n\t\t\tif (value.toLowerCase(Locale.ROOT).endsWith(EXTENSION))\n\t\t\t\treturn Vote.hold(WARNING);\n\t\t}\n\t\tfor (String value : PathArguments.collect(item.arguments(), patternFields)) {\n\t\t\tif (value.toLowerCase(Locale.ROOT).contains(EXTENSION))\n\t\t\t\treturn Vote.hold(WARNING);\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n}\n"
  reason: Create the new HoldMarkdownRule
```
/answer 146e-15-0 allow
Control Request:
```yaml
id: 146e-16-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: RuleRegistry.RuleRegistry
    old_text: register(new DenyRootPathRule());
    new_text: |-
      register(new DenyRootPathRule());
          register(new HoldMarkdownRule());
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: 3sbhBT|qHscGq
    old_text: import java.util.List;
    new_text: |-
      import java.util.List;
      import xy.ai.conscious.autoapprove.rules.HoldMarkdownRule;
  reason: Register the rule and import it
```
/answer 146e-16-0 allow
Control Request:
```yaml
id: 146e-17-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Verdict.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  reason: Check syntax of changed files
```
/answer 146e-17-0 allow
Result Stats: id=b7ac6d2e-184a-479b-9578-a3d807a1a63d, total: 32197, in: 20, out: 7237, reason: 0, read: 141985, write: 24940, start: 1794164045693, end: 1794164051898, time: 6140, total: 61402, io: 0, control: 65, sent: 0, recv: 164
Ich habe die neue Regel `HoldMarkdownRule` angelegt, dazu ein neues Verdict `Hold` und die nötigen Anpassungen am Rest. Die Dateien haben die Syntaxprüfung bestanden. Kompiliert oder getestet habe ich nicht, und Tests dazu habe ich nicht geschrieben.

`Verdict.java` gab es schon (`Allow, Deny, Abstain`). Ich habe es deshalb nicht neu eingeführt, sondern erweitert.

- **`Verdict.java`:** Neu ist `Hold`. Es sticht `Allow` aus, wird aber von `Deny` übertrumpft.
- **`Vote.java`:** `Vote.hold(String warning)` ist neu. Die Warnung steht dann als `//warning:` vor der Antwort.
- **`AutoApprover.java`:** `decide` kennt jetzt `Hold`. In `resultFor` führt `Deny` weiterhin zu `Deny`. `Hold` ergibt `Pass`, der Aufruf wird also nicht automatisch freigegeben und bleibt beim Operator. Ohne `Hold` bleibt das Verhalten wie bisher.
- **`HoldMarkdownRule.java`:** Die Regel greift bei den Tools aus `AllowedToolsRule.ALLOWED_TOOLS`, wenn der Aufruf ein Tool-Call oder Control-Call ist. Sie meldet `Hold`, wenn:
  - `path` oder `paths` (Liste) auf `.md` endet, ohne Beachtung der Groß-/Kleinschreibung, oder
  - `include` den Text `.md` enthält. Das deckt zum Beispiel `*.md` und `\.md$` ab.

  Sonst enthält sie sich. Die Argumente werden wie bei `DenyRootPathRule` über `PathArguments.collect` gelesen, also in beliebiger Tiefe.
- **`RuleRegistry.java`:** Die Regel ist als letzte im Standardsatz registriert.

Ein `include` wie `(md|txt)` ohne Punkt wird nicht erkannt.
