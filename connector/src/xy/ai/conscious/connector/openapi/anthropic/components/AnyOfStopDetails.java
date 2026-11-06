package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfStopDetails {

    private final JsonNode node;

    public AnyOfStopDetails(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the RefusalStopDetails branch. */
    public boolean isRefusalStopDetails() {
        return node.has("category") && node.has("explanation") && node.has("type");
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.RefusalStopDetails getRefusalStopDetails() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.RefusalStopDetails(node);
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
