package xy.ai.workbench.editors;

import java.util.ArrayList;
import java.util.IdentityHashMap;
import java.util.List;
import java.util.Map;

import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.TextAttribute;
import org.eclipse.jface.text.rules.IRule;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.ITokenScanner;
import org.eclipse.jface.text.rules.RuleBasedScanner;
import org.eclipse.jface.text.rules.Token;
import org.eclipse.swt.SWT;
import org.eclipse.swt.graphics.Color;
import org.eclipse.swt.graphics.Font;
import org.eclipse.swt.graphics.FontData;
import org.eclipse.swt.graphics.RGB;
import org.eclipse.swt.widgets.Display;

import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.connectors.claudecode.CCControlClient;
import xy.ai.workbench.connectors.claudecode.ProtocolParser;
import xy.ai.workbench.editors.md.AbstractRule;
import xy.ai.workbench.editors.md.BlockRule;
import xy.ai.workbench.editors.md.EmphasisRule;
import xy.ai.workbench.editors.md.HeaderRule;
import xy.ai.workbench.editors.md.LineMatchRule;
import xy.ai.workbench.editors.md.LinkRule;
import xy.ai.workbench.editors.md.ListRule;
import xy.ai.workbench.editors.md.PrefixLineRule;
import xy.ai.workbench.mdast.MarkdownDocument;
import xy.ai.workbench.mdast.nodes.AbstractNode;
import xy.ai.workbench.mdast.nodes.Elements;
import xy.ai.workbench.mdast.nodes.Node;

/**
 * AST optimized token scanner: instead of trying every markdown rule at every
 * document position, the scanner walks the region of the markdown AST
 * ({@link MarkdownDocument}) that overlaps the requested range and, for every
 * node, only applies the (small) subset of rules that is configured for that
 * node's own text. Text that belongs to a child node is only scanned once,
 * using the rules assigned to that child - never using the rules of an
 * ancestor. This way the changed/requested region is scanned exactly once,
 * split into disjoint sub-regions each processed with only the rules that are
 * allowed there.
 */
