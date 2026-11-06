package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class ResponseToolUseBlock {

    private final JsonNode node;

    public ResponseToolUseBlock(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public ResponseToolUseBlock() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }


    public xy.ai.conscious.connector.openapi.anthropic.operators.OneOfCaller getCaller() {
        JsonNode child = node.get("caller");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.OneOfCaller(child);
    }


    public void setCaller(xy.ai.conscious.connector.openapi.anthropic.operators.OneOfCaller value) {
        ((ObjectNode) node).set("caller", value == null ? null : value.node());
    }


    public String getId() {
        JsonNode child = node.get("id");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }


    public void setId(String value) {
        ((ObjectNode) node).put("id", value);
    }


    public com.fasterxml.jackson.databind.JsonNode getInput() {
        return node.get("input");
    }


    public void setInput(com.fasterxml.jackson.databind.JsonNode value) {
        ((ObjectNode) node).set("input", value);
    }


    public String getName() {
        JsonNode child = node.get("name");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }


    public void setName(String value) {
        ((ObjectNode) node).put("name", value);
    }

    /**
     * For a toolset member tool_use, the toolset family.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getToolsetName() {
        JsonNode child = node.get("toolset_name");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * For a toolset member tool_use, the toolset family.
     */

    public void setToolsetName(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("toolset_name", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.MessageContentElementTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.MessageContentElementTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.MessageContentElementTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
