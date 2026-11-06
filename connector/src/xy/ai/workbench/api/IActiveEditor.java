package xy.ai.workbench.api;

import java.nio.file.Path;

import xy.ai.workbench.editor.IEditorInterface;

public interface IActiveEditor {

	Selection getSelection();

	Selection getFileContent();

	String resolveAbsoluteFilePath();

	Path resolveProjectPath();

	default Object getLastTextEditor() {
		return null;
	}

	public static String removeCommentLines(Selection input) {
		if (input == null || input.selection() == null)
			return null;
		StringBuffer result = new StringBuffer();
		for (String line : input.selection())
			if (!line.trim().startsWith(IEditorInterface.LINE_COMMENT))
				result.append(line).append(System.lineSeparator());
		return result.toString();
	}

	public record Selection(String[] selection, Integer cursorOffset) {
	}
}
