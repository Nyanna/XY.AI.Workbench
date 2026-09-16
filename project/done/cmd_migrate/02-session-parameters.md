# Step 02 – `SessionParameters` extrahieren (Phase 1)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
**Ziel:** Generische Session-Parameter (inkl. deterministischem Hash) aus `ClaudeSessionParameters`
in eine neue Basisklasse `connector/SessionParameters.java` herauslösen. `ClaudeSessionParameters`
leitet davon ab und behält nur CC-Spezifika (CLI-/Env-Aufbau, Title).
**Abhängigkeiten:** keine (kann parallel zu Step 01). Grundlage für `FrozenConfig`-Hash (Step 05).

## Ist-Zustand `connector/claudecode/ClaudeSessionParameters.java`
Felder (Node `ClaudeSessionParameters.igRtKw` + `.ZbqmG7`):
```java
private static final String SCRIPT = System.getProperty("user.home") + "/xyan/xy.ai.workbench/claude-code/claude-session.sh";
private static final String COMMAND = "claude";
public final Path cwd; public final String systemPrompt; public final List<String> tools;
public final Model model; public final Reasoning reasoning; public final AgentProfile agentProfile;
public final String cliProfile; public final String filePath; public final CacheMode cacheMode;
private String hash; private String title;
```
Methoden (Node-IDs): `.ClaudeSessionParameters` (2 Konstruktoren), `.fromConfig`, `.getFilePath`,
`.buildBaseCommand`, `.buildEvironment`, `.getHash`, `.computeHash`, `.setTitle`, `.getTitle`.

`computeHash()` (Ist):
```java
String input = String.join(",", tools) + "|" + cwd.toString() + "|" + model.apiName + "|"
    + reasoning.name() + "|" + (agentProfile != null ? agentProfile.name : "") + "|"
    + cliProfile + "|" + (filePath != null ? filePath : "");
// MD5 -> hex, substring(0,8); Fallback: 31er-Rolling-Hash
```
`fromConfig(ConfigManager cfg, Path cwd, String filePath, String systemPrompt, List<String> tools)`
nutzt `cfg.getModel/getReasoning/getProfile/getKeys/getCacheMode`.

## Aktionen
1. Neue Datei `connector/SessionParameters.java` (package `xy.ai.workbench.connector`):
   - Generische `public final` Felder: `cwd`, `filePath`, `systemPrompt`, `tools`, `model`,
     `reasoning`, `agentProfile`, `cliProfile`, `cacheMode`.
   - Optionale Config-Felder für spätere Nutzung durch `FrozenConfig`: `topP`, `temperature`,
     `maxOutputTokens` (Double/Integer, nullbar).
   - Konstruktor mit Validierung (aus `ClaudeSessionParameters` übernehmen: cwd!=null, model!=null,
     model.apiName nicht blank, reasoning!=null).
   - `getHash()` + `computeHash()` (Logik 1:1 übernehmen). Hash-Parameter-Set unverändert lassen,
     um bestehende Session-Zuordnung stabil zu halten (Prefix-Caching!).
   - `getFilePath()`.
2. `ClaudeSessionParameters extends SessionParameters`:
   - Nur noch behalten: `SCRIPT`, `COMMAND`, `title`, `setTitle`, `getTitle`, `buildBaseCommand`,
     `buildEvironment`, `fromConfig` (Factory bleibt hier).
   - Konstruktoren rufen `super(...)` auf; `hash`/generische Felder entfallen lokal.
   - `getHash`/`computeHash`/generische Felder aus dieser Klasse **entfernen** (kommen von Basis).
3. Zugriffe prüfen: `params.model`, `params.tools`, `params.cwd`, `params.getHash()`, `params.filePath`
   etc. funktionieren weiter über Vererbung. `CCConnector.executeRequest` nutzt
   `ClaudeSessionParameters.fromConfig(...)` + `params.setTitle(req.title)` – unverändert.

## Hinweise / Fallstricke
- **Hash-Stabilität:** Parameter-String für den Hash NICHT verändern.
- CC-spezifische Env/CLI-Logik (`buildEvironment`, `buildBaseCommand`) bleibt in `ClaudeSessionParameters`.
- Sichtbarkeit der Felder als `public final` beibehalten (viele Direktzugriffe).

## Validierung
`ast_validate` für `connector/SessionParameters.java`,
`connector/claudecode/ClaudeSessionParameters.java`, `connector/claudecode/CCConnector.java`,
`connector/claudecode/CCSessionManager.java` (nutzt Parameter), sowie alle Referenzen von
`ClaudeSessionParameters` (grep `ClaudeSessionParameters`).

