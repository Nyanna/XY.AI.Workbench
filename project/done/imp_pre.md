Setzte folgende Verbesserungen für die Presets um.

- das Verwenden der "Save" Funktion soll anschließen die Presetliste neu laden
- Wird beim eine bestehende Promptdatei überschrieben sollen zwei Punkte der bestehenden Datei übernommen werden:
	1. Es soll ein bestehender "title" übernommen werden
	2. Es soll eine Auskommentierung bestehender Felder des YAML Front Matter übernommen werden 
- Die Parameter modell, reasoning, cache, profile sollen ebenfalls gespeichert werden aber initial Zeilenweise auskommentiert. Es sei denn eine bestehende Promptdatei wird überschrieben dann wird der Kommentierungszustand aber nicht der Feldinhalt übernommen 

`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java`
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java`
SystemInit:  id=9cf9f8ce-4d69-467a-804e-0750c702353d, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 3429-4d-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
  reason: Struktur der beiden Dateien verstehen
```
/answer 3429-4d-0 allow
Control Request:
```yaml
id: 3429-4e-4d
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
        nodes:
        - id: 1vWmR7|l8ZbXg
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
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
        nodes:
        - id: 1vWmR7|KBjIcT
          excert: import java.util.ArrayList;
        - id: PresetList
          excert: "/**  * Discovers prompt preset files across the workspace, deduplicates them by…"
          docstring: "Discovers prompt preset files across the workspace, deduplicates them by"
          children:
          - id: PresetList.Entry
            excert: "public static class Entry {"
            children:
            - id: PresetList.Entry.4CPtF5|P6LD4w
              excert: public final IFile file;
            - id: PresetList.Entry.Entry
              excert: "Entry(IFile file, String label)"
          - id: PresetList.Raw
            excert: "private static class Raw {"
          - id: PresetList.load
            excert: public List<Entry> load()
          - id: PresetList.readTitle
            excert: private String readTitle(IFile file)
          - id: PresetList.dedupe
            excert: "private List<IFile> dedupe(IFile[] files)"
          - id: PresetList.key
            excert: private String key(IFile file)
          - id: PresetList.computeLabels
            excert: "private Map<String, String> computeLabels(List<Raw> raws)"
          - id: PresetList.baseName
            excert: private String baseName(IFile file)
          - id: PresetList.resolveUntitledGroup
            excert: "private void resolveUntitledGroup(String base, List<Raw> group, Map<String, Str…"
          - id: PresetList.segmentCount
            excert: private int segmentCount(IFile file)
          - id: PresetList.lastSegments
            excert: "private String lastSegments(IFile file, int count)"
```
/answer 3429-4e-4d allow
Control Request:
```yaml
id: 3429-4f-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    ids:
    - PresetsTab
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
    ids:
    - PresetList.load
    - PresetList.readTitle
  reason: Quelltext lesen
```
/answer 3429-4f-0 allow
Control Request:
```yaml
id: 3429-50-4f
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
        nodes:
        - id: PresetsTab
          type: ClassOrInterfaceDeclaration
          excert: "/**  * \"Presets\" tab: save/load/reset the whole system-prompt configuration.  *…"
          docstring: "\"Presets\" tab: save/load/reset the whole system-prompt configuration."
          children:
          - id: PresetsTab.QtgcNk|ll4T02
            type: StatementGroup
            code: |2
                private List presetList;
                private final PresetList presets = new PresetList();
                private final java.util.List<IFile> presetFiles = new ArrayList<>();
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
                      PresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), inputs.getSelection(), rules.getSelection(), shell);
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
                  }));
                  refreshPresetList();
                }
          - id: PresetsTab.refreshPresetList
            type: MethodDeclaration
            code: |2
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
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
        nodes:
        - id: PresetList.load
          type: MethodDeclaration
          code: |2
              public List<Entry> load() {
                List<IFile> files = dedupe(PresetHandler.listPresetFiles());
                List<Raw> raws = new ArrayList<>();
                for (IFile file : files) {
                  Raw raw = new Raw();
                  raw.file = file;
                  raw.title = readTitle(file);
                  raws.add(raw);
                }
                Map<String, String> labels = computeLabels(raws);
                List<Entry> entries = new ArrayList<>();
                for (Raw raw : raws) entries.add(new Entry(raw.file, labels.get(key(raw.file))));
                return entries;
              }
        - id: PresetList.readTitle
          type: MethodDeclaration
          code: |2
              private String readTitle(IFile file) {
                try {
                  return PresetHandler.loadPreset(file).title;
                } catch (Exception e) {
                  return null;
                }
              }
```
/answer 3429-50-4f allow
ReasoningToken: 45
Control Request:
```yaml
id: 3429-51-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: class PresetHandler
    include:
    - '*.java'
    intent: locate_for_edit
  reason: PresetHandler finden
```
/answer 3429-51-0 allow
Control Request:
```yaml
id: 3429-52-51
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src
        matches:
        - path: xy/ai/workbench/view/PresetHandler.java
          nodes:
          - id: PresetHandler
            matches:
            - "public class PresetHandler {"
```
/answer 3429-52-51 allow
Control Request:
```yaml
id: 3429-53-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
  reason: Outline
```
/answer 3429-53-0 allow
Control Request:
```yaml
id: 3429-54-53
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        nodes:
        - id: eXHs10|SJtOE0
          excert: import java.io.FileWriter;
        - id: PresetHandler
          excert: "public class PresetHandler {"
          children:
          - id: PresetHandler.uJ1JRT|RbU3NM
            excert: public static final String PROMPT_TXT = ".prompt.txt";
          - id: PresetHandler.FrontMatter
            excert: "public static class FrontMatter {"
          - id: PresetHandler.Preset
            excert: "public static class Preset {"
          - id: PresetHandler.listPresetFiles
            excert: "public static IFile[] listPresetFiles()"
          - id: PresetHandler.loadPreset
            excert: public static Preset loadPreset(IFile file)
          - id: PresetHandler.parsePreset
            excert: public static Preset parsePreset(String content)
          - id: PresetHandler.writePreset
            excert: "public static void writePreset(String[] body, String[] tools, OutputMode output…"
          - id: PresetHandler.resolveDefaultSaveDir
            excert: private static String resolveDefaultSaveDir()
          - id: PresetHandler.serializePreset
            excert: "public static String serializePreset(String[] body, String[] tools, OutputMode …"
```
/answer 3429-54-53 allow
Control Request:
```yaml
id: 3429-55-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
  reason: Gesamte Datei lesen
```
/answer 3429-55-0 allow
Control Request:
```yaml
id: 3429-56-55
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        nodes:
        - id: eXHs10|SJtOE0
          excert: import java.io.FileWriter;
        - id: PresetHandler
          excert: "public class PresetHandler {"
          children:
          - id: PresetHandler.uJ1JRT|RbU3NM
            excert: public static final String PROMPT_TXT = ".prompt.txt";
          - id: PresetHandler.FrontMatter
            excert: "public static class FrontMatter {"
          - id: PresetHandler.Preset
            excert: "public static class Preset {"
          - id: PresetHandler.listPresetFiles
            excert: "public static IFile[] listPresetFiles()"
          - id: PresetHandler.loadPreset
            excert: public static Preset loadPreset(IFile file)
          - id: PresetHandler.parsePreset
            excert: public static Preset parsePreset(String content)
          - id: PresetHandler.writePreset
            excert: "public static void writePreset(String[] body, String[] tools, OutputMode output…"
          - id: PresetHandler.resolveDefaultSaveDir
            excert: private static String resolveDefaultSaveDir()
          - id: PresetHandler.serializePreset
            excert: "public static String serializePreset(String[] body, String[] tools, OutputMode …"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java instead."
```
/answer 3429-56-55 allow
Control Request:
```yaml
id: 3429-57-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    ids:
    - PresetHandler.FrontMatter
    - PresetHandler.Preset
    - PresetHandler.listPresetFiles
    - PresetHandler.loadPreset
    - PresetHandler.parsePreset
    - PresetHandler.writePreset
    - PresetHandler.resolveDefaultSaveDir
    - PresetHandler.serializePreset
    - eXHs10|SJtOE0
  reason: Quelltext der PresetHandler-Methoden lesen
```
/answer 3429-57-0 allow
Control Request:
```yaml
id: 3429-58-57
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        nodes:
        - id: PresetHandler.FrontMatter
          type: ClassOrInterfaceDeclaration
          code: |2
              public static class FrontMatter {
                public String title;
                public List<String> tools;
                public String outputMode;
                public List<String> inputs;
                public Map<String, Map<String, Object>> rules;
              }
        - id: PresetHandler.Preset
          type: ClassOrInterfaceDeclaration
          code: |2
              public static class Preset {
                public String title;
                public String[] body;
                public String[] tools;
                public OutputMode outputMode;
                public String[] inputs;
                public Map<String, Map<String, Object>> rules;
              }
        - id: PresetHandler.listPresetFiles
          type: MethodDeclaration
          code: |2
              public static IFile[] listPresetFiles() {
                List<IFile> files = new ArrayList<>();
                for (IProject project : ResourcesPlugin.getWorkspace().getRoot().getProjects()) {
                  try {
                    IResourceVisitor visitor = resource -> {
                      if (resource instanceof IFile) {
                        IFile file = (IFile) resource;
                        if (file.getName().endsWith(PROMPT_TXT))
                          files.add(file);
                      }
                      return true;
                    };
                    project.accept(visitor);
                  } catch (CoreException e) {
                    // return things like project closed
                    // LOG.error(e.getMessage(), e);
                  }
                }
                files.sort(Comparator.comparing(f -> f.getFullPath().toString()));
                return files.toArray(new IFile[0]);
              }
        - id: PresetHandler.loadPreset
          type: MethodDeclaration
          code: |2
              public static Preset loadPreset(IFile file) {
                try {
                  String content = new String(Files.readAllBytes(file.getLocation().toFile().toPath()), StandardCharsets.UTF_8);
                  return parsePreset(content);
                } catch (IOException e) {
                  throw new IllegalStateException(e);
                }
              }
        - id: PresetHandler.parsePreset
          type: MethodDeclaration
          code: |2
              public static Preset parsePreset(String content) {
                Preset preset = new Preset();
                String body = content;
                Matcher matcher = FRONT_MATTER.matcher(content);
                if (matcher.find() && matcher.start() == 0) {
                  String yaml = matcher.group(1);
                  body = content.substring(matcher.end());
                  if (yaml != null && !yaml.isBlank())
                    try {
                      FrontMatter fm = YAML.readValue(yaml, FrontMatter.class);
                      if (fm.title != null && !fm.title.trim().isEmpty())
                        preset.title = fm.title.trim();
                      if (fm.tools != null)
                        preset.tools = fm.tools.toArray(new String[0]);
                      if (fm.outputMode != null && !fm.outputMode.isBlank())
                        preset.outputMode = OutputMode.valueOf(fm.outputMode.trim());
                      if (fm.inputs != null)
                        preset.inputs = fm.inputs.toArray(new String[0]);
                      if (fm.rules != null)
                        preset.rules = fm.rules;
                    } catch (Exception e) {
                      LOG.error(e.getMessage(), e);
                    }
                }
                if (body.endsWith("\n"))
                  body = body.substring(0, body.length() - 1);
                preset.body = body.split("\n", -1);
                return preset;
              }
        - id: PresetHandler.writePreset
          type: MethodDeclaration
          code: |2
              public static void writePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, Shell shell) {
                FileDialog dialog = new FileDialog(shell, SWT.SAVE);
                dialog.setFilterPath(resolveDefaultSaveDir());
                dialog.setFilterExtensions(new String[] { "*.prompt.txt" });
                dialog.setFilterNames(new String[] { "Prompt Files (*prompt.txt)" });
                String filePath = dialog.open();
                if (filePath == null)
                  return;
                String content = serializePreset(body, tools, outputMode, inputs, rules, "");
                try (PrintWriter writer = new PrintWriter(new FileWriter(filePath, StandardCharsets.UTF_8))) {
                  writer.print(content);
                } catch (IOException e) {
                  throw new IllegalStateException(e);
                }
              }
        - id: PresetHandler.resolveDefaultSaveDir
          type: MethodDeclaration
          code: |2
              private static String resolveDefaultSaveDir() {
                try {
                  java.nio.file.Path projectPath = xy.ai.workbench.Activator.getDefault().editorListener.resolveProjectPath();
                  if (projectPath != null)
                    return projectPath.toString();
                } catch (Exception e) {
                  // fall back to first project below
                }
                IProject[] projects = ResourcesPlugin.getWorkspace().getRoot().getProjects();
                return projects.length > 0 ? projects[0].getLocation().toOSString() : "";
              }
        - id: PresetHandler.serializePreset
          type: MethodDeclaration
          code: |2
              public static String serializePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, String title) {
                FrontMatter fm = new FrontMatter();
                fm.title = title != null ? title : "";
                if (tools != null && tools.length > 0)
                  fm.tools = Arrays.asList(tools);
                if (outputMode != null)
                  fm.outputMode = outputMode.name();
                if (inputs != null && inputs.length > 0)
                  fm.inputs = Arrays.asList(inputs);
                if (rules != null && !rules.isEmpty())
                  fm.rules = rules;
                StringBuilder sb = new StringBuilder();
                try {
                  String yaml = YAML.writeValueAsString(fm);
                  sb.append("---\n");
                  sb.append(yaml);
                  sb.append("---\n");
                } catch (Exception e) {
                  LOG.error(e.getMessage(), e);
                }
                sb.append(String.join("\n", body));
                return sb.toString();
              }
        - id: eXHs10|SJtOE0
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
            import java.util.Map;
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
            import xy.ai.conscious.core.LOG;
            import xy.ai.conscious.core.OutputMode;
```
/answer 3429-58-57 allow
ReasoningToken: 149
Control Request:
```yaml
id: 3429-59-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: serializePreset|writePreset|parsePreset|loadPreset|FRONT_MATTER|YAML =
    include:
    - '*.java'
    intent: locate_for_edit
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: (getModel|getReasoning|getCache|getProfile|setModel|setReasoning|setCache|setProfile)\(
    include:
    - ConfigManager.java
    intent: locate_for_edit
  reason: Aufrufer und ConfigManager-Getter für model/reasoning/cache/profile finden
```
/answer 3429-59-0 allow
Control Request:
```yaml
id: 3429-5a-59
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: src/xy/ai/workbench/view/PresetHandler.java
          nodes:
          - id: PresetHandler.uJ1JRT|RbU3NM
            matches:
            - "private static final Pattern FRONT_MATTER = Pattern.compile(\"\\\\A---\\\\s*\\\\n(.*?\\\\n)?---\\\\s*\\\\n?\", Pattern.DOTALL);"
            - private static final ObjectMapper YAML = new ObjectMapper(new YAMLFactory().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER));
          - id: PresetHandler.loadPreset
            matches:
            - "public static Preset loadPreset(IFile file) {"
            - return parsePreset(content);
          - id: PresetHandler.parsePreset
            matches:
            - "public static Preset parsePreset(String content) {"
            - Matcher matcher = FRONT_MATTER.matcher(content);
          - id: PresetHandler.writePreset
            matches:
            - "public static void writePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, Shell shell) {"
            - "String content = serializePreset(body, tools, outputMode, inputs, rules, \"\");"
          - id: PresetHandler.serializePreset
            matches:
            - "public static String serializePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, String title) {"
        - path: src/xy/ai/workbench/view/session/PresetList.java
          nodes:
          - id: PresetList.readTitle
            matches:
            - return PresetHandler.loadPreset(file).title;
        - path: src/xy/ai/workbench/view/session/PresetsTab.java
          nodes:
          - id: PresetsTab.create
            matches:
            - "PresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), inputs.getSelection(), rules.getSelection(), shell);"
            - PresetHandler.Preset preset = PresetHandler.loadPreset(presetFiles.get(idx));
        - path: connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
          nodes:
          - id: AnswerParser.W4ObVs|pztlqt
            matches:
            - private static final YamlRenderer YAML = new YamlRenderer();
        - path: connector/src/xy/ai/conscious/connector/harness/SessionRenderer.java
          nodes:
          - id: SessionRenderer.97qJP9|Yi2Gjf
            matches:
            - private static final YamlRenderer YAML = new YamlRenderer();
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: connector/src/xy/ai/conscious/core/ConfigManager.java
          nodes:
          - id: ConfigManager.setModel
            matches:
            - "public void setModel(Model model) {"
            - "if (Objects.equals(cfg.getModel(), model))"
            - cfg.setModel(model);
            - "setReasoning(getCapabilities().getReasonings()[0]);"
            - "setProfile(profiles.length > 0 ? profiles[0] : null);"
          - id: ConfigManager.setReasoning
            matches:
            - "public void setReasoning(Reasoning reasoning) {"
            - "if (Objects.equals(cfg.getReasoning(), reasoning))"
            - cfg.setReasoning(reasoning);
          - id: ConfigManager.setProfile
            matches:
            - "public void setProfile(AgentProfile profile) {"
            - "if (Objects.equals(cfg.getProfile(), profile))"
            - cfg.setProfile(profile);
          - id: ConfigManager.getModel
            matches:
            - "public Model getModel() {"
            - return cfg.getModel();
          - id: ConfigManager.getProfile
            matches:
            - "public AgentProfile getProfile() {"
            - return cfg.getProfile();
        warnings:
        - "Limit of 15 matches reached (total 18); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 3429-5a-59 allow
Control Request:
```yaml
id: 3429-5b-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core
    pattern: public .* (get|set|is)(Model|Reasoning|Cache|Caching|Profile)\w*\(
    include:
    - ConfigManager.java
    intent: locate_for_edit
  reason: "Getter/Setter für Modell, Reasoning, Cache, Profile"
```
/answer 3429-5b-0 allow
Control Request:
```yaml
id: 3429-5c-5b
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core
        matches:
        - path: ConfigManager.java
          nodes:
          - id: ConfigManager.setModel
            matches:
            - "public void setModel(Model model) {"
          - id: ConfigManager.setReasoningBudget
            matches:
            - "public void setReasoningBudget(Integer reasoningBudget) {"
          - id: ConfigManager.setReasoning
            matches:
            - "public void setReasoning(Reasoning reasoning) {"
          - id: ConfigManager.setCacheMode
            matches:
            - "public void setCacheMode(CacheMode cacheMode) {"
          - id: ConfigManager.setProfile
            matches:
            - "public void setProfile(AgentProfile profile) {"
          - id: ConfigManager.getModel
            matches:
            - "public Model getModel() {"
          - id: ConfigManager.getReasoningBudget
            matches:
            - "public Integer getReasoningBudget() {"
          - id: ConfigManager.getProfile
            matches:
            - "public AgentProfile getProfile() {"
          - id: ConfigManager.getReasoning
            matches:
            - "public Reasoning getReasoning() {"
          - id: ConfigManager.getCacheMode
            matches:
            - "public CacheMode getCacheMode() {"
          - id: ConfigManager.getReasonings
            matches:
            - "public String[] getReasonings() {"
```
/answer 3429-5c-5b allow
ReasoningToken: 42
Control Request:
```yaml
id: 3429-5d-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: (enum|class|interface|record) (Model|Reasoning|CacheMode|AgentProfile)\b
    include:
    - '*.java'
    intent: locate_for_edit
  reason: "Typen von Model, Reasoning, CacheMode, AgentProfile klären"
```
/answer 3429-5d-0 allow
Control Request:
```yaml
id: 3429-5e-5d
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src
        matches:
        - path: xy/ai/conscious/model/Reasoning.java
          nodes:
          - id: Reasoning
            matches:
            - "public enum Reasoning {"
        - path: xy/ai/conscious/model/Model.java
          nodes:
          - id: Model
            matches:
            - "public class Model {"
        - path: xy/ai/conscious/model/CacheMode.java
          nodes:
          - id: CacheMode
            matches:
            - "public enum CacheMode {"
        - path: xy/ai/conscious/core/AgentProfile.java
          nodes:
          - id: AgentProfile
            matches:
            - "public enum AgentProfile {"
```
/answer 3429-5e-5d allow
Control Request:
```yaml
id: 3429-5f-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/Model.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
    ids:
    - ConfigManager.setModel
    - ConfigManager.setReasoning
    - ConfigManager.setCacheMode
    - ConfigManager.setProfile
    - ConfigManager.getModel
  reason: Model-Klasse und Setter-Logik ansehen
```
/answer 3429-5f-0 allow
Control Request:
```yaml
id: 3429-60-5f
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/Model.java
        nodes:
        - id: HBOdGy|jhfM1U
          excert: import java.util.Objects;
        - id: Model
          excert: "public class Model {"
          children:
          - id: Model.POnqvV|mXa3RJ
            excert: public final String apiName;
          - id: Model.Model
            excert: "public Model(String apiName, String displayName, Capabilities cap)"
          - id: Model.Model_2
            excert: "public Model(String apiName, Capabilities cap)"
          - id: Model.toString
            excert: public String toString()
          - id: Model.matches
            excert: "public boolean matches(KeyPattern provider, String displayName)"
          - id: Model.KeyPattern
            excert: "public static enum KeyPattern {"
            children:
            - id: Model.KeyPattern.s6rIgg|EdEm71
              excert: "OpenAI(\"^sk-proj-.*$\"), Gemini(\"^[a-zA-Z0-9]{39}$\"), Claude(\"^sk-ant-(api|oat).…"
            - id: Model.KeyPattern.Bqnit0|2j2jUt
              excert: public final Pattern pattern;
            - id: Model.KeyPattern.KeyPattern
              excert: private KeyPattern(String pattern)
            - id: Model.KeyPattern.matches
              excert: public boolean matches(String key)
          - id: Model.MWJtgT|UiAmaW
            excert: "public static final Model NONE = new Model(\"none\", new Capabilities()//"
          - id: Model.zlmvFF|ErtsJF
            excert: "public static final Model CC_OPUS = new Model(\"opus\", \"CC Opus\", new Capabiliti…"
          - id: Model.CrfdyV|DJFmBU
            excert: "public static final Model CC_MCPC_OPUS = new Model(\"opus\", \"CC Opus (mcpc)\", ne…"
          - id: Model.defaultsFor
            excert: "public static Model[] defaultsFor(KeyPattern provider)"
            docstring: "Default (static) catalog of models for a given provider, used by the"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/Model.java instead."
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
        nodes:
        - id: ConfigManager.setModel
          type: MethodDeclaration
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
          type: MethodDeclaration
          code: |2
              public void setReasoning(Reasoning reasoning) {
                if (Objects.equals(cfg.getReasoning(), reasoning))
                  return;
                ensureOwnConfig();
                cfg.setReasoning(reasoning);
                reasonObs.forEach(c -> c.accept(cfg.reasoning));
              }
        - id: ConfigManager.setCacheMode
          type: MethodDeclaration
          code: |2
              public void setCacheMode(CacheMode cacheMode) {
                if (Objects.equals(cfg.getCacheMode(), cacheMode))
                  return;
                ensureOwnConfig();
                cfg.setCacheMode(cacheMode);
                cacheObs.forEach(c -> c.accept(cfg.cacheMode));
              }
        - id: ConfigManager.setProfile
          type: MethodDeclaration
          code: |2
              public void setProfile(AgentProfile profile) {
                if (Objects.equals(cfg.getProfile(), profile))
                  return;
                ensureOwnConfig();
                cfg.setProfile(profile);
                profileObs.forEach(c -> c.accept(cfg.profile));
              }
        - id: ConfigManager.getModel
          type: MethodDeclaration
          code: |2
              public Model getModel() {
                return cfg.getModel();
              }
```
/answer 3429-60-5f allow
ReasoningToken: 386
Control Request:
```yaml
id: 3429-61-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/Model.java
    ids:
    - Model.toString
    - Model.Model
  reason: Model-Stringdarstellung prüfen
```
/answer 3429-61-0 allow
Control Request:
```yaml
id: 3429-62-61
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/model/Model.java
        nodes:
        - id: Model.toString
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String toString() {\n\t\treturn displayName;\n\t}\n"
        - id: Model.Model
          type: ConstructorDeclaration
          code: "\tpublic Model(String apiName, String displayName, Capabilities cap) {\n\t\tthis.apiName = apiName;\n\t\tthis.displayName = displayName;\n\t\tthis.cap = cap;\n\t}\n"
```
/answer 3429-62-61 allow
ReasoningToken: 1794
Control Request:
```yaml
id: 3429-63-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    ids:
    - PresetHandler.uJ1JRT|RbU3NM
  reason: Statische Felder (YAML-Mapper-Konfiguration) lesen
```
/answer 3429-63-0 allow
Control Request:
```yaml
id: 3429-64-63
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        nodes:
        - id: PresetHandler.uJ1JRT|RbU3NM
          type: StatementGroup
          code: |2
              public static final String PROMPT_TXT = ".prompt.txt";
              private static final Pattern FRONT_MATTER = Pattern.compile("\\A---\\s*\\n(.*?\\n)?---\\s*\\n?", Pattern.DOTALL);
              private static final ObjectMapper YAML = new ObjectMapper(new YAMLFactory().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER));
