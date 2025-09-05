package xy.ai.workbench.views;

import java.util.Arrays;
import java.util.stream.Collectors;

import org.eclipse.jface.viewers.ITableLabelProvider;
import org.eclipse.jface.viewers.LabelProvider;
import org.eclipse.swt.SWT;
import org.eclipse.swt.events.FocusListener;
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
import xy.ai.workbench.OutputMode;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.models.AIAnswer;

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

	private Label usageLabel;
	private List instructionList;
	private Text instructionEdit;
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
		form.setText("AI Session");
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
			keyInput.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			keyInput.addModifyListener(e -> cfg.setKey(keyInput.getText()));
			keyInput.setText(cfg.getKey() + "");

			toolkit.createLabel(top, "Model:");
			Combo modelSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);
			modelSel.setItems(cfg.getModels());
			modelSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			modelSel.setText(cfg.getModel().name());
			modelSel.addSelectionListener(
					SelectionListener.widgetSelectedAdapter(e -> cfg.setModel(Model.valueOf(modelSel.getText()))));

			toolkit.createLabel(top, "Max Token:");
			Text maxToken = toolkit.createText(top, "", SWT.BORDER);
			maxToken.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			maxToken.addFocusListener(
					FocusListener.focusLostAdapter(e -> cfg.setMaxOutputTokens(Long.parseLong(maxToken.getText()))));
			cfg.addOutputTokenObs(ot -> maxToken.setText(ot + ""), true);

			Label tempLabel = toolkit.createLabel(top, "Temp:");
			tempLabel.setLayoutData(new GridData());
			Text temp = toolkit.createText(top, "", SWT.BORDER);
			temp.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			temp.setText(cfg.getTemperature() + "");
			temp.addModifyListener(e -> cfg.setTemperature(Double.parseDouble(temp.getText())));

			Label topPLabel = toolkit.createLabel(top, "TopP:");
			topPLabel.setLayoutData(new GridData());
			Text topP = toolkit.createText(top, "", SWT.BORDER);
			topP.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
			topP.setText(cfg.getTopP() + "");
			topP.addModifyListener(e -> cfg.setTopP(Double.parseDouble(topP.getText())));

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
			cfg.addBudgetObs(bg -> budget.setText(bg + ""), true);

			cfg.addModelObs(m -> {
				{
					boolean enabled = m.cap.isSupportTemperature();
					tempLabel.setEnabled(enabled);
					tempLabel.setVisible(enabled);
					temp.setEnabled(enabled);
					temp.setVisible(enabled);
					((GridData) tempLabel.getLayoutData()).exclude = !enabled;
					((GridData) temp.getLayoutData()).exclude = !enabled;
				}

				{
					boolean enabled = m.cap.isSupportTopP();
					topPLabel.setEnabled(enabled);
					topPLabel.setVisible(enabled);
					topP.setEnabled(enabled);
					topP.setVisible(enabled);
					((GridData) topPLabel.getLayoutData()).exclude = !enabled;
					((GridData) topP.getLayoutData()).exclude = !enabled;
				}

				reasSel.setItems(cfg.getReasonings());
				reasSel.setText(cfg.getReasoning().name());
				body.layout();
			}, true);
			cfg.addReasoningObs(r -> {

				boolean enabled = Reasoning.Budget.equals(r);
				budget.setEnabled(enabled);
				budget.setVisible(enabled);
				((GridData) budget.getLayoutData()).exclude = !enabled;

				body.layout();
			}, true);
		}
		{ // instruction section

			Composite middle = new Composite(body, SWT.NONE);
			middle.setLayout(new GridLayout(1, false));
			GridData ldat2 = new GridData(SWT.FILL, SWT.FILL, true, true);
			;
			ldat2.heightHint = 100;
			middle.setLayoutData(ldat2);

			toolkit.createLabel(middle, "Instructions:");

			TabFolder instr = new TabFolder(middle, SWT.NONE);
			GridData ldat1 = new GridData(SWT.FILL, SWT.FILL, true, true);
			ldat1.heightHint = 100;
			instr.setLayoutData(ldat1);
			TabItem instrSel = new TabItem(instr, SWT.NONE);
			instrSel.setText("Select");
			TabItem instrEdit = new TabItem(instr, SWT.NONE);
			instrEdit.setText("Edit");

			{ // instruction select
				Composite comp = new Composite(instr, SWT.NONE);
				comp.setLayout(new GridLayout());
				GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
				ldat3.heightHint = 100;
				comp.setLayoutData(ldat3);
				instrSel.setControl(comp);
				instructionList = new List(comp, SWT.BORDER | SWT.MULTI | SWT.V_SCROLL);
				cfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);

				GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
				gridData.widthHint = 1;
				gridData.heightHint = 100;
				instructionList.setLayoutData(gridData);
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
				instructionEdit = toolkit.createText(comp, "", SWT.BORDER | SWT.MULTI | SWT.WRAP | SWT.V_SCROLL);
				GridData gdMulti = new GridData(GridData.FILL_BOTH);
				cfg.addSystemPromptObs(p -> {
					if (!instructionEdit.isFocusControl())
						updateEditList(p.systemPrompt);
				}, true);
				instructionEdit.setLayoutData(gdMulti);
				instructionEdit.addModifyListener(e -> {
					if (isUpdating)
						return;
					cfg.setSystemPrompt(instructionEdit.getText().split("\n"));
				});
				GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
				gridData.widthHint = 1;
				instructionEdit.setLayoutData(gridData);
			}
			{ // inputs section
				Table table = new Table(middle, SWT.CHECK | SWT.BORDER | SWT.V_SCROLL | SWT.H_SCROLL);
				table.setHeaderVisible(true);
				table.setLinesVisible(true);

				TableColumn column1 = new TableColumn(table, SWT.NONE);
				column1.setText("Enable");
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
			actions.setLayout(new GridLayout(2, false));

			Button btn = new Button(actions, SWT.PUSH);
			btn.setText("Submit");
			btn.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> session.execute(btn.getDisplay())));

			Button bbtn = new Button(actions, SWT.PUSH);
			bbtn.setText("Batch");
			bbtn.addSelectionListener(SelectionListener
					.widgetSelectedAdapter(e -> session.queue(bbtn.getDisplay(), Activator.getDefault().batch)));
		}
		{ // status display
			Composite footer = new Composite(body, SWT.NONE);
			footer.setLayout(new GridLayout(1, false));

			toolkit.createLabel(footer, "Token:");
			this.usageLabel = toolkit.createLabel(footer, new AIAnswer().print());
		}

		session.addAnswerObs(a -> {
			form.getDisplay().asyncExec(() -> {
				if (a != null)
					usageLabel.setText(a.print());
				else
					usageLabel.setText("running ...");
			});
		});
		session.initializeInputs();

		form.reflow(true);
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
		} finally {
			isUpdating = false;
		}
	}

	@Override
	public void setFocus() {
		form.setFocus();
	}
}
