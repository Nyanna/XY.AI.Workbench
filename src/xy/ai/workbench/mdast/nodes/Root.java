package xy.ai.workbench.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class Root extends AbstractNode {
	public static final Root INSTANCE = new Root();

	private Root() {
		super(Category.Section);
		this.enableSpellcheck = true;
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
		return Elements.ALL;
	}
}
