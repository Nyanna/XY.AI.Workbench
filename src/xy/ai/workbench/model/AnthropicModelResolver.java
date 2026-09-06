package xy.ai.workbench.model;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

import xy.ai.workbench.LOG;
import xy.ai.workbench.Model;
import xy.ai.workbench.Model.Capabilities;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.Reasoning;

/** Resolves mainstream models currently available for an Anthropic API key. */
public class AnthropicModelResolver implements ModelResolver {

	@Override
	public List<Model> resolve(String apiKey) {
		try {
			String body = fetch(apiKey);
			ObjectMapper om = new ObjectMapper();
			JsonNode data = om.readTree(body).path("data");
			List<Model> result = new ArrayList<>();
			for (JsonNode entry : data) {
				String id = entry.path("id").asText(null);
				if (id == null || !ModelFilter.isMainstream(id))
					continue;
				String displayName = entry.path("display_name").asText(id);
				result.add(new Model(id, displayName, capabilitiesFor(entry)));
			}
			if (result.isEmpty()) {
				LOG.error("Anthropic model list was empty, falling back to defaults");
				return Arrays.asList(Model.defaultsFor(KeyPattern.Claude));
			}
			return result;
		} catch (Exception e) {
			LOG.error("Unable to resolve Anthropic models, falling back to defaults", e);
			return Arrays.asList(Model.defaultsFor(KeyPattern.Claude));
		}
	}

	private Capabilities capabilitiesFor(JsonNode entry) {
		Capabilities cap = new Capabilities().key(KeyPattern.Claude);
		JsonNode capNode = entry.path("capabilities");
		int maxTokens = entry.path("max_tokens").asInt(32000);
		cap.outTokens(0, maxTokens);
		boolean thinking = capNode.path("thinking").path("supported").asBoolean(false);
		if (thinking)
			cap.reasonings(Reasoning.Budget, Reasoning.Disabled).budget(1024, Math.max(1024, maxTokens - 1));
		else
			cap.reasonings(Reasoning.Disabled);
		return cap;
	}

	private String fetch(String apiKey) throws Exception {
		HttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build();
		HttpRequest req = HttpRequest.newBuilder(URI.create("https://api.anthropic.com/v1/models?limit=200"))
				.header("x-api-key", apiKey).header("anthropic-version", "2023-06-01").timeout(Duration.ofSeconds(20))
				.GET().build();
		HttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());
		if (resp.statusCode() / 100 != 2)
			throw new java.io.IOException("Anthropic /v1/models returned HTTP " + resp.statusCode());
		return resp.body();
	}
}
