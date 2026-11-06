package xy.ai.workbench.models;

import java.util.function.Supplier;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class TimeStats {
	/** Epoch millis of the last prompt */
	public long startAt;
	public long endAt;
	/** Runtime of the last prompt in ms, excluding control client time */
	public long durationMs;
	/** Cumulative durationMs of all prompts in the session */
	public long totalMs;
	public long ioMs;
	public long controlMs;
	public long sent;
	public long received;

	private static final Pattern PATTERN = Pattern.compile(
			"start: (\\d+), end: (\\d+), time: (\\d+), total: (\\d+), io: (\\d+), control: (\\d+), sent: (\\d+), recv: (\\d+)");

	public void start() {
		startAt = System.currentTimeMillis();
	}

	/** Ends the measurement; control time is excluded from the duration. */
	public void stop() {
		endAt = System.currentTimeMillis();
		durationMs = Math.max(0, endAt - startAt - controlMs);
	}

	public <T> T control(Supplier<T> call) {
		long t = System.nanoTime();
		try {
			return call.get();
		} finally {
			controlMs += (System.nanoTime() - t) / 1_000_000;
		}
	}

	public <T> T io(Supplier<T> call) {
		long t = System.nanoTime();
		try {
			return call.get();
		} finally {
			ioMs += (System.nanoTime() - t) / 1_000_000;
		}
	}

	@Override
	public String toString() {
		return String.format(
				"TimeStats [startAt=%s, endAt=%s, durationMs=%s, totalMs=%s, ioMs=%s, controlMs=%s, sent=%s, received=%s]",
				startAt, endAt, durationMs, totalMs, ioMs, controlMs, sent, received);
	}

	public String print() {
		return String.format("start: %s, end: %s, time: %s, total: %s, io: %s, control: %s, sent: %s, recv: %s", startAt,
				endAt, durationMs, totalMs, ioMs, controlMs, sent, received);
	}

	public static TimeStats fromString(String s) {
		if (s == null)
			return null;
		Matcher m = PATTERN.matcher(s);
		if (!m.find())
			return null;
		TimeStats t = new TimeStats();
		t.startAt = Long.parseLong(m.group(1));
		t.endAt = Long.parseLong(m.group(2));
		t.durationMs = Long.parseLong(m.group(3));
		t.totalMs = Long.parseLong(m.group(4));
		t.ioMs = Long.parseLong(m.group(5));
		t.controlMs = Long.parseLong(m.group(6));
		t.sent = Long.parseLong(m.group(7));
		t.received = Long.parseLong(m.group(8));
		return t;
	}

	/** Accumulates a finished prompt into this (session) total. */
	public void add(TimeStats t) {
		startAt = t.startAt;
		endAt = t.endAt;
		durationMs = t.durationMs;
		totalMs += t.durationMs;
		ioMs += t.ioMs;
		controlMs += t.controlMs;
		sent += t.sent;
		received += t.received;
	}
}
