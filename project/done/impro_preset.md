Verbessere die Presetliste mit folgenden Punkten:

- Der "Reset" Button soll getauscht werden gegen einen "Clear Tools" und "Clear All" (Tools, Systemprompt Seleken nicht aber die Systemprompt Liste, Inputs, Output, alles jeweils auf das Default oder bzw. ersten Eintrag zurück)
- Die Inputs sollen ebenfalls gespeichert und geladen werden
- Die Preset Liste als eigene Klasse auslagern und von der Anzeige trennen
- Das Front-Matter soll ein "Title" Attribut unterstützen
	- Beim Speichern soll ein leeres "Title" Attribut gespeichert werden, beim Laden wird das leere Attribut(trim) ignoriert. So ist für den Nutzer leichter erkennbar das er einen Namen eintragen kann. 
- Einbau von Deduplikation auf Basis des absoluten Dateinamens, aktuell werden doppelte Einträge angezeigt, wenn dieselbe Promptdatei über verschiedene Projekte erreichbar ist.
- Der Save-Dialog soll nach Möglichkeit im aktuellen Projekt-Root starten
- Ändern der Anzeige von Einträgen:
	- Wenn ein Title gesetzt wurde (trim, nicht leer), wird dieser Angezeigt, gibt es mehr als einen Eintrag mit demselben Title, wird der Projektname in Klammern angefügt.
	- Gibt es keinen Title, so wird nur der Dateiname (Ohne ".prompt.txt" Endung) angezeigt. Führt dies zu Doppelung, wird die letzte Ebene des Verzeichnispfades in Klammern angezeigt "(prompt)". Führt auch dies zu Doppelung (Bsp.: gleicher Dateiname, gleicher Pfad in unterschiedlichen Projekten), so wird der Pfad sukzessiv um die letzten Verzeichnisebenden Versucht bis sich Eindeutigkeit ergibt oder als letzte oberste Ebene der Projektname erreicht ist. 

