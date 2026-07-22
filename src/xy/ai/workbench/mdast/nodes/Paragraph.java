package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class Paragraph extends AbstractNode {
	public static final Paragraph INSTANCE = new Paragraph();
	private AbstractNode[] childNodes = new AbstractNode[0];

	private char[] prefix = "\n\n".toCharArray();

	private Paragraph() {
		super(Category.Section);
	}

	@Override
	protected boolean isStart(Scanner s) {
		return s.isNextSequence(prefix);
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		for (int i = 0; i < HeadingSection.HEADINGS.length; i++) {
			Scanner sub = new Scanner(s);
			if (HeadingSection.HEADINGS[i].isStart(sub)) {
				sub.reset();
				return true;
			}
		}

		if (s.isNextSequence(prefix)) {
			s.unread();
			return true;
		}
		return false;
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
