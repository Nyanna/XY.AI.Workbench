package xy.ai.workbench.mdast.nodes;

import java.util.stream.Stream;

import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.connectors.claudecode.CCControlClient;
import xy.ai.workbench.connectors.claudecode.ProtocolParser;
import xy.ai.workbench.editors.md.AbstractRule;

public final class Elements {
	// Instances

	public static final Root ROOT = new Root();
	public static final PageSection PAGESECTION = new PageSection();
	public static final ScriptBlock SCRIPTBLOCK = new ScriptBlock();
	public static final PrefixBlock THINKING = new PrefixBlock(ProtocolParser.THINKING);
	public static final PrefixBlock TEXT = new PrefixBlock(ProtocolParser.TEXT);
	public static final PrefixBlock TOOLUSE = new PrefixBlock(ProtocolParser.TOOLUSE);
	public static final PrefixBlock ANSWER = new PrefixBlock(CCControlClient.ANSWER);
	public static final PrefixBlock REASONING_TOKEN = new PrefixBlock(ProtocolParser.REASONING_TOKEN);
	public static final PrefixBlock TOKEN_STATS = new PrefixBlock(ProtocolParser.TOKEN_STATS);
	public static final PrefixBlock SYSTEM_INIT = new PrefixBlock(ProtocolParser.SYSTEM_INIT);
	public static final PrefixBlock LINE_COMMENT = new PrefixBlock(AbstractRule.LINE_COMMENT);
	public static final LineSection USER = new LineSection(EditorInterface.USER, true, Elements.PAGE);
	public static final LineSection AGENT = new LineSection(EditorInterface.AGENT, false, Elements.PAGE);
	public static final LineSection CONTROL_REQUEST = new LineSection(CCControlClient.CONTROL_REQUEST, false,
			new AbstractNode[] { ANSWER, SCRIPTBLOCK });
	public static final Paragraph PARAGRAPH = new Paragraph();

	public static final HeadingSection[] HEADINGS = new HeadingSection[HeadingSection.MAX_ORDER];
	static {
		for (int i = 0; i < HEADINGS.length; i++)
			HEADINGS[i] = new HeadingSection(HeadingSection.MAX_ORDER - i);

		for (int i = 0; i < HEADINGS.length; i++) {
			AbstractNode[] childNodes = new AbstractNode[i];
			for (int j = 0; j < i; j++)
				childNodes[j] = HEADINGS[j];

			HEADINGS[i].childNodes = Stream.concat( //
					Stream.of(childNodes), //
					Stream.of(new AbstractNode[] { //
							SCRIPTBLOCK, //
							LINE_COMMENT, //
							PARAGRAPH //
					})).toArray(AbstractNode[]::new);
		}
	}

	// Sets

	public static final LineSection[] LINE_SECTION_FAMILY = { Elements.USER, Elements.AGENT, Elements.CONTROL_REQUEST };

	public static final AbstractNode[] PAGE = new AbstractNode[] { //
			HEADINGS[0], //
			HEADINGS[1], //
			HEADINGS[2], //
			HEADINGS[3], //
			HEADINGS[4], //
			HEADINGS[5], //
			USER, //
			AGENT, //
			CONTROL_REQUEST, //
			SCRIPTBLOCK, //
			THINKING, //
			TEXT, //
			TOOLUSE, //
			ANSWER, //
			REASONING_TOKEN, //
			TOKEN_STATS, //
			SYSTEM_INIT, //
			LINE_COMMENT, //
			PARAGRAPH //
	};

	public static final AbstractNode[] ALL = Stream.concat( //
			Stream.of(PAGESECTION), //
			Stream.of(PAGE) //
	).toArray(AbstractNode[]::new);

	public static final AbstractNode[] PARAGRAPH_ENDS = Stream.concat( //
			Stream.of(HEADINGS), //
			Stream.of(new AbstractNode[] { //
					SCRIPTBLOCK, //
					LINE_COMMENT //
			})).toArray(AbstractNode[]::new);

	public static final AbstractNode[] NONE = new AbstractNode[0];

	private Elements() {
	}
}
