package xy.ai.conscious.connector.harness;

import java.util.Arrays;
import java.util.List;
import java.util.Map;
import xy.ai.conscious.autoapprove.rules.RuleSettings;
import xy.ai.conscious.core.AgentProfile;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.conscious.core.InputMode;
import xy.ai.conscious.core.OutputMode;
import xy.ai.conscious.model.CacheMode;
import xy.ai.conscious.model.Model;
import xy.ai.conscious.model.Reasoning;
import xy.ai.conscious.tools.Hash;

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
  /**
   * Rule id to settings deviating from the rule defaults. Not part of the hash:
   * it only steers the evaluation of answers, not the session.
   */
  public final Map<String, RuleSettings> ruleSettings;
  private String hash;

  private FrozenConfig(String keys, Model model, AgentProfile profile, Reasoning reasoning, List<String> tools, String systemPrompt, Double topP, Double temperature, Long maxOutputTokens, Integer reasoningBudget, CacheMode cacheMode, OutputMode outputMode, Map<String, RuleSettings> ruleSettings) {
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
    this.ruleSettings = Map.copyOf(RuleSettings.copyOf(ruleSettings));
  }

  /**
   * Independent copy; {@code model} and {@code keys} are mutated by prompt preprocessing.
   */
  public FrozenConfig copy() {
    return new FrozenConfig(keys, model, profile, reasoning, tools, systemPrompt, topP, temperature, maxOutputTokens, reasoningBudget, cacheMode, outputMode, ruleSettings);
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
    return new FrozenConfig(keys, cfg.getModel(), cfg.getProfile(), cfg.getReasoning(), tools, systemPrompt, cfg.getTopP(), cfg.getTemperature(), cfg.getMaxOutputTokens(), cfg.getReasoningBudget(), cfg.getCacheMode(), cfg.getOuputMode(), cfg.getRuleSettings());
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
