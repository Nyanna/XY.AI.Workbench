package xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.NullNode;

/** Root response object; status code and content type are transport metadata, never part of the body.
 * Client-side, use {@link #from}; server-side, use the no-arg constructor plus one of the
 * setCode&lt;NNN&gt;(...) methods to construct the exact response your operation implementation returns. */
@SuppressWarnings("unused")
public final class V1MessagesBatchesMessageBatchIdResponse {

    private JsonNode node;
    private String statusCode;
    private String contentType;

    /** For server-side construction: follow up with a setCode&lt;NNN&gt;(...) call. */
    public V1MessagesBatchesMessageBatchIdResponse() {
    }

    private V1MessagesBatchesMessageBatchIdResponse(JsonNode node, int statusCode, String contentType) {
        this.node = node;
        this.statusCode = String.valueOf(statusCode);
        this.contentType = contentType;
    }

    public static V1MessagesBatchesMessageBatchIdResponse from(String body, int statusCode, String contentType) {
        return new V1MessagesBatchesMessageBatchIdResponse(xy.ai.conscious.connector.openapi.anthropic.JsonSupport.parse(body), statusCode, contentType);
    }

    public String statusCode() {
        return statusCode;
    }

    public String contentType() {
        return contentType;
    }
    
    /** Response body as JSON text, or an empty string if there is none. */
    public String body() {
        return node == null ? "" : node.toString();
    }

    /** Present only if the response's status code is 200. */
    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code200.json.V1MessagesBatchesMessageBatchIdResponseCode200Json getCode200() {
        if (!statusCode.equals("200")) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code200.json.V1MessagesBatchesMessageBatchIdResponseCode200Json(node, contentType);
    }

    /** Present only if the response's status code is 4XX. */
    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code4XX.json.V1MessagesBatchesMessageBatchIdResponseCode4XXJson getCode4XX() {
        if (!statusCode.equals("4XX")) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code4XX.json.V1MessagesBatchesMessageBatchIdResponseCode4XXJson(node, contentType);
    }

    /** Builds a 200 / "application/json" response from an already-typed body value. */
    public void setCode200(xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code200.json.DeleteMessageBatchResponse value) {
        this.node = value == null ? NullNode.instance : value.node();
        this.statusCode = "200";
        this.contentType = "application/json";
    }

    /** Builds a 4XX / "application/json" response from an already-typed body value. */
    public void setCode4XX(xy.ai.conscious.connector.openapi.anthropic.components.ErrorResponse value) {
        this.node = value == null ? NullNode.instance : value.node();
        this.statusCode = "4XX";
        this.contentType = "application/json";
    }
}
