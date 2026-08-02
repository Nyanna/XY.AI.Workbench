package xy.ai.workbench.editor.outline;

import java.util.HashMap;
import java.util.Map;

import org.eclipse.jface.text.IDocument;
import xy.ai.workbench.editor.mdast.nodes.Node;

public class NodeElement {
	private final Node node;
	private final IDocument doc;
	private final NodeElement parent;
	private NodeElement[] children;
	private int nodeHash; // cache shortcut

	public NodeElement(Node node, IDocument doc, NodeElement parent) {
		this.node = node;
		this.doc = doc;
		this.parent = parent;
	}

	public NodeElement find(Node node) {
		if (this.node == node)
			return this;
		NodeElement res;
		for (var c : children())
			if ((res = c.find(node)) != null)
				return res;
		return null;
	}

	public Node node() {
		return node;
	}

	public NodeElement parent() {
		return parent;
	}

	public IDocument doc() {
		return doc;
	}

	public NodeElement[] children() {
		int newHash;
		if (nodeHash != (newHash = node.children.hashCode())) {
			Map<Node, NodeElement> previous = new HashMap<>();
			if (children != null)
				for (NodeElement c : children)
					previous.put(c.node(), c);
			this.children = node.children.stream()
					.map(child -> previous.getOrDefault(child, new NodeElement(child, doc, this)))
					.toArray(NodeElement[]::new);
			nodeHash = newHash;
		}
		return children;
	}
}