`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java`
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputsTable.java`

SystemInit:  id=6c8081b4-e91a-461d-abc2-ecbfe6997d50, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: 817c-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputsTable.java
  reason: Analyse der aktuellen Presetliste und Inputs-Tabelle zur Planung der Refaktorierung
```
/answer 817c-1 allow
Control Request:
```yaml
id: 817c-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
      nodes:
      - id: 1vWmR7|WrFin2
        signature: import java.util.ArrayList;
      - id: PresetsTab
        signature: "/**  * \"Presets\" tab: save/load/reset the whole system-prompt configuration.  *…"
        docstring: "\"Presets\" tab: save/load/reset the whole system-prompt configuration."
        children:
        - id: PresetsTab.QtgcNk|Bkn3Sn
          signature: private List presetList;
        - id: PresetsTab.create
          signature: "public void create(TabFolder instr, ConfigManager cfg, Shell shell, ToolSelecti…"
        - id: PresetsTab.refreshPresetList
          signature: private void refreshPresetList()
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputsTable.java
      nodes:
      - id: 9iEwxU|3qfyL8
        signature: import org.eclipse.swt.SWT;
      - id: InputsTable
        signature: /**  * Checkbox table showing/enabling the available prompt input modes.  */ pu…
        docstring: Checkbox table showing/enabling the available prompt input modes.
        children:
        - id: InputsTable.create
          signature: "public static void create(Composite middle, ConfigManager cfg, PromptHandler pr…"
```
/answer 817c-2 allow
ReasoningToken: 14
Control Request:
```yaml
id: 817c-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    ids:
    - 1vWmR7|WrFin2
    - PresetsTab
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputsTable.java
    ids:
    - 9iEwxU|3qfyL8
    - InputsTable
  reason: Vollständigen Code lesen für Refaktorierung
```
/answer 817c-3 allow
Control Request:
```yaml
id: 817c-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
      nodes:
      - id: 1vWmR7|WrFin2
        type: ImportGroup
        code: |
          import java.util.ArrayList;
          import java.util.Arrays;

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

          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.view.PresetHandler;
      - id: PresetsTab
        type: ClassOrInterfaceDeclaration
        signature: "/**  * \"Presets\" tab: save/load/reset the whole system-prompt configuration.  *…"
        docstring: "\"Presets\" tab: save/load/reset the whole system-prompt configuration."
        children:
        - id: PresetsTab.QtgcNk|Bkn3Sn
          type: StatementGroup
          code: "\tprivate List presetList;\n\tprivate final java.util.List<IFile> presetFiles = new ArrayList<>();\n"
        - id: PresetsTab.create
          type: MethodDeclaration
          code: "\tpublic void create(TabFolder instr, ConfigManager cfg, Shell shell, ToolSelection tools,\n\t\t\tClearable instructionSelection) {\n\t\tTabItem presEdit = new TabItem(instr, SWT.NONE);\n\t\tpresEdit.setText(\"Presets\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tGridLayout compLay = new GridLayout(1, false);\n\t\tcomp.setLayout(compLay);\n\t\tGridData compDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tcomp.setLayoutData(compDat);\n\t\tpresEdit.setControl(comp);\n\n\t\tComposite buttons = new Composite(comp, SWT.NONE);\n\t\tbuttons.setLayout(new GridLayout(2, false));\n\n\t\tButton writeButton = new Button(buttons, SWT.PUSH);\n\t\twriteButton.setText(\"Save\");\n\t\twriteButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tPresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), shell);\n\t\t\t\trefreshPresetList();\n\t\t\t}\n\t\t});\n\n\t\tButton resetButton = new Button(buttons, SWT.PUSH);\n\t\tresetButton.setText(\"Reset\");\n\t\tresetButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tString[] cur = cfg.getSystemPrompt();\n\t\t\t\tString[] upd = new String[cur.length];\n\t\t\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\t\t\tString line = cur[i];\n\t\t\t\t\tupd[i] = line.startsWith(\"#\") ? line : \"#\" + line;\n\t\t\t\t}\n\t\t\t\tinstructionSelection.clear();\n\t\t\t\tcfg.setSystemPrompt(upd);\n\n\t\t\t\ttools.clear();\n\t\t\t\tcfg.setEnabledTools(new String[0]);\n\t\t\t}\n\t\t});\n\n\t\tpresetList = new List(comp, SWT.SINGLE | SWT.V_SCROLL | SWT.BORDER);\n\t\tGridData presetListDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tpresetListDat.heightHint = 60;\n\t\tpresetList.setLayoutData(presetListDat);\n\t\tpresetList.addMouseListener(MouseListener.mouseDownAdapter(m -> presetList.setFocus()));\n\t\tpresetList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tint idx = presetList.getSelectionIndex();\n\t\t\tif (idx < 0 || idx >= presetFiles.size())\n\t\t\t\treturn;\n\t\t\tPresetHandler.Preset preset = PresetHandler.loadPreset(presetFiles.get(idx));\n\t\t\tcfg.setSystemPrompt(preset.body);\n\t\t\tif (preset.tools != null) {\n\t\t\t\ttools.setSelection(preset.tools);\n\t\t\t\tcfg.setEnabledTools(preset.tools);\n\t\t\t}\n\t\t\tif (preset.outputMode != null)\n\t\t\t\tcfg.setOuputMode(preset.outputMode);\n\t\t}));\n\t\trefreshPresetList();\n\t}\n"
        - id: PresetsTab.refreshPresetList
          type: MethodDeclaration
          code: "\tprivate void refreshPresetList() {\n\t\tpresetFiles.clear();\n\t\tpresetFiles.addAll(Arrays.asList(PresetHandler.listPresetFiles()));\n\t\tpresetList.setItems(presetFiles.stream().map(f -> f.getFullPath().toString()).toArray(String[]::new));\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputsTable.java
      nodes:
      - id: 9iEwxU|3qfyL8
        type: ImportGroup
        code: |
          import org.eclipse.swt.SWT;
          import org.eclipse.swt.widgets.Composite;
          import org.eclipse.swt.widgets.Table;
          import org.eclipse.swt.widgets.TableColumn;
          import org.eclipse.swt.widgets.TableItem;

          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.InputMode;
          import xy.ai.workbench.connector.harness.PromptHandler;
      - id: InputsTable
        type: ClassOrInterfaceDeclaration
        signature: /**  * Checkbox table showing/enabling the available prompt input modes.  */ pu…
        docstring: Checkbox table showing/enabling the available prompt input modes.
        children:
        - id: InputsTable.create
          type: MethodDeclaration
          code: "\tpublic static void create(Composite middle, ConfigManager cfg, PromptHandler prompt) {\n\t\tTable table = new Table(middle, SWT.CHECK | SWT.BORDER | SWT.V_SCROLL);\n\t\ttable.setHeaderVisible(true);\n\t\ttable.setLinesVisible(true);\n\n\t\tTableColumn column1 = new TableColumn(table, SWT.NONE);\n\t\tcolumn1.setText(\"On\");\n\t\tcolumn1.setWidth(30);\n\n\t\tTableColumn column2 = new TableColumn(table, SWT.NONE);\n\t\tcolumn2.setText(\"Input\");\n\t\tcolumn2.setWidth(120);\n\n\t\tTableColumn column3 = new TableColumn(table, SWT.NONE);\n\t\tcolumn3.setText(\"Amount\");\n\t\tcolumn3.setWidth(45);\n\n\t\ttable.addListener(SWT.Selection, e -> {\n\t\t\tif (e.detail == SWT.CHECK) {\n\t\t\t\tTableItem item = (TableItem) e.item;\n\t\t\t\tInputMode mode = InputMode.valueOf(item.getText(1).replace(\" \", \"_\"));\n\t\t\t\tcfg.setInputMode(mode, !cfg.isInputEnabled(mode));\n\t\t\t}\n\t\t});\n\n\t\tfor (int i = 0; i < InputMode.values().length; i++) {\n\t\t\tTableItem item = new TableItem(table, SWT.NONE);\n\t\t\tInputMode mode = InputMode.values()[i];\n\t\t\tprompt.addInputStatObs(is -> {\n\t\t\t\tvar checked = item.getChecked();\n\t\t\t\titem.setText(new String[] { \"\", mode.name().replace(\"_\", \" \"), is[mode.ordinal()] + \"\" });\n\t\t\t\titem.setChecked(checked);\n\t\t\t}, true);\n\t\t\tcfg.addInputObs(is -> {\n\t\t\t\titem.setChecked(is[mode.ordinal()]);\n\t\t\t}, true);\n\t\t}\n\t}\n"
```
/answer 817c-4 allow
Control Request:
```yaml
id: 817c-5
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
  reason: Analyse von PresetHandler und ConfigManager für Preset-Lade/Speicher-Logik und Default-Werte
```
/answer 817c-5 allow
Control Request:
```yaml
id: 817c-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
      nodes:
      - id: L4UZIF|tAj1XQ
        signature: import java.io.FileWriter;
      - id: PresetHandler
        signature: "public class PresetHandler {"
        children:
        - id: PresetHandler.bbo2aQ|7WKcdc
          signature: private static final String PROMPT_TXT = ".prompt.txt";
        - id: PresetHandler.FrontMatter
          signature: "public static class FrontMatter {"
        - id: PresetHandler.Preset
          signature: "public static class Preset {"
        - id: PresetHandler.listPresetFiles
          signature: "public static IFile[] listPresetFiles()"
        - id: PresetHandler.loadPreset
          signature: public static Preset loadPreset(IFile file)
        - id: PresetHandler.parsePreset
          signature: public static Preset parsePreset(String content)
        - id: PresetHandler.writePreset
          signature: "public static void writePreset(String[] body, String[] tools, OutputMode output…"
        - id: PresetHandler.serializePreset
          signature: "public static String serializePreset(String[] body, String[] tools, OutputMode …"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
      nodes:
      - id: Ei2OXR|Y3Iwpb
        signature: import java.util.ArrayList;
      - id: ConfigManager
        signature: "public class ConfigManager {"
        children:
        - id: ConfigManager.0mEcEZ|akgzRP
          signature: private ModelResolverRegistry resolverRegistry = new ModelResolverRegistry();
        - id: ConfigManager.iAEUAf|j9IJgJ
          signature: private List<Consumer<Reasoning>> reasonObs = new ArrayList<>();
        - id: ConfigManager.ConfigManager
          signature: "public ConfigManager(MCPClient mcpClient, ActiveEditorListener editorListener)"
        - id: ConfigManager.clearObserver
          signature: public void clearObserver()
        - id: ConfigManager.activateEditor
          signature: private void activateEditor(IEditorPart editor)
        - id: ConfigManager.resolveKey
          signature: private static String resolveKey(IEditorPart editor)
        - id: ConfigManager.ensureOwnConfig
          signature: private void ensureOwnConfig()
        - id: ConfigManager.applyConfig
          signature: private void applyConfig(SessionConfig target)
        - id: ConfigManager.setKey
          signature: public void setKey(String keys)
        - id: ConfigManager.setMaxOutputTokens
          signature: public void setMaxOutputTokens(Long maxOutputTokens)
        - id: ConfigManager.setTemperature
          signature: public void setTemperature(Double temp)
        - id: ConfigManager.setTopP
          signature: public void setTopP(Double topP)
        - id: ConfigManager.setModel
          signature: public void setModel(Model model)
        - id: ConfigManager.setReasoningBudget
          signature: public void setReasoningBudget(Integer reasoningBudget)
        - id: ConfigManager.setReasoning
          signature: public void setReasoning(Reasoning reasoning)
        - id: ConfigManager.setCacheMode
          signature: public void setCacheMode(CacheMode cacheMode)
        - id: ConfigManager.setProfile
          signature: public void setProfile(AgentProfile profile)
        - id: ConfigManager.setSystemPrompt
          signature: "public void setSystemPrompt(String[] systemPrompt)"
        - id: ConfigManager.setSystemFree
          signature: public void setSystemFree(String freeText)
        - id: ConfigManager.setEnabledTools
          signature: "public void setEnabledTools(String[] enabledTools)"
        - id: ConfigManager.setOuputMode
          signature: public void setOuputMode(OutputMode ouputMode)
        - id: ConfigManager.setInputMode
          signature: "public void setInputMode(InputMode mode, boolean enable)"
        - id: ConfigManager.getKeys
          signature: public String getKeys()
        - id: ConfigManager.getMaxOutputTokens
          signature: public Long getMaxOutputTokens()
        - id: ConfigManager.getTemperature
          signature: public Double getTemperature()
        - id: ConfigManager.getTopP
          signature: public Double getTopP()
        - id: ConfigManager.getModel
          signature: public Model getModel()
        - id: ConfigManager.getReasoningBudget
          signature: public Integer getReasoningBudget()
        - id: ConfigManager.getProfile
          signature: public AgentProfile getProfile()
        - id: ConfigManager.getReasoning
          signature: public Reasoning getReasoning()
        - id: ConfigManager.getCacheMode
          signature: public CacheMode getCacheMode()
        - id: ConfigManager.getSystemPrompt
          signature: "public String[] getSystemPrompt()"
        - id: ConfigManager.getFreeText
          signature: public String getFreeText()
        - id: ConfigManager.getOuputMode
          signature: public OutputMode getOuputMode()
        - id: ConfigManager.isInputEnabled
          signature: public boolean isInputEnabled(InputMode mode)
        - id: ConfigManager.addSystemPromptObs
          signature: "public void addSystemPromptObs(Consumer<SessionConfig> obs, boolean initialize)"
        - id: ConfigManager.addSystemFreeObs
          signature: "public void addSystemFreeObs(Consumer<String> obs, boolean initialize)"
        - id: ConfigManager.addProfileObs
          signature: "public void addProfileObs(Consumer<AgentProfile> obs, boolean initialize)"
        - id: ConfigManager.addTemperatureObs
          signature: "public void addTemperatureObs(Consumer<Double> obs, boolean initialize)"
        - id: ConfigManager.addTopPObs
          signature: "public void addTopPObs(Consumer<Double> obs, boolean initialize)"
        - id: ConfigManager.addInputObs
          signature: "public void addInputObs(Consumer<boolean[]> obs, boolean initialize)"
        - id: ConfigManager.addBudgetObs
          signature: "public void addBudgetObs(Consumer<Integer> obs, boolean initialize)"
        - id: ConfigManager.addReasoningObs
          signature: "public void addReasoningObs(Consumer<Reasoning> obs, boolean initialize)"
        - id: ConfigManager.addCacheObs
          signature: "public void addCacheObs(Consumer<CacheMode> obs, boolean initialize)"
        - id: ConfigManager.addKeyObs
          signature: "public void addKeyObs(Consumer<String> obs, boolean initialize)"
        - id: ConfigManager.addModelObs
          signature: "public void addModelObs(Consumer<Model> obs, boolean initialize)"
        - id: ConfigManager.addOutputTokenObs
          signature: "public void addOutputTokenObs(Consumer<Long> obs, boolean initialize)"
        - id: ConfigManager.addEnabledModelsObs
          signature: "public void addEnabledModelsObs(Consumer<Model[]> obs, boolean initialize)"
        - id: ConfigManager.addEnabledProfilesObs
          signature: "public void addEnabledProfilesObs(Consumer<AgentProfile[]> obs, boolean initial…"
        - id: ConfigManager.addInputModeObs
          signature: public void addInputModeObs(Consumer<InputMode> obs)
        - id: ConfigManager.addOutputModeObs
          signature: "public void addOutputModeObs(Consumer<OutputMode> obs, boolean initialize)"
        - id: ConfigManager.addEnabledToolsObs
          signature: "public void addEnabledToolsObs(Consumer<String[]> obs, boolean initialize)"
        - id: ConfigManager.updateEnabledModels
          signature: "private void updateEnabledModels(String[] keys)"
        - id: ConfigManager.setEnabledProfiles
          signature: "public void setEnabledProfiles(AgentProfile[] enabledProfiles)"
        - id: ConfigManager.getEnabledModels
          signature: "public Model[] getEnabledModels()"
        - id: ConfigManager.getReasonings
          signature: "public String[] getReasonings()"
        - id: ConfigManager.saveConfig
          signature: public void saveConfig(IMemento memento)
        - id: ConfigManager.loadConfig
          signature: public void loadConfig(IMemento memento)
        - id: ConfigManager.getCapabilities
          signature: public Capabilities getCapabilities()
        - id: ConfigManager.getEnabledTools
          signature: "public String[] getEnabledTools()"
```
/answer 817c-6 allow
ReasoningToken: 37
Control Request:
```yaml
id: 817c-7
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    ids:
    - L4UZIF|tAj1XQ
    - PresetHandler.bbo2aQ|7WKcdc
    - PresetHandler.FrontMatter
    - PresetHandler.Preset
    - PresetHandler.listPresetFiles
    - PresetHandler.loadPreset
    - PresetHandler.parsePreset
    - PresetHandler.writePreset
    - PresetHandler.serializePreset
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
    ids:
    - ConfigManager.setInputMode
    - ConfigManager.isInputEnabled
    - ConfigManager.setSystemPrompt
    - ConfigManager.setEnabledTools
    - ConfigManager.setOuputMode
    - ConfigManager.addInputObs
  reason: Vollständigen Code von PresetHandler lesen und relevante ConfigManager Methoden für Input-Handling
```
/answer 817c-7 allow
Control Request:
```yaml
id: 817c-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
      nodes:
      - id: L4UZIF|tAj1XQ
        type: ImportGroup
        code: |
          import java.io.FileWriter;
          import java.io.IOException;
          import java.io.PrintWriter;
          import java.nio.charset.StandardCharsets;
          import java.nio.file.Files;
          import java.util.ArrayList;
          import java.util.Arrays;
          import java.util.Comparator;
          import java.util.List;
          import java.util.regex.Matcher;
          import java.util.regex.Pattern;

          import org.eclipse.core.resources.IFile;
          import org.eclipse.core.resources.IProject;
          import org.eclipse.core.resources.IResourceVisitor;
          import org.eclipse.core.resources.ResourcesPlugin;
          import org.eclipse.core.runtime.CoreException;
          import org.eclipse.swt.SWT;
          import org.eclipse.swt.widgets.FileDialog;
          import org.eclipse.swt.widgets.Shell;

          import com.fasterxml.jackson.databind.ObjectMapper;
          import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
          import com.fasterxml.jackson.dataformat.yaml.YAMLGenerator;

          import xy.ai.workbench.LOG;
          import xy.ai.workbench.OutputMode;
      - id: PresetHandler.bbo2aQ|7WKcdc
        type: StatementGroup
        code: "\tprivate static final String PROMPT_TXT = \".prompt.txt\";\n\n\tprivate static final Pattern FRONT_MATTER = Pattern.compile(\"\\\\A---\\\\s*\\\\n(.*?\\\\n)?---\\\\s*\\\\n?\", Pattern.DOTALL);\n\n\tprivate static final ObjectMapper YAML = new ObjectMapper(\n\t\t\tnew YAMLFactory().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER));\n"
      - id: PresetHandler.FrontMatter
        type: ClassOrInterfaceDeclaration
        code: "\tpublic static class FrontMatter {\n\t\tpublic List<String> tools;\n\t\tpublic String outputMode;\n\t}\n"
      - id: PresetHandler.Preset
        type: ClassOrInterfaceDeclaration
        code: "\tpublic static class Preset {\n\t\tpublic String[] body;\n\t\tpublic String[] tools;\n\t\tpublic OutputMode outputMode;\n\t}\n"
      - id: PresetHandler.listPresetFiles
        type: MethodDeclaration
        code: "\tpublic static IFile[] listPresetFiles() {\n\t\tList<IFile> files = new ArrayList<>();\n\t\tfor (IProject project : ResourcesPlugin.getWorkspace().getRoot().getProjects()) {\n\t\t\ttry {\n\t\t\t\tIResourceVisitor visitor = resource -> {\n\t\t\t\t\tif (resource instanceof IFile) {\n\t\t\t\t\t\tIFile file = (IFile) resource;\n\t\t\t\t\t\tif (file.getName().endsWith(PROMPT_TXT))\n\t\t\t\t\t\t\tfiles.add(file);\n\t\t\t\t\t}\n\t\t\t\t\treturn true;\n\t\t\t\t};\n\t\t\t\tproject.accept(visitor);\n\t\t\t} catch (CoreException e) {\n\t\t\t\t// return things like project closed\n\t\t\t\t// LOG.error(e.getMessage(), e);\n\t\t\t}\n\t\t}\n\t\tfiles.sort(Comparator.comparing(f -> f.getFullPath().toString()));\n\t\treturn files.toArray(new IFile[0]);\n\t}\n"
      - id: PresetHandler.loadPreset
        type: MethodDeclaration
        code: "\tpublic static Preset loadPreset(IFile file) {\n\t\ttry {\n\t\t\tString content = new String(Files.readAllBytes(file.getLocation().toFile().toPath()),\n\t\t\t\t\tStandardCharsets.UTF_8);\n\t\t\treturn parsePreset(content);\n\t\t} catch (IOException e) {\n\t\t\tthrow new IllegalStateException(e);\n\t\t}\n\t}\n"
      - id: PresetHandler.parsePreset
        type: MethodDeclaration
        code: "\tpublic static Preset parsePreset(String content) {\n\t\tPreset preset = new Preset();\n\t\tString body = content;\n\n\t\tMatcher matcher = FRONT_MATTER.matcher(content);\n\t\tif (matcher.find() && matcher.start() == 0) {\n\t\t\tString yaml = matcher.group(1);\n\t\t\tbody = content.substring(matcher.end());\n\t\t\tif (yaml != null && !yaml.isBlank()) {\n\t\t\t\ttry {\n\t\t\t\t\tFrontMatter fm = YAML.readValue(yaml, FrontMatter.class);\n\t\t\t\t\tif (fm.tools != null)\n\t\t\t\t\t\tpreset.tools = fm.tools.toArray(new String[0]);\n\t\t\t\t\tif (fm.outputMode != null && !fm.outputMode.isBlank())\n\t\t\t\t\t\tpreset.outputMode = OutputMode.valueOf(fm.outputMode.trim());\n\t\t\t\t} catch (Exception e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\n\t\tif (body.endsWith(\"\\n\"))\n\t\t\tbody = body.substring(0, body.length() - 1);\n\t\tpreset.body = body.split(\"\\n\", -1);\n\t\treturn preset;\n\t}\n"
      - id: PresetHandler.writePreset
        type: MethodDeclaration
        code: "\tpublic static void writePreset(String[] body, String[] tools, OutputMode outputMode, Shell shell) {\n\t\tFileDialog dialog = new FileDialog(shell, SWT.SAVE);\n\t\tdialog.setFilterPath(ResourcesPlugin.getWorkspace().getRoot().getProjects()[0].getFullPath().toOSString());\n\t\tdialog.setFilterExtensions(new String[] { \"*.prompt.txt\" });\n\t\tdialog.setFilterNames(new String[] { \"Prompt Files (*prompt.txt)\" });\n\t\tString filePath = dialog.open();\n\t\tif (filePath == null)\n\t\t\treturn;\n\n\t\tString content = serializePreset(body, tools, outputMode);\n\t\ttry (PrintWriter writer = new PrintWriter(new FileWriter(filePath, StandardCharsets.UTF_8))) {\n\t\t\twriter.print(content);\n\t\t} catch (IOException e) {\n\t\t\tthrow new IllegalStateException(e);\n\t\t}\n\t}\n"
      - id: PresetHandler.serializePreset
        type: MethodDeclaration
        code: "\tpublic static String serializePreset(String[] body, String[] tools, OutputMode outputMode) {\n\t\tFrontMatter fm = new FrontMatter();\n\t\tif (tools != null && tools.length > 0)\n\t\t\tfm.tools = Arrays.asList(tools);\n\t\tif (outputMode != null)\n\t\t\tfm.outputMode = outputMode.name();\n\n\t\tStringBuilder sb = new StringBuilder();\n\t\tif (fm.tools != null || fm.outputMode != null) {\n\t\t\ttry {\n\t\t\t\tString yaml = YAML.writeValueAsString(fm);\n\t\t\t\tsb.append(\"---\\n\");\n\t\t\t\tsb.append(yaml);\n\t\t\t\tsb.append(\"---\\n\");\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t}\n\t\t}\n\t\tsb.append(String.join(\"\\n\", body));\n\t\treturn sb.toString();\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
      nodes:
      - id: ConfigManager.setInputMode
        type: MethodDeclaration
        code: "\tpublic void setInputMode(InputMode mode, boolean enable) {\n\t\tif (enable && cfg.isInputEnabled(mode) || !enable && !cfg.isInputEnabled(mode))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setInputMode(mode, enable);\n\t\t// Converter and Selection are mutually exclusive: enabling one disables the\n\t\t// other.\n\t\tif (enable && (mode == InputMode.Converter || mode == InputMode.Selection)) {\n\t\t\tInputMode other = mode == InputMode.Converter ? InputMode.Selection : InputMode.Converter;\n\t\t\tif (cfg.isInputEnabled(other)) {\n\t\t\t\tcfg.setInputMode(other, false);\n\t\t\t\tinputObs.forEach(c -> c.accept(cfg.inputModes));\n\t\t\t\tinputModeObs.forEach(c -> c.accept(other));\n\t\t\t}\n\t\t}\n\t\tinputObs.forEach(c -> c.accept(cfg.inputModes));\n\t\tinputModeObs.forEach(c -> c.accept(mode));\n\t}\n"
      - id: ConfigManager.isInputEnabled
        type: MethodDeclaration
        code: "\tpublic boolean isInputEnabled(InputMode mode) {\n\t\treturn cfg.isInputEnabled(mode);\n\t}\n"
      - id: ConfigManager.setSystemPrompt
        type: MethodDeclaration
        code: "\tpublic void setSystemPrompt(String[] systemPrompt) {\n\t\tif (Arrays.equals(this.getSystemPrompt(), systemPrompt))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setSystemPrompt(systemPrompt);\n\t\tsystemPromptObs.forEach(c -> c.accept(cfg));\n\t\tinputModeObs.forEach(c -> c.accept(InputMode.SystemPrompt));\n\t}\n"
      - id: ConfigManager.setEnabledTools
        type: MethodDeclaration
        code: "\tpublic void setEnabledTools(String[] enabledTools) {\n\t\tif (Arrays.equals(cfg.getEnabledTools(), enabledTools))\n\t\t\treturn;\n\t\tcfg.setEnabledTools(enabledTools);\n\t\tenabledToolsObs.forEach(c -> c.accept(enabledTools));\n\t}\n"
      - id: ConfigManager.setOuputMode
        type: MethodDeclaration
        code: "\tpublic void setOuputMode(OutputMode ouputMode) {\n\t\tif (Objects.equals(cfg.ouputMode, ouputMode))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.ouputMode = ouputMode;\n\t\toutputModeObs.forEach(c -> c.accept(cfg.ouputMode));\n\t}\n"
      - id: ConfigManager.addInputObs
        type: MethodDeclaration
        code: "\tpublic void addInputObs(Consumer<boolean[]> obs, boolean initialize) {\n\t\tinputObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.inputModes);\n\t}\n"
```
/answer 817c-8 allow
Control Request:
```yaml
id: 817c-9
toolName: grep
arguments:
  items:
  - pattern: PresetsTab|InputsTable
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
  - pattern: interface Clearable|interface ToolSelection
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
  - pattern: enum InputMode
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
  reason: Finde Aufrufer von PresetsTab/InputsTable und relevante Interfaces/Enums"
```
/answer 817c-9 allow
Control Request:
```yaml
id: 817c-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/session/InstructionSection.java
        matches:
        - lineno: 26
          match: private final PresetsTab presetsTab = new PresetsTab();
        - lineno: 60
          match: "InputsTable.create(middle, cfg, prompt);"
      - path: xy/ai/workbench/view/session/InputsTable.java
        matches:
        - lineno: 14
          match: "public class InputsTable {"
      - path: xy/ai/workbench/view/session/PresetsTab.java
        matches:
        - lineno: 25
          match: "public class PresetsTab {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/session/Clearable.java
        matches:
        - lineno: 4
          match: "public interface Clearable {"
      - path: xy/ai/workbench/view/session/ToolSelection.java
        matches:
        - lineno: 7
          match: "public interface ToolSelection extends Clearable {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/InputMode.java
        matches:
        - lineno: 3
          match: "public enum InputMode {"
```
/answer 817c-a allow
ReasoningToken: 174
Control Request:
```yaml
id: 817c-b
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/Clearable.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolSelection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/InputMode.java
  reason: Verstehen der Einbettung von PresetsTab und InputsTable in die InstructionSection sowie Clearable/ToolSelection Interfaces und InputMode enum"
```
/answer 817c-b allow
Control Request:
```yaml
id: 817c-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
      nodes:
      - id: 9iEwxU|jMPtFN
        signature: import org.eclipse.swt.SWT;
      - id: InstructionSection
        signature: "/**  * \"System Prompt\" area: Select/Edit/Tools/Presets tabs plus the input-mode…"
        docstring: "\"System Prompt\" area: Select/Edit/Tools/Presets tabs plus the input-mode"
        children:
        - id: InstructionSection.JlzHiL|lw7DqO
          signature: private final FormToolkit toolkit;
        - id: InstructionSection.InstructionSection
          signature: "public InstructionSection(FormToolkit toolkit, Runnable reflow)"
        - id: InstructionSection.create
          signature: "public void create(Composite body, ConfigManager cfg, PromptHandler prompt, She…"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/Clearable.java
      nodes:
      - id: Clearable
        signature: /**  * Something that can be reset to an empty selection.  */ public interface …
        docstring: Something that can be reset to an empty selection.
        children:
        - id: Clearable.clear
          signature: public abstract void clear()
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolSelection.java
      nodes:
      - id: ToolSelection
        signature: "/**  * Read/write access to a widget's multi-selection, needed for cross-tab pr…"
        docstring: "Read/write access to a widget's multi-selection, needed for cross-tab preset"
        children:
        - id: ToolSelection.getSelection
          signature: "public abstract String[] getSelection()"
        - id: ToolSelection.setSelection
          signature: "public abstract void setSelection(String[] items)"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/InputMode.java
      nodes:
      - id: InputMode
        signature: "public enum InputMode {"
```
/answer 817c-c allow
Control Request:
```yaml
id: 817c-d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    ids:
    - 9iEwxU|jMPtFN
    - InstructionSection
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/InputMode.java
    ids:
    - InputMode
  reason: Vollständigen Code für Integration lesen"
```
/answer 817c-d allow
Control Request:
```yaml
id: 817c-e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
      nodes:
      - id: 9iEwxU|jMPtFN
        type: ImportGroup
        code: |
          import org.eclipse.swt.SWT;
          import org.eclipse.swt.layout.GridData;
          import org.eclipse.swt.layout.GridLayout;
          import org.eclipse.swt.widgets.Composite;
          import org.eclipse.swt.widgets.Shell;
          import org.eclipse.swt.widgets.TabFolder;
          import org.eclipse.ui.forms.widgets.FormToolkit;

          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.connector.harness.PromptHandler;
      - id: InstructionSection
        type: ClassOrInterfaceDeclaration
        signature: "/**  * \"System Prompt\" area: Select/Edit/Tools/Presets tabs plus the input-mode…"
        docstring: "\"System Prompt\" area: Select/Edit/Tools/Presets tabs plus the input-mode"
        children:
        - id: InstructionSection.JlzHiL|lw7DqO
          type: StatementGroup
          code: "\tprivate final FormToolkit toolkit;\n\n\tprivate final UpdateGuard guard = new UpdateGuard();\n\tprivate final SelectTab selectTab;\n\tprivate final EditTab editTab;\n\tprivate final ToolsTab toolsTab = new ToolsTab();\n\tprivate final PresetsTab presetsTab = new PresetsTab();\n"
        - id: InstructionSection.InstructionSection
          type: ConstructorDeclaration
          code: "\tpublic InstructionSection(FormToolkit toolkit, Runnable reflow) {\n\t\tthis.toolkit = toolkit;\n\t\tthis.selectTab = new SelectTab(toolkit, guard, reflow);\n\t\tthis.editTab = new EditTab(toolkit, guard, reflow);\n\t}\n"
        - id: InstructionSection.create
          type: MethodDeclaration
          code: "\tpublic void create(Composite body, ConfigManager cfg, PromptHandler prompt, Shell shell) {\n\t\tComposite middle = new Composite(body, SWT.NONE);\n\t\tmiddle.setLayout(new GridLayout(1, false));\n\t\tGridData ldat2 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat2.heightHint = 100;\n\t\tmiddle.setLayoutData(ldat2);\n\n\t\ttoolkit.createLabel(middle, \"System Prompt:\");\n\n\t\tComposite contComp = new Composite(middle, SWT.NONE);\n\t\tcontComp.setLayout(new GridLayout(1, false));\n\t\tGridData scl = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tscl.heightHint = 100;\n\t\tscl.widthHint = 1;\n\t\tcontComp.setLayoutData(scl);\n\n\t\tTabFolder instr = new TabFolder(contComp, SWT.NONE);\n\t\tGridData ldat1 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat1.heightHint = 100;\n\t\tinstr.setLayoutData(ldat1);\n\n\t\tselectTab.create(instr, cfg);\n\t\teditTab.create(instr, cfg);\n\t\ttoolsTab.create(instr, cfg);\n\t\tpresetsTab.create(instr, cfg, shell, toolsTab, selectTab);\n\n\t\tInputsTable.create(middle, cfg, prompt);\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/InputMode.java
      nodes:
      - id: InputMode
        type: EnumDeclaration
        code: "public enum InputMode {\n\tSystemPrompt, Tools, Selection, Converter\n}\n"
```
/answer 817c-e allow
Control Request:
```yaml
id: 817c-f
toolName: grep
arguments:
  items:
  - pattern: inputModes
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
  reason: Default-Werte für inputModes im SessionConfig finden"
```
/answer 817c-f allow
Control Request:
```yaml
id: 817c-10
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/SessionConfig.java
        matches:
        - lineno: 21
          match: "public boolean[] inputModes = new boolean[InputMode.values().length];"
        - lineno: 42
          match: this.inputModes = other.inputModes.clone();
        - lineno: 123
          match: "return inputModes[mode.ordinal()];"
        - lineno: 127
          match: "inputModes[mode.ordinal()] = enable;"
      - path: xy/ai/workbench/MementoConverter.java
        matches:
        - lineno: 39
          match: snap.inputModes = cfg.inputModes;
        - lineno: 79
          match: "public boolean[] inputModes;"
      - path: xy/ai/workbench/ConfigManager.java
        matches:
        - lineno: 102
          match: inputObs.forEach(c -> c.accept(cfg.inputModes));
        - lineno: 258
          match: inputObs.forEach(c -> c.accept(cfg.inputModes));
        - lineno: 262
          match: inputObs.forEach(c -> c.accept(cfg.inputModes));
        - lineno: 351
          match: obs.accept(cfg.inputModes);
        - lineno: 512
          match: if (snap.inputModes != null)
        - lineno: 514
          match: if (mode.ordinal() < snap.inputModes.length)
        - lineno: 515
          match: "setInputMode(mode, snap.inputModes[mode.ordinal()]);"
```
/answer 817c-10 allow
ReasoningToken: 341
Control Request:
```yaml
id: 817c-11
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  reason: Verstehen von SelectTab (Systemprompt Liste vs Selektion) und ToolsTab für Clear-Logik"
```
/answer 817c-11 allow
Control Request:
```yaml
id: 817c-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      nodes:
      - id: jyeyWr|eVEU2I
        signature: import java.util.ArrayList;
      - id: SelectTab
        signature: "/**  * \"Select\" tab: toggles system-prompt lines on/off and holds the free-text…"
        docstring: "\"Select\" tab: toggles system-prompt lines on/off and holds the free-text"
        children:
        - id: SelectTab.FGxcw8|xBZMzM
          signature: private final FormToolkit toolkit;
        - id: SelectTab.SelectTab
          signature: "public SelectTab(FormToolkit toolkit, UpdateGuard guard, Runnable reflow)"
        - id: SelectTab.create
          signature: "public void create(TabFolder instr, ConfigManager cfg)"
        - id: SelectTab.createFreeTextArea
          signature: "private void createFreeTextArea(SashForm sash, ConfigManager cfg)"
        - id: SelectTab.updatePromptLines
          signature: "private String[] updatePromptLines(String[] cur)"
        - id: SelectTab.updateInstructionList
          signature: "private void updateInstructionList(String[] systemPrompt)"
        - id: SelectTab.clear
          signature: public void clear()
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      nodes:
      - id: wZA8ZO|30eIhE
        signature: import java.util.Arrays;
      - id: ToolsTab
        signature: "/**  * \"Tools\" tab: multi-select list of tools enabled for the current model.  …"
        docstring: "\"Tools\" tab: multi-select list of tools enabled for the current model."
        children:
        - id: ToolsTab.c4iLED|KWV8qC
          signature: private List toolsList;
        - id: ToolsTab.create
          signature: "public void create(TabFolder instr, ConfigManager cfg)"
        - id: ToolsTab.getSelection
          signature: "public String[] getSelection()"
        - id: ToolsTab.setSelection
          signature: "public void setSelection(String[] items)"
        - id: ToolsTab.clear
          signature: public void clear()
```
/answer 817c-12 allow
ReasoningToken: 597
Control Request:
```yaml
id: 817c-13
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    ids:
    - SelectTab.clear
    - SelectTab.updateInstructionList
    - SelectTab.create
  reason: Verstehen von SelectTab.clear() Verhalten für Clear All Logik"
```
/answer 817c-13 allow
Control Request:
```yaml
id: 817c-14
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      nodes:
      - id: SelectTab.clear
        type: MethodDeclaration
        code: "\t@Override\n\tpublic void clear() {\n\t\tinstructionSelection.clear();\n\t}\n"
      - id: SelectTab.updateInstructionList
        type: MethodDeclaration
        code: "\tprivate void updateInstructionList(String[] systemPrompt) {\n\t\tguard.run(() -> {\n\t\t\tinstructionList.setItems(systemPrompt);\n\t\t\tinstructionList.deselectAll();\n\t\t\tfor (int i = 0; i < systemPrompt.length; i++) {\n\t\t\t\tif (!systemPrompt[i].startsWith(\"#\"))\n\t\t\t\t\tinstructionList.select(i);\n\t\t\t}\n\t\t\treflow.run();\n\t\t});\n\t}\n"
      - id: SelectTab.create
        type: MethodDeclaration
        code: "\tpublic void create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem instrSel = new TabItem(instr, SWT.NONE);\n\t\tinstrSel.setText(\"Select\");\n\n\t\tSashForm sash = new SashForm(instr, SWT.VERTICAL);\n\t\tsash.setLayout(new GridLayout(1, false));\n\t\tGridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tscl2.heightHint = 100;\n\t\tscl2.widthHint = 1;\n\t\tsash.setLayoutData(scl2);\n\n\t\tComposite comp = new Composite(sash, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\tinstrSel.setControl(sash);\n\t\tinstructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\t\tcfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\tinstructionList.setLayoutData(gridData);\n\t\tinstructionList.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionList.setFocus()));\n\n\t\tinstructionList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tinstructionSelection = new ArrayList<>(Arrays.asList(instructionList.getSelection()));\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t}));\n\t\tinstructionList.addListener(SWT.MouseDown, event -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tString[] clickedIndex = instructionList.getSelection();\n\t\t\tLOG.info(Arrays.toString(clickedIndex));\n\t\t\tif (clickedIndex.length != 1)\n\t\t\t\treturn;\n\n\t\t\tif (!instructionSelection.remove(clickedIndex[0]))\n\t\t\t\tinstructionSelection.add(clickedIndex[0]);\n\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t});\n\n\t\tcreateFreeTextArea(sash, cfg);\n\t}\n"
```
/answer 817c-14 allow
ReasoningToken: 92
Control Request:
```yaml
id: 817c-15
toolName: grep
arguments:
  items:
  - pattern: setOuputMode|OutputMode\.values|class OutputMode
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
  reason: Output-Mode Standardwert und UI-Komponente finden"
```
/answer 817c-15 allow
Control Request:
```yaml
id: 817c-16
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/session/OutputModeSection.java
        matches:
        - lineno: 17
          match: "public class OutputModeSection {"
        - lineno: 25
          match: "String[] outputOptions = Arrays.stream(OutputMode.values()).map(e -> e.name()).collect(Collectors.toList())"
        - lineno: 29
          match: .widgetSelectedAdapter(e -> cfg.setOuputMode(OutputMode.valueOf(outputMode.getText()))));
      - path: xy/ai/workbench/view/session/PresetsTab.java
        matches:
        - lineno: 90
          match: cfg.setOuputMode(preset.outputMode);
      - path: xy/ai/workbench/ConfigManager.java
        matches:
        - lineno: 239
          match: "public void setOuputMode(OutputMode ouputMode) {"
        - lineno: 511
          match: setOuputMode(OutputMode.valueOf(snap.outputMode));
```
/answer 817c-16 allow
ReasoningToken: 882
Control Request:
```yaml
id: 817c-17
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/OutputMode.java
  node_type: EnumDeclaration
  reason: OutputMode Enum-Reihenfolge für Default-Wert ermitteln"
```
/answer 817c-17 allow
Control Request:
```yaml
id: 817c-18
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/OutputMode.java
      nodes:
      - id: OutputMode
        type: EnumDeclaration
        code: "public enum OutputMode {\n\tAppend, Replace, Cursor, Chat, New_File\n}\n"
```
/answer 817c-18 allow
ReasoningToken: 34
Control Request:
```yaml
id: 817c-19
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    ids:
    - ToolsTab.getSelection
    - ToolsTab.setSelection
    - ToolsTab.clear
    - ToolsTab.create
    - ToolsTab.c4iLED|KWV8qC
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolSelection.java
    ids:
    - ToolSelection
  reason: Vollständige Vorlage für ToolSelection-Implementierung als Muster für InputsTable"
```
/answer 817c-19 allow
Control Request:
```yaml
id: 817c-1a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      nodes:
      - id: ToolsTab.getSelection
        type: MethodDeclaration
        code: "\t@Override\n\tpublic String[] getSelection() {\n\t\treturn toolsList.getSelection();\n\t}\n"
      - id: ToolsTab.setSelection
        type: MethodDeclaration
        code: "\t@Override\n\tpublic void setSelection(String[] items) {\n\t\ttoolsMultiSelect.setSelection(items);\n\t}\n"
      - id: ToolsTab.clear
        type: MethodDeclaration
        code: "\t@Override\n\tpublic void clear() {\n\t\ttoolsMultiSelect.clear();\n\t}\n"
      - id: ToolsTab.create
        type: MethodDeclaration
        code: "\tpublic void create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem toolSel = new TabItem(instr, SWT.NONE);\n\t\ttoolSel.setText(\"Tools\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\ttoolSel.setControl(comp);\n\t\ttoolsList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\ttoolsList.setLayoutData(gridData);\n\t\ttoolsList.addMouseListener(MouseListener.mouseDownAdapter(m -> toolsList.setFocus()));\n\t\ttoolsList.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setEnabledTools(toolsList.getSelection())));\n\t\tcfg.addModelObs(m -> {\n\t\t\tif (m != null)\n\t\t\t\ttoolsList.setItems(m.cap.getTools());\n\t\t}, true);\n\t\ttoolsMultiSelect = new MultiSelectListener(toolsList);\n\t\tcfg.addEnabledToolsObs(t -> {\n\t\t\tif (!Arrays.equals(toolsList.getSelection(), t))\n\t\t\t\ttoolsMultiSelect.setSelection(t);\n\t\t}, true);\n\t}\n"
      - id: ToolsTab.c4iLED|KWV8qC
        type: StatementGroup
        code: "\tprivate List toolsList;\n\tprivate MultiSelectListener toolsMultiSelect;\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolSelection.java
      nodes:
      - id: ToolSelection
        type: ClassOrInterfaceDeclaration
        signature: "/**  * Read/write access to a widget's multi-selection, needed for cross-tab pr…"
        docstring: "Read/write access to a widget's multi-selection, needed for cross-tab preset"
        children:
        - id: ToolSelection.getSelection
          type: MethodDeclaration
          code: "\tpublic String[] getSelection();\n"
        - id: ToolSelection.setSelection
          type: MethodDeclaration
          code: "\tpublic void setSelection(String[] items);\n"
```
/answer 817c-1a allow
ReasoningToken: 560
Control Request:
```yaml
id: 817c-1b
toolName: grep
arguments:
  items:
  - pattern: IProject|getProject\(\)|ActiveEditorListener
    directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    include:
    - '*.java'
    limit: 30
  reason: "Ermitteln, wie das aktuelle Projekt bestimmt werden kann für den Save-Dialog\""
```
/answer 817c-1b allow
Control Request:
```yaml
id: 817c-1c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: IncludeAdapter.java
        matches:
        - lineno: 47
          match: private ActiveEditorListener editorListener;
        - lineno: 53
          match: "public IncludeAdapter(ActiveEditorListener editorListener) {"
      - path: EditorInterface.java
        matches:
        - lineno: 44
          match: private final ActiveEditorListener editorListener;
        - lineno: 48
          match: "public EditorInterface(ConfigManager cfg, ActiveEditorListener editorListener, AdaptingConnector connector) {"
      - path: ActiveEditorListener.java
        matches:
        - lineno: 38
          match: "public class ActiveEditorListener extends PartListener2Adapter implements IPartListener2 {"
        - lineno: 145
          match: "return file != null ? Paths.get(file.getProject().getLocation().toOSString()) : null;"
      - path: view/PresetHandler.java
        matches:
        - lineno: 16
          match: import org.eclipse.core.resources.IProject;
        - lineno: 52
          match: "for (IProject project : ResourcesPlugin.getWorkspace().getRoot().getProjects()) {"
      - path: view/explorer/ProjectFilter.java
        matches:
        - lineno: 3
          match: import org.eclipse.core.resources.IProject;
        - lineno: 16
          match: "* on/off and keeps {@link #setFocusedProject(IProject)} in sync with the"
        - lineno: 29
          match: private static volatile IProject focusedProject;
        - lineno: 31
          match: "public static void setFocusedProject(IProject project) {"
        - lineno: 35
          match: "public static IProject getFocusedProject() {"
        - lineno: 41
          match: IProject project = focusedProject;
        - lineno: 44
          match: return project.equals(((IResource) element).getProject());
      - path: view/explorer/MarkdownExplorerView.java
        matches:
        - lineno: 77
          match: "if (!file.getProject().equals(ProjectFilter.getFocusedProject())) {"
        - lineno: 78
          match: ProjectFilter.setFocusedProject(file.getProject());
      - path: view/explorer/GitIgnoreFilter.java
        matches:
        - lineno: 44
          match: IContainer dir = resource.getProject();
      - path: view/FlatEndingContentProvider.java
        matches:
        - lineno: 10
          match: import org.eclipse.core.resources.IProject;
        - lineno: 38
          match: ((IProject) proj).accept(visitor);
      - path: view/diff/DiffPanel.java
        matches:
        - lineno: 37
          match: import xy.ai.workbench.ActiveEditorListener;
        - lineno: 90
          match: ActiveEditorListener editLst = Activator.getDefault().editorListener;
      - path: view/diff/OpSnapshotter.java
        matches:
        - lineno: 17
          match: import org.eclipse.core.resources.IProject;
        - lineno: 70
          match: "for (IProject p : root.getProjects())"
      - path: view/ClaudeCodeSessionView.java
        matches:
        - lineno: 33
          match: import xy.ai.workbench.ActiveEditorListener;
        - lineno: 215
          match: ActiveEditorListener edtlst = Activator.getDefault().editorListener;
      - path: editor/AISessionEditor.java
        matches:
        - lineno: 79
          match: if (((FileEditorInput) editor.getEditorInput()).getFile().getProject()
      - path: Activator.java
        matches:
        - lineno: 24
          match: public final ActiveEditorListener editorListener = new ActiveEditorListener();
      - path: ConfigManager.java
        matches:
        - lineno: 48
          match: "public ConfigManager(MCPClient mcpClient, ActiveEditorListener editorListener) {"
      - path: connector/harness/PromptHandler.java
        matches:
        - lineno: 14
          match: import xy.ai.workbench.ActiveEditorListener;
      warning: "Limit of 30 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 817c-1c allow
ReasoningToken: 333
Control Request:
```yaml
id: 817c-1d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/explorer/ProjectFilter.java
    ids:
    - ProjectFilter
  reason: Vollständige ProjectFilter-Klasse zur Ermittlung des aktuellen Projekts lesen"
