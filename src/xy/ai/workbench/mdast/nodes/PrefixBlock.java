package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class PrefixBlock extends AbstractNode {
	private char[] prefix;

	PrefixBlock(String marker) {
		super(Category.Block, Elements.NONE);
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
	public String toString() {
		return String.valueOf(prefix).replace('\n', ' ');
	}
}
