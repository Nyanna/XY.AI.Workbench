Es scheint das Parameter nicht richtig gesetzt werden nach dem Laden eines Preset in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java`.
Das kann mit der Editorlogik zu haben. Das Preset soll die Settings für den aktuellen Editor setzen. Es scheint mindestens ein UI update zu fehlen. Es schein das anwenden des Presets wirk nur verzögert.
SystemInit:  id=a096b7c1-7308-4969-8726-4ba8ead1eed5, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 04ff-1-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
  reason: Understand structure of PresetsTab
```
/answer 04ff-1-0 allow
Control Request:
```yaml
id: 04ff-2-1
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
        nodes:
        - id: 1vWmR7|FuAgX0
          excert: import java.util.ArrayList;
        - id: PresetsTab
          excert: "/**  * \"Presets\" tab: save/load/reset the whole system-prompt configuration.  *…"
          docstring: "\"Presets\" tab: save/load/reset the whole system-prompt configuration."
          children:
          - id: PresetsTab.QtgcNk|ll4T02
            excert: private List presetList;
          - id: PresetsTab.create
            excert: "public void create(TabFolder instr, ConfigManager cfg, Shell shell, ToolSelecti…"
          - id: PresetsTab.refreshPresetList
            excert: private void refreshPresetList()
          - id: PresetsTab.applySettings
            excert: "private void applySettings(ConfigManager cfg, PresetHandler.Preset preset)"
            docstring: Forces the settings a preset has active (uncommented) in its front matter.
```
/answer 04ff-2-1 allow
Control Request:
```yaml
id: 04ff-3-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    ids:
    - PresetsTab.create
    - PresetsTab.applySettings
  reason: Read load/apply logic
```
/answer 04ff-3-0 allow
Control Request:
```yaml
id: 04ff-4-3
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
        nodes:
        - id: PresetsTab.create
          type: MethodDeclaration
          code: |2
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
        - id: PresetsTab.applySettings
          type: MethodDeclaration
          code: |2
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
```
/answer 04ff-4-3 allow
ReasoningToken: 29
Control Request:
```yaml
id: 04ff-5-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: "void (setModel|setReasoning|setCacheMode|setProfile|setSystemPrompt|setOuputMode)\\(|getCapabilities\\(\\) *\\{"
    include:
    - '*.java'
    limit: 20
  reason: Find ConfigManager setters
```
/answer 04ff-5-0 allow
Control Request:
```yaml
id: 04ff-6-5
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src
```
/answer 04ff-6-5 allow
Control Request:
```yaml
id: 04ff-7-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: (setModel|setReasoning|setCacheMode|setProfile)\(
    include:
    - '*.java'
    limit: 40
  reason: Find setter usage/definitions
```
/answer 04ff-7-0 allow
Control Request:
```yaml
id: 04ff-8-7
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src
        matches:
        - path: xy/ai/workbench/view/session/ModelParametersSection.java
          matches:
          - "cfg.setModel(items[idx]);"
          - ".setProfile(profileSel.getText().isBlank() ? null : AgentProfile.fromName(profileSel.getText()))));"
          - SelectionListener.widgetSelectedAdapter(e -> cfg.setReasoning(Reasoning.valueOf(reasSel.getText()))));
          - cacheSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg.setCacheMode(
        - path: xy/ai/workbench/view/session/PresetsTab.java
          matches:
          - cfg.setModel(m);
          - cfg.setReasoning(r);
          - cfg.setCacheMode(c);
          - cfg.setProfile(p);
        - path: xy/ai/workbench/eclipse/ConfigManagerAdapter.java
          matches:
          - '* capability-dependent settings are applied - otherwise setModel()''s own'
          - cm.setModel(resolved);
          - cm.setReasoning(Reasoning.valueOf(snap.reasoning));
          - cm.setCacheMode(CacheMode.valueOf(snap.cacheMode));
          - cm.setProfile(AgentProfile.valueOf(snap.profile));
```
/answer 04ff-8-7 allow
Control Request:
```yaml
id: 04ff-9-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
  reason: Understand UI sync and adapter
```
/answer 04ff-9-0 allow
Control Request:
```yaml
id: 04ff-a-9
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
        nodes:
        - id: THYa8e|4GE4cM
          excert: import java.util.Arrays;
        - id: ModelParametersSection
          excert: /**  * Key/Model/Profile/MaxToken/Temp/TopP/Reasoning/Cache row at the top of t…
          docstring: Key/Model/Profile/MaxToken/Temp/TopP/Reasoning/Cache row at the top of the
          children:
          - id: ModelParametersSection.YoFCYV|mp3uQC
            excert: private final FormToolkit toolkit;
          - id: ModelParametersSection.ModelParametersSection
            excert: public ModelParametersSection(FormToolkit toolkit)
          - id: ModelParametersSection.create
            excert: "public void create(Composite body, ConfigManager cfg)"
          - id: ModelParametersSection.createKeyField
            excert: "private void createKeyField(Composite top, ConfigManager cfg)"
          - id: ModelParametersSection.createModelCombo
            excert: "private void createModelCombo(Composite top, ConfigManager cfg)"
          - id: ModelParametersSection.createProfileCombo
            excert: "private void createProfileCombo(Composite top, ConfigManager cfg)"
          - id: ModelParametersSection.createReasoningSection
            excert: "private void createReasoningSection(Composite top, Composite body, ConfigManage…"
          - id: ModelParametersSection.createCacheSection
            excert: "private void createCacheSection(Composite top, ConfigManager cfg)"
          - id: ModelParametersSection.toggleControl
            excert: "private void toggleControl(Label label, Control ctrl, boolean enabled)"
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
        nodes:
        - id: FrqhCb|7k5EU6
          excert: import java.util.Arrays;
        - id: ConfigManagerAdapter
          excert: "public class ConfigManagerAdapter extends ConfigManager {"
          children:
          - id: ConfigManagerAdapter.3AMMWd|LiSBBr
            excert: private ConfigManager cm;
          - id: ConfigManagerAdapter.ConfigManagerAdapter
            excert: "public ConfigManagerAdapter(ActiveEditorListener editorListener, MCPClient mcpC…"
          - id: ConfigManagerAdapter.getManager
            excert: public ConfigManager getManager()
          - id: ConfigManagerAdapter.activateEditor
            excert: private void activateEditor(IEditorPart editor)
          - id: ConfigManagerAdapter.resolveKey
            excert: private static String resolveKey(IEditorPart editor)
          - id: ConfigManagerAdapter.ensureOwnConfig
            excert: protected void ensureOwnConfig()
          - id: ConfigManagerAdapter.saveConfig
            excert: public void saveConfig(IMemento memento)
          - id: ConfigManagerAdapter.loadConfig
            excert: public void loadConfig(IMemento memento)
```
/answer 04ff-a-9 allow
Control Request:
```yaml
id: 04ff-b-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
    ids:
    - ConfigManagerAdapter.ConfigManagerAdapter
    - ConfigManagerAdapter.activateEditor
    - ConfigManagerAdapter.ensureOwnConfig
    - ConfigManagerAdapter.loadConfig
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
    ids:
    - ModelParametersSection.create
    - ModelParametersSection.createModelCombo
    - ModelParametersSection.createProfileCombo
    - ModelParametersSection.createReasoningSection
    - ModelParametersSection.createCacheSection
  reason: Read editor logic and UI combos
```
/answer 04ff-b-0 allow
Control Request:
```yaml
id: 04ff-c-b
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
        nodes:
        - id: ConfigManagerAdapter.ConfigManagerAdapter
          type: ConstructorDeclaration
          code: |2
              public ConfigManagerAdapter(ActiveEditorListener editorListener, MCPClient mcpClient, IFileCache cache) {
                super(mcpClient, cache);
                cm = this;
                cfg = defaultCfg;
                editorListener.addTextEditorObserver(e -> activateEditor(e));
              }
        - id: ConfigManagerAdapter.activateEditor
          type: MethodDeclaration
          code: |2
              private void activateEditor(IEditorPart editor) {
                String key = resolveKey(editor);
                if (Objects.equals(activeEditorKey, key))
                  return;
                activeEditorKey = key;
                SessionConfig target = key == null ? defaultCfg : editorConfigs.getOrDefault(key, cfg);
                cm.applyConfig(target);
              }
        - id: ConfigManagerAdapter.ensureOwnConfig
          type: MethodDeclaration
          code: |2
              protected void ensureOwnConfig() {
                if (activeEditorKey == null || editorConfigs.containsKey(activeEditorKey))
                  return;
                cfg = new SessionConfig(cfg);
                editorConfigs.put(activeEditorKey, cfg);
              }
        - id: ConfigManagerAdapter.loadConfig
          type: MethodDeclaration
          code: "  public void loadConfig(IMemento memento) {\n    String savedKey = activeEditorKey;\n    activeEditorKey = null;\n    cfg = defaultCfg;\n    try {\n      MementoConverter.Snapshot snap = MementoConverter.loadConfig(memento);\n      if (snap == null)\n        return;\n      /*\n\t\t\t * Order matters: keys must be set (models resolved) before the model itself can\n\t\t\t * be restored, and the model must be set before the remaining,\n\t\t\t * capability-dependent settings are applied - otherwise setModel()'s own\n\t\t\t * defaulting logic would clobber them.\n\t\t\t */\n      if (snap.keys != null)\n        cm.setKey(snap.keys);\n      if (snap.modelProvider != null) {\n        Model resolved = Arrays.stream(cm.getEnabledModels()).filter(m -> m.matches(KeyPattern.valueOf(snap.modelProvider), snap.modelDisplayName)).findFirst().orElse(null);\n        if (resolved != null)\n          cm.setModel(resolved);\n        else\n          LOG.error(\"Unable to resolve persisted model \" + snap.modelProvider + \":\" + snap.modelDisplayName + \", no fallback applied\");\n      }\n      if (snap.temperature != null)\n        cm.setTemperature(snap.temperature);\n      if (snap.topP != null)\n        cm.setTopP(snap.topP);\n      if (snap.maxOutputTokens != null)\n        cm.setMaxOutputTokens(snap.maxOutputTokens);\n      if (snap.reasoningBudget != null)\n        cm.setReasoningBudget(snap.reasoningBudget);\n      if (snap.reasoning != null)\n        cm.setReasoning(Reasoning.valueOf(snap.reasoning));\n      if (snap.cacheMode != null)\n        cm.setCacheMode(CacheMode.valueOf(snap.cacheMode));\n      if (snap.profile != null)\n        cm.setProfile(AgentProfile.valueOf(snap.profile));\n      if (snap.systemPrompt != null)\n        cm.setSystemPrompt(snap.systemPrompt);\n      if (snap.freeText != null)\n        cm.setSystemFree(snap.freeText);\n      if (snap.outputMode != null)\n        cm.setOuputMode(OutputMode.valueOf(snap.outputMode));\n      if (snap.inputModes != null)\n        for (InputMode mode : InputMode.values()) if (mode.ordinal() < snap.inputModes.length)\n          cm.setInputMode(mode, snap.inputModes[mode.ordinal()]);\n      if (snap.enabledTools != null)\n        cm.setEnabledTools(snap.enabledTools);\n      if (snap.ruleSettings != null)\n        cm.setRuleSettings(RuleSettings.importAll(snap.ruleSettings));\n    } catch (Exception e) {\n      LOG.error(e.getMessage(), e);\n      LOG.info(\"Unable to restore config\", e);\n    } finally {\n      activeEditorKey = savedKey;\n      cfg = savedKey == null ? defaultCfg : editorConfigs.getOrDefault(savedKey, defaultCfg);\n    }\n  }\n"
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
        nodes:
        - id: ModelParametersSection.create
          type: MethodDeclaration
          code: "\tpublic void create(Composite body, ConfigManager cfg) {\n\t\tComposite top = new Composite(body, SWT.NONE);\n\t\ttop.setLayout(new GridLayout(2, false));\n\t\ttop.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\n\t\tcreateKeyField(top, cfg);\n\t\tcreateModelCombo(top, cfg);\n\t\tcreateProfileCombo(top, cfg);\n\n\t\tLabel maxTokenLabel = toolkit.createLabel(top, \"Max Token:\");\n\t\tmaxTokenLabel.setLayoutData(new GridData());\n\t\tText maxToken = toolkit.createText(top, \"\", SWT.BORDER);\n\t\tmaxToken.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tmaxToken.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setMaxOutputTokens(Long.parseLong(maxToken.getText()))));\n\t\tmaxToken.addMouseListener(MouseListener.mouseDownAdapter(m -> maxToken.setFocus()));\n\t\tcfg.addOutputTokenObs(ot -> maxToken.setText(ot + \"\"), true);\n\n\t\tLabel tempLabel = toolkit.createLabel(top, \"Temp:\");\n\t\ttempLabel.setLayoutData(new GridData());\n\t\tText temp = toolkit.createText(top, \"\", SWT.BORDER);\n\t\ttemp.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\ttemp.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setTemperature(Double.parseDouble(temp.getText()))));\n\t\ttemp.addMouseListener(MouseListener.mouseDownAdapter(m -> temp.setFocus()));\n\t\tcfg.addTemperatureObs(t -> temp.setText(t + \"\"), true);\n\n\t\tLabel topPLabel = toolkit.createLabel(top, \"TopP:\");\n\t\ttopPLabel.setLayoutData(new GridData());\n\t\tText topP = toolkit.createText(top, \"\", SWT.BORDER);\n\t\ttopP.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\ttopP.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setTopP(Double.parseDouble(topP.getText()))));\n\t\ttopP.addMouseListener(MouseListener.mouseDownAdapter(m -> topP.setFocus()));\n\t\tcfg.addTopPObs(tp -> topP.setText(tp + \"\"), true);\n\n\t\tcreateReasoningSection(top, body, cfg, tempLabel, temp, topPLabel, topP, maxTokenLabel, maxToken);\n\t\tcreateCacheSection(top, cfg);\n\t}\n"
        - id: ModelParametersSection.createModelCombo
          type: MethodDeclaration
          code: "\tprivate void createModelCombo(Composite top, ConfigManager cfg) {\n\t\ttoolkit.createLabel(top, \"Model:\");\n\t\tCombo modelSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tmodelSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tAtomicReference<Model[]> modelSelItems = new AtomicReference<>(new Model[0]);\n\t\tmodelSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tint idx = modelSel.getSelectionIndex();\n\t\t\tModel[] items = modelSelItems.get();\n\t\t\tif (idx >= 0 && idx < items.length)\n\t\t\t\tcfg.setModel(items[idx]);\n\t\t}));\n\t\tcfg.addEnabledModelsObs(k -> {\n\t\t\tmodelSelItems.set(k);\n\t\t\tmodelSel.setItems(\n\t\t\t\t\tArrays.stream(k).map((m) -> m.displayName).collect(Collectors.toList()).toArray(new String[0]));\n\t\t\tModel current = cfg.getModel();\n\t\t\tmodelSel.setText(current != null ? current.displayName : \"\");\n\t\t}, true);\n\t}\n"
        - id: ModelParametersSection.createProfileCombo
          type: MethodDeclaration
          code: "\tprivate void createProfileCombo(Composite top, ConfigManager cfg) {\n\t\tLabel profileLabel = toolkit.createLabel(top, \"Profile:\");\n\t\tprofileLabel.setLayoutData(new GridData());\n\t\tCombo profileSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tprofileSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tprofileSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg\n\t\t\t\t.setProfile(profileSel.getText().isBlank() ? null : AgentProfile.fromName(profileSel.getText()))));\n\t\tcfg.addEnabledProfilesObs(k -> {\n\t\t\tprofileSel\n\t\t\t\t\t.setItems(Arrays.stream(k).map((m) -> m.name).collect(Collectors.toList()).toArray(new String[0]));\n\t\t\tprofileSel.setText(k.length > 0 ? k[0].name : \"\");\n\t\t\ttoggleControl(profileLabel, profileSel, k.length > 0);\n\t\t\ttop.layout();\n\t\t}, true);\n\t\tcfg.addProfileObs(p -> {\n\t\t\tprofileSel.setText(p != null ? p.name : \"\");\n\t\t}, true);\n\t}\n"
        - id: ModelParametersSection.createReasoningSection
          type: MethodDeclaration
          code: "\tprivate void createReasoningSection(Composite top, Composite body, ConfigManager cfg, Label tempLabel, Text temp,\n\t\t\tLabel topPLabel, Text topP, Label maxTokenLabel, Text maxToken) {\n\t\tLabel reasoningLabel = toolkit.createLabel(top, \"Reasoning:\");\n\t\treasoningLabel.setLayoutData(new GridData());\n\t\tComposite secReason = new Composite(top, SWT.NONE);\n\t\tGridLayout secRLay = new GridLayout(2, false);\n\t\tsecRLay.marginHeight = secRLay.marginWidth = 0;\n\t\tsecReason.setLayout(secRLay);\n\t\tsecReason.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\n\t\tCombo reasSel = new Combo(secReason, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\treasSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\treasSel.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setReasoning(Reasoning.valueOf(reasSel.getText()))));\n\n\t\tText budget = toolkit.createText(secReason, \"\", SWT.BORDER);\n\t\tbudget.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tbudget.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setReasoningBudget(Integer.parseInt(budget.getText()))));\n\t\tbudget.addMouseListener(MouseListener.mouseDownAdapter(m -> budget.setFocus()));\n\t\tcfg.addBudgetObs(bg -> budget.setText(bg + \"\"), true);\n\n\t\tcfg.addModelObs(m -> {\n\t\t\ttoggleControl(tempLabel, temp, m.cap.isSupportTemperature(cfg.getReasoning()));\n\t\t\ttoggleControl(topPLabel, topP, m.cap.isSupportTopP(cfg.getReasoning()));\n\t\t\ttoggleControl(maxTokenLabel, maxToken, m.cap.isSupportMaxToken());\n\n\t\t\tString[] reasonings = cfg.getReasonings();\n\t\t\treasSel.setItems(reasonings);\n\t\t\treasSel.setText(cfg.getReasoning().name());\n\t\t\tboolean hasReasoning = reasonings.length > 0 && !(reasonings.length == 1\n\t\t\t\t\t&& (\"Default\".equals(reasonings[0]) || \"Disabled\".equals(reasonings[0])));\n\t\t\ttoggleControl(reasoningLabel, secReason, hasReasoning);\n\t\t\tbody.layout();\n\t\t}, true);\n\t\tcfg.addReasoningObs(r -> {\n\n\t\t\tboolean enabled = Reasoning.Budget.equals(r);\n\t\t\tbudget.setEnabled(enabled);\n\t\t\tbudget.setVisible(enabled);\n\t\t\t((GridData) budget.getLayoutData()).exclude = !enabled;\n\n\t\t\ttoggleControl(tempLabel, temp, cfg.getModel().cap.isSupportTemperature(r));\n\t\t\ttoggleControl(topPLabel, topP, cfg.getModel().cap.isSupportTopP(cfg.getReasoning()));\n\n\t\t\tsecReason.layout();\n\t\t\tbody.layout();\n\t\t}, true);\n\t}\n"
        - id: ModelParametersSection.createCacheSection
          type: MethodDeclaration
          code: "\tprivate void createCacheSection(Composite top, ConfigManager cfg) {\n\t\tLabel cacheLabel = toolkit.createLabel(top, \"Cache:\");\n\t\tcacheLabel.setLayoutData(new GridData());\n\t\tCombo cacheSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tcacheSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tcacheSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg.setCacheMode(\n\t\t\t\tcacheSel.getText().isBlank() ? CacheMode.Default : CacheMode.valueOf(cacheSel.getText()))));\n\t\tcfg.addModelObs(m -> {\n\t\t\tcacheSel.setItems(Arrays.stream(m.cap.getCacheMode()).map((c) -> c.name()).collect(Collectors.toList())\n\t\t\t\t\t.toArray(new String[0]));\n\t\t\tcacheSel.setText(cfg.getCacheMode() != null ? cfg.getCacheMode().name() : \"\");\n\t\t\ttoggleControl(cacheLabel, cacheSel, m.cap.getCacheMode().length > 0);\n\t\t}, true);\n\t\tcfg.addCacheObs(c -> {\n\t\t\tcacheSel.setText(c != null ? c.name() : \"\");\n\t\t}, true);\n\t}\n"
```
/answer 04ff-c-b allow
ReasoningToken: 183
Control Request:
```yaml
id: 04ff-d-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan
    pattern: class ConfigManager\b|void setReasoning\(|void setModel\(|void applyConfig\(
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    limit: 10
  reason: Locate ConfigManager
```
/answer 04ff-d-0 allow
Control Request:
```yaml
id: 04ff-e-d
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan
        matches:
        - path: Gadgetbridge/app/src/main/java/nodomain/freeyourgadget/gadgetbridge/impl/GBDevice.java
          matches:
          - "public void setModel(String model) {"
        - path: eclipse.platform.ui/examples/org.eclipse.ui.forms.examples/src/org/eclipse/ui/forms/examples/internal/rcp/NamedObject.java
          matches:
          - "public void setModel(SimpleModel model) {"
        - path: eclipse.platform.ui/bundles/org.eclipse.e4.ui.workbench.renderers.swt/src/org/eclipse/e4/ui/workbench/renderers/swt/HandledContributionItem.java
          matches:
          - "public void setModel(MItem item) {"
          - "public void setModel(MHandledItem item) {"
        - path: eclipse.platform.ui/bundles/org.eclipse.e4.ui.workbench.renderers.swt/src/org/eclipse/e4/ui/workbench/renderers/swt/AbstractContributionItem.java
          matches:
          - "public void setModel(MItem item) {"
        - path: eclipse.platform.ui/bundles/org.eclipse.e4.ui.workbench.renderers.swt/src/org/eclipse/e4/ui/workbench/renderers/swt/ToolControlContribution.java
          matches:
          - "public void setModel(MToolControl c) {"
        - path: eclipse.platform.ui/bundles/org.eclipse.ui.editors/src/org/eclipse/ui/internal/texteditor/AnnotationColumn.java
          matches:
          - "public void setModel(IAnnotationModel model) {"
        - path: eclipse.platform.ui/bundles/org.eclipse.ui.editors/src/org/eclipse/ui/internal/texteditor/LineNumberColumn.java
          matches:
          - "public void setModel(IAnnotationModel model) {"
        - path: eclipse.platform.ui/bundles/org.eclipse.jface.text/projection/org/eclipse/jface/text/source/projection/ProjectionRulerColumn.java
          matches:
          - "public void setModel(IAnnotationModel model) {"
        - path: eclipse.platform.ui/bundles/org.eclipse.jface.text/src/org/eclipse/jface/internal/text/source/DiffPainter.java
          matches:
          - "public void setModel(IAnnotationModel model) {"
        warnings:
        - "Limit of 10 matches reached (total 28); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 04ff-e-d allow
Control Request:
```yaml
id: 04ff-f-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class ConfigManager\b
    include:
    - '*.java'
    limit: 5
  reason: Locate ConfigManager narrowly
