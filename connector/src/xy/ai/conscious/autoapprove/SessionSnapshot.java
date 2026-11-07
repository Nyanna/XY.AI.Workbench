package xy.ai.conscious.autoapprove;

import java.time.Instant;

import xy.ai.conscious.model.TokenStats;

/**
 * Session state as seen by the rules. Includes the answer under evaluation.
 *
 * @param turn   number of answers carrying stats; control request answers are not counted
 * @param tokens accumulated over all answers of the session
 */
public record SessionSnapshot(String id, int turn, TokenStats tokens, Instant lastActivity) {
}
