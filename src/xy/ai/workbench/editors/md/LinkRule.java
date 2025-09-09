package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.ICharacterScanner;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.Token;

public class LinkRule extends AbstractRule {

	public LinkRule(IToken tkn) {
		super(tkn);
	}

	public IToken evaluate(ICharacterScanner scn) {
		char[][] delims = scn.getLegalLineDelimiters();
		int c;
		if ((c = scn.read()) != '[') {
			if (isNotProtocoll(scn)) {
				scn.unread();
				return Token.UNDEFINED;
			}

			while ((c = scn.read()) != ICharacterScanner.EOF && !Character.isWhitespace(c))
				for (int i = 0; i < delims.length; i++)
					if (c == delims[i][0] && isSequence(scn, delims[i], true))
						return token;
			return token;
		}
		int readCount = 1;

		boolean sequenceFound = false;
		int delimiterCount = 0;
		while ((c = scn.read()) != ICharacterScanner.EOF && delimiterCount < 2) {
			readCount++;
			if (!sequenceFound && c == ']') {
				c = scn.read();
				if (c == '(') {
					readCount++;
					sequenceFound = true;
				} else
					scn.unread();

			} else if (c == ')')
				return token;

			int i;
			for (i = 0; i < delims.length; i++)
				if (c == delims[i][0] && isSequence(scn, delims[i], true)) {
					delimiterCount++;
					break;
				}
			if (i == delims.length)
				delimiterCount = 0;
		}

		for (; readCount > 0; readCount--)
			scn.unread();
		return Token.UNDEFINED;
	}

	private boolean isNotProtocoll(ICharacterScanner scn) {
		for (String prot : new String[] { "http", "https", "file" })
			if (isSequence(scn, prot + "://", false))
				return false;
		return true;
	}

}
