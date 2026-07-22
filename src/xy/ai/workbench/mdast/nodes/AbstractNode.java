package xy.ai.workbench.mdast.nodes;

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

	public boolean containChild(AbstractNode child) {
		for (AbstractNode c : getChildNodes())
			if (c == child)
				return true;
		return false;
	}

	private boolean isEnd(Scanner s, Node n) {
		return n.parent != null && n.parent.instance.isEnd(s, n.parent) || isEndInner(s);
	}

	public final boolean scan(Scanner s, Node n) {
		if (!isStart(s))
			return false;

		nextChar: while (!isEnd(s, n)) {
			for (var child : getChildNodes()) {
				var nn = new Node(n, child);
				nn.start = s.getReadCount();
				Scanner sub = new Scanner(s);

				if (child.scan(sub, nn)) {
					n.children.add(nn);
					continue nextChar;
				} else
					sub.reset();
			}
			if (!s.readNext())
				break;
		}
		n.end = n.start + s.getReadCount();
		return true;
	}

	protected abstract AbstractNode[] getChildNodes();

	protected abstract boolean isStart(Scanner s);

	protected abstract boolean isEndInner(Scanner s);

	@Override
	public int hashCode() {
		throw new UnsupportedOperationException("Not implemented");
	}

	@Override
	public boolean equals(Object obj) {
		throw new UnsupportedOperationException("Not implemented");
	}
}
