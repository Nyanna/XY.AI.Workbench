package xy.ai.workbench.mdast;

import xy.ai.workbench.mdast.nodes.Node;

public record TextRegion(int offset, int length, Node n) {
}