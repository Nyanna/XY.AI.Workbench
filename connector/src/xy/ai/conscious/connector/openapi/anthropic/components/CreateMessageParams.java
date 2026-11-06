package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class CreateMessageParams {

    private final JsonNode node;

    public CreateMessageParams(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public CreateMessageParams() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfModel getModel() {
        JsonNode child = node.get("model");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfModel(child);
    }


    public void setModel(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfModel value) {
        ((ObjectNode) node).set("model", value == null ? null : value.node());
    }

    /**
     * Input messages.
     * 
     * Our models are trained to operate on alternating `user` and `assistant` conversational turns. When creating a new `Message`, you specify the prior conversational turns with the `messages` parameter, and the model then generates the next `Message` in the conversation. Consecutive `user` or `assistant` turns in your request will be combined into a single turn.
     * 
     * Each input message must be an object with a `role` and `content`. You can specify a single `user`-role message, or you can include multiple `user` and `assistant` messages.
     * 
     * If the final message uses the `assistant` role, the response content will continue immediately from the content in that message. This can be used to constrain part of the model's response.
     * 
     * Example with a single `user` message:
     * 
     * ```json
     * [{"role": "user", "content": "Hello, Claude"}]
     * ```
     * 
     * Example with multiple conversational turns:
     * 
     * ```json
     * [
     *   {"role": "user", "content": "Hello there."},
     *   {"role": "assistant", "content": "Hi, I'm Claude. How can I help you?"},
     *   {"role": "user", "content": "Can you explain LLMs in plain English?"},
     * ]
     * ```
     * 
     * Example with a partially-filled response from Claude:
     * 
     * ```json
     * [
     *   {"role": "user", "content": "What's the Greek name for Sun? (A) Sol (B) Helios (C) Sun"},
     *   {"role": "assistant", "content": "The best answer is ("},
     * ]
     * ```
     * 
     * Each input message `content` may be either a single `string` or an array of content blocks, where each block has a specific `type`. Using a `string` for `content` is shorthand for an array of one content block of type `"text"`. The following input messages are equivalent:
     * 
     * ```json
     * {"role": "user", "content": "Hello, Claude"}
     * ```
     * 
     * ```json
     * {"role": "user", "content": [{"type": "text", "text": "Hello, Claude"}]}
     * ```
     * 
     * See [input examples](https://platform.claude.com/docs/en/build-with-claude/working-with-messages).
     * 
     * Note that if you want to include a [system prompt](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices#give-claude-a-role), you can use the top-level `system` parameter — there is no `"system"` role for input messages in the Messages API.
     * 
     * There is a limit of 100,000 messages in a single request.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.MessagesList getMessages() {
        JsonNode child = node.get("messages");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.MessagesList(child);
    }

    /**
     * Input messages.
     * 
     * Our models are trained to operate on alternating `user` and `assistant` conversational turns. When creating a new `Message`, you specify the prior conversational turns with the `messages` parameter, and the model then generates the next `Message` in the conversation. Consecutive `user` or `assistant` turns in your request will be combined into a single turn.
     * 
     * Each input message must be an object with a `role` and `content`. You can specify a single `user`-role message, or you can include multiple `user` and `assistant` messages.
     * 
     * If the final message uses the `assistant` role, the response content will continue immediately from the content in that message. This can be used to constrain part of the model's response.
     * 
     * Example with a single `user` message:
     * 
     * ```json
     * [{"role": "user", "content": "Hello, Claude"}]
     * ```
     * 
     * Example with multiple conversational turns:
     * 
     * ```json
     * [
     *   {"role": "user", "content": "Hello there."},
     *   {"role": "assistant", "content": "Hi, I'm Claude. How can I help you?"},
     *   {"role": "user", "content": "Can you explain LLMs in plain English?"},
     * ]
     * ```
     * 
     * Example with a partially-filled response from Claude:
     * 
     * ```json
     * [
     *   {"role": "user", "content": "What's the Greek name for Sun? (A) Sol (B) Helios (C) Sun"},
     *   {"role": "assistant", "content": "The best answer is ("},
     * ]
     * ```
     * 
     * Each input message `content` may be either a single `string` or an array of content blocks, where each block has a specific `type`. Using a `string` for `content` is shorthand for an array of one content block of type `"text"`. The following input messages are equivalent:
     * 
     * ```json
     * {"role": "user", "content": "Hello, Claude"}
     * ```
     * 
     * ```json
     * {"role": "user", "content": [{"type": "text", "text": "Hello, Claude"}]}
     * ```
     * 
     * See [input examples](https://platform.claude.com/docs/en/build-with-claude/working-with-messages).
     * 
     * Note that if you want to include a [system prompt](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices#give-claude-a-role), you can use the top-level `system` parameter — there is no `"system"` role for input messages in the Messages API.
     * 
     * There is a limit of 100,000 messages in a single request.
     */

    public void setMessages(xy.ai.conscious.connector.openapi.anthropic.components.MessagesList value) {
        ((ObjectNode) node).set("messages", value == null ? null : value.node());
    }

    /**
     * Top-level cache control automatically applies a cache_control marker to the last cacheable block in the request.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheControl getCacheControl() {
        JsonNode child = node.get("cache_control");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheControl(child);
    }

    /**
     * Top-level cache control automatically applies a cache_control marker to the last cacheable block in the request.
     */

    public void setCacheControl(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfCacheControl value) {
        ((ObjectNode) node).set("cache_control", value == null ? null : value.node());
    }

    /**
     * Container identifier for reuse across requests.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getContainer() {
        JsonNode child = node.get("container");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * Container identifier for reuse across requests.
     */

    public void setContainer(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("container", value == null ? null : value.node());
    }

    /**
     * Specifies the geographic region for inference processing. If not specified, the workspace's `default_inference_geo` is used.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getInferenceGeo() {
        JsonNode child = node.get("inference_geo");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * Specifies the geographic region for inference processing. If not specified, the workspace's `default_inference_geo` is used.
     */

    public void setInferenceGeo(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("inference_geo", value == null ? null : value.node());
    }

    /**
     * The maximum number of tokens to generate before stopping.
     * 
     * Note that our models may stop _before_ reaching this maximum. This parameter only specifies the absolute maximum number of tokens to generate.
     * 
     * Set to `0` to populate the [prompt cache](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pre-warming-the-cache) without generating a response.
     * 
     * Different models have different maximum values for this parameter.  See [models](https://platform.claude.com/docs/en/about-claude/models/overview) for details.
     */

    public Long getMaxTokens() {
        JsonNode child = node.get("max_tokens");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * The maximum number of tokens to generate before stopping.
     * 
     * Note that our models may stop _before_ reaching this maximum. This parameter only specifies the absolute maximum number of tokens to generate.
     * 
     * Set to `0` to populate the [prompt cache](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pre-warming-the-cache) without generating a response.
     * 
     * Different models have different maximum values for this parameter.  See [models](https://platform.claude.com/docs/en/about-claude/models/overview) for details.
     */

    public void setMaxTokens(Long value) {
        ((ObjectNode) node).put("max_tokens", value);
    }

    /**
     * An object describing metadata about the request.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.Metadata getMetadata() {
        JsonNode child = node.get("metadata");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.Metadata(child);
    }

    /**
     * An object describing metadata about the request.
     */

    public void setMetadata(xy.ai.conscious.connector.openapi.anthropic.components.Metadata value) {
        ((ObjectNode) node).set("metadata", value == null ? null : value.node());
    }

    /**
     * Configuration options for the model's output, such as the output format.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.OutputConfig getOutputConfig() {
        JsonNode child = node.get("output_config");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.OutputConfig(child);
    }

    /**
     * Configuration options for the model's output, such as the output format.
     */

    public void setOutputConfig(xy.ai.conscious.connector.openapi.anthropic.components.OutputConfig value) {
        ((ObjectNode) node).set("output_config", value == null ? null : value.node());
    }

    /**
     * Determines whether to use priority capacity (if available) or standard capacity for this request.
     * 
     * Anthropic offers different levels of service for your API requests. See [service-tiers](https://platform.claude.com/docs/en/api/service-tiers) for details.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ServiceTierEnum getServiceTier() {
        JsonNode child = node.get("service_tier");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ServiceTierEnum.fromValue(child.asText());
    }

    /**
     * Determines whether to use priority capacity (if available) or standard capacity for this request.
     * 
     * Anthropic offers different levels of service for your API requests. See [service-tiers](https://platform.claude.com/docs/en/api/service-tiers) for details.
     */

    public void setServiceTier(xy.ai.conscious.connector.openapi.anthropic.components.ServiceTierEnum value) {
        ((ObjectNode) node).put("service_tier", value == null ? null : value.rawValue());
    }

    /**
     * Custom text sequences that will cause the model to stop generating.
     * 
     * Our models will normally stop when they have naturally completed their turn, which will result in a response `stop_reason` of `"end_turn"`.
     * 
     * If you want the model to stop generating when it encounters custom strings of text, you can use the `stop_sequences` parameter. If the model encounters one of the custom sequences, the response `stop_reason` value will be `"stop_sequence"` and the response `stop_sequence` value will contain the matched stop sequence.
     */

    public xy.ai.conscious.connector.openapi.anthropic.lists.RequiredList getStopSequences() {
        JsonNode child = node.get("stop_sequences");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.lists.RequiredList(child);
    }

    /**
     * Custom text sequences that will cause the model to stop generating.
     * 
     * Our models will normally stop when they have naturally completed their turn, which will result in a response `stop_reason` of `"end_turn"`.
     * 
     * If you want the model to stop generating when it encounters custom strings of text, you can use the `stop_sequences` parameter. If the model encounters one of the custom sequences, the response `stop_reason` value will be `"stop_sequence"` and the response `stop_sequence` value will contain the matched stop sequence.
     */

    public void setStopSequences(xy.ai.conscious.connector.openapi.anthropic.lists.RequiredList value) {
        ((ObjectNode) node).set("stop_sequences", value == null ? null : value.node());
    }

    /**
     * Whether to incrementally stream the response using server-sent events.
     * 
     * See [streaming](https://platform.claude.com/docs/en/build-with-claude/streaming) for details.
     * Example: False
     */

    public Boolean getStream() {
        JsonNode child = node.get("stream");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asBoolean();
    }

    /**
     * Whether to incrementally stream the response using server-sent events.
     * 
     * See [streaming](https://platform.claude.com/docs/en/build-with-claude/streaming) for details.
     * Example: False
     */

    public void setStream(Boolean value) {
        ((ObjectNode) node).put("stream", value);
    }

    /**
     * System prompt.
     * 
     * A system prompt is a way of providing context and instructions to Claude, such as specifying a particular goal or role. See our [guide to system prompts](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices#give-claude-a-role).
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.AnyOfSystem getSystem() {
        JsonNode child = node.get("system");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.AnyOfSystem(child);
    }

    /**
     * System prompt.
     * 
     * A system prompt is a way of providing context and instructions to Claude, such as specifying a particular goal or role. See our [guide to system prompts](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices#give-claude-a-role).
     */

    public void setSystem(xy.ai.conscious.connector.openapi.anthropic.components.AnyOfSystem value) {
        ((ObjectNode) node).set("system", value == null ? null : value.node());
    }

    /**
     * Amount of randomness injected into the response.
     * 
     * Defaults to `1.0`. Ranges from `0.0` to `1.0`. Use `temperature` closer to `0.0` for analytical / multiple choice, and closer to `1.0` for creative and generative tasks.
     * 
     * Note that even with `temperature` of `0.0`, the results will not be fully deterministic.
     */

    public Double getTemperature() {
        JsonNode child = node.get("temperature");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asDouble();
    }

    /**
     * Amount of randomness injected into the response.
     * 
     * Defaults to `1.0`. Ranges from `0.0` to `1.0`. Use `temperature` closer to `0.0` for analytical / multiple choice, and closer to `1.0` for creative and generative tasks.
     * 
     * Note that even with `temperature` of `0.0`, the results will not be fully deterministic.
     */

    public void setTemperature(Double value) {
        ((ObjectNode) node).put("temperature", value);
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.OneOfThinking getThinking() {
        JsonNode child = node.get("thinking");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.OneOfThinking(child);
    }


    public void setThinking(xy.ai.conscious.connector.openapi.anthropic.components.OneOfThinking value) {
        ((ObjectNode) node).set("thinking", value == null ? null : value.node());
    }


    public xy.ai.conscious.connector.openapi.anthropic.components.OneOfToolChoice getToolChoice() {
        JsonNode child = node.get("tool_choice");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.OneOfToolChoice(child);
    }


    public void setToolChoice(xy.ai.conscious.connector.openapi.anthropic.components.OneOfToolChoice value) {
        ((ObjectNode) node).set("tool_choice", value == null ? null : value.node());
    }

    /**
     * Definitions of tools that the model may use.
     * 
     * If you include `tools` in your API request, the model may return `tool_use` content blocks that represent the model's use of those tools. You can then run those tools using the tool input generated by the model and then optionally return results back to the model using `tool_result` content blocks.
     * 
     * There are two types of tools: **client tools** and **server tools**. The behavior described below applies to client tools. For [server tools](https://platform.claude.com/docs/en/agents-and-tools/tool-use/server-tools), see their individual documentation as each has its own behavior (e.g., the [web search tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool)).
     * 
     * Each tool definition includes:
     * 
     * * `name`: Name of the tool.
     * * `description`: Optional, but strongly-recommended description of the tool.
     * * `input_schema`: [JSON schema](https://json-schema.org/draft/2020-12) for the tool `input` shape that the model will produce in `tool_use` output content blocks.
     * 
     * For example, if you defined `tools` as:
     * 
     * ```json
     * [
     *   {
     *     "name": "get_stock_price",
     *     "description": "Get the current stock price for a given ticker symbol.",
     *     "input_schema": {
     *       "type": "object",
     *       "properties": {
     *         "ticker": {
     *           "type": "string",
     *           "description": "The stock ticker symbol, e.g. AAPL for Apple Inc."
     *         }
     *       },
     *       "required": ["ticker"]
     *     }
     *   }
     * ]
     * ```
     * 
     * And then asked the model "What's the S&P 500 at today?", the model might produce `tool_use` content blocks in the response like this:
     * 
     * ```json
     * [
     *   {
     *     "type": "tool_use",
     *     "id": "toolu_01D7FLrfh4GYq7yT1ULFeyMV",
     *     "name": "get_stock_price",
     *     "input": { "ticker": "^GSPC" }
     *   }
     * ]
     * ```
     * 
     * You might then run your `get_stock_price` tool with `{"ticker": "^GSPC"}` as an input, and return the following back to the model in a subsequent `user` message:
     * 
     * ```json
     * [
     *   {
     *     "type": "tool_result",
     *     "tool_use_id": "toolu_01D7FLrfh4GYq7yT1ULFeyMV",
     *     "content": "259.75 USD"
     *   }
     * ]
     * ```
     * 
     * Tools can be used for workflows that include running client-side tools and functions, or more generally whenever you want the model to produce a particular JSON structure of output.
     * 
     * See our [guide](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview) for more details.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ToolsList getTools() {
        JsonNode child = node.get("tools");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.ToolsList(child);
    }

    /**
     * Definitions of tools that the model may use.
     * 
     * If you include `tools` in your API request, the model may return `tool_use` content blocks that represent the model's use of those tools. You can then run those tools using the tool input generated by the model and then optionally return results back to the model using `tool_result` content blocks.
     * 
     * There are two types of tools: **client tools** and **server tools**. The behavior described below applies to client tools. For [server tools](https://platform.claude.com/docs/en/agents-and-tools/tool-use/server-tools), see their individual documentation as each has its own behavior (e.g., the [web search tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool)).
     * 
     * Each tool definition includes:
     * 
     * * `name`: Name of the tool.
     * * `description`: Optional, but strongly-recommended description of the tool.
     * * `input_schema`: [JSON schema](https://json-schema.org/draft/2020-12) for the tool `input` shape that the model will produce in `tool_use` output content blocks.
     * 
     * For example, if you defined `tools` as:
     * 
     * ```json
     * [
     *   {
     *     "name": "get_stock_price",
     *     "description": "Get the current stock price for a given ticker symbol.",
     *     "input_schema": {
     *       "type": "object",
     *       "properties": {
     *         "ticker": {
     *           "type": "string",
     *           "description": "The stock ticker symbol, e.g. AAPL for Apple Inc."
     *         }
     *       },
     *       "required": ["ticker"]
     *     }
     *   }
     * ]
     * ```
     * 
     * And then asked the model "What's the S&P 500 at today?", the model might produce `tool_use` content blocks in the response like this:
     * 
     * ```json
     * [
     *   {
     *     "type": "tool_use",
     *     "id": "toolu_01D7FLrfh4GYq7yT1ULFeyMV",
     *     "name": "get_stock_price",
     *     "input": { "ticker": "^GSPC" }
     *   }
     * ]
     * ```
     * 
     * You might then run your `get_stock_price` tool with `{"ticker": "^GSPC"}` as an input, and return the following back to the model in a subsequent `user` message:
     * 
     * ```json
     * [
     *   {
     *     "type": "tool_result",
     *     "tool_use_id": "toolu_01D7FLrfh4GYq7yT1ULFeyMV",
     *     "content": "259.75 USD"
     *   }
     * ]
     * ```
     * 
     * Tools can be used for workflows that include running client-side tools and functions, or more generally whenever you want the model to produce a particular JSON structure of output.
     * 
     * See our [guide](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview) for more details.
     */

    public void setTools(xy.ai.conscious.connector.openapi.anthropic.components.ToolsList value) {
        ((ObjectNode) node).set("tools", value == null ? null : value.node());
    }

    /**
     * Only sample from the top K options for each subsequent token.
     * 
     * Used to remove "long tail" low probability responses. [Learn more technical details here](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277).
     * 
     * Recommended for advanced use cases only.
     */

    public Long getTopK() {
        JsonNode child = node.get("top_k");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * Only sample from the top K options for each subsequent token.
     * 
     * Used to remove "long tail" low probability responses. [Learn more technical details here](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277).
     * 
     * Recommended for advanced use cases only.
     */

    public void setTopK(Long value) {
        ((ObjectNode) node).put("top_k", value);
    }

    /**
     * Use nucleus sampling.
     * 
     * In nucleus sampling, we compute the cumulative distribution over all the options for each subsequent token in decreasing probability order and cut it off once it reaches a particular probability specified by `top_p`.
     * 
     * Recommended for advanced use cases only.
     */

    public Double getTopP() {
        JsonNode child = node.get("top_p");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asDouble();
    }

    /**
     * Use nucleus sampling.
     * 
     * In nucleus sampling, we compute the cumulative distribution over all the options for each subsequent token in decreasing probability order and cut it off once it reaches a particular probability specified by `top_p`.
     * 
     * Recommended for advanced use cases only.
     */

    public void setTopP(Double value) {
        ((ObjectNode) node).put("top_p", value);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }

    public static CreateMessageParams fromString(String body) {
        return new CreateMessageParams(xy.ai.conscious.connector.openapi.anthropic.JsonSupport.parse(body));
    }
}
