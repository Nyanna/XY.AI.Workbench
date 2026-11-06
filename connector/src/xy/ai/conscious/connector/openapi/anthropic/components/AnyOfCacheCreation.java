package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfCacheCreation {

    private final JsonNode node;

    public AnyOfCacheCreation(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the CacheCreation branch. */
    public boolean isCacheCreation() {
        return node.has("ephemeral_1h_input_tokens") && node.has("ephemeral_5m_input_tokens");
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.CacheCreation getCacheCreation() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.CacheCreation(node);
    }

    /** Whether the bound node currently matches the Null branch. */
    public boolean isNull() {
        return node.isNull();
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
