package xy.ai.mcpc.ast.engine;

import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.NodeList;
import java.util.List;

/**
 * A contiguous run of sibling declarations (imports, or non-definition class
 * members such as fields/initializers) collapsed into one addressable
 * "ImportGroup"/"StatementGroup" segment, mirroring the Python/tree-sitter
 * engines: JavaParser's per-declaration granularity (every import, every
 * field, on its own) is far too fine to be individually useful.
 */
public final class NodeGroup {
  public static final String KIND_IMPORTS = "ImportGroup";
  public static final String KIND_STATEMENTS = "StatementGroup";

  public final String kind;
  public final NodeList<?> container;
  public final int start;
  public final int end;

  public NodeGroup(String kind, NodeList<?> container, int start, int end) {
    this.kind = kind;
    this.container = container;
    this.start = start;
    this.end = end;
  }

  @SuppressWarnings("unchecked")
  public List<Node> members() {
    return (List<Node>) container.subList(start, end);
  }
}
