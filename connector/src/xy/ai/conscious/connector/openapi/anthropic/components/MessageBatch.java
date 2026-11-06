package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class MessageBatch {

    private final JsonNode node;

    public MessageBatch(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public MessageBatch() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * RFC 3339 datetime string representing the time at which the Message Batch was archived and its results became unavailable.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getArchivedAt() {
        JsonNode child = node.get("archived_at");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * RFC 3339 datetime string representing the time at which the Message Batch was archived and its results became unavailable.
     */

    public void setArchivedAt(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("archived_at", value == null ? null : value.node());
    }

    /**
     * RFC 3339 datetime string representing the time at which cancellation was initiated for the Message Batch. Specified only if cancellation was initiated.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getCancelInitiatedAt() {
        JsonNode child = node.get("cancel_initiated_at");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * RFC 3339 datetime string representing the time at which cancellation was initiated for the Message Batch. Specified only if cancellation was initiated.
     */

    public void setCancelInitiatedAt(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("cancel_initiated_at", value == null ? null : value.node());
    }

    /**
     * RFC 3339 datetime string representing the time at which the Message Batch was created.
     */

    public String getCreatedAt() {
        JsonNode child = node.get("created_at");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }

    /**
     * RFC 3339 datetime string representing the time at which the Message Batch was created.
     */

    public void setCreatedAt(String value) {
        ((ObjectNode) node).put("created_at", value);
    }

    /**
     * RFC 3339 datetime string representing the time at which processing for the Message Batch ended. Specified only once processing ends.
     * 
     * Processing ends when every request in a Message Batch has either succeeded, errored, canceled, or expired.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getEndedAt() {
        JsonNode child = node.get("ended_at");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * RFC 3339 datetime string representing the time at which processing for the Message Batch ended. Specified only once processing ends.
     * 
     * Processing ends when every request in a Message Batch has either succeeded, errored, canceled, or expired.
     */

    public void setEndedAt(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("ended_at", value == null ? null : value.node());
    }

    /**
     * RFC 3339 datetime string representing the time at which the Message Batch will expire and end processing, which is 24 hours after creation.
     */

    public String getExpiresAt() {
        JsonNode child = node.get("expires_at");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }

    /**
     * RFC 3339 datetime string representing the time at which the Message Batch will expire and end processing, which is 24 hours after creation.
     */

    public void setExpiresAt(String value) {
        ((ObjectNode) node).put("expires_at", value);
    }

    /**
     * Unique object identifier.
     * 
     * The format and length of IDs may change over time.
     */

    public String getId() {
        JsonNode child = node.get("id");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asText();
    }

    /**
     * Unique object identifier.
     * 
     * The format and length of IDs may change over time.
     */

    public void setId(String value) {
        ((ObjectNode) node).put("id", value);
    }

    /**
     * Processing status of the Message Batch.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.ProcessingStatusEnum getProcessingStatus() {
        JsonNode child = node.get("processing_status");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.ProcessingStatusEnum.fromValue(child.asText());
    }

    /**
     * Processing status of the Message Batch.
     */

    public void setProcessingStatus(xy.ai.conscious.connector.openapi.anthropic.components.ProcessingStatusEnum value) {
        ((ObjectNode) node).put("processing_status", value == null ? null : value.rawValue());
    }

    /**
     * Tallies requests within the Message Batch, categorized by their status.
     * 
     * Requests start as `processing` and move to one of the other statuses only once processing of the entire batch ends. The sum of all values always matches the total number of requests in the batch.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.RequestCounts getRequestCounts() {
        JsonNode child = node.get("request_counts");
        if (child == null || child.isNull()) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.components.RequestCounts(child);
    }

    /**
     * Tallies requests within the Message Batch, categorized by their status.
     * 
     * Requests start as `processing` and move to one of the other statuses only once processing of the entire batch ends. The sum of all values always matches the total number of requests in the batch.
     */

    public void setRequestCounts(xy.ai.conscious.connector.openapi.anthropic.components.RequestCounts value) {
        ((ObjectNode) node).set("request_counts", value == null ? null : value.node());
    }

    /**
     * URL to a `.jsonl` file containing the results of the Message Batch requests. Specified only once processing ends.
     * 
     * Results in the file are not guaranteed to be in the same order as requests. Use the `custom_id` field to match results to requests.
     */

    public xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation getResultsUrl() {
        JsonNode child = node.get("results_url");
        if (child == null) {
            return null;
        }
        return new xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation(child);
    }

    /**
     * URL to a `.jsonl` file containing the results of the Message Batch requests. Specified only once processing ends.
     * 
     * Results in the file are not guaranteed to be in the same order as requests. Use the `custom_id` field to match results to requests.
     */

    public void setResultsUrl(xy.ai.conscious.connector.openapi.anthropic.operators.AnyOfExplanation value) {
        ((ObjectNode) node).set("results_url", value == null ? null : value.node());
    }

    /**
     * Object type.
     * 
     * For Message Batches, this is always `"message_batch"`.
     */

    public xy.ai.conscious.connector.openapi.anthropic.components.BodyTypeEnum getType() {
        JsonNode child = node.get("type");
        if (child == null || child.isNull()) {
            return null;
        }
        return xy.ai.conscious.connector.openapi.anthropic.components.BodyTypeEnum.fromValue(child.asText());
    }

    /**
     * Object type.
     * 
     * For Message Batches, this is always `"message_batch"`.
     */

    public void setType(xy.ai.conscious.connector.openapi.anthropic.components.BodyTypeEnum value) {
        ((ObjectNode) node).put("type", value == null ? null : value.rawValue());
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
