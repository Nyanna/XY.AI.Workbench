Der Java AST in `/home/user/xyan/xy.ai.workbench/ast-engines/java/src` scheint ein Problem zu haben.
Das Ersetzen von `parse` ist fehlgeschlagen, weil ich zwei Methoden in einem Node übergeben habe. Ich mache es getrennt.
Vermutlich muss hier ein Fallbacklogik herhalten.

```yaml
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
    id: AnswerParser.parse
    source: "\tstatic List<ApprovalItem> parse(String text) {\n\t\tList<ApprovalItem> items = new ArrayList<>();\n\t\tParsedCommand parsed = PromptInputHandler.parseTrailingCommand(text);\n\t\tif (parsed == null || parsed.yaml() == null)\n\t\t\treturn items;\n\t\tKind kind = kindOf(parsed.command());\n\t\tApprovalItem item = kind == null ? null : toItem(kind, parsed.yaml());\n\t\tif (item != null)\n\t\t\titems.add(item);\n\t\treturn items;\n\t}\n\n\tprivate static Kind kindOf(Command command) {\n\t\tif (command instanceof AnswerCommand ac)\n\t\t\treturn ac.blockType() == BlockType.Result ? Kind.ControlResult : Kind.ControlCall;\n\t\tif (command instanceof CallCommand)\n\t\t\treturn Kind.ToolCall;\n\t\treturn null;\n\t}\n"

result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
      error: |
        invalid member declaration: Encountered unexpected token: "private" "private"
            at line 13, column 3.

        Was expecting:

            <EOF>
      id: AnswerParser.parse
```

