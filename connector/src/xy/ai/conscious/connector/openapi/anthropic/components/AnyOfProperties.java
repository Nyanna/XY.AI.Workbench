package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfProperties {

    private final JsonNode node;

    public AnyOfProperties(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the SchemaAnyDict branch. */
    public boolean isSchemaAnyDict() {
        return node.isObject();
    }


    public com.fasterxml.jackson.databind.JsonNode getSchemaAnyDict() {
        return node;
    }

    /** Whether the bound node currently matches the Null branch. */
    public boolean isNull() {
        return node.isNull();
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
