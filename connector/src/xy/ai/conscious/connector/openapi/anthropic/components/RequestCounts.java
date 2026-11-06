package xy.ai.conscious.connector.openapi.anthropic.components;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;

/** Proxy over a JSON object node; holds no data of its own. */
public final class RequestCounts {

    private final JsonNode node;

    public RequestCounts(JsonNode node) {
        this.node = node;
    }

    /** Backed by a fresh, empty JSON object; populate it via the setters below. */
    public RequestCounts() {
        this(JsonNodeFactory.instance.objectNode());
    }

    public JsonNode node() {
        return node;
    }

    /**
     * Number of requests in the Message Batch that have been canceled.
     * 
     * This is zero until processing of the entire Message Batch has ended.
     */

    public Long getCanceled() {
        JsonNode child = node.get("canceled");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * Number of requests in the Message Batch that have been canceled.
     * 
     * This is zero until processing of the entire Message Batch has ended.
     */

    public void setCanceled(Long value) {
        ((ObjectNode) node).put("canceled", value);
    }

    /**
     * Number of requests in the Message Batch that encountered an error.
     * 
     * This is zero until processing of the entire Message Batch has ended.
     */

    public Long getErrored() {
        JsonNode child = node.get("errored");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * Number of requests in the Message Batch that encountered an error.
     * 
     * This is zero until processing of the entire Message Batch has ended.
     */

    public void setErrored(Long value) {
        ((ObjectNode) node).put("errored", value);
    }

    /**
     * Number of requests in the Message Batch that have expired.
     * 
     * This is zero until processing of the entire Message Batch has ended.
     */

    public Long getExpired() {
        JsonNode child = node.get("expired");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * Number of requests in the Message Batch that have expired.
     * 
     * This is zero until processing of the entire Message Batch has ended.
     */

    public void setExpired(Long value) {
        ((ObjectNode) node).put("expired", value);
    }

    /**
     * Number of requests in the Message Batch that are processing.
     */

    public Long getProcessing() {
        JsonNode child = node.get("processing");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * Number of requests in the Message Batch that are processing.
     */

    public void setProcessing(Long value) {
        ((ObjectNode) node).put("processing", value);
    }

    /**
     * Number of requests in the Message Batch that have completed successfully.
     * 
     * This is zero until processing of the entire Message Batch has ended.
     */

    public Long getSucceeded() {
        JsonNode child = node.get("succeeded");
        if (child == null || child.isNull()) {
            return null;
        }
        return child.asLong();
    }

    /**
     * Number of requests in the Message Batch that have completed successfully.
     * 
     * This is zero until processing of the entire Message Batch has ended.
     */

    public void setSucceeded(Long value) {
        ((ObjectNode) node).put("succeeded", value);
    }

    @Override
    public String toString() {
        return xy.ai.conscious.connector.openapi.anthropic.JsonSupport.write(node);
    }
}
