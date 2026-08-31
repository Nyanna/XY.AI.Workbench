package xy.ai.workbench.editor.mdast.nodes;

import java.util.stream.Stream;

import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.connector.claudecode.CCControlClient;
import xy.ai.workbench.connector.claudecode.ProtocolParser;
import xy.ai.workbench.editor.md.AbstractRule;

/*
 * Sorting must be inverted, root on bottom contains all 
 */
public class Elements {
	public static final AbstractNode[] NONE = new AbstractNode[0];

	public static class Basics {
		public static final ScriptBlock SCRIPTBLOCK = new ScriptBlock();
		public static final LineCommentBlock LINE_COMMENT = new LineCommentBlock(AbstractRule.LINE_COMMENT);
		public static final Paragraph PARAGRAPH = new Paragraph(NONE); // replaced later
		public static final AbstractNode[] ALL = of(SCRIPTBLOCK, LINE_COMMENT, PARAGRAPH);
	}

	public static class Headings {
		public static final HeadingSection[] HEADINGS = new HeadingSection[HeadingSection.MAX_ORDER];
		static {
			for (int i = 0; i < HEADINGS.length; i++)
				HEADINGS[i] = new HeadingSection(HeadingSection.MAX_ORDER - i, NONE, HEADINGS); // None will be replaced

			for (int i = 0; i < HEADINGS.length; i++) {
				AbstractNode[] childNodes = new AbstractNode[i];
				for (int j = 0; j < i; j++)
					childNodes[j] = HEADINGS[j];

				HEADINGS[i].childNodes = concat( //
						childNodes, //
						of( //
								Basics.SCRIPTBLOCK, //
								Basics.LINE_COMMENT, //
								Basics.PARAGRAPH //
						));
			}
		}
	}

	public static class Agent {
		public static final PrefixBlock THINKING = new PrefixBlock(ProtocolParser.THINKING);
		public static final LineSection TEXT = new LineSection(ProtocolParser.TEXT, false, of(Basics.PARAGRAPH),
				of(NONE)); // replaced
		public static final PrefixBlock TOOLUSE = new PrefixBlock(ProtocolParser.TOOLUSE);
		public static final PrefixBlock REASONING_TOKEN = new PrefixBlock(ProtocolParser.REASONING_TOKEN);
		public static final PrefixBlock TOKEN_STATS = new PrefixBlock(ProtocolParser.TOKEN_STATS);
		public static final PrefixBlock SYSTEM_INIT = new PrefixBlock(ProtocolParser.SYSTEM_INIT);
		public static final PrefixBlock RESULT = new PrefixBlock(ProtocolParser.RESULT);
		public static final AbstractNode[] ALL = of(THINKING, TEXT, TOOLUSE, REASONING_TOKEN, TOKEN_STATS, SYSTEM_INIT, RESULT);
	}

	public static class Tools {
		public static final PrefixBlock ANSWER = new PrefixBlock(CCControlClient.ANSWER);
		public static final LineSection CONTROL_REQUEST = new LineSection(CCControlClient.CONTROL_REQUEST, false, of(//
				ANSWER, //
				Basics.SCRIPTBLOCK), //
				NONE// replaced later
		);
		public static final AbstractNode[] ALL = of(CONTROL_REQUEST);
	}

	public static class Page {
		private static final AbstractNode[] PAGE_ELEMENTS = concat( //
				Headings.HEADINGS, //
				of(//
						Basics.PARAGRAPH //
				));
		public static final PageSection PAGE = new PageSection(PAGE_ELEMENTS);
		static {
			for (int i = 0; i < Headings.HEADINGS.length; i++)
				Headings.HEADINGS[i].terminals = concat( //
						of(PAGE), //
						Tools.ALL, //
						Agent.ALL);
		}
	}

	public static class Chat {
		private static final AbstractNode[] USER_ELEMENTS = concat( //
				Headings.HEADINGS, //
				of( //
						Basics.SCRIPTBLOCK, //
						Basics.LINE_COMMENT, //
						Page.PAGE, //
						Basics.PARAGRAPH //
				));

		public static final LineSection USER = new LineSection(EditorInterface.USER, true, USER_ELEMENTS, //
				NONE); // later replaced

		private static final AbstractNode[] AGENT_ONLY = of( //
				Tools.CONTROL_REQUEST, //
				Agent.THINKING, //
				Agent.TEXT, //
				Agent.TOOLUSE, //
				Agent.REASONING_TOKEN, //
				Agent.TOKEN_STATS, //
				Agent.SYSTEM_INIT, //
				Agent.RESULT //
		);
		private static final AbstractNode[] AGENT_ELEMENTS = concat( //
				AGENT_ONLY, //
				USER_ELEMENTS //
		);
		public static final LineSection AGENT = new LineSection(EditorInterface.AGENT, false, AGENT_ELEMENTS, of(USER));
		public static final AbstractNode[] ALL = of(USER, AGENT);

		static {
			USER.terminalNodes = of(AGENT);
			Agent.TEXT.terminalNodes = AGENT_ONLY;
			Tools.CONTROL_REQUEST.terminalNodes = concat(//
					of(USER, AGENT, //
							Tools.CONTROL_REQUEST //
					), //
					Headings.HEADINGS, //
					of(Agent.THINKING, //
							Agent.TEXT, //
							Agent.TOOLUSE, //
							Agent.REASONING_TOKEN, //
							Agent.TOKEN_STATS, //
							Agent.SYSTEM_INIT, //
							Agent.RESULT, //
							Basics.PARAGRAPH //
					));
		}
	}

	public static class Roots {
		private static final AbstractNode[] ROOT_ELEMENTS = concat( //
				of(Page.PAGE), //
				Headings.HEADINGS, //
				Chat.ALL, //
				Agent.ALL, //
				Tools.ALL, //
				Basics.ALL //
		);

		public static final Root ROOT = new Root(ROOT_ELEMENTS);

		static {
			// all execpt itself
			Basics.PARAGRAPH.terminals = Stream.of(ROOT_ELEMENTS).filter(e -> e != Basics.PARAGRAPH)
					.toArray(AbstractNode[]::new);
		}

	}

	public static final Root ROOT = Roots.ROOT;

	private static AbstractNode[] of(AbstractNode... nodes) {
		return nodes;
	}

	private static AbstractNode[] concat(AbstractNode[]... s) {
		Stream<AbstractNode> ss = Stream.of(s[0]);
		for (int i = 1; i < s.length; i++)
			ss = Stream.concat(ss, Stream.of(s[i]));
		return ss.toArray(AbstractNode[]::new);
	}
}
