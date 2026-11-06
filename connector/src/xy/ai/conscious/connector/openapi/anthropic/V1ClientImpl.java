package xy.ai.conscious.connector.openapi.anthropic;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

/**
 * {@code java.net.http.HttpClient}-based implementation of {@link xy.ai.conscious.connector.openapi.anthropic.V1Client}.
 * Base URL is a constructor parameter; extend this class and override
 * {@link #customizeRequest} to inject Authorization or other headers --
 * no concrete auth code lives in this class itself.
 */
public class V1ClientImpl implements xy.ai.conscious.connector.openapi.anthropic.V1Client {

    private final String baseUrl;
    private final HttpClient httpClient;

    public V1ClientImpl(String baseUrl) {
        this(baseUrl, HttpClient.newHttpClient());
    }

    public V1ClientImpl(String baseUrl, HttpClient httpClient) {
        this.baseUrl = baseUrl;
        this.httpClient = httpClient;
    }

    /** Hook for subclasses (e.g. to set Authorization); no-op by default. */
    protected void customizeRequest(HttpRequest.Builder builder) {
    }

    @Override
    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.V1MessagesResponse messages_post(xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams request) {
        StringBuilder urlBuilder = new StringBuilder(baseUrl);
        urlBuilder.append("/v1/messages");
        HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                .uri(URI.create(urlBuilder.toString()))
                .header("Content-Type", "application/json")
                .method("POST", HttpRequest.BodyPublishers.ofString(request.toString()));
        customizeRequest(requestBuilder);
        try {
            HttpResponse<String> httpResponse = httpClient.send(requestBuilder.build(), HttpResponse.BodyHandlers.ofString());
            String responseContentType = httpResponse.headers().firstValue("Content-Type").orElse(null);
            return xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.V1MessagesResponse.from(httpResponse.body(), httpResponse.statusCode(), responseContentType);
        } catch (java.io.IOException | InterruptedException e) {
            throw new RuntimeException("request failed: messages_post", e);
        }
    }

    @Override
    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.get.V1MessagesBatchesResponse message_batches_list(String beforeId, String afterId, Long limit) {
        StringBuilder urlBuilder = new StringBuilder(baseUrl);
        urlBuilder.append("/v1/messages/batches");
        java.util.List<String> queryParts = new java.util.ArrayList<>();
        if (beforeId != null) {
            queryParts.add("before_id=" + java.net.URLEncoder.encode(String.valueOf(beforeId), java.nio.charset.StandardCharsets.UTF_8));
        }
        if (afterId != null) {
            queryParts.add("after_id=" + java.net.URLEncoder.encode(String.valueOf(afterId), java.nio.charset.StandardCharsets.UTF_8));
        }
        if (limit != null) {
            queryParts.add("limit=" + java.net.URLEncoder.encode(String.valueOf(limit), java.nio.charset.StandardCharsets.UTF_8));
        }
        if (!queryParts.isEmpty()) {
            urlBuilder.append("?").append(String.join("&", queryParts));
        }
        HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                .uri(URI.create(urlBuilder.toString()))
                .method("GET", HttpRequest.BodyPublishers.noBody());
        customizeRequest(requestBuilder);
        try {
            HttpResponse<String> httpResponse = httpClient.send(requestBuilder.build(), HttpResponse.BodyHandlers.ofString());
            String responseContentType = httpResponse.headers().firstValue("Content-Type").orElse(null);
            return xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.get.V1MessagesBatchesResponse.from(httpResponse.body(), httpResponse.statusCode(), responseContentType);
        } catch (java.io.IOException | InterruptedException e) {
            throw new RuntimeException("request failed: message_batches_list", e);
        }
    }

    @Override
    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.post.V1MessagesBatchesResponse message_batches_post(xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.CreateMessageBatchParams request) {
        StringBuilder urlBuilder = new StringBuilder(baseUrl);
        urlBuilder.append("/v1/messages/batches");
        HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                .uri(URI.create(urlBuilder.toString()))
                .header("Content-Type", "application/json")
                .method("POST", HttpRequest.BodyPublishers.ofString(request.toString()));
        customizeRequest(requestBuilder);
        try {
            HttpResponse<String> httpResponse = httpClient.send(requestBuilder.build(), HttpResponse.BodyHandlers.ofString());
            String responseContentType = httpResponse.headers().firstValue("Content-Type").orElse(null);
            return xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.post.V1MessagesBatchesResponse.from(httpResponse.body(), httpResponse.statusCode(), responseContentType);
        } catch (java.io.IOException | InterruptedException e) {
            throw new RuntimeException("request failed: message_batches_post", e);
        }
    }

