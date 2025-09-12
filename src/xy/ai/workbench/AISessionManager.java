package xy.ai.workbench;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.stream.Collectors;

import org.eclipse.core.resources.IFile;
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
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.texteditor.IDocumentProvider;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.batch.AIBatchManager;
import xy.ai.workbench.connectors.AdaptingConnector;
import xy.ai.workbench.marker.MarkerRessourceScanner;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;
import xy.ai.workbench.tools.AbstractQueryListener;

public class AISessionManager {
	public static final String USER = "User:";
	public static final String AGENT = "Agent:";

	private ActiveEditorListener editorListener = new ActiveEditorListener(this);

	private ConfigManager cfg;
	private AdaptingConnector connector;
	private int[] inputStats = new int[InputMode.values().length];
	private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();
	private List<Consumer<int[]>> inputStatObs = new ArrayList<>();

	private List<IFile> selectedFiles = List.of();
	private ISearchResult result = null;

	public AISessionManager(ConfigManager cfg, AdaptingConnector connector) {
		this.cfg = cfg;
		this.connector = connector;
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
		inputStats[mode.ordinal()] = getInput(mode).length();
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
			System.out.println("Searchresult changed: " + result.getLabel());
			Display.getDefault().asyncExec(() -> updateInputStat(InputMode.Search));
		}
	}

	private String getInput(InputMode mode) {
		ITextEditor textEditor = editorListener.getLastTextEditor();

		switch (mode) {
		case Instructions:
			String systemPrompt = Arrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith("#"))
					.collect(Collectors.joining(", "));
			return systemPrompt;
		case Selection:
			if (textEditor != null) {
				ISelectionProvider selectionProvider = textEditor.getSelectionProvider();
				if (selectionProvider != null) {
					ISelection selection = selectionProvider.getSelection();
					ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;
					if (tsel != null)
						return tsel.getText();
				}
			}
			break;
		case Editor:
			if (textEditor != null) {
				IDocumentProvider documentProvider = textEditor.getDocumentProvider();
				if (documentProvider != null) {
					IDocument doc = documentProvider.getDocument(textEditor.getEditorInput());
					if (doc != null)
						return doc.get();
				}
			}
			break;
		case Current_line:
			if (textEditor != null) {
				ISelectionProvider selectionProvider = textEditor.getSelectionProvider();
				if (selectionProvider != null) {
					ISelection selection = selectionProvider.getSelection();
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
		Job.create("Starting Prompt", (mon) -> {
			SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 4);
			try {
				sub.subTask("Prepare inputs");
				var req = prepareInner(display, false, sub);
				sub.worked(1);
				sub.subTask("Insert Tag");
				insertTag(display, req, sub);
				mon.worked(1);
				sub.subTask("Execute prompt");
				var ans = executeInner(display, req, sub);
				mon.worked(1);
				sub.subTask("Process Answer");
				replaceTag(display, ans, sub);
				mon.worked(1);
			} catch (Exception e) {
				e.printStackTrace();
				return Status.CANCEL_STATUS;
			} finally {
				mon.done();
			}
			return Status.OK_STATUS;
		}).schedule();
	}

	public void queue(Display display, AIBatchManager batch) {
		Job.create("Enqueue Prompt", (mon) -> {
			SubMonitor sub = SubMonitor.convert(mon, "Enqueue prompt", 3);
			try {
				sub.subTask("Prepare inputs");
				var req = prepareInner(display, true, sub);
				sub.worked(1);
				sub.subTask("Insert Tag");
				insertTag(display, req, sub);
				mon.worked(1);
				sub.subTask("Enqueue prompt");
				batch.enqueue(req, sub);
				sub.worked(1);
			} catch (Exception e) {
				e.printStackTrace();
				return Status.CANCEL_STATUS;
			} finally {
				mon.done();
			}
			return Status.OK_STATUS;
		}).schedule();
	}

	private String input;

	private IModelRequest prepareInner(Display display, boolean batchFix, IProgressMonitor mon) {
		System.out.println("Preparing Call");

		input = "";
		display.syncExec(() -> {
			if (cfg.isInputEnabled(InputMode.Editor))
				input += getInput(InputMode.Editor);
			else if (cfg.isInputEnabled(InputMode.Selection))
				input += getInput(InputMode.Selection);
			else if (cfg.isInputEnabled(InputMode.Current_line))
				input += getInput(InputMode.Current_line);
		});

		String systemPrompt = cfg.isInputEnabled(InputMode.Instructions) ? getInput(InputMode.Instructions) : "";

		if ((input == null || input.isBlank()) && systemPrompt.isBlank())
			throw new IllegalArgumentException("Input and instructions Empty");

		if (editorListener.getLastTextEditor() == null && !batchFix)
			throw new IllegalArgumentException("Result editor unset");

		List<String> tools = new ArrayList<String>();
		if (cfg.isInputEnabled(InputMode.Files))
			tools.addAll(selectedFiles.stream().map(f -> {
				try {
					return f.readString();
				} catch (CoreException e) {
					e.printStackTrace();
					return "";
				}
			}).collect(Collectors.toList()));

		if (cfg.isInputEnabled(InputMode.Search)) {
			String search = getInput(InputMode.Search);
			if (search != null && !search.isBlank())
				tools.add(search);
		}

		System.out.println("Input prepared");

		IModelRequest req = connector.createRequest(//
				input, //
				systemPrompt, //
				tools, //
				batchFix, //
				mon//
		);
		return req;
	}

	private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon) {
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));
		IModelResponse resp = connector.executeRequest(req, mon);
		AIAnswer res = connector.convertResponse(resp, mon);
		display.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));
		return res;
	}

	public void insertTag(Display display, IModelRequest req, IProgressMonitor mon) {
		display.syncExec(() -> {
			try {
				ITextEditor textEditor = editorListener.getLastTextEditor();

				IDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());
				ISelection selection = textEditor.getSelectionProvider().getSelection();
				ITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;

				switch (cfg.getOuputMode()) {
				case Chat:
					String replace = String.format("\n%s\n%s\n%s\n", AGENT, generateTag(req), USER);
					doc.replace(doc.getLength(), 0, replace);
					textEditor.selectAndReveal(doc.getLength(), 0);
					break;
				case Append:
					doc.replace(doc.getLength(), 0, "\n" + generateTag(req));
					break;
				case Replace:
					if (tsel != null)
						doc.replace(tsel.getOffset(), tsel.getLength(), generateTag(req));
					break;
				case Cursor:
					if (tsel != null)
						doc.replace(tsel.getOffset(), 0, generateTag(req));
					break;
				}
				textEditor.doSave(mon);
			} catch (BadLocationException e) {
				System.out.println("Error adding text");
			}
		});
	}

	private String generateTag(IModelRequest req) {
		KeyPattern pattern = connector.getConnector(req).getSupportedKeyPattern();
		return MarkerRessourceScanner.getPromptTag(pattern.name(), req.getID());
	}

	public void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {
		if (!Activator.getDefault().markerScanner.findAndReplaceMarkers(ans))
			System.out.println("Error: wasn't able to replace prompt marker with answer:\n" + ans.answer);
	}
}
