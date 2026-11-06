package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same oneOf node (no merge). */
public final class OneOfToolChoice {

    private final JsonNode node;

    public OneOfToolChoice(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the ToolChoiceAuto branch. */
    public boolean isToolChoiceAuto() {
        return "auto".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ToolChoiceAuto getToolChoiceAuto() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ToolChoiceAuto(node);
    }

    /** Whether the bound node currently matches the ToolChoiceAny branch. */
    public boolean isToolChoiceAny() {
        return "any".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ToolChoiceAny getToolChoiceAny() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ToolChoiceAny(node);
    }

    /** Whether the bound node currently matches the ToolChoiceTool branch. */
    public boolean isToolChoiceTool() {
        return "tool".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ToolChoiceTool getToolChoiceTool() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ToolChoiceTool(node);
    }

    /** Whether the bound node currently matches the ToolChoiceNone branch. */
    public boolean isToolChoiceNone() {
        return "none".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ToolChoiceNone getToolChoiceNone() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ToolChoiceNone(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
