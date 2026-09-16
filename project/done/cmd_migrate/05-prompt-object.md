# Step 05 – FrozenConfig + Prompt-Objekt (Phase 4)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
**Ziel:** Eingaben und Konfiguration in unveränderliche Objekte einfrieren. `FrozenConfig` kapselt die
Konfiguration; `Prompt` bündelt Trigger + Eingaben (semantisch getrennt: Inputs, Batch, Config) und
steuert den Aktivierungszustand des SessionProcessor. Session-ID = Hash(Dateipfad, Batch, ConfigHash).
**Abhängigkeiten:** Step 02 (Hash-Mechanik als Vorlage).

## Verzeichnis
Neu in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/` (package `...connector.harness`).

## Config-Quellen (`ConfigManager.java`, gemessen)
Getter: `getModel()`170, `getProfile()`174, `getReasoning()`178, `getTools()`415,
`getSystemPrompt()`186 (`String[]`), `getFreeText()`190, `getTopP()`166 (`Double`),
`getTemperature()`162 (`Double`), `getMaxOutputTokens()` (existiert), `getCacheMode()`182,
`getKeys()`154 (cliProfile), `isInputEnabled(InputMode)`297.

Systemprompt-Aufbau (Ist, aus `AISessionManager.getInput(SystemPrompt)`):
```java
Arrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith("#"))
  .forEach(e -> sb.append("* ").append(e).append(".\n"));
