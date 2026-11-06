package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same oneOf node (no merge). */
public final class OneOfContentElement {

    private final JsonNode node;

    public OneOfContentElement(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the RequestTextBlock branch. */
    public boolean isRequestTextBlock() {
        return "text".equals(node.path("type").asText());
    }

    /**
     * Regular text content.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.RequestTextBlock getRequestTextBlock() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.RequestTextBlock(node);
    }

    /** Whether the bound node currently matches the RequestThinkingBlock branch. */
    public boolean isRequestThinkingBlock() {
        return "thinking".equals(node.path("type").asText());
    }

    /**
     * A block specifying internal thinking by the model.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.RequestThinkingBlock getRequestThinkingBlock() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.RequestThinkingBlock(node);
    }

    /** Whether the bound node currently matches the RequestServerToolUseBlock branch. */
    public boolean isRequestServerToolUseBlock() {
        return "server_tool_use".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.RequestServerToolUseBlock getRequestServerToolUseBlock() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.RequestServerToolUseBlock(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
