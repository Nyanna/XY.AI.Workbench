package xy.ai.workbench.connectors.openai;

import static com.openai.core.ObjectMappers.jsonMapper;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Collection;
import java.util.List;
import java.util.Random;
import java.util.stream.Collectors;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.openai.client.OpenAIClient;
import com.openai.client.okhttp.OpenAIOkHttpClient;
import com.openai.core.JsonValue;
import com.openai.core.http.HttpResponse;
import com.openai.models.batches.Batch;
import com.openai.models.batches.BatchCreateParams;
import com.openai.models.batches.BatchCreateParams.CompletionWindow;
import com.openai.models.batches.BatchCreateParams.Endpoint;
import com.openai.models.batches.BatchCreateParams.Metadata;
import com.openai.models.batches.BatchListPage;
import com.openai.models.batches.BatchListParams;
import com.openai.models.files.FileCreateParams;
import com.openai.models.files.FileObject;
import com.openai.models.files.FilePurpose;
import com.openai.models.responses.ResponseCreateParams;
import com.openai.models.responses.ResponseCreateParams.Body;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.batch.BatchState;
import xy.ai.workbench.connectors.IAIBatchConnector;
import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.models.IModelRequest;

public class OpenAIBatchConnector implements IAIBatchConnector {
	private OpenAIClient client;

	public OpenAIBatchConnector(ConfigManager cfg) {
		cfg.addKeyObs(k -> {
			if (KeyPattern.OpenAI.matches(k))
				this.client = OpenAIOkHttpClient.builder().apiKey(cfg.getKeys()).build();
		}, true);
	}

	@Override
	public List<IAIBatch> updateBatches() {
		BatchListParams bparams = BatchListParams.builder() //
				.limit(100) //
				.build();

		BatchListPage res = client.batches().list(bparams);

		return res.data().stream().map(b -> new OpenAIBatch(b.id(), b)).collect(Collectors.toList());
	}

	@Override
	public IAIBatch submitBatch(String json, Collection<String> reqIds) {
		Path tempFile;
		try {
			tempFile = Files.createTempFile("mydata", ".jsonl");
			Files.write(tempFile, json.getBytes(StandardCharsets.UTF_8));
		} catch (IOException e) {
			throw new IllegalStateException(e);
		}

		FileCreateParams fileParams = FileCreateParams.builder() //
				.purpose(FilePurpose.BATCH) //
				.file(tempFile).build();
		FileObject file = client.files().create(fileParams);

		Metadata meta = Metadata.builder() //
				.putAdditionalProperty(AIBatchManager.KEY_REQIDS, JsonValue.from(//
						reqIds.stream().collect(Collectors.joining(",")))) //
				.build();

		BatchCreateParams batchParams = BatchCreateParams.builder() //
				.inputFileId(file.id()) //
				.metadata(meta).endpoint(Endpoint.V1_RESPONSES) //
				.completionWindow(CompletionWindow._24H) //
				.build();
		Batch returned = client.batches().create(batchParams);
		return new OpenAIBatch(returned.id(), returned);
	}

	@Override
	public IAIBatch cancelBatch(IAIBatch entry) {
		if (BatchState.Proccessing.equals(entry.getState())) {
			Batch batch = client.batches().cancel(entry.getID());
			return new OpenAIBatch(batch.id(), batch);
		}
		return null;
	}

	@Override
	public void loadBatch(IAIBatch entry) {
		OpenAIBatch oentry = ((OpenAIBatch) entry);
		Batch batch = client.batches().retrieve(oentry.getID());
		oentry.setBatch(batch);
		;

		if (oentry.getError() == null && batch.errorFileId().isPresent())
			oentry.setError(getFileAsString(batch.errorFileId().get()));

		if (oentry.getResult() == null && batch.outputFileId().isPresent())
			oentry.setResult(getFileAsString(batch.outputFileId().get()));
	}

	private String getFileAsString(String fileId) {
		try (HttpResponse response = client.files().content(fileId)) {
			ByteArrayOutputStream bos = new ByteArrayOutputStream();
			response.body().transferTo(bos);
			String result = new String(bos.toByteArray());
			return result;
		} catch (IOException e) {
			throw new IllegalStateException(e);
		}
	}

	private String requestToJson(ResponseCreateParams param) {
		try {
			return jsonMapper().writeValueAsString(new BatchElement(param._body()));
		} catch (JsonProcessingException e) {
			throw new IllegalStateException(e);
		}
	}

	private static class BatchElement {
		@JsonProperty
		public String custom_id = new Random().nextInt(Integer.MAX_VALUE) + "_id";

		@JsonProperty
		public String method = "POST";

		@JsonProperty
		public String url = "/v1/responses";

		@JsonProperty
		public Body body;

		public BatchElement(Body body) {
			this.body = body;
		}

	}

	@Override
	public String requestsToJson(Collection<IModelRequest> reqs) {
		return requestsToJsonInner(
				reqs.stream().map(r -> ((OpenAIModelRequest) r).reqquest).collect(Collectors.toList()));
	}

	private String requestsToJsonInner(Collection<ResponseCreateParams> requests) {
		return requests.stream().map((r) -> requestToJson(r)).collect(Collectors.joining("\n"));
	}
}
