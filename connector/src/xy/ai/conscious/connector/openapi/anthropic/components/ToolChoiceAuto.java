package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class ToolChoiceAuto {

    private final JsonNode node;

    public ToolChoiceAuto(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public ToolChoiceAuto() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * Whether to disable parallel tool use.
     * 
     * Defaults to `false`. If set to `true`, the model will output at most one tool use.
     */

    public Boolean getDisableParallelToolUse() {
        JsonNode child = node.get("disable_parallel_tool_use");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asBoolean();
    }

    /**
     * Whether to disable parallel tool use.
     * 
     * Defaults to `false`. If set to `true`, the model will output at most one tool use.
     */

    public void setDisableParallelToolUse(Boolean value) {
        ((ObjectNode) node).put("disable_parallel_tool_use", value);
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ParamsToolChoiceTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ParamsToolChoiceTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.ParamsToolChoiceTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
