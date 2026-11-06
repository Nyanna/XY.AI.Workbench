package xy.ai.conscious.connector.openapi.anthropic.lists;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;

/** Proxy over a JSON array node; holds no data of its own. */
public final class RequiredList {

    private final JsonNode node;

    public RequiredList(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON array; populate it via add(...). */
    public RequiredList() {
        this(JsonNodeFactory.instance.arrayNode());
    }

    public JsonNode node() {
        return node;
    }

    public int size() {
        return node.size();
    }


    public String get(int index) {
        JsonNode child = node.get(index);
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }


    public void add(String value) {
        ((ArrayNode) node).add(value);
    }

    public void remove(int index) {
        ((ArrayNode) node).remove(index);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
