package xy.ai.workbench.batch;

import java.util.ArrayList;
import java.util.List;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.jface.viewers.IStructuredContentProvider;
import org.eclipse.jface.viewers.TableViewer;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.node.IntNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;
import com.openai.core.ObjectMappers;
import com.openai.models.responses.Response;

import xy.ai.workbench.AIAnswer;
import xy.ai.workbench.OpenAIConnector;

public class AIBatchRequestManager implements IStructuredContentProvider {
	private OpenAIConnector connector = new OpenAIConnector(null);
	private List<AIAnswer> loadedAnswers = new ArrayList<>();
	private BatchEntry lastbatch;

	@Override
	public Object[] getElements(Object inputElement) {
		return loadedAnswers.toArray();
	}

	public void load(BatchEntry obj, IProgressMonitor mon) {
		if (obj.equals(lastbatch))
			return;
		lastbatch = obj;

		loadedAnswers.clear();
		if (obj.result != null)
			for (String InputJson : obj.result.split("\n")) {
				loadedAnswers.add(convertJson(InputJson));
				mon.worked(1);
			}
	}

	private AIAnswer convertJson(String inputJson) {
		Response resp;
		try {
			ObjectNode tree = (ObjectNode) ObjectMappers.jsonMapper().readTree(inputJson);
			tree.get("error"); // errors
			// TODO add id for local and remote
			TextNode id = (TextNode) tree.get("id");
			ObjectNode response = (ObjectNode) tree.get("response");
			IntNode statusCode = (IntNode) response.get("status_code");
			ObjectNode body = (ObjectNode) response.get("body");

			String bodyJson = ObjectMappers.jsonMapper().writeValueAsString(body);
			resp = ObjectMappers.jsonMapper().readerFor(Response.class).readValue(bodyJson);

			AIAnswer answer = connector.convertResponse(resp);
			answer.id = id.asText();
			return answer;

		} catch (JsonProcessingException e) {
			throw new IllegalStateException(e);
		}
	}

	public void updateView(TableViewer reqViewer) {
		reqViewer.setInput(this);
	}
}
