package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

public class LineMatchRule extends AbstractRule {
	private char[] match;

	public LineMatchRule(String prefix, IToken token) {
		super(token);
		this.match = ("\n" + prefix + "\n").toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		return s.isNextSequence(match) ? true : false;
	}
}