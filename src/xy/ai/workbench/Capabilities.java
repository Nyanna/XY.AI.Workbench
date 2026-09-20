package xy.ai.workbench;

import xy.ai.workbench.Model.KeyPattern;

public class Capabilities {
	private boolean supportTemperature = false;
	private boolean supportReasoningTemperature = false;
	private boolean supportTopP = false;
	private boolean supportReasoningTopP = false;
	private boolean supportMaxToken = true;
	private boolean supportBatch = true;
	private Reasoning[] reasonings = Reasoning.values();
	private int rsnBudgetMin = 0;
	private int rsnBudgetMax = 32768;
	private int outTknMin = 0;
	private int outTknMax = 128000; // max for gpt5
	private double toppMin = 0d;
	private double toppMax = 1d;
	private double tempMin = 0d;
	private double tempMax = 2d;
	private KeyPattern keyPattern;
	private AgentProfile[] agentProfiles = new AgentProfile[0];
	private String[] tools = new String[0];
	private CacheMode[] cacheMode = new CacheMode[0];

	public Capabilities supportTemperature(boolean flag) {
		supportTemperature = flag;
		return this;
	}

	public Capabilities supportReasoningTemperature(boolean flag) {
		supportReasoningTemperature = flag;
		return this;
	}

	public Capabilities tools(String[] tools) {
		this.tools = tools;
		return this;
	}

	public Capabilities agentProfiles(AgentProfile... profiles) {
		if (profiles.length > 0 && profiles[0] != null)
			this.agentProfiles = profiles;
		return this;
	}

	public Capabilities supportTopP(boolean flag) {
		supportTopP = flag;
		return this;
	}

	public Capabilities supportReasoningTopP(boolean flag) {
		supportReasoningTopP = flag;
		return this;
	}

	public Capabilities topPMin(double topPMin) {
		this.toppMin = topPMin;
		return this;
	}

	public Capabilities topPMax(double topPMax) {
		this.toppMax = topPMax;
		return this;
	}

	public Capabilities tempMax(double tempMax) {
		this.tempMax = tempMax;
		return this;
	}

	public Capabilities supportMaxToken(boolean supportMaxToken) {
		this.supportMaxToken = supportMaxToken;
		return this;
	}

	public Capabilities cacheMode(CacheMode... cacheMode) {
		this.cacheMode = cacheMode;
		return this;
	}

	public Capabilities supportBatch(boolean supportBatch) {
		this.supportBatch = supportBatch;
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

	public Capabilities key(KeyPattern pattern) {
		keyPattern = pattern;
		return this;
	}

	public Capabilities reasonings(Reasoning... val) {
		reasonings = val;
		return this;
	}

	public boolean isSupportTemperature(Reasoning reasoning) {
		if (!Reasoning.Disabled.equals(reasoning))
			return supportReasoningTemperature;
		return supportTemperature;
	}

	public boolean isSupportTopP(Reasoning reasoning) {
		if (!Reasoning.Disabled.equals(reasoning))
			return supportReasoningTopP;
		return supportTopP;
	}

	public boolean isSupportMaxToken() {
		return supportMaxToken;
	}

	public boolean isSupportBatch() {
		return supportBatch;
	}

	public Reasoning[] getReasonings() {
		return reasonings;
	}

	public KeyPattern getKeyPattern() {
		return keyPattern;
	}

	public AgentProfile[] getAgentProfiles() {
		return agentProfiles;
	}

	public String[] getTools() {
		return tools;
	}

	public CacheMode[] getCacheMode() {
		return cacheMode;
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

	public boolean acceptsKey(String key) {
		return keyPattern.matches(key);
	}

	public Double alignTemperature(Double temp) {
		if (temp < tempMin)
			return tempMin;
		if (temp > tempMax)
			return tempMax;
		return temp;
	}

	public Double alignTopP(Double topP) {
		if (topP < toppMin)
			return toppMin;
		else if (topP > toppMax) {
			return toppMax;
		}
		return topP;
	}
}