```
/answer 04ff-f-0 allow
Control Request:
```yaml
id: 04ff-10-f
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: connector/src/xy/ai/conscious/core/ConfigManager.java
          matches:
          - "public class ConfigManager {"
```
/answer 04ff-10-f allow
Control Request:
```yaml
id: 04ff-11-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
    regexp: void (setModel|setReasoning|setCacheMode|setProfile|applyConfig|setSystemPrompt)\(|Obs\(.*(Reasoning|Cache|Profile)
  reason: Read setters and observers
```
/answer 04ff-11-0 allow
Control Request:
```yaml
id: 04ff-12-11
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
        nodes:
        - id: ConfigManager.applyConfig
          code: |2
              public void applyConfig(SessionConfig target) {
                if (target == cfg)
                  return;
                cfg = target;
                systemPromptObs.forEach(c -> c.accept(cfg));
                systemFreeObs.forEach(c -> c.accept(cfg.freeText));
                inputObs.forEach(c -> c.accept(cfg.inputModes));
                if (cfg.keys != null)
                  for (String key : cfg.keys.split(",")) keyObs.forEach(c -> c.accept(key));
                modelObs.forEach(c -> c.accept(cfg.model));
                profileObs.forEach(c -> c.accept(cfg.profile));
                outTokenObs.forEach(c -> c.accept(cfg.maxOutputTokens));
                budgetObs.forEach(c -> c.accept(cfg.reasoningBudget));
                reasonObs.forEach(c -> c.accept(cfg.reasoning));
                cacheObs.forEach(c -> c.accept(cfg.cacheMode));
                temperatureObs.forEach(c -> c.accept(cfg.temperature));
                topPObs.forEach(c -> c.accept(cfg.topP));
                outputModeObs.forEach(c -> c.accept(cfg.ouputMode));
                ruleSettingsObs.forEach(c -> c.accept(getRuleSettings()));
                updateEnabledModels(cfg.keys != null ? cfg.keys.split(",") : new String[0]);
              }
        - id: ConfigManager.setModel
          code: |2
              public void setModel(Model model) {
                if (Objects.equals(cfg.getModel(), model))
                  return;
                ensureOwnConfig();
                cfg.setModel(model);
                if (Arrays.asList(getCapabilities().getReasonings()).indexOf(cfg.reasoning) == -1)
                  setReasoning(getCapabilities().getReasonings()[0]);
                setMaxOutputTokens((long) getCapabilities().alignOutpuTokens(Integer.MAX_VALUE));
                setReasoningBudget(getCapabilities().alignBudget(Integer.MAX_VALUE));
                setEnabledProfiles(getCapabilities().getAgentProfiles());
                var profiles = getCapabilities().getAgentProfiles();
                setProfile(profiles.length > 0 ? profiles[0] : null);
                CacheMode[] cacheModes = getCapabilities().getCacheMode();
                if (!Arrays.asList(cacheModes).contains(cfg.cacheMode))
                  setCacheMode(cacheModes.length > 0 ? cacheModes[0] : CacheMode.Default);
                modelObs.forEach(c -> c.accept(model));
              }
        - id: ConfigManager.setReasoning
          code: |2
              public void setReasoning(Reasoning reasoning) {
                if (Objects.equals(cfg.getReasoning(), reasoning))
                  return;
                ensureOwnConfig();
                cfg.setReasoning(reasoning);
                reasonObs.forEach(c -> c.accept(cfg.reasoning));
              }
        - id: ConfigManager.setCacheMode
          code: |2
              public void setCacheMode(CacheMode cacheMode) {
                if (Objects.equals(cfg.getCacheMode(), cacheMode))
                  return;
                ensureOwnConfig();
                cfg.setCacheMode(cacheMode);
                cacheObs.forEach(c -> c.accept(cfg.cacheMode));
              }
        - id: ConfigManager.setProfile
          code: |2
              public void setProfile(AgentProfile profile) {
                if (Objects.equals(cfg.getProfile(), profile))
                  return;
                ensureOwnConfig();
                cfg.setProfile(profile);
                profileObs.forEach(c -> c.accept(cfg.profile));
              }
        - id: ConfigManager.setSystemPrompt
          code: |2
              public void setSystemPrompt(String[] systemPrompt) {
                if (Arrays.equals(this.getSystemPrompt(), systemPrompt))
                  return;
                ensureOwnConfig();
                cfg.setSystemPrompt(systemPrompt);
                systemPromptObs.forEach(c -> c.accept(cfg));
                inputModeObs.forEach(c -> c.accept(InputMode.SystemPrompt));
              }
        - id: ConfigManager.addProfileObs
          code: |2
              public void addProfileObs(Consumer<AgentProfile> obs, boolean initialize) {
                profileObs.add(obs);
                if (initialize)
                  obs.accept(cfg.profile);
              }
        - id: ConfigManager.addReasoningObs
          code: |2
              public void addReasoningObs(Consumer<Reasoning> obs, boolean initialize) {
                reasonObs.add(obs);
                if (initialize)
                  obs.accept(cfg.reasoning);
              }
        - id: ConfigManager.addCacheObs
          code: |2
              public void addCacheObs(Consumer<CacheMode> obs, boolean initialize) {
                cacheObs.add(obs);
                if (initialize)
                  obs.accept(cfg.cacheMode);
              }
        - id: ConfigManager.addEnabledProfilesObs
          code: |2
              public void addEnabledProfilesObs(Consumer<AgentProfile[]> obs, boolean initialize) {
                enabledProfilesObs.add(obs);
                if (initialize)
                  obs.accept(enabledProfiles);
              }
```
/answer 04ff-12-11 allow
ReasoningToken: 376
Control Request:
```yaml
id: 04ff-13-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
    id: ModelParametersSection.createReasoningSection
    old_text: "\t\tcfg.addReasoningObs(r -> {\n\n"
    new_text: "\t\tcfg.addReasoningObs(r -> {\n\t\t\tif (r != null && !r.name().equals(reasSel.getText()))\n\t\t\t\treasSel.setText(r.name());\n\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
    id: ModelParametersSection.createModelCombo
    old_text: "\t\t}, true);\n\t}"
    new_text: "\t\t}, true);\n\t\tcfg.addModelObs(m -> {\n\t\t\tif (m != null && !m.displayName.equals(modelSel.getText()))\n\t\t\t\tmodelSel.setText(m.displayName);\n\t\t}, false);\n\t}"
  reason: Sync model and reasoning combos with programmatic config changes
```
/answer 04ff-13-0 allow
Control Request:
```yaml
id: 04ff-14-13
result:
  structuredContent:
    edit_block:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
        id: ModelParametersSection.createReasoningSection
        new_id: ModelParametersSection.createReasoningSection
      errors:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
        id: ModelParametersSection.createModelCombo
        error: Text not found in node; its content appears to have changed.
        reason: content_changed
        guess: |2-
              }, true);
            }
