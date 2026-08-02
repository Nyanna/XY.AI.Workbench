package xy.ai.workbench.editor.outline;

import java.util.Arrays;
import java.util.LinkedHashSet;
import java.util.Set;

import org.eclipse.jface.action.Action;
import org.eclipse.jface.action.IAction;
import org.eclipse.jface.action.IMenuCreator;
import org.eclipse.jface.layout.GridDataFactory;
import org.eclipse.jface.layout.GridLayoutFactory;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.SelectionChangedEvent;
import org.eclipse.jface.viewers.StructuredSelection;
import org.eclipse.jface.viewers.TreeViewer;
import org.eclipse.jface.viewers.ViewerFilter;
import org.eclipse.swt.SWT;
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

		createFilterAction();
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