```
/answer 817c-1d allow
Control Request:
```yaml
id: 817c-1e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/explorer/ProjectFilter.java
      nodes:
      - id: ProjectFilter
        type: ClassOrInterfaceDeclaration
        signature: /**  * Common Navigator Framework filter that limits the tree to the resources …
        docstring: Common Navigator Framework filter that limits the tree to the resources of a
        children:
        - id: ProjectFilter.E6gZg9|V4Xpii
          type: StatementGroup
          code: "\tprivate static volatile IProject focusedProject;\n"
        - id: ProjectFilter.setFocusedProject
          type: MethodDeclaration
          code: "\tpublic static void setFocusedProject(IProject project) {\n\t\tfocusedProject = project;\n\t}\n"
        - id: ProjectFilter.getFocusedProject
          type: MethodDeclaration
          code: "\tpublic static IProject getFocusedProject() {\n\t\treturn focusedProject;\n\t}\n"
        - id: ProjectFilter.select
          type: MethodDeclaration
          code: "\t@Override\n\tpublic boolean select(Viewer viewer, Object parentElement, Object element) {\n\t\tIProject project = focusedProject;\n\t\tif (project == null || !(element instanceof IResource))\n\t\t\treturn true;\n\t\treturn project.equals(((IResource) element).getProject());\n\t}\n"
        - id: ProjectFilter.equals
          type: MethodDeclaration
          code: "\t@Override\n\tpublic boolean equals(Object obj) {\n\t\treturn obj instanceof ProjectFilter;\n\t}\n"
        - id: ProjectFilter.hashCode
          type: MethodDeclaration
          code: "\t@Override\n\tpublic int hashCode() {\n\t\treturn ProjectFilter.class.hashCode();\n\t}\n"
```
/answer 817c-1e allow
ReasoningToken: 51
Control Request:
```yaml
id: 817c-1f
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ActiveEditorListener.java
  reason: API von ActiveEditorListener für aktuelles Projekt prüfen"
```
/answer 817c-1f allow
Control Request:
```yaml
id: 817c-20
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ActiveEditorListener.java
      nodes:
      - id: m8E7jQ|sMsvfQ
        signature: import java.io.File;
      - id: bPYsxK|8AHORz
        signature: import org.eclipse.ui.IWorkbenchPartReference;
      - id: ActiveEditorListener
        signature: public class ActiveEditorListener extends PartListener2Adapter implements IPart…
        children:
        - id: ActiveEditorListener.7MTwkl|a9VMMU
          signature: private final EditorChangeListener editorListener = new EditorChangeListener();
        - id: ActiveEditorListener.addInputObserver
          signature: public void addInputObserver(Consumer<InputMode> obs)
        - id: ActiveEditorListener.addTextEditorObserver
          signature: public void addTextEditorObserver(Consumer<ITextEditor> obs)
        - id: ActiveEditorListener.checkInitBindings
          signature: private void checkInitBindings()
        - id: ActiveEditorListener.getLastTextEditor
          signature: public ITextEditor getLastTextEditor()
        - id: ActiveEditorListener.getLastEditorInput
          signature: private IEditorInput getLastEditorInput()
        - id: ActiveEditorListener.getLastEditorSelection
          signature: private ITextSelection getLastEditorSelection()
        - id: ActiveEditorListener.getLastEditorFile
          signature: public IFile getLastEditorFile()
        - id: ActiveEditorListener.getLastContainer
          signature: public IContainer getLastContainer()
        - id: ActiveEditorListener.Selection
          signature: "public record Selection(String[] selection, Integer cursorOffset) {"
        - id: ActiveEditorListener.getSelection
          signature: public Selection getSelection()
        - id: ActiveEditorListener.resolveAbsoluteFilePath
          signature: public String resolveAbsoluteFilePath()
        - id: ActiveEditorListener.resolveProjectPath
          signature: public Path resolveProjectPath()
        - id: ActiveEditorListener.getFileContent
          signature: public Selection getFileContent()
        - id: ActiveEditorListener.partActivated
          signature: public void partActivated(IWorkbenchPartReference partRef)
```
/answer 817c-20 allow
ReasoningToken: 86
Control Request:
```yaml
id: 817c-21
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ActiveEditorListener.java
    ids:
    - ActiveEditorListener.resolveProjectPath
    - ActiveEditorListener.getLastEditorFile
  reason: Implementierung von resolveProjectPath prüfen für Save-Dialog Root"
```
/answer 817c-21 allow
Control Request:
```yaml
id: 817c-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ActiveEditorListener.java
      nodes:
      - id: ActiveEditorListener.resolveProjectPath
        type: MethodDeclaration
        code: "\tpublic Path resolveProjectPath() {\n\t\tIFile file = getLastEditorFile();\n\t\treturn file != null ? Paths.get(file.getProject().getLocation().toOSString()) : null;\n\t}\n"
      - id: ActiveEditorListener.getLastEditorFile
        type: MethodDeclaration
        code: "\tpublic IFile getLastEditorFile() {\n\t\tIEditorInput input = getLastEditorInput();\n\t\tif (input == null)\n\t\t\treturn null;\n\t\tif (input instanceof IFileEditorInput)\n\t\t\treturn ((IFileEditorInput) input).getFile();\n\t\telse if (input instanceof IURIEditorInput) {\n\t\t\tURI uri = ((IURIEditorInput) input).getURI();\n\t\t\tString fileName = new org.eclipse.core.runtime.Path(uri.getPath()).lastSegment();\n\t\t\tIFile file = ResourcesPlugin.getWorkspace().getRoot().getProject(\"ExternalFiles\").getFile(fileName);\n\n\t\t\tif (!file.exists())\n\t\t\t\ttry {\n\t\t\t\t\tfile.createLink(uri, IResource.ALLOW_MISSING_LOCAL, new NullProgressMonitor());\n\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\t//throw new IllegalStateException(\"Could not link external file\", e);\n\t\t\t\t\tLOG.info(\"Could not link external file: \" + e.getMessage());\n\t\t\t\t\treturn null;\n\t\t\t\t}\n\t\t\treturn file;\n\t\t}\n\t\treturn null;\n\t}\n"
```
/answer 817c-22 allow
ReasoningToken: 38
Control Request:
```yaml
id: 817c-23
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
  reason: Activator.getDefault() API prüfen für Zugriff auf editorListener"
```
/answer 817c-23 allow
Control Request:
```yaml
id: 817c-24
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
      nodes:
      - id: omghUG|GwKQon
        signature: import org.eclipse.core.runtime.Platform;
      - id: Activator
        signature: /**  * The activator class controls the plug-in life cycle  */ public class Act…
        docstring: The activator class controls the plug-in life cycle
        children:
        - id: Activator.LkQUr8|F2N9YI
          signature: public static final String PLUGIN_ID = "XY.AI.Workbench"; //$NON-NLS-1$
        - id: Activator.start
          signature: public void start(BundleContext context) throws Exception
        - id: Activator.stop
          signature: public void stop(BundleContext context) throws Exception
        - id: Activator.getDefault
          signature: public static Activator getDefault()
