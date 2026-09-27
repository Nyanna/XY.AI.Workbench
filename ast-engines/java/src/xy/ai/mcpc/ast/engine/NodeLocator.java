package xy.ai.mcpc.ast.engine;

import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.ast.ImportDeclaration;
import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.NodeList;
import com.github.javaparser.ast.body.AnnotationMemberDeclaration;
import com.github.javaparser.ast.body.BodyDeclaration;
import com.github.javaparser.ast.body.EnumConstantDeclaration;
import com.github.javaparser.ast.body.EnumDeclaration;
import com.github.javaparser.ast.body.FieldDeclaration;
import com.github.javaparser.ast.body.InitializerDeclaration;
import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.ast.body.ConstructorDeclaration;
import com.github.javaparser.ast.body.TypeDeclaration;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Pattern;

/**
 * Flattens a {@link CompilationUnit} into every addressable node, in document order —
 * the JavaParser equivalent of the Python/tree-sitter engines' {@code locate_all}.
 * Method/constructor/initializer bodies are treated as leaves (not expanded further),
 * mirroring the tree-sitter Java engine: only class/interface/enum/record/annotation
 * bodies recurse.
 *
 * Ids are name-based structural paths (e.g. {@code "MyClass.foo_2"}), rebuilt fresh on
 * every call from the live tree; stable as long as declaration order/names haven't
 * changed since the id was handed out. A node no longer found by id is reported as a
 * 404 by the caller, not silently misresolved.
 */
public final class NodeLocator {

    private static final Pattern UNSAFE = Pattern.compile("[^A-Za-z0-9_]+");

    private NodeLocator() {
    }

    public static List<AddressableNode> locateAll(CompilationUnit cu) {
        List<AddressableNode> out = new ArrayList<>();
        Map<String, Integer> usedImports = new HashMap<>();
        NodeList<ImportDeclaration> imports = cu.getImports();
        for (int i = 0; i < imports.size(); i++) {
            ImportDeclaration imp = imports.get(i);
            String seg = segment(imp.getNameAsString(), "ImportDeclaration", usedImports);
            out.add(new AddressableNode(seg, imp, "ImportDeclaration", imp.getNameAsString(),
                    line(imp), endLine(imp), "CompilationUnit", false, false, imports));
        }
        Map<String, Integer> usedTypes = new HashMap<>();
        NodeList<TypeDeclaration<?>> types = cu.getTypes();
        for (int i = 0; i < types.size(); i++) {
            walkType(types.get(i), types, "CompilationUnit", "", usedTypes, out);
        }
        return out;
    }

    private static void walkType(TypeDeclaration<?> t, NodeList<?> container, String parentType,
                                  String parentId, Map<String, Integer> used, List<AddressableNode> out) {
        String simple = simpleName(t);
        String seg = segment(t.getNameAsString(), simple, used);
        String id = parentId.isEmpty() ? seg : parentId + "." + seg;
        boolean expandable = hasDefinitionMembers(t);
        out.add(new AddressableNode(id, t, simple, t.getNameAsString(), line(t), endLine(t),
                parentType, expandable, true, container));

        Map<String, Integer> childUsed = new HashMap<>();
        if (t instanceof EnumDeclaration enumDecl) {
            NodeList<EnumConstantDeclaration> entries = enumDecl.getEntries();
            for (int i = 0; i < entries.size(); i++) {
                EnumConstantDeclaration ec = entries.get(i);
                String eseg = segment(ec.getNameAsString(), "EnumConstantDeclaration", childUsed);
                out.add(new AddressableNode(id + "." + eseg, ec, "EnumConstantDeclaration", ec.getNameAsString(),
                        line(ec), endLine(ec), simple, false, false, entries));
            }
        }
        NodeList<BodyDeclaration<?>> members = t.getMembers();
        for (int i = 0; i < members.size(); i++) {
            BodyDeclaration<?> m = members.get(i);
            if (m instanceof TypeDeclaration<?> nested) {
                walkType(nested, members, simple, id, childUsed, out);
                continue;
            }
            Described d = describe(m);
            String mseg = segment(d.name, d.type, childUsed);
            out.add(new AddressableNode(id + "." + mseg, m, d.type, d.name, line(m), endLine(m),
                    simple, false, d.isDefinition, members));
        }
    }

    private static boolean hasDefinitionMembers(TypeDeclaration<?> t) {
        for (BodyDeclaration<?> m : t.getMembers()) {
            if (m instanceof TypeDeclaration<?> || m instanceof MethodDeclaration || m instanceof ConstructorDeclaration) {
                return true;
            }
        }
        return false;
    }

    private record Described(String name, String type, boolean isDefinition) {
    }

    private static Described describe(BodyDeclaration<?> m) {
        if (m instanceof MethodDeclaration md) {
            return new Described(md.getNameAsString(), "MethodDeclaration", true);
        }
        if (m instanceof ConstructorDeclaration cd) {
            return new Described(cd.getNameAsString(), "ConstructorDeclaration", true);
        }
        if (m instanceof FieldDeclaration fd) {
            String name = fd.getVariables().isEmpty() ? null : fd.getVariable(0).getNameAsString();
            return new Described(name, "FieldDeclaration", false);
        }
        if (m instanceof InitializerDeclaration) {
            return new Described(null, "InitializerDeclaration", false);
        }
        if (m instanceof AnnotationMemberDeclaration amd) {
            return new Described(amd.getNameAsString(), "AnnotationMemberDeclaration", false);
        }
        return new Described(null, simpleName(m), false);
    }

    private static String simpleName(Node n) {
        return n.getClass().getSimpleName();
    }

    private static String segment(String name, String type, Map<String, Integer> used) {
        String base = name != null ? sanitize(name) : type;
        int count = used.merge(base, 1, Integer::sum);
        return count == 1 ? base : base + "_" + count;
    }

    private static String sanitize(String name) {
        return UNSAFE.matcher(name).replaceAll("_");
    }

    private static int line(Node n) {
        return n.getRange().map(r -> r.begin.line).orElse(1);
    }

    private static int endLine(Node n) {
        return n.getRange().map(r -> r.end.line).orElse(1);
    }
}
