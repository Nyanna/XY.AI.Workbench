package xy.ai.mcpc.ast.engine;

import com.github.javaparser.JavaParser;
import com.github.javaparser.ParseResult;
import com.github.javaparser.ParserConfiguration;
import com.github.javaparser.Problem;
import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.ast.ImportDeclaration;
import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.NodeList;
import com.github.javaparser.ast.body.BodyDeclaration;
import com.github.javaparser.ast.body.CallableDeclaration;
import com.github.javaparser.ast.body.ClassOrInterfaceDeclaration;
import com.github.javaparser.ast.body.ConstructorDeclaration;
import com.github.javaparser.ast.body.EnumConstantDeclaration;
import com.github.javaparser.ast.body.EnumDeclaration;
import com.github.javaparser.ast.body.InitializerDeclaration;
import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.ast.body.RecordDeclaration;
import com.github.javaparser.ast.body.TypeDeclaration;
import com.github.javaparser.ast.comments.Comment;
import com.github.javaparser.ast.comments.JavadocComment;
import com.github.javaparser.printer.Printer;
import com.github.javaparser.printer.DefaultPrettyPrinter;
import com.github.javaparser.printer.DefaultPrettyPrinterVisitor;
import com.github.javaparser.printer.configuration.DefaultConfigurationOption;
import com.github.javaparser.ast.stmt.BlockStmt;
import com.github.javaparser.ast.stmt.DoStmt;
import com.github.javaparser.ast.stmt.ForEachStmt;
import com.github.javaparser.ast.stmt.ForStmt;
import com.github.javaparser.ast.stmt.IfStmt;
import com.github.javaparser.ast.stmt.LabeledStmt;
import com.github.javaparser.ast.stmt.Statement;
import com.github.javaparser.ast.stmt.WhileStmt;
import com.github.javaparser.ast.visitor.VoidVisitor;
import java.util.ArrayList;
import com.github.javaparser.printer.configuration.DefaultPrinterConfiguration;
import com.github.javaparser.printer.configuration.DefaultPrinterConfiguration.ConfigOption;
import com.github.javaparser.printer.configuration.Indentation;
import com.github.javaparser.printer.configuration.Indentation.IndentType;
import com.github.javaparser.printer.configuration.PrinterConfiguration;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * JavaParser mechanics: parse/print, syntax validation, fragment parsing keyed
 * by the target node's category, and the four mutation primitives. Uses the
 * plain {@code PrettyPrinter} (via {@code Node#toString()}, which is not
 * affected unless {@code LexicalPreservingPrinter.setup} is called — which it
 * never is here) so every write normalises formatting, rather than
 * {@code LexicalPreservingPrinter}.
 */
public final class JavaAstEngine {
  private final JavaParser parser;

  public JavaAstEngine() {
    ParserConfiguration cfg = new ParserConfiguration().setLanguageLevel(ParserConfiguration.LanguageLevel.JAVA_25);
    cfg.setTabSize(2);
    cfg.setDetectOriginalLineSeparator(false);
    cfg.setIgnoreAnnotationsWhenAttributingComments(true);
    this.parser = new JavaParser(cfg);
  }

  public CompilationUnit parseCompilationUnit(String source) {
    return unwrap(parser.parse(source), "compilation unit");
  }

  public CompilationUnit emptyCompilationUnit() {
    return new CompilationUnit();
  }

  /** Dispatches to {@link #print(Node)} or, for a grouped segment, joins each member's own print. */
  public String print(Object node) {
    if (node instanceof NodeGroup group)
      return printGroup(group);
    return print((Node) node);
  }

  private String printGroup(NodeGroup group) {
    StringBuilder sb = new StringBuilder();
    for (Node m : group.members()) {
      if (sb.length() > 0)
        sb.append('\n');
      sb.append(print(m));
    }
    return sb.toString();
  }

  public String print(Node node) {
    Indentation indent = new Indentation(IndentType.SPACES, 2);
    final PrinterConfiguration cfg = new DefaultPrinterConfiguration();
    cfg.addOption(new DefaultConfigurationOption(ConfigOption.INDENTATION, indent));
    cfg.addOption(new DefaultConfigurationOption(ConfigOption.END_OF_LINE_CHARACTER, "\n"));
    Function<PrinterConfiguration, VoidVisitor<Void>> fac = (config) -> new DefaultPrettyPrinterVisitorExtension(config);
    Printer printer = new DefaultPrettyPrinter(fac, cfg);
    return printer.print(node);
  }

  private class DefaultPrettyPrinterVisitorExtension extends DefaultPrettyPrinterVisitor {

    private DefaultPrettyPrinterVisitorExtension(PrinterConfiguration configuration) {
      super(configuration);
    }

    protected void printMembers(final NodeList<BodyDeclaration<?>> members, final Void arg) {
      for (final BodyDeclaration<?> mem : members) {
        if (needsBlankLineAfter(mem))
          printer.println();
        mem.accept(this, arg);
        printer.println();
      }
    }
  }

  private boolean needsBlankLineAfter(BodyDeclaration<?> m) {
    return m instanceof MethodDeclaration || m instanceof ConstructorDeclaration || m instanceof ClassOrInterfaceDeclaration || m instanceof EnumDeclaration || m instanceof RecordDeclaration || m instanceof InitializerDeclaration;
  }

  /**
   * Returns an error message if {@code source} is malformed, else {@code null}.
   */
  public String validate(String source) {
    ParseResult<CompilationUnit> result = parser.parse(source);
    return result.isSuccessful() ? null : problemsMessage(result.getProblems());
  }

  public ImportDeclaration parseImportFragment(String code) {
    String stripped = code.strip();
    return unwrap(parser.parseImport(stripped.endsWith(";") ? stripped : stripped + ";"), "import");
  }

  public TypeDeclaration<?> parseTypeFragment(String code) {
    return unwrap(parser.parseTypeDeclaration(code), "type declaration");
  }

  public BodyDeclaration<?> parseBodyFragment(String code) {
    return unwrap(parser.parseBodyDeclaration(code), "member declaration");
  }

  public EnumConstantDeclaration parseEnumConstantFragment(String code) {
    CompilationUnit wrapper = unwrap(parser.parse("enum __Wrapper__ { " + code + " }"), "enum constant");
    EnumDeclaration decl = (EnumDeclaration) wrapper.getType(0);
    if (decl.getEntries().isEmpty())
      throw new AstEngineException(AstEngineException.Kind.SYNTAX, "not a valid enum constant: " + code);
    return decl.getEntries().get(0).clone();
  }

  /**
   * Parses {@code code} for the node category matching {@code sample}'s own type.
   */
  public Node parseReplacementFor(Node sample, String code) {
    if (sample instanceof ImportDeclaration)
      return parseImportFragment(code);
    if (sample instanceof TypeDeclaration<?>)
      return parseTypeFragment(code);
    if (sample instanceof EnumConstantDeclaration)
      return parseEnumConstantFragment(code);
    return parseBodyFragment(code);
  }

  public record AppendResult(CompilationUnit cu, int units) {
  }

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

  public void delete(AddressableNode target) {
    if (target.astNode instanceof NodeGroup group) {
      deleteGroup(group);
      return;
    }
    Node node = (Node) target.astNode;
    if (!node.remove())
      throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node could not be removed: " + target.id);
  }

  @SuppressWarnings("unchecked")
  private NodeList<Node> groupContainer(NodeGroup group) {
    return (NodeList<Node>) group.container;
  }

  private void replaceGroup(NodeGroup group, String code) {
    NodeList<Node> container = groupContainer(group);
    List<Node> fragments = parseGroupFragments(group.kind, code);
    for (int i = group.end - 1; i >= group.start; i--) container.remove(i);
    container.addAll(group.start, fragments);
  }

  private int insertGroup(NodeGroup group, String code, String position) {
    NodeList<Node> container = groupContainer(group);
    List<Node> fragments = parseGroupFragments(group.kind, code);
    int idx = "after".equals(position) ? group.end : group.start;
    container.addAll(idx, fragments);
    return fragments.size();
  }

  private void deleteGroup(NodeGroup group) {
    NodeList<Node> container = groupContainer(group);
    for (int i = group.end - 1; i >= group.start; i--) container.remove(i);
  }

  private List<Node> parseGroupFragments(String kind, String code) {
    return NodeGroup.KIND_IMPORTS.equals(kind) ? parseImportsFragment(code) : parseMembersFragment(code);
  }

  /** Parses {@code code} as one or more import declarations, for an {@link NodeGroup#KIND_IMPORTS} group. */
  public List<Node> parseImportsFragment(String code) {
    CompilationUnit fragment = unwrap(parser.parse(code), "imports");
    List<Node> result = new ArrayList<>();
    for (ImportDeclaration imp : fragment.getImports()) result.add(imp.clone());
    if (result.isEmpty())
      throw new AstEngineException(AstEngineException.Kind.SYNTAX, "no import declarations found: " + code);
    return result;
  }

  /** Parses {@code code} as one or more member declarations, for a {@link NodeGroup#KIND_STATEMENTS} group. */
  public List<Node> parseMembersFragment(String code) {
    CompilationUnit wrapper = unwrap(parser.parse("class __Wrapper__ { " + code + " }"), "member declarations");
    ClassOrInterfaceDeclaration decl = (ClassOrInterfaceDeclaration) wrapper.getType(0);
    List<Node> result = new ArrayList<>();
    for (BodyDeclaration<?> m : decl.getMembers()) result.add(m.clone());
    if (result.isEmpty())
      throw new AstEngineException(AstEngineException.Kind.SYNTAX, "no member declarations found: " + code);
    return result;
  }

  /** Dispatches to {@link #signature(Node, int)} or, for a grouped segment, its own first printed line. */
  public String signature(Object node, int limit) {
    if (node instanceof NodeGroup group) {
      String text = firstLine(printGroup(group)).replace('\n', ' ').replace('\r', ' ').strip();
      return text.length() <= limit ? text : text.substring(0, limit - 1) + "…";
    }
    return signature((Node) node, limit);
  }

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

  private static String header(String printed) {
    int brace = printed.indexOf('{');
    return brace < 0 ? firstLine(printed) : printed.substring(0, brace + 1);
  }

  private static String firstLine(String printed) {
    for (String line : printed.split("\n")) {
      String stripped = line.strip();
      if (!stripped.isEmpty())
        return stripped;
    }
    return "";
  }

  /** Dispatches to {@link #docstring(Node, int)}; a grouped segment never carries a Javadoc. */
  public String docstring(Object node, int limit) {
    if (node instanceof NodeGroup)
      return null;
    return docstring((Node) node, limit);
  }

  /**
   * Short first-paragraph rendering of {@code node}'s Javadoc, if any.
   */
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

  private static <T> T unwrap(ParseResult<T> result, String what) {
    if (result.isSuccessful() && result.getResult().isPresent()) {
      T node = result.getResult().get();
      if (node instanceof Node astNode)
        unwrapSingleStatementBlocks(astNode);
      return node;
    }
    throw new AstEngineException(AstEngineException.Kind.SYNTAX, "invalid " + what + ": " + problemsMessage(result.getProblems()), result.getProblems().stream().map(Problem::toString).collect(Collectors.toList()));
  }

  private static void unwrapSingleStatementBlocks(Node root) {
    for (BlockStmt block : root.findAll(BlockStmt.class)) {
      if (block.getStatements().size() != 1)
        continue;
      Statement single = block.getStatement(0);
      Node parent = block.getParentNode().orElse(null);
      if (parent instanceof IfStmt ifStmt)
        if (ifStmt.getThenStmt() == block) {
          boolean danglingElse = ifStmt.getElseStmt().isPresent() && single instanceof IfStmt innerIf && innerIf.getElseStmt().isEmpty();
          if (!danglingElse)
            ifStmt.setThenStmt(single);
        } else if (ifStmt.getElseStmt().orElse(null) == block)
          ifStmt.setElseStmt(single);
        else if (parent instanceof WhileStmt whileStmt && whileStmt.getBody() == block)
          whileStmt.setBody(single);
        else if (parent instanceof DoStmt doStmt && doStmt.getBody() == block)
          doStmt.setBody(single);
        else if (parent instanceof ForStmt forStmt && forStmt.getBody() == block)
          forStmt.setBody(single);
        else if (parent instanceof ForEachStmt forEachStmt && forEachStmt.getBody() == block)
          forEachStmt.setBody(single);
        else if (parent instanceof LabeledStmt labeledStmt && labeledStmt.getStatement() == block)
          labeledStmt.setStatement(single);
    }
  }

  private static String problemsMessage(List<Problem> problems) {
    return problems.stream().map(Problem::getVerboseMessage).collect(Collectors.joining("; "));
  }
}
