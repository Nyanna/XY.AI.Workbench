package xy.ai.workbench.connectors.claudecode;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpRequest.BodyPublishers;
import java.net.http.HttpResponse;
import java.net.http.HttpResponse.BodyHandlers;
import java.time.Duration;

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
public class ClaudeCodeControlClient {

	private static final String CONTROL_URL = "http://localhost:9093/control/tool";
	private static final Duration TIMEOUT = Duration.ofSeconds(5);

	private final ObjectMapper mapper = JsonUtil.mapper();
	private final HttpClient http = HttpClient.newBuilder().connectTimeout(TIMEOUT).build();

	public ClaudeCodeResponse checkControlEndpoint(ClaudeCodeRequest req) {
		JsonNode pending = poll();
		if (pending == null || pending.isEmpty())
			return null;

		JsonNode first = pending.get(0);
		// Render the pending item as valid, once-escaped JSON via the shared utility
		// (same pretty-print + safe fallback, no hand-rolled toString()).
		String text = JsonUtil.pretty(first);
		String prepared = "#: Control Request:\n" + ClaudeCodeJsonParser.commented(text) + "\n/allow "
				+ first.path("id").asText();
		return new ClaudeCodeResponse(req.id, prepared, false);
	}

	public boolean isMCPCAvailable() {
		return poll() != null;
	}

	private ArrayNode poll() {
		return post(mapper.createObjectNode());
	}

	/**
	 * Submits a simple approval (no modification) for the given pending item id.
	 */
	public ArrayNode approve(String id) {
		return submit(approvalNode(id, null, null, null));
	}

	/** Submits a rejection with a reason for the given pending item id. */
	public ArrayNode deny(String id, String reason) {
		return submit(approvalNode(id, null, null, reason == null ? "" : reason));
	}

	/**
	 * Submits an approval carrying modified arguments ({@code phase == "request"}).
	 */
	public ArrayNode submitModifiedArguments(String id, JsonNode arguments) {
		return submit(approvalNode(id, arguments, null, null));
	}

	/**
	 * Submits an approval carrying a modified result ({@code phase == "result"}).
	 */
	public ArrayNode submitModifiedResult(String id, JsonNode result) {
		return submit(approvalNode(id, null, result, null));
	}

	/**
	 * Detects whether {@code rawJson} is an edited pending control item: the
	 * (originally unchanged) JSON structure of an open request/result whose "id"
	 * matches one of the currently pending items at the control endpoint. If so,
	 * the modified "arguments" (request phase) or "result" (result phase) are
	 * submitted to the control endpoint.
	 *
	 * @return {@code true} when {@code rawJson} was recognised as a pending item
	 *         and forwarded as a control decision
	 */
	public boolean submitEdit(String rawJson) {
		if (rawJson == null || rawJson.isEmpty() || rawJson.charAt(0) != '{')
			return false;

		JsonNode edited;
		try {
			edited = JsonUtil.readTree(rawJson);
		} catch (Exception e) {
			return false;
		}
		if (!edited.isObject() || !edited.hasNonNull("id"))
			return false;
		String id = edited.path("id").asText();
		String phase = edited.path("phase").asText("");
		if ("result".equals(phase) && edited.has("result"))
			submitModifiedResult(id, edited.path("result"));
		else if (edited.has("arguments"))
			submitModifiedArguments(id, edited.path("arguments"));
		else
			approve(id);
		return true;
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
			LOG.error("ClaudeCodeControlClient: control endpoint unreachable", e);
			if (e instanceof InterruptedException)
				Thread.currentThread().interrupt();
			throw new IllegalStateException("Error on control endpoint", e);
		}
	}
}
