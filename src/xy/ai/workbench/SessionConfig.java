package xy.ai.workbench;

public class SessionConfig {
	public String keys = "dummy";
	public Long maxOutputTokens = 128000L;
	public Double temperature = 0d;
	public Double topP = 0.1d;
	public Model model = Model.GPT_5_NANO;
	public String[] systemPrompt = new String[] { //
			"Answer very short and precise", //
			"Be objective and neutral", //
			"Don't repeat the input", //
			"Don't ask follow-up questions", //
			"#Use $ Markdown inline Latex syntax for math formulas", //
			"#keep the input language", //
			"#For generated code use english language", //
			"#Replace all mathematical symbols and formulas in the input with a $ Latex inline syntax without changing or adding to the text."//
	};
	public OutputMode ouputMode = OutputMode.Append;
	public boolean[] inputModes = new boolean[InputMode.values().length];
	public Reasoning reasoning = Reasoning.minimal;
	public Integer reasoningBudget = -1;
	// TODO editor mit schönschrift

	public SessionConfig() {
		setInputMode(InputMode.Instructions, true);
		setInputMode(InputMode.Editor, true);
	}

	public String getKeys() {
		return keys;
	}

	public void setKeys(String keys) {
		this.keys = keys;
	}

	public Long getMaxOutputTokens() {
		return maxOutputTokens;
	}

	public void setMaxOutputTokens(Long maxOutputTokens) {
		this.maxOutputTokens = maxOutputTokens;
	}

	public Double getTemperature() {
		return temperature;
	}

	public void setTemperature(Double temperature) {
		this.temperature = temperature;
	}

	public Double getTopP() {
		return topP;
	}

	public void setTopP(Double topP) {
		this.topP = topP;
	}

	public Model getModel() {
		return model;
	}

	public void setModel(Model model) {
		this.model = model;
	}

	public Reasoning getReasoning() {
		return reasoning;
	}

	public void setReasoning(Reasoning reasoning) {
		this.reasoning = reasoning;
	}

	public String[] getSystemPrompt() {
		return systemPrompt;
	}

	public void setSystemPrompt(String[] systemPrompt) {
		this.systemPrompt = systemPrompt;
	}

	public boolean isInputEnabled(InputMode mode) {
		return inputModes[mode.ordinal()];
	}

	public void setInputMode(InputMode mode, boolean enable) {
		inputModes[mode.ordinal()] = enable;
	}

	public Integer getReasoningBudget() {
		return reasoningBudget;
	}

	public void setReasoningBudget(Integer reasoningBudget) {
		this.reasoningBudget = reasoningBudget;
	}
}
