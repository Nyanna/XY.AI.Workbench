package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same oneOf node (no merge). */
public final class OneOfResult {

    private final JsonNode node;

    public OneOfResult(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the SucceededResult branch. */
    public boolean isSucceededResult() {
        return "succeeded".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.SucceededResult getSucceededResult() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.SucceededResult(node);
    }

    /** Whether the bound node currently matches the ErroredResult branch. */
    public boolean isErroredResult() {
        return "errored".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ErroredResult getErroredResult() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ErroredResult(node);
    }

    /** Whether the bound node currently matches the CanceledResult branch. */
    public boolean isCanceledResult() {
        return "canceled".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.CanceledResult getCanceledResult() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.CanceledResult(node);
    }

    /** Whether the bound node currently matches the ExpiredResult branch. */
    public boolean isExpiredResult() {
        return "expired".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.ExpiredResult getExpiredResult() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ExpiredResult(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
