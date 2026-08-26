package xy.ai.workbench.view;

import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Instant;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.zip.CRC32;

import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IProject;
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
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connector.claudecode.CCSession;
import xy.ai.workbench.connector.claudecode.CCSessionManager;
import xy.ai.workbench.connector.claudecode.SessionParameters;
import xy.ai.workbench.connector.claudecode.SessionState;
import xy.ai.workbench.view.ActionManager.ActionDescription;

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
 * the table every second so that the mm:ss countdown shown for
 * {@link SessionState#Open} sessions stays accurate, and so that the
 * selection can automatically fall back to "Create new session" once a
 * synced session expires.
 * </p>
 */
public class ClaudeCodeSessionView extends ViewPart {

	/** The ID used in plugin.xml. */
	public static final String ID = "xy.ai.workbench.views.ClaudeCodeSessionView";

	/** Periodic TTL refresh interval in milliseconds. */
	private static final int TTL_REFRESH_INTERVAL_MS = 1_000;
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
	private ActionDescription syncAction;
	private CCSessionManager sessionManager;
	private ConfigManager cfg;

	private final java.util.function.Consumer<List<CCSession>> changeListener = sessions -> refreshAsync();

	private Runnable ttlRefreshRunnable;
	private boolean disposed = false;

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
		cfg = Activator.getDefault().cfg;

		parent.setLayout(new FillLayout());

		// Table composite
		Composite tableComp = new Composite(parent, SWT.NONE);
		tableComp.setLayout(tableLayout = new TableColumnLayout());

		viewer = new TableViewer(tableComp, SWT.SINGLE | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER | SWT.FULL_SELECTION);
		Table table = viewer.getTable();
		table.setHeaderVisible(true);
		table.setLinesVisible(true);

		{
			createColumn("ID", 10)
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> idLabel((CCSession) e)));

			createColumn("State", 35)
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> stateLabel((CCSession) e)));

			createColumn("Detail", 55)
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
		bars.updateActionBars();

		IWorkbenchPage activePage = getSite().getPage();
		if (activePage != null)
			activePage.addPartListener(editorPartListener);
		updateCurrentEditor();

		ttlRefreshRunnable = new Runnable() {
			@Override
			public void run() {
				if (disposed)
					return;
				refreshTable();
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
		syncAction = act.create().toolbar().text("Sync", "Link session selection to the focused editor")
				.image(ISharedImages.IMG_ELCL_SYNCED).runnable(() -> {
					if (syncAction.isChecked())
						syncSelectionToCurrentFile();
				});
		syncAction.done();
		syncAction.setChecked(true);

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

		if (syncAction.isChecked())
			syncSelectionToCurrentFile();
	}

	private SessionParameters currentParameters() {
		if (currentProjectPath == null || cfg == null)
			return null;
		try {
			return SessionParameters.fromConfig(cfg, currentProjectPath, currentRelativeFilePath, "",
					Arrays.asList(cfg.getTools()));
		} catch (RuntimeException e) {
			return null;
		}
	}

	private CCSession findAssociatedSession(List<CCSession> sessions) {
		SessionParameters current = currentParameters();
		if (current == null)
			return null;
		String hash = current.getHash();
		for (CCSession s : sessions) {
			if (!s.isValid())
				continue;
			SessionParameters p = s.getParameters();
			if (p != null && hash.equals(p.getHash()))
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

	private String stateLabel(CCSession s) {
		if (SessionState.Prompting.equals(s.getState()) && s.getLastSentAt() != null)
			return s.getState().name() + " (" + (Instant.now().toEpochMilli() - s.getLastSentAt().toEpochMilli()) + ")";
		if (SessionState.Open.equals(s.getState()))
			return formatMMSS(s.getRemainingTtlSeconds());
		return s.getState().name();
	}

	private static String formatMMSS(long totalSeconds) {
		if (totalSeconds < 0)
			return "--:--";
		long minutes = totalSeconds / 60;
		long seconds = totalSeconds % 60;
		return String.format("%02d:%02d", minutes, seconds);
	}

	private String detailLabel(CCSession s) {
		if (s.getState() == SessionState.Prompting) {
			if (!s.isLastRawLineProcessed() && s.getLastRawLine() != null)
				return abbreviateForDisplay(s.getLastRawLine());
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

	private static String abbreviateForDisplay(String s) {
		if (s == null)
			return "null";
		CRC32 crc = new CRC32();
		crc.update(s.getBytes());
		return String.format("(%d chars total, %d) %s…", s.length(), crc.getValue(),
				s.length() > 40 ? s.substring(0, 40) : s);
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
			display.asyncExec(this::refreshTable);
	}

	private void refreshTable() {
		if (viewer.getControl().isDisposed())
			return;

		List<CCSession> sessions = new ArrayList<>(sessionManager.getSessions());
		sessions.sort(
				Comparator.comparing(CCSession::getLastReceivedAt, Comparator.nullsLast(Comparator.reverseOrder())));

		List<CCSession> withDummy = new ArrayList<>();
		withDummy.add(CNEW_LAUDE_CODE_SESSION);
		withDummy.addAll(sessions);

		viewer.setInput(withDummy);
		viewer.refresh();

		if (syncAction.isChecked()) {
			IStructuredSelection currentSel = viewer.getStructuredSelection();
			Object selected = currentSel.getFirstElement();
			CCSession match = findAssociatedSession(sessions);
			Object desired = match != null ? match : CNEW_LAUDE_CODE_SESSION;

			if (selected != desired)
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
