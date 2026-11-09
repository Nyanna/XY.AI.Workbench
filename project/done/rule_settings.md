Ich möchte das die Settings von Regeln zur Laufzeit und pro Prompt konfiguriert werden können.

Hierfür ist es notwendig, dass die Regeln unter `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules` das angepasste Interface `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java` implementieren und ihre Default Settings exponieren (Default werden per Clone exponiert).

- Settings werden per ID Regeln in der Registry zugeordnet

Settings ist ein Containerobjekt `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java`, das eine Map typisierter Einstellung liefert (Setting Name zu Container Value). Die Values verwenden Container damit eine leichteres Rendering einem dynamischen Setting Editor möglich ist.

Die Filterkonfiguration wird über der `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java` und die `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java` mit in den Prompt an die Rules gegeben.

- Filtersettings werden ebenfalls persistiert sowie über `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java` gehandhabt (ebenfalls Front Matter Support).

Zwischen Tools Tab und Presets Tab `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java`, soll ein weiterer "Rules" Tab angelegt werden.
Dieser Tab besteht ähnlich `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java` aus zwei, per Sash getrennter Bereiche.
Der obere Bereich Analog der Presets List, wird eine Liste mit den Namen der Filter angezeigt (alphabetisch sortiert).
Der untere Bereich wird ein dynamisch generierter Settings Editor.
Beim Click auf eine Regel werden deren aktuelle Settings angezeigt. Initial werden die Defaults angezeigt. Auf diese Art können Regel Settings pro Prompt erweitert werden.
Ein "Clear All" auf dem Presets Tab setzt auch die Regel Setting auf ihre Defaults zurück.

- Regel Setting werden nur dem Prompt übergeben, wenn sie vom Default abweichen (Copy-on-write Ansatz)
- Die Settings enthalten auch ein "activ" Flag um eine Regel zu deaktivieren (Default aktiviert)
- Aufeinander abhängige Regeln verweisen nun auf die Settings oder die Default Settings wenn Abwesend
SystemInit:  id=7f9f0cdd-4630-492a-aae4-84f897630aab, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich verschaffe mir zuerst einen Überblick über die betroffenen Dateien.
Control Request:
```yaml
id: 0a99-1-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
  reason: Struktur der Autoapprove-Regeln und der Session-View-Klassen erfassen
```
/answer 0a99-1-0 allow
Control Request:
```yaml
id: 0a99-2-1
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      entries:
      - ".:"
      - AnswerParser.java
      - ApprovalContext.java
      - ApprovalItem.java
      - ApprovalResult.java
      - AutoApprover.java
      - Decision.java
      - FollowUp.java
      - RuleRegistry.java
      - RuleVote.java
      - SessionSnapshot.java
      - SessionTracker.java
      - "./rules:"
      - AllowedToolsRule.java
      - CANDIDATES.md
      - DeepPathReadRule.java
      - DenyRootPathRule.java
      - HoldMarkdownRule.java
      - IApprovalRule.java
      - PathArguments.java
      - RuleSettings.java
      - SmallResultRule.java
      - Verdict.java
      - Vote.java
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
      entries:
      - ".:"
      - ActionButtonsSection.java
      - Clearable.java
      - EditTab.java
      - InputSelection.java
      - InputsTable.java
      - InstructionSection.java
      - ModelParametersSection.java
      - OutputModeSection.java
      - PresetList.java
      - PresetsTab.java
      - SelectTab.java
      - ToolSelection.java
      - ToolsTab.java
      - UpdateGuard.java
      - UsageLogSection.java
```
/answer 0a99-2-1 allow
ReasoningToken: 13
Control Request:
```yaml
id: 0a99-3-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
  reason: "Interface, ConfigManager und FrozenConfig-Struktur lesen"
```
/answer 0a99-3-0 allow
Control Request:
```yaml
id: 0a99-4-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
      nodes:
      - id: H4HWeP|XqIL50
        signature: import xy.ai.conscious.autoapprove.ApprovalContext;
      - id: IApprovalRule
        signature: /**  * A voter. Every registered rule is evaluated for every answer and decides…
        docstring: A voter. Every registered rule is evaluated for every answer and decides on
        children:
        - id: IApprovalRule.id
          signature: String id()
        - id: IApprovalRule.vote
          signature: abstract Vote vote(ApprovalContext ctx)
        - id: IApprovalRule.getDefaultSetting
          signature: abstract RuleSettings getDefaultSetting()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
      nodes:
      - id: Ei2OXR|xqS3r3
        signature: import java.util.ArrayList;
      - id: ConfigManager
        signature: "public class ConfigManager {"
        children:
        - id: ConfigManager.0mEcEZ|9PgBPn
          signature: private ModelResolverRegistry resolverRegistry;
        - id: ConfigManager.iAEUAf|GboLsd
          signature: "private List<Consumer<AgentProfile[]>> enabledProfilesObs = new ArrayList<>();"
        - id: ConfigManager.ConfigManager
          signature: "public ConfigManager(MCPClient mcpClient, IFileCache cache)"
        - id: ConfigManager.ensureOwnConfig
          signature: protected void ensureOwnConfig()
          docstring: hook
        - id: ConfigManager.clearObserver
          signature: public void clearObserver()
        - id: ConfigManager.applyConfig
          signature: public void applyConfig(SessionConfig target)
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
        - id: ConfigManager.getCapabilities
          signature: public Capabilities getCapabilities()
        - id: ConfigManager.getEnabledTools
          signature: "public String[] getEnabledTools()"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
      nodes:
      - id: 0gUTBT|NhIzDR
        signature: import java.util.Arrays;
      - id: FrozenConfig
        signature: "/**  * Immutable snapshot of the {"
        docstring: "Immutable snapshot of the {@link ConfigManager} state, taken once when a"
        children:
        - id: FrozenConfig.7vEs3l|JgA7Ah
          signature: public Model model;
        - id: FrozenConfig.FrozenConfig
          signature: "private FrozenConfig(String keys, Model model, AgentProfile profile, Reasoning …"
        - id: FrozenConfig.copy
          signature: public FrozenConfig copy()
          docstring: "Independent copy; {@code model} and {@code keys} are mutated by prompt preproce…"
        - id: FrozenConfig.from
          signature: public static FrozenConfig from(ConfigManager cfg)
          docstring: "Reads all relevant values from {@code cfg} once and renders the system"
        - id: FrozenConfig.getHash
          signature: public String getHash()
          docstring: "Deterministic hash over exactly: model, profile, reasoning, tools,"
        - id: FrozenConfig.computeHash
          signature: private String computeHash()
```
/answer 0a99-4-3 allow
ReasoningToken: 40
Control Request:
```yaml
id: 0a99-5-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
    ids:
    - IApprovalRule
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
  reason: "Aktuellen Stand von Interface, RuleSettings, Registry, Context und FrozenConfig vollständig lesen"
```
/answer 0a99-5-0 allow
Control Request:
```yaml
id: 0a99-6-5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
      nodes:
      - id: IApprovalRule
        type: ClassOrInterfaceDeclaration
        signature: /**  * A voter. Every registered rule is evaluated for every answer and decides…
        docstring: A voter. Every registered rule is evaluated for every answer and decides on
        children:
        - id: IApprovalRule.id
          type: MethodDeclaration
          code: "\tdefault String id() {\n\t\treturn getClass().getSimpleName();\n\t}\n"
        - id: IApprovalRule.vote
          type: MethodDeclaration
          code: "\tVote vote(ApprovalContext ctx);\n"
        - id: IApprovalRule.getDefaultSetting
          type: MethodDeclaration
          code: "\tRuleSettings getDefaultSetting();\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
      nodes:
      - id: bG4Cjo|du6ALf
        signature: import java.util.List;
      - id: RuleSettings
        signature: "public class RuleSettings {"
        children:
        - id: RuleSettings.fNjH9x|g6XBMR
          signature: "private Map<String, ? extends Setting> settings;"
        - id: RuleSettings.Setting
          signature: "public static abstract class Setting {"
          children:
          - id: RuleSettings.Setting.getName
            signature: public abstract String getName()
        - id: RuleSettings.StringListSetting
          signature: "public static class StringListSetting extends Setting {"
          children:
          - id: RuleSettings.StringListSetting.kfROuh|Kxz7kb
            signature: private List<String> stringList;
          - id: RuleSettings.StringListSetting.getName
            signature: public String getName()
        - id: RuleSettings.NumberSetting
          signature: "public static class NumberSetting extends Setting {"
          children:
          - id: RuleSettings.NumberSetting.PJcXCX|56OIZK
            signature: private long value = 0;
          - id: RuleSettings.NumberSetting.getName
            signature: public String getName()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: 3sbhBT|4uo6iX
        signature: import java.util.List;
      - id: RuleRegistry
        signature: "/**  * Ordered registry of rules.  */ public class RuleRegistry {"
        docstring: Ordered registry of rules.
        children:
        - id: RuleRegistry.VUx1sY|HlFjwk
          signature: private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayL…
        - id: RuleRegistry.RuleRegistry
          signature: public RuleRegistry()
        - id: RuleRegistry.RuleRegistry_2
          signature: public RuleRegistry(List<IApprovalRule> rules)
        - id: RuleRegistry.register
          signature: public void register(IApprovalRule rule)
        - id: RuleRegistry.unregister
          signature: public boolean unregister(IApprovalRule rule)
        - id: RuleRegistry.rules
          signature: public List<IApprovalRule> rules()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      nodes:
      - id: GwlVvM|lj1u2H
        signature: import xy.ai.conscious.connector.harness.Prompt;
      - id: ApprovalContext
        signature: "/**  * Input of a rule.  *  * @param item {"
        docstring: Input of a rule.
        children:
        - id: ApprovalContext.hasItem
          signature: public boolean hasItem()
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
      nodes:
      - id: 0gUTBT|NhIzDR
        signature: import java.util.Arrays;
      - id: FrozenConfig
        signature: "/**  * Immutable snapshot of the {"
        docstring: "Immutable snapshot of the {@link ConfigManager} state, taken once when a"
        children:
        - id: FrozenConfig.7vEs3l|JgA7Ah
          signature: public Model model;
        - id: FrozenConfig.FrozenConfig
          signature: "private FrozenConfig(String keys, Model model, AgentProfile profile, Reasoning …"
        - id: FrozenConfig.copy
          signature: public FrozenConfig copy()
          docstring: "Independent copy; {@code model} and {@code keys} are mutated by prompt preproce…"
        - id: FrozenConfig.from
          signature: public static FrozenConfig from(ConfigManager cfg)
          docstring: "Reads all relevant values from {@code cfg} once and renders the system"
        - id: FrozenConfig.getHash
          signature: public String getHash()
          docstring: "Deterministic hash over exactly: model, profile, reasoning, tools,"
        - id: FrozenConfig.computeHash
          signature: private String computeHash()
```
/answer 0a99-6-5 allow
ReasoningToken: 150
Control Request:
```yaml
id: 0a99-7-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
  node_type: ^(ClassOrInterfaceDeclaration|RecordDeclaration)$
  reason: Vollständigen Quelltext der Top-Level-Typen holen
```
/answer 0a99-7-0 allow
Control Request:
```yaml
id: 0a99-8-7
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
      nodes:
      - id: RuleSettings
        type: ClassOrInterfaceDeclaration
        signature: "public class RuleSettings {"
        children:
        - id: RuleSettings.Setting
          type: ClassOrInterfaceDeclaration
          code: "\tpublic static abstract class Setting {\n\t\tpublic abstract String getName();\n\t}\n"
        - id: RuleSettings.StringListSetting
          type: ClassOrInterfaceDeclaration
          code: "\tpublic static class StringListSetting extends Setting {\n\t\tprivate List<String> stringList;\n\n\t\t@Override\n\t\tpublic String getName() {\n\t\t\t// TODO Auto-generated method stub\n\t\t\treturn null;\n\t\t}\n\t}\n"
        - id: RuleSettings.NumberSetting
          type: ClassOrInterfaceDeclaration
          code: "\tpublic static class NumberSetting extends Setting {\n\t\tprivate long value = 0;\n\n\t\t@Override\n\t\tpublic String getName() {\n\t\t\t// TODO Auto-generated method stub\n\t\t\treturn null;\n\t\t}\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes:
      - id: RuleRegistry
        type: ClassOrInterfaceDeclaration
        code: |
          public class RuleRegistry {
            private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayList<>();

            public RuleRegistry() {
              register(new AllowedToolsRule());
              register(new SmallResultRule());
              register(new DeepPathReadRule());
              register(new DenyRootPathRule());
              register(new HoldMarkdownRule());
            }

            public RuleRegistry(List<IApprovalRule> rules) {
              this.rules.addAll(rules);
            }

            public void register(IApprovalRule rule) {
              rules.addIfAbsent(Objects.requireNonNull(rule));
            }

            public boolean unregister(IApprovalRule rule) {
              return rules.remove(rule);
            }

            public List<IApprovalRule> rules() {
              return rules;
            }
          }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      nodes:
      - id: ApprovalContext
        type: RecordDeclaration
        code: "public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call,\n\t\tSessionSnapshot session) {\n\n\tpublic boolean hasItem() {\n\t\treturn item != null;\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
      nodes:
      - id: FrozenConfig
        type: ClassOrInterfaceDeclaration
        code: |
          public class FrozenConfig {
            public Model model;
            public String keys;
            public final AgentProfile profile;
            public final Reasoning reasoning;
            public final List<String> tools;
            public final String systemPrompt;
            public final Double topP;
            public final Double temperature;
            public final Long maxOutputTokens;
            public final Integer reasoningBudget;
            public final CacheMode cacheMode;
            public final OutputMode outputMode;
            private String hash;

            private FrozenConfig(String keys, Model model, AgentProfile profile, Reasoning reasoning, List<String> tools, String systemPrompt, Double topP, Double temperature, Long maxOutputTokens, Integer reasoningBudget, CacheMode cacheMode, OutputMode outputMode) {
              this.keys = keys;
              this.model = model;
              this.profile = profile;
              this.reasoning = reasoning;
              this.tools = tools;
              this.systemPrompt = systemPrompt;
              this.topP = topP;
              this.temperature = temperature;
              this.maxOutputTokens = maxOutputTokens;
              this.reasoningBudget = reasoningBudget;
              this.cacheMode = cacheMode;
              this.outputMode = outputMode;
            }

            /**
             * Independent copy; {@code model} and {@code keys} are mutated by prompt preprocessing.
             */
            public FrozenConfig copy() {
              return new FrozenConfig(keys, model, profile, reasoning, tools, systemPrompt, topP, temperature, maxOutputTokens, reasoningBudget, cacheMode, outputMode);
            }

            /**
             * Reads all relevant values from {@code cfg} once and renders the system
             * prompt.
             */
            public static FrozenConfig from(ConfigManager cfg) {
              String systemPrompt = null;
              if (cfg.isInputEnabled(InputMode.SystemPrompt)) {
                String[] rawPrompt = cfg.getSystemPrompt();
                StringBuilder sb = new StringBuilder();
                Arrays.stream(rawPrompt != null ? rawPrompt : new String[0]).filter(e -> !e.startsWith("#")).forEach(e -> sb.append("- ").append(e).append("\n"));
                String freeText = cfg.getFreeText();
                if (freeText != null && !freeText.isBlank())
                  sb.append("\n- ").append(freeText).append("\n");
                systemPrompt = sb.toString();
              }
              String[] rawTools = cfg.isInputEnabled(InputMode.Tools) ? cfg.getEnabledTools() : null;
              List<String> tools = List.of(rawTools != null ? rawTools : new String[0]);
              String keys = cfg.getKeys();
              for (String key : keys.split(",")) if (cfg.getModel().cap.acceptsKey(key))
                keys = key;
              return new FrozenConfig(keys, cfg.getModel(), cfg.getProfile(), cfg.getReasoning(), tools, systemPrompt, cfg.getTopP(), cfg.getTemperature(), cfg.getMaxOutputTokens(), cfg.getReasoningBudget(), cfg.getCacheMode(), cfg.getOuputMode());
            }

            /**
             * Deterministic hash over exactly: model, profile, reasoning, tools,
             * systemPrompt, topP, temperature, maxOutputTokens, reasoningBudget.
             */
            public String getHash() {
              if (hash == null)
                hash = computeHash();
              return hash;
            }

            private String computeHash() {
              String input = (model != null ? model.apiName : "") + "|" + (profile != null ? profile.name : "") + "|" + (reasoning != null ? reasoning.name() : "") + "|" + String.join(",", tools) + "|" + systemPrompt + "|" + topP + "|" + temperature + "|" + maxOutputTokens + "|" + reasoningBudget;
              return Hash.hash(input);
            }
          }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
      nodes:
      - id: IApprovalRule
        type: ClassOrInterfaceDeclaration
        code: "public interface IApprovalRule {\n\n\tdefault String id() {\n\t\treturn getClass().getSimpleName();\n\t}\n\n\tVote vote(ApprovalContext ctx);\n\n\tRuleSettings getDefaultSetting();\n}\n"
```
/answer 0a99-8-7 allow
Control Request:
```yaml
id: 0a99-9-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/PathArguments.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  node_type: ^(ClassOrInterfaceDeclaration|RecordDeclaration|EnumDeclaration)$
  reason: "Bestehende Regeln und AutoApprover lesen, um Settings-Anbindung zu planen"
```
/answer 0a99-9-0 allow
Control Request:
```yaml
id: 0a99-a-9
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
      nodes:
      - id: AllowedToolsRule
        type: ClassOrInterfaceDeclaration
        code: "public class AllowedToolsRule implements IApprovalRule {\n\n\tpublic static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\", \"ast_validate\", \"run_command\", \"ast_find\" };\n\n\tprivate final Set<String> allowed;\n\n\tpublic AllowedToolsRule() {\n\t\tthis(ALLOWED_TOOLS);\n\t}\n\n\tpublic AllowedToolsRule(String... toolNames) {\n\t\tthis.allowed = Set.of(toolNames);\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\treturn allowed.contains(item.tool()) ? Vote.allow() : Vote.abstain();\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
      nodes:
      - id: DeepPathReadRule
        type: ClassOrInterfaceDeclaration
        code: "public class DeepPathReadRule implements IApprovalRule {\n\n\tpublic static final String[] TOOLS = { \"search\", \"list\" };\n\tpublic static final String[] PATH_FIELDS = { \"path\", \"directory\" };\n\tpublic static final int MIN_DEPTH = 3;\n\tpublic static final int MAX_LENGTH = 3000;\n\n\tprivate final Set<String> tools;\n\tprivate final Set<String> pathFields;\n\tprivate final int minDepth;\n\tprivate final Path cwd;\n\n\tpublic DeepPathReadRule() {\n\t\tthis(Set.of(TOOLS), Set.of(PATH_FIELDS), MIN_DEPTH, Path.of(\"\").toAbsolutePath().normalize());\n\t}\n\n\tpublic DeepPathReadRule(Set<String> tools, Set<String> pathFields, int minDepth, Path cwd) {\n\t\tthis.tools = Set.copyOf(tools);\n\t\tthis.pathFields = Set.copyOf(pathFields);\n\t\tthis.minDepth = minDepth;\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null)\n\t\t\treturn Vote.abstain();\n\t\tswitch (item.kind()) {\n\t\tcase ControlCall, ToolCall:\n\t\t\treturn item.tool() != null && tools.contains(item.tool()) && allDeep(item) ? Vote.allow() : Vote.abstain();\n\t\tcase ControlResult, ToolResult:\n\t\t\tApprovalItem call = ctx.call();\n\t\t\tif (call == null || call.tool() == null || !tools.contains(call.tool()) || !allDeep(call))\n\t\t\t\treturn Vote.abstain();\n\t\t\treturn length(item) < MAX_LENGTH ? Vote.allow() : Vote.abstain();\n\t\tdefault:\n\t\t\treturn Vote.abstain();\n\t\t}\n\t}\n\n\tprivate boolean allDeep(ApprovalItem call) {\n\t\tList<String> values = PathArguments.collect(call.arguments(), pathFields);\n\t\tif (values.isEmpty())\n\t\t\treturn false;\n\t\tfor (String value : values) {\n\t\t\tPath path = PathArguments.resolve(cwd, value);\n\t\t\tif (path == null || !path.startsWith(cwd) || cwd.relativize(path).getNameCount() < minDepth)\n\t\t\t\treturn false;\n\t\t}\n\t\treturn true;\n\t}\n\n\tprivate static int length(ApprovalItem item) {\n\t\tJsonNode result = item.result();\n\t\tif (result.isMissingNode())\n\t\t\treturn item.yaml().length();\n\t\treturn result.isTextual() ? result.asText().length() : result.toString().length();\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
      nodes:
      - id: DenyRootPathRule
        type: ClassOrInterfaceDeclaration
        code: "public class DenyRootPathRule implements IApprovalRule {\n\n\tpublic static final String REASON = \"Don't use the root directory.\";\n\n\tprivate final Set<String> tools;\n\tprivate final Set<String> pathFields;\n\tprivate final Path cwd;\n\n\tpublic DenyRootPathRule() {\n\t\tthis(Set.of(DeepPathReadRule.TOOLS), Set.of(DeepPathReadRule.PATH_FIELDS),\n\t\t\t\tPath.of(\"\").toAbsolutePath().normalize());\n\t}\n\n\tpublic DenyRootPathRule(Set<String> tools, Set<String> pathFields, Path cwd) {\n\t\tthis.tools = Set.copyOf(tools);\n\t\tthis.pathFields = Set.copyOf(pathFields);\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null || !tools.contains(item.tool()))\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tfor (String value : PathArguments.collect(item.arguments(), pathFields)) {\n\t\t\tPath path = PathArguments.resolve(cwd, value);\n\t\t\tif (path != null && PathArguments.isRoot(path))\n\t\t\t\treturn Vote.deny(REASON);\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
      nodes:
      - id: HoldMarkdownRule
        type: ClassOrInterfaceDeclaration
        code: "public class HoldMarkdownRule implements IApprovalRule {\n\n\tpublic static final String[] PATH_FIELDS = { \"path\", \"paths\" };\n\tpublic static final String[] PATTERN_FIELDS = { \"include\" };\n\n\tprivate static final String EXTENSION = \".md\";\n\n\tprivate final Set<String> tools;\n\tprivate final Set<String> pathFields;\n\tprivate final Set<String> patternFields;\n\n\tpublic HoldMarkdownRule() {\n\t\tthis(Set.of(AllowedToolsRule.ALLOWED_TOOLS), Set.of(PATH_FIELDS), Set.of(PATTERN_FIELDS));\n\t}\n\n\tpublic HoldMarkdownRule(Set<String> tools, Set<String> pathFields, Set<String> patternFields) {\n\t\tthis.tools = Set.copyOf(tools);\n\t\tthis.pathFields = Set.copyOf(pathFields);\n\t\tthis.patternFields = Set.copyOf(patternFields);\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null || !tools.contains(item.tool()))\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tfor (String value : PathArguments.collect(item.arguments(), pathFields)) {\n\t\t\tif (value.toLowerCase(Locale.ROOT).endsWith(EXTENSION))\n\t\t\t\treturn Vote.hold();\n\t\t}\n\t\tfor (String value : PathArguments.collect(item.arguments(), patternFields)) {\n\t\t\tif (value.toLowerCase(Locale.ROOT).contains(EXTENSION))\n\t\t\t\treturn Vote.hold();\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
      nodes:
      - id: SmallResultRule
        type: ClassOrInterfaceDeclaration
        code: "public class SmallResultRule implements IApprovalRule {\n\n\tpublic static final int MAX_LENGTH = 3000;\n\n\tprivate final Set<String> allowed;\n\n\tpublic SmallResultRule() {\n\t\tthis(AllowedToolsRule.ALLOWED_TOOLS);\n\t}\n\n\tpublic SmallResultRule(String... toolNames) {\n\t\tthis.allowed = Set.of(toolNames);\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tApprovalItem call = ctx.call();\n\t\tif (item == null || call == null || call.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlResult && item.kind() != Kind.ToolResult)\n\t\t\treturn Vote.abstain();\n\t\tif (!allowed.contains(call.tool()))\n\t\t\treturn Vote.abstain();\n\t\treturn length(item) < MAX_LENGTH ? Vote.allow() : Vote.abstain();\n\t}\n\n\tprivate static int length(ApprovalItem item) {\n\t\tJsonNode result = item.result();\n\t\tif (result.isMissingNode())\n\t\t\treturn item.yaml().length();\n\t\treturn result.isTextual() ? result.asText().length() : result.toString().length();\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/PathArguments.java
      nodes:
      - id: PathArguments
        type: ClassOrInterfaceDeclaration
        code: "public class PathArguments {\n\n\tprivate PathArguments() {\n\t}\n\n\t/** Collects all string values (or string array elements) of fields named like {@code fields}, at any depth. */\n\tstatic List<String> collect(JsonNode node, Set<String> fields) {\n\t\tList<String> out = new ArrayList<>();\n\t\tcollect(node, fields, out);\n\t\treturn out;\n\t}\n\n\t@SuppressWarnings(\"deprecation\")\n\tprivate static void collect(JsonNode node, Set<String> fields, List<String> out) {\n\t\tif (node == null)\n\t\t\treturn;\n\t\tif (node.isObject()) {\n\t\t\tnode.fields().forEachRemaining(e -> {\n\t\t\t\tif (fields.contains(e.getKey()))\n\t\t\t\t\taddValues(e.getValue(), out);\n\t\t\t\tcollect(e.getValue(), fields, out);\n\t\t\t});\n\t\t} else if (node.isArray()) {\n\t\t\tnode.forEach(child -> collect(child, fields, out));\n\t\t}\n\t}\n\n\tprivate static void addValues(JsonNode value, List<String> out) {\n\t\tif (value.isTextual())\n\t\t\tout.add(value.asText());\n\t\telse if (value.isArray())\n\t\t\tvalue.forEach(v -> addValues(v, out));\n\t}\n\n\t/** Resolves against {@code cwd} and normalizes; null if the value is not a valid path. */\n\tstatic Path resolve(Path cwd, String value) {\n\t\ttry {\n\t\t\treturn cwd.resolve(value).normalize();\n\t\t} catch (InvalidPathException e) {\n\t\t\treturn null;\n\t\t}\n\t}\n\n\tstatic boolean isRoot(Path path) {\n\t\treturn path.isAbsolute() && path.getNameCount() == 0;\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
      nodes:
      - id: AutoApprover
        type: ClassOrInterfaceDeclaration
        code: |
          public class AutoApprover {
            public static final int DEFAULT_MAX_CHAIN = 20;
            private final RuleRegistry registry;
            private final SessionTracker sessions = new SessionTracker();
            private final List<Consumer<Decision>> listeners = new CopyOnWriteArrayList<>();
            private volatile int maxChain = DEFAULT_MAX_CHAIN;
            private volatile String idleInstruction = "Continue with the task.";
            private volatile boolean enabled = true;

            public AutoApprover() {
              this(new RuleRegistry());
            }

            public AutoApprover(RuleRegistry registry) {
              this.registry = registry;
            }

            public RuleRegistry rules() {
              return registry;
            }

            public void addListener(Consumer<Decision> listener) {
              listeners.add(listener);
            }

            public void removeListener(Consumer<Decision> listener) {
              listeners.remove(listener);
            }

            public boolean isEnabled() {
              return enabled;
            }

            /**
             * Globally switches the processing on or off; while off, answers pass untouched.
             */
            public void setEnabled(boolean enabled) {
              this.enabled = enabled;
            }

            /**
             * Maximum number of consecutive automatic follow-up prompts.
             */
            public void setMaxChain(int maxChain) {
              this.maxChain = maxChain;
            }

            public void setIdleInstruction(String instruction) {
              this.idleInstruction = instruction == null ? "" : instruction;
            }

            public void forget(String sessionId) {
              sessions.forget(sessionId);
            }

            public Decision evaluate(AIAnswer ans, int depth) {
              if (!enabled)
                return Decision.pass(ans.sessionId);
              SessionSnapshot session = sessions.record(ans);
              List<ApprovalItem> items = AnswerParser.parse(ans.answer);
              ApprovalItem item = items.size() == 1 ? items.get(0) : null;
              sessions.track(session.id(), items);
              ApprovalItem call = sessions.callFor(session.id(), item);
              ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, call, session);
              Decision decision = decide(ctx, collect(ctx), depth);
              enrich(ans, decision.warnings());
              publish(decision);
              return decision;
            }

            public Optional<FollowUp> onIdle(String sessionId) {
              if (!enabled)
                return Optional.empty();
              Prompt origin = sessions.lastPrompt(sessionId);
              String instruction = idleInstruction;
              if (origin == null || instruction.isBlank())
                return Optional.empty();
              FollowUp followUp = new FollowUp(origin, selection(instruction.lines().toList()));
              publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp, 0));
              return Optional.of(followUp);
            }

            private List<RuleVote> collect(ApprovalContext ctx) {
              List<RuleVote> votes = new ArrayList<>();
              for (IApprovalRule rule : registry.rules()) {
                try {
                  Vote vote = rule.vote(ctx);
                  votes.add(new RuleVote(rule.id(), vote == null ? Vote.abstain() : vote));
                } catch (RuntimeException e) {
                  LOG.error("Approval rule failed: " + rule.id(), e);
                }
              }
              return votes;
            }

            private Decision decide(ApprovalContext ctx, List<RuleVote> votes, int depth) {
              List<String> reasons = new ArrayList<>();
              List<String> hints = new ArrayList<>();
              List<String> warnings = new ArrayList<>();
              List<String> instructions = new ArrayList<>();
              boolean deny = false;
              boolean hold = false;
              boolean allow = false;
              for (RuleVote rv : votes) {
                Vote v = rv.vote();
                warnings.addAll(v.warnings());
                addNonBlank(instructions, v.instruction());
                switch(v.verdict()) {
                  case Deny ->
                    {
                      deny = true;
                      addNonBlank(reasons, v.reason());
                    }
                  case Hold ->
                    {
                      hold = true;
                    }
                  case Allow ->
                    {
                      allow = true;
                      addNonBlank(hints, v.hint());
                    }
                  case Abstain ->
                    {
                    }
                }
              }
              ApprovalResult result = resultFor(ctx.item(), deny, hold, allow);
              String reason = String.join("; ", reasons);
              String hint = String.join("; ", hints);
              String instruction = String.join("\n", instructions);
              FollowUp followUp = null;
              if (depth >= maxChain) {
                if (result != ApprovalResult.Pass || !instruction.isEmpty())
                  warnings.add("auto-approve chain limit reached (" + maxChain + ")");
                result = ApprovalResult.Pass;
              } else {
                followUp = followUpFor(ctx, result, reason, hint, instruction);
                if (followUp == null)
                  result = ApprovalResult.Pass;
              }
              return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());
            }

            private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolean hold, boolean allow) {
              if (item == null || item.id().isBlank() || item.kind() == Kind.ToolResult)
                return ApprovalResult.Pass;
              if (deny)
                return ApprovalResult.Deny;
              if (hold || !allow)
                return ApprovalResult.Pass;
              return item.kind() == Kind.ToolCall ? ApprovalResult.Call : ApprovalResult.Allow;
            }

            private FollowUp followUpFor(ApprovalContext ctx, ApprovalResult result, String reason, String hint, String instruction) {
              Prompt origin = ctx.prompt();
              ApprovalItem item = ctx.item();
              if (origin == null)
                return null;
              List<String> lines = switch(result) {
                case Pass ->
                  instruction.isBlank() ? null : instruction.lines().toList();
                case Allow ->
                  withBlock(item, command(AnswerCommand.CMD_ANSWER + " " + item.id() + " allow", hint));
                case Deny ->
                  item.isControl() ? withBlock(item, command(AnswerCommand.CMD_ANSWER + " " + item.id() + " deny", reason)) : List.of(denial(item, reason));
                case Call ->
                  withBlock(item, CallCommand.CMD_CALL + " " + item.id());
              };
              return lines == null ? null : new FollowUp(origin, selection(lines));
            }

            /**
             * Yaml block followed by the command line, as the operator would select it.
             */
            private static List<String> withBlock(ApprovalItem item, String commandLine) {
              List<String> lines = new ArrayList<>();
              lines.add("```yaml");
              item.yaml().lines().forEach(lines::add);
              lines.add("```");
              lines.add(commandLine);
              return lines;
            }

            private static String denial(ApprovalItem item, String reason) {
              String tool = item.tool().isBlank() ? "" : " (" + item.tool() + ")";
              String why = oneLine(reason);
              return "Tool call " + item.id() + tool + " was denied" + (why.isEmpty() ? "." : ": " + why);
            }

            private static String command(String base, String extra) {
              String text = oneLine(extra);
              return text.isEmpty() ? base : base + " " + text;
            }

            private static String oneLine(String s) {
              return s == null ? "" : s.replaceAll("\\s+", " ").strip();
            }

            private static Selection selection(List<String> lines) {
              return new Selection(lines.toArray(String[]::new), null);
            }

            private static void addNonBlank(List<String> list, String text) {
              if (text != null && !text.isBlank())
                list.add(text);
            }

            /**
             * Warnings go in front so the command line stays the last line of the answer.
             */
            private static void enrich(AIAnswer ans, List<String> warnings) {
              if (warnings.isEmpty())
                return;
              StringBuilder sb = new StringBuilder();
              for (String w : warnings) sb.append("//warning: ").append(oneLine(w)).append('\n');
              ans.answer = sb + (ans.answer == null ? "" : ans.answer);
            }

            private void publish(Decision decision) {
              for (Consumer<Decision> l : listeners) {
                try {
                  l.accept(decision);
                } catch (RuntimeException e) {
                  LOG.error(e.getMessage(), e);
                }
              }
            }
          }
