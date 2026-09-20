package xy.ai.workbench;

import java.util.Objects;
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

	/*
	 * Deliberately no equals()/hashCode() override: identity equality is fine since
	 * resolvers (incl. the in-memory ModelResolverRegistry cache) always hand out
	 * the same instances for the same key, and a single apiName may legitimately
	 * represent more than one model configuration (e.g. Claude Code "haiku" vs. the
	 * MCPC-root "haiku" with different tool/profile capabilities). Use matches()
	 * for explicit id-based lookups (e.g. restoring a persisted model reference)
	 */
	public boolean matches(KeyPattern provider, String apiName) {
		return cap.getKeyPattern() == provider && Objects.equals(this.apiName, apiName);
	}

	public static enum KeyPattern {
		OpenAI("^sk-proj-.*$"), Gemini("^[a-zA-Z0-9]{39}$"), Claude("^sk-ant-api.*$"), Deepseek("^sk-[a-z0-9]{32}$"),
		None("^none$"), ClaudeCode("^(work|personal)$"), Misc("^.*$");

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
	);
	public static final Model CC_HAIKU = new Model("haiku", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.agentProfiles(AgentProfile.values())//
			.reasonings(Reasoning.ClaudeCode)//
	);
	public static final Model CC_SONNET = new Model("sonnet", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.agentProfiles(AgentProfile.values())//
			.reasonings(Reasoning.ClaudeCode)//
	);
	public static final Model CC_OPUS = new Model("opus", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.agentProfiles(AgentProfile.values())//
			.reasonings(Reasoning.ClaudeCode)//
	);
	public static final Model CC_MCPC_HAIKU = new Model("haiku", "haiku (mcpc)", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.tools(Tools.ALL)//
			.agentProfiles(AgentProfile.MCPC)//
			.reasonings(Reasoning.ClaudeCode)//
	);
	public static final Model CC_MCPC_SONNET = new Model("sonnet", "sonnet (mcpc)", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.tools(Tools.ALL)//
			.agentProfiles(AgentProfile.MCPC)//
			.reasonings(Reasoning.ClaudeCode)//
	);
	public static final Model CC_MCPC_OPUS = new Model("opus", "opus (mcpc)", new Capabilities()//
			.key(KeyPattern.ClaudeCode)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.cacheMode(CacheMode.ClaudeCode)//
			.tools(Tools.ALL)//
			.agentProfiles(AgentProfile.MCPC)//
			.reasonings(Reasoning.ClaudeCode)//
	);

	public static final Model MCP_TOOLS = new Model("mcp-tools", "MCP Tools", new Capabilities()//
			.key(KeyPattern.Misc)//
			.supportMaxToken(false)//
			.supportBatch(false)//
			.reasonings(Reasoning.Disabled)//
	);

	public static final Model[] DEFAULTS = { NONE, CC_HAIKU, CC_SONNET, CC_OPUS, CC_MCPC_HAIKU, CC_MCPC_SONNET,
			CC_MCPC_OPUS, MCP_TOOLS };

	/**
	 * Default (static) catalog of models for a given provider, used by the
	 * DefaultModelResolver.
	 */
	public static Model[] defaultsFor(KeyPattern provider) {
		return java.util.Arrays.stream(DEFAULTS).filter(m -> m.cap.getKeyPattern() == provider).toArray(Model[]::new);
	}
}