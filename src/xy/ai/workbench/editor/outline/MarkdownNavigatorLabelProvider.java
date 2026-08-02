package xy.ai.workbench.editor.outline;

import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.viewers.LabelProvider;

import xy.ai.workbench.editor.mdast.nodes.Node;

public class MarkdownNavigatorLabelProvider extends LabelProvider {

	public static void setActiveDocument(IDocument document) {
		NodeLabels.setActiveDocument(document);
	}

	@Override
	public String getText(Object element) {
		return element instanceof Node node ? NodeLabels.getText(node) : String.valueOf(element);
	}
}
