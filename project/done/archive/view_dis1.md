- Profile sollen ausgeblendet werden, wenn sie keinen Eintrag enthalten
- Reasoning soll ausgeblendet werden wenn es kein Reasoning gibt oder der einzigste Eintrag "Default" oder "Disabled" ist.

```json
{
  "results" : [ {
    "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java",
    "nodes" : [ {
      "id" : "YsNcr0",
      "type" : "imports"
    }, {
      "id" : "WTIi3p",
      "type" : "imports"
    }, {
      "id" : "H0MvPR",
      "type" : "imports"
    }, {
      "id" : "KCOjV4",
      "type" : "imports"
    }, {
      "id" : "AISessionView",
      "type" : "class_declaration",
      "signature" : "public class AISessionView extends ViewPart {",
      "children" : [ {
        "id" : "AISessionView.R6IMVC",
        "type" : "statements"
      }, {
        "id" : "AISessionView.2Q8Bc1",
        "type" : "statements"
      }, {
        "id" : "AISessionView.saveState",
        "type" : "method_declaration",
        "signature" : "@Override"
      }, {
        "id" : "AISessionView.init",
        "type" : "method_declaration",
        "signature" : "@Override"
      }, {
        "id" : "AISessionView.createPartControl",
        "type" : "method_declaration",
        "signature" : "@Override"
      }, {
        "id" : "AISessionView.createTopParametersSection",
        "type" : "method_declaration",
        "signature" : "private void createTopParametersSection(Composite body, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createKeyField",
        "type" : "method_declaration",
        "signature" : "private void createKeyField(Composite top, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.Hxfc0y",
        "type" : "statements"
      }, {
        "id" : "AISessionView.createModelCombo",
        "type" : "method_declaration",
        "signature" : "private void createModelCombo(Composite top, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createProfileCombo",
        "type" : "method_declaration",
        "signature" : "private void createProfileCombo(Composite top, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createReasoningSection",
        "type" : "method_declaration",
        "signature" : "private void createReasoningSection(Composite top, Composite body, ConfigManage…"
      }, {
        "id" : "AISessionView.createCacheSection",
        "type" : "method_declaration",
        "signature" : "private void createCacheSection(Composite top, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createInstructionSection",
        "type" : "method_declaration",
        "signature" : "private void createInstructionSection(Composite body, ConfigManager cfg, AISess…"
      }, {
        "id" : "AISessionView.createInstructionSelectTab",
        "type" : "method_declaration",
        "signature" : "private void createInstructionSelectTab(TabFolder instr, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createInstructionEditTab",
        "type" : "method_declaration",
        "signature" : "private void createInstructionEditTab(TabFolder instr, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createToolsTab",
        "type" : "method_declaration",
        "signature" : "private void createToolsTab(TabFolder instr, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createPresetsTab",
        "type" : "method_declaration",
        "signature" : "private void createPresetsTab(TabFolder instr, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createFreeTextArea",
        "type" : "method_declaration",
        "signature" : "private void createFreeTextArea(SashForm sash, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createInputsTable",
        "type" : "method_declaration",
        "signature" : "private void createInputsTable(Composite middle, ConfigManager cfg, AISessionMa…"
      }, {
        "id" : "AISessionView.createOutputModeSection",
        "type" : "method_declaration",
        "signature" : "private void createOutputModeSection(Composite body, ConfigManager cfg) {"
      }, {
        "id" : "AISessionView.createActionButtonsSection",
        "type" : "method_declaration",
        "signature" : "private void createActionButtonsSection(Composite body, ConfigManager cfg, AISe…"
      }, {
        "id" : "AISessionView.createUsageLogSection",
        "type" : "method_declaration",
        "signature" : "private void createUsageLogSection(Composite body, AISessionManager session) {"
      }, {
        "id" : "AISessionView.refreshPresetList",
        "type" : "method_declaration",
        "signature" : "private void refreshPresetList() {"
      }, {
        "id" : "AISessionView.updatePromptLines",
        "type" : "method_declaration",
        "signature" : "private String[] updatePromptLines(String[] cur) {"
      }, {
        "id" : "AISessionView.isTemperatureEnabled",
        "type" : "method_declaration",
        "signature" : "private boolean isTemperatureEnabled(Model m, Reasoning reasoning) {"
      }, {
        "id" : "AISessionView.toogleControl",
        "type" : "method_declaration",
        "signature" : "private void toogleControl(Label label, Control ctrl, boolean enabled) {"
      }, {
        "id" : "AISessionView.updateInstructionList",
        "type" : "method_declaration",
        "signature" : "private void updateInstructionList(String[] systemPrompt) {"
      }, {
        "id" : "AISessionView.updateEditList",
        "type" : "method_declaration",
        "signature" : "private void updateEditList(String[] systemPrompt) {"
      }, {
        "id" : "AISessionView.setFocus",
        "type" : "method_declaration",
        "signature" : "@Override"
      } ]
    } ]
  } ]
}
```
SystemInit:  id=170a8f9f-f2fd-407d-afad-24257f2d4610, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 33a1-f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
    ids:
    - AISessionView.createProfileCombo
    - AISessionView.createReasoningSection
  reason: "Code der Methoden lesen, um Sichtbarkeits-Logik für Profile und Reasoning zu implementieren"
