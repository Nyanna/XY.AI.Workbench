package xy.ai.workbench.tools;

import java.util.Arrays;

public class LineIndex {
	private static final int DEFAULT_SEGMENT = 1 << 16; // 64K
	private static final char[] EMPTY = new char[0];

	private int[] nlIndex = new int[64];
	private int count; // valid entries in index
	private int bufferLength;
	private int[] scratch = new int[64];

	public void update(Buffer buffer) {
		int len = buffer.length();
		this.count = 0;
		this.bufferLength = len;
		if (len == 0)
			return;
		ensureCapacity(Math.max(16, len / 40));
		char[] seg = new char[Math.min(DEFAULT_SEGMENT, len)];
		for (int off = 0; off < len; off += seg.length) {
			int n = Math.min(seg.length, len - off);
			buffer.getChars(off, n, seg, 0);
			update(seg, 0, n, off, n);
		}
	}

	public void update(Buffer buffer, int offset, int removedLen, int insertedLen) {
		char[] chars = EMPTY;
		if (insertedLen > 0) {
			chars = new char[insertedLen];
			buffer.getChars(offset, insertedLen, chars, 0);
		}
		update(chars, 0, insertedLen, offset, removedLen);
	}

	public void update(char[] chars, int arrayOffset, int newLength, int bufferOffset, int oldLength) {
		int scanEnd = arrayOffset + newLength;
		int base = bufferOffset - arrayOffset;

		// scan lines
		int found = 0;
		int[] tmp = scratch;
		for (int i = arrayOffset; i < scanEnd; i++) {
			if (chars[i] == '\n') {
				if (found == tmp.length)
					tmp = Arrays.copyOf(tmp, tmp.length + (tmp.length >> 1));
				tmp[found++] = base + i;
			}
		}
		scratch = tmp;

		// invalidate
		int from = lowerBound(bufferOffset);
		int to = lowerBound(bufferOffset + oldLength);

		int delta = newLength - oldLength;
		int resultCount = count - (to - from) + found;
		ensureCapacity(resultCount);

		// insert
		int tailLen = count - to;
		int newTailStart = from + found;
		if (tailLen > 0 && newTailStart != to)
			System.arraycopy(nlIndex, to, nlIndex, newTailStart, tailLen);
		if (delta != 0)
			for (int i = newTailStart, e = newTailStart + tailLen; i < e; i++)
				nlIndex[i] += delta;

		if (found > 0)
			System.arraycopy(tmp, 0, nlIndex, from, found);

		count = resultCount;
		bufferLength += delta;
	}

	public void addOffset(int offset) {
		if (count > 0 && offset <= nlIndex[count - 1]) {
			int idx = lowerBound(offset);
			if (idx < count && nlIndex[idx] == offset)
				return; // offset already present, nothing to do
			ensureCapacity(count + 1);
			insertShifted(idx, offset);
		} else {
			ensureCapacity(count + 1);
			nlIndex[count++] = offset;
		}
		if (offset >= bufferLength)
			bufferLength = offset + 1;
	}

	private void insertShifted(int idx, int offset) {
		int tailLen = count - idx;
		if (tailLen > 0)
			System.arraycopy(nlIndex, idx, nlIndex, idx + 1, tailLen);
		nlIndex[idx] = offset;
		count++;
	}

	public int lineCount() {
		return count + 1;
	}

	public int bufferLength() {
		return bufferLength;
	}

	public int lineStartOffset(int line) {
		if (line <= 0)
			return 0;
		if (line > count)
			line = count;
		return nlIndex[line - 1] + 1;
	}

	public int lineEndOffset(int line) {
		if (line < 0)
			line = 0;
		if (line >= count)
			return bufferLength;
		return nlIndex[line];
	}

	public int lineOfOffset(int offset) {
		if (count == 0 || offset <= nlIndex[0])
			return 0;
		int idx = lowerBound(offset);
		if (idx < count && nlIndex[idx] < offset)
			idx++;
		return idx;
	}

	private int lowerBound(int value) {
		int lo = 0, hi = count;
		while (lo < hi) {
			int mid = (lo + hi) >>> 1;
			if (nlIndex[mid] < value)
				lo = mid + 1;
			else
				hi = mid;
		}
		return lo;
	}

	private void ensureCapacity(int min) {
		if (nlIndex.length >= min)
			return;
		int next = nlIndex.length + (nlIndex.length >> 1);
		if (next < min)
			next = min;
		nlIndex = Arrays.copyOf(nlIndex, next);
	}

	public static interface Buffer {
		public int length();

		public CharSequence subSequence(int start, int end);

		public default void getChars(int start, int length, char[] dest, int destOff) {
			CharSequence cs = subSequence(start, start + length);
			if (cs instanceof String s)
				s.getChars(0, length, dest, destOff);
			else
				for (int i = 0; i < length; i++)
					dest[destOff + i] = cs.charAt(i);
		}
	}
}
