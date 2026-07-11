package xy.ai.workbench.connectors.claudecode;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.function.Consumer;

import xy.ai.workbench.LOG;

public class ClaudeCodeSessionManager {
	public final static String CREATE_NEW_MARKER = "CREATE_NEW_MARKER";

	/** Ordered list of all known sessions. Access must be {@code synchronized}. */
	private final List<ClaudeCodeSession> sessions = new ArrayList<>();

	/**
	 * UUID of the session currently selected in the panel, or {@code null}. Updated
	 * by the panel view on selection change.
	 */
	private volatile String selectedSessionUuid;

	private final List<Consumer<List<ClaudeCodeSession>>> changeListeners = new ArrayList<>();

	public synchronized ClaudeCodeSession requestSession(String selectedUuid, SessionParameters params) {
		cleanupInvalidTerminated();

		ClaudeCodeSession session = null;
		if (CREATE_NEW_MARKER.equals(selectedUuid) || selectedUuid == null) {
			session = new ClaudeCodeSession(this, params);
			sessions.add(session);
			LOG.info("ClaudeCodeSessionManager: created session, hash=" + params.getHash());
			fireChanged();
		} else {
			session = findByUuid(selectedUuid);
			if (session == null)
				throw new IllegalStateException("Selected session not found: " + selectedUuid);
			if (session.isExpired())
				throw new IllegalStateException("Selected session has expired");
			if (!params.getHash().equals(session.getParameters().getHash()))
				throw new IllegalStateException(
						"Selected session parameters are incompatible with the current configuration");

			session = findByHash(params.getHash());
		}
		if (session == null)
			throw new IllegalStateException("No Session available");
		if (session.isExpired())
			throw new IllegalStateException("Session has expired");
		return session;
	}

	public synchronized ClaudeCodeSession getSession(String selectedUuid, SessionParameters params) {
		cleanupInvalidTerminated();
		ClaudeCodeSession res = null;

		if (!CREATE_NEW_MARKER.equals(selectedUuid))
			res = selectedUuid != null ? findByUuid(selectedUuid) : findByHash(params.getHash());

		if (res == null)
			throw new IllegalStateException("Cannot process /exit: no active Claude Code session exists");
		return res;
	}

	public synchronized ClaudeCodeSession importSession(String uuid, SessionParameters params) {
		cleanupInvalidTerminated();

		ClaudeCodeSession session = new ClaudeCodeSession(uuid, this, params);
		sessions.add(session);
		LOG.info("ClaudeCodeSessionManager: imported session, uuid=" + uuid);
		fireChanged();
		return session;
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

	public synchronized List<ClaudeCodeSession> getSessions() {
		return Collections.unmodifiableList(new ArrayList<>(sessions));
	}

	public void addChangeListener(Consumer<List<ClaudeCodeSession>> listener) {
		synchronized (changeListeners) {
			changeListeners.add(listener);
		}
	}

	public void removeChangeListener(Consumer<List<ClaudeCodeSession>> listener) {
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

	void onSessionChanged(ClaudeCodeSession session) {
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

	private ClaudeCodeSession findByUuid(String uuid) {
		if (uuid != null)
			for (ClaudeCodeSession s : sessions)
				if (uuid.equals(s.getSessionUuid()))
					return s;
		return null;
	}

	private ClaudeCodeSession findByHash(String hash) {
		for (ClaudeCodeSession s : sessions)
			if (hash.equals(s.getParameters().getHash()))
				return s;
		return null;
	}

	private void fireChanged() {
		List<ClaudeCodeSession> snapshot = Collections.unmodifiableList(new ArrayList<>(sessions));
		List<Consumer<List<ClaudeCodeSession>>> copy;
		synchronized (changeListeners) {
			copy = new ArrayList<>(changeListeners);
		}
		for (Consumer<List<ClaudeCodeSession>> l : copy) {
			try {
				l.accept(snapshot);
			} catch (Exception e) {
				LOG.error("Change listener threw", e);
			}
		}
	}
}
