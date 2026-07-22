package xy.ai.workbench.editors;

import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;

import xy.ai.workbench.mdast.IDocumentBuffer;

public final class DocumentBuffer implements IDocumentBuffer {

	private final IDocument document;

	public DocumentBuffer(IDocument document) {
		this.document = document;
	}

	public IDocument document() {
		return document;
	}

	@Override
	public int length() {
		return document.getLength();
	}

	@Override
	public CharSequence subSequence(int start, int end) {
		try {
			return document.get(start, end - start);
		} catch (BadLocationException e) {
			throw new IndexOutOfBoundsException("[" + start + ", " + end + ")");
		}
	}

	@Override
	public void getChars(int start, int length, char[] dest, int destOff) {
		try {
			document.get(start, length).getChars(0, length, dest, destOff);
		} catch (BadLocationException e) {
			throw new IndexOutOfBoundsException("[" + start + ", " + (start + length) + ")");
		}
	}

	@Override
	public void replace(int offset, int length, String text) {
		try {
			document.replace(offset, length, text);
		} catch (BadLocationException e) {
			throw new IndexOutOfBoundsException("replace at " + offset + " len " + length);
		}
	}
}
