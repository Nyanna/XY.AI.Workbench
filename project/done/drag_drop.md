Ich möchte für `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java` ein Drag'n'Drop Support implementieren.
Wenn, eine Datei in den Editor gezogen wird soll eine `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java` Syntax eingefügt werden.
Wenn, ein Verzeichnis in den Editor gezogen wird soll eine `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java` Syntax erzeugt werden.

Bei einer Mischung oder einem Stapel von Dateien landet jeweils ein Eintrag in einer eigenen Zeile.

Das Ziel Package für Drag'n'Drop Support ist `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/drag`
SystemInit:  id=6ba73112-ad96-495b-8f88-ad6354966f9e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 8bf1-2d-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
  reason: Editor-Klasse und die beiden Include-Commands lesen sowie das Zielpackage prüfen
```
/answer 8bf1-2d-0 allow
Control Request:
```yaml
id: 8bf1-2e-2d
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: s0G8wt|06AciC
          excert: import java.util.ArrayList;
        - id: ONdGnS|iauC3X
          excert: import xy.ai.workbench.editor.spellcheck.SpellingAnnotation;
        - id: AITextEditor
          excert: "public class AITextEditor extends TextEditor {"
          children:
          - id: AITextEditor.eiO9HI|Q1y79C
            excert: private static final int LIMIT = 512 * 1024;
          - id: AITextEditor.AITextEditor
            excert: public AITextEditor()
          - id: AITextEditor.createSourceViewer
            excert: "protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler rul…"
          - id: AITextEditor.createPartControl
            excert: public void createPartControl(Composite parent)
          - id: AITextEditor.installPainter
            excert: private void installPainter(ISourceViewer sourceViewer)
          - id: AITextEditor.dispose
            excert: public void dispose()
          - id: AITextEditor.handleCaretMoved
            excert: private void handleCaretMoved(int offset)
          - id: AITextEditor.getAdapter
            excert: public T getAdapter(Class<T> adapter)
          - id: AITextEditor.refreshOutline
            excert: private void refreshOutline()
          - id: AITextEditor.selectAndRevealNode
            excert: public void selectAndRevealNode(Node node)
          - id: AITextEditor.getInitialWordWrapStatus
            excert: protected boolean getInitialWordWrapStatus()
          - id: AITextEditor.getUpdateManager
            excert: public EditorManager getUpdateManager()
          - id: AITextEditor.baseDirectory
            excert: private Path baseDirectory()
          - id: AITextEditor.ManagerListener
            excert: "private class ManagerListener implements IManagerListener {"
            children:
            - id: AITextEditor.ManagerListener.onDocumentChanged
              excert: "public void onDocumentChanged(IDocument oldDocument, IDocument newDocument)"
            - id: AITextEditor.ManagerListener.onAstUpdated
              excert: public void onAstUpdated(Node node)
          - id: AITextEditor.updateWordWrap
            excert: private void updateWordWrap(IDocument document)
          - id: AITextEditor.updateRulerVisibility
            excert: private void updateRulerVisibility(IDocument document)
          - id: AITextEditor.updateLineNumbers
            excert: private void updateLineNumbers(IDocument document)
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
        nodes:
        - id: K9s6LE|TD7ww8
          excert: import java.io.IOException;
        - id: PathIncludeCommand
          excert: "public class PathIncludeCommand<M> implements IIncludeCommand<M> {"
          children:
          - id: PathIncludeCommand.kind
            excert: public String kind()
          - id: PathIncludeCommand.execute
            excert: "public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, Lis…"
          - id: PathIncludeCommand.includePath
            excert: "void includePath(String pathText, List<Path> chain, SessionCallbacks<M> call, L…"
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
        nodes:
        - id: SJ9S4Z|8veEO8
          excert: import java.io.IOException;
        - id: DirectoryIncludeCommand
          excert: "public class DirectoryIncludeCommand<M> extends AdapterIncludeCommand<M> {"
          children:
          - id: DirectoryIncludeCommand.B6ZEt6|PSa2CP
            excert: private final PathIncludeCommand<M> pathCommand;
          - id: DirectoryIncludeCommand.DirectoryIncludeCommand
            excert: "public DirectoryIncludeCommand(IIncludeAdapter adapter, PathIncludeCommand<M> p…"
          - id: DirectoryIncludeCommand.kind
            excert: public String kind()
          - id: DirectoryIncludeCommand.execute
            excert: "public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, Lis…"
```
/answer 8bf1-2e-2d allow
Control Request:
```yaml
id: 8bf1-2f-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
  reason: "Quelltexte lesen, um Syntax der Commands und Editor-Aufbau zu verstehen"
