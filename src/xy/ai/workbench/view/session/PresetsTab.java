package xy.ai.workbench.view.session;

import java.util.ArrayList;
import org.eclipse.core.resources.IFile;
import org.eclipse.swt.SWT;
import org.eclipse.swt.events.MouseListener;
import org.eclipse.swt.events.SelectionAdapter;
import org.eclipse.swt.events.SelectionEvent;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Button;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.List;
import org.eclipse.swt.widgets.Shell;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;
import xy.ai.conscious.core.AgentProfile;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.conscious.core.OutputMode;
import xy.ai.conscious.model.CacheMode;
import xy.ai.conscious.model.Model;
import xy.ai.conscious.model.Reasoning;
import xy.ai.workbench.view.PresetHandler;

/**
 * "Presets" tab: save/load/reset the whole system-prompt configuration.
 */
public class PresetsTab {
  private List presetList;
  private final PresetList presets = new PresetList();
  private final java.util.List<IFile> presetFiles = new ArrayList<>();

  public void create(TabFolder instr, ConfigManager cfg, Shell shell, ToolSelection tools, Clearable instructionSelection, InputSelection inputs, RuleSelection rules) {
    TabItem presEdit = new TabItem(instr, SWT.NONE);
    presEdit.setText("Presets");
    Composite comp = new Composite(instr, SWT.NONE);
    GridLayout compLay = new GridLayout(1, false);
    comp.setLayout(compLay);
    GridData compDat = new GridData(SWT.FILL, SWT.FILL, true, true);
    comp.setLayoutData(compDat);
    presEdit.setControl(comp);
    Composite buttons = new Composite(comp, SWT.NONE);
    buttons.setLayout(new GridLayout(3, false));
    Button writeButton = new Button(buttons, SWT.PUSH);
    writeButton.setText("Save");
    writeButton.addSelectionListener(new SelectionAdapter() {

      @Override
      public void widgetSelected(SelectionEvent e) {
        PresetHandler.Settings settings = PresetHandler.Settings.of(cfg.getModel(), cfg.getReasoning(), cfg.getCacheMode(), cfg.getProfile());
        if (PresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), inputs.getSelection(), rules.getSelection(), settings, shell))
          refreshPresetList();
      }
    });
    Button clearToolsButton = new Button(buttons, SWT.PUSH);
    clearToolsButton.setText("Clear Tools");
    clearToolsButton.addSelectionListener(new SelectionAdapter() {

      @Override
      public void widgetSelected(SelectionEvent e) {
        tools.clear();
        cfg.setEnabledTools(new String[0]);
      }
    });
    Button clearAllButton = new Button(buttons, SWT.PUSH);
    clearAllButton.setText("Clear All");
    clearAllButton.addSelectionListener(new SelectionAdapter() {

      @Override
      public void widgetSelected(SelectionEvent e) {
        String[] cur = cfg.getSystemPrompt();
        String[] upd = new String[cur.length];
        for (int i = 0; i < cur.length; i++) {
          String line = cur[i];
          upd[i] = line.startsWith("#") ? line : "#" + line;
        }
        instructionSelection.clear();
        cfg.setSystemPrompt(upd);
        tools.clear();
        cfg.setEnabledTools(new String[0]);
        inputs.clear();
        rules.clear();
        cfg.setOuputMode(OutputMode.values()[0]);
      }
    });
    presetList = new List(comp, SWT.SINGLE | SWT.V_SCROLL | SWT.BORDER);
    GridData presetListDat = new GridData(SWT.FILL, SWT.FILL, true, true);
    presetListDat.heightHint = 60;
    presetList.setLayoutData(presetListDat);
    presetList.addMouseListener(MouseListener.mouseDownAdapter(m -> presetList.setFocus()));
    presetList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {
      int idx = presetList.getSelectionIndex();
      if (idx < 0 || idx >= presetFiles.size())
        return;
      PresetHandler.Preset preset = PresetHandler.loadPreset(presetFiles.get(idx));
      cfg.setSystemPrompt(preset.body);
      if (preset.tools != null) {
        tools.setSelection(preset.tools);
        cfg.setEnabledTools(preset.tools);
      }
      if (preset.outputMode != null)
        cfg.setOuputMode(preset.outputMode);
      if (preset.inputs != null)
        inputs.setSelection(preset.inputs);
      if (preset.rules != null)
        rules.setSelection(preset.rules);
      applySettings(cfg, preset);
    }));
    refreshPresetList();
  }

  private void refreshPresetList() {
    java.util.List<PresetList.Entry> entries = presets.load();
    presetFiles.clear();
    String[] labels = new String[entries.size()];
    for (int i = 0; i < entries.size(); i++) {
      presetFiles.add(entries.get(i).file);
      labels[i] = entries.get(i).label;
    }
    presetList.setItems(labels);
  }

  /**
   * Forces the settings a preset has active (uncommented) in its front matter.
   * The model is set first because changing it resets reasoning, cache and
   * profile to the model's defaults. Unknown or unsupported values are ignored.
   */
  private void applySettings(ConfigManager cfg, PresetHandler.Preset preset) {
    if (preset.model != null && !preset.model.isBlank())
      for (Model m : cfg.getEnabledModels()) {
        if (m.toString().equalsIgnoreCase(preset.model.trim())) {
          cfg.setModel(m);
          break;
        }
      }
    if (preset.reasoning != null && !preset.reasoning.isBlank())
      for (Reasoning r : cfg.getCapabilities().getReasonings()) {
        if (r.name().equalsIgnoreCase(preset.reasoning.trim())) {
          cfg.setReasoning(r);
          break;
        }
      }
    if (preset.cache != null && !preset.cache.isBlank())
      for (CacheMode c : cfg.getCapabilities().getCacheMode()) {
        if (c.name().equalsIgnoreCase(preset.cache.trim())) {
          cfg.setCacheMode(c);
          break;
        }
      }
    if (preset.profile != null && !preset.profile.isBlank())
      for (AgentProfile p : cfg.getCapabilities().getAgentProfiles()) {
        if (p.name().equalsIgnoreCase(preset.profile.trim())) {
          cfg.setProfile(p);
          break;
        }
      }
  }
}
