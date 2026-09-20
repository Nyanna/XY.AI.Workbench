package xy.ai.workbench.view.session;

import java.util.Arrays;
import java.util.concurrent.atomic.AtomicReference;
import java.util.stream.Collectors;

import org.eclipse.swt.SWT;
import org.eclipse.swt.events.FocusListener;
import org.eclipse.swt.events.MouseListener;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Combo;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Control;
import org.eclipse.swt.widgets.Label;
import org.eclipse.swt.widgets.Text;
import org.eclipse.ui.forms.widgets.FormToolkit;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.CacheMode;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model;
import xy.ai.workbench.Reasoning;

/**
 * Key/Model/Profile/MaxToken/Temp/TopP/Reasoning/Cache row at the top of the
 * session view.
 */
public class ModelParametersSection {

	private final FormToolkit toolkit;

	public ModelParametersSection(FormToolkit toolkit) {
		this.toolkit = toolkit;
	}

	public void create(Composite body, ConfigManager cfg) {
		Composite top = new Composite(body, SWT.NONE);
		top.setLayout(new GridLayout(2, false));
		top.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));

		createKeyField(top, cfg);
		createModelCombo(top, cfg);
		createProfileCombo(top, cfg);

		Label maxTokenLabel = toolkit.createLabel(top, "Max Token:");
		maxTokenLabel.setLayoutData(new GridData());
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

		createReasoningSection(top, body, cfg, tempLabel, temp, topPLabel, topP, maxTokenLabel, maxToken);
		createCacheSection(top, cfg);
	}

	private void createKeyField(Composite top, ConfigManager cfg) {
		toolkit.createLabel(top, "Key:");
		Text keyInput = toolkit.createText(top, "", SWT.BORDER | SWT.PASSWORD);
		GridData kilay = new GridData(GridData.FILL_HORIZONTAL);
		kilay.widthHint = 10;
		keyInput.setLayoutData(kilay);
		keyInput.addModifyListener(e -> cfg.setKey(keyInput.getText()));
		keyInput.setText(cfg.getKeys() + "");
		keyInput.addMouseListener(MouseListener.mouseDownAdapter(m -> keyInput.setFocus()));
	}

	private void createModelCombo(Composite top, ConfigManager cfg) {
		toolkit.createLabel(top, "Model:");
		Combo modelSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);
		modelSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
		AtomicReference<Model[]> modelSelItems = new AtomicReference<>(new Model[0]);
		modelSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {
			int idx = modelSel.getSelectionIndex();
			Model[] items = modelSelItems.get();
			if (idx >= 0 && idx < items.length)
				cfg.setModel(items[idx]);
		}));
		cfg.addEnabledModelsObs(k -> {
			modelSelItems.set(k);
			modelSel.setItems(
					Arrays.stream(k).map((m) -> m.displayName).collect(Collectors.toList()).toArray(new String[0]));
			Model current = cfg.getModel();
			modelSel.setText(current != null ? current.displayName : "");
		}, true);
	}

	private void createProfileCombo(Composite top, ConfigManager cfg) {
		Label profileLabel = toolkit.createLabel(top, "Profile:");
		profileLabel.setLayoutData(new GridData());
		Combo profileSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);
		profileSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
		profileSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg
				.setProfile(profileSel.getText().isBlank() ? null : AgentProfile.fromName(profileSel.getText()))));
		cfg.addEnabledProfilesObs(k -> {
			profileSel
					.setItems(Arrays.stream(k).map((m) -> m.name).collect(Collectors.toList()).toArray(new String[0]));
			profileSel.setText(k.length > 0 ? k[0].name : "");
			toggleControl(profileLabel, profileSel, k.length > 0);
			top.layout();
		}, true);
		cfg.addProfileObs(p -> {
			profileSel.setText(p != null ? p.name : "");
		}, true);
	}

	private void createReasoningSection(Composite top, Composite body, ConfigManager cfg, Label tempLabel, Text temp,
			Label topPLabel, Text topP, Label maxTokenLabel, Text maxToken) {
		Label reasoningLabel = toolkit.createLabel(top, "Reasoning:");
		reasoningLabel.setLayoutData(new GridData());
		Composite secReason = new Composite(top, SWT.NONE);
		GridLayout secRLay = new GridLayout(2, false);
		secRLay.marginHeight = secRLay.marginWidth = 0;
		secReason.setLayout(secRLay);
		secReason.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));

		Combo reasSel = new Combo(secReason, SWT.DROP_DOWN | SWT.READ_ONLY);
		reasSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
		reasSel.addSelectionListener(
				SelectionListener.widgetSelectedAdapter(e -> cfg.setReasoning(Reasoning.valueOf(reasSel.getText()))));

		Text budget = toolkit.createText(secReason, "", SWT.BORDER);
		budget.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
		budget.addFocusListener(
				FocusListener.focusLostAdapter(e -> cfg.setReasoningBudget(Integer.parseInt(budget.getText()))));
		budget.addMouseListener(MouseListener.mouseDownAdapter(m -> budget.setFocus()));
		cfg.addBudgetObs(bg -> budget.setText(bg + ""), true);

		cfg.addModelObs(m -> {
			toggleControl(tempLabel, temp, m.cap.isSupportTemperature(cfg.getReasoning()));
			toggleControl(topPLabel, topP, m.cap.isSupportTopP(cfg.getReasoning()));
			toggleControl(maxTokenLabel, maxToken, m.cap.isSupportMaxToken());

			String[] reasonings = cfg.getReasonings();
			reasSel.setItems(reasonings);
			reasSel.setText(cfg.getReasoning().name());
			boolean hasReasoning = reasonings.length > 0 && !(reasonings.length == 1
					&& ("Default".equals(reasonings[0]) || "Disabled".equals(reasonings[0])));
			toggleControl(reasoningLabel, secReason, hasReasoning);
			body.layout();
		}, true);
		cfg.addReasoningObs(r -> {

			boolean enabled = Reasoning.Budget.equals(r);
			budget.setEnabled(enabled);
			budget.setVisible(enabled);
			((GridData) budget.getLayoutData()).exclude = !enabled;

			toggleControl(tempLabel, temp, cfg.getModel().cap.isSupportTemperature(r));

			secReason.layout();
			body.layout();
		}, true);
	}

	private void createCacheSection(Composite top, ConfigManager cfg) {
		Label cacheLabel = toolkit.createLabel(top, "Cache:");
		cacheLabel.setLayoutData(new GridData());
		Combo cacheSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);
		cacheSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));
		cacheSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg.setCacheMode(
				cacheSel.getText().isBlank() ? CacheMode.Default : CacheMode.valueOf(cacheSel.getText()))));
		cfg.addModelObs(m -> {
			cacheSel.setItems(Arrays.stream(m.cap.getCacheMode()).map((c) -> c.name()).collect(Collectors.toList())
					.toArray(new String[0]));
			cacheSel.setText(cfg.getCacheMode() != null ? cfg.getCacheMode().name() : "");
			toggleControl(cacheLabel, cacheSel, m.cap.getCacheMode().length > 0);
		}, true);
		cfg.addCacheObs(c -> {
			cacheSel.setText(c != null ? c.name() : "");
		}, true);
	}

	private void toggleControl(Label label, Control ctrl, boolean enabled) {
		label.setEnabled(enabled);
		label.setVisible(enabled);
		ctrl.setEnabled(enabled);
		ctrl.setVisible(enabled);
		((GridData) label.getLayoutData()).exclude = !enabled;
		((GridData) ctrl.getLayoutData()).exclude = !enabled;
	}
}