```
/answer 0a99-a-9 allow
ReasoningToken: 52
Control Request:
```yaml
id: 0a99-b-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: RuleRegistry|IApprovalRule|AutoApprover|RuleSettings
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: RuleRegistry|IApprovalRule|AutoApprover|RuleSettings
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    limit: 50
  reason: "Verwendungsstellen der Registry, Regeln und des AutoApprovers finden"
```
/answer 0a99-b-0 allow
Control Request:
```yaml
id: 0a99-c-b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/approver/AutoApprovePanel.java
        matches:
        - import xy.ai.conscious.autoapprove.AutoApprover;
        - private AutoApprover approver;
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/RuleRegistry.java
        matches:
        - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
        - "public class RuleRegistry {"
        - private final CopyOnWriteArrayList<IApprovalRule> rules = new CopyOnWriteArrayList<>();
        - "public RuleRegistry() {"
        - "public RuleRegistry(List<IApprovalRule> rules) {"
        - "public void register(IApprovalRule rule) {"
        - "public boolean unregister(IApprovalRule rule) {"
        - "public List<IApprovalRule> rules() {"
      - path: xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
        - "public class AutoApprover {"
        - private final RuleRegistry registry;
        - "public AutoApprover() {"
        - this(new RuleRegistry());
        - "public AutoApprover(RuleRegistry registry) {"
        - "public RuleRegistry rules() {"
        - "for (IApprovalRule rule : registry.rules()) {"
      - path: xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
        matches:
        - "public class DenyRootPathRule implements IApprovalRule {"
      - path: xy/ai/conscious/autoapprove/rules/IApprovalRule.java
        matches:
        - "public interface IApprovalRule {"
        - RuleSettings getDefaultSetting();
      - path: xy/ai/conscious/autoapprove/rules/SmallResultRule.java
        matches:
        - "public class SmallResultRule implements IApprovalRule {"
      - path: xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
        matches:
        - "public class AllowedToolsRule implements IApprovalRule {"
      - path: xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
        matches:
        - "public class HoldMarkdownRule implements IApprovalRule {"
      - path: xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
        matches:
        - "public class DeepPathReadRule implements IApprovalRule {"
      - path: xy/ai/conscious/autoapprove/rules/RuleSettings.java
        matches:
        - "public class RuleSettings {"
      - path: xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - import xy.ai.conscious.autoapprove.AutoApprover;
        - private final AutoApprover approver = new AutoApprover();
        - "public AutoApprover approver() {"
```
/answer 0a99-c-b allow
Control Request:
```yaml
id: 0a99-d-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/Clearable.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolSelection.java
  node_type: ^(ClassOrInterfaceDeclaration|RecordDeclaration|EnumDeclaration)$
  reason: "UI-Klassen für Tabs, Presets und Clear-Logik lesen"
```
/answer 0a99-d-0 allow
Control Request:
```yaml
id: 0a99-e-d
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
      nodes:
      - id: InstructionSection
        type: ClassOrInterfaceDeclaration
        code: |
          public class InstructionSection {
            private final FormToolkit toolkit;
            private final UpdateGuard guard = new UpdateGuard();
            private final SelectTab selectTab;
            private final EditTab editTab;
            private final ToolsTab toolsTab = new ToolsTab();
            private final PresetsTab presetsTab = new PresetsTab();
            private final InputsTable inputsTable = new InputsTable();

            public InstructionSection(FormToolkit toolkit, Runnable reflow) {
              this.toolkit = toolkit;
              this.selectTab = new SelectTab(toolkit, guard, reflow);
              this.editTab = new EditTab(toolkit, guard, reflow);
            }

            public void create(Composite body, ConfigManager cfg, InputStats stats, Shell shell) {
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
              inputsTable.create(middle, cfg, stats);
              presetsTab.create(instr, cfg, shell, toolsTab, selectTab, inputsTable);
            }
          }
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
      nodes:
      - id: PresetList
        type: ClassOrInterfaceDeclaration
        signature: "/**  * Discovers prompt preset files across the workspace, deduplicates them by…"
        docstring: "Discovers prompt preset files across the workspace, deduplicates them by"
        children:
        - id: PresetList.Entry
          type: ClassOrInterfaceDeclaration
          code: |2
              public static class Entry {
                public final IFile file;
                public final String label;

                Entry(IFile file, String label) {
                  this.file = file;
                  this.label = label;
                }
              }
        - id: PresetList.Raw
          type: ClassOrInterfaceDeclaration
          code: |2
              private static class Raw {
                IFile file;
                String title;
              }
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
      nodes:
      - id: PresetsTab
        type: ClassOrInterfaceDeclaration
        code: |
          public class PresetsTab {
            private List presetList;
            private final PresetList presets = new PresetList();
            private final java.util.List<IFile> presetFiles = new ArrayList<>();

            public void create(TabFolder instr, ConfigManager cfg, Shell shell, ToolSelection tools, Clearable instructionSelection, InputSelection inputs) {
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
                  PresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), inputs.getSelection(), shell);
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
          }
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      nodes:
      - id: SelectTab
        type: ClassOrInterfaceDeclaration
        code: |
          public class SelectTab implements Clearable {
            private final FormToolkit toolkit;
            private final UpdateGuard guard;
            private final Runnable reflow;
            private MultiSelectTable instructionList;
            private Text instructionFree;

            public SelectTab(FormToolkit toolkit, UpdateGuard guard, Runnable reflow) {
              this.toolkit = toolkit;
              this.guard = guard;
              this.reflow = reflow;
            }

            public void create(TabFolder instr, ConfigManager cfg) {
              TabItem instrSel = new TabItem(instr, SWT.NONE);
              instrSel.setText("Select");
              SashForm sash = new SashForm(instr, SWT.VERTICAL);
              sash.setLayout(new GridLayout(1, false));
              GridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);
              scl2.heightHint = 100;
              scl2.widthHint = 1;
              sash.setLayoutData(scl2);
              Composite comp = new Composite(sash, SWT.NONE);
              comp.setLayout(new GridLayout());
              GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
              ldat3.heightHint = 100;
              comp.setLayoutData(ldat3);
              instrSel.setControl(sash);
              instructionList = new MultiSelectTable(comp, () -> cfg.setSystemPrompt(togglePromptLines()));
              cfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);
              GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
              gridData.widthHint = 1;
              gridData.heightHint = 100;
              instructionList.setLayoutData(gridData);
              createFreeTextArea(sash, cfg);
            }

            private void createFreeTextArea(SashForm sash, ConfigManager cfg) {
              instructionFree = toolkit.createText(sash, "", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);
              cfg.addSystemFreeObs(p -> {
                if (!instructionFree.isFocusControl())
                  guard.run(() -> {
                    instructionFree.setText(p != null ? p : "");
                    reflow.run();
                  });
              }, true);
              instructionFree.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setSystemFree(instructionFree.getText())));
              GridData gridData = new GridData(GridData.FILL_HORIZONTAL);
              gridData.widthHint = 1;
              instructionFree.setLayoutData(gridData);
              instructionFree.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionFree.setFocus()));
              sash.setWeights(3, 1);
            }

            /**
             * Rewrites the "#" prefix of every line according to the active rows.
             */
            private String[] togglePromptLines() {
              String[] lines = instructionList.getItems();
              boolean[] active = new boolean[lines.length];
              for (int i : instructionList.getSelectionIndices()) active[i] = true;
              String[] upd = new String[lines.length];
              for (int i = 0; i < lines.length; i++) {
                String line = lines[i];
                if (!active[i] && !line.startsWith("#"))
                  line = "#" + line;
                else if (active[i] && line.startsWith("#"))
                  line = line.substring(1);
                upd[i] = line;
              }
              return upd;
            }

            private void updateInstructionList(String[] systemPrompt) {
              guard.run(() -> {
                if (!instructionList.updateTexts(systemPrompt))
                  instructionList.setItems(systemPrompt);
                instructionList.setSelectionIndices(IntStream.range(0, systemPrompt.length).filter(i -> !systemPrompt[i].startsWith("#")).toArray());
                reflow.run();
              });
            }

            @Override
            public void clear() {
              instructionList.resetAnchor();
            }
          }
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      nodes:
      - id: ToolsTab
        type: ClassOrInterfaceDeclaration
        code: |
          public class ToolsTab implements ToolSelection {
            private MultiSelectTable toolsTable;

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

            @Override
            public String[] getSelection() {
              return toolsTable.getSelection();
            }

            @Override
            public void setSelection(String[] items) {
              toolsTable.setSelection(items);
            }

            @Override
            public void clear() {
              toolsTable.clear();
            }
          }
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/Clearable.java
      nodes:
      - id: Clearable
        type: ClassOrInterfaceDeclaration
        code: "public interface Clearable {\n\tpublic void clear();\n}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolSelection.java
      nodes:
      - id: ToolSelection
        type: ClassOrInterfaceDeclaration
        code: "public interface ToolSelection extends Clearable {\n\tpublic String[] getSelection();\n\n\tpublic void setSelection(String[] items);\n}\n"
```
/answer 0a99-e-d allow
ReasoningToken: 64
Control Request:
```yaml
id: 0a99-f-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
  node_type: ^(ClassOrInterfaceDeclaration)$
  reason: "ConfigManager-Quelltext lesen (Setter/Getter/Observer-Muster, applyConfig)"
