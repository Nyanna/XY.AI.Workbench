package xy.ai.workbench.editor.mdast;

import xy.ai.workbench.editor.mdast.nodes.Node;

public class ModificationRange {

	private final Node node;
	private final int start;
	private final int end;

	public ModificationRange(Node node, int start, int end) {
		this.node = node;
		this.start = start;
		this.end = end;
	}

	public Node getNode() {
		return node;
	}

	public int getStart() {
		return start;
	}

	public int getEnd() {
		return end;
	}

	public int length() {
		return end - start;
	}
}