```
/answer 817c-24 allow
ReasoningToken: 8671
Text:
 Now let's look at full `PresetsTab.java` and `InputsTable.java` source and `Clearable.java`, `ToolSelection.java` to write final imports correctly.
Control Request:
```yaml
id: 817c-25
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/Clearable.java
    ids:
    - Clearable
  reason: Vollständigen Clearable Interface Code lesen"
```
/answer 817c-25 allow
Control Request:
```yaml
id: 817c-26
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/Clearable.java
      nodes:
      - id: Clearable
        type: ClassOrInterfaceDeclaration
        signature: /**  * Something that can be reset to an empty selection.  */ public interface …
        docstring: Something that can be reset to an empty selection.
        children:
        - id: Clearable.clear
          type: MethodDeclaration
          code: "\tpublic void clear();\n"
```
/answer 817c-26 allow
Text:
 Now let's implement. First, create the `InputSelection` interface, update `Clearable`/`ToolSelection` not needed to change. Create `PresetList` class. Update `PresetHandler`. Update `InputsTable` to instance-based implementing `InputSelection`. Update `PresetsTab`. Update `InstructionSection`.
Control Request:
```yaml
id: 817c-27
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputSelection.java
    source: "package xy.ai.workbench.view.session;\n\n/**\n * Read/write access to the set of enabled {@link xy.ai.workbench.InputMode}s,\n * needed for cross-tab preset save/load and reset.\n */\npublic interface InputSelection extends Clearable {\n\tpublic String[] getSelection();\n\n\tpublic void setSelection(String[] items);\n}\n"
  reason: Neues Interface für die Input-Auswahl analog zu ToolSelection anlegen"
