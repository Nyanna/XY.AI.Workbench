package xy.ai.workbench.tools;

import org.eclipse.search.ui.IQueryListener;
import org.eclipse.search.ui.ISearchQuery;

public abstract class AbstractQueryListener implements IQueryListener {

	@Override
	public void queryAdded(ISearchQuery query) {
	}

	@Override
	public void queryRemoved(ISearchQuery query) {
	}

	@Override
	public void queryStarting(ISearchQuery query) {
	}

	@Override
	public void queryFinished(ISearchQuery query) {
	}
}
