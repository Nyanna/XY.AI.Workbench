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

/** Resolves mainstream generateContent models currently available for a Gemini API key. */
public class GeminiModelResolver implements ModelResolver {

	@Override
	public List<Model> resolve(String apiKey) {
		try {
			String body = fetch(apiKey);
			ObjectMapper om = new ObjectMapper();
			JsonNode models = om.readTree(body).path("models");
			List<Model> result = new ArrayList<>();
			for (JsonNode entry : models) {
				boolean supportsGenerate = false;
				for (JsonNode method : entry.path("supportedGenerationMethods"))
					if ("generateContent".equals(method.asText()))
						supportsGenerate = true;
				if (!supportsGenerate)
					continue;
				String id = entry.path("baseModelId").asText(null);
				if (id == null || id.isBlank())
					id = entry.path("name").asText("").replace("models/", "");
				if (id.isBlank() || !ModelFilter.isMainstream(id))
					continue;
				String displayName = entry.path("displayName").asText(id);
				result.add(new Model(id, displayName, capabilitiesFor(entry)));
			}
			if (result.isEmpty()) {
				LOG.error("Gemini model list was empty, falling back to defaults");
				return Arrays.asList(Model.defaultsFor(KeyPattern.Gemini));
			}
			return result;
		} catch (Exception e) {
			LOG.error("Unable to resolve Gemini models, falling back to defaults", e);
			return Arrays.asList(Model.defaultsFor(KeyPattern.Gemini));
		}
	}

	private Capabilities capabilitiesFor(JsonNode entry) {
		Capabilities cap = new Capabilities().key(KeyPattern.Gemini);
		int outLimit = entry.path("outputTokenLimit").asInt(65536);
		cap.outTokens(0, outLimit);
		boolean thinking = entry.path("thinking").asBoolean(false);
		if (thinking)
			cap.reasonings(Reasoning.Budgets).budget(0, outLimit);
		else
			cap.reasonings(new Reasoning[0]);
		return cap;
	}

	private String fetch(String apiKey) throws Exception {
		HttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build();
		HttpRequest req = HttpRequest
				.newBuilder(URI.create("https://generativelanguage.googleapis.com/v1beta/models?pageSize=1000"))
				.header("x-goog-api-key", apiKey).timeout(Duration.ofSeconds(20)).GET().build();
		HttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());
		if (resp.statusCode() / 100 != 2)
			throw new java.io.IOException("Gemini /v1beta/models returned HTTP " + resp.statusCode());
		return resp.body();
	}
}
