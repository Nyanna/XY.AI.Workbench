package xy.ai.conscious.autoapprove.rules;

import java.time.Clock;
import java.time.Duration;
import java.time.Instant;
import java.util.Iterator;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import com.fasterxml.jackson.databind.JsonNode;
import xy.ai.conscious.autoapprove.ApprovalItem;

/**
 * Tool calls by id, tracked by {@link SmallEditRule} and approved by the
 * operator. Entries expire after a fixed validity (renewed by an approval) and
 * are evicted lazily on access; the oldest entries make room once the maximum
 * is exceeded.
 * <p>
 * Control call ids are {@code <session>-<number>-<ref>}; approvals are shared
 * between the calls of the same session prefix only.
 */
final class ApprovalLedger {

	static final int MAX_ENTRIES = 500;
	static final Duration VALIDITY = Duration.ofMinutes(30);

	/**
	 * @param arguments the arguments effectively approved (edited ones win)
	 */
	record Approved(String callId, JsonNode arguments) {
	}

	private record Entry(JsonNode arguments, boolean approved, Instant expires) {
	}

	private final int maxEntries;
	private final Duration validity;
	private final Clock clock;
	// insertion order equals expiry order: every put carries now + validity
	private final LinkedHashMap<String, Entry> entries = new LinkedHashMap<>();

	ApprovalLedger() {
		this(MAX_ENTRIES, VALIDITY, Clock.systemUTC());
	}

	ApprovalLedger(int maxEntries, Duration validity, Clock clock) {
		this.maxEntries = maxEntries;
		this.validity = validity;
		this.clock = clock;
	}

	/** Remembers a call as awaiting approval; a call already known stays as it is. */
	synchronized void track(ApprovalItem call) {
		evictExpired();
		if (call.id().isBlank() || entries.containsKey(call.id()))
			return;
		put(call.id(), new Entry(call.arguments(), false, expiry()));
	}

	/** Approves a tracked call; unknown ids are ignored. */
	synchronized void approve(String callId, JsonNode editedArguments) {
		evictExpired();
		Entry tracked = callId == null ? null : entries.remove(callId);
		if (tracked == null)
			return;
		boolean edited = editedArguments != null && !editedArguments.isMissingNode();
		put(callId, new Entry(edited ? editedArguments : tracked.arguments(), true, expiry()));
	}

	/** Unexpired approved calls of the session the given call id belongs to. */
	synchronized List<Approved> approved(String callId) {
		evictExpired();
		String session = sessionOf(callId);
		return entries.entrySet().stream().filter(e -> e.getValue().approved() && sessionOf(e.getKey()).equals(session)).map(e -> new Approved(e.getKey(), e.getValue().arguments())).toList();
	}

	private void put(String callId, Entry entry) {
		entries.put(callId, entry);
		Iterator<Map.Entry<String, Entry>> oldest = entries.entrySet().iterator();
		while (entries.size() > maxEntries && oldest.hasNext()) {
			oldest.next();
			oldest.remove();
		}
	}

	private Instant expiry() {
		return clock.instant().plus(validity);
	}

	private void evictExpired() {
		Instant now = clock.instant();
		Iterator<Entry> it = entries.values().iterator();
		while (it.hasNext() && !it.next().expires().isAfter(now)) it.remove();
	}

	private static String sessionOf(String callId) {
		int last = callId.lastIndexOf('-');
		int previous = last > 0 ? callId.lastIndexOf('-', last - 1) : -1;
		return previous > 0 ? callId.substring(0, previous) : callId;
	}
}
