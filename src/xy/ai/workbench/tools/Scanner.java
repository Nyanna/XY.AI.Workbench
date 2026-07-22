package xy.ai.workbench.tools;

public class Scanner {
	private char[] NUMBERS = "0123456789".toCharArray();
	private CharacterScanner scan;
	private Scanner parent;
	private LineIndex lineIndex;
	private int p;
	private int c;
	private int readCount = 0;

	public Scanner(Scanner parent) {
		this.parent = parent;
	}

	public Scanner(CharacterScanner scan) {
		this.scan = scan;
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
		return c == CharacterScanner.EOF;
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
		Scanner sub = new Scanner(this);
		int sr = 0;
		for (; sr < seq.length && sub.readNext(); sr++)
			if (sub.getChar() != seq[sr])
				return sub.reset();
		return sr == seq.length ? true : sub.reset();
	}

	public boolean unread(int count) {
		for (; count > 0; count--)
			unread();
		return false;
	}

	public boolean read(int count) {
		for (; count > 0 && readNext(); count--)
			; // consume
		return count == 0;
	}

	public boolean equals(char o) {
		return !isEOF() && (int) o == c;
	}

	public boolean readNext() {
		read();
		if (lineIndex != null && isNewLine())
			lineIndex.addOffset(readCount);
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
		p = CharacterScanner.EOF;
		readCount--;
	}

	public int getReadCount() {
		return readCount;
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

	public void setLineIndex(LineIndex lineIndex) {
		this.lineIndex = lineIndex;
	}

	public LineIndex getLineIndex() {
		return lineIndex;
	}

	public interface CharacterScanner {

		public static final int EOF = -1;

		public abstract int read();

		public abstract void unread();
	}
}
