package xy.ai.workbench;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.stream.Collectors;

import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.DocumentEvent;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IDocumentListener;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.texteditor.IDocumentProvider;
import org.eclipse.ui.texteditor.ITextEditor;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchPart;
import org.eclipse.ui.IWorkbenchPartReference;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IPartListener2;
import org.eclipse.jface.viewers.ISelectionChangedListener;
import org.eclipse.jface.viewers.ISelectionProvider;
import org.eclipse.jface.viewers.SelectionChangedEvent;

import com.openai.models.ChatModel;

import xy.ai.workbench.editors.AISessionEditor;

public class AISessionManager {
	private ActiveEditorListener editorListener = new ActiveEditorListener();

	private SessionConfig cfg = new SessionConfig();
	private int[] inputStats = new int[InputMode.values().length];

	List<Consumer<SessionConfig>> systemPromptObs = new ArrayList<>();
	List<Consumer<AIAnswer>> answerObs = new ArrayList<>();
	List<Consumer<int[]>> inputStatObs = new ArrayList<>();
	List<Consumer<boolean[]>> inputObs = new ArrayList<>();

	public void initializeInputs() {
		for (var mode : InputMode.values())
			updateInputStat(mode);

		IWorkbenchPage activePage = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();
		if (activePage != null)
			activePage.addPartListener(editorListener);
	}

	public void clearObserver() {
		systemPromptObs.clear();
		answerObs.clear();
		inputStatObs.clear();
		inputObs.clear();
	}

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
		updateInputStat(InputMode.Instructions);
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

	public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {
		inputStatObs.add(obs);
		if (initialize)
			obs.accept(inputStats);
	}

	public void addAnswerObs(Consumer<AIAnswer> obs) {
		answerObs.add(obs);
	}

	public void addInputObs(Consumer<boolean[]> obs, boolean initialize) {
		inputObs.add(obs);
		if (initialize)
			obs.accept(cfg.inputModes);
	}

	public OutputMode getOuputMode() {
		return cfg.ouputMode;
	}

	public void setOuputMode(OutputMode ouputMode) {
		cfg.ouputMode = ouputMode;
	}

	public boolean isInputEnabled(InputMode mode) {
		return cfg.isInputEnabled(mode);
	}

	public void setInputMode(InputMode mode, boolean enable) {
		cfg.setInputMode(mode, enable);

		if (InputMode.Selection.equals(mode) && enable) {
			cfg.setInputMode(InputMode.Editor, false);
		} else if (InputMode.Editor.equals(mode) && enable) {
			cfg.setInputMode(InputMode.Selection, false);
		}

		inputObs.forEach(c -> c.accept(cfg.inputModes));
		updateInputStat(mode);
	}

	private void updateInputStat(InputMode mode) {
		inputStats[mode.ordinal()] = getInput(mode).length();
		inputStatObs.forEach(c -> c.accept(inputStats));
	}

	public String[] getModels() {
		ChatModel[] models = new ChatModel[] { ChatModel.GPT_5_NANO, ChatModel.GPT_5_MINI, ChatModel.GPT_5 };
		String[] options = Arrays.stream(models).map((m) -> m.asString()).collect(Collectors.toList())
				.toArray(new String[0]);
		return options;
	}

