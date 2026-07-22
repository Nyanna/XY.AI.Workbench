package xy.ai.workbench.editors;

import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.viewers.ITreeContentProvider;
import org.eclipse.jface.viewers.LabelProvider;
import org.eclipse.jface.viewers.SelectionChangedEvent;
import org.eclipse.jface.viewers.StructuredSelection;
import org.eclipse.jface.viewers.TreeViewer;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.ui.views.contentoutline.ContentOutlinePage;

import xy.ai.workbench.mdast.MarkdownDocument;
import xy.ai.workbench.mdast.nodes.HeadingSection;
import xy.ai.workbench.mdast.nodes.Node;
import xy.ai.workbench.mdast.nodes.Paragraph;
import xy.ai.workbench.mdast.nodes.Root;

/**
 * Content outline page that visualizes the current state of the
 * {@link MarkdownDocument} MDast. It only reflects the tree structure and keeps
 * the node in which the editor caret is located highlighted.
 */
public class MarkdownOutlinePage extends ContentOutlinePage {

	private static final Object[] EMPTY = new Object[0];
	private static final int LABEL_LIMIT = 80;

	private final AITextEditor editor;
	private boolean syncingFromEditor;

	public MarkdownOutlinePage(AITextEditor editor) {
		this.editor = editor;
	}

	@Override
	public void createControl(Composite parent) {
		super.createControl(parent);

		TreeViewer viewer = getTreeViewer();
		viewer.setContentProvider(new OutlineContentProvider());
		viewer.setLabelProvider(new OutlineLabelProvider());
		viewer.setInput(editor.getMarkdownAst());
		viewer.addSelectionChangedListener(this::onOutlineSelection);
	}

	private void onOutlineSelection(SelectionChangedEvent event) {
		if (syncingFromEditor)
			return;
		if (event.getSelection() instanceof StructuredSelection sel
				&& sel.getFirstElement() instanceof Node node)
			editor.selectAndRevealNode(node);
	}

	/** Rebuilds the tree from the current AST state. */
	public void refresh() {
		TreeViewer viewer = getTreeViewer();
		if (!isAlive(viewer))
			return;
		if (viewer.getInput() != editor.getMarkdownAst())
			viewer.setInput(editor.getMarkdownAst());
		else
			viewer.refresh();
	}

	/** Highlights the deepest node that contains the given document offset. */
	public void selectNodeForOffset(int offset) {
		TreeViewer viewer = getTreeViewer();
		if (!isAlive(viewer))
			return;
		MarkdownDocument ast = editor.getMarkdownAst();
		if (ast == null || ast.getRoot() == null)
			return;

		Node node = deepest(ast.getRoot(), offset);
		if (node == null || node.instance == Root.INSTANCE)
			return;

		syncingFromEditor = true;
		try {
			viewer.setSelection(new StructuredSelection(node), true);
		} finally {
			syncingFromEditor = false;
		}
	}

	private Node deepest(Node node, int offset) {
		for (Node child : node.children)
			if (offset >= child.getOffset() && offset < child.getEndOffset())
				return deepest(child, offset);
		return node;
	}

	private boolean isAlive(TreeViewer viewer) {
		return viewer != null && viewer.getControl() != null && !viewer.getControl().isDisposed();
	}

	private String label(Node node) {
		String snippet = snippet(node);
		if (!snippet.isEmpty())
			return snippet;
		if (node.instance instanceof HeadingSection)
			return "Heading";
		if (node.instance instanceof Paragraph)
			return "Paragraph";
		return node.instance.getCategory().name();
	}

	private String snippet(Node node) {
		IDocument doc = editor.getMarkdownDocument();
		if (doc == null)
			return "";
		int offset = node.getOffset();
		int length = node.length();
		if (offset < 0 || length <= 0)
			return "";
		length = Math.min(length, doc.getLength() - offset);
		if (length <= 0)
			return "";
		try {
			String text = doc.get(offset, length).strip();
			int nl = text.indexOf('\n');
			if (nl >= 0)
				text = text.substring(0, nl).strip();
			if (text.length() > LABEL_LIMIT)
				text = text.substring(0, LABEL_LIMIT) + "…";
			return text;
		} catch (BadLocationException e) {
			return "";
		}
	}

	private final class OutlineContentProvider implements ITreeContentProvider {
		@Override
		public Object[] getElements(Object input) {
			if (input instanceof MarkdownDocument doc && doc.getRoot() != null)
				return doc.getRoot().children.toArray();
			return EMPTY;
		}

		@Override
		public Object[] getChildren(Object element) {
			return element instanceof Node node ? node.children.toArray() : EMPTY;
		}

		@Override
		public Object getParent(Object element) {
			return element instanceof Node node ? node.parent : null;
		}

		@Override
		public boolean hasChildren(Object element) {
			return element instanceof Node node && !node.children.isEmpty();
		}
	}

	private final class OutlineLabelProvider extends LabelProvider {
		@Override
		public String getText(Object element) {
			return element instanceof Node node ? label(node) : String.valueOf(element);
		}
	}
}
