Ich benötige eine Regel Analog zu `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java` die einen Tool-Call mit PASS markiert (Also dem Operator zur Entscheidung überlässt), wenn ein File oder Pfadparameter außerhalb des Projektverzeichnisses liegt.
SystemInit:  id=40737270-8d66-4e39-96bb-61a352febf84, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: d545-1-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: DeepPathReadRule
    include:
    - '*.java'
    - '*.xml'
    - '*.properties'
    - '*.json'
    exclude:
    - target/**
    - build/**
  reason: Bestehende Regel DeepPathReadRule lesen und Nachbarn im Paket auflisten
```
/answer d545-1-0 allow
Control Request:
```yaml
id: d545-2-1
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
        matches:
        - path: RuleRegistry.java
          matches:
          - import xy.ai.conscious.autoapprove.rules.DeepPathReadRule;
          - register(new DeepPathReadRule());
        - path: rules/DenyRootPathRule.java
          matches:
          - "* {@link DeepPathReadRule}."
          - RuleSettings deep = ctx.settings(DeepPathReadRule.ID);
          - if (!deep.getStringSet(DeepPathReadRule.TOOLS_SETTING).contains(item.tool()))
          - Set<String> pathFields = deep.getStringSet(DeepPathReadRule.PATH_FIELDS_SETTING);
        - path: rules/DeepPathReadRule.java
          matches:
          - "public class DeepPathReadRule implements IApprovalRule {"
          - public static final String ID = DeepPathReadRule.class.getSimpleName();
          - "public DeepPathReadRule() {"
          - "public DeepPathReadRule(Set<String> tools, Set<String> pathFields, int minDepth, Path cwd) {"
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
        nodes:
        - id: vsV4Dz|pJd0WY
          excert: import java.nio.file.Path;
        - id: DeepPathReadRule
          excert: "public class DeepPathReadRule implements IApprovalRule {"
          children:
          - id: DeepPathReadRule.KkcWlY|HbusYF
            excert: public static final String ID = DeepPathReadRule.class.getSimpleName();
          - id: DeepPathReadRule.DeepPathReadRule
            excert: public DeepPathReadRule()
          - id: DeepPathReadRule.DeepPathReadRule_2
            excert: "public DeepPathReadRule(Set<String> tools, Set<String> pathFields, int minDepth…"
          - id: DeepPathReadRule.id
            excert: public String id()
          - id: DeepPathReadRule.getDefaultSetting
            excert: public RuleSettings getDefaultSetting()
          - id: DeepPathReadRule.vote
            excert: public Vote vote(ApprovalContext ctx)
          - id: DeepPathReadRule.allDeep
            excert: "private boolean allDeep(ApprovalItem call, RuleSettings settings)"
          - id: DeepPathReadRule.length
            excert: private static int length(ApprovalItem item)
```
/answer d545-2-1 allow
Control Request:
```yaml
id: d545-3-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
    ids:
    - vsV4Dz|pJd0WY
    - DeepPathReadRule
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    node_type: .
  reason: "Vollständigen Quelltext der Regel, DenyRootPathRule und RuleRegistry lesen"
```
/answer d545-3-0 allow
Control Request:
```yaml
id: d545-4-3
result:
  structuredContent:
    find:
      results:
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
        - id: DenyRootPathRule
          type: ClassOrInterfaceDeclaration
          excert: "/**  * Denies the root directory for the tools and path fields of  * {"
          docstring: Denies the root directory for the tools and path fields of
          children:
          - id: DenyRootPathRule.KkcWlY|V39Vko
            type: StatementGroup
            code: "\tpublic static final String ID = DenyRootPathRule.class.getSimpleName();\n\tpublic static final String REASON = \"Don't use the root directory.\";\n\n\tprivate final RuleSettings defaults = new RuleSettings();\n\tprivate final Path cwd;\n"
          - id: DenyRootPathRule.DenyRootPathRule
            type: ConstructorDeclaration
            code: "\tpublic DenyRootPathRule() {\n\t\tthis(Path.of(\"\").toAbsolutePath().normalize());\n\t}\n"
          - id: DenyRootPathRule.DenyRootPathRule_2
            type: ConstructorDeclaration
            code: "\tpublic DenyRootPathRule(Path cwd) {\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n"
          - id: DenyRootPathRule.id
            type: MethodDeclaration
            code: "\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n"
          - id: DenyRootPathRule.getDefaultSetting
            type: MethodDeclaration
            code: "\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n"
          - id: DenyRootPathRule.vote
            type: MethodDeclaration
            code: "\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tRuleSettings deep = ctx.settings(DeepPathReadRule.ID);\n\t\tif (!deep.getStringSet(DeepPathReadRule.TOOLS_SETTING).contains(item.tool()))\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tSet<String> pathFields = deep.getStringSet(DeepPathReadRule.PATH_FIELDS_SETTING);\n\t\tfor (String value : PathArguments.collect(item.arguments(), pathFields)) {\n\t\t\tPath path = PathArguments.resolve(cwd, value);\n\t\t\tif (path != null && PathArguments.isRoot(path))\n\t\t\t\treturn Vote.deny(REASON);\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n"
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
        nodes:
        - id: rBWicj|46f5Dd
          type: ImportGroup
          code: |
            import java.util.List;
            import xy.ai.conscious.autoapprove.rules.HoldMarkdownRule;
            import java.util.Objects;
            import java.util.Optional;
            import xy.ai.conscious.autoapprove.rules.RuleSettings;
            import java.util.concurrent.CopyOnWriteArrayList;
            import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
            import xy.ai.conscious.autoapprove.rules.DeepPathReadRule;
            import xy.ai.conscious.autoapprove.rules.DenyRootPathRule;
            import xy.ai.conscious.autoapprove.rules.IApprovalRule;
            import xy.ai.conscious.autoapprove.rules.SmallResultRule;
            import xy.ai.conscious.autoapprove.rules.ChainLimitRule;
            import xy.ai.conscious.autoapprove.rules.DenyOwnPromptFileRule;
        - id: RuleRegistry
          type: ClassOrInterfaceDeclaration
          excert: "/**  * Ordered registry of rules.  */ public class RuleRegistry {"
          docstring: Ordered registry of rules.
          children:
          - id: RuleRegistry.VUx1sY|HlFjwk
            type: StatementGroup
            code: |2
                private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayList<>();
          - id: RuleRegistry.RuleRegistry
            type: ConstructorDeclaration
            code: |2
                public RuleRegistry() {
                  register(new AllowedToolsRule());
                  register(new SmallResultRule());
                  register(new ChainLimitRule());
                  register(new DeepPathReadRule());
                  register(new DenyRootPathRule());
                  register(new DenyOwnPromptFileRule());
                  register(new HoldMarkdownRule());
                }
          - id: RuleRegistry.RuleRegistry_2
            type: ConstructorDeclaration
            code: |2
                public RuleRegistry(List<IApprovalRule> rules) {
                  this.rules.addAll(rules);
                }
          - id: RuleRegistry.register
            type: MethodDeclaration
            code: |2
                public void register(IApprovalRule rule) {
                  rules.addIfAbsent(Objects.requireNonNull(rule));
                }
          - id: RuleRegistry.unregister
            type: MethodDeclaration
            code: |2
                public boolean unregister(IApprovalRule rule) {
                  return rules.remove(rule);
                }
          - id: RuleRegistry.rules
            type: MethodDeclaration
            code: |2
                public List<IApprovalRule> rules() {
                  return rules;
                }
          - id: RuleRegistry.find
            type: MethodDeclaration
            code: |2
                public Optional<IApprovalRule> find(String id) {
                  return rules.stream().filter(r -> r.id().equals(id)).findFirst();
                }
          - id: RuleRegistry.defaultSettings
            type: MethodDeclaration
            code: |2
                public RuleSettings defaultSettings(String id) {
                  return find(id).map(r -> r.getDefaultSetting().clone()).orElseGet(RuleSettings::new);
                }
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
        nodes:
        - id: vsV4Dz|pJd0WY
          type: ImportGroup
          code: |
            import java.nio.file.Path;
            import java.util.List;
            import java.util.Set;

            import com.fasterxml.jackson.databind.JsonNode;

            import xy.ai.conscious.autoapprove.ApprovalContext;
            import xy.ai.conscious.autoapprove.ApprovalItem;
        - id: DeepPathReadRule
          type: ClassOrInterfaceDeclaration
          excert: "public class DeepPathReadRule implements IApprovalRule {"
          children:
          - id: DeepPathReadRule.KkcWlY|HbusYF
            type: StatementGroup
            code: "\tpublic static final String ID = DeepPathReadRule.class.getSimpleName();\n\tpublic static final String TOOLS_SETTING = \"tools\";\n\tpublic static final String PATH_FIELDS_SETTING = \"pathFields\";\n\tpublic static final String MIN_DEPTH_SETTING = \"minDepth\";\n\tpublic static final String MAX_LENGTH_SETTING = \"maxLength\";\n\n\tpublic static final String[] TOOLS = { \"search\", \"list\" };\n\tpublic static final String[] PATH_FIELDS = { \"path\", \"directory\" };\n\tpublic static final int MIN_DEPTH = 3;\n\tpublic static final int MAX_LENGTH = 3000;\n\n\tprivate final RuleSettings defaults;\n\tprivate final Path cwd;\n"
          - id: DeepPathReadRule.DeepPathReadRule
            type: ConstructorDeclaration
            code: "\tpublic DeepPathReadRule() {\n\t\tthis(Set.of(TOOLS), Set.of(PATH_FIELDS), MIN_DEPTH, Path.of(\"\").toAbsolutePath().normalize());\n\t}\n"
          - id: DeepPathReadRule.DeepPathReadRule_2
            type: ConstructorDeclaration
            code: "\tpublic DeepPathReadRule(Set<String> tools, Set<String> pathFields, int minDepth, Path cwd) {\n\t\tthis.defaults = new RuleSettings() //\n\t\t\t\t.addStrings(TOOLS_SETTING, tools.stream().sorted().toList()) //\n\t\t\t\t.addStrings(PATH_FIELDS_SETTING, pathFields.stream().sorted().toList()) //\n\t\t\t\t.addNumber(MIN_DEPTH_SETTING, minDepth) //\n\t\t\t\t.addNumber(MAX_LENGTH_SETTING, MAX_LENGTH);\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n"
          - id: DeepPathReadRule.id
            type: MethodDeclaration
            code: "\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n"
          - id: DeepPathReadRule.getDefaultSetting
            type: MethodDeclaration
            code: "\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n"
          - id: DeepPathReadRule.vote
            type: MethodDeclaration
            code: "\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null)\n\t\t\treturn Vote.abstain();\n\t\tRuleSettings settings = ctx.settings(ID);\n\t\tSet<String> tools = settings.getStringSet(TOOLS_SETTING);\n\t\tswitch (item.kind()) {\n\t\tcase ControlCall, ToolCall:\n\t\t\treturn item.tool() != null && tools.contains(item.tool()) && allDeep(item, settings) ? Vote.allow() : Vote.abstain();\n\t\tcase ControlResult, ToolResult:\n\t\t\tApprovalItem call = ctx.call();\n\t\t\tif (call == null || call.tool() == null || !tools.contains(call.tool()) || !allDeep(call, settings))\n\t\t\t\treturn Vote.abstain();\n\t\t\treturn length(item) < settings.getNumber(MAX_LENGTH_SETTING) ? Vote.allow() : Vote.abstain();\n\t\tdefault:\n\t\t\treturn Vote.abstain();\n\t\t}\n\t}\n"
          - id: DeepPathReadRule.allDeep
            type: MethodDeclaration
            code: "\tprivate boolean allDeep(ApprovalItem call, RuleSettings settings) {\n\t\tList<String> values = PathArguments.collect(call.arguments(), settings.getStringSet(PATH_FIELDS_SETTING));\n\t\tif (values.isEmpty())\n\t\t\treturn false;\n\t\tlong minDepth = settings.getNumber(MIN_DEPTH_SETTING);\n\t\tfor (String value : values) {\n\t\t\tPath path = PathArguments.resolve(cwd, value);\n\t\t\tif (path == null || !path.startsWith(cwd) || cwd.relativize(path).getNameCount() < minDepth)\n\t\t\t\treturn false;\n\t\t}\n\t\treturn true;\n\t}\n"
          - id: DeepPathReadRule.length
            type: MethodDeclaration
            code: "\tprivate static int length(ApprovalItem item) {\n\t\tJsonNode result = item.result();\n\t\tif (result.isMissingNode())\n\t\t\treturn item.yaml().length();\n\t\treturn result.isTextual() ? result.asText().length() : result.toString().length();\n\t}\n"
```
/answer d545-4-3 allow
ReasoningToken: 35
Control Request:
```yaml
id: d545-5-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: "static Vote (pass|abstain|allow|deny)|class PathArguments|static .*\\(.*\\) *\\{|PASS|Pass"
    include:
    - '*.java'
    exclude:
    - target/**
    limit: 40
  reason: "Vote-API (pass?), PathArguments und IApprovalRule prüfen"
```
/answer d545-5-0 allow
Control Request:
```yaml
id: d545-6-5
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
        matches:
        - path: AutoApprover.java
          matches:
          - "private static void omitDenied(AIAnswer ans, Decision decision) {"
          - "publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp, 0));"
          - result = ApprovalResult.Pass;
          - "private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolean hold, boolean allow) {"
          - return ApprovalResult.Pass;
          - case Pass ->
          - "private static List<String> withBlock(ApprovalItem item, String commandLine) {"
          - "private static List<String> withBlock(String yaml, String commandLine) {"
          - "private static String deniedYaml(ApprovalItem item) {"
          - "private static String denial(ApprovalItem item, String reason) {"
          - "private static String command(String base, String extra) {"
          - "private static String oneLine(String s) {"
          - "private static Selection selection(List<String> lines) {"
          - "private static void addNonBlank(List<String> list, String text) {"
          - "private static void enrich(AIAnswer ans, List<String> warnings) {"
        - path: Decision.java
          matches:
          - "result = result == null ? ApprovalResult.Pass : result;"
          - "public static Decision pass(String sessionId) {"
          - "return new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, null, null, null, 0);"
        - path: SessionTracker.java
          matches:
          - "static String keyOf(AIAnswer ans) {"
          - "static String callIdOf(ApprovalItem result) {"
          - "private static boolean isCall(ApprovalItem item) {"
        - path: AnswerParser.java
          matches:
          - "static List<ApprovalItem> parse(String text) {"
          - "private static ApprovalItem parseCommandItem(String text) {"
          - "private static ApprovalItem parseToolResult(String text) {"
          - "private static Kind kindOf(Command command) {"
          - "private static ApprovalItem toItem(Kind kind, String yaml) {"
          - "static String omitResult(String text, ApprovalItem item) {"
          - "static String omittedYaml(ApprovalItem item) {"
          - "private static ObjectNode omitted(ApprovalItem item) {"
          - "private static String omitControlResult(String text, ApprovalItem item) {"
          - "private static String omitToolResult(String text, String id) {"
          - "private static ToolResult omitted(ToolResult result) {"
        - path: ApprovalResult.java
          matches:
          - "Pass,"
        - path: rules/DenyOwnPromptFileRule.java
          matches:
          - "private static String reason(boolean call, String absolute) {"
          - "private static String tail(Path file, int levels) {"
          - "private static String normalize(String value) {"
        - path: rules/Vote.java
          matches:
          - "public static Vote abstain() {"
          - "public static Vote allow() {"
        warnings:
        - "Limit of 40 matches reached (total 59); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer d545-6-5 allow
Control Request:
```yaml
id: d545-7-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/PathArguments.java
    node_type: .
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    node_type: MethodDeclaration
    regexp: resultFor|hold|Vote
  reason: "Vote, PathArguments, AutoApprover-Auswertung der Votes lesen"
```
/answer d545-7-0 allow
Control Request:
```yaml
id: d545-8-7
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
        nodes:
        - id: UbJ2xB|ENVO48
          type: ImportGroup
          code: |
            import java.util.ArrayList;
            import java.util.List;
        - id: Vote
          type: RecordDeclaration
          excert: /**  * Result of a single rule. Rules only decide; the approver executes. Besid…
          docstring: Result of a single rule. Rules only decide; the approver executes. Besides the
          children:
          - id: Vote.2F7xfC|B2J1EK
            type: StatementGroup
            code: |2
                private static final Vote ABSTAIN = new Vote(Verdict.Abstain, null, null, null, null);
                public Vote {
                  verdict = verdict == null ? Verdict.Abstain : verdict;
                  reason = reason == null ? "" : reason;
                  hint = hint == null ? "" : hint;
                  warnings = warnings == null ? List.of() : List.copyOf(warnings);
                  instruction = instruction == null ? "" : instruction;
                }
          - id: Vote.abstain
            type: MethodDeclaration
            code: |2
                public static Vote abstain() {
                  return ABSTAIN;
                }
          - id: Vote.allow
            type: MethodDeclaration
            code: |2
                public static Vote allow() {
                  return allow(null);
                }
          - id: Vote.allow_2
            type: MethodDeclaration
            code: |2
                public static Vote allow(String hint) {
                  return new Vote(Verdict.Allow, null, hint, null, null);
                }
          - id: Vote.deny
            type: MethodDeclaration
            code: |2
                public static Vote deny(String reason) {
                  return new Vote(Verdict.Deny, reason, null, null, null);
                }
          - id: Vote.hold
            type: MethodDeclaration
            code: |2
                public static Vote hold() {
                  return new Vote(Verdict.Hold, null, null, null, null);
                }
          - id: Vote.hold_2
            type: MethodDeclaration
            code: |2
                public static Vote hold(String warning) {
                  return new Vote(Verdict.Hold, null, null, List.of(warning), null);
                }
          - id: Vote.warning
            type: MethodDeclaration
            code: |2
                public static Vote warning(String text) {
                  return new Vote(Verdict.Abstain, null, null, List.of(text), null);
                }
          - id: Vote.instruction
            type: MethodDeclaration
            code: |2
                public static Vote instruction(String text) {
                  return new Vote(Verdict.Abstain, null, null, null, text);
                }
          - id: Vote.withWarning
            type: MethodDeclaration
            code: |2
                public Vote withWarning(String text) {
                  List<String> all = new ArrayList<>(warnings);
                  all.add(text);
                  return new Vote(verdict, reason, hint, all, instruction);
                }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/PathArguments.java
        nodes:
        - id: AKHURJ|3QhjAZ
          type: ImportGroup
          code: |
            import java.nio.file.InvalidPathException;
            import java.nio.file.Path;
            import java.util.ArrayList;
            import java.util.List;
            import java.util.Set;

            import com.fasterxml.jackson.databind.JsonNode;
        - id: PathArguments
          type: ClassOrInterfaceDeclaration
          excert: /**  * Helpers to find and evaluate path-like values in tool arguments.  */ pub…
          docstring: Helpers to find and evaluate path-like values in tool arguments.
          children:
          - id: PathArguments.PathArguments
            type: ConstructorDeclaration
            code: "\tprivate PathArguments() {\n\t}\n"
          - id: PathArguments.collect
            type: MethodDeclaration
            code: "\tstatic List<String> collect(JsonNode node, Set<String> fields) {\n\t\tList<String> out = new ArrayList<>();\n\t\tcollect(node, fields, out);\n\t\treturn out;\n\t}\n"
          - id: PathArguments.collect_2
            type: MethodDeclaration
            code: "\t@SuppressWarnings(\"deprecation\")\n\tprivate static void collect(JsonNode node, Set<String> fields, List<String> out) {\n\t\tif (node == null)\n\t\t\treturn;\n\t\tif (node.isObject()) {\n\t\t\tnode.fields().forEachRemaining(e -> {\n\t\t\t\tif (fields.contains(e.getKey()))\n\t\t\t\t\taddValues(e.getValue(), out);\n\t\t\t\tcollect(e.getValue(), fields, out);\n\t\t\t});\n\t\t} else if (node.isArray()) {\n\t\t\tnode.forEach(child -> collect(child, fields, out));\n\t\t}\n\t}\n"
          - id: PathArguments.addValues
            type: MethodDeclaration
            code: "\tprivate static void addValues(JsonNode value, List<String> out) {\n\t\tif (value.isTextual())\n\t\t\tout.add(value.asText());\n\t\telse if (value.isArray())\n\t\t\tvalue.forEach(v -> addValues(v, out));\n\t}\n"
          - id: PathArguments.resolve
            type: MethodDeclaration
            code: "\tstatic Path resolve(Path cwd, String value) {\n\t\ttry {\n\t\t\treturn cwd.resolve(value).normalize();\n\t\t} catch (InvalidPathException e) {\n\t\t\treturn null;\n\t\t}\n\t}\n"
          - id: PathArguments.isRoot
            type: MethodDeclaration
            code: "\tstatic boolean isRoot(Path path) {\n\t\treturn path.isAbsolute() && path.getNameCount() == 0;\n\t}\n"
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
        nodes:
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
              private Decision decide(ApprovalContext ctx, List<RuleVote> votes) {
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
                FollowUp followUp = followUpFor(ctx, result, reason, hint, instruction);
                if (followUp == null)
                  result = ApprovalResult.Pass;
                return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());
              }
        - id: AutoApprover.resultFor
          type: MethodDeclaration
          code: |2
              private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolean hold, boolean allow) {
                if (item == null || item.id().isBlank())
                  return ApprovalResult.Pass;
                if (deny)
                  return ApprovalResult.Deny;
                if (item.kind() == Kind.ToolResult)
                  return ApprovalResult.Pass;
                if (hold || !allow)
                  return ApprovalResult.Pass;
                return item.kind() == Kind.ToolCall ? ApprovalResult.Call : ApprovalResult.Allow;
              }