	private String getInput(InputMode mode) {
		var active = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage().getActiveEditor();
		switch (mode) {
		case Instructions:
			String systemPrompt = Arrays.stream(cfg.systemPrompt).filter(e -> !e.startsWith("#"))
					.collect(Collectors.joining(", "));
			return systemPrompt;
		case Selection:
			if (active instanceof ITextEditor) {
				ITextEditor editor = (ITextEditor) active;
				ISelection selection = editor.getSelectionProvider().getSelection();
				ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
				if (tsel != null)
					return tsel.getText();
			}
			break;
		case Editor:
			if (active instanceof ITextEditor) {
				ITextEditor editor = (ITextEditor) active;
				IDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());
				return doc.get();
			}
		case Files:
			// TODO implement files
			return "";
		}
		return "";
	}

	public void execute() {
		System.out.println("Starting Call");

		String input = "";

		if (isInputEnabled(InputMode.Editor))
			input += getInput(InputMode.Editor);

		else if (isInputEnabled(InputMode.Selection))
			input += getInput(InputMode.Selection);

		String systemPrompt = isInputEnabled(InputMode.Instructions) ? getInput(InputMode.Instructions) : "";

		if (input == null || input.isBlank()) {
			System.out.println("Input Empty");
			return;
		}

		answerObs.forEach(c -> c.accept(null));
		AIAnswer res = new OpenAPIConnector(cfg).sendRequest(input, systemPrompt);
		answerObs.forEach(c -> c.accept(res));

		try {
			var active = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage().getActiveEditor();

			if (active instanceof ITextEditor) {
				ITextEditor editor = (ITextEditor) active;
				IDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());
				ISelection selection = editor.getSelectionProvider().getSelection();
				ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;

				switch (cfg.ouputMode) {
				case Append:
					String replace = "\n" + res.answer;
					doc.replace(doc.getLength(), 0, replace);
					break;
				case Replace:
					if (tsel != null)
						doc.replace(tsel.getOffset(), tsel.getLength(), res.answer);
					break;
				case Cursor:
					if (tsel != null)
						doc.replace(tsel.getOffset(), 0, res.answer);
					break;
				}
			}
		} catch (BadLocationException e) {
			System.out.println("Error adding text");
		}
	}

	public void queue() {
		// TODO implement batch managing
	}

	public class ActiveEditorListener implements IPartListener2 {
		private SelectionListener selectionListener = new SelectionListener();
		private IDocumentListener documentListener = new DocumentListener();
		private ITextEditor textEditor;

		@Override
		public void partActivated(IWorkbenchPartReference partRef) {
			// remove old
			if (textEditor != null) {
				ISelectionProvider selectionProvider = textEditor.getSelectionProvider();
				if (selectionProvider != null)
					selectionProvider.removeSelectionChangedListener(selectionListener);

				IDocumentProvider documentProvider = textEditor.getDocumentProvider();
				if (documentProvider != null) {
					IDocument doc = documentProvider.getDocument(textEditor.getEditorInput());
					if (doc != null)
						doc.removeDocumentListener(documentListener);
				}
			}

			IEditorPart editor = null;
			IWorkbenchPart part = partRef.getPart(false);
			if (part instanceof IEditorPart) {
				editor = (IEditorPart) partRef.getPart(false);
			} else if (part instanceof AISessionEditor) {
				editor = ((AISessionEditor) part).getEditor();
			}

			if (editor instanceof ITextEditor) {

				textEditor = (ITextEditor) editor;
				textEditor.getSelectionProvider().addSelectionChangedListener(selectionListener);

				updateInputStat(InputMode.Editor);

				IDocumentProvider documentProvider = textEditor.getDocumentProvider();
				if (documentProvider != null) {
					IDocument doc = documentProvider.getDocument(textEditor.getEditorInput());
					if (doc != null)
						doc.addDocumentListener(documentListener);
				}
			}
		}

		public class DocumentListener implements IDocumentListener {
			@Override
			public void documentAboutToBeChanged(DocumentEvent event) {
			}

			@Override
			public void documentChanged(DocumentEvent event) {
				updateInputStat(InputMode.Editor);
			}
		}

		public class SelectionListener implements ISelectionChangedListener {
			@Override
			public void selectionChanged(SelectionChangedEvent event) {
				updateInputStat(InputMode.Selection);
			}
		}

		@Override
		public void partBroughtToTop(IWorkbenchPartReference partRef) {
		}

		@Override
		public void partClosed(IWorkbenchPartReference partRef) {
		}

		@Override
		public void partDeactivated(IWorkbenchPartReference partRef) {
		}

		@Override
		public void partOpened(IWorkbenchPartReference partRef) {
		}

		@Override
		public void partHidden(IWorkbenchPartReference partRef) {
		}

		@Override
		public void partVisible(IWorkbenchPartReference partRef) {
		}

		@Override
		public void partInputChanged(IWorkbenchPartReference partRef) {
		}
	}
}
