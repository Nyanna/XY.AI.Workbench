package xy.ai.workbench.connector.harness;

import java.lang.ref.WeakReference;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import xy.ai.workbench.commands.Command;

public class PromptArguments {
	public List<String> inputs = new ArrayList<>();
	public String absoluteFilePath;
	public Path project;
	public Command command;
	public String yaml;
	// Editor active when the prompt was built; used as a hint for tag replacement.
	private WeakReference<Object> editor;
	public boolean processorEnabled;

	public void setEditor(Object editor) {
		this.editor = editor != null ? new WeakReference<>(editor) : null;
	}

	public Object getEditor() {
		return editor != null ? editor.get() : null;
	}
}