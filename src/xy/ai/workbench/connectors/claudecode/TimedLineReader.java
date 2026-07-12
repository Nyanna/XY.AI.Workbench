package xy.ai.workbench.connectors.claudecode;

import java.io.BufferedReader;
import java.io.IOException;
import java.time.Duration;

public class TimedLineReader {
	private static final Duration TIMEOUT = Duration.ofMillis(500);
	private static final Duration AFTER_START_TIMEOUT = Duration.ofSeconds(10);
	private static final long POLL_INTERVAL_MS = 250;

	private final BufferedReader delegate;
	private StringBuilder line = new StringBuilder();

	TimedLineReader(BufferedReader delegate) {
		this.delegate = delegate;
	}

	public String readLine() throws IOException {
		line.setLength(0);
		boolean started = false;

		while (true) {
			int c = started ? read(line, POLL_INTERVAL_MS, AFTER_START_TIMEOUT) : read(line, POLL_INTERVAL_MS, TIMEOUT);

			if (c == -1)
				return started ? line.toString() : null;

			started = true;
			if (c == '\n')
				return line.toString();
			if (c != '\r') // tolerate CRLF line endings
				line.append((char) c);
		}
	}

	private int read(StringBuilder line, long pollMs, Duration timeout) throws IOException {
		long deadline = System.currentTimeMillis() + timeout.toMillis();
		while (!delegate.ready()) {
			long remaining = deadline - System.currentTimeMillis();
			if (remaining <= 0)
				throw new IOException("Timed out after " + timeout.getSeconds()
						+ "s waiting for the next character; partial line so far: '" + line + "'");
			try {
				Thread.sleep(Math.min(pollMs, remaining));
			} catch (InterruptedException e) {
				Thread.currentThread().interrupt();
				throw new IOException(
						"Interrupted while waiting for the next character; partial line so far: '" + line + "'", e);
			}
		}
		return delegate.read();
	}
}