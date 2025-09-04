package xy.ai.workbench.views;

import java.text.DateFormat;
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
import org.eclipse.ui.IWorkbenchActionConstants;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.part.ViewPart;

import jakarta.inject.Inject;
import xy.ai.workbench.Activator;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.batch.AIBatchRequestManager;
import xy.ai.workbench.connectors.openai.IBatchEntry;
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
	private Action actUpdate, actEnqueue, actDetails, actInsert, actCpJson, actCancel, actCpResponse, actCpError;

	private AIBatchManager batch;
	private AIBatchRequestManager batchRequests = new AIBatchRequestManager();
	private TableColumnLayout tableLayout;

	@Override
	public void createPartControl(Composite parent) {
		batch = Activator.getDefault().batch;

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
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> ((IBatchEntry) e).getID()));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "Loaded", 5))
					.setLabelProvider(ColumnLabelProvider.createTextImageProvider((e) -> "", (e) -> {
						return workbench.getSharedImages()
								.getImage(((IBatchEntry) e).getResult() != null ? ISharedImages.IMG_OBJ_FILE
										: ISharedImages.IMG_ETOOL_DELETE_DISABLED);
					}));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "Progress", 15))
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> {
						IBatchEntry be = (IBatchEntry) e;
						return be.getTaskCount() + " (" + be.getCompletion() * 100 + "%)";
					}));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "State", 40))
					.setLabelProvider(ColumnLabelProvider.createTextProvider(e -> //
					((IBatchEntry) e).getState().toString() + " (" + ((IBatchEntry) e).getBatchStatusString() + ")"));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "Updated", 50)).setLabelProvider(
					ColumnLabelProvider.createTextProvider(e -> format(((IBatchEntry) e).getStateDate())));
			new TableViewerColumn(batchViewer, createColumn(batchViewer.getTable(), "Duration", 10)).setLabelProvider(
					ColumnLabelProvider.createTextProvider(e -> Time.secsToReadable(((IBatchEntry) e).getDuration())));

			batchViewer.setComparator(new ViewerComparator() {
				public int compare(Viewer viewer, Object e1, Object e2) {
					IBatchEntry t1 = (IBatchEntry) e1;
					IBatchEntry t2 = (IBatchEntry) e2;
					return t1.getStateDate().compareTo(t2.getStateDate());
				};
			});

			table.requestLayout();
			batch.setViewer(batchViewer);
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
				AIBatchView.this.fillContextMenu(manager);
			}
		});
		Menu menu = menuMgr.createContextMenu(batchViewer.getControl());
		batchViewer.getControl().setMenu(menu);
		getSite().registerContextMenu(menuMgr, batchViewer);
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
		manager.add(actCpResponse);
		manager.add(actCpError);
		manager.add(actCancel);
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
						batch.updateBatches();
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
						batch.submitBatches();
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
				IStructuredSelection selection = (IStructuredSelection) batchViewer.getSelection();
				if (!selection.isEmpty()) {
					Object selectedElement = selection.getFirstElement();
					if (selectedElement instanceof IBatchEntry) {
						IBatchEntry elem = (IBatchEntry) selectedElement;
						if (elem.hasRequests()) {
							showMessage("Batch contains no original requests");
						} else
							copyToClipboard(batch.requestsToString(elem));
					}
				}
			}
		};
		actCpJson.setText("Copy JSON");
		actCpJson.setToolTipText("Copy JSON for use in batches");
		actCpJson.setImageDescriptor(workbench.getSharedImages().getImageDescriptor(ISharedImages.IMG_TOOL_COPY));

		actCpResponse = new Action() {
			public void run() {
				IStructuredSelection selection = (IStructuredSelection) batchViewer.getSelection();
				if (!selection.isEmpty()) {
					Object selectedElement = selection.getFirstElement();
					if (selectedElement instanceof IBatchEntry) {
						IBatchEntry elem = (IBatchEntry) selectedElement;
						if (elem.getResult() == null) {
							showMessage("Batch contains no response");
						} else
							copyToClipboard(elem.getResult());
					}
				}
			}
		};
		actCpResponse.setText("Copy Result JSON");
		actCpResponse.setToolTipText("Copy Result JSON");
		actCpResponse.setImageDescriptor(workbench.getSharedImages().getImageDescriptor(ISharedImages.IMG_TOOL_COPY));

		actCpError = new Action() {
			public void run() {
				IStructuredSelection selection = (IStructuredSelection) batchViewer.getSelection();
				if (!selection.isEmpty()) {
					Object selectedElement = selection.getFirstElement();
					if (selectedElement instanceof IBatchEntry) {
						IBatchEntry elem = (IBatchEntry) selectedElement;
						if (elem.getError() == null) {
							showMessage("Batch contains no errors");
						} else
							copyToClipboard(elem.getError());
					}
				}
			}
		};
		actCpError.setText("Copy Error JSON");
		actCpError.setToolTipText("Copy error JSON");
		actCpError.setImageDescriptor(workbench.getSharedImages().getImageDescriptor(ISharedImages.IMG_TOOL_COPY));

		actCancel = new Action() {
			public void run() {
				IStructuredSelection selection = (IStructuredSelection) batchViewer.getSelection();
				if (!selection.isEmpty()) {
					Object selectedElement = selection.getFirstElement();
					if (selectedElement instanceof IBatchEntry) {
						IBatchEntry elem = (IBatchEntry) selectedElement;
						batch.cancelbatch(elem);
					}
				}
			}
		};
		actCancel.setText("Cancel");
		actCancel.setToolTipText("Cancel a processing batch");
		actCancel.setImageDescriptor(workbench.getSharedImages().getImageDescriptor(ISharedImages.IMG_TOOL_DELETE));

		actDetails = new Action() {
			public void run() {
				IStructuredSelection selection = batchViewer.getStructuredSelection();
				Object obj = selection.getFirstElement();
				if (obj instanceof IBatchEntry) {
					new Job("Load Batches") {
						@Override
						protected IStatus run(IProgressMonitor mon) {
							mon.subTask("Loading result from API");
							batch.loadBatch((IBatchEntry) obj);
							mon.worked(1);
							mon.subTask("Converting respones");
							batchRequests.load((IBatchEntry) obj, mon);
							mon.worked(1);
							Display.getDefault().asyncExec(() -> batchRequests.updateView(reqViewer));
							return Status.OK_STATUS;
						}
					}.schedule();
				}
			}
		};
		actInsert = new Action() {
			public void run() {
				IStructuredSelection selection = reqViewer.getStructuredSelection();
				Object obj = selection.getFirstElement();
				if (obj instanceof AIAnswer) {
					Activator.getDefault().session.processAnswer(Display.getDefault(), (AIAnswer) obj);
				}
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
		batchViewer.addDoubleClickListener(new IDoubleClickListener() {
			public void doubleClick(DoubleClickEvent event) {
				actDetails.run();
			}
		});
		reqViewer.addDoubleClickListener(new IDoubleClickListener() {
			public void doubleClick(DoubleClickEvent event) {
				actInsert.run();
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
