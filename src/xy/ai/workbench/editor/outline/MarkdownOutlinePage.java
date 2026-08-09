package xy.ai.workbench.editor.outline;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Set;

import org.eclipse.jface.action.Action;
import org.eclipse.jface.action.IAction;
import org.eclipse.jface.action.IMenuCreator;
import org.eclipse.jface.layout.GridDataFactory;
import org.eclipse.jface.layout.GridLayoutFactory;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.SelectionChangedEvent;
import org.eclipse.jface.viewers.StructuredSelection;
import org.eclipse.jface.viewers.TreeViewer;
import org.eclipse.jface.viewers.ViewerFilter;
import org.eclipse.swt.SWT;
import org.eclipse.swt.dnd.Clipboard;
import org.eclipse.swt.dnd.TextTransfer;
import org.eclipse.swt.dnd.Transfer;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Control;
import org.eclipse.swt.widgets.Display;
import org.eclipse.swt.widgets.Menu;
import org.eclipse.swt.widgets.MenuItem;
import org.eclipse.swt.widgets.Text;
import org.eclipse.ui.IActionBars;
import org.eclipse.ui.navigator.CommonViewer;
import org.eclipse.ui.navigator.ICommonFilterDescriptor;
import org.eclipse.ui.navigator.INavigatorFilterService;
import org.eclipse.ui.views.contentoutline.ContentOutlinePage;

import xy.ai.workbench.editor.AITextEditor;
import xy.ai.workbench.editor.mdast.MarkdownDocument;
import xy.ai.workbench.editor.mdast.nodes.Elements;
import xy.ai.workbench.editor.mdast.nodes.Node;

public class MarkdownOutlinePage extends ContentOutlinePage {

	/**
	 * Viewer id used to bind the CNF content extension and filters (see
	 * plugin.xml).
	 */
	private static final String VIEWER_ID = "xy.ai.workbench.editor.outline";

	private final AITextEditor editor;
	private boolean syncingFromEditor;

	private Composite container;
	private Text patternText;
	private CommonViewer viewer;
	private final RegexNodeFilter regexFilter = new RegexNodeFilter();

	public MarkdownOutlinePage(AITextEditor editor) {
		this.editor = editor;
	}

	@Override
	public void createControl(Composite parent) {
		container = new Composite(parent, SWT.NONE);
		container.setLayout(GridLayoutFactory.fillDefaults().margins(2, 2).spacing(0, 2).create());

		patternText = new Text(container, SWT.BORDER | SWT.SEARCH | SWT.ICON_CANCEL);
		patternText.setMessage("RegExp filter pattern…");
		patternText.setToolTipText("Regexp filter");
		patternText.setLayoutData(GridDataFactory.fillDefaults().grab(true, false).create());
		patternText.addModifyListener(e -> {
			regexFilter.setPattern(patternText.getText());
			refresh();
		});

		viewer = new CommonViewer(VIEWER_ID, container, getTreeStyle());
		viewer.getControl().setLayoutData(GridDataFactory.fillDefaults().grab(true, true).create());
		MarkdownDocument ast = editor.getUpdateManager().getAst();
		if (ast != null && ast.getRoot() != null)
			viewer.setInput(new NodeElement(ast.getRoot(), editor.getUpdateManager().getDocument(), null));
		viewer.addSelectionChangedListener(this::onOutlineSelection);
		applyFilters();

		createCopyAction();
		createFilterAction();
	}

	private void createCopyAction() {
		IActionBars bars = getSite().getActionBars();
		if (bars == null)
			return;

		Action action = new Action("Copy", IAction.AS_PUSH_BUTTON) {
			@Override
			public void run() {
				copySelectionToClipboard();
			}
		};
		action.setToolTipText("Copy the text of the selected nodes to the clipboard");
		bars.getToolBarManager().add(action);
		bars.updateActionBars();
	}

