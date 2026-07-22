package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

import xy.ai.workbench.tools.Scanner;

public class EmphasisRule extends AbstractRule {
	private static final int MAX_READ = 200;
	private char[] sseq;
	private char[] eseq;

	public EmphasisRule(String sequence, IToken tkn) {
		this(sequence, sequence, tkn);
	}
	
	public EmphasisRule(String start, String end, IToken tkn) {
		super(tkn);
		sseq = start.toCharArray();
		eseq = end.toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		if (!s.isNextSequence(sseq))
			return s.reset();
		
		if (s.isNextSequence(eseq)) // no direct closure
			return s.reset();

		boolean nextSequence = false;
		while (s.getReadCount() <= MAX_READ && s.readNext() && !s.isNewLine() && !(nextSequence = s.isNextSequence(eseq)))
			; // consume

		return nextSequence ? true : s.reset();
	}
}
