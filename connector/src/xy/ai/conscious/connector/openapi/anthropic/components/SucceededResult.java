package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class SucceededResult {

    private final JsonNode node;

    public SucceededResult(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public SucceededResult() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.Message getMessage() {
        JsonNode child = node.get("message");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.Message(child);
    }


    public void setMessage(xy.ai.conscious.connector.openapi.anthropic.components.Message value) {
        ((ObjectNode) node).set("message", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.SucceededEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.SucceededEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.SucceededEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
