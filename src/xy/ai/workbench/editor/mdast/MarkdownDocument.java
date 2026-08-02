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

			// Fast path: only new siblings were appended at the tail that do not
			// fit into `parent` but belong to an ancestor. Splice them in instead
			// of re-parsing the whole parent section on the next loop iteration.
			Node appended = appendSiblings(sec, parent, rn.children, delta);
			if (appended != null)
				return expand(appended, offset, offset + inserted);

			sec = parent;
		}
	}

	/**
	 * Handles the common "append within a section" case without re-parsing the
	 * whole parent. The re-parse (anchored at {@code sec}) is split into a
	 * {@code head} that stays inside {@code parent} (starting with the re-parsed
	 * {@code sec}) and a {@code tail} of newly appended nodes that overflow into
	 * the nearest ancestor able to contain them.
	 *
	 * <p>
	 * Only applied when the edit is provably a clean tail append (spine of
	 * last-children, every touched ancestor ending exactly at {@code sec}).
	 * Returns the highest changed node, or {@code null} to fall back to the
	 * generic re-parse/climb behavior.
	 */
	private Node appendSiblings(Node sec, Node parent, List<Node> rchilds, int delta) {
		if (rchilds.isEmpty() || rchilds.get(0).instance != sec.instance)
			return null;
		if (!isSpineTail(sec, parent))
			return null;

		int absStart = sec.getOffset();
		int oldSecEnd = absStart + sec.length();

		// Split re-parsed nodes: leading nodes that fit into parent vs. overflow.
		int split = 1; // rchilds[0] mirrors sec and therefore fits into parent
		while (split < rchilds.size() && parent.instance.containChild(rchilds.get(split).instance))
			split++;
		if (split == rchilds.size())
			return null; // nothing overflows -> handled by isCompatible

		List<Node> head = rchilds.subList(0, split);
		List<Node> tail = rchilds.subList(split, rchilds.size());

		// Find the nearest ancestor able to host the whole overflow while every
		// intermediate ancestor cleanly ends at sec (last-child, no trailing).
		Node host = null;
		for (Node anc = parent.parent; anc != null; anc = anc.parent) {
			if (anc.getEndOffset() != oldSecEnd)
				return null;
			if (canContainAll(anc, tail)) {
				host = anc;
				break;
			}
			if (!isLastChild(anc))
				return null;
		}
		if (host == null)
			return null;

		int headEndAbs = absStart + head.get(head.size() - 1).end;
		int tailEndAbs = absStart + tail.get(tail.size() - 1).end;

		// 1) replace sec with head inside parent
		List<Node> siblings = parent.children;
		siblings.remove(siblings.size() - 1); // sec is the last child
		for (Node c : head) {
			c.start += sec.start;
			c.end += sec.start;
			siblings.add(reparent(c, parent));
		}

		// 2) parent and every ancestor up to host now end after the head content
		for (Node anc = parent; anc != host; anc = anc.parent)
			setEndOffset(anc, headEndAbs);

		// 3) attach the overflow as new trailing children of host
		int hostOffset = host.getOffset();
		for (Node c : tail) {
			c.start = absStart + c.start - hostOffset;
			c.end = absStart + c.end - hostOffset;
			host.children.add(reparent(c, host));
		}
		setEndOffset(host, tailEndAbs);

		// 4) propagate the growth to the ancestors above host
		for (Node anc = host.parent; anc != null; anc = anc.parent) {
			anc.end += delta;
			Node ap = anc.parent;
			if (ap == null)
				continue;
			List<Node> as = ap.children;
			int ai = as.indexOf(anc);
			for (int i = ai + 1; i < as.size(); i++)
				shift(as.get(i), delta);
		}
		return host;
	}

	private boolean isSpineTail(Node sec, Node parent) {
		List<Node> siblings = parent.children;
		if (siblings.isEmpty() || siblings.get(siblings.size() - 1) != sec)
			return false;
		return parent.getEndOffset() == sec.getEndOffset();
	}

	private boolean isLastChild(Node node) {
		Node parent = node.parent;
		if (parent == null)
			return false;
		List<Node> siblings = parent.children;
		return !siblings.isEmpty() && siblings.get(siblings.size() - 1) == node;
	}

	private boolean canContainAll(Node parent, List<Node> nodes) {
		for (Node n : nodes)
			if (!parent.instance.containChild(n.instance))
				return false;
		return true;
	}

	private void setEndOffset(Node node, int absEnd) {
		node.end = absEnd - (node.parent != null ? node.parent.getOffset() : 0);
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

		// TODO optimized when changing the exact same Node
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
