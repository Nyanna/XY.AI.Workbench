package xy.ai.workbench.editors.spellcheck;

import java.util.List;

import org.eclipse.jface.text.source.Annotation;

public class SpellingAnnotation extends Annotation {

    public static final String TYPE = "xy.ai.workbench.editors.spellcheck.spelling";

    private final SpellingProblem problem;

    public SpellingAnnotation(SpellingProblem problem) {
        super(TYPE, false, buildText(problem));
        this.problem = problem;
    }

    public SpellingProblem getProblem() {
        return problem;
    }

    private static String buildText(SpellingProblem problem) {
        List<String> suggestions = problem.getSuggestions();
        if (suggestions.isEmpty()) {
            return problem.getMessage();
        }
        return problem.getMessage() + "\nSuggestions: " + String.join(", ", suggestions);
    }
}