```
/answer 3429-64-63 allow
ReasoningToken: 787
Control Request:
```yaml
id: 3429-65-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
    ids:
    - PresetList.dedupe
    - PresetList.key
    - PresetList
    - 1vWmR7|KBjIcT
  reason: dedupe/key in PresetList prüfen
```
/answer 3429-65-0 allow
Control Request:
```yaml
id: 3429-66-65
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
        nodes:
        - id: PresetList.dedupe
          type: MethodDeclaration
          code: |2
              private List<IFile> dedupe(IFile[] files) {
                List<IFile> result = new ArrayList<>();
                Set<String> seen = new HashSet<>();
                for (IFile file : files) if (seen.add(key(file)))
                  result.add(file);
                return result;
              }
        - id: PresetList.key
          type: MethodDeclaration
          code: |2
              private String key(IFile file) {
                return file.getLocation() != null ? file.getLocation().toOSString() : file.getFullPath().toOSString();
              }
        - id: PresetList
          type: ClassOrInterfaceDeclaration
          excert: "/**  * Discovers prompt preset files across the workspace, deduplicates them by…"
          docstring: "Discovers prompt preset files across the workspace, deduplicates them by"
          children:
          - id: PresetList.Entry
            type: ClassOrInterfaceDeclaration
            excert: "public static class Entry {"
            children:
            - id: PresetList.Entry.4CPtF5|P6LD4w
              type: StatementGroup
              code: |2
                    public final IFile file;
                    public final String label;
            - id: PresetList.Entry.Entry
              type: ConstructorDeclaration
              code: |2
                    Entry(IFile file, String label) {
                      this.file = file;
                      this.label = label;
                    }
          - id: PresetList.Raw
            type: ClassOrInterfaceDeclaration
            code: |2
                private static class Raw {
                  IFile file;
                  String title;
                }
          - id: PresetList.load
            type: MethodDeclaration
            code: |2
                public List<Entry> load() {
                  List<IFile> files = dedupe(PresetHandler.listPresetFiles());
                  List<Raw> raws = new ArrayList<>();
                  for (IFile file : files) {
                    Raw raw = new Raw();
                    raw.file = file;
                    raw.title = readTitle(file);
                    raws.add(raw);
                  }
                  Map<String, String> labels = computeLabels(raws);
                  List<Entry> entries = new ArrayList<>();
                  for (Raw raw : raws) entries.add(new Entry(raw.file, labels.get(key(raw.file))));
                  return entries;
                }
          - id: PresetList.readTitle
            type: MethodDeclaration
            code: |2
                private String readTitle(IFile file) {
                  try {
                    return PresetHandler.loadPreset(file).title;
                  } catch (Exception e) {
                    return null;
                  }
                }
          - id: PresetList.dedupe
            type: MethodDeclaration
            code: |2
                private List<IFile> dedupe(IFile[] files) {
                  List<IFile> result = new ArrayList<>();
                  Set<String> seen = new HashSet<>();
                  for (IFile file : files) if (seen.add(key(file)))
                    result.add(file);
                  return result;
                }
          - id: PresetList.key
            type: MethodDeclaration
            code: |2
                private String key(IFile file) {
                  return file.getLocation() != null ? file.getLocation().toOSString() : file.getFullPath().toOSString();
                }
          - id: PresetList.computeLabels
            type: MethodDeclaration
            code: |2
                private Map<String, String> computeLabels(List<Raw> raws) {
                  Map<String, String> result = new HashMap<>();
                  Map<String, List<Raw>> titled = new LinkedHashMap<>();
                  List<Raw> untitled = new ArrayList<>();
                  for (Raw raw : raws) {
                    if (raw.title != null && !raw.title.trim().isEmpty())
                      titled.computeIfAbsent(raw.title.trim(), t -> new ArrayList<>()).add(raw);
                    else
                      untitled.add(raw);
                  }
                  for (Map.Entry<String, List<Raw>> group : titled.entrySet()) {
                    boolean ambiguous = group.getValue().size() > 1;
                    for (Raw raw : group.getValue()) {
                      String label = ambiguous ? group.getKey() + " (" + raw.file.getProject().getName() + ")" : group.getKey();
                      result.put(key(raw.file), label);
                    }
                  }
                  Map<String, List<Raw>> byBaseName = new LinkedHashMap<>();
                  for (Raw raw : untitled) byBaseName.computeIfAbsent(baseName(raw.file), n -> new ArrayList<>()).add(raw);
                  for (Map.Entry<String, List<Raw>> group : byBaseName.entrySet()) resolveUntitledGroup(group.getKey(), group.getValue(), result);
                  return result;
                }
          - id: PresetList.baseName
            type: MethodDeclaration
            code: |2
                private String baseName(IFile file) {
                  String name = file.getName();
                  if (name.endsWith(PresetHandler.PROMPT_TXT))
                    name = name.substring(0, name.length() - PresetHandler.PROMPT_TXT.length());
                  return name;
                }
          - id: PresetList.resolveUntitledGroup
            type: MethodDeclaration
            code: |2
                private void resolveUntitledGroup(String base, List<Raw> group, Map<String, String> result) {
                  if (group.size() == 1) {
                    result.put(key(group.get(0).file), base);
                    return;
                  }
                  List<Raw> remaining = group;
                  int level = 1;
                  while (!remaining.isEmpty()) {
                    Map<String, List<Raw>> byLabel = new LinkedHashMap<>();
                    for (Raw raw : remaining) {
                      String label = base + " (" + lastSegments(raw.file, level) + ")";
                      byLabel.computeIfAbsent(label, l -> new ArrayList<>()).add(raw);
                    }
                    List<Raw> stillColliding = new ArrayList<>();
                    for (Map.Entry<String, List<Raw>> candidate : byLabel.entrySet()) {
                      List<Raw> entries = candidate.getValue();
                      if (entries.size() == 1) {
                        result.put(key(entries.get(0).file), candidate.getKey());
                        continue;
                      }
                      final int currentLevel = level;
                      boolean reachedProjectRoot = entries.stream().allMatch(raw -> segmentCount(raw.file) <= currentLevel);
                      if (reachedProjectRoot)
                        for (Raw raw : entries) result.put(key(raw.file), candidate.getKey());
                      else
                        stillColliding.addAll(entries);
                    }
                    remaining = stillColliding;
                    level++;
                  }
                }
          - id: PresetList.segmentCount
            type: MethodDeclaration
            code: |2
                private int segmentCount(IFile file) {
                  return file.getFullPath().removeLastSegments(1).segmentCount();
                }
          - id: PresetList.lastSegments
            type: MethodDeclaration
            code: |2
                private String lastSegments(IFile file, int count) {
                  IPath dirPath = file.getFullPath().removeLastSegments(1);
                  String[] segments = dirPath.segments();
                  int from = Math.max(0, segments.length - count);
                  StringBuilder sb = new StringBuilder();
                  for (int i = from; i < segments.length; i++) {
                    if (sb.length() > 0)
                      sb.append("/");
                    sb.append(segments[i]);
                  }
                  return sb.toString();
                }
        - id: 1vWmR7|KBjIcT
          type: ImportGroup
          code: |
            import java.util.ArrayList;
            import java.util.HashMap;
            import java.util.HashSet;
            import java.util.LinkedHashMap;
            import java.util.List;
            import java.util.Map;
            import java.util.Set;

            import org.eclipse.core.resources.IFile;
            import org.eclipse.core.runtime.IPath;

            import xy.ai.workbench.view.PresetHandler;
