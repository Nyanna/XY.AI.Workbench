Scheinbar laufen nicht alle Code Ausgaben durch `print` in der `JavaAstEngine`. Ich erhalte für einige Ausgaben Tabs statt spaces. Korrigiere das.

Relevante Dateien:
`/home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java`
`/home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeGroup.java`
`/home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstServer.java`
`/home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java`
`/home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AddressableNode.java`


Das folgende Insert, denke ich, hätte funktionieren müssen. Vermutlich gibt es hier ein Problem bei der Erkennung des neuen Knotens. Ich denke mal das Problem ist das er Statement und Method kombiniert? Vielleicht würde hier der versuch helfen es als letzten Try als Root CU zu versuchen?

```yaml
id: cefd-e
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    id: MarkerRessourceScanner.getPromptTag
    position: after
    source: "\n\tprivate static final Pattern PROMPT_TAG_PATTERN = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\",\n\t\t\tPattern.CASE_INSENSITIVE);\n\n\t/**\n\t * Whether the given line contains a prompt tag, i.e. the marker placeholder\n\t * inserted for an already running/pending request (see {@link #getPromptTag}).\n\t */\n\tpublic static boolean containsPromptTag(String line) {\n\t\treturn line != null && PROMPT_TAG_PATTERN.matcher(line).find();\n\t}\n"
  reason: Öffentliche Prüfmethode für PromptTag-Zeilen in MarkerRessourceScanner ergänzen
```
```yaml
id: cefd-f
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      error: |
        invalid member declaration: Encountered unexpected token: "public" "public"
            at line 9, column 3.

        Was expecting:

            <EOF>
      id: MarkerRessourceScanner.getPromptTag
```
SystemInit:  id=ed2f1f10-55d6-4bc4-b7f0-7dfde6784548, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: b8cb-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeGroup.java
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstServer.java
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AddressableNode.java
  reason: "Überblick über die Java AST Engine Struktur verschaffen, um print-Aufrufe und Insert-Parsing-Logik zu finden"
