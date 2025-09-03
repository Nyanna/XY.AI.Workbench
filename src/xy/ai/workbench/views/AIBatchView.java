package xy.ai.workbench.views;

import java.text.SimpleDateFormat;
import java.util.Date;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.IStatus;
import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.jface.action.Action;
import org.eclipse.jface.action.IMenuListener;
import org.eclipse.jface.action.IMenuManager;
import org.eclipse.jface.action.IToolBarManager;
import org.eclipse.jface.action.MenuManager;
import org.eclipse.jface.action.Separator;
import org.eclipse.jface.dialogs.MessageDialog;
import org.eclipse.jface.layout.TableColumnLayout;
import org.eclipse.jface.viewers.ColumnLabelProvider;
import org.eclipse.jface.viewers.ColumnWeightData;
import org.eclipse.jface.viewers.DoubleClickEvent;
import org.eclipse.jface.viewers.IDoubleClickListener;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.TableViewer;
import org.eclipse.jface.viewers.TableViewerColumn;
import org.eclipse.swt.SWT;
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
import org.eclipse.ui.IWorkbenchActionConstants;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.part.ViewPart;

import jakarta.inject.Inject;
import xy.ai.workbench.Activator;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.batch.AIBatchManager.BatchEntry;

public class AIBatchView extends ViewPart {

	/**
	 * The ID of the view as specified by the extension.
	 */
	public static final String ID = "xy.ai.workbench.views.AIBatchView";

	@Inject
	IWorkbench workbench;

	private TableViewer viewer;
	private Action actUpdate, actEnqueue, actDetails, actCpJson;

	private AIBatchManager batch;
	private TableColumnLayout tableLayout;

	@Override
	public void createPartControl(Composite parent) {
		batch = Activator.getDefault().batch;

		viewer = new TableViewer(parent, SWT.MULTI | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER | SWT.FULL_SELECTION);
		viewer.getControl().setLayoutData(new GridData(GridData.FILL, GridData.FILL, true, true));

		Table table = viewer.getTable();
		table.setHeaderVisible(true);
		table.setLinesVisible(true);
		parent.setLayout(tableLayout = new TableColumnLayout());

		new TableViewerColumn(viewer, createColumn(viewer.getTable(), "Id", 70))
				.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((BatchEntry) e).getID()));
//		new TableViewerColumn(viewer, createColumn(viewer.getTable(), "Inputfile", 50))
//				.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((BatchEntry) e).getInputFileID()));
		new TableViewerColumn(viewer, createColumn(viewer.getTable(), "Progress", 20))
				.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> {
					BatchEntry be = (BatchEntry) e;
					return be.getTaskCount() + "(" + be.getCompletion() * 100 + "%)";
				}));
		new TableViewerColumn(viewer, createColumn(viewer.getTable(), "State", 50))
				.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> //
				((BatchEntry) e).getState().toString() + " (" + ((BatchEntry) e).getBatchStatusString() + ")"));
		new TableViewerColumn(viewer, createColumn(viewer.getTable(), "Updated", 40))
				.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> format(((BatchEntry) e).getStateDate())));
