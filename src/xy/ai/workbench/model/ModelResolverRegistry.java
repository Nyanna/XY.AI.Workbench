package xy.ai.workbench.model;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.EnumMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import com.fasterxml.jackson.databind.JsonNode;

import xy.ai.workbench.Activator;
import xy.ai.workbench.LOG;
import xy.ai.workbench.Model;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.Tools;
import xy.ai.workbench.connector.mcp.MCPClient;

/**
 * Central, pluggable registry mapping a provider ({@link KeyPattern}) to its
 * {@link ModelResolver}. Provider-specific (HTTP based) resolvers kick in as
 * soon as a matching key is detected; results are cached in memory for the
 * lifetime of the application (once per key, no repeated HTTP calls on every
 * settings/session restore).
 */
public class ModelResolverRegistry {
	private static final Map<KeyPattern, ModelResolver> RESOLVERS = new EnumMap<>(KeyPattern.class);
	private static final Map<String, List<Model>> CACHE = new ConcurrentHashMap<>();
	private static final String TOOLS_FILE = "tools.txt";

	/** Cached tool list, lazily loaded from the state location on first resolve(). */
	private static String[] tools;

	static {
		for (KeyPattern p : KeyPattern.values())
			RESOLVERS.put(p, new DefaultModelResolver(p));
		RESOLVERS.put(KeyPattern.OpenAI, new OpenAIModelResolver());
		RESOLVERS.put(KeyPattern.Gemini, new GeminiModelResolver());
		RESOLVERS.put(KeyPattern.Claude, new AnthropicModelResolver());
		RESOLVERS.put(KeyPattern.Deepseek, new DeepseekModelResolver());
		// ClaudeCode/None/Misc stay on the DefaultModelResolver
	}

	public static List<Model> resolve(KeyPattern provider, String apiKey) {
		ensureToolsLoaded();
		List<Model> models = CACHE.computeIfAbsent(provider + ":" + apiKey, k -> RESOLVERS.get(provider).resolve(apiKey));
		applyTools(models);
		return models;
	}

	/** Wires the registry as a connect observer of the given (not yet connected) MCP client. */
	public static void attach(MCPClient client) {
		client.addConnectObserver(c -> onConnect(c));
	}

	private static synchronized void ensureToolsLoaded() {
		if (tools != null)
			return;
		String[] loaded = loadTools();
		tools = loaded != null ? loaded : new String[0];
	}

	private static void onConnect(MCPClient client) {
		List<String> discovered = new ArrayList<>();
		for (JsonNode tool : client.listTools())
			discovered.add(tool.path("name").asText());
		String[] sorted = sortByToolOrder(discovered);

		synchronized (ModelResolverRegistry.class) {
			if (Arrays.equals(sorted, tools))
				return;
			tools = sorted;
			saveTools(sorted);
		}
		for (List<Model> models : CACHE.values())
			applyTools(models);
	}

	private static void applyTools(List<Model> models) {
		for (Model model : models)
			model.cap.tools(tools);
	}

	private static String[] sortByToolOrder(Collection<String> names) {
		LinkedHashSet<String> remaining = new LinkedHashSet<>(names);
		List<String> ordered = new ArrayList<>();
		for (String known : Tools.ALL)
			if (remaining.remove(known))
				ordered.add(known);
		ordered.addAll(remaining);
		return ordered.toArray(new String[0]);
	}

	private static String[] loadTools() {
		try {
			File file = toolsFile();
			if (!file.exists())
				return null;
			String content = Files.readString(file.toPath());
			return content.isBlank() ? new String[0] : content.split("\n");
		} catch (IOException e) {
			LOG.error("Could not load persisted tool list", e);
			return null;
		}
	}

	private static void saveTools(String[] tools) {
		try {
			Files.writeString(toolsFile().toPath(), String.join("\n", tools));
		} catch (IOException e) {
			LOG.error("Could not persist tool list", e);
		}
	}

	private static File toolsFile() {
		return Activator.getDefault().getStateLocation().append(TOOLS_FILE).toFile();
	}
}
