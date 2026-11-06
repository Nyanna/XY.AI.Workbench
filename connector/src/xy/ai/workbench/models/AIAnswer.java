package xy.ai.workbench.models;

import java.util.regex.Matcher;
import java.util.regex.Pattern;
import xy.ai.workbench.connector.harness.Prompt;

public class AIAnswer {
  public static final String RESULT = "Result Stats: ";
  private static final Pattern ID_PATTERN = Pattern.compile("id=([^,]*), ");
  public final String id;
  public String sessionId;
  public final TokenStats stats = new TokenStats();
  public final TimeStats time = new TimeStats();
  public String answer = "";
  public String instructions = "";
  public boolean showStats = true;
  /**
   * Prompt that produced this answer; null once no longer available
   */
  public Prompt prompt;

  public AIAnswer(String id) {
    this.id = id;
    this.sessionId = id;
  }

  @Override
  public String toString() {
    return String.format("AIAnswer [stats=%s]", stats);
  }

  public String print() {
    return String.format("%sid=%s, %s, %s", RESULT, sessionId, stats.print(), time.print());
  }

  public static AIAnswer fromString(String line) {
    if (line == null)
      return null;
    Matcher idMatcher = ID_PATTERN.matcher(line);
    if (!idMatcher.find())
      return null;
    String rest = line.substring(idMatcher.end());
    TokenStats stats = TokenStats.fromString(rest);
    if (stats == null)
      return null;
    AIAnswer ans = new AIAnswer(idMatcher.group(1));
    ans.stats.add(stats);
    TimeStats t = TimeStats.fromString(rest);
    if (t != null)
      ans.time.add(t);
    return ans;
  }
}
