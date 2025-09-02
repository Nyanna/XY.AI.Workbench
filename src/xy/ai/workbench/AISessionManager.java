package xy.ai.workbench;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.stream.Collectors;

import org.eclipse.core.resources.IFile;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.IStatus;
import org.eclipse.core.runtime.NullProgressMonitor;
import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.Document;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.TreeSelection;
import org.eclipse.search.ui.ISearchQuery;
import org.eclipse.search.ui.ISearchResult;
import org.eclipse.search.ui.ISearchResultListener;
import org.eclipse.search.ui.NewSearchUI;
import org.eclipse.search.ui.SearchResultEvent;
import org.eclipse.search.ui.text.AbstractTextSearchResult;
import org.eclipse.search.ui.text.Match;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IMemento;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.texteditor.ITextEditor;

import com.openai.models.ChatModel;
import com.openai.models.responses.ResponseCreateParams;

import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.tools.AbstractQueryListener;

public class AISessionManager {
	private ActiveEditorListener editorListener = new ActiveEditorListener(this);

	private SessionConfig cfg = new SessionConfig();
	OpenAIConnector connector = new OpenAIConnector(cfg);
	private int[] inputStats = new int[InputMode.values().length];

	private List<Consumer<SessionConfig>> systemPromptObs = new ArrayList<>();
	private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();
	private List<Consumer<int[]>> inputStatObs = new ArrayList<>();
	private List<Consumer<boolean[]>> inputObs = new ArrayList<>();

	private List<IFile> selectedFiles = List.of();
	private ISearchResult result = null;

	public void initializeInputs() {
		for (var mode : InputMode.values())
			updateInputStat(mode);

		IWorkbenchWindow window = PlatformUI.getWorkbench().getActiveWorkbenchWindow();
		if (window != null) {

			SearchResultListener resObs = new SearchResultListener();
			NewSearchUI.addQueryListener(new AbstractQueryListener() {
				@Override
				public void queryAdded(ISearchQuery query) {
					query.getSearchResult().addListener(resObs);
				}
			});

			IWorkbenchPage activePage = window.getActivePage();
			if (activePage != null) {
				activePage.addPartListener(editorListener);

				activePage.addSelectionListener("org.eclipse.ui.navigator.ProjectExplorer", (part, selection) -> {
					if (selection instanceof TreeSelection) {
						selectedFiles = ((TreeSelection) selection).stream().filter(o -> o instanceof IFile)
								.map(obj -> (IFile) obj).collect(Collectors.toList());
						updateInputStat(InputMode.Files);
					}
				});
			}
		}
	}

	public class SearchResultListener implements ISearchResultListener {
		@Override
		public void searchResultChanged(SearchResultEvent e) {
			result = e.getSearchResult();
			System.out.println("Searchresult changed: " + result.getLabel());
			Display.getDefault().asyncExec(() -> updateInputStat(InputMode.Search));
		}
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
			cfg.setInputMode(InputMode.Current_line, false);
			cfg.setInputMode(InputMode.Editor, false);
		} else if (InputMode.Editor.equals(mode) && enable) {
			cfg.setInputMode(InputMode.Current_line, false);
			cfg.setInputMode(InputMode.Selection, false);
		} else if (InputMode.Current_line.equals(mode) && enable) {
			cfg.setInputMode(InputMode.Editor, false);
			cfg.setInputMode(InputMode.Selection, false);
		}

