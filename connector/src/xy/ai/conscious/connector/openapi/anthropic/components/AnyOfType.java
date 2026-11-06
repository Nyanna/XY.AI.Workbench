package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfType {

    private final JsonNode node;

    public AnyOfType(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the Null branch. */
    public boolean isNull() {
        return node.isNull();
    }

    /** Whether the bound node currently matches the ElementTypeEnum branch. */
    public boolean isElementTypeEnum() {
        return node.isTextual();
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ElementTypeEnum getElementTypeEnum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ElementTypeEnum.fromValue(node.asText());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