//		new TableViewerColumn(viewer, createColumn(viewer.getTable(), "Expires", 50))
//				.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> format(((BatchEntry) e).getExpires())));
//		new TableViewerColumn(viewer, createColumn(viewer.getTable(), "Outputfile", 50))
//				.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((BatchEntry) e).getOutputFileID()));
//		new TableViewerColumn(viewer, createColumn(viewer.getTable(), "Errorfile", 50))
//				.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((BatchEntry) e).getErrorFileID()));
//		new TableViewerColumn(viewer, createColumn(viewer.getTable(), "Report", 50))
//				.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((BatchEntry) e).getBatchStatusString()));

		table.requestLayout();
		batch.setViewer(viewer);

		// Create the help context id for the viewer's control
		workbench.getHelpSystem().setHelp(viewer.getControl(), "XY.AI.Workbench.viewer");
		getSite().setSelectionProvider(viewer);
		makeActions();
		hookContextMenu();
		hookDoubleClickAction();
		contributeToActionBars();
	}

	private String format(Date in) {
		return in != null
				? SimpleDateFormat.getDateTimeInstance(SimpleDateFormat.SHORT, SimpleDateFormat.SHORT).format(in)
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
				AIBatchView.this.fillContextMenu(manager);
			}
		});
		Menu menu = menuMgr.createContextMenu(viewer.getControl());
		viewer.getControl().setMenu(menu);
		getSite().registerContextMenu(menuMgr, viewer);
	}

	private void contributeToActionBars() {
		IActionBars bars = getViewSite().getActionBars();
		fillLocalPullDown(bars.getMenuManager());
		fillLocalToolBar(bars.getToolBarManager());
	}

	private void fillLocalPullDown(IMenuManager manager) {
		manager.add(actUpdate);
		manager.add(new Separator());
		manager.add(actEnqueue);
	}

	private void fillContextMenu(IMenuManager manager) {
		manager.add(actCpJson);
		// Other plug-ins can contribute there actions here
		manager.add(new Separator(IWorkbenchActionConstants.MB_ADDITIONS));
	}

	private void fillLocalToolBar(IToolBarManager manager) {
		manager.add(actUpdate);
		manager.add(actEnqueue);
	}

	private void makeActions() {
		actUpdate = new Action() {
			public void run() {
				new Job("Check Batches") {
					@Override
					protected IStatus run(IProgressMonitor monitor) {
						Activator.getDefault().batch.updateBatches();
						return Status.OK_STATUS;
					}
				}.schedule();
			}
		};
		actUpdate.setText("Update Batches");
		actUpdate.setToolTipText("Retrieves and update batch states");
		actUpdate.setImageDescriptor(
				PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_ELCL_SYNCED));

		actEnqueue = new Action() {
			public void run() {
				new Job("Check Batches") {
					@Override
					protected IStatus run(IProgressMonitor monitor) {
						Activator.getDefault().batch.submitBatches();
						return Status.OK_STATUS;
					}
				}.schedule();
			}
		};
		actEnqueue.setText("Submit Batches");
		actEnqueue.setToolTipText("Submit unscheduled batches");
		actEnqueue.setImageDescriptor(workbench.getSharedImages().getImageDescriptor(ISharedImages.IMG_TOOL_FORWARD));

		actCpJson = new Action() {
			public void run() {
				IStructuredSelection selection = (IStructuredSelection) viewer.getSelection();
				if (!selection.isEmpty()) {
					Object selectedElement = selection.getFirstElement();
					if (selectedElement instanceof BatchEntry) {
						BatchEntry elem = (BatchEntry) selectedElement;
						if (elem.request.isEmpty()) {
							showMessage("Batch contains no original requests");
						} else
							copyToClipboard(batch.requestsToString(elem.request.values()));
					}
				}
			}
		};
		actCpJson.setText("Copy JSON");
		actCpJson.setToolTipText("Copy JSON for use in batches");
		actCpJson.setImageDescriptor(workbench.getSharedImages().getImageDescriptor(ISharedImages.IMG_TOOL_COPY));

		actDetails = new Action() {
			public void run() {
				IStructuredSelection selection = viewer.getStructuredSelection();
				Object obj = selection.getFirstElement();
				showMessage("Double-click detected on " + obj.toString());
			}
		};
	}

	private void copyToClipboard(String string) {
		Display display = Display.getDefault();
		Clipboard clipboard = new Clipboard(display);
		TextTransfer textTransfer = TextTransfer.getInstance();
		try {
			clipboard.setContents(new Object[] { string }, new Transfer[] { textTransfer });
		} finally {
			clipboard.dispose();
		}
	}

	private void hookDoubleClickAction() {
		viewer.addDoubleClickListener(new IDoubleClickListener() {
			public void doubleClick(DoubleClickEvent event) {
				actDetails.run();
			}
		});
	}

	private void showMessage(String message) {
		MessageDialog.openInformation(viewer.getControl().getShell(), "XY.AI Batch View", message);
	}

	@Override
	public void setFocus() {
		viewer.getControl().setFocus();
	}
}