    @Override
    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.V1MessagesBatchesMessageBatchIdResponse message_batches_delete(String messageBatchId) {
        StringBuilder urlBuilder = new StringBuilder(baseUrl);
        urlBuilder.append("/v1/messages/batches/" + java.net.URLEncoder.encode(String.valueOf(messageBatchId), java.nio.charset.StandardCharsets.UTF_8));
        HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                .uri(URI.create(urlBuilder.toString()))
                .method("DELETE", HttpRequest.BodyPublishers.noBody());
        customizeRequest(requestBuilder);
        try {
            HttpResponse<String> httpResponse = httpClient.send(requestBuilder.build(), HttpResponse.BodyHandlers.ofString());
            String responseContentType = httpResponse.headers().firstValue("Content-Type").orElse(null);
            return xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.V1MessagesBatchesMessageBatchIdResponse.from(httpResponse.body(), httpResponse.statusCode(), responseContentType);
        } catch (java.io.IOException | InterruptedException e) {
            throw new RuntimeException("request failed: message_batches_delete", e);
        }
    }

    @Override
    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.get.V1MessagesBatchesMessageBatchIdResponse message_batches_retrieve(String messageBatchId) {
        StringBuilder urlBuilder = new StringBuilder(baseUrl);
        urlBuilder.append("/v1/messages/batches/" + java.net.URLEncoder.encode(String.valueOf(messageBatchId), java.nio.charset.StandardCharsets.UTF_8));
        HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                .uri(URI.create(urlBuilder.toString()))
                .method("GET", HttpRequest.BodyPublishers.noBody());
        customizeRequest(requestBuilder);
        try {
            HttpResponse<String> httpResponse = httpClient.send(requestBuilder.build(), HttpResponse.BodyHandlers.ofString());
            String responseContentType = httpResponse.headers().firstValue("Content-Type").orElse(null);
            return xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.get.V1MessagesBatchesMessageBatchIdResponse.from(httpResponse.body(), httpResponse.statusCode(), responseContentType);
        } catch (java.io.IOException | InterruptedException e) {
            throw new RuntimeException("request failed: message_batches_retrieve", e);
        }
    }

    @Override
    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.cancel.V1MessagesBatchesMessageBatchIdCancelResponse message_batches_cancel(String messageBatchId) {
        StringBuilder urlBuilder = new StringBuilder(baseUrl);
        urlBuilder.append("/v1/messages/batches/" + java.net.URLEncoder.encode(String.valueOf(messageBatchId), java.nio.charset.StandardCharsets.UTF_8) + "/cancel");
        HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                .uri(URI.create(urlBuilder.toString()))
                .method("POST", HttpRequest.BodyPublishers.noBody());
        customizeRequest(requestBuilder);
        try {
            HttpResponse<String> httpResponse = httpClient.send(requestBuilder.build(), HttpResponse.BodyHandlers.ofString());
            String responseContentType = httpResponse.headers().firstValue("Content-Type").orElse(null);
            return xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.cancel.V1MessagesBatchesMessageBatchIdCancelResponse.from(httpResponse.body(), httpResponse.statusCode(), responseContentType);
        } catch (java.io.IOException | InterruptedException e) {
            throw new RuntimeException("request failed: message_batches_cancel", e);
        }
    }

    @Override
    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.results.V1MessagesBatchesMessageBatchIdResultsResponse message_batches_results(String messageBatchId) {
        StringBuilder urlBuilder = new StringBuilder(baseUrl);
        urlBuilder.append("/v1/messages/batches/" + java.net.URLEncoder.encode(String.valueOf(messageBatchId), java.nio.charset.StandardCharsets.UTF_8) + "/results");
        HttpRequest.Builder requestBuilder = HttpRequest.newBuilder()
                .uri(URI.create(urlBuilder.toString()))
                .method("GET", HttpRequest.BodyPublishers.noBody());
        customizeRequest(requestBuilder);
        try {
            HttpResponse<String> httpResponse = httpClient.send(requestBuilder.build(), HttpResponse.BodyHandlers.ofString());
            String responseContentType = httpResponse.headers().firstValue("Content-Type").orElse(null);
            return xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.results.V1MessagesBatchesMessageBatchIdResultsResponse.from(httpResponse.body(), httpResponse.statusCode(), responseContentType);
        } catch (java.io.IOException | InterruptedException e) {
            throw new RuntimeException("request failed: message_batches_results", e);
        }
    }
}
