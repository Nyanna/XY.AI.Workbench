package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class InputMessage {

    private final JsonNode node;

    public InputMessage(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public InputMessage() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfContent getContent() {
        JsonNode child = node.get("content");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfContent(child);
    }


    public void setContent(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfContent value) {
        ((ObjectNode) node).set("content", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.RoleEnum getRole() {
        JsonNode child = node.get("role");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.RoleEnum.fromValue(child.asText());
    }


    public void setRole(xy.ai.conscious.connector.openapi.anthropic.components.RoleEnum value) {
        ((ObjectNode) node).put("role", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
