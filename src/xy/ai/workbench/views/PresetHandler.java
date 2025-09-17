package xy.ai.workbench.views;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;

import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IFolder;
import org.eclipse.core.resources.IProject;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.jface.viewers.Viewer;
import org.eclipse.jface.viewers.ViewerFilter;
import org.eclipse.swt.SWT;
import org.eclipse.swt.widgets.FileDialog;
import org.eclipse.swt.widgets.Shell;
import org.eclipse.ui.dialogs.ElementTreeSelectionDialog;
import org.eclipse.ui.model.WorkbenchLabelProvider;

public class PresetHandler {
	private static final String PROMPT_TXT = ".prompt.txt";

	public static String readStringFromFile(Shell shell) {
		ElementTreeSelectionDialog dialog = new ElementTreeSelectionDialog(shell, new WorkbenchLabelProvider(),
				new FlatEndingContentProvider(PROMPT_TXT));

		dialog.setTitle("Prompt Preset");
		dialog.setMessage("Choose a prompt preset");
		dialog.setInput(ResourcesPlugin.getWorkspace().getRoot().getProjects());
		dialog.addFilter(new ViewerFilter() {
			@Override
			public boolean select(Viewer viewer, Object parent, Object element) {
				if (element instanceof IFile) {
					IFile file = (IFile) element;
					String fileExtension = file.getFileExtension();
					return "txt".equalsIgnoreCase(fileExtension);
				}
				if (element instanceof IFolder || element instanceof IProject)
					return true;
				return false;
			}
		});

		if (dialog.open() == ElementTreeSelectionDialog.OK) {
			Object result = dialog.getFirstResult();
			if (result instanceof IFile) {
				IFile file = (IFile) result;
				try (BufferedReader reader = new BufferedReader(new FileReader(file.getLocation().toFile()))) {
					StringBuilder sb = new StringBuilder();
					String line;
					while ((line = reader.readLine()) != null) {
						sb.append(line).append("\n");
					}
					return sb.toString();
				} catch (IOException e) {
					throw new IllegalStateException(e);
				}
			}
		}
		return null;
	}

	public static void writeStringToFile(String content, Shell shell) {
		FileDialog dialog = new FileDialog(shell, SWT.SAVE);
		dialog.setFilterPath(ResourcesPlugin.getWorkspace().getRoot().getProjects()[0].getFullPath().toOSString());
		dialog.setFilterExtensions(new String[] { "*.prompt.txt" });
		dialog.setFilterNames(new String[] { "Prompt Files (*prompt.txt)" });
		String filePath = dialog.open();
		if (filePath != null)
			try (PrintWriter writer = new PrintWriter(new FileWriter(filePath, StandardCharsets.UTF_8))) {
				writer.print(content);
			} catch (IOException e) {
				throw new IllegalStateException(e);
			}
	}
}
