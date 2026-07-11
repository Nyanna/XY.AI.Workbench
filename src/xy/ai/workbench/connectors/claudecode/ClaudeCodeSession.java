package xy.ai.workbench.connectors.claudecode;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.PrintWriter;
import java.io.Writer;
import java.nio.file.Files;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.List;
import java.util.Objects;
import java.util.UUID;

import xy.ai.workbench.LOG;

public class ClaudeCodeSession {
	private static final long TTL_HOURS = 1;

	/**
	 * The Claude session UUID. {@code null} until the process is started for the
	 * first time (or until a UUID is pre-assigned via {@link #assignUuid}).
	 */
	private final String uuid;
	private final SessionParameters parameters;

	private Process process;
	private PrintWriter stdin;
	private BufferedReader stdout;
	private BufferedReader stderr;
	private Writer mirror;

	@SuppressWarnings("unused")
	private final Instant createdAt = Instant.now();
	@SuppressWarnings("unused")
	private Instant startedAt;
	/** The last time a prompt was sent to STDIN. Determines TTL. */
	private Instant lastSentAt;
	@SuppressWarnings("unused")
	private Instant lastReceivedAt;

	private volatile boolean inPrompt;
	private volatile String lastParsedMessage;
	private boolean resume;

	private final ClaudeCodeSessionManager manager;

	public ClaudeCodeSession(ClaudeCodeSessionManager manager, SessionParameters parameters) {
		this(UUID.randomUUID().toString(), false, manager, parameters);
	}

	public ClaudeCodeSession(String sessionUuid, ClaudeCodeSessionManager manager, SessionParameters parameters) {
		this(sessionUuid, true, manager, parameters);
	}

	private ClaudeCodeSession(String sessionUuid, boolean resume, ClaudeCodeSessionManager manager,
			SessionParameters parameters) {
		if (sessionUuid == null || sessionUuid.isBlank())
			throw new IllegalArgumentException("Session UUID must not be null or blank");
		Objects.requireNonNull(parameters, "session parameters must not be null");
		this.uuid = sessionUuid;
		this.resume = resume;
		this.manager = manager;
		this.parameters = parameters;
	}

	public long getRemainingTtlMinutes() {
		if (lastSentAt == null)
			return -1;
		long elapsed = ChronoUnit.MINUTES.between(lastSentAt, Instant.now());
		return Math.max(0, TTL_HOURS * 60 - elapsed);
	}

	public SessionState getState() {
		if (isExpired())
			return SessionState.EXPIRED;
		if (inPrompt)
			return SessionState.PROMPT;
		if (isProcessAlive())
			return SessionState.READY;
		return SessionState.CREATED;
	}

	private synchronized void start() throws IOException {
		if (process != null) {
			if (process.isAlive())
				return;
			terminate();
		}

		List<String> cmd = parameters.buildBaseCommand();
		if (resume)
			cmd.add("--resume");
		else
			cmd.add("--session-id");
		cmd.add(uuid);

		ProcessBuilder pb = new ProcessBuilder(cmd);
		parameters.buildEvironment(pb);
		pb.environment().put("MCPC_SESSION_ID", uuid);
		pb.redirectErrorStream(false);

		process = pb.start();
		stdin = JsonUtil.newWriter(process.getOutputStream());
		stdout = JsonUtil.newReader(process.getInputStream());
		stderr = JsonUtil.newReader(process.getErrorStream());
		startedAt = Instant.now();
		// after first start use resume
		resume = true;

		LOG.info("ClaudeCodeSession: CLI started, uuid=" + uuid + ", workDir=" + parameters.cwd);
		notifyChanged();
	}

	public synchronized void terminate() {
		if (process != null) {
			try {
				if (stdin != null)
					stdin.close();
			} catch (Exception ignored) {
			}
			process.destroy();
			process = null;
			stdin = null;
			stdout = null;
			stderr = null;

			if (mirror != null)
				try {
					mirror.close();
				} catch (IOException ignored) {
				} finally {
					mirror = null;
				}
			LOG.info("ClaudeCodeSession: terminated, uuid=" + uuid);
			notifyChanged();
		}
	}

