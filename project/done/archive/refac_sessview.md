`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java` ist zu lang geworden. Überlege eine Aufteilung für eine Extraktion von Aspekten unter Berücksichtigung der geteilten Felder.
SystemInit:  id=7524f0d0-1820-430b-adf1-781ec86a8d2c, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 21e8-1
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
  reason: "Überblick über Klassenstruktur, Felder und Methoden zur Analyse einer möglichen Aufteilung."
```
/answer 21e8-1 allow
Control Request:
```yaml
id: 21e8-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
      nodes:
      - id: 8lg35S|YsNcr0
        signature: package xy.ai.workbench.view;
      - id: 2C6knn|WTIi3p
        signature: import org.eclipse.swt.layout.GridData;
      - id: 3JDbMD|JtZOqz
        signature: import org.eclipse.swt.widgets.TableColumn;
      - id: GjvraN|AaS6yV
        signature: import xy.ai.workbench.CacheMode;
      - id: AISessionView
        signature: "public class AISessionView extends ViewPart {"
        children:
        - id: AISessionView.uxef2K|R6IMVC
          signature: /**
        - id: AISessionView.JXDY3E|2Q8Bc1
          signature: private final java.util.List<IFile> presetFiles = new ArrayList<>();
        - id: AISessionView.saveState
          signature: "public void saveState(IMemento memento) {"
        - id: AISessionView.init
          signature: "public void init(IViewSite site, IMemento memento) throws PartInitException {"
        - id: AISessionView.createPartControl
          signature: "public void createPartControl(Composite parent) {"
        - id: AISessionView.createTopParametersSection
          signature: "private void createTopParametersSection(Composite body, ConfigManager cfg) {"
        - id: AISessionView.createKeyField
          signature: "private void createKeyField(Composite top, ConfigManager cfg) {"
        - id: AISessionView.OW0xbN|Hxfc0y
          signature: "private final AtomicReference<Model[]> modelSelItems = new AtomicReference<>(ne…"
        - id: AISessionView.createModelCombo
          signature: "private void createModelCombo(Composite top, ConfigManager cfg) {"
        - id: AISessionView.createProfileCombo
          signature: "private void createProfileCombo(Composite top, ConfigManager cfg) {"
        - id: AISessionView.createReasoningSection
          signature: "private void createReasoningSection(Composite top, Composite body, ConfigManage…"
        - id: AISessionView.createCacheSection
          signature: "private void createCacheSection(Composite top, ConfigManager cfg) {"
        - id: AISessionView.createInstructionSection
          signature: "private void createInstructionSection(Composite body, ConfigManager cfg, Prompt…"
        - id: AISessionView.createInstructionSelectTab
          signature: "private void createInstructionSelectTab(TabFolder instr, ConfigManager cfg) {"
        - id: AISessionView.createInstructionEditTab
          signature: "private void createInstructionEditTab(TabFolder instr, ConfigManager cfg) {"
        - id: AISessionView.createToolsTab
          signature: "private void createToolsTab(TabFolder instr, ConfigManager cfg) {"
        - id: AISessionView.createPresetsTab
          signature: "private void createPresetsTab(TabFolder instr, ConfigManager cfg) {"
        - id: AISessionView.createFreeTextArea
          signature: "private void createFreeTextArea(SashForm sash, ConfigManager cfg) {"
        - id: AISessionView.createInputsTable
          signature: "private void createInputsTable(Composite middle, ConfigManager cfg, PromptHandl…"
        - id: AISessionView.createOutputModeSection
          signature: "private void createOutputModeSection(Composite body, ConfigManager cfg) {"
        - id: AISessionView.createActionButtonsSection
          signature: "private void createActionButtonsSection(Composite body, ConfigManager cfg, Prom…"
        - id: AISessionView.createUsageLogSection
          signature: "private void createUsageLogSection(Composite body, PromptHandler prompt) {"
        - id: AISessionView.refreshPresetList
          signature: "private void refreshPresetList() {"
        - id: AISessionView.updatePromptLines
          signature: "private String[] updatePromptLines(String[] cur) {"
        - id: AISessionView.isTemperatureEnabled
          signature: "private boolean isTemperatureEnabled(Model m, Reasoning reasoning) {"
        - id: AISessionView.toggleControl
          signature: "private void toggleControl(Label label, Control ctrl, boolean enabled) {"
        - id: AISessionView.updateInstructionList
          signature: "private void updateInstructionList(String[] systemPrompt) {"
        - id: AISessionView.updateEditList
          signature: "private void updateEditList(String[] systemPrompt) {"
        - id: AISessionView.setFocus
          signature: "public void setFocus() {"
```
/answer 21e8-2 allow
ReasoningToken: 13
Control Request:
```yaml
id: 21e8-3
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
  reason: "Alle Felddeklarationen der Klasse identifizieren, um geteilten State für die Extraktion zu bestimmen."
  node_type: FieldDeclaration|VariableDeclarationFragment
```
/answer 21e8-3 allow
Control Request:
```yaml
id: 21e8-4
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
      nodes: []
```
/answer 21e8-4 allow
Control Request:
```yaml
id: 21e8-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
    ids:
    - AISessionView
  reason: "Vollständigen Quellcode der Klasse lesen, um Felder, Methodenzusammenhänge und geteilten State zu analysieren."
```
/answer 21e8-5 allow
Control Request:
```yaml
id: 21e8-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
      nodes:
      - id: AISessionView
        type: class_declaration
        signature: "public class AISessionView extends ViewPart {"
        children:
        - id: AISessionView.uxef2K|R6IMVC
          type: statements
          code: "/**\n\t * The ID of the view as specified by the extension.\n\t */\n\tpublic static final String ID = \"xy.ai.workbench.views.AISessionView\";\n\n\tpublic static AISessionView currentInstance;\n\n\t@Inject\n\tIWorkbench workbench;\n\n\tprivate FormToolkit toolkit;\n\tprivate ScrolledForm form;\n\n\tprivate Text usageLog;\n\tprivate List instructionList;\n\tprivate List toolsList;\n\tprivate List presetList;\n\tprivate Text instructionEdit, instructionFree;\n\tprivate boolean isUpdating = false;\n\tprivate MultiSelectListener toolsMultiSelect;"
        - id: AISessionView.JXDY3E|2Q8Bc1
          type: statements
          code: "private final java.util.List<IFile> presetFiles = new ArrayList<>();\n\n\tpublic Display display;\n\n\tjava.util.List<String> instructionSelection = new ArrayList<String>();"
        - id: AISessionView.saveState
          type: method_declaration
          code: "@Override\n\tpublic void saveState(IMemento memento) {\n\t\tsuper.saveState(memento);\n\t\tActivator.getDefault().cfg.saveConfig(memento);\n\t}"
        - id: AISessionView.init
          type: method_declaration
          code: "@Override\n\tpublic void init(IViewSite site, IMemento memento) throws PartInitException {\n\t\tsuper.init(site, memento);\n\t\tActivator.getDefault().cfg.loadConfig(memento);\n\t}"
        - id: AISessionView.createPartControl
          type: method_declaration
          code: "@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tcurrentInstance = this;\n\t\tdisplay = parent.getDisplay();\n\t\ttoolkit = new FormToolkit(parent.getDisplay());\n\t\tform = toolkit.createScrolledForm(parent);\n\t\tConfigManager cfg = Activator.getDefault().cfg;\n\t\tPromptHandler promptHandler = Activator.getDefault().session.getPromptHandler();\n\n\t\tComposite body = form.getBody();\n\t\tbody.setLayout(new GridLayout());\n\n\t\tcreateTopParametersSection(body, cfg);\n\t\tcreateInstructionSection(body, cfg, promptHandler);\n\t\tcreateOutputModeSection(body, cfg);\n\t\tcreateActionButtonsSection(body, cfg, promptHandler);\n\t\tcreateUsageLogSection(body, promptHandler);\n\n\t\tpromptHandler.initializeInputs();\n\n\t\tform.reflow(true);\n\t}"
        - id: AISessionView.createTopParametersSection
          type: method_declaration
          code: "private void createTopParametersSection(Composite body, ConfigManager cfg) {\n\t\tComposite top = new Composite(body, SWT.NONE);\n\t\ttop.setLayout(new GridLayout(2, false));\n\t\ttop.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\n\t\tcreateKeyField(top, cfg);\n\t\tcreateModelCombo(top, cfg);\n\t\tcreateProfileCombo(top, cfg);\n\n\t\tLabel maxTokenLabel = toolkit.createLabel(top, \"Max Token:\");\n\t\tmaxTokenLabel.setLayoutData(new GridData());\n\t\tText maxToken = toolkit.createText(top, \"\", SWT.BORDER);\n\t\tmaxToken.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tmaxToken.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setMaxOutputTokens(Long.parseLong(maxToken.getText()))));\n\t\tmaxToken.addMouseListener(MouseListener.mouseDownAdapter(m -> maxToken.setFocus()));\n\t\tcfg.addOutputTokenObs(ot -> maxToken.setText(ot + \"\"), true);\n\n\t\tLabel tempLabel = toolkit.createLabel(top, \"Temp:\");\n\t\ttempLabel.setLayoutData(new GridData());\n\t\tText temp = toolkit.createText(top, \"\", SWT.BORDER);\n\t\ttemp.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\ttemp.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setTemperature(Double.parseDouble(temp.getText()))));\n\t\ttemp.addMouseListener(MouseListener.mouseDownAdapter(m -> temp.setFocus()));\n\t\tcfg.addTemperatureObs(t -> temp.setText(t + \"\"), true);\n\n\t\tLabel topPLabel = toolkit.createLabel(top, \"TopP:\");\n\t\ttopPLabel.setLayoutData(new GridData());\n\t\tText topP = toolkit.createText(top, \"\", SWT.BORDER);\n\t\ttopP.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\ttopP.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setTopP(Double.parseDouble(topP.getText()))));\n\t\ttopP.addMouseListener(MouseListener.mouseDownAdapter(m -> topP.setFocus()));\n\t\tcfg.addTopPObs(tp -> topP.setText(tp + \"\"), true);\n\n\t\tcreateReasoningSection(top, body, cfg, tempLabel, temp, topPLabel, topP, maxTokenLabel, maxToken);\n\t\tcreateCacheSection(top, cfg);\n\t}"
        - id: AISessionView.createKeyField
          type: method_declaration
          code: "private void createKeyField(Composite top, ConfigManager cfg) {\n\t\ttoolkit.createLabel(top, \"Key:\");\n\t\tText keyInput = toolkit.createText(top, \"\", SWT.BORDER | SWT.PASSWORD);\n\t\tGridData kilay = new GridData(GridData.FILL_HORIZONTAL);\n\t\tkilay.widthHint = 10;\n\t\tkeyInput.setLayoutData(kilay);\n\t\tkeyInput.addModifyListener(e -> cfg.setKey(keyInput.getText()));\n\t\tkeyInput.setText(cfg.getKeys() + \"\");\n\t\tkeyInput.addMouseListener(MouseListener.mouseDownAdapter(m -> keyInput.setFocus()));\n\t}"
        - id: AISessionView.OW0xbN|Hxfc0y
          type: statements
          code: "private final AtomicReference<Model[]> modelSelItems = new AtomicReference<>(new Model[0]);"
        - id: AISessionView.createModelCombo
          type: method_declaration
          code: "private void createModelCombo(Composite top, ConfigManager cfg) {\n\t\ttoolkit.createLabel(top, \"Model:\");\n\t\tCombo modelSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tmodelSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tmodelSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tint idx = modelSel.getSelectionIndex();\n\t\t\tModel[] items = modelSelItems.get();\n\t\t\tif (idx >= 0 && idx < items.length)\n\t\t\t\tcfg.setModel(items[idx]);\n\t\t}));\n\t\tcfg.addEnabledModelsObs(k -> {\n\t\t\tmodelSelItems.set(k);\n\t\t\tmodelSel.setItems(\n\t\t\t\t\tArrays.stream(k).map((m) -> m.displayName).collect(Collectors.toList()).toArray(new String[0]));\n\t\t\tModel current = cfg.getModel();\n\t\t\tmodelSel.setText(current != null ? current.displayName : \"\");\n\t\t}, true);\n\t}"
        - id: AISessionView.createProfileCombo
          type: method_declaration
          code: "private void createProfileCombo(Composite top, ConfigManager cfg) {\n\t\tLabel profileLabel = toolkit.createLabel(top, \"Profile:\");\n\t\tprofileLabel.setLayoutData(new GridData());\n\t\tCombo profileSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tprofileSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tprofileSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg\n\t\t\t\t.setProfile(profileSel.getText().isBlank() ? null : AgentProfile.fromName(profileSel.getText()))));\n\t\tcfg.addEnabledProfilesObs(k -> {\n\t\t\tprofileSel\n\t\t\t\t\t.setItems(Arrays.stream(k).map((m) -> m.name).collect(Collectors.toList()).toArray(new String[0]));\n\t\t\tprofileSel.setText(k.length > 0 ? k[0].name : \"\");\n\t\t\ttoggleControl(profileLabel, profileSel, k.length > 0);\n\t\t\ttop.layout();\n\t\t}, true);\n\t\tcfg.addProfileObs(p -> {\n\t\t\tprofileSel.setText(p != null ? p.name : \"\");\n\t\t}, true);\n\t}"
        - id: AISessionView.createReasoningSection
          type: method_declaration
          code: "private void createReasoningSection(Composite top, Composite body, ConfigManager cfg, Label tempLabel, Text temp,\n\t\t\tLabel topPLabel, Text topP, Label maxTokenLabel, Text maxToken) {\n\t\tLabel reasoningLabel = toolkit.createLabel(top, \"Reasoning:\");\n\t\treasoningLabel.setLayoutData(new GridData());\n\t\tComposite secReason = new Composite(top, SWT.NONE);\n\t\tGridLayout secRLay = new GridLayout(2, false);\n\t\tsecRLay.marginHeight = secRLay.marginWidth = 0;\n\t\tsecReason.setLayout(secRLay);\n\t\tsecReason.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\n\t\tCombo reasSel = new Combo(secReason, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\treasSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\treasSel.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setReasoning(Reasoning.valueOf(reasSel.getText()))));\n\n\t\tText budget = toolkit.createText(secReason, \"\", SWT.BORDER);\n\t\tbudget.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tbudget.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setReasoningBudget(Integer.parseInt(budget.getText()))));\n\t\tbudget.addMouseListener(MouseListener.mouseDownAdapter(m -> budget.setFocus()));\n\t\tcfg.addBudgetObs(bg -> budget.setText(bg + \"\"), true);\n\n\t\tcfg.addModelObs(m -> {\n\t\t\ttoggleControl(tempLabel, temp, isTemperatureEnabled(m, cfg.getReasoning()));\n\t\t\ttoggleControl(topPLabel, topP, m.cap.isSupportTopP());\n\t\t\ttoggleControl(maxTokenLabel, maxToken, m.cap.isSupportMaxToken());\n\n\t\t\tString[] reasonings = cfg.getReasonings();\n\t\t\treasSel.setItems(reasonings);\n\t\t\treasSel.setText(cfg.getReasoning().name());\n\t\t\tboolean hasReasoning = reasonings.length > 0\n\t\t\t\t\t&& !(reasonings.length == 1 && (\"Default\".equals(reasonings[0]) || \"Disabled\".equals(reasonings[0])));\n\t\t\ttoggleControl(reasoningLabel, secReason, hasReasoning);\n\t\t\tbody.layout();\n\t\t}, true);\n\t\tcfg.addReasoningObs(r -> {\n\n\t\t\tboolean enabled = Reasoning.Budget.equals(r);\n\t\t\tbudget.setEnabled(enabled);\n\t\t\tbudget.setVisible(enabled);\n\t\t\t((GridData) budget.getLayoutData()).exclude = !enabled;\n\n\t\t\ttoggleControl(tempLabel, temp, isTemperatureEnabled(cfg.getModel(), r));\n\n\t\t\tsecReason.layout();\n\t\t\tbody.layout();\n\t\t}, true);\n\t}"
        - id: AISessionView.createCacheSection
          type: method_declaration
          code: "private void createCacheSection(Composite top, ConfigManager cfg) {\n\t\tLabel cacheLabel = toolkit.createLabel(top, \"Cache:\");\n\t\tcacheLabel.setLayoutData(new GridData());\n\t\tCombo cacheSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tcacheSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tcacheSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg.setCacheMode(\n\t\t\t\tcacheSel.getText().isBlank() ? CacheMode.Default : CacheMode.valueOf(cacheSel.getText()))));\n\t\tcfg.addModelObs(m -> {\n\t\t\tcacheSel.setItems(Arrays.stream(m.cap.getCacheMode()).map((c) -> c.name()).collect(Collectors.toList())\n\t\t\t\t\t.toArray(new String[0]));\n\t\t\tcacheSel.setText(cfg.getCacheMode() != null ? cfg.getCacheMode().name() : \"\");\n\t\t\ttoggleControl(cacheLabel, cacheSel, m.cap.getCacheMode().length > 0);\n\t\t}, true);\n\t\tcfg.addCacheObs(c -> {\n\t\t\tcacheSel.setText(c != null ? c.name() : \"\");\n\t\t}, true);\n\t}"
        - id: AISessionView.createInstructionSection
          type: method_declaration
          code: "private void createInstructionSection(Composite body, ConfigManager cfg, PromptHandler prompt) {\n\t\tComposite middle = new Composite(body, SWT.NONE);\n\t\tmiddle.setLayout(new GridLayout(1, false));\n\t\tGridData ldat2 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat2.heightHint = 100;\n\t\tmiddle.setLayoutData(ldat2);\n\n\t\ttoolkit.createLabel(middle, \"System Prompt:\");\n\n\n\t\tComposite contComp = new Composite(middle, SWT.NONE);\n\t\tcontComp.setLayout(new GridLayout(1, false));\n\t\tGridData scl = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tscl.heightHint = 100;\n\t\tscl.widthHint = 1;\n\t\tcontComp.setLayoutData(scl);\n\n\n\t\tTabFolder instr = new TabFolder(contComp, SWT.NONE);\n\t\tGridData ldat1 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat1.heightHint = 100;\n\t\tinstr.setLayoutData(ldat1);\n\n\t\tcreateInstructionSelectTab(instr, cfg);\n\t\tcreateInstructionEditTab(instr, cfg);\n\t\tcreateToolsTab(instr, cfg);\n\t\tcreatePresetsTab(instr, cfg);\n\n\t\tcreateInputsTable(middle, cfg, prompt);\n\t}"
        - id: AISessionView.createInstructionSelectTab
          type: method_declaration
          code: "private void createInstructionSelectTab(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem instrSel = new TabItem(instr, SWT.NONE);\n\t\tinstrSel.setText(\"Select\");\n\n\t\tSashForm sash = new SashForm(instr, SWT.VERTICAL);\n\t\tsash.setLayout(new GridLayout(1, false));\n\t\tGridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tscl2.heightHint = 100;\n\t\tscl2.widthHint = 1;\n\t\tsash.setLayoutData(scl2);\n\n\t\tComposite comp = new Composite(sash, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\tinstrSel.setControl(sash);\n\t\tinstructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\t\tcfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\tinstructionList.setLayoutData(gridData);\n\t\tinstructionList.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionList.setFocus()));\n\n\t\tinstructionList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tif (isUpdating)\n\t\t\t\treturn;\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tinstructionSelection = new ArrayList<>(Arrays.asList(instructionList.getSelection()));\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t}));\n\t\tinstructionList.addListener(SWT.MouseDown, event -> {\n\t\t\tif (isUpdating)\n\t\t\t\treturn;\n\t\t\tString[] clickedIndex = instructionList.getSelection();\n\t\t\tLOG.info(Arrays.toString(clickedIndex));\n\t\t\tif (clickedIndex.length != 1)\n\t\t\t\treturn;\n\n\t\t\tif (!instructionSelection.remove(clickedIndex[0]))\n\t\t\t\tinstructionSelection.add(clickedIndex[0]);\n\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t});\n\n\t\tcreateFreeTextArea(sash, cfg);\n\t}"
        - id: AISessionView.createInstructionEditTab
          type: method_declaration
          code: "private void createInstructionEditTab(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem instrEdit = new TabItem(instr, SWT.NONE);\n\t\tinstrEdit.setText(\"Edit\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tinstrEdit.setControl(comp);\n\t\tinstructionEdit = toolkit.createText(comp, \"\", SWT.WRAP | SWT.V_SCROLL);\n\t\tcfg.addSystemPromptObs(p -> {\n\t\t\tif (!instructionEdit.isFocusControl())\n\t\t\t\tupdateEditList(p.systemPrompt);\n\t\t}, true);\n\t\tinstructionEdit.addModifyListener(e -> {\n\t\t\tif (isUpdating)\n\t\t\t\treturn;\n\t\t\tcfg.setSystemPrompt(instructionEdit.getText().split(\"\\n\"));\n\t\t});\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\tinstructionEdit.setLayoutData(gridData);\n\t\tinstructionEdit.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionEdit.setFocus()));\n\t}"
        - id: AISessionView.createToolsTab
          type: method_declaration
          code: "private void createToolsTab(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem toolSel = new TabItem(instr, SWT.NONE);\n\t\ttoolSel.setText(\"Tools\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\ttoolSel.setControl(comp);\n\t\ttoolsList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\ttoolsList.setLayoutData(gridData);\n\t\ttoolsList.addMouseListener(MouseListener.mouseDownAdapter(m -> toolsList.setFocus()));\n\t\ttoolsList.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setEnabledTools(toolsList.getSelection())));\n\t\tcfg.addModelObs(m -> {\n\t\t\tif (m != null)\n\t\t\t\ttoolsList.setItems(m.cap.getTools());\n\t\t}, true);\n\t\ttoolsMultiSelect = new MultiSelectListener(toolsList);\n\t}"
        - id: AISessionView.createPresetsTab
          type: method_declaration
          code: "private void createPresetsTab(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem presEdit = new TabItem(instr, SWT.NONE);\n\t\tpresEdit.setText(\"Presets\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tGridLayout compLay = new GridLayout(1, false);\n\t\tcomp.setLayout(compLay);\n\t\tGridData compDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tcomp.setLayoutData(compDat);\n\t\tpresEdit.setControl(comp);\n\n\t\tComposite buttons = new Composite(comp, SWT.NONE);\n\t\tbuttons.setLayout(new GridLayout(2, false));\n\n\t\tButton writeButton = new Button(buttons, SWT.PUSH);\n\t\twriteButton.setText(\"Save\");\n\t\twriteButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tPresetHandler.writePreset(cfg.getSystemPrompt(), toolsList.getSelection(), cfg.getOuputMode(),\n\t\t\t\t\t\tgetSite().getShell());\n\t\t\t\trefreshPresetList();\n\t\t\t}\n\t\t});\n\n\t\tButton resetButton = new Button(buttons, SWT.PUSH);\n\t\tresetButton.setText(\"Reset\");\n\t\tresetButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tString[] cur = cfg.getSystemPrompt();\n\t\t\t\tString[] upd = new String[cur.length];\n\t\t\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\t\t\tString line = cur[i];\n\t\t\t\t\tupd[i] = line.startsWith(\"#\") ? line : \"#\" + line;\n\t\t\t\t}\n\t\t\t\tinstructionSelection.clear();\n\t\t\t\tcfg.setSystemPrompt(upd);\n\n\t\t\t\ttoolsMultiSelect.clear();\n\t\t\t\tcfg.setEnabledTools(new String[0]);\n\t\t\t}\n\t\t});\n\n\t\tpresetList = new List(comp, SWT.SINGLE | SWT.V_SCROLL | SWT.BORDER);\n\t\tGridData presetListDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tpresetListDat.heightHint = 60;\n\t\tpresetList.setLayoutData(presetListDat);\n\t\tpresetList.addMouseListener(MouseListener.mouseDownAdapter(m -> presetList.setFocus()));\n\t\tpresetList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tint idx = presetList.getSelectionIndex();\n\t\t\tif (idx < 0 || idx >= presetFiles.size())\n\t\t\t\treturn;\n\t\t\tPresetHandler.Preset preset = PresetHandler.loadPreset(presetFiles.get(idx));\n\t\t\tcfg.setSystemPrompt(preset.body);\n\t\t\tif (preset.tools != null) {\n\t\t\t\ttoolsMultiSelect.setSelection(preset.tools);\n\t\t\t\tcfg.setEnabledTools(preset.tools);\n\t\t\t}\n\t\t\tif (preset.outputMode != null)\n\t\t\t\tcfg.setOuputMode(preset.outputMode);\n\t\t}));\n\t\trefreshPresetList();\n\t}"
        - id: AISessionView.createFreeTextArea
          type: method_declaration
          code: "private void createFreeTextArea(SashForm sash, ConfigManager cfg) {\n\t\tinstructionFree = toolkit.createText(sash, \"\", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);\n\t\tcfg.addSystemFreeObs(p -> {\n\t\t\tif (!instructionFree.isFocusControl())\n\t\t\t\ttry {\n\t\t\t\t\tisUpdating = true;\n\t\t\t\t\tinstructionFree.setText(p != null ? p : \"\");\n\t\t\t\t\tform.reflow(true);\n\t\t\t\t} finally {\n\t\t\t\t\tisUpdating = false;\n\t\t\t\t}\n\t\t}, true);\n\t\tinstructionFree\n\t\t\t\t.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setSystemFree(instructionFree.getText())));\n\t\tGridData gridData = new GridData(GridData.FILL_HORIZONTAL);\n\t\tgridData.widthHint = 1;\n\t\tinstructionFree.setLayoutData(gridData);\n\t\tinstructionFree.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionFree.setFocus()));\n\t\tsash.setWeights(3, 1);\n\t}"
        - id: AISessionView.createInputsTable
          type: method_declaration
          code: "private void createInputsTable(Composite middle, ConfigManager cfg, PromptHandler prompt) {\n\t\tTable table = new Table(middle, SWT.CHECK | SWT.BORDER | SWT.V_SCROLL);\n\t\ttable.setHeaderVisible(true);\n\t\ttable.setLinesVisible(true);\n\n\t\tTableColumn column1 = new TableColumn(table, SWT.NONE);\n\t\tcolumn1.setText(\"On\");\n\t\tcolumn1.setWidth(30);\n\n\t\tTableColumn column2 = new TableColumn(table, SWT.NONE);\n\t\tcolumn2.setText(\"Input\");\n\t\tcolumn2.setWidth(120);\n\n\t\tTableColumn column3 = new TableColumn(table, SWT.NONE);\n\t\tcolumn3.setText(\"Amount\");\n\t\tcolumn3.setWidth(45);\n\n\t\ttable.addListener(SWT.Selection, e -> {\n\t\t\tif (e.detail == SWT.CHECK) {\n\t\t\t\tTableItem item = (TableItem) e.item;\n\t\t\t\tInputMode mode = InputMode.valueOf(item.getText(1).replace(\" \", \"_\"));\n\t\t\t\tcfg.setInputMode(mode, !cfg.isInputEnabled(mode));\n\t\t\t}\n\t\t});\n\n\t\tfor (int i = 0; i < InputMode.values().length; i++) {\n\t\t\tTableItem item = new TableItem(table, SWT.NONE);\n\t\t\tInputMode mode = InputMode.values()[i];\n\t\t\tprompt.addInputStatObs(is -> {\n\t\t\t\tvar checked = item.getChecked();\n\t\t\t\titem.setText(new String[] { \"\", mode.name().replace(\"_\", \" \"), is[mode.ordinal()] + \"\" });\n\t\t\t\titem.setChecked(checked);\n\t\t\t}, true);\n\t\t\tcfg.addInputObs(is -> {\n\t\t\t\titem.setChecked(is[mode.ordinal()]);\n\t\t\t}, true);\n\t\t}\n\t}"
        - id: AISessionView.createOutputModeSection
          type: method_declaration
          code: "private void createOutputModeSection(Composite body, ConfigManager cfg) {\n\t\tComposite bottom = new Composite(body, SWT.NONE);\n\t\tbottom.setLayout(new GridLayout(2, false));\n\n\t\ttoolkit.createLabel(bottom, \"Output:\");\n\t\tCombo outputMode = new Combo(bottom, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tString[] outputOptions = Arrays.stream(OutputMode.values()).map(e -> e.name()).collect(Collectors.toList())\n\t\t\t\t.toArray(new String[0]);\n\t\toutputMode.setItems(outputOptions);\n\t\toutputMode.addSelectionListener(SelectionListener\n\t\t\t\t.widgetSelectedAdapter(e -> cfg.setOuputMode(OutputMode.valueOf(outputMode.getText()))));\n\t\tcfg.addOutputModeObs(m -> outputMode.setText(m.name()), true);\n\t}"
        - id: AISessionView.createActionButtonsSection
          type: method_declaration
          code: "private void createActionButtonsSection(Composite body, ConfigManager cfg, PromptHandler prompt) {\n\t\tComposite actions = new Composite(body, SWT.NONE);\n\t\tactions.setLayout(new GridLayout(3, false));\n\n\t\tButton btn = new Button(actions, SWT.PUSH);\n\t\tbtn.setText(\"Prompt\");\n\t\tbtn.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> prompt.execute(btn.getDisplay())));\n\n\t\tButton bbtn = new Button(actions, SWT.PUSH);\n\t\tbbtn.setText(\"Enqueue\");\n\t\tbbtn.addSelectionListener(SelectionListener\n\t\t\t\t.widgetSelectedAdapter(e -> prompt.queueAsync(bbtn.getDisplay())));\n\n\t\tButton bsbtn = new Button(actions, SWT.PUSH);\n\t\tbsbtn.setText(\"Batch\");\n\t\tbsbtn.addSelectionListener(SelectionListener\n\t\t\t\t.widgetSelectedAdapter(e -> prompt.queueAndSubmit(bsbtn.getDisplay())));\n\n\t\tcfg.addModelObs(m -> {\n\t\t\tbbtn.setEnabled(m.cap.isSupportBatch());\n\t\t\tbsbtn.setEnabled(m.cap.isSupportBatch());\n\t\t\tbody.layout();\n\t\t}, true);\n\t}"
        - id: AISessionView.createUsageLogSection
          type: method_declaration
          code: "private void createUsageLogSection(Composite body, PromptHandler prompt) {\n\t\tComposite footer = new Composite(body, SWT.NONE);\n\t\tfooter.setLayout(new GridLayout(1, false));\n\t\tfooter.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\n\t\tusageLog = toolkit.createText(footer, \"\", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);\n\t\tusageLog.setText(\"Total, Out, Reason, CRead, CCreate, In\\n\");\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.heightHint = 50;\n\t\tusageLog.setLayoutData(gridData);\n\n\t\tprompt.addAnswerObs(a -> {\n\t\t\tform.getDisplay().asyncExec(() -> {\n\t\t\t\tif (a != null && a.stats.inputToken > 0) {\n\t\t\t\t\tString text = usageLog.getText();\n\t\t\t\t\tif (text == null || text.isEmpty())\n\t\t\t\t\t\ttext = \"Total, Out, Reason, CRead, CCreate, In\\n\";\n\n\t\t\t\t\tString newtext = String.format(\"%6d,%6d,%5d,%6d,%6d,%6d\\n\", a.stats.totalToken, a.stats.outputToken,\n\t\t\t\t\t\t\ta.stats.reasoningToken, a.stats.cacheRead, a.stats.cacheCreate, a.stats.inputToken);\n\t\t\t\t\tint idx = text.indexOf('\\n');\n\t\t\t\t\ttext = text.substring(0, idx + 1) + newtext + text.substring(idx + 1);\n\n\t\t\t\t\tusageLog.setText(text);\n\t\t\t\t}\n\t\t\t});\n\t\t});\n\t}"
        - id: AISessionView.refreshPresetList
          type: method_declaration
          code: "private void refreshPresetList() {\n\t\tpresetFiles.clear();\n\t\tpresetFiles.addAll(Arrays.asList(PresetHandler.listPresetFiles()));\n\t\tpresetList.setItems(presetFiles.stream().map(f -> f.getFullPath().toString()).toArray(String[]::new));\n\t}"
        - id: AISessionView.updatePromptLines
          type: method_declaration
          code: "private String[] updatePromptLines(String[] cur) {\n\t\tString[] upd = new String[cur.length];\n\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\tString line = cur[i];\n\t\t\tboolean isSelected = instructionSelection.contains(line);\n\t\t\tif (!isSelected && !line.startsWith(\"#\")) {\n\t\t\t\tline = \"#\" + line;\n\t\t\t} else if (isSelected && line.startsWith(\"#\")) {\n\t\t\t\tline = line.substring(1);\n\t\t\t}\n\t\t\tupd[i] = line;\n\t\t}\n\t\treturn upd;\n\t}"
        - id: AISessionView.isTemperatureEnabled
          type: method_declaration
          code: "private boolean isTemperatureEnabled(Model m, Reasoning reasoning) {\n\t\tif (m.cap.getKeyPattern().equals(KeyPattern.Claude))\n\t\t\treturn m.cap.isSupportTemperature() && Reasoning.Disabled.equals(reasoning);\n\t\telse\n\t\t\treturn m.cap.isSupportTemperature();\n\t}"
        - id: AISessionView.toggleControl
          type: method_declaration
          code: "private void toggleControl(Label label, Control ctrl, boolean enabled) {\n\t\tlabel.setEnabled(enabled);\n\t\tlabel.setVisible(enabled);\n\t\tctrl.setEnabled(enabled);\n\t\tctrl.setVisible(enabled);\n\t\t((GridData) label.getLayoutData()).exclude = !enabled;\n\t\t((GridData) ctrl.getLayoutData()).exclude = !enabled;\n\t}"
        - id: AISessionView.updateInstructionList
          type: method_declaration
          code: "private void updateInstructionList(String[] systemPrompt) {\n\t\ttry {\n\t\t\tisUpdating = true;\n\t\t\tinstructionList.setItems(systemPrompt);\n\t\t\tinstructionList.deselectAll();\n\t\t\tfor (int i = 0; i < systemPrompt.length; i++) {\n\t\t\t\tif (!systemPrompt[i].startsWith(\"#\"))\n\t\t\t\t\tinstructionList.select(i);\n\t\t\t}\n\t\t\tform.reflow(true);\n\t\t} finally {\n\t\t\tisUpdating = false;\n\t\t}\n\t}"
        - id: AISessionView.updateEditList
          type: method_declaration
          code: "private void updateEditList(String[] systemPrompt) {\n\t\ttry {\n\t\t\tisUpdating = true;\n\t\t\tinstructionEdit.setText(String.join(\"\\n\", systemPrompt));\n\t\t\tform.reflow(true);\n\t\t} finally {\n\t\t\tisUpdating = false;\n\t\t}\n\t}"
        - id: AISessionView.setFocus
          type: method_declaration
          code: "@Override\n\tpublic void setFocus() {\n\t\t// form.setFocus();\n\t}"
