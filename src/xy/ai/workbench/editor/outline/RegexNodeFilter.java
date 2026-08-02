package xy.ai.workbench.editor.outline;

import java.util.regex.Pattern;
import java.util.regex.PatternSyntaxException;

import org.eclipse.jface.viewers.Viewer;
import org.eclipse.jface.viewers.ViewerFilter;

import xy.ai.workbench.editor.mdast.nodes.Node;

public class RegexNodeFilter extends ViewerFilter {

	private volatile Pattern pattern;

	public void setPattern(String regex) {
		if (regex == null || regex.isBlank()) {
			pattern = null;
			return;
		}
		try {
			pattern = Pattern.compile(regex, Pattern.CASE_INSENSITIVE);
		} catch (PatternSyntaxException e) {
			// keep the previous pattern until the expression is valid again
		}
	}

	public boolean isActive() {
		return pattern != null;
	}

	@Override
	public boolean select(Viewer viewer, Object parentElement, Object element) {
		Pattern p = pattern;
		if (p == null || !(element instanceof Node node))
			return true;
		return matches(p, node);
	}

	private boolean matches(Pattern p, Node node) {
		if (p.matcher(NodeLabels.getText(node)).find())
			return true;
		for (Node child : node.children)
			if (matches(p, child))
				return true;
		return false;
	}
}