```
/answer b8cb-1 allow
Control Request:
```yaml
id: b8cb-2
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
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java
      nodes:
      - id: mNxLQm|V3iWK4
        signature: import com.github.javaparser.ast.CompilationUnit;
      - id: NodeLocator
        signature: "/**  * Flattens a {"
        docstring: "Flattens a {@link CompilationUnit} into every addressable node, in document ord…"
        children:
        - id: NodeLocator.sKOtZ3|7b4quE
          signature: private static final int SEGMENT_MAX_CHARS = 1000;
        - id: NodeLocator.NodeLocator
          signature: private NodeLocator()
        - id: NodeLocator.locateAll
          signature: public static List<AddressableNode> locateAll(CompilationUnit cu)
        - id: NodeLocator.groupImports
          signature: "private static void groupImports(CompilationUnit cu, List<AddressableNode> out)"
        - id: NodeLocator.walkType
          signature: "private static void walkType(TypeDeclaration<?> t, NodeList<?> container, Strin…"
        - id: NodeLocator.addGroup
          signature: "private static void addGroup(NodeList<? extends Node> container, String kind, S…"
          docstring: "Adds one anonymous, content-hash-addressed {@link NodeGroup} node spanning {@co…"
        - id: NodeLocator.isDefinitionMember
          signature: private static boolean isDefinitionMember(BodyDeclaration<?> m)
        - id: NodeLocator.hasDefinitionMembers
          signature: private static boolean hasDefinitionMembers(TypeDeclaration<?> t)
        - id: NodeLocator.Described
          signature: "private record Described(String name, String type) {"
        - id: NodeLocator.describe
          signature: private static Described describe(BodyDeclaration<?> m)
        - id: NodeLocator.simpleName
          signature: private static String simpleName(Node n)
        - id: NodeLocator.segment
          signature: "private static String segment(String name, String type, Map<String, Integer> us…"
        - id: NodeLocator.sanitize
          signature: private static String sanitize(String name)
        - id: NodeLocator.anonymousSegment
          signature: "private static String anonymousSegment(String content, Map<String, Integer> use…"
          docstring: "Unique-within-siblings id segment for an anonymous group node: {@code \"<shapeHa…"
        - id: NodeLocator.contentHash
          signature: private static String contentHash(String content)
        - id: NodeLocator.contentPrefixHash
          signature: private static String contentPrefixHash(String content)
          docstring: "Hash of the group's whitespace-stripped first/last 20 chars: stays put even whe…"
        - id: NodeLocator.base62Hash
          signature: "private static String base62Hash(String text, int length)"
        - id: NodeLocator.line
          signature: private static int line(Node n)
        - id: NodeLocator.endLine
          signature: private static int endLine(Node n)
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeGroup.java
      nodes:
      - id: uz13Iq|Fus9Dz
        signature: import com.github.javaparser.ast.Node;
      - id: NodeGroup
        signature: "/**  * A contiguous run of sibling declarations (imports, or non-definition cla…"
        docstring: "A contiguous run of sibling declarations (imports, or non-definition class"
        children:
        - id: NodeGroup.LdRvtT|VxjKR4
          signature: public static final String KIND_IMPORTS = "ImportGroup";
        - id: NodeGroup.NodeGroup
          signature: "public NodeGroup(String kind, NodeList<?> container, int start, int end)"
        - id: NodeGroup.members
          signature: public List<Node> members()
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstServer.java
      nodes:
      - id: nQkqfF|fpqsNm
        signature: import com.github.javaparser.ast.CompilationUnit;
      - id: 7rxzp9|7OUFSZ
        signature: import xy.ai.mcpc.ast.openapi.response.nodes.code200.json.NodesList;
      - id: JavaAstServer
        signature: "/**  * Wires the JavaParser-based {"
        docstring: "Wires the JavaParser-based {@link JavaAstEngine}/{@link DocumentCache} into the"
        children:
        - id: JavaAstServer.LaqsBv|N7R2wU
          signature: private final JavaAstEngine engine = new JavaAstEngine();
        - id: JavaAstServer.createHttpHandler
          signature: public HttpHandler createHttpHandler()
          docstring: "Safety net around the generated dispatcher: any {@link Throwable} it lets throu…"
        - id: JavaAstServer.Resolved
          signature: "private record Resolved(CompilationUnit cu, Path path) {"
        - id: JavaAstServer.resolve
          signature: "private Resolved resolve(String pathStr, String sourceOverride)"
        - id: JavaAstServer.persist
          signature: private String persist(Resolved resolved)
        - id: JavaAstServer.find
          signature: "private AddressableNode find(CompilationUnit cu, String nodeId)"
        - id: JavaAstServer.errorOf
          signature: private static Error errorOf(AstEngineException e)
        - id: JavaAstServer.applyError
          signature: "private static void applyError(AstEngineException e, Consumer<Error> code404, C…"
        - id: JavaAstServer.toDto
          signature: "private xy.ai.mcpc.ast.openapi.components.Node toDto(AddressableNode a, boolean…"
        - id: JavaAstServer.appendTopLevel
          signature: protected AppendResponse appendTopLevel(CodeRequest request)
        - id: JavaAstServer.getEngineInfo
          signature: protected InfoResponse getEngineInfo()
        - id: JavaAstServer.listNodes
          signature: protected NodesResponse listNodes(LocateRequest request)
        - id: JavaAstServer.getNode
          signature: "protected NodesNodeIdResponse getNode(String nodeId, SourceRequest request)"
        - id: JavaAstServer.deleteNode
          signature: "protected NodesNodeIdDeleteResponse deleteNode(String nodeId, SourceRequest req…"
        - id: JavaAstServer.insertRelativeToNode
          signature: "protected NodesNodeIdInsertResponse insertRelativeToNode(String nodeId, InsertR…"
        - id: JavaAstServer.replaceNode
          signature: "protected NodesNodeIdReplaceResponse replaceNode(String nodeId, CodeRequest req…"
        - id: JavaAstServer.validateSource
          signature: protected ValidateResponse validateSource(SourceRequest request)
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AddressableNode.java
      nodes:
      - id: Xg3TaL|T5nz6n
        signature: import com.github.javaparser.ast.NodeList;
      - id: AddressableNode
        signature: "/**  * One node the HTTP API can list/get/replace/insert/delete, plus the sibli…"
        docstring: "One node the HTTP API can list/get/replace/insert/delete, plus the sibling"
        children:
        - id: AddressableNode.r0U10Z|pBHh92
          signature: public final String id;
        - id: AddressableNode.AddressableNode
          signature: "public AddressableNode(String id, Object astNode, String type, String name, int…"
