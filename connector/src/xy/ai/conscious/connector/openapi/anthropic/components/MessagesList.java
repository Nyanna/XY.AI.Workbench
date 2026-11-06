package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;

/** Proxy over a JSON array node; holds no data of its own. */
public final class MessagesList {

    private final JsonNode node;

    public MessagesList(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON array; populate it via add(...). */
    public MessagesList() {
        this(JsonNodeFactory.instance.arrayNode());
    }

    public JsonNode node() {
        return node;
    }

    public int size() {
        return node.size();
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.InputMessage get(int index) {
        JsonNode child = node.get(index);
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.InputMessage(child);
    }


    public void add(xy.ai.conscious.connector.openapi.anthropic.components.InputMessage value) {
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