SystemInit:  id=a5a2863b-1247-4d18-b4e5-5d40513f4a7c, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst die Java-Engine an und suche die Replace-Logik.
Control Request:
```yaml
id: cf6b-1c-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src
    pattern: \.java$
  reason: "Dateistruktur der Java-AST-Engine erfassen, um die Replace-Logik zu finden"
```
/answer cf6b-1c-0 allow
ReasoningToken: 206
Control Request:
```yaml
id: cf6b-1d-1c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src
      entries:
      - "./xy/ai/mcpc/ast/engine: (8 files)"
      - AddressableNode.java
      - "./xy/ai/mcpc/ast/openapi: (4 files)"
      - AppendInfoNodesValidateClient.java
      - "./xy/ai/mcpc/ast/openapi/components: (6 files)"
      - CandidatesList.java
      - "./xy/ai/mcpc/ast/openapi/request/nodes/nodeid/insert/post/json: (2 files)"
      - InsertRequest.java
      - "./xy/ai/mcpc/ast/openapi/request/nodes/post/json: (1 file)"
      - LocateRequest.java
      - "./xy/ai/mcpc/ast/openapi/response/append: (1 file)"
      - AppendResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/append/code200/json: (2 files)"
      - AppendResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/append/code422/json: (1 file)"
      - AppendResponseCode422Json.java
      - "./xy/ai/mcpc/ast/openapi/response/info: (1 file)"
      - InfoResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/info/code200/json: (2 files)"
      - EngineInfo.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes: (1 file)"
      - NodesResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/code200/json: (3 files)"
      - LocateResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/code422/json: (1 file)"
      - NodesResponseCode422Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid: (1 file)"
      - NodesNodeIdResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code200/json: (1 file)"
      - NodesNodeIdResponseCode200Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code404/json: (1 file)"
      - NodesNodeIdResponseCode404Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code409/json: (1 file)"
      - NodesNodeIdResponseCode409Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code422/json: (1 file)"
      - NodesNodeIdResponseCode422Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete: (1 file)"
      - NodesNodeIdDeleteResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code200/json: (1 file)"
      - NodesNodeIdDeleteResponseCode200Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code404/json: (1 file)"
      - NodesNodeIdDeleteResponseCode404Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code409/json: (1 file)"
      - NodesNodeIdDeleteResponseCode409Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code422/json: (1 file)"
      - NodesNodeIdDeleteResponseCode422Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert: (1 file)"
      - NodesNodeIdInsertResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code200/json: (2 files)"
      - InsertResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code404/json: (1 file)"
      - NodesNodeIdInsertResponseCode404Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code409/json: (1 file)"
      - NodesNodeIdInsertResponseCode409Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code422/json: (1 file)"
      - NodesNodeIdInsertResponseCode422Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace: (1 file)"
      - NodesNodeIdReplaceResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code200/json: (1 file)"
      - NodesNodeIdReplaceResponseCode200Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code404/json: (1 file)"
      - NodesNodeIdReplaceResponseCode404Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code409/json: (1 file)"
      - NodesNodeIdReplaceResponseCode409Json.java
      - "./xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code422/json: (1 file)"
      - NodesNodeIdReplaceResponseCode422Json.java
      - "./xy/ai/mcpc/ast/openapi/response/validate: (1 file)"
      - ValidateResponse.java
      - "./xy/ai/mcpc/ast/openapi/response/validate/code200/json: (2 files)"
      - ValidateResponse.java
      warning: Result reduced because the total number of entries (57) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer cf6b-1d-1c allow
Control Request:
```yaml
id: cf6b-1e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
    pattern: invalid member declaration|parseBodyDeclaration|parseMethodDeclaration|parseStatement|parseBlock
    include:
    - '*.java'
    exclude:
    - build/**
    - target/**
    limit: 30
  intent: editing
  reason: "Stellen finden, an denen der Ersetzungsquelltext geparst wird"
```
/answer cf6b-1e-0 allow
Control Request:
```yaml
id: cf6b-1f-1e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
      matches:
      - path: xy/ai/mcpc/ast/engine/JavaAstEngine.java
        nodes:
        - id: JavaAstEngine.parseBodyFragment
          matches:
          - "return unwrap(parser.parseBodyDeclaration(code), \"member declaration\");"
```
/answer cf6b-1f-1e allow
Control Request:
```yaml
id: cf6b-20-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  reason: "Struktur der Engine ansehen, um Replace-/Parse-Methoden zu lokalisieren"
```
/answer cf6b-20-0 allow
Control Request:
```yaml
id: cf6b-21-20
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: NHeqvp|VsGI8e
        signature: import com.github.javaparser.JavaParser;
      - id: 7UMP2r|2zZKdQ
        signature: import com.github.javaparser.ast.comments.JavadocComment;
      - id: 57sJO3|T8bwk3
        signature: import com.github.javaparser.printer.configuration.Indentation;
      - id: JavaAstEngine
        signature: "/**  * JavaParser mechanics: parse/print, syntax validation, fragment parsing k…"
        docstring: "JavaParser mechanics: parse/print, syntax validation, fragment parsing keyed"
        children:
        - id: JavaAstEngine.CYgqlK|8W8fvh
          signature: private final JavaParser parser;
        - id: JavaAstEngine.JavaAstEngine
          signature: public JavaAstEngine()
        - id: JavaAstEngine.parseCompilationUnit
          signature: public CompilationUnit parseCompilationUnit(String source)
        - id: JavaAstEngine.emptyCompilationUnit
          signature: public CompilationUnit emptyCompilationUnit()
        - id: JavaAstEngine.print
          signature: public String print(Object node)
          docstring: "Dispatches to {@link #print(Node)} or, for a grouped segment, joins each member…"
        - id: JavaAstEngine.printGroup
          signature: private String printGroup(NodeGroup group)
        - id: JavaAstEngine.print_2
          signature: public String print(Node node)
        - id: JavaAstEngine.DefaultPrettyPrinterVisitorExtension
          signature: private class DefaultPrettyPrinterVisitorExtension extends DefaultPrettyPrinter…
          children:
          - id: JavaAstEngine.DefaultPrettyPrinterVisitorExtension.DefaultPrettyPrinterVisitorExtension
            signature: private DefaultPrettyPrinterVisitorExtension(PrinterConfiguration configuration)
          - id: JavaAstEngine.DefaultPrettyPrinterVisitorExtension.printMembers
            signature: "protected void printMembers(final NodeList<BodyDeclaration<?>> members, final V…"
        - id: JavaAstEngine.needsBlankLineAfter
          signature: private boolean needsBlankLineAfter(BodyDeclaration<?> m)
        - id: JavaAstEngine.validate
          signature: public String validate(String source)
          docstring: "Returns an error message if {@code source} is malformed, else {@code null}."
        - id: JavaAstEngine.parseImportFragment
          signature: public ImportDeclaration parseImportFragment(String code)
        - id: JavaAstEngine.parseTypeFragment
          signature: public TypeDeclaration<?> parseTypeFragment(String code)
        - id: JavaAstEngine.parseBodyFragment
          signature: public BodyDeclaration<?> parseBodyFragment(String code)
        - id: JavaAstEngine.parseEnumConstantFragment
          signature: public EnumConstantDeclaration parseEnumConstantFragment(String code)
        - id: JavaAstEngine.parseReplacementFor
          signature: "public Node parseReplacementFor(Node sample, String code)"
          docstring: "Parses {@code code} for the node category matching {@code sample}'s own type."
        - id: JavaAstEngine.AppendResult
          signature: "public record AppendResult(CompilationUnit cu, int units) {"
        - id: JavaAstEngine.append
          signature: "public AppendResult append(CompilationUnit cu, String code)"
        - id: JavaAstEngine.replace
          signature: "public void replace(AddressableNode target, String code)"
        - id: JavaAstEngine.insert
          signature: "public int insert(AddressableNode target, String code, String position)"
        - id: JavaAstEngine.parseInsertionFragments
          signature: "private List<Node> parseInsertionFragments(Node sample, String code)"
        - id: JavaAstEngine.delete
          signature: public void delete(AddressableNode target)
        - id: JavaAstEngine.groupContainer
          signature: private NodeList<Node> groupContainer(NodeGroup group)
        - id: JavaAstEngine.replaceGroup
          signature: "private void replaceGroup(NodeGroup group, String code)"
        - id: JavaAstEngine.insertGroup
          signature: "private int insertGroup(NodeGroup group, String code, String position)"
        - id: JavaAstEngine.deleteGroup
          signature: private void deleteGroup(NodeGroup group)
        - id: JavaAstEngine.parseGroupFragments
          signature: "private List<Node> parseGroupFragments(String kind, String code)"
        - id: JavaAstEngine.parseImportsFragment
          signature: public List<Node> parseImportsFragment(String code)
          docstring: "Parses {@code code} as one or more import declarations, for an {@link NodeGroup…"
        - id: JavaAstEngine.parseMembersFragment
          signature: public List<Node> parseMembersFragment(String code)
          docstring: "Parses {@code code} as one or more member declarations, for a {@link NodeGroup#…"
        - id: JavaAstEngine.signature
          signature: "public String signature(Object node, int limit)"
          docstring: "Dispatches to {@link #signature(Node, int)} or, for a grouped segment, its own …"
        - id: JavaAstEngine.signature_2
          signature: "public String signature(Node node, int limit)"
        - id: JavaAstEngine.header
          signature: private static String header(String printed)
        - id: JavaAstEngine.firstLine
          signature: private static String firstLine(String printed)
        - id: JavaAstEngine.docstring
          signature: "public String docstring(Object node, int limit)"
          docstring: "Dispatches to {@link #docstring(Node, int)}; a grouped segment never carries a …"
        - id: JavaAstEngine.docstring_2
          signature: "public String docstring(Node node, int limit)"
          docstring: "Short first-paragraph rendering of {@code node}'s Javadoc, if any."
        - id: JavaAstEngine.unwrap
          signature: "private static T unwrap(ParseResult<T> result, String what)"
        - id: JavaAstEngine.unwrapSingleStatementBlocks
          signature: private static void unwrapSingleStatementBlocks(Node root)
        - id: JavaAstEngine.problemsMessage
          signature: private static String problemsMessage(List<Problem> problems)
```
/answer cf6b-21-20 allow
Control Request:
```yaml
id: cf6b-22-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.parseBodyFragment
    - JavaAstEngine.parseReplacementFor
    - JavaAstEngine.replace
    - JavaAstEngine.insert
    - JavaAstEngine.parseInsertionFragments
    - JavaAstEngine.parseMembersFragment
    - JavaAstEngine.parseImportsFragment
    - JavaAstEngine.parseGroupFragments
    - JavaAstEngine.unwrap
  reason: "Replace-, Insert- und Fragment-Parsing-Code lesen, um die Fallback-Stelle zu bestimmen"
```
/answer cf6b-22-0 allow
Control Request:
```yaml
id: cf6b-23-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: JavaAstEngine.parseBodyFragment
        type: MethodDeclaration
        code: |2
            public BodyDeclaration<?> parseBodyFragment(String code) {
              return unwrap(parser.parseBodyDeclaration(code), "member declaration");
            }
      - id: JavaAstEngine.parseReplacementFor
        type: MethodDeclaration
        code: |2
            public Node parseReplacementFor(Node sample, String code) {
              if (sample instanceof ImportDeclaration)
                return parseImportFragment(code);
              if (sample instanceof TypeDeclaration<?>)
                return parseTypeFragment(code);
              if (sample instanceof EnumConstantDeclaration)
                return parseEnumConstantFragment(code);
              return parseBodyFragment(code);
            }
      - id: JavaAstEngine.replace
        type: MethodDeclaration
        code: |2
            public void replace(AddressableNode target, String code) {
              if (target.astNode instanceof NodeGroup group) {
                replaceGroup(group, code);
                return;
              }
              Node node = (Node) target.astNode;
              Node replacement = parseReplacementFor(node, code);
              if (!node.replace(replacement))
                throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node could not be replaced in place: " + target.id);
            }
      - id: JavaAstEngine.insert
        type: MethodDeclaration
        code: |2
            @SuppressWarnings("unchecked")
            public int insert(AddressableNode target, String code, String position) {
              if (target.astNode instanceof NodeGroup group)
                return insertGroup(group, code, position);
              Node node = (Node) target.astNode;
              List<Node> fragments = parseInsertionFragments(node, code);
              NodeList<Node> container = (NodeList<Node>) target.container;
              int idx = container.indexOf(node);
              if (idx < 0)
                throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node no longer present: " + target.id);
              if ("after".equals(position))
                idx++;
              container.addAll(idx, fragments);
              return fragments.size();
            }
      - id: JavaAstEngine.parseInsertionFragments
        type: MethodDeclaration
        code: |2
            private List<Node> parseInsertionFragments(Node sample, String code) {
              if (sample instanceof BodyDeclaration<?> && !(sample instanceof TypeDeclaration<?>))
                try {
                  return List.of(parseReplacementFor(sample, code));
                } catch (AstEngineException e) {
                  if (e.kind() != AstEngineException.Kind.SYNTAX)
                    throw e;
                  return parseMembersFragment(code);
                }
              return List.of(parseReplacementFor(sample, code));
            }
      - id: JavaAstEngine.parseMembersFragment
        type: MethodDeclaration
        code: |2
            public List<Node> parseMembersFragment(String code) {
              CompilationUnit wrapper = unwrap(parser.parse("class __Wrapper__ { " + code + " }"), "member declarations");
              ClassOrInterfaceDeclaration decl = (ClassOrInterfaceDeclaration) wrapper.getType(0);
              List<Node> result = new ArrayList<>();
              for (BodyDeclaration<?> m : decl.getMembers()) result.add(m.clone());
              if (result.isEmpty())
                throw new AstEngineException(AstEngineException.Kind.SYNTAX, "no member declarations found: " + code);
              return result;
            }
      - id: JavaAstEngine.parseImportsFragment
        type: MethodDeclaration
        code: |2
            public List<Node> parseImportsFragment(String code) {
              CompilationUnit fragment = unwrap(parser.parse(code), "imports");
              List<Node> result = new ArrayList<>();
              for (ImportDeclaration imp : fragment.getImports()) result.add(imp.clone());
              if (result.isEmpty())
                throw new AstEngineException(AstEngineException.Kind.SYNTAX, "no import declarations found: " + code);
              return result;
            }
      - id: JavaAstEngine.parseGroupFragments
        type: MethodDeclaration
        code: |2
            private List<Node> parseGroupFragments(String kind, String code) {
              return NodeGroup.KIND_IMPORTS.equals(kind) ? parseImportsFragment(code) : parseMembersFragment(code);
            }
      - id: JavaAstEngine.unwrap
        type: MethodDeclaration
        code: |2
            private static <T> T unwrap(ParseResult<T> result, String what) {
              if (result.isSuccessful() && result.getResult().isPresent()) {
                T node = result.getResult().get();
                if (node instanceof Node astNode)
                  unwrapSingleStatementBlocks(astNode);
                return node;
              }
              throw new AstEngineException(AstEngineException.Kind.SYNTAX, "invalid " + what + ": " + problemsMessage(result.getProblems()), result.getProblems().stream().map(Problem::toString).collect(Collectors.toList()));
            }
```
/answer cf6b-23-22 allow
Text:
 `insert` hat die Fallback-Logik schon (`parseInsertionFragments`). `replace` nutzt sie nicht, sondern parst nur ein einzelnes Member. Ich übernehme den Fallback dort.
ReasoningToken: 294
Control Request:
```yaml
id: cf6b-24-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: JavaAstEngine.replace
    source: |2
        @SuppressWarnings("unchecked")
        public void replace(AddressableNode target, String code) {
          if (target.astNode instanceof NodeGroup group) {
            replaceGroup(group, code);
            return;
          }
          Node node = (Node) target.astNode;
          List<Node> fragments = parseInsertionFragments(node, code);
          if (fragments.size() == 1) {
            if (!node.replace(fragments.get(0)))
              throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node could not be replaced in place: " + target.id);
            return;
          }
          NodeList<Node> container = (NodeList<Node>) target.container;
          int idx = container == null ? -1 : container.indexOf(node);
          if (idx < 0)
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node no longer present: " + target.id);
          container.set(idx, fragments.get(0));
          container.addAll(idx + 1, fragments.subList(1, fragments.size()));
        }
  reason: "replace um den Mehrfach-Member-Fallback erweitern, analog zu insert"
```
/answer cf6b-24-0 allow
Control Request:
```yaml
id: cf6b-25-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  reason: Syntax der geänderten Datei prüfen
```
/answer cf6b-25-0 allow
Result Stats: id=a5a2863b-1247-4d18-b4e5-5d40513f4a7c, total: 13747, in: 12, out: 3023, reason: 0, read: 64081, write: 10712, start: 1794081255504, end: 1794081262304, time: 6733, total: 26654, io: 0, control: 67, sent: 0, recv: 168
Ich habe `replace` in `JavaAstEngine.java` um den Fallback erweitert. Er ist weder kompiliert noch getestet; die Syntaxprüfung der Datei war fehlerfrei.

**Ursache:** `replace` hat den Ersatztext nur mit `parseReplacementFor` geparst. Für Methoden ist das `parser.parseBodyDeclaration`, und das akzeptiert genau ein Member. Bei zwei Methoden kommt deshalb „Encountered unexpected token: private“. `insert` hatte den Fallback über `parseInsertionFragments` schon, `replace` nicht.

**Änderung:**
- `replace` nutzt jetzt `parseInsertionFragments`. Das parst zuerst ein einzelnes Member und fällt bei einem Syntaxfehler auf `parseMembersFragment` zurück. Das wickelt den Code in eine Dummy-Klasse und liest alle Member heraus.
- Bei genau einem Fragment bleibt das Verhalten wie bisher (`node.replace(...)`).
- Bei mehreren Fragmenten ersetzt das erste Fragment den Zielknoten im Container. Die übrigen werden direkt dahinter eingefügt. Ist der Knoten nicht mehr im Container, gibt es einen `CONFLICT`-Fehler.
- Der Fallback gilt nur für Member, die keine Typdeklarationen sind (wie bei `insert`). Imports, Typen und Enum-Konstanten verhalten sich unverändert.

**Aktivierung:** Die Engine muss neu gebaut und neu gestartet werden. Danach sollte dein ursprünglicher Aufruf mit `parse` und `kindOf` in einem Node funktionieren.

**Folgen:**
- Die Node-ID des ersten Members bleibt nicht zwingend erhalten. Wegen des Namens `AnswerParser.parse` ist sie nach dem Ersetzen aber wieder eindeutig.
- Wer eine Methode ersetzt und dabei versehentlich weitere mitschickt, bekommt jetzt keinen Fehler mehr, sondern zusätzliche Methoden.
