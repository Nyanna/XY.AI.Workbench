package xy.ai.conscious.connector.claudecode;

import com.fasterxml.jackson.databind.JsonNode;

/**
 * Answer of the operator to a control request, as accepted by the control endpoint.
 *
 * @param id control request id
 * @param rejected the request was denied
 * @param arguments arguments edited by the operator, {@code null} if the requested ones stand
 */
public record ControlApproval(String id, boolean rejected, JsonNode arguments) {
}
