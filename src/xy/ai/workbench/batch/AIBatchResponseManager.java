package xy.ai.workbench.batch;

import java.util.ArrayList;
import java.util.List;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.SubMonitor;
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

		SubMonitor sub = SubMonitor.convert(mon, "Converting answers", 1);

		loadedAnswers.clear();
		if (obj.getResult() != null || obj.getError() != null || obj instanceof NewBatch) {
			connector.convertAnswers(obj, sub);
			mon.worked(1);
		}

		sub.done();
		loadedAnswers.addAll(obj.getAnswers());
	}

	public void updateView(TableViewer reqViewer) {
		reqViewer.setInput(this);
	}

	public void clearAnswers() {
		loadedAnswers.clear();
	}
}
