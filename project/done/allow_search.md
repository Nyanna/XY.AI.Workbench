Implementiere eine Regel analog zu `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java`.
Diese Regel soll Tool-Calls wie "search" und "list" (konfigurierbar) zulassen und deren Results (wenn kleiner als 3000 Zeichen), wenn alle rekursiv in den Argumenten enthaltenen Felder wie "path" und "directoy" (konfigurierbar) mindestens 3 Ebenen (konfigurierbar) unter das CWD zeigen.

Beispiele:
```yaml
id: b725-1c-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/a
    pattern: ""
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/l010
    pattern: layout|mod\.rs
  reason: See files in target dir and locate layout module
```
```yaml
id: b725-20-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020
    pattern: DType|^pub mod|^mod |use crate
    include:
    - mod.rs
    - supported_precisions.rs
    - cpu_compute.rs
    exclude:
    - test/**
  reason: See how layout DType is imported and mod registration
```

Implementiere dann zusätzlich eine Regel die Tool-Calls wie "search" und "list" (konfigurierbar) ablehnt, mit dem Reason "Don't use Root directory.", wenn auch nur ein rekursiv in den Argumenten enthaltenen Feld wie "path" und "directoy" (konfigurierbar) auf das Root directory ("/") zeigt. 
SystemInit:  id=77800c72-6935-4647-83c6-fbe10a961cd9, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: b725-2a-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
  reason: Explore autoapprove package to see the existing rule structure
```
/answer b725-2a-0 allow
Control Request:
```yaml
id: b725-2b-2a
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
      - IApprovalRule.java
      - SmallResultRule.java
      - Verdict.java
      - Vote.java
```
/answer b725-2b-2a allow
Control Request:
```yaml
id: b725-2c-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
  reason: Read the reference rule to mirror it
```
/answer b725-2c-0 allow
Control Request:
```yaml
id: b725-2d-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
      nodes:
      - id: z2FhVL|9B9F9F
        signature: import java.util.Set;
      - id: AllowedToolsRule
        signature: "/**  * Allows every call of the tools listed in {"
        docstring: "Allows every call of the tools listed in {@link #ALLOWED_TOOLS}, regardless"
        children:
        - id: AllowedToolsRule.9ooLSz|ef3YJG
          signature: "public static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\", \"ast_…"
        - id: AllowedToolsRule.AllowedToolsRule
          signature: public AllowedToolsRule()
        - id: AllowedToolsRule.AllowedToolsRule_2
          signature: public AllowedToolsRule(String... toolNames)
        - id: AllowedToolsRule.vote
          signature: public Vote vote(ApprovalContext ctx)
