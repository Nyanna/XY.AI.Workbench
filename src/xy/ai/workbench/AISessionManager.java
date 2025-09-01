package xy.ai.workbench;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.stream.Collectors;

import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.texteditor.ITextEditor;

import com.openai.models.ChatModel;

public class AISessionManager {
	private SessionConfig cfg = new SessionConfig();

	List<Consumer<SessionConfig>> systemPromptObs = new ArrayList<>();
	List<Consumer<AIAnswer>> answerObs = new ArrayList<>();

	public void setKey(String key) {
		cfg.setKey(key);
	}

	public void setMaxOutputTokens(Long maxOutputTokens) {
		cfg.setMaxOutputTokens(maxOutputTokens);
	}

	public void setTemperature(Double temperature) {
		cfg.setTemperature(temperature);
	}

	public void setTopP(Double topP) {
		cfg.setTopP(topP);
	}

	public void setModel(ChatModel model) {
		cfg.setModel(model);
	}

	public void setSystemPrompt(String[] systemPrompt) {
		cfg.setSystemPrompt(systemPrompt);
		systemPromptObs.forEach(c -> c.accept(cfg));
	}

	public String getKey() {
		return cfg.getKey();
	}

	public Long getMaxOutputTokens() {
		return cfg.getMaxOutputTokens();
	}

	public Double getTemperature() {
		return cfg.getTemperature();
	}

	public Double getTopP() {
		return cfg.getTopP();
	}

	public ChatModel getModel() {
		return cfg.getModel();
	}

	public String[] getSystemPrompt() {
		return cfg.getSystemPrompt();
	}

	public void addSystemPromptObs(Consumer<SessionConfig> obs, boolean initialize) {
		systemPromptObs.add(obs);
		if (initialize)
			obs.accept(cfg);
	}

	public void addAnswerObs(Consumer<AIAnswer> obs) {
		answerObs.add(obs);
	}

	public String[] getModels() {

		ChatModel[] models = new ChatModel[] { ChatModel.GPT_5_NANO, ChatModel.GPT_5_MINI, ChatModel.GPT_5 };
		String[] options = Arrays.stream(models).map((m) -> m.asString()).collect(Collectors.toList())
				.toArray(new String[0]);
		return options;
	}

	public void execute() {
		System.out.println("Starting Call");

		var active = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage().getActiveEditor();
		if (!(active instanceof ITextEditor))
			return;

		ITextEditor editor = (ITextEditor) active;

		IDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());

		String input = null;
		ISelection selection = editor.getSelectionProvider().getSelection();
		ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
		if (tsel != null && !tsel.isEmpty()) {
			input = tsel.getText();
		}

		if (input == null || input.isBlank()) {
			input = doc.get();
		}

		if (input == null || input.isBlank()) {
			return;
		}

		answerObs.forEach(c -> c.accept(null));
		AIAnswer res = new OpenAPIConnector(cfg).sendRequest(input);
		answerObs.forEach(c -> c.accept(res));

		try {
			String replace = "\n" + res.answer;
			doc.replace(doc.getLength(), 0, replace);
		} catch (BadLocationException e) {
			System.out.println("Error adding text");
		}
	}
}
