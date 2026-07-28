package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class LineCommentBlock extends AbstractNode {
	private char[] prefix;
	private char[] marker;

	LineCommentBlock(String marker) {
		super(Category.Block, Elements.NONE);
		this.prefix = ("\n" + marker).toCharArray();
		this.marker = marker.toCharArray();
	}

	@Override
	protected boolean isStart(Scanner s) {
		return s.isNextSequenceBounded(prefix);
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		Scanner sub = s.getSubscanner();
		boolean end;
		if (!sub.readNext())
			end = true;
		else if (!sub.isNewLine())
			end = false;
		else
			// next line does not start with the marker -> block ends here,
			// without consuming this newline
			end = !sub.isNextSequence(marker);
		sub.reset();
		return end;
	}

	@Override
	public String toString() {
		return "Comment " + String.valueOf(prefix).replace('\n', ' ');
	}
}
