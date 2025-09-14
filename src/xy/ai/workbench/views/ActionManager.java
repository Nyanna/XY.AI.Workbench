package xy.ai.workbench.views;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.jobs.IJobFunction;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.jface.action.Action;
import org.eclipse.jface.action.IMenuManager;
import org.eclipse.jface.action.IToolBarManager;
import org.eclipse.jface.viewers.IStructuredSelection;
import org.eclipse.jface.viewers.StructuredViewer;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.PlatformUI;

public class ActionManager {
	private List<ActionDescription> actions = new ArrayList<>();

	public void add(ActionDescription actionDescription) {
		actions.add(actionDescription);
	}

	public ActionDescription create() {
		return new ActionDescription(this);
	}

	public void fillLocalPullDown(IMenuManager manager) {
		actions.stream().filter(a -> a.isPullDown()).forEach(a -> manager.add(a));
	}

	public void fillContextMenu(IMenuManager manager) {
		actions.stream().filter(a -> a.isContextMenu()).forEach(a -> manager.add(a));
	}

	public void fillLocalToolBar(IToolBarManager manager) {
		actions.stream().filter(a -> a.isToolbar()).forEach(a -> manager.add(a));
	}

	public static class ActionDescription extends Action {
		private boolean isPullDown = false;
		private boolean isContextMenu = false;
		private boolean isToolbar = false;

		private Runnable run;
		private Runnable display;
		private ActionManager manager;
		private IJobFunction job;

		public ActionDescription(ActionManager manager) {
			this.manager = manager;
		}

		public boolean isPullDown() {
			return isPullDown;
		}

		public boolean isContextMenu() {
			return isContextMenu;
		}

		public boolean isToolbar() {
			return isToolbar;
		}

		public void done() {
			manager.add(this);
		}

		public ActionDescription pullDown() {
			this.isPullDown = true;
			return this;
		}

		public ActionDescription contextMenu() {
			this.isContextMenu = true;
			return this;
		}

		public ActionDescription toolbar() {
			this.isToolbar = true;
			return this;
		}

		public ActionDescription text(String text, String tooltip) {
			setText(text);
			setToolTipText(tooltip);
			return this;
		}

		public ActionDescription image(String image) {
			setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(image));
			return this;
		}

		public ActionDescription runnable(Runnable run) {
			this.run = run;
			return this;
		}

		@SuppressWarnings("unchecked")
		public <C> ActionDescription selection(StructuredViewer viewer, Class<C> clazz, Consumer<C> run) {
			this.run = () -> {
				IStructuredSelection sel = (IStructuredSelection) viewer.getSelection();
				if (!sel.isEmpty()) {
					Object elem = sel.getFirstElement();
					if (clazz.isInstance(elem))
						run.accept((C) elem);
				}
			};
			return this;
		}

		public ActionDescription selection(StructuredViewer viewer, Consumer<IStructuredSelection> run) {
			this.run = () -> {
				IStructuredSelection sel = (IStructuredSelection) viewer.getSelection();
				if (!sel.isEmpty())
					run.accept(sel);
			};
			return this;
		}

		public ActionDescription job(Consumer<IProgressMonitor> run) {
			this.job = (mon) -> {
				run.accept(mon);
				return Status.OK_STATUS;
			};
			return this;
		}

		public ActionDescription jobFunc(IJobFunction job) {
			this.job = job;
			return this;
		}

		public ActionDescription display(Runnable run) {
			this.display = run;
			return this;
		}

		@Override
		public void run() {
			if (display != null)
				Display.getDefault().asyncExec(display);
			else if (job != null)
				Job.create(getText(), job).schedule();
			else
				run.run();
		}
	}
}
