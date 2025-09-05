package xy.ai.workbench.batch;

import java.util.ArrayList;
import java.util.List;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.jface.viewers.IStructuredContentProvider;
import org.eclipse.jface.viewers.TableViewer;

import xy.ai.workbench.connectors.IAIBatch;
import xy.ai.workbench.connectors.IAIBatchConnector;
import xy.ai.workbench.models.AIAnswer;

public class AIBatchResponseManager implements IStructuredContentProvider {
	private IAIBatchConnector connector;
	private List<AIAnswer> loadedAnswers = new ArrayList<>();
	private IAIBatch lastbatch;

	public AIBatchResponseManager(IAIBatchConnector connector) {
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
		if (obj.getResult() != null || obj.getError() != null || obj instanceof NewBatch) {
			connector.convertAnswers(obj);
			mon.worked(1);
		}
		
		loadedAnswers.addAll(obj.getAnswers());
	}

	public void updateView(TableViewer reqViewer) {
		reqViewer.setInput(this);
	}
}
