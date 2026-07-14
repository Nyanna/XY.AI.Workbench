package xy.ai.workbench.connectors.claudecode;

import java.nio.file.Path;

/** The project's root directory and the relative path of the focused file. */
public class EditorLocation {
	public Path projectPath;
	public String relativeFilePath;

	public void set(Path projectPath, String relativeFilePath) {
		this.projectPath = projectPath;
		this.relativeFilePath = relativeFilePath;
	}
}