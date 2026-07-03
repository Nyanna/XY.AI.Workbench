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

		int c = -1, count = 0, underlines = 0;
		boolean abort = false;
		do {
			c = scn.read();
			count++;

			if (isUnderline((char) c))
				underlines++;
			else if (isWhitespace((char) c) || c == '\r')
				; // ok
			else if (isNewLine((char) c) || c == ICharacterScanner.EOF)
				break;
			else {
				abort = true;
				break;
			}
		} while (true);

		if (underlines < 3 || abort) {
			for (; count > 0; count--)
				scn.unread();
			return Token.UNDEFINED;
		}

		return token;
	}
}