```
/answer d545-8-7 allow
ReasoningToken: 207
Control Request:
```yaml
id: d545-9-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
    node_type: .
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: DenyRootPathRule|HoldMarkdownRule
    include:
    - '*.java'
    - '*.md'
    - '*.yaml'
    - '*.json'
    - '*.properties'
    exclude:
    - target/**
    - '**/rules/DenyRootPathRule.java'
    - '**/rules/HoldMarkdownRule.java'
    limit: 20
  reason: HoldMarkdownRule als Hold-Vorlage und RuleSettings-API lesen
```
/answer d545-9-0 allow
Control Request:
```yaml
id: d545-a-9
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
          matches:
          - import xy.ai.conscious.autoapprove.rules.HoldMarkdownRule;
          - import xy.ai.conscious.autoapprove.rules.DenyRootPathRule;
          - register(new DenyRootPathRule());
          - register(new HoldMarkdownRule());
        - path: connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
          matches:
          - "public class DenyRootPathRule implements IApprovalRule {"
          - public static final String ID = DenyRootPathRule.class.getSimpleName();
          - "public DenyRootPathRule() {"
          - "public DenyRootPathRule(Path cwd) {"
        - path: connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
          matches:
          - "public class HoldMarkdownRule implements IApprovalRule {"
          - public static final String ID = HoldMarkdownRule.class.getSimpleName();
          - "public HoldMarkdownRule() {"
          - "public HoldMarkdownRule(Set<String> pathFields, Set<String> patternFields) {"
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
        nodes:
        - id: xDPc5p|pJjrWN
          type: ImportGroup
          code: |
            import java.util.Locale;
            import java.util.Set;

            import xy.ai.conscious.autoapprove.ApprovalContext;
            import xy.ai.conscious.autoapprove.ApprovalItem;
            import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
        - id: HoldMarkdownRule
          type: ClassOrInterfaceDeclaration
          excert: /**  * Holds the automatic approval for markdown access with the tools allowed …
          docstring: Holds the automatic approval for markdown access with the tools allowed by
          children:
          - id: HoldMarkdownRule.d86O60|8RIU7p
            type: StatementGroup
            code: "\tpublic static final String ID = HoldMarkdownRule.class.getSimpleName();\n\tpublic static final String PATH_FIELDS_SETTING = \"pathFields\";\n\tpublic static final String PATTERN_FIELDS_SETTING = \"patternFields\";\n\n\tpublic static final String[] PATH_FIELDS = { \"path\", \"paths\" };\n\tpublic static final String[] PATTERN_FIELDS = { \"include\" };\n\n\tprivate static final String EXTENSION = \".md\";\n\n\tprivate final RuleSettings defaults;\n"
          - id: HoldMarkdownRule.HoldMarkdownRule
            type: ConstructorDeclaration
            code: "\tpublic HoldMarkdownRule() {\n\t\tthis(Set.of(PATH_FIELDS), Set.of(PATTERN_FIELDS));\n\t}\n"
          - id: HoldMarkdownRule.HoldMarkdownRule_2
            type: ConstructorDeclaration
            code: "\tpublic HoldMarkdownRule(Set<String> pathFields, Set<String> patternFields) {\n\t\tthis.defaults = new RuleSettings() //\n\t\t\t\t.addStrings(PATH_FIELDS_SETTING, pathFields.stream().sorted().toList()) //\n\t\t\t\t.addStrings(PATTERN_FIELDS_SETTING, patternFields.stream().sorted().toList());\n\t}\n"
          - id: HoldMarkdownRule.id
            type: MethodDeclaration
            code: "\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n"
          - id: HoldMarkdownRule.getDefaultSetting
            type: MethodDeclaration
            code: "\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n"
          - id: HoldMarkdownRule.vote
            type: MethodDeclaration
            code: "\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (!ctx.settings(AllowedToolsRule.ID).getStringSet(AllowedToolsRule.TOOLS).contains(item.tool()))\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tRuleSettings settings = ctx.settings(ID);\n\t\tfor (String value : PathArguments.collect(item.arguments(), settings.getStringSet(PATH_FIELDS_SETTING))) {\n\t\t\tif (value.toLowerCase(Locale.ROOT).endsWith(EXTENSION))\n\t\t\t\treturn Vote.hold();\n\t\t}\n\t\tfor (String value : PathArguments.collect(item.arguments(), settings.getStringSet(PATTERN_FIELDS_SETTING))) {\n\t\t\tif (value.toLowerCase(Locale.ROOT).contains(EXTENSION))\n\t\t\t\treturn Vote.hold();\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n"
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
        nodes:
        - id: j4xNuG|nv5gBW
          type: ImportGroup
          code: |
            import java.util.ArrayList;
            import java.util.Collections;
            import java.util.LinkedHashMap;
            import java.util.List;
            import java.util.Map;
            import java.util.Objects;
            import java.util.Set;
            import java.util.TreeMap;
            import java.util.stream.Collectors;
        - id: RuleSettings
          type: ClassOrInterfaceDeclaration
          excert: "/**  * Typed, named settings of one rule. Every value lives in a {"
          docstring: "Typed, named settings of one rule. Every value lives in a {@link Setting}"
          children:
          - id: RuleSettings.ShN4I8|Kidw0u
            type: StatementGroup
            code: |2
                public static final String ACTIVE = "active";
                private final Map<String, Setting> settings = new LinkedHashMap<>();
          - id: RuleSettings.RuleSettings
            type: ConstructorDeclaration
            code: |2
                public RuleSettings() {
                  add(new BooleanSetting(ACTIVE, true));
                }
          - id: RuleSettings.RuleSettings_2
            type: ConstructorDeclaration
            code: |2
                private RuleSettings(boolean empty) {
                }
          - id: RuleSettings.empty
            type: MethodDeclaration
            code: |2
                public static RuleSettings empty() {
                  return new RuleSettings(true);
                }
          - id: RuleSettings.add
            type: MethodDeclaration
            code: |2
                public RuleSettings add(Setting setting) {
                  settings.put(setting.getName(), setting);
                  return this;
                }
          - id: RuleSettings.addBoolean
            type: MethodDeclaration
            code: |2
                public RuleSettings addBoolean(String name, boolean value) {
                  return add(new BooleanSetting(name, value));
                }
          - id: RuleSettings.addNumber
            type: MethodDeclaration
            code: |2
                public RuleSettings addNumber(String name, long value) {
                  return add(new NumberSetting(name, value));
                }
          - id: RuleSettings.addStrings
            type: MethodDeclaration
            code: |2
                public RuleSettings addStrings(String name, Iterable<String> values) {
                  return add(new StringListSetting(name, values));
                }
          - id: RuleSettings.addStrings_2
            type: MethodDeclaration
            code: |2
                public RuleSettings addStrings(String name, String... values) {
                  return add(new StringListSetting(name, List.of(values)));
                }
          - id: RuleSettings.settings
            type: MethodDeclaration
            code: |2
                public Map<String, Setting> settings() {
                  return Collections.unmodifiableMap(settings);
                }
          - id: RuleSettings.isEmpty
            type: MethodDeclaration
            code: |2
                public boolean isEmpty() {
                  return settings.isEmpty();
                }
          - id: RuleSettings.isActive
            type: MethodDeclaration
            code: |2
                public boolean isActive() {
                  Setting s = settings.get(ACTIVE);
                  return !(s instanceof BooleanSetting b) || b.get();
                }
          - id: RuleSettings.getNumber
            type: MethodDeclaration
            code: |2
                public long getNumber(String name) {
                  return settings.get(name) instanceof NumberSetting n ? n.get() : 0;
                }
          - id: RuleSettings.getBoolean
            type: MethodDeclaration
            code: |2
                public boolean getBoolean(String name) {
                  return settings.get(name) instanceof BooleanSetting b && b.get();
                }
          - id: RuleSettings.getStrings
            type: MethodDeclaration
            code: |2
                public List<String> getStrings(String name) {
                  return settings.get(name) instanceof StringListSetting l ? l.get() : List.of();
                }
          - id: RuleSettings.getStringSet
            type: MethodDeclaration
            code: |2
                public Set<String> getStringSet(String name) {
                  return Set.copyOf(getStrings(name));
                }
          - id: RuleSettings.clone
            type: MethodDeclaration
            code: |2
                @Override
                public RuleSettings clone() {
                  RuleSettings copy = new RuleSettings(true);
                  for (Setting s : settings.values()) copy.settings.put(s.getName(), s.copy());
                  return copy;
                }
          - id: RuleSettings.overlay
            type: MethodDeclaration
            code: |2
                public RuleSettings overlay(RuleSettings override) {
                  RuleSettings result = clone();
                  if (override == null)
                    return result;
                  for (Setting o : override.settings.values()) {
                    Setting own = result.settings.get(o.getName());
                    if (own != null && own.getClass() == o.getClass())
                      result.settings.put(own.getName(), o.copy());
                  }
                  return result;
                }
          - id: RuleSettings.diff
            type: MethodDeclaration
            code: |2
                public RuleSettings diff(RuleSettings base) {
                  RuleSettings result = new RuleSettings(true);
                  for (Setting s : settings.values()) if (base == null || !s.equals(base.settings.get(s.getName())))
                    result.settings.put(s.getName(), s.copy());
                  return result;
                }
          - id: RuleSettings.toValues
            type: MethodDeclaration
            code: |2
                public Map<String, Object> toValues() {
                  Map<String, Object> out = new TreeMap<>();
                  for (Setting s : settings.values()) out.put(s.getName(), s.getValue());
                  return out;
                }
          - id: RuleSettings.fromValues
            type: MethodDeclaration
            code: |2
                public static RuleSettings fromValues(Map<String, ?> values) {
                  RuleSettings result = new RuleSettings(true);
                  if (values == null)
                    return result;
                  for (Map.Entry<String, ?> e : values.entrySet()) {
                    Object v = e.getValue();
                    if (v instanceof Boolean b)
                      result.add(new BooleanSetting(e.getKey(), b));
                    else if (v instanceof Number n)
                      result.add(new NumberSetting(e.getKey(), n.longValue()));
                    else if (v instanceof List<?> l)
                      result.add(new StringListSetting(e.getKey(), l.stream().map(String::valueOf).collect(Collectors.toList())));
                  }
                  return result;
                }
          - id: RuleSettings.exportAll
            type: MethodDeclaration
            code: |2
                public static Map<String, Map<String, Object>> exportAll(Map<String, RuleSettings> byRule) {
                  Map<String, Map<String, Object>> out = new TreeMap<>();
                  if (byRule != null)
                    byRule.forEach((id, rs) -> out.put(id, rs.toValues()));
                  return out;
                }
          - id: RuleSettings.importAll
            type: MethodDeclaration
            code: |2
                public static Map<String, RuleSettings> importAll(Map<String, ? extends Map<String, ?>> byRule) {
                  Map<String, RuleSettings> out = new LinkedHashMap<>();
                  if (byRule != null)
                    byRule.forEach((id, values) -> out.put(id, fromValues(values)));
                  return out;
                }
          - id: RuleSettings.copyOf
            type: MethodDeclaration
            code: |2
                public static Map<String, RuleSettings> copyOf(Map<String, RuleSettings> byRule) {
                  Map<String, RuleSettings> out = new LinkedHashMap<>();
                  if (byRule != null)
                    byRule.forEach((id, rs) -> out.put(id, rs.clone()));
                  return out;
                }
          - id: RuleSettings.equals
            type: MethodDeclaration
            code: |2
                @Override
                public boolean equals(Object obj) {
                  return obj instanceof RuleSettings other && settings.equals(other.settings);
                }
          - id: RuleSettings.hashCode
            type: MethodDeclaration
            code: |2
                @Override
                public int hashCode() {
                  return settings.hashCode();
                }
          - id: RuleSettings.toString
            type: MethodDeclaration
            code: |2
                @Override
                public String toString() {
                  return settings.values().toString();
                }
          - id: RuleSettings.Setting
            type: ClassOrInterfaceDeclaration
            excert: /**  * Named container of one typed value.  */ public static abstract class Set…
            docstring: Named container of one typed value.
            children:
            - id: RuleSettings.Setting.kWIfMz|XRM62a
              type: StatementGroup
              code: |2
                    private final String name;
            - id: RuleSettings.Setting.Setting
              type: ConstructorDeclaration
              code: |2
                    protected Setting(String name) {
                      this.name = Objects.requireNonNull(name);
                    }
            - id: RuleSettings.Setting.getName
              type: MethodDeclaration
              code: |2
                    public String getName() {
                      return name;
                    }
            - id: RuleSettings.Setting.copy
              type: MethodDeclaration
              code: |2
                    public abstract Setting copy();
            - id: RuleSettings.Setting.getValue
              type: MethodDeclaration
              code: |2
                    public abstract Object getValue();
            - id: RuleSettings.Setting.setValue
              type: MethodDeclaration
              code: |2
                    public abstract void setValue(Object value);
            - id: RuleSettings.Setting.equals
              type: MethodDeclaration
              code: |2
                    @Override
                    public boolean equals(Object obj) {
                      return obj != null && obj.getClass() == getClass() && ((Setting) obj).name.equals(name) && ((Setting) obj).getValue().equals(getValue());
                    }
            - id: RuleSettings.Setting.hashCode
              type: MethodDeclaration
              code: |2
                    @Override
                    public int hashCode() {
                      return Objects.hash(name, getValue());
                    }
            - id: RuleSettings.Setting.toString
              type: MethodDeclaration
              code: |2
                    @Override
                    public String toString() {
                      return name + "=" + getValue();
                    }
          - id: RuleSettings.BooleanSetting
            type: ClassOrInterfaceDeclaration
            excert: "public static class BooleanSetting extends Setting {"
            children:
            - id: RuleSettings.BooleanSetting.jC8RlT|1AAXPG
              type: StatementGroup
              code: |2
                    private boolean value;
            - id: RuleSettings.BooleanSetting.BooleanSetting
              type: ConstructorDeclaration
              code: |2
                    public BooleanSetting(String name, boolean value) {
                      super(name);
                      this.value = value;
                    }
            - id: RuleSettings.BooleanSetting.get
              type: MethodDeclaration
              code: |2
                    public boolean get() {
                      return value;
                    }
            - id: RuleSettings.BooleanSetting.set
              type: MethodDeclaration
              code: |2
                    public void set(boolean value) {
                      this.value = value;
                    }
            - id: RuleSettings.BooleanSetting.copy
              type: MethodDeclaration
              code: |2
                    @Override
                    public Setting copy() {
                      return new BooleanSetting(getName(), value);
                    }
            - id: RuleSettings.BooleanSetting.getValue
              type: MethodDeclaration
              code: |2
                    @Override
                    public Object getValue() {
                      return value;
                    }
            - id: RuleSettings.BooleanSetting.setValue
              type: MethodDeclaration
              code: |2
                    @Override
                    public void setValue(Object v) {
                      if (v instanceof Boolean b)
                        value = b;
                    }
          - id: RuleSettings.NumberSetting
            type: ClassOrInterfaceDeclaration
            excert: "public static class NumberSetting extends Setting {"
            children:
            - id: RuleSettings.NumberSetting.DmzNLn|Kx0rHm
              type: StatementGroup
              code: |2
                    private long value;
            - id: RuleSettings.NumberSetting.NumberSetting
              type: ConstructorDeclaration
              code: |2
                    public NumberSetting(String name, long value) {
                      super(name);
                      this.value = value;
                    }
            - id: RuleSettings.NumberSetting.get
              type: MethodDeclaration
              code: |2
                    public long get() {
                      return value;
                    }
            - id: RuleSettings.NumberSetting.set
              type: MethodDeclaration
              code: |2
                    public void set(long value) {
                      this.value = value;
                    }
            - id: RuleSettings.NumberSetting.copy
              type: MethodDeclaration
              code: |2
                    @Override
                    public Setting copy() {
                      return new NumberSetting(getName(), value);
                    }
            - id: RuleSettings.NumberSetting.getValue
              type: MethodDeclaration
              code: |2
                    @Override
                    public Object getValue() {
                      return value;
                    }
            - id: RuleSettings.NumberSetting.setValue
              type: MethodDeclaration
              code: |2
                    @Override
                    public void setValue(Object v) {
                      if (v instanceof Number n)
                        value = n.longValue();
                    }
          - id: RuleSettings.StringListSetting
            type: ClassOrInterfaceDeclaration
            excert: "public static class StringListSetting extends Setting {"
            children:
            - id: RuleSettings.StringListSetting.osZ0Ct|AVp1uM
              type: StatementGroup
              code: |2
                    private List<String> values;
            - id: RuleSettings.StringListSetting.StringListSetting
              type: ConstructorDeclaration
              code: |2
                    public StringListSetting(String name, Iterable<String> values) {
                      super(name);
                      this.values = new ArrayList<>();
                      values.forEach(this.values::add);
                    }
            - id: RuleSettings.StringListSetting.get
              type: MethodDeclaration
              code: |2
                    public List<String> get() {
                      return Collections.unmodifiableList(values);
                    }
            - id: RuleSettings.StringListSetting.set
              type: MethodDeclaration
              code: |2
                    public void set(List<String> values) {
                      this.values = new ArrayList<>(values);
                    }
            - id: RuleSettings.StringListSetting.copy
              type: MethodDeclaration
              code: |2
                    @Override
                    public Setting copy() {
                      return new StringListSetting(getName(), values);
                    }
            - id: RuleSettings.StringListSetting.getValue
              type: MethodDeclaration
              code: |2
                    @Override
                    public Object getValue() {
                      return List.copyOf(values);
                    }
            - id: RuleSettings.StringListSetting.setValue
              type: MethodDeclaration
              code: |2
                    @Override
                    public void setValue(Object v) {
                      if (v instanceof List<?> l)
                        values = l.stream().map(String::valueOf).collect(Collectors.toCollection(ArrayList::new));
                    }
