package xy.ai.workbench.editor.spellcheck;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;

import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.ISynchronizable;
import org.eclipse.jface.text.ITextViewerExtension2;
import org.eclipse.jface.text.Position;
import org.eclipse.jface.text.source.Annotation;
import org.eclipse.jface.text.source.IAnnotationModel;
import org.eclipse.jface.text.source.IAnnotationModelExtension;
import org.eclipse.jface.text.source.ISourceViewer;

import xy.ai.workbench.editor.mdast.nodes.Node;

public class SpellingStrategy {

	private final ISourceViewer viewer;
	private final LanguageToolClient client = new LanguageToolClient();
	private static final int LIMIT = 512 * 1024;

	private IDocument doc;

	public SpellingStrategy(ISourceViewer viewer) {
		this.viewer = viewer;
	}

	public void setDocument(IDocument doc) {
		this.doc = doc;
	}

	public void reconcile(Node node) {
		if (doc == null)
			return;

		final String text = doc.get();
		final int docLength = text.length();

		int offset = node.getOffset();
		int length = Math.max(node.length(), 1);

		// Expand the dirty range to full line boundaries.
		int start = Math.min(offset, docLength);
		int end = Math.min(start + length, docLength);
		if (end - start > LIMIT)
			return;

		while (start > 0 && text.charAt(start - 1) != '\n')
			start--;
		while (end < docLength && text.charAt(end) != '\n')
			end++;

		final int regionOffset = start;
		final String regionText = text.substring(start, end);

		List<SpellingProblem> problems = client.check(regionText);

		// LT offsets are relative to regionText – shift them to document offsets.
		List<SpellingProblem> valid = new ArrayList<>();
		for (SpellingProblem p : problems) {
			int absOffset = p.getOffset() + regionOffset;
			if (absOffset >= 0 && absOffset + p.getLength() <= docLength)
				valid.add(new SpellingProblem(absOffset, p.getLength(), p.getMessage(), p.getSuggestions()));
		}

		final int checkedOffset = regionOffset;
		final int checkedLength = end - start;
		viewer.getTextWidget().getDisplay().asyncExec(() -> applyAnnotations(valid, checkedOffset, checkedLength));
	}

	public void clear(Node node) {
		if (doc == null)
			return;
		int docLength = doc.getLength();
		int offset = node.getOffset();
		int length = Math.max(node.length(), 1);
		int start = Math.max(0, Math.min(offset, docLength));
		int end = Math.max(start, Math.min(start + length, docLength));
		final int clearedOffset = start;
		final int clearedLength = end - start;
		viewer.getTextWidget().getDisplay()
				.asyncExec(() -> applyAnnotations(new ArrayList<>(), clearedOffset, clearedLength));
	}

	// in UI thread
	private void applyAnnotations(List<SpellingProblem> problems, int offset, int length) {
		IAnnotationModel model = viewer.getAnnotationModel();
		if (!(model instanceof IAnnotationModelExtension))
			return;

		int docLength = doc != null ? doc.getLength() : Integer.MAX_VALUE;
		Map<String, SpellingProblem> desired = new HashMap<>();
		for (SpellingProblem p : problems) {
			if (p.getOffset() < 0 || p.getLength() < 0 || p.getOffset() + p.getLength() > docLength)
				continue;
			String text = textAt(p.getOffset(), p.getLength());
			if (text != null)
				desired.put(contentKey(text, p.getMessage()), p);
		}

		Object lock = lockObject(model);
		synchronized (lock) {
			// Every SpellingAnnotation that lies inside the just scanned
			// region is either re-confirmed by a matching problem (same text
			// + message) or is stale and must be removed – nothing from that
			// region is allowed to survive unaccounted for.
			Map<String, Annotation> existing = new HashMap<>();
			List<Annotation> toRemove = new ArrayList<>();
			Iterator<Annotation> it = model.getAnnotationIterator();
			while (it.hasNext()) {
				Annotation a = it.next();
				if (!(a instanceof SpellingAnnotation) || !SpellingAnnotation.TYPE.equals(a.getType()))
					continue;
				Position pos = model.getPosition(a);
				if (pos == null || pos.isDeleted() || pos.getOffset() < offset || pos.getOffset() >= offset + length)
					continue;
				String text = textAt(pos.getOffset(), pos.getLength());
				String key = text == null ? null : contentKey(text, ((SpellingAnnotation) a).getProblem().getMessage());
				if (key != null && desired.containsKey(key))
					existing.put(key, a);
				else
					toRemove.add(a);
			}

			// Add: problems not represented by any still-valid annotation.
			Map<Annotation, Position> toAdd = new HashMap<>();
			for (Map.Entry<String, SpellingProblem> e : desired.entrySet()) {
				if (existing.containsKey(e.getKey()))
					continue;
				SpellingProblem p = e.getValue();
				toAdd.put(new SpellingAnnotation(p), new Position(p.getOffset(), p.getLength()));
			}

			if (!toRemove.isEmpty() || !toAdd.isEmpty())
				((IAnnotationModelExtension) model).replaceAnnotations(toRemove.toArray(new Annotation[0]), toAdd);

			// Change: matched annotations are kept as-is
			for (Map.Entry<String, Annotation> e : existing.entrySet()) {
				SpellingProblem p = desired.get(e.getKey());
				Annotation a = e.getValue();
				Position current = model.getPosition(a);
				if (current != null && (current.getOffset() != p.getOffset() || current.getLength() != p.getLength()))
					((IAnnotationModelExtension) model).modifyAnnotationPosition(a,
							new Position(p.getOffset(), p.getLength()));
			}
		}

		if (viewer instanceof ITextViewerExtension2)
			try {
				((ITextViewerExtension2) viewer).invalidateTextPresentation(offset, length);
			} catch (IllegalArgumentException ex) {
				// ignore out of bound errors
			}
		else
			viewer.invalidateTextPresentation();
	}

	private String textAt(int offset, int length) {
		if (doc == null)
			return null;
		try {
			return doc.get(offset, length);
		} catch (BadLocationException e) {
			return null;
		}
	}

	private static String contentKey(String text, String message) {
		return Integer.toHexString(text.hashCode()) + ":" + text.length() + ":" + message;
	}

	private static Object lockObject(IAnnotationModel model) {
		if (model instanceof ISynchronizable) {
			Object lock = ((ISynchronizable) model).getLockObject();
			if (lock != null)
				return lock;
		}
		return model;
	}
}
