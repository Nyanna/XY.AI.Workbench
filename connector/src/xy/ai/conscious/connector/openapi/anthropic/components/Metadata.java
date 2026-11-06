package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class Metadata {

    private final JsonNode node;

    public Metadata(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public Metadata() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * An external identifier for the user who is associated with the request.
     * 
     * This should be a uuid, hash value, or other opaque identifier. Anthropic may use this id to help detect abuse. Do not include any identifying information such as name, email address, or phone number.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getUserId() {
        JsonNode child = node.get("user_id");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * An external identifier for the user who is associated with the request.
     * 
     * This should be a uuid, hash value, or other opaque identifier. Anthropic may use this id to help detect abuse. Do not include any identifying information such as name, email address, or phone number.
     */

    public void setUserId(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("user_id", value == null ? null : value.node());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
