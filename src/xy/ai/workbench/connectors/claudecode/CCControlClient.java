package xy.ai.workbench.connectors.claudecode;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpRequest.BodyPublishers;
import java.net.http.HttpResponse;
import java.net.http.HttpResponse.BodyHandlers;
import java.time.Duration;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.workbench.LOG;

/**
 * Minimal HTTP client for the MCPC human-in-the-loop tool-control endpoint
 * ({@code POST /control/tool}). Replaces the standalone {@code control.sh}
 * client: this class is used directly from the connector's retrieval loop to
 * poll for pending tool-call requests/results and to submit approval,
 * rejection, or modification decisions.
 *
 * <p>
 * Request body: {@code {"approvals":[...]}} (may be empty for a plain poll).
 * Response body: {@code {"pending":[...]}}.
 */
public class CCControlClient {
	public static final String ANSWER = "/answer";
	public static final String CONTROL_REQUEST = "Control Request:";
	private static final String CONTROL_URL = "http://localhost:9093/control/tool";
	private static final Duration TIMEOUT = Duration.ofSeconds(5);

	private final ObjectMapper mapper = JsonUtil.mapper();
	private YamlRenderer yaml = new YamlRenderer();
	private final HttpClient http = HttpClient.newBuilder().connectTimeout(TIMEOUT).build();

	public void checkControlEndpoint(CCResponse resp) {
		JsonNode pending = poll();
		if (pending.isEmpty())
			return;
		StringBuilder res = new StringBuilder();
		ProtocolParser.appendEvents(resp.events, res);

		JsonNode first = pending.get(0);
		res.append(String.format("%s\n```yaml\n%s\n```\n%s %s allow", CONTROL_REQUEST, toYaml(first), ANSWER,
				first.path("id").asText()));
		resp.resultText = res.toString();
	}

	public String toYaml(JsonNode node) {
		return yaml.toYaml(node);
	}

	public JsonNode fromYaml(String text) throws JsonProcessingException {
		return yaml.readTree(text);
	}

	public boolean isMCPCAvailable() {
		return poll() != null; // never null, just check exceptions
	}

	private ArrayNode poll() {
		return post(mapper.createObjectNode());
	}

	public ArrayNode approve(String id) {
		return submit(approvalNode(id, null, null, null));
	}

	public ArrayNode deny(String id, String reason) {
		return submit(approvalNode(id, null, null, reason == null ? "" : reason));
	}

	public boolean submitEdit(String rawText) {
		if (rawText == null)
			return false;
		String block = extractYamlBlock(rawText.strip());
		if (block == null)
			return false;

		JsonNode edited;
		try {
			edited = fromYaml(block);
		} catch (Exception e) {
			throw new IllegalArgumentException("YAML Error", e);
		}
		if (edited == null || !edited.isObject() || !edited.hasNonNull("id"))
			return false;
		submit((ObjectNode) edited);
		return true;
	}

	private String extractYamlBlock(String text) {
		int start = text.indexOf("```yaml");
		if (start != 0)
			return null;
		int contentStart = start + "```yaml".length();
		int end = text.indexOf("```", contentStart);
		if (end == -1)
			return null;
		return text.substring(contentStart, end).strip();
	}

	private ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result, String rejectReason) {
		ObjectNode approval = mapper.createObjectNode();
		approval.put("id", id);
		if (arguments != null)
			approval.set("arguments", arguments);
		if (result != null)
			approval.set("result", result);
		if (rejectReason != null) {
			approval.put("rejected", true);
			approval.put("reason", rejectReason);
		}
		return approval;
	}

	private ArrayNode submit(ObjectNode approval) {
		ObjectNode body = mapper.createObjectNode();
		body.putArray("approvals").add(approval);
		return post(body);
	}

	private ArrayNode post(ObjectNode body) {
		try {
			String json = JsonUtil.write(body);
			HttpRequest request = HttpRequest.newBuilder().uri(URI.create(CONTROL_URL)).timeout(TIMEOUT)
					.header("Content-Type", "application/json").POST(BodyPublishers.ofString(json)).build();
			HttpResponse<String> response = http.send(request, BodyHandlers.ofString());
			if (response.statusCode() / 100 != 2) {
				LOG.error("control endpoint returned status " + response.statusCode());
				return mapper.createArrayNode();
			}
			String responseBody = response.body();
			if (responseBody == null || responseBody.isBlank()) {
				LOG.error("control endpoint returned an empty body");
				return mapper.createArrayNode();
			}
			JsonNode root = JsonUtil.readTree(responseBody);
			JsonNode pending = root.path("pending");
			return pending.isArray() ? (ArrayNode) pending : mapper.createArrayNode();
		} catch (IOException | InterruptedException e) {
			LOG.error("Control endpoint unreachable", e);
			if (e instanceof InterruptedException)
				Thread.currentThread().interrupt();
			throw new IllegalStateException("Error on control endpoint", e);
		}
	}
}