	public synchronized void writeLine(String jsonLine) throws IOException {
		if (isExpired()) {
			if (isProcessAlive())
				terminate();
			throw new IllegalStateException("Session has expired and can no longer be used");
		}

		Objects.requireNonNull(jsonLine, "line to write must not be null");
		start(); // idempotent
		if (stdin == null)
			throw new IllegalStateException("STDIN unavailable after start(); process=" + process + ", uuid=" + uuid);

		mirrorLine(jsonLine);
		stdin.println(jsonLine);
		stdin.flush();
		lastSentAt = Instant.now();
		notifyChanged();
	}

	private void mirrorLine(String jsonLine) throws IOException {
		openMirrorIfNeeded();
		if (jsonLine != null && jsonLine.length() > 0) {
			mirror.write(jsonLine);
			mirror.write(System.lineSeparator());
			mirror.flush();
		}
	}

	public String readLine() {
		return readLine(stdout, "STDOUT");
	}

	public String readError() {
		return readLine(stderr, "STDERR");
	}

	private String readLine(BufferedReader reader, String channel) {
		// Fail fast instead of throwing a cryptic NullPointerException deep inside
		// readLine (the reported "reader is null" defect): reading before the CLI
		// process was started is always a programming error in the calling loop.
		if (reader == null)
			throw new IllegalStateException(
					"Cannot read " + channel + ": the CLI process is not started" + " (uuid=" + uuid + ", processAlive="
							+ isProcessAlive() + ")." + " A prompt must be sent (writeLine) before reading.");

		try {
			var line = reader.readLine();
			mirrorLine(line);

			return line;
		} catch (IOException e) {
			throw new IllegalStateException("Failed to read " + channel + " for session " + uuid, e);
		}
	}

	private void openMirrorIfNeeded() {
		if (mirror != null)
			return;
		// Guard against ever producing a "null.json" (or similar) mirror file.
		if (uuid == null || uuid.isBlank())
			throw new IllegalStateException("Refusing to create a mirror file: session UUID is null/blank");
		File filePath = null;
		try {
			var di = parameters.cwd.resolve(".claude/logs/");
			Files.createDirectories(di);
			filePath = di.resolve(uuid + ".json").toFile();
			mirror = JsonUtil.newWriter(new FileOutputStream(filePath, true), false);
			LOG.info("Created mirror file: " + filePath);
		} catch (IOException e) {
			LOG.error("ClaudeCodeConnector: cannot open mirror file: " + filePath, e);
			throw new IllegalStateException("Cannot open mirror file: " + filePath, e);
		}
	}

	public String getSessionUuid() {
		return uuid;
	}

	public Instant getLastSentAt() {
		return lastSentAt;
	}

	public String getLastParsedMessage() {
		return lastParsedMessage;
	}

	public void setLastParsedMessage(String msg) {
		this.lastParsedMessage = msg != null ? msg.replace('\n', ' ').strip() : "empty";
		notifyChanged();
	}

	public boolean isExpired() {
		if (lastSentAt == null)
			return false;
		return Instant.now().isAfter(lastSentAt.plus(TTL_HOURS, ChronoUnit.HOURS));
	}

	private boolean isProcessAlive() {
		return process != null && process.isAlive();
	}

	public void setInPrompt(boolean inPrompt) {
		if (this.inPrompt == inPrompt)
			return;
		this.inPrompt = inPrompt;
		notifyChanged();
	}

	public SessionParameters getParameters() {
		return parameters;
	}

	private void notifyChanged() {
		manager.onSessionChanged(this);
	}

	public String getID() {
		if (uuid != null)
			return uuid;
		return getParameters().getHash();
	}
}
