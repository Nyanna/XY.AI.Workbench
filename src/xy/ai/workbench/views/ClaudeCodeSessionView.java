package xy.ai.workbench.views;

import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashSet;
import java.util.List;
import java.util.Objects;
import java.util.Set;
import java.util.stream.Collectors;

import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IProject;
import org.eclipse.jface.action.Action;
import org.eclipse.jface.action.IAction;
import org.eclipse.jface.layout.TableColumnLayout;
import org.eclipse.jface.viewers.ArrayContentProvider;
import org.eclipse.jface.viewers.ColumnLabelProvider;
import org.eclipse.jface.viewers.ColumnWeightData;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.StructuredSelection;
import org.eclipse.jface.viewers.TableViewer;
import org.eclipse.jface.viewers.TableViewerColumn;
import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.FillLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.swt.widgets.Table;
import org.eclipse.swt.widgets.TableColumn;
import org.eclipse.ui.IActionBars;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchPartReference;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.part.ViewPart;
import org.eclipse.ui.IPartListener2;

import jakarta.inject.Inject;
import xy.ai.workbench.Activator;
import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.CacheMode;
import xy.ai.workbench.Model;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connectors.claudecode.CCSession;
import xy.ai.workbench.connectors.claudecode.CCSessionManager;
import xy.ai.workbench.connectors.claudecode.JsonUtil;
import xy.ai.workbench.connectors.claudecode.SessionParameters;
import xy.ai.workbench.connectors.claudecode.SessionState;

/**
 * Eclipse ViewPart that displays active Claude Code CLI sessions in real time.
 *
 * <h3>Layout</h3>
 * <ul>
 * <li>Single area containing a {@link TableViewer}.</li>
 * <li>Toolbar with a "Terminate" action for the selected session and a "Sync"
 * toggle that links the table selection to the currently focused editor.</li>
 * </ul>
 *
 * <h3>Table columns</h3>
 * <ol>
 * <li><b>ID</b> — abbreviated hash/UUID (first group).</li>
 * <li><b>State</b> — created / open / prompting / expired.</li>
 * <li><b>Detail</b> — live status information, see {@link #detailLabel}.</li>
 * </ol>
 *
 * <p>
 * The table is sorted by the time the last message was received (most recent
 * first); the "Create new session" dummy entry always stays on top. Double
 * clicking a row opens a popup with the full, copyable session details (full
 * id, TTL, model, effort, tools, systemprompt).
 * </p>
 *
 * <p>
 * The view registers a change listener with the {@link CCSessionManager} and
 * refreshes the table on any session state change. A periodic timer refreshes
 * the TTL column every 30 seconds even when no prompt is active.
 * </p>
 */
public class ClaudeCodeSessionView extends ViewPart {

	/** The ID used in plugin.xml. */
	public static final String ID = "xy.ai.workbench.views.ClaudeCodeSessionView";

	/** Periodic TTL refresh interval in milliseconds. */
	private static final int TTL_REFRESH_INTERVAL_MS = 30_000;
	private static final CCSession CNEW_LAUDE_CODE_SESSION = new CCSession(CCSessionManager.CREATE_NEW_MARKER, null,
			new SessionParameters(Path.of("", ""), "", null, Model.NONE, Reasoning.Disabled, AgentProfile.basic, "",
					CacheMode.Default) {
				public String getHash() {
					return "Create new session";
				};
			});

	@Inject
	org.eclipse.ui.IWorkbench workbench;

	private TableViewer viewer;
	private TableColumnLayout tableLayout;
	private ActionManager act = new ActionManager();
	private CCSessionManager sessionManager;

	private final java.util.function.Consumer<List<CCSession>> changeListener = sessions -> refreshAsync();

	private Runnable ttlRefreshRunnable;
	private boolean disposed = false;

	/** Whether the table selection follows the currently focused editor. */
	private boolean syncEnabled = true;
	private Set<String> knownSessionIds = new HashSet<>();

	private Path currentProjectPath;
	private String currentRelativeFilePath;