String freeText = cfg.getFreeText();
if (freeText != null && !freeText.isBlank()) sb.append(".\n").append(freeText).append(".\n");
```

## Aktionen
1. `connector/harness/FrozenConfig.java` – unveränderliche Momentaufnahme:
   - Felder: `model`, `profile`, `reasoning`, `tools` (List/Array, kopiert), `systemPrompt` (fertig
     gerenderter String), `topP`, `temperature`, `maxOutputTokens`, ergänzend `cacheMode`, `cliProfile`.
   - Factory `static FrozenConfig from(ConfigManager cfg)`: liest alle Werte einmalig aus, rendert den
     Systemprompt (Logik oben), friert Tools ein.
   - `String getHash()` – deterministischer Hash NUR über: **Modell, Profil, Reasoning, Tools,
     Systemprompt, TopP, Temperature, MaxTokens** (exakt diese Bestandteile). Hash-Mechanik analog
     `SessionParameters.computeHash` (MD5→hex, stabiler Fallback).
2. `connector/harness/Prompt.java` – eingefrorenes Prompt-Objekt (immutable):
   - **Semantisch getrennte** Bestandteile:
     - Inputs: `List<String> inputs` (ausgewählter Textbereich / Selection / Converter-Inhalt),
       plus Meta `InputKind kind { LineInput, BlockSelection, FullFile }` (für Step 06).
     - Batch: `boolean batch` (Batch-Trigger ja/nein).
     - Config: `FrozenConfig config`.
   - `boolean processorEnabled` – ersetzt das direkte `sessionProcessor.setEnabled(...)`.
   - `String absoluteFilePath` – absoluter Pfad der auslösenden Datei.
   - `String sessionId()` – Hash über: `absoluteFilePath` + `"|"` + `batch` + `"|"` + `config.getHash()`
     (Mechanik analog `ClaudeSessionParameters.computeHash`, siehe Step 02). Referenz-Semantik:
     `ClaudeSessionParameters` nutzt den Datei-/Parameter-Hash analog.
   - Optionale, typisierte Kommando-Payload (für Step 06/08): `Command command` + ggf. erfasster
     `String yamlBlock` (für `/call`).
   - Konstruktion nur über Builder/Factory im PromptHandler (Step 06) – hier nur die Datenklasse.
3. Optionale Ableitung: statische Hilfsmethode, um aus `Prompt.config` eine `ClaudeSessionParameters`
   bzw. generische `SessionParameters` zu erzeugen (ersetzt in Step 07 `fromConfig(cfg, ...)`).

## Hinweise
- **Kein** direktes `cfg` in Connectoren mehr (Step 07) – Werte kommen aus `Prompt.config`.
- Der Systemprompt wird beim Einfrieren einmalig gerendert (Determinismus, Multi-Session-fähig).
- Hash-Bestandteile für `FrozenConfig` strikt auf die 8 genannten begrenzen.

## Validierung
`ast_validate` für `connector/harness/FrozenConfig.java`, `connector/harness/Prompt.java`.

SystemInit:  id=959d78c1-3602-4b48-9232-6b7c79fa496c, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7fa5-81
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: class SessionParameters|computeHash
    include:
    - '*.java'
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: class ClaudeSessionParameters
    include:
    - '*.java'
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: class ConfigManager
    include:
    - '*.java'
    limit: 10
  reason: Codebasis und relevante Klassen für Step 05 lokalisieren
```
/answer 7fa5-81 allow
Control Request:
```yaml
id: 7fa5-82
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      pattern: class SessionParameters|computeHash
      matches:
      - path: xy/ai/workbench/connector/SessionParameters.java
        lineno: 16
        match: "public class SessionParameters {"
      - path: xy/ai/workbench/connector/SessionParameters.java
        lineno: 74
        match: "\t\t\thash = computeHash();"
      - path: xy/ai/workbench/connector/SessionParameters.java
        lineno: 78
        match: "\tprivate String computeHash() {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      pattern: class ClaudeSessionParameters
      matches:
      - path: xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
        lineno: 14
        match: "public class ClaudeSessionParameters extends SessionParameters {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      pattern: class ConfigManager
      matches:
      - path: xy/ai/workbench/ConfigManager.java
        lineno: 16
        match: "public class ConfigManager {"
```
/answer 7fa5-82 allow
ReasoningToken: 62
Control Request:
```yaml
id: 7fa5-83
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/SessionParameters.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
  reason: Struktur der relevanten Klassen für Step 05 prüfen
```
/answer 7fa5-83 allow
Control Request:
```yaml
id: 7fa5-84
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/SessionParameters.java
      nodes:
      - id: tadoql
        type: imports
      - id: 8qXu0W
        type: statements
      - id: SessionParameters
        type: class_declaration
        signature: "public class SessionParameters {"
        children:
        - id: SessionParameters.J6CNxL
          type: statements
        - id: SessionParameters.zREYpo
          type: statements
        - id: SessionParameters.SessionParameters
          type: constructor_declaration
          signature: "public SessionParameters(Path cwd, String systemPrompt, List<String> tools, Mod…"
        - id: SessionParameters.SessionParameters_1
          type: constructor_declaration
          signature: "public SessionParameters(Path cwd, String systemPrompt, List<String> tools, Mod…"
        - id: SessionParameters.getFilePath
          type: method_declaration
          signature: "public String getFilePath() {"
        - id: SessionParameters.getHash
          type: method_declaration
          signature: "public String getHash() {"
        - id: SessionParameters.computeHash
          type: method_declaration
          signature: "private String computeHash() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
      nodes:
      - id: ELFb2q
        type: imports
      - id: ClaudeSessionParameters
        type: class_declaration
        signature: "public class ClaudeSessionParameters extends SessionParameters {"
        children:
        - id: ClaudeSessionParameters.03OBcZ
          type: statements
        - id: ClaudeSessionParameters.ClaudeSessionParameters
          type: constructor_declaration
          signature: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tool…"
        - id: ClaudeSessionParameters.ClaudeSessionParameters_1
          type: constructor_declaration
          signature: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tool…"
        - id: ClaudeSessionParameters.fromConfig
          type: method_declaration
          signature: "public static ClaudeSessionParameters fromConfig(ConfigManager cfg, Path cwd, S…"
        - id: ClaudeSessionParameters.buildBaseCommand
          type: method_declaration
          signature: "public List<String> buildBaseCommand() {"
        - id: ClaudeSessionParameters.buildEvironment
          type: method_declaration
          signature: "public void buildEvironment(ProcessBuilder pb) {"
        - id: ClaudeSessionParameters.setTitle
          type: method_declaration
          signature: "public void setTitle(String title) {"
        - id: ClaudeSessionParameters.getTitle
          type: method_declaration
          signature: "public String getTitle() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
      nodes:
      - id: j4fa8w
        type: imports
      - id: ConfigManager
        type: class_declaration
        signature: "public class ConfigManager {"
        children:
        - id: ConfigManager.kX6U2g
          type: statements
        - id: ConfigManager.qZ3cyQ
          type: statements
        - id: ConfigManager.QApGiA
          type: statements
        - id: ConfigManager.clearObserver
          type: method_declaration
          signature: "public void clearObserver() {"
        - id: ConfigManager.setKey
          type: method_declaration
          signature: "public void setKey(String keys) {"
        - id: ConfigManager.setMaxOutputTokens
          type: method_declaration
          signature: "public void setMaxOutputTokens(Long maxOutputTokens) {"
        - id: ConfigManager.setTemperature
          type: method_declaration
          signature: "public void setTemperature(Double temperature) {"
        - id: ConfigManager.setTopP
          type: method_declaration
          signature: "public void setTopP(Double topP) {"
        - id: ConfigManager.setModel
          type: method_declaration
          signature: "public void setModel(Model model) {"
        - id: ConfigManager.getReasoningBudget
          type: method_declaration
          signature: "public Integer getReasoningBudget() {"
        - id: ConfigManager.setReasoningBudget
          type: method_declaration
          signature: "public void setReasoningBudget(Integer reasoningBudget) {"
        - id: ConfigManager.setReasoning
          type: method_declaration
          signature: "public void setReasoning(Reasoning reasoning) {"
        - id: ConfigManager.setCacheMode
          type: method_declaration
          signature: "public void setCacheMode(CacheMode cacheMode) {"
        - id: ConfigManager.setProfile
          type: method_declaration
          signature: "public void setProfile(AgentProfile profile) {"
        - id: ConfigManager.setSystemPrompt
          type: method_declaration
          signature: "public void setSystemPrompt(String[] systemPrompt) {"
        - id: ConfigManager.setSystemFree
          type: method_declaration
          signature: "public void setSystemFree(String systemPrompt) {"
        - id: ConfigManager.setEnabledTools
          type: method_declaration
          signature: "public void setEnabledTools(String[] enabledTools) {"
        - id: ConfigManager.addEnabledToolsObs
          type: method_declaration
          signature: "public void addEnabledToolsObs(Consumer<String[]> obs, boolean initialize) {"
        - id: ConfigManager.getKeys
          type: method_declaration
          signature: "public String getKeys() {"
        - id: ConfigManager.getMaxOutputTokens
          type: method_declaration
          signature: "public Long getMaxOutputTokens() {"
        - id: ConfigManager.getTemperature
          type: method_declaration
          signature: "public Double getTemperature() {"
        - id: ConfigManager.getTopP
          type: method_declaration
          signature: "public Double getTopP() {"
        - id: ConfigManager.getModel
          type: method_declaration
          signature: "public Model getModel() {"
        - id: ConfigManager.getProfile
          type: method_declaration
          signature: "public AgentProfile getProfile() {"
        - id: ConfigManager.getReasoning
          type: method_declaration
          signature: "public Reasoning getReasoning() {"
        - id: ConfigManager.getCacheMode
          type: method_declaration
          signature: "public CacheMode getCacheMode() {"
        - id: ConfigManager.getSystemPrompt
          type: method_declaration
          signature: "public String[] getSystemPrompt() {"
        - id: ConfigManager.getFreeText
          type: method_declaration
          signature: "public String getFreeText() {"
        - id: ConfigManager.addSystemPromptObs
          type: method_declaration
          signature: "public void addSystemPromptObs(Consumer<SessionConfig> obs, boolean initialize)…"
        - id: ConfigManager.addSystemFreeObs
          type: method_declaration
          signature: "public void addSystemFreeObs(Consumer<String> obs, boolean initialize) {"
        - id: ConfigManager.addProfileObs
          type: method_declaration
          signature: "public void addProfileObs(Consumer<AgentProfile> obs, boolean initialize) {"
        - id: ConfigManager.addTemperatureObs
          type: method_declaration
          signature: "public void addTemperatureObs(Consumer<Double> obs, boolean initialize) {"
        - id: ConfigManager.addTopPObs
          type: method_declaration
          signature: "public void addTopPObs(Consumer<Double> obs, boolean initialize) {"
        - id: ConfigManager.addInputObs
          type: method_declaration
          signature: "public void addInputObs(Consumer<boolean[]> obs, boolean initialize) {"
        - id: ConfigManager.addBudgetObs
          type: method_declaration
          signature: "public void addBudgetObs(Consumer<Integer> obs, boolean initialize) {"
        - id: ConfigManager.addReasoningObs
          type: method_declaration
          signature: "public void addReasoningObs(Consumer<Reasoning> obs, boolean initialize) {"
        - id: ConfigManager.addCacheObs
          type: method_declaration
          signature: "public void addCacheObs(Consumer<CacheMode> obs, boolean initialize) {"
        - id: ConfigManager.addKeyObs
          type: method_declaration
          signature: "public void addKeyObs(Consumer<String> obs, boolean initialize) {"
        - id: ConfigManager.addModelObs
          type: method_declaration
          signature: "public void addModelObs(Consumer<Model> obs, boolean initialize) {"
        - id: ConfigManager.addOutputTokenObs
          type: method_declaration
          signature: "public void addOutputTokenObs(Consumer<Long> obs, boolean initialize) {"
        - id: ConfigManager.addEnabledModelsObs
          type: method_declaration
          signature: "public void addEnabledModelsObs(Consumer<Model[]> obs, boolean initialize) {"
        - id: ConfigManager.addEnabledProfilesObs
          type: method_declaration
          signature: "public void addEnabledProfilesObs(Consumer<AgentProfile[]> obs, boolean initial…"
        - id: ConfigManager.addInputModeObs
          type: method_declaration
          signature: "public void addInputModeObs(Consumer<InputMode> obs) {"
        - id: ConfigManager.getOuputMode
          type: method_declaration
          signature: "public OutputMode getOuputMode() {"
        - id: ConfigManager.setOuputMode
          type: method_declaration
          signature: "public void setOuputMode(OutputMode ouputMode) {"
        - id: ConfigManager.addOutputModeObs
          type: method_declaration
          signature: "public void addOutputModeObs(Consumer<OutputMode> obs, boolean initialize) {"
        - id: ConfigManager.isInputEnabled
          type: method_declaration
          signature: "public boolean isInputEnabled(InputMode mode) {"
        - id: ConfigManager.setInputMode
          type: method_declaration
          signature: "public void setInputMode(InputMode mode, boolean enable) {"
        - id: ConfigManager.updateEnabledModels
          type: method_declaration
          signature: "private void updateEnabledModels(String[] keys) {"
        - id: ConfigManager.setEnabledProfiles
          type: method_declaration
          signature: "public void setEnabledProfiles(AgentProfile[] enabledProfiles) {"
        - id: ConfigManager.getEnabledModels
          type: method_declaration
          signature: "public Model[] getEnabledModels() {"
        - id: ConfigManager.getReasonings
          type: method_declaration
          signature: "public String[] getReasonings() {"
        - id: ConfigManager.saveConfig
          type: method_declaration
          signature: "public void saveConfig(IMemento memento) {"
        - id: ConfigManager.loadConfig
          type: method_declaration
          signature: "public void loadConfig(IMemento memento) {"
        - id: ConfigManager.getCapabilities
          type: method_declaration
          signature: "public Capabilities getCapabilities() {"
        - id: ConfigManager.getTools
          type: method_declaration
          signature: "public String[] getTools() {"
```
/answer 7fa5-84 allow
Control Request:
```yaml
id: 7fa5-85
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/SessionParameters.java
    ids:
    - tadoql
    - SessionParameters
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    ids:
    - ELFb2q
    - ClaudeSessionParameters
  reason: Vollständigen Code der Hash-Mechanik und fromConfig für Step 05 als Vorlage lesen
```
/answer 7fa5-85 allow
Control Request:
```yaml
id: 7fa5-86
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/SessionParameters.java
      nodes:
      - id: tadoql
        type: imports
        code: |-
          package xy.ai.workbench.connector;

          import java.nio.charset.StandardCharsets;
          import java.nio.file.Path;
          import java.security.MessageDigest;
          import java.security.NoSuchAlgorithmException;
          import java.util.Collections;
          import java.util.List;

          import xy.ai.workbench.AgentProfile;
          import xy.ai.workbench.CacheMode;
          import xy.ai.workbench.Model;
          import xy.ai.workbench.Reasoning;
      - id: SessionParameters
        type: class_declaration
        signature: "public class SessionParameters {"
        children:
        - id: SessionParameters.J6CNxL
          type: statements
          code: "/** Deterministic hash of the session parameters. Immutable. */\n\tpublic final Path cwd;\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final Model model;\n\tpublic final Reasoning reasoning;\n\tpublic final AgentProfile agentProfile;\n\tpublic final String cliProfile;\n\tpublic final String filePath;\n\tpublic final CacheMode cacheMode;\n\n\t// Reserved for later use by FrozenConfig (not part of the hash).\n\tpublic final Double topP;\n\tpublic final Double temperature;\n\tpublic final Integer maxOutputTokens;"
        - id: SessionParameters.zREYpo
          type: statements
          code: private String hash;
        - id: SessionParameters.SessionParameters
          type: constructor_declaration
          code: "public SessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath) {\n\t\tthis(cwd, systemPrompt, tools, model, reasoning, agentProfile, cliProfile, cacheMode, filePath, null, null,\n\t\t\t\tnull);\n\t}"
        - id: SessionParameters.SessionParameters_1
          type: constructor_declaration
          code: "public SessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath, Double topP,\n\t\t\tDouble temperature, Integer maxOutputTokens) {\n\t\tif (cwd == null)\n\t\t\tthrow new IllegalStateException(\"Work directory (cwd) not set\");\n\t\tif (model == null)\n\t\t\tthrow new IllegalArgumentException(\"Model must not be null\");\n\t\tif (model.apiName == null || model.apiName.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"Model apiName must not be null or blank\");\n\t\tif (reasoning == null)\n\t\t\tthrow new IllegalArgumentException(\"Reasoning must not be null\");\n\n\t\tthis.cwd = cwd;\n\t\tthis.systemPrompt = systemPrompt != null ? systemPrompt : \"\";\n\t\tthis.tools = tools != null ? tools : Collections.emptyList();\n\t\tthis.model = model;\n\t\tthis.reasoning = reasoning;\n\t\tthis.agentProfile = agentProfile;\n\t\tthis.cliProfile = cliProfile;\n\t\tthis.filePath = filePath;\n\t\tthis.cacheMode = cacheMode;\n\t\tthis.topP = topP;\n\t\tthis.temperature = temperature;\n\t\tthis.maxOutputTokens = maxOutputTokens;\n\t}"
        - id: SessionParameters.getFilePath
          type: method_declaration
          code: "public String getFilePath() {\n\t\treturn filePath;\n\t}"
        - id: SessionParameters.getHash
          type: method_declaration
          code: "public String getHash() {\n\t\tif (hash == null)\n\t\t\thash = computeHash();\n\t\treturn hash;\n\t}"
        - id: SessionParameters.computeHash
          type: method_declaration
          code: "private String computeHash() {\n\t\tString input = String.join(\",\", tools) + \"|\" + cwd.toString() + \"|\"\n\t\t\t\t+ model.apiName + \"|\" + reasoning.name() + \"|\" + (agentProfile != null ? agentProfile.name : \"\") + \"|\"\n\t\t\t\t+ cliProfile + \"|\" + (filePath != null ? filePath : \"\");\n\t\ttry {\n\t\t\tMessageDigest md = MessageDigest.getInstance(\"MD5\");\n\t\t\tbyte[] bytes = md.digest(input.getBytes(StandardCharsets.UTF_8));\n\t\t\tStringBuilder sb = new StringBuilder();\n\t\t\tfor (byte b : bytes)\n\t\t\t\tsb.append(String.format(\"%02x\", b));\n\t\t\treturn sb.substring(0, 8);\n\t\t} catch (NoSuchAlgorithmException e) {\n\t\t\t// Stable fallback (no external dependency)\n\t\t\tlong h = 0;\n\t\t\tfor (char c : input.toCharArray())\n\t\t\t\th = h * 31L + c;\n\t\t\treturn String.format(\"%08x\", h & 0xFFFFFFFFL);\n\t\t}\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
      nodes:
      - id: ELFb2q
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import java.nio.file.Path;
          import java.util.ArrayList;
          import java.util.List;

          import xy.ai.workbench.AgentProfile;
          import xy.ai.workbench.CacheMode;
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.Model;
          import xy.ai.workbench.Reasoning;
          import xy.ai.workbench.connector.SessionParameters;
      - id: ClaudeSessionParameters
        type: class_declaration
        signature: "public class ClaudeSessionParameters extends SessionParameters {"
        children:
        - id: ClaudeSessionParameters.03OBcZ
          type: statements
          code: "private static final String SCRIPT = System.getProperty(\"user.home\")\n\t\t\t+ \"/xyan/xy.ai.workbench/claude-code/claude-session.sh\";\n\tprivate static final String COMMAND = \"claude\";\n\tpublic final CacheMode cacheMode;\n\tprivate String hash;\n\tprivate String title;\nprivate String title;"
        - id: ClaudeSessionParameters.ClaudeSessionParameters
          type: constructor_declaration
          code: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode) {\n\t\tthis(cwd, systemPrompt, tools, model, reasoning, agentProfile, cliProfile, cacheMode, null);\n\t}"
        - id: ClaudeSessionParameters.ClaudeSessionParameters_1
          type: constructor_declaration
          code: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath) {\n\t\tsuper(cwd, systemPrompt, tools, model, reasoning, agentProfile, cliProfile, cacheMode, filePath);\n\t}"
        - id: ClaudeSessionParameters.fromConfig
          type: method_declaration
          code: "public static ClaudeSessionParameters fromConfig(ConfigManager cfg, Path cwd, String filePath, String systemPrompt,\n\t\t\tList<String> tools) {\n\t\treturn new ClaudeSessionParameters(cwd, systemPrompt, tools, cfg.getModel(), cfg.getReasoning(), cfg.getProfile(),\n\t\t\t\tcfg.getKeys(), cfg.getCacheMode(), filePath);\n\t}"
        - id: ClaudeSessionParameters.buildBaseCommand
          type: method_declaration
          code: "public List<String> buildBaseCommand() {\n\t\tList<String> cmd = new ArrayList<>();\n\t\tif (AgentProfile.MCPC.equals(agentProfile)) {\n\t\t\tcmd.add(COMMAND);\n\t\t\tcmd.add(\"--system-prompt\");\n\t\t\tcmd.add(systemPrompt);\n\t\t\tcmd.add(\"--tools\");\n\t\t\tcmd.add(\"\"); // restrict builtin tools\n\t\t\t// evil: breaks STDIN handling\n//\t\t\tcmd.add(\"--debug\");\n//\t\t\tcmd.add(\"mcp\");\n\t\t\tcmd.add(\"--settings\");\n\t\t\tcmd.add(\"\"\"\n\t\t\t\t\t{\n\t\t\t\t\t\t\"hooks\": {\n\t\t\t\t\t\t\t\"PreToolUse\": [\n\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\"hooks\": [\n\t\t\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\t\t\"type\": \"http\",\n\t\t\t\t\t\t\t\t\t\t\t\"url\":\"http://localhost:9093/hooks/tool\",\n\t\t\t\t\t\t\t\t\t\t\t\"headers\":{\n\t\t\t\t\t\t\t\t\t\t\t   \"X-MCPC-SESSION-ID\":\"$MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t},\n\t\t\t\t\t\t\t\t\t\t\t\"allowedEnvVars\":[\n\t\t\t\t\t\t\t\t\t\t\t   \"MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t],\n\t\t\t\t\t\t\t\t\t\t\t\"timeout\": 86400\n\t\t\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t]\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t],\n\t\t\t\t\t\t\t\"PermissionRequest\": [\n\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\"hooks\": [\n\t\t\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\t\t\"type\": \"http\",\n\t\t\t\t\t\t\t\t\t\t\t\"url\":\"http://localhost:9093/hooks/permission\",\n\t\t\t\t\t\t\t\t\t\t\t\"headers\":{\n\t\t\t\t\t\t\t\t\t\t\t   \"X-MCPC-SESSION-ID\":\"$MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t},\n\t\t\t\t\t\t\t\t\t\t\t\"allowedEnvVars\":[\n\t\t\t\t\t\t\t\t\t\t\t   \"MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t],\n\t\t\t\t\t\t\t\t\t\t\t\"timeout\": 86400\n\t\t\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t]\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t]\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t\t\"\"\".replace(\"\\t\", \"\").replace(\"\\n\", \"\").strip());\n\t\t\tcmd.add(\"--mcp-config\");\n\t\t\tcmd.add(\"\"\"\n\t\t\t\t\t{\n\t\t\t\t\t\t\"mcpServers\": {\n\t\t\t\t\t\t\t\"mcpc\": {\n\t\t\t\t\t\t\t\t\"type\": \"ws\",\n\t\t\t\t\t\t\t\t\"url\": \"http://localhost:9094/mcp\",\n\t\t\t\t\t\t\t\t\"timeout\": 86400000,\n\t\t\t\t\t\t\t\t\"alwaysLoad\": true,\n\t\t\t\t\t\t\t\t\"headers\": {\n\t\t\t\t\t\t\t\t\t\"X-MCPC-SESSION-ID\": \"${MCPC_SESSION_ID}\",\n\t\t\t\t\t\t\t\t\t\"X-MCPC-TOOLS\": \"${MCPC_TOOLS}\",\n\t\t\t\t\t\t\t\t\t\"X-MCPC-CC-PROFILE\": \"${MCPC_CC_PROFILE}\"\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t\t\"\"\".replace(\"\\t\", \"\").replace(\"\\n\", \"\").strip());\n\t\t} else {\n\t\t\tcmd.add(SCRIPT);\n\t\t\tcmd.add(agentProfile != null ? agentProfile.name : \"\"); // Agent definition\n\t\t\tcmd.add(\"--profile\");\n\t\t\tcmd.add(cliProfile);\n\t\t}\n\n\t\tcmd.add(\"--verbose\");\n\t\t// replaced by MCPC\n\t\t// cmd.add(\"--include-hook-events\");\n\t\tcmd.add(\"--include-partial-messages\");\n\t\tcmd.add(\"--input-format\");\n\t\tcmd.add(\"stream-json\");\n\t\tcmd.add(\"--output-format\");\n\t\tcmd.add(\"stream-json\");\n\t\t// replaced by self mirror input\n\t\t// cmd.add(\"--replay-user-messages\");\n\t\tcmd.add(\"--model\");\n\t\tcmd.add(model.apiName);\n\t\tif (Reasoning.Disabled != reasoning) {\n\t\t\tcmd.add(\"--effort\");\n\t\t\tcmd.add(reasoning.name().toLowerCase());\n\t\t}\n\t\tcmd.add(\"--dangerously-skip-permissions\"); // as long there is no permission prompt handling implemented\n\t\treturn cmd;\n\t}"
        - id: ClaudeSessionParameters.buildEvironment
          type: method_declaration
          code: "public void buildEvironment(ProcessBuilder pb) {\n\t\tpb.directory(cwd.toFile());\n\t\tif (AgentProfile.MCPC.equals(agentProfile)) {\n\t\t\tif (!tools.isEmpty()) {\n\t\t\t\tif (CacheMode.Disabled.equals(cacheMode))\n\t\t\t\t\tthrow new IllegalArgumentException(\"Cache is required for tool loops\");\n\t\t\t\tpb.environment().put(\"MCPC_TOOLS\", String.join(\",\", tools));\n\t\t\t} else\n\t\t\t\tpb.environment().put(\"MCPC_TOOLS\", \"None\");\n\t\t\tpb.environment().put(\"MCPC_CC_PROFILE\", cliProfile);\n\t\t\tpb.environment().put(\"CLAUDE_CONFIG_DIR\", System.getProperty(\"user.home\") + \"/.claude-\" + cliProfile);\n\n\t\t\tpb.environment().put(\"CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_AGENT_VIEW\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_BACKGROUND_TASKS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_BUNDLED_SKILLS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_CLAUDE_MDS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_CRON\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_POLICY_SKILLS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_WORKFLOWS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_ENABLE_AWAY_SUMMARY\", \"0\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_ENABLE_BACKGROUND_PLUGIN_REFRESH\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_FORK_SUBAGENT\", \"0\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY\", \"1\"); // number of parralel read tools\n\t\t\tpb.environment().put(\"ENABLE_TOOL_SEARCH\", \"false\");\n\t\t\tpb.environment().put(\"MAX_MCP_OUTPUT_TOKENS\", \"\" + (25000 * 10));\n\t\t}\n\n\t\tif (cacheMode != null)\n\t\t\tswitch (cacheMode) {\n\t\t\tcase Disabled:\n\t\t\t\tpb.environment().put(\"DISABLE_PROMPT_CACHING\", \"1\");\n\t\t\t\tbreak;\n\t\t\tcase Minutes_5:\n\t\t\t\tpb.environment().put(\"FORCE_PROMPT_CACHING_5M\", \"1\");\n\t\t\t\tbreak;\n\t\t\tcase Hours_1:\n\t\t\t\tpb.environment().put(\"ENABLE_PROMPT_CACHING_1H\", \"1\");\n\t\t\tcase Default:\n\t\t\tdefault:\n\t\t\t}\n\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_SPELLCHECK\", \"true\");\n\t\tpb.environment().put(\"CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT\", \"0\");\n\t\tpb.environment().put(\"MCP_TOOL_TIMEOUT\", \"86400000\");\n\t\tpb.environment().put(\"CLAUDE_ENABLE_STREAM_WATCHDOG\", \"0\");\n\t\tpb.environment().put(\"CLAUDE_ENABLE_BYTE_WATCHDOG\", \"0\");\n\t\tpb.environment().put(\"CLAUDE_STREAM_IDLE_TIMEOUT_MS\", \"86400000\");\n\t\tpb.environment().put(\"API_FORCE_IDLE_TIMEOUT\", \"0\");\n\t\tpb.environment().put(\"MCP_TIMEOUT\", \"86400000\");\n\n\t\tif (Reasoning.Disabled == reasoning) {\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING\", \"1\");\n\t\t\tpb.environment().put(\"MAX_THINKING_TOKENS\", \"0\");\n\t\t}\n\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_ADVISOR_TOOL\", \"1\");\n\t}"
        - id: ClaudeSessionParameters.setTitle
          type: method_declaration
          code: "public void setTitle(String title) {\n\t\tif (this.title == null)\n\t\t\tthis.title = title;\n\t}"
        - id: ClaudeSessionParameters.getTitle
          type: method_declaration
          code: "public String getTitle() {\n\t\treturn title;\n\t}"
```
/answer 7fa5-86 allow
ReasoningToken: 83
Control Request:
```yaml
id: 7fa5-87
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
  reason: Vorhandene Struktur des connector-Packages und evtl. bereits vorhandenes harness-Package prüfen
```
/answer 7fa5-87 allow
Control Request:
```yaml
id: 7fa5-88
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      error: Not a regular file.
```
/answer 7fa5-88 allow
ReasoningToken: 243
Control Request:
```yaml
id: 7fa5-89
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
    pattern: .*
    include:
    - '*.java'
    limit: 50
  reason: Dateien im connector-Package auflisten inkl. evtl. bereits vorhandenem harness-Unterpaket
```
/answer 7fa5-89 allow
Control Request:
```yaml
id: 7fa5-8a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      pattern: .*
      matches:
      - path: mcp/MCPClient.java
        lineno: 1
        match: package xy.ai.workbench.connector.mcp;
      - path: mcp/MCPClient.java
        lineno: 2
        match: ""
      - path: mcp/MCPClient.java
        lineno: 3
        match: import java.io.IOException;
      - path: mcp/MCPClient.java
        lineno: 4
        match: import java.net.URI;
      - path: mcp/MCPClient.java
        lineno: 5
        match: import java.net.http.HttpClient;
      - path: mcp/MCPClient.java
        lineno: 6
        match: import java.net.http.HttpRequest;
      - path: mcp/MCPClient.java
        lineno: 7
        match: import java.net.http.HttpRequest.BodyPublishers;
      - path: mcp/MCPClient.java
        lineno: 8
        match: import java.net.http.HttpResponse;
      - path: mcp/MCPClient.java
        lineno: 9
        match: import java.net.http.HttpResponse.BodyHandlers;
      - path: mcp/MCPClient.java
        lineno: 10
        match: import java.time.Duration;
      - path: mcp/MCPClient.java
        lineno: 11
        match: import java.util.UUID;
      - path: mcp/MCPClient.java
        lineno: 12
        match: ""
      - path: mcp/MCPClient.java
        lineno: 13
        match: import com.fasterxml.jackson.core.JsonProcessingException;
      - path: mcp/MCPClient.java
        lineno: 14
        match: import com.fasterxml.jackson.databind.JsonNode;
      - path: mcp/MCPClient.java
        lineno: 15
        match: import com.fasterxml.jackson.databind.ObjectMapper;
      - path: mcp/MCPClient.java
        lineno: 16
        match: import com.fasterxml.jackson.databind.node.ArrayNode;
      - path: mcp/MCPClient.java
        lineno: 17
        match: import com.fasterxml.jackson.databind.node.ObjectNode;
      - path: mcp/MCPClient.java
        lineno: 18
        match: ""
      - path: mcp/MCPClient.java
        lineno: 19
        match: import xy.ai.workbench.LOG;
      - path: mcp/MCPClient.java
        lineno: 20
        match: import xy.ai.workbench.connector.claudecode.JsonUtil;
      - path: mcp/MCPClient.java
        lineno: 21
        match: import xy.ai.workbench.connector.claudecode.YamlRenderer;
      - path: mcp/MCPClient.java
        lineno: 22
        match: ""
      - path: mcp/MCPClient.java
        lineno: 23
        match: /**
      - path: mcp/MCPClient.java
        lineno: 24
        match: " * Minimal MCP client over the Streamable-HTTP transport. Connection, session id"
      - path: mcp/MCPClient.java
        lineno: 25
        match: ' * and tool cache live 1:1 in this instance; no shared manager.'
      - path: mcp/MCPClient.java
        lineno: 26
        match: ' */'
      - path: mcp/MCPClient.java
        lineno: 27
        match: "public class MCPClient {"
      - path: mcp/MCPClient.java
        lineno: 28
        match: "\tprivate static final String SERVER_URL = \"http://localhost:9093/mcp\";"
      - path: mcp/MCPClient.java
        lineno: 29
        match: "\tprivate static final String PROTOCOL_VERSION = \"2025-06-18\";"
      - path: mcp/MCPClient.java
        lineno: 30
        match: "\tprivate static final Duration TIMEOUT = Duration.ofSeconds(300);"
      - path: mcp/MCPClient.java
        lineno: 31
        match: ""
      - path: mcp/MCPClient.java
        lineno: 32
        match: "\tprivate final ObjectMapper mapper = JsonUtil.mapper();"
      - path: mcp/MCPClient.java
        lineno: 33
        match: "\tprivate final YamlRenderer yaml = new YamlRenderer();"
      - path: mcp/MCPClient.java
        lineno: 34
        match: "\tprivate final HttpClient http = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(5)).build();"
      - path: mcp/MCPClient.java
        lineno: 35
        match: ""
      - path: mcp/MCPClient.java
        lineno: 36
        match: "\t/** Instance-stable session id sent as X-MCPC-SESSION-ID. */"
      - path: mcp/MCPClient.java
        lineno: 37
        match: "\tprivate final String sessionId = UUID.randomUUID().toString();"
      - path: mcp/MCPClient.java
        lineno: 38
        match: ""
      - path: mcp/MCPClient.java
        lineno: 39
        match: "\tprivate boolean initialized;"
      - path: mcp/MCPClient.java
        lineno: 40
        match: "\tprivate String mcpSessionId;"
      - path: mcp/MCPClient.java
        lineno: 41
        match: "\tprivate ArrayNode toolCache;"
      - path: mcp/MCPClient.java
        lineno: 42
        match: "\tprivate long nextId = 1;"
      - path: mcp/MCPClient.java
        lineno: 43
        match: ""
      - path: mcp/MCPClient.java
        lineno: 44
        match: "\tpublic JsonNode findTool(String name) {"
      - path: mcp/MCPClient.java
        lineno: 45
        match: "\t\tfor (JsonNode tool : listTools())"
      - path: mcp/MCPClient.java
        lineno: 46
        match: "\t\t\tif (name.equals(tool.path(\"name\").asText()))"
      - path: mcp/MCPClient.java
        lineno: 47
        match: "\t\t\t\treturn tool;"
      - path: mcp/MCPClient.java
        lineno: 48
        match: "\t\treturn null;"
      - path: mcp/MCPClient.java
        lineno: 49
        match: "\t}"
      - path: mcp/MCPClient.java
        lineno: 50
        match: ""
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 7fa5-8a allow
Control Request:
```yaml
id: 7fa5-8b
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: enum InputMode|enum Model|enum AgentProfile|enum Reasoning|enum CacheMode
    include:
    - '*.java'
    limit: 20
  reason: Enum-Typen für FrozenConfig-Felder identifizieren
