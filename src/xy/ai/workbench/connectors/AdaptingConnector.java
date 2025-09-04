package xy.ai.workbench.connectors;

import java.util.Collection;
import java.util.List;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonMappingException;

import xy.ai.workbench.SessionConfig;
import xy.ai.workbench.connectors.openai.OpenAIBatchConnector;
import xy.ai.workbench.connectors.openai.OpenAIConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class AdaptingConnector implements IAIConnector, IAIBatchConnector {

	private SessionConfig cfg;
	private IAIConnector connector;
	private IAIBatchConnector batchConnector;

	public AdaptingConnector(SessionConfig cfg) {
		this.cfg = cfg;
		connector = new OpenAIConnector(cfg);
		batchConnector = new OpenAIBatchConnector();
	}

	public IModelRequest createRequest(String input, String systemPrompt, List<String> tools) {
		// TODO based on selected model and key
		return getConnector().createRequest(input, systemPrompt, tools);
	}

	public List<IAIBatch> updateBatches() {
		// TODO based on selected model and key
		return getBatchConnector().updateBatches();
	}

	public IAIBatch submitBatch(String json, Collection<String> reqIds) {
		// TODO based on selected model and key
		return getBatchConnector().submitBatch(json, reqIds);
	}

	public IModelResponse executeRequest(IModelRequest request) {
		// TODO based on request type
		return getConnector().executeRequest(request);
	}

	public String requestsToJson(Collection<IModelRequest> reqs) {
		// TODO based on request type
		return getBatchConnector().requestsToJson(reqs);
	}

	public AIAnswer convertResponse(IModelResponse response) {
		// TODO based on response type
		return getConnector().convertResponse(response);
	}

	public AIAnswer convertToAnswer(String bodyJson) throws JsonProcessingException, JsonMappingException {
		// TODO based on structure detector
		return getConnector().convertToAnswer(bodyJson);
	}

	public IAIBatch cancelBatch(IAIBatch entry) {
		// TODO based on batch type
		return getBatchConnector().cancelBatch(entry);
	}

	public void loadBatch(IAIBatch entry) {
		// TODO based on batch type
		getBatchConnector().loadBatch(entry);
	}

	public IAIConnector getConnector() {
		return connector;
	}

	public IAIBatchConnector getBatchConnector() {
		return batchConnector;
	}

}
