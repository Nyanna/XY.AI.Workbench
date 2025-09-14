package xy.ai.workbench.views;

import java.text.DateFormat;
import java.text.SimpleDateFormat;
import java.util.Date;

import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.jface.action.IMenuListener;
import org.eclipse.jface.action.IMenuManager;
import org.eclipse.jface.action.MenuManager;
import org.eclipse.jface.dialogs.MessageDialog;
import org.eclipse.jface.layout.TableColumnLayout;
import org.eclipse.jface.viewers.ColumnLabelProvider;
import org.eclipse.jface.viewers.ColumnWeightData;
import org.eclipse.jface.viewers.DoubleClickEvent;
import org.eclipse.jface.viewers.IDoubleClickListener;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.TableViewer;
import org.eclipse.jface.viewers.TableViewerColumn;
import org.eclipse.jface.viewers.Viewer;
import org.eclipse.jface.viewers.ViewerComparator;
import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.SashForm;
import org.eclipse.swt.dnd.Clipboard;
import org.eclipse.swt.dnd.TextTransfer;
import org.eclipse.swt.dnd.Transfer;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.swt.widgets.Menu;
import org.eclipse.swt.widgets.Table;
import org.eclipse.swt.widgets.TableColumn;
import org.eclipse.ui.IActionBars;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.IWorkbench;
import org.eclipse.ui.part.ViewPart;

import jakarta.inject.Inject;
import xy.ai.workbench.Activator;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.batch.AIBatchResponseManager;
import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.tools.Time;

public class AIBatchView extends ViewPart {

	/**
	 * The ID of the view as specified by the extension.
	 */
	public static final String ID = "xy.ai.workbench.views.AIBatchView";

	@Inject
	IWorkbench workbench;

	private TableViewer batchViewer;
	private TableViewer reqViewer;
	private ActionManager act = new ActionManager();

	private AIBatchManager batch;
	private AIBatchResponseManager batchRequests;
	private TableColumnLayout tableLayout;