```
/answer 7fa5-8b allow
Control Request:
```yaml
id: 7fa5-8c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      pattern: enum InputMode|enum Model|enum AgentProfile|enum Reasoning|enum CacheMode
      matches:
      - path: AgentProfile.java
        lineno: 3
        match: "public enum AgentProfile {"
      - path: Reasoning.java
        lineno: 3
        match: "public enum Reasoning {"
      - path: CacheMode.java
        lineno: 3
        match: "public enum CacheMode {"
      - path: InputMode.java
        lineno: 3
        match: "public enum InputMode {"
      - path: connector/openapi/deepseek/components/ModelEnum.java
        lineno: 4
        match: "public enum ModelEnum {"
```
/answer 7fa5-8c allow
ReasoningToken: 290
Control Request:
```yaml
id: 7fa5-8d
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Model.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/InputMode.java
  reason: Model- und InputMode-Definitionen prüfen für FrozenConfig/Prompt-Felder
```
/answer 7fa5-8d allow
Control Request:
```yaml
id: 7fa5-8e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Model.java
      nodes:
      - id: 0K7tDI
        type: imports
      - id: Model
        type: class_declaration
        signature: "public class Model {"
        children:
        - id: Model.QWbGOe
          type: statements
        - id: Model.Model
          type: constructor_declaration
          signature: "public Model(String apiName, String displayName, Capabilities cap) {"
        - id: Model.Model_1
          type: constructor_declaration
          signature: "public Model(String apiName, Capabilities cap) {"
        - id: Model.toString
          type: method_declaration
          signature: '@Override'
        - id: Model.M2feyf
          type: statements
        - id: Model.matches
          type: method_declaration
          signature: "public boolean matches(KeyPattern provider, String apiName) {"
        - id: Model.KeyPattern
          type: enum_declaration
          signature: "public static enum KeyPattern {"
        - id: Model.OzDieF
          type: statements
        - id: Model.pYppjh
          type: statements
        - id: Model.vVIZoo
          type: statements
        - id: Model.DXquGb
          type: statements
        - id: Model.XrqNac
          type: statements
        - id: Model.eKYUmB
          type: statements
        - id: Model.q9Siib
          type: statements
        - id: Model.rD8bED
          type: statements
        - id: Model.AxuOBM
          type: statements
        - id: Model.qRUyIY
          type: statements
        - id: Model.P6unl5
          type: statements
        - id: Model.KeBr6V
          type: statements
        - id: Model.W6DKaK
          type: statements
        - id: Model.defaultsFor
          type: method_declaration
          signature: "public static Model[] defaultsFor(KeyPattern provider) {"
        - id: Model.Capabilities
          type: class_declaration
          signature: "public static class Capabilities {"
          children:
          - id: Model.Capabilities.CTo3aN
            type: statements
          - id: Model.Capabilities.sQJsIa
            type: statements
          - id: Model.Capabilities.supportTemperature
            type: method_declaration
            signature: "public Capabilities supportTemperature(boolean flag) {"
          - id: Model.Capabilities.tools
            type: method_declaration
            signature: "public Capabilities tools(String[] tools) {"
          - id: Model.Capabilities.agentProfiles
            type: method_declaration
            signature: "public Capabilities agentProfiles(AgentProfile... profiles) {"
          - id: Model.Capabilities.supportTopP
            type: method_declaration
            signature: "public Capabilities supportTopP(boolean flag) {"
          - id: Model.Capabilities.supportMaxToken
            type: method_declaration
            signature: "public Capabilities supportMaxToken(boolean supportMaxToken) {"
          - id: Model.Capabilities.cacheMode
            type: method_declaration
            signature: "public Capabilities cacheMode(CacheMode... cacheMode) {"
          - id: Model.Capabilities.supportBatch
            type: method_declaration
            signature: "public Capabilities supportBatch(boolean supportBatch) {"
          - id: Model.Capabilities.budget
            type: method_declaration
            signature: "public Capabilities budget(int min, int max) {"
          - id: Model.Capabilities.outTokens
            type: method_declaration
            signature: "public Capabilities outTokens(int min, int max) {"
          - id: Model.Capabilities.key
            type: method_declaration
            signature: "public Capabilities key(KeyPattern pattern) {"
          - id: Model.Capabilities.reasonings
            type: method_declaration
            signature: "public Capabilities reasonings(Reasoning... val) {"
          - id: Model.Capabilities.isSupportTemperature
            type: method_declaration
            signature: "public boolean isSupportTemperature() {"
          - id: Model.Capabilities.isSupportTopP
            type: method_declaration
            signature: "public boolean isSupportTopP() {"
          - id: Model.Capabilities.isSupportMaxToken
            type: method_declaration
            signature: "public boolean isSupportMaxToken() {"
          - id: Model.Capabilities.isSupportBatch
            type: method_declaration
            signature: "public boolean isSupportBatch() {"
          - id: Model.Capabilities.getReasonings
            type: method_declaration
            signature: "public Reasoning[] getReasonings() {"
          - id: Model.Capabilities.getKeyPattern
            type: method_declaration
            signature: "public KeyPattern getKeyPattern() {"
          - id: Model.Capabilities.getAgentProfiles
            type: method_declaration
            signature: "public AgentProfile[] getAgentProfiles() {"
          - id: Model.Capabilities.getTools
            type: method_declaration
            signature: "public String[] getTools() {"
          - id: Model.Capabilities.getCacheMode
            type: method_declaration
            signature: "public CacheMode[] getCacheMode() {"
          - id: Model.Capabilities.alignOutpuTokens
            type: method_declaration
            signature: "public int alignOutpuTokens(int tokens) {"
          - id: Model.Capabilities.alignBudget
            type: method_declaration
            signature: "public int alignBudget(int budget) {"
          - id: Model.Capabilities.acceptsKey
            type: method_declaration
            signature: "public boolean acceptsKey(String key) {"
          - id: Model.Capabilities.alignTemperature
            type: method_declaration
            signature: "public Double alignTemperature(Double temperature) {"
          - id: Model.Capabilities.alignTopP
            type: method_declaration
            signature: "public Double alignTopP(Double topP) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/InputMode.java
      nodes:
      - id: 2atbOS
        type: imports
      - id: InputMode
        type: enum_declaration
        signature: "public enum InputMode {"
