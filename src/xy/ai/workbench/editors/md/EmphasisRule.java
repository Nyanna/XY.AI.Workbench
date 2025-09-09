package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.ICharacterScanner;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.Token;

public class EmphasisRule extends AbstractRule {
	private char[] seq;

	public EmphasisRule(String sequence, IToken tkn) {
		super(tkn);
		seq = sequence.toCharArray();
	}

	@Override
	public IToken evaluate(ICharacterScanner scn) {
		char[][] delim = scn.getLegalLineDelimiters();
		scn.unread();
		boolean spaceBefore = Character.isWhitespace(scn.read());
		if (!spaceBefore && scn.getColumn() != 0)
			return Token.UNDEFINED;

		int c = scn.read();
		if (c != seq[0] || !isSequence(scn, seq, false)) {
			scn.unread();
			return Token.UNDEFINED;
		}
		
		int readCount = seq.length;
		int dFound = 0;
		if (spaceBefore) {
			boolean after = Character.isWhitespace(scn.read());
			if (after)
				dFound = 2;
			scn.unread();
		}

		while (dFound < 2 && (c = scn.read()) != ICharacterScanner.EOF) {
			readCount++;

			if (!spaceBefore && c == seq[0] && isSequence(scn, seq, false))
				return token;

			int i;
			for (i = 0; i < delim.length; i++)
				if (c == delim[i][0] && isSequence(scn, delim[i], true)) {
					dFound++;
					break;
				}
			if (i == delim.length)
				dFound = 0;
			spaceBefore = Character.isWhitespace(c);
		}
		for (; readCount > 0; readCount--)
			scn.unread();
		return Token.UNDEFINED;
	}
}