```
/answer 21e8-6 allow
ReasoningToken: 66
ReasoningToken: 2983
Result Stats:  id=7524f0d0-1820-430b-adf1-781ec86a8d2c, total: 0, in: 137, out: 4822, reason: 0, read: 12767, write: 22721
## Analyse

Die Klasse hat 3 Gruppen eng gekoppelter Felder:

1. `modelSelItems` – nur in Key/Model/Profile/Reasoning/Cache-Methoden verwendet (Top-Parameter-Bereich, inkl. `toggleControl`, `isTemperatureEnabled`).
2. `instructionList`, `instructionEdit`, `instructionFree`, `toolsList`, `toolsMultiSelect`, `presetList`, `presetFiles`, `instructionSelection`, `isUpdating` – stark verflochten (Select-Tab schreibt `instructionSelection`/`isUpdating`, Edit-Tab und FreeText lesen `isUpdating`, Presets-Tab greift auf `toolsList`, `toolsMultiSelect` und `instructionSelection` zurück).
3. `usageLog` – isoliert, nur in `createUsageLogSection`.

`toolkit`, `form`, `display`, `cfg` werden von allen Bereichen benötigt → bleiben zentral in der View bzw. werden injiziert.

## Vorgeschlagene Aufteilung (je eigene Klasse, package-private, im selben Package)

**AISessionView** (bleibt `ViewPart`)
- `ID`, `currentInstance`, `workbench`, `toolkit`, `form`, `display`
- `saveState`, `init`, `createPartControl` (orchestriert), `setFocus`
- instanziiert die Panels unten und ruft `create(...)` auf

**TopParametersPanel**
- Feld: `modelSelItems`
- `create`, `createKeyField`, `createModelCombo`, `createProfileCombo`, `createReasoningSection`, `createCacheSection`, `isTemperatureEnabled`, `toggleControl`

**SystemPromptPanel** (fasst Select/Edit/Tools/Presets/FreeText zusammen, da diese Felder gemeinsam nutzen)
- Felder: `instructionList`, `instructionEdit`, `instructionFree`, `toolsList`, `toolsMultiSelect`, `presetList`, `presetFiles`, `instructionSelection`, `isUpdating`
- `create` (ehem. `createInstructionSection`), `createInstructionSelectTab`, `createInstructionEditTab`, `createToolsTab`, `createPresetsTab`, `createFreeTextArea`, `createInputsTable`, `refreshPresetList`, `updatePromptLines`, `updateInstructionList`, `updateEditList`

**OutputModePanel** – zustandslos, nur `create`

**ActionButtonsPanel** – zustandslos, nur `create`

**UsageLogPanel**
- Feld: `usageLog`
- `create`

## Begründung
- `toggleControl`/`isTemperatureEnabled` werden ausschließlich vom Top-Bereich genutzt → dort belassen statt als globales Util.
- Select/Edit/Tools/Presets/FreeText nicht weiter aufsplitten, da `isUpdating`, `instructionSelection`, `toolsMultiSelect` und `toolsList` querlaufend gebraucht werden – eine feinere Trennung würde die Felder wieder public/package-shared machen müssen.
- `toolkit`, `form`, `cfg`, `display` werden den Panels per Konstruktor übergeben; `body`/`Composite`-Parameter bleiben wie bisher Methodenparameter.
Hm ich dachte das eher anders. Bessere Schnittsetllen schaffen. Im endeffekt ist das ein GUI Builder aber dieser ist hierarchisch organisiert. Lassen sich nicht Ebenen besser schneiden und auslagern?
SystemInit:  id=7524f0d0-1820-430b-adf1-781ec86a8d2c, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 1527
Result Stats:  id=7524f0d0-1820-430b-adf1-781ec86a8d2c, total: 0, in: 139, out: 7571, reason: 0, read: 35488, write: 27005
Verstanden – Schnitt entlang der Widget-Hierarchie statt entlang loser "Aspekte", mit expliziten schmalen Schnittstellen statt geteilter Felder.

## Hierarchie (spiegelt den Composite-Baum)

```
AISessionView
 ├─ TopParametersSection            (top: Key/Model/Profile/MaxToken/Temp/TopP)
 │   ├─ ReasoningSubSection         (Kind von Top)
 │   └─ CacheSubSection             (Kind von Top)
 ├─ InstructionSection              (Label + TabFolder-Wrapper)
 │   ├─ SelectTab
 │   ├─ EditTab
 │   ├─ ToolsTab
 │   ├─ PresetsTab                  (hängt von ToolsTab & SelectTab ab)
 │   └─ InputsTable
 ├─ OutputModeSection
 ├─ ActionButtonsSection
 └─ UsageLogSection
