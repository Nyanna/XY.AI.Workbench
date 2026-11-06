package xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;

/** Proxy over a JSON array node; holds no data of its own. */
public final class RequestsList {

    private final JsonNode node;

    public RequestsList(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON array; populate it via add(...). */
    public RequestsList() {
        this(JsonNodeFactory.instance.arrayNode());
    }

    public JsonNode node() {
        return node;
    }

    public int size() {
        return node.size();
    }


    public xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.MessageBatchIndividualRequestParams get(int index) {
        JsonNode child = node.get(index);
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.MessageBatchIndividualRequestParams(child);
    }


    public void add(xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.MessageBatchIndividualRequestParams value) {
        ((ArrayNode) node).add(value == null ? null : value.node());
    }

    public void remove(int index) {
        ((ArrayNode) node).remove(index);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
