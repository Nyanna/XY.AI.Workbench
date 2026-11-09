package xy.ai.conscious.autoapprove;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.function.Consumer;
import xy.ai.conscious.api.IActiveEditor.Selection;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
import xy.ai.conscious.autoapprove.rules.IApprovalRule;
import xy.ai.conscious.autoapprove.rules.Vote;
import xy.ai.conscious.commands.AnswerCommand;
import xy.ai.conscious.commands.CallCommand;
import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.core.LOG;
import xy.ai.conscious.model.AIAnswer;

public class AutoApprover {
  public static final int DEFAULT_MAX_CHAIN = 20;
  private final RuleRegistry registry;
  private final SessionTracker sessions = new SessionTracker();
  private final List<Consumer<Decision>> listeners = new CopyOnWriteArrayList<>();
  private volatile int maxChain = DEFAULT_MAX_CHAIN;
  private volatile String idleInstruction = "Continue with the task.";
  private volatile boolean enabled = true;

  public AutoApprover() {
    this(new RuleRegistry());
  }

  public AutoApprover(RuleRegistry registry) {
    this.registry = registry;
  }

  public RuleRegistry rules() {
    return registry;
  }

  public void addListener(Consumer<Decision> listener) {
    listeners.add(listener);
  }

  public void removeListener(Consumer<Decision> listener) {
    listeners.remove(listener);
  }

  public boolean isEnabled() {
    return enabled;
  }

  /**
   * Globally switches the processing on or off; while off, answers pass untouched.
   */
  public void setEnabled(boolean enabled) {
    this.enabled = enabled;
  }

  /**
   * Maximum number of consecutive automatic follow-up prompts.
   */
  public void setMaxChain(int maxChain) {
    this.maxChain = maxChain;
  }

  public void setIdleInstruction(String instruction) {
    this.idleInstruction = instruction == null ? "" : instruction;
  }

  public void forget(String sessionId) {
    sessions.forget(sessionId);
  }

  public Decision evaluate(AIAnswer ans, int depth) {
    if (!enabled)
      return Decision.pass(ans.sessionId);
    SessionSnapshot session = sessions.record(ans);
    List<ApprovalItem> items = AnswerParser.parse(ans.answer);
    ApprovalItem item = items.size() == 1 ? items.get(0) : null;
    sessions.track(session.id(), items);
    ApprovalItem call = sessions.callFor(session.id(), item);
    ApprovalContext ctx = new ApprovalContext(ans, ans.prompt, item, call, session, registry);
    Decision decision = decide(ctx, collect(ctx), depth);
    enrich(ans, decision.warnings());
    publish(decision);
    return decision;
  }

  public Optional<FollowUp> onIdle(String sessionId) {
    if (!enabled)
      return Optional.empty();
    Prompt origin = sessions.lastPrompt(sessionId);
    String instruction = idleInstruction;
    if (origin == null || instruction.isBlank())
      return Optional.empty();
    FollowUp followUp = new FollowUp(origin, selection(instruction.lines().toList()));
    publish(new Decision(sessionId, ApprovalResult.Pass, null, null, null, null, instruction, null, followUp, 0));
    return Optional.of(followUp);
  }

  private List<RuleVote> collect(ApprovalContext ctx) {
    List<RuleVote> votes = new ArrayList<>();
    for (IApprovalRule rule : registry.rules()) {
      if (!ctx.settings(rule.id()).isActive())
        continue;
      try {
        Vote vote = rule.vote(ctx);
        votes.add(new RuleVote(rule.id(), vote == null ? Vote.abstain() : vote));
      } catch (RuntimeException e) {
        LOG.error("Approval rule failed: " + rule.id(), e);
      }
    }
    return votes;
  }