```
/answer 0a99-f-0 allow
Control Request:
```yaml
id: 0a99-10-f
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
      nodes:
      - id: ConfigManager
        type: ClassOrInterfaceDeclaration
        code: "public class ConfigManager {\n\tprivate ModelResolverRegistry resolverRegistry;\n\n\tprotected SessionConfig cfg;\n\tprivate Model[] enabledModels = new Model[0];\n\tprivate AgentProfile[] enabledProfiles = new AgentProfile[0];\n\tprivate List<Consumer<SessionConfig>> systemPromptObs = new ArrayList<>();\n\tprivate List<Consumer<String>> systemFreeObs = new ArrayList<>();\n\tprivate List<Consumer<boolean[]>> inputObs = new ArrayList<>();\n\tprivate List<Consumer<InputMode>> inputModeObs = new ArrayList<>();\n\tprivate List<Consumer<Model>> modelObs = new ArrayList<>();\n\tprivate List<Consumer<AgentProfile>> profileObs = new ArrayList<>();\n\tprivate List<Consumer<String>> keyObs = new ArrayList<>();\n\tprivate List<Consumer<Long>> outTokenObs = new ArrayList<>();\n\tprivate List<Consumer<Integer>> budgetObs = new ArrayList<>();\n\tprivate List<Consumer<Reasoning>> reasonObs = new ArrayList<>();\n\tprivate List<Consumer<CacheMode>> cacheObs = new ArrayList<>();\n\tprivate List<Consumer<Model[]>> enabledModelsObs = new ArrayList<>();\n\tprivate List<Consumer<AgentProfile[]>> enabledProfilesObs = new ArrayList<>();\n\tprivate List<Consumer<Double>> temperatureObs = new ArrayList<>();\n\tprivate List<Consumer<Double>> topPObs = new ArrayList<>();\n\tprivate List<Consumer<OutputMode>> outputModeObs = new ArrayList<>();\n\tprivate List<Consumer<String[]>> enabledToolsObs = new ArrayList<>();\n\n\tpublic ConfigManager(MCPClient mcpClient, IFileCache cache) {\n\t\tresolverRegistry = new ModelResolverRegistry(cache);\n\t\tmcpClient.addConnectObserver(resolverRegistry);\n\t}\n\n\t/**\n\t * hook\n\t */\n\tprotected void ensureOwnConfig() {\n\t}\n\n\tpublic void clearObserver() {\n\t\tsystemPromptObs.clear();\n\t\tsystemFreeObs.clear();\n\t\tinputObs.clear();\n\t\tinputModeObs.clear();\n\t\tkeyObs.clear();\n\t\tmodelObs.clear();\n\t\tprofileObs.clear();\n\t\toutTokenObs.clear();\n\t\tbudgetObs.clear();\n\t\treasonObs.clear();\n\t\tcacheObs.clear();\n\t\tenabledModelsObs.clear();\n\t\tenabledProfilesObs.clear();\n\t\ttemperatureObs.clear();\n\t\ttopPObs.clear();\n\t\toutputModeObs.clear();\n\t\tenabledToolsObs.clear();\n\t}\n\n\n\n\tpublic void applyConfig(SessionConfig target) {\n\t\tif (target == cfg)\n\t\t\treturn;\n\t\tcfg = target;\n\t\tsystemPromptObs.forEach(c -> c.accept(cfg));\n\t\tsystemFreeObs.forEach(c -> c.accept(cfg.freeText));\n\t\tinputObs.forEach(c -> c.accept(cfg.inputModes));\n\t\tif (cfg.keys != null)\n\t\t\tfor (String key : cfg.keys.split(\",\"))\n\t\t\t\tkeyObs.forEach(c -> c.accept(key));\n\t\tmodelObs.forEach(c -> c.accept(cfg.model));\n\t\tprofileObs.forEach(c -> c.accept(cfg.profile));\n\t\toutTokenObs.forEach(c -> c.accept(cfg.maxOutputTokens));\n\t\tbudgetObs.forEach(c -> c.accept(cfg.reasoningBudget));\n\t\treasonObs.forEach(c -> c.accept(cfg.reasoning));\n\t\tcacheObs.forEach(c -> c.accept(cfg.cacheMode));\n\t\ttemperatureObs.forEach(c -> c.accept(cfg.temperature));\n\t\ttopPObs.forEach(c -> c.accept(cfg.topP));\n\t\toutputModeObs.forEach(c -> c.accept(cfg.ouputMode));\n\t\tupdateEnabledModels(cfg.keys != null ? cfg.keys.split(\",\") : new String[0]);\n\t}\n\n\tpublic void setKey(String keys) {\n\t\tif (Objects.equals(cfg.getKeys(), keys))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setKeys(keys);\n\n\t\tString[] keysa = cfg.keys.split(\",\");\n\t\tupdateEnabledModels(keysa);\n\t\tfor (String key : keysa)\n\t\t\tkeyObs.forEach(c -> c.accept(key));\n\t}\n\n\tpublic void setMaxOutputTokens(Long maxOutputTokens) {\n\t\tmaxOutputTokens = (long) getCapabilities().alignOutpuTokens(maxOutputTokens.intValue());\n\t\tif (Objects.equals(cfg.getMaxOutputTokens(), maxOutputTokens))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setMaxOutputTokens(maxOutputTokens);\n\t\toutTokenObs.forEach(c -> c.accept(cfg.maxOutputTokens));\n\t}\n\n\tpublic void setTemperature(Double temp) {\n\t\ttemp = getCapabilities().alignTemperature(temp);\n\t\tvar old = cfg.getTemperature();\n\t\tif (Objects.equals(temp, old) || (temp != null && old != null && Math.abs(temp - old) < 0.01))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setTemperature(temp);\n\t\ttemperatureObs.forEach(c -> c.accept(cfg.temperature));\n\t}\n\n\tpublic void setTopP(Double topP) {\n\t\ttopP = getCapabilities().alignTopP(topP);\n\t\tvar old = cfg.getTopP();\n\t\tif (Objects.equals(topP, old) || (topP != null && old != null && Math.abs(topP - old) < 0.01))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setTopP(topP);\n\t\ttopPObs.forEach(c -> c.accept(cfg.topP));\n\t}\n\n\tpublic void setModel(Model model) {\n\t\tif (Objects.equals(cfg.getModel(), model))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setModel(model);\n\n\t\tif (Arrays.asList(getCapabilities().getReasonings()).indexOf(cfg.reasoning) == -1)\n\t\t\tsetReasoning(getCapabilities().getReasonings()[0]);\n\t\tsetMaxOutputTokens((long) getCapabilities().alignOutpuTokens(Integer.MAX_VALUE));\n\t\tsetReasoningBudget(getCapabilities().alignBudget(Integer.MAX_VALUE));\n\t\tsetEnabledProfiles(getCapabilities().getAgentProfiles());\n\n\t\tvar profiles = getCapabilities().getAgentProfiles();\n\t\tsetProfile(profiles.length > 0 ? profiles[0] : null);\n\n\t\tCacheMode[] cacheModes = getCapabilities().getCacheMode();\n\t\tif (!Arrays.asList(cacheModes).contains(cfg.cacheMode))\n\t\t\tsetCacheMode(cacheModes.length > 0 ? cacheModes[0] : CacheMode.Default);\n\n\t\tmodelObs.forEach(c -> c.accept(model));\n\t}\n\n\tpublic void setReasoningBudget(Integer reasoningBudget) {\n\t\treasoningBudget = getCapabilities().alignBudget(reasoningBudget);\n\t\tif (Objects.equals(cfg.getReasoningBudget(), reasoningBudget))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setReasoningBudget(reasoningBudget);\n\t\tbudgetObs.forEach(c -> c.accept(cfg.reasoningBudget));\n\t}\n\n\tpublic void setReasoning(Reasoning reasoning) {\n\t\tif (Objects.equals(cfg.getReasoning(), reasoning))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setReasoning(reasoning);\n\t\treasonObs.forEach(c -> c.accept(cfg.reasoning));\n\t}\n\n\tpublic void setCacheMode(CacheMode cacheMode) {\n\t\tif (Objects.equals(cfg.getCacheMode(), cacheMode))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setCacheMode(cacheMode);\n\t\tcacheObs.forEach(c -> c.accept(cfg.cacheMode));\n\t}\n\n\tpublic void setProfile(AgentProfile profile) {\n\t\tif (Objects.equals(cfg.getProfile(), profile))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setProfile(profile);\n\t\tprofileObs.forEach(c -> c.accept(cfg.profile));\n\t}\n\n\tpublic void setSystemPrompt(String[] systemPrompt) {\n\t\tif (Arrays.equals(this.getSystemPrompt(), systemPrompt))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setSystemPrompt(systemPrompt);\n\t\tsystemPromptObs.forEach(c -> c.accept(cfg));\n\t\tinputModeObs.forEach(c -> c.accept(InputMode.SystemPrompt));\n\t}\n\n\tpublic void setSystemFree(String freeText) {\n\t\tif (Objects.equals(cfg.getFreeText(), freeText))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setFreeText(freeText);\n\t\tsystemFreeObs.forEach(c -> c.accept(cfg.freeText));\n\t\tinputModeObs.forEach(c -> c.accept(InputMode.SystemPrompt));\n\t}\n\n\tpublic void setEnabledTools(String[] enabledTools) {\n\t\tif (Arrays.equals(cfg.getEnabledTools(), enabledTools))\n\t\t\treturn;\n\t\tcfg.setEnabledTools(enabledTools);\n\t\tenabledToolsObs.forEach(c -> c.accept(enabledTools));\n\t}\n\n\tpublic void setOuputMode(OutputMode ouputMode) {\n\t\tif (Objects.equals(cfg.ouputMode, ouputMode))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.ouputMode = ouputMode;\n\t\toutputModeObs.forEach(c -> c.accept(cfg.ouputMode));\n\t}\n\n\tpublic void setInputMode(InputMode mode, boolean enable) {\n\t\tif (enable && cfg.isInputEnabled(mode) || !enable && !cfg.isInputEnabled(mode))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setInputMode(mode, enable);\n\t\t// Converter and Selection are mutually exclusive: enabling one disables the\n\t\t// other.\n\t\tif (enable && (mode == InputMode.Converter || mode == InputMode.Selection)) {\n\t\t\tInputMode other = mode == InputMode.Converter ? InputMode.Selection : InputMode.Converter;\n\t\t\tif (cfg.isInputEnabled(other)) {\n\t\t\t\tcfg.setInputMode(other, false);\n\t\t\t\tinputObs.forEach(c -> c.accept(cfg.inputModes));\n\t\t\t\tinputModeObs.forEach(c -> c.accept(other));\n\t\t\t}\n\t\t}\n\t\tinputObs.forEach(c -> c.accept(cfg.inputModes));\n\t\tinputModeObs.forEach(c -> c.accept(mode));\n\t}\n\n\tpublic String getKeys() {\n\t\treturn cfg.getKeys();\n\t}\n\n\tpublic Long getMaxOutputTokens() {\n\t\treturn cfg.getMaxOutputTokens();\n\t}\n\n\tpublic Double getTemperature() {\n\t\treturn cfg.getTemperature();\n\t}\n\n\tpublic Double getTopP() {\n\t\treturn cfg.getTopP();\n\t}\n\n\tpublic Model getModel() {\n\t\treturn cfg.getModel();\n\t}\n\n\tpublic Integer getReasoningBudget() {\n\t\treturn cfg.getReasoningBudget();\n\t}\n\n\tpublic AgentProfile getProfile() {\n\t\treturn cfg.getProfile();\n\t}\n\n\tpublic Reasoning getReasoning() {\n\t\treturn cfg.getReasoning();\n\t}\n\n\tpublic CacheMode getCacheMode() {\n\t\treturn cfg.getCacheMode();\n\t}\n\n\tpublic String[] getSystemPrompt() {\n\t\treturn cfg.getSystemPrompt();\n\t}\n\n\tpublic String getFreeText() {\n\t\treturn cfg.getFreeText();\n\t}\n\n\tpublic OutputMode getOuputMode() {\n\t\treturn cfg.ouputMode;\n\t}\n\n\tpublic boolean isInputEnabled(InputMode mode) {\n\t\treturn cfg.isInputEnabled(mode);\n\t}\n\n\tpublic void addSystemPromptObs(Consumer<SessionConfig> obs, boolean initialize) {\n\t\tsystemPromptObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg);\n\t}\n\n\tpublic void addSystemFreeObs(Consumer<String> obs, boolean initialize) {\n\t\tsystemFreeObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.freeText);\n\t}\n\n\tpublic void addProfileObs(Consumer<AgentProfile> obs, boolean initialize) {\n\t\tprofileObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.profile);\n\t}\n\n\tpublic void addTemperatureObs(Consumer<Double> obs, boolean initialize) {\n\t\ttemperatureObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.temperature);\n\t}\n\n\tpublic void addTopPObs(Consumer<Double> obs, boolean initialize) {\n\t\ttopPObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.topP);\n\t}\n\n\tpublic void addInputObs(Consumer<boolean[]> obs, boolean initialize) {\n\t\tinputObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.inputModes);\n\t}\n\n\tpublic void addBudgetObs(Consumer<Integer> obs, boolean initialize) {\n\t\tbudgetObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.reasoningBudget);\n\t}\n\n\tpublic void addReasoningObs(Consumer<Reasoning> obs, boolean initialize) {\n\t\treasonObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.reasoning);\n\t}\n\n\tpublic void addCacheObs(Consumer<CacheMode> obs, boolean initialize) {\n\t\tcacheObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.cacheMode);\n\t}\n\n\tpublic void addKeyObs(Consumer<String> obs, boolean initialize) {\n\t\tkeyObs.add(obs);\n\t\tif (initialize && cfg.keys != null)\n\t\t\tfor (String key : cfg.keys.split(\",\"))\n\t\t\t\tobs.accept(key);\n\t}\n\n\tpublic void addModelObs(Consumer<Model> obs, boolean initialize) {\n\t\tmodelObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.model);\n\t}\n\n\tpublic void addOutputTokenObs(Consumer<Long> obs, boolean initialize) {\n\t\toutTokenObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.maxOutputTokens);\n\t}\n\n\tpublic void addEnabledModelsObs(Consumer<Model[]> obs, boolean initialize) {\n\t\tenabledModelsObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(enabledModels);\n\t}\n\n\tpublic void addEnabledProfilesObs(Consumer<AgentProfile[]> obs, boolean initialize) {\n\t\tenabledProfilesObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(enabledProfiles);\n\t}\n\n\tpublic void addInputModeObs(Consumer<InputMode> obs) {\n\t\tinputModeObs.add(obs);\n\t}\n\n\tpublic void addOutputModeObs(Consumer<OutputMode> obs, boolean initialize) {\n\t\toutputModeObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.ouputMode);\n\t}\n\n\tpublic void addEnabledToolsObs(Consumer<String[]> obs, boolean initialize) {\n\t\tenabledToolsObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(cfg.getEnabledTools());\n\t}\n\n\tprivate void updateEnabledModels(String[] keys) {\n\t\tList<Model> avail = new ArrayList<Model>();\n\t\tfor (String key : keys)\n\t\t\tfor (KeyPattern pattern : KeyPattern.values())\n\t\t\t\tif (pattern.matches(key))\n\t\t\t\t\tfor (Model mod : resolverRegistry.resolve(pattern, key))\n\t\t\t\t\t\tif (!avail.contains(mod))\n\t\t\t\t\t\t\tavail.add(mod);\n\n\t\tenabledModels = avail.toArray(new Model[0]);\n\n\t\tif (!avail.contains(cfg.model) && !avail.isEmpty())\n\t\t\tsetModel(avail.get(0));\n\n\t\tenabledModelsObs.forEach(c -> c.accept(enabledModels));\n\n\t\tif (cfg.model != null)\n\t\t\tenabledProfiles = cfg.model.cap.getAgentProfiles();\n\n\t\tenabledProfilesObs.forEach(c -> c.accept(enabledProfiles));\n\t}\n\n\tpublic void setEnabledProfiles(AgentProfile[] enabledProfiles) {\n\t\tif (Arrays.equals(this.enabledProfiles, enabledProfiles))\n\t\t\treturn;\n\t\tthis.enabledProfiles = enabledProfiles;\n\t\tenabledProfilesObs.forEach(c -> c.accept(enabledProfiles));\n\t}\n\n\tpublic Model[] getEnabledModels() {\n\t\treturn enabledModels;\n\t}\n\n\tpublic String[] getReasonings() {\n\t\tReasoning[] reasonings = getCapabilities().getReasonings();\n\t\tString[] options = Arrays.stream(reasonings).map((m) -> m.name()).collect(Collectors.toList())\n\t\t\t\t.toArray(new String[0]);\n\t\treturn options;\n\t}\n\n\tpublic Capabilities getCapabilities() {\n\t\treturn cfg.model.cap;\n\t}\n\n\tpublic String[] getEnabledTools() {\n\t\treturn cfg.getEnabledTools();\n\t}\n}\n"
```
/answer 0a99-10-f allow
ReasoningToken: 88
Control Request:
```yaml
id: 0a99-11-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: class (SessionConfig|PresetHandler|Prompt|PromptHandler|MultiSelectTable|FrontMatter\w*)\b|FrozenConfig\.from|\.frozen|enabledTools|getEnabledTools
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: class (SessionConfig|PresetHandler|Prompt|PromptHandler|MultiSelectTable|FrontMatter\w*)\b|FrozenConfig\.from|\.frozen|enabledTools|getEnabledTools
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    limit: 50
  reason: "Zugehörige Klassen (SessionConfig, PresetHandler, Prompt, PromptHandler, Persistenz) und Verwendung von enabledTools finden"
```
/answer 0a99-11-0 allow
Control Request:
```yaml
id: 0a99-12-11
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/MementoConverter.java
        matches:
        - snap.enabledTools = cfg.getEnabledTools();
        - "public String[] enabledTools;"
      - path: xy/ai/workbench/view/PresetHandler.java
        matches:
        - "public class PresetHandler {"
        - "public static class FrontMatter {"
      - path: xy/ai/workbench/view/InputStats.java
        matches:
        - "String[] tools = cfg.getEnabledTools();"
      - path: xy/ai/workbench/view/MultiSelectTable.java
        matches:
        - "public class MultiSelectTable implements Clearable {"
      - path: xy/ai/workbench/view/ClaudeCodeSessionView.java
        matches:
        - "List<String> tools = cfg.isInputEnabled(InputMode.Tools) ? Arrays.asList(cfg.getEnabledTools()) : List.of();"
      - path: xy/ai/workbench/eclipse/ConfigManagerAdapter.java
        matches:
        - if (snap.enabledTools != null)
        - cm.setEnabledTools(snap.enabledTools);
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/core/SessionConfig.java
        matches:
        - "public class SessionConfig {"
        - "public String[] enabledTools = new String[0];"
        - this.enabledTools = other.enabledTools;
        - "public String[] getEnabledTools() {"
        - return enabledTools;
        - "public void setEnabledTools(String[] enabledTools) {"
        - this.enabledTools = enabledTools;
      - path: xy/ai/conscious/core/ConfigManager.java
        matches:
        - "private List<Consumer<String[]>> enabledToolsObs = new ArrayList<>();"
        - enabledToolsObs.clear();
        - "public void setEnabledTools(String[] enabledTools) {"
        - "if (Arrays.equals(cfg.getEnabledTools(), enabledTools))"
        - cfg.setEnabledTools(enabledTools);
        - enabledToolsObs.forEach(c -> c.accept(enabledTools));
        - enabledToolsObs.add(obs);
        - obs.accept(cfg.getEnabledTools());
        - "public String[] getEnabledTools() {"
        - return cfg.getEnabledTools();
      - path: xy/ai/conscious/connector/claudecode/CCSessionParameters.java
        matches:
        - FrozenConfig fc = FrozenConfig.from(cfg);
      - path: xy/ai/conscious/connector/harness/PromptHandler.java
        matches:
        - "public class PromptHandler {"
      - path: xy/ai/conscious/connector/harness/FrozenConfig.java
        matches:
        - "String[] rawTools = cfg.isInputEnabled(InputMode.Tools) ? cfg.getEnabledTools() : null;"
      - path: xy/ai/conscious/connector/harness/PromptInputHandler.java
        matches:
        - FrozenConfig frozen = FrozenConfig.from(cfg);
      - path: xy/ai/conscious/connector/harness/Prompt.java
        matches:
        - "public class Prompt {"
```
/answer 0a99-12-11 allow
Control Request:
```yaml
id: 0a99-13-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/SessionConfig.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
  node_type: ^(ClassOrInterfaceDeclaration)$
  reason: "SessionConfig, PresetHandler/FrontMatter, Prompt, PromptHandler, Memento-Persistenz lesen"