```
/answer b725-2d-2c allow
Control Request:
```yaml
id: b725-2e-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
    ids:
    - AllowedToolsRule
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Verdict.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  reason: "Read reference rule, interfaces, context and registry"
```
/answer b725-2e-0 allow
Control Request:
```yaml
id: b725-2f-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
      nodes:
      - id: AllowedToolsRule
        type: ClassOrInterfaceDeclaration
        signature: "/**  * Allows every call of the tools listed in {"
        docstring: "Allows every call of the tools listed in {@link #ALLOWED_TOOLS}, regardless"
        children:
        - id: AllowedToolsRule.9ooLSz|ef3YJG
          type: StatementGroup
          code: "\tpublic static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\", \"ast_validate\", \"run_command\" };\n\n\tprivate final Set<String> allowed;\n"
        - id: AllowedToolsRule.AllowedToolsRule
          type: ConstructorDeclaration
          code: "\tpublic AllowedToolsRule() {\n\t\tthis(ALLOWED_TOOLS);\n\t}\n"
        - id: AllowedToolsRule.AllowedToolsRule_2
          type: ConstructorDeclaration
          code: "\tpublic AllowedToolsRule(String... toolNames) {\n\t\tthis.allowed = Set.of(toolNames);\n\t}\n"
        - id: AllowedToolsRule.vote
          type: MethodDeclaration
          code: "\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\treturn allowed.contains(item.tool()) ? Vote.allow() : Vote.abstain();\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
      nodes:
      - id: z2FhVL|v3y3zx
        signature: import java.util.Set;
      - id: SmallResultRule
        signature: "/**  * Allows results shorter than {"
        docstring: "Allows results shorter than {@link #MAX_LENGTH} characters if the tool call the…"
        children:
        - id: SmallResultRule.JhRlpz|xI9Rc5
          signature: public static final int MAX_LENGTH = 3000;
        - id: SmallResultRule.SmallResultRule
          signature: public SmallResultRule()
        - id: SmallResultRule.SmallResultRule_2
          signature: public SmallResultRule(String... toolNames)
        - id: SmallResultRule.vote
          signature: public Vote vote(ApprovalContext ctx)
        - id: SmallResultRule.length
          signature: private static int length(ApprovalItem item)
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: 3sbhBT|fTTgdD
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
/answer b725-2f-2e allow
Control Request:
```yaml
id: b725-30-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
    ids:
    - SmallResultRule
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
    ids:
    - IApprovalRule
  reason: "Read SmallResultRule body, ApprovalItem shape, IApprovalRule id default"
```
/answer b725-30-0 allow
Control Request:
```yaml
id: b725-31-30
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
      nodes:
      - id: SmallResultRule
        type: ClassOrInterfaceDeclaration
        signature: "/**  * Allows results shorter than {"
        docstring: "Allows results shorter than {@link #MAX_LENGTH} characters if the tool call the…"
        children:
        - id: SmallResultRule.JhRlpz|xI9Rc5
          type: StatementGroup
          code: "\tpublic static final int MAX_LENGTH = 3000;\n\n\tprivate final Set<String> allowed;\n"
        - id: SmallResultRule.SmallResultRule
          type: ConstructorDeclaration
          code: "\tpublic SmallResultRule() {\n\t\tthis(AllowedToolsRule.ALLOWED_TOOLS);\n\t}\n"
        - id: SmallResultRule.SmallResultRule_2
          type: ConstructorDeclaration
          code: "\tpublic SmallResultRule(String... toolNames) {\n\t\tthis.allowed = Set.of(toolNames);\n\t}\n"
        - id: SmallResultRule.vote
          type: MethodDeclaration
          code: "\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tApprovalItem call = ctx.call();\n\t\tif (item == null || call == null || call.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlResult && item.kind() != Kind.ToolResult)\n\t\t\treturn Vote.abstain();\n\t\tif (!allowed.contains(call.tool()))\n\t\t\treturn Vote.abstain();\n\t\treturn length(item) < MAX_LENGTH ? Vote.allow() : Vote.abstain();\n\t}\n"
        - id: SmallResultRule.length
          type: MethodDeclaration
          code: "\tprivate static int length(ApprovalItem item) {\n\t\tJsonNode result = item.result();\n\t\tif (result.isMissingNode())\n\t\t\treturn item.yaml().length();\n\t\treturn result.isTextual() ? result.asText().length() : result.toString().length();\n\t}\n"
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
```
/answer b725-31-30 allow
ReasoningToken: 81
Control Request:
```yaml
id: b725-32-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ^(package|import)
    include:
    - AllowedToolsRule.java
    - SmallResultRule.java
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: new (AllowedToolsRule|SmallResultRule)\(
    include:
    - '*.java'
  reason: See package/imports and where rules are registered
```
/answer b725-32-0 allow
Control Request:
```yaml
id: b725-33-32
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: rules/SmallResultRule.java
        matches:
        - package xy.ai.conscious.autoapprove.rules;
        - import java.util.Set;
        - import com.fasterxml.jackson.databind.JsonNode;
        - import xy.ai.conscious.autoapprove.ApprovalContext;
        - import xy.ai.conscious.autoapprove.ApprovalItem;
        - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
      - path: rules/AllowedToolsRule.java
        matches:
        - package xy.ai.conscious.autoapprove.rules;
        - import java.util.Set;
        - import xy.ai.conscious.autoapprove.ApprovalContext;
        - import xy.ai.conscious.autoapprove.ApprovalItem;
        - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/RuleRegistry.java
        matches:
        - register(new AllowedToolsRule());
        - register(new SmallResultRule());
```
/answer b725-33-32 allow
ReasoningToken: 650
Control Request:
```yaml
id: b725-34-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: register\(new|Verdict\.Deny|isDeny
    include:
    - '*.java'
    intent: editing
  reason: Locate registration site and deny precedence handling
```
/answer b725-34-0 allow
Control Request:
```yaml
id: b725-35-34
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: RuleRegistry.java
        nodes:
        - id: RuleRegistry.RuleRegistry
          matches:
          - register(new AllowedToolsRule());
          - register(new SmallResultRule());
      - path: rules/Vote.java
        nodes:
        - id: Vote.deny
          matches:
          - "return new Vote(Verdict.Deny, reason, null, null, null);"
```
/answer b725-35-34 allow
Control Request:
```yaml
id: b725-36-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    ids:
    - RuleRegistry.RuleRegistry
    - 3sbhBT|fTTgdD
  reason: Read registry constructor to add the new rules
```
/answer b725-36-0 allow
Control Request:
```yaml
id: b725-37-36
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: RuleRegistry.RuleRegistry
        type: ConstructorDeclaration
        code: |2
            public RuleRegistry() {
              register(new AllowedToolsRule());
              register(new SmallResultRule());
            }
      - id: 3sbhBT|fTTgdD
        type: ImportGroup
        code: |
          import java.util.List;
          import java.util.Objects;
          import java.util.concurrent.CopyOnWriteArrayList;
          import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
          import xy.ai.conscious.autoapprove.rules.IApprovalRule;
          import xy.ai.conscious.autoapprove.rules.SmallResultRule;
```
/answer b725-37-36 allow
Control Request:
```yaml
id: b725-38-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/PathArguments.java
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.nio.file.InvalidPathException;\nimport java.nio.file.Path;\nimport java.util.ArrayList;\nimport java.util.List;\nimport java.util.Set;\n\nimport com.fasterxml.jackson.databind.JsonNode;\n\n/** Helpers to find and evaluate path-like values in tool arguments. */\nfinal class PathArguments {\n\n\tprivate PathArguments() {\n\t}\n\n\t/** Collects all string values (or string array elements) of fields named like {@code fields}, at any depth. */\n\tstatic List<String> collect(JsonNode node, Set<String> fields) {\n\t\tList<String> out = new ArrayList<>();\n\t\tcollect(node, fields, out);\n\t\treturn out;\n\t}\n\n\tprivate static void collect(JsonNode node, Set<String> fields, List<String> out) {\n\t\tif (node == null)\n\t\t\treturn;\n\t\tif (node.isObject()) {\n\t\t\tnode.fields().forEachRemaining(e -> {\n\t\t\t\tif (fields.contains(e.getKey()))\n\t\t\t\t\taddValues(e.getValue(), out);\n\t\t\t\tcollect(e.getValue(), fields, out);\n\t\t\t});\n\t\t} else if (node.isArray()) {\n\t\t\tnode.forEach(child -> collect(child, fields, out));\n\t\t}\n\t}\n\n\tprivate static void addValues(JsonNode value, List<String> out) {\n\t\tif (value.isTextual())\n\t\t\tout.add(value.asText());\n\t\telse if (value.isArray())\n\t\t\tvalue.forEach(v -> addValues(v, out));\n\t}\n\n\t/** Resolves against {@code cwd} and normalizes; null if the value is not a valid path. */\n\tstatic Path resolve(Path cwd, String value) {\n\t\ttry {\n\t\t\treturn cwd.resolve(value).normalize();\n\t\t} catch (InvalidPathException e) {\n\t\t\treturn null;\n\t\t}\n\t}\n\n\tstatic boolean isRoot(Path path) {\n\t\treturn path.isAbsolute() && path.getNameCount() == 0;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.nio.file.Path;\nimport java.util.List;\nimport java.util.Set;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\nimport com.fasterxml.jackson.databind.JsonNode;\n\n/**\n * Allows calls of path-based read tools (e.g. search, list) and their results\n * shorter than {@link #MAX_LENGTH} characters, if every path-like argument\n * points at least {@code minDepth} levels below the working directory.\n */\npublic class DeepPathReadRule implements IApprovalRule {\n\n\tpublic static final String[] TOOLS = { \"search\", \"list\" };\n\tpublic static final String[] PATH_FIELDS = { \"path\", \"directory\" };\n\tpublic static final int MIN_DEPTH = 3;\n\tpublic static final int MAX_LENGTH = 3000;\n\n\tprivate final Set<String> tools;\n\tprivate final Set<String> pathFields;\n\tprivate final int minDepth;\n\tprivate final Path cwd;\n\n\tpublic DeepPathReadRule() {\n\t\tthis(Set.of(TOOLS), Set.of(PATH_FIELDS), MIN_DEPTH, Path.of(\"\").toAbsolutePath().normalize());\n\t}\n\n\tpublic DeepPathReadRule(Set<String> tools, Set<String> pathFields, int minDepth, Path cwd) {\n\t\tthis.tools = Set.copyOf(tools);\n\t\tthis.pathFields = Set.copyOf(pathFields);\n\t\tthis.minDepth = minDepth;\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null)\n\t\t\treturn Vote.abstain();\n\t\tswitch (item.kind()) {\n\t\tcase ControlCall, ToolCall:\n\t\t\treturn item.tool() != null && tools.contains(item.tool()) && allDeep(item) ? Vote.allow() : Vote.abstain();\n\t\tcase ControlResult, ToolResult:\n\t\t\tApprovalItem call = ctx.call();\n\t\t\tif (call == null || call.tool() == null || !tools.contains(call.tool()) || !allDeep(call))\n\t\t\t\treturn Vote.abstain();\n\t\t\treturn length(item) < MAX_LENGTH ? Vote.allow() : Vote.abstain();\n\t\tdefault:\n\t\t\treturn Vote.abstain();\n\t\t}\n\t}\n\n\tprivate boolean allDeep(ApprovalItem call) {\n\t\tList<String> values = PathArguments.collect(call.arguments(), pathFields);\n\t\tif (values.isEmpty())\n\t\t\treturn false;\n\t\tfor (String value : values) {\n\t\t\tPath path = PathArguments.resolve(cwd, value);\n\t\t\tif (path == null || !path.startsWith(cwd) || cwd.relativize(path).getNameCount() < minDepth)\n\t\t\t\treturn false;\n\t\t}\n\t\treturn true;\n\t}\n\n\tprivate static int length(ApprovalItem item) {\n\t\tJsonNode result = item.result();\n\t\tif (result.isMissingNode())\n\t\t\treturn item.yaml().length();\n\t\treturn result.isTextual() ? result.asText().length() : result.toString().length();\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.nio.file.Path;\nimport java.util.Set;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\n/** Denies calls of path-based tools if any path-like argument points at the root directory. */\npublic class DenyRootPathRule implements IApprovalRule {\n\n\tpublic static final String REASON = \"Don't use Root directory.\";\n\n\tprivate final Set<String> tools;\n\tprivate final Set<String> pathFields;\n\tprivate final Path cwd;\n\n\tpublic DenyRootPathRule() {\n\t\tthis(Set.of(DeepPathReadRule.TOOLS), Set.of(DeepPathReadRule.PATH_FIELDS),\n\t\t\t\tPath.of(\"\").toAbsolutePath().normalize());\n\t}\n\n\tpublic DenyRootPathRule(Set<String> tools, Set<String> pathFields, Path cwd) {\n\t\tthis.tools = Set.copyOf(tools);\n\t\tthis.pathFields = Set.copyOf(pathFields);\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null || !tools.contains(item.tool()))\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tfor (String value : PathArguments.collect(item.arguments(), pathFields)) {\n\t\t\tPath path = PathArguments.resolve(cwd, value);\n\t\t\tif (path != null && PathArguments.isRoot(path))\n\t\t\t\treturn Vote.deny(REASON);\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n}\n"
  reason: "Create shared path helper, the deep-path allow rule and the root-deny rule"
```
/answer b725-38-0 allow
ReasoningToken: 29
Control Request:
```yaml
id: b725-39-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: RuleRegistry.RuleRegistry
    old_text: register(new SmallResultRule());
    new_text: |-
      register(new SmallResultRule());
          register(new DeepPathReadRule());
          register(new DenyRootPathRule());
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: 3sbhBT|fTTgdD
    old_text: import xy.ai.conscious.autoapprove.rules.IApprovalRule;
    new_text: |-
      import xy.ai.conscious.autoapprove.rules.DeepPathReadRule;
      import xy.ai.conscious.autoapprove.rules.DenyRootPathRule;
      import xy.ai.conscious.autoapprove.rules.IApprovalRule;
  reason: Register the new rules
```
/answer b725-39-0 allow
ReasoningToken: 71
Control Request:
```yaml
id: b725-3a-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/PathArguments.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  reason: Check the new and edited files parse
```
/answer b725-3a-0 allow
Result Stats: id=77800c72-6935-4647-83c6-fbe10a961cd9, total: 28118, in: 18, out: 7754, reason: 0, read: 95689, write: 20346, start: 1794091710777, end: 1794091718382, time: 7534, total: 50705, io: 0, control: 71, sent: 0, recv: 207
Ich habe die beiden Regeln angelegt und in `RuleRegistry` registriert. Die Dateien parsen fehlerfrei. Kompiliert oder getestet habe ich nichts.

