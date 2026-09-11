package xy.ai.workbench.connector.mcp;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpRequest.BodyPublishers;
import java.net.http.HttpResponse;
import java.net.http.HttpResponse.BodyHandlers;
import java.time.Duration;
import java.util.UUID;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.workbench.LOG;
import xy.ai.workbench.connector.claudecode.JsonUtil;

/**
 * Minimal MCP client over the Streamable-HTTP transport. Connection, session id
 * and tool cache live 1:1 in this instance; no shared manager.
 */
public class MCPClient {
	private static final String SERVER_URL = "http://localhost:9094/mcp";
	private static final String PROTOCOL_VERSION = "2025-06-18";
	private static final Duration TIMEOUT = Duration.ofSeconds(300);

	private final ObjectMapper mapper = JsonUtil.mapper();
	private final HttpClient http = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(5)).build();

	/** Instance-stable session id sent as X-MCPC-SESSION-ID. */
	private final String sessionId = UUID.randomUUID().toString();

	private boolean initialized;
	private String mcpSessionId;
	private ArrayNode toolCache;
	private long nextId = 1;

	public JsonNode findTool(String name) {
		for (JsonNode tool : listTools())
			if (name.equals(tool.path("name").asText()))
				return tool;
		return null;
	}

	public synchronized ArrayNode listTools() {
		if (toolCache != null)
			return toolCache;
		ensureConnected();
		JsonNode result = send(request("tools/list", mapper.createObjectNode()), true);
		JsonNode tools = result.path("tools");
		toolCache = tools.isArray() ? (ArrayNode) tools : mapper.createArrayNode();
		return toolCache;
	}

	public synchronized JsonNode callTool(String name, JsonNode arguments) {
		ensureConnected();
		ObjectNode params = mapper.createObjectNode();
		params.put("name", name);
		params.set("arguments", arguments != null && arguments.isObject() ? arguments : mapper.createObjectNode());
		return send(request("tools/call", params), true);
	}

	public synchronized void close() {
		initialized = false;
		mcpSessionId = null;
		toolCache = null;
	}

	private void ensureConnected() {
		if (initialized)
			return;
		// Fresh (re)connect: drop stale session and cached tools.
		mcpSessionId = null;
		toolCache = null;

		ObjectNode params = mapper.createObjectNode();
		params.put("protocolVersion", PROTOCOL_VERSION);
		params.putObject("capabilities");
		ObjectNode info = params.putObject("clientInfo");
		info.put("name", "xy.ai.workbench");
		info.put("version", "1.0");

		send(request("initialize", params), true);
		send(notification("notifications/initialized"), false);
		initialized = true;
	}

	private JsonNode send(ObjectNode rpc, boolean expectResult) {
		try {
			String body = mapper.writeValueAsString(rpc);
			HttpRequest.Builder builder = HttpRequest.newBuilder().uri(URI.create(SERVER_URL)).timeout(TIMEOUT)
					.header("Content-Type", "application/json")
					.header("Accept", "application/json, text/event-stream")
					.header("X-MCPC-TOOLS", "all")
					.header("X-MCPC-CONTROL", "off")
					.header("X-MCPC-SESSION-ID", sessionId);
			if (mcpSessionId != null)
				builder.header("Mcp-Session-Id", mcpSessionId);

			HttpResponse<String> response = http.send(builder.POST(BodyPublishers.ofString(body)).build(),
					BodyHandlers.ofString());
			response.headers().firstValue("Mcp-Session-Id").ifPresent(id -> mcpSessionId = id);

			if (response.statusCode() / 100 != 2) {
				initialized = false;
				throw new IllegalStateException("MCP server returned status " + response.statusCode());
			}
			if (!expectResult)
				return null;

			JsonNode message = parseMessage(response);
			if (message.has("error"))
				throw new IllegalArgumentException("MCP error: " + message.path("error").path("message").asText());
			return message.path("result");
		} catch (IOException | InterruptedException e) {
			initialized = false;
			if (e instanceof InterruptedException)
				Thread.currentThread().interrupt();
			LOG.error("MCP server unreachable", e);
			throw new IllegalStateException("MCP server unreachable", e);
		}
	}

	private JsonNode parseMessage(HttpResponse<String> response) throws JsonProcessingException {
		String contentType = response.headers().firstValue("Content-Type").orElse("");
		String body = response.body();
		if (!contentType.contains("text/event-stream"))
			return JsonUtil.readTree(body);

		// SSE: reassemble data: payloads and keep the frame carrying result/error.
		JsonNode found = null;
		StringBuilder data = new StringBuilder();
		for (String raw : body.split("\n")) {
			String line = raw.strip();
			if (line.startsWith("data:")) {
				if (data.length() > 0)
					data.append("\n");
				data.append(line.substring(5).strip());
			} else if (line.isEmpty() && data.length() > 0) {
				found = pickResponse(data.toString(), found);
				data.setLength(0);
			}
		}
		if (data.length() > 0)
			found = pickResponse(data.toString(), found);
		if (found == null)
			throw new IllegalStateException("No JSON-RPC response in SSE stream");
		return found;
	}

	private JsonNode pickResponse(String json, JsonNode current) {
		try {
			JsonNode node = mapper.readTree(json);
			if (node != null && (node.has("result") || node.has("error")))
				return node;
		} catch (JsonProcessingException e) {
			LOG.error("Ignoring malformed SSE frame", e);
		}
		return current;
	}

	private ObjectNode request(String method, JsonNode params) {
		ObjectNode rpc = mapper.createObjectNode();
		rpc.put("jsonrpc", "2.0");
		rpc.put("id", nextId++);
		rpc.put("method", method);
		if (params != null)
			rpc.set("params", params);
		return rpc;
	}

	private ObjectNode notification(String method) {
		ObjectNode rpc = mapper.createObjectNode();
		rpc.put("jsonrpc", "2.0");
		rpc.put("method", method);
		return rpc;
	}
}
