package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class Tool {

    private final JsonNode node;

    public Tool(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public Tool() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfType getType() {
        JsonNode child = node.get("type");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfType(child);
    }


    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfType value) {
        ((ObjectNode) node).set("type", value == null ? null : value.node());
    }

    /**
     * Description of what this tool does.
     * 
     * Tool descriptions should be as detailed as possible. The more information that the model has about what the tool is and how to use it, the better it will perform. You can use natural language descriptions to reinforce important aspects of the tool input JSON schema.
     */

    public String getDescription() {
        JsonNode child = node.get("description");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }

    /**
     * Description of what this tool does.
     * 
     * Tool descriptions should be as detailed as possible. The more information that the model has about what the tool is and how to use it, the better it will perform. You can use natural language descriptions to reinforce important aspects of the tool input JSON schema.
     */

    public void setDescription(String value) {
        ((ObjectNode) node).put("description", value);
    }

    /**
     * Name of the tool.
     * 
     * This is how the tool will be called by the model and in `tool_use` blocks.
     */

    public String getName() {
        JsonNode child = node.get("name");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }

    /**
     * Name of the tool.
     * 
     * This is how the tool will be called by the model and in `tool_use` blocks.
     */

    public void setName(String value) {
        ((ObjectNode) node).put("name", value);
    }

    /**
     * [JSON schema](https://json-schema.org/draft/2020-12) for this tool's input.
     * 
     * This defines the shape of the `input` that your tool accepts and that the model will produce.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.InputSchema getInputSchema() {
        JsonNode child = node.get("input_schema");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.InputSchema(child);
    }

    /**
     * [JSON schema](https://json-schema.org/draft/2020-12) for this tool's input.
     * 
     * This defines the shape of the `input` that your tool accepts and that the model will produce.
     */

    public void setInputSchema(xy.ai.conscious.connector.openapi.anthropic.components.InputSchema value) {
        ((ObjectNode) node).set("input_schema", value == null ? null : value.node());
    }

    /**
     * Create a cache control breakpoint at this content block.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheControl getCacheControl() {
        JsonNode child = node.get("cache_control");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheControl(child);
    }

    /**
     * Create a cache control breakpoint at this content block.
     */

    public void setCacheControl(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheControl value) {
        ((ObjectNode) node).set("cache_control", value == null ? null : value.node());
    }

    /**
     * When true, guarantees schema validation on tool names and inputs
     */

    public Boolean getStrict() {
        JsonNode child = node.get("strict");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asBoolean();
    }

    /**
     * When true, guarantees schema validation on tool names and inputs
     */

    public void setStrict(Boolean value) {
        ((ObjectNode) node).put("strict", value);
    }

    /**
     * Enable eager input streaming for this tool. When true, tool input parameters will be streamed incrementally as they are generated, and types will be inferred on-the-fly rather than buffering the full JSON output. When false, streaming is disabled for this tool even if the fine-grained-tool-streaming beta is active. When null (default), uses the default behavior based on beta headers.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfEagerInputStreaming getEagerInputStreaming() {
        JsonNode child = node.get("eager_input_streaming");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfEagerInputStreaming(child);
    }

    /**
     * Enable eager input streaming for this tool. When true, tool input parameters will be streamed incrementally as they are generated, and types will be inferred on-the-fly rather than buffering the full JSON output. When false, streaming is disabled for this tool even if the fine-grained-tool-streaming beta is active. When null (default), uses the default behavior based on beta headers.
     */

    public void setEagerInputStreaming(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfEagerInputStreaming value) {
        ((ObjectNode) node).set("eager_input_streaming", value == null ? null : value.node());
    }

    /**
     * If true, tool will not be included in initial system prompt. Only loaded when returned via tool_reference from tool search.
     */

    public Boolean getDeferLoading() {
        JsonNode child = node.get("defer_loading");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asBoolean();
    }

    /**
     * If true, tool will not be included in initial system prompt. Only loaded when returned via tool_reference from tool search.
     */

    public void setDeferLoading(Boolean value) {
        ((ObjectNode) node).put("defer_loading", value);
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.InputExamplesList getInputExamples() {
        JsonNode child = node.get("input_examples");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.InputExamplesList(child);
    }


    public void setInputExamples(xy.ai.conscious.connector.openapi.anthropic.components.InputExamplesList value) {
        ((ObjectNode) node).set("input_examples", value == null ? null : value.node());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
