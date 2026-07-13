package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.ICharacterScanner;
import org.eclipse.jface.text.rules.IRule;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.Token;

public abstract class AbstractRule implements IRule {
	private IToken token = Token.UNDEFINED;

	public AbstractRule() {
	}

	public AbstractRule(IToken token) {
		this.token = token;
	}

	public final IToken evaluate(ICharacterScanner s) {
		return evaluateToken(new Scanner(s));
	}

	protected IToken evaluateToken(Scanner s) {
		return evaluateMatch(s) ? token : s.reset() ? null : Token.UNDEFINED;
	}

	protected boolean evaluateMatch(Scanner s) {
		return false;
	}

	protected static class Scanner {
		private char[] NUMBERS = "0123456789".toCharArray();
		private ICharacterScanner scan;
		private Scanner parent;
		private int p;
		private int c;
		private int readCount = 0;

		public Scanner(ICharacterScanner scan) {
			this.scan = scan;
		}

		public Scanner(Scanner parent) {
			this.parent = parent;
		}

		public boolean reset() {
			return unread(readCount);
		}

		public boolean isNewLine() {
			return c == '\n';
		}

		public boolean isWhitespace() {
			return isSpace() || c == '\t';
		}

		public boolean isSpace() {
			return c == ' ';
		}

		public boolean isUnderline() {
			return c == '=' || c == '-';
		}

		public boolean isEOF() {
			return c == ICharacterScanner.EOF;
		}

		public char getLast() {
			return (char) p;
		}

		public char getChar() {
			return (char) c;
		}

		public boolean isNextSequence(String str) {
			return isNextSequence(str.toCharArray());
		}

		public boolean isNextSequence(char[] seq) {
			for (int sr = 0; sr < seq.length; sr++)
				if (read() != seq[sr])
					return unread(sr + 1);
			return true;
		}

		public boolean unread(int count) {
			for (; count > 0; count--)
				unread();
			return false;
		}

		public boolean equals(char o) {
			return !isEOF() && (int) o == c;
		}

		public boolean readNext() {
			read();
			return !isEOF();
		}

		private int read() {
			readCount++;
			p = c;
			return (parent != null ? (c = parent.read()) : (c = scan.read()));
		}

		public void unread() {
			if (parent != null)
				parent.unread();
			else
				scan.unread();
			p = ICharacterScanner.EOF;
			readCount--;
		}
		
		public int getReadCount() {
			return readCount;
		}

		public int getColumn() {
			return parent != null ? parent.getColumn() : scan.getColumn();
		}

		public boolean isOneOf(char[] chars) {
			char c = getChar();
			for (char s : chars)
				if (c == s)
					return true;
			return false;
		}

		public boolean isNumber() {
			return isOneOf(NUMBERS);
		}

	}
}
