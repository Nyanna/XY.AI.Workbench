package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class ThinkingConfigEnabled {

    private final JsonNode node;

    public ThinkingConfigEnabled(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public ThinkingConfigEnabled() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * Determines how many tokens Claude can use for its internal reasoning process. Larger budgets can enable more thorough analysis for complex problems, improving response quality.
     * 
     * Must be ≥1024 and less than `max_tokens`.
     * 
     * See [extended thinking](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) for details.
     */

    public Long getBudgetTokens() {
        JsonNode child = node.get("budget_tokens");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * Determines how many tokens Claude can use for its internal reasoning process. Larger budgets can enable more thorough analysis for complex problems, improving response quality.
     * 
     * Must be ≥1024 and less than `max_tokens`.
     * 
     * See [extended thinking](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) for details.
     */

    public void setBudgetTokens(Long value) {
        ((ObjectNode) node).put("budget_tokens", value);
    }

    /**
     * Controls how thinking content appears in the response. When set to `summarized`, thinking is returned normally. When set to `omitted`, thinking content is redacted but a signature is returned for multi-turn continuity. Defaults to `summarized`.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfDisplay getDisplay() {
        JsonNode child = node.get("display");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfDisplay(child);
    }

    /**
     * Controls how thinking content appears in the response. When set to `summarized`, thinking is returned normally. When set to `omitted`, thinking content is redacted but a signature is returned for multi-turn continuity. Defaults to `summarized`.
     */

    public void setDisplay(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfDisplay value) {
        ((ObjectNode) node).set("display", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ElementParamsThinkingTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ElementParamsThinkingTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.ElementParamsThinkingTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
