package xy.ai.workbench.model;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.time.Instant;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

import xy.ai.workbench.LOG;
import xy.ai.workbench.Model.Capabilities;
import xy.ai.workbench.Reasoning;

/**
 * Loads and caches the generic capability catalog from https://models.dev/api.json. The catalog
 * is cached as a temp file and only refreshed if the cached copy is older than 7 days (or
 * missing/unreadable). Used as capability fallback for providers whose own API does not expose
 * capabilities directly (e.g. OpenAI, Deepseek).
 */
public final class ModelsDevCatalog {

	private static final String URL = "https://models.dev/api.json";
	private static final Duration MAX_AGE = Duration.ofDays(7);
	private static final Path CACHE_FILE = Path
			.of(System.getProperty("java.io.tmpdir"), "xy-ai-workbench-models-dev-cache.json");

	private static ModelsDevCatalog instance;

	private final JsonNode root;

	public static synchronized ModelsDevCatalog get() {
		if (instance == null)
			instance = new ModelsDevCatalog();
		return instance;
	}

	private ModelsDevCatalog() {
		this.root = load();
	}

	private JsonNode load() {
		ObjectMapper om = new ObjectMapper();
		try {
			String json = readFreshCache();
			if (json == null) {
				json = fetch();
				try {
					Files.writeString(CACHE_FILE, json, StandardCharsets.UTF_8);
				} catch (IOException e) {
					LOG.error("Unable to cache models.dev catalog", e);
				}
			}
			return om.readTree(json);
		} catch (Exception e) {
			LOG.error("Unable to load models.dev catalog", e);
			return om.createObjectNode();
		}
	}

	private String readFreshCache() {
		try {
			if (!Files.isRegularFile(CACHE_FILE))
				return null;
			Instant modified = Files.getLastModifiedTime(CACHE_FILE).toInstant();
			if (Duration.between(modified, Instant.now()).compareTo(MAX_AGE) > 0)
				return null;
			return Files.readString(CACHE_FILE, StandardCharsets.UTF_8);
		} catch (IOException e) {
			LOG.error("Unable to read cached models.dev catalog", e);
			return null;
		}
	}

	private String fetch() throws IOException, InterruptedException {
		HttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build();
		HttpRequest req = HttpRequest.newBuilder(URI.create(URL)).GET().timeout(Duration.ofSeconds(20)).build();
		HttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());
		if (resp.statusCode() / 100 != 2)
			throw new IOException("models.dev returned HTTP " + resp.statusCode());
		return resp.body();
	}

	/** Looks up the raw model entry for a models.dev provider id (e.g. "openai", "deepseek"). */
	public JsonNode findModel(String providerId, String modelId) {
		JsonNode provider = root.path(providerId).path("models");
		if (provider.isMissingNode())
			return null;
		JsonNode model = provider.path(modelId);
		return model.isMissingNode() ? null : model;
	}

	/** Derives {@link Capabilities} from a models.dev model entry, seeded with the given key pattern. */
	public Capabilities toCapabilities(xy.ai.workbench.Model.KeyPattern provider, JsonNode node) {
		Capabilities cap = new Capabilities().key(provider);
		if (node == null)
			return cap;
		if (node.has("temperature"))
			cap.supportTemperature(node.path("temperature").asBoolean(true));
		boolean reasoning = node.path("reasoning").asBoolean(false);
		cap.reasonings(reasoning ? Reasoning.OpenAI : new Reasoning[0]);
		int outLimit = node.path("limit").path("output").asInt(0);
		if (outLimit > 0)
			cap.outTokens(0, outLimit);
		return cap;
	}
}
