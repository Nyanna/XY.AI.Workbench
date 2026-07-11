package xy.ai.workbench.connectors.claudecode;

import java.io.BufferedReader;
import java.io.IOException;
import java.time.Duration;

public class TimedLineReader {
	static final Duration CHAR_TIMEOUT = Duration.ofSeconds(10);
	private static final long POLL_INTERVAL_MS = 20;

	private final BufferedReader delegate;

	TimedLineReader(BufferedReader delegate) {
		this.delegate = delegate;
	}

	public String readLine() throws IOException {
		StringBuilder line = new StringBuilder();
		boolean started = false;

		while (true) {
			int c = started ? readCharWithTimeout(line) : delegate.read(); // first char: block indefinitely

			if (c == -1)
				return started ? line.toString() : null;

			started = true;
			if (c == '\n')
				return line.toString();
			if (c != '\r') // tolerate CRLF line endings
				line.append((char) c);
		}
	}

	private int readCharWithTimeout(StringBuilder lineSoFar) throws IOException {
		long deadline = System.currentTimeMillis() + CHAR_TIMEOUT.toMillis();
		while (!delegate.ready()) {
			long remaining = deadline - System.currentTimeMillis();
			if (remaining <= 0)
				throw new IOException("Timed out after " + CHAR_TIMEOUT.getSeconds()
						+ "s waiting for the next character; partial line so far: '" + lineSoFar + "'");
			try {
				Thread.sleep(Math.min(POLL_INTERVAL_MS, remaining));
			} catch (InterruptedException e) {
				Thread.currentThread().interrupt();
				throw new IOException(
						"Interrupted while waiting for the next character; partial line so far: '" + lineSoFar + "'",
						e);
			}
		}
		return delegate.read();
	}
}