package xy.ai.workbench.editors.spellcheck;

import org.eclipse.jface.text.source.Annotation;

public class SpellingAnnotation extends Annotation {

    public static final String TYPE = "xy.ai.workbench.editors.spellcheck.spelling";

    private final SpellingProblem problem;

    public SpellingAnnotation(SpellingProblem problem) {
        super(TYPE, false, problem.getMessage());
        this.problem = problem;
    }

    public SpellingProblem getProblem() {
        return problem;
    }
}
