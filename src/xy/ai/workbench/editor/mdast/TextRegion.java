package xy.ai.workbench.editor.mdast;

import xy.ai.workbench.editor.mdast.nodes.Node;

public record TextRegion(int offset, int length, Node n) {
}