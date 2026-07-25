package xy.ai.workbench.mdast.nodes;

public final class Elements {
	public static final AbstractNode[] ALL = new AbstractNode[] { //
			HeadingSection.HEADINGS[0], //
			HeadingSection.HEADINGS[1], //
			HeadingSection.HEADINGS[2], //
			HeadingSection.HEADINGS[3], //
			HeadingSection.HEADINGS[4], //
			HeadingSection.HEADINGS[5], //
			PageSection.INSTANCE, //
			LineSection.USER, //
			LineSection.AGENT, //
			LineSection.CONTROL_REQUEST, //
			ScriptBlock.INSTANCE, //
			PrefixBlock.THINKING, //
			PrefixBlock.TEXT, //
			PrefixBlock.TOOLUSE, //
			PrefixBlock.ANSWER, //
			PrefixBlock.REASONING_TOKEN, //
			PrefixBlock.TOKEN_STATS, //
			PrefixBlock.SYSTEM_INIT, //
			PrefixBlock.LINE_COMMENT, //
			Paragraph.INSTANCE //
	};

	private Elements() {
	}
}
