package xy.ai.conscious.connector.openapi.anthropic.components;

import java.util.Iterator;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node used as an untyped string-keyed map; holds no data of its own. */
public final class SchemaAnyDict {

    private final JsonNode node;

    public SchemaAnyDict(JsonNode node) {
        this.node = node;
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

    public JsonNode get(String key) {
        return node.get(key);
    }

    public void put(String key, JsonNode value) {
        ((ObjectNode) node).set(key, value);
    }

    public void remove(String key) {
        ((ObjectNode) node).remove(key);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
