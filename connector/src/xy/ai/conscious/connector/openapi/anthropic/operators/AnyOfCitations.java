package xy.ai.conscious.connector.openapi.anthropic.operators;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfCitations {

    private final JsonNode node;

    public AnyOfCitations(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the CitationsList branch. */
    public boolean isCitationsList() {
        return node.isArray();
    }


    public xy.ai.conscious.connector.openapi.anthropic.operators.CitationsList getCitationsList() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.CitationsList(node);
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
