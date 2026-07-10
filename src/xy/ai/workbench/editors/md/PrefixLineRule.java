package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

public class PrefixLineRule extends AbstractRule {
	private char[] prefix;

	public PrefixLineRule(String prefix, IToken token) {
		super(token);
		this.prefix = prefix.toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (s.getColumn() != 0)
			return false;

		if (!s.isNextSequence(prefix))
			return s.reset();

		while (s.readNext() && !s.isNewLine())
			; // consume
		return true;
	}
}