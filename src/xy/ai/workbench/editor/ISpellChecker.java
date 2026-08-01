package xy.ai.workbench.editor;

import org.eclipse.jface.text.IDocument;

import xy.ai.workbench.editor.mdast.ModificationRange;

public interface ISpellChecker {

	public void onDocumentChanged(IDocument document);

	public void reconcile(ModificationRange range);
}
