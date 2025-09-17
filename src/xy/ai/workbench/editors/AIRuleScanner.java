package xy.ai.workbench.editors;

import java.util.ArrayList;
import java.util.List;

import org.eclipse.jface.text.TextAttribute;
import org.eclipse.jface.text.rules.IRule;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.IWhitespaceDetector;
import org.eclipse.jface.text.rules.IWordDetector;
import org.eclipse.jface.text.rules.MultiLineRule;
import org.eclipse.jface.text.rules.RuleBasedScanner;
import org.eclipse.jface.text.rules.Token;
import org.eclipse.jface.text.rules.WhitespaceRule;
import org.eclipse.jface.text.rules.WordRule;
import org.eclipse.swt.SWT;
import org.eclipse.swt.graphics.Color;
import org.eclipse.swt.graphics.Font;
import org.eclipse.swt.graphics.FontData;
import org.eclipse.swt.graphics.RGB;
import org.eclipse.swt.widgets.Display;

import xy.ai.workbench.AISessionManager;
import xy.ai.workbench.editors.md.EmphasisRule;
import xy.ai.workbench.editors.md.PrefixLineRule;
import xy.ai.workbench.editors.md.HeaderRule;
import xy.ai.workbench.editors.md.LinkRule;
import xy.ai.workbench.editors.md.ListRule;

public class AIRuleScanner extends RuleBasedScanner {
	public static final String LINE_COMMENT = "#:";
	private static final TextAttribute USER_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_BLACK),
			new Color(Display.getCurrent(), new RGB(230, 230, 230)), SWT.BOLD);
	private static final TextAttribute AGENT_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_BLACK),
			new Color(Display.getCurrent(), new RGB(200, 200, 255)), SWT.BOLD);
	private static final TextAttribute BLUE_ATTR = new TextAttribute(
			new Color(Display.getCurrent(), new RGB(100, 100, 255)), null, SWT.NONE);
	private static final TextAttribute DEFAULT_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_WIDGET_FOREGROUND), null, SWT.NONE);
	private static final TextAttribute COMMENT_ATTR = new TextAttribute(
			new Color(Display.getCurrent(), new RGB(200, 200, 200)), null, SWT.NONE);
	private static final TextAttribute SPACER_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_BLACK),
			new Color(Display.getCurrent(), new RGB(200, 200, 200)), SWT.NONE);

	public AIRuleScanner(Font basefont) {
		Color c = Display.getCurrent().getSystemColor(SWT.COLOR_WIDGET_FOREGROUND);
		IToken userToken = new Token(USER_ATTR);
		IToken agentToken = new Token(AGENT_ATTR);
		IToken blueToken = new Token(BLUE_ATTR);
		IToken defaultToken = new Token(DEFAULT_ATTR);
		IToken commentToken = new Token(COMMENT_ATTR);
		IToken spacerToken = new Token(SPACER_ATTR);
		IToken normal = new Token(new TextAttribute(c, null, SWT.NORMAL));
		IToken bold = new Token(new TextAttribute(c, null, SWT.BOLD));
		IToken italic = new Token(new TextAttribute(c, null, SWT.ITALIC));
		IToken bolditalic = new Token(new TextAttribute(c, null, SWT.BOLD | SWT.ITALIC));
		IToken underline = new Token(new TextAttribute(c, null, TextAttribute.UNDERLINE));

		Font[] headings = getOrCreateFonts(basefont.getFontData()[0]);

		List<IRule> rules = new ArrayList<>();

		WordRule wordRule = new WordRule(new IWordDetector() {
			@Override
			public boolean isWordStart(char c) {
				return Character.isLetter(c);
			}

			@Override
			public boolean isWordPart(char c) {
				return Character.isLetter(c) || c == ':';
			}
		}, defaultToken);
		wordRule.addWord(AISessionManager.USER, userToken);
		wordRule.addWord(AISessionManager.AGENT, agentToken);
		rules.add(wordRule);

		rules.add(new WhitespaceRule(new IWhitespaceDetector() {
			@Override
			public boolean isWhitespace(char c) {
				return Character.isWhitespace(c);
			}
		}));

		rules.add(new EmphasisRule("***", bolditalic));
		rules.add(new EmphasisRule("**", bold));
		rules.add(new EmphasisRule("*", italic));
		rules.add(new EmphasisRule("$", italic));
		rules.add(new MultiLineRule("<!--", "-->", normal));
		rules.add(new MultiLineRule("```", "```", blueToken));
		rules.add(new HeaderRule(new Token(new TextAttribute(c, null, SWT.BOLD))));
		rules.add(new PrefixLineRule(LINE_COMMENT, commentToken));
		rules.add(new PrefixLineRule("---", spacerToken));
		rules.add(new PrefixLineRule("###### ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[0]))));
		rules.add(new PrefixLineRule("##### ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[1]))));
		rules.add(new PrefixLineRule("#### ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[2]))));
		rules.add(new PrefixLineRule("### ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[3]))));
		rules.add(new PrefixLineRule("## ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[4]))));
		rules.add(new PrefixLineRule("# ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[5]))));
		rules.add(new ListRule(new Token(new TextAttribute(c, null, SWT.BOLD))));
		rules.add(new LinkRule(underline));

		setRules(rules.toArray(new IRule[0]));
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