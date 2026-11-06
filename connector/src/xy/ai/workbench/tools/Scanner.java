package xy.ai.workbench.tools;

public class Scanner {
	private static final char[] NUMBERS = "0123456789".toCharArray();
	private CharacterScanner scan;
	private Scanner parent;
	private LineIndex lineIndex;
	private int p;
	private int c;
	private int readCount = 0;

	private boolean docStart;
	private boolean docEnd;

	public Scanner(Scanner parent) {
		this.parent = parent;
	}

	private Scanner subScanner;

	public Scanner getSubscanner() {
		if (subScanner == null)
			subScanner = new Scanner(this);
		else
			subScanner.clear();
		return subScanner;
	}

	private Scanner isNextSequenceSub;

	private Scanner getSequenceScanner() {
		if (isNextSequenceSub == null)
			isNextSequenceSub = new Scanner(this);
		else
			isNextSequenceSub.clear();
		return isNextSequenceSub;
	}

	public Scanner(CharacterScanner scan) {
		this(scan, false, false);
	}

	public Scanner(CharacterScanner scan, boolean docStart, boolean docEnd) {
		this.scan = scan;
		this.docStart = docStart;
		this.docEnd = docEnd;
	}

	public void clear() {
		p = c = readCount = 0;
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
		return matchSequence(seq, 0);
	}

	public boolean isNextSequenceBounded(char[] seq) {
		int from = seq.length > 0 && seq[0] == '\n' && isDocStart() ? 1 : 0;
		return matchSequence(seq, from);
	}

	private boolean matchSequence(char[] seq, int from) {
		Scanner sub = getSequenceScanner();
		int sr = from;
		for (; sr < seq.length && sub.readNext(); sr++)
			if (sub.getChar() != seq[sr])
				return sub.reset();
		if (sr == seq.length)
			return true;
		// ran out of real input before completing the match; tolerate a
		// missing trailing '\n' exactly at the true doc end
		if (sr == seq.length - 1 && seq[sr] == '\n' && isDocEnd())
			return true;
		return sub.reset();
	}

	public boolean isDocStart() {
		Scanner root = this;
		while (root.parent != null)
			root = root.parent;
		return root.docStart && root.readCount == 0;
	}

	public boolean isDocEnd() {
		Scanner root = this;
		while (root.parent != null)
			root = root.parent;
		return root.docEnd;
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
		int prev = c;
		int next = (parent != null ? parent.read() : scan.read());
		if (next != CharacterScanner.EOF)
			readCount++;
		p = prev;
		c = next;
		return next;
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