```
/answer 0a99-13-0 allow
Control Request:
```yaml
id: 0a99-14-13
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/SessionConfig.java
      nodes:
      - id: SessionConfig
        type: ClassOrInterfaceDeclaration
        code: "public class SessionConfig {\n\tpublic String keys = \"dummy\";\n\tpublic Long maxOutputTokens = 128000L;\n\tpublic Double temperature = 0d;\n\tpublic Double topP = 0.1d;\n\tpublic Model model = Model.MCP_TOOLS;\n\tpublic AgentProfile profile;\n\tpublic String[] systemPrompt = new String[] { //\n\t\t\t\"Answer very short and precise\", //\n\t\t\t\"Be objective and neutral\", //\n\t\t\t\"Don't repeat the input\", //\n\t\t\t\"Don't ask follow-up questions\", //\n\t\t\t\"#Use $ Markdown inline Latex syntax for math formulas\", //\n\t\t\t\"#keep the input language\", //\n\t\t\t\"#For generated code use english language\", //\n\t\t\t\"#Replace all mathematical symbols and formulas in the input with a $ Latex inline syntax without changing or adding to the text.\"//\n\t};\n\tpublic OutputMode ouputMode = OutputMode.Append;\n\tpublic boolean[] inputModes = new boolean[InputMode.values().length];\n\tpublic Reasoning reasoning = Reasoning.minimal;\n\tpublic CacheMode cacheMode = CacheMode.Default;\n\tpublic Integer reasoningBudget = -1;\n\tpublic String freeText;\n\tpublic String[] enabledTools = new String[0];\n\n\tpublic SessionConfig() {\n\t\tsetInputMode(InputMode.SystemPrompt, true);\n\t\tsetInputMode(InputMode.Tools, true);\n\t}\n\n\tpublic SessionConfig(SessionConfig other) {\n\t\tthis.keys = other.keys;\n\t\tthis.maxOutputTokens = other.maxOutputTokens;\n\t\tthis.temperature = other.temperature;\n\t\tthis.topP = other.topP;\n\t\tthis.model = other.model;\n\t\tthis.profile = other.profile;\n\t\tthis.systemPrompt = other.systemPrompt != null ? other.systemPrompt.clone() : null;\n\t\tthis.ouputMode = other.ouputMode;\n\t\tthis.inputModes = other.inputModes.clone();\n\t\tthis.reasoning = other.reasoning;\n\t\tthis.cacheMode = other.cacheMode;\n\t\tthis.reasoningBudget = other.reasoningBudget;\n\t\tthis.freeText = other.freeText;\n\t\tthis.enabledTools = other.enabledTools;\n\t}\n\n\tpublic String getKeys() {\n\t\treturn keys;\n\t}\n\n\tpublic void setKeys(String keys) {\n\t\tthis.keys = keys;\n\t}\n\n\tpublic Long getMaxOutputTokens() {\n\t\treturn maxOutputTokens;\n\t}\n\n\tpublic void setMaxOutputTokens(Long maxOutputTokens) {\n\t\tthis.maxOutputTokens = maxOutputTokens;\n\t}\n\n\tpublic Double getTemperature() {\n\t\treturn temperature;\n\t}\n\n\tpublic void setTemperature(Double temperature) {\n\t\tthis.temperature = temperature;\n\t}\n\n\tpublic Double getTopP() {\n\t\treturn topP;\n\t}\n\n\tpublic void setTopP(Double topP) {\n\t\tthis.topP = topP;\n\t}\n\n\tpublic Model getModel() {\n\t\treturn model;\n\t}\n\n\tpublic void setModel(Model model) {\n\t\tthis.model = model;\n\t}\n\n\tpublic void setProfile(AgentProfile profile) {\n\t\tthis.profile = profile;\n\t}\n\n\tpublic AgentProfile getProfile() {\n\t\treturn profile;\n\t}\n\n\tpublic Reasoning getReasoning() {\n\t\treturn reasoning;\n\t}\n\n\tpublic void setReasoning(Reasoning reasoning) {\n\t\tthis.reasoning = reasoning;\n\t}\n\n\tpublic CacheMode getCacheMode() {\n\t\treturn cacheMode;\n\t}\n\n\tpublic void setCacheMode(CacheMode cacheMode) {\n\t\tthis.cacheMode = cacheMode;\n\t}\n\n\tpublic String[] getSystemPrompt() {\n\t\treturn systemPrompt;\n\t}\n\n\tpublic void setSystemPrompt(String[] systemPrompt) {\n\t\tthis.systemPrompt = systemPrompt;\n\t}\n\n\tpublic boolean isInputEnabled(InputMode mode) {\n\t\treturn inputModes[mode.ordinal()];\n\t}\n\n\tpublic void setInputMode(InputMode mode, boolean enable) {\n\t\tinputModes[mode.ordinal()] = enable;\n\t}\n\n\tpublic Integer getReasoningBudget() {\n\t\treturn reasoningBudget;\n\t}\n\n\tpublic void setReasoningBudget(Integer reasoningBudget) {\n\t\tthis.reasoningBudget = reasoningBudget;\n\t}\n\n\tpublic void setFreeText(String freeText) {\n\t\tthis.freeText = freeText;\n\t}\n\n\tpublic String getFreeText() {\n\t\treturn freeText;\n\t}\n\n\tpublic String[] getEnabledTools() {\n\t\treturn enabledTools;\n\t}\n\n\tpublic void setEnabledTools(String[] enabledTools) {\n\t\tthis.enabledTools = enabledTools;\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
      nodes:
      - id: PresetHandler
        type: ClassOrInterfaceDeclaration
        signature: "public class PresetHandler {"
        children:
        - id: PresetHandler.FrontMatter
          type: ClassOrInterfaceDeclaration
          code: |2
              public static class FrontMatter {
                public String title;
                public List<String> tools;
                public String outputMode;
                public List<String> inputs;
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
              }
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/Prompt.java
      nodes:
      - id: Prompt
        type: ClassOrInterfaceDeclaration
        code: "public class Prompt {\n\tpublic final List<String> inputs;\n\tpublic final boolean batch;\n\tpublic final FrozenConfig config;\n\tpublic final PromptArguments arg;\n\n\tprivate String sessionId;\n\n\tpublic Prompt(List<String> inputs, boolean batch, FrozenConfig config, PromptArguments arg) {\n\t\tif (config == null)\n\t\t\tthrow new IllegalArgumentException(\"config must not be null\");\n\t\tif (arg.absoluteFilePath == null || arg.absoluteFilePath.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"absoluteFilePath must not be blank\");\n\t\tif (arg.project == null)\n\t\t\tthrow new IllegalArgumentException(\"projectPath must not be null\");\n\t\tthis.inputs = inputs != null ? List.copyOf(inputs) : List.of();\n\t\tthis.batch = batch;\n\t\tthis.config = config;\n\t\tthis.arg = arg;\n\t}\n\n\t/**\n\t * Deterministic session id: hash(absoluteFilePath, batch, config.getHash()).\n\t */\n\tpublic String sessionId() {\n\t\tif (sessionId == null)\n\t\t\tsessionId = computeSessionId();\n\t\treturn sessionId;\n\t}\n\n\tprivate String computeSessionId() {\n\t\tString input = arg.absoluteFilePath + \"|\" + batch + \"|\" + config.getHash();\n\t\treturn Hash.hash(input);\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler
        type: ClassOrInterfaceDeclaration
        signature: "/**  * Prompt-specific orchestration extracted from {"
        docstring: "Prompt-specific orchestration extracted from {@code AISessionManager}: drives"
        children:
        - id: PromptHandler.PromptRun
          type: ClassOrInterfaceDeclaration
          code: |2
              private class PromptRun implements IMonitoredRunnable {
                private final IExecutor exec;
                private final Prompt preset;
                private final int turnDepth;

                private PromptRun(IExecutor exec) {
                  this(exec, null, 0);
                }

                private PromptRun(IExecutor exec, Prompt preset, int depth) {
                  this.exec = exec;
                  this.preset = preset;
                  this.turnDepth = depth;
                }

                @Override
                public void run(IProgress mon) {
                  IProgress sub = mon.setTask("Executing prompt", 4);
                  String reqId = null;
                  Decision decision = null;
                  try {
                    sub.setTaskName("Prepare inputs");
                    var req = prepareRequest(exec, false, preset, sub);
                    sub.worked();
                    sub.setTaskName("Insert Tag");
                    editIfc.insertTag(exec, req, sub);
                    reqId = req.getID();
                    mon.worked();
                    sub.setTaskName("Execute prompt");
                    AIAnswer ans = executeInner(exec, req, sub);
                    mon.worked();
                    sub.setTaskName("Process Answer");
                    Decision evaluated = evaluate(ans, turnDepth);
                    editIfc.replaceTag(ans, sub);
                    decision = evaluated;
                    mon.worked();
                  } catch (Exception e) {
                    LOG.error(e.getMessage(), e);
                    // uncatched error case
                    if (reqId != null) {
                      AIAnswer error = new AIAnswer(reqId);
                      error.answer = e.getMessage();
                      editIfc.replaceTag(error, sub);
                    }
                  } finally {
                    mon.done();
                  }
                  if (decision != null && decision.followUp() != null)
                    scheduleFollowUp(exec, decision.followUp(), turnDepth + 1);
                }
              }
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
      nodes:
      - id: MementoConverter
        type: ClassOrInterfaceDeclaration
        signature: "/**  * (De)serializes {"
        docstring: "(De)serializes {@link SessionConfig} as a single JSON blob instead of"
        children:
        - id: MementoConverter.Snapshot
          type: ClassOrInterfaceDeclaration
          code: "\tpublic static class Snapshot {\n\t\tpublic String keys;\n\t\tpublic Long maxOutputTokens;\n\t\tpublic Double temperature;\n\t\tpublic Double topP;\n\t\tpublic String freeText;\n\t\tpublic String modelProvider;\n\t\tpublic String modelDisplayName;\n\t\tpublic String reasoning;\n\t\tpublic String cacheMode;\n\t\tpublic String profile;\n\t\tpublic Integer reasoningBudget;\n\t\tpublic String[] systemPrompt;\n\t\tpublic String outputMode;\n\t\tpublic boolean[] inputModes;\n\t\tpublic String[] enabledTools;\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
      nodes:
      - id: ConfigManagerAdapter
        type: ClassOrInterfaceDeclaration
        code: "public class ConfigManagerAdapter extends ConfigManager{\n\tprivate ConfigManager cm;\n\tprivate String activeEditorKey;\n\tprivate Map<String, SessionConfig> editorConfigs = new HashMap<>();\n\tprivate SessionConfig defaultCfg = new SessionConfig();\n\n\n\tpublic ConfigManagerAdapter(ActiveEditorListener editorListener, MCPClient mcpClient, IFileCache cache) {\n\t\tsuper(mcpClient, cache);\n\t\tcm = this;\n\t\tcfg = defaultCfg;\n\t\teditorListener.addTextEditorObserver(e -> activateEditor(e));\n\t}\n\n\tpublic ConfigManager getManager() {\n\t\treturn cm;\n\t}\n\n\tprivate void activateEditor(IEditorPart editor) {\n\t\tString key = resolveKey(editor);\n\t\tif (Objects.equals(activeEditorKey, key))\n\t\t\treturn;\n\t\tactiveEditorKey = key;\n\t\tSessionConfig target = key == null ? defaultCfg : editorConfigs.getOrDefault(key, cfg);\n\t\tcm.applyConfig(target);\n\t}\n\n\tprivate static String resolveKey(IEditorPart editor) {\n\t\tif (editor == null)\n\t\t\treturn null;\n\t\tIFile file = ResourceUtil.getFile(editor.getEditorInput());\n\t\treturn file != null ? file.getFullPath().toString() : null;\n\t}\n\n\tprotected void ensureOwnConfig() {\n\t\tif (activeEditorKey == null || editorConfigs.containsKey(activeEditorKey))\n\t\t\treturn;\n\t\tcfg = new SessionConfig(cfg);\n\t\teditorConfigs.put(activeEditorKey, cfg);\n\t}\n\n\n\tpublic void saveConfig(IMemento memento) {\n\t\tMementoConverter.saveConfig(memento, cfg);\n\t}\n\n\tpublic void loadConfig(IMemento memento) {\n\t\tString savedKey = activeEditorKey;\n\t\tactiveEditorKey = null;\n\t\tcfg = defaultCfg;\n\t\ttry {\n\t\t\tMementoConverter.Snapshot snap = MementoConverter.loadConfig(memento);\n\t\t\tif (snap == null)\n\t\t\t\treturn;\n\n\t\t\t/*\n\t\t\t * Order matters: keys must be set (models resolved) before the model itself can\n\t\t\t * be restored, and the model must be set before the remaining,\n\t\t\t * capability-dependent settings are applied - otherwise setModel()'s own\n\t\t\t * defaulting logic would clobber them.\n\t\t\t */\n\t\t\tif (snap.keys != null)\n\t\t\t\tcm.setKey(snap.keys);\n\n\t\t\tif (snap.modelProvider != null) {\n\t\t\t\tModel resolved = Arrays.stream(cm.getEnabledModels())\n\t\t\t\t\t\t.filter(m -> m.matches(KeyPattern.valueOf(snap.modelProvider), snap.modelDisplayName)).findFirst()\n\t\t\t\t\t\t.orElse(null);\n\t\t\t\tif (resolved != null)\n\t\t\t\t\tcm.setModel(resolved);\n\t\t\t\telse\n\t\t\t\t\tLOG.error(\"Unable to resolve persisted model \" + snap.modelProvider + \":\" + snap.modelDisplayName\n\t\t\t\t\t\t\t+ \", no fallback applied\");\n\t\t\t}\n\n\t\t\tif (snap.temperature != null)\n\t\t\t\tcm.setTemperature(snap.temperature);\n\t\t\tif (snap.topP != null)\n\t\t\t\tcm.setTopP(snap.topP);\n\t\t\tif (snap.maxOutputTokens != null)\n\t\t\t\tcm.setMaxOutputTokens(snap.maxOutputTokens);\n\t\t\tif (snap.reasoningBudget != null)\n\t\t\t\tcm.setReasoningBudget(snap.reasoningBudget);\n\t\t\tif (snap.reasoning != null)\n\t\t\t\tcm.setReasoning(Reasoning.valueOf(snap.reasoning));\n\t\t\tif (snap.cacheMode != null)\n\t\t\t\tcm.setCacheMode(CacheMode.valueOf(snap.cacheMode));\n\t\t\tif (snap.profile != null)\n\t\t\t\tcm.setProfile(AgentProfile.valueOf(snap.profile));\n\t\t\tif (snap.systemPrompt != null)\n\t\t\t\tcm.setSystemPrompt(snap.systemPrompt);\n\t\t\tif (snap.freeText != null)\n\t\t\t\tcm.setSystemFree(snap.freeText);\n\t\t\tif (snap.outputMode != null)\n\t\t\t\tcm.setOuputMode(OutputMode.valueOf(snap.outputMode));\n\t\t\tif (snap.inputModes != null)\n\t\t\t\tfor (InputMode mode : InputMode.values())\n\t\t\t\t\tif (mode.ordinal() < snap.inputModes.length)\n\t\t\t\t\t\tcm.setInputMode(mode, snap.inputModes[mode.ordinal()]);\n\t\t\tif (snap.enabledTools != null)\n\t\t\t\tcm.setEnabledTools(snap.enabledTools);\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t\tLOG.info(\"Unable to restore config\", e);\n\t\t} finally {\n\t\t\tactiveEditorKey = savedKey;\n\t\t\tcfg = savedKey == null ? defaultCfg : editorConfigs.getOrDefault(savedKey, defaultCfg);\n\t\t}\n\t}\n}\n"
```
/answer 0a99-14-13 allow
Control Request:
```yaml
id: 0a99-15-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetList.java
  node_type: ^(MethodDeclaration|FieldDeclaration)$
  reason: Methoden von PresetList lesen (Front Matter Handling)
```
/answer 0a99-15-0 allow
Control Request:
```yaml
id: 0a99-16-15
result:
  structuredContent:
    files:
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
```
/answer 0a99-16-15 allow
Control Request:
```yaml
id: 0a99-17-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
  node_type: ^(MethodDeclaration|FieldDeclaration|ImportDeclaration)$
  reason: "Preset-Persistenz (Front Matter, Schreiben/Laden) und Memento-Serialisierung im Detail lesen"
