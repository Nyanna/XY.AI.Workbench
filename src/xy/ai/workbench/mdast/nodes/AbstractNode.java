package xy.ai.workbench.mdast.nodes;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

import xy.ai.workbench.tools.Scanner;

public abstract class AbstractNode {
	private Category category;

	public AbstractNode(Category category) {
		Objects.requireNonNull(category);
		this.category = category;
	}

	public Category getCategory() {
		return category;
	}

	private boolean isEnd(Scanner s, Node n) {
		return n.parent != null && n.parent.instance.isEnd(s, n.parent) || isEndInner(s);
	}

	public final boolean scan(Scanner s, Node n) {
		int actual = s.getReadCount();
		if (!isStart(s))
			return false;
		n.start = actual - (n.parent != null ? n.parent.start : 0);

		nextChar: while (!isEnd(s, n)) {
			for (var child : getChildNodes()) {
				Scanner sub = new Scanner(s);
				var nn = new Node(n, child);

				if (child.scan(sub, nn)) {
					n.children.add(nn);
					continue nextChar;
				} else
					sub.reset();
			}
			if (!s.readNext())
				break;
		}
		n.end = s.getReadCount() - (n.parent != null ? n.parent.start : 0);
		return true;
	}

	protected abstract AbstractNode[] getChildNodes();

	protected abstract boolean isStart(Scanner s);

	protected abstract boolean isEndInner(Scanner s);

	public static class Node {
		public final Node parent;
		public final AbstractNode instance;
		public final List<Node> children = new ArrayList<>();
		public int start;
		public int end;

		public Node(Node parent, AbstractNode instance) {
			super();
			this.parent = parent;
			this.instance = instance;
		}

		public int getOffset() {
			return start + (parent != null ? parent.getOffset() : 0);
		}
	}

	@Override
	public int hashCode() {
		throw new UnsupportedOperationException("Not implemented");
	}

	@Override
	public boolean equals(Object obj) {
		throw new UnsupportedOperationException("Not implemented");
	}
}
