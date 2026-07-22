package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;

import xy.ai.workbench.tools.Scanner;

public class WordRule extends AbstractRule {
	private char[] word;

	public WordRule(String word, IToken token) {
		super(token);
		this.word = word.toCharArray();
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		return !s.isNextSequence(word) ? s.reset() : true;
	}
}
