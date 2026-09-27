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
import com.github.javaparser.ast.body.EnumConstantDeclaration;
import com.github.javaparser.ast.body.EnumDeclaration;
import com.github.javaparser.ast.body.TypeDeclaration;
import com.github.javaparser.ast.comments.Comment;
import com.github.javaparser.ast.comments.JavadocComment;

import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

/**
 * JavaParser mechanics: parse/print, syntax validation, fragment parsing keyed by the
 * target node's category, and the four mutation primitives. Uses the plain
 * {@code PrettyPrinter} (via {@code Node#toString()}, which is not affected unless
 * {@code LexicalPreservingPrinter.setup} is called — which it never is here) so every
 * write normalises formatting, rather than {@code LexicalPreservingPrinter}.
 */
public final class JavaAstEngine {

    private final JavaParser parser;

    public JavaAstEngine() {
        ParserConfiguration configuration = new ParserConfiguration()
                .setLanguageLevel(ParserConfiguration.LanguageLevel.CURRENT);
        this.parser = new JavaParser(configuration);
    }

    public CompilationUnit parseCompilationUnit(String source) {
        return unwrap(parser.parse(source), "compilation unit");
    }

    public CompilationUnit emptyCompilationUnit() {
        return new CompilationUnit();
    }

    public String print(Node node) {
        return node.toString();
    }

    /** Returns an error message if {@code source} is malformed, else {@code null}. */
    public String validate(String source) {
        ParseResult<CompilationUnit> result = parser.parse(source);
        return result.isSuccessful() ? null : problemsMessage(result.getProblems());
    }

    // -- fragment parsing, one entry point per node category --

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
        if (decl.getEntries().isEmpty()) {
            throw new AstEngineException(AstEngineException.Kind.SYNTAX, "not a valid enum constant: " + code);
        }
        return decl.getEntries().get(0).clone();
    }

    /** Parses {@code code} for the node category matching {@code sample}'s own type. */
    public Node parseReplacementFor(Node sample, String code) {
        if (sample instanceof ImportDeclaration) {
            return parseImportFragment(code);
        }
        if (sample instanceof TypeDeclaration<?>) {
            return parseTypeFragment(code);
        }
        if (sample instanceof EnumConstantDeclaration) {
            return parseEnumConstantFragment(code);
        }
        return parseBodyFragment(code);
    }

    // -- CU-level append --

    public record AppendResult(CompilationUnit cu, int units) {
    }

    public AppendResult append(CompilationUnit cu, String code) {
        boolean empty = cu.getTypes().isEmpty() && cu.getImports().isEmpty() && cu.getPackageDeclaration().isEmpty();
        if (empty) {
            return new AppendResult(parseCompilationUnit(code), 1);
        }
        String trimmed = code.strip();
        if (trimmed.startsWith("import ")) {
            cu.getImports().add(parseImportFragment(trimmed));
        } else {
            cu.addType(parseTypeFragment(trimmed));
        }
        return new AppendResult(cu, 1);
    }

    // -- mutation on a resolved node --

    public void replace(AddressableNode target, String code) {
        Node replacement = parseReplacementFor(target.astNode, code);
        if (!target.astNode.replace(replacement)) {
            throw new AstEngineException(AstEngineException.Kind.CONFLICT,
                    "node could not be replaced in place: " + target.id);
        }
    }

    @SuppressWarnings("unchecked")
    public int insert(AddressableNode target, String code, String position) {
        Node fragment = parseReplacementFor(target.astNode, code);
        NodeList<Node> container = (NodeList<Node>) target.container;
        int idx = container.indexOf(target.astNode);
        if (idx < 0) {
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node no longer present: " + target.id);
        }
        if ("after".equals(position)) {
            idx++;
        }
        container.add(idx, fragment);
        return 1;
    }

    public void delete(AddressableNode target) {
        if (!target.astNode.remove()) {
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node could not be removed: " + target.id);
        }
    }

    // -- signature / docstring --

    public String signature(Node node, int limit) {
        String text;
        if (node instanceof CallableDeclaration<?> callable) {
            text = callable.getDeclarationAsString();
        } else if (node instanceof TypeDeclaration<?>) {
            text = header(node.toString());
        } else {
            text = firstLine(node.toString());
        }
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
            if (!stripped.isEmpty()) {
                return stripped;
            }
        }
        return "";
    }

    /** Short first-paragraph rendering of {@code node}'s Javadoc, if any. */
    public String docstring(Node node, int limit) {
        Optional<Comment> comment = node.getComment();
        if (comment.isEmpty() || !(comment.get() instanceof JavadocComment javadoc)) {
            return null;
        }
        String cleaned = Arrays.stream(javadoc.getContent().split("\n"))
                .map(line -> line.strip().replaceFirst("^\\*\\s?", ""))
                .filter(line -> !line.isBlank())
                .findFirst().orElse("").strip();
        if (cleaned.isEmpty()) {
            return null;
        }
        return cleaned.length() <= limit ? cleaned : cleaned.substring(0, limit - 1) + "…";
    }

    // -- helpers --

    private static <T> T unwrap(ParseResult<T> result, String what) {
        if (result.isSuccessful() && result.getResult().isPresent()) {
            return result.getResult().get();
        }
        throw new AstEngineException(AstEngineException.Kind.SYNTAX,
                "invalid " + what + ": " + problemsMessage(result.getProblems()),
                result.getProblems().stream().map(Problem::toString).collect(Collectors.toList()));
    }

    private static String problemsMessage(List<Problem> problems) {
        return problems.stream().map(Problem::getVerboseMessage).collect(Collectors.joining("; "));
    }
}
