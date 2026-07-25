package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.connectors.claudecode.CCControlClient;
import xy.ai.workbench.tools.Scanner;

public class LineSection extends AbstractNode {
	public static final LineSection USER = new LineSection(EditorInterface.USER, true);
	public static final LineSection AGENT = new LineSection(EditorInterface.AGENT, false);
	public static final LineSection CONTROL_REQUEST = new LineSection(CCControlClient.CONTROL_REQUEST, false);

	private static final LineSection[] FAMILY = { USER, AGENT, CONTROL_REQUEST };

	private char[] prefix;

	private LineSection(String marker, boolean spellcheck) {
		super(Category.Section);
		this.prefix = ("\n" + marker + "\n").toCharArray();
		this.enableSpellcheck = spellcheck;
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
		for (LineSection l : FAMILY)
			if (l.isStart(sub)) {
				sub.reset();
				return true;
			}
		return false;
	}

	@Override
	protected AbstractNode[] getChildNodes() {
		return Elements.ALL;
	}
}