```
/answer 33a1-f allow
Control Request:
```yaml
id: 33a1-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
      nodes:
      - id: AISessionView.createProfileCombo
        type: method_declaration
        code: "private void createProfileCombo(Composite top, ConfigManager cfg) {\n\t\ttoolkit.createLabel(top, \"Profile:\");\n\t\tCombo profileSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tprofileSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tprofileSel.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg\n\t\t\t\t.setProfile(profileSel.getText().isBlank() ? null : AgentProfile.fromName(profileSel.getText()))));\n\t\tcfg.addEnabledProfilesObs(k -> {\n\t\t\tprofileSel\n\t\t\t\t\t.setItems(Arrays.stream(k).map((m) -> m.name).collect(Collectors.toList()).toArray(new String[0]));\n\t\t\tprofileSel.setText(k.length > 0 ? k[0].name : \"\");\n\t\t}, true);\n\t\tcfg.addProfileObs(p -> {\n\t\t\tprofileSel.setText(p != null ? p.name : \"\");\n\t\t}, true);\n\t}"
      - id: AISessionView.createReasoningSection
        type: method_declaration
        code: "private void createReasoningSection(Composite top, Composite body, ConfigManager cfg, Label tempLabel, Text temp,\n\t\t\tLabel topPLabel, Text topP, Label maxTokenLabel, Text maxToken) {\n\t\ttoolkit.createLabel(top, \"Reasoning:\");\n\t\tComposite secReason = new Composite(top, SWT.NONE);\n\t\tGridLayout secRLay = new GridLayout(2, false);\n\t\tsecRLay.marginHeight = secRLay.marginWidth = 0;\n\t\tsecReason.setLayout(secRLay);\n\t\tsecReason.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\n\t\tCombo reasSel = new Combo(secReason, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\treasSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\treasSel.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setReasoning(Reasoning.valueOf(reasSel.getText()))));\n\n\t\tText budget = toolkit.createText(secReason, \"\", SWT.BORDER);\n\t\tbudget.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));\n\t\tbudget.addFocusListener(\n\t\t\t\tFocusListener.focusLostAdapter(e -> cfg.setReasoningBudget(Integer.parseInt(budget.getText()))));\n\t\tbudget.addMouseListener(MouseListener.mouseDownAdapter(m -> budget.setFocus()));\n\t\tcfg.addBudgetObs(bg -> budget.setText(bg + \"\"), true);\n\n\t\tcfg.addModelObs(m -> {\n\t\t\ttoogleControl(tempLabel, temp, isTemperatureEnabled(m, cfg.getReasoning()));\n\t\t\ttoogleControl(topPLabel, topP, m.cap.isSupportTopP());\n\t\t\ttoogleControl(maxTokenLabel, maxToken, m.cap.isSupportMaxToken());\n\n\t\t\treasSel.setItems(cfg.getReasonings());\n\t\t\treasSel.setText(cfg.getReasoning().name());\n\t\t\tbody.layout();\n\t\t}, true);\n\t\tcfg.addReasoningObs(r -> {\n\n\t\t\tboolean enabled = Reasoning.Budget.equals(r);\n\t\t\tbudget.setEnabled(enabled);\n\t\t\tbudget.setVisible(enabled);\n\t\t\t((GridData) budget.getLayoutData()).exclude = !enabled;\n\n\t\t\ttoogleControl(tempLabel, temp, isTemperatureEnabled(cfg.getModel(), r));\n\n\t\t\tsecReason.layout();\n\t\t\tbody.layout();\n\t\t}, true);\n\t}"
```
/answer 33a1-10 allow
ReasoningToken: 32
Control Request:
```yaml
id: 33a1-11
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
    ids:
    - AISessionView.toogleControl
    - AISessionView.createKeyField
  reason: toogleControl-Implementierung prüfen für konsistentes Ausblenden von Label+Control
