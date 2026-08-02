package xy.ai.workbench.editor.outline.filter;

import xy.ai.workbench.editor.mdast.nodes.AbstractNode;
import xy.ai.workbench.editor.mdast.nodes.HeadingSection;

/**
 * Hides all {@link HeadingSection} nodes (all heading levels at once, since
 * hiding a heading also hides its substructure).
 */
public class HeadingFilter extends ElementFilter {
	@Override
	protected boolean matches(AbstractNode instance) {
		return instance instanceof HeadingSection;
	}
}