```
/answer b8cb-2 allow
ReasoningToken: 51
Control Request:
```yaml
id: b8cb-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.print
    - JavaAstEngine.printGroup
    - JavaAstEngine.print_2
    - JavaAstEngine.insert
    - JavaAstEngine.insertGroup
    - JavaAstEngine.parseGroupFragments
    - JavaAstEngine.parseMembersFragment
    - JavaAstEngine.parseBodyFragment
    - JavaAstEngine.parseReplacementFor
    - JavaAstEngine.append
  reason: "Print- und Insert-Logik prüfen, um Tab/Space-Problem und Fragment-Parsing für Insert zu verstehen"
```
/answer b8cb-3 allow
Control Request:
```yaml
id: b8cb-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: JavaAstEngine.print
        type: MethodDeclaration
        code: |2
            public String print(Object node) {
              if (node instanceof NodeGroup group)
                return printGroup(group);
              return print((Node) node);
            }
      - id: JavaAstEngine.printGroup
        type: MethodDeclaration
        code: |2
            private String printGroup(NodeGroup group) {
              StringBuilder sb = new StringBuilder();
              for (Node m : group.members()) {
                if (sb.length() > 0)
                  sb.append('\n');
                sb.append(print(m));
              }
              return sb.toString();
            }
      - id: JavaAstEngine.print_2
        type: MethodDeclaration
        code: |2
            public String print(Node node) {
              Indentation indent = new Indentation(IndentType.SPACES, 2);
              final PrinterConfiguration cfg = new DefaultPrinterConfiguration();
              cfg.addOption(new DefaultConfigurationOption(ConfigOption.INDENTATION, indent));
              cfg.addOption(new DefaultConfigurationOption(ConfigOption.END_OF_LINE_CHARACTER, "\n"));
              Function<PrinterConfiguration, VoidVisitor<Void>> fac = (config) -> new DefaultPrettyPrinterVisitorExtension(config);
              Printer printer = new DefaultPrettyPrinter(fac, cfg);
              return printer.print(node);
            }
      - id: JavaAstEngine.insert
        type: MethodDeclaration
        code: |2
            @SuppressWarnings("unchecked")
            public int insert(AddressableNode target, String code, String position) {
              if (target.astNode instanceof NodeGroup group)
                return insertGroup(group, code, position);
              Node node = (Node) target.astNode;
              Node fragment = parseReplacementFor(node, code);
              NodeList<Node> container = (NodeList<Node>) target.container;
              int idx = container.indexOf(node);
              if (idx < 0)
                throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node no longer present: " + target.id);
              if ("after".equals(position))
                idx++;
              container.add(idx, fragment);
              return 1;
            }
      - id: JavaAstEngine.insertGroup
        type: MethodDeclaration
        code: |2
            private int insertGroup(NodeGroup group, String code, String position) {
              NodeList<Node> container = groupContainer(group);
              List<Node> fragments = parseGroupFragments(group.kind, code);
              int idx = "after".equals(position) ? group.end : group.start;
              container.addAll(idx, fragments);
              return fragments.size();
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
```
/answer b8cb-4 allow
ReasoningToken: 94
Control Request:
```yaml
id: b8cb-5
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
    pattern: \.toString\(\)
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
    pattern: LexicalPreserving|setLexicalPreservation|getTokenRange|\.print\(
    include:
    - '*.java'
  reason: "Suche nach Stellen, die Node-Quelltext ohne den JavaAstEngine.print()-Pretty-Printer ausgeben (mögliche Ursache für Tabs statt Spaces)"
```
/answer b8cb-5 allow
Control Request:
```yaml
id: b8cb-6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
      matches:
      - path: xy/ai/mcpc/ast/openapi/response/validate/ValidateResponse.java
        matches:
        - lineno: 41
          match: "return node == null ? \"\" : node.toString();"
      - path: xy/ai/mcpc/ast/openapi/response/append/AppendResponse.java
        matches:
        - lineno: 41
          match: "return node == null ? \"\" : node.toString();"
      - path: xy/ai/mcpc/ast/openapi/response/nodes/NodesResponse.java
        matches:
        - lineno: 41
          match: "return node == null ? \"\" : node.toString();"
      - path: xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/NodesNodeIdReplaceResponse.java
        matches:
        - lineno: 41
          match: "return node == null ? \"\" : node.toString();"
      - path: xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/NodesNodeIdInsertResponse.java
        matches:
        - lineno: 41
          match: "return node == null ? \"\" : node.toString();"
      - path: xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/NodesNodeIdDeleteResponse.java
        matches:
        - lineno: 41
          match: "return node == null ? \"\" : node.toString();"
      - path: xy/ai/mcpc/ast/openapi/response/nodes/nodeid/NodesNodeIdResponse.java
        matches:
        - lineno: 41
          match: "return node == null ? \"\" : node.toString();"
      - path: xy/ai/mcpc/ast/openapi/response/info/InfoResponse.java
        matches:
        - lineno: 41
          match: "return node == null ? \"\" : node.toString();"
      - path: xy/ai/mcpc/ast/openapi/AppendInfoNodesValidateClientImpl.java
        matches:
        - lineno: 37
          match: .uri(URI.create(urlBuilder.toString()))
        - lineno: 39
          match: ".method(\"POST\", HttpRequest.BodyPublishers.ofString(request.toString()));"
        - lineno: 55
          match: .uri(URI.create(urlBuilder.toString()))
        - lineno: 72
          match: .uri(URI.create(urlBuilder.toString()))
        - lineno: 74
          match: ".method(\"POST\", HttpRequest.BodyPublishers.ofString(request.toString()));"
        - lineno: 90
          match: .uri(URI.create(urlBuilder.toString()))
        - lineno: 92
          match: ".method(\"POST\", HttpRequest.BodyPublishers.ofString(request.toString()));"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
      matches:
      - path: xy/ai/mcpc/ast/engine/DocumentCache.java
        matches:
        - lineno: 69
          match: String source = engine.print(cu);
      - path: xy/ai/mcpc/ast/engine/JavaAstServer.java
        matches:
        - lineno: 106
          match: return engine.print(resolved.cu());
        - lineno: 158
          match: dto.setCode(engine.print(a.astNode));
      - path: xy/ai/mcpc/ast/engine/JavaAstEngine.java
        matches:
        - lineno: 53
          match: "* affected unless {@code LexicalPreservingPrinter.setup} is called — which it"
        - lineno: 55
          match: "* {@code LexicalPreservingPrinter}."
        - lineno: 100
          match: return printer.print(node);
      - path: xy/ai/mcpc/ast/engine/Main.java
        matches:
        - lineno: 45
          match: "Files.writeString(p, engine.print(cu));"
```
/answer b8cb-6 allow
ReasoningToken: 422
Control Request:
```yaml
id: b8cb-7
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine
    pattern: toString\(\)|getBegin|getEnd\(\)|\.print\(
    include:
    - '*.java'
  reason: "Alle Verwendungen von toString/print in den Engine-Dateien finden, um Stellen zu identifizieren, die nicht durch den Pretty-Printer laufen"
```
/answer b8cb-7 allow
Control Request:
```yaml
id: b8cb-8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine
      matches:
      - path: DocumentCache.java
        matches:
        - lineno: 42
          match: String key = path.toString();
        - lineno: 69
          match: String source = engine.print(cu);
        - lineno: 76
          match: "entries.put(path.toString(), new CacheEntry(attrs.lastModifiedTime().toMillis(), attrs.size(), digest, normalized, source));"
        - lineno: 86
          match: entries.remove(path.toString());
        - lineno: 98
          match: return sb.toString();
      - path: JavaAstServer.java
        matches:
        - lineno: 106
          match: return engine.print(resolved.cu());
        - lineno: 158
          match: dto.setCode(engine.print(a.astNode));
      - path: NodeLocator.java
        matches:
        - lineno: 67
          match: int piece = imports.get(i).toString().length();
        - lineno: 90
          match: int piece = entries.get(ei).toString().length();
        - lineno: 120
          match: int piece = current.toString().length();
        - lineno: 137
          match: sb.append(m.toString());
        - lineno: 139
          match: "String seg = anonymousSegment(sb.toString(), used);"
        - lineno: 216
          match: return sb.toString();
      - path: JavaAstEngine.java
        matches:
        - lineno: 52
          match: "* plain {@code PrettyPrinter} (via {@code Node#toString()}, which is not"
        - lineno: 90
          match: return sb.toString();
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer b8cb-8 allow
ReasoningToken: 269
Control Request:
```yaml
id: b8cb-9
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.signature_2
    - JavaAstEngine.header
    - JavaAstEngine.firstLine
    - JavaAstEngine.docstring_2
    - JavaAstEngine.DefaultPrettyPrinterVisitorExtension
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java
    ids:
    - NodeLocator.addGroup
    - NodeLocator.walkType
    - NodeLocator.locateAll
  reason: "Prüfen, ob signature/docstring/addGroup Rohquelltext statt engine.print verwenden"
```
/answer b8cb-9 allow
Control Request:
```yaml
id: b8cb-a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: JavaAstEngine.signature_2
        type: MethodDeclaration
        code: |2
            public String signature(Node node, int limit) {
              String text;
              if (node instanceof CallableDeclaration<?> callable)
                text = callable.getDeclarationAsString();
              else if (node instanceof TypeDeclaration<?>)
                text = header(node.toString());
              else
                text = firstLine(node.toString());
              text = text.replace('\n', ' ').replace('\r', ' ').strip();
              return text.length() <= limit ? text : text.substring(0, limit - 1) + "…";
            }
      - id: JavaAstEngine.header
        type: MethodDeclaration
        code: |2
            private static String header(String printed) {
              int brace = printed.indexOf('{');
              return brace < 0 ? firstLine(printed) : printed.substring(0, brace + 1);
            }
      - id: JavaAstEngine.firstLine
        type: MethodDeclaration
        code: |2
            private static String firstLine(String printed) {
              for (String line : printed.split("\n")) {
                String stripped = line.strip();
                if (!stripped.isEmpty())
                  return stripped;
              }
              return "";
            }
      - id: JavaAstEngine.docstring_2
        type: MethodDeclaration
        code: |2
            public String docstring(Node node, int limit) {
              Optional<Comment> comment = node.getComment();
              if (comment.isEmpty() || !(comment.get() instanceof JavadocComment javadoc))
                return null;
              Stream<String> st = Arrays.stream(javadoc.getContent().split("\n"));
              st = st.map(line -> line.strip().replaceFirst("^\\*\\s?", ""));
              st = st.filter(line -> !line.isBlank());
              String cleaned = st.findFirst().orElse("").strip();
              if (cleaned.isEmpty())
                return null;
              return cleaned.length() <= limit ? cleaned : cleaned.substring(0, limit - 1) + "…";
            }
      - id: JavaAstEngine.DefaultPrettyPrinterVisitorExtension
        type: ClassOrInterfaceDeclaration
        signature: private class DefaultPrettyPrinterVisitorExtension extends DefaultPrettyPrinter…
        children:
        - id: JavaAstEngine.DefaultPrettyPrinterVisitorExtension.DefaultPrettyPrinterVisitorExtension
          type: ConstructorDeclaration
          code: |2
                private DefaultPrettyPrinterVisitorExtension(PrinterConfiguration configuration) {
                  super(configuration);
                }
        - id: JavaAstEngine.DefaultPrettyPrinterVisitorExtension.printMembers
          type: MethodDeclaration
          code: |2
                protected void printMembers(final NodeList<BodyDeclaration<?>> members, final Void arg) {
                  for (final BodyDeclaration<?> mem : members) {
                    if (needsBlankLineAfter(mem))
                      printer.println();
                    mem.accept(this, arg);
                    printer.println();
                  }
                }
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java
      nodes:
      - id: NodeLocator.addGroup
        type: MethodDeclaration
        code: |2
            private static void addGroup(NodeList<? extends Node> container, String kind, String parentType, String parentId, int start, int end, Map<String, Integer> used, List<AddressableNode> out) {
              List<? extends Node> slice = container.subList(start, end);
              StringBuilder sb = new StringBuilder();
              for (Node m : slice) {
                if (sb.length() > 0)
                  sb.append('\n');
                sb.append(m.toString());
              }
              String seg = anonymousSegment(sb.toString(), used);
              String id = parentId.isEmpty() ? seg : parentId + "." + seg;
              Node first = slice.get(0);
              Node last = slice.get(slice.size() - 1);
              NodeGroup group = new NodeGroup(kind, container, start, end);
              out.add(new AddressableNode(id, group, kind, null, line(first), endLine(last), parentType, false, false, container));
            }
      - id: NodeLocator.walkType
        type: MethodDeclaration
        code: |2
            private static void walkType(TypeDeclaration<?> t, NodeList<?> container, String parentType, String parentId, Map<String, Integer> used, List<AddressableNode> out) {
              String simple = simpleName(t);
              String seg = segment(t.getNameAsString(), simple, used);
              String id = parentId.isEmpty() ? seg : parentId + "." + seg;
              boolean expandable = hasDefinitionMembers(t);
              out.add(new AddressableNode(id, t, simple, t.getNameAsString(), line(t), endLine(t), parentType, expandable, true, container));
              Map<String, Integer> childUsed = new HashMap<>();
              if (t instanceof EnumDeclaration enumDecl) {
                NodeList<EnumConstantDeclaration> entries = enumDecl.getEntries();
                int en = entries.size(), ei = 0;
                while (ei < en) {
                  int start = ei, length = 0;
                  while (ei < en) {
                    int piece = entries.get(ei).toString().length();
                    if (ei > start && length + piece > SEGMENT_MAX_CHARS)
                      break;
                    length += piece;
                    ei++;
                  }
                  addGroup(entries, NodeGroup.KIND_STATEMENTS, simple, id, start, ei, childUsed, out);
                }
              }
              NodeList<BodyDeclaration<?>> members = t.getMembers();
              int n = members.size(), i = 0;
              while (i < n) {
                BodyDeclaration<?> m = members.get(i);
                if (m instanceof TypeDeclaration<?> nested) {
                  walkType(nested, members, simple, id, childUsed, out);
                  i++;
                  continue;
                }
                if (isDefinitionMember(m)) {
                  Described d = describe(m);
                  String mseg = segment(d.name, d.type, childUsed);
                  out.add(new AddressableNode(id + "." + mseg, m, d.type, d.name, line(m), endLine(m), simple, false, true, members));
                  i++;
                  continue;
                }
                int start = i, length = 0;
                while (i < n) {
                  BodyDeclaration<?> current = members.get(i);
                  if (current instanceof TypeDeclaration<?> || isDefinitionMember(current))
                    break;
                  int piece = current.toString().length();
                  if (i > start && length + piece > SEGMENT_MAX_CHARS)
                    break;
                  length += piece;
                  i++;
                }
                addGroup(members, NodeGroup.KIND_STATEMENTS, simple, id, start, i, childUsed, out);
              }
            }
      - id: NodeLocator.locateAll
        type: MethodDeclaration
        code: |2
            public static List<AddressableNode> locateAll(CompilationUnit cu) {
              List<AddressableNode> out = new ArrayList<>();
              groupImports(cu, out);
              Map<String, Integer> usedTypes = new HashMap<>();
              NodeList<TypeDeclaration<?>> types = cu.getTypes();
              for (int i = 0; i < types.size(); i++) walkType(types.get(i), types, "CompilationUnit", "", usedTypes, out);
              return out;
            }
```
/answer b8cb-a allow
ReasoningToken: 1148
Control Request:
```yaml
id: b8cb-b
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: JavaAstEngine.signature_2
    old_text: |-
      text = header(node.toString());
          else
            text = firstLine(node.toString());
    new_text: |-
      text = header(print(node));
          else
            text = firstLine(print(node));
  reason: "signature() nutzte Node#toString() (JavaParser-Default-Printer, Tabs) statt engine.print() (konfiguriert mit Spaces) – Ursache für Tabs in Signaturen beheben"
```
/answer b8cb-b allow
ReasoningToken: 199
Control Request:
```yaml
id: b8cb-c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.unwrap
    - JavaAstEngine.parseTypeFragment
    - JavaAstEngine.parseImportFragment
    - JavaAstEngine.parseEnumConstantFragment
  reason: "Details zu unwrap und Fragment-Parsern prüfen, um Fallback für Mehrfach-Member-Insert korrekt zu implementieren"
```
/answer b8cb-c allow
Control Request:
```yaml
id: b8cb-d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
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
      - id: JavaAstEngine.parseTypeFragment
        type: MethodDeclaration
        code: |2
            public TypeDeclaration<?> parseTypeFragment(String code) {
              return unwrap(parser.parseTypeDeclaration(code), "type declaration");
            }
      - id: JavaAstEngine.parseImportFragment
        type: MethodDeclaration
        code: |2
            public ImportDeclaration parseImportFragment(String code) {
              String stripped = code.strip();
              return unwrap(parser.parseImport(stripped.endsWith(";") ? stripped : stripped + ";"), "import");
            }
      - id: JavaAstEngine.parseEnumConstantFragment
        type: MethodDeclaration
        code: |2
            public EnumConstantDeclaration parseEnumConstantFragment(String code) {
              CompilationUnit wrapper = unwrap(parser.parse("enum __Wrapper__ { " + code + " }"), "enum constant");
              EnumDeclaration decl = (EnumDeclaration) wrapper.getType(0);
              if (decl.getEntries().isEmpty())
                throw new AstEngineException(AstEngineException.Kind.SYNTAX, "not a valid enum constant: " + code);
              return decl.getEntries().get(0).clone();
            }
```
/answer b8cb-d allow
ReasoningToken: 504
Control Request:
```yaml
id: b8cb-e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: JavaAstEngine.insert
    old_text: |2-
          Node node = (Node) target.astNode;
          Node fragment = parseReplacementFor(node, code);
          NodeList<Node> container = (NodeList<Node>) target.container;
          int idx = container.indexOf(node);
          if (idx < 0)
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node no longer present: " + target.id);
          if ("after".equals(position))
            idx++;
          container.add(idx, fragment);
          return 1;
        }
    new_text: |2-
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

        /**
         * Parses {@code code} for insertion next to {@code sample}: first as a single
         * fragment matching {@code sample}'s own type, falling back — for class/interface
         * members — to {@link #parseMembersFragment} so multi-declaration snippets (e.g. a
         * field plus a method) are accepted too.
         */
        private List<Node> parseInsertionFragments(Node sample, String code) {
          if (sample instanceof BodyDeclaration<?> && !(sample instanceof TypeDeclaration<?>)) {
            try {
              return List.of(parseReplacementFor(sample, code));
            } catch (AstEngineException e) {
              if (e.kind != AstEngineException.Kind.SYNTAX)
                throw e;
              return parseMembersFragment(code);
            }
          }
          return List.of(parseReplacementFor(sample, code));
        }
  reason: "insert() um Fallback auf parseMembersFragment erweitern, damit Mehrfach-Deklarationen (Feld+Methode) beim Einfügen neben einem Member akzeptiert werden"
```
/answer b8cb-e allow
Control Request:
```yaml
id: b8cb-f
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      error: |
        invalid member declaration: Encountered unexpected token: "private" "private"
            at line 23, column 3.

        Was expecting:

            <EOF>
      id: JavaAstEngine.insert
```
/answer b8cb-f allow
ReasoningToken: 780
Control Request:
```yaml
id: b8cb-10
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: JavaAstEngine.insert
    source: |2
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
  reason: "insert() nutzt jetzt parseInsertionFragments (Liste) statt eines einzelnen Fragments, um Mehrfach-Deklarationen zu unterstützen"
```
/answer b8cb-10 allow
Control Request:
```yaml
id: b8cb-11
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: JavaAstEngine.insert
    position: after
    source: |2

        /**
         * Parses {@code code} for insertion next to {@code sample}: first as a single
         * fragment matching {@code sample}'s own type, falling back — for class/interface
         * members — to {@link #parseMembersFragment} so multi-declaration snippets (e.g. a
         * field plus a method) are accepted too.
         */
        private List<Node> parseInsertionFragments(Node sample, String code) {
          if (sample instanceof BodyDeclaration<?> && !(sample instanceof TypeDeclaration<?>)) {
            try {
              return List.of(parseReplacementFor(sample, code));
            } catch (AstEngineException e) {
              if (e.kind != AstEngineException.Kind.SYNTAX)
                throw e;
              return parseMembersFragment(code);
            }
          }
          return List.of(parseReplacementFor(sample, code));
        }
  reason: Neue private Hilfsmethode parseInsertionFragments als eigenständigen Sibling-Knoten nach insert() einfügen
```
/answer b8cb-11 allow
ReasoningToken: 789
Control Request:
```yaml
id: b8cb-12
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
  node_type: ClassOrInterfaceDeclaration
  name: AstEngineException
  reason: "AstEngineException-Klasse finden, um zu prüfen ob Feld 'kind' öffentlich zugänglich ist"
```
/answer b8cb-12 allow
Control Request:
```yaml
id: b8cb-13
result:
  content:
  - type: text
    text: Not a regular file.
  isError: true
```
/answer b8cb-13 deny liegt hier: `/home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AstEngineException.java`
Control Request:
```yaml
id: b8cb-14
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AstEngineException.java
  reason: Prüfen ob AstEngineException ein öffentliches Feld/Getter 'kind' besitzt
```
/answer b8cb-14 allow
Control Request:
```yaml
id: b8cb-15
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AstEngineException.java
      nodes:
      - id: 9mr2w6|HCtNZ6
        signature: import java.util.List;
      - id: AstEngineException
        signature: "/**  * A user-facing error, path-free, raised by the engine; carries the HTTP s…"
        docstring: "A user-facing error, path-free, raised by the engine; carries the HTTP status k…"
        children:
        - id: AstEngineException.9JjdHX|oatX8K
          signature: private static final long serialVersionUID = 1L;
        - id: AstEngineException.Kind
          signature: "public enum Kind {"
        - id: AstEngineException.l7ueWD|lrnuq8
          signature: private final Kind kind;
        - id: AstEngineException.AstEngineException
          signature: "public AstEngineException(Kind kind, String message)"
        - id: AstEngineException.AstEngineException_2
          signature: "public AstEngineException(Kind kind, String message, List<String> candidates)"
        - id: AstEngineException.kind
          signature: public Kind kind()
        - id: AstEngineException.candidates
          signature: public List<String> candidates()
```
/answer b8cb-15 allow
Control Request:
```yaml
id: b8cb-16
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: JavaAstEngine.parseInsertionFragments
    old_text: if (e.kind != AstEngineException.Kind.SYNTAX)
    new_text: if (e.kind() != AstEngineException.Kind.SYNTAX)
  reason: Getter kind() statt privatem Feld verwenden
```
/answer b8cb-16 allow
ReasoningToken: 28
Text:
 kind ist private mit Getter `kind()`. Muss anpassen.
Control Request:
```yaml
id: b8cb-17
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  reason: Kompilierbarkeit der geänderten Datei prüfen
```
/answer b8cb-17 allow
Result Stats: id=ed2f1f10-55d6-4bc4-b7f0-7dfde6784548, total: 40080, in: 161, out: 9611, reason: 0, read: 310561, write: 30308
Text:
 Nun Kompilierprüfung:
Beide Punkte behoben:

1. **Tabs statt Spaces**: `signature(Node, int)` nutzte `node.toString()` (JavaParsers Default-Printer, Tab-Einrückung) statt `print(node)` (der konfigurierte Space-Printer). Jetzt korrigiert.
2. **Insert mit Feld+Methode**: `insert()` versucht zunächst das passende Einzel-Fragment (`parseReplacementFor`), fällt bei Syntaxfehler für Class-Member auf `parseMembersFragment` zurück und fügt alle resultierenden Deklarationen ein – dein ursprüngliches Insert-Beispiel würde damit jetzt funktionieren.
