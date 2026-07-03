package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.ICharacterScanner;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.Token;

public class ListRule extends AbstractRule {
	public ListRule(IToken tkn) {
		super(tkn);
	}

	public IToken evaluate(ICharacterScanner scn) {
		if (scn.getColumn() != 0)
			return Token.UNDEFINED;

		int reads = 0, c;
		while ((c = scn.read()) != ICharacterScanner.EOF) {
			reads++;
			if (!isWhitespace((char) c)) {
				if (c == '-' || c == '*' || c == '+') {
					int d = scn.read();
					scn.unread();
					if (isWhitespace((char) d))
						return token;
				}
				break;
			}
		}

		for (; reads > 0; reads--)
			scn.unread();
		return Token.UNDEFINED;
	}
}