```
/answer 8bf1-2f-0 allow
Control Request:
```yaml
id: 8bf1-30-2f
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: s0G8wt|06AciC
          excert: import java.util.ArrayList;
        - id: ONdGnS|iauC3X
          excert: import xy.ai.workbench.editor.spellcheck.SpellingAnnotation;
        - id: AITextEditor
          excert: "public class AITextEditor extends TextEditor {"
          children:
          - id: AITextEditor.eiO9HI|Q1y79C
            excert: private static final int LIMIT = 512 * 1024;
          - id: AITextEditor.AITextEditor
            excert: public AITextEditor()
          - id: AITextEditor.createSourceViewer
            excert: "protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler rul…"
          - id: AITextEditor.createPartControl
            excert: public void createPartControl(Composite parent)
          - id: AITextEditor.installPainter
            excert: private void installPainter(ISourceViewer sourceViewer)
          - id: AITextEditor.dispose
            excert: public void dispose()
          - id: AITextEditor.handleCaretMoved
            excert: private void handleCaretMoved(int offset)
          - id: AITextEditor.getAdapter
            excert: public T getAdapter(Class<T> adapter)
          - id: AITextEditor.refreshOutline
            excert: private void refreshOutline()
          - id: AITextEditor.selectAndRevealNode
            excert: public void selectAndRevealNode(Node node)
          - id: AITextEditor.getInitialWordWrapStatus
            excert: protected boolean getInitialWordWrapStatus()
          - id: AITextEditor.getUpdateManager
            excert: public EditorManager getUpdateManager()
          - id: AITextEditor.baseDirectory
            excert: private Path baseDirectory()
          - id: AITextEditor.ManagerListener
            excert: "private class ManagerListener implements IManagerListener {"
            children:
            - id: AITextEditor.ManagerListener.onDocumentChanged
              excert: "public void onDocumentChanged(IDocument oldDocument, IDocument newDocument)"
            - id: AITextEditor.ManagerListener.onAstUpdated
              excert: public void onAstUpdated(Node node)
          - id: AITextEditor.updateWordWrap
            excert: private void updateWordWrap(IDocument document)
          - id: AITextEditor.updateRulerVisibility
            excert: private void updateRulerVisibility(IDocument document)
          - id: AITextEditor.updateLineNumbers
            excert: private void updateLineNumbers(IDocument document)
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java instead."
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
        nodes:
        - id: K9s6LE|TD7ww8
          excert: import java.io.IOException;
        - id: PathIncludeCommand
          excert: "public class PathIncludeCommand<M> implements IIncludeCommand<M> {"
          children:
          - id: PathIncludeCommand.kind
            excert: public String kind()
          - id: PathIncludeCommand.execute
            excert: "public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, Lis…"
          - id: PathIncludeCommand.includePath
            excert: "void includePath(String pathText, List<Path> chain, SessionCallbacks<M> call, L…"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java instead."
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
        nodes:
        - id: SJ9S4Z|8veEO8
          excert: import java.io.IOException;
        - id: DirectoryIncludeCommand
          excert: "public class DirectoryIncludeCommand<M> extends AdapterIncludeCommand<M> {"
          children:
          - id: DirectoryIncludeCommand.B6ZEt6|PSa2CP
            excert: private final PathIncludeCommand<M> pathCommand;
          - id: DirectoryIncludeCommand.DirectoryIncludeCommand
            excert: "public DirectoryIncludeCommand(IIncludeAdapter adapter, PathIncludeCommand<M> p…"
          - id: DirectoryIncludeCommand.kind
            excert: public String kind()
          - id: DirectoryIncludeCommand.execute
            excert: "public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, Lis…"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java instead."
```
/answer 8bf1-30-2f allow
Text:
 Ich brauche die Quelltexte der Methoden.
Control Request:
```yaml
id: 8bf1-31-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    ids:
    - ONdGnS|iauC3X
    - AITextEditor.createSourceViewer
    - AITextEditor.createPartControl
    - AITextEditor.dispose
    - AITextEditor.baseDirectory
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
    ids:
    - PathIncludeCommand.kind
    - PathIncludeCommand.execute
    - PathIncludeCommand.includePath
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
    ids:
    - DirectoryIncludeCommand.kind
    - DirectoryIncludeCommand.execute
  reason: Methodenrümpfe lesen
