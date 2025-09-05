package xy.ai.workbench;

public enum Model {
	GPT_5_NANO("GPT_5_NANO", new Capabilities()//
			.supportTemperature(false)//
			.supportTopP(false)//
			.openAIReasonings()//
	), //
	GPT_5_MINI("GPT_5_MINI", new Capabilities()//
			.supportTemperature(false)//
			.supportTopP(false)//
			.openAIReasonings()//
	), //
	GPT_5("GPT_5", new Capabilities()//
			.supportTemperature(false)//
			.supportTopP(false)//
			.openAIReasonings()//
	), //
	GEMINI_25_PRO("gemini-2.5-pro", new Capabilities()//
			.outTokens(0, 65536) //
			.reasonings(new Reasoning[] { Reasoning.Budget, Reasoning.Unlimited })//
			.budget(128, 32768)//
	), //
	GEMINI_25_FLASH("gemini-2.5-flash", new Capabilities()//
			.outTokens(0, 65536)//
			.budgetReasonings()//
			.budget(0, 24576)//
	), //
	GEMINI_25_LIGHT("gemini-2.5-flash-lite", new Capabilities()//
			.outTokens(0, 65536) //
			.budgetReasonings()//
			.budget(512, 24576) //
	) //
	;

	public final String connectorName;
	public final Capabilities cap;

	private Model(String connectorName, Capabilities cap) {
		this.connectorName = connectorName;
		this.cap = cap;
	}

	public static class Capabilities {
		private boolean supportTemperature = true;
		private boolean supportTopP = true;
		private Reasoning[] reasonings = Reasoning.values();
		private int rsnBudgetMin = 0;
		private int rsnBudgetMax = 32768;
		private int outTknMin = 0;
		private int outTknMax = 128000; // max for gpt5

		public Capabilities supportTemperature(boolean flag) {
			supportTemperature = flag;
			return this;
		}

		public Capabilities supportTopP(boolean flag) {
			supportTopP = flag;
			return this;
		}

		public Capabilities budget(int min, int max) {
			rsnBudgetMin = min;
			rsnBudgetMax = max;
			return this;
		}

		public Capabilities outTokens(int min, int max) {
			outTknMin = min;
			outTknMax = max;
			return this;
		}

		public Capabilities openAIReasonings() {
			reasonings = new Reasoning[] { Reasoning.MINIMAL, Reasoning.LOW, Reasoning.MEDIUM, Reasoning.HIGH };
			return this;
		}

		public Capabilities budgetReasonings() {
			reasonings = new Reasoning[] { Reasoning.Budget, Reasoning.Disabled, Reasoning.Unlimited };
			return this;
		}

		public Capabilities reasonings(Reasoning[] val) {
			reasonings = val;
			return this;
		}

		public boolean isSupportTemperature() {
			return supportTemperature;
		}

		public boolean isSupportTopP() {
			return supportTopP;
		}

		public Reasoning[] getReasonings() {
			return reasonings;
		}

		public int alignOutpuTokens(int tokens) {
			if (tokens < outTknMin)
				return outTknMin;
			else if (tokens > outTknMax) {
				return outTknMax;
			}
			return tokens;
		}

		public int alignBudget(int budget) {
			if (budget < rsnBudgetMin)
				return rsnBudgetMin;
			else if (budget > rsnBudgetMax) {
				return rsnBudgetMax;
			}
			return budget;
		}
	}
}