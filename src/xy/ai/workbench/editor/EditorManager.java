package xy.ai.workbench.editor;

import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import org.eclipse.jface.text.DocumentEvent;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IDocumentListener;
import org.eclipse.jface.text.ITextInputListener;
import org.eclipse.jface.text.ITextViewer;
import org.eclipse.swt.widgets.Display;

import xy.ai.workbench.editor.mdast.MarkdownDocument;
import xy.ai.workbench.editor.mdast.nodes.Node;

public final class EditorManager {

	public static final int DEBOUNCE_DELAY_MS = 100;

	private final List<IManagerListener> listeners = new CopyOnWriteArrayList<>();

	private final ExecutorService background = Executors.newSingleThreadExecutor(r -> {
		Thread t = new Thread(r, "EditorManager-Background");
		t.setDaemon(true);
		return t;
	});

	private ITextViewer viewer;
	private Display display;
	private IDocument doc;
	private DocumentBuffer buffer;
	private MarkdownDocument ast;
	private ISpellChecker spell;

	// ── pending, composed (not yet reparsed) edit ────────────────────────────────
	private boolean pendingActive;
	private int pendingStart;
	private int pendingOldLen;
	private int pendingNewLen;

	private final Runnable flush = new Flush();
	private final IDocumentListener documentLst = new Document();
	private final ITextInputListener textInputListener = new TextInput();

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
		if (doc != null)
			doc.removeDocumentListener(documentLst);
		if (viewer != null)
			viewer.removeTextInputListener(textInputListener);
		listeners.clear();
		spell = null;
	}

	public void addListener(IManagerListener listener) {
		listeners.add(listener);
	}

	public boolean removeListener(IManagerListener listener) {
		return listeners.remove(listener);
	}

	public MarkdownDocument getAst() {
		return ast;
	}

	public IDocument getDocument() {
		return doc;
	}

	public void setSpellChecker(ISpellChecker spellChecker) {
		this.spell = spellChecker;
		if (spellChecker != null && doc != null && ast != null) {
			spellChecker.onDocumentChanged(doc);
			runSpellCheck(ast.getRoot());
		}
	}

	private void runSpellCheck(Node node) {
		if (spell != null && !background.isShutdown())
			background.execute(() -> spell.reconcile(node));
	}

	private void changed(int offset, int removedLen, int insertedLen) {
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
		scheduleFlush();
	}

	private void update(Node node) {
		runSpellCheck(node);
		for (IManagerListener l : listeners)
			l.onAstUpdated(node);
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
			if (!pendingActive || ast == null || buffer == null)
				return;
			int offset = pendingStart;
			int removed = pendingOldLen;
			int inserted = pendingNewLen;
			pendingActive = false;

			update(ast.update(offset, removed, inserted));
		}
	}

	private class Document implements IDocumentListener {
		private int removedLen;

		@Override
		public void documentAboutToBeChanged(DocumentEvent event) {
			removedLen = event.getLength();
		}

		@Override
		public void documentChanged(DocumentEvent event) {
			String text = event.getText();
			changed(event.getOffset(), removedLen, text == null ? 0 : text.length());
		}
	}

	private class TextInput implements ITextInputListener {
		@Override
		public void inputDocumentAboutToBeChanged(IDocument oldInput, IDocument newInput) {
			cancelPending();
			if (oldInput != null)
				oldInput.removeDocumentListener(documentLst);
		}

		@Override
		public void inputDocumentChanged(IDocument oldInput, IDocument newInput) {
			doc = newInput;
			Node initial = null;
			if (newInput != null) {
				buffer = new DocumentBuffer(newInput);
				ast = new MarkdownDocument(buffer);
				initial = ast.update(0, 0, buffer.length());
				newInput.addDocumentListener(documentLst);
			} else {
				buffer = null;
				ast = null;
			}
			if (spell != null)
				spell.onDocumentChanged(newInput);
			for (IManagerListener l : listeners)
				l.onDocumentChanged(oldInput, newInput);
			if (initial != null)
				update(initial);
		}
	}
}
