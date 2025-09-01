package xy.ai.workbench;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.stream.Collectors;

import org.eclipse.core.runtime.NullProgressMonitor;
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
import org.eclipse.ui.IMemento;
import org.eclipse.ui.IPartListener2;
import org.eclipse.jface.viewers.ISelectionChangedListener;
import org.eclipse.jface.viewers.ISelectionProvider;
import org.eclipse.jface.viewers.SelectionChangedEvent;
import org.eclipse.swt.widgets.Display;

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
		switch (mode) {
		case Instructions:
			String systemPrompt = Arrays.stream(cfg.systemPrompt).filter(e -> !e.startsWith("#"))
					.collect(Collectors.joining(", "));
			return systemPrompt;
		case Selection:
			if (editorListener.textEditor != null) {
				ISelection selection = editorListener.textEditor.getSelectionProvider().getSelection();
				ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
				if (tsel != null)
					return tsel.getText();
			}
			break;
		case Editor:
			if (editorListener.textEditor != null) {
				IDocument doc = editorListener.textEditor.getDocumentProvider()
						.getDocument(editorListener.textEditor.getEditorInput());
				return doc.get();
			}
			break;
		case Files:
			// TODO implement files
			return "";
		}
		return "";
	}

	public void execute(Display display) {
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

		display.asyncExec(() -> processAnswer(res));
	}

	private void processAnswer(AIAnswer res) {
		try {

			IDocument doc = editorListener.textEditor.getDocumentProvider()
					.getDocument(editorListener.textEditor.getEditorInput());
			ISelection selection = editorListener.textEditor.getSelectionProvider().getSelection();
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
			editorListener.textEditor.doSave(new NullProgressMonitor());
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
		public ITextEditor textEditor;

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
			if (part instanceof AISessionEditor) {
				editor = ((AISessionEditor) part).getEditor();
			} else if (part instanceof IEditorPart) {
				editor = (IEditorPart) part;
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

	public void saveConfig(IMemento memento) {
		var m = memento.createChild("cfg");

		if (cfg.key != null)
			m.putString("key", cfg.key);
		if (cfg.maxOutputTokens != null)
			m.putString("maxOutputTokens", String.valueOf(cfg.maxOutputTokens));
		if (cfg.temperature != null)
			m.putString("temperature", String.valueOf(cfg.temperature));
		if (cfg.topP != null)
			m.putString("topP", String.valueOf(cfg.topP));
		if (cfg.model != null)
			m.putString("model", cfg.model.asString());
		int spLen = cfg.systemPrompt != null ? cfg.systemPrompt.length : 0;
		m.putInteger("systemPrompt.length", spLen);
		if (spLen > 0) {
			IMemento sp = m.createChild("systemPrompt");
			for (int i = 0; i < spLen; i++) {
				IMemento item = sp.createChild("item");
				item.putInteger("index", i);
				item.putString("value", cfg.systemPrompt[i]);
			}
		}
		if (cfg.ouputMode != null)
			m.putString("outputMode", cfg.ouputMode.name());
		int imLen = cfg.inputModes != null ? cfg.inputModes.length : 0;
		m.putInteger("inputModes.length", imLen);
		if (imLen > 0) {
			IMemento im = m.createChild("inputModes");
			for (int i = 0; i < imLen; i++) {
				IMemento item = im.createChild("item");
				item.putInteger("index", i);
				item.putString("value", Boolean.toString(cfg.inputModes[i]));
			}
		}
	}

	public void loadConfig(IMemento memento) {
		if (memento == null)
			return;
		var m = memento.getChild("cfg");
		if (m == null)
			return;

		cfg.key = m.getString("key");
		String maxTok = m.getString("maxOutputTokens");
		cfg.maxOutputTokens = maxTok != null ? Long.valueOf(maxTok) : null;
		String tmp = m.getString("temperature");
		cfg.temperature = tmp != null ? Double.valueOf(tmp) : null;
		String tp = m.getString("topP");
		cfg.topP = tp != null ? Double.valueOf(tp) : null;
		String mdl = m.getString("model");
		cfg.model = mdl != null ? ChatModel.of(mdl) : null;
		Integer spLen = m.getInteger("systemPrompt.length");
		int sLen = spLen != null ? spLen : 0;
		if (sLen > 0) {
			IMemento sp = m.getChild("systemPrompt");
			String[] arr = new String[sLen];
			if (sp != null) {
				IMemento[] items = sp.getChildren("item");
				for (IMemento it : items) {
					Integer idx = it.getInteger("index");
					String val = it.getString("value");
					if (idx != null && idx >= 0 && idx < sLen)
						arr[idx] = val;
				}
			}
			cfg.systemPrompt = arr;
		} else {
			cfg.systemPrompt = new String[0];
		}
		String om = m.getString("outputMode");
		cfg.ouputMode = om != null ? OutputMode.valueOf(om) : null;
		Integer imLen = m.getInteger("inputModes.length");
		int iLen = imLen != null ? imLen : 0;
		if (iLen > 0) {
			IMemento im = m.getChild("inputModes");
			boolean[] arr = new boolean[iLen];
			if (im != null) {
				IMemento[] items = im.getChildren("item");
				for (IMemento it : items) {
					Integer idx = it.getInteger("index");
					String val = it.getString("value");
					if (idx != null && idx >= 0 && idx < iLen)
						arr[idx] = Boolean.parseBoolean(val);
				}
			}
			cfg.inputModes = arr;
		} else {
			cfg.inputModes = new boolean[0];
		}
	}
}
