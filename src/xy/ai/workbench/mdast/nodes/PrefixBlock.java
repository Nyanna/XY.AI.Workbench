package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.connectors.claudecode.CCControlClient;
import xy.ai.workbench.connectors.claudecode.ProtocolParser;
import xy.ai.workbench.editors.md.AbstractRule;
import xy.ai.workbench.tools.Scanner;

public class PrefixBlock extends AbstractNode {
	public static final PrefixBlock THINKING = new PrefixBlock(ProtocolParser.THINKING);
	public static final PrefixBlock TEXT = new PrefixBlock(ProtocolParser.TEXT);
	public static final PrefixBlock TOOLUSE = new PrefixBlock(ProtocolParser.TOOLUSE);
	public static final PrefixBlock ANSWER = new PrefixBlock(CCControlClient.ANSWER);
	public static final PrefixBlock REASONING_TOKEN = new PrefixBlock(ProtocolParser.REASONING_TOKEN);
	public static final PrefixBlock TOKEN_STATS = new PrefixBlock(ProtocolParser.TOKEN_STATS);
	public static final PrefixBlock SYSTEM_INIT = new PrefixBlock(ProtocolParser.SYSTEM_INIT);
	public static final PrefixBlock LINE_COMMENT = new PrefixBlock(AbstractRule.LINE_COMMENT);

	private char[] prefix;

	private PrefixBlock(String marker) {
		super(Category.Block);
		this.prefix = ("\n" + marker).toCharArray();
	}

	@Override
	protected boolean isStart(Scanner s) {
		return s.isNextSequence(prefix);
	}

	@Override
	protected boolean isEndInner(Scanner s) {
		Scanner sub = s.getSubscanner();
		boolean end = !sub.readNext() || sub.isNewLine();
		sub.reset();
		return end;
	}

	@Override
	protected AbstractNode[] getChildNodes() {
		return NO_CHILDREN;
	}
}
