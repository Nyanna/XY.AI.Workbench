package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class InputSchema {

    private final JsonNode node;

    public InputSchema(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public InputSchema() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfProperties getProperties() {
        JsonNode child = node.get("properties");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfProperties(child);
    }


    public void setProperties(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfProperties value) {
        ((ObjectNode) node).set("properties", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfRequired getRequired() {
        JsonNode child = node.get("required");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfRequired(child);
    }


    public void setRequired(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfRequired value) {
        ((ObjectNode) node).set("required", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.InputSchemaTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.InputSchemaTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.InputSchemaTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
