package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

public class BlockRule extends AbstractRule {
	private static final int LIMIT = 20 * 200; // 20 lines a 200 chars
	private char[] startBlock;
	private char[] endBlock;

	public BlockRule(String start, String end, IToken token) {
		super(token);
		this.startBlock = start.toCharArray();
		this.endBlock = ("\n" + end).toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (s.getColumn() != 0)
			return false;

		if (!s.isNextSequence(startBlock))
			return s.reset();

		boolean endblock = false;
		while (s.getReadCount() < LIMIT && s.readNext() && !(endblock = s.isNextSequence(endBlock)))
			; // consume
		return endblock ? true : s.reset();
	}
}
