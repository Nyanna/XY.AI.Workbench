package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfOutputTokensDetails {

    private final JsonNode node;

    public AnyOfOutputTokensDetails(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the OutputTokensDetails branch. */
    public boolean isOutputTokensDetails() {
        return node.has("thinking_tokens");
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.OutputTokensDetails getOutputTokensDetails() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.OutputTokensDetails(node);
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
