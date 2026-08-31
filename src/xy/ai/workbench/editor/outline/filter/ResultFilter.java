package xy.ai.workbench.editor.outline.filter;

import xy.ai.workbench.editor.mdast.nodes.AbstractNode;
import xy.ai.workbench.editor.mdast.nodes.Elements;

/** Hides {@link Elements.Agent#RESULT} nodes. */
public class ResultFilter extends ElementFilter {
	@Override
	protected boolean matches(AbstractNode instance) {
		return instance == Elements.Agent.RESULT;
	}
}