	private final IPartListener2 editorPartListener = new PartListener2Adapter() {
		@Override
		public void partActivated(IWorkbenchPartReference partRef) {
			maybeUpdate(partRef);
		}

		private void maybeUpdate(IWorkbenchPartReference partRef) {
			if (partRef.getPart(false) instanceof IEditorPart)
				updateCurrentEditor();
		}

		@Override
		public void partOpened(IWorkbenchPartReference partRef) {
			maybeUpdate(partRef);
		}
	};

	@Override
	public void createPartControl(Composite parent) {
		sessionManager = Activator.getDefault().cliSessionManager;

		parent.setLayout(new FillLayout());

		// Table composite
		Composite tableComp = new Composite(parent, SWT.NONE);
		tableComp.setLayout(tableLayout = new TableColumnLayout());

		viewer = new TableViewer(tableComp, SWT.SINGLE | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER | SWT.FULL_SELECTION);
		Table table = viewer.getTable();
		table.setHeaderVisible(true);
		table.setLinesVisible(true);

		{
			createColumn("ID", 20)
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> idLabel((CCSession) e)));

			createColumn("State", 15)
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((CCSession) e).getState().name()));

			createColumn("Detail", 65)
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> detailLabel((CCSession) e)));
		}

		viewer.setContentProvider(ArrayContentProvider.getInstance());
		viewer.setInput(new ArrayList<CCSession>());

		viewer.addSelectionChangedListener(event -> {
			IStructuredSelection sel = viewer.getStructuredSelection();
			if (sel.isEmpty()) {
				sessionManager.setSelectedSessionUuid(null);
			} else {
				CCSession s = (CCSession) sel.getFirstElement();
				sessionManager.setSelectedSessionUuid(s.getSessionUuid());
			}
		});

		viewer.addDoubleClickListener(event -> {
			IStructuredSelection sel = (IStructuredSelection) event.getSelection();
			if (!sel.isEmpty() && sel.getFirstElement() instanceof CCSession) {
				CCSession s = (CCSession) sel.getFirstElement();
				if (s != CNEW_LAUDE_CODE_SESSION)
					new SessionDetailDialog(viewer.getControl().getShell(), s).open();
			}
		});

		sessionManager.addChangeListener(changeListener);

		// Toolbar
		makeActions();
		IActionBars bars = getViewSite().getActionBars();
		act.fillLocalToolBar(bars.getToolBarManager());
		act.fillLocalPullDown(bars.getMenuManager());

		Action syncAction = new Action("Sync", IAction.AS_CHECK_BOX) {
			@Override
			public void run() {
				syncEnabled = isChecked();
				if (syncEnabled)
					syncSelectionToCurrentFile();
			}
		};
		syncAction.setToolTipText("Link session selection to the focused editor");
		syncAction.setChecked(syncEnabled);
		bars.getToolBarManager().add(syncAction);
		bars.getToolBarManager().update(true);

		IWorkbenchPage activePage = getSite().getPage();
		if (activePage != null)
			activePage.addPartListener(editorPartListener);
		updateCurrentEditor();

		ttlRefreshRunnable = new Runnable() {
			@Override
			public void run() {
				if (disposed)
					return;
				refreshTable(false);
				Display.getCurrent().timerExec(TTL_REFRESH_INTERVAL_MS, this);
			}
		};
		Display.getDefault().timerExec(TTL_REFRESH_INTERVAL_MS, ttlRefreshRunnable);
	}

	@Override
	public void dispose() {
		disposed = true;
		sessionManager.removeChangeListener(changeListener);
		IWorkbenchPage activePage = getSite().getPage();
		if (activePage != null)
			activePage.removePartListener(editorPartListener);
		Display.getDefault().timerExec(-1, ttlRefreshRunnable);
		super.dispose();
	}

	@Override
	public void setFocus() {
		viewer.getControl().setFocus();
	}

	private void makeActions() {
		act.create().text("Terminate Session", "Terminates the selected CLI session")
				.image(ISharedImages.IMG_TOOL_DELETE).toolbar().pullDown()
				.selection(viewer, CCSession.class, session -> {
					sessionManager.terminateSessions(java.util.List.of(session.getID()));
				}).done();
	}

	private void updateCurrentEditor() {
		currentProjectPath = null;
		currentRelativeFilePath = null;

		IWorkbenchPage page = getSite() != null ? getSite().getPage() : null;
		IEditorPart editor = page != null ? page.getActiveEditor() : null;
		if (editor != null) {
			IEditorInput input = editor.getEditorInput();
			if (input instanceof IFileEditorInput) {
				IFile file = ((IFileEditorInput) input).getFile();
				IProject project = file.getProject();
				if (project != null && project.getLocation() != null) {
					currentProjectPath = Paths.get(project.getLocation().toOSString());
					currentRelativeFilePath = file.getProjectRelativePath().toString();
				}
			}
		}

		if (syncEnabled)
			syncSelectionToCurrentFile();
	}

	private CCSession findAssociatedSession(List<CCSession> sessions) {
		if (currentProjectPath == null)
			return null;
		for (CCSession s : sessions) {
			SessionParameters p = s.getParameters();
			if (p != null && currentProjectPath.equals(p.cwd) && Objects.equals(currentRelativeFilePath, p.filePath))
				return s;
		}
		return null;
	}

	private void selectSession(CCSession session) {
		if (viewer == null || viewer.getControl().isDisposed())
			return;
		Object toSelect = session != null ? session : CNEW_LAUDE_CODE_SESSION;
		viewer.setSelection(new StructuredSelection(toSelect), true);
	}

	private void syncSelectionToCurrentFile() {
		if (viewer == null || viewer.getControl().isDisposed() || sessionManager == null)
			return;
		selectSession(findAssociatedSession(sessionManager.getSessions()));
	}

	private String idLabel(CCSession s) {
		String id = s.getID();
		if (id == null)
			return "";
		int dash = id.indexOf('-');
		return dash > 0 ? id.substring(0, dash) : id;
	}

	private String detailLabel(CCSession s) {
		if (s.getState() == SessionState.Prompting) {
			if (!s.isLastRawLineProcessed() && s.getLastRawLine() != null)
				return JsonUtil.abbreviate(s.getLastRawLine());
			String msg = s.getLastParsedMessage();
			if (msg != null && !msg.isBlank())
				return msg;
		}

		String fileName = fileNameOf(s.getParameters().getFilePath());
		String title = s.getParameters().getTitle();
		if (fileName != null && !fileName.isBlank())
			return fileName + ": " + s.stats.print();
		return title != null && !title.isBlank() ? title : "—";
	}

	private static String fileNameOf(String path) {
		if (path == null || path.isBlank())
			return null;
		int idx = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));
		return idx >= 0 ? path.substring(idx + 1) : path;
	}

	private void refreshAsync() {
		Display display = PlatformUI.getWorkbench().getDisplay();
		if (display != null && !display.isDisposed())
			display.asyncExec(() -> refreshTable(true));
	}

	/** Must be called on the UI thread. */
	private void refreshTable(boolean allowSyncOnNewSession) {
		if (viewer.getControl().isDisposed())
			return;

		List<CCSession> sessions = new ArrayList<>(sessionManager.getSessions());
		sessions.sort(
				Comparator.comparing(CCSession::getLastReceivedAt, Comparator.nullsLast(Comparator.reverseOrder())));

		Set<String> newIds = sessions.stream().map(CCSession::getID).collect(Collectors.toSet());
		List<CCSession> added = sessions.stream().filter(s -> !knownSessionIds.contains(s.getID()))
				.collect(Collectors.toList());

		List<CCSession> withDummy = new ArrayList<>();
		withDummy.add(CNEW_LAUDE_CODE_SESSION);
		withDummy.addAll(sessions);

		viewer.setInput(withDummy);
		viewer.refresh();

		knownSessionIds = newIds;

		if (syncEnabled && allowSyncOnNewSession && !added.isEmpty()) {
			CCSession match = findAssociatedSession(sessions);
			if (match != null && added.contains(match))
				selectSession(match);
		}
	}

	private TableViewerColumn createColumn(String label, int weight) {
		TableColumn col = new TableColumn(viewer.getTable(), SWT.NONE);
		col.setText(label);
		col.setMoveable(true);
		col.setResizable(true);
		col.setWidth(50);
		tableLayout.setColumnData(col, new ColumnWeightData(weight));
		return new TableViewerColumn(viewer, col);
	}
}
