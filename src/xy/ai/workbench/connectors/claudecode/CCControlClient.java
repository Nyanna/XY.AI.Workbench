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
import com.fasterxml.jackson.dataformat.yaml.YAMLGenerator;
import com.fasterxml.jackson.dataformat.yaml.YAMLMapper;

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
	private final HttpClient http = HttpClient.newBuilder().connectTimeout(TIMEOUT).build();

	/**
	 * A second, YAML-flavoured mapper used exclusively to render/parse the
	 * human-facing side of the control loop (never for the wire protocol, which
	 * stays plain JSON via {@link #mapper} / {@link JsonUtil}).
	 *
	 * <p>
	 * Multi-line String values are written as literal block scalars ({@code |...})
	 * instead of {@code \n}-escaped one-liners &mdash; that is the whole point: a
	 * human can read and edit them as real, multi-line text. Everything else keeps
	 * the default double-quoting ({@code MINIMIZE_QUOTES} stays disabled) so YAML's
	 * implicit scalar typing never applies to untouched values: an unmodified
	 * String such as {@code country_code: "NO"} can never silently turn into the
	 * boolean {@code false} on the way back (the "Norway problem"), because it is
	 * never written as a bare, unquoted scalar in the first place. That risk only
	 * exists for values a user edits and (mistakenly) unquotes by hand &mdash; an
	 * accepted trade-off for readability.
	 */
	private final YAMLMapper yaml = YAMLMapper.builder().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER)
			.disable(YAMLGenerator.Feature.SPLIT_LINES).enable(YAMLGenerator.Feature.LITERAL_BLOCK_STYLE).build();

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
		if (node == null || node.isMissingNode() || node.isNull())
			return "";
		try {
			return yaml.writeValueAsString(node).stripTrailing();
		} catch (JsonProcessingException e) {
			// Should not happen for a tree that Jackson itself produced; fall back to
			// plain JSON rather than losing the payload.
			LOG.error("Failed to render control item as YAML", e);
			return JsonUtil.pretty(node);
		}
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
