package xy.ai.workbench.model;

import java.util.EnumMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import xy.ai.workbench.Model;
import xy.ai.workbench.Model.KeyPattern;

/**
 * Central, pluggable registry mapping a provider ({@link KeyPattern}) to its {@link ModelResolver}.
 * Provider-specific (HTTP based) resolvers kick in as soon as a matching key is detected; results
 * are cached in memory for the lifetime of the application (once per key, no repeated HTTP calls
 * on every settings/session restore).
 */
public final class ModelResolverRegistry {

	private static final Map<KeyPattern, ModelResolver> RESOLVERS = new EnumMap<>(KeyPattern.class);
	private static final Map<String, List<Model>> CACHE = new ConcurrentHashMap<>();

	static {
		for (KeyPattern p : KeyPattern.values())
			RESOLVERS.put(p, new DefaultModelResolver(p));
		RESOLVERS.put(KeyPattern.OpenAI, new OpenAIModelResolver());
		RESOLVERS.put(KeyPattern.Gemini, new GeminiModelResolver());
		RESOLVERS.put(KeyPattern.Claude, new AnthropicModelResolver());
		RESOLVERS.put(KeyPattern.Deepseek, new DeepseekModelResolver());
		// ClaudeCode/None/Misc stay on the DefaultModelResolver: no provider API to discover models from.
	}

	private ModelResolverRegistry() {
	}

	/** Resolves the models available for the given key, using the resolver registered for its provider. */
	public static List<Model> resolve(KeyPattern provider, String apiKey) {
		return CACHE.computeIfAbsent(provider + ":" + apiKey, k -> RESOLVERS.get(provider).resolve(apiKey));
	}
}
