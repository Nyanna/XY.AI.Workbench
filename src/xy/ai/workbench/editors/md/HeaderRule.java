package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

public class HeaderRule extends AbstractRule {

	public HeaderRule(IToken tkn) {
		super(tkn);
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (!s.readNext() || !s.isNewLine())
			return s.reset();

		int found = 0;
		while (s.readNext() && !s.isNewLine() && (s.isWhitespace() || s.isUnderline()))
			if (s.isUnderline())
				found++;

		return found < 3 ? s.reset() : true;
	}
}
