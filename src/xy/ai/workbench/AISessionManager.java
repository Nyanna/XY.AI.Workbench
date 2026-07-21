package xy.ai.workbench;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.stream.Collectors;

import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.Status;
import org.eclipse.core.runtime.SubMonitor;
import org.eclipse.core.runtime.jobs.Job;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.Document;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextSelection;
import org.eclipse.jface.viewers.ISelection;
import org.eclipse.jface.viewers.ISelectionProvider;
import org.eclipse.jface.viewers.TreeSelection;
import org.eclipse.search.ui.ISearchQuery;
import org.eclipse.search.ui.ISearchResult;
import org.eclipse.search.ui.ISearchResultListener;
import org.eclipse.search.ui.NewSearchUI;
import org.eclipse.search.ui.SearchResultEvent;
import org.eclipse.search.ui.text.AbstractTextSearchResult;
import org.eclipse.search.ui.text.Match;
import org.eclipse.swt.widgets.Display;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.connectors.AdaptingConnector;
import xy.ai.workbench.editors.AIRuleScanner;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;
import xy.ai.workbench.tools.AbstractQueryListener;

public class AISessionManager {
	public static final String CONTEXT_PROMPT_TXT = "context.prompt.txt";

	private ActiveEditorListener editorListener = new ActiveEditorListener(this);

	private final ConfigManager cfg;
	private final AdaptingConnector connector;
	public final EditorInterface editIfc;
	private int[] inputStats = new int[InputMode.values().length];
	private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();
	private List<Consumer<int[]>> inputStatObs = new ArrayList<>();

	private List<IFile> selectedFiles = List.of();
	private ISearchResult result = null;

	public AISessionManager(ConfigManager cfg, AdaptingConnector connector) {
		this.cfg = cfg;
		this.connector = connector;
		editIfc = new EditorInterface(editorListener, connector, cfg);
		cfg.addInputModeObs(i -> updateInputStat(i));
	}

	public void clearObserver() {
		answerObs.clear();
		inputStatObs.clear();
	}

	public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {
		inputStatObs.add(obs);
		if (initialize)
			obs.accept(inputStats);
	}

	public void addAnswerObs(Consumer<AIAnswer> obs) {
		answerObs.add(obs);
	}

