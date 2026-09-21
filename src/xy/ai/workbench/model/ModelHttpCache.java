package xy.ai.workbench.model;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.time.Instant;

import xy.ai.workbench.Activator;
import xy.ai.workbench.LOG;

/** File cache for provider HTTP model-list responses, valid for 7 days based on the cache file's last-modified time. */
public class ModelHttpCache {

	private static final Duration MAX_AGE = Duration.ofDays(7);

	public interface Fetcher {
		String fetch() throws Exception;
	}

	/** Returns the cached body for {@code fileName} if still fresh, otherwise fetches, caches and returns it. */
	static String get(String fileName, Fetcher fetcher) throws Exception {
		Path file = Activator.getDefault().getStateLocation().append(fileName).toFile().toPath();
		String cached = readFresh(file);
		if (cached != null)
			return cached;
		String body = fetcher.fetch();
		try {
			Files.writeString(file, body, StandardCharsets.UTF_8);
		} catch (IOException e) {
			LOG.error("Unable to cache " + fileName, e);
		}
		return body;
	}

	private static String readFresh(Path file) {
		try {
			if (!Files.isRegularFile(file))
				return null;
			Instant modified = Files.getLastModifiedTime(file).toInstant();
			if (Duration.between(modified, Instant.now()).compareTo(MAX_AGE) > 0)
				return null;
			return Files.readString(file, StandardCharsets.UTF_8);
		} catch (IOException e) {
			LOG.error("Unable to read cache " + file, e);
			return null;
		}
	}
}
