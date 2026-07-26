package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class PageSection extends AbstractNode {
	private char[] separator = "\n---\n".toCharArray();

	PageSection() {
		super(Category.Section);
		this.enableSpellcheck = true;
	}

	@Override
	protected boolean isStart(Scanner s) {
		if (!s.isNextSequenceBounded(separator))
			return false;
		// keep trailing NL for child scanning
		if (!s.isEOF())
			s.unread();
		return true;
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		Scanner sub = s.getSubscanner();
		boolean end = sub.isNextSequence(separator);
		sub.reset();
		return end;
	}

	@Override
	protected AbstractNode[] getChildNodes() {
		return Elements.PAGE;
	}
}
