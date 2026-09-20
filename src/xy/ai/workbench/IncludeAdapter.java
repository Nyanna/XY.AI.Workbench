package xy.ai.workbench;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.Document;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.viewers.TreeSelection;
import org.eclipse.search.ui.ISearchQuery;
import org.eclipse.search.ui.ISearchResult;
import org.eclipse.search.ui.ISearchResultListener;
import org.eclipse.search.ui.NewSearchUI;
import org.eclipse.search.ui.SearchResultEvent;
import org.eclipse.search.ui.text.AbstractTextSearchResult;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IFileEditorInput;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.IWorkbenchWindow;
import org.eclipse.ui.PlatformUI;
import org.eclipse.ui.texteditor.ITextEditor;

import xy.ai.workbench.connector.harness.IIncludeAdapter;
import xy.ai.workbench.connector.harness.SessionProcessor;
import xy.ai.workbench.tools.AbstractQueryListener;

/**
 * Resolves the {@link SessionProcessor}'s typed include kinds against Eclipse
 * resources, selection and search state - keeping the processor itself free of
 * Eclipse APIs.
 */
public class IncludeAdapter implements IIncludeAdapter {

	private ActiveEditorListener editorListener;
	private List<IFile> selectedFiles = List.of();
	private ISearchResult result = null;

	public IncludeAdapter(ActiveEditorListener editorListener) {
		this.editorListener = editorListener;
	}

	public ITextEditor getCurrentEditor() {
		return editorListener.getLastTextEditor();
	}

	@Override
	public String contextPrompt(String dir) {
		IContainer container = resolveContainer(dir);
		IResource promptResource = container == null ? null : container.findMember(AISessionManager.CONTEXT_PROMPT_TXT);
		if (!(promptResource instanceof IFile))
			return null;
		try (InputStream is = ((IFile) promptResource).getContents()) {
			return new String(is.readAllBytes(), StandardCharsets.UTF_8);
		} catch (IOException | CoreException e) {
			throw new IllegalStateException(e);
		}
	}

	@Override
	public List<Entry> files(String arg) {
		if ("selected".equals(arg))
			return toEntries(selectedFiles);
		IContainer container = resolveContainer(arg);
		if (container == null)
			return List.of();
		try {
			return toEntries(Arrays.stream(container.members()).filter(m -> m instanceof IFile).map(m -> (IFile) m)
					.collect(Collectors.toList()));
		} catch (CoreException e) {
			LOG.error(e.getMessage(), e);
			return List.of();
		}
	}

	@Override
	public String file(String path) {
		IFile file = resolveFile(path);
		if (file == null)
			return null;
		try {
			return file.readString();
		} catch (CoreException e) {
			LOG.error("Error on reading " + path, e);
			return null;
		}
	}

	@Override
	public List<Entry> searchFiles() {
		return toEntries(searchResultFiles());
	}

	@Override
	public List<Match> searchMatches() {
		List<Match> matches = new ArrayList<>();
		if (!(result instanceof AbstractTextSearchResult))
			return matches;
		AbstractTextSearchResult textRes = (AbstractTextSearchResult) result;
		for (IFile file : searchResultFiles())
			for (org.eclipse.search.ui.text.Match m : textRes.getMatches(file))
				try {
					matches.add(toSearchMatch(m));
				} catch (BadLocationException | CoreException e) {
					LOG.error("Exception", e);
				}
		return matches;
	}

	public void initializeInputs() {
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
					}
				});
			}
		}
	}

	public class SearchResultListener implements ISearchResultListener {
		@Override
		public void searchResultChanged(SearchResultEvent e) {
			result = e.getSearchResult();
			// LOG.info("Searchresult changed: " + result.getLabel());
		}
	}

	private List<IFile> searchResultFiles() {
		if (!(result instanceof AbstractTextSearchResult))
			return List.of();
		AbstractTextSearchResult textRes = (AbstractTextSearchResult) result;
		return Arrays.stream(textRes.getElements()).filter(e -> e instanceof IFile).map(e -> (IFile) e)
				.collect(Collectors.toList());
	}

	private List<Entry> toEntries(List<IFile> files) {
		List<Entry> entries = new ArrayList<>();
		for (IFile file : files)
			try {
				entries.add(new Entry(file.getFullPath().toString(), file.readString()));
			} catch (CoreException e) {
				LOG.error("Error on reading " + file.getName(), e);
			}
		return entries;
	}

	/**
	 * Resolves {@code pathText} - absolute filesystem path or path relative to the
	 * active editor's folder.
	 */
	private IContainer resolveContainer(String pathText) {
		if (pathText == null || pathText.isBlank())
			return baseContainer();
		Path p = Paths.get(pathText);
		if (p.isAbsolute()) {
			IContainer[] found = ResourcesPlugin.getWorkspace().getRoot().findContainersForLocationURI(p.toUri());
			return found.length > 0 ? found[0] : null;
		}
		IContainer base = baseContainer();
		IResource member = base == null ? null : base.findMember(pathText);
		return member instanceof IContainer ? (IContainer) member : null;
	}

	private IFile resolveFile(String pathText) {
		Path p = Paths.get(pathText);
		if (p.isAbsolute()) {
			IFile[] found = ResourcesPlugin.getWorkspace().getRoot().findFilesForLocationURI(p.toUri());
			return found.length > 0 ? found[0] : null;
		}
		IContainer base = baseContainer();
		IResource member = base == null ? null : base.findMember(pathText);
		return member instanceof IFile ? (IFile) member : null;
	}

	private IContainer baseContainer() {
		ITextEditor textEditor = getCurrentEditor();
		if (textEditor != null) {
			IEditorInput input = textEditor.getEditorInput();
			if (input instanceof IFileEditorInput)
				return ((IFileEditorInput) input).getFile().getParent();
		}
		return ResourcesPlugin.getWorkspace().getRoot();
	}

	private Match toSearchMatch(org.eclipse.search.ui.text.Match match) throws BadLocationException, CoreException {
		IFile file = (IFile) match.getElement();
		String fileContent = file.readString();

		IDocument doc = new Document(fileContent);
		int lineNumber = doc.getLineOfOffset(match.getOffset());
		int lineOffset = doc.getLineOffset(lineNumber);
		int lineLength = doc.getLineLength(lineNumber);
		return new IncludeAdapter.Match(file.getFullPath().toString(), lineNumber + 1, doc.get(lineOffset, lineLength));
	}
}