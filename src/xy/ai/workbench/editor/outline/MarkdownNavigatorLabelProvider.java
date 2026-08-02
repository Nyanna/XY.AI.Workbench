package xy.ai.workbench.editor.outline;

import org.eclipse.jface.viewers.LabelProvider;

public class MarkdownNavigatorLabelProvider extends LabelProvider {

	@Override
	public String getText(Object element) {
		return element instanceof NodeElement ne ? NodeLabels.getText(ne.node(), ne.doc()) : String.valueOf(element);
	}
}