```
/answer 0a99-17-0 allow
Control Request:
```yaml
id: 0a99-18-17
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
      nodes:
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
            public static void writePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Shell shell) {
              FileDialog dialog = new FileDialog(shell, SWT.SAVE);
              dialog.setFilterPath(resolveDefaultSaveDir());
              dialog.setFilterExtensions(new String[] { "*.prompt.txt" });
              dialog.setFilterNames(new String[] { "Prompt Files (*prompt.txt)" });
              String filePath = dialog.open();
              if (filePath == null)
                return;
              String content = serializePreset(body, tools, outputMode, inputs, "");
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
            public static String serializePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, String title) {
              FrontMatter fm = new FrontMatter();
              fm.title = title != null ? title : "";
              if (tools != null && tools.length > 0)
                fm.tools = Arrays.asList(tools);
              if (outputMode != null)
                fm.outputMode = outputMode.name();
              if (inputs != null && inputs.length > 0)
                fm.inputs = Arrays.asList(inputs);
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
      nodes:
      - id: MementoConverter.saveConfig
        type: MethodDeclaration
        code: "\tpublic static void saveConfig(IMemento memento, SessionConfig cfg) {\n\t\tSnapshot snap = new Snapshot();\n\t\tsnap.keys = cfg.keys;\n\t\tsnap.maxOutputTokens = cfg.maxOutputTokens;\n\t\tsnap.temperature = cfg.temperature;\n\t\tsnap.topP = cfg.topP;\n\t\tsnap.freeText = cfg.freeText;\n\t\tif (cfg.model != null) {\n\t\t\tsnap.modelProvider = cfg.model.cap.getKeyPattern().name();\n\t\t\tsnap.modelDisplayName = cfg.model.displayName;\n\t\t}\n\t\tsnap.reasoning = cfg.reasoning != null ? cfg.reasoning.name() : null;\n\t\tsnap.cacheMode = cfg.cacheMode != null ? cfg.cacheMode.name() : null;\n\t\tsnap.profile = cfg.profile != null ? cfg.profile.name() : null;\n\t\tsnap.reasoningBudget = cfg.reasoningBudget;\n\t\tsnap.systemPrompt = cfg.systemPrompt;\n\t\tsnap.outputMode = cfg.ouputMode != null ? cfg.ouputMode.name() : null;\n\t\tsnap.inputModes = cfg.inputModes;\n\t\tsnap.enabledTools = cfg.getEnabledTools();\n\n\t\ttry {\n\t\t\tmemento.createChild(\"cfg\").putString(\"json\", MAPPER.writeValueAsString(snap));\n\t\t} catch (JsonProcessingException e) {\n\t\t\tLOG.error(\"Unable to serialize session config\", e);\n\t\t}\n\t}\n"
      - id: MementoConverter.loadConfig
        type: MethodDeclaration
        code: "\tpublic static Snapshot loadConfig(IMemento memento) {\n\t\tif (memento == null)\n\t\t\treturn null;\n\t\tIMemento m = memento.getChild(\"cfg\");\n\t\tString json = m != null ? m.getString(\"json\") : null;\n\t\tif (json == null)\n\t\t\treturn null;\n\t\ttry {\n\t\t\treturn MAPPER.readValue(json, Snapshot.class);\n\t\t} catch (IOException e) {\n\t\t\tLOG.error(\"Unable to deserialize session config\", e);\n\t\t\treturn null;\n\t\t}\n\t}\n"
```
/answer 0a99-18-17 allow
ReasoningToken: 806
Control Request:
```yaml
id: 0a99-19-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    - /home/user/xyan/xy.ai.workbench/src
    pattern: FrozenConfig|\.approver\(\)|AutoApprovePanel\(
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    limit: 50
  reason: "Alle Stellen finden, an denen FrozenConfig erzeugt/genutzt wird und der Approver erreichbar ist"
```
/answer 0a99-19-0 allow
Control Request:
```yaml
id: 0a99-1a-19
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/conscious/connector/claudecode/CCSessionParameters.java
        matches:
        - import xy.ai.conscious.connector.harness.FrozenConfig;
        - FrozenConfig fc = FrozenConfig.from(cfg);
      - path: xy/ai/conscious/connector/claudecode/CCConnector.java
        matches:
        - import xy.ai.conscious.connector.harness.FrozenConfig;
        - FrozenConfig fc = req.config;
      - path: xy/ai/conscious/connector/claudecode/CCRequest.java
        matches:
        - import xy.ai.conscious.connector.harness.FrozenConfig;
        - public final FrozenConfig config;
        - "public CCRequest(String id, String title, FrozenConfig config, String promptText, PromptArguments arg) {"
      - path: xy/ai/conscious/connector/google/GeminiConnector.java
        matches:
        - import xy.ai.conscious.connector.harness.FrozenConfig;
        - FrozenConfig fc = prompt.config;
        - "private Integer getThinkingBudget(Reasoning reasoning, FrozenConfig fc) {"
      - path: xy/ai/conscious/connector/claude/ClaudeConnector.java
        matches:
        - import xy.ai.conscious.connector.harness.FrozenConfig;
        - FrozenConfig fc = prompt.config;
      - path: xy/ai/conscious/connector/openai/OpenAIConnector.java
        matches:
        - import xy.ai.conscious.connector.harness.FrozenConfig;
        - FrozenConfig fc = prompt.config;
      - path: xy/ai/conscious/connector/deepseek/DeepSeekConnector.java
        matches:
        - import xy.ai.conscious.connector.harness.FrozenConfig;
        - FrozenConfig fc = prompt.config;
      - path: xy/ai/conscious/connector/SessionParameters.java
        matches:
        - import xy.ai.conscious.connector.harness.FrozenConfig;
        - "public boolean equalsConfig(FrozenConfig cfg, Path cwd, String filePath, List<String> tools) {"
      - path: xy/ai/conscious/connector/harness/FrozenConfig.java
        matches:
        - "public class FrozenConfig {"
        - "private FrozenConfig(String keys, Model model, AgentProfile profile, Reasoning reasoning, List<String> tools, String systemPrompt, Double topP, Double temperature, Long maxOutputTokens, Integer reasoningBudget, CacheMode cacheMode, OutputMode outputMode) {"
        - "public FrozenConfig copy() {"
        - "return new FrozenConfig(keys, model, profile, reasoning, tools, systemPrompt, topP, temperature, maxOutputTokens, reasoningBudget, cacheMode, outputMode);"
        - "public static FrozenConfig from(ConfigManager cfg) {"
        - "return new FrozenConfig(keys, cfg.getModel(), cfg.getProfile(), cfg.getReasoning(), tools, systemPrompt, cfg.getTopP(), cfg.getTemperature(), cfg.getMaxOutputTokens(), cfg.getReasoningBudget(), cfg.getCacheMode(), cfg.getOuputMode());"
      - path: xy/ai/conscious/connector/harness/PromptInputHandler.java
        matches:
        - FrozenConfig frozen = FrozenConfig.from(cfg);
      - path: xy/ai/conscious/connector/harness/Prompt.java
        matches:
        - public final FrozenConfig config;
        - "public Prompt(List<String> inputs, boolean batch, FrozenConfig config, PromptArguments arg) {"
      - path: xy/ai/workbench/view/approver/AutoApprovePanel.java
        matches:
        - approver = Activator.getDefault().promptHandler.approver();
```
/answer 0a99-1a-19 allow
Thinking:
Die Struktur ist erfasst: Rules haben schon `getDefaultSetting()`, aber `RuleSettings`, `FrozenConfig` und `ConfigManager` kennen noch keine Regel-Settings. Als Nächstes prüfe ich Tests, UpdateGuard und Mapper, bevor ich umsetze.


Control Request:
```yaml
id: 0a99-1b-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: new ApprovalContext\(|new (AllowedToolsRule|SmallResultRule|DeepPathReadRule|DenyRootPathRule|HoldMarkdownRule)\(|new RuleRegistry\(|implements IApprovalRule|new FrozenConfig|FrozenConfig\.copy|\.copy\(\)
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: new ApprovalContext\(|new (AllowedToolsRule|SmallResultRule|DeepPathReadRule|DenyRootPathRule|HoldMarkdownRule)\(|new RuleRegistry\(|implements IApprovalRule|new FrozenConfig|FrozenConfig\.copy|\.copy\(\)
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
    pattern: instructionSection|InstructionSection|class UpdateGuard|new PresetsTab|MAPPER =|YAML =|FRONT_MATTER =
    include:
    - '*.java'
    limit: 20
  reason: "Prüfen, ob Tests/andere Stellen Konstruktoren und ApprovalContext nutzen, und wo InstructionSection/YAML-Mapper verdrahtet sind"
```
/answer 0a99-1b-0 allow
ReasoningToken: 8308
Control Request:
```yaml
id: 0a99-1c-1b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src
      matches:
      - path: xy/ai/conscious/autoapprove/RuleRegistry.java
        matches:
        - register(new AllowedToolsRule());
        - register(new SmallResultRule());
        - register(new DeepPathReadRule());
        - register(new DenyRootPathRule());
        - register(new HoldMarkdownRule());
      - path: xy/ai/conscious/autoapprove/AutoApprover.java
        matches:
        - this(new RuleRegistry());
        - "ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, call, session);"
      - path: xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
        matches:
        - "public class DenyRootPathRule implements IApprovalRule {"
      - path: xy/ai/conscious/autoapprove/rules/SmallResultRule.java
        matches:
        - "public class SmallResultRule implements IApprovalRule {"
      - path: xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
        matches:
        - "public class AllowedToolsRule implements IApprovalRule {"
      - path: xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
        matches:
        - "public class HoldMarkdownRule implements IApprovalRule {"
      - path: xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
        matches:
        - "public class DeepPathReadRule implements IApprovalRule {"
      - path: xy/ai/conscious/connector/harness/FrozenConfig.java
        matches:
        - "return new FrozenConfig(keys, model, profile, reasoning, tools, systemPrompt, topP, temperature, maxOutputTokens, reasoningBudget, cacheMode, outputMode);"
        - "return new FrozenConfig(keys, cfg.getModel(), cfg.getProfile(), cfg.getReasoning(), tools, systemPrompt, cfg.getTopP(), cfg.getTemperature(), cfg.getMaxOutputTokens(), cfg.getReasoningBudget(), cfg.getCacheMode(), cfg.getOuputMode());"
      - path: xy/ai/conscious/connector/harness/PromptInputHandler.java
        matches:
        - "return new Prompt(arg.inputs, origin.batch, origin.config.copy(), arg);"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
      matches:
      - path: PresetHandler.java
        matches:
        - "private static final Pattern FRONT_MATTER = Pattern.compile(\"\\\\A---\\\\s*\\\\n(.*?\\\\n)?---\\\\s*\\\\n?\", Pattern.DOTALL);"
        - private static final ObjectMapper YAML = new ObjectMapper(new YAMLFactory().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER));
      - path: session/InstructionSection.java
        matches:
        - "public class InstructionSection {"
        - private final PresetsTab presetsTab = new PresetsTab();
        - "public InstructionSection(FormToolkit toolkit, Runnable reflow) {"
      - path: session/UpdateGuard.java
        matches:
        - "public class UpdateGuard {"
      - path: AISessionView.java
        matches:
        - import xy.ai.workbench.view.session.InstructionSection;
        - "new InstructionSection(toolkit, () -> form.reflow(true)).create(body, cfg, inputStats, getSite().getShell());"
```
/answer 0a99-1c-1b allow
Control Request:
```yaml
id: 0a99-1d-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/UpdateGuard.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputSelection.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
  node_type: ^(ClassOrInterfaceDeclaration|RecordDeclaration)$
  reason: UpdateGuard-API und Vote-API lesen
```
/answer 0a99-1d-0 allow
Control Request:
```yaml
id: 0a99-1e-1d
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/UpdateGuard.java
      nodes:
      - id: UpdateGuard
        type: ClassOrInterfaceDeclaration
        code: "public class UpdateGuard {\n\n\tprivate boolean updating = false;\n\n\tpublic boolean isUpdating() {\n\t\treturn updating;\n\t}\n\n\tpublic void run(Runnable action) {\n\t\ttry {\n\t\t\tupdating = true;\n\t\t\taction.run();\n\t\t} finally {\n\t\t\tupdating = false;\n\t\t}\n\t}\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InputSelection.java
      nodes:
      - id: InputSelection
        type: ClassOrInterfaceDeclaration
        code: "public interface InputSelection extends Clearable {\n\tpublic String[] getSelection();\n\n\tpublic void setSelection(String[] items);\n}\n"
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/Vote.java
      nodes:
      - id: Vote
        type: RecordDeclaration
        code: |
          public record Vote(Verdict verdict, String reason, String hint, List<String> warnings, String instruction) {
            private static final Vote ABSTAIN = new Vote(Verdict.Abstain, null, null, null, null);
            public Vote {
              verdict = verdict == null ? Verdict.Abstain : verdict;
              reason = reason == null ? "" : reason;
              hint = hint == null ? "" : hint;
              warnings = warnings == null ? List.of() : List.copyOf(warnings);
              instruction = instruction == null ? "" : instruction;
            }

            public static Vote abstain() {
              return ABSTAIN;
            }

            public static Vote allow() {
              return allow(null);
            }

            public static Vote allow(String hint) {
              return new Vote(Verdict.Allow, null, hint, null, null);
            }

            public static Vote deny(String reason) {
              return new Vote(Verdict.Deny, reason, null, null, null);
            }

            public static Vote hold() {
              return new Vote(Verdict.Hold, null, null, null, null);
            }

            /**
             * Revokes automatic approval and adds a warning to the answer.
             */
            public static Vote hold(String warning) {
              return new Vote(Verdict.Hold, null, null, List.of(warning), null);
            }

            /**
             * Abstains but adds a warning to the answer.
             */
            public static Vote warning(String text) {
              return new Vote(Verdict.Abstain, null, null, List.of(text), null);
            }

            /**
             * Abstains but requests a follow-up prompt with the given instruction.
             */
            public static Vote instruction(String text) {
              return new Vote(Verdict.Abstain, null, null, null, text);
            }

            public Vote withWarning(String text) {
              List<String> all = new ArrayList<>(warnings);
              all.add(text);
              return new Vote(verdict, reason, hint, all, instruction);
            }
          }
```
/answer 0a99-1e-1d allow
Text:
 Jetzt setze ich um. Zuerst `RuleSettings` (typisierte Container, Clone, Diff/Overlay, Export/Import).
ReasoningToken: 126
Control Request:
```yaml
id: 0a99-1f-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
    overwrite: true
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.util.ArrayList;\nimport java.util.Collections;\nimport java.util.LinkedHashMap;\nimport java.util.List;\nimport java.util.Map;\nimport java.util.Objects;\nimport java.util.Set;\nimport java.util.TreeMap;\nimport java.util.stream.Collectors;\n\n/**\n * Typed, named settings of one rule. Every value lives in a {@link Setting}\n * container so an editor can render the settings generically by type.\n * <p>\n * A rule exposes its defaults (by clone) through\n * {@link IApprovalRule#getDefaultSetting()}. Per-prompt overrides are held as\n * {@link #diff(RuleSettings) diff} against the defaults and resolved with\n * {@link #overlay(RuleSettings)}.\n */\npublic class RuleSettings {\n\n\tpublic static final String ACTIVE = \"active\";\n\n\tprivate final Map<String, Setting> settings = new LinkedHashMap<>();\n\n\t/** Creates settings containing only the {@value #ACTIVE} flag (enabled). */\n\tpublic RuleSettings() {\n\t\tadd(new BooleanSetting(ACTIVE, true));\n\t}\n\n\tprivate RuleSettings(boolean empty) {\n\t}\n\n\t/** Settings without any entry; used for sparse overrides. */\n\tpublic static RuleSettings empty() {\n\t\treturn new RuleSettings(true);\n\t}\n\n\tpublic RuleSettings add(Setting setting) {\n\t\tsettings.put(setting.getName(), setting);\n\t\treturn this;\n\t}\n\n\tpublic RuleSettings addBoolean(String name, boolean value) {\n\t\treturn add(new BooleanSetting(name, value));\n\t}\n\n\tpublic RuleSettings addNumber(String name, long value) {\n\t\treturn add(new NumberSetting(name, value));\n\t}\n\n\tpublic RuleSettings addStrings(String name, Iterable<String> values) {\n\t\treturn add(new StringListSetting(name, values));\n\t}\n\n\tpublic RuleSettings addStrings(String name, String... values) {\n\t\treturn add(new StringListSetting(name, List.of(values)));\n\t}\n\n\t/** Unmodifiable view, setting name to container. */\n\tpublic Map<String, Setting> settings() {\n\t\treturn Collections.unmodifiableMap(settings);\n\t}\n\n\tpublic boolean isEmpty() {\n\t\treturn settings.isEmpty();\n\t}\n\n\t/** Absent flag counts as active. */\n\tpublic boolean isActive() {\n\t\tSetting s = settings.get(ACTIVE);\n\t\treturn !(s instanceof BooleanSetting b) || b.get();\n\t}\n\n\tpublic long getNumber(String name) {\n\t\treturn settings.get(name) instanceof NumberSetting n ? n.get() : 0;\n\t}\n\n\tpublic boolean getBoolean(String name) {\n\t\treturn settings.get(name) instanceof BooleanSetting b && b.get();\n\t}\n\n\tpublic List<String> getStrings(String name) {\n\t\treturn settings.get(name) instanceof StringListSetting l ? l.get() : List.of();\n\t}\n\n\tpublic Set<String> getStringSet(String name) {\n\t\treturn Set.copyOf(getStrings(name));\n\t}\n\n\t/** Deep copy. */\n\t@Override\n\tpublic RuleSettings clone() {\n\t\tRuleSettings copy = new RuleSettings(true);\n\t\tfor (Setting s : settings.values())\n\t\t\tcopy.settings.put(s.getName(), s.copy());\n\t\treturn copy;\n\t}\n\n\t/**\n\t * Copy of these settings where every entry with a same-named, same-typed entry\n\t * in {@code override} takes over its value. Entries only present in\n\t * {@code override} are ignored.\n\t */\n\tpublic RuleSettings overlay(RuleSettings override) {\n\t\tRuleSettings result = clone();\n\t\tif (override == null)\n\t\t\treturn result;\n\t\tfor (Setting o : override.settings.values()) {\n\t\t\tSetting own = result.settings.get(o.getName());\n\t\t\tif (own != null && own.getClass() == o.getClass())\n\t\t\t\tresult.settings.put(own.getName(), o.copy());\n\t\t}\n\t\treturn result;\n\t}\n\n\t/** Only the entries that differ from {@code base} (or are missing there). */\n\tpublic RuleSettings diff(RuleSettings base) {\n\t\tRuleSettings result = new RuleSettings(true);\n\t\tfor (Setting s : settings.values())\n\t\t\tif (base == null || !s.equals(base.settings.get(s.getName())))\n\t\t\t\tresult.settings.put(s.getName(), s.copy());\n\t\treturn result;\n\t}\n\n\t/** Plain values (Boolean, Long, List of String) by setting name. */\n\tpublic Map<String, Object> toValues() {\n\t\tMap<String, Object> out = new TreeMap<>();\n\t\tfor (Setting s : settings.values())\n\t\t\tout.put(s.getName(), s.getValue());\n\t\treturn out;\n\t}\n\n\t/**\n\t * Inverse of {@link #toValues()}; the setting type is derived from the value\n\t * kind. Unsupported values are skipped.\n\t */\n\tpublic static RuleSettings fromValues(Map<String, ?> values) {\n\t\tRuleSettings result = new RuleSettings(true);\n\t\tif (values == null)\n\t\t\treturn result;\n\t\tfor (Map.Entry<String, ?> e : values.entrySet()) {\n\t\t\tObject v = e.getValue();\n\t\t\tif (v instanceof Boolean b)\n\t\t\t\tresult.add(new BooleanSetting(e.getKey(), b));\n\t\t\telse if (v instanceof Number n)\n\t\t\t\tresult.add(new NumberSetting(e.getKey(), n.longValue()));\n\t\t\telse if (v instanceof List<?> l)\n\t\t\t\tresult.add(new StringListSetting(e.getKey(), l.stream().map(String::valueOf).collect(Collectors.toList())));\n\t\t}\n\t\treturn result;\n\t}\n\n\tpublic static Map<String, Map<String, Object>> toValues(Map<String, RuleSettings> byRule) {\n\t\tMap<String, Map<String, Object>> out = new TreeMap<>();\n\t\tif (byRule != null)\n\t\t\tbyRule.forEach((id, rs) -> out.put(id, rs.toValues()));\n\t\treturn out;\n\t}\n\n\tpublic static Map<String, RuleSettings> fromValues(Map<String, ? extends Map<String, ?>> byRule, boolean byRuleMarker) {\n\t\tMap<String, RuleSettings> out = new LinkedHashMap<>();\n\t\tif (byRule != null)\n\t\t\tbyRule.forEach((id, values) -> out.put(id, fromValues(values)));\n\t\treturn out;\n\t}\n\n\t/** Deep copy of a settings map. */\n\tpublic static Map<String, RuleSettings> copyOf(Map<String, RuleSettings> byRule) {\n\t\tMap<String, RuleSettings> out = new LinkedHashMap<>();\n\t\tif (byRule != null)\n\t\t\tbyRule.forEach((id, rs) -> out.put(id, rs.clone()));\n\t\treturn out;\n\t}\n\n\t@Override\n\tpublic boolean equals(Object obj) {\n\t\treturn obj instanceof RuleSettings other && settings.equals(other.settings);\n\t}\n\n\t@Override\n\tpublic int hashCode() {\n\t\treturn settings.hashCode();\n\t}\n\n\t@Override\n\tpublic String toString() {\n\t\treturn settings.values().toString();\n\t}\n\n\t/** Named container of one typed value. */\n\tpublic static abstract class Setting {\n\t\tprivate final String name;\n\n\t\tprotected Setting(String name) {\n\t\t\tthis.name = Objects.requireNonNull(name);\n\t\t}\n\n\t\tpublic String getName() {\n\t\t\treturn name;\n\t\t}\n\n\t\tpublic abstract Setting copy();\n\n\t\t/** Plain value: Boolean, Long or List of String. */\n\t\tpublic abstract Object getValue();\n\n\t\t/** Tolerant assignment of a plain value; incompatible values are ignored. */\n\t\tpublic abstract void setValue(Object value);\n\n\t\t@Override\n\t\tpublic boolean equals(Object obj) {\n\t\t\treturn obj != null && obj.getClass() == getClass() && ((Setting) obj).name.equals(name) && ((Setting) obj).getValue().equals(getValue());\n\t\t}\n\n\t\t@Override\n\t\tpublic int hashCode() {\n\t\t\treturn Objects.hash(name, getValue());\n\t\t}\n\n\t\t@Override\n\t\tpublic String toString() {\n\t\t\treturn name + \"=\" + getValue();\n\t\t}\n\t}\n\n\tpublic static class BooleanSetting extends Setting {\n\t\tprivate boolean value;\n\n\t\tpublic BooleanSetting(String name, boolean value) {\n\t\t\tsuper(name);\n\t\t\tthis.value = value;\n\t\t}\n\n\t\tpublic boolean get() {\n\t\t\treturn value;\n\t\t}\n\n\t\tpublic void set(boolean value) {\n\t\t\tthis.value = value;\n\t\t}\n\n\t\t@Override\n\t\tpublic Setting copy() {\n\t\t\treturn new BooleanSetting(getName(), value);\n\t\t}\n\n\t\t@Override\n\t\tpublic Object getValue() {\n\t\t\treturn value;\n\t\t}\n\n\t\t@Override\n\t\tpublic void setValue(Object v) {\n\t\t\tif (v instanceof Boolean b)\n\t\t\t\tvalue = b;\n\t\t}\n\t}\n\n\tpublic static class NumberSetting extends Setting {\n\t\tprivate long value;\n\n\t\tpublic NumberSetting(String name, long value) {\n\t\t\tsuper(name);\n\t\t\tthis.value = value;\n\t\t}\n\n\t\tpublic long get() {\n\t\t\treturn value;\n\t\t}\n\n\t\tpublic void set(long value) {\n\t\t\tthis.value = value;\n\t\t}\n\n\t\t@Override\n\t\tpublic Setting copy() {\n\t\t\treturn new NumberSetting(getName(), value);\n\t\t}\n\n\t\t@Override\n\t\tpublic Object getValue() {\n\t\t\treturn value;\n\t\t}\n\n\t\t@Override\n\t\tpublic void setValue(Object v) {\n\t\t\tif (v instanceof Number n)\n\t\t\t\tvalue = n.longValue();\n\t\t}\n\t}\n\n\tpublic static class StringListSetting extends Setting {\n\t\tprivate List<String> values;\n\n\t\tpublic StringListSetting(String name, Iterable<String> values) {\n\t\t\tsuper(name);\n\t\t\tthis.values = new ArrayList<>();\n\t\t\tvalues.forEach(this.values::add);\n\t\t}\n\n\t\t/** Unmodifiable view. */\n\t\tpublic List<String> get() {\n\t\t\treturn Collections.unmodifiableList(values);\n\t\t}\n\n\t\tpublic void set(List<String> values) {\n\t\t\tthis.values = new ArrayList<>(values);\n\t\t}\n\n\t\t@Override\n\t\tpublic Setting copy() {\n\t\t\treturn new StringListSetting(getName(), values);\n\t\t}\n\n\t\t@Override\n\t\tpublic Object getValue() {\n\t\t\treturn List.copyOf(values);\n\t\t}\n\n\t\t@Override\n\t\tpublic void setValue(Object v) {\n\t\t\tif (v instanceof List<?> l)\n\t\t\t\tvalues = l.stream().map(String::valueOf).collect(Collectors.toCollection(ArrayList::new));\n\t\t}\n\t}\n}\n"
  reason: "RuleSettings mit typisierten Setting-Containern (Boolean/Number/StringList), Clone, Diff/Overlay und Import/Export implementieren"
```
/answer 0a99-1f-0 allow
Text:
 Die Überladung mit dem Dummy-Boolean ist hässlich. Ich benenne die Map-Varianten sauber um.
ReasoningToken: 121
Control Request:
```yaml
id: 0a99-20-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
    id: RuleSettings.toValues_2
    old_text: "public static Map<String, Map<String, Object>> toValues(Map<String, RuleSettings> byRule) {"
    new_text: "public static Map<String, Map<String, Object>> exportAll(Map<String, RuleSettings> byRule) {"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
    id: RuleSettings.fromValues_2
    old_text: "public static Map<String, RuleSettings> fromValues(Map<String, ? extends Map<String, ?>> byRule, boolean byRuleMarker) {"
    new_text: "public static Map<String, RuleSettings> importAll(Map<String, ? extends Map<String, ?>> byRule) {"
  reason: Überladene Map-Varianten eindeutig benennen (exportAll/importAll)
```
/answer 0a99-20-0 allow
Control Request:
```yaml
id: 0a99-21-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    id: ApprovalContext
    old_text: "ApprovalItem call,\n\t\tSessionSnapshot session) {"
    new_text: "ApprovalItem call,\n\t\tSessionSnapshot session, RuleRegistry registry) {"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    id: ApprovalContext
    old_text: "public boolean hasItem() {\n\t\treturn item != null;\n\t}"
    new_text: "public boolean hasItem() {\n\t\treturn item != null;\n\t}\n\n\t/**\n\t * Effective settings of the rule: its defaults, overlaid with the per-prompt\n\t * override if the prompt carries one. Also used by rules that depend on the\n\t * settings of another rule.\n\t */\n\tpublic RuleSettings settings(String ruleId) {\n\t\tRuleSettings defaults = registry.defaultSettings(ruleId);\n\t\tRuleSettings override = prompt == null ? null : prompt.config.ruleSettings.get(ruleId);\n\t\treturn defaults.overlay(override);\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: RuleRegistry.rules
    old_text: |-
      public List<IApprovalRule> rules() {
          return rules;
        }
    new_text: |-
      public List<IApprovalRule> rules() {
          return rules;
        }

        public Optional<IApprovalRule> find(String id) {
          return rules.stream().filter(r -> r.id().equals(id)).findFirst();
        }

        /**
         * Default settings of the rule as an independent clone; empty settings for an
         * unknown id.
         */
        public RuleSettings defaultSettings(String id) {
          return find(id).map(r -> r.getDefaultSetting().clone()).orElseGet(RuleSettings::new);
        }
  reason: ApprovalContext um Registry und settings(ruleId) erweitern; RuleRegistry um find/defaultSettings
```
/answer 0a99-21-0 allow
ReasoningToken: 313
Control Request:
```yaml
id: 0a99-22-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  node_type: ImportDeclaration
  reason: "Imports prüfen, um fehlende Imports (RuleSettings, Optional) zu ergänzen"
```
/answer 0a99-22-0 allow
Control Request:
```yaml
id: 0a99-23-22
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
      nodes: []
```
/answer 0a99-23-22 allow
ReasoningToken: 62
Control Request:
```yaml
id: 0a99-24-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
    pattern: ^(import|package)
    include:
    - ApprovalContext.java
    - RuleRegistry.java
    - AutoApprover.java
    intent: editing
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules
    pattern: ^(import|package)
    include:
    - '*Rule.java'
    intent: editing
    limit: 50
  reason: "Import-Zeilen mit Adressierungsinfo holen, um Imports gezielt zu ergänzen"
```
/answer 0a99-24-0 allow
Control Request:
```yaml
id: 0a99-25-24
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
      matches:
      - path: RuleRegistry.java
        nodes:
        - id: 3sbhBT|4uo6iX
          matches:
          - import java.util.List;
          - import xy.ai.conscious.autoapprove.rules.HoldMarkdownRule;
          - import java.util.Objects;
          - import java.util.concurrent.CopyOnWriteArrayList;
          - import xy.ai.conscious.autoapprove.rules.AllowedToolsRule;
          - import xy.ai.conscious.autoapprove.rules.DeepPathReadRule;
          - import xy.ai.conscious.autoapprove.rules.DenyRootPathRule;
          - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
          - import xy.ai.conscious.autoapprove.rules.SmallResultRule;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.autoapprove;
      - path: AutoApprover.java
        nodes:
        - id: Zpzs7d|sxLd1L
          matches:
          - import java.util.ArrayList;
          - import java.util.List;
          - import java.util.Optional;
          - import java.util.concurrent.CopyOnWriteArrayList;
          - import java.util.function.Consumer;
          - import xy.ai.conscious.api.IActiveEditor.Selection;
          - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
          - import xy.ai.conscious.autoapprove.rules.IApprovalRule;
          - import xy.ai.conscious.autoapprove.rules.Vote;
          - import xy.ai.conscious.commands.AnswerCommand;
          - import xy.ai.conscious.commands.CallCommand;
          - import xy.ai.conscious.connector.harness.Prompt;
          - import xy.ai.conscious.core.LOG;
          - import xy.ai.conscious.model.AIAnswer;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.autoapprove;
      - path: ApprovalContext.java
        nodes:
        - id: GwlVvM|lj1u2H
          matches:
          - import xy.ai.conscious.connector.harness.Prompt;
          - import xy.ai.conscious.model.AIAnswer;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.autoapprove;
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules
      matches:
      - path: DenyRootPathRule.java
        nodes:
        - id: 09A9xJ|Ot0X0x
          matches:
          - import java.nio.file.Path;
          - import java.util.Set;
          - import xy.ai.conscious.autoapprove.ApprovalContext;
          - import xy.ai.conscious.autoapprove.ApprovalItem;
          - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.autoapprove.rules;
      - path: IApprovalRule.java
        nodes:
        - id: H4HWeP|XqIL50
          matches:
          - import xy.ai.conscious.autoapprove.ApprovalContext;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.autoapprove.rules;
      - path: SmallResultRule.java
        nodes:
        - id: z2FhVL|v3y3zx
          matches:
          - import java.util.Set;
          - import com.fasterxml.jackson.databind.JsonNode;
          - import xy.ai.conscious.autoapprove.ApprovalContext;
          - import xy.ai.conscious.autoapprove.ApprovalItem;
          - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.autoapprove.rules;
      - path: AllowedToolsRule.java
        nodes:
        - id: z2FhVL|9B9F9F
          matches:
          - import java.util.Set;
          - import xy.ai.conscious.autoapprove.ApprovalContext;
          - import xy.ai.conscious.autoapprove.ApprovalItem;
          - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.autoapprove.rules;
      - path: HoldMarkdownRule.java
        nodes:
        - id: xDPc5p|pJjrWN
          matches:
          - import java.util.Locale;
          - import java.util.Set;
          - import xy.ai.conscious.autoapprove.ApprovalContext;
          - import xy.ai.conscious.autoapprove.ApprovalItem;
          - import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.autoapprove.rules;
      - path: DeepPathReadRule.java
        nodes:
        - id: AKHURJ|uCLMlL
          matches:
          - import java.nio.file.Path;
          - import java.util.List;
          - import java.util.Set;
          - import xy.ai.conscious.autoapprove.ApprovalContext;
          - import xy.ai.conscious.autoapprove.ApprovalItem;
          - import com.fasterxml.jackson.databind.JsonNode;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.autoapprove.rules;
```
/answer 0a99-25-24 allow
Text:
 Jetzt die fünf Regeln: Defaults per Clone exponieren, Werte zur Laufzeit aus dem Kontext lesen, abhängige Regeln verweisen auf die Settings der Bezugsregel.
ReasoningToken: 1463
Control Request:
```yaml
id: 0a99-26-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
    overwrite: true
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.util.Set;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\npublic class AllowedToolsRule implements IApprovalRule {\n\n\tpublic static final String ID = AllowedToolsRule.class.getSimpleName();\n\tpublic static final String TOOLS = \"tools\";\n\tpublic static final String[] ALLOWED_TOOLS = { \"ast_outline\", \"ast_read\", \"ast_validate\", \"run_command\", \"ast_find\" };\n\n\tprivate final RuleSettings defaults;\n\n\tpublic AllowedToolsRule() {\n\t\tthis(ALLOWED_TOOLS);\n\t}\n\n\tpublic AllowedToolsRule(String... toolNames) {\n\t\tthis.defaults = new RuleSettings().addStrings(TOOLS, toolNames);\n\t}\n\n\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n\n\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tSet<String> allowed = ctx.settings(ID).getStringSet(TOOLS);\n\t\treturn allowed.contains(item.tool()) ? Vote.allow() : Vote.abstain();\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
    overwrite: true
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.util.Set;\n\nimport com.fasterxml.jackson.databind.JsonNode;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\n/**\n * Allows small results of the tools allowed by {@link AllowedToolsRule}.\n */\npublic class SmallResultRule implements IApprovalRule {\n\n\tpublic static final String ID = SmallResultRule.class.getSimpleName();\n\tpublic static final String MAX_LENGTH_SETTING = \"maxLength\";\n\tpublic static final int MAX_LENGTH = 3000;\n\n\tprivate final RuleSettings defaults;\n\n\tpublic SmallResultRule() {\n\t\tthis(MAX_LENGTH);\n\t}\n\n\tpublic SmallResultRule(long maxLength) {\n\t\tthis.defaults = new RuleSettings().addNumber(MAX_LENGTH_SETTING, maxLength);\n\t}\n\n\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n\n\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tApprovalItem call = ctx.call();\n\t\tif (item == null || call == null || call.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlResult && item.kind() != Kind.ToolResult)\n\t\t\treturn Vote.abstain();\n\t\tSet<String> allowed = ctx.settings(AllowedToolsRule.ID).getStringSet(AllowedToolsRule.TOOLS);\n\t\tif (!allowed.contains(call.tool()))\n\t\t\treturn Vote.abstain();\n\t\treturn length(item) < ctx.settings(ID).getNumber(MAX_LENGTH_SETTING) ? Vote.allow() : Vote.abstain();\n\t}\n\n\tprivate static int length(ApprovalItem item) {\n\t\tJsonNode result = item.result();\n\t\tif (result.isMissingNode())\n\t\t\treturn item.yaml().length();\n\t\treturn result.isTextual() ? result.asText().length() : result.toString().length();\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
    overwrite: true
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.nio.file.Path;\nimport java.util.List;\nimport java.util.Set;\n\nimport com.fasterxml.jackson.databind.JsonNode;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\n\npublic class DeepPathReadRule implements IApprovalRule {\n\n\tpublic static final String ID = DeepPathReadRule.class.getSimpleName();\n\tpublic static final String TOOLS_SETTING = \"tools\";\n\tpublic static final String PATH_FIELDS_SETTING = \"pathFields\";\n\tpublic static final String MIN_DEPTH_SETTING = \"minDepth\";\n\tpublic static final String MAX_LENGTH_SETTING = \"maxLength\";\n\n\tpublic static final String[] TOOLS = { \"search\", \"list\" };\n\tpublic static final String[] PATH_FIELDS = { \"path\", \"directory\" };\n\tpublic static final int MIN_DEPTH = 3;\n\tpublic static final int MAX_LENGTH = 3000;\n\n\tprivate final RuleSettings defaults;\n\tprivate final Path cwd;\n\n\tpublic DeepPathReadRule() {\n\t\tthis(Set.of(TOOLS), Set.of(PATH_FIELDS), MIN_DEPTH, Path.of(\"\").toAbsolutePath().normalize());\n\t}\n\n\tpublic DeepPathReadRule(Set<String> tools, Set<String> pathFields, int minDepth, Path cwd) {\n\t\tthis.defaults = new RuleSettings() //\n\t\t\t\t.addStrings(TOOLS_SETTING, tools.stream().sorted().toList()) //\n\t\t\t\t.addStrings(PATH_FIELDS_SETTING, pathFields.stream().sorted().toList()) //\n\t\t\t\t.addNumber(MIN_DEPTH_SETTING, minDepth) //\n\t\t\t\t.addNumber(MAX_LENGTH_SETTING, MAX_LENGTH);\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n\n\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n\n\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null)\n\t\t\treturn Vote.abstain();\n\t\tRuleSettings settings = ctx.settings(ID);\n\t\tSet<String> tools = settings.getStringSet(TOOLS_SETTING);\n\t\tswitch (item.kind()) {\n\t\tcase ControlCall, ToolCall:\n\t\t\treturn item.tool() != null && tools.contains(item.tool()) && allDeep(item, settings) ? Vote.allow() : Vote.abstain();\n\t\tcase ControlResult, ToolResult:\n\t\t\tApprovalItem call = ctx.call();\n\t\t\tif (call == null || call.tool() == null || !tools.contains(call.tool()) || !allDeep(call, settings))\n\t\t\t\treturn Vote.abstain();\n\t\t\treturn length(item) < settings.getNumber(MAX_LENGTH_SETTING) ? Vote.allow() : Vote.abstain();\n\t\tdefault:\n\t\t\treturn Vote.abstain();\n\t\t}\n\t}\n\n\tprivate boolean allDeep(ApprovalItem call, RuleSettings settings) {\n\t\tList<String> values = PathArguments.collect(call.arguments(), settings.getStringSet(PATH_FIELDS_SETTING));\n\t\tif (values.isEmpty())\n\t\t\treturn false;\n\t\tlong minDepth = settings.getNumber(MIN_DEPTH_SETTING);\n\t\tfor (String value : values) {\n\t\t\tPath path = PathArguments.resolve(cwd, value);\n\t\t\tif (path == null || !path.startsWith(cwd) || cwd.relativize(path).getNameCount() < minDepth)\n\t\t\t\treturn false;\n\t\t}\n\t\treturn true;\n\t}\n\n\tprivate static int length(ApprovalItem item) {\n\t\tJsonNode result = item.result();\n\t\tif (result.isMissingNode())\n\t\t\treturn item.yaml().length();\n\t\treturn result.isTextual() ? result.asText().length() : result.toString().length();\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
    overwrite: true
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.nio.file.Path;\nimport java.util.Set;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\n/**\n * Denies the root directory for the tools and path fields of\n * {@link DeepPathReadRule}.\n */\npublic class DenyRootPathRule implements IApprovalRule {\n\n\tpublic static final String ID = DenyRootPathRule.class.getSimpleName();\n\tpublic static final String REASON = \"Don't use the root directory.\";\n\n\tprivate final RuleSettings defaults = new RuleSettings();\n\tprivate final Path cwd;\n\n\tpublic DenyRootPathRule() {\n\t\tthis(Path.of(\"\").toAbsolutePath().normalize());\n\t}\n\n\tpublic DenyRootPathRule(Path cwd) {\n\t\tthis.cwd = cwd.toAbsolutePath().normalize();\n\t}\n\n\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n\n\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tRuleSettings deep = ctx.settings(DeepPathReadRule.ID);\n\t\tif (!deep.getStringSet(DeepPathReadRule.TOOLS_SETTING).contains(item.tool()))\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tSet<String> pathFields = deep.getStringSet(DeepPathReadRule.PATH_FIELDS_SETTING);\n\t\tfor (String value : PathArguments.collect(item.arguments(), pathFields)) {\n\t\t\tPath path = PathArguments.resolve(cwd, value);\n\t\t\tif (path != null && PathArguments.isRoot(path))\n\t\t\t\treturn Vote.deny(REASON);\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
    overwrite: true
    source: "package xy.ai.conscious.autoapprove.rules;\n\nimport java.util.Locale;\nimport java.util.Set;\n\nimport xy.ai.conscious.autoapprove.ApprovalContext;\nimport xy.ai.conscious.autoapprove.ApprovalItem;\nimport xy.ai.conscious.autoapprove.ApprovalItem.Kind;\n\n/**\n * Holds the automatic approval for markdown access with the tools allowed by\n * {@link AllowedToolsRule}.\n */\npublic class HoldMarkdownRule implements IApprovalRule {\n\n\tpublic static final String ID = HoldMarkdownRule.class.getSimpleName();\n\tpublic static final String PATH_FIELDS_SETTING = \"pathFields\";\n\tpublic static final String PATTERN_FIELDS_SETTING = \"patternFields\";\n\n\tpublic static final String[] PATH_FIELDS = { \"path\", \"paths\" };\n\tpublic static final String[] PATTERN_FIELDS = { \"include\" };\n\n\tprivate static final String EXTENSION = \".md\";\n\n\tprivate final RuleSettings defaults;\n\n\tpublic HoldMarkdownRule() {\n\t\tthis(Set.of(PATH_FIELDS), Set.of(PATTERN_FIELDS));\n\t}\n\n\tpublic HoldMarkdownRule(Set<String> pathFields, Set<String> patternFields) {\n\t\tthis.defaults = new RuleSettings() //\n\t\t\t\t.addStrings(PATH_FIELDS_SETTING, pathFields.stream().sorted().toList()) //\n\t\t\t\t.addStrings(PATTERN_FIELDS_SETTING, patternFields.stream().sorted().toList());\n\t}\n\n\t@Override\n\tpublic String id() {\n\t\treturn ID;\n\t}\n\n\t@Override\n\tpublic RuleSettings getDefaultSetting() {\n\t\treturn defaults.clone();\n\t}\n\n\t@Override\n\tpublic Vote vote(ApprovalContext ctx) {\n\t\tApprovalItem item = ctx.item();\n\t\tif (item == null || item.tool() == null)\n\t\t\treturn Vote.abstain();\n\t\tif (!ctx.settings(AllowedToolsRule.ID).getStringSet(AllowedToolsRule.TOOLS).contains(item.tool()))\n\t\t\treturn Vote.abstain();\n\t\tif (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)\n\t\t\treturn Vote.abstain();\n\t\tRuleSettings settings = ctx.settings(ID);\n\t\tfor (String value : PathArguments.collect(item.arguments(), settings.getStringSet(PATH_FIELDS_SETTING))) {\n\t\t\tif (value.toLowerCase(Locale.ROOT).endsWith(EXTENSION))\n\t\t\t\treturn Vote.hold();\n\t\t}\n\t\tfor (String value : PathArguments.collect(item.arguments(), settings.getStringSet(PATTERN_FIELDS_SETTING))) {\n\t\t\tif (value.toLowerCase(Locale.ROOT).contains(EXTENSION))\n\t\t\t\treturn Vote.hold();\n\t\t}\n\t\treturn Vote.abstain();\n\t}\n}\n"
  reason: "Alle fünf Regeln auf Settings umstellen: Defaults per Clone exponieren, Werte aus dem ApprovalContext lesen, abhängige Regeln verweisen auf Bezugsregel"
```
/answer 0a99-26-0 allow
ReasoningToken: 204
Control Request:
```yaml
id: 0a99-27-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
    id: IApprovalRule
    old_text: RuleSettings getDefaultSetting();
    new_text: "/**\n\t * Defaults of this rule, always as an independent clone. Effective values are\n\t * read via {@code ApprovalContext.settings(id())}.\n\t */\n\tRuleSettings getDefaultSetting();"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
    id: GwlVvM|lj1u2H
    old_text: import xy.ai.conscious.connector.harness.Prompt;
    new_text: |-
      import xy.ai.conscious.autoapprove.rules.RuleSettings;
      import xy.ai.conscious.connector.harness.Prompt;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
    id: 3sbhBT|4uo6iX
    old_text: import java.util.Objects;
    new_text: |-
      import java.util.Objects;
      import java.util.Optional;
      import xy.ai.conscious.autoapprove.rules.RuleSettings;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover
    old_text: "new ApprovalContext(ans, ans.prompt, item, call, session);"
    new_text: "new ApprovalContext(ans, ans.prompt, item, call, session, registry);"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
    id: AutoApprover
    old_text: |-
      for (IApprovalRule rule : registry.rules()) {
            try {
    new_text: |-
      for (IApprovalRule rule : registry.rules()) {
            if (!ctx.settings(rule.id()).isActive())
              continue;
            try {
  reason: "Interface-Doku, Imports, Registry im Context und active-Flag im AutoApprover anwenden"
```
/answer 0a99-27-0 allow
Text:
 Jetzt der Datenpfad: `SessionConfig` → `ConfigManager` → `FrozenConfig`.
Control Request:
```yaml
id: 0a99-28-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/SessionConfig.java
    id: SessionConfig
    old_text: |
      public String[] enabledTools = new String[0];
    new_text: "public String[] enabledTools = new String[0];\n\t/** Rule id to settings that deviate from the rule defaults; absent means defaults. */\n\tpublic Map<String, RuleSettings> ruleSettings = new HashMap<>();\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/SessionConfig.java
    id: SessionConfig
    old_text: "this.enabledTools = other.enabledTools;\n\t}"
    new_text: "this.enabledTools = other.enabledTools;\n\t\tthis.ruleSettings = RuleSettings.copyOf(other.ruleSettings);\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/SessionConfig.java
    id: SessionConfig
    old_text: "public void setEnabledTools(String[] enabledTools) {\n\t\tthis.enabledTools = enabledTools;\n\t}"
    new_text: "public void setEnabledTools(String[] enabledTools) {\n\t\tthis.enabledTools = enabledTools;\n\t}\n\n\tpublic Map<String, RuleSettings> getRuleSettings() {\n\t\treturn ruleSettings;\n\t}\n\n\tpublic void setRuleSettings(Map<String, RuleSettings> ruleSettings) {\n\t\tthis.ruleSettings = ruleSettings;\n\t}"
  reason: SessionConfig um Regel-Settings (nur Abweichungen vom Default) erweitern
```
/answer 0a99-28-0 allow
ReasoningToken: 171
Control Request:
```yaml
id: 0a99-29-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
    pattern: ^(import|package)
    include:
    - SessionConfig.java
    - ConfigManager.java
    - FrozenConfig.java
    intent: editing
    limit: 80
  reason: "Import-Blöcke von SessionConfig, ConfigManager und FrozenConfig adressieren"
```
/answer 0a99-29-0 allow
Control Request:
```yaml
id: 0a99-2a-29
result:
  structuredContent:
    errors:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
      pattern: ^(import|package)
      error: limit must be between 1 and 50.
```
/answer 0a99-2a-29 allow
Control Request:
```yaml
id: 0a99-2b-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
    pattern: ^(import|package)
    include:
    - SessionConfig.java
    - ConfigManager.java
    - FrozenConfig.java
    intent: editing
    limit: 50
  reason: "Import-Blöcke von SessionConfig, ConfigManager und FrozenConfig adressieren"
```
/answer 0a99-2b-0 allow
Control Request:
```yaml
id: 0a99-2c-2b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
      matches:
      - path: core/SessionConfig.java
        nodes:
        - id: K9F4h4|pEAecA
          matches:
          - import xy.ai.conscious.model.CacheMode;
          - import xy.ai.conscious.model.Model;
          - import xy.ai.conscious.model.Reasoning;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.core;
      - path: core/ConfigManager.java
        nodes:
        - id: Ei2OXR|xqS3r3
          matches:
          - import java.util.ArrayList;
          - import java.util.Arrays;
          - import java.util.List;
          - import java.util.Objects;
          - import java.util.function.Consumer;
          - import java.util.stream.Collectors;
          - import xy.ai.conscious.api.ext.IFileCache;
          - import xy.ai.conscious.connector.mcp.MCPClient;
          - import xy.ai.conscious.model.CacheMode;
          - import xy.ai.conscious.model.Capabilities;
          - import xy.ai.conscious.model.Model;
          - import xy.ai.conscious.model.Reasoning;
          - import xy.ai.conscious.model.Model.KeyPattern;
          - import xy.ai.conscious.model.resolver.ModelResolverRegistry;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.core;
      - path: connector/harness/FrozenConfig.java
        nodes:
        - id: 0gUTBT|NhIzDR
          matches:
          - import java.util.Arrays;
          - import java.util.List;
          - import xy.ai.conscious.core.AgentProfile;
          - import xy.ai.conscious.core.ConfigManager;
          - import xy.ai.conscious.core.InputMode;
          - import xy.ai.conscious.core.OutputMode;
          - import xy.ai.conscious.model.CacheMode;
          - import xy.ai.conscious.model.Model;
          - import xy.ai.conscious.model.Reasoning;
          - import xy.ai.conscious.tools.Hash;
        matches:
        - lineno: 1
          match: package xy.ai.conscious.connector.harness;
```
/answer 0a99-2c-2b allow
ReasoningToken: 168
Control Request:
```yaml
id: 0a99-2d-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/SessionConfig.java
    id: K9F4h4|pEAecA
    old_text: import xy.ai.conscious.model.CacheMode;
    new_text: |-
      import java.util.HashMap;
      import java.util.Map;

      import xy.ai.conscious.autoapprove.rules.RuleSettings;
      import xy.ai.conscious.model.CacheMode;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
    id: Ei2OXR|xqS3r3
    old_text: import java.util.Objects;
    new_text: |-
      import java.util.Map;
      import java.util.Objects;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
    id: Ei2OXR|xqS3r3
    old_text: import xy.ai.conscious.api.ext.IFileCache;
    new_text: |-
      import xy.ai.conscious.api.ext.IFileCache;
      import xy.ai.conscious.autoapprove.rules.RuleSettings;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
    id: ConfigManager
    old_text: |
      private List<Consumer<String[]>> enabledToolsObs = new ArrayList<>();
    new_text: "private List<Consumer<String[]>> enabledToolsObs = new ArrayList<>();\n\tprivate List<Consumer<Map<String, RuleSettings>>> ruleSettingsObs = new ArrayList<>();\n"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
    id: ConfigManager
    old_text: "enabledToolsObs.clear();\n\t}"
    new_text: "enabledToolsObs.clear();\n\t\truleSettingsObs.clear();\n\t}"
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
    id: ConfigManager
    old_text: "outputModeObs.forEach(c -> c.accept(cfg.ouputMode));\n\t\tupdateEnabledModels("
    new_text: "outputModeObs.forEach(c -> c.accept(cfg.ouputMode));\n\t\truleSettingsObs.forEach(c -> c.accept(getRuleSettings()));\n\t\tupdateEnabledModels("
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
    id: ConfigManager
    old_text: "public String[] getEnabledTools() {\n\t\treturn cfg.getEnabledTools();\n\t}"
    new_text: "public String[] getEnabledTools() {\n\t\treturn cfg.getEnabledTools();\n\t}\n\n\t/**\n\t * Replaces the per-rule overrides. Callers pass only settings that deviate from\n\t * the rule defaults; rules without an entry use their defaults.\n\t */\n\tpublic void setRuleSettings(Map<String, RuleSettings> ruleSettings) {\n\t\tMap<String, RuleSettings> copy = RuleSettings.copyOf(ruleSettings);\n\t\tcopy.values().removeIf(RuleSettings::isEmpty);\n\t\tif (Objects.equals(cfg.getRuleSettings(), copy))\n\t\t\treturn;\n\t\tensureOwnConfig();\n\t\tcfg.setRuleSettings(copy);\n\t\truleSettingsObs.forEach(c -> c.accept(getRuleSettings()));\n\t}\n\n\t/** Deep copy of the per-rule overrides. */\n\tpublic Map<String, RuleSettings> getRuleSettings() {\n\t\treturn RuleSettings.copyOf(cfg.getRuleSettings());\n\t}\n\n\tpublic void addRuleSettingsObs(Consumer<Map<String, RuleSettings>> obs, boolean initialize) {\n\t\truleSettingsObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(getRuleSettings());\n\t}"
  reason: ConfigManager um Regel-Settings (Setter/Getter/Observer) und Imports erweitern
```
/answer 0a99-2d-0 allow
ReasoningToken: 64
Control Request:
```yaml
id: 0a99-2e-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
    id: 0gUTBT|NhIzDR
    old_text: import java.util.List;
    new_text: |-
      import java.util.List;
      import java.util.Map;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
    id: 0gUTBT|NhIzDR
    old_text: import xy.ai.conscious.core.AgentProfile;
    new_text: |-
      import xy.ai.conscious.autoapprove.rules.RuleSettings;
      import xy.ai.conscious.core.AgentProfile;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
    id: FrozenConfig
    old_text: |2-
        public final OutputMode outputMode;
        private String hash;
    new_text: |2-
        public final OutputMode outputMode;
        /**
         * Rule id to settings deviating from the rule defaults. Not part of the hash:
         * it only steers the evaluation of answers, not the session.
         */
        public final Map<String, RuleSettings> ruleSettings;
        private String hash;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
    id: FrozenConfig
    old_text: |-
      CacheMode cacheMode, OutputMode outputMode) {
          this.keys = keys;
    new_text: |-
      CacheMode cacheMode, OutputMode outputMode, Map<String, RuleSettings> ruleSettings) {
          this.keys = keys;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
    id: FrozenConfig
    old_text: |2-
          this.outputMode = outputMode;
        }
    new_text: |2-
          this.outputMode = outputMode;
          this.ruleSettings = Map.copyOf(RuleSettings.copyOf(ruleSettings));
        }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
    id: FrozenConfig
    old_text: |-
      reasoningBudget, cacheMode, outputMode);
        }

        /**
         * Reads
    new_text: |-
      reasoningBudget, cacheMode, outputMode, ruleSettings);
        }

        /**
         * Reads
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
    id: FrozenConfig
    old_text: "cfg.getCacheMode(), cfg.getOuputMode());"
    new_text: "cfg.getCacheMode(), cfg.getOuputMode(), cfg.getRuleSettings());"
  reason: "FrozenConfig um die abweichenden Regel-Settings erweitern (Konstruktor, copy, from)"
```
/answer 0a99-2e-0 allow
Text:
 Core ist durch. Nun Persistenz: Memento (Session-Config) und Preset-Front-Matter.
ReasoningToken: 248
Control Request:
```yaml
id: 0a99-2f-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: serializePreset|writePreset|parsePreset|PresetHandler\.Preset|MAPPER\s*=
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    limit: 30
    intent: editing
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: ^import (java\.util|xy\.ai\.conscious)
    include:
    - PresetHandler.java
    - MementoConverter.java
    - ConfigManagerAdapter.java
    intent: editing
    limit: 50
  reason: Aufrufer der Preset-Funktionen und Import-Blöcke der Persistenz-Klassen finden
```
/answer 0a99-2f-0 allow
Control Request:
```yaml
id: 0a99-30-2f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/MementoConverter.java
        nodes:
        - id: MementoConverter.ZN7FHA|6C0rPe
          matches:
          - private static final ObjectMapper MAPPER = new ObjectMapper();
      - path: xy/ai/workbench/view/PresetHandler.java
        nodes:
        - id: PresetHandler.loadPreset
          matches:
          - return parsePreset(content);
        - id: PresetHandler.parsePreset
          matches:
          - "public static Preset parsePreset(String content) {"
        - id: PresetHandler.writePreset
          matches:
          - "public static void writePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Shell shell) {"
          - "String content = serializePreset(body, tools, outputMode, inputs, \"\");"
        - id: PresetHandler.serializePreset
          matches:
          - "public static String serializePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, String title) {"
      - path: xy/ai/workbench/view/session/PresetsTab.java
        nodes:
        - id: PresetsTab.create
          matches:
          - "PresetHandler.writePreset(cfg.getSystemPrompt(), tools.getSelection(), cfg.getOuputMode(), inputs.getSelection(), shell);"
          - PresetHandler.Preset preset = PresetHandler.loadPreset(presetFiles.get(idx));
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: MementoConverter.java
        nodes:
        - id: BpLy9W|YrvRaJ
          matches:
          - import xy.ai.conscious.core.ConfigManager;
          - import xy.ai.conscious.core.LOG;
          - import xy.ai.conscious.core.SessionConfig;
      - path: view/PresetHandler.java
        nodes:
        - id: eXHs10|iFNY4b
          matches:
          - import java.util.ArrayList;
          - import java.util.Arrays;
          - import java.util.Comparator;
          - import java.util.List;
          - import java.util.regex.Matcher;
          - import java.util.regex.Pattern;
          - import xy.ai.conscious.core.LOG;
          - import xy.ai.conscious.core.OutputMode;
      - path: eclipse/ConfigManagerAdapter.java
        nodes:
        - id: FrqhCb|xsgHlk
          matches:
          - import java.util.Arrays;
          - import java.util.HashMap;
          - import java.util.Map;
          - import java.util.Objects;
          - import xy.ai.conscious.api.ext.IFileCache;
          - import xy.ai.conscious.connector.mcp.MCPClient;
          - import xy.ai.conscious.core.AgentProfile;
          - import xy.ai.conscious.core.ConfigManager;
          - import xy.ai.conscious.core.InputMode;
          - import xy.ai.conscious.core.LOG;
          - import xy.ai.conscious.model.CacheMode;
          - import xy.ai.conscious.model.Model;
          - import xy.ai.conscious.model.Reasoning;
          - import xy.ai.conscious.model.Model.KeyPattern;
          - import xy.ai.conscious.core.OutputMode;
          - import xy.ai.conscious.core.SessionConfig;
```
/answer 0a99-30-2f allow
ReasoningToken: 444
Control Request:
```yaml
id: 0a99-31-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: eXHs10|iFNY4b
    old_text: import java.util.List;
    new_text: |-
      import java.util.List;
      import java.util.Map;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.FrontMatter
    old_text: |2
          public List<String> inputs;
    new_text: |2
          public List<String> inputs;
          public Map<String, Map<String, Object>> rules;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.Preset
    old_text: |2
          public String[] inputs;
    new_text: |2
          public String[] inputs;
          /** Rule id to the setting values that deviate from the rule defaults; null if absent. */
          public Map<String, Map<String, Object>> rules;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.parsePreset
    old_text: "preset.inputs = fm.inputs.toArray(new String[0]);"
    new_text: |-
      preset.inputs = fm.inputs.toArray(new String[0]);
                if (fm.rules != null)
                  preset.rules = fm.rules;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.writePreset
    old_text: "OutputMode outputMode, String[] inputs, Shell shell) {"
    new_text: "OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, Shell shell) {"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.writePreset
    old_text: "serializePreset(body, tools, outputMode, inputs, \"\");"
    new_text: "serializePreset(body, tools, outputMode, inputs, rules, \"\");"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.serializePreset
    old_text: "OutputMode outputMode, String[] inputs, String title) {"
    new_text: "OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, String title) {"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
    id: PresetHandler.serializePreset
    old_text: fm.inputs = Arrays.asList(inputs);
    new_text: |-
      fm.inputs = Arrays.asList(inputs);
          if (rules != null && !rules.isEmpty())
            fm.rules = rules;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
    id: BpLy9W|YrvRaJ
    old_text: import xy.ai.conscious.core.ConfigManager;
    new_text: |-
      import java.util.Map;

      import xy.ai.conscious.autoapprove.rules.RuleSettings;
      import xy.ai.conscious.core.ConfigManager;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
    id: MementoConverter.Snapshot
    old_text: |
      public String[] enabledTools;
    new_text: "public String[] enabledTools;\n\t\tpublic Map<String, Map<String, Object>> ruleSettings;\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
    id: MementoConverter.saveConfig
    old_text: |
      snap.enabledTools = cfg.getEnabledTools();
    new_text: "snap.enabledTools = cfg.getEnabledTools();\n\t\tsnap.ruleSettings = RuleSettings.exportAll(cfg.getRuleSettings());\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
    id: ConfigManagerAdapter.loadConfig
    old_text: cm.setEnabledTools(snap.enabledTools);
    new_text: "cm.setEnabledTools(snap.enabledTools);\n\t\t\tif (snap.ruleSettings != null)\n\t\t\t\tcm.setRuleSettings(RuleSettings.importAll(snap.ruleSettings));"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
    id: FrqhCb|xsgHlk
    old_text: import xy.ai.conscious.api.ext.IFileCache;
    new_text: |-
      import xy.ai.conscious.api.ext.IFileCache;
      import xy.ai.conscious.autoapprove.rules.RuleSettings;
  reason: Regel-Settings im Preset-Front-Matter und im Memento persistieren
```
/answer 0a99-31-0 allow
ReasoningToken: 3364
Control Request:
```yaml
id: 0a99-32-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: ^import .*(Activator|swt\.custom|swt\.layout|swt\.events)|promptHandler
    include:
    - AutoApprovePanel.java
    - Activator.java
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
    pattern: ^import
    include:
    - InstructionSection.java
    - PresetsTab.java
    intent: editing
    limit: 50
  reason: Activator-Zugriff auf PromptHandler und Import-Blöcke von InstructionSection/PresetsTab klären
```
/answer 0a99-32-0 allow
Control Request:
```yaml
id: 0a99-33-32
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: view/approver/AutoApprovePanel.java
        matches:
        - import org.eclipse.swt.custom.StyledText;
        - import xy.ai.workbench.Activator;
        - approver = Activator.getDefault().promptHandler.approver();
      - path: Activator.java
        matches:
        - public PromptHandler promptHandler;
        - "promptHandler = new PromptHandler(cma.getManager(), editorListener, connector, batch, editorInterface,"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
      matches:
      - path: InstructionSection.java
        nodes:
        - id: WofJxN|EvsjZp
          matches:
          - import org.eclipse.swt.SWT;
          - import org.eclipse.swt.layout.GridData;
          - import org.eclipse.swt.layout.GridLayout;
          - import org.eclipse.swt.widgets.Composite;
          - import org.eclipse.swt.widgets.Shell;
          - import org.eclipse.swt.widgets.TabFolder;
          - import org.eclipse.ui.forms.widgets.FormToolkit;
          - import xy.ai.conscious.core.ConfigManager;
          - import xy.ai.workbench.view.InputStats;
      - path: PresetsTab.java
        nodes:
        - id: 1vWmR7|l8ZbXg
          matches:
          - import java.util.ArrayList;
          - import org.eclipse.core.resources.IFile;
          - import org.eclipse.swt.SWT;
          - import org.eclipse.swt.events.MouseListener;
          - import org.eclipse.swt.events.SelectionAdapter;
          - import org.eclipse.swt.events.SelectionEvent;
          - import org.eclipse.swt.events.SelectionListener;
          - import org.eclipse.swt.layout.GridData;
          - import org.eclipse.swt.layout.GridLayout;
          - import org.eclipse.swt.widgets.Button;
          - import org.eclipse.swt.widgets.Composite;
          - import org.eclipse.swt.widgets.List;
          - import org.eclipse.swt.widgets.Shell;
          - import org.eclipse.swt.widgets.TabFolder;
          - import org.eclipse.swt.widgets.TabItem;
          - import xy.ai.conscious.core.ConfigManager;
          - import xy.ai.conscious.core.OutputMode;
          - import xy.ai.workbench.view.PresetHandler;
```
/answer 0a99-33-32 allow
Text:
 Jetzt der neue Rules-Tab inkl. dynamischem Settings-Editor.
ReasoningToken: 65
Control Request:
```yaml
id: 0a99-34-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RuleSelection.java
    source: "package xy.ai.workbench.view.session;\n\nimport java.util.Map;\n\npublic interface RuleSelection extends Clearable {\n\n\t/** Rule id to the setting values that deviate from the rule defaults. */\n\tpublic Map<String, Map<String, Object>> getSelection();\n\n\t/** Replaces all rule overrides; rules without an entry fall back to their defaults. */\n\tpublic void setSelection(Map<String, Map<String, Object>> rules);\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    source: "package xy.ai.workbench.view.session;\n\nimport java.util.ArrayList;\nimport java.util.Comparator;\nimport java.util.LinkedHashMap;\nimport java.util.Map;\nimport java.util.Optional;\nimport java.util.TreeMap;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.custom.SashForm;\nimport org.eclipse.swt.custom.ScrolledComposite;\nimport org.eclipse.swt.events.FocusListener;\nimport org.eclipse.swt.events.MouseListener;\nimport org.eclipse.swt.events.SelectionListener;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Button;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Control;\nimport org.eclipse.swt.widgets.Label;\nimport org.eclipse.swt.widgets.List;\nimport org.eclipse.swt.widgets.Spinner;\nimport org.eclipse.swt.widgets.TabFolder;\nimport org.eclipse.swt.widgets.TabItem;\nimport org.eclipse.swt.widgets.Text;\n\nimport xy.ai.conscious.autoapprove.RuleRegistry;\nimport xy.ai.conscious.autoapprove.rules.IApprovalRule;\nimport xy.ai.conscious.autoapprove.rules.RuleSettings;\nimport xy.ai.conscious.autoapprove.rules.RuleSettings.BooleanSetting;\nimport xy.ai.conscious.autoapprove.rules.RuleSettings.NumberSetting;\nimport xy.ai.conscious.autoapprove.rules.RuleSettings.Setting;\nimport xy.ai.conscious.autoapprove.rules.RuleSettings.StringListSetting;\nimport xy.ai.conscious.core.ConfigManager;\n\n/**\n * Lists the approval rules and renders the settings of the selected rule\n * dynamically by setting type. Only deviations from the rule defaults are\n * written to the {@link ConfigManager}.\n */\npublic class RulesTab implements RuleSelection {\n\tprivate final UpdateGuard guard = new UpdateGuard();\n\tprivate final java.util.List<IApprovalRule> rules = new ArrayList<>();\n\tprivate ConfigManager cfg;\n\tprivate RuleRegistry registry;\n\tprivate List ruleList;\n\tprivate ScrolledComposite scroller;\n\tprivate Composite editor;\n\n\tpublic void create(TabFolder instr, ConfigManager cfg, RuleRegistry registry) {\n\t\tthis.cfg = cfg;\n\t\tthis.registry = registry;\n\t\tTabItem item = new TabItem(instr, SWT.NONE);\n\t\titem.setText(\"Rules\");\n\t\tSashForm sash = new SashForm(instr, SWT.VERTICAL);\n\t\titem.setControl(sash);\n\n\t\truleList = new List(sash, SWT.SINGLE | SWT.V_SCROLL | SWT.BORDER);\n\t\truleList.addMouseListener(MouseListener.mouseDownAdapter(m -> ruleList.setFocus()));\n\t\truleList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> showSelected()));\n\n\t\tscroller = new ScrolledComposite(sash, SWT.V_SCROLL | SWT.H_SCROLL | SWT.BORDER);\n\t\tscroller.setExpandHorizontal(true);\n\t\tscroller.setExpandVertical(true);\n\t\teditor = new Composite(scroller, SWT.NONE);\n\t\teditor.setLayout(new GridLayout(2, false));\n\t\tscroller.setContent(editor);\n\t\tsash.setWeights(1, 2);\n\n\t\trules.addAll(registry.rules());\n\t\trules.sort(Comparator.comparing(IApprovalRule::id, String.CASE_INSENSITIVE_ORDER));\n\t\truleList.setItems(rules.stream().map(IApprovalRule::id).toArray(String[]::new));\n\t\tif (!rules.isEmpty()) {\n\t\t\truleList.select(0);\n\t\t\tshowSelected();\n\t\t}\n\t\tcfg.addRuleSettingsObs(overrides -> {\n\t\t\tif (!guard.isUpdating())\n\t\t\t\tshowSelected();\n\t\t}, false);\n\t}\n\n\t@Override\n\tpublic Map<String, Map<String, Object>> getSelection() {\n\t\tMap<String, Map<String, Object>> out = new TreeMap<>();\n\t\tfor (Map.Entry<String, RuleSettings> e : cfg.getRuleSettings().entrySet()) {\n\t\t\tOptional<IApprovalRule> rule = registry.find(e.getKey());\n\t\t\tif (rule.isEmpty())\n\t\t\t\tcontinue;\n\t\t\tRuleSettings diff = e.getValue().diff(rule.get().getDefaultSetting());\n\t\t\tif (!diff.isEmpty())\n\t\t\t\tout.put(e.getKey(), diff.toValues());\n\t\t}\n\t\treturn out;\n\t}\n\n\t@Override\n\tpublic void setSelection(Map<String, Map<String, Object>> selection) {\n\t\tMap<String, RuleSettings> overrides = new LinkedHashMap<>();\n\t\tfor (Map.Entry<String, Map<String, Object>> e : selection.entrySet()) {\n\t\t\tOptional<IApprovalRule> rule = registry.find(e.getKey());\n\t\t\tif (rule.isEmpty())\n\t\t\t\tcontinue;\n\t\t\tRuleSettings defaults = rule.get().getDefaultSetting();\n\t\t\tRuleSettings diff = defaults.overlay(RuleSettings.fromValues(e.getValue())).diff(defaults);\n\t\t\tif (!diff.isEmpty())\n\t\t\t\toverrides.put(e.getKey(), diff);\n\t\t}\n\t\tcfg.setRuleSettings(overrides);\n\t}\n\n\t@Override\n\tpublic void clear() {\n\t\tcfg.setRuleSettings(Map.of());\n\t}\n\n\tprivate void showSelected() {\n\t\tfor (Control child : editor.getChildren()) child.dispose();\n\t\tint idx = ruleList.getSelectionIndex();\n\t\tif (idx >= 0 && idx < rules.size()) {\n\t\t\tIApprovalRule rule = rules.get(idx);\n\t\t\tRuleSettings working = rule.getDefaultSetting().overlay(cfg.getRuleSettings().get(rule.id()));\n\t\t\tfor (Setting setting : working.settings().values()) createRow(rule, working, setting);\n\t\t}\n\t\teditor.layout(true, true);\n\t\tscroller.setMinSize(editor.computeSize(SWT.DEFAULT, SWT.DEFAULT));\n\t}\n\n\tprivate void createRow(IApprovalRule rule, RuleSettings working, Setting setting) {\n\t\tif (setting instanceof BooleanSetting flag) {\n\t\t\tButton check = new Button(editor, SWT.CHECK);\n\t\t\tcheck.setText(flag.getName());\n\t\t\tcheck.setSelection(flag.get());\n\t\t\tcheck.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false, 2, 1));\n\t\t\tcheck.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\t\tflag.set(check.getSelection());\n\t\t\t\tcommit(rule, working);\n\t\t\t}));\n\t\t} else if (setting instanceof NumberSetting number) {\n\t\t\tnew Label(editor, SWT.NONE).setText(number.getName());\n\t\t\tSpinner spinner = new Spinner(editor, SWT.BORDER);\n\t\t\tspinner.setMinimum(0);\n\t\t\tspinner.setMaximum(Integer.MAX_VALUE);\n\t\t\tspinner.setSelection((int) Math.max(0, Math.min(number.get(), Integer.MAX_VALUE)));\n\t\t\tspinner.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));\n\t\t\tspinner.addModifyListener(e -> {\n\t\t\t\tnumber.set(spinner.getSelection());\n\t\t\t\tcommit(rule, working);\n\t\t\t});\n\t\t} else if (setting instanceof StringListSetting list) {\n\t\t\tLabel label = new Label(editor, SWT.NONE);\n\t\t\tlabel.setText(list.getName());\n\t\t\tlabel.setLayoutData(new GridData(SWT.LEFT, SWT.TOP, false, false));\n\t\t\tText text = new Text(editor, SWT.MULTI | SWT.BORDER | SWT.V_SCROLL);\n\t\t\ttext.setText(String.join(text.getLineDelimiter(), list.get()));\n\t\t\tGridData data = new GridData(SWT.FILL, SWT.FILL, true, false);\n\t\t\tdata.widthHint = 1;\n\t\t\tdata.heightHint = 60;\n\t\t\ttext.setLayoutData(data);\n\t\t\ttext.addMouseListener(MouseListener.mouseDownAdapter(m -> text.setFocus()));\n\t\t\ttext.addFocusListener(FocusListener.focusLostAdapter(e -> {\n\t\t\t\tlist.set(text.getText().lines().map(String::strip).filter(s -> !s.isEmpty()).toList());\n\t\t\t\tcommit(rule, working);\n\t\t\t}));\n\t\t}\n\t}\n\n\t/** Stores only the deviations from the rule defaults; no entry remains for pure defaults. */\n\tprivate void commit(IApprovalRule rule, RuleSettings working) {\n\t\tMap<String, RuleSettings> all = cfg.getRuleSettings();\n\t\tRuleSettings diff = working.diff(rule.getDefaultSetting());\n\t\tif (diff.isEmpty())\n\t\t\tall.remove(rule.id());\n\t\telse\n\t\t\tall.put(rule.id(), diff);\n\t\tguard.run(() -> cfg.setRuleSettings(all));\n\t}\n}\n"
  reason: "RuleSelection-Interface und RulesTab (Sash: sortierte Regelliste oben, dynamischer Settings-Editor unten) anlegen"
```
/answer 0a99-34-0 allow
Text:
 Ein Randfall: Beim Dispose eines fokussierten Textfelds kann `focusLost` feuern und veraltete Werte committen. Das sichere ich über den Guard ab.
ReasoningToken: 620
Control Request:
```yaml
id: 0a99-35-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.showSelected
    old_text: "for (Control child : editor.getChildren()) child.dispose();"
    new_text: "guard.run(() -> {\n\t\t\tfor (Control child : editor.getChildren()) child.dispose();\n\t\t});"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.commit
    old_text: MapSettings_placeholder
    new_text: x
  reason: Dispose des Editors gegen stale focusLost-Commits absichern
```
/answer 0a99-35-0 allow
Control Request:
```yaml
id: 0a99-36-35
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
      result: success
      id: RulesTab.showSelected
      new_id: RulesTab.showSelected
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
      error: Text not found in node.
      id: RulesTab.commit
      reason: not_found
      next_step: reread_node
```
/answer 0a99-36-35 allow
Text:
 Der zweite Eintrag war ein Platzhalter-Fehler von mir. Hier die eigentliche Änderung an `commit`:
ReasoningToken: 32
Control Request:
```yaml
id: 0a99-37-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.commit
    old_text: "Map<String, RuleSettings> all = cfg.getRuleSettings();"
    new_text: "if (guard.isUpdating())\n\t\t\treturn;\n\t\tMap<String, RuleSettings> all = cfg.getRuleSettings();"
  reason: commit während Editor-Neuaufbau ignorieren
```
/answer 0a99-37-0 allow
Text:
 Jetzt `InstructionSection` und `PresetsTab` verdrahten (Tab-Reihenfolge, Save/Load/Clear All).
Control Request:
```yaml
id: 0a99-38-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    id: InstructionSection
    old_text: private final PresetsTab presetsTab = new PresetsTab();
    new_text: |-
      private final RulesTab rulesTab = new RulesTab();
        private final PresetsTab presetsTab = new PresetsTab();
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    id: InstructionSection
    old_text: |
      toolsTab.create(instr, cfg);
    new_text: |
      toolsTab.create(instr, cfg);
          rulesTab.create(instr, cfg, Activator.getDefault().promptHandler.approver().rules());
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    id: InstructionSection
    old_text: "toolsTab, selectTab, inputsTable);"
    new_text: "toolsTab, selectTab, inputsTable, rulesTab);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
    id: WofJxN|EvsjZp
    old_text: import xy.ai.conscious.core.ConfigManager;
    new_text: |-
      import xy.ai.conscious.core.ConfigManager;
      import xy.ai.workbench.Activator;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.create
    old_text: "Clearable instructionSelection, InputSelection inputs) {"
    new_text: "Clearable instructionSelection, InputSelection inputs, RuleSelection rules) {"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.create
    old_text: "inputs.getSelection(), shell);"
    new_text: "inputs.getSelection(), rules.getSelection(), shell);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.create
    old_text: |-
      inputs.clear();
              cfg.setOuputMode(OutputMode.values()[0]);
    new_text: |-
      inputs.clear();
              rules.clear();
              cfg.setOuputMode(OutputMode.values()[0]);
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
    id: PresetsTab.create
    old_text: inputs.setSelection(preset.inputs);
    new_text: |-
      inputs.setSelection(preset.inputs);
            if (preset.rules != null)
              rules.setSelection(preset.rules);
  reason: Rules-Tab zwischen Tools und Presets einhängen; Presets Save/Load/Clear All um Regel-Settings erweitern
```
/answer 0a99-38-0 allow
Control Request:
```yaml
id: 0a99-39-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/RuleSettings.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/rules/IApprovalRule.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/ApprovalContext.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/RuleRegistry.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/SessionConfig.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/core/ConfigManager.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/connector/harness/FrozenConfig.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/PresetHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/MementoConverter.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/ConfigManagerAdapter.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RuleSelection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/InstructionSection.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/PresetsTab.java
  reason: Alle geänderten/neuen Dateien auf Syntaxfehler prüfen
```
/answer 0a99-39-0 allow
ReasoningToken: 256
Control Request:
```yaml
id: 0a99-3a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: Bundle-RequiredExecutionEnvironment|JavaSE-|release>|maven.compiler
    include:
    - MANIFEST.MF
    - pom.xml
    - '*.prefs'
    exclude:
    - '**/target/**'
    - '**/bin/**'
    limit: 10
  reason: "Java-Version prüfen (Pattern-Matching-instanceof, Stream.toList, Records erfordern Java 16+)"
```
/answer 0a99-3a-0 deny Java 25
Text:
 Java 25, passt. Eine OSGi-Frage bleibt: Der Workbench-Plugin importiert jetzt erstmals `...autoapprove.rules.*`. Ich prüfe, ob das Package exportiert wird.
ReasoningToken: 117
Control Request:
```yaml
id: 0a99-3b-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: xy\.ai\.conscious\.autoapprove
    include:
    - MANIFEST.MF
    - build.properties
    - '*.bnd'
    exclude:
    - '**/target/**'
    - '**/bin/**'
    limit: 10
    intent: editing
  reason: "Prüfen, ob das Package autoapprove.rules im Bundle-Manifest exportiert wird"
```
/answer 0a99-3b-0 deny nicht notwendig, rules waren zuvor bereits verfügbar
Result Stats: id=7f9f0cdd-4630-492a-aae4-84f897630aab, total: 173797, in: 58, out: 53141, reason: 0, read: 2124619, write: 120598, start: 1794258511972, end: 1794258523970, time: 11916, total: 326257, io: 0, control: 82, sent: 0, recv: 321
Verstanden, das Export-Thema ist damit erledigt. Die Umsetzung ist vollständig, und alle 20 geänderten oder neuen Dateien bestehen die Syntaxprüfung. Kompiliert und in der IDE getestet habe ich nicht, `ast_validate` prüft nur die Syntax.

## Was jetzt gilt

**Settings-Modell (`RuleSettings`)**
- `RuleSettings` hält eine Map von Setting-Name zu typisiertem Container: `BooleanSetting`, `NumberSetting`, `StringListSetting`.
- Jede Instanz enthält ein `active`-Flag, Default ist aktiv.
- `clone()` erzeugt eine Deep Copy, `diff(base)` liefert nur die abweichenden Einträge, `overlay(override)` legt Overrides auf Defaults.
- Zum Persistieren gibt es `exportAll`/`importAll` und `toValues`/`fromValues`. Der Typ wird dabei aus dem Wert abgeleitet.

**Regeln**
- Alle fünf Regeln liefern ihre Defaults über `getDefaultSetting()` als Clone und lesen die Werte zur Laufzeit aus `ApprovalContext.settings(id)`.
- Die Defaults sind: `AllowedToolsRule` (`tools`), `SmallResultRule` (`maxLength`), `DeepPathReadRule` (`tools`, `pathFields`, `minDepth`, `maxLength`), `HoldMarkdownRule` (`pathFields`, `patternFields`) und `DenyRootPathRule` (nur `active`).
- Abhängige Regeln lesen die Settings ihrer Bezugsregel.
  - `SmallResultRule` und `HoldMarkdownRule` nutzen `AllowedToolsRule.tools`.
  - `DenyRootPathRule` nutzt `DeepPathReadRule.tools` und `pathFields`.
  - Fehlt ein Override, gelten die Defaults der Bezugsregel.
- `AutoApprover` überspringt Regeln mit `active=false`.
- `RuleRegistry` hat neu `find(id)` und `defaultSettings(id)`.

**Durchreichen an den Prompt**
- Der Weg ist `SessionConfig` → `ConfigManager` → `FrozenConfig`.
- Die Map enthält nur Abweichungen vom Default, ein Eintrag verschwindet wieder, sobald alles auf Default steht.
- `ConfigManager` bekommt `set/getRuleSettings` und `addRuleSettingsObs`.
- Die Settings sind bewusst nicht Teil von `FrozenConfig.getHash()`. Sie steuern nur die Auswertung der Antworten, und die Session-ID soll sich dadurch nicht ändern.
- Folge-Prompts erben sie über `copy()`.

**Persistenz**
- Presets bekommen im Front Matter einen Block `rules:` (Regel-ID → Setting-Werte), gespeichert werden nur die abweichenden Werte.
- Die Session-Config nimmt die Settings im Memento als `ruleSettings` mit auf.
- Das Laden eines Presets ersetzt die Regel-Settings nur, wenn das Preset einen `rules:`-Block hat. Das entspricht dem Verhalten bei `tools` und `inputs`.

**UI**
- Der Tab „Rules“ liegt zwischen Tools und Presets.
- Oben steht per Sash getrennt die alphabetisch sortierte Regelliste.
- Unten rendert der Editor die Settings nach Typ: Checkbox, Spinner oder mehrzeiliges Textfeld mit einem Eintrag pro Zeile.
- Der Editor zeigt initial die Defaults. Die erste Regel ist vorausgewählt.
- Änderungen gehen sofort an den `ConfigManager`. Das Textfeld übernimmt beim Fokusverlust.
- „Clear All“ im Presets-Tab setzt die Regel-Settings auf die Defaults zurück.

## Was du wissen solltest
- **Entfernte Konstruktoren:** Die Konstruktoren mit festen Tool-Sets sind weggefallen (`SmallResultRule(String...)`, `DenyRootPathRule(Set, Set, Path)`, `HoldMarkdownRule` mit Tools). Ich habe im Repo keine Aufrufer gefunden. Bei externem Code gibt es einen Compile-Fehler.
- **Registry ohne Bezugsregel:** Wird eine eigene `RuleRegistry` ohne `AllowedToolsRule` oder `DeepPathReadRule` gebaut, haben abhängige Regeln keine Tools mehr und enthalten sich. Vorher galten die statischen Konstanten.
- **Zahlenfelder:** Zahlen im Editor sind auf `0 … Integer.MAX_VALUE` begrenzt.
Okay, lass mich das Prüfen
Result Stats: id=7f9f0cdd-4630-492a-aae4-84f897630aab, total: 176009, in: 60, out: 53556, reason: 0, read: 2245217, write: 122393, start: 1794258746212, end: 1794258750193, time: 3926, total: 330183, io: 0, control: 55, sent: 1, recv: 94
SystemInit:  id=7f9f0cdd-4630-492a-aae4-84f897630aab, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Gern. Zum Testen in der IDE würde ich diese Punkte prüfen:

- **Rules-Tab:** Er sollte zwischen Tools und Presets stehen, mit der sortierten Regelliste oben und dem Editor unten.
- **Prompt-Auswertung:** Ändere z. B. `maxLength` bei `SmallResultRule` oder setze `active=false`. Die Änderung sollte beim nächsten Prompt greifen, und beim Zurücksetzen auf den Default sollte kein Override mehr existieren.
- **Preset-Roundtrip:** Speichere ein Preset mit geänderten Regeln. Im Front Matter sollte ein `rules:`-Block mit nur den abweichenden Werten stehen. Beim Laden sollte der Editor sie wieder anzeigen.
- **Clear All:** Danach sollten alle Regeln wieder auf Default stehen.
- **Editorwechsel und Neustart:** Die Settings sollten pro Editor erhalten bleiben (Memento).

Wenn etwas beim Kompilieren oder zur Laufzeit hakt, schick mir die Meldung. Dann kümmere ich mich darum.
Ich prüfe zuerst die Codeänderungen
Result Stats: id=7f9f0cdd-4630-492a-aae4-84f897630aab, total: 176801, in: 62, out: 53885, reason: 0, read: 2367610, write: 122854, start: 1794259007484, end: 1794259010503, time: 2967, total: 333150, io: 0, control: 52, sent: 1, recv: 70
SystemInit:  id=7f9f0cdd-4630-492a-aae4-84f897630aab, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Alles klar, nimm dir die Zeit. Zur Orientierung, wo der Kern liegt:

- **Datenmodell:** `rules/RuleSettings.java`. Hier sind `diff`, `overlay` und die Typen `Boolean`/`Number`/`StringList` am wichtigsten.
- **Auflösung der effektiven Settings:** `ApprovalContext.settings(id)` mit `RuleRegistry.defaultSettings(id)` und dem `active`-Check in `AutoApprover.collect`.
- **Durchreichen:** `SessionConfig`, `ConfigManager` (`setRuleSettings`) und `FrozenConfig`.
- **UI:** `RulesTab.java`. Die Logik für „nur Abweichungen speichern“ steckt in `commit`, `getSelection` und `setSelection`.

Wenn dir beim Review etwas auffällt, das anders sein soll, sag Bescheid.
Ok, codereview sieht gut aus, kommen wir zum testen.
Result Stats: id=7f9f0cdd-4630-492a-aae4-84f897630aab, total: 178126, in: 64, out: 54827, reason: 61, read: 2490464, write: 123235, start: 1794259202023, end: 1794259209454, time: 7361, total: 340511, io: 0, control: 70, sent: 1, recv: 187
SystemInit:  id=7f9f0cdd-4630-492a-aae4-84f897630aab, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 61
Ich kann die Workbench selbst nicht starten, du testest also in der IDE. Ich würde in dieser Reihenfolge vorgehen, weil die späteren Schritte auf den früheren aufbauen:

1. **Build:** Connector und Workbench kompilieren. Falls es hakt, schick mir die Fehlermeldung. Am ehesten wackelt etwas bei Imports oder der SWT-Version, zum Beispiel `MouseListener.mouseDownAdapter` und `SashForm.setWeights(int...)`. Das nutzen `PresetsTab` und `SelectTab` allerdings schon.
2. **Rules-Tab:** Er sollte zwischen Tools und Presets stehen, die Liste sollte sortiert sein und der Editor die Defaults zeigen. Das gilt auch für die vorausgewählte erste Regel.
3. **Override setzen:** Ändere z. B. `maxLength` bei `SmallResultRule` und wechsle zwischen Regeln hin und her. Der Wert sollte erhalten bleiben. Stelle ihn danach wieder auf den Default. Der Override sollte dann verschwinden, was du daran siehst, dass `getSelection` im Preset-Save nichts mehr schreibt.
4. **Wirkung im Prompt:** Setze `active=false` bei `AllowedToolsRule` und prüfe, ob die Auto-Approval für dessen Tools ausbleibt. Prüfe auch, dass die abhängigen Regeln (`SmallResultRule`, `HoldMarkdownRule`) dann trotzdem die Tool-Liste von `AllowedToolsRule` lesen. Ändere dazu `tools` dort und schau, ob sich die abhängigen Regeln anpassen.
5. **Preset-Roundtrip:** Speichere ein Preset mit geänderten Regeln. Im Front Matter sollte nur der `rules:`-Block mit den abweichenden Werten stehen. Lade es danach nach einem „Clear All“ wieder.
6. **Clear All:** Die Regeln sollten auf Default zurückspringen, und der sichtbare Editor sollte sich mit aktualisieren.
7. **Editorwechsel und Neustart:** Die Settings sollten pro Editor erhalten bleiben und einen Neustart überleben (Memento).

Wenn du bei einem Punkt ein unerwartetes Verhalten siehst, schick mir kurz den Schritt und das Ergebnis, dann gehe ich dem nach.
Die active Checkbox im Rules Tab zeichnet kein X. Scheint als wird sie nicht richtig gewählt.
SystemInit:  id=7f9f0cdd-4630-492a-aae4-84f897630aab, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich schaue zuerst, wie andere Checkboxen im Projekt erzeugt werden, ob dort etwas Besonderes nötig ist (z. B. FormToolkit).
ReasoningToken: 859
Control Request:
```yaml
id: 0a99-3c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: SWT\.CHECK|createButton\(
    include:
    - '*.java'
    exclude:
    - '**/bin/**'
    limit: 30
    intent: editing
  reason: "Vergleichen, wie bestehende Checkboxen erzeugt werden, um die fehlende Darstellung des Häkchens einzuordnen"
```
/answer 0a99-3c-0 allow
Control Request:
```yaml
id: 0a99-3d-3c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/session/InputsTable.java
        nodes:
        - id: InputsTable.create
          matches:
          - "Table table = new Table(middle, SWT.CHECK | SWT.BORDER | SWT.V_SCROLL);"
          - "if (e.detail == SWT.CHECK) {"
      - path: xy/ai/workbench/view/session/RulesTab.java
        nodes:
        - id: RulesTab.createRow
          matches:
          - "Button check = new Button(editor, SWT.CHECK);"
      - path: xy/ai/workbench/view/approver/AutoApprovePanel.java
        nodes:
        - id: AutoApprovePanel.fillMenu
          matches:
          - "MenuItem item = new MenuItem(menu, SWT.CHECK);"
      - path: xy/ai/workbench/view/SessionDetailDialog.java
        nodes:
        - id: SessionDetailDialog.createButtonsForButtonBar
          matches:
          - "createButton(parent, IDialogConstants.OK_ID, IDialogConstants.OK_LABEL, true);"
      - path: xy/ai/workbench/editor/outline/MarkdownOutlinePage.java
        nodes:
        - id: MarkdownOutlinePage.populate
          matches:
          - "MenuItem item = new MenuItem(menu, SWT.CHECK);"
```
/answer 0a99-3d-3c allow
ReasoningToken: 851
Control Request:
```yaml
id: 0a99-3e-0
toolName: ask_user
arguments:
  question: "Kurze Rückfrage zur Eingrenzung: (1) Ist die „active“-Checkbox direkt nach dem Öffnen des Rules-Tabs leer (Default wäre aktiv, also mit Häkchen)? (2) Wenn du sie anklickst: ändert sich optisch etwas (Fokusrahmen, Hover), und wirkt sich das Umschalten trotzdem aus, z. B. wird beim Wechsel auf eine andere Regel und zurück der Zustand korrekt wiederhergestellt? (3) Läuft die IDE mit dem Dark Theme?"
  reason: "Ursache für das fehlende Häkchen eingrenzen: Unterscheiden zwischen Darstellungsproblem (Theme/Native Checkbox) und Logikfehler (Selection wird nicht gesetzt/übernommen), bevor ich Code ändere"
```
/answer 0a99-3e-0 allow Es scheint die Border ändert sich und indiziert den state schlecht sichtbar, aber es gibt keinen Hacken. Wirkt vielleicht eher wie ein renderingfehler
Thinking:
Der Zustand schaltet korrekt, nur das Häkchen erscheint nicht, was wohl ein Rendering-Problem der nativen Checkbox ist. Ich ersetze sie jetzt durch einen Toggle-Button mit „On“/„Off“, da Push-Buttons bei dir bereits sauber rendern.


ReasoningToken: 1725
Control Request:
```yaml
id: 0a99-3f-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.createRow
    start_marker: Button check = new Button
    end_marker: "commit(rule, working);\n\t\t\t}));"
    exact: false
    content: "new Label(editor, SWT.NONE).setText(flag.getName());\n\t\t\tButton toggle = new Button(editor, SWT.TOGGLE);\n\t\t\ttoggle.setSelection(flag.get());\n\t\t\ttoggle.setText(flag.get() ? \"On\" : \"Off\");\n\t\t\tGridData data = new GridData(SWT.LEFT, SWT.CENTER, false, false);\n\t\t\tdata.widthHint = 60;\n\t\t\ttoggle.setLayoutData(data);\n\t\t\ttoggle.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\t\tflag.set(toggle.getSelection());\n\t\t\t\ttoggle.setText(flag.get() ? \"On\" : \"Off\");\n\t\t\t\tcommit(rule, working);\n\t\t\t}));"
  reason: Native Checkbox (rendert kein Häkchen) durch Toggle-Button mit On/Off-Beschriftung ersetzen
```
/answer 0a99-3f-0 allow Ich habe den Preset export probiert dort wird active korrekt in allen Zuständen gepeichert
Control Request:
```yaml
id: 0a99-40-3f
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
      error: End marker is ambiguous – found 2 occurrences in node.
      id: RulesTab.createRow
      reason: ambiguous
```
/answer 0a99-40-3f allow
Text:
 Gut: Das Speichern bestätigt, dass die Logik stimmt und nur das Zeichnen fehlt. Ich passe die Zeilen einzeln an, weil der Block-Marker mehrdeutig war.
ReasoningToken: 588
Control Request:
```yaml
id: 0a99-41-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.createRow
    old_text: "Button check = new Button(editor, SWT.CHECK);"
    new_text: "new Label(editor, SWT.NONE).setText(flag.getName());\n\t\t\tButton toggle = new Button(editor, SWT.TOGGLE);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.createRow
    old_text: check.setText(flag.getName());
    new_text: "toggle.setText(flag.get() ? \"On\" : \"Off\");"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.createRow
    old_text: check.setSelection(flag.get());
    new_text: toggle.setSelection(flag.get());
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.createRow
    old_text: "check.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false, 2, 1));"
    new_text: "GridData toggleData = new GridData(SWT.LEFT, SWT.CENTER, false, false);\n\t\t\ttoggleData.widthHint = 60;\n\t\t\ttoggle.setLayoutData(toggleData);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.createRow
    old_text: check.addSelectionListener(
    new_text: toggle.addSelectionListener(
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    id: RulesTab.createRow
    old_text: flag.set(check.getSelection());
    new_text: "flag.set(toggle.getSelection());\n\t\t\t\ttoggle.setText(flag.get() ? \"On\" : \"Off\");"
  reason: Boolean-Setting im Rules-Tab als Toggle-Button mit On/Off statt nativer Checkbox rendern
```
/answer 0a99-41-0 allow
Control Request:
```yaml
id: 0a99-42-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
    ids:
    - RulesTab.createRow
  reason: "Ergebnis der Einzeländerungen prüfen (Konsistenz, keine verbliebenen check-Referenzen)"
```
/answer 0a99-42-0 allow
Control Request:
```yaml
id: 0a99-43-42
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/RulesTab.java
      nodes:
      - id: RulesTab.createRow
        type: MethodDeclaration
        code: |2
            private void createRow(IApprovalRule rule, RuleSettings working, Setting setting) {
              if (setting instanceof BooleanSetting flag) {
                new Label(editor, SWT.NONE).setText(flag.getName());
                Button toggle = new Button(editor, SWT.TOGGLE);
                toggle.setText(flag.get() ? "On" : "Off");
                toggle.setSelection(flag.get());
                GridData toggleData = new GridData(SWT.LEFT, SWT.CENTER, false, false);
                toggleData.widthHint = 60;
                toggle.setLayoutData(toggleData);
                toggle.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {
                  flag.set(toggle.getSelection());
                  toggle.setText(flag.get() ? "On" : "Off");
                  commit(rule, working);
                }));
              } else if (setting instanceof NumberSetting number) {
                new Label(editor, SWT.NONE).setText(number.getName());
                Spinner spinner = new Spinner(editor, SWT.BORDER);
                spinner.setMinimum(0);
                spinner.setMaximum(Integer.MAX_VALUE);
                spinner.setSelection((int) Math.max(0, Math.min(number.get(), Integer.MAX_VALUE)));
                spinner.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
                spinner.addModifyListener(e -> {
                  number.set(spinner.getSelection());
                  commit(rule, working);
                });
              } else if (setting instanceof StringListSetting list) {
                Label label = new Label(editor, SWT.NONE);
                label.setText(list.getName());
                label.setLayoutData(new GridData(SWT.LEFT, SWT.TOP, false, false));
                Text text = new Text(editor, SWT.MULTI | SWT.BORDER | SWT.V_SCROLL);
                text.setText(String.join(text.getLineDelimiter(), list.get()));
                GridData data = new GridData(SWT.FILL, SWT.FILL, true, false);
                data.widthHint = 1;
                data.heightHint = 60;
                text.setLayoutData(data);
                text.addMouseListener(MouseListener.mouseDownAdapter(m -> text.setFocus()));
                text.addFocusListener(FocusListener.focusLostAdapter(e -> {
                  list.set(text.getText().lines().map(String::strip).filter(s -> !s.isEmpty()).toList());
                  commit(rule, working);
                }));
              }
            }
```
/answer 0a99-43-42 allow
Result Stats: id=7f9f0cdd-4630-492a-aae4-84f897630aab, total: 194983, in: 76, out: 61753, reason: 0, read: 3252815, write: 133154, start: 1794259509308, end: 1794259513034, time: 3670, total: 393559, io: 0, control: 56, sent: 0, recv: 91
Die „active“-Checkbox ist jetzt ein Toggle-Button mit „On“/„Off“ und Beschriftung davor, in derselben Zeile wie die anderen Settings. Ich habe es nicht in der IDE ausprobiert.

- **Ursache:** Nach deinen Beobachtungen ist es ein Rendering-Problem der nativen Checkbox. Die Border schaltet um und `active` wird in allen Zuständen korrekt ins Preset geschrieben, die Logik stimmt also. Warum das Häkchen fehlt, habe ich nicht herausgefunden. Das Theme ist nur eine Vermutung.
- **Workaround:** Push-Buttons rendern bei dir bereits korrekt (Presets-Tab). Deshalb habe ich den Toggle gewählt. Es betrifft nur `RulesTab.createRow`.
- **Zum Prüfen:** Der Toggle sollte nach dem Öffnen auf „On“ stehen, sich beim Klicken umschalten und beim Wechsel zwischen Regeln seinen Zustand behalten.
- **Alternative:** Wenn du lieber eine echte Checkbox willst, kann ich eine Variante über `FormToolkit.createButton` ausprobieren. Ob die das Häkchen zeichnet, weiß ich nicht.
