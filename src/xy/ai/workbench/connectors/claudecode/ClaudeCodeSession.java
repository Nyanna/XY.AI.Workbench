package xy.ai.workbench.connectors.claudecode;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.nio.file.Files;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.List;
import java.util.UUID;

import xy.ai.workbench.LOG;
import xy.ai.workbench.Reasoning;

public class ClaudeCodeSession {
	private static final long TTL_HOURS = 1;

	/**
	 * The Claude session UUID. {@code null} until the process is started for the
	 * first time (or until a UUID is pre-assigned via {@link #assignUuid}).
	 */
	private String uuid;
	private SessionParameters parameters;

	private Process process;
	private PrintWriter stdin;
	private BufferedReader stdout;
	private BufferedReader stderr;
	private FileWriter mirror;

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

	private final ClaudeCodeSessionManager manager;

	public ClaudeCodeSession(ClaudeCodeSessionManager manager, SessionParameters parameters) {
		this(null, manager, parameters);
	}

	public ClaudeCodeSession(String sessionUuid, ClaudeCodeSessionManager manager, SessionParameters parameters) {
		this.uuid = sessionUuid;
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
		if (uuid == null) {
			uuid = UUID.randomUUID().toString();
			cmd.add("--session-id");
		} else
			cmd.add("--resume");
		cmd.add(uuid);

		ProcessBuilder pb = new ProcessBuilder(cmd);
		pb.directory(this.parameters.cwd.toFile());
		pb.redirectErrorStream(false);
		pb.environment().put("CLAUDE_CODE_DISABLE_SPELLCHECK", "true");
		if (Reasoning.Disabled == parameters.reasoning)
			pb.environment().put("MAX_THINKING_TOKENS", "0");

		process = pb.start();
		stdin = new PrintWriter(process.getOutputStream());
		stdout = new BufferedReader(new InputStreamReader(process.getInputStream()));
		stderr = new BufferedReader(new InputStreamReader(process.getErrorStream()));
		startedAt = Instant.now();

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

		start(); // idempotent

		stdin.println(jsonLine);
		stdin.flush();
		lastSentAt = Instant.now();
		notifyChanged();
	}

	public String readLine() {
		return readLine(stdout);
	}

	public String readError() {
		return readLine(stderr);
	}

	private String readLine(BufferedReader reader) {
		if (mirror == null) {
			File filePath = null;
			try {
				var di = parameters.cwd.resolve(".claude/logs/");
				Files.createDirectories(di);
				filePath = di.resolve(uuid + ".json").toFile();
				mirror = new FileWriter(filePath, true);
				LOG.info("Created mirror file: " + filePath);
			} catch (IOException e) {
				LOG.error("ClaudeCodeConnector: cannot open mirror file: " + filePath, e);
				throw new IllegalStateException(e);
			}
		}

		try {
			var line = reader.readLine();

			if (line != null && line.length() > 0) {
				mirror.write(line);
				mirror.write(System.lineSeparator());
				mirror.flush();
			}

			return line;
		} catch (IOException e) {
			throw new IllegalStateException(e);
		}
	}

	public String getSessionUuid() {
		return uuid;
	}

	public void setSessionUuid(String sessionUuid) {
		this.uuid = sessionUuid;
		notifyChanged();
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
