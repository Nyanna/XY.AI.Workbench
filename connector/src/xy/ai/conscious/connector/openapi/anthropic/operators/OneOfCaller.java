package xy.ai.conscious.connector.openapi.anthropic.operators;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same oneOf node (no merge). */
public final class OneOfCaller {

    private final JsonNode node;

    public OneOfCaller(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the DirectCaller branch. */
    public boolean isDirectCaller() {
        return "direct".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.operators.DirectCaller getDirectCaller() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.DirectCaller(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
