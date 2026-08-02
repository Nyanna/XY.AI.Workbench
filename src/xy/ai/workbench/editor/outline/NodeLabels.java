package xy.ai.workbench.editor.outline;

import java.util.regex.Pattern;

import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;

import xy.ai.workbench.editor.mdast.nodes.Elements;
import xy.ai.workbench.editor.mdast.nodes.Node;

public class NodeLabels {
	private static final int LABEL_LIMIT = 40;

	public static String getText(Node node, IDocument doc) {
		String prefix = "";
		// no prefix
		if (!Elements.Tools.ANSWER.equals(node.instance)) {
			prefix = node.instance.toString();
			if (!prefix.endsWith(":") && !prefix.endsWith(" "))
				prefix += ":";
			if (!prefix.endsWith(" "))
				prefix += " ";
		}
		return prefix + String.format("%s (%d)", snippet(node, doc), node.end - node.start);
	}

	private static String snippet(Node node, IDocument doc) {
		if (doc == null)
			return "";

		if (Elements.Agent.TEXT.equals(node.instance) || Elements.Chat.AGENT.equals(node.instance)
				|| Elements.Tools.CONTROL_REQUEST.equals(node.instance)) {
			if (node.children.size() > 0)
				node = node.children.get(0);
		} else if (Elements.Chat.USER.equals(node.instance)) {
			if (node.children.size() == 0)
				return "";
		}

		if (Elements.Tools.CONTROL_REQUEST.equals(node.instance)) {
			if (node.children.size() > 0)
				node = node.children.get(0);
		}

		int offset = node.getOffset();
		int length = node.length();
		if (offset < 0 || length <= 0)
			return "Empty";
		length = Math.min(length, doc.getLength() - offset);
		if (length <= 0)
			return "Empty";
		try {
			String text = doc.get(offset, length).strip();

			if (Elements.Basics.SCRIPTBLOCK.equals(node.instance)) {
				var m = Pattern.compile("\ntoolName: (.*)\n").matcher(text);
				if (m.find())
					return "Tool " + m.group(1);
				m = Pattern.compile("\nresult:\n").matcher(text);
				if (m.find())
					return "Result";
			}

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