```
/answer 3429-66-65 allow
ReasoningToken: 3791
Control Request:
```yaml
id: 3429-67-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.FrontMatter
    source: |2
        public static class FrontMatter {
          public String title;
          public List<String> tools;
          public String outputMode;
          public List<String> inputs;
          public Map<String, Map<String, Object>> rules;
          public String model;
          public String reasoning;
          public String cache;
          public String profile;
        }

        /** Session settings stored (initially commented out) in the front matter. */
        public static class Settings {
          public String model;
          public String reasoning;
          public String cache;
          public String profile;

          public static Settings of(Model model, Reasoning reasoning, CacheMode cache, AgentProfile profile) {
            Settings s = new Settings();
            s.model = model != null ? model.toString() : null;
            s.reasoning = reasoning != null ? reasoning.name() : null;
            s.cache = cache != null ? cache.name() : null;
            s.profile = profile != null ? profile.name() : null;
            return s;
          }
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.Preset
    source: |2
        public static class Preset {
          public String title;
          public String[] body;
          public String[] tools;
          public OutputMode outputMode;
          public String[] inputs;
          public Map<String, Map<String, Object>> rules;
          public String model;
          public String reasoning;
          public String cache;
          public String profile;
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.writePreset
    source: |2
        /**
         * Asks for a target file and writes the preset.
         *
         * @return true if a file was written
         */
        public static boolean writePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, Settings settings, Shell shell) {
          FileDialog dialog = new FileDialog(shell, SWT.SAVE);
          dialog.setFilterPath(resolveDefaultSaveDir());
          dialog.setFilterExtensions(new String[] { "*.prompt.txt" });
          dialog.setFilterNames(new String[] { "Prompt Files (*prompt.txt)" });
          String filePath = dialog.open();
          if (filePath == null)
            return false;
          String existing = null;
          java.nio.file.Path target = java.nio.file.Paths.get(filePath);
          if (Files.isRegularFile(target))
            try {
              existing = new String(Files.readAllBytes(target), StandardCharsets.UTF_8);
            } catch (IOException e) {
              LOG.error(e.getMessage(), e);
            }
          String content = serializePreset(body, tools, outputMode, inputs, rules, "", settings, existing);
          try (PrintWriter writer = new PrintWriter(new FileWriter(filePath, StandardCharsets.UTF_8))) {
            writer.print(content);
          } catch (IOException e) {
            throw new IllegalStateException(e);
          }
          refreshWorkspace(filePath);
          return true;
        }

        /** Makes the workspace aware of a file written outside of the resource API. */
        private static void refreshWorkspace(String filePath) {
          IFile[] files = ResourcesPlugin.getWorkspace().getRoot().findFilesForLocationURI(new java.io.File(filePath).toURI());
          for (IFile file : files) {
            try {
              IContainer parent = file.getParent();
              IContainer known = parent;
              while (known != null && !known.exists()) known = known.getParent();
              if (known == null)
                continue;
              known.refreshLocal(known == parent ? IResource.DEPTH_ONE : IResource.DEPTH_INFINITE, null);
            } catch (CoreException e) {
              LOG.error(e.getMessage(), e);
            }
          }
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.serializePreset
    source: |2
        /**
         * Serializes a preset. If <code>existing</code> (content of the file being
         * overwritten) is given, its title and the comment state of its front matter
         * fields are kept. The settings fields (model, reasoning, cache, profile) are
         * commented out unless the existing file has them active.
         */
        public static String serializePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, String title, Settings settings, String existing) {
          Map<String, Boolean> commented = new HashMap<>();
          String effectiveTitle = title != null ? title : "";
          if (existing != null) {
            String existingYaml = frontMatterYaml(existing);
            if (existingYaml != null) {
              commented = readCommentState(existingYaml);
              String existingTitle = readExistingTitle(existingYaml);
              if (existingTitle != null)
                effectiveTitle = existingTitle;
            }
          }
          Map<String, Object> fm = new LinkedHashMap<>();
          fm.put("title", effectiveTitle);
          if (tools != null && tools.length > 0)
            fm.put("tools", Arrays.asList(tools));
          if (outputMode != null)
            fm.put("outputMode", outputMode.name());
          if (inputs != null && inputs.length > 0)
            fm.put("inputs", Arrays.asList(inputs));
          if (rules != null && !rules.isEmpty())
            fm.put("rules", rules);
          if (settings != null) {
            if (settings.model != null)
              fm.put("model", settings.model);
            if (settings.reasoning != null)
              fm.put("reasoning", settings.reasoning);
            if (settings.cache != null)
              fm.put("cache", settings.cache);
            if (settings.profile != null)
              fm.put("profile", settings.profile);
          }
          StringBuilder sb = new StringBuilder();
          try {
            String yaml = applyComments(YAML.writeValueAsString(fm), commented);
            sb.append("---\n");
            sb.append(yaml);
            if (!yaml.endsWith("\n"))
              sb.append("\n");
            sb.append("---\n");
          } catch (Exception e) {
            LOG.error(e.getMessage(), e);
          }
          sb.append(String.join("\n", body));
          return sb.toString();
        }

        private static String frontMatterYaml(String content) {
          Matcher matcher = FRONT_MATTER.matcher(content);
          if (matcher.find() && matcher.start() == 0)
            return matcher.group(1);
          return null;
        }

        /**
         * Comment state per top-level front matter key of an existing file (true =
         * commented out). A key counts as commented only if all its occurrences are.
         */
        private static Map<String, Boolean> readCommentState(String yaml) {
          Map<String, Boolean> state = new HashMap<>();
          for (String line : yaml.split("\n")) {
            Matcher m = FIELD_LINE.matcher(line);
            if (m.find())
              state.merge(m.group(2), m.group(1) != null, (a, b) -> a && b);
          }
          return state;
        }

        /** Title of an existing file; an active title wins over a commented one. */
        private static String readExistingTitle(String yaml) {
          String commentedTitle = null;
          for (String line : yaml.split("\n")) {
            Matcher m = FIELD_LINE.matcher(line);
            if (!m.find() || !"title".equals(m.group(2)))
              continue;
            String value = parseTitle(line.substring(m.end() - "title:".length()));
            if (value == null)
              continue;
            if (m.group(1) == null)
              return value;
            if (commentedTitle == null)
              commentedTitle = value;
          }
          return commentedTitle;
        }

        private static String parseTitle(String titleLine) {
          try {
            Object value = YAML.readValue(titleLine, Map.class).get("title");
            if (value == null)
              return null;
            String text = value.toString().trim();
            return text.isEmpty() ? null : text;
          } catch (Exception e) {
            return null;
          }
        }

        /**
         * Comments out whole field blocks line by line. Fields without a known state
         * are commented out if they are settings fields, active otherwise.
         */
        private static String applyComments(String yaml, Map<String, Boolean> commented) {
          StringBuilder out = new StringBuilder();
          boolean comment = false;
          for (String line : yaml.split("\n")) {
            Matcher m = FIELD_LINE.matcher(line);
            if (m.find()) {
              String key = m.group(2);
              comment = commented.containsKey(key) ? commented.get(key) : SETTING_KEYS.contains(key);
            }
            out.append(comment ? "# " + line : line).append('\n');
          }
          return out.toString();
        }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.uJ1JRT|RbU3NM
    source: |2
        public static final String PROMPT_TXT = ".prompt.txt";
        private static final Pattern FRONT_MATTER = Pattern.compile("\\A---\\s*\\n(.*?\\n)?---\\s*\\n?", Pattern.DOTALL);
        private static final ObjectMapper YAML = new ObjectMapper(new YAMLFactory().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER));
        /** Top-level front matter key line, optionally commented out ("key:" or "# key:"). */
        private static final Pattern FIELD_LINE = Pattern.compile("^(#[ ]?)?(title|tools|outputMode|inputs|rules|model|reasoning|cache|profile):");
        private static final java.util.Set<String> SETTING_KEYS = java.util.Set.of("model", "reasoning", "cache", "profile");
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.parsePreset
    old_text: |2-
                if (fm.rules != null)
                  preset.rules = fm.rules;
    new_text: |2-
                if (fm.rules != null)
                  preset.rules = fm.rules;
                preset.model = fm.model;
                preset.reasoning = fm.reasoning;
                preset.cache = fm.cache;
                preset.profile = fm.profile;
  reason: "PresetHandler: neue Felder (model/reasoning/cache/profile), Titel- und Kommentarzustand-Übernahme beim Überschreiben, Workspace-Refresh nach Save"
