package xy.ai.conscious.api.ext;

import java.io.File;
import java.nio.file.Path;

public interface IFileCache {
	default File getFile(String fileName) {
		return Path.of(System.getProperty("java.io.tmpdir"), fileName).toFile();
	}
}
