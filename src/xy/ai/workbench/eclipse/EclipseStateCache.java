package xy.ai.workbench.eclipse;

import java.io.File;

import xy.ai.workbench.Activator;
import xy.ai.workbench.api.ext.IFileCache;

public class EclipseStateCache implements IFileCache {
	@Override
	public File getFile(String fileName) {
		return Activator.getDefault().getStateLocation().append(fileName).toFile();
	}
}
