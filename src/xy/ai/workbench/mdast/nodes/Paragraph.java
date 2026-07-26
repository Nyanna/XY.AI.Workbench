package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class Paragraph extends AbstractNode {
	private final char[] prefix = "\n".toCharArray();
	private final char[] postfix = "\n\n".toCharArray();
	AbstractNode[] terminals;

	Paragraph(AbstractNode[] terminals) {
		super(Category.Section, Elements.NONE);
		this.enableSpellcheck = true;
		this.terminals = terminals;
	}

	@Override
	protected boolean isStart(Scanner s) {
		return s.isNextSequenceBounded(prefix);
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		for (int i = 0; i < terminals.length; i++) {
			Scanner sub = s.getSubscanner();
			if (terminals[i].isStart(sub)) {
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
}
