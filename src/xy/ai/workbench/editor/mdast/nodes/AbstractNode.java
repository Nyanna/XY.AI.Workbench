package xy.ai.workbench.editor.mdast.nodes;

import java.util.Objects;

import xy.ai.workbench.tools.Scanner;

public abstract class AbstractNode {
	protected static enum Precedence {
		Childs, Terminals
	};

	private Category category;
	AbstractNode[] childNodes;
	protected boolean enableSpellcheck;
	protected Precedence precedense = Precedence.Childs;

	protected AbstractNode(Category category, AbstractNode[] childNodes) {
		Objects.requireNonNull(category);
		Objects.requireNonNull(childNodes);
		this.category = category;
		this.childNodes = childNodes;
	}

	public Category getCategory() {
		return category;
	}

	public boolean containChild(AbstractNode child) {
		for (AbstractNode c : childNodes)
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

		nextChar: while (true) {
			for (var child : childNodes) {
				if (Precedence.Terminals.equals(child.precedense) && isEnd(s, n))
					break nextChar;
				var nn = new Node(n, child);
				nn.start = s.getReadCount();
				Scanner sub = s.getSubscanner();

				if (child.scan(sub, nn)) {
					n.children.add(nn);
					nn.enableSpellcheck = child.enableSpellcheck && enableSpellcheck;
					continue nextChar;
				}
				sub.reset();
			}
			if (isEnd(s, n) || !s.readNext())
				break;
		}
		n.end = n.start + s.getReadCount();
		return isValid(n);
	}

	protected abstract boolean isStart(Scanner s);

	protected abstract boolean isEndInner(Scanner s);

	protected boolean isValid(Node n) {
		return true;
	}
}
