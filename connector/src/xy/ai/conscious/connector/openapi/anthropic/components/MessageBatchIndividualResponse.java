package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class MessageBatchIndividualResponse {

    private final JsonNode node;

    public MessageBatchIndividualResponse(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public MessageBatchIndividualResponse() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * Developer-provided ID created for each request in a Message Batch. Useful for matching results to requests, as results may be given out of request order.
     * 
     * Must be unique for each request within the Message Batch.
     */

    public String getCustomId() {
        JsonNode child = node.get("custom_id");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }

    /**
     * Developer-provided ID created for each request in a Message Batch. Useful for matching results to requests, as results may be given out of request order.
     * 
     * Must be unique for each request within the Message Batch.
     */

    public void setCustomId(String value) {
        ((ObjectNode) node).put("custom_id", value);
    }

    /**
     * Processing result for this request.
     * 
     * Contains a Message output if processing was successful, an error response if processing failed, or the reason why processing was not attempted, such as cancellation or expiration.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.OneOfResult getResult() {
        JsonNode child = node.get("result");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.OneOfResult(child);
    }

    /**
     * Processing result for this request.
     * 
     * Contains a Message output if processing was successful, an error response if processing failed, or the reason why processing was not attempted, such as cancellation or expiration.
     */

    public void setResult(xy.ai.conscious.connector.openapi.anthropic.components.OneOfResult value) {
        ((ObjectNode) node).set("result", value == null ? null : value.node());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
