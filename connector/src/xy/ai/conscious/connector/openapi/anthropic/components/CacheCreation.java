package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class CacheCreation {

    private final JsonNode node;

    public CacheCreation(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public CacheCreation() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * The number of input tokens used to create the 1 hour cache entry.
     */

    public Long getEphemeral1hInputTokens() {
        JsonNode child = node.get("ephemeral_1h_input_tokens");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * The number of input tokens used to create the 1 hour cache entry.
     */

    public void setEphemeral1hInputTokens(Long value) {
        ((ObjectNode) node).put("ephemeral_1h_input_tokens", value);
    }

    /**
     * The number of input tokens used to create the 5 minute cache entry.
     */

    public Long getEphemeral5mInputTokens() {
        JsonNode child = node.get("ephemeral_5m_input_tokens");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * The number of input tokens used to create the 5 minute cache entry.
     */

    public void setEphemeral5mInputTokens(Long value) {
        ((ObjectNode) node).put("ephemeral_5m_input_tokens", value);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
