package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.ICharacterScanner;
import org.eclipse.jface.text.rules.IRule;
import org.eclipse.jface.text.rules.IToken;

public abstract class AbstractRule implements IRule {
	protected IToken token;

	public AbstractRule(IToken tkn) {
		token = tkn;
	}

	protected boolean isNewLine(char c) {
		return c == '\n';
	}

	protected boolean isWhitespace(char c) {
		return c == ' ' || c == '\t';
	}

	protected boolean isUnderline(char c) {
		return c == '=' || c == '-';
	}

	protected boolean isSequence(ICharacterScanner scn, String str, boolean canEOF) {
		return isSequence(scn, str.toCharArray(), canEOF);
	}

	protected boolean isSequence(ICharacterScanner scn, char[] seq, boolean canEOF) {
		boolean res = true;
		int c;
		for (int i = 1; i < seq.length; i++)
			if ((c = scn.read()) == ICharacterScanner.EOF && canEOF)
				break;
			else if (c != seq[i]) {
				for (int j = i; j > 0; j--)
					scn.unread();
				res = false;
				break;
			}
		return res;
	}
}