```
/answer 33a1-11 allow
Control Request:
```yaml
id: 33a1-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
      nodes:
      - id: AISessionView.toogleControl
        type: method_declaration
        code: "private void toogleControl(Label label, Control ctrl, boolean enabled) {\n\t\tlabel.setEnabled(enabled);\n\t\tlabel.setVisible(enabled);\n\t\tctrl.setEnabled(enabled);\n\t\tctrl.setVisible(enabled);\n\t\t((GridData) label.getLayoutData()).exclude = !enabled;\n\t\t((GridData) ctrl.getLayoutData()).exclude = !enabled;\n\t}"
      - id: AISessionView.createKeyField
        type: method_declaration
        code: "private void createKeyField(Composite top, ConfigManager cfg) {\n\t\ttoolkit.createLabel(top, \"Key:\");\n\t\tText keyInput = toolkit.createText(top, \"\", SWT.BORDER | SWT.PASSWORD);\n\t\tGridData kilay = new GridData(GridData.FILL_HORIZONTAL);\n\t\tkilay.widthHint = 10;\n\t\tkeyInput.setLayoutData(kilay);\n\t\tkeyInput.addModifyListener(e -> cfg.setKey(keyInput.getText()));\n\t\tkeyInput.setText(cfg.getKeys() + \"\");\n\t\tkeyInput.addMouseListener(MouseListener.mouseDownAdapter(m -> keyInput.setFocus()));\n\t}"
```
/answer 33a1-12 allow
ReasoningToken: 292
Control Request:
```yaml
id: 33a1-13
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
    id: AISessionView.createProfileCombo
    old_text: "\t\ttoolkit.createLabel(top, \"Profile:\");\n\t\tCombo profileSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tprofileSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));"
    new_text: "\t\tLabel profileLabel = toolkit.createLabel(top, \"Profile:\");\n\t\tCombo profileSel = new Combo(top, SWT.DROP_DOWN | SWT.READ_ONLY);\n\t\tprofileSel.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
    id: AISessionView.createProfileCombo
    old_text: "\t\t\tprofileSel.setText(k.length > 0 ? k[0].name : \"\");\n\t\t}, true);"
    new_text: "\t\t\tprofileSel.setText(k.length > 0 ? k[0].name : \"\");\n\t\t\ttoogleControl(profileLabel, profileSel, k.length > 0);\n\t\t\ttop.layout();\n\t\t}, true);"
  reason: "Profile-Zeile ausblenden, wenn keine Profile vorhanden sind"
```
/answer 33a1-13 allow
ReasoningToken: 217
Control Request:
```yaml
id: 33a1-14
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
    id: AISessionView.createReasoningSection
    old_text: "\t\ttoolkit.createLabel(top, \"Reasoning:\");\n\t\tComposite secReason = new Composite(top, SWT.NONE);"
    new_text: "\t\tLabel reasoningLabel = toolkit.createLabel(top, \"Reasoning:\");\n\t\tComposite secReason = new Composite(top, SWT.NONE);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
    id: AISessionView.createReasoningSection
    old_text: "\t\t\treasSel.setItems(cfg.getReasonings());\n\t\t\treasSel.setText(cfg.getReasoning().name());\n\t\t\tbody.layout();\n\t\t}, true);"
    new_text: "\t\t\tString[] reasonings = cfg.getReasonings();\n\t\t\treasSel.setItems(reasonings);\n\t\t\treasSel.setText(cfg.getReasoning().name());\n\t\t\tboolean hasReasoning = reasonings.length > 0\n\t\t\t\t\t&& !(reasonings.length == 1 && (\"Default\".equals(reasonings[0]) || \"Disabled\".equals(reasonings[0])));\n\t\t\ttoogleControl(reasoningLabel, secReason, hasReasoning);\n\t\t\tbody.layout();\n\t\t}, true);"
  reason: "Reasoning-Zeile ausblenden, wenn kein Reasoning oder nur Default/Disabled vorhanden ist"
```
/answer 33a1-14 allow
Text:
 Jetzt Reasoning-Sektion anpassen:
Control Request:
```yaml
id: 33a1-15
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
  reason: Kompilierbarkeit nach Änderungen prüfen
```
/answer 33a1-15 allow
Result Stats:  id=170a8f9f-f2fd-407d-afad-24257f2d4610, total: 0, in: 143, out: 2463, reason: 0, read: 53754, write: 13169
Änderungen erfolgreich umgesetzt und validiert.
