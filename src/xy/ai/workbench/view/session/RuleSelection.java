package xy.ai.workbench.view.session;

import java.util.Map;

public interface RuleSelection extends Clearable {

	/** Rule id to the setting values that deviate from the rule defaults. */
	public Map<String, Map<String, Object>> getSelection();

	/** Replaces all rule overrides; rules without an entry fall back to their defaults. */
	public void setSelection(Map<String, Map<String, Object>> rules);
}