public class AIRuleScanner implements ITokenScanner {
	public static final TextAttribute DEFAULT_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_WIDGET_FOREGROUND), null, SWT.NONE);

	private static final TextAttribute USER_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_BLACK),
			new Color(Display.getCurrent(), new RGB(230, 230, 230)), SWT.BOLD);
	private static final TextAttribute AGENT_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_BLACK),
			new Color(Display.getCurrent(), new RGB(200, 200, 255)), SWT.BOLD);
	private static final TextAttribute BLUE_ATTR = new TextAttribute(
			new Color(Display.getCurrent(), new RGB(100, 100, 255)), null, SWT.NONE);
	private static final TextAttribute GREY_ATTR = new TextAttribute(
			new Color(Display.getCurrent(), new RGB(150, 150, 150)), null, SWT.NONE);
	private static final TextAttribute COMMENT_ATTR = new TextAttribute(
			new Color(Display.getCurrent(), new RGB(200, 200, 200)), null, SWT.NONE);
	private static final TextAttribute COMMENT_DARK_ATTR = new TextAttribute(
			new Color(Display.getCurrent(), new RGB(130, 130, 130)), null, SWT.NONE);
	private static final TextAttribute SPACER_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_BLACK),
			new Color(Display.getCurrent(), new RGB(200, 200, 200)), SWT.BOLD);

	/** Token used to reset styling of regions for which no rule is configured. */
	private static final IToken RESET_TOKEN = new Token(null);

	private final AITextEditor editor;

	/** One dedicated (stateless) rule based sub-scanner per AST node type. */
	private final Map<AbstractNode, RuleBasedScanner> scannerByNode = new IdentityHashMap<>();
	private final Map<RuleBasedScanner, IRule[]> ruleCache = new IdentityHashMap<>();

	/** Fallback scanner (all rules) used while no AST is available yet. */
	private final RuleBasedScanner fallbackScanner = new RuleBasedScanner();

	private final List<Piece> pieces = new ArrayList<>();
	private int pieceIndex;
	private int tokenOffset;
	private int tokenLength;

	public AIRuleScanner(Font basefont, AITextEditor editor) {
		this.editor = editor;

		Color c = Display.getCurrent().getSystemColor(SWT.COLOR_WIDGET_FOREGROUND);
		IToken userToken = new Token(USER_ATTR);
		IToken agentToken = new Token(AGENT_ATTR);
		IToken blueToken = new Token(BLUE_ATTR);
		IToken greyToken = new Token(GREY_ATTR);
		IToken commentToken = new Token(COMMENT_ATTR);
		IToken commentDarkToken = new Token(COMMENT_DARK_ATTR);
		IToken spacerToken = new Token(SPACER_ATTR);
		IToken normal = new Token(new TextAttribute(c, null, SWT.NORMAL));
		IToken bold = new Token(new TextAttribute(c, null, SWT.BOLD));
		IToken italic = new Token(new TextAttribute(c, null, SWT.ITALIC));
		IToken bolditalic = new Token(new TextAttribute(c, null, SWT.BOLD | SWT.ITALIC));
		IToken underline = new Token(new TextAttribute(c, null, TextAttribute.UNDERLINE));

		IRule commentRule = new BlockRule("<!--", "-->", normal);

		// ---- section: Root - only html comments may appear directly at the root ----
		register(Elements.ROOT, commentRule);

		// ---- section: headings - the marker/title line and setext-style headers ----
		Font[] headingFonts = getOrCreateFonts(basefont.getFontData()[0]);
		String[] headingPrefixes = { "###### ", "##### ", "#### ", "### ", "## ", "# " };
		HeaderRule headerRule = new HeaderRule(new Token(new TextAttribute(c, null, SWT.BOLD)));
		for (int i = 0; i < Elements.Headings.HEADINGS.length; i++) {
			IToken headingToken = new Token(new TextAttribute(c, null, SWT.BOLD, headingFonts[i]));
			register(Elements.Headings.HEADINGS[i], new PrefixLineRule(headingPrefixes[i], headingToken), headerRule);
		}

		// ---- section: page separator ----
		register(Elements.Page.PAGE, new PrefixLineRule("***", spacerToken));

		// ---- section: chat line markers, each only valid for its own element ----
		register(Elements.Chat.USER, new LineMatchRule(EditorInterface.USER, userToken));
		register(Elements.Chat.AGENT, new LineMatchRule(EditorInterface.AGENT, agentToken));
		register(Elements.Tools.CONTROL_REQUEST, new LineMatchRule(CCControlClient.CONTROL_REQUEST, agentToken));

		// ---- block: protocol prefix lines, each tied 1:1 to its own AST element ----
		register(Elements.Agent.THINKING, new PrefixLineRule(ProtocolParser.THINKING, agentToken));
		register(Elements.Agent.TEXT, new PrefixLineRule(ProtocolParser.TEXT, agentToken));
		register(Elements.Agent.TOOLUSE, new PrefixLineRule(ProtocolParser.TOOLUSE, agentToken));
		register(Elements.Tools.ANSWER, new PrefixLineRule(CCControlClient.ANSWER, commentDarkToken));
		register(Elements.Agent.REASONING_TOKEN, new PrefixLineRule(ProtocolParser.REASONING_TOKEN, commentDarkToken));
		register(Elements.Agent.TOKEN_STATS, new PrefixLineRule(ProtocolParser.TOKEN_STATS, commentDarkToken));
		register(Elements.Agent.SYSTEM_INIT, new PrefixLineRule(ProtocolParser.SYSTEM_INIT, agentToken));
		register(Elements.Basics.LINE_COMMENT, new PrefixLineRule(AbstractRule.LINE_COMMENT, commentToken));

		// ---- block: fenced code, only valid inside a ScriptBlock ----
		register(Elements.Basics.SCRIPTBLOCK, new BlockRule("```", "```", blueToken));

		// ---- section: paragraph - lists, emphasis, links and quote/glossary prefixes
		// ----
		register(Elements.Basics.PARAGRAPH, //
				commentRule, //
				new PrefixLineRule(": ", italic), // glossary syntax
				new PrefixLineRule("> ", italic), // citation syntax
				new ListRule(bold), //
				new EmphasisRule("***", bolditalic), //
				new EmphasisRule("**", bold), //
				new EmphasisRule("*", italic), //
				new EmphasisRule("$", italic), //
				new EmphasisRule("`", blueToken), // file or variable
				new EmphasisRule("„", "\"", greyToken), // literally
				new EmphasisRule("\"", greyToken), // literally
				new EmphasisRule("'", greyToken), // literally
				new EmphasisRule("»", "«", greyToken), // literally
				new EmphasisRule("›", "‹", greyToken), // literally
				new PrefixLineRule("---", spacerToken),
				new LinkRule(underline));

		// ---- fallback (used while no AST is available, e.g. huge documents) ----
		List<IRule> all = new ArrayList<>();
		for (RuleBasedScanner s : scannerByNode.values())
			for (IRule r : ruleCache.getOrDefault(s, new IRule[0]))
				all.add(r);
		IRule[] allRules = all.toArray(new IRule[0]);
		fallbackScanner.setRules(allRules);
		ruleCache.put(fallbackScanner, allRules);
	}

	private void register(AbstractNode node, IRule... rules) {
		RuleBasedScanner scanner = new RuleBasedScanner();
		scanner.setRules(rules);
		scannerByNode.put(node, scanner);
		ruleCache.put(scanner, rules);
	}

	private void applyDocumentBounds(RuleBasedScanner scanner, IDocument document, int start, int end) {
		IRule[] rules = ruleCache.get(scanner);
		if (rules == null)
			return;

		boolean atDocStart = start == 0;
		boolean atDocEnd = end == document.getLength();
		for (IRule rule : rules)
			if (rule instanceof AbstractRule abstractRule)
				abstractRule.setDocumentBounds(atDocStart, atDocEnd);
	}

	@Override
	public void setRange(IDocument document, int offset, int length) {
		pieces.clear();
		pieceIndex = 0;
		tokenOffset = offset;
		tokenLength = 0;

		if (length <= 0)
			return;

		MarkdownDocument ast = editor != null ? editor.getMarkdownAst() : null;
		if (ast == null) {
			scanFlat(fallbackScanner, document, offset, offset + length);
			return;
		}

		Node governing = ast.find(offset, offset + length);
		collect(document, governing, offset, offset + length);
	}

	/**
	 * Walks the subtree of {@code node} that overlaps [lo, hi), emitting rule
	 * matches for the node's own text (the "gaps" between its children) and
	 * recursing into every overlapping child using the child's own rules. Every
	 * character of [lo, hi) is visited exactly once.
	 */
	private void collect(IDocument document, Node node, int lo, int hi) {
		int cursor = Math.max(node.getOffset(), lo);

		for (Node child : node.children) {
			int cs = child.getOffset();
			int ce = child.getEndOffset();
			if (ce <= lo || cs >= hi)
				continue; // no overlap with requested range

			if (cursor < cs)
				scanGap(document, node.instance, cursor, Math.min(cs, hi));

			collect(document, child, lo, hi);
			cursor = ce;
		}

		int nodeEnd = Math.min(node.getEndOffset(), hi);
		if (cursor < nodeEnd)
			scanGap(document, node.instance, cursor, nodeEnd);
	}

	/**
	 * Scans [start, end), the text directly owned by {@code type} (i.e. not part of
	 * any child node), with the rules configured for {@code type}. The sub-range is
	 * widened by a single character (if available) so that rules relying on a
	 * boundary character shared with the following sibling/child (e.g. the trailing
	 * line break of a line marker) can still match; any resulting token is clipped
	 * back to [start, end).
	 */
	private void scanGap(IDocument document, AbstractNode type, int start, int end) {
		if (start >= end)
			return;

		RuleBasedScanner scanner = scannerByNode.get(type);
		if (scanner == null) {
			pieces.add(new Piece(start, end - start, RESET_TOKEN));
			return;
		}

		applyDocumentBounds(scanner, document, start, end);

		int widenedEnd = Math.min(document.getLength(), end + 1);
		scanner.setRange(document, start, widenedEnd - start);

		while (true) {
			IToken token = scanner.nextToken();
			if (token.isEOF())
				break;

			int off = scanner.getTokenOffset();
			int len = scanner.getTokenLength();
			if (off >= end)
				break;
			if (off + len > end)
				len = end - off;
			if (len <= 0)
				continue;

			pieces.add(new Piece(off, len, token));
		}
	}

	private void scanFlat(RuleBasedScanner scanner, IDocument document, int lo, int hi) {
		applyDocumentBounds(scanner, document, lo, hi);
		scanner.setRange(document, lo, hi - lo);
		while (true) {
			IToken token = scanner.nextToken();
			if (token.isEOF())
				break;
			pieces.add(new Piece(scanner.getTokenOffset(), scanner.getTokenLength(), token));
		}
	}

	@Override
	public IToken nextToken() {
		if (pieceIndex >= pieces.size())
			return Token.EOF;

		Piece p = pieces.get(pieceIndex++);
		tokenOffset = p.offset;
		tokenLength = p.length;
		return p.token;
	}

	@Override
	public int getTokenOffset() {
		return tokenOffset;
	}

	@Override
	public int getTokenLength() {
		return tokenLength;
	}

	record Piece(int offset, int length, IToken token) {
	}

	private static Font[] cachedFonts;

	private Font[] getOrCreateFonts(FontData fdata) {
		if (cachedFonts != null)
			return cachedFonts;

		int count = 6;
		Font[] fonts = new Font[count];
		Display display = Display.getDefault();

		for (int i = 0; i < count; i++)
			fonts[i] = new Font(display,
					new FontData(fdata.getName(), fdata.getHeight() + (i * 2), fdata.getStyle() | SWT.BOLD));

		return cachedFonts = fonts;
	}
}