```
/answer 3429-67-0 allow
Control Request:
```yaml
id: 3429-68-67
result:
  structuredContent:
    replace:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        id: PresetHandler.Preset
        new_id: PresetHandler.Preset
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        id: PresetHandler.writePreset
        new_id: PresetHandler.writePreset
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        id: PresetHandler.serializePreset
        new_id: PresetHandler.serializePreset
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        id: PresetHandler.uJ1JRT|RbU3NM
        new_id: PresetHandler.xGCufa|7QVao2
      errors:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        id: PresetHandler.FrontMatter
        error: |
          invalid type declaration: Encountered unexpected token: "public" "public"
              at line 14, column 3.

          Was expecting:

              <EOF>
    edit_block:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        id: PresetHandler.parsePreset
        new_id: PresetHandler.parsePreset
```
/answer 3429-68-67 allow
ReasoningToken: 142
Control Request:
```yaml
id: 3429-69-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.FrontMatter
    source: |2
        public static class FrontMatter {
          public String title;
          public List<String> tools;
          public String outputMode;
          public List<String> inputs;
          public Map<String, Map<String, Object>> rules;
          public String model;
          public String reasoning;
          public String cache;
          public String profile;
        }
  insert:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.FrontMatter
    position: after
    source: |2

        /** Session settings stored (initially commented out) in the front matter. */
        public static class Settings {
          public String model;
          public String reasoning;
          public String cache;
          public String profile;

          public static Settings of(Model model, Reasoning reasoning, CacheMode cache, AgentProfile profile) {
            Settings s = new Settings();
            s.model = model != null ? model.toString() : null;
            s.reasoning = reasoning != null ? reasoning.name() : null;
            s.cache = cache != null ? cache.name() : null;
            s.profile = profile != null ? profile.name() : null;
            return s;
          }
        }
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: eXHs10|SJtOE0
    old_text: import java.util.Comparator;
    new_text: |-
      import java.util.Comparator;
      import java.util.HashMap;
      import java.util.LinkedHashMap;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: eXHs10|SJtOE0
    old_text: import org.eclipse.core.resources.IFile;
    new_text: |-
      import org.eclipse.core.resources.IContainer;
      import org.eclipse.core.resources.IFile;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: eXHs10|SJtOE0
    old_text: import org.eclipse.core.resources.IResourceVisitor;
    new_text: |-
      import org.eclipse.core.resources.IResource;
      import org.eclipse.core.resources.IResourceVisitor;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: eXHs10|SJtOE0
    old_text: import xy.ai.conscious.core.LOG;
    new_text: |-
      import xy.ai.conscious.core.AgentProfile;
      import xy.ai.conscious.core.LOG;
      import xy.ai.conscious.model.CacheMode;
      import xy.ai.conscious.model.Model;
      import xy.ai.conscious.model.Reasoning;
  reason: "FrontMatter erweitern, Settings-Klasse und Imports ergänzen"