	@Override
	public void createPartControl(Composite parent) {
		batch = Activator.getDefault().batch;
		batchRequests = Activator.getDefault().batchRequests;

		SashForm sash = new SashForm(parent, SWT.VERTICAL);

		{
			Composite viewerComp = new Composite(sash, SWT.NONE);
			batchViewer = new TableViewer(viewerComp,
					SWT.MULTI | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER | SWT.FULL_SELECTION);
			batchViewer.getControl().setLayoutData(new GridData(GridData.FILL, GridData.FILL, true, true));

			Table table = batchViewer.getTable();
			table.setHeaderVisible(true);
			table.setLinesVisible(true);
			viewerComp.setLayout(tableLayout = new TableColumnLayout());

			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "Batch Id", 70))
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((IAIBatch) e).getID()));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "Loaded", 5))
					.setLabelProvider(ColumnLabelProvider.createTextImageProvider((e) -> "", (e) -> {
						return workbench.getSharedImages()
								.getImage(((IAIBatch) e).getResult() != null ? ISharedImages.IMG_OBJ_FILE
										: ISharedImages.IMG_ETOOL_DELETE_DISABLED);
					}));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "Progress", 15))
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> {
						IAIBatch be = (IAIBatch) e;
						return be.getTaskCount() + " (" + be.getCompletion() * 100 + "%)";
					}));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "State", 40))
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> //
					((IAIBatch) e).getState().toString() + " (" + ((IAIBatch) e).getBatchStatusString() + ")"));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "Updated", 50)).setLabelProvider(
					ColumnLabelProvider.createTextProvider(e -> format(((IAIBatch) e).getStateDate())));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "Duration", 10)).setLabelProvider(
					ColumnLabelProvider.createTextProvider(e -> Time.secsToReadable(((IAIBatch) e).getDuration())));

			batchViewer.setComparator(new ViewerComparator() {
				public int compare(Viewer viewer, Object e1, Object e2) {
					IAIBatch t1 = (IAIBatch) e1;
					IAIBatch t2 = (IAIBatch) e2;
					return t2.getStateDate().compareTo(t1.getStateDate());
				};
			});

			table.requestLayout();
			batch.setViewer(batchViewer);
			batchViewer.setContentProvider(batch);
		}

		{
			Composite viewerComp = new Composite(sash, SWT.NONE);
			reqViewer = new TableViewer(viewerComp,
					SWT.MULTI | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER | SWT.FULL_SELECTION);
			reqViewer.getControl().setLayoutData(new GridData(GridData.FILL, GridData.FILL, true, true));
			reqViewer.setContentProvider(batchRequests);

			Table table = reqViewer.getTable();
			table.setHeaderVisible(true);
			table.setLinesVisible(true);
			viewerComp.setLayout(tableLayout = new TableColumnLayout());

			new TableViewerColumn(reqViewer, createColumn(reqViewer.getTable(), "Request Id", 15))
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((AIAnswer) e).id));
			new TableViewerColumn(reqViewer, createColumn(reqViewer.getTable(), "Comment", 80))
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> {
						AIAnswer be = (AIAnswer) e;
						return processComment(be.answer);
					}));
			new TableViewerColumn(reqViewer, createColumn(reqViewer.getTable(), "Size", 5))
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((AIAnswer) e).outputToken + ""));
			new TableViewerColumn(reqViewer, createColumn(reqViewer.getTable(), "Cost", 5))
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((AIAnswer) e).totalToken + ""));

			table.requestLayout();
		}

		// Create the help context id for the viewer's control
		workbench.getHelpSystem().setHelp(batchViewer.getControl(), "XY.AI.Workbench.viewer");
		getSite().setSelectionProvider(batchViewer);
		makeActions();
		hookContextMenu();
		hookDoubleClickAction();
		contributeToActionBars();
	}

	public String processComment(String text) {
		if (text == null) {
			return "";
		}
		String[] lines = text.split("\\R");
		StringBuilder resultBuilder = new StringBuilder();
		int lineCount = 0;
		for (String line : lines) {
			String trimmedLine = line.trim();
			if (!trimmedLine.isEmpty()) {
				resultBuilder.append(trimmedLine).append("\n");
				lineCount++;
				if (lineCount == 3) {
					break;
				}
			}
		}
		String processedString = resultBuilder.toString().trim();
		if (lineCount < lines.length && !processedString.isEmpty()) {
			return processedString + " ...";
		} else {
			return processedString;
		}
	}

	private String format(Date in) {
		return in != null ? SimpleDateFormat.getDateTimeInstance(DateFormat.MEDIUM, SimpleDateFormat.SHORT).format(in)
				: "";
	}

	private TableColumn createColumn(Table parent, String label, int weight) {
		var res = new TableColumn(parent, SWT.NONE);
		res.setMoveable(true);
		res.setResizable(true);
		res.setText(label);
		res.setWidth(50);
		tableLayout.setColumnData(res, new ColumnWeightData(weight));
		return res;
	}

	private void hookContextMenu() {
		MenuManager menuMgr = new MenuManager("#PopupMenu");
		menuMgr.setRemoveAllWhenShown(true);
		menuMgr.addMenuListener(new IMenuListener() {
			public void menuAboutToShow(IMenuManager manager) {
				AIBatchView.this.act.fillContextMenu(manager);
			}
		});
		Menu menu = menuMgr.createContextMenu(batchViewer.getControl());
		batchViewer.getControl().setMenu(menu);
		getSite().registerContextMenu(menuMgr, batchViewer);
	}

	private void contributeToActionBars() {
		IActionBars bars = getViewSite().getActionBars();
		act.fillLocalPullDown(bars.getMenuManager());
		act.fillLocalToolBar(bars.getToolBarManager());
	}

	private void makeActions() {
		act.create().text("Update Batches", "Retrieves and update batch states") //
				.image(ISharedImages.IMG_ELCL_SYNCED).pullDown().toolbar() //
				.job((mon) -> batch.updateBatches(mon, true)).done();
		act.create().text("Update Listed Batches only", "Retrieves and update listed batch states only") //
				.image(ISharedImages.IMG_TOOL_BACK).toolbar() //
				.job((mon) -> batch.updateBatches(mon, false)).done();
		act.create().text("Submit Batches", "Submit unscheduled batches") //
				.image(ISharedImages.IMG_TOOL_FORWARD).pullDown().toolbar() //
				.job((mon) -> batch.submitBatches(mon)).done();
		act.create().text("Clear View", "Clear batch list") //
				.image(ISharedImages.IMG_ETOOL_CLEAR).pullDown().toolbar() //
				.display(() -> {
					batch.clearBatches();
					batchRequests.clearAnswers();
					batchViewer.refresh();
					batchRequests.updateView(reqViewer);
				}).done();
		act.create().text("Copy JSON", "Copy JSON for use in batches") //
				.image(ISharedImages.IMG_TOOL_COPY).contextMenu() //
				.selection(batchViewer, IAIBatch.class, (elem) -> {
					if (elem.hasRequests())
						showMessage("Batch contains no original requests");
					else
						copyToClipboard(batch.requestsToString(elem));
				}).done();
		act.create().text("Copy Result JSON", "Copy Result JSON") //
				.image(ISharedImages.IMG_TOOL_COPY).contextMenu() //
				.selection(batchViewer, IAIBatch.class, (elem) -> {
					if (elem.getResult() == null)
						showMessage("Batch contains no response");
					else
						copyToClipboard(elem.getResult());
				}).done();
		act.create().text("Copy Error JSON", "Copy error JSON") //
				.image(ISharedImages.IMG_TOOL_COPY).contextMenu() //
				.selection(batchViewer, IAIBatch.class, (elem) -> {
					if (elem.getError() == null)
						showMessage("Batch contains no errors");
					else
						copyToClipboard(elem.getError());
				}).done();
		act.create().text("Cancel", "Cancel a processing batch") //
				.image(ISharedImages.IMG_TOOL_DELETE).contextMenu() //
				.selection(batchViewer, IAIBatch.class, (elem) -> {
					Job.create("Cacel Batch", (mon) -> {
						batch.cancelBatch(elem, mon);
						return Status.OK_STATUS;
					}).schedule();
				}).done();
		act.create().text("Remove", "Remove from listing") //
				.image(ISharedImages.IMG_TOOL_DELETE_DISABLED).contextMenu() //
				.selection(batchViewer, IAIBatch.class, (elem) -> {
					Job.create("Cacel Batch", (mon) -> {
						batch.removeBatch(elem, mon);
						return Status.OK_STATUS;
					}).schedule();
				}).done();
	}

	private void copyToClipboard(String string) {
		Clipboard clipboard = new Clipboard(Display.getDefault());
		try {
			clipboard.setContents(new Object[] { string }, new Transfer[] { TextTransfer.getInstance() });
		} finally {
			clipboard.dispose();
		}
	}

	private void hookDoubleClickAction() {
		batchViewer.addDoubleClickListener(new IDoubleClickListener() {
			public void doubleClick(DoubleClickEvent event) {
				IStructuredSelection selection = batchViewer.getStructuredSelection();
				Object obj = selection.getFirstElement();
				if (obj instanceof IAIBatch) {
					Job.create("Load Batches", (mon) -> {
						SubMonitor sub = SubMonitor.convert(mon, "Loading Batch", 3);
						sub.subTask("Loading result from API");
						batch.loadBatch((IAIBatch) obj, sub);
						sub.worked(1);
						sub.subTask("Converting respones");
						batchRequests.load((IAIBatch) obj, sub);
						sub.worked(1);
						sub.subTask("updating view");
						Display.getDefault().asyncExec(() -> batchRequests.updateView(reqViewer));
						sub.worked(1);
						sub.done();
						return Status.OK_STATUS;
					}).schedule();
				}
			}
		});
		reqViewer.addDoubleClickListener(new IDoubleClickListener() {
			public void doubleClick(DoubleClickEvent event) {
				IStructuredSelection selection = reqViewer.getStructuredSelection();
				Object obj = selection.getFirstElement();
				if (obj instanceof AIAnswer) {
					Job.create("Process Awswer", (mon) -> {
						Activator.getDefault().session.replaceTag(Display.getDefault(), (AIAnswer) obj, mon);
					}).schedule();
				}
			}
		});
	}

	private void showMessage(String message) {
		MessageDialog.openInformation(batchViewer.getControl().getShell(), "XY.AI Batch View", message);
	}

	@Override
	public void setFocus() {
		batchViewer.getControl().setFocus();
	}
}
