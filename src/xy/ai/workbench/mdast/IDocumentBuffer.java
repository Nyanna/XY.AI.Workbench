package xy.ai.workbench.mdast;

import xy.ai.workbench.tools.LineIndex.Buffer;

public interface IDocumentBuffer extends Buffer {

	public void replace(int offset, int length, String text);
}
