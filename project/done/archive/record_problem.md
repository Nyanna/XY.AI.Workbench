Der Java AST in `/home/user/xyan/xy.ai.workbench/ast-engines/java/src` scheint ein Problem zu haben und konnte den record nicht ersetzen.

Die Änderung an `ApprovalContext` ist fehlgeschlagen, weil das Werkzeug den Record nicht über Textersetzung bearbeiten kann. Ich ersetze deshalb den ganzen Knoten.

```yaml
id: dabf-1f-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    id: ApprovalContext
    source: "/**\n * Input of a rule.\n *\n * @param item {@code null} if the answer contains no (or more than one) tool call/result\n * @param call the remembered tool call {@code item} is the result of; {@code null} if {@code item} is no result or\n *        the call is unknown (e.g. older than the remembered calls)\n */\npublic record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call,\n\t\tSessionSnapshot session) {\n\n\tpublic boolean hasItem() {\n\t\treturn item != null;\n\t}\n}\n"
    
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
SystemInit:  id=1c80e676-0ca8-4575-9970-218ee6bd9cfb, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Der Fehler ("Was expecting: class, enum, interface") sieht nach einem zu niedrigen Java-Language-Level im Parser aus, vermutlich JavaParser. Ich suche die Parser-Konfiguration in der Engine.
ReasoningToken: 124
Control Request:
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
    - '*.xml'
    - '*.gradle'
    - '*.kts'
    exclude:
    - build/**
    - target/**
    limit: 50
  reason: "Parser-Konfiguration (Language Level) der Java-AST-Engine finden, um zu klären, warum 'record' nicht geparst wird."
```
/answer 7cbd-1-0 allow
Control Request:
```yaml
id: 7cbd-2-1
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
      matches:
      - path: xy/ai/mcpc/ast/engine/JavaAstServer.java
        matches:
        - "* Wires the JavaParser-based {@link JavaAstEngine}/{@link DocumentCache} into the"
      - path: xy/ai/mcpc/ast/engine/AddressableNode.java
        matches:
        - "* {@link NodeList} needed to splice a replacement fragment in (JavaParser's"
        - "* {@code astNode} is either a JavaParser {@link com.github.javaparser.ast.Node} or,"
      - path: xy/ai/mcpc/ast/engine/NodeLocator.java
        matches:
        - "* the JavaParser equivalent of the Python/tree-sitter engines' {@code locate_all}."
      - path: xy/ai/mcpc/ast/engine/JavaAstEngine.java
        matches:
        - import com.github.javaparser.JavaParser;
        - import com.github.javaparser.ParserConfiguration;
        - "* JavaParser mechanics: parse/print, syntax validation, fragment parsing keyed"
        - private final JavaParser parser;
        - ParserConfiguration cfg = new ParserConfiguration().setLanguageLevel(ParserConfiguration.LanguageLevel.JAVA_25);
        - this.parser = new JavaParser(cfg);
      - path: xy/ai/mcpc/ast/engine/Main.java
        matches:
        - '* Starts the JavaParser AST engine as a standalone JSON/HTTP backed engine.'
      - path: xy/ai/mcpc/ast/engine/NodeGroup.java
        matches:
        - "* engines: JavaParser's per-declaration granularity (every import, every"
```
/answer 7cbd-2-1 allow
Control Request:
```yaml
id: 7cbd-3-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
    pattern: pom\.xml|build\.gradle|\.java$
    recursive: true
  reason: Struktur der Java-AST-Engine und Build-Dateien überblicken.
```
/answer 7cbd-3-0 allow
Control Request:
```yaml
id: 7cbd-4-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
      entries:
      - "./src/xy/ai/mcpc/ast/engine: (8 files)"
      - AddressableNode.java
      - "./src/xy/ai/mcpc/ast/openapi: (4 files)"
      - AppendInfoNodesValidateClient.java
      - "./src/xy/ai/mcpc/ast/openapi/components: (6 files)"
      - CandidatesList.java
      - "./src/xy/ai/mcpc/ast/openapi/request/nodes/nodeid/insert/post/json: (2 files)"
      - InsertRequest.java
      - "./src/xy/ai/mcpc/ast/openapi/request/nodes/post/json: (1 file)"
      - LocateRequest.java
      - "./src/xy/ai/mcpc/ast/openapi/response/append: (1 file)"
      - AppendResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/append/code200/json: (2 files)"
      - AppendResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/append/code422/json: (1 file)"
      - AppendResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/info: (1 file)"
      - InfoResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/info/code200/json: (2 files)"
      - EngineInfo.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes: (1 file)"
      - NodesResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/code200/json: (3 files)"
      - LocateResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/code422/json: (1 file)"
      - NodesResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid: (1 file)"
      - NodesNodeIdResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code200/json: (1 file)"
      - NodesNodeIdResponseCode200Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code404/json: (1 file)"
      - NodesNodeIdResponseCode404Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code409/json: (1 file)"
      - NodesNodeIdResponseCode409Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code422/json: (1 file)"
      - NodesNodeIdResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete: (1 file)"
      - NodesNodeIdDeleteResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code200/json: (1 file)"
      - NodesNodeIdDeleteResponseCode200Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code404/json: (1 file)"
      - NodesNodeIdDeleteResponseCode404Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code409/json: (1 file)"
      - NodesNodeIdDeleteResponseCode409Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code422/json: (1 file)"
      - NodesNodeIdDeleteResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert: (1 file)"
      - NodesNodeIdInsertResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code200/json: (2 files)"
      - InsertResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code404/json: (1 file)"
      - NodesNodeIdInsertResponseCode404Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code409/json: (1 file)"
      - NodesNodeIdInsertResponseCode409Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code422/json: (1 file)"
      - NodesNodeIdInsertResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace: (1 file)"
      - NodesNodeIdReplaceResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code200/json: (1 file)"
      - NodesNodeIdReplaceResponseCode200Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code404/json: (1 file)"
      - NodesNodeIdReplaceResponseCode404Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code409/json: (1 file)"
      - NodesNodeIdReplaceResponseCode409Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code422/json: (1 file)"
      - NodesNodeIdReplaceResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/validate: (1 file)"
      - ValidateResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/validate/code200/json: (2 files)"
      - ValidateResponse.java
      warning: Result reduced because the total number of entries (57) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 7cbd-4-3 allow
Text:
 Der Parser ist schon auf `JAVA_25` gestellt, die Ursache liegt also woanders. Ich lese `JavaAstEngine`, besonders das Parsen von Fragmenten.
ReasoningToken: 46
Control Request:
```yaml
id: 7cbd-5-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  reason: "Aufbau von JavaAstEngine ansehen, um die Fragment-Parsing-Logik zu finden."
```
/answer 7cbd-5-0 allow
Control Request:
```yaml
id: 7cbd-6-5
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
/answer 7cbd-6-5 allow
Control Request:
```yaml
id: 7cbd-7-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.JavaAstEngine
    - JavaAstEngine.parseTypeFragment
    - JavaAstEngine.parseBodyFragment
    - JavaAstEngine.parseReplacementFor
    - JavaAstEngine.replace
    - JavaAstEngine.unwrap
  reason: "Konstruktor, Fragment-Parser und replace lesen, um zu sehen, wie ein Record-Fragment geparst wird."
```
/answer 7cbd-7-0 allow
Control Request:
```yaml
id: 7cbd-8-7
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: JavaAstEngine.JavaAstEngine
        type: ConstructorDeclaration
        code: |2
            public JavaAstEngine() {
              ParserConfiguration cfg = new ParserConfiguration().setLanguageLevel(ParserConfiguration.LanguageLevel.JAVA_25);
              cfg.setTabSize(2);
              cfg.setDetectOriginalLineSeparator(false);
              cfg.setIgnoreAnnotationsWhenAttributingComments(true);
              this.parser = new JavaParser(cfg);
            }
      - id: JavaAstEngine.parseTypeFragment
        type: MethodDeclaration
        code: |2
            public TypeDeclaration<?> parseTypeFragment(String code) {
              return unwrap(parser.parseTypeDeclaration(code), "type declaration");
            }
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
/answer 7cbd-8-7 allow
Thinking:
Die Meldung "line 8, column 8" zeigt auf `record` in `public record …`, also kennt der Parser Records trotz gesetztem `JAVA_25` nicht. Ich prüfe jetzt die JavaParser-Version und die vorhandenen Tests.


Control Request:
```yaml
id: 7cbd-9-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
    pattern: (pom\.xml|\.gradle(\.kts)?|\.classpath|\.jar|Test.*\.java|\.sh|README.*)$
    recursive: true
  - path: /home/user/xyan/xy.ai.workbench/ast-engines
    pattern: "^[^/]*$"
    recursive: false
  reason: "Build-Dateien, Jars und Tests der Java-Engine finden, um die JavaParser-Version zu bestimmen."
