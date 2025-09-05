package xy.ai.workbench.connectors.google;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.stream.Collectors;

import com.google.genai.Client;
import com.google.genai.Pager;
import com.google.genai.types.BatchJob;
import com.google.genai.types.BatchJobDestination;
import com.google.genai.types.BatchJobSource;
import com.google.genai.types.CancelBatchJobConfig;
import com.google.genai.types.CreateBatchJobConfig;
import com.google.genai.types.DownloadFileConfig;
import com.google.genai.types.GenerateContentResponse;
import com.google.genai.types.GetBatchJobConfig;
import com.google.genai.types.InlinedRequest;
import com.google.genai.types.InlinedResponse;
import com.google.genai.types.JobError;
import com.google.genai.types.ListBatchJobsConfig;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.batch.BatchState;
import xy.ai.workbench.batch.NewBatch;
import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.connectors.IAIBatchConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;

public class GeminiBatchConnector implements IAIBatchConnector {
	private Client client;
	private GeminiConnector connector;
	private ConfigManager cfg;

	public GeminiBatchConnector(ConfigManager cfg, GeminiConnector connector) {
		this.cfg = cfg;
		this.connector = connector;
		cfg.addKeyObs(k -> {
			if (KeyPattern.Gemini.matches(k))
				this.client = Client.builder()//
						.apiKey(k)//
						.build();
		}, true);
	}

	@Override
	public List<IAIBatch> updateBatches() {

		ListBatchJobsConfig param = ListBatchJobsConfig.builder()//
				.pageSize(100)//
				.build();
		Pager<BatchJob> res = client.batches.list(param);

		List<BatchJob> batches = new ArrayList<>();
		for (BatchJob job : res)
			batches.add(job);
		return batches.stream().map(b -> new GeminiBatch(b.name().orElse("unkown"), b)).collect(Collectors.toList());
	}

	@Override
	public IAIBatch cancelBatch(IAIBatch entry) {
		if (BatchState.Proccessing.equals(entry.getState())) {
			CancelBatchJobConfig params = CancelBatchJobConfig.builder().build();
			client.batches.cancel(entry.getID(), params);

			GetBatchJobConfig paramsg = GetBatchJobConfig.builder().build();
			BatchJob batch = client.batches.get(entry.getID(), paramsg);
			return new GeminiBatch(batch.name().orElseThrow(), batch);
		}
		return null;
	}

	@Override
	public void loadBatch(IAIBatch entry) {
		GeminiBatch oentry = ((GeminiBatch) entry);
		GetBatchJobConfig paramsg = GetBatchJobConfig.builder().build();
		BatchJob batch = client.batches.get(entry.getID(), paramsg);
		oentry.setBatch(batch);

		if (BatchState.Completed.equals(entry.getState()) && oentry.getResult() == null)
			if (batch.dest().isPresent()) {
				BatchJobDestination dest = batch.dest().get();
				if (dest.fileName().isPresent()) {

					DownloadFileConfig params = DownloadFileConfig.builder().build();

					try {
						Path tempFile = Files.createTempFile("mydata", ".jsonl");
						client.files.download(dest.fileName().get(), tempFile.toAbsolutePath().toString(), params);

						String content = Files.readString(tempFile.toAbsolutePath());
						oentry.setResult(content);

					} catch (IOException e) {
						throw new IllegalStateException(e);
					}

				} else if (dest.inlinedResponses().isPresent()) {
					List<AIAnswer> answers = new ArrayList<>();
					List<InlinedResponse> inline = dest.inlinedResponses().get();
					for (InlinedResponse resp : inline) {

						StringBuffer res = new StringBuffer();
						if (resp.error().isPresent())
							res.append(errorToString(resp.error().get()));
						if (resp.response().isPresent()) {
							GenerateContentResponse r = resp.response().get();
							AIAnswer ans = connector.convertResponse(new GeminiModelResponse(r));
							answers.add(ans);
						}
					}
					oentry.setAnswers(answers);
				}
			}
	}

	private String errorToString(JobError error) {
		String msg = error.code().orElse(-1) + ": ";
		error.message().orElse("");
		if (error.details().isPresent())
			msg += " (" + error.details().get().stream().collect(Collectors.joining(" ,")) + ")";
		return msg;
	}

	@Override
	public String requestsToJson(Collection<IModelRequest> reqs) {
		return requestsToJsonInner(reqs.stream().map(r -> ((GeminiModelRequest) r)).collect(Collectors.toList()));
	}

	private String requestsToJsonInner(Collection<GeminiModelRequest> requests) {
		return requests.stream().map((r) -> requestToJson(r)).collect(Collectors.joining("\n"));
	}

	private String requestToJson(GeminiModelRequest r) {
		return generateBatchJobSource(List.of(r)).toJson();
	}

	@Override
	public IAIBatch submitBatch(NewBatch entry) {

		String reqIds = String.join(",", entry.getRequestIDs());
		BatchJobSource bjs = generateBatchJobSource(entry.getRequests());
		CreateBatchJobConfig cbjc = CreateBatchJobConfig.builder()//
				.displayName(reqIds)//
				.build();

		BatchJob batch = client.batches.create(cfg.getModel().apiName, bjs, cbjc);
		return new GeminiBatch(batch.name().orElseThrow(), batch);
	}

	private BatchJobSource generateBatchJobSource(List<IModelRequest> request) {
		List<InlinedRequest> inlines = new ArrayList<>();

		List<GeminiModelRequest> reqs = request.stream().map(r -> (GeminiModelRequest) r)
				.collect(Collectors.toList());
		for (GeminiModelRequest req : reqs) {
			inlines.add(InlinedRequest.builder()//
					.model(req.model.apiName)//
					.config(req.config) //
					.contents(req.prompt)//
					.build());
		}

		BatchJobSource bjs = BatchJobSource.builder()//
				.inlinedRequests(inlines)//
				.build();
		return bjs;
	}

	@Override
	public void convertAnswers(IAIBatch obj) {
		GeminiBatch entry = ((GeminiBatch) obj);
		BatchJob batch = entry.getBatch();
		// could already contain inline answers
		List<AIAnswer> answ = new ArrayList<>(entry.getAnswers());

		if (entry.getResult() != null)
			for (String InputJson : entry.getResult().split("\n"))
				answ.add(convertToAnswer(InputJson));

		if (batch.error().isPresent()) {
			JobError error = batch.error().get();
			String msg = errorToString(error);
			AIAnswer eans = new AIAnswer();
			eans.answer = msg;
			answ.add(eans);
		}

		obj.setAnswers(answ);
	}

	private AIAnswer convertToAnswer(String responseJson) {
		GenerateContentResponse resp = GenerateContentResponse.fromJson(responseJson);

		AIAnswer answer = connector.convertResponse(new GeminiModelResponse(resp));
		return answer;
	}
}