SystemInit:  id=7029cc3a-91cb-4a2d-9f4b-09790cf765e7, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7fa5-36
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/claudecode/ClaudeSessionParameters.java
    ids:
    - .igRtKw
    - .ZbqmG7
    - .ClaudeSessionParameters
    - .fromConfig
    - .getFilePath
    - .buildBaseCommand
    - .buildEvironment
    - .getHash
    - .computeHash
    - .setTitle
    - .getTitle
  reason: "Aktuellen Code von ClaudeSessionParameters lesen, um Refactoring durchzuführen."
```
/answer 7fa5-36 allow
Control Request:
```yaml
id: 7fa5-37
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/claudecode/ClaudeSessionParameters.java
      error: File not found.
```
/answer 7fa5-37 allow
Control Request:
```yaml
id: 7fa5-38
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/claudecode/ClaudeSessionParameters.java
  reason: Struktur der Datei ermitteln für genaue Node-IDs.
```
/answer 7fa5-38 allow
Control Request:
```yaml
id: 7fa5-39
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/connector/claudecode/ClaudeSessionParameters.java
      error: File not found.
```
/answer 7fa5-39 allow
Control Request:
```yaml
id: 7fa5-3a
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class ClaudeSessionParameters
    include:
    - '*.java'
  reason: Tatsächlichen Pfad der Datei ClaudeSessionParameters.java finden.
```
/answer 7fa5-3a allow
Control Request:
```yaml
id: 7fa5-3b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      pattern: class ClaudeSessionParameters
      matches:
      - path: src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
        lineno: 17
        match: "public class ClaudeSessionParameters {"
```
/answer 7fa5-3b allow
ReasoningToken: 14
Control Request:
```yaml
id: 7fa5-3c
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
  reason: Struktur der Datei ermitteln für genaue Node-IDs.
```
/answer 7fa5-3c allow
Control Request:
```yaml
id: 7fa5-3d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
      nodes:
      - id: UIM5xW
        type: imports
      - id: ClaudeSessionParameters
        type: class_declaration
        signature: "public class ClaudeSessionParameters {"
        children:
        - id: ClaudeSessionParameters.igRtKw
          type: statements
        - id: ClaudeSessionParameters.ZbqmG7
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
        - id: ClaudeSessionParameters.getFilePath
          type: method_declaration
          signature: "public String getFilePath() {"
        - id: ClaudeSessionParameters.buildBaseCommand
          type: method_declaration
          signature: "public List<String> buildBaseCommand() {"
        - id: ClaudeSessionParameters.buildEvironment
          type: method_declaration
          signature: "public void buildEvironment(ProcessBuilder pb) {"
        - id: ClaudeSessionParameters.getHash
          type: method_declaration
          signature: "public String getHash() {"
        - id: ClaudeSessionParameters.computeHash
          type: method_declaration
          signature: "private String computeHash() {"
        - id: ClaudeSessionParameters.setTitle
          type: method_declaration
          signature: "public void setTitle(String title) {"
        - id: ClaudeSessionParameters.getTitle
          type: method_declaration
          signature: "public String getTitle() {"
```
/answer 7fa5-3d allow
ReasoningToken: 10
Control Request:
```yaml
id: 7fa5-3e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    ids:
    - UIM5xW
    - ClaudeSessionParameters.igRtKw
    - ClaudeSessionParameters.ZbqmG7
    - ClaudeSessionParameters.ClaudeSessionParameters
    - ClaudeSessionParameters.ClaudeSessionParameters_1
    - ClaudeSessionParameters.fromConfig
    - ClaudeSessionParameters.getFilePath
    - ClaudeSessionParameters.buildBaseCommand
    - ClaudeSessionParameters.buildEvironment
    - ClaudeSessionParameters.getHash
    - ClaudeSessionParameters.computeHash
    - ClaudeSessionParameters.setTitle
    - ClaudeSessionParameters.getTitle
  reason: Vollständigen Code lesen für Refactoring in Basisklasse.