```
/answer 3429-69-0 allow
Control Request:
```yaml
id: 3429-6a-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.create
    old_text: |2-
              PresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), inputs.getSelection(), rules.getSelection(), shell);
              refreshPresetList();
    new_text: |2-
              PresetHandler.Settings settings = PresetHandler.Settings.of(cfg.getModel(), cfg.getReasoning(), cfg.getCacheMode(), cfg.getProfile());
              if (PresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), inputs.getSelection(), rules.getSelection(), settings, shell))
                refreshPresetList();
  reason: "PresetsTab: neue writePreset-Signatur verwenden und Liste nur nach erfolgreichem Speichern neu laden"
```
/answer 3429-6a-0 allow
ReasoningToken: 39
Control Request:
```yaml
id: 3429-6b-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    ids:
    - psbef3|qC0acD
    - eXHs10|BZ2cvr
    - L6sQZe|xJycJ5
    - bzpiS7|R03WO8
  reason: Importe prüfen
```
/answer 3429-6b-0 allow
Control Request:
```yaml
id: 3429-6c-6b
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
        nodes:
        - id: L6sQZe|xJycJ5
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
            import java.util.HashMap;
            import java.util.LinkedHashMap;
            import java.util.List;
            import java.util.Map;
            import java.util.regex.Matcher;
            import java.util.regex.Pattern;
            import org.eclipse.core.resources.IContainer;
            import org.eclipse.core.resources.IFile;
            import org.eclipse.core.resources.IProject;
            import org.eclipse.core.resources.IResource;
            import org.eclipse.core.resources.IResourceVisitor;
            import org.eclipse.core.resources.ResourcesPlugin;
            import org.eclipse.core.runtime.CoreException;
            import org.eclipse.swt.SWT;
            import org.eclipse.swt.widgets.FileDialog;
            import org.eclipse.swt.widgets.Shell;
            import com.fasterxml.jackson.databind.ObjectMapper;
            import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
        - id: bzpiS7|R03WO8
          type: ImportGroup
          code: |
            import com.fasterxml.jackson.dataformat.yaml.YAMLGenerator;
            import xy.ai.conscious.core.AgentProfile;
            import xy.ai.conscious.core.LOG;
            import xy.ai.conscious.model.CacheMode;
            import xy.ai.conscious.model.Model;
            import xy.ai.conscious.model.Reasoning;
            import xy.ai.conscious.core.OutputMode;
        warnings:
        - No node matched 'psbef3|qC0acD'.
        - No node matched 'eXHs10|BZ2cvr'.
