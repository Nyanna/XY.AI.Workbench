package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfContent {

    private final JsonNode node;

    public AnyOfContent(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the String branch. */
    public boolean isString() {
        return node.isTextual();
    }


    public String getString() {
        if (node.isNull()) {
            return null;
        }
        return node.asText();
    }

    /** Whether the bound node currently matches the ContentList branch. */
    public boolean isContentList() {
        return node.isArray();
    }

    /**
     * Example: [{'type': 'text', 'text': 'What is a quaternion?'}]
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ContentList getContentList() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ContentList(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
