package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class HeadingSection extends AbstractNode {
	static final int MAX_ORDER = 6;

	private int order;
	private char[] prefix;
	AbstractNode[] childNodes;

	HeadingSection(int order) {
		super(Category.Section);
		this.order = order;
		this.enableSpellcheck = true;

		// starts with "\n## "
		prefix = new char[order + 2];
		prefix[0] = '\n';
		for (int i = 1; i < prefix.length - 1; i++)
			prefix[i] = '#';
		prefix[prefix.length - 1] = ' ';
	}

	@Override
	protected boolean isStart(Scanner s) {
		return s.isNextSequence(prefix);
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		Scanner sub = s.getSubscanner();
		for (int i = MAX_ORDER - order; i < Elements.HEADINGS.length; i++)
			if (Elements.HEADINGS[i].isStart(sub)) {
				sub.reset();
				return true;
			}
		return false;
	}

	@Override
	protected AbstractNode[] getChildNodes() {
		return childNodes;
	}
}