  private Decision decide(ApprovalContext ctx, List<RuleVote> votes, int depth) {
    List<String> reasons = new ArrayList<>();
    List<String> hints = new ArrayList<>();
    List<String> warnings = new ArrayList<>();
    List<String> instructions = new ArrayList<>();
    boolean deny = false;
    boolean hold = false;
    boolean allow = false;
    for (RuleVote rv : votes) {
      Vote v = rv.vote();
      warnings.addAll(v.warnings());
      addNonBlank(instructions, v.instruction());
      switch(v.verdict()) {
        case Deny ->
          {
            deny = true;
            addNonBlank(reasons, v.reason());
          }
        case Hold ->
          {
            hold = true;
          }
        case Allow ->
          {
            allow = true;
            addNonBlank(hints, v.hint());
          }
        case Abstain ->
          {
          }
      }
    }
    ApprovalResult result = resultFor(ctx.item(), deny, hold, allow);
    String reason = String.join("; ", reasons);
    String hint = String.join("; ", hints);
    String instruction = String.join("\n", instructions);
    FollowUp followUp = null;
    if (depth >= maxChain) {
      if (result != ApprovalResult.Pass || !instruction.isEmpty())
        warnings.add("auto-approve chain limit reached (" + maxChain + ")");
      result = ApprovalResult.Pass;
    } else {
      followUp = followUpFor(ctx, result, reason, hint, instruction);
      if (followUp == null)
        result = ApprovalResult.Pass;
    }
    return new Decision(ctx.session().id(), result, ctx.item(), reason, hint, warnings, instruction, votes, followUp, ctx.answer().answer == null ? 0 : ctx.answer().answer.length());
  }

  private static ApprovalResult resultFor(ApprovalItem item, boolean deny, boolean hold, boolean allow) {
    if (item == null || item.id().isBlank() || item.kind() == Kind.ToolResult)
      return ApprovalResult.Pass;
    if (deny)
      return ApprovalResult.Deny;
    if (hold || !allow)
      return ApprovalResult.Pass;
    return item.kind() == Kind.ToolCall ? ApprovalResult.Call : ApprovalResult.Allow;
  }

  private FollowUp followUpFor(ApprovalContext ctx, ApprovalResult result, String reason, String hint, String instruction) {
    Prompt origin = ctx.prompt();
    ApprovalItem item = ctx.item();
    if (origin == null)
      return null;
    List<String> lines = switch(result) {
      case Pass ->
        instruction.isBlank() ? null : instruction.lines().toList();
      case Allow ->
        withBlock(item, command(AnswerCommand.CMD_ANSWER + " " + item.id() + " allow", hint));
      case Deny ->
        item.isControl() ? withBlock(item, command(AnswerCommand.CMD_ANSWER + " " + item.id() + " deny", reason)) : List.of(denial(item, reason));
      case Call ->
        withBlock(item, CallCommand.CMD_CALL + " " + item.id());
    };
    return lines == null ? null : new FollowUp(origin, selection(lines));
  }

  /**
   * Yaml block followed by the command line, as the operator would select it.
   */
  private static List<String> withBlock(ApprovalItem item, String commandLine) {
    List<String> lines = new ArrayList<>();
    lines.add("```yaml");
    item.yaml().lines().forEach(lines::add);
    lines.add("```");
    lines.add(commandLine);
    return lines;
  }

  private static String denial(ApprovalItem item, String reason) {
    String tool = item.tool().isBlank() ? "" : " (" + item.tool() + ")";
    String why = oneLine(reason);
    return "Tool call " + item.id() + tool + " was denied" + (why.isEmpty() ? "." : ": " + why);
  }

  private static String command(String base, String extra) {
    String text = oneLine(extra);
    return text.isEmpty() ? base : base + " " + text;
  }

  private static String oneLine(String s) {
    return s == null ? "" : s.replaceAll("\\s+", " ").strip();
  }

  private static Selection selection(List<String> lines) {
    return new Selection(lines.toArray(String[]::new), null);
  }

  private static void addNonBlank(List<String> list, String text) {
    if (text != null && !text.isBlank())
      list.add(text);
  }

  /**
   * Warnings go in front so the command line stays the last line of the answer.
   */
  private static void enrich(AIAnswer ans, List<String> warnings) {
    if (warnings.isEmpty())
      return;
    StringBuilder sb = new StringBuilder();
    for (String w : warnings) sb.append("//warning: ").append(oneLine(w)).append('\n');
    ans.answer = sb + (ans.answer == null ? "" : ans.answer);
  }

  private void publish(Decision decision) {
    for (Consumer<Decision> l : listeners) {
      try {
        l.accept(decision);
      } catch (RuntimeException e) {
        LOG.error(e.getMessage(), e);
      }
    }
  }
}