```

Jede Klasse besitzt **nur ihre eigenen** Widgets/Felder (z.B. `SelectTab` besitzt `instructionList`, `isUpdating`, `instructionSelection`; `ToolsTab` besitzt `toolsList`, `toolsMultiSelect`; `PresetsTab` besitzt `presetList`, `presetFiles`). Kein Feld wird mehr klassenübergreifend geteilt.

## Schnittstellen statt geteilter Felder

Wo eine Kind-Klasse eine Nachbar-Klasse braucht, bekommt sie ein **minimales Interface** statt eines konkreten Objekts oder Feldzugriffs:

```java
interface SelectionSource { String[] getSelection(); }
interface Clearable       { void clear(); }
```

- `ToolsTab implements SelectionSource, Clearable`
- `SelectTab implements Clearable` (für `instructionSelection`-Reset)

`PresetsTab` bekommt im Konstruktor `SelectionSource tools, Clearable toolsClear, Clearable selectionClear` statt direkten Zugriff auf `toolsList`/`toolsMultiSelect`/`instructionSelection`.

`InstructionSection` (Elternknoten) baut die Kinder in Reihenfolge und verdrahtet sie:

```java
SelectTab select = new SelectTab(toolkit, cfg);
EditTab edit     = new EditTab(toolkit, cfg);
ToolsTab tools   = new ToolsTab(toolkit, cfg);
PresetsTab presets = new PresetsTab(toolkit, cfg, tools, select::clear);
```

## Übrig bleibendes „geteiltes" Zeug

- `isUpdating` (Programmatic-Update-Guard) wird von Select/Edit/FreeText gebraucht → eigenes kleines Objekt `UpdateGuard { boolean isUpdating; run(Runnable) }`, das allen dreien im Konstruktor übergeben wird – kein Feld auf Panel-Ebene mehr.
- `toolkit`, `form`, `display`, `cfg` bleiben Konstruktorparameter jeder Ebene (top-down durchgereicht, passend zur Composite-Elternschaft).

Damit folgt der Klassenschnitt exakt dem SWT-Composite-Baum, und Abhängigkeiten zwischen Geschwister-Tabs laufen über schmale Interfaces statt über gemeinsame private Felder.
Ja klingt besser. Den Baum schneiden, der Root AISessionView bietet die gemeinsamen Schnitteln und koordiniert.
SystemInit:  id=7524f0d0-1820-430b-adf1-781ec86a8d2c, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 21e8-7
toolName: ask_user
arguments:
  question: "Soll ich die Aufteilung jetzt konkret umsetzen (neue Klassen anlegen, Code verschieben, AISessionView als Koordinator umbauen)? Falls ja: gewünschtes Package/Verzeichnis (z.B. view/session/) und Namenskonvention (z.B. *Section vs. *Panel)?"
  reason: "Bevor umfangreiche Datei-Umstrukturierung in Produktionscode durchgeführt wird, Bestätigung und Konventionsvorgabe einholen."
```
/answer 21e8-7 deny ja schneide es direkt. Zielpackage für alles wird `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session`
ReasoningToken: 312
Control Request:
```yaml
id: 21e8-8
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
    ids:
    - 8lg35S|YsNcr0
    - 2C6knn|WTIi3p
    - 3JDbMD|JtZOqz
    - GjvraN|AaS6yV
  reason: "Vollständige Import-Liste und Package-Deklaration lesen, um neue Klassen mit korrekten Imports zu erzeugen."
```
/answer 21e8-8 allow
Control Request:
```yaml
id: 21e8-9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
      nodes:
      - id: 8lg35S|YsNcr0
        type: imports
        code: |-
          package xy.ai.workbench.view;

          import java.util.ArrayList;
          import java.util.Arrays;
          import java.util.concurrent.atomic.AtomicReference;
          import java.util.stream.Collectors;

          import org.eclipse.core.resources.IFile;
          import org.eclipse.swt.SWT;
          import org.eclipse.swt.custom.SashForm;
          import org.eclipse.swt.events.FocusListener;
          import org.eclipse.swt.events.MouseListener;
          import org.eclipse.swt.events.SelectionAdapter;
          import org.eclipse.swt.events.SelectionEvent;
          import org.eclipse.swt.events.SelectionListener;
      - id: 2C6knn|WTIi3p
        type: imports
        code: |-
          import org.eclipse.swt.layout.GridData;
          import org.eclipse.swt.layout.GridLayout;
          import org.eclipse.swt.widgets.Button;
          import org.eclipse.swt.widgets.Combo;
          import org.eclipse.swt.widgets.Composite;
          import org.eclipse.swt.widgets.Control;
          import org.eclipse.swt.widgets.Display;
          import org.eclipse.swt.widgets.Label;
          import org.eclipse.swt.widgets.List;
          import org.eclipse.swt.widgets.TabFolder;
          import org.eclipse.swt.widgets.TabItem;
          import org.eclipse.swt.widgets.Table;
      - id: 3JDbMD|JtZOqz
        type: imports
        code: |-
          import org.eclipse.swt.widgets.TableColumn;
          import org.eclipse.swt.widgets.TableItem;
          import org.eclipse.swt.widgets.Text;
          import org.eclipse.ui.IMemento;
          import org.eclipse.ui.IViewSite;
          import org.eclipse.ui.IWorkbench;
          import org.eclipse.ui.PartInitException;
          import org.eclipse.ui.forms.widgets.FormToolkit;
          import org.eclipse.ui.forms.widgets.ScrolledForm;
          import org.eclipse.ui.part.ViewPart;

          import jakarta.inject.Inject;
          import xy.ai.workbench.Activator;
          import xy.ai.workbench.AgentProfile;
      - id: GjvraN|AaS6yV
        type: imports
        code: |-
          import xy.ai.workbench.CacheMode;
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model;
          import xy.ai.workbench.Model.KeyPattern;
          import xy.ai.workbench.OutputMode;
          import xy.ai.workbench.Reasoning;
          import xy.ai.workbench.connector.harness.PromptHandler;
```
/answer 21e8-9 allow
Control Request:
```yaml
id: 21e8-a
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: LOG
    include:
    - '*.java'
    exclude:
    - '*.min.js'
  reason: Herkunft des statischen Loggers LOG bestimmen für korrekten Import in extrahierten Klassen.
```
/answer 21e8-a allow
Control Request:
```yaml
id: 21e8-b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/IncludeAdapter.java
        matches:
        - lineno: 77
          match: "LOG.error(e.getMessage(), e);"
        - lineno: 90
          match: "LOG.error(\"Error on reading \" + path, e);"
        - lineno: 111
          match: "LOG.error(\"Exception\", e);"
        - lineno: 145
          match: "LOG.info(\"Searchresult changed: \" + result.getLabel());"
        - lineno: 163
          match: "LOG.error(\"Error on reading \" + file.getName(), e);"
      - path: xy/ai/workbench/MementoConverter.java
        matches:
        - lineno: 43
          match: "LOG.error(\"Unable to serialize session config\", e);"
        - lineno: 58
          match: "LOG.error(\"Unable to deserialize session config\", e);"
      - path: xy/ai/workbench/LOG.java
        matches:
        - lineno: 7
          match: "public class LOG {"
      - path: xy/ai/workbench/EditorInterface.java
        matches:
        - lineno: 99
          match: LOG.info("Error opening new editor file");
        - lineno: 101
          match: LOG.info("Error writting file");
        - lineno: 103
          match: LOG.info("Error unsupported encoding");
        - lineno: 137
          match: LOG.info("Error adding text");
        - lineno: 150
          match: "LOG.info(\"Error: wasn't able to replace prompt marker with answer:\\n\" + ans.answer);"
      - path: xy/ai/workbench/model/DeepseekModelResolver.java
        matches:
        - lineno: 15
          match: import xy.ai.workbench.LOG;
        - lineno: 43
          match: LOG.error("Deepseek model list was empty");
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 21e8-b allow
Control Request:
```yaml
id: 21e8-c
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: class MultiSelectListener|class PresetHandler
    include:
    - '*.java'
  reason: Packages von MultiSelectListener und PresetHandler ermitteln für Imports in neuen Klassen.
