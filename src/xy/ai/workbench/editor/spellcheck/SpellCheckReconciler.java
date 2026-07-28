package xy.ai.workbench.editor.spellcheck;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.ScheduledFuture;
import java.util.concurrent.TimeUnit;

import org.eclipse.jface.text.DocumentEvent;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IDocumentListener;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextInputListener;
import org.eclipse.jface.text.ITextViewer;
import org.eclipse.jface.text.Region;
import org.eclipse.jface.text.reconciler.IReconciler;
import org.eclipse.jface.text.reconciler.IReconcilingStrategy;

import xy.ai.workbench.editor.AITextEditor;
import xy.ai.workbench.editor.mdast.TextRegion;
import xy.ai.workbench.editor.mdast.nodes.Node;

/**
 * Reconciler that tracks only the document region actually changed by the user.
 * <p>
 * On every {@link DocumentEvent} the affected region is merged into a pending
 * dirty region. After {@code delay} ms of inactivity the dirty region is handed
 * to {@link SpellingStrategy#reconcile(IRegion)}, which then expands it to full
 * line boundaries before calling LanguageTool.
 */
public class SpellCheckReconciler implements IReconciler {

	private final SpellingStrategy fStrategy;
	private final int fDelayMs;
	private final AITextEditor fEditor;

	private ITextViewer fViewer;
	private IDocument fDocument;

	private final List<int[]> fDirtyRegions = new ArrayList<>();
	private final List<int[]> fClearRegions = new ArrayList<>();

	private final ScheduledExecutorService fScheduler = Executors.newSingleThreadScheduledExecutor(r -> {
		Thread t = new Thread(r, "SpellCheck-Reconciler");
		t.setDaemon(true);
		return t;
	});

	private ScheduledFuture<?> fPending;

	// ── Listeners ──────────────────────────────────────────────────────────────

	private final IDocumentListener fDocumentListener = new IDocumentListener() {
		@Override
		public void documentAboutToBeChanged(DocumentEvent event) {
		}

		@Override
		public void documentChanged(DocumentEvent event) {
			TextRegion astRegion = fEditor != null ? fEditor.getLastAstChangeRegion() : null;
			if (astRegion != null) {
				if (!mergeEnabledLeaves(astRegion.n()))
					return;
			} else {
				int start = event.getOffset();
				int end = start + Math.max(event.getLength(), event.getText() != null ? event.getText().length() : 0);
				mergeDirty(start, Math.max(end, start + 1));
			}
			scheduleReconcile();
		}
	};

	private final ITextInputListener fTextInputListener = new ITextInputListener() {
		@Override
		public void inputDocumentAboutToBeChanged(IDocument oldInput, IDocument newInput) {
			if (oldInput != null) {
				oldInput.removeDocumentListener(fDocumentListener);
			}
		}

		@Override
		public void inputDocumentChanged(IDocument oldInput, IDocument newInput) {
			fDocument = newInput;
			fStrategy.setDocument(newInput);
			if (newInput != null) {
				newInput.addDocumentListener(fDocumentListener);
				// Trigger a full-document check on the initial load.
				mergeDirty(0, newInput.getLength());
				scheduleReconcile();
			}
		}
	};

	// ── Constructor ────────────────────────────────────────────────────────────

	public SpellCheckReconciler(SpellingStrategy strategy, int delayMs, AITextEditor editor) {
		fStrategy = strategy;
		fDelayMs = delayMs;
		fEditor = editor;
	}

	// ── IReconciler ────────────────────────────────────────────────────────────

	@Override
	public void install(ITextViewer textViewer) {
		fViewer = textViewer;
		textViewer.addTextInputListener(fTextInputListener);

		// Handle a document that is already set on the viewer.
		IDocument doc = textViewer.getDocument();
		if (doc != null) {
			fTextInputListener.inputDocumentChanged(null, doc);
		}
	}

	@Override
	public void uninstall() {
		cancelPending();
		fScheduler.shutdownNow();
		if (fDocument != null) {
			fDocument.removeDocumentListener(fDocumentListener);
		}
		if (fViewer != null) {
			fViewer.removeTextInputListener(fTextInputListener);
		}
	}

	@Override
	public IReconcilingStrategy getReconcilingStrategy(String contentType) {
		return IDocument.DEFAULT_CONTENT_TYPE.equals(contentType) ? fStrategy : null;
	}

	private boolean mergeEnabledLeaves(Node node) {
		if (node == null)
			return false;
		if (node.children.isEmpty()) {
			int start = node.getOffset();
			int end = node.getEndOffset();
			if (!node.enableSpellcheck) {
				mergeClear(start, Math.max(end, start + 1));
				return true;
			}
			mergeDirty(start, Math.max(end, start + 1));
			return true;
		}
		boolean merged = false;
		for (Node child : node.children)
			merged |= mergeEnabledLeaves(child);
		return merged;
	}

	private synchronized void mergeDirty(int start, int end) {
		merge(fDirtyRegions, start, end);
	}

	private synchronized void mergeClear(int start, int end) {
		merge(fClearRegions, start, end);
	}

	private static void merge(List<int[]> regions, int start, int end) {
		int newStart = start;
		int newEnd = end;
		for (Iterator<int[]> it = regions.iterator(); it.hasNext();) {
			int[] r = it.next();
			// Overlapping or directly adjacent -> merge.
			if (newStart <= r[1] && r[0] <= newEnd) {
				newStart = Math.min(newStart, r[0]);
				newEnd = Math.max(newEnd, r[1]);
				it.remove();
			}
		}
		regions.add(new int[] { newStart, newEnd });
	}

	private synchronized List<IRegion> takeDirty() {
		return take(fDirtyRegions);
	}

	private synchronized List<IRegion> takeClear() {
		return take(fClearRegions);
	}

	private static List<IRegion> take(List<int[]> regions) {
		if (regions.isEmpty())
			return null;
		List<IRegion> result = new ArrayList<>(regions.size());
		for (int[] r : regions)
			result.add(new Region(r[0], r[1] - r[0]));
		regions.clear();
		return result;
	}

	private synchronized void cancelPending() {
		if (fPending != null) {
			fPending.cancel(false);
			fPending = null;
		}
	}

	private void scheduleReconcile() {
		cancelPending();
		fPending = fScheduler.schedule(() -> {
			List<IRegion> clear = takeClear();
			if (clear != null)
				for (IRegion region : clear)
					fStrategy.clear(region);

			List<IRegion> dirty = takeDirty();
			if (dirty != null)
				for (IRegion region : dirty)
					fStrategy.reconcile(region);
		}, fDelayMs, TimeUnit.MILLISECONDS);
	}
}
