package xy.ai.workbench.editors.spellcheck;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

import org.eclipse.jface.text.Position;
import org.eclipse.jface.text.contentassist.CompletionProposal;
import org.eclipse.jface.text.contentassist.ICompletionProposal;
import org.eclipse.jface.text.quickassist.IQuickAssistInvocationContext;
import org.eclipse.jface.text.quickassist.IQuickAssistProcessor;
import org.eclipse.jface.text.source.Annotation;
import org.eclipse.jface.text.source.IAnnotationModel;
import org.eclipse.jface.text.source.ISourceViewer;

import xy.ai.workbench.LOG;

/**
 * Quick-assist processor that offers LanguageTool suggestions as
 * {@link CompletionProposal}s when the cursor is on a {@link SpellingAnnotation}.
 * Activated via Ctrl+1.
 */
public class SpellingQuickAssistProcessor implements IQuickAssistProcessor {

    @Override
    public boolean canFix(Annotation annotation) {
        if (!SpellingAnnotation.TYPE.equals(annotation.getType())) {
            return false;
        }
        SpellingProblem problem = ((SpellingAnnotation) annotation).getProblem();
        return !problem.getSuggestions().isEmpty();
    }

    @Override
    public boolean canAssist(IQuickAssistInvocationContext context) {
        return false;
    }

    @Override
    public ICompletionProposal[] computeQuickAssistProposals(IQuickAssistInvocationContext context) {
        ISourceViewer viewer = context.getSourceViewer();
        IAnnotationModel model = viewer.getAnnotationModel();
        if (model == null) {
            return null;
        }

        int offset = context.getOffset();
        List<ICompletionProposal> proposals = new ArrayList<>();

        Iterator<Annotation> it = model.getAnnotationIterator();
        while (it.hasNext()) {
            Annotation annotation = it.next();
            if (!SpellingAnnotation.TYPE.equals(annotation.getType())) {
                continue;
            }
            Position pos = model.getPosition(annotation);
            if (pos == null || !pos.overlapsWith(offset, 1)) {
                continue;
            }

            SpellingProblem problem = ((SpellingAnnotation) annotation).getProblem();
            LOG.info("SpellCheck: quick-assist at offset " + offset + " – "
                    + problem.getSuggestions().size() + " suggestion(s)");

            for (String suggestion : problem.getSuggestions()) {
                proposals.add(new CompletionProposal(
                        suggestion,          // replacement text
                        pos.offset,          // replacement offset
                        pos.length,          // replacement length
                        suggestion.length(), // cursor position after replacement
                        null,                // image
                        suggestion,          // display string
                        null,                // context information
                        problem.getMessage() // additional info shown on the right
                ));
            }
        }

        return proposals.isEmpty() ? null : proposals.toArray(new ICompletionProposal[0]);
    }

    @Override
    public String getErrorMessage() {
        return null;
    }
}
