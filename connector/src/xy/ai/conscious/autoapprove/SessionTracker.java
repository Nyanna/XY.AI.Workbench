package xy.ai.conscious.autoapprove;

import java.time.Instant;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.TokenStats;

final class SessionTracker {

	private static final class State {
		int turn;
		final TokenStats tokens = new TokenStats();
		Instant lastActivity;
		Prompt lastPrompt;
	}

	private final Map<String, State> sessions = new ConcurrentHashMap<>();

	/** Connectors without own session id leave {@code sessionId == id}; fall back to the prompt's session. */
	static String keyOf(AIAnswer ans) {
		if (ans.sessionId != null && !ans.sessionId.equals(ans.id))
			return ans.sessionId;
		return ans.prompt != null ? ans.prompt.sessionId() : ans.id;
	}

	SessionSnapshot record(AIAnswer ans) {
		String key = keyOf(ans);
		State s = sessions.computeIfAbsent(key, k -> new State());
		synchronized (s) {
			if (ans.showStats)
				s.turn++;
			s.tokens.add(ans.stats);
			s.lastActivity = Instant.now();
			if (ans.prompt != null)
				s.lastPrompt = ans.prompt;
			TokenStats copy = new TokenStats();
			copy.add(s.tokens);
			return new SessionSnapshot(key, s.turn, copy, s.lastActivity);
		}
	}

	Prompt lastPrompt(String key) {
		State s = sessions.get(key);
		if (s == null)
			return null;
		synchronized (s) {
			return s.lastPrompt;
		}
	}

	void forget(String key) {
		sessions.remove(key);
	}
}
