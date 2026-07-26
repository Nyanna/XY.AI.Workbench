package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.ICharacterScanner;
import org.eclipse.jface.text.rules.IRule;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.Token;

import xy.ai.workbench.tools.Scanner;

public abstract class AbstractRule implements IRule {
	public static final String LINE_COMMENT = "#:";
	private IToken token = Token.UNDEFINED;

	private boolean docStart;
	private boolean docEnd;

	public AbstractRule() {
	}

	public AbstractRule(IToken token) {
		this.token = token;
	}

	public void setDocumentBounds(boolean docStart, boolean docEnd) {
		this.docStart = docStart;
		this.docEnd = docEnd;
	}

	public final IToken evaluate(ICharacterScanner s) {
		return evaluateToken(new Scanner(new Scanner.CharacterScanner() {
			@Override
			public void unread() {
				s.unread();
			}

			@Override
			public int read() {
				return s.read();
			}
		}, docStart, docEnd));
	}

	protected IToken evaluateToken(Scanner s) {
		return evaluateMatch(s) ? token : s.reset() ? null : Token.UNDEFINED;
	}

	protected boolean evaluateMatch(Scanner s) {
		return false;
	}
}