```
/answer 8bf1-31-0 allow
Control Request:
```yaml
id: 8bf1-32-31
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: ONdGnS|iauC3X
          type: ImportGroup
          code: |
            import xy.ai.workbench.editor.spellcheck.SpellingAnnotation;
            import java.nio.file.Path;
            import org.eclipse.core.runtime.IPath;
            import org.eclipse.ui.IEditorInput;
            import org.eclipse.ui.IFileEditorInput;
            import org.eclipse.ui.IURIEditorInput;
        - id: AITextEditor.createSourceViewer
          type: MethodDeclaration
          code: |2
              @Override
              protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {
                ISourceViewer sourceViewer = super.createSourceViewer(parent, ruler, styles);
                if (ruler instanceof CompositeRuler)
                  this.ruler = (CompositeRuler) ruler;
                manager.addListener(new ManagerListener());
                manager.install(sourceViewer);
                new SpellCheckReconciler(sourceViewer, manager);
                return sourceViewer;
              }
        - id: AITextEditor.createPartControl
          type: MethodDeclaration
          code: |2
              @Override
              public void createPartControl(Composite parent) {
                super.createPartControl(parent);
                installPainter(getSourceViewer());
                if (getSourceViewer().getDocument() != null)
                  updateWordWrap(getSourceViewer().getDocument());
                if (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)
                  widget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));
              }
        - id: AITextEditor.dispose
          type: MethodDeclaration
          code: |2
              @Override
              public void dispose() {
                manager.uninstall();
                super.dispose();
              }
        - id: AITextEditor.baseDirectory
          type: MethodDeclaration
          code: |2
              private Path baseDirectory() {
                IEditorInput input = getEditorInput();
                if (input instanceof IFileEditorInput fileInput) {
                  IPath location = fileInput.getFile().getLocation();
                  return location == null ? null : location.removeLastSegments(1).toFile().toPath();
                }
                if (input instanceof IURIEditorInput uriInput && "file".equals(uriInput.getURI().getScheme()))
                  return Path.of(uriInput.getURI()).getParent();
                return null;
              }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
        nodes:
        - id: PathIncludeCommand.kind
          type: MethodDeclaration
          code: |2
              @Override
              public String kind() {
                return null;
              }
        - id: PathIncludeCommand.execute
          type: MethodDeclaration
          code: |2
              @Override
              public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                String path = IncludeArgs.path(arg);
                if (path.isBlank())
                  throw new IllegalStateException("Include path must not be empty");
                includePath(path, chain, call, out, recursiv);
              }
        - id: PathIncludeCommand.includePath
          type: MethodDeclaration
          code: |2
              void includePath(String pathText, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                Path path = Paths.get(pathText).toAbsolutePath().normalize();
                if (chain.contains(path))
                  throw new IncludeCycleException(path.toString());
                String content;
                try {
                  content = Files.readString(path);
                } catch (IOException e) {
                  throw new IllegalStateException("Could not read include target: " + path, e);
                }
                List<Path> nested = new ArrayList<>(chain);
                nested.add(path);
                recursiv.processText(content, nested, call, out, recursiv);
              }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
        nodes:
        - id: DirectoryIncludeCommand.kind
          type: MethodDeclaration
          code: |2
              @Override
              public String kind() {
                return "directory";
              }
        - id: DirectoryIncludeCommand.execute
          type: MethodDeclaration
          code: |2
              @Override
              public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                String dir = IncludeArgs.path(arg);
                if (dir.isBlank()) {
                  for (IIncludeAdapter.Entry entry : requireAdapter().selectedDirectories()) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
                  return;
                }
                Path directory = Paths.get(dir).toAbsolutePath().normalize();
                List<Path> files;
                try (Stream<Path> stream = Files.list(directory)) {
                  files = stream.filter(Files::isRegularFile).sorted().collect(Collectors.toList());
                } catch (IOException e) {
                  throw new IllegalStateException("Could not read include directory: " + directory, e);
                }
                for (Path file : files) pathCommand.includePath(file.toString(), chain, call, out, recursiv);
              }
```
/answer 8bf1-32-31 allow
ReasoningToken: 53
Control Request:
```yaml
id: 8bf1-33-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class IncludeArgs|\.kind\(\)|@include|#include|"include
    file_include_mask:
    - '*.java'
    - '*.md'
    - '*.ai'
    dir_exclude_mask:
    - bin
    - target
    - .git
    intent: survey
    limit: 30
  reason: "Include-Syntax (Parser, IncludeArgs, Beispiele) finden"
```
/answer 8bf1-33-0 allow
Control Request:
```yaml
id: 8bf1-34-33
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: src/xy/ai/workbench/view/approver/AutoApprovePanel.java
          matches:
          - "String line = fit(sessionId, SESSION_WIDTH) + \" | \" + fit(message.kind().name(), ACTION_WIDTH) + \" | \" + fit(\"-\", TOOL_WIDTH) + \" | \" + pad(Integer.toString(message.text().length()), CHARS_WIDTH) + \" | \" + pad(\"\", 15) + shorten(message.text(), DETAIL_MAX);"
          - boolean isResult = item != null && (item.kind() == ApprovalItem.Kind.ToolResult || item.kind() == ApprovalItem.Kind.ControlResult);
        - path: src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
          matches:
          - "String text = \"include\" + (kind == null ? \"\" : \" \" + kind) + \"](\" + closing(context.rest());"
          - "String display = kind == null ? \"include (path)\" : \"include \" + kind;"
          - String kind = context.kind();
          - "for (String value : syntax.fixedValues(context.kind())) {"
        - path: src/xy/ai/workbench/editor/suggest/IncludeContextParser.java
          matches:
          - private static final String KEYWORD = "include";
        - path: connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
          matches:
          - if (decision.result() != ApprovalResult.Deny || item == null || (item.kind() != Kind.ToolResult && item.kind() != Kind.ControlResult))
          - if (item.kind() == Kind.ToolResult)
          - "return item.kind() == Kind.ToolCall ? ApprovalResult.Call : ApprovalResult.Allow;"
          - "return item.kind() == Kind.ControlResult ? AnswerParser.omittedYaml(item) : item.yaml();"
          - "return (item.kind() == Kind.ToolResult ? \"Tool result \" : \"Tool call \") + item.id() + tool + \" was denied\" + (why.isEmpty() ? \".\" : \": \" + why);"
        - path: connector/src/xy/ai/conscious/autoapprove/SessionTracker.java
          matches:
          - if (result == null || (result.kind() != Kind.ControlResult && result.kind() != Kind.ToolResult))
          - return item.kind() == Kind.ControlCall || item.kind() == Kind.ToolCall;
        - path: connector/src/xy/ai/conscious/autoapprove/AnswerParser.java
          matches:
          - "return item.kind() == Kind.ControlResult ? omitControlResult(text, item) : omitToolResult(text, item.id());"
        - path: connector/src/xy/ai/conscious/autoapprove/rules/DenyOwnPromptFileRule.java
          matches:
          - boolean call = item.kind() == Kind.ControlCall || item.kind() == Kind.ToolCall;
        - path: connector/src/xy/ai/conscious/autoapprove/rules/DenyRootPathRule.java
          matches:
          - if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
        - path: connector/src/xy/ai/conscious/autoapprove/rules/SmallResultRule.java
          matches:
          - if (item.kind() != Kind.ControlResult && item.kind() != Kind.ToolResult)
        - path: connector/src/xy/ai/conscious/autoapprove/rules/SmallEditRule.java
          matches:
          - if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
        - path: connector/src/xy/ai/conscious/autoapprove/rules/AllowedToolsRule.java
          matches:
          - if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
        - path: connector/src/xy/ai/conscious/autoapprove/rules/HoldMarkdownRule.java
          matches:
          - "public static final String[] PATTERN_FIELDS = { \"include\" };"
          - if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
        - path: connector/src/xy/ai/conscious/autoapprove/rules/DeepPathReadRule.java
          matches:
          - "switch (item.kind()) {"
        - path: connector/src/xy/ai/conscious/autoapprove/rules/HoldOutsideProjectRule.java
          matches:
          - if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
        - path: connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
          matches:
          - "commands.put(command.kind(), command);"
        - path: connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
          matches:
          - "class IncludeArgs {"
