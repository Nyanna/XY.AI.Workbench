package xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code4XX.json;

import com.fasterxml.jackson.databind.JsonNode;

/** One status-code view; the content-type header selects which typed getter applies. */
public final class V1MessagesBatchesMessageBatchIdResponseCode4XXJson {

    private final JsonNode node;
    private final String contentType;

    public V1MessagesBatchesMessageBatchIdResponseCode4XXJson(JsonNode node, String contentType) {
        this.node = node;
        this.contentType = contentType;
    }

    public String contentType() {
        return contentType;
    }

    /** Whether the bound content type is "application/json". */
    public boolean isJson() {
        return "application/json".equals(contentType);
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ErrorResponse getJson() {
        if (!isJson()) {
            return null;
        }
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ErrorResponse(node);
    }
}
