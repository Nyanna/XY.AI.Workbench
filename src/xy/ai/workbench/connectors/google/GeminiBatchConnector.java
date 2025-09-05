package xy.ai.workbench.connectors.google;

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

import com.google.genai.Client;

import kotlin.NotImplementedError;
import xy.ai.workbench.Activator;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.batch.BatchState;
import xy.ai.workbench.connectors.IAIBatchConnector;
import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.models.IModelRequest;

public class GeminiBatchConnector implements IAIBatchConnector {
	private Client client;

	public GeminiBatchConnector(ConfigManager cfg) {
		cfg.addKeyObs(k -> {
			if (KeyPattern.Gemini.matches(k))
				this.client = Client.builder()//
						.apiKey(k)//
						.build();
		}, true);
	}

	@Override
	public List<IAIBatch> updateBatches() {
//		BatchListParams bparams = BatchListParams.builder() //
//				.limit(100) //
//				.build();
//
//		BatchListPage res = client.batches().list(bparams);
//
//		return res.data().stream().map(b -> new GeminiAIBatch(b.id(), b)).collect(Collectors.toList());
		throw new NotImplementedError();
	}

	@Override
	public IAIBatch submitBatch(String json, Collection<String> reqIds) {
//		Path tempFile;
//		try {
//			tempFile = Files.createTempFile("mydata", ".jsonl");
//			Files.write(tempFile, json.getBytes(StandardCharsets.UTF_8));
//		} catch (IOException e) {
//			throw new IllegalStateException(e);
//		}
//
//		FileCreateParams fileParams = FileCreateParams.builder() //
//				.purpose(FilePurpose.BATCH) //
//				.file(tempFile).build();
//		FileObject file = client.files().create(fileParams);
//
//		Metadata meta = Metadata.builder() //
//				.putAdditionalProperty(AIBatchManager.KEY_REQIDS, JsonValue.from(//
//						reqIds.stream().collect(Collectors.joining(",")))) //
//				.build();
//
//		BatchCreateParams batchParams = BatchCreateParams.builder() //
//				.inputFileId(file.id()) //
//				.metadata(meta).endpoint(Endpoint.V1_RESPONSES) //
//				.completionWindow(CompletionWindow._24H) //
//				.build();
//		Batch returned = client.batches().create(batchParams);
//		return new GeminiAIBatch(returned.id(), returned);
		throw new NotImplementedError();
	}

	@Override
	public IAIBatch cancelBatch(IAIBatch entry) {
//		if (BatchState.Proccessing.equals(entry.getState())) {
//			Batch batch = client.batches().cancel(entry.getID());
//			return new GeminiAIBatch(batch.id(), batch);
//		}
//		return null;
		throw new NotImplementedError();
	}

	@Override
	public void loadBatch(IAIBatch entry) {
//		GeminiAIBatch oentry = ((GeminiAIBatch) entry);
//		Batch batch = client.batches().retrieve(oentry.getID());
//		oentry.setBatch(batch);
//		;
//
//		if (oentry.getError() == null && batch.errorFileId().isPresent())
//			oentry.setError(getFileAsString(batch.errorFileId().get()));
//
//		if (oentry.getResult() == null && batch.outputFileId().isPresent())
//			oentry.setResult(getFileAsString(batch.outputFileId().get()));
		throw new NotImplementedError();
	}

//	private String getFileAsString(String fileId) {
//		try (HttpResponse response = client.files().content(fileId)) {
//			ByteArrayOutputStream bos = new ByteArrayOutputStream();
//			response.body().transferTo(bos);
//			String result = new String(bos.toByteArray());
//			return result;
//		} catch (IOException e) {
//			throw new IllegalStateException(e);
//		}
//	}
//
//	private String requestToJson(ResponseCreateParams param) {
//		try {
//			return jsonMapper().writeValueAsString(new BatchElement(param._body()));
//		} catch (JsonProcessingException e) {
//			throw new IllegalStateException(e);
//		}
//	}

	@Override
	public String requestsToJson(Collection<IModelRequest> reqs) {
//		return requestsToJsonInner(
//				reqs.stream().map(r -> ((GeminiModelRequest) r).reqquest).collect(Collectors.toList()));
		throw new NotImplementedError();
	}

//	private String requestsToJsonInner(Collection<ResponseCreateParams> requests) {
//		return requests.stream().map((r) -> requestToJson(r)).collect(Collectors.joining("\n"));
//	}
}