```
/answer 3429-6c-6b allow
Result Stats: id=9cf9f8ce-4d69-467a-804e-0750c702353d, total: 56725, in: 34, out: 17066, reason: 37, read: 284736, write: 39625, start: 1795421760310, end: 1795421767504, time: 7133, total: 127080, io: 0, control: 61, sent: 0, recv: 193
ReasoningToken: 37
Ich habe die drei Verbesserungen umgesetzt, aber noch nicht kompiliert oder getestet. Die Änderungen stehen in `PresetsTab.java` und in `PresetHandler.java`. `PresetList.java` habe ich nicht geändert.

**1. Presetliste nach "Save" neu laden**
- Die Liste wurde nach dem Speichern schon neu geladen. Die neue Datei fehlte trotzdem, weil sie am Eclipse-Workspace vorbei geschrieben wurde und `listPresetFiles()` sie deshalb nicht sah.
- `writePreset` aktualisiert den Ordner der Datei jetzt im Workspace (`refreshLocal`) und gibt `boolean` zurück. `PresetsTab` lädt die Liste nur neu, wenn wirklich gespeichert wurde. Bricht man den Dialog ab, passiert nichts.

**2. Überschreiben einer bestehenden Promptdatei**
- **Titel:** Der bestehende `title` bleibt erhalten. Ein aktiver Titel hat Vorrang vor einem auskommentierten.
- **Auskommentierung:** Für jedes Front-Matter-Feld (`title`, `tools`, `outputMode`, `inputs`, `rules`, `model`, `reasoning`, `cache`, `profile`) wird übernommen, ob es auskommentiert war. Auskommentiert wird zeilenweise mit `# `, auch bei mehrzeiligen Feldern wie `tools` oder `rules`.

