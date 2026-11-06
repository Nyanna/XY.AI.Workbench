package xy.ai.conscious.connector.openapi.anthropic.operators;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class CacheControlEphemeral {

    private final JsonNode node;

    public CacheControlEphemeral(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public CacheControlEphemeral() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * The time-to-live for the cache control breakpoint.
     * 
     * This may be one the following values:
     * - `5m`: 5 minutes
     * - `1h`: 1 hour
     * 
     * Defaults to `5m`. See [prompt caching pricing](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) for details.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.TtlEnum getTtl() {
        JsonNode child = node.get("ttl");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.operators.TtlEnum.fromValue(child.asText());
    }

    /**
     * The time-to-live for the cache control breakpoint.
     * 
     * This may be one the following values:
     * - `5m`: 5 minutes
     * - `1h`: 1 hour
     * 
     * Defaults to `5m`. See [prompt caching pricing](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) for details.
     */

    public void setTtl(xy.ai.conscious.connector.openapi.anthropic.operators.TtlEnum value) {
        ((ObjectNode) node).put("ttl", value == null ? null : value.rawValue());
    }


    public xy.ai.conscious.connector.openapi.anthropic.operators.TypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.operators.TypeEnum.fromValue(child.asText());
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.operators.TypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
