package xy.ai.conscious.connector.claude;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.stream.Collectors;
import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.batch.BatchState;
import xy.ai.conscious.batch.NewBatch;
import xy.ai.conscious.connector.IAIBatch;
import xy.ai.conscious.connector.IAIBatchConnector;
import xy.ai.conscious.connector.openapi.anthropic.V1ClientImpl;
import xy.ai.conscious.connector.openapi.anthropic.components.MessageBatch;
import xy.ai.conscious.connector.openapi.anthropic.components.OneOfError;
import xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.results.code200.x_jsonl.BodyList;
import xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.results.code200.x_jsonl.MessageBatchIndividualResponse;
import xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.results.code200.x_jsonl.OneOfResult;
import xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.CreateMessageBatchParams;
import xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.MessageBatchIndividualRequestParams;
import xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.RequestsList;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.IModelRequest;
import xy.ai.conscious.model.Model.KeyPattern;

public class ClaudeBatchConnector implements IAIBatchConnector {
  private static final long PAGE_SIZE = 100;
  private V1ClientImpl client;
  private ClaudeConnector connector;

  public ClaudeBatchConnector(ConfigManager cfg, ClaudeConnector connector) {
    this.connector = connector;
    cfg.addKeyObs(k -> {
      if (KeyPattern.Claude.matches(k))
        this.client = ClaudeConnector.createClient(k);
    }, true);
  }

  @Override
  public List<IAIBatch> updateBatches(IProgress mon) {
    List<IAIBatch> batches = new ArrayList<>();
    String afterId = null;
    boolean hasMore = true;
    while (hasMore) {
      var resp = client.message_batches_list(null, afterId, PAGE_SIZE);
      if (resp.getCode200() == null)
        throw new IllegalStateException("List batches failed (" + resp.statusCode() + "): " + resp.body());
      var page = resp.getCode200().getJson();
      var data = page.getData();
      for (int i = 0; data != null && i < data.size(); i++) {
        MessageBatch batch = data.get(i);
        batches.add(new ClaudeBatch(batch.getId(), batch));
      }
      var lastId = page.getLastId();
      afterId = lastId != null && lastId.isString() ? lastId.getString() : null;
      hasMore = Boolean.TRUE.equals(page.getHasMore()) && afterId != null;
    }
    return batches;
  }

  @Override
  public IAIBatch cancelBatch(IAIBatch entry, IProgress mon) {
    var resp = client.message_batches_cancel(entry.getID());
    if (resp.getCode200() == null)
      throw new IllegalStateException("Cancel batch failed (" + resp.statusCode() + "): " + resp.body());
    MessageBatch batch = resp.getCode200().getJson();
    return new ClaudeBatch(batch.getId(), batch);
  }

  @Override
  public void loadBatch(IAIBatch entry, IProgress mon) {
    IProgress sub = mon.setTask("Load batch", 2);
    sub.setTaskName("Retrieve Batch");
    var resp = client.message_batches_retrieve(entry.getID());
    if (resp.getCode200() == null)
      throw new IllegalStateException("Retrieve batch failed (" + resp.statusCode() + "): " + resp.body());
    MessageBatch batch = resp.getCode200().getJson();
    ClaudeBatch oentry = ((ClaudeBatch) entry);
    oentry.setBatch(batch);
    sub.worked();
    sub.setTaskName("Retrieve Outputs");
    if ((oentry.getAnswers() == null || oentry.getAnswers().isEmpty()) && (BatchState.Completed.equals(entry.getState()) || BatchState.Failed.equals(entry.getState()))) {
      var results = client.message_batches_results(batch.getId());
      if (results.getCode200() == null)
        throw new IllegalStateException("Retrieve batch results failed (" + results.statusCode() + "): " + results.body());
      BodyList list = results.getCode200().getXJsonl();
      IProgress sub1 = mon.setTask("Load output", entry.getTaskCount());
      List<AIAnswer> answers = new ArrayList<>();
      for (int i = 0; i < list.size(); i++) answers.add(convertResult(list.get(i), sub1));
      oentry.setAnswers(answers);
    }
    sub.worked();
  }

  private AIAnswer convertResult(MessageBatchIndividualResponse res, IProgress progress) {
    OneOfResult result = res.getResult();
    if (result.isSucceededResult())
      return connector.convertResponse(new ClaudeResponse(result.getSucceededResult().getMessage(), res.getCustomId()), progress.subProgress());
    AIAnswer an = new AIAnswer(res.getCustomId());
    if (result.isErroredResult())
      an.answer = describeError(result.getErroredResult().getError().getError());
    else if (result.isCanceledResult())
      an.answer = "Canceled: " + result.getCanceledResult();
    else if (result.isExpiredResult())
      an.answer = "Expired: " + result.getExpiredResult();
    progress.worked();
    return an;
  }

  private String describeError(OneOfError error) {
    if (error.isAPIError())
      return "API Error: " + error.getAPIError().getMessage();
    if (error.isInvalidRequestError())
      return "Invalid Request: " + error.getInvalidRequestError().getMessage();
    if (error.isAuthenticationError())
      return "Authentication Error: " + error.getAuthenticationError().getMessage();
    if (error.isBillingError())
      return "Billing Problem: " + error.getBillingError().getMessage();
    if (error.isNotFoundError())
      return "Not Found: " + error.getNotFoundError().getMessage();
    if (error.isOverloadedError())
      return "Overloaded: " + error.getOverloadedError().getMessage();
    if (error.isPermissionError())
      return "Permission Error: " + error.getPermissionError().getMessage();
    if (error.isRateLimitError())
      return "Rate Limit: " + error.getRateLimitError().getMessage();
    if (error.isGatewayTimeoutError())
      return "Timeout: " + error.getGatewayTimeoutError().getMessage();
    return "Error: " + error;
  }

  @Override
  public String requestsToJson(Collection<IModelRequest> reqs) {
    return requestsToJsonInner(reqs.stream().map(r -> ((ClaudeRequest) r)).collect(Collectors.toList()));
  }

  private String requestsToJsonInner(Collection<ClaudeRequest> requests) {
    return requests.stream().map((r) -> r.params.toString()).collect(Collectors.joining("\n"));
  }

  @Override
  public IAIBatch submitBatch(NewBatch entry, IProgress mon) {
    IProgress sub = mon.setTask("Submit batch", 3);
    sub.setTaskName("Collect requests");
    List<ClaudeRequest> reqs = entry.getRequests().stream().map(r -> (ClaudeRequest) r).collect(Collectors.toList());
    sub.worked();
    sub.setTaskName("Convert requests");
    IProgress sub1 = mon.setTask("Convert request", reqs.size());
    RequestsList requests = new RequestsList();
    for (ClaudeRequest req : reqs) {
      MessageBatchIndividualRequestParams item = new MessageBatchIndividualRequestParams();
      item.setCustomId(req.getID());
      item.setParams(req.params);
      requests.add(item);
      sub1.worked();
    }
    CreateMessageBatchParams params = new CreateMessageBatchParams();
    params.setRequests(requests);
    sub.worked();
    sub.setTaskName("Submit batch");
    var resp = client.message_batches_post(params);
    if (resp.getCode200() == null)
      throw new IllegalStateException("Submit batch failed (" + resp.statusCode() + "): " + resp.body());
    MessageBatch batch = resp.getCode200().getJson();
    sub.worked();
    return new ClaudeBatch(batch.getId(), batch);
  }

  @Override
  public void convertAnswers(IAIBatch obj, IProgress mon) {
    // already converted while loading the batch
  }
}
