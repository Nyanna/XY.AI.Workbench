package xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class CreateMessageBatchParams {

    private final JsonNode node;

    public CreateMessageBatchParams(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public CreateMessageBatchParams() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * List of requests for prompt completion. Each is an individual request to create a Message.
     */

    public xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.RequestsList getRequests() {
        JsonNode child = node.get("requests");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.RequestsList(child);
    }

    /**
     * List of requests for prompt completion. Each is an individual request to create a Message.
     */

    public void setRequests(xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.RequestsList value) {
        ((ObjectNode) node).set("requests", value == null ? null : value.node());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }

    public static CreateMessageBatchParams fromString(String body) {
        return new CreateMessageBatchParams(xy.ai.conscious.connector.openapi.anthropic.JsonSupport.parse(body));
    }
}
