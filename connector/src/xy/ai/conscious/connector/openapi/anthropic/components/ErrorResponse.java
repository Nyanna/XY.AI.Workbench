package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class ErrorResponse {

    private final JsonNode node;

    public ErrorResponse(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public ErrorResponse() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.OneOfError getError() {
        JsonNode child = node.get("error");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.OneOfError(child);
    }


    public void setError(xy.ai.conscious.connector.openapi.anthropic.components.OneOfError value) {
        ((ObjectNode) node).set("error", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getRequestId() {
        JsonNode child = node.get("request_id");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }


    public void setRequestId(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("request_id", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ResultErrorTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ResultErrorTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.ResultErrorTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
