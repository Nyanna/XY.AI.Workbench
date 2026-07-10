package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

public class RestRule extends AbstractRule {

	public RestRule(IToken defaultToken) {
		super(defaultToken);
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		s.readNext(); // min 1 char
		while (s.readNext() && s.isWhitespace())
			; // consume
		return true;
	}
}
