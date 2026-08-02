package xy.ai.workbench.editor.outline;

import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;

import xy.ai.workbench.editor.mdast.nodes.Elements;
import xy.ai.workbench.editor.mdast.nodes.Node;

public class NodeLabels {
	private static final int LABEL_LIMIT = 40;

	public static String getText(Node node, IDocument doc) {
		return node.instance.toString() + ": " + String.format("%s (%d)", snippet(node, doc), node.end - node.start);
	}

	private static String snippet(Node node, IDocument doc) {
		if (doc == null)
			return "";
		
		if(Elements.Agent.TEXT.equals(node.instance) && node.children.size() > 0)
			node = node.children.get(0);
		
		int offset = node.getOffset();
		int length = node.length();
		if (offset < 0 || length <= 0)
			return "Empty";
		length = Math.min(length, doc.getLength() - offset);
		if (length <= 0)
			return "Empty";
		try {
			String text = doc.get(offset, length).strip();
			int nl = text.indexOf('\n');
			if (nl >= 0)
				text = text.substring(0, nl).strip();
			if (text.length() > LABEL_LIMIT)
				text = text.substring(0, LABEL_LIMIT) + "…";
			return text.strip();
		} catch (BadLocationException e) {
			return "";
		}
	}
}