```
/answer 21e8-c allow
ReasoningToken: 311
Control Request:
```yaml
id: 21e8-d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/MultiSelectListener.java
        matches:
        - lineno: 9
          match: "public class MultiSelectListener {"
      - path: xy/ai/workbench/view/PresetHandler.java
        matches:
        - lineno: 31
          match: "public class PresetHandler {"
```
/answer 21e8-d allow
ReasoningToken: 50
Control Request:
```yaml
id: 21e8-e
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: AISessionView\.currentInstance|currentInstance\.display|AISessionView\.currentInstance\.display
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: \.presetFiles|\.instructionSelection|\.toolsMultiSelect|\.instructionList|\.instructionEdit|\.instructionFree|\.toolsList|\.presetList|\.usageLog
    include:
    - '*.java'
    exclude:
    - AISessionView.java
  reason: "Prüfen, ob externe Klassen auf jetzt zu verschiebende Felder/currentInstance.display zugreifen, um Sichtbarkeit korrekt zu setzen."
```
/answer 21e8-e allow
Control Request:
```yaml
id: 21e8-f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/handlers/SessionExecuteHandler.java
        matches:
        - lineno: 16
          match: prompt.execute(AISessionView.currentInstance.display);
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches: []
```
/answer 21e8-f allow
ReasoningToken: 4276
Control Request:
```yaml
id: 21e8-10
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/UpdateGuard.java
    source: "package xy.ai.workbench.view.session;\n\n/**\n * Suppresses re-entrant selection/modify events while a control's content is\n * being set programmatically.\n */\nfinal class UpdateGuard {\n\n\tprivate boolean updating = false;\n\n\tboolean isUpdating() {\n\t\treturn updating;\n\t}\n\n\tvoid run(Runnable action) {\n\t\ttry {\n\t\t\tupdating = true;\n\t\t\taction.run();\n\t\t} finally {\n\t\t\tupdating = false;\n\t\t}\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/Contracts.java
    source: "package xy.ai.workbench.view.session;\n\n/** Something that can be reset to an empty selection. */\ninterface Clearable {\n\tvoid clear();\n}\n\n/** Read/write access to a widget's multi-selection, needed for cross-tab preset handling. */\ninterface ToolSelection extends Clearable {\n\tString[] getSelection();\n\n\tvoid setSelection(String[] items);\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/TopParametersSection.java
    source: "package xy.ai.workbench.view.session;\n\nimport java.util.Arrays;\nimport java.util.concurrent.atomic.AtomicReference;\nimport java.util.stream.Collectors;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.events.FocusListener;\nimport org.eclipse.swt.events.MouseListener;\nimport org.eclipse.swt.events.SelectionListener;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Combo;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Control;\nimport org.eclipse.swt.widgets.Label;\nimport org.eclipse.swt.widgets.Text;\nimport org.eclipse.ui.forms.widgets.FormToolkit;\n\nimport xy.ai.workbench.AgentProfile;\nimport xy.ai.workbench.CacheMode;\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.Model;\nimport xy.ai.workbench.Model.KeyPattern;\nimport xy.ai.workbench.Reasoning;\n\n/** Key/Model/Profile/MaxToken/Temp/TopP/Reasoning/Cache row at the top of the session view. */\npublic final class TopParametersSection {\n\n\tprivate final FormToolkit toolkit;\n\n\tpublic TopParametersSection(FormToolkit toolkit) {\n\t\tthis.toolkit = toolkit;\n\t}\n\n\tpublic void create(Composite body, ConfigManager cfg) {\n\t\tComposite top = new Composite(body, SWT.NONE);\n\t\ttop.setLayout(new GridLayout(2, false));\n\t\ttop.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\n\t\tcreateKeyField(top, cfg);\n\t\tcreateModelCombo(top, cfg);\n\t\tcreateProfileCombo(top, cfg);\n\n\t\tLabel maxTokenLabel = toolkit.createLabel(top, \"Max Token:\");\n\t\tmaxTokenLabel.setLayoutData(new GridData());\n\t\tText maxToken = toolkit.createText(top, \"\", SWT.BORDER);\n\t\tmaxToken.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tmaxToken.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setMaxOutputTokens(Long.parseLong(maxToken.getText()))));\n\t\tmaxToken.addMouseListener(MouseListener.mouseDownAdapter(m -> maxToken.setFocus()));\n\t\tcfg.addOutputTokenObs(ot -> maxToken.setText(ot + \"\"), true);\n\n\t\tLabel tempLabel = toolkit.createLabel(top, \"Temp:\");\n\t\ttempLabel.setLayoutData(new GridData());\n\t\tText temp = toolkit.createText(top, \"\", SWT.BORDER);\n\t\ttemp.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\ttemp.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setTemperature(Double.parseDouble(temp.getText()))));\n\t\ttemp.addMouseListener(MouseListener.mouseDownAdapter(m -> temp.setFocus()));\n\t\tcfg.addTemperatureObs(t -> temp.setText(t + \"\"), true);\n\n\t\tLabel topPLabel = toolkit.createLabel(top, \"TopP:\");\n\t\ttopPLabel.setLayoutData(new GridData());\n\t\tText topP = toolkit.createText(top, \"\", SWT.BORDER);\n\t\ttopP.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\ttopP.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setTopP(Double.parseDouble(topP.getText()))));\n\t\ttopP.addMouseListener(MouseListener.mouseDownAdapter(m -> topP.setFocus()));\n\t\tcfg.addTopPObs(tp -> topP.setText(tp + \"\"), true);\n\n\t\tcreateReasoningSection(top, body, cfg, tempLabel, temp, topPLabel, topP, maxTokenLabel, maxToken);\n\t\tcreateCacheSection(top, cfg);\n\t}\n\n\tprivate void createKeyField(Composite top, ConfigManager cfg) {\n\t\ttoolkit.createLabel(top, \"Key:\");\n\t\tText keyInput = toolkit.createText(top, \"\", SWT.BORDER | SWT.PASSWORD);\n\t\tGridData kilay = new GridData(GridData.FILL_HORIZONTAL);\n\t\tkilay.widthHint = 10;\n\t\tkeyInput.setLayoutData(kilay);\n\t\tkeyInput.addModifyListener(e -> cfg.setKey(keyInput.getText()));\n\t\tkeyInput.setText(cfg.getKeys() + \"\");\n\t\tkeyInput.addMouseListener(MouseListener.mouseDownAdapter(m -> keyInput.setFocus()));\n\t}\n\n\tprivate void createModelCombo(Composite top, ConfigManager cfg) {\n\t\ttoolkit.createLabel(top, \"Model:\");\n\t\tCombo modelSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tmodelSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tAtomicReference<Model[]> modelSelItems = new AtomicReference<>(new Model[0]);\n\t\tmodelSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tint idx = modelSel.getSelectionIndex();\n\t\t\tModel[] items = modelSelItems.get();\n\t\t\tif (idx >= 0 && idx < items.length)\n\t\t\t\tcfg.setModel(items[idx]);\n\t\t}));\n\t\tcfg.addEnabledModelsObs(k -> {\n\t\t\tmodelSelItems.set(k);\n\t\t\tmodelSel.setItems(\n\t\t\t\t\tArrays.stream(k).map((m) -> m.displayName).collect(Collectors.toList()).toArray(new String[0]));\n\t\t\tModel current = cfg.getModel();\n\t\t\tmodelSel.setText(current != null ? current.displayName : \"\");\n\t\t}, true);\n\t}\n\n\tprivate void createProfileCombo(Composite top, ConfigManager cfg) {\n\t\tLabel profileLabel = toolkit.createLabel(top, \"Profile:\");\n\t\tprofileLabel.setLayoutData(new GridData());\n\t\tCombo profileSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tprofileSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tprofileSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg\n\t\t\t\t.setProfile(profileSel.getText().isBlank() ? null : AgentProfile.fromName(profileSel.getText()))));\n\t\tcfg.addEnabledProfilesObs(k -> {\n\t\t\tprofileSel\n\t\t\t\t\t.setItems(Arrays.stream(k).map((m) -> m.name).collect(Collectors.toList()).toArray(new String[0]));\n\t\t\tprofileSel.setText(k.length > 0 ? k[0].name : \"\");\n\t\t\ttoggleControl(profileLabel, profileSel, k.length > 0);\n\t\t\ttop.layout();\n\t\t}, true);\n\t\tcfg.addProfileObs(p -> {\n\t\t\tprofileSel.setText(p != null ? p.name : \"\");\n\t\t}, true);\n\t}\n\n\tprivate void createReasoningSection(Composite top, Composite body, ConfigManager cfg, Label tempLabel, Text temp,\n\t\t\tLabel topPLabel, Text topP, Label maxTokenLabel, Text maxToken) {\n\t\tLabel reasoningLabel = toolkit.createLabel(top, \"Reasoning:\");\n\t\treasoningLabel.setLayoutData(new GridData());\n\t\tComposite secReason = new Composite(top, SWT.NONE);\n\t\tGridLayout secRLay = new GridLayout(2, false);\n\t\tsecRLay.marginHeight = secRLay.marginWidth = 0;\n\t\tsecReason.setLayout(secRLay);\n\t\tsecReason.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\n\t\tCombo reasSel = new Combo(secReason, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\treasSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\treasSel.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setReasoning(Reasoning.valueOf(reasSel.getText()))));\n\n\t\tText budget = toolkit.createText(secReason, \"\", SWT.BORDER);\n\t\tbudget.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tbudget.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setReasoningBudget(Integer.parseInt(budget.getText()))));\n\t\tbudget.addMouseListener(MouseListener.mouseDownAdapter(m -> budget.setFocus()));\n\t\tcfg.addBudgetObs(bg -> budget.setText(bg + \"\"), true);\n\n\t\tcfg.addModelObs(m -> {\n\t\t\ttoggleControl(tempLabel, temp, isTemperatureEnabled(m, cfg.getReasoning()));\n\t\t\ttoggleControl(topPLabel, topP, m.cap.isSupportTopP());\n\t\t\ttoggleControl(maxTokenLabel, maxToken, m.cap.isSupportMaxToken());\n\n\t\t\tString[] reasonings = cfg.getReasonings();\n\t\t\treasSel.setItems(reasonings);\n\t\t\treasSel.setText(cfg.getReasoning().name());\n\t\t\tboolean hasReasoning = reasonings.length > 0\n\t\t\t\t\t&& !(reasonings.length == 1 && (\"Default\".equals(reasonings[0]) || \"Disabled\".equals(reasonings[0])));\n\t\t\ttoggleControl(reasoningLabel, secReason, hasReasoning);\n\t\t\tbody.layout();\n\t\t}, true);\n\t\tcfg.addReasoningObs(r -> {\n\n\t\t\tboolean enabled = Reasoning.Budget.equals(r);\n\t\t\tbudget.setEnabled(enabled);\n\t\t\tbudget.setVisible(enabled);\n\t\t\t((GridData) budget.getLayoutData()).exclude = !enabled;\n\n\t\t\ttoggleControl(tempLabel, temp, isTemperatureEnabled(cfg.getModel(), r));\n\n\t\t\tsecReason.layout();\n\t\t\tbody.layout();\n\t\t}, true);\n\t}\n\n\tprivate void createCacheSection(Composite top, ConfigManager cfg) {\n\t\tLabel cacheLabel = toolkit.createLabel(top, \"Cache:\");\n\t\tcacheLabel.setLayoutData(new GridData());\n\t\tCombo cacheSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tcacheSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tcacheSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg.setCacheMode(\n\t\t\t\tcacheSel.getText().isBlank() ? CacheMode.Default : CacheMode.valueOf(cacheSel.getText()))));\n\t\tcfg.addModelObs(m -> {\n\t\t\tcacheSel.setItems(Arrays.stream(m.cap.getCacheMode()).map((c) -> c.name()).collect(Collectors.toList())\n\t\t\t\t\t.toArray(new String[0]));\n\t\t\tcacheSel.setText(cfg.getCacheMode() != null ? cfg.getCacheMode().name() : \"\");\n\t\t\ttoggleControl(cacheLabel, cacheSel, m.cap.getCacheMode().length > 0);\n\t\t}, true);\n\t\tcfg.addCacheObs(c -> {\n\t\t\tcacheSel.setText(c != null ? c.name() : \"\");\n\t\t}, true);\n\t}\n\n\tprivate boolean isTemperatureEnabled(Model m, Reasoning reasoning) {\n\t\tif (m.cap.getKeyPattern().equals(KeyPattern.Claude))\n\t\t\treturn m.cap.isSupportTemperature() && Reasoning.Disabled.equals(reasoning);\n\t\telse\n\t\t\treturn m.cap.isSupportTemperature();\n\t}\n\n\tprivate void toggleControl(Label label, Control ctrl, boolean enabled) {\n\t\tlabel.setEnabled(enabled);\n\t\tlabel.setVisible(enabled);\n\t\tctrl.setEnabled(enabled);\n\t\tctrl.setVisible(enabled);\n\t\t((GridData) label.getLayoutData()).exclude = !enabled;\n\t\t((GridData) ctrl.getLayoutData()).exclude = !enabled;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    source: "package xy.ai.workbench.view.session;\n\nimport java.util.ArrayList;\nimport java.util.Arrays;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.custom.SashForm;\nimport org.eclipse.swt.events.FocusListener;\nimport org.eclipse.swt.events.MouseListener;\nimport org.eclipse.swt.events.SelectionListener;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.List;\nimport org.eclipse.swt.widgets.TabFolder;\nimport org.eclipse.swt.widgets.TabItem;\nimport org.eclipse.swt.widgets.Text;\nimport org.eclipse.ui.forms.widgets.FormToolkit;\n\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.LOG;\n\n/** \"Select\" tab: toggles system-prompt lines on/off and holds the free-text addendum below it. */\nfinal class SelectTab implements Clearable {\n\n\tprivate final FormToolkit toolkit;\n\tprivate final UpdateGuard guard;\n\tprivate final Runnable reflow;\n\n\tprivate List instructionList;\n\tprivate Text instructionFree;\n\tprivate java.util.List<String> instructionSelection = new ArrayList<>();\n\n\tSelectTab(FormToolkit toolkit, UpdateGuard guard, Runnable reflow) {\n\t\tthis.toolkit = toolkit;\n\t\tthis.guard = guard;\n\t\tthis.reflow = reflow;\n\t}\n\n\tvoid create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem instrSel = new TabItem(instr, SWT.NONE);\n\t\tinstrSel.setText(\"Select\");\n\n\t\tSashForm sash = new SashForm(instr, SWT.VERTICAL);\n\t\tsash.setLayout(new GridLayout(1, false));\n\t\tGridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tscl2.heightHint = 100;\n\t\tscl2.widthHint = 1;\n\t\tsash.setLayoutData(scl2);\n\n\t\tComposite comp = new Composite(sash, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\tinstrSel.setControl(sash);\n\t\tinstructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\t\tcfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\tinstructionList.setLayoutData(gridData);\n\t\tinstructionList.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionList.setFocus()));\n\n\t\tinstructionList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tinstructionSelection = new ArrayList<>(Arrays.asList(instructionList.getSelection()));\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t}));\n\t\tinstructionList.addListener(SWT.MouseDown, event -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tString[] clickedIndex = instructionList.getSelection();\n\t\t\tLOG.info(Arrays.toString(clickedIndex));\n\t\t\tif (clickedIndex.length != 1)\n\t\t\t\treturn;\n\n\t\t\tif (!instructionSelection.remove(clickedIndex[0]))\n\t\t\t\tinstructionSelection.add(clickedIndex[0]);\n\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t});\n\n\t\tcreateFreeTextArea(sash, cfg);\n\t}\n\n\tprivate void createFreeTextArea(SashForm sash, ConfigManager cfg) {\n\t\tinstructionFree = toolkit.createText(sash, \"\", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);\n\t\tcfg.addSystemFreeObs(p -> {\n\t\t\tif (!instructionFree.isFocusControl())\n\t\t\t\tguard.run(() -> {\n\t\t\t\t\tinstructionFree.setText(p != null ? p : \"\");\n\t\t\t\t\treflow.run();\n\t\t\t\t});\n\t\t}, true);\n\t\tinstructionFree.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setSystemFree(instructionFree.getText())));\n\t\tGridData gridData = new GridData(GridData.FILL_HORIZONTAL);\n\t\tgridData.widthHint = 1;\n\t\tinstructionFree.setLayoutData(gridData);\n\t\tinstructionFree.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionFree.setFocus()));\n\t\tsash.setWeights(3, 1);\n\t}\n\n\tprivate String[] updatePromptLines(String[] cur) {\n\t\tString[] upd = new String[cur.length];\n\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\tString line = cur[i];\n\t\t\tboolean isSelected = instructionSelection.contains(line);\n\t\t\tif (!isSelected && !line.startsWith(\"#\")) {\n\t\t\t\tline = \"#\" + line;\n\t\t\t} else if (isSelected && line.startsWith(\"#\")) {\n\t\t\t\tline = line.substring(1);\n\t\t\t}\n\t\t\tupd[i] = line;\n\t\t}\n\t\treturn upd;\n\t}\n\n\tprivate void updateInstructionList(String[] systemPrompt) {\n\t\tguard.run(() -> {\n\t\t\tinstructionList.setItems(systemPrompt);\n\t\t\tinstructionList.deselectAll();\n\t\t\tfor (int i = 0; i < systemPrompt.length; i++) {\n\t\t\t\tif (!systemPrompt[i].startsWith(\"#\"))\n\t\t\t\t\tinstructionList.select(i);\n\t\t\t}\n\t\t\treflow.run();\n\t\t});\n\t}\n\n\t@Override\n\tpublic void clear() {\n\t\tinstructionSelection.clear();\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/EditTab.java
    source: "package xy.ai.workbench.view.session;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.events.MouseListener;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.TabFolder;\nimport org.eclipse.swt.widgets.TabItem;\nimport org.eclipse.swt.widgets.Text;\nimport org.eclipse.ui.forms.widgets.FormToolkit;\n\nimport xy.ai.workbench.ConfigManager;\n\n/** \"Edit\" tab: raw multi-line editor for the system prompt. */\nfinal class EditTab {\n\n\tprivate final FormToolkit toolkit;\n\tprivate final UpdateGuard guard;\n\tprivate final Runnable reflow;\n\n\tprivate Text instructionEdit;\n\n\tEditTab(FormToolkit toolkit, UpdateGuard guard, Runnable reflow) {\n\t\tthis.toolkit = toolkit;\n\t\tthis.guard = guard;\n\t\tthis.reflow = reflow;\n\t}\n\n\tvoid create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem instrEdit = new TabItem(instr, SWT.NONE);\n\t\tinstrEdit.setText(\"Edit\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tinstrEdit.setControl(comp);\n\t\tinstructionEdit = toolkit.createText(comp, \"\", SWT.WRAP | SWT.V_SCROLL);\n\t\tcfg.addSystemPromptObs(p -> {\n\t\t\tif (!instructionEdit.isFocusControl())\n\t\t\t\tupdateEditList(p.systemPrompt);\n\t\t}, true);\n\t\tinstructionEdit.addModifyListener(e -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tcfg.setSystemPrompt(instructionEdit.getText().split(\"\\n\"));\n\t\t});\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\tinstructionEdit.setLayoutData(gridData);\n\t\tinstructionEdit.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionEdit.setFocus()));\n\t}\n\n\tprivate void updateEditList(String[] systemPrompt) {\n\t\tguard.run(() -> {\n\t\t\tinstructionEdit.setText(String.join(\"\\n\", systemPrompt));\n\t\t\treflow.run();\n\t\t});\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    source: "package xy.ai.workbench.view.session;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.events.MouseListener;\nimport org.eclipse.swt.events.SelectionListener;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.List;\nimport org.eclipse.swt.widgets.TabFolder;\nimport org.eclipse.swt.widgets.TabItem;\n\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.view.MultiSelectListener;\n\n/** \"Tools\" tab: multi-select list of tools enabled for the current model. */\nfinal class ToolsTab implements ToolSelection {\n\n\tprivate List toolsList;\n\tprivate MultiSelectListener toolsMultiSelect;\n\n\tvoid create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem toolSel = new TabItem(instr, SWT.NONE);\n\t\ttoolSel.setText(\"Tools\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\ttoolSel.setControl(comp);\n\t\ttoolsList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\ttoolsList.setLayoutData(gridData);\n\t\ttoolsList.addMouseListener(MouseListener.mouseDownAdapter(m -> toolsList.setFocus()));\n\t\ttoolsList.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setEnabledTools(toolsList.getSelection())));\n\t\tcfg.addModelObs(m -> {\n\t\t\tif (m != null)\n\t\t\t\ttoolsList.setItems(m.cap.getTools());\n\t\t}, true);\n\t\ttoolsMultiSelect = new MultiSelectListener(toolsList);\n\t}\n\n\t@Override\n\tpublic String[] getSelection() {\n\t\treturn toolsList.getSelection();\n\t}\n\n\t@Override\n\tpublic void setSelection(String[] items) {\n\t\ttoolsMultiSelect.setSelection(items);\n\t}\n\n\t@Override\n\tpublic void clear() {\n\t\ttoolsMultiSelect.clear();\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    source: "package xy.ai.workbench.view.session;\n\nimport java.util.ArrayList;\nimport java.util.Arrays;\n\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.events.MouseListener;\nimport org.eclipse.swt.events.SelectionAdapter;\nimport org.eclipse.swt.events.SelectionEvent;\nimport org.eclipse.swt.events.SelectionListener;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Button;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.List;\nimport org.eclipse.swt.widgets.Shell;\nimport org.eclipse.swt.widgets.TabFolder;\nimport org.eclipse.swt.widgets.TabItem;\n\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.view.PresetHandler;\n\n/** \"Presets\" tab: save/load/reset the whole system-prompt configuration. */\nfinal class PresetsTab {\n\n\tprivate List presetList;\n\tprivate final java.util.List<IFile> presetFiles = new ArrayList<>();\n\n\tvoid create(TabFolder instr, ConfigManager cfg, Shell shell, ToolSelection tools, Clearable instructionSelection) {\n\t\tTabItem presEdit = new TabItem(instr, SWT.NONE);\n\t\tpresEdit.setText(\"Presets\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tGridLayout compLay = new GridLayout(1, false);\n\t\tcomp.setLayout(compLay);\n\t\tGridData compDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tcomp.setLayoutData(compDat);\n\t\tpresEdit.setControl(comp);\n\n\t\tComposite buttons = new Composite(comp, SWT.NONE);\n\t\tbuttons.setLayout(new GridLayout(2, false));\n\n\t\tButton writeButton = new Button(buttons, SWT.PUSH);\n\t\twriteButton.setText(\"Save\");\n\t\twriteButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tPresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), shell);\n\t\t\t\trefreshPresetList();\n\t\t\t}\n\t\t});\n\n\t\tButton resetButton = new Button(buttons, SWT.PUSH);\n\t\tresetButton.setText(\"Reset\");\n\t\tresetButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tString[] cur = cfg.getSystemPrompt();\n\t\t\t\tString[] upd = new String[cur.length];\n\t\t\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\t\t\tString line = cur[i];\n\t\t\t\t\tupd[i] = line.startsWith(\"#\") ? line : \"#\" + line;\n\t\t\t\t}\n\t\t\t\tinstructionSelection.clear();\n\t\t\t\tcfg.setSystemPrompt(upd);\n\n\t\t\t\ttools.clear();\n\t\t\t\tcfg.setEnabledTools(new String[0]);\n\t\t\t}\n\t\t});\n\n\t\tpresetList = new List(comp, SWT.SINGLE | SWT.V_SCROLL | SWT.BORDER);\n\t\tGridData presetListDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tpresetListDat.heightHint = 60;\n\t\tpresetList.setLayoutData(presetListDat);\n\t\tpresetList.addMouseListener(MouseListener.mouseDownAdapter(m -> presetList.setFocus()));\n\t\tpresetList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tint idx = presetList.getSelectionIndex();\n\t\t\tif (idx < 0 || idx >= presetFiles.size())\n\t\t\t\treturn;\n\t\t\tPresetHandler.Preset preset = PresetHandler.loadPreset(presetFiles.get(idx));\n\t\t\tcfg.setSystemPrompt(preset.body);\n\t\t\tif (preset.tools != null) {\n\t\t\t\ttools.setSelection(preset.tools);\n\t\t\t\tcfg.setEnabledTools(preset.tools);\n\t\t\t}\n\t\t\tif (preset.outputMode != null)\n\t\t\t\tcfg.setOuputMode(preset.outputMode);\n\t\t}));\n\t\trefreshPresetList();\n\t}\n\n\tprivate void refreshPresetList() {\n\t\tpresetFiles.clear();\n\t\tpresetFiles.addAll(Arrays.asList(PresetHandler.listPresetFiles()));\n\t\tpresetList.setItems(presetFiles.stream().map(f -> f.getFullPath().toString()).toArray(String[]::new));\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputsTable.java
    source: "package xy.ai.workbench.view.session;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Table;\nimport org.eclipse.swt.widgets.TableColumn;\nimport org.eclipse.swt.widgets.TableItem;\n\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.InputMode;\nimport xy.ai.workbench.connector.harness.PromptHandler;\n\n/** Checkbox table showing/enabling the available prompt input modes. */\nfinal class InputsTable {\n\n\tprivate InputsTable() {\n\t}\n\n\tstatic void create(Composite middle, ConfigManager cfg, PromptHandler prompt) {\n\t\tTable table = new Table(middle, SWT.CHECK | SWT.BORDER | SWT.V_SCROLL);\n\t\ttable.setHeaderVisible(true);\n\t\ttable.setLinesVisible(true);\n\n\t\tTableColumn column1 = new TableColumn(table, SWT.NONE);\n\t\tcolumn1.setText(\"On\");\n\t\tcolumn1.setWidth(30);\n\n\t\tTableColumn column2 = new TableColumn(table, SWT.NONE);\n\t\tcolumn2.setText(\"Input\");\n\t\tcolumn2.setWidth(120);\n\n\t\tTableColumn column3 = new TableColumn(table, SWT.NONE);\n\t\tcolumn3.setText(\"Amount\");\n\t\tcolumn3.setWidth(45);\n\n\t\ttable.addListener(SWT.Selection, e -> {\n\t\t\tif (e.detail == SWT.CHECK) {\n\t\t\t\tTableItem item = (TableItem) e.item;\n\t\t\t\tInputMode mode = InputMode.valueOf(item.getText(1).replace(\" \", \"_\"));\n\t\t\t\tcfg.setInputMode(mode, !cfg.isInputEnabled(mode));\n\t\t\t}\n\t\t});\n\n\t\tfor (int i = 0; i < InputMode.values().length; i++) {\n\t\t\tTableItem item = new TableItem(table, SWT.NONE);\n\t\t\tInputMode mode = InputMode.values()[i];\n\t\t\tprompt.addInputStatObs(is -> {\n\t\t\t\tvar checked = item.getChecked();\n\t\t\t\titem.setText(new String[] { \"\", mode.name().replace(\"_\", \" \"), is[mode.ordinal()] + \"\" });\n\t\t\t\titem.setChecked(checked);\n\t\t\t}, true);\n\t\t\tcfg.addInputObs(is -> {\n\t\t\t\titem.setChecked(is[mode.ordinal()]);\n\t\t\t}, true);\n\t\t}\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    source: "package xy.ai.workbench.view.session;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Shell;\nimport org.eclipse.swt.widgets.TabFolder;\nimport org.eclipse.ui.forms.widgets.FormToolkit;\n\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.connector.harness.PromptHandler;\n\n/** \"System Prompt\" area: Select/Edit/Tools/Presets tabs plus the input-mode table. */\npublic final class InstructionSection {\n\n\tprivate final FormToolkit toolkit;\n\n\tprivate final UpdateGuard guard = new UpdateGuard();\n\tprivate final SelectTab selectTab;\n\tprivate final EditTab editTab;\n\tprivate final ToolsTab toolsTab = new ToolsTab();\n\tprivate final PresetsTab presetsTab = new PresetsTab();\n\n\tpublic InstructionSection(FormToolkit toolkit, Runnable reflow) {\n\t\tthis.toolkit = toolkit;\n\t\tthis.selectTab = new SelectTab(toolkit, guard, reflow);\n\t\tthis.editTab = new EditTab(toolkit, guard, reflow);\n\t}\n\n\tpublic void create(Composite body, ConfigManager cfg, PromptHandler prompt, Shell shell) {\n\t\tComposite middle = new Composite(body, SWT.NONE);\n\t\tmiddle.setLayout(new GridLayout(1, false));\n\t\tGridData ldat2 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat2.heightHint = 100;\n\t\tmiddle.setLayoutData(ldat2);\n\n\t\ttoolkit.createLabel(middle, \"System Prompt:\");\n\n\t\tComposite contComp = new Composite(middle, SWT.NONE);\n\t\tcontComp.setLayout(new GridLayout(1, false));\n\t\tGridData scl = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tscl.heightHint = 100;\n\t\tscl.widthHint = 1;\n\t\tcontComp.setLayoutData(scl);\n\n\t\tTabFolder instr = new TabFolder(contComp, SWT.NONE);\n\t\tGridData ldat1 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat1.heightHint = 100;\n\t\tinstr.setLayoutData(ldat1);\n\n\t\tselectTab.create(instr, cfg);\n\t\teditTab.create(instr, cfg);\n\t\ttoolsTab.create(instr, cfg);\n\t\tpresetsTab.create(instr, cfg, shell, toolsTab, selectTab);\n\n\t\tInputsTable.create(middle, cfg, prompt);\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/OutputModeSection.java
    source: "package xy.ai.workbench.view.session;\n\nimport java.util.Arrays;\nimport java.util.stream.Collectors;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.events.SelectionListener;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Combo;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.ui.forms.widgets.FormToolkit;\n\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.OutputMode;\n\n/** Output mode selector below the system prompt area. */\npublic final class OutputModeSection {\n\n\tprivate OutputModeSection() {\n\t}\n\n\tpublic static void create(FormToolkit toolkit, Composite body, ConfigManager cfg) {\n\t\tComposite bottom = new Composite(body, SWT.NONE);\n\t\tbottom.setLayout(new GridLayout(2, false));\n\n\t\ttoolkit.createLabel(bottom, \"Output:\");\n\t\tCombo outputMode = new Combo(bottom, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tString[] outputOptions = Arrays.stream(OutputMode.values()).map(e -> e.name()).collect(Collectors.toList())\n\t\t\t\t.toArray(new String[0]);\n\t\toutputMode.setItems(outputOptions);\n\t\toutputMode.addSelectionListener(SelectionListener\n\t\t\t\t.widgetSelectedAdapter(e -> cfg.setOuputMode(OutputMode.valueOf(outputMode.getText()))));\n\t\tcfg.addOutputModeObs(m -> outputMode.setText(m.name()), true);\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ActionButtonsSection.java
    source: "package xy.ai.workbench.view.session;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.events.SelectionListener;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Button;\nimport org.eclipse.swt.widgets.Composite;\n\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.connector.harness.PromptHandler;\n\n/** Prompt/Enqueue/Batch action buttons. */\npublic final class ActionButtonsSection {\n\n\tprivate ActionButtonsSection() {\n\t}\n\n\tpublic static void create(Composite body, ConfigManager cfg, PromptHandler prompt) {\n\t\tComposite actions = new Composite(body, SWT.NONE);\n\t\tactions.setLayout(new GridLayout(3, false));\n\n\t\tButton btn = new Button(actions, SWT.PUSH);\n\t\tbtn.setText(\"Prompt\");\n\t\tbtn.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> prompt.execute(btn.getDisplay())));\n\n\t\tButton bbtn = new Button(actions, SWT.PUSH);\n\t\tbbtn.setText(\"Enqueue\");\n\t\tbbtn.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> prompt.queueAsync(bbtn.getDisplay())));\n\n\t\tButton bsbtn = new Button(actions, SWT.PUSH);\n\t\tbsbtn.setText(\"Batch\");\n\t\tbsbtn.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> prompt.queueAndSubmit(bsbtn.getDisplay())));\n\n\t\tcfg.addModelObs(m -> {\n\t\t\tbbtn.setEnabled(m.cap.isSupportBatch());\n\t\t\tbsbtn.setEnabled(m.cap.isSupportBatch());\n\t\t\tbody.layout();\n\t\t}, true);\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/UsageLogSection.java
    source: "package xy.ai.workbench.view.session;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.swt.widgets.Text;\nimport org.eclipse.ui.forms.widgets.FormToolkit;\n\nimport xy.ai.workbench.connector.harness.PromptHandler;\n\n/** Rolling CSV log of token usage per answer. */\npublic final class UsageLogSection {\n\n\tprivate UsageLogSection() {\n\t}\n\n\tpublic static void create(FormToolkit toolkit, Composite body, PromptHandler prompt, Display display) {\n\t\tComposite footer = new Composite(body, SWT.NONE);\n\t\tfooter.setLayout(new GridLayout(1, false));\n\t\tfooter.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\n\t\tText usageLog = toolkit.createText(footer, \"\", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);\n\t\tusageLog.setText(\"Total, Out, Reason, CRead, CCreate, In\\n\");\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.heightHint = 50;\n\t\tusageLog.setLayoutData(gridData);\n\n\t\tprompt.addAnswerObs(a -> {\n\t\t\tdisplay.asyncExec(() -> {\n\t\t\t\tif (a != null && a.stats.inputToken > 0) {\n\t\t\t\t\tString text = usageLog.getText();\n\t\t\t\t\tif (text == null || text.isEmpty())\n\t\t\t\t\t\ttext = \"Total, Out, Reason, CRead, CCreate, In\\n\";\n\n\t\t\t\t\tString newtext = String.format(\"%6d,%6d,%5d,%6d,%6d,%6d\\n\", a.stats.totalToken,\n\t\t\t\t\t\t\ta.stats.outputToken, a.stats.reasoningToken, a.stats.cacheRead, a.stats.cacheCreate,\n\t\t\t\t\t\t\ta.stats.inputToken);\n\t\t\t\t\tint idx = text.indexOf('\\n');\n\t\t\t\t\ttext = text.substring(0, idx + 1) + newtext + text.substring(idx + 1);\n\n\t\t\t\t\tusageLog.setText(text);\n\t\t\t\t}\n\t\t\t});\n\t\t});\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
    overwrite: true
    source: "package xy.ai.workbench.view;\n\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.IMemento;\nimport org.eclipse.ui.IViewSite;\nimport org.eclipse.ui.IWorkbench;\nimport org.eclipse.ui.PartInitException;\nimport org.eclipse.ui.forms.widgets.FormToolkit;\nimport org.eclipse.ui.forms.widgets.ScrolledForm;\nimport org.eclipse.ui.part.ViewPart;\n\nimport jakarta.inject.Inject;\nimport xy.ai.workbench.Activator;\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.connector.harness.PromptHandler;\nimport xy.ai.workbench.view.session.ActionButtonsSection;\nimport xy.ai.workbench.view.session.InstructionSection;\nimport xy.ai.workbench.view.session.OutputModeSection;\nimport xy.ai.workbench.view.session.TopParametersSection;\nimport xy.ai.workbench.view.session.UsageLogSection;\n\n/**\n * Coordinates the session view's sections; owns only cross-cutting handles\n * (toolkit/form/display) that every section needs.\n */\npublic class AISessionView extends ViewPart {\n\n\t/**\n\t * The ID of the view as specified by the extension.\n\t */\n\tpublic static final String ID = \"xy.ai.workbench.views.AISessionView\";\n\n\tpublic static AISessionView currentInstance;\n\n\t@Inject\n\tIWorkbench workbench;\n\n\tprivate FormToolkit toolkit;\n\tprivate ScrolledForm form;\n\n\tpublic Display display;\n\n\t@Override\n\tpublic void saveState(IMemento memento) {\n\t\tsuper.saveState(memento);\n\t\tActivator.getDefault().cfg.saveConfig(memento);\n\t}\n\n\t@Override\n\tpublic void init(IViewSite site, IMemento memento) throws PartInitException {\n\t\tsuper.init(site, memento);\n\t\tActivator.getDefault().cfg.loadConfig(memento);\n\t}\n\n\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tcurrentInstance = this;\n\t\tdisplay = parent.getDisplay();\n\t\ttoolkit = new FormToolkit(parent.getDisplay());\n\t\tform = toolkit.createScrolledForm(parent);\n\t\tConfigManager cfg = Activator.getDefault().cfg;\n\t\tPromptHandler promptHandler = Activator.getDefault().session.getPromptHandler();\n\n\t\tComposite body = form.getBody();\n\t\tbody.setLayout(new GridLayout());\n\n\t\tnew TopParametersSection(toolkit).create(body, cfg);\n\t\tnew InstructionSection(toolkit, () -> form.reflow(true)).create(body, cfg, promptHandler,\n\t\t\t\tgetSite().getShell());\n\t\tOutputModeSection.create(toolkit, body, cfg);\n\t\tActionButtonsSection.create(body, cfg, promptHandler);\n\t\tUsageLogSection.create(toolkit, body, promptHandler, display);\n\n\t\tpromptHandler.initializeInputs();\n\n\t\tform.reflow(true);\n\t}\n\n\t@Override\n\tpublic void setFocus() {\n\t\t// form.setFocus();\n\t}\n}\n"
  reason: "Extrahierte, nach Widget-Hierarchie geschnittene Klassen im neuen Package anlegen und AISessionView als schlanken Koordinator neu schreiben."
```
/answer 21e8-10 allow
Control Request:
```yaml
id: 21e8-11
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/UpdateGuard.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/Contracts.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/TopParametersSection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/EditTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputsTable.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/OutputModeSection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ActionButtonsSection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/UsageLogSection.java
  reason: Syntaktische Korrektheit aller neuen und geänderten Dateien nach der Extraktion prüfen.
```
/answer 21e8-11 allow
ReasoningToken: 18783
ReasoningToken: 50
Result Stats:  id=7524f0d0-1820-430b-adf1-781ec86a8d2c, total: 0, in: 153, out: 53489, reason: 0, read: 266999, write: 117454
Umsetzung fertig. Struktur:

**`view/AISessionView.java`** – Koordinator: hält nur `toolkit`, `form`, `display` (Cross-Cutting) und verdrahtet die Sections top-down.

**`view/session/`** (neues Package):
- `TopParametersSection` – Key/Model/Profile/MaxToken/Temp/TopP/Reasoning/Cache, inkl. `toggleControl`/`isTemperatureEnabled`
- `InstructionSection` – baut TabFolder, hält `UpdateGuard` und die vier Tabs
  - `SelectTab` (implements `Clearable`) – Zeilenauswahl + FreeText-Area als Kind
  - `EditTab`
  - `ToolsTab` (implements `ToolSelection`)
  - `PresetsTab` – bekommt `ToolSelection` und `Clearable` per Konstruktorparameter statt Feldzugriff
- `InputsTable` – zustandslos, `static create(...)`
- `OutputModeSection`, `ActionButtonsSection` – zustandslos
- `UsageLogSection`
- `UpdateGuard` – ersetzt geteiltes `isUpdating`
- `Contracts.java` – `Clearable`, `ToolSelection`

Alle Dateien syntaktisch validiert (`ast_validate`: `all_ok: true`). Externe Nutzung von `AISessionView.currentInstance.display` bleibt unverändert funktionsfähig (Feld weiterhin `public`).
