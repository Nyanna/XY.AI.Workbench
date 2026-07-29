package xy.ai.workbench.tools;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Objects;

public class RegionList<T> implements Iterable<RegionList.Region<T>> {

	public record Region<T>(int offset, int length, T value) {
		public int end() {
			return offset + length;
		}
	}

	private final List<Region<T>> pieces = new ArrayList<>();
	private int cursor;

	public void add(int offset, int length, T value) {
		int start = offset;
		int end = offset + length;
		for (Iterator<Region<T>> it = pieces.iterator(); it.hasNext();) {
			Region<T> p = it.next();
			if (start <= p.end() && p.offset() <= end && Objects.equals(p.value(), value)) {
				start = Math.min(start, p.offset());
				end = Math.max(end, p.end());
				it.remove();
			}
		}
		pieces.add(new Region<>(start, end - start, value));
	}

	public boolean isEmpty() {
		return pieces.isEmpty();
	}

	public int size() {
		return pieces.size();
	}

	public List<Region<T>> asList() {
		return pieces;
	}

	public void clear() {
		pieces.clear();
		cursor = 0;
	}

	public void resetCursor() {
		cursor = 0;
	}

	public boolean hasNext() {
		return cursor < pieces.size();
	}

	public Region<T> next() {
		return pieces.get(cursor++);
	}

	@Override
	public Iterator<Region<T>> iterator() {
		return pieces.iterator();
	}
}
