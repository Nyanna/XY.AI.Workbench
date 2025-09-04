package xy.ai.workbench;

public class SessionConfig {
	public static enum Model {
		GPT_5_NANO, GPT_5_MINI, GPT_5, //
		GEMINI_25_PRO("gemini-2.5-pro"), GEMINI_25_FLASH("gemini-2.5-flash"), GEMINI_25_LIGHT("gemini-2.5-flash-lite");

		public final String connectorName;

		private Model() {
			this(null);
		}

		private Model(String connectorName) {
			this.connectorName = connectorName;
		}
	};

	public static enum Reasoning {
		MINIMAL, LOW, MEDIUM, HIGH, Budget
	};

	public String key = "dummy";
	public Long maxOutputTokens = 128000L; // 128K for 5 models, Gemini 2.5 65,536
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
	// gemini pro 128-32768, -1
	// gemini flash 0 to 24576, -1
	// gemini light 512 to 24576, 0,-1

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
