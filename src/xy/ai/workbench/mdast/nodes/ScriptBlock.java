package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class ScriptBlock extends AbstractNode {
	private static final int LIMIT = 100_000; // 50 lines a 200 chars

	private char[] startBlock = "\n```".toCharArray();
	private char[] endBlock = "\n```\n".toCharArray();
	private char[] intermediateBreak = "```".toCharArray();

	ScriptBlock() {
		super(Category.Block, Elements.NONE);
	}

	@Override
	protected boolean isStart(Scanner s) {
		if (!s.isNextSequenceBounded(startBlock))
			return false;

		boolean endblock = false, basicEnd = false;
		while (s.getReadCount() < LIMIT && s.readNext() && !(endblock = s.isNextSequenceBounded(endBlock))
				&& !(basicEnd = s.isNextSequence(intermediateBreak)))
			; // consume

		if (basicEnd || !endblock)
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

}
