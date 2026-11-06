package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class JsonOutputFormat {

    private final JsonNode node;

    public JsonOutputFormat(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public JsonOutputFormat() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * The JSON schema of the format
     */

    public com.fasterxml.jackson.databind.JsonNode getSchema() {
        return node.get("schema");
    }

    /**
     * The JSON schema of the format
     */

    public void setSchema(com.fasterxml.jackson.databind.JsonNode value) {
        ((ObjectNode) node).set("schema", value);
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.FormatTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.FormatTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.FormatTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
