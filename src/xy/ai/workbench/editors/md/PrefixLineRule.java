package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

public class PrefixLineRule extends AbstractRule {
	private static final int MAX_READ = 200;
	private char[] prefix;

	public PrefixLineRule(String prefix, IToken token) {
		super(token);
		this.prefix = ("\n" + prefix).toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (!s.isNextSequence(prefix))
			return s.reset();

		boolean nl = false;
		while (s.getReadCount() <= MAX_READ && s.readNext() && !(nl = s.isNewLine()))
			; // consume
		if(nl)
			s.unread();
		return true;
	}
}