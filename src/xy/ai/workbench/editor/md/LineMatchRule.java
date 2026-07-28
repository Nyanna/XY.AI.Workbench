package xy.ai.workbench.editor.md;

import org.eclipse.jface.text.rules.IToken;

import xy.ai.workbench.tools.Scanner;

public class LineMatchRule extends AbstractRule {
	private char[] match;

	public LineMatchRule(String prefix, IToken token) {
		super(token);
		this.match = ("\n" + prefix + "\n").toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (!s.isNextSequenceBounded(match))
			return false;
		if (!s.isEOF())
			s.unread(); // unread NL, unless doc end was reached
		return true;
	}
}