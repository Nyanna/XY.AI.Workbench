package xy.ai.workbench.mdast;

import java.util.List;

import xy.ai.workbench.mdast.nodes.Node;
import xy.ai.workbench.mdast.nodes.Root;
import xy.ai.workbench.tools.LineIndex;
import xy.ai.workbench.tools.Scanner;

public class MarkdownDocument {
	private final IDocumentBuffer buffer;
	private final LineIndex lines = new LineIndex();

	private Node root = new Node(null, Root.INSTANCE);

	public MarkdownDocument(IDocumentBuffer buffer) {
		this.buffer = buffer;
	}

	public void update(int offset, int removed, int inserted) {
		int lo = offset;
		int hi = offset + removed;
		int delta = inserted - removed;
		lines.update(buffer, offset, removed, inserted);

		Node sec = find(lo, hi);
		while (true) {
			Node parent = sec.parent;
			int absStart = sec.getOffset();
			int newLen = sec.length() + delta;
			Node rn = parse(absStart, absStart + newLen);

			if (parent == null || isCompatible(rn.children, sec, parent)) {
				replace(sec, rn.children, delta);
				return;
			}
			sec = parent;
		}
	}

	private Node parse(int absStart, int absEnd) {
		char[] slice = readChars(absStart, absEnd - absStart);
		Node rn = new Node(null, Root.INSTANCE);
		Root.INSTANCE.scan(new Scanner(new BufferReader(slice, 0)), rn);
		return rn;
	}

	private boolean isCompatible(List<Node> rchilds, Node sec, Node parent) {
		if (rchilds.isEmpty())
			return false;
		if (rchilds.get(0).instance != sec.instance)
			return false;
		for (Node c : rchilds)
			if (!parent.instance.containChild(c.instance))
				return false;
		return true;
	}

	private void replace(Node sec, List<Node> nchilds, int delta) {
		Node parent = sec.parent;
		if (parent == null) {
			root.children.clear();
			for (Node c : nchilds)
				root.children.add(reparent(c, root));
			root.end += delta;
			return;
		}

		List<Node> siblings = parent.children;
		int idx = siblings.indexOf(sec);
		for (int i = idx + 1; i < siblings.size(); i++)
			shift(siblings.get(i), delta);

		siblings.remove(idx);
		int at = idx;
		for (Node c : nchilds) {
			c.start += sec.start;
			c.end += sec.start;
			siblings.add(at++, reparent(c, parent));
		}

		for (Node anc = parent; anc != null; anc = anc.parent) {
			anc.end += delta;
			Node ap = anc.parent;
			if (ap == null)
				continue;
			List<Node> as = ap.children;
			int ai = as.indexOf(anc);
			for (int i = ai + 1; i < as.size(); i++)
				shift(as.get(i), delta);
		}
	}

	private Node reparent(Node src, Node newParent) {
		src.parent = newParent;
		return src;
	}

	private void shift(Node node, int delta) {
		node.start += delta;
		node.end += delta;
	}

	public Node getRoot() {
		return root;
	}

	public Node find(int lo, int hi) {
		return find(root, lo, hi);
	}

	private Node find(Node node, int lo, int hi) {
		for (Node child : node.children) {
			int cs = child.getOffset();
			if (cs < lo && hi < cs + child.length())
				return find(child, lo, hi);
		}
		return node;
	}

	private char[] readChars(int offset, int length) {
		char[] chars = new char[Math.max(0, length)];
		if (length > 0)
			buffer.getChars(offset, length, chars, 0);
		return chars;
	}

	private static class BufferReader implements Scanner.CharacterScanner {
		private final char[] chars;
		private int pos;

		BufferReader(char[] chars, int start) {
			this.chars = chars;
			this.pos = start;
		}

		@Override
		public int read() {
			int p = pos++;
			return p >= 0 && p < chars.length ? chars[p] : EOF;
		}

		@Override
		public void unread() {
			pos--;
		}
	}
}
