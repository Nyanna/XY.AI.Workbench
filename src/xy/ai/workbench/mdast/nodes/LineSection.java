package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class LineSection extends AbstractNode {
	private char[] prefix;
	AbstractNode[] terminalNodes;

	LineSection(String marker, boolean spellcheck, AbstractNode[] childNodes, AbstractNode[] terminalNodes) {
		super(Category.Section, childNodes);
		this.prefix = ("\n" + marker + "\n").toCharArray();
		this.enableSpellcheck = spellcheck;
		this.terminalNodes = terminalNodes;
	}

	@Override
	protected boolean isStart(Scanner s) {
		if (!s.isNextSequenceBounded(prefix))
			return false;
		// keep trailing NL for child scanning
		if (!s.isEOF())
			s.unread();
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
}