	public void updateInputStat(InputMode mode) {
		String input = getInput(mode);
		inputStats[mode.ordinal()] = input != null ? input.length() : -1;
		inputStatObs.forEach(c -> c.accept(inputStats));
	}

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
			LOG.info("Searchresult changed: " + result.getLabel());
			Display.getDefault().asyncExec(() -> updateInputStat(InputMode.Search));
		}
	}

	private String getInput(InputMode mode) {
		ITextEditor textEditor = editorListener.getLastTextEditor();

		switch (mode) {
		case SystemPrompt:
			StringBuffer systemPrompt = new StringBuffer();
			Arrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith("#"))
					.forEach(e -> systemPrompt.append("* ").append(e).append(".\n"));
			String freeText = cfg.getFreeText();
			if (freeText != null && !freeText.isBlank())
				systemPrompt.append(".\n").append(cfg.getFreeText()).append(".\n");
			String prompttext = systemPrompt.toString();
			return prompttext.length() > 0 && !prompttext.isBlank() ? prompttext : null;
		case Selection:
			if (textEditor != null) {
				ISelectionProvider selectionProvider = textEditor.getSelectionProvider();
				if (selectionProvider != null) {
					ISelection selection = selectionProvider.getSelection();
					ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
					if (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1)
						return removeCommentLines(tsel.getText());

					if (tsel != null) {
						int line = tsel.getEndLine();
						IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
						try {
							IRegion lineInfo = doc.getLineInformation(line);
							return doc.get(lineInfo.getOffset(), lineInfo.getLength());
						} catch (BadLocationException e1) {
							LOG.error("Exception", e1);
						}
					}
				}
			}
			break;
		case Context_prompt:
			if (textEditor != null) {
				IEditorInput input = textEditor.getEditorInput();
				if (input instanceof IFileEditorInput) {
					IResource promptResource = ((IFileEditorInput) input).getFile().getParent()
							.findMember(CONTEXT_PROMPT_TXT);

					if (promptResource instanceof IFile) {
						IFile promptFile = (IFile) promptResource;
						try (InputStream is = promptFile.getContents()) {
							return new String(is.readAllBytes(), StandardCharsets.UTF_8);
						} catch (IOException | CoreException e) {
							throw new IllegalStateException(e);
						}
					}
				}
			}
			break;
		case Files:
			return getFilsAsString(selectedFiles);
		case Search:
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
						LOG.error("Exception", e1);
						return "";
					}
				}).collect(Collectors.joining("\n"));

				return lines.length() > 0 ? lines : null;
			}
			break;
		}
		return null;
	}

	public String removeCommentLines(String input) {
		if (input == null || input.isEmpty())
			return input;

		StringBuffer result = new StringBuffer();
		String[] lines = input.split("\\R");

		for (String line : lines)
			if (!line.trim().startsWith(AIRuleScanner.LINE_COMMENT))
				result.append(line).append(System.lineSeparator());

		return result.toString();
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
				LOG.error("Error on reading " + file.getName(), e);
			}
		}
		return fullContent.length() > 0 ? fullContent.toString() : null;
	}

	public void execute(Display display) {
		Job.create("Starting Prompt", (mon) -> {
			SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 4);
			try {
				sub.subTask("Prepare inputs");
				var req = prepareInner(display, false, sub);
				sub.worked(1);
				sub.subTask("Insert Tag");
				editIfc.insertTag(display, req, sub);
				mon.worked(1);
				sub.subTask("Execute prompt");
				var ans = executeInner(display, req, sub);
				mon.worked(1);
				sub.subTask("Process Answer");
				editIfc.replaceTag(display, ans, sub);
				mon.worked(1);
			} catch (Exception e) {
				LOG.error(e.getMessage(), e);
				return Status.CANCEL_STATUS;
			} finally {
				mon.done();
			}
			return Status.OK_STATUS;
		}).schedule();
	}

	public void queueAsync(Display display, AIBatchManager batch) {
		Job.create("Enqueue Prompt", (mon) -> {
			try {
				queueSync(display, batch, mon);
			} catch (Exception e) {
				LOG.error(e.getMessage(), e);
				return Status.CANCEL_STATUS;
			} finally {
				mon.done();
			}
			return Status.OK_STATUS;
		}).schedule();
	}

	private void queueSync(Display display, AIBatchManager batch, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Enqueue batch prompt", 3);
		sub.subTask("Prepare inputs");
		var req = prepareInner(display, true, sub.split(1));
		sub.subTask("Insert Tag");
		editIfc.insertTag(display, req, sub.split(1));
		sub.subTask("Enqueue prompt");
		batch.enqueue(req, sub.split(1));
	}

	public void queueAndSubmit(Display display, AIBatchManager batch) {
		Job.create("Enqueue Prompt", (mon) -> {
			try {
				queueSync(display, batch, mon);
				batch.submitBatches(mon);
			} catch (Exception e) {
				LOG.error(e.getMessage(), e);
				return Status.CANCEL_STATUS;
			} finally {
				mon.done();
			}
			return Status.OK_STATUS;
		}).schedule();
	}

	private IModelRequest prepareInner(Display display, boolean batchFix, IProgressMonitor mon) {
		SubMonitor sub = SubMonitor.convert(mon, "Preparing Call", 1);
		sub.subTask("Preparing Call");

		List<String> inputs = new ArrayList<String>();
		display.syncExec(() -> {
			String input = null;
			if (cfg.isInputEnabled(InputMode.Selection))
				input = getInput(InputMode.Selection);
			if (input != null)
				inputs.add(input);
		});

		StringBuffer systemPrompt = new StringBuffer();
		if (cfg.isInputEnabled(InputMode.SystemPrompt)) {
			String input = getInput(InputMode.SystemPrompt);
			if (input == null)
				throw new IllegalArgumentException("Systemprompt is selected but null");
			systemPrompt.append(input);
		}
		if (cfg.isInputEnabled(InputMode.Context_prompt)) {
			if (systemPrompt.length() > 0)
				systemPrompt.append("\n");
			String input = getInput(InputMode.Context_prompt);
			if (input == null)
				throw new IllegalArgumentException("Context prompt is selected but null");
			systemPrompt.append(input);
		}

		if ((inputs == null || inputs.isEmpty()) && systemPrompt.length() == 0)
			throw new IllegalArgumentException("Input and System Prompt Empty");

		if (editorListener.getLastTextEditor() == null && !batchFix)
			throw new IllegalArgumentException("Result editor unset");

		List<String> tools = List.of(cfg.getTools());

		if (cfg.isInputEnabled(InputMode.Files))
			inputs.addAll(selectedFiles.stream().map(f -> {
				try {
					return f.readString();
				} catch (CoreException e) {
					LOG.error(e.getMessage(), e);
					return "";
				}
			}).collect(Collectors.toList()));

		if (cfg.isInputEnabled(InputMode.Search)) {
			String search = getInput(InputMode.Search);
			if (search != null && !search.isBlank())
				inputs.add(search);
			else
				throw new IllegalArgumentException("Search prompt is selected but null");
		}

		sub.subTask("Input prepared");

		IModelRequest req = connector.createRequest(//
				inputs, //
				systemPrompt.toString(), //
				tools, //
				batchFix, //
				sub//
		);
		sub.worked(1);
		return req;
	}

	private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon) {
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));
		IModelResponse resp = connector.executeRequest(req, mon);
		AIAnswer res = connector.convertResponse(resp, mon);
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));
		return res;
	}
}
