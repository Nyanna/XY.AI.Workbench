package xy.ai.conscious.connector.openapi.anthropic.operators;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfDisplay {

    private final JsonNode node;

    public AnyOfDisplay(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the DisplayEnum branch. */
    public boolean isDisplayEnum() {
        return node.isTextual();
    }


    public xy.ai.conscious.connector.openapi.anthropic.operators.DisplayEnum getDisplayEnum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.operators.DisplayEnum.fromValue(node.asText());
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
