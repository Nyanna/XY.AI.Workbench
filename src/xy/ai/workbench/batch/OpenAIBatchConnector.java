package xy.ai.workbench.batch;

import static com.openai.core.ObjectMappers.jsonMapper;

import java.util.List;
import java.util.Map;
import java.util.Random;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.openai.client.OpenAIClient;
import com.openai.client.okhttp.OpenAIOkHttpClient;
import com.openai.core.JsonValue;
import com.openai.models.batches.Batch;
import com.openai.models.batches.BatchListPage;
import com.openai.models.batches.BatchListParams;
import com.openai.models.responses.ResponseCreateParams;

import xy.ai.workbench.Activator;

public class OpenAIBatchConnector {
	private OpenAIClient client;

	public List<Batch> updateBatches() {
		if (this.client == null) {
			var key = Activator.getDefault().session.getKey();
			this.client = OpenAIOkHttpClient.builder().apiKey(key).build();
		}

		BatchListParams bparams = BatchListParams.builder() //
				.limit(100) //
				.build();

		BatchListPage res = client.batches().list(bparams);
		return res.data();
	}

	public String requestToBatch(ResponseCreateParams param) {
		ResponseCreateParams forBatch = param.toBuilder()
				.additionalBodyProperties(Map.of("custom_id", JsonValue.from(new Random().nextInt())))//
				.build();
		try {
			return jsonMapper().writeValueAsString(forBatch._body());
		} catch (JsonProcessingException e) {
			throw new IllegalStateException(e);
		}
	}
}
