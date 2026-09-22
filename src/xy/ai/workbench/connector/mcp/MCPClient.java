package xy.ai.workbench.connector.mcp;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpRequest.BodyPublishers;
import java.net.http.HttpResponse;
import java.net.http.HttpResponse.BodyHandlers;
import java.time.Duration;
import java.util.List;
import java.util.UUID;
import java.util.concurrent.CopyOnWriteArrayList;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;

import xy.ai.workbench.LOG;
import xy.ai.workbench.connector.claudecode.JsonUtil;
import xy.ai.workbench.connector.claudecode.YamlRenderer;

/**
 * Minimal MCP client over the Streamable-HTTP transport. Connection, session id
 * and tool cache live 1:1 in this instance; no shared manager.
 */
public class MCPClient {
	private static final String SERVER_URL = "http://localhost:9093/mcp";
	private static final String PROTOCOL_VERSION = "2025-06-18";
	private static final Duration TIMEOUT = Duration.ofSeconds(300);

	private final ObjectMapper mapper = JsonUtil.mapper();
	private final YamlRenderer yaml = new YamlRenderer();
	private final HttpClient http = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(5)).build();

	/** Instance-stable session id sent as X-MCPC-SESSION-ID. */
	private final String sessionId = UUID.randomUUID().toString();
	private final List<ConnectObserver> connectObservers = new CopyOnWriteArrayList<>();

	private boolean initialized;
	private boolean connectNotified;
	private String mcpSessionId;
	private ArrayNode toolCache;
	private long nextId = 1;

	public interface ConnectObserver {
		void onConnect(MCPClient client);
	}

	public void addConnectObserver(ConnectObserver observer) {
		connectObservers.add(observer);
	}

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

	/**
	 * Renders a model-issued tool call as a fenced ```yaml block, in the same shape
	 * used for MCPC tool call round-trips ({@code tool}/{@code arguments}).
	 */
	public String renderToolCall(String name, JsonNode arguments) {
		ObjectNode call = mapper.createObjectNode();
		call.put("tool", name);
		ObjectNode args = arguments != null && arguments.isObject() ? ((ObjectNode) arguments).deepCopy() : mapper.createObjectNode();
		args.remove("reason");
		call.set("arguments", args);
		return yaml.toYamlBlock(call);
	}

	public synchronized JsonNode callTool(String name, JsonNode arguments, JsonNode toolCallId) {
		ensureConnected();
		ObjectNode params = mapper.createObjectNode();
		params.put("name", name);
		params.set("arguments", arguments != null && arguments.isObject() ? arguments : mapper.createObjectNode());
		JsonNode result = send(request("tools/call", params), true);

		boolean hasToolCallId = toolCallId != null && !toolCallId.isMissingNode() && !toolCallId.isNull();
		boolean isError = result.path("isError").asBoolean(false);
		if (!hasToolCallId && isError)
			throw new IllegalArgumentException("MCP Error: " + extractErrorText(result));

		JsonNode id = hasToolCallId ? toolCallId : TextNode.valueOf("call_" + UUID.randomUUID());
		return withId(isError ? result : unwrapPayload(result), id);
	}

	private JsonNode unwrapPayload(JsonNode result) {
		JsonNode structured = result.path("structuredContent");
		if (structured.isObject())
			return structured;
		JsonNode content = result.path("content");
		if (content.isArray()) {
			ObjectNode wrapped = mapper.createObjectNode();
			wrapped.set("content", content);
			return wrapped;
		}
		return result;
	}

	private JsonNode withId(JsonNode payload, JsonNode id) {
		ObjectNode out = mapper.createObjectNode();
		out.set("id", id);
		if (payload.isObject())
			out.setAll((ObjectNode) payload);
		else
			out.set("value", payload);
		return out;
	}

	private String extractErrorText(JsonNode result) {
		JsonNode content = result.path("content");
		if (!content.isArray())
			return result.toString();
		StringBuilder sb = new StringBuilder();
		for (JsonNode c : content) {
			if (sb.length() > 0)
				sb.append("\n");
			sb.append(c.path("text").asText(""));
		}
		return sb.toString();
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

		if (!connectNotified) {
			connectNotified = true;
			new Thread(this::notifyConnected, "mcp-connect-notify").start();
		}
	}

	private void notifyConnected() {
		for (ConnectObserver observer : connectObservers)
			observer.onConnect(this);
	}

	private JsonNode send(ObjectNode rpc, boolean expectResult) {
		try {
			String body = mapper.writeValueAsString(rpc);
			HttpRequest.Builder builder = HttpRequest.newBuilder().uri(URI.create(SERVER_URL)).timeout(TIMEOUT)
					.header("Content-Type", "application/json").header("Accept", "application/json, text/event-stream")
					.header("X-MCPC-TOOLS", "all").header("X-MCPC-CONTROL", "off")
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
