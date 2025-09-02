package xy.ai.workbench.batch;

import java.util.List;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.openai.client.OpenAIClient;
import com.openai.client.okhttp.OpenAIOkHttpClient;
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

	private String requestToBatch(ResponseCreateParams param) {
		try {
			return new ObjectMapper().writeValueAsString(param);
		} catch (JsonProcessingException e) {
			throw new IllegalStateException(e);
		}
	}
}