		inputObs.forEach(c -> c.accept(cfg.inputModes));
		updateInputStat(mode);
	}

	public void updateInputStat(InputMode mode) {
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
		ITextEditor textEditor = editorListener.getLastTextEditor();

		switch (mode) {
		case Instructions:
			String systemPrompt = Arrays.stream(cfg.systemPrompt).filter(e -> !e.startsWith("#"))
					.collect(Collectors.joining(", "));
			return systemPrompt;
		case Selection:
			if (textEditor != null) {
				ISelection selection = textEditor.getSelectionProvider().getSelection();
				ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
				if (tsel != null)
					return tsel.getText();
			}
			break;
		case Editor:
			if (textEditor != null) {
				IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
				return doc.get();
			}
			break;
		case Current_line:
			if (textEditor != null) {
				ISelection selection = textEditor.getSelectionProvider().getSelection();
				ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
				if (tsel != null) {
					int line = tsel.getEndLine();
					IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
					try {
						IRegion lineInfo = doc.getLineInformation(line);
						return doc.get(lineInfo.getOffset(), lineInfo.getLength());
					} catch (BadLocationException e1) {
						e1.printStackTrace();
					}
				}
			}
			break;
		case Files:
			return getFilsAsString(selectedFiles);
		case Search:
			// TODO use throttler for update call
			if (result instanceof AbstractTextSearchResult) {
				AbstractTextSearchResult textRes = (AbstractTextSearchResult) result;
				List<IFile> files = Arrays.stream(textRes.getElements()) //
						.filter(e -> e instanceof IFile) //
						.map(e -> (IFile) e)//
						.collect(Collectors.toList());

				List<Match> matches = files.stream() //
						.flatMap(f -> Arrays.stream(textRes.getMatches(f))) //
						.collect(Collectors.toList());

				String lines = matches.stream().map(m -> {
					try {
						return getLineFromFileMatch(m);
					} catch (BadLocationException | CoreException e1) {
						e1.printStackTrace();
						return "";
					}
				}).collect(Collectors.joining("\n"));

				return lines;
			}
			break;
		}
		return "";
	}

	private String getLineFromFileMatch(Match match) throws BadLocationException, CoreException {
		IFile file = (IFile) match.getElement();
		String fileContent = file.readString();

		IDocument doc = new Document(fileContent);
		int lineNumber = doc.getLineOfOffset(match.getOffset());
		int lineOffset = doc.getLineOffset(lineNumber);
		int lineLength = doc.getLineLength(lineNumber);
		return doc.get(lineOffset, lineLength);
	}

	private String getFilsAsString(List<IFile> files) {
		StringBuilder fullContent = new StringBuilder();
		for (IFile file : files) {
			try {
				String content = file.readString();
				fullContent.append(content).append("\n");
			} catch (CoreException e) {
				System.err.println("Error on reading " + file.getName() + ": " + e.getMessage());
			}
		}
		return fullContent.toString();
	}

	public void execute(Display display) {
		Job submit = new Job("Starting prompt...") {
			protected IStatus run(IProgressMonitor monitor) {
				monitor.beginTask("Executing prompt...", IProgressMonitor.UNKNOWN);
				try {
					monitor.worked(1);
					var req = prepareInner(display);
					monitor.worked(1);
					var ans = executeInner(display, req);
					monitor.worked(1);
					processAnswer(display, ans);
					monitor.worked(1);
				} catch (Exception e) {
					e.printStackTrace();
					return Status.CANCEL_STATUS;
				} finally {
					monitor.done();
				}
				return Status.OK_STATUS;
			}
		};
		submit.schedule();
	}

	public void queue(Display display, AIBatchManager batch) {
		Job submit = new Job("Starting prompt creation...") {
			protected IStatus run(IProgressMonitor monitor) {
				monitor.beginTask("Batching prompt...", IProgressMonitor.UNKNOWN);
				try {
					monitor.worked(1);
					var req = prepareInner(display);
					monitor.worked(1);
					batch.enqueue(req);
					monitor.worked(1);
				} catch (Exception e) {
					e.printStackTrace();
					return Status.CANCEL_STATUS;
				} finally {
					monitor.done();
				}
				return Status.OK_STATUS;
			}
		};
		submit.schedule();
	}

	private String input;

	private ResponseCreateParams prepareInner(Display display) {
		System.out.println("Preparing Call");

		input = "";
		display.syncExec(() -> {
			if (isInputEnabled(InputMode.Editor))
				input += getInput(InputMode.Editor);
			else if (isInputEnabled(InputMode.Selection))
				input += getInput(InputMode.Selection);
			else if (isInputEnabled(InputMode.Current_line))
				input += getInput(InputMode.Current_line);
		});

		String systemPrompt = isInputEnabled(InputMode.Instructions) ? getInput(InputMode.Instructions) : "";

		if ((input == null || input.isBlank()) && systemPrompt.isBlank())
			throw new IllegalArgumentException("Input and instructions Empty");

		if (editorListener.getLastTextEditor() == null)
			throw new IllegalArgumentException("Result editor unset");

		List<String> tools = new ArrayList<String>();
		if (isInputEnabled(InputMode.Files))
			tools.addAll(selectedFiles.stream().map(f -> {
				try {
					return f.readString();
				} catch (CoreException e) {
					e.printStackTrace();
					return "";
				}
			}).collect(Collectors.toList()));

		if (isInputEnabled(InputMode.Search)) {
			String search = getInput(InputMode.Search);
			if (search != null && !search.isBlank())
				tools.add(search);
		}

		System.out.println("Input prepared");

		ResponseCreateParams req = connector.createRequest(//
				input, //
				systemPrompt, //
				tools //
		);
		return req;
	}

	private AIAnswer executeInner(Display display, ResponseCreateParams req) {
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));
		AIAnswer res = connector.executeRequest(req);
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));
		return res;
	}

	private void processAnswer(Display display, AIAnswer res) {
		display.asyncExec(() -> {
			try {
				ITextEditor textEditor = editorListener.getLastTextEditor();

				IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
				ISelection selection = textEditor.getSelectionProvider().getSelection();
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
				textEditor.doSave(new NullProgressMonitor());
			} catch (BadLocationException e) {
				System.out.println("Error adding text");
			}
		});
	}

	public void saveConfig(IMemento memento) {
		MementoConverter.saveConfig(memento, cfg);
	}

	public void loadConfig(IMemento memento) {
		MementoConverter.loadConfig(memento, cfg);
	}
}