```
/answer 04ff-14-13 allow
Control Request:
```yaml
id: 04ff-15-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
    id: ModelParametersSection.createModelCombo
    old_text: "modelSel.setText(current != null ? current.displayName : \"\");\n\t\t}, true);"
    new_text: "modelSel.setText(current != null ? current.displayName : \"\");\n\t\t}, true);\n\t\tcfg.addModelObs(m -> {\n\t\t\tif (m != null && !m.displayName.equals(modelSel.getText()))\n\t\t\t\tmodelSel.setText(m.displayName);\n\t\t}, false);"
  reason: Add model observer to sync model combo
```
/answer 04ff-15-0 allow
Result Stats: id=a096b7c1-7308-4969-8726-4ba8ead1eed5, total: 27077, in: 24, out: 3940, reason: 0, read: 148696, write: 23113, start: 1795423025191, end: 1795423030845, time: 5588, total: 52784, io: 0, control: 66, sent: 0, recv: 149
Ich habe die Ursache für die fehlenden UI-Updates gefunden und behoben. Kompiliert und getestet habe ich nichts.

**Ursache:** `PresetsTab.applySettings` ruft `cfg.setModel/setReasoning/…` korrekt auf. Das Modell landet auch im Editor-Config, weil `ensureOwnConfig()` greift. Die Widgets in `ModelParametersSection` haben diese Änderungen aber nicht übernommen:
- **Model-Combo:** Sie hatte keinen Model-Observer. Sie wurde nur aktualisiert, wenn sich die Liste der aktivierten Modelle änderte. Nach einem Preset zeigte sie deshalb weiter das alte Modell.
- **Reasoning-Combo:** Der Reasoning-Observer hat nur das Budget-Feld und das Layout angepasst, aber nicht den Combo-Text. Der Combo wurde nur bei einem Modellwechsel neu befüllt.

Dass das Preset „verzögert" wirkt, passt dazu. Der Wert wurde sofort gesetzt, aber erst bei einem späteren Refresh (zum Beispiel beim Editorwechsel über `applyConfig`) im UI sichtbar.

**Änderungen in `ModelParametersSection.java`:**
- **`createModelCombo`:** Ich habe einen `addModelObs` ergänzt, der den Combo-Text auf `m.displayName` setzt.
- **`createReasoningSection`:** Der Reasoning-Observer setzt jetzt auch `reasSel.setText(r.name())`.

Profile und Cache hatten bereits eigene Observer und waren nicht betroffen.

Bitte teste, ob nach dem Laden eines Presets Modell, Reasoning, Cache und Profile sofort in den Combos stehen. Falls danach noch etwas verzögert wirkt, liegt es an einer anderen Stelle, etwa an `ensureOwnConfig()` oder am Editorwechsel.
