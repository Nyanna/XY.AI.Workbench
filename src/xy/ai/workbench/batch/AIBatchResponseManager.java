package xy.ai.workbench.batch;

import java.util.ArrayList;
import java.util.List;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.jface.viewers.IStructuredContentProvider;
import org.eclipse.jface.viewers.TableViewer;

import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.models.AIAnswer;

public class AIBatchResponseManager implements IStructuredContentProvider {
	private IAIConnector connector;
	private List<AIAnswer> loadedAnswers = new ArrayList<>();
	private IAIBatch lastbatch;

	public AIBatchResponseManager(IAIConnector connector) {
		this.connector = connector;
	}

	@Override
	public Object[] getElements(Object inputElement) {
		return loadedAnswers.toArray();
	}

	public void load(IAIBatch obj, IProgressMonitor mon) {
		if (obj.equals(lastbatch))
			return;
		lastbatch = obj;

		loadedAnswers.clear();
		if (obj.getResult() != null)
			for (String InputJson : obj.getResult().split("\n")) {
				loadedAnswers.add(connector.convertToAnswer(InputJson));
				mon.worked(1);
			}

		if (obj.getError() != null) {
			loadedAnswers.add(connector.convertToAnswer(obj.getError()));
			mon.worked(1);
		}
	}

	public void updateView(TableViewer reqViewer) {
		reqViewer.setInput(this);
	}
}
