package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same oneOf node (no merge). */
public final class OneOfError {

    private final JsonNode node;

    public OneOfError(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the InvalidRequestError branch. */
    public boolean isInvalidRequestError() {
        return "invalid_request_error".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.InvalidRequestError getInvalidRequestError() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.InvalidRequestError(node);
    }

    /** Whether the bound node currently matches the AuthenticationError branch. */
    public boolean isAuthenticationError() {
        return "authentication_error".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.AuthenticationError getAuthenticationError() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AuthenticationError(node);
    }

    /** Whether the bound node currently matches the BillingError branch. */
    public boolean isBillingError() {
        return "billing_error".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.BillingError getBillingError() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.BillingError(node);
    }

    /** Whether the bound node currently matches the PermissionError branch. */
    public boolean isPermissionError() {
        return "permission_error".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.PermissionError getPermissionError() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.PermissionError(node);
    }

    /** Whether the bound node currently matches the NotFoundError branch. */
    public boolean isNotFoundError() {
        return "not_found_error".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.NotFoundError getNotFoundError() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.NotFoundError(node);
    }

    /** Whether the bound node currently matches the RateLimitError branch. */
    public boolean isRateLimitError() {
        return "rate_limit_error".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.RateLimitError getRateLimitError() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.RateLimitError(node);
    }

    /** Whether the bound node currently matches the GatewayTimeoutError branch. */
    public boolean isGatewayTimeoutError() {
        return "timeout_error".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.GatewayTimeoutError getGatewayTimeoutError() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.GatewayTimeoutError(node);
    }

    /** Whether the bound node currently matches the APIError branch. */
    public boolean isAPIError() {
        return "api_error".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.APIError getAPIError() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.APIError(node);
    }

    /** Whether the bound node currently matches the OverloadedError branch. */
    public boolean isOverloadedError() {
        return "overloaded_error".equals(node.path("type").asText());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.OverloadedError getOverloadedError() {
        if (node.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.OverloadedError(node);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
