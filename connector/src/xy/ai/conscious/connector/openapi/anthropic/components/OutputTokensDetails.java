package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class OutputTokensDetails {

    private final JsonNode node;

    public OutputTokensDetails(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public OutputTokensDetails() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * Number of output tokens the model generated as internal reasoning, including
     * the thinking-block delimiter tokens.
     * 
     * Reflects the raw reasoning the model produced, not the (possibly shorter)
     * summarized thinking text returned in the response body. Computed by
     * re-tokenizing the raw reasoning text, so it may differ from the model's exact
     * generation count by a small number of tokens. Always ≤ `output_tokens`;
     * `output_tokens - thinking_tokens` approximates the non-reasoning output.
     */

    public Long getThinkingTokens() {
        JsonNode child = node.get("thinking_tokens");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * Number of output tokens the model generated as internal reasoning, including
     * the thinking-block delimiter tokens.
     * 
     * Reflects the raw reasoning the model produced, not the (possibly shorter)
     * summarized thinking text returned in the response body. Computed by
     * re-tokenizing the raw reasoning text, so it may differ from the model's exact
     * generation count by a small number of tokens. Always ≤ `output_tokens`;
     * `output_tokens - thinking_tokens` approximates the non-reasoning output.
     */

    public void setThinkingTokens(Long value) {
        ((ObjectNode) node).put("thinking_tokens", value);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
