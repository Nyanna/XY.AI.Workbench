package xy.ai.workbench.editors.spellcheck;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;

import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.Position;
import org.eclipse.jface.text.Region;
import org.eclipse.jface.text.reconciler.DirtyRegion;
import org.eclipse.jface.text.reconciler.IReconcilingStrategy;
import org.eclipse.jface.text.source.Annotation;
import org.eclipse.jface.text.source.IAnnotationModel;
import org.eclipse.jface.text.source.IAnnotationModelExtension;
import org.eclipse.jface.text.source.ISourceViewer;

/**
 * Reconciling strategy that runs spell-checking via LanguageTool on a
 * background thread and posts resulting annotations back to the UI thread.
 * <p>
 * The region passed to {@link #reconcile(IRegion)} is expanded to full line
 * boundaries before being sent to LanguageTool, so only the affected lines are
 * ever checked.
 */
public class SpellingStrategy implements IReconcilingStrategy {

	private final ISourceViewer fViewer;
	private final LanguageToolClient fClient = new LanguageToolClient();
	private static final int LIMIT = 512 * 1024;

	private IDocument fDocument;

	public SpellingStrategy(ISourceViewer viewer) {
		fViewer = viewer;
	}

	@Override
	public void setDocument(IDocument document) {
		fDocument = document;
	}

	@Override
	public void reconcile(IRegion partition) {
		if (fDocument == null) {
			return;
		}

		final String text = fDocument.get();
		final int docLength = text.length();

		// Expand the dirty region to full line boundaries.
		int start = Math.min(partition.getOffset(), docLength);
		int end = Math.min(start + partition.getLength(), docLength);
		if (end - start > LIMIT)
			return;

		while (start > 0 && text.charAt(start - 1) != '\n')
			start--;
		while (end < docLength && text.charAt(end) != '\n')
			end++;

		final int regionOffset = start;
		final String regionText = text.substring(start, end);

		List<SpellingProblem> problems = fClient.check(regionText);

		// LT offsets are relative to regionText – shift them to document offsets.
		List<SpellingProblem> valid = new ArrayList<>();
		for (SpellingProblem p : problems) {
			int absOffset = p.getOffset() + regionOffset;
			if (absOffset >= 0 && absOffset + p.getLength() <= docLength) {
				valid.add(new SpellingProblem(absOffset, p.getLength(), p.getMessage(), p.getSuggestions()));
			}
		}

		final IRegion checkedRegion = new Region(regionOffset, end - start);
		fViewer.getTextWidget().getDisplay().asyncExec(() -> applyAnnotations(valid, checkedRegion));
	}

	@Override
	public void reconcile(DirtyRegion dirtyRegion, IRegion subRegion) {
		reconcile(subRegion);
	}

	// ── UI thread ──────────────────────────────────────────────────────────────

	private void applyAnnotations(List<SpellingProblem> problems, IRegion region) {
		IAnnotationModel model = fViewer.getAnnotationModel();
		if (!(model instanceof IAnnotationModelExtension)) {
			return;
		}

		// Collect all existing spelling annotations in the checked region.
		List<Annotation> toRemove = new ArrayList<>();
		synchronized (model) {
			Iterator<Annotation> it = model.getAnnotationIterator();
			while (it.hasNext()) {
				Annotation a = it.next();
				if (SpellingAnnotation.TYPE.equals(a.getType())) {
					Position pos = model.getPosition(a);
					if (pos != null && pos.offset >= region.getOffset()
							&& pos.offset < region.getOffset() + region.getLength()) {
						toRemove.add(a);
					}
				}
			}
		}

		// Build new annotations.
		Map<Annotation, Position> toAdd = new HashMap<>();
		for (SpellingProblem p : problems) {
			toAdd.put(new SpellingAnnotation(p), new Position(p.getOffset(), p.getLength()));
		}

		// Atomic swap – removes old, adds new in one operation.
		synchronized (model) {
			((IAnnotationModelExtension) model).replaceAnnotations(toRemove.toArray(new Annotation[0]), toAdd);
		}

		// Explicitly invalidate the region so the AnnotationPainter redraws it.
		fViewer.invalidateTextPresentation();
	}
}
