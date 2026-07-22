package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class HeadingSection extends AbstractNode {
	private static final int MAX_ORDER = 6;
	public static final HeadingSection[] HEADINGS = new HeadingSection[MAX_ORDER];
	static {
		for (int i = 0; i < HEADINGS.length; i++)
			HEADINGS[i] = new HeadingSection(MAX_ORDER - i);

		for (int i = 0; i < HEADINGS.length; i++) {
			var childNodes = new AbstractNode[i + 1];
			for (int j = 0; j < i; j++)
				childNodes[j] = HEADINGS[j];
			// and catch all paragraph
			childNodes[childNodes.length - 1] = Paragraph.INSTANCE;
			HEADINGS[i].childNodes = childNodes;
		}
	}

	private int order;
	private char[] prefix;
	private AbstractNode[] childNodes;

	private HeadingSection(int order) {
		super(Category.Section);
		this.order = order;

		// starts with "\n## "
		prefix = new char[order + 2];
		prefix[0] = '\n';
		for (int i = 1; i < prefix.length - 1; i++)
			prefix[i] = '#';
		prefix[prefix.length - 1] = ' ';
	}

	@Override
	protected boolean isStart(Scanner s) {
		return s.isNextSequence(prefix);
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		Scanner sub = new Scanner(s);
		for (int i = MAX_ORDER - order; i < HEADINGS.length; i++)
			if (HEADINGS[i].isStart(sub)) {
				sub.reset();
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
		final int prime = 31;
		int result = 1;
		result = prime * result + order;
		return result;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null)
			return false;
		if (getClass() != obj.getClass())
			return false;
		HeadingSection other = (HeadingSection) obj;
		return order == other.order;
	}
}