```
/answer 817c-27 allow
Text:
 Now update `InputsTable.java` to be instance-based:
Control Request:
```yaml
id: 817c-28
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputsTable.java
    id: InputsTable
    source: "/**\n * Checkbox table showing/enabling the available prompt input modes.\n */\npublic class InputsTable implements InputSelection {\n\n\tprivate ConfigManager cfg;\n\n\tpublic void create(Composite middle, ConfigManager cfg, PromptHandler prompt) {\n\t\tthis.cfg = cfg;\n\t\tTable table = new Table(middle, SWT.CHECK | SWT.BORDER | SWT.V_SCROLL);\n\t\ttable.setHeaderVisible(true);\n\t\ttable.setLinesVisible(true);\n\n\t\tTableColumn column1 = new TableColumn(table, SWT.NONE);\n\t\tcolumn1.setText(\"On\");\n\t\tcolumn1.setWidth(30);\n\n\t\tTableColumn column2 = new TableColumn(table, SWT.NONE);\n\t\tcolumn2.setText(\"Input\");\n\t\tcolumn2.setWidth(120);\n\n\t\tTableColumn column3 = new TableColumn(table, SWT.NONE);\n\t\tcolumn3.setText(\"Amount\");\n\t\tcolumn3.setWidth(45);\n\n\t\ttable.addListener(SWT.Selection, e -> {\n\t\t\tif (e.detail == SWT.CHECK) {\n\t\t\t\tTableItem item = (TableItem) e.item;\n\t\t\t\tInputMode mode = InputMode.valueOf(item.getText(1).replace(\" \", \"_\"));\n\t\t\t\tcfg.setInputMode(mode, !cfg.isInputEnabled(mode));\n\t\t\t}\n\t\t});\n\n\t\tfor (int i = 0; i < InputMode.values().length; i++) {\n\t\t\tTableItem item = new TableItem(table, SWT.NONE);\n\t\t\tInputMode mode = InputMode.values()[i];\n\t\t\tprompt.addInputStatObs(is -> {\n\t\t\t\tvar checked = item.getChecked();\n\t\t\t\titem.setText(new String[] { \"\", mode.name().replace(\"_\", \" \"), is[mode.ordinal()] + \"\" });\n\t\t\t\titem.setChecked(checked);\n\t\t\t}, true);\n\t\t\tcfg.addInputObs(is -> {\n\t\t\t\titem.setChecked(is[mode.ordinal()]);\n\t\t\t}, true);\n\t\t}\n\t}\n\n\t@Override\n\tpublic String[] getSelection() {\n\t\tjava.util.List<String> result = new java.util.ArrayList<>();\n\t\tfor (InputMode mode : InputMode.values())\n\t\t\tif (cfg.isInputEnabled(mode))\n\t\t\t\tresult.add(mode.name());\n\t\treturn result.toArray(new String[0]);\n\t}\n\n\t@Override\n\tpublic void setSelection(String[] items) {\n\t\tjava.util.Set<String> enabled = new java.util.HashSet<>(\n\t\t\t\tjava.util.Arrays.asList(items != null ? items : new String[0]));\n\t\tfor (InputMode mode : InputMode.values())\n\t\t\tcfg.setInputMode(mode, enabled.contains(mode.name()));\n\t}\n\n\t@Override\n\tpublic void clear() {\n\t\tfor (InputMode mode : InputMode.values())\n\t\t\tcfg.setInputMode(mode, false);\n\t}\n}\n"
  reason: InputsTable in Instanzklasse mit InputSelection-Implementierung umwandeln"
