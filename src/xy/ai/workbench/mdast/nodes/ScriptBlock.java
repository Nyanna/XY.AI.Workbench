package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class ScriptBlock extends AbstractNode {
	public static final ScriptBlock INSTANCE = new ScriptBlock();
	private static final int LIMIT = 100_000; // 50 lines a 200 chars

	private char[] startBlock = "\n```".toCharArray();
	private char[] endBlock = "\n```\n".toCharArray();
	private char[] intermediateBreak = "```".toCharArray();

	private ScriptBlock() {
		super(Category.Block);
	}

	@Override
	protected boolean isStart(Scanner s) {
		if (!s.isNextSequence(startBlock))
			return false;

		boolean endblock = false, basicEnd = false;
		while (s.getReadCount() < LIMIT && s.readNext() && !(endblock = s.isNextSequence(endBlock))
				&& !(basicEnd = s.isNextSequence(intermediateBreak)))
			; // consume

		if (basicEnd || !endblock)
			return false;

		s.unread(); // keep trailing NL for sibling scanning
		return true;
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		// the whole block is already consumed by isStart
		return true;
	}

	@Override
	protected AbstractNode[] getChildNodes() {
		return NO_CHILDREN;
	}
}
