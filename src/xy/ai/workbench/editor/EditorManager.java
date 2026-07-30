package xy.ai.workbench.editor;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.Iterator;
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

import xy.ai.workbench.editor.mdast.MarkdownDocument;
import xy.ai.workbench.editor.mdast.nodes.Node;
import xy.ai.workbench.tools.RegionList;

public class EditorManager {

	public static final int DEBOUNCE_DELAY_MS = 500;

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

	private final RegionList<Integer> pending = new RegionList<>();

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
			update(ast.getRoot());
		}
	}

	private void changed(int offset, int removedLen, int insertedLen) {
		int accStart = offset;
		int accOldLen = removedLen;
		int accSpan = removedLen;

		List<RegionList.Region<Integer>> rest = new ArrayList<>(pending.asList());
		pending.clear();

		boolean mergedAny;
		do {
			mergedAny = false;
			for (Iterator<RegionList.Region<Integer>> it = rest.iterator(); it.hasNext();) {
				RegionList.Region<Integer> r = it.next();
				int rEnd = r.end();
				int accEnd = accStart + accSpan;
				if (rEnd < accStart || r.offset() > accEnd)
					continue; // no overlap and not touching -> stays separate

				int newStart = Math.min(accStart, r.offset());
				int newEnd = Math.max(accEnd, rEnd);
				int priorDelta = (accSpan - accOldLen) + (r.length() - r.value());
				accStart = newStart;
				accSpan = newEnd - newStart;
				accOldLen = accSpan - priorDelta;
				it.remove();
				mergedAny = true;
			}
		} while (mergedAny);

		int preDeltaEnd = accStart + accSpan;
		int mergedNewLen = accSpan + (insertedLen - removedLen);
		int mergedOldLen = accOldLen;
		int totalDelta = mergedNewLen - mergedOldLen;

		for (RegionList.Region<Integer> r : rest)
			if (r.offset() >= preDeltaEnd)
				pending.add(r.offset() + totalDelta, r.length(), r.value());
			else
				pending.add(r.offset(), r.length(), r.value());
		pending.add(accStart, mergedNewLen, mergedOldLen);

		scheduleFlush();
	}

	private void update(Node node) {
		for (IManagerListener l : listeners)
			l.onAstUpdated(node);
		if (viewer instanceof ITextViewerExtension2 ext2)
			try {
				ext2.invalidateTextPresentation(node.getOffset(), node.length());
			} catch (IllegalArgumentException e) {
				// region outside the (possibly just replaced) document - ignore.
			}

		if (spell != null && !background.isShutdown())
			background.execute(() -> {
				spell.reconcile(node);
				viewer.getTextWidget().getDisplay().asyncExec(() -> {
					if (viewer instanceof ITextViewerExtension2 ext2)
						try {
							ext2.invalidateTextPresentation(node.getOffset(), node.length());
						} catch (IllegalArgumentException e) {
							// region outside the (possibly just replaced) document - ignore.
						}
				});
			});
	}

	private void scheduleFlush() {
		if (display == null || display.isDisposed())
			return;
		display.timerExec(DEBOUNCE_DELAY_MS, flush);
	}

	private void cancelPending() {
		pending.clear();
		if (display != null && !display.isDisposed())
			display.timerExec(-1, flush);
	}

	private class Flush implements Runnable {
		@Override
		public void run() {
			if (pending.isEmpty() || ast == null || buffer == null)
				return;

			if (pending.millisSinceLastInsert() < DEBOUNCE_DELAY_MS) {
				scheduleFlush();
				return;
			}
			// LOG.info("AST Flushed");

			List<RegionList.Region<Integer>> regions = new ArrayList<>(pending.asList());
			pending.clear();
			regions.sort(Comparator.comparingInt(RegionList.Region::offset));

			for (RegionList.Region<Integer> r : regions)
				update(ast.update(r.offset(), r.value(), r.length()));
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
