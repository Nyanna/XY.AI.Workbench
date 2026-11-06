package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same oneOf node (no merge). */
public final class OneOfThinking {

    private final JsonNode node;

    public OneOfThinking(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the ThinkingConfigEnabled branch. */
    public boolean isThinkingConfigEnabled() {
        return "enabled".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ThinkingConfigEnabled getThinkingConfigEnabled() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ThinkingConfigEnabled(node);
    }

    /** Whether the bound node currently matches the ThinkingConfigDisabled branch. */
    public boolean isThinkingConfigDisabled() {
        return "disabled".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ThinkingConfigDisabled getThinkingConfigDisabled() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ThinkingConfigDisabled(node);
    }

    /** Whether the bound node currently matches the ThinkingConfigAdaptive branch. */
    public boolean isThinkingConfigAdaptive() {
        return "adaptive".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ThinkingConfigAdaptive getThinkingConfigAdaptive() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ThinkingConfigAdaptive(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
