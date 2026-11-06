package xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.post.code200.json;

import com.fasterxml.jackson.databind.JsonNode;

/** One status-code view; the content-type header selects which typed getter applies. */
public final class V1MessagesBatchesResponseCode200Json {

    private final JsonNode node;
    private final String contentType;

    public V1MessagesBatchesResponseCode200Json(JsonNode node, String contentType) {
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


    public xy.ai.conscious.connector.openapi.anthropic.components.MessageBatch getJson() {
        if (!isJson()) {
            return null;
        }
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.MessageBatch(node);
    }
}
