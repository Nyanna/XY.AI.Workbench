package xy.ai.workbench.connectors.claudecode;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.function.Consumer;

import xy.ai.workbench.LOG;

public class CCSessionManager {
	public final static String CREATE_NEW_MARKER = "CREATE_NEW_MARKER";

	/** Ordered list of all known sessions. Access must be {@code synchronized}. */
	private final List<CCSession> sessions = new ArrayList<>();

	/**
	 * UUID of the session currently selected in the panel, or {@code null}. Updated
	 * by the panel view on selection change.
	 */
	private volatile String selectedSessionUuid;

	private final List<Consumer<List<CCSession>>> changeListeners = new ArrayList<>();

	public synchronized CCSession requestSession(String selectedUuid, SessionParameters params) {
		cleanupInvalidTerminated();

		CCSession session = null;
		if (CREATE_NEW_MARKER.equals(selectedUuid)) {
			session = addSession(new CCSession(this, params));
			LOG.info("New session created, hash=" + params.getHash());
		} else if (selectedUuid != null) {
			if ((session = findByUuid(selectedUuid)) == null)
				throw new IllegalStateException("Selected session not found: " + selectedUuid);
			else if (!params.getHash().equals(session.getParameters().getHash()))
				throw new IllegalStateException(
						"Selected session parameters are incompatible with the current configuration");
			if (session.isExpired())
				throw new IllegalStateException("Session has expired");
		} else if ((session = findByHash(params.getHash())) != null) {
			LOG.info("Use param hash session, hash=" + params.getHash());
			if (session.isExpired())
				throw new IllegalStateException("Session has expired");
		} else {
			session = addSession(new CCSession(this, params));
			LOG.info("New session created, hash=" + params.getHash());
		}

		return session;
	}

	public synchronized CCSession getSession(String selectedUuid, SessionParameters params) {
		cleanupInvalidTerminated();
		CCSession res = null;

		if (!CREATE_NEW_MARKER.equals(selectedUuid))
			res = selectedUuid != null ? findByUuid(selectedUuid) : findByHash(params.getHash());

		if (res == null)
			throw new IllegalStateException("Cannot process command: no active Claude Code session exists");
		return res;
	}

	private CCSession addSession(CCSession session) {
		sessions.add(session);
		fireChanged();
		return session;
	}

	public synchronized CCSession importSession(String uuid, SessionParameters params) {
		cleanupInvalidTerminated();
		LOG.info("Imported session, uuid=" + uuid);
		return addSession(new CCSession(uuid, this, params));
	}

	public synchronized void terminateSessions(List<String> toTerminate) {
		for (String id : toTerminate)
			terminateSession(id);
	}

	public synchronized void terminateSession(String uuidOrHash) {
		var session = findByUuid(uuidOrHash);
		if (session == null)
			session = findByHash(uuidOrHash);

		if (session == null)
			return;

		session.terminate();
		fireChanged();
	}

	public synchronized List<CCSession> getSessions() {
		return Collections.unmodifiableList(new ArrayList<>(sessions));
	}

	public void addChangeListener(Consumer<List<CCSession>> listener) {
		synchronized (changeListeners) {
			changeListeners.add(listener);
		}
		fireChanged(List.of(listener));
	}

	public void removeChangeListener(Consumer<List<CCSession>> listener) {
		synchronized (changeListeners) {
			changeListeners.remove(listener);
		}
	}

	public String getSelectedSessionUuid() {
		return selectedSessionUuid;
	}

	public void setSelectedSessionUuid(String uuid) {
		this.selectedSessionUuid = uuid;
	}

	void onSessionChanged(CCSession session) {
		fireChanged();
	}

	private void cleanupInvalidTerminated() {
		sessions.removeIf(s -> {
			if (s.isExpired()) {
				s.terminate();
				return true;
			}
			return false;
		});
	}

	private CCSession findByUuid(String uuid) {
		if (uuid != null)
			for (CCSession s : sessions)
				if (uuid.equals(s.getSessionUuid()))
					return s;
		return null;
	}

	private CCSession findByHash(String hash) {
		for (CCSession s : sessions)
			if (hash.equals(s.getParameters().getHash()))
				return s;
		return null;
	}

	private void fireChanged() {
		fireChanged(changeListeners);
	}

	private void fireChanged(List<Consumer<List<CCSession>>> changeListeners) {
		List<CCSession> snapshot = Collections.unmodifiableList(new ArrayList<>(sessions));
		List<Consumer<List<CCSession>>> copy;
		synchronized (changeListeners) {
			copy = new ArrayList<>(changeListeners);
		}
		for (Consumer<List<CCSession>> l : copy) {
			try {
				l.accept(snapshot);
			} catch (Exception e) {
				LOG.error("Change listener threw", e);
			}
		}
	}
}
