package xy.ai.workbench.connectors.claudecode;

import java.io.BufferedReader;
import java.io.IOException;
import java.nio.CharBuffer;
import java.time.Duration;
import java.util.Objects;
import java.util.concurrent.TimeoutException;

public class TimedLineReader {
	private static final Duration TIMEOUT = Duration.ofMillis(500);
	private static final Duration AFTER_START_TIMEOUT = Duration.ofSeconds(10);
	private static final long POLL_INTERVAL_MS = 250;
	private static final int BUFFER_SIZE = 4 * 1024 * 1024; // 4 MB

	private BufferedReader delegate;
	private final StringBuilder pending = new StringBuilder();
	private final CharBuffer buffer = CharBuffer.allocate(BUFFER_SIZE);
	private boolean eof = false;

	TimedLineReader(BufferedReader delegate) {
		Objects.requireNonNull(delegate);
		this.delegate = delegate;
	}

	public String readLine() throws IOException {
		while (true) {
			int nl = pending.indexOf("\n");
			if (nl >= 0) {
				String result = pending.substring(0, nl);
				pending.delete(0, nl + 1);
				if (result.endsWith("\r")) // tolerate CRLF line endings
					result = result.substring(0, result.length() - 1);
				return result;
			}

			if (eof) { // return remaining
				if (pending.length() > 0) {
					String result = pending.toString();
					pending.setLength(0);
					return result;
				}
				// once return null
				if (delegate != null) {
					delegate = null;
					return null;
				}
				throw new IOException("Tried to read after EOF");
			}

			boolean started = pending.length() > 0;
			if (started) {
				try {
					fill(AFTER_START_TIMEOUT);
				} catch (TimeoutException e) {
					throw new IOException("Stuck after started", e);
				}
			} else {
				try {
					fill(TIMEOUT);
				} catch (TimeoutException e) {
					// no message by now
					return null;
				}
			}
		}
	}

	/**
	 * Waits (with the given timeout) until at least one character is ready, then
	 * reads all currently available characters into {@link #pending} in as few
	 * calls as possible.
	 * 
	 * @throws TimeoutException
	 */
	private void fill(Duration timeout) throws IOException, TimeoutException {
		long deadline = System.currentTimeMillis() + timeout.toMillis();
		while (!delegate.ready()) {
			long remaining = deadline - System.currentTimeMillis();
			if (remaining <= 0)
				throw new TimeoutException("Timed out after " + timeout.getSeconds()
						+ "s waiting for the next character; partial line so far: '" + pending + "'");
			try {
				Thread.sleep(Math.min(POLL_INTERVAL_MS, remaining));
			} catch (InterruptedException e) {
				Thread.currentThread().interrupt();
				throw new IOException(
						"Interrupted while waiting for the next character; partial line so far: '" + pending + "'", e);
			}
		}

		do {
			buffer.clear();
			int n = delegate.read(buffer);
			if (n == -1) {
				eof = true;
				break;
			}
			buffer.flip();
			pending.append(buffer);
		} while (delegate.ready());
	}
}
