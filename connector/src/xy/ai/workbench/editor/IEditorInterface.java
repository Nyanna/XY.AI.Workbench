package xy.ai.workbench.editor;

import java.util.regex.Pattern;

import xy.ai.workbench.api.IExecutor;
import xy.ai.workbench.api.IExecutor.IProgress;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;

public interface IEditorInterface {
	public static final String LINE_COMMENT = "#:";
	public static final String USER = "User:";
	public static final String AGENT = "Agent:";

	public static final String TEXT = "Text:";
	public static final String THINKING = "Thinking:";
	public static final String THINKING_META = "Thinking Meta:";
	public static final String TOOLUSE = "Tool:";
	public static final String TOOLRESULT = "ToolResult:";
	public static final String CONTROL_REQUEST = "Control Request:";

	public static final String AIREQ_PREFIX = "xy.ai.req";
	public static final Pattern PROMPT_TAG_PATTERN = Pattern.compile("\\[" + AIREQ_PREFIX + ":(.*):(.*)\\]", Pattern.CASE_INSENSITIVE);

	public void insertTag(IExecutor exec, IModelRequest req, IProgress mon);

	public void replaceTag(AIAnswer ans, IProgress mon);

	public static String getPromptTag(String meta, String id) {
		return String.format("[%s:%s:%s]", IEditorInterface.AIREQ_PREFIX, meta, id);
	}

	public static boolean containsPromptTag(String line) {
		return line != null && IEditorInterface.PROMPT_TAG_PATTERN.matcher(line).find();
	}
}
