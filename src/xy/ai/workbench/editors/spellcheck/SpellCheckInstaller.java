package xy.ai.workbench.editors.spellcheck;

import org.eclipse.jface.text.ITextViewerExtension2;
import org.eclipse.jface.text.reconciler.IReconciler;
import org.eclipse.jface.text.source.AnnotationPainter;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.text.source.SourceViewer;
import org.eclipse.swt.SWT;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.texteditor.DefaultMarkerAnnotationAccess;

public class SpellCheckInstaller {

    private static final int RECONCILE_DELAY_MS = 500;

    public static IReconciler createReconciler(ISourceViewer sourceViewer) {
        SpellingStrategy strategy = new SpellingStrategy(sourceViewer);
        return new SpellCheckReconciler(strategy, RECONCILE_DELAY_MS);
    }

    public static void installPainter(ISourceViewer sourceViewer) {
        Display display = sourceViewer.getTextWidget().getDisplay();

        AnnotationPainter painter = new AnnotationPainter(
                sourceViewer, new DefaultMarkerAnnotationAccess());

        painter.addTextStyleStrategy(
                SpellingAnnotation.TYPE,
                new AnnotationPainter.UnderlineStrategy(SWT.UNDERLINE_SQUIGGLE));

        painter.addAnnotationType(SpellingAnnotation.TYPE, SpellingAnnotation.TYPE);

        painter.setAnnotationTypeColor(
                SpellingAnnotation.TYPE,
                display.getSystemColor(SWT.COLOR_RED));

        // addTextStyleStrategy works through ITextPresentationListener – register explicitly,
        // because addPainter() alone does NOT do this registration.
        // addTextPresentationListener is only on the concrete SourceViewer class, not on ISourceViewer.
        if (sourceViewer instanceof SourceViewer) {
            ((SourceViewer) sourceViewer).addTextPresentationListener(painter);
        }
        ((ITextViewerExtension2) sourceViewer).addPainter(painter);
    }
}
