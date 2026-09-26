package xy.ai.workbench.view.diff;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.util.function.Predicate;
import java.util.stream.Collectors;

import org.eclipse.egit.ui.internal.commit.DiffDocument;
import org.eclipse.egit.ui.internal.commit.DiffRegionFormatter;
import org.eclipse.egit.ui.internal.commit.DiffViewer;
import org.eclipse.jface.text.Document;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jgit.diff.DiffDriver;
import org.eclipse.jgit.diff.DiffEntry;
import org.eclipse.jgit.diff.DiffEntry.ChangeType;
import org.eclipse.jgit.diff.RawText;
import org.eclipse.jgit.lib.Repository;
import org.eclipse.jgit.patch.FileHeader;
import org.eclipse.jgit.revwalk.RevCommit;
import org.eclipse.jgit.revwalk.RevWalk;
import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.FillLayout;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Label;
import org.eclipse.ui.IActionBars;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.editors.text.EditorsUI;
import org.eclipse.ui.part.ViewPart;

import xy.ai.workbench.Activator;
import xy.ai.workbench.ActiveEditorListener;
import xy.ai.workbench.LOG;
import xy.ai.workbench.view.ActionManager;
import xy.ai.workbench.view.ActionManager.ActionDescription;

public class DiffPanel extends ViewPart {
	public static final String ID = "xy.ai.workbench.view.diff.DiffPanel";

	public static DiffPanel INSTANCE;

	private Label statusLabel;
	@SuppressWarnings("restriction")
	private DiffViewer diffViewer;

	private ActionManager act = new ActionManager();
	private ActionDescription syncAction;

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
		diffViewer = new DiffViewer(parent, null, SWT.V_SCROLL | SWT.H_SCROLL);
		diffViewer.getControl().setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));
		diffViewer.setDocument(new Document("Empty"));

		makeActions();
		IActionBars bars = getViewSite().getActionBars();
		act.fillLocalToolBar(bars.getToolBarManager());
		bars.updateActionBars();
	}

	private void makeActions() {
		syncAction = act.create().toolbar().text("Sync", "Automatically show the latest diff on editor change")
				.image(ISharedImages.IMG_ELCL_SYNCED).runnable(() -> {
					if (syncAction.isChecked())
						onEditorChange();
				});
		syncAction.done();
		syncAction.setChecked(false);
	}

	public void onEditorChange() {
		if (syncAction != null && !syncAction.isChecked())
			return;
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
		if (diffViewer == null || diffViewer.getControl().isDisposed() || !result.created())
			return;
		try {
			statusLabel.setText(result.chain() + " / " + result.ref());

			DiffDocument document = new DiffDocument();
			try (DiffRegionFormatter formatter = new Formatter(document); RevWalk walk = new RevWalk(repo)) {
				formatter.setRepository(repo);
				RevCommit oldC = walk.parseCommit(result.parent());
				RevCommit newC = walk.parseCommit(result.commit());
				formatter.format(oldC.getTree(), newC.getTree());
				formatter.flush();
				document.connect(formatter);
			}

			diffViewer.unconfigure();
			diffViewer.configure(new DiffViewer.Configuration(EditorsUI.getPreferenceStore()));
			diffViewer.setDocument(document);

		} catch (IOException e) {
			diffViewer.setDocument(new Document("Error loading diff: " + e.getMessage()));
		}
	}

	@Override
	public void setFocus() {
		diffViewer.getControl().setFocus();
	}

	@SuppressWarnings("restriction")
	private class Formatter extends DiffRegionFormatter {
		private Formatter(IDocument document) {
			super(document);
		}

		@Override
		public void format(FileHeader h, RawText a, RawText b, DiffDriver d) throws IOException {
			Predicate<? super String> pre = l -> !l.startsWith("--- ") && !l.startsWith("+++ ");
			String fil = new String(h.getBuffer(), StandardCharsets.UTF_8).lines().filter(pre)
					.collect(Collectors.joining("\n", "", "\n"));
			byte[] headerLines = fil.getBytes(StandardCharsets.UTF_8);
			super.format(new FileHeader(headerLines, h.toEditList(), h.getPatchType()), a, b, d);
		}

		@Override
		protected void writeHunkHeader(int aStartLine, int aEndLine, int bStartLine, int bEndLine, String funcName) {
		}

		@Override
		protected void formatGitDiffFirstHeaderLine(ByteArrayOutputStream o, ChangeType type, String oldPath,
				String newPath) throws IOException {
			o.write((String.format("%-20s", oldPath) + "\n\n").getBytes());
		}

		@Override
		protected void formatIndexLine(OutputStream o, DiffEntry ent) throws IOException {
		}
	}
}
