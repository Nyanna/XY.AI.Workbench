package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class Usage {

    private final JsonNode node;

    public Usage(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public Usage() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * Breakdown of cached tokens by TTL
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfCacheCreation getCacheCreation() {
        JsonNode child = node.get("cache_creation");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfCacheCreation(child);
    }

    /**
     * Breakdown of cached tokens by TTL
     */

    public void setCacheCreation(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfCacheCreation value) {
        ((ObjectNode) node).set("cache_creation", value == null ? null : value.node());
    }

    /**
     * The number of input tokens used to create the cache entry.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheCreationInputTokens getCacheCreationInputTokens() {
        JsonNode child = node.get("cache_creation_input_tokens");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheCreationInputTokens(child);
    }

    /**
     * The number of input tokens used to create the cache entry.
     */

    public void setCacheCreationInputTokens(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheCreationInputTokens value) {
        ((ObjectNode) node).set("cache_creation_input_tokens", value == null ? null : value.node());
    }

    /**
     * The number of input tokens read from the cache.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheCreationInputTokens getCacheReadInputTokens() {
        JsonNode child = node.get("cache_read_input_tokens");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheCreationInputTokens(child);
    }

    /**
     * The number of input tokens read from the cache.
     */

    public void setCacheReadInputTokens(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheCreationInputTokens value) {
        ((ObjectNode) node).set("cache_read_input_tokens", value == null ? null : value.node());
    }

    /**
     * The geographic region where inference was performed for this request.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getInferenceGeo() {
        JsonNode child = node.get("inference_geo");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * The geographic region where inference was performed for this request.
     */

    public void setInferenceGeo(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("inference_geo", value == null ? null : value.node());
    }

    /**
     * The number of input tokens which were used.
     */

    public Long getInputTokens() {
        JsonNode child = node.get("input_tokens");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * The number of input tokens which were used.
     */

    public void setInputTokens(Long value) {
        ((ObjectNode) node).put("input_tokens", value);
    }

    /**
     * The number of output tokens which were used.
     */

    public Long getOutputTokens() {
        JsonNode child = node.get("output_tokens");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * The number of output tokens which were used.
     */

    public void setOutputTokens(Long value) {
        ((ObjectNode) node).put("output_tokens", value);
    }

    /**
     * Breakdown of output tokens by category.
     * 
     * `output_tokens` remains the inclusive, authoritative total used for billing.
     * This object provides a read-only decomposition for observability — for example,
     * how many of the billed output tokens were spent on internal reasoning that may
     * have been summarized before being returned to you.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfOutputTokensDetails getOutputTokensDetails() {
        JsonNode child = node.get("output_tokens_details");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfOutputTokensDetails(child);
    }

    /**
     * Breakdown of output tokens by category.
     * 
     * `output_tokens` remains the inclusive, authoritative total used for billing.
     * This object provides a read-only decomposition for observability — for example,
     * how many of the billed output tokens were spent on internal reasoning that may
     * have been summarized before being returned to you.
     */

    public void setOutputTokensDetails(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfOutputTokensDetails value) {
        ((ObjectNode) node).set("output_tokens_details", value == null ? null : value.node());
    }

    /**
     * If the request used the priority, standard, or batch tier.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfServiceTier getServiceTier() {
        JsonNode child = node.get("service_tier");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfServiceTier(child);
    }

    /**
     * If the request used the priority, standard, or batch tier.
     */

    public void setServiceTier(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfServiceTier value) {
        ((ObjectNode) node).set("service_tier", value == null ? null : value.node());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
