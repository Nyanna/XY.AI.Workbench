package xy.ai.workbench.views;

import java.util.Arrays;
import java.util.stream.Collectors;

import org.eclipse.jface.viewers.ITableLabelProvider;
import org.eclipse.jface.viewers.LabelProvider;
import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.SashForm;
import org.eclipse.swt.events.FocusListener;
import org.eclipse.swt.events.MouseListener;
import org.eclipse.swt.events.SelectionAdapter;
import org.eclipse.swt.events.SelectionEvent;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.graphics.Image;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Button;
import org.eclipse.swt.widgets.Combo;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.swt.widgets.Label;
import org.eclipse.swt.widgets.List;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;
import org.eclipse.swt.widgets.Table;
import org.eclipse.swt.widgets.TableColumn;
import org.eclipse.swt.widgets.TableItem;
import org.eclipse.swt.widgets.Text;
import org.eclipse.ui.IMemento;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.IViewSite;
import org.eclipse.ui.IWorkbench;
import org.eclipse.ui.PartInitException;
import org.eclipse.ui.forms.widgets.FormToolkit;
import org.eclipse.ui.forms.widgets.ScrolledForm;
import org.eclipse.ui.part.ViewPart;

import jakarta.inject.Inject;
import xy.ai.workbench.AISessionManager;
import xy.ai.workbench.Activator;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.InputMode;
import xy.ai.workbench.Model;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.OutputMode;
import xy.ai.workbench.Reasoning;

public class AISessionView extends ViewPart {

	/**
	 * The ID of the view as specified by the extension.
	 */
	public static final String ID = "xy.ai.workbench.views.AISessionView";

	public static AISessionView currentInstance;

	@Inject
	IWorkbench workbench;

	private FormToolkit toolkit;
	private ScrolledForm form;

	private Table usageLog;
	private List instructionList;
	private Text instructionEdit, instructionFree;
	private boolean isUpdating = false;

	public Display display;

	class ViewLabelProvider extends LabelProvider implements ITableLabelProvider {
		@Override
		public String getColumnText(Object obj, int index) {
			return getText(obj);
		}

		@Override
		public Image getColumnImage(Object obj, int index) {
			return getImage(obj);
		}

		@Override
		public Image getImage(Object obj) {
			return workbench.getSharedImages().getImage(ISharedImages.IMG_OBJ_ELEMENT);
		}
	}

	@Override
	public void saveState(IMemento memento) {
		super.saveState(memento);
		Activator.getDefault().cfg.saveConfig(memento);
	}

	@Override
	public void init(IViewSite site, IMemento memento) throws PartInitException {
		super.init(site, memento);
		Activator.getDefault().cfg.loadConfig(memento);
	}

	@Override
	public void createPartControl(Composite parent) {
		currentInstance = this;
		display = parent.getDisplay();
		toolkit = new FormToolkit(parent.getDisplay());
		form = toolkit.createScrolledForm(parent);
		ConfigManager cfg = Activator.getDefault().cfg;
		AISessionManager session = Activator.getDefault().session;

		Composite body = form.getBody();
		body.setLayout(new GridLayout());
		{ // upper parameters
			Composite top = new Composite(body, SWT.NONE);
			top.setLayout(new GridLayout(2, false));
			top.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));

			toolkit.createLabel(top, "Key:");
			Text keyInput = toolkit.createText(top, "", SWT.BORDER | SWT.PASSWORD);
			GridData kilay = new GridData(GridData.FILL_HORIZONTAL);
			kilay.widthHint = 10;
			keyInput.setLayoutData(kilay);
			keyInput.addModifyListener(e -> cfg.setKey(keyInput.getText()));
			keyInput.setText(cfg.getKeys() + "");
			keyInput.addMouseListener(MouseListener.mouseDownAdapter(m -> keyInput.setFocus()));

