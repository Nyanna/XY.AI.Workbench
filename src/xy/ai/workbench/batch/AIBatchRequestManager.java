package xy.ai.workbench.batch;

import java.util.ArrayList;
import java.util.List;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.jface.viewers.IStructuredContentProvider;
import org.eclipse.jface.viewers.TableViewer;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;
import com.openai.core.ObjectMappers;

import xy.ai.workbench.connectors.openai.IBatchEntry;
import xy.ai.workbench.connectors.openai.OpenAIConnector;
import xy.ai.workbench.models.AIAnswer;

public class AIBatchRequestManager implements IStructuredContentProvider {
	private OpenAIConnector connector = new OpenAIConnector(null);
	private List<AIAnswer> loadedAnswers = new ArrayList<>();
	private IBatchEntry lastbatch;

	@Override
	public Object[] getElements(Object inputElement) {
		return loadedAnswers.toArray();
	}

	public void load(IBatchEntry obj, IProgressMonitor mon) {
		if (obj.equals(lastbatch))
			return;
		lastbatch = obj;

		loadedAnswers.clear();
		if (obj.getResult() != null)
			for (String InputJson : obj.getResult().split("\n")) {
				loadedAnswers.add(convertJson(InputJson));
				mon.worked(1);
			}

		if (obj.getError() != null) {
			loadedAnswers.add(convertJson(obj.getError()));
			mon.worked(1);
		}
	}

	private AIAnswer convertJson(String inputJson) {
		try {
			ObjectNode tree = (ObjectNode) ObjectMappers.jsonMapper().readTree(inputJson);
			tree.get("error"); // errors
			// TODO add id for local and remote
			TextNode id = (TextNode) tree.get("id");
			ObjectNode response = (ObjectNode) tree.get("response");
			// IntNode statusCode = (IntNode) response.get("status_code");
			ObjectNode body = (ObjectNode) response.get("body");

			String bodyJson = ObjectMappers.jsonMapper().writeValueAsString(body);
			AIAnswer answer = connector.convertToAnswer(bodyJson);
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
