package xy.ai.workbench.editor.mdast;

import java.util.List;

import xy.ai.workbench.editor.mdast.nodes.Elements;
import xy.ai.workbench.editor.mdast.nodes.Node;
import xy.ai.workbench.tools.LineIndex;
import xy.ai.workbench.tools.Scanner;

public class MarkdownDocument {
	private final IDocumentBuffer buffer;
	private final LineIndex lines = new LineIndex();

	private Node root = new Node(null, Elements.ROOT);

	public MarkdownDocument(IDocumentBuffer buffer) {
		this.buffer = buffer;
		root.enableSpellcheck = true;
	}

	public ModificationRange update(int offset, int removed, int inserted) {
		int lo = offset;
		int hi = offset + removed;
		int delta = inserted - removed;
		lines.update(buffer, offset, removed, inserted);

		Node sec = findForUpdate(lo, hi);
		while (true) {
			Node parent = sec.parent;
			int absStart = sec.getOffset();
			int newLen = Math.max(hi - absStart, sec.length()) + delta;
			Node rn = parse(absStart, absStart + newLen);

			if (parent == null || isCompatible(rn.children, sec, parent)) {
				Node changed = replace(sec, rn.children, delta);
				return expand(changed, offset, offset + inserted);
			}
			sec = parent;
		}
	}

	private Node parse(int absStart, int absEnd) {
		char[] slice = readChars(absStart, absEnd - absStart);
		Node rn = new Node(null, Elements.ROOT);
		boolean documentStart = absStart == 0;
		boolean documentEnd = absEnd == buffer.length();
		Scanner scanner = new Scanner(new BufferReader(slice, 0), documentStart, documentEnd);
		Elements.ROOT.scan(scanner, rn);
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

	private Node replace(Node sec, List<Node> nchilds, int delta) {
		Node parent = sec.parent;
		if (parent == null) {
			root.children.clear();
			for (Node c : nchilds)
				root.children.add(reparent(c, root));
			root.end += delta;
			return sec;
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
		return parent;
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

	public ModificationRange find(int lo, int hi) {
		return expand(find(root, lo, hi), lo, hi);
	}

	private ModificationRange expand(Node node, int lo, int hi) {
		int docLen = lines.bufferLength();
		lo = clamp(lo, docLen);
		hi = clamp(hi, docLen);
		if (hi < lo)
			hi = lo;

		int startLine = lines.lineOfOffset(lo);
		int endLine = hi > lo ? lines.lineOfOffset(hi - 1) : startLine;
		int start = lines.lineStartOffset(startLine);
		int end = lines.lineEndOffset(endLine);
		return new ModificationRange(node, start, end);
	}

	private static int clamp(int value, int max) {
		return Math.max(0, Math.min(value, max));
	}

	private Node findForUpdate(int lo, int hi) {
		Node tail = lastLeaf(root);
		if (tail != root && tail.getEndOffset() <= lo)
			return tail;
		return find(root, lo, hi);
	}

	private Node lastLeaf(Node node) {
		while (!node.children.isEmpty())
			node = node.children.get(node.children.size() - 1);
		return node;
	}

	private Node find(Node node, int lo, int hi) {
		for (Node child : node.children) {
			int cs = child.getOffset();
			if (cs <= lo && hi <= cs + child.length())
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
