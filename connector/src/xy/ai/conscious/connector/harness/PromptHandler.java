package xy.ai.conscious.connector.harness;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;
import xy.ai.conscious.api.IActiveEditor;
import xy.ai.conscious.api.IExecutor;
import xy.ai.conscious.api.IExecutor.IMonitoredRunnable;
import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.api.ext.IGitSupport;
import xy.ai.conscious.autoapprove.AutoApprover;
import xy.ai.conscious.autoapprove.Decision;
import xy.ai.conscious.autoapprove.FollowUp;
import xy.ai.conscious.batch.AIBatchManager;
import xy.ai.conscious.commands.AnswerCommand;
import xy.ai.conscious.commands.CallCommand;
import xy.ai.conscious.commands.Command;
import xy.ai.conscious.commands.ToolCommand;
import xy.ai.conscious.connector.AdaptingConnector;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.conscious.core.IEditorInterface;
import xy.ai.conscious.core.LOG;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.IModelRequest;
import xy.ai.conscious.model.IModelResponse;
import xy.ai.conscious.model.Model;

/**
 * Prompt-specific orchestration extracted from {@code AISessionManager}: drives
 * the prepare/insertTag/execute/replaceTag job pipeline, delegating input
 * aggregation/preprocessing to {@link PromptInputHandler}.
 */
public class PromptHandler {
  private final AdaptingConnector connector;
  private final IEditorInterface editIfc;
  private final AIBatchManager batch;
  private final PromptInputHandler input;
  private final IGitSupport gitSupport;
  private final AutoApprover approver = new AutoApprover();
  private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();

  public PromptHandler(ConfigManager cfg, IActiveEditor editorListener, AdaptingConnector connector, AIBatchManager batch, IEditorInterface editIfc, IGitSupport gitSupport) {
    this.connector = connector;
    this.editIfc = editIfc;
    this.batch = batch;
    this.input = new PromptInputHandler(cfg, editorListener);
    this.gitSupport = gitSupport;
  }
  
  public AutoApprover approver() {
	return approver;
  }

  public void addAnswerObs(Consumer<AIAnswer> obs) {
    answerObs.add(obs);
  }

  public void execute(IExecutor exec) {
    exec.schedule("Starting Prompt", new PromptRun(exec));
  }

  private class PromptRun implements IMonitoredRunnable {
    private final IExecutor exec;
    private final Prompt preset;
    private final int turnDepth;

    private PromptRun(IExecutor exec) {
      this(exec, null, 0);
    }

    private PromptRun(IExecutor exec, Prompt preset, int depth) {
      this.exec = exec;
      this.preset = preset;
      this.turnDepth = depth;
    }

    @Override
    public void run(IProgress mon) {
      IProgress sub = mon.setTask("Executing prompt", 4);
      String reqId = null;
      Decision decision = null;
      try {
        sub.setTaskName("Prepare inputs");
        var req = prepareRequest(exec, false, preset, sub);
        sub.worked();
        sub.setTaskName("Insert Tag");
        editIfc.insertTag(exec, req, sub);
        reqId = req.getID();
        mon.worked();
        sub.setTaskName("Execute prompt");
        AIAnswer ans = executeInner(exec, req, sub);
        mon.worked();
        sub.setTaskName("Process Answer");
        Decision evaluated = evaluate(ans, turnDepth);
        editIfc.replaceTag(ans, sub);
        decision = evaluated;
        mon.worked();
      } catch (Exception e) {
        LOG.error(e.getMessage(), e);
        // uncatched error case
        if (reqId != null) {
          AIAnswer error = new AIAnswer(reqId);
          error.answer = e.getMessage();
          editIfc.replaceTag(error, sub);
        }
      } finally {
        mon.done();
      }
      if (decision != null && decision.followUp() != null)
        scheduleFollowUp(exec, decision.followUp(), turnDepth + 1);
    }
  }

  public void queueAsync(IExecutor exec) {
    exec.schedule("Enqueue Prompt", mon -> {
      queueSync(exec, mon);
    });
  }

  private void queueSync(IExecutor exec, IProgress mon) {
    IProgress sub = mon.setTask("Enqueue batch prompt", 3);
    sub.setTaskName("Prepare inputs");
    var req = prepareRequest(exec, true, sub.subProgress());
    sub.setTaskName("Insert Tag");
    editIfc.insertTag(exec, req, sub.subProgress());
    sub.setTaskName("Enqueue prompt");
    batch.enqueue(req, sub.subProgress());
  }

  public void queueAndSubmit(IExecutor exec) {
    exec.schedule("Enqueue Prompt", (mon) -> {
      queueSync(exec, mon);
      batch.submitBatches(mon);
    });
  }

  private IModelRequest prepareRequest(IExecutor exec, boolean batchFix, IProgress mon) {
    return prepareRequest(exec, batchFix, null, mon);
  }

  private IModelRequest prepareRequest(IExecutor exec, boolean batchFix, Prompt preset, IProgress mon) {
    IProgress sub = mon.setTask("Preparing Call", 1);
    sub.setTaskName("Preparing Call");
    Prompt prompt = preset != null ? preset : input.buildPrompt(exec, batchFix);
    preprocessPrompt(prompt);
    IModelRequest req = connector.createRequest(prompt, sub);
    req.setPrompt(prompt);
    sub.worked();
    return req;
  }

  private void preprocessPrompt(Prompt prompt) {
    for (String key : prompt.config.keys.split(",")) if (prompt.config.model.cap.acceptsKey(key))
      prompt.config.keys = key;
    if (prompt.arg.command instanceof CallCommand || prompt.arg.command instanceof ToolCommand)
      prompt.config.model = Model.MCP_TOOLS;
  }

  private AIAnswer executeInner(IExecutor exec, IModelRequest req, IProgress mon) {
    exec.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));
    IModelResponse resp = connector.executeRequest(req, mon);
    AIAnswer res = connector.convertResponse(resp, mon);
    res.prompt = req.getPrompt();
    exec.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));
    checkGitSupport(exec, req);
    return res;
  }

  /**
   * Entry point for an external idle trigger.
   */
  public void onIdle(IExecutor exec, String sessionId) {
    approver.onIdle(sessionId).ifPresent(f -> scheduleFollowUp(exec, f, 1));
  }

  private void scheduleFollowUp(IExecutor exec, FollowUp followUp, int depth) {
    try {
      Prompt prompt = input.followUp(followUp.origin(), followUp.selection());
      exec.schedule("Auto Approve", new PromptRun(exec, prompt, depth));
    } catch (RuntimeException e) {
      LOG.error(e.getMessage(), e);
    }
  }

  private Decision evaluate(AIAnswer ans, int depth) {
    try {
      return approver.evaluate(ans, depth);
    } catch (RuntimeException e) {
      LOG.error(e.getMessage(), e);
      return Decision.pass(ans.sessionId);
    }
  }

  public void checkGitSupport(IExecutor exec, IModelRequest req) {
    if (gitSupport == null)
      return;
    Prompt prompt = req.getPrompt();
    if (prompt == null || prompt.arg == null)
      return;
    Command cmd = prompt.arg.command;
    if (!(cmd instanceof AnswerCommand) && !(cmd instanceof CallCommand))
      return;
    gitSupport.triggerSnapshotIfNeeded(exec, req);
  }
}
