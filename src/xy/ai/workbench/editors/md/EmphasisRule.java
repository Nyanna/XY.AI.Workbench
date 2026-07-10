package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

public class EmphasisRule extends AbstractRule {
	private char[] seq;

	public EmphasisRule(String sequence, IToken tkn) {
		super(tkn);
		seq = sequence.toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (!s.isNextSequence(seq))
			return s.reset();

		boolean nextSequence = false;
		while (s.readNext() && !s.isNewLine() && !(nextSequence = s.isNextSequence(seq)))
			; // consume

		return nextSequence ? true : s.reset();
	}
}