Alle neuen Dateien liegen in `connector/src/xy/ai/conscious/autoapprove/rules/`.

- **`DeepPathReadRule`** lässt Calls von `search` und `list` zu, wenn alle Pfadwerte mindestens 3 Ebenen unter dem CWD liegen. Die Results lässt sie zu, wenn sie kürzer als 3000 Zeichen sind und der zugehörige Call die gleiche Prüfung besteht.
  - Die Pfade werden rekursiv in den Argumenten gesucht, in den Feldern `path` und `directory`. Das Feld darf ein String oder eine Liste von Strings sein, wie bei `directory:` in deinem zweiten Beispiel.
  - Relative Pfade werden gegen das CWD aufgelöst und normalisiert, `..` kommt also nicht an der Prüfung vorbei.
  - Ohne Pfadwert stimmt die Regel nicht zu.
- **`DenyRootPathRule`** lehnt Calls von `search` und `list` mit dem Reason `Don't use Root directory.` ab, sobald ein Pfadwert nach der Normalisierung `/` ist. Das gilt auch für `/.` oder `/a/..`.
- **`PathArguments`** ist ein gemeinsamer Helfer, der die Pfade einsammelt und auflöst.

Tools, Feldnamen, Mindesttiefe und CWD sind über Konstruktorparameter konfigurierbar. Die Defaults sind `search`/`list`, `path`/`directory`, 3 Ebenen und `user.dir`.

- **CWD in deinen Beispielen:** Beide Pfade liegen unter `.../xy.ai.workbench/rag/...`. Dein aktuelles Arbeitsverzeichnis ist `.../xy.ai.workbench/project`, also liegen sie nicht darunter. Sie würden nur zugelassen, wenn die Anwendung mit dem CWD `/home/user/xyan/xy.ai.workbench` läuft. Sonst übergibst du das CWD über den Konstruktor.
- **Vorrang von Deny:** Ich habe in `AutoApprover` nicht geprüft, ob ein Deny-Vote einen Allow-Vote übertrumpft. Das solltest du dort bestätigen, sonst könnte ein Root-Pfad trotz Deny durchgehen. Bei Root-Pfaden stimmt `DeepPathReadRule` aber nie zu, weil `/` keine 3 Ebenen unter dem CWD liegt.
- **Unbenutzter Import:** In `DeepPathReadRule` und `DenyRootPathRule` ist der Import von `Kind` teilweise ungenutzt.
