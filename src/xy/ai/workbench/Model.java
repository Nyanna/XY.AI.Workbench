package xy.ai.workbench;

import java.util.regex.Pattern;

public class Model {

	public final String apiName;
	public final String displayName;
	public final Capabilities cap;

	public Model(String apiName, String displayName, Capabilities cap) {
		this.apiName = apiName;
		this.displayName = displayName;
		this.cap = cap;
	}

	public Model(String apiName, Capabilities cap) {
		this(apiName, apiName, cap);
	}

	@Override
	public String toString() {
		return displayName;
	}

	// Deliberately no equals()/hashCode() override: identity equality is fine since resolvers
	// (incl. the in-memory ModelResolverRegistry cache) always hand out the same instances for the
	// same key, and a single apiName may legitimately represent more than one model configuration
	// (e.g. Claude Code "haiku" vs. the MCPC-root "haiku" with different tool/profile capabilities).
	// Use matches() for explicit id-based lookups (e.g. restoring a persisted model reference).
	public boolean matches(KeyPattern provider, String apiName) {
		return cap.getKeyPattern() == provider && java.util.Objects.equals(this.apiName, apiName);
	}

	public static enum KeyPattern {
		OpenAI("^sk-proj-.*$"), Gemini("^[a-zA-Z0-9]{39}$"), Claude("^sk-ant-api.*$"), Deepseek("^sk-[a-z0-9]{32}$"), None("^none$"),
		ClaudeCode("^(work|personal)$");

		public final Pattern pattern;

		private KeyPattern(String pattern) {
			this.pattern = Pattern.compile(pattern);
		}

		public boolean matches(String key) {
			return pattern.matcher(key).matches();
		}

	}

	// Default model catalog: used by the DefaultModelResolver, i.e. as long as no
	// provider-specific resolver takes over (or as a fallback if it fails).
	public static final Model NONE = new Model("none", new Capabilities()//
			.key(KeyPattern.None)//
			.supportTemperature(false)//
			.supportTopP(false)//
	);
	public static final Model GPT_5_NANO = new Model("gpt-5-nano", new Capabilities()//
			.key(KeyPattern.OpenAI)//
			.supportTemperature(false)//
			.supportTopP(false)//
			.reasonings(Reasoning.OpenAI)//
	);
	public static final Model GPT_5_MINI = new Model("gpt-5-mini", new Capabilities()//
			.key(KeyPattern.OpenAI)//
			.supportTemperature(false)//
			.supportTopP(false)//
			.reasonings(Reasoning.OpenAI)//
	);
	public static final Model GPT_5 = new Model("gpt-5", new Capabilities()//
			.key(KeyPattern.OpenAI)//
			.supportTemperature(false)//
			.supportTopP(false)//
			.reasonings(Reasoning.high, Reasoning.medium, Reasoning.low)//
	);
	public static final Model GEMINI_25_PRO = new Model("gemini-2.5-pro", new Capabilities()//
			.key(KeyPattern.Gemini)//
			.outTokens(0, 65536)//
			.reasonings(Reasoning.Unlimited, Reasoning.Budget)//
			.budget(128, 32768)//
	);
	public static final Model GEMINI_25_FLASH = new Model("gemini-2.5-flash", new Capabilities()//
			.key(KeyPattern.Gemini)//
			.outTokens(0, 65536)//
			.reasonings(Reasoning.Budgets)//
			.budget(0, 24576)//
	);
	public static final Model GEMINI_25_LIGHT = new Model("gemini-2.5-flash-lite", new Capabilities()//
			.key(KeyPattern.Gemini)//
			.outTokens(0, 65536)//
			.reasonings(Reasoning.Budgets)//
			.budget(512, 24576)//
	);
	public static final Model CLAUDE_OPUS = new Model("claude-opus-4-1", new Capabilities()//
			.key(KeyPattern.Claude)//
			.outTokens(0, 32000)//
			.reasonings(Reasoning.Budget, Reasoning.Disabled)//
			.budget(1024, 31999)//
	);
	public static final Model CLAUDE_SONNET = new Model("claude-sonnet-4-0", new Capabilities()//
			.key(KeyPattern.Claude)//
			.outTokens(0, 32000)//
			.reasonings(Reasoning.Budget, Reasoning.Disabled)//
			.budget(1024, 31999)//
	);
	public static final Model CC_HAIKU = new Model("haiku", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportTemperature(false)//
			.supportTopP(false)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.agentProfiles(AgentProfile.values())//
			.reasonings(Reasoning.ClaudeCode)//
	);
	public static final Model CC_SONNET = new Model("sonnet", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportTemperature(false)//
			.supportTopP(false)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.agentProfiles(AgentProfile.values())//
			.reasonings(Reasoning.ClaudeCode)//
	);
	public static final Model CC_OPUS = new Model("opus", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportTemperature(false)//
			.supportTopP(false)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.agentProfiles(AgentProfile.values())//
			.reasonings(Reasoning.ClaudeCode)//
	);
	// MCPC Root Agents (displayName kept distinct from CC_* despite identical apiName)
	public static final Model CC_MCPC_HAIKU = new Model("haiku", "haiku (mcpc)", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportTemperature(false)//
			.supportTopP(false)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.tools(Tools.ALL)//
			.agentProfiles(AgentProfile.MCPC)//
			.reasonings(Reasoning.ClaudeCode)//
	);
	public static final Model CC_MCPC_SONNET = new Model("sonnet", "sonnet (mcpc)", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportTemperature(false)//
			.supportTopP(false)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.tools(Tools.ALL)//
			.agentProfiles(AgentProfile.MCPC)//
			.reasonings(Reasoning.ClaudeCode)//
	);
	public static final Model CC_MCPC_OPUS = new Model("opus", "opus (mcpc)", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportTemperature(false)//
			.supportTopP(false)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.tools(Tools.ALL)//
			.agentProfiles(AgentProfile.MCPC)//
			.reasonings(Reasoning.ClaudeCode)//
	);

