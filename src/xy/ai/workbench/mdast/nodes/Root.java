package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class Root extends AbstractNode {
	public static final Root INSTANCE = new Root();
	// contains all possible
	private AbstractNode[] childNodes = new AbstractNode[] { //
			HeadingSection.HEADINGS[0], //
			HeadingSection.HEADINGS[1], //
			HeadingSection.HEADINGS[2], //
			HeadingSection.HEADINGS[3], //
			HeadingSection.HEADINGS[4], //
			HeadingSection.HEADINGS[5], //
			Paragraph.INSTANCE //
	};

	private Root() {
		super(Category.Section);
	}

	@Override
	protected boolean isStart(Scanner s) {
		return true;
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		return s.isEOF();
	}

	@Override
	protected AbstractNode[] getChildNodes() {
		return childNodes;
	}

	@Override
	public int hashCode() {
		return getClass().hashCode();
	}

	@Override
	public boolean equals(Object obj) {
		return obj != null && getClass().equals(obj.getClass());
	}
}