```
/answer 7fa5-8e allow
ReasoningToken: 26
Control Request:
```yaml
id: 7fa5-8f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
    ids:
    - ConfigManager.getTools
    - ConfigManager.getSystemPrompt
    - ConfigManager.getFreeText
    - ConfigManager.getKeys
    - ConfigManager.getMaxOutputTokens
  reason: Implementierungsdetails prüfen (Null-Handling) für FrozenConfig.from
```
/answer 7fa5-8f allow
Control Request:
```yaml
id: 7fa5-90
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
      nodes:
      - id: ConfigManager.getTools
        type: method_declaration
        code: "public String[] getTools() {\n\t\treturn enabledTools;\n\t}"
      - id: ConfigManager.getSystemPrompt
        type: method_declaration
        code: "public String[] getSystemPrompt() {\n\t\treturn cfg.getSystemPrompt();\n\t}"
      - id: ConfigManager.getFreeText
        type: method_declaration
        code: "public String getFreeText() {\n\t\treturn cfg.getFreeText();\n\t}"
      - id: ConfigManager.getKeys
        type: method_declaration
        code: "public String getKeys() {\n\t\treturn cfg.getKeys();\n\t}"
      - id: ConfigManager.getMaxOutputTokens
        type: method_declaration
        code: "public Long getMaxOutputTokens() {\n\t\treturn cfg.getMaxOutputTokens();\n\t}"
