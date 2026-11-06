package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;

/** Proxy over one JsonNode; every branch below is a view of the same anyOf node (no merge). */
public final class AnyOfModel {

    private final JsonNode node;

    public AnyOfModel(JsonNode node) {
        this.node = node;
    }

    public JsonNode node() {
        return node;
    }

    /** Whether the bound node currently matches the String branch. */
    public boolean isString() {
        return node.isTextual();
    }


    public String getString() {
        if (node.isNull()) {
            return null;
        }
        return node.asText();
    }

    /** Whether the bound node currently matches the ClaudeSonnet5Enum branch. */
    public boolean isClaudeSonnet5Enum() {
        return node.isTextual();
    }

    /**
     * High-performance model for coding and agents
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeSonnet5Enum getClaudeSonnet5Enum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeSonnet5Enum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ClaudeFable5Enum branch. */
    public boolean isClaudeFable5Enum() {
        return node.isTextual();
    }

    /**
     * Next generation of intelligence for the hardest knowledge work and coding problems
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeFable5Enum getClaudeFable5Enum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeFable5Enum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ClaudeMythos5Enum branch. */
    public boolean isClaudeMythos5Enum() {
        return node.isTextual();
    }

    /**
     * Most capable model for cybersecurity and biology research
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeMythos5Enum getClaudeMythos5Enum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeMythos5Enum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ClaudeOpus5Enum branch. */
    public boolean isClaudeOpus5Enum() {
        return node.isTextual();
    }

    /**
     * Powerful intelligence for long-running agents and coding
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus5Enum getClaudeOpus5Enum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus5Enum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ClaudeOpus48Enum branch. */
    public boolean isClaudeOpus48Enum() {
        return node.isTextual();
    }

    /**
     * Powerful intelligence for long-running agents and coding
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus48Enum getClaudeOpus48Enum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus48Enum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ElementParamsModelEnum branch. */
    public boolean isElementParamsModelEnum() {
        return node.isTextual();
    }

    /**
     * Powerful intelligence for long-running agents and coding
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ElementParamsModelEnum getElementParamsModelEnum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ElementParamsModelEnum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ClaudeMythosPreviewEnum branch. */
    public boolean isClaudeMythosPreviewEnum() {
        return node.isTextual();
    }

    /**
     * New class of intelligence, strongest in coding and cybersecurity
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeMythosPreviewEnum getClaudeMythosPreviewEnum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeMythosPreviewEnum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ClaudeOpus46Enum branch. */
    public boolean isClaudeOpus46Enum() {
        return node.isTextual();
    }

    /**
     * Powerful intelligence for long-running agents and coding
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus46Enum getClaudeOpus46Enum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus46Enum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ModelEnum branch. */
    public boolean isModelEnum() {
        return node.isTextual();
    }

    /**
     * Best combination of speed and intelligence
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ModelEnum getModelEnum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ModelEnum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ClaudeHaiku45Enum branch. */
    public boolean isClaudeHaiku45Enum() {
        return node.isTextual();
    }

    /**
     * Fastest model with near-frontier intelligence
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeHaiku45Enum getClaudeHaiku45Enum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeHaiku45Enum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the BodyRequestsElementParamsModelEnum branch. */
    public boolean isBodyRequestsElementParamsModelEnum() {
        return node.isTextual();
    }

    /**
     * Fastest model with near-frontier intelligence
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.BodyRequestsElementParamsModelEnum getBodyRequestsElementParamsModelEnum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.BodyRequestsElementParamsModelEnum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ClaudeOpus45Enum branch. */
    public boolean isClaudeOpus45Enum() {
        return node.isTextual();
    }

    /**
     * Powerful intelligence for long-running agents and coding
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus45Enum getClaudeOpus45Enum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus45Enum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ClaudeOpus4520251101Enum branch. */
    public boolean isClaudeOpus4520251101Enum() {
        return node.isTextual();
    }

    /**
     * Powerful intelligence for long-running agents and coding
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus4520251101Enum getClaudeOpus4520251101Enum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ClaudeOpus4520251101Enum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the ParamsModelEnum branch. */
    public boolean isParamsModelEnum() {
        return node.isTextual();
    }

    /**
     * High-performance model for agents and coding
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ParamsModelEnum getParamsModelEnum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ParamsModelEnum.fromValue(node.asText());
    }

    /** Whether the bound node currently matches the RequestsElementParamsModelEnum branch. */
    public boolean isRequestsElementParamsModelEnum() {
        return node.isTextual();
    }

    /**
     * High-performance model for agents and coding
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.RequestsElementParamsModelEnum getRequestsElementParamsModelEnum() {
        if (node.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.RequestsElementParamsModelEnum.fromValue(node.asText());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
