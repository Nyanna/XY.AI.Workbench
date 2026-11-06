package xy.ai.conscious.connector.openapi.anthropic.operators;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfCacheCreationInputTokens {

    private final JsonNode node;

    public AnyOfCacheCreationInputTokens(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the Integer branch. */
    public boolean isInteger() {
        return node.isIntegralNumber();
    }


    public Long getInteger() {
        if (node.isNull()) {
            return null;
        }
        return node.asLong();
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