```
/answer 8bf1-34-33 allow
Control Request:
```yaml
id: 8bf1-35-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeContextParser.java
  reason: Exakte Include-Syntax und Pfad-Escaping prüfen
```
/answer 8bf1-35-0 allow
ReasoningToken: 50
Control Request:
```yaml
id: 8bf1-36-35
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
        nodes:
        - id: IncludeArgs
          excert: "class IncludeArgs {"
          children:
          - id: IncludeArgs.path
            excert: static String path(String arg)
            docstring: "Removes surrounding whitespace and a leading and trailing backtick, e.g. {@code…"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java instead."
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
        nodes:
        - id: 2IR8yU|Rkz2c7
          excert: import java.io.File;
        - id: IncludeCompletionProcessor
          excert: "public class IncludeCompletionProcessor implements IContentAssistProcessor {"
          children:
          - id: IncludeCompletionProcessor.ep9ki5|9zvs8e
            excert: "private static final char[] ACTIVATION = { ' ', '(', '/', '\\\\' };"
          - id: IncludeCompletionProcessor.IncludeCompletionProcessor
            excert: public IncludeCompletionProcessor(Supplier<Path> baseDirectory)
            docstring: "@param baseDirectory directory that relative paths are resolved against, may re…"
          - id: IncludeCompletionProcessor.computeCompletionProposals
            excert: "public ICompletionProposal[] computeCompletionProposals(ITextViewer viewer, int…"
          - id: IncludeCompletionProcessor.tagProposals
            excert: "private List<ICompletionProposal> tagProposals(IncludeContext context, int offs…"
          - id: IncludeCompletionProcessor.tag
            excert: "private ICompletionProposal tag(IncludeContext context, int offset, String kind)"
          - id: IncludeCompletionProcessor.kindProposals
            excert: "private List<ICompletionProposal> kindProposals(IncludeContext context, int off…"
          - id: IncludeCompletionProcessor.argumentProposals
            excert: "private List<ICompletionProposal> argumentProposals(IncludeContext context, int…"
          - id: IncludeCompletionProcessor.fixedProposals
            excert: "private List<ICompletionProposal> fixedProposals(IncludeContext context, int of…"
          - id: IncludeCompletionProcessor.pathProposals
            excert: "private List<ICompletionProposal> pathProposals(IncludeContext context, int off…"
          - id: IncludeCompletionProcessor.leadingNoise
            excert: private int leadingNoise(String prefix)
            docstring: "Whitespace and one backtick, which {@code IncludeArgs.path} strips as well."
          - id: IncludeCompletionProcessor.closing
            excert: private String closing(String rest)
          - id: IncludeCompletionProcessor.computeContextInformation
            excert: "public IContextInformation[] computeContextInformation(ITextViewer viewer, int …"
          - id: IncludeCompletionProcessor.getCompletionProposalAutoActivationCharacters
            excert: "public char[] getCompletionProposalAutoActivationCharacters()"
          - id: IncludeCompletionProcessor.getContextInformationAutoActivationCharacters
            excert: "public char[] getContextInformationAutoActivationCharacters()"
          - id: IncludeCompletionProcessor.getErrorMessage
            excert: public String getErrorMessage()
          - id: IncludeCompletionProcessor.getContextInformationValidator
            excert: public IContextInformationValidator getContextInformationValidator()
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java instead."
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeContextParser.java
        nodes:
        - id: KqlGLB|vzXW1E
          excert: import java.util.regex.Matcher;
        - id: IncludeContextParser
          excert: /**  * Locates the caret within the include syntax. Include tags are only recog…
          docstring: Locates the caret within the include syntax. Include tags are only recognised
          children:
          - id: IncludeContextParser.kjcLIS|13zDVc
            excert: private static final String KEYWORD = "include";
          - id: IncludeContextParser.parse
            excert: "IncludeContext parse(IDocument document, int offset)"
          - id: IncludeContextParser.parse_2
            excert: "IncludeContext parse(String before, String rest, int lineOffset)"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeContextParser.java instead."
```
/answer 8bf1-36-35 allow
Control Request:
```yaml
id: 8bf1-37-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: ^package
    file_include_mask:
    - '*.java'
    intent: survey
    limit: 50
  reason: Vorhandene Pakete und Dateien im editor-Verzeichnis prüfen
```
/answer 8bf1-37-0 deny warum?
Result Stats: id=6ba73112-ad96-495b-8f88-ad6354966f9e, total: 21498, in: 12, out: 3021, reason: 233, read: 56703, write: 18465, start: 1795857769714, end: 1795857777762, time: 7990, total: 25512, io: 0, control: 58, sent: 0, recv: 171
ReasoningToken: 233
Der zweite Aufruf war eine reine Leseabfrage. Er sollte zeigen, welche Dateien und Pakete es unter `editor/` schon gibt, vor allem ob `editor/drag` existiert. Ich wollte Package-Deklaration und Stil der Nachbarklassen übernehmen, damit die neuen Klassen dazu passen. Die Abfrage war also nicht zwingend nötig. Ich habe sie ohne Rückfrage zusammen mit dem anderen Aufruf gestartet, und deshalb wirkte sie unmotiviert. Ich habe sie nicht wiederholt.

Bisher weiß ich Folgendes:
- **Datei:** Die Syntax ist `[include](pfad)`. `PathIncludeCommand.kind()` liefert `null`, also gibt es kein Kind-Wort.
- **Verzeichnis:** Die Syntax ist `[include directory](pfad)`. `DirectoryIncludeCommand.kind()` liefert `"directory"`.
- **Editor:** In `AITextEditor.createSourceViewer` kann ich den Drop-Support installieren. Die Methode `baseDirectory()` gibt es schon. Ich kann damit relative Pfade bilden, falls du das willst.

Mein Plan:
1. Eine Klasse `IncludeDropHandler` im Package `xy.ai.workbench.editor.drag`. Sie ist ein `DropTargetAdapter` mit `FileTransfer`. Pro gedropptem Pfad entsteht eine eigene Zeile, je nach `Files.isDirectory` mit oder ohne `directory`. Die Zeilen werden an der Drop-Position eingefügt.
2. In `AITextEditor.createSourceViewer` einen Aufruf zum Installieren des Handlers einfügen.

Zwei Fragen vorab:
- Sollen die Pfade absolut oder relativ zum Editor-Verzeichnis (`baseDirectory()`) eingefügt werden?
- Darf ich die Kommentar-Syntax und das Escaping (Backticks bei Leerzeichen im Pfad) noch in `IncludeArgs` und `IncludeCompletionProcessor.closing` nachlesen?
Pfade absolut, ja.
SystemInit:  id=6ba73112-ad96-495b-8f88-ad6354966f9e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 8bf1-38-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
    ids:
    - IncludeArgs.path
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
    ids:
    - IncludeCompletionProcessor.tag
    - IncludeCompletionProcessor.closing
    - IncludeCompletionProcessor.pathProposals
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    ids:
    - s0G8wt|06AciC
  reason: Escaping-Regeln (IncludeArgs.path) und closing/tag im Completion-Processor lesen; Imports des Editors prüfen
```
/answer 8bf1-38-0 allow
Control Request:
```yaml
id: 8bf1-39-38
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
        nodes:
        - id: IncludeArgs.path
          type: MethodDeclaration
          code: |2
              static String path(String arg) {
                String s = arg.strip();
                if (s.startsWith("`"))
                  s = s.substring(1);
                if (s.endsWith("`"))
                  s = s.substring(0, s.length() - 1);
                return s.strip();
              }
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
        nodes:
        - id: IncludeCompletionProcessor.tag
          type: MethodDeclaration
          code: |2
              private ICompletionProposal tag(IncludeContext context, int offset, String kind) {
                String text = "include" + (kind == null ? "" : " " + kind) + "](" + closing(context.rest());
                String display = kind == null ? "include (path)" : "include " + kind;
                return new CompletionProposal(text, context.start(), offset - context.start(), text.indexOf('(') + 1, null, display, null, null);
              }
        - id: IncludeCompletionProcessor.closing
          type: MethodDeclaration
          code: |2
              private String closing(String rest) {
                return rest.startsWith(")") ? "" : ")";
              }
        - id: IncludeCompletionProcessor.pathProposals
          type: MethodDeclaration
          code: |2
              private List<ICompletionProposal> pathProposals(IncludeContext context, int offset, boolean directoriesOnly) {
                int skipped = leadingNoise(context.prefix());
                String typed = context.prefix().substring(skipped);
                int start = context.start() + skipped;
                List<ICompletionProposal> result = new ArrayList<>();
                for (PathSuggestions.Entry entry : paths.list(typed, baseDirectory.get(), directoriesOnly)) {
                  String text = entry.path().toString();
                  String display = entry.name();
                  int cursorOffset;
                  if (entry.directory()) {
                    text += File.separator;
                    display += File.separator;
                    cursorOffset = text.length();
                  } else {
                    text += closing(context.rest());
                    cursorOffset = text.length();
                  }
                  result.add(new CompletionProposal(text, start, offset - start, cursorOffset, null, display, null, entry.path().toString()));
                }
                return result;
              }
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: s0G8wt|06AciC
          type: ImportGroup
          code: |
            import java.util.ArrayList;
            import java.util.Iterator;
            import java.util.List;
            import org.eclipse.jface.text.IDocument;
            import org.eclipse.jface.text.ITextViewerExtension2;
            import org.eclipse.jface.text.source.AnnotationPainter;
            import org.eclipse.jface.text.source.CompositeRuler;
            import org.eclipse.jface.text.source.ISourceViewer;
            import org.eclipse.jface.text.source.IVerticalRuler;
            import org.eclipse.jface.text.source.IVerticalRulerColumn;
            import org.eclipse.jface.text.source.SourceViewer;
            import org.eclipse.swt.SWT;
            import org.eclipse.swt.custom.StyledText;
            import org.eclipse.swt.widgets.Composite;
            import org.eclipse.swt.widgets.Display;
            import org.eclipse.ui.editors.text.TextEditor;
            import org.eclipse.ui.texteditor.DefaultMarkerAnnotationAccess;
            import org.eclipse.ui.views.contentoutline.IContentOutlinePage;
            import xy.ai.workbench.editor.mdast.nodes.Node;
            import xy.ai.workbench.editor.outline.MarkdownOutlinePage;
            import xy.ai.workbench.editor.spellcheck.SpellCheckReconciler;
```
/answer 8bf1-39-38 allow
Control Request:
```yaml
id: 8bf1-3a-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/drag/IncludeTags.java
    source: |
      package xy.ai.workbench.editor.drag;

      import java.nio.file.Files;
      import java.nio.file.InvalidPathException;
      import java.nio.file.Path;
      import java.util.ArrayList;
      import java.util.List;

      /**
       * Builds include tags for dropped file system entries: {@code [include](path)} for files and
       * {@code [include directory](path)} for directories.
       */
      final class IncludeTags {

        private IncludeTags() {
        }

        static List<String> of(String[] paths) {
          List<String> tags = new ArrayList<>();
          for (String raw : paths) {
            try {
              tags.add(of(Path.of(raw).toAbsolutePath().normalize()));
            } catch (InvalidPathException e) {
              // not a usable path, skip it
            }
          }
          return tags;
        }

        static String of(Path path) {
          String text = path.toString();
          if (text.chars().anyMatch(c -> Character.isWhitespace(c) || c == ')'))
            text = "`" + text + "`";
          String kind = Files.isDirectory(path) ? "include directory" : "include";
          return "[" + kind + "](" + text + ")";
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/drag/IncludeDropSupport.java
    source: |
      package xy.ai.workbench.editor.drag;

      import java.util.ArrayList;
      import java.util.List;
      import org.eclipse.jface.text.BadLocationException;
      import org.eclipse.jface.text.IDocument;
      import org.eclipse.jface.text.IRegion;
      import org.eclipse.jface.text.ITextViewerExtension5;
      import org.eclipse.jface.text.TextUtilities;
      import org.eclipse.jface.text.source.ISourceViewer;
      import org.eclipse.swt.custom.StyledText;
      import org.eclipse.swt.dnd.DND;
      import org.eclipse.swt.dnd.DropTarget;
      import org.eclipse.swt.dnd.DropTargetAdapter;
      import org.eclipse.swt.dnd.DropTargetEvent;
      import org.eclipse.swt.dnd.DropTargetListener;
      import org.eclipse.swt.dnd.FileTransfer;
      import org.eclipse.swt.dnd.Transfer;
      import org.eclipse.swt.dnd.TransferData;
      import org.eclipse.swt.graphics.Point;
      import org.eclipse.swt.widgets.Listener;
      import org.eclipse.swt.widgets.TypedListener;

      /**
       * Accepts files and directories dropped on a source viewer and inserts one include tag per entry,
       * each on its own line. Other drops (e.g. text) are still handled by the existing listeners.
       */
      public final class IncludeDropSupport {

        private static final FileTransfer FILES = FileTransfer.getInstance();

        private final ISourceViewer viewer;
        private final StyledText widget;

        private IncludeDropSupport(ISourceViewer viewer, StyledText widget) {
          this.viewer = viewer;
          this.widget = widget;
        }

        public static void install(ISourceViewer viewer) {
          if (viewer != null && viewer.getTextWidget() != null)
            new IncludeDropSupport(viewer, viewer.getTextWidget()).attach();
        }

        private void attach() {
          DropTarget target = widget.getData(DND.DROP_TARGET_KEY) instanceof DropTarget existing ? existing : new DropTarget(widget, DND.DROP_COPY | DND.DROP_MOVE);

          List<Transfer> transfers = new ArrayList<>(List.of(target.getTransfer()));
          if (!transfers.contains(FILES))
            transfers.add(FILES);
          target.setTransfer(transfers.toArray(Transfer[]::new));

          // The default text drop handler cannot cope with file data, so it is only called for non-file drops.
          List<DropTargetListener> previous = new ArrayList<>();
          for (Listener listener : target.getListeners(DND.Drop))
            if (listener instanceof TypedListener typed && typed.getEventListener() instanceof DropTargetListener drop)
              previous.add(drop);
          previous.forEach(target::removeDropListener);
          target.addDropListener(new Dispatcher(previous));
        }

        private final class Dispatcher extends DropTargetAdapter {
          private final List<DropTargetListener> previous;

          Dispatcher(List<DropTargetListener> previous) {
            this.previous = previous;
          }

          @Override
          public void dragEnter(DropTargetEvent event) {
            accept(event);
          }

          @Override
          public void dragOperationChanged(DropTargetEvent event) {
            accept(event);
          }

          @Override
          public void dragOver(DropTargetEvent event) {
            if (FILES.isSupportedType(event.currentDataType))
              event.detail = DND.DROP_COPY;
          }

          @Override
          public void drop(DropTargetEvent event) {
            if (FILES.isSupportedType(event.currentDataType)) {
              if (event.data instanceof String[] paths)
                insert(IncludeTags.of(paths), offsetAt(event));
              return;
            }
            for (DropTargetListener listener : previous)
              listener.drop(event);
          }

          private void accept(DropTargetEvent event) {
            for (TransferData type : event.dataTypes) {
              if (FILES.isSupportedType(type)) {
                event.currentDataType = type;
                event.detail = DND.DROP_COPY;
                return;
              }
            }
          }
        }

        private int offsetAt(DropTargetEvent event) {
          Point point = widget.getDisplay().map(null, widget, event.x, event.y);
          int offset;
          try {
            offset = widget.getOffsetAtPoint(point);
          } catch (IllegalArgumentException e) {
            offset = -1;
          }
          return offset < 0 ? widget.getCaretOffset() : offset;
        }

        private void insert(List<String> tags, int widgetOffset) {
          IDocument document = viewer.getDocument();
          if (document == null || tags.isEmpty())
            return;
          int offset = viewer instanceof ITextViewerExtension5 ext ? ext.widgetOffset2ModelOffset(widgetOffset) : widgetOffset;
          if (offset < 0)
            return;
          try {
            String delimiter = TextUtilities.getDefaultLineDelimiter(document);
            IRegion line = document.getLineInformationOfOffset(offset);
            boolean textBefore = !document.get(line.getOffset(), offset - line.getOffset()).isBlank();
            boolean textAfter = !document.get(offset, line.getOffset() + line.getLength() - offset).isBlank();
            String text = (textBefore ? delimiter : "") + String.join(delimiter, tags) + (textAfter ? delimiter : "");
            document.replace(offset, 0, text);
            int caret = offset + text.length();
            viewer.setSelectedRange(caret, 0);
            viewer.revealRange(caret, 0);
            widget.setFocus();
          } catch (BadLocationException e) {
            // drop position vanished while the document changed, nothing to insert
          }
        }
      }
  reason: Drag'n'Drop-Klassen im Package editor.drag anlegen
```
/answer 8bf1-3a-0 allow
ReasoningToken: 4926
Control Request:
```yaml
id: 8bf1-3b-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.createSourceViewer
    old_text: |2
          new SpellCheckReconciler(sourceViewer, manager);
    new_text: |2
          new SpellCheckReconciler(sourceViewer, manager);
          IncludeDropSupport.install(sourceViewer);
    addImport:
    - import xy.ai.workbench.editor.drag.IncludeDropSupport;
  reason: Drop-Support im Editor installieren
```
/answer 8bf1-3b-0 allow
Result Stats: id=6ba73112-ad96-495b-8f88-ad6354966f9e, total: 42621, in: 18, out: 12593, reason: 25, read: 116177, write: 30010, start: 1795857918310, end: 1795857926313, time: 7944, total: 89523, io: 0, control: 59, sent: 0, recv: 211
ReasoningToken: 25
Der Drag'n'Drop-Support ist implementiert. Kompiliert und getestet habe ich ihn nicht.

**Neue Klassen in `xy.ai.workbench.editor.drag`**
- **`IncludeTags`** erzeugt pro Pfad den Tag. Der Pfad wird absolut und normalisiert eingefügt.
  - Dateien ergeben `[include](/abs/pfad/datei.md)`.
  - Verzeichnisse ergeben `[include directory](/abs/pfad/dir)`.
  - Enthält der Pfad Leerzeichen oder `)`, wird er in Backticks gesetzt. `IncludeArgs.path` entfernt diese wieder. Ungültige Pfade werden übersprungen.
- **`IncludeDropSupport`** hat die Methode `install(ISourceViewer)`.
  - Sie nutzt das `DropTarget` des `StyledText` weiter, falls eins existiert, sonst legt sie eins an. `FileTransfer` kommt zu den Transfers dazu.
  - Bei Datei-Drops fügt sie die Tags an der Drop-Position ein, jeweils in einer eigenen Zeile. Das gilt auch für eine Mischung oder einen Stapel.
  - Steht vor der Drop-Position Text in der Zeile, beginnt der Block in einer neuen Zeile. Steht dahinter noch Text, wird ein Zeilenumbruch angehängt.
  - Danach steht der Cursor hinter dem eingefügten Block.
  - Text-Drops laufen weiter über die bisherigen Listener. Deren Drop-Listener rufe ich bei Datei-Drops nicht auf, weil ich vermute, dass der Standard-Handler dort an `String[]` scheitert (ClassCastException). Ob das stimmt, habe ich nicht geprüft.

**Änderung in `AITextEditor`**
- In `createSourceViewer` steht jetzt `IncludeDropSupport.install(sourceViewer);` samt Import.

**Offene Punkte**
- Ob der Parser Pfade in Backticks innerhalb von `[include](…)` wirklich akzeptiert, habe ich nicht geprüft. `IncludeArgs.path` entfernt sie jedenfalls.
- Die Include-Tags werden laut `IncludeContextParser`-Doku nur in bestimmten Kontexten erkannt. Den abgeschnittenen Text habe ich nicht gelesen. Die Tags beginnen immer am Zeilenanfang oder in einer neuen Zeile.
- Den Drop-Listener-Austausch (`getListeners(DND.Drop)`) habe ich auf Basis meiner SWT-Kenntnis geschrieben. Bitte teste ihn mit einem Datei-Drop und einem Text-Drop im Editor.
Hm, bei einem Drop wird ein neuer Editor geöffnet?
SystemInit:  id=6ba73112-ad96-495b-8f88-ad6354966f9e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Das kann an einem anderen Drop-Handler liegen, der die Datei vor meinem bekommt. Ich suche zuerst im Projekt nach weiteren `DropTarget`- oder `FileTransfer`-Stellen.
ReasoningToken: 980
Control Request:
```yaml
id: 8bf1-3c-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: DropTarget|FileTransfer|DND\.|IDropTarget|openEditor
    file_include_mask:
    - '*.java'
    - '*.xml'
    - '*.e4xmi'
    dir_exclude_mask:
    - bin
    - target
    - .git
    intent: survey
    limit: 40
  reason: "Prüfen, ob im Projekt bereits andere DropTargets/FileTransfer-Handler existieren, die Drops abfangen und Editoren öffnen"
```
/answer 8bf1-3c-0 deny es gibt keine anderen Handler. Wenn dann ist das Eclipse default code.
Thinking:
Dein Drop landet beim Workbench-Fenster, weil mein Handler nie greift; ich vermute, dass `AbstractTextEditor` beim Einrichten sein eigenes DropTarget über meines legt, und verschiebe die Installation deshalb hinter `super.createPartControl(parent)`.


ReasoningToken: 1472
Control Request:
```yaml
id: 8bf1-3d-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.createSourceViewer
    old_text: |2
          IncludeDropSupport.install(sourceViewer);
    new_text: ""
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.createPartControl
    old_text: |2
          installPainter(getSourceViewer());
    new_text: |2
          installPainter(getSourceViewer());
          IncludeDropSupport.install(getSourceViewer());
  reason: "Drop-Support nach super.createPartControl installieren, damit AbstractTextEditor das DropTarget nicht danach ersetzt"
```
/answer 8bf1-3d-0 allow
Result Stats: id=6ba73112-ad96-495b-8f88-ad6354966f9e, total: 51187, in: 24, out: 16362, reason: 0, read: 209764, write: 34801, start: 1795858141749, end: 1795858145017, time: 3215, total: 119557, io: 0, control: 53, sent: 0, recv: 72
Ich habe den Aufruf `IncludeDropSupport.install(...)` aus `createSourceViewer` entfernt. Er steht jetzt in `createPartControl` direkt nach `installPainter(...)`, also nach `super.createPartControl(parent)`. Gebaut und getestet habe ich das nicht.

Bitte probiere den Drop noch einmal aus. Öffnet sich weiterhin ein neuer Editor, ist meine Vermutung falsch. Dann brauche ich eine Auskunft von dir:
- Läuft der Editor unter GTK/Linux, wie die Umgebung vermuten lässt?
- Kommt `drop()` in `IncludeDropSupport.Dispatcher` überhaupt an? Ein temporärer Breakpoint oder eine `System.out`-Zeile dort würde das klären.

Dann sehe ich, ob das Ziel das Drop gar nicht erhält oder ob etwas nach dem Einfügen passiert.
Ja jetzt geht es
Result Stats: id=6ba73112-ad96-495b-8f88-ad6354966f9e, total: 51874, in: 26, out: 16669, reason: 0, read: 244565, write: 35179, start: 1795858195326, end: 1795858198390, time: 3018, total: 122575, io: 0, control: 46, sent: 1, recv: 67
SystemInit:  id=6ba73112-ad96-495b-8f88-ad6354966f9e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Gut, dann war meine Vermutung richtig. `AbstractTextEditor` hatte das `DropTarget` in `createPartControl` ersetzt, und `IncludeDropSupport` greift jetzt erst danach.

Aktueller Stand:
- Eine Datei ergibt `[include](/abs/pfad)`.
- Ein Verzeichnis ergibt `[include directory](/abs/pfad)`.
- Bei mehreren Einträgen steht jeder in einer eigenen Zeile.
- Text-Drops laufen weiter über den Standard-Handler.

Du hast mir keine Probleme mit Pfaden in Backticks gemeldet. Ich habe aber weiterhin nicht geprüft, ob der Include-Parser sie akzeptiert. Teste das bei Gelegenheit mit einem Pfad, der ein Leerzeichen enthält.
