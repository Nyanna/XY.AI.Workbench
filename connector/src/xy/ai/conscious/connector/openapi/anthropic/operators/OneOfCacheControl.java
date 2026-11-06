package xy.ai.conscious.connector.openapi.anthropic.operators;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same oneOf node (no merge). */
public final class OneOfCacheControl {

    private final JsonNode node;

    public OneOfCacheControl(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the CacheControlEphemeral branch. */
    public boolean isCacheControlEphemeral() {
        return "ephemeral".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.operators.CacheControlEphemeral getCacheControlEphemeral() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.CacheControlEphemeral(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
