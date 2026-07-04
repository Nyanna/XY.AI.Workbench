package xy.ai.workbench.editors.spellcheck;

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

    private ITextViewer fViewer;
    private IDocument fDocument;

    // Pending dirty region – merged across rapid edits; guarded by 'this'.
    private int fDirtyStart = Integer.MAX_VALUE;
    private int fDirtyEnd   = 0;

    private final ScheduledExecutorService fScheduler =
            Executors.newSingleThreadScheduledExecutor(r -> {
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
            int start = event.getOffset();
            int end   = start + Math.max(
                    event.getLength(),
                    event.getText() != null ? event.getText().length() : 0);
            mergeDirty(start, Math.max(end, start + 1));
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

    public SpellCheckReconciler(SpellingStrategy strategy, int delayMs) {
        fStrategy = strategy;
        fDelayMs  = delayMs;
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

    // ── Internal ───────────────────────────────────────────────────────────────

    private synchronized void mergeDirty(int start, int end) {
        fDirtyStart = Math.min(fDirtyStart, start);
        fDirtyEnd   = Math.max(fDirtyEnd, end);
    }

    private synchronized IRegion takeDirty() {
        if (fDirtyStart > fDirtyEnd) {
            return null;
        }
        IRegion region = new Region(fDirtyStart, fDirtyEnd - fDirtyStart);
        fDirtyStart = Integer.MAX_VALUE;
        fDirtyEnd   = 0;
        return region;
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
            IRegion dirty = takeDirty();
            if (dirty != null)
                fStrategy.reconcile(dirty);
        }, fDelayMs, TimeUnit.MILLISECONDS);
    }
}
