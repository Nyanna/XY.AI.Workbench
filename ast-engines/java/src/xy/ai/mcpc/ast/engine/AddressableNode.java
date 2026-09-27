package xy.ai.mcpc.ast.engine;

import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.NodeList;

/**
 * One node the HTTP API can list/get/replace/insert/delete, plus the sibling
 * {@link NodeList} needed to splice a replacement fragment in (JavaParser's
 * {@code Node.replace}/{@code Node.remove} cover the other two mutations directly).
 */
public final class AddressableNode {
  public final String id;
  public final Node astNode;
  public final String type;
  public final String name;
  public final int lineno;
  public final int endLineno;
  public final String parentType;
  public final boolean expandable;
  public final boolean isDefinition;
  public final NodeList<?> container;

  public AddressableNode(String id, Node astNode, String type, String name, int lineno, int endLineno, String parentType, boolean expandable, boolean isDefinition, NodeList<?> container) {
    this.id = id;
    this.astNode = astNode;
    this.type = type;
    this.name = name;
    this.lineno = lineno;
    this.endLineno = endLineno;
    this.parentType = parentType;
    this.expandable = expandable;
    this.isDefinition = isDefinition;
    this.container = container;
  }
}
