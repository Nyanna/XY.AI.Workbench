package xy.ai.workbench.editor.outline.filter;

import xy.ai.workbench.editor.mdast.nodes.AbstractNode;
import xy.ai.workbench.editor.mdast.nodes.Elements;

/** Hides {@link Elements.Page#PAGE} nodes. */
public class PageFilter extends ElementFilter {
	@Override
	protected boolean matches(AbstractNode instance) {
		return instance == Elements.Page.PAGE;
	}
}
