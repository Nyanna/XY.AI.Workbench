package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same oneOf node (no merge). */
public final class OneOfMessageContentElement {

    private final JsonNode node;

    public OneOfMessageContentElement(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the ResponseTextBlock branch. */
    public boolean isResponseTextBlock() {
        return "text".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ResponseTextBlock getResponseTextBlock() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ResponseTextBlock(node);
    }

    /** Whether the bound node currently matches the ResponseThinkingBlock branch. */
    public boolean isResponseThinkingBlock() {
        return "thinking".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ResponseThinkingBlock getResponseThinkingBlock() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ResponseThinkingBlock(node);
    }

    /** Whether the bound node currently matches the ResponseToolUseBlock branch. */
    public boolean isResponseToolUseBlock() {
        return "tool_use".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ResponseToolUseBlock getResponseToolUseBlock() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ResponseToolUseBlock(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
