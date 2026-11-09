package xy.ai.conscious.autoapprove;

import xy.ai.conscious.autoapprove.rules.RuleSettings;
import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.model.AIAnswer;

public record ApprovalContext(AIAnswer answer, Prompt prompt, ApprovalItem item, ApprovalItem call, SessionSnapshot session, RuleRegistry registry) {

  public boolean hasItem() {
    return item != null;
  }

  /**
   * Effective settings of the rule: its defaults, overlaid with the per-prompt
   * override if the prompt carries one. Also used by rules that depend on the
   * settings of another rule.
   */
  public RuleSettings settings(String ruleId) {
    RuleSettings defaults = registry.defaultSettings(ruleId);
    RuleSettings override = prompt == null ? null : prompt.config.ruleSettings.get(ruleId);
    return defaults.overlay(override);
  }
}
