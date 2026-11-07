Es gibt leider ein Missverständnis im Approver. Es gibt zwei Dinge die Reasons heißen:

1. Die Begründung des Agenten zum Tool-Call
2. Die Begründung der Rule bei Ablehnung für den Nutzer

- `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java`
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java`

Angezeigt werden sollte erste im Panel nicht Punkt 2. Die Reason Liste kann erhalten bleiben aber korrigiere Punkt 1.

Beispiel
```yaml
id: 7cbd-1-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
    pattern: LanguageLevel|ParserConfiguration|JavaParser|StaticJavaParser|setLanguageLevel
    include:
    - '*.java'
    exclude:
    - target/**
  reason: "Parser-Konfiguration (Language Level) der Java-AST-Engine finden, um zu klären, warum 'record' nicht geparst wird."
```

Korrigiere zusätzlich die Reason Anzeige `fit(d.reason(), REASON_WIDTH)` - sie bekommt eine variable breite und wird nicht abgeschnitten.
SystemInit:  id=f309e502-f55f-414f-a499-f97351ed097e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: cf6b-5-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  reason: Understand structure of both files to find reason handling
```
/answer cf6b-5-0 allow
Control Request:
```yaml
id: cf6b-6-5
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
      nodes:
      - id: MGpBD7|9gbchz
        signature: import java.util.ArrayDeque;
      - id: AutoApprovePanel
        signature: /**  * Console like log of the decisions of the auto approver for all sessions.…
        docstring: Console like log of the decisions of the auto approver for all sessions.
        children:
        - id: AutoApprovePanel.uxnwF9|TH6iq5
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
```
/answer cf6b-6-5 allow
Control Request:
```yaml
id: cf6b-7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver
    pattern: "[rR]eason"
    include:
    - '*.java'
    limit: 50
  intent: editing
  reason: Find all reason usages
```
/answer cf6b-7-0 allow
Control Request:
```yaml
id: cf6b-8-7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver
      matches:
      - path: AutoApprover.java
        nodes:
        - id: AutoApprover.decide
          matches:
          - List<String> reasons = new ArrayList<>();
          - "addNonBlank(reasons, v.reason());"
          - "String reason = String.join(\"; \", reasons);"
          - "followUp = followUpFor(ctx, result, reason, hint, instruction);"
          - "return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());"
        - id: AutoApprover.followUpFor
          matches:
          - "private FollowUp followUpFor(ApprovalContext ctx, ApprovalResult result, String reason, String hint, String instruction) {"
          - "item.isControl() ? withBlock(item, command(AnswerCommand.CMD_ANSWER + \" \" + item.id() + \" deny\", reason)) : List.of(denial(item, reason));"
        - id: AutoApprover.denial
          matches:
          - "private static String denial(ApprovalItem item, String reason) {"
          - String why = oneLine(reason);
        matches:
        - lineno: 24
          match: "* Aggregation: Deny beats Allow beats Abstain. Reasons, hints, warnings and"
      - path: Decision.java
        nodes:
        - id: Decision
          matches:
          - "public record Decision(String sessionId, ApprovalResult result, ApprovalItem item, String reason, String hint,"
        - id: Decision.v2lM9h|x96jHo
          matches:
          - "reason = reason == null ? \"\" : reason;"
      - path: AnswerParser.java
        nodes:
        - id: AnswerParser.parseToolResult
          matches:
          - "public Object reasoning(Role role, List<String> texts, String meta) {"
      - path: rules/Vote.java
        nodes:
        - id: Vote
          matches:
          - "public record Vote(Verdict verdict, String reason, String hint, List<String> warnings, String instruction) {"
        - id: Vote.2F7xfC|B2J1EK
          matches:
          - "reason = reason == null ? \"\" : reason;"
        - id: Vote.deny
          matches:
          - "public static Vote deny(String reason) {"
          - "return new Vote(Verdict.Deny, reason, null, null, null);"
        - id: Vote.withWarning
          matches:
          - "return new Vote(verdict, reason, hint, all, instruction);"
        matches:
        - lineno: 8
          match: "* verdict a rule may attach metadata: a {@code reason} (relevant for Deny), a"
      - path: AutoApprovePanel.java
        nodes:
        - id: AutoApprovePanel.toEntry
          matches:
          - "String line = fit(d.sessionId(), SESSION_WIDTH) + \" | \" + fit(action, ACTION_WIDTH) + \" | \" + fit(tool, TOOL_WIDTH) + \" | \" + pad(Integer.toString(d.answerLength()), CHARS_WIDTH) + \" | \" + fit(d.reason(), REASON_WIDTH) + \": \" + shorten(detail, DETAIL_MAX);"
```
/answer cf6b-8-7 allow
Control Request:
```yaml
id: cf6b-9-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    ids:
    - AutoApprovePanel.toEntry
    - AutoApprovePanel.text
    - AutoApprovePanel.oneLine
    - AutoApprovePanel.shorten
    - AutoApprovePanel.fit
    - AutoApprovePanel.pad
    - AutoApprovePanel.onDecision
    - AutoApprovePanel.rebuild
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover.decide
  reason: Read relevant methods
```
/answer cf6b-9-0 allow
Control Request:
```yaml
id: cf6b-a-9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
      nodes:
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
              String line = fit(d.sessionId(), SESSION_WIDTH) + " | " + fit(action, ACTION_WIDTH) + " | " + fit(tool, TOOL_WIDTH) + " | " + pad(Integer.toString(d.answerLength()), CHARS_WIDTH) + " | " + fit(d.reason(), REASON_WIDTH) + ": " + shorten(detail, DETAIL_MAX);
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
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
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
              return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());
            }
```
/answer cf6b-a-9 allow
Control Request:
```yaml
id: cf6b-b-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver
    pattern: _WIDTH|_MAX
    include:
    - AutoApprovePanel.java
  reason: Find width constants
```
/answer cf6b-b-0 allow
Control Request:
```yaml
id: cf6b-c-b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver
      matches:
      - path: AutoApprovePanel.java
        matches:
        - private static final int SESSION_WIDTH = 10;
        - private static final int ACTION_WIDTH = 8;
        - private static final int TOOL_WIDTH = 16;
        - private static final int REASON_WIDTH = 40;
        - private static final int CHARS_WIDTH = 7;
        - private static final int DETAIL_MAX = 200;
        - "String line = fit(d.sessionId(), SESSION_WIDTH) + \" | \" + fit(action, ACTION_WIDTH) + \" | \" + fit(tool, TOOL_WIDTH) + \" | \" + pad(Integer.toString(d.answerLength()), CHARS_WIDTH) + \" | \" + fit(d.reason(), REASON_WIDTH) + \": \" + shorten(detail, DETAIL_MAX);"
```
/answer cf6b-c-b allow
ReasoningToken: 172
Control Request:
```yaml
id: cf6b-d-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    - /home/user/xyan/xy.ai.workbench/src
    pattern: \.reason\(\)|"reason"|record ApprovalItem|class ApprovalItem
    include:
    - '*.java'
    exclude:
    - target/**
    limit: 50
  reason: Find usages of Decision.reason and the agent reason in approval code
```
/answer cf6b-d-0 allow
Control Request:
```yaml
id: cf6b-e-d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - "addNonBlank(reasons, v.reason());"
      - path: xy/ai/conscious/autoapprove/ApprovalItem.java
        matches:
        - "public record ApprovalItem(Kind kind, String id, String tool, JsonNode data, String yaml) {"
      - path: xy/ai/conscious/connector/mcp/MCPClient.java
        matches:
        - args.remove("reason");
      - path: xy/ai/conscious/connector/mcp/MCPControlClient.java
        matches:
        - if ("reason".equals(e.getKey()))
        - "* Ensures a \"reason\" argument is present when the tool's schema declares one,"
        - if (props != null && props.has("reason") && !args.has("reason"))
        - "args.put(\"reason\", \"\");"
      - path: xy/ai/conscious/connector/claudecode/CCControlClient.java
        matches:
        - "approval.put(\"reason\", rejectReason);"
      - path: xy/ai/conscious/connector/openai/OpenAIConnector.java
        matches:
        - "LOG.error(\"Incomplete: \" + incomplete.reason().get().toString() + \" \");"
      - path: xy/ai/conscious/connector/harness/SessionProcessor.java
        matches:
        - if (!(arguments instanceof ObjectNode) || !arguments.has("reason"))
        - JsonNode reason = src.get("reason");
        - if (!"reason".equals(entry.getKey()))
        - "ordered.set(\"reason\", reason);"
      - path: xy/ai/workbench/view/approver/AutoApprovePanel.java
        matches:
        - "String line = fit(d.sessionId(), SESSION_WIDTH) + \" | \" + fit(action, ACTION_WIDTH) + \" | \" + fit(tool, TOOL_WIDTH) + \" | \" + pad(Integer.toString(d.answerLength()), CHARS_WIDTH) + \" | \" + fit(d.reason(), REASON_WIDTH) + \": \" + shorten(detail, DETAIL_MAX);"
```
/answer cf6b-e-d allow
ReasoningToken: 45
Control Request:
```yaml
id: cf6b-f-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
  node_type: .*
  regexp: .
  reason: Read ApprovalItem fully
```
/answer cf6b-f-0 allow
Control Request:
```yaml
id: cf6b-10-f
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalItem.java
      nodes:
      - id: D3sKkB|eJTl0D
        type: ImportGroup
        code: |
          import com.fasterxml.jackson.databind.JsonNode;
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
```
/answer cf6b-10-f allow
ReasoningToken: 138
Control Request:
```yaml
id: cf6b-11-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    ids:
    - AutoApprover.denial
    - AutoApprover.followUpFor
    - AutoApprover.collect
    - AutoApprover.publish
    - AutoApprover.oneLine
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/Decision.java
  reason: Check how reason is used in AutoApprover and Decision definition
```
/answer cf6b-11-0 allow
Control Request:
```yaml
id: cf6b-12-11
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover.denial
        type: MethodDeclaration
        code: |2
            private static String denial(ApprovalItem item, String reason) {
              String tool = item.tool().isBlank() ? "" : " (" + item.tool() + ")";
              String why = oneLine(reason);
              return "Tool call " + item.id() + tool + " was denied" + (why.isEmpty() ? "." : ": " + why);
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
      - id: AutoApprover.oneLine
        type: MethodDeclaration
        code: |2
            private static String oneLine(String s) {
              return s == null ? "" : s.replaceAll("\\s+", " ").strip();
            }
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
```
/answer cf6b-12-11 allow
ReasoningToken: 315
Control Request:
```yaml
id: cf6b-13-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  text: REASON_WIDTH = 40
  node_type: FieldDeclaration
  reason: Get id of REASON_WIDTH constant to remove it
```
/answer cf6b-13-0 allow
Control Request:
```yaml
id: cf6b-14-13
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
      nodes: []
```
/answer cf6b-14-13 allow
Control Request:
```yaml
id: cf6b-15-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.toEntry
    old_text: '    String line = fit(d.sessionId()'
    new_text: |2-
          String reason = item == null || isResult ? "" : oneLine(item.arguments().path("reason").asText(""));
          String line = fit(d.sessionId()
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.toEntry
    old_text: "fit(d.reason(), REASON_WIDTH) + \": \""
    new_text: "reason + \": \""
  reason: "Show the agent's reason from tool arguments, untruncated and unpadded"
```
/answer cf6b-15-0 allow
Control Request:
```yaml
id: cf6b-16-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver
    pattern: REASON_WIDTH
    include:
    - AutoApprovePanel.java
    intent: editing
  reason: Locate the now-unused constant
```
/answer cf6b-16-0 allow
Control Request:
```yaml
id: cf6b-17-16
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver
      matches:
      - path: AutoApprovePanel.java
        nodes:
        - id: AutoApprovePanel.uxnwF9|TH6iq5
          matches:
          - private static final int REASON_WIDTH = 40;
```
/answer cf6b-17-16 allow
Control Request:
```yaml
id: cf6b-18-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    ids:
    - AutoApprovePanel.uxnwF9|TH6iq5
  reason: Read the constants node before removing REASON_WIDTH
```
/answer cf6b-18-0 allow
Control Request:
```yaml
id: cf6b-19-18
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
      nodes:
      - id: AutoApprovePanel.uxnwF9|TH6iq5
        type: StatementGroup
        code: |2
            public static final String ID = "xy.ai.workbench.view.approver.AutoApprovePanel";
            private static final int MAX_LINES = 1000;
            private static final int SESSION_WIDTH = 10;
            private static final int ACTION_WIDTH = 8;
            private static final int TOOL_WIDTH = 16;
            private static final int REASON_WIDTH = 40;
            private static final int CHARS_WIDTH = 7;
            private static final int DETAIL_MAX = 200;
```
/answer cf6b-19-18 allow
Control Request:
```yaml
id: cf6b-1a-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
    id: AutoApprovePanel.uxnwF9|TH6iq5
    old_text: |2
        private static final int REASON_WIDTH = 40;
    new_text: ""
    exact: true
  reason: Remove unused REASON_WIDTH constant
```
/answer cf6b-1a-0 allow
Control Request:
```yaml
id: cf6b-1b-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/approver/AutoApprovePanel.java
  reason: Check the edited file still parses
```
/answer cf6b-1b-0 allow
Result Stats: id=f309e502-f55f-414f-a499-f97351ed097e, total: 24419, in: 22, out: 4198, reason: 0, read: 155033, write: 20199, start: 1794080987972, end: 1794080994069, time: 6036, total: 41417, io: 0, control: 61, sent: 0, recv: 150
Das Panel zeigt jetzt die Begründung des Agenten aus dem Tool-Call statt der Rule-Begründung. Ich habe es nur auf Syntaxfehler geprüft (die Datei parst), aber nicht gebaut oder gestartet.

**`AutoApprovePanel.java`**
- Die Begründung kommt aus `item.arguments().path("reason")`, also dem Feld `reason` auf oberster Ebene der Argumente, wie in deinem `search`-Beispiel.
- Sie wird nur auf eine Zeile normalisiert (`oneLine`). Es gibt keine feste Breite, kein Auffüllen und kein Abschneiden mehr, die Spalte wächst mit dem Text.
- Bei Tool-Results und Items ohne Argumente bleibt die Begründung leer, weil dort kein Agent-Reason existiert.
- `REASON_WIDTH` war nicht mehr in Gebrauch und ist entfernt.

**`AutoApprover.java`**
- Hier habe ich nichts geändert. Der Code nutzt `reason` dort durchgehend als Rule-Begründung: Sie wird in `decide` gesammelt, in `Decision.reason()` abgelegt und in `denial(...)` sowie im `deny`-Befehl an den Agenten zurückgegeben. Das ist die Rule-Begründung bei Ablehnung und soll so bleiben.
- Die Rule-Begründung steht weiterhin in `Decision.reason()`, wird im Panel aber nicht mehr angezeigt.

Falls `reason` bei manchen Tools nicht oben in den Argumenten steht, etwa verschachtelt in `items[]`, zeigt das Panel dafür nichts an. Sag Bescheid, wenn ich das ergänzen soll.