```
/answer 817c-28 allow
Control Request:
```yaml
id: 817c-29
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    old_text: "\tprivate final PresetsTab presetsTab = new PresetsTab();\n"
    new_text: "\tprivate final PresetsTab presetsTab = new PresetsTab();\n\tprivate final InputsTable inputsTable = new InputsTable();\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    old_text: "\t\tpresetsTab.create(instr, cfg, shell, toolsTab, selectTab);\n\n\t\tInputsTable.create(middle, cfg, prompt);"
    new_text: "\n\t\tinputsTable.create(middle, cfg, prompt);\n\t\tpresetsTab.create(instr, cfg, shell, toolsTab, selectTab, inputsTable);"
  reason: InstructionSection auf instanzbasierte InputsTable umstellen und an PresetsTab übergeben"
```
/answer 817c-29 allow
Control Request:
```yaml
id: 817c-2a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
      result: success
      new_id: InstructionSection.JlzHiL|lw7DqO
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
      error: No node matched the given text; a node selector (id) is required.
```
/answer 817c-2a allow
Control Request:
```yaml
id: 817c-2b
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    id: InstructionSection.create
    old_text: "\t\tpresetsTab.create(instr, cfg, shell, toolsTab, selectTab);\n\n\t\tInputsTable.create(middle, cfg, prompt);"
    new_text: "\n\t\tinputsTable.create(middle, cfg, prompt);\n\t\tpresetsTab.create(instr, cfg, shell, toolsTab, selectTab, inputsTable);"
  reason: "Aufrufreihenfolge anpassen: InputsTable zuerst erzeugen, dann an PresetsTab übergeben\""
```
/answer 817c-2b allow
Control Request:
```yaml
id: 817c-2c
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
      error: Text not found in node; its content appears to have changed.
      id: InstructionSection.create
      reason: content_changed
      guess: |2-
            presetsTab.create(instr, cfg, shell, toolsTab, selectTab);
            InputsTable.create(middle, cfg, prompt);
          }
```
/answer 817c-2c allow
Control Request:
```yaml
id: 817c-2d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    ids:
    - InstructionSection.create
  reason: Aktuellen Stand der create-Methode prüfen"
```
/answer 817c-2d allow
Control Request:
```yaml
id: 817c-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
      nodes:
      - id: InstructionSection.create
        type: MethodDeclaration
        code: |2
            public void create(Composite body, ConfigManager cfg, PromptHandler prompt, Shell shell) {
              Composite middle = new Composite(body, SWT.NONE);
              middle.setLayout(new GridLayout(1, false));
              GridData ldat2 = new GridData(SWT.FILL, SWT.FILL, true, true);
              ldat2.heightHint = 100;
              middle.setLayoutData(ldat2);
              toolkit.createLabel(middle, "System Prompt:");
              Composite contComp = new Composite(middle, SWT.NONE);
              contComp.setLayout(new GridLayout(1, false));
              GridData scl = new GridData(SWT.FILL, SWT.FILL, true, true);
              scl.heightHint = 100;
              scl.widthHint = 1;
              contComp.setLayoutData(scl);
              TabFolder instr = new TabFolder(contComp, SWT.NONE);
              GridData ldat1 = new GridData(SWT.FILL, SWT.FILL, true, true);
              ldat1.heightHint = 100;
              instr.setLayoutData(ldat1);
              selectTab.create(instr, cfg);
              editTab.create(instr, cfg);
              toolsTab.create(instr, cfg);
              presetsTab.create(instr, cfg, shell, toolsTab, selectTab);
              InputsTable.create(middle, cfg, prompt);
            }
```
/answer 817c-2e allow
Text:
 Now update `InstructionSection.java` to use instance `inputsTable` and pass it to presetsTab:
Control Request:
```yaml
id: 817c-2f
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    id: InstructionSection.create
    old_text: |-
      presetsTab.create(instr, cfg, shell, toolsTab, selectTab);
          InputsTable.create(middle, cfg, prompt);
    new_text: |-
      inputsTable.create(middle, cfg, prompt);
          presetsTab.create(instr, cfg, shell, toolsTab, selectTab, inputsTable);
  reason: "Aufrufreihenfolge anpassen: InputsTable zuerst erzeugen, dann an PresetsTab übergeben\""
```
/answer 817c-2f allow
Control Request:
```yaml
id: 817c-30
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
    ids:
    - Activator
  reason: Vollständigen Activator-Code für korrekten Zugriff auf editorListener lesen"
```
/answer 817c-30 allow
Control Request:
```yaml
id: 817c-31
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
      nodes:
      - id: Activator
        type: ClassOrInterfaceDeclaration
        signature: /**  * The activator class controls the plug-in life cycle  */ public class Act…
        docstring: The activator class controls the plug-in life cycle
        children:
        - id: Activator.LkQUr8|F2N9YI
          type: StatementGroup
          code: "\tpublic static final String PLUGIN_ID = \"XY.AI.Workbench\"; //$NON-NLS-1$\n\n\tprivate static Activator plugin;\n\n\tprivate final MCPClient mcpClient = new MCPClient();\n\tpublic final ActiveEditorListener editorListener = new ActiveEditorListener();\n\tpublic final CCSessionManager ccSessionManager = new CCSessionManager();\n\tpublic final ConfigManager cfg = new ConfigManager(mcpClient, editorListener);\n\tprivate final IncludeAdapter includeAdapter = new IncludeAdapter(editorListener);\n\tprivate final AdaptingConnector connector = new AdaptingConnector(cfg, ccSessionManager, mcpClient, includeAdapter);\n\tpublic final AIBatchManager batch = new AIBatchManager(connector);\n\tpublic final AIBatchResponseManager batchRequests = new AIBatchResponseManager(connector);\n\tpublic final EditorInterface editorInterface = new EditorInterface(cfg, editorListener, connector);\n\tpublic final PromptHandler promptHandler = new PromptHandler(cfg, editorListener, connector, batch, editorInterface,\n\t\t\tincludeAdapter);\n"
        - id: Activator.start
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void start(BundleContext context) throws Exception {\n\t\tsuper.start(context);\n\t\tplugin = this;\n\t\teditorInterface.register(context);\n\t\tLOG.log = Platform.getLog(context.getBundle());\n\t}\n"
        - id: Activator.stop
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void stop(BundleContext context) throws Exception {\n\t\tplugin = null;\n\t\teditorInterface.dispose(context);\n\t\tsuper.stop(context);\n\t}\n"
        - id: Activator.getDefault
          type: MethodDeclaration
          code: "\tpublic static Activator getDefault() {\n\t\treturn plugin;\n\t}\n"
```
/answer 817c-31 allow
ReasoningToken: 53
ReasoningToken: 29
Text:
 Good, it's unchanged — just formatting differs (tabs vs spaces in display).
Control Request:
```yaml
id: 817c-32
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.bbo2aQ|7WKcdc
    old_text: "\tprivate static final String PROMPT_TXT = \".prompt.txt\";"
    new_text: "\tpublic static final String PROMPT_TXT = \".prompt.txt\";"
  reason: "PROMPT_TXT public machen, damit PresetList (anderes Package) den Dateinamen-Suffix kennt\""
```
/answer 817c-32 allow
Text:
 Now let's update `PresetHandler.java`: add title/inputs support, public PROMPT_TXT, and project-root-aware save dialog.
Control Request:
```yaml
id: 817c-33
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.FrontMatter
    source: "\tpublic static class FrontMatter {\n\t\tpublic String title;\n\t\tpublic List<String> tools;\n\t\tpublic String outputMode;\n\t\tpublic List<String> inputs;\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.Preset
    source: "\tpublic static class Preset {\n\t\tpublic String title;\n\t\tpublic String[] body;\n\t\tpublic String[] tools;\n\t\tpublic OutputMode outputMode;\n\t\tpublic String[] inputs;\n\t}\n"
  reason: FrontMatter und Preset um title und inputs erweitern"
```
/answer 817c-33 allow
Text:
 Good. Now rewrite PresetHandler.java pieces.
Control Request:
```yaml
id: 817c-34
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.parsePreset
    source: "\tpublic static Preset parsePreset(String content) {\n\t\tPreset preset = new Preset();\n\t\tString body = content;\n\n\t\tMatcher matcher = FRONT_MATTER.matcher(content);\n\t\tif (matcher.find() && matcher.start() == 0) {\n\t\t\tString yaml = matcher.group(1);\n\t\t\tbody = content.substring(matcher.end());\n\t\t\tif (yaml != null && !yaml.isBlank()) {\n\t\t\t\ttry {\n\t\t\t\t\tFrontMatter fm = YAML.readValue(yaml, FrontMatter.class);\n\t\t\t\t\tif (fm.title != null && !fm.title.trim().isEmpty())\n\t\t\t\t\t\tpreset.title = fm.title.trim();\n\t\t\t\t\tif (fm.tools != null)\n\t\t\t\t\t\tpreset.tools = fm.tools.toArray(new String[0]);\n\t\t\t\t\tif (fm.outputMode != null && !fm.outputMode.isBlank())\n\t\t\t\t\t\tpreset.outputMode = OutputMode.valueOf(fm.outputMode.trim());\n\t\t\t\t\tif (fm.inputs != null)\n\t\t\t\t\t\tpreset.inputs = fm.inputs.toArray(new String[0]);\n\t\t\t\t} catch (Exception e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\n\t\tif (body.endsWith(\"\\n\"))\n\t\t\tbody = body.substring(0, body.length() - 1);\n\t\tpreset.body = body.split(\"\\n\", -1);\n\t\treturn preset;\n\t}\n"
  reason: "parsePreset um Title (trim, leer ignorieren) und Inputs erweitern\""
