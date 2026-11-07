Füge save und restore für `enabledTools` hinzu. Diese wurden bisher nicht gespeichert.

`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java`
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java`
SystemInit:  id=49cc0513-dc88-49c2-b060-5a5edd707b3b, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 66b9-f
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
  reason: "Struktur der Dateien verstehen, um enabledTools save/restore zu ergänzen"
```
/answer 66b9-f allow
Control Request:
```yaml
id: 66b9-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
      nodes:
      - id: EdXLLM|0emUU5
        signature: package xy.ai.workbench;
      - id: SNsr3c|Awhaig
        signature: /**
      - id: MementoConverter
        signature: "public class MementoConverter {"
        children:
        - id: MementoConverter.ZN7FHA|6C0rPe
          signature: private static final ObjectMapper MAPPER = new ObjectMapper();
        - id: MementoConverter.saveConfig
          signature: "public static void saveConfig(IMemento memento, SessionConfig cfg) {"
        - id: MementoConverter.loadConfig
          signature: "public static Snapshot loadConfig(IMemento memento) {"
        - id: MementoConverter.RcGVkC|vFpRv8
          signature: "/** Plain, capability-agnostic data snapshot of a {@link SessionConfig}. */"
        - id: MementoConverter.Snapshot
          signature: "public static class Snapshot {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
      nodes:
      - id: DSKSeM|ZX9Edu
        signature: package xy.ai.workbench;
      - id: uuKbBR|xxp1XO
        signature: import xy.ai.workbench.model.ModelResolverRegistry;
      - id: ConfigManager
        signature: "public class ConfigManager {"
        children:
        - id: ConfigManager.0mEcEZ|IEotgO
          signature: private ModelResolverRegistry resolverRegistry = new ModelResolverRegistry();
        - id: ConfigManager.iAEUAf|mtudNU
          signature: private List<Consumer<String>> systemFreeObs = new ArrayList<>();
        - id: ConfigManager.iAEUAf|2wRnzs
          signature: private List<Consumer<Integer>> budgetObs = new ArrayList<>();
        - id: ConfigManager.iAEUAf|OeE6ob
          signature: private List<Consumer<OutputMode>> outputModeObs = new ArrayList<>();
        - id: ConfigManager.ConfigManager
          signature: "public ConfigManager(MCPClient mcpClient, ActiveEditorListener editorListener) {"
        - id: ConfigManager.clearObserver
          signature: "public void clearObserver() {"
        - id: ConfigManager.activateEditor
          signature: "private void activateEditor(IEditorPart editor) {"
        - id: ConfigManager.resolveKey
          signature: "private static String resolveKey(IEditorPart editor) {"
        - id: ConfigManager.ensureOwnConfig
          signature: "private void ensureOwnConfig() {"
        - id: ConfigManager.applyConfig
          signature: "private void applyConfig(SessionConfig target) {"
        - id: ConfigManager.setKey
          signature: "public void setKey(String keys) {"
        - id: ConfigManager.setMaxOutputTokens
          signature: "public void setMaxOutputTokens(Long maxOutputTokens) {"
        - id: ConfigManager.setTemperature
          signature: "public void setTemperature(Double temp) {"
        - id: ConfigManager.setTopP
          signature: "public void setTopP(Double topP) {"
        - id: ConfigManager.setModel
          signature: "public void setModel(Model model) {"
        - id: ConfigManager.setReasoningBudget
          signature: "public void setReasoningBudget(Integer reasoningBudget) {"
        - id: ConfigManager.setReasoning
          signature: "public void setReasoning(Reasoning reasoning) {"
        - id: ConfigManager.setCacheMode
          signature: "public void setCacheMode(CacheMode cacheMode) {"
        - id: ConfigManager.setProfile
          signature: "public void setProfile(AgentProfile profile) {"
        - id: ConfigManager.setSystemPrompt
          signature: "public void setSystemPrompt(String[] systemPrompt) {"
        - id: ConfigManager.setSystemFree
          signature: "public void setSystemFree(String freeText) {"
        - id: ConfigManager.setEnabledTools
          signature: "public void setEnabledTools(String[] enabledTools) {"
        - id: ConfigManager.setOuputMode
          signature: "public void setOuputMode(OutputMode ouputMode) {"
        - id: ConfigManager.setInputMode
          signature: "public void setInputMode(InputMode mode, boolean enable) {"
        - id: ConfigManager.getKeys
          signature: "public String getKeys() {"
        - id: ConfigManager.getMaxOutputTokens
          signature: "public Long getMaxOutputTokens() {"
        - id: ConfigManager.getTemperature
          signature: "public Double getTemperature() {"
        - id: ConfigManager.getTopP
          signature: "public Double getTopP() {"
        - id: ConfigManager.getModel
          signature: "public Model getModel() {"
        - id: ConfigManager.getReasoningBudget
          signature: "public Integer getReasoningBudget() {"
        - id: ConfigManager.getProfile
          signature: "public AgentProfile getProfile() {"
        - id: ConfigManager.getReasoning
          signature: "public Reasoning getReasoning() {"
        - id: ConfigManager.getCacheMode
          signature: "public CacheMode getCacheMode() {"
        - id: ConfigManager.getSystemPrompt
          signature: "public String[] getSystemPrompt() {"
        - id: ConfigManager.getFreeText
          signature: "public String getFreeText() {"
        - id: ConfigManager.getOuputMode
          signature: "public OutputMode getOuputMode() {"
        - id: ConfigManager.isInputEnabled
          signature: "public boolean isInputEnabled(InputMode mode) {"
        - id: ConfigManager.addSystemPromptObs
          signature: "public void addSystemPromptObs(Consumer<SessionConfig> obs, boolean initialize)…"
        - id: ConfigManager.addSystemFreeObs
          signature: "public void addSystemFreeObs(Consumer<String> obs, boolean initialize) {"
        - id: ConfigManager.addProfileObs
          signature: "public void addProfileObs(Consumer<AgentProfile> obs, boolean initialize) {"
        - id: ConfigManager.addTemperatureObs
          signature: "public void addTemperatureObs(Consumer<Double> obs, boolean initialize) {"
        - id: ConfigManager.addTopPObs
          signature: "public void addTopPObs(Consumer<Double> obs, boolean initialize) {"
        - id: ConfigManager.addInputObs
          signature: "public void addInputObs(Consumer<boolean[]> obs, boolean initialize) {"
        - id: ConfigManager.addBudgetObs
          signature: "public void addBudgetObs(Consumer<Integer> obs, boolean initialize) {"
        - id: ConfigManager.addReasoningObs
          signature: "public void addReasoningObs(Consumer<Reasoning> obs, boolean initialize) {"
        - id: ConfigManager.addCacheObs
          signature: "public void addCacheObs(Consumer<CacheMode> obs, boolean initialize) {"
        - id: ConfigManager.addKeyObs
          signature: "public void addKeyObs(Consumer<String> obs, boolean initialize) {"
        - id: ConfigManager.addModelObs
          signature: "public void addModelObs(Consumer<Model> obs, boolean initialize) {"
        - id: ConfigManager.addOutputTokenObs
          signature: "public void addOutputTokenObs(Consumer<Long> obs, boolean initialize) {"
        - id: ConfigManager.addEnabledModelsObs
          signature: "public void addEnabledModelsObs(Consumer<Model[]> obs, boolean initialize) {"
        - id: ConfigManager.addEnabledProfilesObs
          signature: "public void addEnabledProfilesObs(Consumer<AgentProfile[]> obs, boolean initial…"
        - id: ConfigManager.addInputModeObs
          signature: "public void addInputModeObs(Consumer<InputMode> obs) {"
        - id: ConfigManager.addOutputModeObs
          signature: "public void addOutputModeObs(Consumer<OutputMode> obs, boolean initialize) {"
        - id: ConfigManager.addEnabledToolsObs
          signature: "public void addEnabledToolsObs(Consumer<String[]> obs, boolean initialize) {"
        - id: ConfigManager.updateEnabledModels
          signature: "private void updateEnabledModels(String[] keys) {"
        - id: ConfigManager.setEnabledProfiles
          signature: "public void setEnabledProfiles(AgentProfile[] enabledProfiles) {"
        - id: ConfigManager.getEnabledModels
          signature: "public Model[] getEnabledModels() {"
        - id: ConfigManager.getReasonings
          signature: "public String[] getReasonings() {"
        - id: ConfigManager.saveConfig
          signature: "public void saveConfig(IMemento memento) {"
        - id: ConfigManager.loadConfig
          signature: "public void loadConfig(IMemento memento) {"
        - id: ConfigManager.getCapabilities
          signature: "public Capabilities getCapabilities() {"
        - id: ConfigManager.getTools
          signature: "public String[] getTools() {"
```
/answer 66b9-10 allow
ReasoningToken: 11
Control Request:
```yaml
id: 66b9-11
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
    ids:
    - MementoConverter.saveConfig
    - MementoConverter.loadConfig
    - MementoConverter.Snapshot
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
    ids:
    - ConfigManager.saveConfig
    - ConfigManager.loadConfig
    - ConfigManager.setEnabledTools
    - ConfigManager.getTools
    - ConfigManager.applyConfig
  reason: "Details der save/load Methoden einsehen, um enabledTools zu ergänzen"
```
/answer 66b9-11 allow
Control Request:
```yaml
id: 66b9-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
      nodes:
      - id: MementoConverter.saveConfig
        type: method_declaration
        code: "public static void saveConfig(IMemento memento, SessionConfig cfg) {\n\t\tSnapshot snap = new Snapshot();\n\t\tsnap.keys = cfg.keys;\n\t\tsnap.maxOutputTokens = cfg.maxOutputTokens;\n\t\tsnap.temperature = cfg.temperature;\n\t\tsnap.topP = cfg.topP;\n\t\tsnap.freeText = cfg.freeText;\n\t\tif (cfg.model != null) {\n\t\t\tsnap.modelProvider = cfg.model.cap.getKeyPattern().name();\n\t\t\tsnap.modelDisplayName = cfg.model.displayName;\n\t\t}\n\t\tsnap.reasoning = cfg.reasoning != null ? cfg.reasoning.name() : null;\n\t\tsnap.cacheMode = cfg.cacheMode != null ? cfg.cacheMode.name() : null;\n\t\tsnap.profile = cfg.profile != null ? cfg.profile.name() : null;\n\t\tsnap.reasoningBudget = cfg.reasoningBudget;\n\t\tsnap.systemPrompt = cfg.systemPrompt;\n\t\tsnap.outputMode = cfg.ouputMode != null ? cfg.ouputMode.name() : null;\n\t\tsnap.inputModes = cfg.inputModes;\n\n\t\ttry {\n\t\t\tmemento.createChild(\"cfg\").putString(\"json\", MAPPER.writeValueAsString(snap));\n\t\t} catch (JsonProcessingException e) {\n\t\t\tLOG.error(\"Unable to serialize session config\", e);\n\t\t}\n\t}"
      - id: MementoConverter.loadConfig
        type: method_declaration
        code: "public static Snapshot loadConfig(IMemento memento) {\n\t\tif (memento == null)\n\t\t\treturn null;\n\t\tIMemento m = memento.getChild(\"cfg\");\n\t\tString json = m != null ? m.getString(\"json\") : null;\n\t\tif (json == null)\n\t\t\treturn null;\n\t\ttry {\n\t\t\treturn MAPPER.readValue(json, Snapshot.class);\n\t\t} catch (IOException e) {\n\t\t\tLOG.error(\"Unable to deserialize session config\", e);\n\t\t\treturn null;\n\t\t}\n\t}"
      - id: MementoConverter.Snapshot
        type: class_declaration
        code: "public static class Snapshot {\n\t\tpublic String keys;\n\t\tpublic Long maxOutputTokens;\n\t\tpublic Double temperature;\n\t\tpublic Double topP;\n\t\tpublic String freeText;\n\t\tpublic String modelProvider;\n\t\tpublic String modelDisplayName;\n\t\tpublic String reasoning;\n\t\tpublic String cacheMode;\n\t\tpublic String profile;\n\t\tpublic Integer reasoningBudget;\n\t\tpublic String[] systemPrompt;\n\t\tpublic String outputMode;\n\t\tpublic boolean[] inputModes;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
      nodes:
      - id: ConfigManager.saveConfig
        type: method_declaration
        code: "public void saveConfig(IMemento memento) {\n\t\tMementoConverter.saveConfig(memento, cfg);\n\t}"
      - id: ConfigManager.loadConfig
        type: method_declaration
        code: "public void loadConfig(IMemento memento) {\n\t\tString savedKey = activeEditorKey;\n\t\tactiveEditorKey = null;\n\t\tcfg = defaultCfg;\n\t\ttry {\n\t\t\tMementoConverter.Snapshot snap = MementoConverter.loadConfig(memento);\n\t\t\tif (snap == null)\n\t\t\t\treturn;\n\n\t\t\t/*\n\t\t\t * Order matters: keys must be set (models resolved) before the model itself can\n\t\t\t * be restored, and the model must be set before the remaining,\n\t\t\t * capability-dependent settings are applied - otherwise setModel()'s own\n\t\t\t * defaulting logic would clobber them.\n\t\t\t */\n\t\t\tif (snap.keys != null)\n\t\t\t\tsetKey(snap.keys);\n\n\t\t\tif (snap.modelProvider != null) {\n\t\t\t\tModel resolved = Arrays.stream(enabledModels)\n\t\t\t\t\t\t.filter(m -> m.matches(KeyPattern.valueOf(snap.modelProvider), snap.modelDisplayName)).findFirst()\n\t\t\t\t\t\t.orElse(null);\n\t\t\t\tif (resolved != null)\n\t\t\t\t\tsetModel(resolved);\n\t\t\t\telse\n\t\t\t\t\tLOG.error(\"Unable to resolve persisted model \" + snap.modelProvider + \":\" + snap.modelDisplayName\n\t\t\t\t\t\t\t+ \", no fallback applied\");\n\t\t\t}\n\n\t\t\tif (snap.temperature != null)\n\t\t\t\tsetTemperature(snap.temperature);\n\t\t\tif (snap.topP != null)\n\t\t\t\tsetTopP(snap.topP);\n\t\t\tif (snap.maxOutputTokens != null)\n\t\t\t\tsetMaxOutputTokens(snap.maxOutputTokens);\n\t\t\tif (snap.reasoningBudget != null)\n\t\t\t\tsetReasoningBudget(snap.reasoningBudget);\n\t\t\tif (snap.reasoning != null)\n\t\t\t\tsetReasoning(Reasoning.valueOf(snap.reasoning));\n\t\t\tif (snap.cacheMode != null)\n\t\t\t\tsetCacheMode(CacheMode.valueOf(snap.cacheMode));\n\t\t\tif (snap.profile != null)\n\t\t\t\tsetProfile(AgentProfile.valueOf(snap.profile));\n\t\t\tif (snap.systemPrompt != null)\n\t\t\t\tsetSystemPrompt(snap.systemPrompt);\n\t\t\tif (snap.freeText != null)\n\t\t\t\tsetSystemFree(snap.freeText);\n\t\t\tif (snap.outputMode != null)\n\t\t\t\tsetOuputMode(OutputMode.valueOf(snap.outputMode));\n\t\t\tif (snap.inputModes != null)\n\t\t\t\tfor (InputMode mode : InputMode.values())\n\t\t\t\t\tif (mode.ordinal() < snap.inputModes.length)\n\t\t\t\t\t\tsetInputMode(mode, snap.inputModes[mode.ordinal()]);\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t\tLOG.info(\"Unable to restore config\", e);\n\t\t} finally {\n\t\t\tactiveEditorKey = savedKey;\n\t\t\tcfg = savedKey == null ? defaultCfg : editorConfigs.getOrDefault(savedKey, defaultCfg);\n\t\t}\n\t}"
      - id: ConfigManager.setEnabledTools
        type: method_declaration
        code: "public void setEnabledTools(String[] enabledTools) {\n\t\tif (Arrays.equals(this.enabledTools, enabledTools))\n\t\t\treturn;\n\t\tthis.enabledTools = enabledTools;\n\t\tenabledToolsObs.forEach(c -> c.accept(enabledTools));\n\t}"
      - id: ConfigManager.getTools
        type: method_declaration
        code: "public String[] getTools() {\n\t\treturn enabledTools;\n\t}"
      - id: ConfigManager.applyConfig
        type: method_declaration
        code: "private void applyConfig(SessionConfig target) {\n\t\tif (target == cfg)\n\t\t\treturn;\n\t\tcfg = target;\n\t\tsystemPromptObs.forEach(c -> c.accept(cfg));\n\t\tsystemFreeObs.forEach(c -> c.accept(cfg.freeText));\n\t\tinputObs.forEach(c -> c.accept(cfg.inputModes));\n\t\tif (cfg.keys != null)\n\t\t\tfor (String key : cfg.keys.split(\",\"))\n\t\t\t\tkeyObs.forEach(c -> c.accept(key));\n\t\tmodelObs.forEach(c -> c.accept(cfg.model));\n\t\tprofileObs.forEach(c -> c.accept(cfg.profile));\n\t\toutTokenObs.forEach(c -> c.accept(cfg.maxOutputTokens));\n\t\tbudgetObs.forEach(c -> c.accept(cfg.reasoningBudget));\n\t\treasonObs.forEach(c -> c.accept(cfg.reasoning));\n\t\tcacheObs.forEach(c -> c.accept(cfg.cacheMode));\n\t\ttemperatureObs.forEach(c -> c.accept(cfg.temperature));\n\t\ttopPObs.forEach(c -> c.accept(cfg.topP));\n\t\toutputModeObs.forEach(c -> c.accept(cfg.ouputMode));\n\t\tupdateEnabledModels(cfg.keys != null ? cfg.keys.split(\",\") : new String[0]);\n\t}"
```
/answer 66b9-12 allow
Control Request:
```yaml
id: 66b9-13
toolName: grep
arguments:
  items:
  - pattern: enabledTools
    directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    include:
    - '*.java'
  reason: "Alle Vorkommen von enabledTools finden, um Feldzugehörigkeit zu klären"
```
/answer 66b9-13 allow
Control Request:
```yaml
id: 66b9-14
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: ConfigManager.java
        matches:
        - lineno: 30
          match: "private String[] enabledTools = new String[0];"
        - lineno: 47
          match: "private List<Consumer<String[]>> enabledToolsObs = new ArrayList<>();"
        - lineno: 71
          match: enabledToolsObs.clear();
        - lineno: 233
          match: "public void setEnabledTools(String[] enabledTools) {"
        - lineno: 234
          match: "if (Arrays.equals(this.enabledTools, enabledTools))"
        - lineno: 236
          match: this.enabledTools = enabledTools;
        - lineno: 237
          match: enabledToolsObs.forEach(c -> c.accept(enabledTools));
        - lineno: 415
          match: enabledToolsObs.add(obs);
        - lineno: 417
          match: obs.accept(enabledTools);
        - lineno: 531
          match: return enabledTools;
```
/answer 66b9-14 allow
ReasoningToken: 92
Control Request:
```yaml
id: 66b9-15
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
    id: MementoConverter.saveConfig
    old_text: "public static void saveConfig(IMemento memento, SessionConfig cfg) {"
    new_text: "public static void saveConfig(IMemento memento, SessionConfig cfg, String[] enabledTools) {"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
    id: MementoConverter.saveConfig
    old_text: "snap.inputModes = cfg.inputModes;\n\n\t\ttry {"
    new_text: "snap.inputModes = cfg.inputModes;\n\t\tsnap.enabledTools = enabledTools;\n\n\t\ttry {"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
    id: MementoConverter.Snapshot
    old_text: "public boolean[] inputModes;\n\t}"
    new_text: "public boolean[] inputModes;\n\t\tpublic String[] enabledTools;\n\t}"
  reason: enabledTools zum Snapshot hinzufügen und saveConfig-Signatur erweitern
```
/answer 66b9-15 allow
Control Request:
```yaml
id: 66b9-16
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
    id: ConfigManager.saveConfig
    old_text: "MementoConverter.saveConfig(memento, cfg);"
    new_text: "MementoConverter.saveConfig(memento, cfg, enabledTools);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
    id: ConfigManager.loadConfig
    old_text: "if (snap.inputModes != null)\n\t\t\t\tfor (InputMode mode : InputMode.values())\n\t\t\t\t\tif (mode.ordinal() < snap.inputModes.length)\n\t\t\t\t\t\tsetInputMode(mode, snap.inputModes[mode.ordinal()]);"
    new_text: "if (snap.inputModes != null)\n\t\t\t\tfor (InputMode mode : InputMode.values())\n\t\t\t\t\tif (mode.ordinal() < snap.inputModes.length)\n\t\t\t\t\t\tsetInputMode(mode, snap.inputModes[mode.ordinal()]);\n\t\t\tif (snap.enabledTools != null)\n\t\t\t\tsetEnabledTools(snap.enabledTools);"
  reason: enabledTools beim Speichern und Laden berücksichtigen
```
/answer 66b9-16 allow
ReasoningToken: 276
Text:
 Jetzt ConfigManager anpassen:
Control Request:
```yaml
id: 66b9-17
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
  reason: Änderungen kompilieren lassen
```
/answer 66b9-17 allow
Result Stats: id=49cc0513-dc88-49c2-b060-5a5edd707b3b, total: 18626, in: 145, out: 2485, reason: 0, read: 72197, write: 15996
Erledigt: `enabledTools` wird nun im `Snapshot` gespeichert (`MementoConverter.saveConfig`/`loadConfig`) und in `ConfigManager.saveConfig`/`loadConfig` entsprechend persistiert bzw. via `setEnabledTools` wiederhergestellt.
