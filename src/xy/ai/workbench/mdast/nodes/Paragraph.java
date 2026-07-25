package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class Paragraph extends AbstractNode {
	private AbstractNode[] childNodes = new AbstractNode[0];

	private char[] prefix = "\n".toCharArray();
	private char[] postfix = "\n\n".toCharArray();

	Paragraph() {
		super(Category.Section);
		this.enableSpellcheck = true;
	}

	@Override
	protected boolean isStart(Scanner s) {
		return s.isNextSequence(prefix);
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		for (int i = 0; i < Elements.HEADINGS.length; i++) {
			Scanner sub = s.getSubscanner();
			if (Elements.HEADINGS[i].isStart(sub)) {
				sub.reset();
				return true;
			}
		}

		if (s.isNextSequence(postfix)) {
			s.unread();
			return true;
		}
		return false;
	}

	@Override
	protected boolean isValid(Node n) {
		return n.end - n.start > 3;
	}

	@Override
	protected AbstractNode[] getChildNodes() {
		return childNodes;
	}
}