	public static final Model[] DEFAULTS = { NONE, GPT_5_NANO, GPT_5_MINI, GPT_5, GEMINI_25_PRO, GEMINI_25_FLASH,
			GEMINI_25_LIGHT, CLAUDE_OPUS, CLAUDE_SONNET, CC_HAIKU, CC_SONNET, CC_OPUS, CC_MCPC_HAIKU, CC_MCPC_SONNET,
			CC_MCPC_OPUS };

	/** Default (static) catalog of models for a given provider, used by the DefaultModelResolver. */
	public static Model[] defaultsFor(KeyPattern provider) {
		return java.util.Arrays.stream(DEFAULTS).filter(m -> m.cap.getKeyPattern() == provider).toArray(Model[]::new);
	}

	public static class Capabilities {
		private boolean supportTemperature = true;
		private boolean supportTopP = true;
		private boolean supportMaxToken = true;
		private boolean supportBatch = true;
		private Reasoning[] reasonings = Reasoning.values();
		private int rsnBudgetMin = 0;
		private int rsnBudgetMax = 32768;
		private int outTknMin = 0;
		private int outTknMax = 128000; // max for gpt5
		private KeyPattern keyPattern;
		private AgentProfile[] agentProfiles = new AgentProfile[0];
		private String[] tools = new String[0];
		private CacheMode[] cacheMode = new CacheMode[0];

		public Capabilities supportTemperature(boolean flag) {
			supportTemperature = flag;
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

		public boolean isSupportTemperature() {
			return supportTemperature;
		}

		public boolean isSupportTopP() {
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

		public Double alignTemperature(Double temperature) {
			if (temperature < 0d)
				return 0d;
			if (temperature > 1d)
				return 1d;
			return temperature;
		}

		public Double alignTopP(Double topP) {
			if (topP < 0d)
				return 0d;
			if (topP > 1d)
				return 1d;
			return topP;
		}
	}
}