package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.ICharacterScanner;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.Token;

/**
 * Recognizes exactly three link patterns, none of which may span a line break:
 * <ol>
 * <li>Markdown link with title: {@code [url](title)}</li>
 * <li>Markdown link without title: {@code [url]}</li>
 * <li>Bare URL occurrence: {@code protocol://...}</li>
 * </ol>
 * Valid protocols for the bare-URL form are {@code http}, {@code https} and
 * {@code file}. Since the scanner marks the entire region it read as soon as
 * {@link #evaluate(ICharacterScanner)} returns a token, every character read
 * while probing a pattern that ultimately fails to match must be unread again
 * before returning {@link Token#UNDEFINED}.
 */
public class LinkRule extends AbstractRule {

	private static final String[] PROTOCOLS = { "http", "https", "file" };

	public LinkRule(IToken tkn) {
		super(tkn);
	}

	@Override
	protected boolean evaluateMatch(Scanner s) {
		return evaluateMarkdownLink(s) || evaluateBareUrl(s);
	}

	private boolean evaluateMarkdownLink(Scanner s) {
		if (!s.readNext() || !s.equals('['))
			return s.reset();

		while (s.readNext() && !s.equals(']') && !s.isNewLine())
			; // consume

		if (!s.equals(']'))
			return s.reset();

		Scanner s2 = new Scanner(s);
		if (!s2.readNext() || !s.equals('('))
			return s2.reset() || true;

		while (s2.readNext() && !s2.equals(')') && !s2.isNewLine())
			; // consume

		if (!s2.equals(')'))
			s2.reset();
		return true;
	}

	private boolean evaluateBareUrl(Scanner s) {
		if (!isProtocol(s))
			return false;
		while (s.readNext() && !isUrlTerminator(s.getChar()))
			; // consume
		return s.unread(1) || true;
	}

	private boolean isUrlTerminator(char c) {
		return Character.isWhitespace(c) || c == '"' || c == '<' || c == '>' || c == '`';
	}

	private boolean isProtocol(Scanner s) {
		for (String protocol : PROTOCOLS)
			if (s.isNextSequence(protocol + "://"))
				return true;
		return false;
	}
}
