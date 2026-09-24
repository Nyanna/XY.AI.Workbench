package xy.ai.workbench.view.diff;

import java.io.File;
import java.io.IOException;
import java.nio.file.Path;

import org.eclipse.egit.ui.internal.commit.DiffDocument;
import org.eclipse.egit.ui.internal.commit.DiffRegionFormatter;
import org.eclipse.egit.ui.internal.commit.DiffViewer;
import org.eclipse.jface.text.Document;
import org.eclipse.jgit.lib.Repository;
import org.eclipse.jgit.revwalk.RevCommit;
import org.eclipse.jgit.revwalk.RevWalk;
import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.FillLayout;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Label;
import org.eclipse.ui.editors.text.EditorsUI;
import org.eclipse.ui.part.ViewPart;

import xy.ai.workbench.Activator;
import xy.ai.workbench.ActiveEditorListener;
import xy.ai.workbench.LOG;

public class DiffPanel extends ViewPart {

	public static final String ID = "xy.ai.workbench.view.diff.DiffPanel";

	public static DiffPanel INSTANCE;

	private Label statusLabel;
	@SuppressWarnings("restriction")
	private DiffViewer diffText;

	@SuppressWarnings("restriction")
	@Override
	public void createPartControl(Composite parent) {
		INSTANCE = this;
		Activator.getDefault().editorListener.addTextEditorObserver(e -> onEditorChange());
		parent.setLayout(new GridLayout(1, false));

		statusLabel = new Label(parent, SWT.NONE);
		statusLabel.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
		statusLabel.setText("Empty");

		Composite diffSection = new Composite(parent, SWT.NONE);
		diffSection.setLayout(new FillLayout());
		diffText = new DiffViewer(parent, null, SWT.V_SCROLL | SWT.H_SCROLL);
		diffText.getControl().setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
		diffText.setDocument(new Document("Empty"));
	}

	public void onEditorChange() {
		ActiveEditorListener editLst = Activator.getDefault().editorListener;
		Path path = editLst.resolveProjectPath();
		if (path == null)
			return;
		File projectPath = path.toFile();
		try {
			OpSnapshotter snap = new OpSnapshotter(projectPath);
			SnapshotResult result = snap.findLatest();
			if (result != null)
				onSnapshot(result, snap.getRepository());
		} catch (IOException e) {
			LOG.error(e.getMessage(), e);
		}
	}

	@SuppressWarnings("restriction")
	public void onSnapshot(SnapshotResult result, Repository repo) {
		if (diffText == null || diffText.getControl().isDisposed() || !result.created())
			return;
		try {
			statusLabel.setText(result.chain() + " / " + result.ref());

			DiffDocument document = new DiffDocument();
			try (DiffRegionFormatter formatter = new DiffRegionFormatter(document); RevWalk walk = new RevWalk(repo)) {
				formatter.setRepository(repo);
				RevCommit oldC = walk.parseCommit(result.parent());
				RevCommit newC = walk.parseCommit(result.commit());
				formatter.format(oldC.getTree(), newC.getTree());
				formatter.flush();
				document.connect(formatter);
			}

			diffText.configure(new DiffViewer.Configuration(EditorsUI.getPreferenceStore()));
			diffText.setDocument(document);

		} catch (IOException e) {
			diffText.setDocument(new Document("Error loading diff: " + e.getMessage()));
		}
	}

	@Override
	public void setFocus() {
		diffText.getControl().setFocus();
	}
}
