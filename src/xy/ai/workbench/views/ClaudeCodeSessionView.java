package xy.ai.workbench.views;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

import org.eclipse.jface.layout.TableColumnLayout;
import org.eclipse.jface.viewers.ArrayContentProvider;
import org.eclipse.jface.viewers.ColumnLabelProvider;
import org.eclipse.jface.viewers.ColumnWeightData;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.TableViewer;
import org.eclipse.jface.viewers.TableViewerColumn;
import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.FillLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.swt.widgets.Table;
import org.eclipse.swt.widgets.TableColumn;
import org.eclipse.ui.IActionBars;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.part.ViewPart;

import jakarta.inject.Inject;
import xy.ai.workbench.Activator;
import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.Model;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connectors.claudecode.ClaudeCodeSession;
import xy.ai.workbench.connectors.claudecode.ClaudeCodeSessionManager;
import xy.ai.workbench.connectors.claudecode.SessionParameters;
import xy.ai.workbench.connectors.claudecode.SessionState;

/**
 * Eclipse ViewPart that displays active Claude Code CLI sessions in real time.
 *
 * <h3>Layout</h3>
 * <ul>
 * <li>Single area containing a {@link TableViewer}.</li>
 * <li>Toolbar with a "Terminate" action for the selected session.</li>
 * </ul>
 *
 * <h3>Table columns</h3>
 * <ol>
 * <li><b>Session-UUID</b> — parameter hash before first start, then the real
 * UUID.</li>
 * <li><b>State</b> — Expired / Prompt / Ready / Created (priority order).</li>
 * <li><b>TTL</b> — remaining session life in minutes, or "—" if not yet
 * started.</li>
 * <li><b>Model</b> — API model name.</li>
 * <li><b>Effort</b> — reasoning/effort level.</li>
 * <li><b>Prompt</b> — initial prompt snippet, or live {@code lastParsedMessage}
 * while in use.</li>
 * </ol>
 *
 * <p>
 * The view registers a change listener with the
 * {@link ClaudeCodeSessionManager} and refreshes the table on any session state
 * change. A periodic timer refreshes the TTL column every 30 seconds even when
 * no prompt is active.
 * </p>
 */
public class ClaudeCodeSessionView extends ViewPart {

	/** The ID used in plugin.xml. */
	public static final String ID = "xy.ai.workbench.views.ClaudeCodeSessionView";

	/** Periodic TTL refresh interval in milliseconds. */
	private static final int TTL_REFRESH_INTERVAL_MS = 30_000;
	private static final ClaudeCodeSession CNEW_LAUDE_CODE_SESSION = new ClaudeCodeSession(
			ClaudeCodeSessionManager.CREATE_NEW_MARKER, null,
			new SessionParameters(Path.of("", ""), "", null, Model.NONE, Reasoning.Disabled, AgentProfile.basic, "") {
				public String getHash() {
					return "Create new session";
				};
			});

	@Inject
	org.eclipse.ui.IWorkbench workbench;

	private TableViewer viewer;
	private TableColumnLayout tableLayout;
	private ActionManager act = new ActionManager();
	private ClaudeCodeSessionManager sessionManager;

	private final java.util.function.Consumer<List<ClaudeCodeSession>> changeListener = sessions -> refreshAsync();

	private Runnable ttlRefreshRunnable;
	private boolean disposed = false;

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
			createColumn("Session-UUID", 15)
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((ClaudeCodeSession) e).getID()));

			createColumn("State", 10)
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> stateLabel((ClaudeCodeSession) e)));

			createColumn("TTL", 8)
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ttlLabel((ClaudeCodeSession) e)));

			createColumn("Model", 10).setLabelProvider(
					ColumnLabelProvider.createTextProvider(e -> ((ClaudeCodeSession) e).getParameters().model.name()));

			createColumn("Effort", 8).setLabelProvider(ColumnLabelProvider
					.createTextProvider(e -> ((ClaudeCodeSession) e).getParameters().reasoning.name()));

			createColumn("Prompt", 50)
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> promptLabel((ClaudeCodeSession) e)));
		}

		viewer.setContentProvider(ArrayContentProvider.getInstance());
		viewer.setInput(new ArrayList<ClaudeCodeSession>());

		viewer.addSelectionChangedListener(event -> {
			IStructuredSelection sel = viewer.getStructuredSelection();
			if (sel.isEmpty()) {
				sessionManager.setSelectedSessionUuid(null);
			} else {
				ClaudeCodeSession s = (ClaudeCodeSession) sel.getFirstElement();
				sessionManager.setSelectedSessionUuid(s.getSessionUuid());
			}
		});

		sessionManager.addChangeListener(changeListener);

		// Toolbar
		makeActions();
		IActionBars bars = getViewSite().getActionBars();
		act.fillLocalToolBar(bars.getToolBarManager());
		act.fillLocalPullDown(bars.getMenuManager());

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
				.selection(viewer, ClaudeCodeSession.class, session -> {
					sessionManager.terminateSessions(java.util.List.of(session.getID()));
				}).done();
	}

	private String stateLabel(ClaudeCodeSession s) {
		switch (s.getState()) {
		case EXPIRED:
			return "expired";
		case PROMPT:
			return "prompting";
		case READY:
			return "ready";
		case CREATED:
		default:
			return "created";
		}
	}

	private String ttlLabel(ClaudeCodeSession s) {
		long remaining = s.getRemainingTtlMinutes();
		if (remaining < 0)
			return "—"; // em dash: not yet started
		return remaining + " min";
	}

	private String promptLabel(ClaudeCodeSession s) {
		if (s.getState() == SessionState.PROMPT) {
			String msg = s.getLastParsedMessage();
			return msg != null ? msg : "Last message empty";
		}
		String snippet = s.getParameters().getTitle();
		return snippet != null ? snippet : "";
	}

	private void refreshAsync() {
		Display display = PlatformUI.getWorkbench().getDisplay();
		if (display != null && !display.isDisposed())
			display.asyncExec(this::refreshTable);
	}

	/** Updates the viewer input and refreshes. Must be called on the UI thread. */
	private void refreshTable() {
		if (viewer.getControl().isDisposed())
			return;
		var sessions = new ArrayList<ClaudeCodeSession>();
		sessions.add(CNEW_LAUDE_CODE_SESSION);
		sessions.addAll(sessionManager.getSessions());
		viewer.setInput(sessions);
		viewer.refresh();
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