```
/answer 7fa5-90 allow
ReasoningToken: 2875
Control Request:
```yaml
id: 7fa5-91
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AgentProfile.java
    ids:
    - AgentProfile
  reason: Sichtbarkeit des name-Feldes von AgentProfile prüfen
```
/answer 7fa5-91 allow
Control Request:
```yaml
id: 7fa5-92
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AgentProfile.java
      nodes:
      - id: AgentProfile
        type: enum_declaration
        code: "public enum AgentProfile {\n\tbasic(\"default\"), author, research, code, python, web_research(\"web-research\"), markdown, code_rw(\"code-rw\"),\n\tcode_plan(\"code-plan\"), github_research(\"github-research\"), MCPC;\n\n\tpublic final String name;\n\n\tAgentProfile() {\n\t\tthis.name = this.name();\n\t}\n\n\tAgentProfile(String name) {\n\t\tthis.name = name;\n\t}\n\n\tpublic static AgentProfile fromName(String value) {\n\t\tfor (var e : values())\n\t\t\tif (e.name.equals(value))\n\t\t\t\treturn e;\n\t\tthrow new IllegalArgumentException(\"Not a valid AgentProfile name\");\n\t}\n}"
```
/answer 7fa5-92 allow
ReasoningToken: 3498
Text:
 Jetzt erstelle ich beide Dateien.
Control Request:
```yaml
id: 7fa5-93
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/FrozenConfig.java
    source: "package xy.ai.workbench.connector.harness;\n\nimport java.nio.charset.StandardCharsets;\nimport java.security.MessageDigest;\nimport java.security.NoSuchAlgorithmException;\nimport java.util.Arrays;\nimport java.util.List;\n\nimport xy.ai.workbench.AgentProfile;\nimport xy.ai.workbench.CacheMode;\nimport xy.ai.workbench.ConfigManager;\nimport xy.ai.workbench.Model;\nimport xy.ai.workbench.Reasoning;\n\n/**\n * Immutable snapshot of the {@link ConfigManager} state, taken once when a\n * {@link Prompt} is frozen. The system prompt is rendered eagerly so that\n * later changes to the live config do not affect already-frozen sessions.\n */\npublic final class FrozenConfig {\n\n\tpublic final Model model;\n\tpublic final AgentProfile profile;\n\tpublic final Reasoning reasoning;\n\tpublic final List<String> tools;\n\tpublic final String systemPrompt;\n\tpublic final Double topP;\n\tpublic final Double temperature;\n\tpublic final Long maxOutputTokens;\n\tpublic final CacheMode cacheMode;\n\tpublic final String cliProfile;\n\n\tprivate String hash;\n\n\tprivate FrozenConfig(Model model, AgentProfile profile, Reasoning reasoning, List<String> tools,\n\t\t\tString systemPrompt, Double topP, Double temperature, Long maxOutputTokens, CacheMode cacheMode,\n\t\t\tString cliProfile) {\n\t\tthis.model = model;\n\t\tthis.profile = profile;\n\t\tthis.reasoning = reasoning;\n\t\tthis.tools = tools;\n\t\tthis.systemPrompt = systemPrompt;\n\t\tthis.topP = topP;\n\t\tthis.temperature = temperature;\n\t\tthis.maxOutputTokens = maxOutputTokens;\n\t\tthis.cacheMode = cacheMode;\n\t\tthis.cliProfile = cliProfile;\n\t}\n\n\t/** Reads all relevant values from {@code cfg} once and renders the system prompt. */\n\tpublic static FrozenConfig from(ConfigManager cfg) {\n\t\tString[] rawPrompt = cfg.getSystemPrompt();\n\t\tStringBuilder sb = new StringBuilder();\n\t\tArrays.stream(rawPrompt != null ? rawPrompt : new String[0]).filter(e -> !e.startsWith(\"#\"))\n\t\t\t\t.forEach(e -> sb.append(\"* \").append(e).append(\".\\n\"));\n\t\tString freeText = cfg.getFreeText();\n\t\tif (freeText != null && !freeText.isBlank())\n\t\t\tsb.append(\".\\n\").append(freeText).append(\".\\n\");\n\n\t\tString[] rawTools = cfg.getTools();\n\t\tList<String> tools = List.of(rawTools != null ? rawTools : new String[0]);\n\n\t\treturn new FrozenConfig(cfg.getModel(), cfg.getProfile(), cfg.getReasoning(), tools, sb.toString(),\n\t\t\t\tcfg.getTopP(), cfg.getTemperature(), cfg.getMaxOutputTokens(), cfg.getCacheMode(), cfg.getKeys());\n\t}\n\n\t/**\n\t * Deterministic hash over exactly: model, profile, reasoning, tools,\n\t * systemPrompt, topP, temperature, maxOutputTokens.\n\t */\n\tpublic String getHash() {\n\t\tif (hash == null)\n\t\t\thash = computeHash();\n\t\treturn hash;\n\t}\n\n\tprivate String computeHash() {\n\t\tString input = (model != null ? model.apiName : \"\") + \"|\" + (profile != null ? profile.name : \"\") + \"|\"\n\t\t\t\t+ (reasoning != null ? reasoning.name() : \"\") + \"|\" + String.join(\",\", tools) + \"|\" + systemPrompt\n\t\t\t\t+ \"|\" + topP + \"|\" + temperature + \"|\" + maxOutputTokens;\n\t\ttry {\n\t\t\tMessageDigest md = MessageDigest.getInstance(\"MD5\");\n\t\t\tbyte[] bytes = md.digest(input.getBytes(StandardCharsets.UTF_8));\n\t\t\tStringBuilder sb = new StringBuilder();\n\t\t\tfor (byte b : bytes)\n\t\t\t\tsb.append(String.format(\"%02x\", b));\n\t\t\treturn sb.substring(0, 8);\n\t\t} catch (NoSuchAlgorithmException e) {\n\t\t\t// Stable fallback (no external dependency)\n\t\t\tlong h = 0;\n\t\t\tfor (char c : input.toCharArray())\n\t\t\t\th = h * 31L + c;\n\t\t\treturn String.format(\"%08x\", h & 0xFFFFFFFFL);\n\t\t}\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
    source: "package xy.ai.workbench.connector.harness;\n\nimport java.nio.charset.StandardCharsets;\nimport java.nio.file.Path;\nimport java.security.MessageDigest;\nimport java.security.NoSuchAlgorithmException;\nimport java.util.List;\n\nimport xy.ai.workbench.connector.SessionParameters;\nimport xy.ai.workbench.connector.claudecode.ClaudeSessionParameters;\n\n/**\n * Immutable, frozen prompt: semantically separated Inputs / Batch / Config,\n * plus the activation state for the SessionProcessor. Construction happens\n * exclusively via {@link Builder}, driven by the PromptHandler (Step 06).\n */\npublic final class Prompt {\n\n\t/** Origin of the collected input text (see Step 06). */\n\tpublic enum InputKind {\n\t\tLineInput, BlockSelection, FullFile\n\t}\n\n\t/** Typed command payload, captured alongside an optional YAML block (Step 06/08). */\n\tpublic enum Command {\n\t\tNONE, CALL\n\t}\n\n\t// Inputs\n\tpublic final List<String> inputs;\n\tpublic final InputKind kind;\n\n\t// Batch\n\tpublic final boolean batch;\n\n\t// Config\n\tpublic final FrozenConfig config;\n\n\tpublic final boolean processorEnabled;\n\tpublic final String absoluteFilePath;\n\n\t// Optional command payload\n\tpublic final Command command;\n\tpublic final String yamlBlock;\n\n\tprivate String sessionId;\n\n\tprivate Prompt(List<String> inputs, InputKind kind, boolean batch, FrozenConfig config, boolean processorEnabled,\n\t\t\tString absoluteFilePath, Command command, String yamlBlock) {\n\t\tif (config == null)\n\t\t\tthrow new IllegalArgumentException(\"config must not be null\");\n\t\tif (absoluteFilePath == null || absoluteFilePath.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"absoluteFilePath must not be blank\");\n\t\tthis.inputs = inputs != null ? List.copyOf(inputs) : List.of();\n\t\tthis.kind = kind;\n\t\tthis.batch = batch;\n\t\tthis.config = config;\n\t\tthis.processorEnabled = processorEnabled;\n\t\tthis.absoluteFilePath = absoluteFilePath;\n\t\tthis.command = command != null ? command : Command.NONE;\n\t\tthis.yamlBlock = yamlBlock;\n\t}\n\n\t/** Deterministic session id: hash(absoluteFilePath, batch, config.getHash()). */\n\tpublic String sessionId() {\n\t\tif (sessionId == null)\n\t\t\tsessionId = computeSessionId();\n\t\treturn sessionId;\n\t}\n\n\tprivate String computeSessionId() {\n\t\tString input = absoluteFilePath + \"|\" + batch + \"|\" + config.getHash();\n\t\ttry {\n\t\t\tMessageDigest md = MessageDigest.getInstance(\"MD5\");\n\t\t\tbyte[] bytes = md.digest(input.getBytes(StandardCharsets.UTF_8));\n\t\t\tStringBuilder sb = new StringBuilder();\n\t\t\tfor (byte b : bytes)\n\t\t\t\tsb.append(String.format(\"%02x\", b));\n\t\t\treturn sb.substring(0, 8);\n\t\t} catch (NoSuchAlgorithmException e) {\n\t\t\t// Stable fallback (no external dependency)\n\t\t\tlong h = 0;\n\t\t\tfor (char c : input.toCharArray())\n\t\t\t\th = h * 31L + c;\n\t\t\treturn String.format(\"%08x\", h & 0xFFFFFFFFL);\n\t\t}\n\t}\n\n\t/** Derives generic SessionParameters from {@code prompt.config} (replaces Step 07's fromConfig(cfg, ...)). */\n\tpublic static SessionParameters toSessionParameters(Prompt prompt, Path cwd) {\n\t\tFrozenConfig c = prompt.config;\n\t\treturn new SessionParameters(cwd, c.systemPrompt, c.tools, c.model, c.reasoning, c.profile, c.cliProfile,\n\t\t\t\tc.cacheMode, prompt.absoluteFilePath, c.topP, c.temperature,\n\t\t\t\tc.maxOutputTokens != null ? c.maxOutputTokens.intValue() : null);\n\t}\n\n\t/** Derives Claude-specific SessionParameters from {@code prompt.config}. */\n\tpublic static ClaudeSessionParameters toClaudeSessionParameters(Prompt prompt, Path cwd) {\n\t\tFrozenConfig c = prompt.config;\n\t\treturn new ClaudeSessionParameters(cwd, c.systemPrompt, c.tools, c.model, c.reasoning, c.profile, c.cliProfile,\n\t\t\t\tc.cacheMode, prompt.absoluteFilePath);\n\t}\n\n\tpublic static Builder builder() {\n\t\treturn new Builder();\n\t}\n\n\t/** Builder for {@link Prompt}; intended for use by the PromptHandler (Step 06). */\n\tpublic static final class Builder {\n\t\tprivate List<String> inputs;\n\t\tprivate InputKind kind;\n\t\tprivate boolean batch;\n\t\tprivate FrozenConfig config;\n\t\tprivate boolean processorEnabled;\n\t\tprivate String absoluteFilePath;\n\t\tprivate Command command;\n\t\tprivate String yamlBlock;\n\n\t\tprivate Builder() {\n\t\t}\n\n\t\tpublic Builder inputs(List<String> inputs) {\n\t\t\tthis.inputs = inputs;\n\t\t\treturn this;\n\t\t}\n\n\t\tpublic Builder kind(InputKind kind) {\n\t\t\tthis.kind = kind;\n\t\t\treturn this;\n\t\t}\n\n\t\tpublic Builder batch(boolean batch) {\n\t\t\tthis.batch = batch;\n\t\t\treturn this;\n\t\t}\n\n\t\tpublic Builder config(FrozenConfig config) {\n\t\t\tthis.config = config;\n\t\t\treturn this;\n\t\t}\n\n\t\tpublic Builder processorEnabled(boolean processorEnabled) {\n\t\t\tthis.processorEnabled = processorEnabled;\n\t\t\treturn this;\n\t\t}\n\n\t\tpublic Builder absoluteFilePath(String absoluteFilePath) {\n\t\t\tthis.absoluteFilePath = absoluteFilePath;\n\t\t\treturn this;\n\t\t}\n\n\t\tpublic Builder command(Command command) {\n\t\t\tthis.command = command;\n\t\t\treturn this;\n\t\t}\n\n\t\tpublic Builder yamlBlock(String yamlBlock) {\n\t\t\tthis.yamlBlock = yamlBlock;\n\t\t\treturn this;\n\t\t}\n\n\t\tpublic Prompt build() {\n\t\t\treturn new Prompt(inputs, kind, batch, config, processorEnabled, absoluteFilePath, command, yamlBlock);\n\t\t}\n\t}\n}\n"
  reason: FrozenConfig.java und Prompt.java gemäß Step 05 im neuen harness-Package anlegen
```
/answer 7fa5-93 allow
Control Request:
```yaml
id: 7fa5-94
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/FrozenConfig.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
  reason: Kompilierbarkeit der neu erstellten Dateien prüfen
```
/answer 7fa5-94 allow
Result Stats:  id=959d78c1-3602-4b48-9232-6b7c79fa496c, total: 0, in: 155, out: 14415, reason: 0, read: 254687, write: 34907
Beide Dateien wurden erstellt und validiert erfolgreich:

- **`FrozenConfig.java`**: unveränderliche Momentaufnahme mit Factory `from(ConfigManager)`, gerendertem Systemprompt und `getHash()` über genau die 8 geforderten Felder (MD5→hex, mit Fallback).
- **`Prompt.java`**: immutable, mit `InputKind`, `Command`, semantisch getrennten Inputs/Batch/Config, `processorEnabled`, `sessionId()` (Hash aus Pfad+Batch+ConfigHash), Builder für Step 06 sowie statischen Ableitungen `toSessionParameters`/`toClaudeSessionParameters` für Step 07.