**3. Neue Felder `model`, `reasoning`, `cache`, `profile`**
- Die Werte kommen aus der aktuellen Konfiguration (`cfg`). Das Modell wird mit seinem Anzeigenamen gespeichert (`Model.toString()`), die anderen drei mit `name()`.
- Bei einer neuen Datei sind sie auskommentiert. Beim Überschreiben gilt der bisherige Kommentierungszustand. Fehlt ein Feld in der alten Datei, wird es auskommentiert geschrieben.
- Der Feldinhalt stammt immer aus der aktuellen Konfiguration.

**Zu beachten**
- **`null`-Felder:** Felder ohne Wert (zum Beispiel leere `tools`) werden nicht mehr als `tools: null` geschrieben, sondern weggelassen.
- **Laden:** Die neuen Felder werden beim Laden gelesen, damit ein einkommentiertes Feld keinen Parse-Fehler auslöst. Auf die Konfiguration angewendet werden sie nicht, weil du nur das Speichern verlangt hattest. Soll ein einkommentiertes Feld beim Laden Modell, Reasoning, Cache oder Profil setzen, ergänze ich das.
Ja, das soll es. Modell, Reasoning, Cache oder Profi soll zwar initial nur als Hint gespeichert werden aber der Nutzer soll die einkommentieren können um eine EInstellung für das Preset zu forcieren.
SystemInit:  id=9cf9f8ce-4d69-467a-804e-0750c702353d, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 75
Control Request:
```yaml
id: 3429-6d-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core
    pattern: public .*(Models|Capabilities|EnabledProfiles|getCacheMode|getReasonings|getAgentProfiles)\w*\(
    include:
    - ConfigManager.java
    - Capabilities.java
    intent: locate_for_edit
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: \.setModel\(|getModels\(|availableModels
    include:
    - '*.java'
    limit: 20
    intent: locate_for_edit
  reason: "Herausfinden, wie verfügbare Modelle/Capabilities abrufbar sind, um Preset-Werte aufzulösen"
```
/answer 3429-6d-0 allow
Control Request:
```yaml
id: 3429-6e-6d
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core
        matches:
        - path: ConfigManager.java
          nodes:
          - id: ConfigManager.getCacheMode
            matches:
            - "public CacheMode getCacheMode() {"
          - id: ConfigManager.addEnabledModelsObs
            matches:
            - "public void addEnabledModelsObs(Consumer<Model[]> obs, boolean initialize) {"
          - id: ConfigManager.addEnabledProfilesObs
            matches:
            - "public void addEnabledProfilesObs(Consumer<AgentProfile[]> obs, boolean initialize) {"
          - id: ConfigManager.setEnabledProfiles
            matches:
            - "public void setEnabledProfiles(AgentProfile[] enabledProfiles) {"
          - id: ConfigManager.getEnabledModels
            matches:
            - "public Model[] getEnabledModels() {"
          - id: ConfigManager.getReasonings
            matches:
            - "public String[] getReasonings() {"
          - id: ConfigManager.getCapabilities
            matches:
            - "public Capabilities getCapabilities() {"
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: src/xy/ai/workbench/view/session/ModelParametersSection.java
          nodes:
          - id: ModelParametersSection.createModelCombo
            matches:
            - "cfg.setModel(items[idx]);"
        - path: src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
          nodes:
          - id: ConfigManagerAdapter.loadConfig
            matches:
            - cm.setModel(resolved);
        - path: connector/src/xy/ai/conscious/core/ConfigManager.java
          nodes:
          - id: ConfigManager.setModel
            matches:
            - cfg.setModel(model);
        - path: connector/src/xy/ai/conscious/connector/claude/ClaudeConnector.java
          nodes:
          - id: ClaudeConnector.createRequest
            matches:
            - params.setModel(new AnyOfModel(TextNode.valueOf(fc.model.apiName)));
        - path: connector/src/xy/ai/conscious/connector/deepseek/DeepSeekConnector.java
          nodes:
          - id: DeepSeekConnector.createRequest
            matches:
            - requestBody.getResponseProperties().setModel(new AnyOfBodyModel(TextNode.valueOf(fc.model.apiName)));
```
/answer 3429-6e-6d allow
Control Request:
```yaml
id: 3429-6f-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
    ids:
    - ConfigManager.getEnabledModels
    - ConfigManager.getCapabilities
    - ConfigManager.getReasonings
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java
  reason: Getter und UI-Bindung der Modell-Parameter verstehen
```
/answer 3429-6f-0 allow
Control Request:
```yaml
id: 3429-70-6f
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
        nodes:
        - id: ConfigManager.getEnabledModels
          type: MethodDeclaration
          code: |2
              public Model[] getEnabledModels() {
                return enabledModels;
              }
        - id: ConfigManager.getCapabilities
          type: MethodDeclaration
          code: |2
              public Capabilities getCapabilities() {
                return cfg.model.cap;
              }
        - id: ConfigManager.getReasonings
          type: MethodDeclaration
          code: |2
              public String[] getReasonings() {
                Reasoning[] reasonings = getCapabilities().getReasonings();
                String[] options = Arrays.stream(reasonings).map((m) -> m.name()).collect(Collectors.toList()).toArray(new String[0]);
                return options;
              }
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
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ModelParametersSection.java instead."
```
/answer 3429-70-6f allow
ReasoningToken: 200
Control Request:
```yaml
id: 3429-71-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    ids:
    - 1vWmR7|l8ZbXg
  reason: Imports von PresetsTab lesen
```
/answer 3429-71-0 allow
Control Request:
```yaml
id: 3429-72-71
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
        nodes:
        - id: 1vWmR7|l8ZbXg
          type: ImportGroup
          code: |
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
            import xy.ai.conscious.core.ConfigManager;
            import xy.ai.conscious.core.OutputMode;
            import xy.ai.workbench.view.PresetHandler;
```
/answer 3429-72-71 allow
ReasoningToken: 12
Control Request:
```yaml
id: 3429-73-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: public .* (getReasonings|getCacheMode|getAgentProfiles)\(
    include:
    - Capabilities.java
    intent: locate_for_edit
  reason: Capabilities-Signaturen prüfen
```
/answer 3429-73-0 allow
Control Request:
```yaml
id: 3429-74-73
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src
        matches:
        - path: xy/ai/conscious/model/Capabilities.java
          nodes:
          - id: Capabilities.getReasonings
            matches:
            - "public Reasoning[] getReasonings() {"
          - id: Capabilities.getAgentProfiles
            matches:
            - "public AgentProfile[] getAgentProfiles() {"
          - id: Capabilities.getCacheMode
            matches:
            - "public CacheMode[] getCacheMode() {"
```
/answer 3429-74-73 allow
Control Request:
```yaml
id: 3429-75-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.create
    old_text: |2
            if (preset.rules != null)
              rules.setSelection(preset.rules);
    new_text: |2
            if (preset.rules != null)
              rules.setSelection(preset.rules);
            applySettings(cfg, preset);
  insert:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.refreshPresetList
    position: after
    source: |2

        /**
         * Forces the settings a preset has active (uncommented) in its front matter.
         * The model is set first because changing it resets reasoning, cache and
         * profile to the model's defaults. Unknown or unsupported values are ignored.
         */
        private void applySettings(ConfigManager cfg, PresetHandler.Preset preset) {
          if (preset.model != null && !preset.model.isBlank()) {
            for (Model m : cfg.getEnabledModels()) {
              if (m.toString().equalsIgnoreCase(preset.model.trim())) {
                cfg.setModel(m);
                break;
              }
            }
          }
          if (preset.reasoning != null && !preset.reasoning.isBlank()) {
            for (Reasoning r : cfg.getCapabilities().getReasonings()) {
              if (r.name().equalsIgnoreCase(preset.reasoning.trim())) {
                cfg.setReasoning(r);
                break;
              }
            }
          }
          if (preset.cache != null && !preset.cache.isBlank()) {
            for (CacheMode c : cfg.getCapabilities().getCacheMode()) {
              if (c.name().equalsIgnoreCase(preset.cache.trim())) {
                cfg.setCacheMode(c);
                break;
              }
            }
          }
          if (preset.profile != null && !preset.profile.isBlank()) {
            for (AgentProfile p : cfg.getCapabilities().getAgentProfiles()) {
              if (p.name().equalsIgnoreCase(preset.profile.trim())) {
                cfg.setProfile(p);
                break;
              }
            }
          }
        }
  reason: Beim Laden eines Presets einkommentierte model/reasoning/cache/profile auf die Konfiguration anwenden
```
/answer 3429-75-0 allow
ReasoningToken: 48
Control Request:
```yaml
id: 3429-76-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: 1vWmR7|l8ZbXg
    old_text: |-
      import xy.ai.conscious.core.ConfigManager;
      import xy.ai.conscious.core.OutputMode;
    new_text: |-
      import xy.ai.conscious.core.AgentProfile;
      import xy.ai.conscious.core.ConfigManager;
      import xy.ai.conscious.core.OutputMode;
      import xy.ai.conscious.model.CacheMode;
      import xy.ai.conscious.model.Model;
      import xy.ai.conscious.model.Reasoning;
  reason: "Imports für Model, Reasoning, CacheMode, AgentProfile ergänzen"
```
/answer 3429-76-0 allow
Result Stats: id=9cf9f8ce-4d69-467a-804e-0750c702353d, total: 67175, in: 46, out: 20427, reason: 0, read: 540075, write: 46702, start: 1795421953104, end: 1795421958835, time: 5674, total: 152352, io: 0, control: 57, sent: 0, recv: 142
Ein einkommentiertes `model`, `reasoning`, `cache` oder `profile` im Front Matter wird jetzt beim Laden des Presets auf die Konfiguration angewendet. Ich habe es nicht kompiliert und nicht getestet.

