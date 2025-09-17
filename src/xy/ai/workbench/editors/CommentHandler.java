package xy.ai.workbench.editors;

import org.eclipse.core.commands.AbstractHandler;
import org.eclipse.core.commands.ExecutionEvent;
import org.eclipse.core.commands.ExecutionException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.ui.handlers.HandlerUtil;
import org.eclipse.ui.texteditor.ITextEditor;

public class CommentHandler extends AbstractHandler {
	@Override
	public Object execute(ExecutionEvent event) throws ExecutionException {
		ITextEditor editor = ((AISessionEditor) HandlerUtil.getActiveEditorChecked(event)).getEditor();

		ITextSelection sel = (ITextSelection) editor.getSelectionProvider().getSelection();
		if (sel.isEmpty())
			return null;
		IDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());

		try {
			int start = sel.getStartLine();
			int end = sel.getEndLine();

			for (int i = start; i <= end; i++) {
				int lineOffset = doc.getLineOffset(i);
				String line = doc.get(lineOffset, doc.getLineLength(i));

				if (line.trim().startsWith(AIRuleScanner.LINE_COMMENT))
					doc.replace(lineOffset + line.indexOf(AIRuleScanner.LINE_COMMENT), 2, "");
				else
					doc.replace(lineOffset, 0, AIRuleScanner.LINE_COMMENT);
			}
		} catch (Exception e) {
			throw new ExecutionException("Failed to toggle comment", e);
		}
		return null;
	}
}