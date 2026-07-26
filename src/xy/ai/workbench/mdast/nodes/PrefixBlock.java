package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class PrefixBlock extends AbstractNode {
	private char[] prefix;

	PrefixBlock(String marker) {
		super(Category.Block);
		this.prefix = ("\n" + marker).toCharArray();
	}

	@Override
	protected boolean isStart(Scanner s) {
		return s.isNextSequenceBounded(prefix);
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		Scanner sub = s.getSubscanner();
		boolean end = !sub.readNext() || sub.isNewLine();
		sub.reset();
		return end;
	}

	@Override
	protected AbstractNode[] getChildNodes() {
		return Elements.NONE;
	}
}
