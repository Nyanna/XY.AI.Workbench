package xy.ai.conscious.connector.openapi.anthropic.components;

import java.util.Iterator;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node used as a string-keyed map; holds no data of its own. */
public final class ElementDict {

    private final JsonNode node;

    public ElementDict(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via put(...). */
    public ElementDict() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    public int size() {
        return node.size();
    }

    public Iterator<String> keys() {
        return node.fieldNames();
    }

    public boolean containsKey(String key) {
        return node.has(key);
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.SchemaAnyDict get(String key) {
        JsonNode child = node.get(key);
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.SchemaAnyDict(child);
    }


    public void put(String key, xy.ai.conscious.connector.openapi.anthropic.components.SchemaAnyDict value) {
        ((ObjectNode) node).set(key, value == null ? null : value.node());
    }

    public void remove(String key) {
        ((ObjectNode) node).remove(key);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
