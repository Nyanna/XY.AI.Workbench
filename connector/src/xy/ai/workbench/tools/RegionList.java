package xy.ai.workbench.tools;

import java.util.ArrayList;
import java.util.Deque;
import java.util.Iterator;
import java.util.List;
import java.util.Objects;
import java.util.concurrent.ConcurrentLinkedDeque;

public class RegionList<T> implements Iterable<RegionList.Region<T>> {

	public record Region<T>(int offset, int length, T value) {
		public int end() {
			return offset + length;
		}
	}

	private final List<Region<T>> pieces = new ArrayList<>();
	private int cursor;

	private final Deque<Long> insertTimes = new ConcurrentLinkedDeque<>();

	public void add(int offset, int length, T value) {
		insertTimes.addLast(System.currentTimeMillis());
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
		insertTimes.clear();
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

	public long lastInsertTime() {
		Long last = insertTimes.peekLast();
		return last == null ? -1L : last;
	}

	public long millisSinceLastInsert() {
		long last = lastInsertTime();
		return last < 0 ? Long.MAX_VALUE : System.currentTimeMillis() - last;
	}

	@Override
	public Iterator<Region<T>> iterator() {
		return pieces.iterator();
	}
}
