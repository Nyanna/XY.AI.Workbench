package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class LineSection extends AbstractNode {
	private char[] prefix;
	private AbstractNode[] childNodes;
	private AbstractNode[] terminalNodes;

	LineSection(String marker, boolean spellcheck, AbstractNode[] childNodes, AbstractNode[] terminalNodes) {
		super(Category.Section);
		this.prefix = ("\n" + marker + "\n").toCharArray();
		this.enableSpellcheck = spellcheck;
		this.childNodes = childNodes;
		this.terminalNodes = terminalNodes;
	}

	@Override
	protected boolean isStart(Scanner s) {
		if (!s.isNextSequence(prefix))
			return false;
		s.unread(); // keep trailing NL for child scanning
		return true;
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		Scanner sub = s.getSubscanner();
		for (AbstractNode l : terminalNodes)
			if (l.isStart(sub)) {
				sub.reset();
				return true;
			}
		return false;
	}

	@Override
	protected AbstractNode[] getChildNodes() {
		return childNodes;
	}
}