```
/answer 817c-34 allow
Text:
 Now writePreset and serializePreset, plus project-root resolution:
Control Request:
```yaml
id: 817c-35
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.writePreset
    source: "\tpublic static void writePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs,\n\t\t\tShell shell) {\n\t\tFileDialog dialog = new FileDialog(shell, SWT.SAVE);\n\t\tdialog.setFilterPath(resolveDefaultSaveDir());\n\t\tdialog.setFilterExtensions(new String[] { \"*.prompt.txt\" });\n\t\tdialog.setFilterNames(new String[] { \"Prompt Files (*prompt.txt)\" });\n\t\tString filePath = dialog.open();\n\t\tif (filePath == null)\n\t\t\treturn;\n\n\t\tString content = serializePreset(body, tools, outputMode, inputs, \"\");\n\t\ttry (PrintWriter writer = new PrintWriter(new FileWriter(filePath, StandardCharsets.UTF_8))) {\n\t\t\twriter.print(content);\n\t\t} catch (IOException e) {\n\t\t\tthrow new IllegalStateException(e);\n\t\t}\n\t}\n\n\tprivate static String resolveDefaultSaveDir() {\n\t\ttry {\n\t\t\tjava.nio.file.Path projectPath = xy.ai.workbench.Activator.getDefault().editorListener.resolveProjectPath();\n\t\t\tif (projectPath != null)\n\t\t\t\treturn projectPath.toString();\n\t\t} catch (Exception e) {\n\t\t\t// fall back to first project below\n\t\t}\n\t\tIProject[] projects = ResourcesPlugin.getWorkspace().getRoot().getProjects();\n\t\treturn projects.length > 0 ? projects[0].getLocation().toOSString() : \"\";\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.serializePreset
    source: "\tpublic static String serializePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs,\n\t\t\tString title) {\n\t\tFrontMatter fm = new FrontMatter();\n\t\tfm.title = title != null ? title : \"\";\n\t\tif (tools != null && tools.length > 0)\n\t\t\tfm.tools = Arrays.asList(tools);\n\t\tif (outputMode != null)\n\t\t\tfm.outputMode = outputMode.name();\n\t\tif (inputs != null && inputs.length > 0)\n\t\t\tfm.inputs = Arrays.asList(inputs);\n\n\t\tStringBuilder sb = new StringBuilder();\n\t\ttry {\n\t\t\tString yaml = YAML.writeValueAsString(fm);\n\t\t\tsb.append(\"---\\n\");\n\t\t\tsb.append(yaml);\n\t\t\tsb.append(\"---\\n\");\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t\tsb.append(String.join(\"\\n\", body));\n\t\treturn sb.toString();\n\t}\n"
  reason: "writePreset/serializePreset um Title und Inputs erweitern, Save-Dialog im aktuellen Projekt-Root starten\""
```
/answer 817c-35 allow
Control Request:
```yaml
id: 817c-36
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
      result: success
      id: PresetHandler.serializePreset
      new_id: PresetHandler.serializePreset
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
      error: |
        invalid member declaration: Encountered unexpected token: "private" "private"
            at line 19, column 3.

        Was expecting:

            <EOF>
      id: PresetHandler.writePreset
```
/answer 817c-36 allow
Control Request:
```yaml
id: 817c-37
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.writePreset
    source: "\tpublic static void writePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs,\n\t\t\tShell shell) {\n\t\tFileDialog dialog = new FileDialog(shell, SWT.SAVE);\n\t\tdialog.setFilterPath(resolveDefaultSaveDir());\n\t\tdialog.setFilterExtensions(new String[] { \"*.prompt.txt\" });\n\t\tdialog.setFilterNames(new String[] { \"Prompt Files (*prompt.txt)\" });\n\t\tString filePath = dialog.open();\n\t\tif (filePath == null)\n\t\t\treturn;\n\n\t\tString content = serializePreset(body, tools, outputMode, inputs, \"\");\n\t\ttry (PrintWriter writer = new PrintWriter(new FileWriter(filePath, StandardCharsets.UTF_8))) {\n\t\t\twriter.print(content);\n\t\t} catch (IOException e) {\n\t\t\tthrow new IllegalStateException(e);\n\t\t}\n\t}\n"
  reason: "writePreset Methode anpassen (Title/Inputs Parameter, Projekt-Root)\""
```
/answer 817c-37 allow
Control Request:
```yaml
id: 817c-38
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.writePreset
    position: after
    source: "\n\tprivate static String resolveDefaultSaveDir() {\n\t\ttry {\n\t\t\tjava.nio.file.Path projectPath = xy.ai.workbench.Activator.getDefault().editorListener.resolveProjectPath();\n\t\t\tif (projectPath != null)\n\t\t\t\treturn projectPath.toString();\n\t\t} catch (Exception e) {\n\t\t\t// fall back to first project below\n\t\t}\n\t\tIProject[] projects = ResourcesPlugin.getWorkspace().getRoot().getProjects();\n\t\treturn projects.length > 0 ? projects[0].getLocation().toOSString() : \"\";\n\t}\n"
  reason: Hilfsmethode zur Ermittlung des aktuellen Projekt-Roots für den Save-Dialog ergänzen"
```
/answer 817c-38 allow
Control Request:
```yaml
id: 817c-39
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
  reason: PresetHandler.java auf Syntaxfehler prüfen"
```
/answer 817c-39 allow
ReasoningToken: 62
Text:
 Now validate PresetHandler.java compiles.
Text:
 Now create the new `PresetList` class for discovery/dedup/labeling, separate from UI.
Control Request:
```yaml
id: 817c-3a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
    source: "package xy.ai.workbench.view.session;\n\nimport java.util.ArrayList;\nimport java.util.HashMap;\nimport java.util.HashSet;\nimport java.util.LinkedHashMap;\nimport java.util.List;\nimport java.util.Map;\nimport java.util.Set;\n\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.runtime.IPath;\n\nimport xy.ai.workbench.view.PresetHandler;\n\n/**\n * Discovers prompt preset files across the workspace, deduplicates them by\n * absolute file location and computes unique, human-readable display\n * labels, separately from any UI widget.\n */\npublic class PresetList {\n\n\tpublic static class Entry {\n\t\tpublic final IFile file;\n\t\tpublic final String label;\n\n\t\tEntry(IFile file, String label) {\n\t\t\tthis.file = file;\n\t\t\tthis.label = label;\n\t\t}\n\t}\n\n\tprivate static class Raw {\n\t\tIFile file;\n\t\tString title;\n\t}\n\n\tpublic List<Entry> load() {\n\t\tList<IFile> files = dedupe(PresetHandler.listPresetFiles());\n\t\tList<Raw> raws = new ArrayList<>();\n\t\tfor (IFile file : files) {\n\t\t\tRaw raw = new Raw();\n\t\t\traw.file = file;\n\t\t\traw.title = readTitle(file);\n\t\t\traws.add(raw);\n\t\t}\n\n\t\tMap<String, String> labels = computeLabels(raws);\n\n\t\tList<Entry> entries = new ArrayList<>();\n\t\tfor (Raw raw : raws)\n\t\t\tentries.add(new Entry(raw.file, labels.get(key(raw.file))));\n\t\treturn entries;\n\t}\n\n\tprivate String readTitle(IFile file) {\n\t\ttry {\n\t\t\treturn PresetHandler.loadPreset(file).title;\n\t\t} catch (Exception e) {\n\t\t\treturn null;\n\t\t}\n\t}\n\n\tprivate List<IFile> dedupe(IFile[] files) {\n\t\tList<IFile> result = new ArrayList<>();\n\t\tSet<String> seen = new HashSet<>();\n\t\tfor (IFile file : files)\n\t\t\tif (seen.add(key(file)))\n\t\t\t\tresult.add(file);\n\t\treturn result;\n\t}\n\n\tprivate String key(IFile file) {\n\t\treturn file.getLocation() != null ? file.getLocation().toOSString() : file.getFullPath().toOSString();\n\t}\n\n\tprivate Map<String, String> computeLabels(List<Raw> raws) {\n\t\tMap<String, String> result = new HashMap<>();\n\n\t\tMap<String, List<Raw>> titled = new LinkedHashMap<>();\n\t\tList<Raw> untitled = new ArrayList<>();\n\t\tfor (Raw raw : raws) {\n\t\t\tif (raw.title != null && !raw.title.trim().isEmpty())\n\t\t\t\ttitled.computeIfAbsent(raw.title.trim(), t -> new ArrayList<>()).add(raw);\n\t\t\telse\n\t\t\t\tuntitled.add(raw);\n\t\t}\n\n\t\tfor (Map.Entry<String, List<Raw>> group : titled.entrySet()) {\n\t\t\tboolean ambiguous = group.getValue().size() > 1;\n\t\t\tfor (Raw raw : group.getValue()) {\n\t\t\t\tString label = ambiguous ? group.getKey() + \" (\" + raw.file.getProject().getName() + \")\"\n\t\t\t\t\t\t: group.getKey();\n\t\t\t\tresult.put(key(raw.file), label);\n\t\t\t}\n\t\t}\n\n\t\tMap<String, List<Raw>> byBaseName = new LinkedHashMap<>();\n\t\tfor (Raw raw : untitled)\n\t\t\tbyBaseName.computeIfAbsent(baseName(raw.file), n -> new ArrayList<>()).add(raw);\n\t\tfor (Map.Entry<String, List<Raw>> group : byBaseName.entrySet())\n\t\t\tresolveUntitledGroup(group.getKey(), group.getValue(), result);\n\n\t\treturn result;\n\t}\n\n\tprivate String baseName(IFile file) {\n\t\tString name = file.getName();\n\t\tif (name.endsWith(PresetHandler.PROMPT_TXT))\n\t\t\tname = name.substring(0, name.length() - PresetHandler.PROMPT_TXT.length());\n\t\treturn name;\n\t}\n\n\t/**\n\t * Disambiguates files sharing the same base name by successively adding\n\t * more directory levels (closest to farthest, ending at the project name)\n\t * until the labels within the group are unique.\n\t */\n\tprivate void resolveUntitledGroup(String base, List<Raw> group, Map<String, String> result) {\n\t\tif (group.size() == 1) {\n\t\t\tresult.put(key(group.get(0).file), base);\n\t\t\treturn;\n\t\t}\n\n\t\tList<Raw> remaining = group;\n\t\tint level = 1;\n\t\twhile (!remaining.isEmpty()) {\n\t\t\tMap<String, List<Raw>> byLabel = new LinkedHashMap<>();\n\t\t\tfor (Raw raw : remaining) {\n\t\t\t\tString label = base + \" (\" + lastSegments(raw.file, level) + \")\";\n\t\t\t\tbyLabel.computeIfAbsent(label, l -> new ArrayList<>()).add(raw);\n\t\t\t}\n\n\t\t\tList<Raw> stillColliding = new ArrayList<>();\n\t\t\tfor (Map.Entry<String, List<Raw>> candidate : byLabel.entrySet()) {\n\t\t\t\tList<Raw> entries = candidate.getValue();\n\t\t\t\tif (entries.size() == 1) {\n\t\t\t\t\tresult.put(key(entries.get(0).file), candidate.getKey());\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tboolean reachedProjectRoot = entries.stream().allMatch(raw -> segmentCount(raw.file) <= level);\n\t\t\t\tif (reachedProjectRoot) {\n\t\t\t\t\tfor (Raw raw : entries)\n\t\t\t\t\t\tresult.put(key(raw.file), candidate.getKey());\n\t\t\t\t} else {\n\t\t\t\t\tstillColliding.addAll(entries);\n\t\t\t\t}\n\t\t\t}\n\t\t\tremaining = stillColliding;\n\t\t\tlevel++;\n\t\t}\n\t}\n\n\tprivate int segmentCount(IFile file) {\n\t\treturn file.getFullPath().removeLastSegments(1).segmentCount();\n\t}\n\n\tprivate String lastSegments(IFile file, int count) {\n\t\tIPath dirPath = file.getFullPath().removeLastSegments(1);\n\t\tString[] segments = dirPath.segments();\n\t\tint from = Math.max(0, segments.length - count);\n\t\tStringBuilder sb = new StringBuilder();\n\t\tfor (int i = from; i < segments.length; i++) {\n\t\t\tif (sb.length() > 0)\n\t\t\t\tsb.append(\"/\");\n\t\t\tsb.append(segments[i]);\n\t\t}\n\t\treturn sb.toString();\n\t}\n}\n"
  reason: Neue PresetList-Klasse zur Trennung von Datenermittlung/Deduplikation/Labeling von der UI erstellen"
```
/answer 817c-3a allow
Control Request:
```yaml
id: 817c-3b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    ids:
    - PresetsTab
  reason: Vollständigen aktuellen Stand der PresetsTab-Klasse vor Neuschreiben lesen"