```
/answer 7fa5-3e allow
Control Request:
```yaml
id: 7fa5-3f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
      nodes:
      - id: UIM5xW
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import java.nio.charset.StandardCharsets;
          import java.nio.file.Path;
          import java.security.MessageDigest;
          import java.security.NoSuchAlgorithmException;
          import java.util.ArrayList;
          import java.util.Collections;
          import java.util.List;

          import xy.ai.workbench.AgentProfile;
          import xy.ai.workbench.CacheMode;
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.Model;
          import xy.ai.workbench.Reasoning;
      - id: ClaudeSessionParameters.igRtKw
        type: statements
        code: "private static final String SCRIPT = System.getProperty(\"user.home\")\n\t\t\t+ \"/xyan/xy.ai.workbench/claude-code/claude-session.sh\";\n\tprivate static final String COMMAND = \"claude\";\n\n\t/** Deterministic hash of the session parameters. Immutable. */\n\tpublic final Path cwd;\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final Model model;\n\tpublic final Reasoning reasoning;\n\tpublic final AgentProfile agentProfile;\n\tpublic final String cliProfile;\n\tpublic final String filePath;"
      - id: ClaudeSessionParameters.ZbqmG7
        type: statements
        code: "public final CacheMode cacheMode;\n\tprivate String hash;\n\tprivate String title;"
      - id: ClaudeSessionParameters.ClaudeSessionParameters
        type: constructor_declaration
        code: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode) {\n\t\tthis(cwd, systemPrompt, tools, model, reasoning, agentProfile, cliProfile, cacheMode, null);\n\t}"
      - id: ClaudeSessionParameters.ClaudeSessionParameters_1
        type: constructor_declaration
        code: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath) {\n\t\tif (cwd == null)\n\t\t\tthrow new IllegalStateException(\"Work directory (cwd) not set\");\n\t\tif (model == null)\n\t\t\tthrow new IllegalArgumentException(\"Model must not be null\");\n\t\tif (model.apiName == null || model.apiName.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"Model apiName must not be null or blank\");\n\t\tif (reasoning == null)\n\t\t\tthrow new IllegalArgumentException(\"Reasoning must not be null\");\n\n\t\tthis.cwd = cwd;\n\t\tthis.systemPrompt = systemPrompt != null ? systemPrompt : \"\";\n\t\tthis.tools = tools != null ? tools : Collections.emptyList();\n\t\tthis.model = model;\n\t\tthis.reasoning = reasoning;\n\t\tthis.agentProfile = agentProfile;\n\t\tthis.cliProfile = cliProfile;\n\t\tthis.filePath = filePath;\n\t\tthis.cacheMode = cacheMode;\n\t}"
      - id: ClaudeSessionParameters.fromConfig
        type: method_declaration
        code: "public static ClaudeSessionParameters fromConfig(ConfigManager cfg, Path cwd, String filePath, String systemPrompt,\n\t\t\tList<String> tools) {\n\t\treturn new ClaudeSessionParameters(cwd, systemPrompt, tools, cfg.getModel(), cfg.getReasoning(), cfg.getProfile(),\n\t\t\t\tcfg.getKeys(), cfg.getCacheMode(), filePath);\n\t}"
      - id: ClaudeSessionParameters.getFilePath
        type: method_declaration
        code: "public String getFilePath() {\n\t\treturn filePath;\n\t}"
      - id: ClaudeSessionParameters.buildBaseCommand
        type: method_declaration
        code: "public List<String> buildBaseCommand() {\n\t\tList<String> cmd = new ArrayList<>();\n\t\tif (AgentProfile.MCPC.equals(agentProfile)) {\n\t\t\tcmd.add(COMMAND);\n\t\t\tcmd.add(\"--system-prompt\");\n\t\t\tcmd.add(systemPrompt);\n\t\t\tcmd.add(\"--tools\");\n\t\t\tcmd.add(\"\"); // restrict builtin tools\n\t\t\t// evil: breaks STDIN handling\n//\t\t\tcmd.add(\"--debug\");\n//\t\t\tcmd.add(\"mcp\");\n\t\t\tcmd.add(\"--settings\");\n\t\t\tcmd.add(\"\"\"\n\t\t\t\t\t{\n\t\t\t\t\t\t\"hooks\": {\n\t\t\t\t\t\t\t\"PreToolUse\": [\n\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\"hooks\": [\n\t\t\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\t\t\"type\": \"http\",\n\t\t\t\t\t\t\t\t\t\t\t\"url\":\"http://localhost:9093/hooks/tool\",\n\t\t\t\t\t\t\t\t\t\t\t\"headers\":{\n\t\t\t\t\t\t\t\t\t\t\t   \"X-MCPC-SESSION-ID\":\"$MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t},\n\t\t\t\t\t\t\t\t\t\t\t\"allowedEnvVars\":[\n\t\t\t\t\t\t\t\t\t\t\t   \"MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t],\n\t\t\t\t\t\t\t\t\t\t\t\"timeout\": 86400\n\t\t\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t]\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t],\n\t\t\t\t\t\t\t\"PermissionRequest\": [\n\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\"hooks\": [\n\t\t\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\t\t\"type\": \"http\",\n\t\t\t\t\t\t\t\t\t\t\t\"url\":\"http://localhost:9093/hooks/permission\",\n\t\t\t\t\t\t\t\t\t\t\t\"headers\":{\n\t\t\t\t\t\t\t\t\t\t\t   \"X-MCPC-SESSION-ID\":\"$MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t},\n\t\t\t\t\t\t\t\t\t\t\t\"allowedEnvVars\":[\n\t\t\t\t\t\t\t\t\t\t\t   \"MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t],\n\t\t\t\t\t\t\t\t\t\t\t\"timeout\": 86400\n\t\t\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t]\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t]\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t\t\"\"\".replace(\"\\t\", \"\").replace(\"\\n\", \"\").strip());\n\t\t\tcmd.add(\"--mcp-config\");\n\t\t\tcmd.add(\"\"\"\n\t\t\t\t\t{\n\t\t\t\t\t\t\"mcpServers\": {\n\t\t\t\t\t\t\t\"mcpc\": {\n\t\t\t\t\t\t\t\t\"type\": \"ws\",\n\t\t\t\t\t\t\t\t\"url\": \"http://localhost:9094/mcp\",\n\t\t\t\t\t\t\t\t\"timeout\": 86400000,\n\t\t\t\t\t\t\t\t\"alwaysLoad\": true,\n\t\t\t\t\t\t\t\t\"headers\": {\n\t\t\t\t\t\t\t\t\t\"X-MCPC-SESSION-ID\": \"${MCPC_SESSION_ID}\",\n\t\t\t\t\t\t\t\t\t\"X-MCPC-TOOLS\": \"${MCPC_TOOLS}\",\n\t\t\t\t\t\t\t\t\t\"X-MCPC-CC-PROFILE\": \"${MCPC_CC_PROFILE}\"\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t\t\"\"\".replace(\"\\t\", \"\").replace(\"\\n\", \"\").strip());\n\t\t} else {\n\t\t\tcmd.add(SCRIPT);\n\t\t\tcmd.add(agentProfile != null ? agentProfile.name : \"\"); // Agent definition\n\t\t\tcmd.add(\"--profile\");\n\t\t\tcmd.add(cliProfile);\n\t\t}\n\n\t\tcmd.add(\"--verbose\");\n\t\t// replaced by MCPC\n\t\t// cmd.add(\"--include-hook-events\");\n\t\tcmd.add(\"--include-partial-messages\");\n\t\tcmd.add(\"--input-format\");\n\t\tcmd.add(\"stream-json\");\n\t\tcmd.add(\"--output-format\");\n\t\tcmd.add(\"stream-json\");\n\t\t// replaced by self mirror input\n\t\t// cmd.add(\"--replay-user-messages\");\n\t\tcmd.add(\"--model\");\n\t\tcmd.add(model.apiName);\n\t\tif (Reasoning.Disabled != reasoning) {\n\t\t\tcmd.add(\"--effort\");\n\t\t\tcmd.add(reasoning.name().toLowerCase());\n\t\t}\n\t\tcmd.add(\"--dangerously-skip-permissions\"); // as long there is no permission prompt handling implemented\n\t\treturn cmd;\n\t}"
      - id: ClaudeSessionParameters.buildEvironment
        type: method_declaration
        code: "public void buildEvironment(ProcessBuilder pb) {\n\t\tpb.directory(cwd.toFile());\n\t\tif (AgentProfile.MCPC.equals(agentProfile)) {\n\t\t\tif (!tools.isEmpty()) {\n\t\t\t\tif (CacheMode.Disabled.equals(cacheMode))\n\t\t\t\t\tthrow new IllegalArgumentException(\"Cache is required for tool loops\");\n\t\t\t\tpb.environment().put(\"MCPC_TOOLS\", String.join(\",\", tools));\n\t\t\t} else\n\t\t\t\tpb.environment().put(\"MCPC_TOOLS\", \"None\");\n\t\t\tpb.environment().put(\"MCPC_CC_PROFILE\", cliProfile);\n\t\t\tpb.environment().put(\"CLAUDE_CONFIG_DIR\", System.getProperty(\"user.home\") + \"/.claude-\" + cliProfile);\n\n\t\t\tpb.environment().put(\"CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_AGENT_VIEW\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_BACKGROUND_TASKS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_BUNDLED_SKILLS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_CLAUDE_MDS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_CRON\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_POLICY_SKILLS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_WORKFLOWS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_ENABLE_AWAY_SUMMARY\", \"0\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_ENABLE_BACKGROUND_PLUGIN_REFRESH\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_FORK_SUBAGENT\", \"0\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY\", \"1\"); // number of parralel read tools\n\t\t\tpb.environment().put(\"ENABLE_TOOL_SEARCH\", \"false\");\n\t\t\tpb.environment().put(\"MAX_MCP_OUTPUT_TOKENS\", \"\" + (25000 * 10));\n\t\t}\n\n\t\tif (cacheMode != null)\n\t\t\tswitch (cacheMode) {\n\t\t\tcase Disabled:\n\t\t\t\tpb.environment().put(\"DISABLE_PROMPT_CACHING\", \"1\");\n\t\t\t\tbreak;\n\t\t\tcase Minutes_5:\n\t\t\t\tpb.environment().put(\"FORCE_PROMPT_CACHING_5M\", \"1\");\n\t\t\t\tbreak;\n\t\t\tcase Hours_1:\n\t\t\t\tpb.environment().put(\"ENABLE_PROMPT_CACHING_1H\", \"1\");\n\t\t\tcase Default:\n\t\t\tdefault:\n\t\t\t}\n\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_SPELLCHECK\", \"true\");\n\t\tpb.environment().put(\"CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT\", \"0\");\n\t\tpb.environment().put(\"MCP_TOOL_TIMEOUT\", \"86400000\");\n\t\tpb.environment().put(\"CLAUDE_ENABLE_STREAM_WATCHDOG\", \"0\");\n\t\tpb.environment().put(\"CLAUDE_ENABLE_BYTE_WATCHDOG\", \"0\");\n\t\tpb.environment().put(\"CLAUDE_STREAM_IDLE_TIMEOUT_MS\", \"86400000\");\n\t\tpb.environment().put(\"API_FORCE_IDLE_TIMEOUT\", \"0\");\n\t\tpb.environment().put(\"MCP_TIMEOUT\", \"86400000\");\n\n\t\tif (Reasoning.Disabled == reasoning) {\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING\", \"1\");\n\t\t\tpb.environment().put(\"MAX_THINKING_TOKENS\", \"0\");\n\t\t}\n\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_ADVISOR_TOOL\", \"1\");\n\t}"
      - id: ClaudeSessionParameters.getHash
        type: method_declaration
        code: "public String getHash() {\n\t\tif (hash == null)\n\t\t\thash = computeHash();\n\t\treturn hash;\n\t}"
      - id: ClaudeSessionParameters.computeHash
        type: method_declaration
        code: "private String computeHash() {\n\t\tString input = String.join(\",\", tools) + \"|\" + cwd.toString() + \"|\"\n\t\t\t\t+ model.apiName + \"|\" + reasoning.name() + \"|\" + (agentProfile != null ? agentProfile.name : \"\") + \"|\"\n\t\t\t\t+ cliProfile + \"|\" + (filePath != null ? filePath : \"\");\n\t\ttry {\n\t\t\tMessageDigest md = MessageDigest.getInstance(\"MD5\");\n\t\t\tbyte[] bytes = md.digest(input.getBytes(StandardCharsets.UTF_8));\n\t\t\tStringBuilder sb = new StringBuilder();\n\t\t\tfor (byte b : bytes)\n\t\t\t\tsb.append(String.format(\"%02x\", b));\n\t\t\treturn sb.substring(0, 8);\n\t\t} catch (NoSuchAlgorithmException e) {\n\t\t\t// Stable fallback (no external dependency)\n\t\t\tlong h = 0;\n\t\t\tfor (char c : input.toCharArray())\n\t\t\t\th = h * 31L + c;\n\t\t\treturn String.format(\"%08x\", h & 0xFFFFFFFFL);\n\t\t}\n\t}"
      - id: ClaudeSessionParameters.setTitle
        type: method_declaration
        code: "public void setTitle(String title) {\n\t\tif (this.title == null)\n\t\t\tthis.title = title;\n\t}"
      - id: ClaudeSessionParameters.getTitle
        type: method_declaration
        code: "public String getTitle() {\n\t\treturn title;\n\t}"
```
/answer 7fa5-3f allow
Control Request:
```yaml
id: 7fa5-40
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: ClaudeSessionParameters
    include:
    - '*.java'
    limit: 50
  reason: Alle Referenzen auf ClaudeSessionParameters finden für Validierung nach Refactoring.
```
/answer 7fa5-40 allow
Control Request:
```yaml
id: 7fa5-41
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      pattern: ClaudeSessionParameters
      matches:
      - path: xy/ai/workbench/view/ClaudeCodeSessionView.java
        lineno: 48
        match: import xy.ai.workbench.connector.claudecode.ClaudeSessionParameters;
      - path: xy/ai/workbench/view/ClaudeCodeSessionView.java
        lineno: 93
        match: "\t\t\tnew ClaudeSessionParameters(Path.of(\"\", \"\"), \"\", null, Model.NONE, Reasoning.Disabled, AgentProfile.basic, \"\","
      - path: xy/ai/workbench/view/ClaudeCodeSessionView.java
        lineno: 265
        match: "\tprivate ClaudeSessionParameters currentParameters() {"
      - path: xy/ai/workbench/view/ClaudeCodeSessionView.java
        lineno: 269
        match: "\t\t\treturn ClaudeSessionParameters.fromConfig(cfg, currentProjectPath, currentRelativeFilePath, \"\","
      - path: xy/ai/workbench/view/ClaudeCodeSessionView.java
        lineno: 277
        match: "\t\tClaudeSessionParameters current = currentParameters();"
      - path: xy/ai/workbench/view/ClaudeCodeSessionView.java
        lineno: 284
        match: "\t\t\tClaudeSessionParameters p = s.getParameters();"
      - path: xy/ai/workbench/view/SessionDetailDialog.java
        lineno: 14
        match: import xy.ai.workbench.connector.claudecode.ClaudeSessionParameters;
      - path: xy/ai/workbench/view/SessionDetailDialog.java
        lineno: 52
        match: "\t\tClaudeSessionParameters p = s.getParameters();"
      - path: xy/ai/workbench/connector/claudecode/CCSession.java
        lineno: 24
        match: "\tprivate final ClaudeSessionParameters parameters;"
      - path: xy/ai/workbench/connector/claudecode/CCSession.java
        lineno: 49
        match: "\tpublic CCSession(CCSessionManager manager, ClaudeSessionParameters parameters) {"
      - path: xy/ai/workbench/connector/claudecode/CCSession.java
        lineno: 53
        match: "\tpublic CCSession(String sessionUuid, CCSessionManager manager, ClaudeSessionParameters parameters) {"
      - path: xy/ai/workbench/connector/claudecode/CCSession.java
        lineno: 57
        match: "\tprivate CCSession(String sessionUuid, boolean resume, CCSessionManager manager, ClaudeSessionParameters parameters) {"
      - path: xy/ai/workbench/connector/claudecode/CCSession.java
        lineno: 316
        match: "\tpublic ClaudeSessionParameters getParameters() {"
      - path: xy/ai/workbench/connector/claudecode/CCConnector.java
        lineno: 95
        match: "\t\tClaudeSessionParameters params = ClaudeSessionParameters.fromConfig(cfg, loc.projectPath, loc.relativeFilePath,"
      - path: xy/ai/workbench/connector/claudecode/CCSessionManager.java
        lineno: 24
        match: "\tpublic synchronized CCSession requestSession(String selectedUuid, ClaudeSessionParameters params) {"
      - path: xy/ai/workbench/connector/claudecode/CCSessionManager.java
        lineno: 51
        match: "\tpublic synchronized CCSession getSession(String selectedUuid, ClaudeSessionParameters params) {"
      - path: xy/ai/workbench/connector/claudecode/CCSessionManager.java
        lineno: 69
        match: "\tpublic synchronized CCSession importSession(String uuid, ClaudeSessionParameters params) {"
      - path: xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
        lineno: 17
        match: "public class ClaudeSessionParameters {"
      - path: xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
        lineno: 35
        match: "\tpublic ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,"
      - path: xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
        lineno: 40
        match: "\tpublic ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,"
      - path: xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
        lineno: 62
        match: "\tpublic static ClaudeSessionParameters fromConfig(ConfigManager cfg, Path cwd, String filePath, String systemPrompt,"
      - path: xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
        lineno: 64
        match: "\t\treturn new ClaudeSessionParameters(cwd, systemPrompt, tools, cfg.getModel(), cfg.getReasoning(), cfg.getProfile(),"
```
/answer 7fa5-41 allow
ReasoningToken: 48
Text:
 Alle Referenzen bleiben kompatibel, da nur der Konstruktor/Factory/Getter genutzt werden – Vererbung genügt. Jetzt neue Basisklasse erstellen und `ClaudeSessionParameters` umbauen.
Control Request:
```yaml
id: 7fa5-42
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/SessionParameters.java
    source: "package xy.ai.workbench.connector;\n\nimport java.nio.charset.StandardCharsets;\nimport java.nio.file.Path;\nimport java.security.MessageDigest;\nimport java.security.NoSuchAlgorithmException;\nimport java.util.Collections;\nimport java.util.List;\n\nimport xy.ai.workbench.AgentProfile;\nimport xy.ai.workbench.CacheMode;\nimport xy.ai.workbench.Model;\nimport xy.ai.workbench.Reasoning;\n\n/** Generic, connector-agnostic session parameters incl. deterministic hash. */\npublic class SessionParameters {\n\n\t/** Deterministic hash of the session parameters. Immutable. */\n\tpublic final Path cwd;\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final Model model;\n\tpublic final Reasoning reasoning;\n\tpublic final AgentProfile agentProfile;\n\tpublic final String cliProfile;\n\tpublic final String filePath;\n\tpublic final CacheMode cacheMode;\n\n\t// Reserved for later use by FrozenConfig (not part of the hash).\n\tpublic final Double topP;\n\tpublic final Double temperature;\n\tpublic final Integer maxOutputTokens;\n\n\tprivate String hash;\n\n\tpublic SessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath) {\n\t\tthis(cwd, systemPrompt, tools, model, reasoning, agentProfile, cliProfile, cacheMode, filePath, null, null,\n\t\t\t\tnull);\n\t}\n\n\tpublic SessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath, Double topP,\n\t\t\tDouble temperature, Integer maxOutputTokens) {\n\t\tif (cwd == null)\n\t\t\tthrow new IllegalStateException(\"Work directory (cwd) not set\");\n\t\tif (model == null)\n\t\t\tthrow new IllegalArgumentException(\"Model must not be null\");\n\t\tif (model.apiName == null || model.apiName.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"Model apiName must not be null or blank\");\n\t\tif (reasoning == null)\n\t\t\tthrow new IllegalArgumentException(\"Reasoning must not be null\");\n\n\t\tthis.cwd = cwd;\n\t\tthis.systemPrompt = systemPrompt != null ? systemPrompt : \"\";\n\t\tthis.tools = tools != null ? tools : Collections.emptyList();\n\t\tthis.model = model;\n\t\tthis.reasoning = reasoning;\n\t\tthis.agentProfile = agentProfile;\n\t\tthis.cliProfile = cliProfile;\n\t\tthis.filePath = filePath;\n\t\tthis.cacheMode = cacheMode;\n\t\tthis.topP = topP;\n\t\tthis.temperature = temperature;\n\t\tthis.maxOutputTokens = maxOutputTokens;\n\t}\n\n\tpublic String getFilePath() {\n\t\treturn filePath;\n\t}\n\n\tpublic String getHash() {\n\t\tif (hash == null)\n\t\t\thash = computeHash();\n\t\treturn hash;\n\t}\n\n\tprivate String computeHash() {\n\t\tString input = String.join(\",\", tools) + \"|\" + cwd.toString() + \"|\"\n\t\t\t\t+ model.apiName + \"|\" + reasoning.name() + \"|\" + (agentProfile != null ? agentProfile.name : \"\") + \"|\"\n\t\t\t\t+ cliProfile + \"|\" + (filePath != null ? filePath : \"\");\n\t\ttry {\n\t\t\tMessageDigest md = MessageDigest.getInstance(\"MD5\");\n\t\t\tbyte[] bytes = md.digest(input.getBytes(StandardCharsets.UTF_8));\n\t\t\tStringBuilder sb = new StringBuilder();\n\t\t\tfor (byte b : bytes)\n\t\t\t\tsb.append(String.format(\"%02x\", b));\n\t\t\treturn sb.substring(0, 8);\n\t\t} catch (NoSuchAlgorithmException e) {\n\t\t\t// Stable fallback (no external dependency)\n\t\t\tlong h = 0;\n\t\t\tfor (char c : input.toCharArray())\n\t\t\t\th = h * 31L + c;\n\t\t\treturn String.format(\"%08x\", h & 0xFFFFFFFFL);\n\t\t}\n\t}\n}\n"
  reason: Neue generische Basisklasse SessionParameters gemäß Step 02 erstellen.
```
/answer 7fa5-42 allow
Control Request:
```yaml
id: 7fa5-43
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: UIM5xW
    source: |-
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
  reason: "Imports anpassen: MD5/StandardCharsets/Collections nicht mehr benötigt, SessionParameters importieren."
```
/answer 7fa5-43 allow
Control Request:
```yaml
id: 7fa5-44
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: ClaudeSessionParameters
    old_text: "public class ClaudeSessionParameters {"
    new_text: "public class ClaudeSessionParameters extends SessionParameters {"
  reason: Vererbung von SessionParameters einführen.
```
/answer 7fa5-44 allow
Control Request:
```yaml
id: 7fa5-45
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: ClaudeSessionParameters.igRtKw
    source: "private static final String SCRIPT = System.getProperty(\"user.home\")\n\t\t\t+ \"/xyan/xy.ai.workbench/claude-code/claude-session.sh\";\n\tprivate static final String COMMAND = \"claude\";"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: ClaudeSessionParameters.ZbqmG7
    source: private String title;
  reason: "Generische Felder entfernen, da sie nun in SessionParameters liegen."
```
/answer 7fa5-45 allow
Control Request:
```yaml
id: 7fa5-46
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
      result: success
      id: ClaudeSessionParameters.igRtKw
      new_id: ClaudeSessionParameters.k3wWKd
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
      error: No node matched the selector.
      id: ClaudeSessionParameters.ZbqmG7
```
/answer 7fa5-46 allow
Control Request:
```yaml
id: 7fa5-47
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
  reason: Aktuelle Node-IDs nach vorheriger Änderung ermitteln.
```
/answer 7fa5-47 allow
Control Request:
```yaml
id: 7fa5-48
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
      nodes:
      - id: ELFb2q
        type: imports
      - id: ClaudeSessionParameters
        type: class_declaration
        signature: "public class ClaudeSessionParameters extends SessionParameters {"
        children:
        - id: ClaudeSessionParameters.k3wWKd
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
        - id: ClaudeSessionParameters.getFilePath
          type: method_declaration
          signature: "public String getFilePath() {"
        - id: ClaudeSessionParameters.buildBaseCommand
          type: method_declaration
          signature: "public List<String> buildBaseCommand() {"
        - id: ClaudeSessionParameters.buildEvironment
          type: method_declaration
          signature: "public void buildEvironment(ProcessBuilder pb) {"
        - id: ClaudeSessionParameters.getHash
          type: method_declaration
          signature: "public String getHash() {"
        - id: ClaudeSessionParameters.computeHash
          type: method_declaration
          signature: "private String computeHash() {"
        - id: ClaudeSessionParameters.setTitle
          type: method_declaration
          signature: "public void setTitle(String title) {"
        - id: ClaudeSessionParameters.getTitle
          type: method_declaration
          signature: "public String getTitle() {"
```
/answer 7fa5-48 allow
Control Request:
```yaml
id: 7fa5-49
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: ClaudeSessionParameters.k3wWKd
    position: after
    source: private String title;
  reason: "title-Feld wieder hinzufügen, das CC-spezifisch bleibt."
```
/answer 7fa5-49 allow
Control Request:
```yaml
id: 7fa5-4a
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: ClaudeSessionParameters.getFilePath
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: ClaudeSessionParameters.getHash
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: ClaudeSessionParameters.computeHash
  reason: "getFilePath/getHash/computeHash entfernen, da nun in SessionParameters vererbt."
```
/answer 7fa5-4a allow
Control Request:
```yaml
id: 7fa5-4b
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: ClaudeSessionParameters.ClaudeSessionParameters
    source: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode) {\n\t\tthis(cwd, systemPrompt, tools, model, reasoning, agentProfile, cliProfile, cacheMode, null);\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    id: ClaudeSessionParameters.ClaudeSessionParameters_1
    source: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath) {\n\t\tsuper(cwd, systemPrompt, tools, model, reasoning, agentProfile, cliProfile, cacheMode, filePath);\n\t}"
  reason: "Konstruktoren auf Basisklassenaufruf umstellen, Validierung/Feldzuweisung entfällt lokal."
```
/answer 7fa5-4b allow
Control Request:
```yaml
id: 7fa5-4c
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/SessionParameters.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSessionManager.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/SessionDetailDialog.java
  reason: Alle betroffenen und referenzierenden Dateien nach Refactoring validieren.
```
/answer 7fa5-4c allow
ReasoningToken: 160
Text:
 Jetzt die Klassensignatur und Felder anpassen:
ReasoningToken: 34
Text:
 The ZbqmG7 statements node got merged into k3wWKd since old fields were removed. Now let's add `private String title;` and delete getFilePath/getHash/computeHash, update constructors and fromConfig.
Text:
 Jetzt Konstruktoren auf `super(...)` umstellen:
Text:
 Nun validieren:
Result Stats:  id=7029cc3a-91cb-4a2d-9f4b-09790cf765e7, total: 0, in: 159, out: 6808, reason: 0, read: 232989, write: 21291
Alle Dateien validieren erfolgreich (ast_validate = syntaktische Prüfung, kein vollständiger Compile). Refactoring abgeschlossen:

- Neu: `connector/SessionParameters.java` mit generischen Feldern, Validierung, `getHash()`/`computeHash()` (unveränderte Hash-Logik) und `getFilePath()`.
- `ClaudeSessionParameters extends SessionParameters`, behält nur `SCRIPT`, `COMMAND`, `title`, `setTitle`/`getTitle`, `buildBaseCommand`, `buildEvironment`, `fromConfig`; Konstruktoren rufen `super(...)`.
