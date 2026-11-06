package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class ToolChoiceTool {

    private final JsonNode node;

    public ToolChoiceTool(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public ToolChoiceTool() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * Whether to disable parallel tool use.
     * 
     * Defaults to `false`. If set to `true`, the model will output exactly one tool use.
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
     * Defaults to `false`. If set to `true`, the model will output exactly one tool use.
     */

    public void setDisableParallelToolUse(Boolean value) {
        ((ObjectNode) node).put("disable_parallel_tool_use", value);
    }

    /**
     * The name of the tool to use.
     */

    public String getName() {
        JsonNode child = node.get("name");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }

    /**
     * The name of the tool to use.
     */

    public void setName(String value) {
        ((ObjectNode) node).put("name", value);
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.RequestsElementParamsToolChoiceTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.RequestsElementParamsToolChoiceTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.RequestsElementParamsToolChoiceTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
