package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.ICharacterScanner;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.Token;

public class HeaderRule extends AbstractRule {

	public HeaderRule(IToken tkn) {
		super(tkn);
	}

	@Override
	public IToken evaluate(ICharacterScanner scn) {
		if (scn.getColumn() != 0)
			return Token.UNDEFINED;

		int c = -1, count = 0;
		do {
			c = scn.read();
			count++;
		} while (!isNewLine((char) c) && c != ICharacterScanner.EOF);

		if (c == ICharacterScanner.EOF) {
			for (; count > 0; count--)
				scn.unread();
			return Token.UNDEFINED;
		}

		c = scn.read();
		count++;
		if (c == '\r') {
			c = scn.read();
			count++;
		}

		if (!isUnderline((char) c)) {
			for (; count > 0; count--)
				scn.unread();
			return Token.UNDEFINED;
		}

		while (true) {
			c = scn.read();
			count++;
			if (isNewLine((char) c) || c == ICharacterScanner.EOF)
				return token;
			if (!isUnderline((char) c) && !isWhitespace((char) c) && c != '\r') {
				for (; count > 0; count--)
					scn.unread();
				return Token.UNDEFINED;
			}
		}
	}
}
