package xy.ai.workbench.connectors.claude;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.stream.Collectors;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;

import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.core.ObjectMappers;
import com.anthropic.core.http.StreamResponse;
import com.anthropic.models.ErrorObject;
import com.anthropic.models.messages.batches.BatchCreateParams;
import com.anthropic.models.messages.batches.BatchCreateParams.Builder;
import com.anthropic.models.messages.batches.BatchListPage;
import com.anthropic.models.messages.batches.MessageBatch;
import com.anthropic.models.messages.batches.MessageBatchIndividualResponse;
import com.fasterxml.jackson.core.JsonProcessingException;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.batch.BatchState;
import xy.ai.workbench.batch.NewBatch;
import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.connectors.IAIBatchConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;

public class ClaudeBatchConnector implements IAIBatchConnector {
	private AnthropicClient client;
	private ClaudeConnector connector;

	public ClaudeBatchConnector(ConfigManager cfg, ClaudeConnector connector) {
		this.connector = connector;
		cfg.addKeyObs(k -> {
			if (KeyPattern.Claude.matches(k))
				this.client = AnthropicOkHttpClient.builder().apiKey(k).build();
		}, true);
	}

	@Override
	public List<IAIBatch> updateBatches(IProgressMonitor mon) {

		BatchListPage page = client.messages().batches().list();

		List<MessageBatch> batches = new ArrayList<>();
		for (MessageBatch batch : page.autoPager())
			batches.add(batch);

		return batches.stream().map(b -> new ClaudeBatch(b.id(), b)).collect(Collectors.toList());
	}

	@Override
	public IAIBatch cancelBatch(IAIBatch entry, IProgressMonitor mon) {
		MessageBatch batch = client.messages().batches().cancel(entry.getID());
		return new ClaudeBatch(batch.id(), batch);
	}

	@Override
	public void loadBatch(IAIBatch entry, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Load batch", 2);

		sub.subTask("Retrieve Batch");
		MessageBatch batch = client.messages().batches().retrieve(entry.getID());
		ClaudeBatch oentry = ((ClaudeBatch) entry);
		oentry.setBatch(batch);
		sub.worked(1);

		sub.subTask("Retrieve Outputs");
		if ((oentry.getAnswers() == null || oentry.getAnswers().isEmpty())
				&& (BatchState.Completed.equals(entry.getState()) || BatchState.Failed.equals(entry.getState())))
			try (StreamResponse<MessageBatchIndividualResponse> response = client.messages().batches()
					.resultsStreaming(batch.id())) {

				SubMonitor sub1 = SubMonitor.convert(mon, "Load output", entry.getTaskCount());
				oentry.setAnswers(response.stream().map(res -> {
					sub1.worked(1);
					AIAnswer an = new AIAnswer(res.customId());
					if (res.result().isSucceeded())
						return connector.convertResponse(new ClaudeResponse(res.result().asSucceeded().message()), sub);
					else if (res.result().isErrored()) {
						ErrorObject error = res.result().asErrored().error().error();

						if (error.isApiError())
							an.answer = "API Error: " + error.asApiError().message();
						else if (error.isInvalidRequestError())
							an.answer = "Invalid Request: " + error.asInvalidRequestError().message();
						else if (error.isAuthenticationError())
							an.answer = "Authentication Error: " + error.asAuthenticationError().message();
						else if (error.isBillingError())
							an.answer = "Billing Problem: " + error.asBillingError().message();
						else if (error.isNotFoundError())
							an.answer = "Not Found: " + error.asNotFoundError().message();
						else if (error.isOverloadedError())
							an.answer = "Overloaded: " + error.asOverloadedError().message();
						else if (error.isPermissionError())
							an.answer = "Permission Error: " + error.asPermissionError().message();
						else if (error.isRateLimitError())
							an.answer = "Rate Limit: " + error.asRateLimitError().message();
						else if (error.isTimeoutError())
							an.answer = "Timeout: " + error.asTimeoutError().message();
						else
							an.answer = "Error: " + error.toString();
					} else if (res.result().isCanceled())
						an.answer = "Canceled: " + res.result().asCanceled().toString();
					else if (res.result().isExpired())
						an.answer = "Expired: " + res.result().asExpired().toString();
					return an;
				}).collect(Collectors.toList()));
				sub1.done();
			}
		sub.worked(1);
		sub.done();
	}

	@Override
	public String requestsToJson(Collection<IModelRequest> reqs) {
		return requestsToJsonInner(reqs.stream().map(r -> ((ClaudeRequest) r)).collect(Collectors.toList()));
	}

	private String requestsToJsonInner(Collection<ClaudeRequest> requests) {
		return requests.stream().map((r) -> requestToJson(r)).collect(Collectors.joining("\n"));
	}

	private String requestToJson(ClaudeRequest r) {
		try {
			return ObjectMappers.jsonMapper().writeValueAsString(r.params._body());
		} catch (JsonProcessingException e) {
			throw new IllegalStateException("Unable to convert", e);
		}
	}

	@Override
	public IAIBatch submitBatch(NewBatch entry, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Submit batch", 3);
		sub.subTask("Collect requests");

		Builder builder = BatchCreateParams.builder();
		List<ClaudeRequest> reqs = entry.getRequests().stream().map(r -> (ClaudeRequest) r)
				.collect(Collectors.toList());
		sub.worked(1);

		sub.subTask("Convert requests");
		SubMonitor sub1 = SubMonitor.convert(sub, "Convert request", reqs.size());
		for (ClaudeRequest req : reqs) {
			com.anthropic.models.messages.batches.BatchCreateParams.Request.Builder pbuilder = BatchCreateParams.Request
					.builder();

			String json = requestToJson(req);
			BatchCreateParams.Request.Params batchRequestParams;
			try {
				batchRequestParams = ObjectMappers.jsonMapper().readerFor(BatchCreateParams.Request.Params.class)
						.readValue(json);
			} catch (JsonProcessingException e) {
				throw new IllegalStateException("Unable to convert", e);
			}

			pbuilder.customId(req.getID());
			pbuilder.params(batchRequestParams);
			builder.addRequest(pbuilder.build());
			sub1.worked(1);
		}
		sub1.done();
		sub.worked(1);

		sub.subTask("Submit batch");
		MessageBatch batch = client.messages().batches().create(builder.build());
		sub.done();

		return new ClaudeBatch(batch.id(), batch);
	}

	@Override
	public void convertAnswers(IAIBatch obj, IProgressMonitor mon) {
		// allready converted from SDK
	}
}
