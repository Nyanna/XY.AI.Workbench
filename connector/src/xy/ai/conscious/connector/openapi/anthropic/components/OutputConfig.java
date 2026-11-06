package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class OutputConfig {

    private final JsonNode node;

    public OutputConfig(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public OutputConfig() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * How much effort the model should put into its response. Higher effort levels may result in more thorough analysis but take longer.
     * 
     * Valid values are `low`, `medium`, `high`, `xhigh`, or `max`.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfEffort getEffort() {
        JsonNode child = node.get("effort");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfEffort(child);
    }

    /**
     * How much effort the model should put into its response. Higher effort levels may result in more thorough analysis but take longer.
     * 
     * Valid values are `low`, `medium`, `high`, `xhigh`, or `max`.
     */

    public void setEffort(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfEffort value) {
        ((ObjectNode) node).set("effort", value == null ? null : value.node());
    }

    /**
     * A schema to specify Claude's output format in responses. See [structured outputs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfFormat getFormat() {
        JsonNode child = node.get("format");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfFormat(child);
    }

    /**
     * A schema to specify Claude's output format in responses. See [structured outputs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)
     */

    public void setFormat(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfFormat value) {
        ((ObjectNode) node).set("format", value == null ? null : value.node());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
