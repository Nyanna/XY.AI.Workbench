package xy.ai.conscious.connector.openapi.anthropic;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.net.URLDecoder;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;

/**
 * Transport-agnostic server base class (JDK {@code com.sun.net.httpserver} stdlib, no
 * external framework). Implement one abstract method per operation -- each has the exact
 * same signature (parameters and {@code XxxResponse} return type) as the corresponding
 * client-interface method, so no HTTP type (status code, content type, ...) ever appears
 * in your implementation. Build the returned response via its no-arg constructor plus one
 * of its setCode&lt;NNN&gt;(...) methods.
 */
public abstract class V1Server {
    private static final Pattern PATTERN_MESSAGES_POST = Pattern.compile("/v1/messages");
    private static final Pattern PATTERN_MESSAGE_BATCHES_LIST = Pattern.compile("/v1/messages/batches");
    private static final Pattern PATTERN_MESSAGE_BATCHES_POST = Pattern.compile("/v1/messages/batches");
    private static final Pattern PATTERN_MESSAGE_BATCHES_DELETE = Pattern.compile("/v1/messages/batches/([^/]+)");
    private static final Pattern PATTERN_MESSAGE_BATCHES_RETRIEVE = Pattern.compile("/v1/messages/batches/([^/]+)");
    private static final Pattern PATTERN_MESSAGE_BATCHES_CANCEL = Pattern.compile("/v1/messages/batches/([^/]+)/cancel");
    private static final Pattern PATTERN_MESSAGE_BATCHES_RESULTS = Pattern.compile("/v1/messages/batches/([^/]+)/results");

    /**
     * Send a structured list of input messages with text and/or image content, and the model will generate the next message in the conversation.
     * 
     * The Messages API can be used for either single queries or stateless multi-turn conversations.
     * 
     * Learn more about the Messages API in our user guide
     */

    protected abstract xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.V1MessagesResponse messages_post(xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams request);

    /**
     * List all Message Batches within a Workspace. Most recently created batches are returned first.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    protected abstract xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.get.V1MessagesBatchesResponse message_batches_list(String beforeId, String afterId, Long limit);

    /**
     * Send a batch of Message creation requests.
     * 
     * The Message Batches API can be used to process multiple Messages API requests at once. Once a Message Batch is created, it begins processing immediately. Batches can take up to 24 hours to complete.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    protected abstract xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.post.V1MessagesBatchesResponse message_batches_post(xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.CreateMessageBatchParams request);

    /**
     * Delete a Message Batch.
     * 
     * Message Batches can only be deleted once they've finished processing. If you'd like to delete an in-progress batch, you must first cancel it.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    protected abstract xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.V1MessagesBatchesMessageBatchIdResponse message_batches_delete(String messageBatchId);

    /**
     * This endpoint is idempotent and can be used to poll for Message Batch completion. To access the results of a Message Batch, make a request to the `results_url` field in the response.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    protected abstract xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.get.V1MessagesBatchesMessageBatchIdResponse message_batches_retrieve(String messageBatchId);

    /**
     * Batches may be canceled any time before processing ends. Once cancellation is initiated, the batch enters a `canceling` state, at which time the system may complete any in-progress, non-interruptible requests before finalizing cancellation.
     * 
     * The number of canceled requests is specified in `request_counts`. To determine which requests were canceled, check the individual results within the batch. Note that cancellation may not result in any canceled requests if they were non-interruptible.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    protected abstract xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.cancel.V1MessagesBatchesMessageBatchIdCancelResponse message_batches_cancel(String messageBatchId);

    /**
     * Streams the results of a Message Batch as a `.jsonl` file.
     * 
     * Each line in the file is a JSON object containing the result of a single request in the Message Batch. Results are not guaranteed to be in the same order as requests. Use the `custom_id` field to match results to requests.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    protected abstract xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.results.V1MessagesBatchesMessageBatchIdResultsResponse message_batches_results(String messageBatchId);

    /** Binds and starts a JDK {@link HttpServer} on the given port, routed by {@link #createHttpHandler()}. */
    public HttpServer start(int port) throws IOException {
        HttpServer server = HttpServer.create(new InetSocketAddress(port), 0);
        server.createContext("/", createHttpHandler());
        server.start();
        return server;
    }

    /** The routing {@link HttpHandler}; wire it into your own {@link HttpServer} setup if {@link #start} isn't used. */
    public HttpHandler createHttpHandler() {
        return this::handle;
    }

