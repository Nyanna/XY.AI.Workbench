package xy.ai.workbench.editor.mdast;

import xy.ai.conscious.tools.LineIndex.Buffer;

public interface IDocumentBuffer extends Buffer {

	public void replace(int offset, int length, String text);
}
