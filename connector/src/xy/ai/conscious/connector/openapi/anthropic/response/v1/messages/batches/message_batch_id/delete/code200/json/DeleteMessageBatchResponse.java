package xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code200.json;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class DeleteMessageBatchResponse {

    private final JsonNode node;

    public DeleteMessageBatchResponse(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public DeleteMessageBatchResponse() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * ID of the Message Batch.
     */

    public String getId() {
        JsonNode child = node.get("id");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }

    /**
     * ID of the Message Batch.
     */

    public void setId(String value) {
        ((ObjectNode) node).put("id", value);
    }

    /**
     * Deleted object type.
     * 
     * For Message Batches, this is always `"message_batch_deleted"`.
     */

    public xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code200.json.TypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code200.json.TypeEnum.fromValue(child.asText());
    }

    /**
     * Deleted object type.
     * 
     * For Message Batches, this is always `"message_batch_deleted"`.
     */

    public void setType(xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.code200.json.TypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