```
/answer 7cbd-9-0 deny das ist raten und nicht suchen
ReasoningToken: 729
Control Request:
```yaml
id: 7cbd-a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java
    pattern: javaparser
    include:
    - pom.xml
    - '*.gradle'
    - '*.kts'
    - '*.classpath'
    - '*.properties'
    - '*.MF'
    exclude:
    - node_modules/**
    limit: 20
  reason: JavaParser-Dependency-Version nachschlagen (JAVA_25 / Record-Unterstützung).
```
/answer 7cbd-a-0 allow
Control Request:
```yaml
id: 7cbd-b-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java
      matches:
      - path: .classpath
        matches:
        - <classpathentry kind="lib" path="libs/javaparser-core-3.28.2.jar" sourcepath="libs/javaparser-core-3.28.2-sources.jar">
        - <attribute name="javadoc_location" value="jar:platform:/resource/XY.AI.ast-java/libs/javaparser-core-3.28.2-javadoc.jar!/"/>
```
/answer 7cbd-b-a allow
Thinking:
Die Abhängigkeit ist JavaParser 3.28.2, und ich lese jetzt die Stellen, die Typ-Fragmente parsen.


Control Request:
```yaml
id: 7cbd-c-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.append
    - JavaAstEngine.parseInsertionFragments
    - JavaAstEngine.parseGroupFragments
    - JavaAstEngine.parseMembersFragment
    - JavaAstEngine.validate
  reason: "Alle Aufrufer von parseTypeFragment/parseBodyFragment prüfen, damit der Fix an der richtigen Stelle ansetzt und nichts anderes bricht."
```
/answer 7cbd-c-0 allow
Control Request:
```yaml
id: 7cbd-d-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: JavaAstEngine.append
        type: MethodDeclaration
        code: |2
            public AppendResult append(CompilationUnit cu, String code) {
              boolean empty = cu.getTypes().isEmpty() && cu.getImports().isEmpty() && cu.getPackageDeclaration().isEmpty();
              if (empty)
                return new AppendResult(parseCompilationUnit(code), 1);
              String trimmed = code.strip();
              if (trimmed.startsWith("import "))
                cu.getImports().add(parseImportFragment(trimmed));
              else
                cu.addType(parseTypeFragment(trimmed));
              return new AppendResult(cu, 1);
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
      - id: JavaAstEngine.parseGroupFragments
        type: MethodDeclaration
        code: |2
            private List<Node> parseGroupFragments(String kind, String code) {
              return NodeGroup.KIND_IMPORTS.equals(kind) ? parseImportsFragment(code) : parseMembersFragment(code);
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
      - id: JavaAstEngine.validate
        type: MethodDeclaration
        code: |2
            public String validate(String source) {
              ParseResult<CompilationUnit> result = parser.parse(source);
              return result.isSuccessful() ? null : problemsMessage(result.getProblems());
            }
```
/answer 7cbd-d-c allow
ReasoningToken: 2014
Control Request:
```yaml
id: 7cbd-e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
    pattern: parseTypeFragment|parseTypeDeclaration|RecordDeclaration
    include:
    - '*.java'
    exclude:
    - build/**
    - target/**
    limit: 30
    intent: editing
  reason: Alle Verwendungen von parseTypeFragment und RecordDeclaration in der Engine finden.
```
/answer 7cbd-e-0 allow
Control Request:
```yaml
id: 7cbd-f-e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
      matches:
      - path: xy/ai/mcpc/ast/engine/JavaAstEngine.java
        nodes:
        - id: NHeqvp|VsGI8e
          matches:
          - import com.github.javaparser.ast.body.RecordDeclaration;
        - id: JavaAstEngine.needsBlankLineAfter
          matches:
          - return m instanceof MethodDeclaration || m instanceof ConstructorDeclaration || m instanceof ClassOrInterfaceDeclaration || m instanceof EnumDeclaration || m instanceof RecordDeclaration || m instanceof InitializerDeclaration;
        - id: JavaAstEngine.parseTypeFragment
          matches:
          - "public TypeDeclaration<?> parseTypeFragment(String code) {"
          - "return unwrap(parser.parseTypeDeclaration(code), \"type declaration\");"
        - id: JavaAstEngine.parseReplacementFor
          matches:
          - return parseTypeFragment(code);
        - id: JavaAstEngine.append
          matches:
          - cu.addType(parseTypeFragment(trimmed));
```
/answer 7cbd-f-e allow
Thinking:
Die Ursache liegt im Typ-Knoten beim Replace: `parseTypeDeclaration` lehnt `record` ab, während der Compilation-Unit-Pfad Records verarbeitet. Ich ergänze in `parseTypeFragment` einen Fallback über den Compilation-Unit-Parser.


ReasoningToken: 1103
Control Request:
```yaml
id: 7cbd-10-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: JavaAstEngine.parseTypeFragment
    source: |2
        public TypeDeclaration<?> parseTypeFragment(String code) {
          ParseResult<TypeDeclaration<?>> result = parser.parseTypeDeclaration(code);
          if (result.isSuccessful() && result.getResult().isPresent())
            return unwrap(result, "type declaration");
          // parseTypeDeclaration does not accept records, the compilation unit grammar does
          ParseResult<CompilationUnit> unit = parser.parse(code);
          if (unit.isSuccessful() && unit.getResult().isPresent()) {
            CompilationUnit cu = unit.getResult().get();
            if (cu.getTypes().size() == 1 && cu.getImports().isEmpty() && cu.getPackageDeclaration().isEmpty()) {
              TypeDeclaration<?> type = cu.getType(0).clone();
              unwrapSingleStatementBlocks(type);
              return type;
            }
          }
          return unwrap(result, "type declaration");
        }
  reason: "parseTypeFragment um einen Compilation-Unit-Fallback erweitern, damit Records ersetzt, eingefügt und angehängt werden können."
```
/answer 7cbd-10-0 allow
Control Request:
```yaml
id: 7cbd-11-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  reason: Syntax der geänderten Datei prüfen.
```
/answer 7cbd-11-0 allow
Result Stats: id=1c80e676-0ca8-4575-9970-218ee6bd9cfb, total: 29668, in: 16, out: 7607, reason: 0, read: 101125, write: 22045, start: 1794080201901, end: 1794080208914, time: 6945, total: 61990, io: 0, control: 68, sent: 0, recv: 165
Ich habe die Ursache im Java-AST-Engine gefunden und einen Fix eingebaut. Gebaut und getestet habe ich ihn noch nicht. `ast_validate` bestätigt nur, dass die Datei syntaktisch gültig ist.

**Ursache:** Der Fehler lag nicht am Language Level, denn der Parser ist bereits auf `JAVA_25` gestellt. `parseTypeFragment` in `JavaAstEngine.java` ruft `parser.parseTypeDeclaration(code)` auf, und dieser Einstiegspunkt von JavaParser 3.28.2 lehnt `record` ab. Die Meldung "line 8, column 8" zeigt genau auf das Token `record` hinter `public`. Dass `record` grundsätzlich unterstützt wird, sieht man an `JavaAstEngine.java`: Sie enthält selbst `record AppendResult` und lässt sich fehlerfrei parsen. Das gilt auch für den Wrapper-Pfad in `parseMembersFragment`.

**Fix:** `parseTypeFragment` versucht zuerst wie bisher `parseTypeDeclaration`. Schlägt das fehl, parst es den Code als Compilation Unit. Das Ergebnis wird nur übernommen, wenn es genau einen Typ ohne `package` und ohne Imports enthält, damit nichts stillschweigend verloren geht. Sonst kommt wie bisher die ursprüngliche Fehlermeldung. Der Fix wirkt auch bei `replace` und `append`, weil beide über diese Methode laufen, und bei `insert` für Typ-Knoten.

**Noch zu tun:**
- Der Engine muss neu gebaut und neu gestartet werden. Der laufende Server nutzt noch den alten Stand.
- Danach kann der `ast_replace` für `ApprovalContext` wiederholt werden, mit dem unveränderten Aufruf aus deiner Nachricht.

Ich konnte den Code nicht ausführen und habe deshalb nicht geprüft, ob der Fallback mit Javadoc vor dem `record` den Kommentar korrekt an den Typ hängt. Das wäre der erste Punkt, den ich beim Test ansehen würde.
Ich habe neu gestartet
SystemInit:  id=1c80e676-0ca8-4575-9970-218ee6bd9cfb, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Dann wiederhole ich den ursprünglichen Replace für `ApprovalContext`.
ReasoningToken: 81
Control Request:
```yaml
id: cf6b-1-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    id: ApprovalContext
    source: "/**\n * Input of a rule.\n *\n * @param item {@code null} if the answer contains no (or more than one) tool call/result\n * @param call the remembered tool call {@code item} is the result of; {@code null} if {@code item} is no result or\n *        the call is unknown (e.g. older than the remembered calls)\n */\npublic record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call,\n\t\tSessionSnapshot session) {\n\n\tpublic boolean hasItem() {\n\t\treturn item != null;\n\t}\n}\n"
  reason: Den zuvor fehlgeschlagenen Replace des Records ApprovalContext nach dem Engine-Neustart wiederholen.
