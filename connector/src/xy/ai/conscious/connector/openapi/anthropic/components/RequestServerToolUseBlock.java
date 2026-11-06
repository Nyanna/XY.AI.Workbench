package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class RequestServerToolUseBlock {

    private final JsonNode node;

    public RequestServerToolUseBlock(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public RequestServerToolUseBlock() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * Create a cache control breakpoint at this content block.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheControl getCacheControl() {
        JsonNode child = node.get("cache_control");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheControl(child);
    }

    /**
     * Create a cache control breakpoint at this content block.
     */

    public void setCacheControl(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheControl value) {
        ((ObjectNode) node).set("cache_control", value == null ? null : value.node());
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


    public xy.ai.conscious.connector.openapi.anthropic.components.NameEnum getName() {
        JsonNode child = node.get("name");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.NameEnum.fromValue(child.asText());
    }


    public void setName(xy.ai.conscious.connector.openapi.anthropic.components.NameEnum value) {
        ((ObjectNode) node).put("name", value == null ? null : value.rawValue());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ContentElementTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ContentElementTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.ContentElementTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
