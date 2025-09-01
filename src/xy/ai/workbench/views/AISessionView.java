package xy.ai.workbench.views;

import java.util.Arrays;

import org.eclipse.jface.viewers.ITableLabelProvider;
import org.eclipse.jface.viewers.LabelProvider;
import org.eclipse.swt.SWT;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.graphics.Image;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Button;
import org.eclipse.swt.widgets.Combo;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Label;
import org.eclipse.swt.widgets.List;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;
import org.eclipse.swt.widgets.Text;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.IWorkbench;
import org.eclipse.ui.forms.widgets.FormToolkit;
import org.eclipse.ui.forms.widgets.ScrolledForm;
import org.eclipse.ui.part.ViewPart;

import com.openai.models.ChatModel;

import jakarta.inject.Inject;
import xy.ai.workbench.AIAnswer;
import xy.ai.workbench.AISessionManager;
import xy.ai.workbench.Activator;

public class AISessionView extends ViewPart {

	/**
	 * The ID of the view as specified by the extension.
	 */
	public static final String ID = "xy.ai.workbench.views.AISessionView";

	@Inject
	IWorkbench workbench;

	private FormToolkit toolkit;
	private ScrolledForm form;

	private Label usageLabel;
	private List instructionList;
	private Text instructionEdit;
	private boolean isUpdating = false;

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
	public void createPartControl(Composite parent) {
		toolkit = new FormToolkit(parent.getDisplay());
		form = toolkit.createScrolledForm(parent);
		form.setText("AI Session");
		AISessionManager session = Activator.getDefault().session;

		Composite body = form.getBody();
		body.setLayout(new GridLayout());
		GridData defHorizontal = new GridData(GridData.FILL_HORIZONTAL);

		{ // upper parameters
			Composite top = new Composite(body, SWT.NONE);
			top.setLayout(new GridLayout(2, false));
			top.setLayoutData(defHorizontal);

			toolkit.createLabel(top, "Key:");
			Text keyInput = toolkit.createText(top, "", SWT.BORDER);
			keyInput.setLayoutData(defHorizontal);
			keyInput.addModifyListener(e -> session.setKey(keyInput.getText()));

			toolkit.createLabel(top, "Max Token:");
			Text maxToken = toolkit.createText(top, "", SWT.BORDER);
			maxToken.setLayoutData(defHorizontal);
			maxToken.setText(session.getMaxOutputTokens() + "");
			maxToken.addModifyListener(e -> session.setMaxOutputTokens(Long.parseLong(maxToken.getText())));

			toolkit.createLabel(top, "Temp:");
			Text temp = toolkit.createText(top, "", SWT.BORDER);
			temp.setLayoutData(defHorizontal);
			temp.setText(session.getTemperature() + "");
			temp.addModifyListener(e -> session.setTemperature(Double.parseDouble(temp.getText())));

			toolkit.createLabel(top, "topP:");
			Text topP = toolkit.createText(top, "", SWT.BORDER);
			topP.setLayoutData(defHorizontal);
			topP.setText(session.getTopP() + "");
			topP.addModifyListener(e -> session.setTopP(Double.parseDouble(topP.getText())));

			toolkit.createLabel(top, "Model:");
			Combo modelSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);
			modelSel.setItems(session.getModels());
			modelSel.setLayoutData(defHorizontal);
			modelSel.setText(session.getModel().asString());
			modelSel.addSelectionListener(
					SelectionListener.widgetSelectedAdapter(e -> session.setModel(ChatModel.of(modelSel.getText()))));
		}
		{ // instruction section

			Composite middle = new Composite(body, SWT.NONE);
			middle.setLayout(new GridLayout(1, false));
			middle.setLayoutData(defHorizontal);

			toolkit.createLabel(middle, "Instructions:");

			TabFolder instr = new TabFolder(middle, SWT.NONE);
			instr.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
			TabItem instrSel = new TabItem(instr, SWT.NONE);
			instrSel.setText("Select");
			TabItem instrEdit = new TabItem(instr, SWT.NONE);
			instrEdit.setText("Edit");

			{ // instruction select
				Composite comp = new Composite(instr, SWT.NONE);
				comp.setLayout(new GridLayout());
				instrSel.setControl(comp);
				instructionList = new List(comp, SWT.BORDER | SWT.MULTI | SWT.V_SCROLL);
				session.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);

				GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
				gridData.widthHint = 1;
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
					session.setSystemPrompt(upd);
				}));
			}

			{ // instruction edit
				Composite comp = new Composite(instr, SWT.NONE);
				comp.setLayout(new GridLayout());
				instrEdit.setControl(comp);
				instructionEdit = toolkit.createText(comp, "", SWT.BORDER | SWT.MULTI | SWT.WRAP | SWT.V_SCROLL);
				GridData gdMulti = new GridData(GridData.FILL_BOTH);
				session.addSystemPromptObs(p -> updateEditList(p.systemPrompt), true);
				instructionEdit.setLayoutData(gdMulti);
				instructionEdit.addModifyListener(e -> {
					if (isUpdating)
						return;
					session.setSystemPrompt(instructionEdit.getText().split("\n"));
				});
				GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
				gridData.widthHint = 1;
				instructionEdit.setLayoutData(gridData);
			}

			toolkit.createLabel(middle, "Inputs:");
			List inputlist = new List(middle, SWT.BORDER | SWT.MULTI | SWT.V_SCROLL);
			GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
			gridData.widthHint = 1;
			inputlist.setLayoutData(gridData);
			inputlist.setItems(new String[] { "Instructions", "Selection", "Editor", "Files" });
		}
		{
			Composite bottom = new Composite(body, SWT.NONE);
			bottom.setLayout(new GridLayout(2, false));
			bottom.setLayoutData(defHorizontal);

			toolkit.createLabel(bottom, "Output:");
			Combo input = new Combo(bottom, SWT.DROP_DOWN | SWT.READ_ONLY);
			String[] options = new String[] { "Append", "Replace", "Cursor" };
			input.setItems(options);
			input.select(0);

		}
		{
			Composite actions = new Composite(body, SWT.NONE);
			actions.setLayout(new GridLayout(1, false));
			actions.setLayoutData(defHorizontal);

			Button btn = new Button(actions, SWT.PUSH);
			btn.setText("Submit");
			btn.addSelectionListener(
					SelectionListener.widgetSelectedAdapter(e -> btn.getDisplay().asyncExec(() -> session.execute())));

			Button bbtn = new Button(actions, SWT.PUSH);
			bbtn.setText("Batch");
		}
		{
			Composite footer = new Composite(body, SWT.NONE);
			footer.setLayout(new GridLayout(1, false));
			footer.setLayoutData(defHorizontal);

			toolkit.createLabel(footer, "Token:");
			this.usageLabel = toolkit.createLabel(footer, new AIAnswer().print());
		}

		session.addAnswerObs(a -> {
			if (a != null)
				usageLabel.setText(a.print());
			else
				usageLabel.setText("running ...");
		});

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
