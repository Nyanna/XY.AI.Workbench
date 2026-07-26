package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

import xy.ai.workbench.tools.Scanner;

public class BlockRule extends AbstractRule {
	private static final int LIMIT = 20 * 200; // 20 lines a 200 chars
	private char[] intermediateBreaks;
	private char[] startBlock;
	private char[] endBlock;

	public BlockRule(String start, String end, IToken token) {
		super(token);
		this.startBlock = ("\n" + start).toCharArray();
		this.endBlock = ("\n" + end + "\n").toCharArray();
		this.intermediateBreaks = end.toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (!s.isNextSequenceBounded(startBlock))
			return s.reset();

		boolean endblock = false, basicEnd = false;
		while (s.getReadCount() < LIMIT && s.readNext() && !(endblock = s.isNextSequence(endBlock))
				&& !(basicEnd = s.isNextSequence(this.intermediateBreaks)))
			; // consume
		if (basicEnd)
			return s.reset();
		if (!endblock)
			return s.reset();
		if (!s.isEOF())
			s.unread(); // unread trailing NL, unless doc end was reached
		return true;
	}
}