```
/answer d545-a-9 allow
ReasoningToken: 323
Control Request:
```yaml
id: d545-b-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldOutsideProjectRule.java
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.nio.file.Path;\nimport java.util.Set;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\n/**\n * Holds the automatic approval of every tool call with a path argument that\n * resolves outside of the project directory, so the operator decides.\n */\npublic class HoldOutsideProjectRule implements IApprovalRule {\n\n\tpublic static final String ID = HoldOutsideProjectRule.class.getSimpleName();\n\tpublic static final String PATH_FIELDS_SETTING = \"pathFields\";\n\n\tpublic static final String[] PATH_FIELDS = { \"path\", \"paths\", \"directory\", \"file\", \"files\" };\n\n\tprivate final RuleSettings defaults;\n\tprivate final Path cwd;\n\n\tpublic HoldOutsideProjectRule() {\n\t\tthis(Set.of(PATH_FIELDS), Path.of(\"\").toAbsolutePath().normalize());\n\t}\n\n\tpublic HoldOutsideProjectRule(Set<String> pathFields, Path cwd) {\n\t\tthis.defaults = new RuleSettings() //\n\t\t\t\t.addStrings(PATH_FIELDS_SETTING, pathFields.stream().sorted().toList());\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n\n\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n\n\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tSet<String> pathFields = ctx.settings(ID).getStringSet(PATH_FIELDS_SETTING);\n\t\tfor (String value : PathArguments.collect(item.arguments(), pathFields)) {\n\t\t\tif (isOutside(value))\n\t\t\t\treturn Vote.hold(\"Path outside of the project directory: \" + value);\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n\n\tprivate boolean isOutside(String value) {\n\t\tif (value.startsWith(\"~\"))\n\t\t\treturn true;\n\t\tPath path = PathArguments.resolve(cwd, value);\n\t\treturn path == null || !path.startsWith(cwd);\n\t}\n}\n"
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: RuleRegistry.RuleRegistry
    old_text: |2
          register(new DenyRootPathRule());
    new_text: |2
          register(new DenyRootPathRule());
          register(new HoldOutsideProjectRule());
    addImport:
    - import xy.ai.conscious.autoapprove.rules.HoldOutsideProjectRule;
  reason: Neue Regel HoldOutsideProjectRule anlegen und in RuleRegistry registrieren
```
/answer d545-b-0 allow
Result Stats: id=40737270-8d66-4e39-96bb-61a352febf84, total: 32528, in: 14, out: 4539, reason: 0, read: 82868, write: 27975, start: 1795772577358, end: 1795772583614, time: 6192, total: 46070, io: 0, control: 64, sent: 0, recv: 166
Ich habe die neue Regel `HoldOutsideProjectRule` angelegt und in der `RuleRegistry` registriert. Kompiliert oder getestet habe ich sie nicht.

Die Regel liegt neben `DeepPathReadRule` in `connector/src/xy/ai/conscious/autoapprove/rules/HoldOutsideProjectRule.java`. In der `RuleRegistry` steht sie direkt nach `DenyRootPathRule`.

**Verhalten**
- Sie prüft `ToolCall` und `ControlCall` für alle Tools. Es gibt keine Tool-Liste.
- Sie sammelt die Werte der Felder aus der Einstellung `pathFields` mit `PathArguments.collect`. Standard sind `path`, `paths`, `directory`, `file` und `files`.
- Jeden Wert löst sie mit `PathArguments.resolve` gegen das Arbeitsverzeichnis auf. Liegt der Pfad nicht unter dem Projektverzeichnis, gibt sie `Vote.hold(...)` zurück. Die Warnung nennt den Pfad.
- Das Projektverzeichnis ist wie bei `DeepPathReadRule` das normalisierte Arbeitsverzeichnis. Der Konstruktor `HoldOutsideProjectRule(Set<String>, Path)` erlaubt ein anderes Verzeichnis.
- Ein `hold` ergibt im `AutoApprover` das Ergebnis `Pass`, auch wenn eine andere Regel `allow` stimmt. Nur ein `Deny` anderer Regeln setzt sich weiterhin durch.
- Sonst enthält sich die Regel.

**Entscheidungen von mir**
- Pfade mit `~` am Anfang zählen als außerhalb. `Path.resolve` expandiert `~` nicht, sonst läge `~/x` fälschlich im Projekt.
- Ein nicht auflösbarer Pfad (`InvalidPathException`) führt ebenfalls zu `hold`.
- Symlinks löst die Regel nicht auf, es gibt keinen Dateisystemzugriff. Ein Symlink im Projekt, der nach außen zeigt, wird also nicht erkannt.
- Die Regel hat nur die Einstellung `pathFields` und das übliche `active`. Heißen die Pfadparameter deiner Tools anders, kannst du sie dort ergänzen.