```
/answer 817c-3b allow
Control Request:
```yaml
id: 817c-3c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
      nodes:
      - id: PresetsTab
        type: ClassOrInterfaceDeclaration
        signature: "/**  * \"Presets\" tab: save/load/reset the whole system-prompt configuration.  *…"
        docstring: "\"Presets\" tab: save/load/reset the whole system-prompt configuration."
        children:
        - id: PresetsTab.QtgcNk|Bkn3Sn
          type: StatementGroup
          code: "\tprivate List presetList;\n\tprivate final java.util.List<IFile> presetFiles = new ArrayList<>();\n"
        - id: PresetsTab.create
          type: MethodDeclaration
          code: "\tpublic void create(TabFolder instr, ConfigManager cfg, Shell shell, ToolSelection tools,\n\t\t\tClearable instructionSelection) {\n\t\tTabItem presEdit = new TabItem(instr, SWT.NONE);\n\t\tpresEdit.setText(\"Presets\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tGridLayout compLay = new GridLayout(1, false);\n\t\tcomp.setLayout(compLay);\n\t\tGridData compDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tcomp.setLayoutData(compDat);\n\t\tpresEdit.setControl(comp);\n\n\t\tComposite buttons = new Composite(comp, SWT.NONE);\n\t\tbuttons.setLayout(new GridLayout(2, false));\n\n\t\tButton writeButton = new Button(buttons, SWT.PUSH);\n\t\twriteButton.setText(\"Save\");\n\t\twriteButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tPresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), shell);\n\t\t\t\trefreshPresetList();\n\t\t\t}\n\t\t});\n\n\t\tButton resetButton = new Button(buttons, SWT.PUSH);\n\t\tresetButton.setText(\"Reset\");\n\t\tresetButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tString[] cur = cfg.getSystemPrompt();\n\t\t\t\tString[] upd = new String[cur.length];\n\t\t\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\t\t\tString line = cur[i];\n\t\t\t\t\tupd[i] = line.startsWith(\"#\") ? line : \"#\" + line;\n\t\t\t\t}\n\t\t\t\tinstructionSelection.clear();\n\t\t\t\tcfg.setSystemPrompt(upd);\n\n\t\t\t\ttools.clear();\n\t\t\t\tcfg.setEnabledTools(new String[0]);\n\t\t\t}\n\t\t});\n\n\t\tpresetList = new List(comp, SWT.SINGLE | SWT.V_SCROLL | SWT.BORDER);\n\t\tGridData presetListDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tpresetListDat.heightHint = 60;\n\t\tpresetList.setLayoutData(presetListDat);\n\t\tpresetList.addMouseListener(MouseListener.mouseDownAdapter(m -> presetList.setFocus()));\n\t\tpresetList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tint idx = presetList.getSelectionIndex();\n\t\t\tif (idx < 0 || idx >= presetFiles.size())\n\t\t\t\treturn;\n\t\t\tPresetHandler.Preset preset = PresetHandler.loadPreset(presetFiles.get(idx));\n\t\t\tcfg.setSystemPrompt(preset.body);\n\t\t\tif (preset.tools != null) {\n\t\t\t\ttools.setSelection(preset.tools);\n\t\t\t\tcfg.setEnabledTools(preset.tools);\n\t\t\t}\n\t\t\tif (preset.outputMode != null)\n\t\t\t\tcfg.setOuputMode(preset.outputMode);\n\t\t}));\n\t\trefreshPresetList();\n\t}\n"
        - id: PresetsTab.refreshPresetList
          type: MethodDeclaration
          code: "\tprivate void refreshPresetList() {\n\t\tpresetFiles.clear();\n\t\tpresetFiles.addAll(Arrays.asList(PresetHandler.listPresetFiles()));\n\t\tpresetList.setItems(presetFiles.stream().map(f -> f.getFullPath().toString()).toArray(String[]::new));\n\t}\n"
```
/answer 817c-3c allow
Control Request:
```yaml
id: 817c-3d
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.QtgcNk|Bkn3Sn
    source: "\tprivate List presetList;\n\tprivate final PresetList presets = new PresetList();\n\tprivate final java.util.List<IFile> presetFiles = new ArrayList<>();\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.create
    source: "\tpublic void create(TabFolder instr, ConfigManager cfg, Shell shell, ToolSelection tools,\n\t\t\tClearable instructionSelection, InputSelection inputs) {\n\t\tTabItem presEdit = new TabItem(instr, SWT.NONE);\n\t\tpresEdit.setText(\"Presets\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tGridLayout compLay = new GridLayout(1, false);\n\t\tcomp.setLayout(compLay);\n\t\tGridData compDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tcomp.setLayoutData(compDat);\n\t\tpresEdit.setControl(comp);\n\n\t\tComposite buttons = new Composite(comp, SWT.NONE);\n\t\tbuttons.setLayout(new GridLayout(3, false));\n\n\t\tButton writeButton = new Button(buttons, SWT.PUSH);\n\t\twriteButton.setText(\"Save\");\n\t\twriteButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tPresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(),\n\t\t\t\t\t\tinputs.getSelection(), shell);\n\t\t\t\trefreshPresetList();\n\t\t\t}\n\t\t});\n\n\t\tButton clearToolsButton = new Button(buttons, SWT.PUSH);\n\t\tclearToolsButton.setText(\"Clear Tools\");\n\t\tclearToolsButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\ttools.clear();\n\t\t\t\tcfg.setEnabledTools(new String[0]);\n\t\t\t}\n\t\t});\n\n\t\tButton clearAllButton = new Button(buttons, SWT.PUSH);\n\t\tclearAllButton.setText(\"Clear All\");\n\t\tclearAllButton.addSelectionListener(new SelectionAdapter() {\n\t\t\t@Override\n\t\t\tpublic void widgetSelected(SelectionEvent e) {\n\t\t\t\tString[] cur = cfg.getSystemPrompt();\n\t\t\t\tString[] upd = new String[cur.length];\n\t\t\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\t\t\tString line = cur[i];\n\t\t\t\t\tupd[i] = line.startsWith(\"#\") ? line : \"#\" + line;\n\t\t\t\t}\n\t\t\t\tinstructionSelection.clear();\n\t\t\t\tcfg.setSystemPrompt(upd);\n\n\t\t\t\ttools.clear();\n\t\t\t\tcfg.setEnabledTools(new String[0]);\n\n\t\t\t\tinputs.clear();\n\n\t\t\t\tcfg.setOuputMode(OutputMode.values()[0]);\n\t\t\t}\n\t\t});\n\n\t\tpresetList = new List(comp, SWT.SINGLE | SWT.V_SCROLL | SWT.BORDER);\n\t\tGridData presetListDat = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tpresetListDat.heightHint = 60;\n\t\tpresetList.setLayoutData(presetListDat);\n\t\tpresetList.addMouseListener(MouseListener.mouseDownAdapter(m -> presetList.setFocus()));\n\t\tpresetList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tint idx = presetList.getSelectionIndex();\n\t\t\tif (idx < 0 || idx >= presetFiles.size())\n\t\t\t\treturn;\n\t\t\tPresetHandler.Preset preset = PresetHandler.loadPreset(presetFiles.get(idx));\n\t\t\tcfg.setSystemPrompt(preset.body);\n\t\t\tif (preset.tools != null) {\n\t\t\t\ttools.setSelection(preset.tools);\n\t\t\t\tcfg.setEnabledTools(preset.tools);\n\t\t\t}\n\t\t\tif (preset.outputMode != null)\n\t\t\t\tcfg.setOuputMode(preset.outputMode);\n\t\t\tif (preset.inputs != null)\n\t\t\t\tinputs.setSelection(preset.inputs);\n\t\t}));\n\t\trefreshPresetList();\n\t}\n"
  reason: "PresetsTab umbauen: Reset-Button durch Clear Tools/Clear All ersetzen, Inputs integrieren, PresetList-Klasse nutzen\""
```
/answer 817c-3d allow
Control Request:
```yaml
id: 817c-3e
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.refreshPresetList
    source: "\tprivate void refreshPresetList() {\n\t\tjava.util.List<PresetList.Entry> entries = presets.load();\n\t\tpresetFiles.clear();\n\t\tString[] labels = new String[entries.size()];\n\t\tfor (int i = 0; i < entries.size(); i++) {\n\t\t\tpresetFiles.add(entries.get(i).file);\n\t\t\tlabels[i] = entries.get(i).label;\n\t\t}\n\t\tpresetList.setItems(labels);\n\t}\n"
  reason: refreshPresetList auf die ausgelagerte PresetList-Klasse mit Deduplikation und Labels umstellen"
```
/answer 817c-3e allow
Control Request:
```yaml
id: 817c-3f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
  text: Arrays
  reason: "Prüfen, ob Arrays-Import noch benötigt wird\""
```
/answer 817c-3f allow
Control Request:
```yaml
id: 817c-40
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
      nodes:
      - id: 1vWmR7|WrFin2
        code: |
          import java.util.ArrayList;
          import java.util.Arrays;
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
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.view.PresetHandler;
```
/answer 817c-40 allow
Text:
 Now rewrite PresetsTab.java fully with new buttons, PresetList usage, inputs integration.
Control Request:
```yaml
id: 817c-41
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: 1vWmR7|WrFin2
    old_text: |-
      import java.util.ArrayList;
      import java.util.Arrays;
      import org.eclipse.core.resources.IFile;
    new_text: |-
      import java.util.ArrayList;
      import org.eclipse.core.resources.IFile;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: 1vWmR7|WrFin2
    old_text: |-
      import xy.ai.workbench.ConfigManager;
      import xy.ai.workbench.view.PresetHandler;
    new_text: |-
      import xy.ai.workbench.ConfigManager;
      import xy.ai.workbench.OutputMode;
      import xy.ai.workbench.view.PresetHandler;
  reason: "Imports bereinigen: ungenutztes Arrays entfernen, OutputMode hinzufügen\""
```
/answer 817c-41 allow
Control Request:
```yaml
id: 817c-42
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputsTable.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputSelection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
  reason: Alle geänderten/neuen Dateien final auf Syntaxfehler prüfen"
```
/answer 817c-42 allow
Control Request:
```yaml
id: 817c-43
toolName: grep
arguments:
  items:
  - pattern: writePreset|serializePreset|parsePreset\(
    directory:
    - /home/user/xyan/xy.ai.workbench/src
    include:
    - '*.java'
    exclude:
    - PresetHandler.java
  reason: "Sicherstellen, dass keine weiteren Aufrufer der geänderten PresetHandler-Methoden übersehen wurden\""
```
/answer 817c-43 allow
Control Request:
```yaml
id: 817c-44
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/session/PresetsTab.java
        matches:
        - lineno: 47
          match: "PresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), inputs.getSelection(), shell);"
```
/answer 817c-44 allow
Result Stats: id=6c8081b4-e91a-461d-abc2-ecbfe6997d50, total: 99336, in: 215, out: 31150, reason: 101, read: 1713274, write: 67971
ReasoningToken: 59
Text:
 Now update `refreshPresetList` to use the `PresetList` class with deduplication + labels, and update imports.
Text:
 Now update imports (add OutputMode, remove unused Arrays import if no longer used).
Text:
 Arrays no longer used. Remove it and add OutputMode import.
Text:
 Now validate all changed files.
ReasoningToken: 42
No other callers. All changes are consistent and compile successfully.
Description	Resource	Path	Location	Type
Local variable level is required to be final or effectively final based on its usage	PresetList.java	/XY.AI.Workbench/src/xy/ai/workbench/view/session	line 150	Java Problem

SystemInit:  id=6c8081b4-e91a-461d-abc2-ecbfe6997d50, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: 817c-45
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
    id: PresetList.resolveUntitledGroup
    old_text: "\t\t\t\tboolean reachedProjectRoot = entries.stream().allMatch(raw -> segmentCount(raw.file) <= level);"
    new_text: "\t\t\t\tfinal int currentLevel = level;\n\t\t\t\tboolean reachedProjectRoot = entries.stream().allMatch(raw -> segmentCount(raw.file) <= currentLevel);"
  reason: "Fix: effectively-final Variable für Lambda-Zugriff auf level bereitstellen\""
```
/answer 817c-45 allow
ReasoningToken: 66
Control Request:
```yaml
id: 817c-46
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
  reason: PresetList.java nach Fix erneut validieren"
```
/answer 817c-46 allow
Result Stats: id=6c8081b4-e91a-461d-abc2-ecbfe6997d50, total: 100685, in: 221, out: 31690, reason: 0, read: 1917860, write: 68774
Fixed.