	private void copySelectionToClipboard() {
		if (!isAlive(viewer))
			return;
		NodeElement root = (NodeElement) viewer.getInput();
		if (root == null)
			return;

		List<NodeElement> tops;
		ISelection selection = viewer.getSelection();
		if (selection instanceof StructuredSelection ssel && !ssel.isEmpty()) {
			List<NodeElement> selected = new ArrayList<>();
			for (Object o : ssel.toList())
				if (o instanceof NodeElement ne)
					selected.add(ne);
			tops = topLevel(selected);
			tops.sort(Comparator.comparingInt(ne -> ne.node().getOffset()));
		} else
			tops = List.of(root);

		StringBuilder sb = new StringBuilder();
		for (NodeElement ne : tops) {
			String text = extractText(ne);
			if (text.isEmpty())
				continue;
			if (sb.length() > 0)
				sb.append('\n');
			sb.append(text);
		}
		if (sb.length() == 0)
			return;

		Clipboard clipboard = new Clipboard(Display.getDefault());
		try {
			clipboard.setContents(new Object[] { sb.toString() }, new Transfer[] { TextTransfer.getInstance() });
		} finally {
			clipboard.dispose();
		}
	}

	private List<NodeElement> topLevel(List<NodeElement> selected) {
		Set<Node> selectedNodes = new LinkedHashSet<>();
		for (NodeElement ne : selected)
			selectedNodes.add(ne.node());
		List<NodeElement> result = new ArrayList<>();
		for (NodeElement ne : selected) {
			boolean ancestorSelected = false;
			for (Node p = ne.node().parent; p != null; p = p.parent)
				if (selectedNodes.contains(p)) {
					ancestorSelected = true;
					break;
				}
			if (!ancestorSelected)
				result.add(ne);
		}
		return result;
	}

	private String extractText(NodeElement ne) {
		IDocument doc = ne.doc();
		Node node = ne.node();
		int offset = node.getOffset();
		int end = node.getEndOffset();
		if (doc == null || end <= offset)
			return "";
		try {
			StringBuilder sb = new StringBuilder();
			int cursor = offset;
			for (NodeElement child : ne.children()) {
				Node cn = child.node();
				int cStart = cn.getOffset();
				int cEnd = cn.getEndOffset();
				if (cStart > cursor)
					sb.append(doc.get(cursor, cStart - cursor));
				if (!isFiltered(child))
					sb.append(extractText(child));
				cursor = Math.max(cursor, cEnd);
			}
			if (end > cursor)
				sb.append(doc.get(cursor, end - cursor));
			return sb.toString();
		} catch (BadLocationException e) {
			return "";
		}
	}

	/** Whether {@code ne} is currently hidden by one of the active viewer filters. */
	private boolean isFiltered(NodeElement ne) {
		if (!isAlive(viewer))
			return false;
		for (ViewerFilter filter : viewer.getFilters())
			if (!filter.select(viewer, ne.parent(), ne))
				return true;
		return false;
	}

	private void createFilterAction() {
		IActionBars bars = getSite().getActionBars();
		if (bars == null)
			return;

		Action action = new Action("Filters…", IAction.AS_DROP_DOWN_MENU) {
			@Override
			public void run() {
				// selecting the action itself just opens the drop-down menu
			}
		};
		action.setToolTipText("Hide/show nodes by type");
		action.setMenuCreator(new IMenuCreator() {
			private Menu menu;

			@Override
			public void dispose() {
				if (menu != null)
					menu.dispose();
			}

			@Override
			public Menu getMenu(Control parent) {
				if (menu != null)
					menu.dispose();
				menu = new Menu(parent);
				populate(menu);
				return menu;
			}

			@Override
			public Menu getMenu(Menu parent) {
				return null;
			}
		});
		bars.getToolBarManager().add(action);
		bars.updateActionBars();
	}

