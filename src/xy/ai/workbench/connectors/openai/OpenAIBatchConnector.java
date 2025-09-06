package xy.ai.workbench.connectors.openai;

import static com.openai.core.ObjectMappers.jsonMapper;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.Random;
import java.util.stream.Collectors;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonMappingException;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;
import com.openai.client.OpenAIClient;
import com.openai.client.okhttp.OpenAIOkHttpClient;
import com.openai.core.JsonValue;
import com.openai.core.ObjectMappers;
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
import com.openai.models.responses.Response;
import com.openai.models.responses.ResponseCreateParams;
import com.openai.models.responses.ResponseCreateParams.Body;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.batch.BatchState;
import xy.ai.workbench.batch.NewBatch;
import xy.ai.workbench.connectors.IAIBatchConnector;
import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;

public class OpenAIBatchConnector implements IAIBatchConnector {
	private OpenAIClient client;
	private OpenAIConnector connector;

	public OpenAIBatchConnector(ConfigManager cfg, OpenAIConnector connector) {
		this.connector = connector;
		cfg.addKeyObs(k -> {
			if (KeyPattern.OpenAI.matches(k))
				this.client = OpenAIOkHttpClient.builder().apiKey(k).build();
		}, true);
	}

	@Override
	public List<IAIBatch> updateBatches(IProgressMonitor mon) {
		BatchListParams bparams = BatchListParams.builder() //
				.limit(100) //
				.build();

		BatchListPage res = client.batches().list(bparams);

		return res.data().stream().map(b -> new OpenAIBatch(b.id(), b)).collect(Collectors.toList());
	}

	@Override
	public IAIBatch submitBatch(NewBatch entry, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Submit batch", 3);
		sub.subTask("Convert to JSON");
		String json = requestsToJson(entry.getRequests());
		String reqIds = String.join(",", entry.getRequestIDs());
		sub.worked(1);

		sub.subTask("Write temp file");
		Path tempFile;
		try {
			tempFile = Files.createTempFile("mydata", ".jsonl");
			Files.write(tempFile, json.getBytes(StandardCharsets.UTF_8));
		} catch (IOException e) {
			throw new IllegalStateException(e);
		}
		sub.worked(1);

		sub.subTask("Execute request");
		FileCreateParams fileParams = FileCreateParams.builder() //
				.purpose(FilePurpose.BATCH) //
				.file(tempFile).build();
		FileObject file = client.files().create(fileParams);

		Metadata meta = Metadata.builder() //
				.putAdditionalProperty(AIBatchManager.KEY_REQIDS, JsonValue.from(//
						reqIds)) //
				.build();

		BatchCreateParams batchParams = BatchCreateParams.builder() //
				.inputFileId(file.id()) //
				.metadata(meta).endpoint(Endpoint.V1_RESPONSES) //
				.completionWindow(CompletionWindow._24H) //
				.build();
		Batch returned = client.batches().create(batchParams);
		sub.worked(1);
		sub.done();
		return new OpenAIBatch(returned.id(), returned);
	}

	@Override
	public IAIBatch cancelBatch(IAIBatch entry, IProgressMonitor mon) {
		if (BatchState.Proccessing.equals(entry.getState())) {
			Batch batch = client.batches().cancel(entry.getID());
			return new OpenAIBatch(batch.id(), batch);
		}
		return null;
	}

	@Override
	public void loadBatch(IAIBatch entry, IProgressMonitor mon) {
		OpenAIBatch oentry = ((OpenAIBatch) entry);
		SubMonitor sub = SubMonitor.convert(mon, "Load batch", 3);
		sub.subTask("Retrieve Batch");
		Batch batch = client.batches().retrieve(oentry.getID());
		oentry.setBatch(batch);
		sub.worked(1);

		sub.subTask("Retrieve Errors");
		if (oentry.getError() == null && batch.errorFileId().isPresent()) {
			oentry.setError(getFileAsString(batch.errorFileId().get()));
		}
		sub.worked(1);

		sub.subTask("Retrieve Output");
		if (oentry.getResult() == null && batch.outputFileId().isPresent()) {
			oentry.setResult(getFileAsString(batch.outputFileId().get()));
		}
		sub.worked(1);
		sub.done();
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
		return requestsToJsonInner(reqs.stream().map(r -> ((OpenAIRequest) r).reqquest).collect(Collectors.toList()));
	}

	private String requestsToJsonInner(Collection<ResponseCreateParams> requests) {
		return requests.stream().map((r) -> requestToJson(r)).collect(Collectors.joining("\n"));
	}

	@Override
	public void convertAnswers(IAIBatch obj, IProgressMonitor mon) {
		List<AIAnswer> answ = new ArrayList<>();
		if (obj.getResult() != null) {
			String[] split = obj.getResult().split("\n");
			SubMonitor sub = SubMonitor.convert(mon, "Convert Answers", split.length);
			for (String InputJson : split) {
				answ.add(convertToAnswer(InputJson, mon));
				sub.worked(1);
			}
			sub.done();
		}

		if (obj.getError() != null)
			answ.add(convertToAnswer(obj.getError(), mon));

		obj.setAnswers(answ);
	}

	private AIAnswer convertToAnswer(String bodyJson, IProgressMonitor mon) {

		try {
			ObjectNode tree = (ObjectNode) ObjectMappers.jsonMapper().readTree(bodyJson);
			tree.get("error"); // errors
			TextNode id = (TextNode) tree.get("id");
			ObjectNode response = (ObjectNode) tree.get("response");
			// IntNode statusCode = (IntNode) response.get("status_code");
			ObjectNode body = (ObjectNode) response.get("body");

			String cbodyJson = ObjectMappers.jsonMapper().writeValueAsString(body);
			AIAnswer answer = convertToAnswer1(cbodyJson, mon);
			answer.id = id.asText();
			return answer;

		} catch (JsonProcessingException e) {
			throw new IllegalStateException(e);
		}
	}

	private AIAnswer convertToAnswer1(String bodyJson, IProgressMonitor mon)
			throws JsonMappingException, JsonProcessingException {
		Response resp = ObjectMappers.jsonMapper().readerFor(Response.class).readValue(bodyJson);

		AIAnswer answer = connector.convertResponse(new OpenAIResponse(resp), mon);
		return answer;
	}
}
