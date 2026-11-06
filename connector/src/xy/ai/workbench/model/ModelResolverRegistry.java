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

import xy.ai.workbench.api.ext.IFileCache;
import xy.ai.workbench.connector.mcp.MCPClient;
import xy.ai.workbench.connector.mcp.MCPClient.IConnectObserver;
import xy.ai.workbench.core.LOG;
import xy.ai.workbench.core.Model;
import xy.ai.workbench.core.Tools;
import xy.ai.workbench.core.Model.KeyPattern;

/**
 * Central, pluggable registry mapping a provider ({@link KeyPattern}) to its
 * {@link ModelResolver}. Provider-specific (HTTP based) resolvers kick in as
 * soon as a matching key is detected; results are cached in memory for the
 * lifetime of the application (once per key, no repeated HTTP calls on every
 * settings/session restore).
 */
public class ModelResolverRegistry implements IConnectObserver {
	private final static String TOOLS_FILE = "tools.txt";
	private final Map<KeyPattern, ModelResolver> resolvers = new EnumMap<>(KeyPattern.class);
	private final Map<String, List<Model>> modelCache = new ConcurrentHashMap<>();
	private IFileCache cache;

	/**
	 * Cached tool list, lazily loaded from the state location on first resolve().
	 */
	private static String[] tools;
	
	public ModelResolverRegistry(IFileCache cache) {
		this.cache = cache;
		initResolver();
	}

	private void initResolver(){
		for (KeyPattern p : KeyPattern.values())
			resolvers.put(p, new DefaultModelResolver(p));
		
		ModelsDevCatalog catalog = new ModelsDevCatalog(cache);
		
		resolvers.put(KeyPattern.OpenAI, new OpenAIModelResolver(cache, catalog));
		resolvers.put(KeyPattern.Gemini, new GeminiModelResolver(cache));
		resolvers.put(KeyPattern.Claude, new AnthropicModelResolver(cache));
		resolvers.put(KeyPattern.Deepseek, new DeepseekModelResolver(cache, catalog));
		// ClaudeCode/None/Misc stay on the DefaultModelResolver
	}

	public List<Model> resolve(KeyPattern provider, String apiKey) {
		ensureToolsLoaded();
		List<Model> models = modelCache.computeIfAbsent(provider + ":" + apiKey,
				k -> resolvers.get(provider).resolve(apiKey));
		applyTools(models);
		return models;
	}

	private synchronized void ensureToolsLoaded() {
		if (tools != null)
			return;
		String[] loaded = loadTools();
		tools = loaded != null ? loaded : new String[0];
	}

	@Override
	public void onConnect(MCPClient client) {
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
		for (List<Model> models : modelCache.values())
			applyTools(models);
	}

	private void applyTools(List<Model> models) {
		for (Model model : models)
			model.cap.tools(tools);
	}

	private String[] sortByToolOrder(Collection<String> names) {
		LinkedHashSet<String> remaining = new LinkedHashSet<>(names);
		List<String> ordered = new ArrayList<>();
		for (String known : Tools.ALL)
			if (remaining.remove(known))
				ordered.add(known);
		ordered.addAll(remaining);
		return ordered.toArray(new String[0]);
	}

	private String[] loadTools() {
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

	private void saveTools(String[] tools) {
		try {
			Files.writeString(toolsFile().toPath(), String.join("\n", tools));
		} catch (IOException e) {
			LOG.error("Could not persist tool list", e);
		}
	}

	private File toolsFile() {
		return cache.getFile(TOOLS_FILE);
	}
}
