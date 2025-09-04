package xy.ai.workbench;

public class SessionConfig {
	public static enum Model {
		GPT_5_NANO, GPT_5_MINI, GPT_5
	};

	public static enum Reasoning {
		MINIMAL, LOW, MEDIUM, HIGH
	};

	public String key = null;
	public Long maxOutputTokens = 16 * 1024L; // 128K for 5 models
	public Double temperature = 0d; // 0-2
	public Double topP = 0.1d;
	public Model model = Model.GPT_5_NANO;
	public String[] systemPrompt = new String[] { //
			"Answer very short and precise", //
			"Be objective and neutral", //
			"Don't repeat the input", //
			"Don't ask follow-up questions for this one-time prompt", //
			"#Use $ Markdown inline Latex syntax for math formulas", //
			"#For generated code use english language"//
	};
	public OutputMode ouputMode = OutputMode.Append;
	public boolean[] inputModes = new boolean[InputMode.values().length];
	public Reasoning reasoning = Reasoning.MINIMAL;

	public SessionConfig() {
		setInputMode(InputMode.Instructions, true);
		setInputMode(InputMode.Editor, true);
	}

	public String getKey() {
		return key;
	}

	public void setKey(String key) {
		this.key = key;
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
}
