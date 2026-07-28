package xy.ai.workbench.editor.mdast.nodes;

import xy.ai.workbench.tools.Scanner;

public class Root extends AbstractNode {
	Root(AbstractNode[] childNodes) {
		super(Category.Section, childNodes);
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
	public String toString() {
		return "Root";
	}
}