    private void handle(HttpExchange exchange) throws IOException {
        String method = exchange.getRequestMethod();
        String path = exchange.getRequestURI().getPath();
        @SuppressWarnings("unused")
        Map<String, String> queryParams = parseQuery(exchange.getRequestURI().getRawQuery());
        try {
            {
                Matcher matcher = PATTERN_MESSAGES_POST.matcher(path);
                if ("POST".equalsIgnoreCase(method) && matcher.matches()) {
                    String rawBody = readBody(exchange);
                    xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams request = new xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams(xy.ai.conscious.connector.openapi.anthropic.JsonSupport.parse(rawBody));
                    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.V1MessagesResponse response = messages_post(request);
                    writeResult(exchange, response.statusCode(), response.contentType(), response.body());
                    return;
                }
            }
            {
                Matcher matcher = PATTERN_MESSAGE_BATCHES_LIST.matcher(path);
                if ("GET".equalsIgnoreCase(method) && matcher.matches()) {
                    String beforeIdRaw = queryParams.get("before_id");
                    String beforeId = beforeIdRaw == null ? null : beforeIdRaw;
                    String afterIdRaw = queryParams.get("after_id");
                    String afterId = afterIdRaw == null ? null : afterIdRaw;
                    String limitRaw = queryParams.get("limit");
                    Long limit = limitRaw == null ? null : Long.valueOf(limitRaw);
                    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.get.V1MessagesBatchesResponse response = message_batches_list(beforeId, afterId, limit);
                    writeResult(exchange, response.statusCode(), response.contentType(), response.body());
                    return;
                }
            }
            {
                Matcher matcher = PATTERN_MESSAGE_BATCHES_POST.matcher(path);
                if ("POST".equalsIgnoreCase(method) && matcher.matches()) {
                    String rawBody = readBody(exchange);
                    xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.CreateMessageBatchParams request = new xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.CreateMessageBatchParams(xy.ai.conscious.connector.openapi.anthropic.JsonSupport.parse(rawBody));
                    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.post.V1MessagesBatchesResponse response = message_batches_post(request);
                    writeResult(exchange, response.statusCode(), response.contentType(), response.body());
                    return;
                }
            }
            {
                Matcher matcher = PATTERN_MESSAGE_BATCHES_DELETE.matcher(path);
                if ("DELETE".equalsIgnoreCase(method) && matcher.matches()) {
                    String messageBatchId = URLDecoder.decode(matcher.group(1), StandardCharsets.UTF_8);
                    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.V1MessagesBatchesMessageBatchIdResponse response = message_batches_delete(messageBatchId);
                    writeResult(exchange, response.statusCode(), response.contentType(), response.body());
                    return;
                }
            }
            {
                Matcher matcher = PATTERN_MESSAGE_BATCHES_RETRIEVE.matcher(path);
                if ("GET".equalsIgnoreCase(method) && matcher.matches()) {
                    String messageBatchId = URLDecoder.decode(matcher.group(1), StandardCharsets.UTF_8);
                    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.get.V1MessagesBatchesMessageBatchIdResponse response = message_batches_retrieve(messageBatchId);
                    writeResult(exchange, response.statusCode(), response.contentType(), response.body());
                    return;
                }
            }
            {
                Matcher matcher = PATTERN_MESSAGE_BATCHES_CANCEL.matcher(path);
                if ("POST".equalsIgnoreCase(method) && matcher.matches()) {
                    String messageBatchId = URLDecoder.decode(matcher.group(1), StandardCharsets.UTF_8);
                    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.cancel.V1MessagesBatchesMessageBatchIdCancelResponse response = message_batches_cancel(messageBatchId);
                    writeResult(exchange, response.statusCode(), response.contentType(), response.body());
                    return;
                }
            }
            {
                Matcher matcher = PATTERN_MESSAGE_BATCHES_RESULTS.matcher(path);
                if ("GET".equalsIgnoreCase(method) && matcher.matches()) {
                    String messageBatchId = URLDecoder.decode(matcher.group(1), StandardCharsets.UTF_8);
                    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.results.V1MessagesBatchesMessageBatchIdResultsResponse response = message_batches_results(messageBatchId);
                    writeResult(exchange, response.statusCode(), response.contentType(), response.body());
                    return;
                }
            }
            writeResult(exchange, "404", "text/plain", "not found: " + method + " " + path);
        } catch (Exception e) {
            writeResult(exchange, "500", "text/plain", String.valueOf(e.getMessage()));
        }
    }

    private static Map<String, String> parseQuery(String rawQuery) {
        Map<String, String> result = new HashMap<>();
        if (rawQuery == null || rawQuery.isEmpty()) {
            return result;
        }
        for (String pair : rawQuery.split("&")) {
            int idx = pair.indexOf('=');
            String key = idx < 0 ? pair : pair.substring(0, idx);
            String value = idx < 0 ? "" : pair.substring(idx + 1);
            result.put(URLDecoder.decode(key, StandardCharsets.UTF_8), URLDecoder.decode(value, StandardCharsets.UTF_8));
        }
        return result;
    }

    private static String readBody(HttpExchange exchange) throws IOException {
        try (InputStream is = exchange.getRequestBody()) {
            return new String(is.readAllBytes(), StandardCharsets.UTF_8);
        }
    }

    private static void writeResult(HttpExchange exchange, String statusCode, String contentType, String body) throws IOException {
        byte[] bytes = body == null ? new byte[0] : body.getBytes(StandardCharsets.UTF_8);
        if (contentType != null) {
            exchange.getResponseHeaders().set("Content-Type", contentType);
        }
        int code;
        try {
            code = Integer.parseInt(statusCode);
        } catch (NumberFormatException e) {
            code = 500;
        }
        exchange.sendResponseHeaders(code, bytes.length);
        try (OutputStream os = exchange.getResponseBody()) {
            os.write(bytes);
        }
    }
}
