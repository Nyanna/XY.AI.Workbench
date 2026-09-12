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

/**
 * Resolves mainstream chat models currently available for a Deepseek API key.
 */
public class DeepseekModelResolver implements ModelResolver {

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
				JsonNode node = ModelsDevCatalog.get().findModel("deepseek", id);
				Capabilities cap = ModelsDevCatalog.get().toCapabilities(KeyPattern.Deepseek, node);
				cap.reasonings(Reasoning.Disabled, Reasoning.low, Reasoning.high, Reasoning.max);
				result.add(new Model(id, id, cap));
			}
			if (result.isEmpty())
				LOG.error("Deepseek model list was empty");
			return result;
		} catch (Exception e) {
			LOG.error("Unable to resolve Deepseek models", e);
			return Arrays.asList(Model.defaultsFor(KeyPattern.Deepseek));
		}
	}

	private String fetch(String apiKey) throws Exception {
		HttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build();
		HttpRequest req = HttpRequest.newBuilder(URI.create("https://api.deepseek.com/models"))
				.header("Accept", "application/json").header("Authorization", "Bearer " + apiKey)
				.timeout(Duration.ofSeconds(20)).GET().build();
		HttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());
		if (resp.statusCode() / 100 != 2)
			throw new java.io.IOException("Deepseek /models returned HTTP " + resp.statusCode());
		return resp.body();
	}
}