	private void populate(Menu menu) {
		INavigatorFilterService filterService = viewer.getNavigatorContentService().getFilterService();
		for (ICommonFilterDescriptor descriptor : filterService.getVisibleFilterDescriptors()) {
			MenuItem item = new MenuItem(menu, SWT.CHECK);
			item.setText(descriptor.getName());
			if (descriptor.getDescription() != null)
				item.setToolTipText(descriptor.getDescription());
			item.setSelection(filterService.isActive(descriptor.getId()));
			item.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> toggleFilter(descriptor.getId())));
		}
	}

	private void toggleFilter(String filterId) {
		INavigatorFilterService filterService = viewer.getNavigatorContentService().getFilterService();
		Set<String> active = new LinkedHashSet<>();
		for (ICommonFilterDescriptor descriptor : filterService.getVisibleFilterDescriptors())
			if (filterService.isActive(descriptor.getId()))
				active.add(descriptor.getId());
		if (!active.remove(filterId))
			active.add(filterId);
		// Deliberately not using
		// INavigatorFilterService#activateFilterIdsAndUpdateViewer:
		// it replaces the viewer's whole filter list with just the active
		// commonFilters (StructuredViewer#setFilters), which would silently drop
		// the regex filter driven by the pattern text field. setActiveFilterIds
		// only updates the activation bookkeeping, so we can re-apply the merged
		// filter set (commonFilters + regex filter) ourselves via applyFilters().
		filterService.setActiveFilterIds(active.toArray(new String[0]));
		filterService.persistFilterActivationState();
		applyFilters();
	}

	private void applyFilters() {
		if (!isAlive(viewer))
			return;
		ViewerFilter[] cnfFilters = viewer.getNavigatorContentService().getFilterService().getVisibleFilters(true);
		ViewerFilter[] merged = Arrays.copyOf(cnfFilters, cnfFilters.length + 1);
		merged[cnfFilters.length] = regexFilter;
		viewer.setFilters(merged);
	}

	private void onOutlineSelection(SelectionChangedEvent event) {
		if (syncingFromEditor)
			return;
		if (event.getSelection() instanceof StructuredSelection sel && sel.getFirstElement() instanceof NodeElement ne)
			editor.selectAndRevealNode(ne.node());
	}

	public void refresh() {
		if (!isAlive(viewer))
			return;
		MarkdownDocument ast = editor.getUpdateManager().getAst();
		if (ast == null || ast.getRoot() == null)
			return;
		NodeElement current = (NodeElement) viewer.getInput();
		if (current.node() != ast.getRoot() || current.doc() != editor.getUpdateManager().getDocument())
			viewer.setInput(new NodeElement(ast.getRoot(), editor.getUpdateManager().getDocument(), null));
		else
			viewer.refresh();
	}

	public void selectNodeForOffset(int offset) {
		if (!isAlive(viewer))
			return;
		MarkdownDocument ast = editor.getUpdateManager().getAst();
		if (ast == null || ast.getRoot() == null)
			return;

		Node node = ast.find(offset, offset).getNode();
		if (node == null || node.instance == Elements.ROOT)
			return;

		NodeElement root = (NodeElement) viewer.getInput();
		NodeElement child = findNearestPresentAncestor(root, node);
		if (child == null)
			return;
		var sel = viewer.getSelection();
		if (sel instanceof StructuredSelection ssel && child.equals(ssel.getFirstElement()))
			return;
		Display.getDefault().asyncExec(() -> {
			syncingFromEditor = true;
			try {
				viewer.setSelection(new StructuredSelection(child), true);
			} finally {
				syncingFromEditor = false;
			}
		});
	}

	private NodeElement findNearestPresentAncestor(NodeElement root, Node node) {
		for (Node n = node; n != null && n.instance != Elements.ROOT; n = n.parent) {
			NodeElement match = root.find(n);
			if (match != null)
				return match;
		}
		return null;
	}

	private boolean isAlive(TreeViewer v) {
		return v != null && v.getControl() != null && !v.getControl().isDisposed();
	}

	@Override
	protected TreeViewer getTreeViewer() {
		return viewer;
	}

	@Override
	public Control getControl() {
		return container;
	}

	@Override
	public void setFocus() {
		if (isAlive(viewer))
			viewer.getControl().setFocus();
	}

	@Override
	public ISelection getSelection() {
		return isAlive(viewer) ? viewer.getSelection() : StructuredSelection.EMPTY;
	}

	@Override
	public void setSelection(ISelection selection) {
		if (isAlive(viewer))
			viewer.setSelection(selection);
	}
}
