package xy.ai.mcpc.ast.engine;

import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.ast.ImportDeclaration;
import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.NodeList;
import com.github.javaparser.ast.body.BodyDeclaration;
import com.github.javaparser.ast.body.ConstructorDeclaration;
import com.github.javaparser.ast.body.EnumConstantDeclaration;
import com.github.javaparser.ast.body.EnumDeclaration;
import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.ast.body.TypeDeclaration;
import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Pattern;

/**
 * Flattens a {@link CompilationUnit} into every addressable node, in document order —
 * the JavaParser equivalent of the Python/tree-sitter engines' {@code locate_all}.
 * Only types (classes/interfaces/enums/records/annotations), their nested types,
 * methods and constructors are individually addressable "definitions"; method/
 * constructor/initializer bodies are treated as leaves (not expanded further).
 * Everything else — imports at the top level, fields/initializers/annotation
 * members inside a type — is far too fine-grained to be individually useful, so
 * consecutive runs of it are collapsed into anonymous {@link NodeGroup} segments
 * capped at {@link #SEGMENT_MAX_CHARS}, exactly like the Python/tree-sitter engines'
 * "imports"/"statements" segments.
 *
 * Ids are name-based structural paths (e.g. {@code "MyClass.foo_2"}) for named
 * definitions, and stable content-hash paths (e.g. {@code "a1B2c3|d4E5f6"}) for
 * anonymous group segments, rebuilt fresh on every call from the live tree; stable
 * as long as the declaration order/names/content haven't changed since the id was
 * handed out. A node no longer found by id is reported as a 404 by the caller, not
 * silently misresolved.
 */
public final class NodeLocator {
  /** A group keeps accumulating siblings until adding the next one would push its source past this many characters (then it splits). */
  private static final int SEGMENT_MAX_CHARS = 500;
  private static final Pattern UNSAFE = Pattern.compile("[^A-Za-z0-9_]+");
  private static final String HASH_ALPHABET = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

  private NodeLocator() {
  }

  public static List<AddressableNode> locateAll(CompilationUnit cu) {
    List<AddressableNode> out = new ArrayList<>();
    groupImports(cu, out);
    Map<String, Integer> usedTypes = new HashMap<>();
    NodeList<TypeDeclaration<?>> types = cu.getTypes();
    for (int i = 0; i < types.size(); i++) walkType(types.get(i), types, "CompilationUnit", "", usedTypes, out);
    return out;
  }

  private static void groupImports(CompilationUnit cu, List<AddressableNode> out) {
    NodeList<ImportDeclaration> imports = cu.getImports();
    Map<String, Integer> used = new HashMap<>();
    int n = imports.size(), i = 0;
    while (i < n) {
      int start = i, length = 0;
      while (i < n) {
        int piece = imports.get(i).toString().length();
        if (i > start && length + piece > SEGMENT_MAX_CHARS)
          break;
        length += piece;
        i++;
      }
      addGroup(imports, NodeGroup.KIND_IMPORTS, "CompilationUnit", "", start, i, used, out);
    }
  }

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

  /** Adds one anonymous, content-hash-addressed {@link NodeGroup} node spanning {@code [start, end)} of {@code container}. */
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

  private static boolean isDefinitionMember(BodyDeclaration<?> m) {
    return m instanceof MethodDeclaration || m instanceof ConstructorDeclaration;
  }

  private static boolean hasDefinitionMembers(TypeDeclaration<?> t) {
    for (BodyDeclaration<?> m : t.getMembers()) if (m instanceof TypeDeclaration<?> || m instanceof MethodDeclaration || m instanceof ConstructorDeclaration)
      return true;
    return false;
  }

  private record Described(String name, String type) {
  }

  private static Described describe(BodyDeclaration<?> m) {
    if (m instanceof MethodDeclaration md)
      return new Described(md.getNameAsString(), "MethodDeclaration");
    ConstructorDeclaration cd = (ConstructorDeclaration) m;
    return new Described(cd.getNameAsString(), "ConstructorDeclaration");
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

  /** Unique-within-siblings id segment for an anonymous group node: {@code "<shapeHash>|<contentHash>"}, both 6-char base62. */
  private static String anonymousSegment(String content, Map<String, Integer> used) {
    String base = contentPrefixHash(content) + "|" + contentHash(content);
    int count = used.merge(base, 1, Integer::sum);
    return count == 1 ? base : base + "_" + count;
  }

  private static String contentHash(String content) {
    return base62Hash(content, 6);
  }

  /** Hash of the group's whitespace-stripped first/last 20 chars: stays put even when unrelated edits shift the group's interior. */
  private static String contentPrefixHash(String content) {
    String stripped = content.replaceAll("\\s+", "");
    int len = stripped.length();
    String prefix = stripped.substring(0, Math.min(20, len));
    String suffix = stripped.substring(Math.max(0, len - 20));
    return base62Hash(prefix + suffix, 6);
  }

  private static String base62Hash(String text, int length) {
    byte[] digestBytes;
    try {
      digestBytes = MessageDigest.getInstance("SHA-1").digest(text.getBytes(StandardCharsets.UTF_8));
    } catch (NoSuchAlgorithmException e) {
      throw new IllegalStateException(e);
    }
    BigInteger digest = new BigInteger(1, digestBytes);
    BigInteger base = BigInteger.valueOf(HASH_ALPHABET.length());
    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < length; i++) {
      BigInteger[] dr = digest.divideAndRemainder(base);
      digest = dr[0];
      sb.append(HASH_ALPHABET.charAt(dr[1].intValue()));
    }
    return sb.toString();
  }

  private static int line(Node n) {
    return n.getRange().map(r -> r.begin.line).orElse(1);
  }

  private static int endLine(Node n) {
    return n.getRange().map(r -> r.end.line).orElse(1);
  }
}
