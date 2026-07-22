package xy.ai.workbench.mdast.nodes;

import java.util.ArrayList;
import java.util.List;

public class Node {
	public Node parent;
	public final AbstractNode instance;
	public final List<Node> children = new ArrayList<>();
	public int start;
	public int end;

	public Node(Node parent, AbstractNode instance) {
		super();
		this.parent = parent;
		this.instance = instance;
	}

	public int length() {
		return end - start;
	}

	public int getOffset() {
		return start + (parent != null ? parent.getOffset() : 0);
	}

	public int getEndOffset() {
		return getOffset() + length();
	}
}