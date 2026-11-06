package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class RequestTextBlock {

    private final JsonNode node;

    public RequestTextBlock(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public RequestTextBlock() {
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


    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCitations getCitations() {
        JsonNode child = node.get("citations");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCitations(child);
    }


    public void setCitations(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCitations value) {
        ((ObjectNode) node).set("citations", value == null ? null : value.node());
    }


    public String getText() {
        JsonNode child = node.get("text");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }


    public void setText(String value) {
        ((ObjectNode) node).put("text", value);
    }


    public xy.ai.conscious.connector.openapi.anthropic.enums.ElementTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.enums.ElementTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.enums.ElementTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
