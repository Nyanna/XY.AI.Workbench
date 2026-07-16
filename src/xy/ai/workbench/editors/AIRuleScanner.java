package xy.ai.workbench.editors;

import java.util.ArrayList;
import java.util.List;

import org.eclipse.jface.text.TextAttribute;
import org.eclipse.jface.text.rules.IRule;
import org.eclipse.jface.text.rules.IToken;
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
import xy.ai.workbench.editors.md.BlockRule;
import xy.ai.workbench.editors.md.EmphasisRule;
import xy.ai.workbench.editors.md.HeaderRule;
import xy.ai.workbench.editors.md.LineMatchRule;
import xy.ai.workbench.editors.md.LinkRule;
import xy.ai.workbench.editors.md.ListRule;
import xy.ai.workbench.editors.md.PrefixLineRule;

public class AIRuleScanner extends RuleBasedScanner {
	public static final String LINE_COMMENT = "#:";
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

	public AIRuleScanner(Font basefont) {
		Color c = Display.getCurrent().getSystemColor(SWT.COLOR_WIDGET_FOREGROUND);
		IToken userToken = new Token(USER_ATTR);
		IToken agentToken = new Token(AGENT_ATTR);
		IToken blueToken = new Token(BLUE_ATTR);
		IToken greyToken = new Token(GREY_ATTR);
		@SuppressWarnings("unused")
		IToken defaultToken = new Token(DEFAULT_ATTR);
		IToken commentToken = new Token(COMMENT_ATTR);
		IToken commentDarkToken = new Token(COMMENT_DARK_ATTR);
		IToken spacerToken = new Token(SPACER_ATTR);
		IToken normal = new Token(new TextAttribute(c, null, SWT.NORMAL));
		IToken bold = new Token(new TextAttribute(c, null, SWT.BOLD));
		IToken italic = new Token(new TextAttribute(c, null, SWT.ITALIC));
		IToken bolditalic = new Token(new TextAttribute(c, null, SWT.BOLD | SWT.ITALIC));
		IToken underline = new Token(new TextAttribute(c, null, TextAttribute.UNDERLINE));

		List<IRule> rules = new ArrayList<>();

		{
			Font[] headings = getOrCreateFonts(basefont.getFontData()[0]);
			// 1. block
			rules.add(new BlockRule("<!--", "-->", normal));
			rules.add(new BlockRule("```", "```", blueToken));
			// 2. linme start
			rules.add(new LineMatchRule(EditorInterface.USER, userToken));
			rules.add(new LineMatchRule(EditorInterface.AGENT, agentToken));
			rules.add(new LineMatchRule(CCControlClient.CONTROL_REQUEST, agentToken));
			rules.add(new PrefixLineRule("---", spacerToken));
			rules.add(new PrefixLineRule(ProtocolParser.THINKING, agentToken));
			rules.add(new PrefixLineRule(ProtocolParser.TEXT, agentToken));
			rules.add(new PrefixLineRule(ProtocolParser.TOOLUSE, agentToken));
			rules.add(new PrefixLineRule(CCControlClient.ANSWER, commentDarkToken));
			rules.add(new PrefixLineRule(ProtocolParser.REASONING_TOKEN, commentDarkToken));
			rules.add(new PrefixLineRule(ProtocolParser.TOKEN_STATS, commentDarkToken));
			rules.add(new PrefixLineRule(ProtocolParser.SYSTEM_INIT, agentToken));
			rules.add(new PrefixLineRule(LINE_COMMENT, commentToken));
			rules.add(new PrefixLineRule("###### ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[0]))));
			rules.add(new PrefixLineRule("##### ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[1]))));
			rules.add(new PrefixLineRule("#### ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[2]))));
			rules.add(new PrefixLineRule("### ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[3]))));
			rules.add(new PrefixLineRule("## ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[4]))));
			rules.add(new PrefixLineRule("# ", new Token(new TextAttribute(c, null, SWT.BOLD, headings[5]))));
			rules.add(new HeaderRule(new Token(new TextAttribute(c, null, SWT.BOLD))));
			rules.add(new ListRule(bold));
			// 3. in text
			rules.add(new EmphasisRule("***", bolditalic));
			rules.add(new EmphasisRule("**", bold));
			rules.add(new EmphasisRule("*", italic));
			rules.add(new EmphasisRule("$", italic));
			rules.add(new EmphasisRule("`", blueToken)); // file or variable
			rules.add(new EmphasisRule("\"", greyToken)); // literally
			rules.add(new EmphasisRule("'", greyToken)); // literally
			rules.add(new LinkRule(underline));
		}

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