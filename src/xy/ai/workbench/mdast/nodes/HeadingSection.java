package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class HeadingSection extends AbstractNode {
	static final int MAX_ORDER = 6;

	private int order;
	private char[] prefix;
	AbstractNode[] terminals;

	HeadingSection(int order, AbstractNode[] childNodes, AbstractNode[] terminals) {
		super(Category.Section, childNodes);
		this.order = order;
		this.enableSpellcheck = true;
		this.terminals = terminals;

		// starts with "\n## "
		prefix = new char[order + 2];
		prefix[0] = '\n';
		for (int i = 1; i < prefix.length - 1; i++)
			prefix[i] = '#';
		prefix[prefix.length - 1] = ' ';
	}

	@Override
	protected boolean isStart(Scanner s) {
		return s.isNextSequenceBounded(prefix);
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		Scanner sub = s.getSubscanner();
		HeadingSection[] higherHeadings = Elements.Headings.HEADINGS;
		for (int i = MAX_ORDER - order; i < higherHeadings.length; i++)
			if (higherHeadings[i].isStart(sub)) {
				sub.reset();
				return true;
			}
		for (int i = 0; i < terminals.length; i++)
			if (terminals[i].isStart(sub)) {
				sub.reset();
				return true;
			}
		return false;
	}

	@Override
	public String toString() {
		return "H" + order;
	}
}
