package xy.ai.workbench.editor.update;

import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import org.eclipse.jface.text.DocumentEvent;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IDocumentListener;
import org.eclipse.jface.text.ITextInputListener;
import org.eclipse.jface.text.ITextViewer;
import org.eclipse.jface.text.ITextViewerExtension2;
import org.eclipse.swt.widgets.Display;

import xy.ai.workbench.editor.DocumentBuffer;
import xy.ai.workbench.editor.mdast.MarkdownDocument;
import xy.ai.workbench.editor.mdast.TextRegion;

public final class EditorManager {

	public static final int DEBOUNCE_DELAY_MS = 200;

	private final List<Listener> listeners = new CopyOnWriteArrayList<>();

	private final ExecutorService background = Executors.newSingleThreadExecutor(r -> {
		Thread t = new Thread(r, "EditorManager-Background");
		t.setDaemon(true);
		return t;
	});

	private ITextViewer viewer;
	private Display display;
	private IDocument document;
	private DocumentBuffer astBuffer;
	private MarkdownDocument ast;

	// ── pending, composed (not yet reparsed) edit ────────────────────────────────
	private boolean pendingActive;
	private int pendingStart;
	private int pendingOldLen;
	private int pendingNewLen;

	private final Runnable flush = new Flush();
	private final IDocumentListener documentListener = new DocumentListener();
	private final ITextInputListener textInputListener = new TextInputListener();

	public void install(ITextViewer viewer) {
		this.viewer = viewer;
		if (viewer.getTextWidget() != null)
			display = viewer.getTextWidget().getDisplay();
		viewer.addTextInputListener(textInputListener);
		IDocument doc = viewer.getDocument();
		if (doc != null)
			textInputListener.inputDocumentChanged(null, doc);
	}

	public void uninstall() {
		cancelPending();
		background.shutdownNow();
		if (document != null)
			document.removeDocumentListener(documentListener);
		if (viewer != null)
			viewer.removeTextInputListener(textInputListener);
		listeners.clear();
	}

	public void addListener(Listener listener) {
		listeners.add(listener);
	}

	public boolean removeListener(Listener listener) {
		return listeners.remove(listener);
	}

	public MarkdownDocument getAst() {
		return ast;
	}

	public IDocument getDocument() {
		return document;
	}

	public void runAsync(Runnable task) {
		if (!background.isShutdown())
			background.execute(task);
	}

	/**
	 * Losslessly folds a new raw edit (given in current/live document coordinates)
	 * into the still-pending, not yet reparsed edit, so that a whole burst of edits
	 * can be represented - and later applied to the AST - as if it was a single
	 * edit.
	 */
	private void composeEdit(int offset, int removedLen, int insertedLen) {
		if (!pendingActive) {
			pendingActive = true;
			pendingStart = offset;
			pendingOldLen = removedLen;
			pendingNewLen = insertedLen;
			return;
		}

		if (offset < pendingStart) {
			int leftExtra = pendingStart - offset;
			pendingStart = offset;
			pendingOldLen += leftExtra;
			pendingNewLen += leftExtra;
		}

		int pendingNewEnd = pendingStart + pendingNewLen;
		int editEnd = offset + removedLen;
		if (editEnd > pendingNewEnd) {
			int rightExtra = editEnd - pendingNewEnd;
			pendingOldLen += rightExtra;
			pendingNewLen += rightExtra;
		}

		pendingNewLen += insertedLen - removedLen;
	}

	private void scheduleFlush() {
		if (display == null || display.isDisposed())
			return;
		display.timerExec(DEBOUNCE_DELAY_MS, flush);
	}

	private void cancelPending() {
		pendingActive = false;
		if (display != null && !display.isDisposed())
			display.timerExec(-1, flush);
	}

	private class Flush implements Runnable {
		@Override
		public void run() {
			if (!pendingActive || ast == null || astBuffer == null)
				return;
			int offset = pendingStart;
			int removed = pendingOldLen;
			int inserted = pendingNewLen;
			pendingActive = false;

			TextRegion region = ast.update(offset, removed, inserted);
			pushAstUpdated(region);
		}
	}

	/**
	 * Directly drives syntax highlighting from the reparse result, then notifies
	 * listeners.
	 */
	private void pushAstUpdated(TextRegion region) {
		if (viewer instanceof ITextViewerExtension2 ext2)
			try {
				ext2.invalidateTextPresentation(region.offset(), Math.max(1, region.length()));
			} catch (IllegalArgumentException e) {
				// region outside the (possibly just replaced) document - ignore.
			}
		for (Listener l : listeners)
			l.onAstUpdated(region);
	}

	private final class TextInputListener implements ITextInputListener {
		@Override
		public void inputDocumentAboutToBeChanged(IDocument oldInput, IDocument newInput) {
			cancelPending();
			if (oldInput != null)
				oldInput.removeDocumentListener(documentListener);
		}

		@Override
		public void inputDocumentChanged(IDocument oldInput, IDocument newInput) {
			document = newInput;
			TextRegion initial = null;
			if (newInput != null) {
				astBuffer = new DocumentBuffer(newInput);
				ast = new MarkdownDocument(astBuffer);
				initial = ast.update(0, 0, astBuffer.length());
				newInput.addDocumentListener(documentListener);
			} else {
				astBuffer = null;
				ast = null;
			}
			for (Listener l : listeners)
				l.onDocumentChanged(oldInput, newInput);
			if (initial != null)
				pushAstUpdated(initial);
		}
	}

	private final class DocumentListener implements IDocumentListener {
		private int removedLen;

		@Override
		public void documentAboutToBeChanged(DocumentEvent event) {
			removedLen = event.getLength();
		}

		@Override
		public void documentChanged(DocumentEvent event) {
			String text = event.getText();
			int insertedLen = text == null ? 0 : text.length();
			composeEdit(event.getOffset(), removedLen, insertedLen);
			scheduleFlush();
		}
	}

	public interface Listener {

		default void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {
		}

		/**
		 * Fired once, on the UI thread, after the debounced AST reparse for a batch of
		 * edits has completed, with the resulting changed region. Listeners that need
		 * to perform expensive/blocking work (e.g. spell checking) should hand it off
		 * via {@link EditorManager#runAsync(Runnable)} instead of blocking this call.
		 */
		default void onAstUpdated(TextRegion region) {
		}
	}
}
