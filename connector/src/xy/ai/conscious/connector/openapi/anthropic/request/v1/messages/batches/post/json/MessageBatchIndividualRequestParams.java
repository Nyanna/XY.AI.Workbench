package xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class MessageBatchIndividualRequestParams {

    private final JsonNode node;

    public MessageBatchIndividualRequestParams(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public MessageBatchIndividualRequestParams() {
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
     * Messages API creation parameters for the individual request.
     * 
     * See the [Messages API reference](https://platform.claude.com/docs/en/api/messages) for full documentation on available parameters.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams getParams() {
        JsonNode child = node.get("params");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams(child);
    }

    /**
     * Messages API creation parameters for the individual request.
     * 
     * See the [Messages API reference](https://platform.claude.com/docs/en/api/messages) for full documentation on available parameters.
     */

    public void setParams(xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams value) {
        ((ObjectNode) node).set("params", value == null ? null : value.node());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
