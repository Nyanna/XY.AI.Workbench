package xy.ai.conscious.autoapprove;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
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
    final Deque<ApprovalItem> calls = new ArrayDeque<>();
  }
  static final int MAX_CALLS = 10;
  private static final Pattern CONTROL_ID = Pattern.compile("(.+)-(\\d+)-(\\d+)");
  private final Map<String, State> sessions = new ConcurrentHashMap<>();

  /**
   * Connectors without own session id leave {@code sessionId == id}; fall back to the prompt's session.
   */
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

  /**
   * Remembers the tool calls of the items; only the {@link #MAX_CALLS} most recent ones are kept.
   */
  void track(String key, List<ApprovalItem> items) {
    State s = sessions.computeIfAbsent(key, k -> new State());
    synchronized (s) {
      for (ApprovalItem item : items) {
        if (!isCall(item) || item.id().isBlank())
          continue;
        s.calls.removeIf(c -> c.id().equals(item.id()));
        s.calls.addLast(item);
        while (s.calls.size() > MAX_CALLS) s.calls.removeFirst();
      }
    }
  }

  /**
   * The remembered tool call the result belongs to, or {@code null}.
   */
  ApprovalItem callFor(String key, ApprovalItem result) {
    if (result == null || (result.kind() != Kind.ControlResult && result.kind() != Kind.ToolResult))
      return null;
    State s = sessions.get(key);
    String callId = callIdOf(result);
    if (s == null || callId == null)
      return null;
    synchronized (s) {
      for (ApprovalItem c : s.calls) {
        if (c.id().equals(callId) && c.isControl() == result.isControl())
          return c;
      }
      return null;
    }
  }

  /**
   * Control request ids are {@code <hash>-<number>-<ref>}: a call has ref 0, a result carries the number of its call
   * as ref. Other results use the id of their call.
   */
  static String callIdOf(ApprovalItem result) {
    if (!result.isControl())
      return result.id().isBlank() ? null : result.id();
    Matcher m = CONTROL_ID.matcher(result.id());
    if (!m.matches() || m.group(3).equals("0"))
      return null;
    return m.group(1) + "-" + m.group(3) + "-0";
  }

  private static boolean isCall(ApprovalItem item) {
    return item.kind() == Kind.ControlCall || item.kind() == Kind.ToolCall;
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
