package xy.ai.workbench.editors;

import java.util.ArrayList;
import java.util.List;

import org.eclipse.jface.text.rules.IRule;
import org.eclipse.jface.text.rules.IToken;
import org.eclipse.jface.text.rules.IWhitespaceDetector;
import org.eclipse.jface.text.rules.IWordDetector;
import org.eclipse.jface.text.rules.RuleBasedScanner;
import org.eclipse.jface.text.rules.Token;
import org.eclipse.jface.text.rules.WhitespaceRule;
import org.eclipse.jface.text.rules.WordRule;

import xy.ai.workbench.AISessionManager;

public class AIRuleScanner extends RuleBasedScanner {
	public AIRuleScanner() {
		IToken grayToken = new Token(AIText.GRAY_ATTR);
		IToken blueToken = new Token(AIText.BLUE_ATTR);
		IToken defaultToken = new Token(AIText.DEFAULT_ATTR);

		List<IRule> rules = new ArrayList<>();

		WordRule wordRule = new WordRule(new IWordDetector() {
			@Override
			public boolean isWordStart(char c) {
				return Character.isLetter(c);
			}

			@Override
			public boolean isWordPart(char c) {
				return Character.isLetter(c) || c == ':';
			}
		}, defaultToken);
		wordRule.addWord(AISessionManager.USER, grayToken);
		wordRule.addWord(AISessionManager.AGENT, blueToken);
		rules.add(wordRule);

		rules.add(new WhitespaceRule(new IWhitespaceDetector() {
			@Override
			public boolean isWhitespace(char c) {
				return Character.isWhitespace(c);
			}
		}));

		setRules(rules.toArray(new IRule[0]));
	}
}