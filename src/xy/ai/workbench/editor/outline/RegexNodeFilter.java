package xy.ai.workbench.editor.outline;

import java.util.regex.Pattern;
import java.util.regex.PatternSyntaxException;

import org.eclipse.jface.text.IDocument;
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
		if (p == null || !(element instanceof NodeElement ne))
			return true;
		return matches(p, ne.node(), ne.doc());
	}

	private boolean matches(Pattern p, Node node, IDocument doc) {
		if (p.matcher(NodeLabels.getText(node, doc)).find())
			return true;
		for (Node child : node.children)
			if (matches(p, child, doc))
				return true;
		return false;
	}
}