```
/answer cf6b-1-0 allow
Text:
 Ich prüfe noch, ob das Javadoc am Record hängt und die Datei gültig ist.
Control Request:
```yaml
id: cf6b-2-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    ids:
    - ApprovalContext
  reason: "Ergebnis nach dem Replace kontrollieren, besonders die Zuordnung des Javadoc-Kommentars."
```
/answer cf6b-2-0 allow
Control Request:
```yaml
id: cf6b-3-2
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
          code: |2
              public boolean hasItem() {
                return item != null;
              }
```
/answer cf6b-3-2 allow
Control Request:
```yaml
id: cf6b-4-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  reason: Syntax der geänderten Datei prüfen.
```
/answer cf6b-4-0 allow
Result Stats: id=1c80e676-0ca8-4575-9970-218ee6bd9cfb, total: 33215, in: 22, out: 8919, reason: 0, read: 169567, write: 24274, start: 1794080314367, end: 1794080321273, time: 6841, total: 100229, io: 0, control: 65, sent: 0, recv: 92
Der Replace von `ApprovalContext` hat nach dem Neustart funktioniert.

- **Record:** Der Knoten ist jetzt ein `RecordDeclaration`, mit `hasItem()` als Kind.
- **Javadoc:** Es hängt am Record. Der Docstring "Input of a rule." wird erkannt, und `@param item` steht im Signaturtext.
- **Syntax:** `ast_validate` meldet die Datei als gültig.

Der Fallback in `parseTypeFragment` funktioniert also, auch mit Javadoc vor dem Record. Das war die Stelle, die ich vorher nicht prüfen konnte.

Der Printer hat die Einrückung der Datei auf zwei Leerzeichen umgestellt. Das ist die Konfiguration der Engine (`setTabSize(2)`). In deinem Quelltext standen Tabs. Falls die Datei sonst Tabs nutzt, siehst du im Diff eine Einrückungsänderung. Das liegt am Printer und hat mit dem Fix nichts zu tun.

Ich habe nicht geprüft, ob das Projekt kompiliert, nur die Syntax.
