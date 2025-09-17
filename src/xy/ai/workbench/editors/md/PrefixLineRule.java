package xy.ai.workbench.editors.md;

import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.PatternRule;

public class PrefixLineRule extends PatternRule {
	public PrefixLineRule(String prefix, IToken token) {
		super(prefix, null, token, (char) 0, true, true);
		setColumnConstraint(0);
	}
}