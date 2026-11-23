Es gibt hier potenziell eine Lücke im Update der Tools beim Anwenden eines Presetes. Wenn ich das Modell umsetze, werden die Tools gecleared, das ist okay. Wenn ich dann ein Preset mit einem Modell und Tools anwende, geht der Tools Counter hoch auf 3 aber die Tools Liste bleibt leer. Es scheint der interne zustand ist richtig aber es fehlt vielleicht ein GUI Update. 

`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java`
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolSelection.java`
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java`
SystemInit:  id=cf94e197-b5af-474f-bbf2-1fd86eb93368, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 8cf0-1-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolSelection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
  reason: Understand structure of the three files
```
/answer 8cf0-1-0 allow
Control Request:
```yaml
id: 8cf0-2-1
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
        nodes:
        - id: hlGXE3|06lsi9
          excert: import java.util.Arrays;
        - id: ToolsTab
          excert: "/**  * \"Tools\" tab: multi-select list of tools enabled for the current model.  …"
          docstring: "\"Tools\" tab: multi-select list of tools enabled for the current model."
          children:
          - id: ToolsTab.2HWhSr|CcAzu4
            excert: private MultiSelectTable toolsTable;
          - id: ToolsTab.create
            excert: "public void create(TabFolder instr, ConfigManager cfg)"
          - id: ToolsTab.getSelection
            excert: "public String[] getSelection()"
          - id: ToolsTab.setSelection
            excert: "public void setSelection(String[] items)"
          - id: ToolsTab.clear
            excert: public void clear()
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolSelection.java
        nodes:
        - id: ToolSelection
          excert: "/**  * Read/write access to a widget's multi-selection, needed for cross-tab pr…"
          docstring: "Read/write access to a widget's multi-selection, needed for cross-tab preset"
          children:
          - id: ToolSelection.getSelection
            excert: "public abstract String[] getSelection()"
          - id: ToolSelection.setSelection
            excert: "public abstract void setSelection(String[] items)"
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
/answer 8cf0-2-1 allow
Control Request:
```yaml
id: 8cf0-3-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    ids:
    - ToolsTab
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    ids:
    - PresetsTab.create
    - PresetsTab.applySettings
  reason: Read code
```
/answer 8cf0-3-0 allow
Control Request:
```yaml
id: 8cf0-4-3
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
        nodes:
        - id: ToolsTab
          type: ClassOrInterfaceDeclaration
          excert: "/**  * \"Tools\" tab: multi-select list of tools enabled for the current model.  …"
          docstring: "\"Tools\" tab: multi-select list of tools enabled for the current model."
          children:
          - id: ToolsTab.2HWhSr|CcAzu4
            type: StatementGroup
            code: |2
                private MultiSelectTable toolsTable;
          - id: ToolsTab.create
            type: MethodDeclaration
            code: |2
                public void create(TabFolder instr, ConfigManager cfg) {
                  TabItem toolSel = new TabItem(instr, SWT.NONE);
                  toolSel.setText("Tools");
                  Composite comp = new Composite(instr, SWT.NONE);
                  comp.setLayout(new GridLayout());
                  GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
                  ldat3.heightHint = 100;
                  comp.setLayoutData(ldat3);
                  toolSel.setControl(comp);
                  toolsTable = new MultiSelectTable(comp, () -> cfg.setEnabledTools(toolsTable.getSelection()));
                  GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
                  gridData.widthHint = 1;
                  gridData.heightHint = 100;
                  toolsTable.setLayoutData(gridData);
                  cfg.addModelObs(m -> {
                    if (m != null)
                      toolsTable.setItems(m.cap.getTools());
                  }, true);
                  cfg.addEnabledToolsObs(t -> {
                    if (!Arrays.equals(toolsTable.getSelection(), t))
                      toolsTable.setSelection(t);
                  }, true);
                }
          - id: ToolsTab.getSelection
            type: MethodDeclaration
            code: |2
                @Override
                public String[] getSelection() {
                  return toolsTable.getSelection();
                }
          - id: ToolsTab.setSelection
            type: MethodDeclaration
            code: |2
                @Override
                public void setSelection(String[] items) {
                  toolsTable.setSelection(items);
                }
          - id: ToolsTab.clear
            type: MethodDeclaration
            code: |2
                @Override
                public void clear() {
                  toolsTable.clear();
                }
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
/answer 8cf0-4-3 allow
Thinking:
Das Preset setzt die Tools, dann das Modell, und dieser Modellwechsel leert die Tabelle über `toolsTable.setItems(...)`. Ich tausche die Reihenfolge, damit das Modell zuerst gesetzt wird.


ReasoningToken: 202
Control Request:
```yaml
id: 8cf0-5-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.create
    old_text: |2-
            cfg.setSystemPrompt(preset.body);
            if (preset.tools != null) {
    new_text: |2-
            cfg.setSystemPrompt(preset.body);
            applySettings(cfg, preset);
            if (preset.tools != null) {
    exact: false
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.create
    old_text: |2
              rules.setSelection(preset.rules);
            applySettings(cfg, preset);
    new_text: |2
              rules.setSelection(preset.rules);
  reason: Apply model before tools so model change doesn't wipe the tools table
```
/answer 8cf0-5-0 allow
Result Stats: id=cf94e197-b5af-474f-bbf2-1fd86eb93368, total: 12930, in: 8, out: 1789, reason: 0, read: 22749, write: 11133, start: 1795423462029, end: 1795423467384, time: 5288, total: 16166, io: 0, control: 67, sent: 0, recv: 135
Ich habe die Ursache behoben, aber nicht gebaut und nicht getestet. Beim Anwenden eines Presets wurde die Tools-Liste durch den Modellwechsel wieder geleert.

**Ursache:** In `PresetsTab` lief der Preset-Listener in dieser Reihenfolge:
1. `tools.setSelection(...)` und `cfg.setEnabledTools(...)`
2. Danach erst `applySettings(...)`, das `cfg.setModel(...)` aufruft.

Der Modellwechsel ruft in `ToolsTab` den Model-Observer auf. Der führt `toolsTable.setItems(m.cap.getTools())` aus und leert dabei die Auswahl der Tabelle. Der interne Zustand in `cfg` blieb dagegen bei den 3 Tools, deshalb zeigte der Counter 3 bei leerer Liste. Der Enabled-Tools-Observer hat nicht gegengesteuert: `Arrays.equals(toolsTable.getSelection(), t)` prüft nur bei einer Änderung der Tools. Diese kam nach dem Modellwechsel nicht mehr.

**Fix:** In `PresetsTab.create` rufe ich jetzt `applySettings(cfg, preset)` direkt nach `cfg.setSystemPrompt(preset.body)` auf, also vor den Tools. Das Modell und die Tabelleneinträge stehen dann schon fest, wenn die Tools gesetzt werden. Der `applySettings`-Aufruf am Ende des Listeners ist entfernt.

Reasoning, Cache und Profil werden jetzt ebenfalls vor Tools, Output-Mode, Inputs und Rules gesetzt. Das sollte unkritisch sein, aber prüfe es bei einem Preset mit allen Feldern.