			toolkit.createLabel(top, "Model:");
			Combo modelSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);
			modelSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			modelSel.addSelectionListener(
					SelectionListener.widgetSelectedAdapter(e -> cfg.setModel(Model.valueOf(modelSel.getText()))));
			cfg.addEnabledModelsObs(k -> {
				modelSel.setItems(
						Arrays.stream(k).map((m) -> m.name()).collect(Collectors.toList()).toArray(new String[0]));
				modelSel.setText(cfg.getModel().name());
			}, true);

			toolkit.createLabel(top, "Max Token:");
			Text maxToken = toolkit.createText(top, "", SWT.BORDER);
			maxToken.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			maxToken.addFocusListener(
					FocusListener.focusLostAdapter(e -> cfg.setMaxOutputTokens(Long.parseLong(maxToken.getText()))));
			maxToken.addMouseListener(MouseListener.mouseDownAdapter(m -> maxToken.setFocus()));
			cfg.addOutputTokenObs(ot -> maxToken.setText(ot + ""), true);

			Label tempLabel = toolkit.createLabel(top, "Temp:");
			tempLabel.setLayoutData(new GridData());
			Text temp = toolkit.createText(top, "", SWT.BORDER);
			temp.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			temp.addFocusListener(
					FocusListener.focusLostAdapter(e -> cfg.setTemperature(Double.parseDouble(temp.getText()))));
			temp.addMouseListener(MouseListener.mouseDownAdapter(m -> temp.setFocus()));
			cfg.addTemperatureObs(t -> temp.setText(t + ""), true);

			Label topPLabel = toolkit.createLabel(top, "TopP:");
			topPLabel.setLayoutData(new GridData());
			Text topP = toolkit.createText(top, "", SWT.BORDER);
			topP.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			topP.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setTopP(Double.parseDouble(topP.getText()))));
			topP.addMouseListener(MouseListener.mouseDownAdapter(m -> topP.setFocus()));
			cfg.addTopPObs(tp -> topP.setText(tp + ""), true);

			toolkit.createLabel(top, "Reasoning:");
			Composite secReason = new Composite(top, SWT.NONE);
			GridLayout secRLay = new GridLayout(2, false);
			secRLay.marginHeight = secRLay.marginWidth = 0;
			secReason.setLayout(secRLay);
			secReason.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));

			Combo reasSel = new Combo(secReason, SWT.DROP_DOWN | SWT.READ_ONLY);
			reasSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			reasSel.addSelectionListener(SelectionListener
					.widgetSelectedAdapter(e -> cfg.setReasoning(Reasoning.valueOf(reasSel.getText()))));

			Text budget = toolkit.createText(secReason, "", SWT.BORDER);
			budget.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			budget.addFocusListener(
					FocusListener.focusLostAdapter(e -> cfg.setReasoningBudget(Integer.parseInt(budget.getText()))));
			budget.addMouseListener(MouseListener.mouseDownAdapter(m -> budget.setFocus()));
			cfg.addBudgetObs(bg -> budget.setText(bg + ""), true);

			cfg.addModelObs(m -> {
				toogleControl(tempLabel, temp, isTemperatureEnabled(m, cfg.getReasoning()));
				toogleControl(topPLabel, topP, m.cap.isSupportTopP());

				reasSel.setItems(cfg.getReasonings());
				reasSel.setText(cfg.getReasoning().name());
				body.layout();
			}, true);
			cfg.addReasoningObs(r -> {

				boolean enabled = Reasoning.Budget.equals(r);
				budget.setEnabled(enabled);
				budget.setVisible(enabled);
				((GridData) budget.getLayoutData()).exclude = !enabled;

				toogleControl(tempLabel, temp, isTemperatureEnabled(cfg.getModel(), r));

				secReason.layout();
				body.layout();
			}, true);
		}
		{ // instruction section

			Composite middle = new Composite(body, SWT.NONE);
			middle.setLayout(new GridLayout(1, false));
			GridData ldat2 = new GridData(SWT.FILL, SWT.FILL, true, true);
			ldat2.heightHint = 100;
			middle.setLayoutData(ldat2);

			toolkit.createLabel(middle, "Instructions:");
			Composite sashComp = new Composite(middle, SWT.NONE);
			sashComp.setLayout(new GridLayout(1, false));
			GridData scl = new GridData(SWT.FILL, SWT.FILL, true, true);
			scl.heightHint = 100;
			scl.widthHint = 1;
			sashComp.setLayoutData(scl);
			SashForm sash = new SashForm(sashComp, SWT.VERTICAL);
			sash.setLayout(new GridLayout(1, false));
			GridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);
			scl2.heightHint = 100;
			scl2.widthHint = 1;
			sash.setLayoutData(scl2);

			TabFolder instr = new TabFolder(sash, SWT.NONE);
			GridData ldat1 = new GridData(SWT.FILL, SWT.FILL, true, true);
			ldat1.heightHint = 100;
			instr.setLayoutData(ldat1);
			TabItem instrSel = new TabItem(instr, SWT.NONE);
			instrSel.setText("Select");
			TabItem instrEdit = new TabItem(instr, SWT.NONE);
			instrEdit.setText("Edit");
			TabItem presEdit = new TabItem(instr, SWT.NONE);
			presEdit.setText("Presets");

			{ // instruction select
				Composite comp = new Composite(instr, SWT.NONE);
				comp.setLayout(new GridLayout());
				GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
				ldat3.heightHint = 100;
				comp.setLayoutData(ldat3);
				instrSel.setControl(comp);
				instructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);
				cfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);

				GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
				gridData.widthHint = 1;
				gridData.heightHint = 100;
				instructionList.setLayoutData(gridData);
				instructionList.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionList.setFocus()));
				instructionList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {
					if (isUpdating)
						return;
					String[] cur = instructionList.getItems();
					String[] upd = new String[cur.length];
					java.util.List<String> sel = Arrays.asList(instructionList.getSelection());
					for (int i = 0; i < cur.length; i++) {
						String line = cur[i];
						boolean isSelected = sel.contains(line);
						if (!isSelected && !line.startsWith("#")) {
							line = "#" + line;
						} else if (isSelected && line.startsWith("#")) {
							line = line.substring(1);
						}
						upd[i] = line;
					}
					cfg.setSystemPrompt(upd);
				}));
			}

			{ // instruction edit
				Composite comp = new Composite(instr, SWT.NONE);
				comp.setLayout(new GridLayout());
				instrEdit.setControl(comp);
				instructionEdit = toolkit.createText(comp, "", SWT.WRAP | SWT.V_SCROLL);
				cfg.addSystemPromptObs(p -> {
					if (!instructionEdit.isFocusControl())
						updateEditList(p.systemPrompt);
				}, true);
				instructionEdit.addModifyListener(e -> {
					if (isUpdating)
						return;
					cfg.setSystemPrompt(instructionEdit.getText().split("\n"));
				});
				GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
				gridData.widthHint = 1;
				gridData.heightHint = 100;
				instructionEdit.setLayoutData(gridData);
				instructionEdit.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionEdit.setFocus()));
			}
			{ // instruction presets
				Composite comp = new Composite(instr, SWT.NONE);
				comp.setLayout(new GridLayout());
				presEdit.setControl(comp);

				Button readButton = new Button(comp, SWT.PUSH);
				readButton.setText("Load");
				readButton.addSelectionListener(new SelectionAdapter() {
					@Override
					public void widgetSelected(SelectionEvent e) {
						String fileContent = PresetHandler.readStringFromFile(getSite().getShell());
						if (fileContent != null)
							cfg.setSystemPrompt(fileContent.split("\n"));
					}
				});

				Button writeButton = new Button(comp, SWT.PUSH);
				writeButton.setText("Save");
				writeButton.addSelectionListener(new SelectionAdapter() {
					@Override
					public void widgetSelected(SelectionEvent e) {
						PresetHandler.writeStringToFile(String.join("\n", cfg.getSystemPrompt()), getSite().getShell());
					}
				});
			}
			{ // Free text
				instructionFree = toolkit.createText(sash, "", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);
				cfg.addSystemFreeObs(p -> {
					if (!instructionFree.isFocusControl())
						try {
							isUpdating = true;
							instructionFree.setText(p != null ? p : "");
							form.reflow(true);
						} finally {
							isUpdating = false;
						}
				}, true);
				instructionFree.addFocusListener(
						FocusListener.focusLostAdapter(e -> cfg.setSystemFree(instructionFree.getText())));
				GridData gridData = new GridData(GridData.FILL_HORIZONTAL);
				gridData.widthHint = 1;
				instructionFree.setLayoutData(gridData);
				instructionFree.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionFree.setFocus()));
			}
			sash.setWeights(3, 1);
			{ // inputs section
				Table table = new Table(middle, SWT.CHECK | SWT.BORDER | SWT.V_SCROLL);
				table.setHeaderVisible(true);
				table.setLinesVisible(true);

				TableColumn column1 = new TableColumn(table, SWT.NONE);
				column1.setText("On");
				column1.setWidth(50);

				TableColumn column2 = new TableColumn(table, SWT.NONE);
				column2.setText("Input");
				column2.setWidth(120);

				TableColumn column3 = new TableColumn(table, SWT.NONE);
				column3.setText("Chars");
				column3.setWidth(50);

				table.addListener(SWT.Selection, e -> {
					if (e.detail == SWT.CHECK) {
						TableItem item = (TableItem) e.item;
						InputMode mode = InputMode.valueOf(item.getText(1).replace(" ", "_"));
						cfg.setInputMode(mode, item.getChecked());
					}
				});

				for (int i = 0; i < InputMode.values().length; i++) {
					TableItem item = new TableItem(table, SWT.NONE);
					InputMode mode = InputMode.values()[i];
					item.setText(new String[] { "", mode.name().replace("_", " "), "0" });
					item.setChecked(cfg.isInputEnabled(mode));
					session.addInputStatObs(is -> {
						item.setText(new String[] { "", mode.name().replace("_", " "), is[mode.ordinal()] + "" });
					}, true);
					cfg.addInputObs(is -> {
						item.setChecked(is[mode.ordinal()]);
					}, true);
				}
			}
		}
		{ // output mode selection
			Composite bottom = new Composite(body, SWT.NONE);
			bottom.setLayout(new GridLayout(2, false));

			toolkit.createLabel(bottom, "Output:");
			Combo outputMode = new Combo(bottom, SWT.DROP_DOWN | SWT.READ_ONLY);
			String[] outputOptions = Arrays.stream(OutputMode.values()).map(e -> e.name()).collect(Collectors.toList())
					.toArray(new String[0]);
			outputMode.setItems(outputOptions);
			outputMode.select(cfg.getOuputMode().ordinal());
			outputMode.addSelectionListener(SelectionListener
					.widgetSelectedAdapter(e -> cfg.setOuputMode(OutputMode.valueOf(outputMode.getText()))));

		}
		{ // buttons
			Composite actions = new Composite(body, SWT.NONE);
			actions.setLayout(new GridLayout(3, false));

			Button btn = new Button(actions, SWT.PUSH);
			btn.setText("Prompt");
			btn.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> session.execute(btn.getDisplay())));

			Button bbtn = new Button(actions, SWT.PUSH);
			bbtn.setText("Enqueue");
			bbtn.addSelectionListener(SelectionListener
					.widgetSelectedAdapter(e -> session.queueAsync(bbtn.getDisplay(), Activator.getDefault().batch)));

			Button bsbtn = new Button(actions, SWT.PUSH);
			bsbtn.setText("Batch");
			bsbtn.addSelectionListener(SelectionListener.widgetSelectedAdapter(
					e -> session.queueAndSubmit(bsbtn.getDisplay(), Activator.getDefault().batch)));
		}
		{ // status display
			Composite footer = new Composite(body, SWT.NONE);
			footer.setLayout(new GridLayout(1, false));

			usageLog = new Table(footer, SWT.BORDER | SWT.V_SCROLL);
			usageLog.setHeaderVisible(true);
			usageLog.setLinesVisible(true);
			GridData gridData = new GridData();
			gridData.heightHint = 50;
			usageLog.setLayoutData(gridData);

			TableColumn column1 = new TableColumn(usageLog, SWT.NONE);
			column1.setText("Total");
			column1.setWidth(50);

			TableColumn column2 = new TableColumn(usageLog, SWT.NONE);
			column2.setText("In");
			column2.setWidth(50);

			TableColumn column3 = new TableColumn(usageLog, SWT.NONE);
			column3.setText("Out");
			column3.setWidth(50);

			TableColumn column4 = new TableColumn(usageLog, SWT.NONE);
			column4.setText("Reason");
			column4.setWidth(50);
		}

		session.addAnswerObs(a -> {
			form.getDisplay().asyncExec(() -> {
				if (a != null) {
					TableItem item = new TableItem(usageLog, SWT.NONE, 0);
					item.setText(new String[] { a.totalToken + "", a.inputToken + "", a.outputToken + "",
							a.reasoningToken + "" });
					usageLog.setTopIndex(0);
				}
			});
		});
		session.initializeInputs();

		form.reflow(true);
	}

	private boolean isTemperatureEnabled(Model m, Reasoning reasoning) {
		if (m.cap.getKeyPattern().equals(KeyPattern.Claude))
			return m.cap.isSupportTemperature() && Reasoning.Disabled.equals(reasoning);
		else
			return m.cap.isSupportTemperature();
	}

	private void toogleControl(Label label, Text text, boolean enabled) {
		label.setEnabled(enabled);
		label.setVisible(enabled);
		text.setEnabled(enabled);
		text.setVisible(enabled);
		((GridData) label.getLayoutData()).exclude = !enabled;
		((GridData) text.getLayoutData()).exclude = !enabled;
	}

	private void updateInstructionList(String[] systemPrompt) {
		try {
			isUpdating = true;
			instructionList.setItems(systemPrompt);
			instructionList.deselectAll();
			for (int i = 0; i < systemPrompt.length; i++) {
				if (!systemPrompt[i].startsWith("#"))
					instructionList.select(i);
			}
			form.reflow(true);
		} finally {
			isUpdating = false;
		}
	}

	private void updateEditList(String[] systemPrompt) {
		try {
			isUpdating = true;
			instructionEdit.setText(String.join("\n", systemPrompt));
			form.reflow(true);
		} finally {
			isUpdating = false;
		}
	}

	@Override
	public void setFocus() {
		// form.setFocus();
	}
}
