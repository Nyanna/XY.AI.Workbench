package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class ScriptBlock extends AbstractNode {
	private char[] startBlock = "\n```".toCharArray();
	private char[] endBlock = "\n```\n".toCharArray();

	ScriptBlock() {
		super(Category.Block, Elements.NONE);
	}

	@Override
	protected boolean isStart(Scanner s) {
		if (!s.isNextSequenceBounded(startBlock))
			return false;

		boolean endblock = false;
		while (s.readNext() && !(endblock = s.isNextSequenceBounded(endBlock)))
			; // consume

		if (!endblock)
			return false;

		// keep trailing NL for sibling scanning
		if (!s.isEOF())
			s.unread();
		return true;
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		// the whole block is already consumed by isStart
		return true;
	}

	@Override
	public String toString() {
		return "Script";
	}
}
