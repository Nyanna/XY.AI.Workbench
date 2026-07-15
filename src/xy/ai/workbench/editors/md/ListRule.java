package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

public class ListRule extends AbstractRule {
	private char[] MARKER = "-+*".toCharArray();

	public ListRule(IToken tkn) {
		super(tkn);
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (!s.readNext() || !s.isNewLine())
			return s.reset();

		while (s.readNext() && !s.isNewLine() && s.isWhitespace())
			; // consume

		if (!s.isOneOf(MARKER) && !s.isNumber())
			return s.reset();

		if (s.isNumber()) {
			if (!s.readNext() || !s.equals('.'))
				return s.reset();
		} else if (!s.readNext() || !s.isSpace())
			return s.reset();

		return true;
	}
}
