package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

public class LineMatchRule extends AbstractRule {
	private char[] match;

	public LineMatchRule(String prefix, IToken token) {
		super(token);
		this.match = prefix.toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (s.getColumn() != 0)
			return false;

		if (!s.isNextSequence(match) || (s.readNext() && !s.isNewLine()))
			return s.reset();

		return true;
	}
}