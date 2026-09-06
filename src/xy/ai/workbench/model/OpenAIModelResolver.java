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

/** Resolves mainstream chat models currently available for an OpenAI API key. */
public class OpenAIModelResolver implements ModelResolver {

	@Override
	public List<Model> resolve(String apiKey) {
		try {
			String body = fetch(apiKey);
			ObjectMapper om = new ObjectMapper();
			JsonNode data = om.readTree(body).path("data");
			List<Model> result = new ArrayList<>();
			for (JsonNode entry : data) {
				String id = entry.path("id").asText(null);
				JsonNode shutdown = entry.path("shutdown_date");
				boolean hasShutdownDate = !shutdown.isMissingNode() && !shutdown.isNull();
				if (id == null || hasShutdownDate)
					continue;
				if (!ModelFilter.isMainstream(id))
					continue;
				result.add(new Model(id, id, capabilitiesFor(id)));
			}
			if (result.isEmpty()) {
				LOG.error("OpenAI model list was empty, falling back to defaults");
				return Arrays.asList(Model.defaultsFor(KeyPattern.OpenAI));
			}
			return result;
		} catch (Exception e) {
			LOG.error("Unable to resolve OpenAI models, falling back to defaults", e);
			return Arrays.asList(Model.defaultsFor(KeyPattern.OpenAI));
		}
	}

	private Capabilities capabilitiesFor(String id) {
		for (Model dflt : Model.defaultsFor(KeyPattern.OpenAI))
			if (dflt.apiName.equals(id))
				return dflt.cap;
		JsonNode node = ModelsDevCatalog.get().findModel("openai", id);
		return ModelsDevCatalog.get().toCapabilities(KeyPattern.OpenAI, node);
	}

	private String fetch(String apiKey) throws Exception {
		HttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build();
		HttpRequest req = HttpRequest.newBuilder(URI.create("https://api.openai.com/v1/models"))
				.header("Authorization", "Bearer " + apiKey).timeout(Duration.ofSeconds(20)).GET().build();
		HttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());
		if (resp.statusCode() / 100 != 2)
			throw new java.io.IOException("OpenAI /v1/models returned HTTP " + resp.statusCode());
		return resp.body();
	}
}
