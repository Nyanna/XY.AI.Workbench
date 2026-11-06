package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class OverloadedError {

    private final JsonNode node;

    public OverloadedError(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public OverloadedError() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }


    public String getMessage() {
        JsonNode child = node.get("message");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }


    public void setMessage(String value) {
        ((ObjectNode) node).put("message", value);
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.OverloadedErrorEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.OverloadedErrorEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.OverloadedErrorEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