Die Änderung steht in `PresetsTab.java`. Dort setzt eine neue Methode `applySettings(cfg, preset)` nach dem Laden die Werte, die im Preset aktiv sind. Auskommentierte Felder liest der Parser nicht, sie bleiben also reine Hinweise.

- **Reihenfolge:** Das Modell wird zuerst gesetzt. `ConfigManager.setModel` setzt Reasoning, Cache und Profil auf die Standardwerte des Modells zurück. Danach überschreiben die Preset-Werte diese Standardwerte.
- **Zuordnung:** Das Modell wird über den Anzeigenamen gegen `getEnabledModels()` aufgelöst. Reasoning, Cache und Profil werden über `name()` gegen die Fähigkeiten des dann aktiven Modells aufgelöst. Die Groß- und Kleinschreibung ist egal.
- **Ungültige Werte:** Unbekannte Werte und Werte, die das Modell nicht unterstützt, werden ohne Fehler ignoriert. Die Konfiguration behält dann den Wert, den sie hat. Das Modell muss in den aktivierten Modellen stehen, sonst bleibt es unverändert.
- **UI:** Es läuft über die vorhandenen Setter, die ihre Beobachter benachrichtigen. Die Auswahlfelder sollten sich deshalb von selbst aktualisieren. Das habe ich nicht am laufenden Programm geprüft.

Das Speichern ist unverändert: Die vier Felder werden weiter initial auskommentiert geschrieben, und beim Überschreiben bleibt der bisherige Kommentierungszustand erhalten.
