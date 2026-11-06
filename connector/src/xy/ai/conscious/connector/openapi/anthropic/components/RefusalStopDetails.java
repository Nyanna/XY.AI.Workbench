package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class RefusalStopDetails {

    private final JsonNode node;

    public RefusalStopDetails(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public RefusalStopDetails() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * The policy category that triggered the refusal.
     * 
     * `null` when the refusal doesn't map to a named category.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfCategory getCategory() {
        JsonNode child = node.get("category");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfCategory(child);
    }

    /**
     * The policy category that triggered the refusal.
     * 
     * `null` when the refusal doesn't map to a named category.
     */

    public void setCategory(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfCategory value) {
        ((ObjectNode) node).set("category", value == null ? null : value.node());
    }

    /**
     * Human-readable explanation of the refusal.
     * 
     * This text is not guaranteed to be stable. `null` when no explanation is available for the category.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getExplanation() {
        JsonNode child = node.get("explanation");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * Human-readable explanation of the refusal.
     * 
     * This text is not guaranteed to be stable. `null` when no explanation is available for the category.
     */

    public void setExplanation(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("explanation", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.StopDetailsTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.StopDetailsTypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.StopDetailsTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
