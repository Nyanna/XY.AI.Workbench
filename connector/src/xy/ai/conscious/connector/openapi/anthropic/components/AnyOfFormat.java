package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfFormat {

    private final JsonNode node;

    public AnyOfFormat(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the JsonOutputFormat branch. */
    public boolean isJsonOutputFormat() {
        return node.has("schema") && node.has("type");
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.JsonOutputFormat getJsonOutputFormat() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.JsonOutputFormat(node);
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
