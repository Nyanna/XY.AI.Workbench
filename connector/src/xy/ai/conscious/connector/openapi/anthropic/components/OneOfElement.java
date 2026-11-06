package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same oneOf node (no merge). */
public final class OneOfElement {

    private final JsonNode node;

    public OneOfElement(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the Tool branch. */
    public boolean isTool() {
        return node.has("input_schema") && node.has("name");
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.Tool getTool() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.Tool(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
