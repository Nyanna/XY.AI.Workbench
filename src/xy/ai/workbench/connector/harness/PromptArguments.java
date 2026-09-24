package xy.ai.workbench.connector.harness;

import java.lang.ref.WeakReference;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.commands.Command;

public class PromptArguments {
	public List<String> inputs = new ArrayList<>();
	public String absoluteFilePath;
	public Path project;
	public Command command;
	public String yaml;
	// Editor active when the prompt was built; used as a hint for tag replacement.
	private WeakReference<ITextEditor> editor;
	public boolean processorEnabled;

	public void setEditor(ITextEditor editor) {
		this.editor = editor != null ? new WeakReference<>(editor) : null;
	}

	public ITextEditor getEditor() {
		return editor != null ? editor.get() : null;
	}
}