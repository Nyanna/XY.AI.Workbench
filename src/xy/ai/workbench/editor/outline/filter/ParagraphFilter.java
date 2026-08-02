package xy.ai.workbench.editor.outline.filter;

import xy.ai.workbench.editor.mdast.nodes.AbstractNode;
import xy.ai.workbench.editor.mdast.nodes.Elements;

/** Hides {@link Elements.Basics#PARAGRAPH} nodes. */
public class ParagraphFilter extends ElementFilter {
	@Override
	protected boolean matches(AbstractNode instance) {
		return instance == Elements.Basics.PARAGRAPH;
	}
}
