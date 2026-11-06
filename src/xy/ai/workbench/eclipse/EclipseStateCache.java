package xy.ai.workbench.eclipse;

import java.io.File;

import xy.ai.conscious.api.ext.IFileCache;
import xy.ai.workbench.Activator;

public class EclipseStateCache implements IFileCache {
	@Override
	public File getFile(String fileName) {
		return Activator.getDefault().getStateLocation().append(fileName).toFile();
	}
}
