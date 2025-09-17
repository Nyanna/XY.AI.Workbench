package xy.ai.workbench.views.explorer;

import java.io.*;
import org.eclipse.core.resources.*;
import org.eclipse.ui.model.WorkbenchLabelProvider;

import xy.ai.workbench.LOG;

public class MarkdownLabelProvider extends WorkbenchLabelProvider {

	@Override
	protected String decorateText(String input, Object element) {
		if (element instanceof IFile) {
			IFile file = (IFile) element;
			if (isMarkdownFile(file)) {
				String title = getFirstLineFromFile(file);
				if (title != null)
					return title;
			}
		}

		return super.decorateText(input, element);
	}

	private boolean isMarkdownFile(IFile file) {
		return file.getFileExtension() != null && (file.getFileExtension().equalsIgnoreCase("md"));
	}

	private String getFirstLineFromFile(IFile file) {
		try (BufferedReader reader = new BufferedReader(new InputStreamReader(file.getContents(), file.getCharset()))) {

			String firstLine = reader.readLine();
			if (firstLine != null && firstLine.startsWith("#")) {
				firstLine = firstLine.replaceFirst("^#+\\s*", ""); // Remove headers
				firstLine = firstLine.replaceAll("\\*\\*(.+?)\\*\\*", "$1"); // Remove bold
				firstLine = firstLine.replaceAll("\\*(.+?)\\*", "$1"); // Remove italic
				return firstLine.trim();
			}

		} catch (Exception e) {
			LOG.info(e.getMessage(), e);
		}
		return null;
	}
}