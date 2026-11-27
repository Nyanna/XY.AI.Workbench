Ich hätte gerne im `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java` Editor eine Autocompletion für die Syntax der Include Kommandos `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java`.

- Die Klassen dafür sollen in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest` organisiert werden
- `/home/user/xyan/xy.ai.workbench/plugin.xml`
SystemInit:  id=9aaec37c-b8b8-4eb6-8c14-62003f1c5b07, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst den Editor, die Include-Registry und die plugin.xml an.
ReasoningToken: 13
Control Request:
```yaml
id: 78b3-16-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: .*
    include:
    - '*.java'
    intent: survey
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: editor|Editor
    include:
    - plugin.xml
    exclude:
    - bin/**
    - target/**
    intent: survey
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include
    pattern: .*
    include:
    - '*.java'
    intent: survey
    limit: 50
  reason: "Editor, Registry und plugin.xml verstehen"
```
/answer 78b3-16-0 allow
Control Request:
```yaml
id: 78b3-17-16
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
        matches:
        - path: mdast/IDocumentBuffer.java
          matches:
          - package xy.ai.workbench.editor.mdast;
          - ""
          - import xy.ai.conscious.tools.LineIndex.Buffer;
          - "public interface IDocumentBuffer extends Buffer {"
          - "public void replace(int offset, int length, String text);"
          - "}"
        - path: mdast/MarkdownDocument.java
          matches:
          - package xy.ai.workbench.editor.mdast;
          - ""
          - import java.util.List;
          - import xy.ai.conscious.tools.LineIndex;
          - import xy.ai.conscious.tools.Scanner;
          - import xy.ai.workbench.editor.mdast.nodes.Elements;
          - import xy.ai.workbench.editor.mdast.nodes.Node;
          - "public class MarkdownDocument {"
          - private final IDocumentBuffer buffer;
          - private final LineIndex lines = new LineIndex();
          - "private Node root = new Node(null, Elements.ROOT);"
          - "public MarkdownDocument(IDocumentBuffer buffer) {"
          - this.buffer = buffer;
          - root.enableSpellcheck = true;
          - "}"
          - "public ModificationRange update(int offset, int removed, int inserted) {"
          - int lo = offset;
          - int hi = offset + removed;
          - int delta = inserted - removed;
          - "lines.update(buffer, offset, removed, inserted);"
          - "Node sec = findForUpdate(lo, hi);"
          - "while (true) {"
          - Node parent = sec.parent;
          - int absStart = sec.getOffset();
          - "int newLen = Math.max(hi - absStart, sec.length()) + delta;"
          - "Node rn = parse(absStart, absStart + newLen);"
          - "if (parent == null || isCompatible(rn.children, sec, parent)) {"
          - "Node changed = replace(sec, rn.children, delta);"
          - "return expand(changed, offset, offset + inserted);"
          - "// Fast path: only new siblings were appended at the tail that do not"
          - // fit into `parent` but belong to an ancestor. Splice them in instead
          - // of re-parsing the whole parent section on the next loop iteration.
          - "Node appended = appendSiblings(sec, parent, rn.children, delta);"
        warnings:
        - "Limit of 50 matches reached (total 4033); narrow pattern, directory or include/exclude filters."
      - directory:
        - /home/user/xyan/xy.ai.workbench
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include
        matches:
        - path: PathIncludeCommand.java
          matches:
          - package xy.ai.conscious.commands.include;
          - ""
          - import java.io.IOException;
          - import java.nio.file.Files;
          - import java.nio.file.Path;
          - import java.nio.file.Paths;
          - import java.util.ArrayList;
          - import java.util.List;
          - import xy.ai.conscious.harness.IncludeCycleException;
          - import xy.ai.conscious.harness.ISessionIncludeProcessor;
          - import xy.ai.conscious.harness.SessionCallbacks;
          - "public class PathIncludeCommand<M> implements IIncludeCommand<M> {"
          - '@Override'
          - "public String kind() {"
          - return null;
          - "}"
          - "public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {"
          - String path = IncludeArgs.path(arg);
          - if (path.isBlank())
          - throw new IllegalStateException("Include path must not be empty");
          - "includePath(path, chain, call, out, recursiv);"
          - "void includePath(String pathText, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {"
          - Path path = Paths.get(pathText).toAbsolutePath().normalize();
          - if (chain.contains(path))
          - throw new IncludeCycleException(path.toString());
          - String content;
          - "try {"
          - content = Files.readString(path);
          - "} catch (IOException e) {"
          - "throw new IllegalStateException(\"Could not read include target: \" + path, e);"
          - List<Path> nested = new ArrayList<>(chain);
          - nested.add(path);
          - "recursiv.processText(content, nested, call, out, recursiv);"
        - path: SearchIncludeCommand.java
          matches:
          - package xy.ai.conscious.commands.include;
          - ""
          - import java.nio.file.Path;
          - import java.util.List;
          - import com.fasterxml.jackson.databind.node.ArrayNode;
          - import com.fasterxml.jackson.databind.node.JsonNodeFactory;
          - import com.fasterxml.jackson.databind.node.ObjectNode;
        warnings:
        - "Limit of 50 matches reached (total 301); narrow pattern, directory or include/exclude filters."
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: s0G8wt|06AciC
          excert: import java.util.ArrayList;
        - id: DyZ6jU|Qe562U
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
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
        nodes:
        - id: Gwqkgi|FBrTeH
          excert: import java.nio.file.Path;
        - id: IncludeCommandRegistry
          excert: "public class IncludeCommandRegistry<M> {"
          children:
          - id: IncludeCommandRegistry.PUkHFX|YeJZ6z
            excert: "private final Map<String, IIncludeCommand<M>> commands = new HashMap<>();"
          - id: IncludeCommandRegistry.IncludeCommandRegistry
            excert: public IncludeCommandRegistry(IIncludeAdapter adapter)
          - id: IncludeCommandRegistry.register
            excert: public final void register(IIncludeCommand<M> command)
          - id: IncludeCommandRegistry.execute
            excert: "public void execute(String kind, String arg, List<Path> chain, SessionCallbacks…"
```
/answer 78b3-17-16 allow
Control Request:
```yaml
id: 78b3-18-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include
    pattern: kind\(\)|return "|class |interface
    include:
    - '*.java'
    intent: survey
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: org.eclipse.ui.editors|AITextEditor|<extension
    include:
    - plugin.xml
    - MANIFEST.MF
    exclude:
    - bin/**
    - target/**
    intent: survey
  reason: Quelltexte lesen
```
/answer 78b3-18-0 allow
Control Request:
```yaml
id: 78b3-19-18
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include
        matches:
        - path: PathIncludeCommand.java
          matches:
          - "public class PathIncludeCommand<M> implements IIncludeCommand<M> {"
          - "public String kind() {"
        - path: SearchIncludeCommand.java
          matches:
          - "public class SearchIncludeCommand<M> extends AdapterIncludeCommand<M> {"
          - "public String kind() {"
          - return "search";
        - path: FilesIncludeCommand.java
          matches:
          - "public class FilesIncludeCommand<M> extends AdapterIncludeCommand<M> {"
          - "public String kind() {"
          - return "files";
        - path: FileIncludeCommand.java
          matches:
          - "public class FileIncludeCommand<M> extends AdapterIncludeCommand<M> {"
          - "public String kind() {"
          - return "file";
        - path: DirectoryIncludeCommand.java
          matches:
          - "public class DirectoryIncludeCommand<M> extends AdapterIncludeCommand<M> {"
          - "public String kind() {"
          - return "directory";
        - path: IIncludeCommand.java
          matches:
          - "public interface IIncludeCommand<M> {"
          - String kind();
        - path: AdapterIncludeCommand.java
          matches:
          - "abstract class AdapterIncludeCommand<M> implements IIncludeCommand<M> {"
        - path: IncludeCommandRegistry.java
          matches:
          - "public class IncludeCommandRegistry<M> {"
          - "commands.put(command.kind(), command);"
        - path: ContextPromptIncludeCommand.java
          matches:
          - "public class ContextPromptIncludeCommand<M> extends AdapterIncludeCommand<M> {"
          - "public String kind() {"
          - return "contextprompt";
        - path: IncludeArgs.java
          matches:
          - "class IncludeArgs {"
      - directory:
        - /home/user/xyan/xy.ai.workbench
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: s0G8wt|06AciC
          excert: import java.util.ArrayList;
        - id: DyZ6jU|Qe562U
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
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
        nodes:
        - id: Gwqkgi|FBrTeH
          excert: import java.nio.file.Path;
        - id: IncludeCommandRegistry
          excert: "public class IncludeCommandRegistry<M> {"
          children:
          - id: IncludeCommandRegistry.PUkHFX|YeJZ6z
            excert: "private final Map<String, IIncludeCommand<M>> commands = new HashMap<>();"
          - id: IncludeCommandRegistry.IncludeCommandRegistry
            excert: public IncludeCommandRegistry(IIncludeAdapter adapter)
          - id: IncludeCommandRegistry.register
            excert: public final void register(IIncludeCommand<M> command)
          - id: IncludeCommandRegistry.execute
            excert: "public void execute(String kind, String arg, List<Path> chain, SessionCallbacks…"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java instead."
```
/answer 78b3-19-18 allow
ReasoningToken: 25
Control Request:
```yaml
id: 78b3-1a-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    ids:
    - s0G8wt|06AciC
    - AITextEditor
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
    ids:
    - IncludeCommandRegistry
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FilesIncludeCommand.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/SearchIncludeCommand.java
    node_type: class|interface
  reason: Vollständigen Quelltext der relevanten Knoten lesen
```
/answer 78b3-1a-0 allow
Control Request:
```yaml
id: 78b3-1b-1a
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
        nodes:
        - id: IncludeArgs
          type: ClassOrInterfaceDeclaration
          code: |
            class IncludeArgs {

              /** Removes surrounding whitespace and a leading and trailing backtick, e.g. {@code `/home/user/ToolResult.java`}. */
              static String path(String arg) {
                String s = arg.strip();
                if (s.startsWith("`"))
                  s = s.substring(1);
                if (s.endsWith("`"))
                  s = s.substring(0, s.length() - 1);
                return s.strip();
              }
            }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FilesIncludeCommand.java
        nodes:
        - id: FilesIncludeCommand
          type: ClassOrInterfaceDeclaration
          code: |
            public class FilesIncludeCommand<M> extends AdapterIncludeCommand<M> {

              public FilesIncludeCommand(IIncludeAdapter adapter) {
                super(adapter);
              }

              @Override
              public String kind() {
                return "files";
              }

              @Override
              public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                for (IIncludeAdapter.Entry entry : requireAdapter().files(IncludeArgs.path(arg))) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
              }
            }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/SearchIncludeCommand.java
        nodes:
        - id: SearchIncludeCommand
          type: ClassOrInterfaceDeclaration
          code: |
            public class SearchIncludeCommand<M> extends AdapterIncludeCommand<M> {

              private final YamlRenderer yaml = new YamlRenderer();

              public SearchIncludeCommand(IIncludeAdapter adapter) {
                super(adapter);
              }

              @Override
              public String kind() {
                return "search";
              }

              @Override
              public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                IIncludeAdapter a = requireAdapter();
                if ("files".equals(arg))
                  for (IIncludeAdapter.Entry entry : a.searchFiles()) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
                else if ("matches".equals(arg)) {
                  List<IIncludeAdapter.Match> matches = a.searchMatches();
                  out.add(call.toolResult(new ToolResult("search-matches", renderMatches(matches))));
                } else
                  throw new IllegalStateException("Unknown search include argument: " + arg);
              }

              private String renderMatches(List<IIncludeAdapter.Match> matches) {
                ArrayNode arr = JsonNodeFactory.instance.arrayNode();
                for (IIncludeAdapter.Match match : matches) {
                  ObjectNode node = arr.addObject();
                  node.put("file", match.file);
                  node.put("line", match.line);
                  node.put("text", match.text);
                }
                return yaml.toYaml(arr);
              }
            }
    read:
      results:
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
        - id: AITextEditor
          type: ClassOrInterfaceDeclaration
          excert: "public class AITextEditor extends TextEditor {"
          children:
          - id: AITextEditor.eiO9HI|Q1y79C
            type: StatementGroup
            code: |2
                private static final int LIMIT = 512 * 1024;
                private boolean rulerVisible = true;
                private boolean wordWrapSuppressed;
                private CompositeRuler ruler;
                private List<IVerticalRulerColumn> decorators = new ArrayList<>();
                private final EditorManager manager = new EditorManager();
                private MarkdownOutlinePage outlinePage;
          - id: AITextEditor.AITextEditor
            type: ConstructorDeclaration
            code: |2
                public AITextEditor() {
                  super();
                  setSourceViewerConfiguration(new AISourceViewerConfiguration(manager));
                }
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
          - id: AITextEditor.installPainter
            type: MethodDeclaration
            code: |2
                private void installPainter(ISourceViewer sourceViewer) {
                  Display display = sourceViewer.getTextWidget().getDisplay();
                  AnnotationPainter painter = new AnnotationPainter(sourceViewer, new DefaultMarkerAnnotationAccess());
                  painter.addTextStyleStrategy(SpellingAnnotation.TYPE, new AnnotationPainter.UnderlineStrategy(SWT.UNDERLINE_SQUIGGLE));
                  painter.addAnnotationType(SpellingAnnotation.TYPE, SpellingAnnotation.TYPE);
                  painter.setAnnotationTypeColor(SpellingAnnotation.TYPE, display.getSystemColor(SWT.COLOR_RED));
                  // addTextStyleStrategy works through ITextPresentationListener – register
                  // explicitly,
                  // because addPainter() alone does NOT do this registration.
                  // addTextPresentationListener is only on the concrete SourceViewer class, not
                  // on ISourceViewer.
                  if (sourceViewer instanceof SourceViewer)
                    ((SourceViewer) sourceViewer).addTextPresentationListener(painter);
                  ((ITextViewerExtension2) sourceViewer).addPainter(painter);
                }
          - id: AITextEditor.dispose
            type: MethodDeclaration
            code: |2
                @Override
                public void dispose() {
                  manager.uninstall();
                  super.dispose();
                }
          - id: AITextEditor.handleCaretMoved
            type: MethodDeclaration
            code: |2
                private void handleCaretMoved(int offset) {
                  if (outlinePage != null) {
                    var selection = getSourceViewer().getSelectedRange();
                    if (selection.y == 0)
                      Display.getDefault().asyncExec(() -> outlinePage.selectNodeForOffset(offset));
                  }
                }
          - id: AITextEditor.getAdapter
            type: MethodDeclaration
            code: |2
                @Override
                public <T> T getAdapter(Class<T> adapter) {
                  if (IContentOutlinePage.class.equals(adapter)) {
                    if (outlinePage == null)
                      outlinePage = new MarkdownOutlinePage(this);
                    return adapter.cast(outlinePage);
                  }
                  return super.getAdapter(adapter);
                }
          - id: AITextEditor.refreshOutline
            type: MethodDeclaration
            code: |2
                private void refreshOutline() {
                  if (outlinePage != null)
                    outlinePage.refresh();
                }
          - id: AITextEditor.selectAndRevealNode
            type: MethodDeclaration
            code: |2
                public void selectAndRevealNode(Node node) {
                  if (node == null)
                    return;
                  selectAndReveal(node.getOffset(), node.length());
                }
          - id: AITextEditor.getInitialWordWrapStatus
            type: MethodDeclaration
            code: |2
                @Override
                protected boolean getInitialWordWrapStatus() {
                  return true;
                }
          - id: AITextEditor.getUpdateManager
            type: MethodDeclaration
            code: |2
                public EditorManager getUpdateManager() {
                  return manager;
                }
          - id: AITextEditor.ManagerListener
            type: ClassOrInterfaceDeclaration
            excert: "private class ManagerListener implements IManagerListener {"
            children:
            - id: AITextEditor.ManagerListener.onDocumentChanged
              type: MethodDeclaration
              code: |2
                    @Override
                    public void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {
                      if (newDocument == null)
                        // clears the outline; onAstUpdated covers the non-null case
                        refreshOutline();
                    }
            - id: AITextEditor.ManagerListener.onAstUpdated
              type: MethodDeclaration
              code: |2
                    @Override
                    public void onAstUpdated(Node node) {
                      IDocument doc = manager.getDocument();
                      if (doc == null)
                        return;
                      updateWordWrap(doc);
                      updateRulerVisibility(doc);
                      updateLineNumbers(doc);
                      refreshOutline();
                    }
          - id: AITextEditor.updateWordWrap
            type: MethodDeclaration
            code: |2
                private void updateWordWrap(IDocument document) {
                  if (!(getSourceViewer().getTextWidget() instanceof StyledText widget))
                    return;
                  boolean large = document.getLength() > LIMIT;
                  if (large && !wordWrapSuppressed && widget.getWordWrap()) {
                    wordWrapSuppressed = true;
                    widget.setWordWrap(false);
                  } else if (!large && wordWrapSuppressed) {
                    wordWrapSuppressed = false;
                    widget.setWordWrap(true);
                  }
                }
          - id: AITextEditor.updateRulerVisibility
            type: MethodDeclaration
            code: |2
                private void updateRulerVisibility(IDocument document) {
                  boolean shouldShow = document.getLength() <= LIMIT;
                  if (shouldShow != rulerVisible) {
                    rulerVisible = shouldShow;
                    if (getSourceViewer() instanceof SourceViewer sv) {
                      sv.showAnnotations(shouldShow);
                      sv.showAnnotationsOverview(shouldShow);
                    }
                  }
                }
          - id: AITextEditor.updateLineNumbers
            type: MethodDeclaration
            code: |2
                private void updateLineNumbers(IDocument document) {
                  if (ruler == null)
                    return;
                  long size = document.getLength();
                  Iterator<IVerticalRulerColumn> it = ruler.getDecoratorIterator();
                  IVerticalRulerColumn d;
                  if (size > LIMIT && it.hasNext()) {
                    while (it.hasNext() && (d = it.next()) != null) decorators.add(d);
                    for (var dec : decorators) ruler.removeDecorator(dec);
                  } else if (size < LIMIT && !it.hasNext() && !decorators.isEmpty()) {
                    for (var i = 0; i < decorators.size(); i++) ruler.addDecorator(i, decorators.get(i));
                    decorators.clear();
                  }
                }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
        nodes:
        - id: IncludeCommandRegistry
          type: ClassOrInterfaceDeclaration
          excert: "public class IncludeCommandRegistry<M> {"
          children:
          - id: IncludeCommandRegistry.PUkHFX|YeJZ6z
            type: StatementGroup
            code: |2
                private final Map<String, IIncludeCommand<M>> commands = new HashMap<>();
          - id: IncludeCommandRegistry.IncludeCommandRegistry
            type: ConstructorDeclaration
            code: |2
                public IncludeCommandRegistry(IIncludeAdapter adapter) {
                  PathIncludeCommand<M> path = new PathIncludeCommand<>();
                  register(path);
                  register(new DirectoryIncludeCommand<>(adapter, path));
                  register(new ContextPromptIncludeCommand<>(adapter));
                  register(new FilesIncludeCommand<>(adapter));
                  register(new FileIncludeCommand<>(adapter));
                  register(new SearchIncludeCommand<>(adapter));
                }
          - id: IncludeCommandRegistry.register
            type: MethodDeclaration
            code: |2
                public final void register(IIncludeCommand<M> command) {
                  commands.put(command.kind(), command);
                }
          - id: IncludeCommandRegistry.execute
            type: MethodDeclaration
            code: |2
                public void execute(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                  IIncludeCommand<M> command = commands.get(kind);
                  if (command == null)
                    throw new IllegalStateException("Unknown include kind: " + kind);
                  command.execute(arg, chain, call, out, recursiv);
                }
```
/answer 78b3-1b-1a allow
ReasoningToken: 117
Control Request:
```yaml
id: 78b3-1c-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: "registry\\.execute|IncludeCommandRegistry|Pattern\\.compile.*(include|\\[)"
    include:
    - '*.java'
    intent: locate_for_edit
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: class AISourceViewerConfiguration|class EditorManager
    include:
    - '*.java'
    intent: locate_for_edit
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: Require-Bundle|org\.eclipse\.(jface|ui)|AITextEditor|<extension|<editor
    include:
    - plugin.xml
    - MANIFEST.MF
    exclude:
    - bin/**
    - target/**
    - connector/**
    intent: survey
    limit: 40
  reason: "Include-Syntax-Parser, Source-Viewer-Konfiguration und Plugin-Dateien finden"
```
/answer 78b3-1c-0 allow
Control Request:
```yaml
id: 78b3-1d-1c
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src
        matches:
        - path: xy/ai/conscious/model/AIAnswer.java
          nodes:
          - id: AIAnswer.j6EUkY|lLeXfI
            matches:
            - "private static final Pattern ID_PATTERN = Pattern.compile(\"id=([^,]*), \");"
        - path: xy/ai/conscious/core/IEditorInterface.java
          nodes:
          - id: IEditorInterface.gCM0q2|sDVECk
            matches:
            - "public static final Pattern PROMPT_TAG_PATTERN = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\", Pattern.CASE_INSENSITIVE);"
        - path: xy/ai/conscious/harness/IncludeProcessor.java
          nodes:
          - id: CJTOAQ|jn3SR2
            matches:
            - import xy.ai.conscious.commands.include.IncludeCommandRegistry;
          - id: IncludeProcessor.9ULz91|4YRM35
            matches:
            - "public static final Pattern INCLUDE_LINE = Pattern.compile(\"^(\\\\s*\\\\[include(?:\\\\s+\\\\w+)?\\\\]\\\\([^)]*\\\\)\\\\s*)+$\");"
            - "public static final Pattern INCLUDE_TAG = Pattern.compile(\"\\\\[include(?:\\\\s+(\\\\w+))?\\\\]\\\\(([^)]*)\\\\)\");"
            - private final IncludeCommandRegistry<M> commands;
          - id: IncludeProcessor.IncludeProcessor
            matches:
            - this.commands = new IncludeCommandRegistry<>(adapter);
        - path: xy/ai/conscious/autoapprove/SessionTracker.java
          nodes:
          - id: SessionTracker.U6jdLZ|v7Yqoi
            matches:
            - "private static final Pattern CONTROL_ID = Pattern.compile(\"(.+)-([0-9a-z]+)-([0-9a-z]+)\");"
        - path: xy/ai/conscious/connector/claudecode/YamlRenderer.java
          nodes:
          - id: YamlRenderer.Q4YiP4|YP22sB
            matches:
            - "private static final Pattern TRAILING_WS_PER_LINE = Pattern.compile(\"[ \\\\t]+(?=\\\\n|$)\");"
        - path: xy/ai/conscious/commands/include/IncludeCommandRegistry.java
          nodes:
          - id: IncludeCommandRegistry
            matches:
            - "public class IncludeCommandRegistry<M> {"
          - id: IncludeCommandRegistry.IncludeCommandRegistry
            matches:
            - "public IncludeCommandRegistry(IIncludeAdapter adapter) {"
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
        matches:
        - path: AISourceViewerConfiguration.java
          nodes:
          - id: AISourceViewerConfiguration
            matches:
            - "public class AISourceViewerConfiguration extends SourceViewerConfiguration {"
        - path: EditorManager.java
          nodes:
          - id: EditorManager
            matches:
            - "public class EditorManager {"
      - directory:
        - /home/user/xyan/xy.ai.workbench
```
/answer 78b3-1d-1c allow
ReasoningToken: 105
Control Request:
```yaml
id: 78b3-1e-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  - paths:
    - /home/user/xyan/xy.ai.workbench/plugin.xml
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/ContextPromptIncludeCommand.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FileIncludeCommand.java
    node_type: class|interface
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: Require-Bundle|org\.eclipse\.jface\.text|Bundle-ClassPath|Export-Package
    include:
    - MANIFEST.MF
    exclude:
    - bin/**
    - target/**
    intent: survey
  reason: "Source-Viewer-Konfiguration, plugin.xml und Include-Befehle lesen"
```
/answer 78b3-1e-0 allow
Control Request:
```yaml
id: 78b3-1f-1e
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
        nodes:
        - id: NRyWYJ|vuUYRl
          code: |
            import org.eclipse.jface.text.DocumentEvent;
            import org.eclipse.jface.text.IDocument;
            import org.eclipse.jface.text.IRegion;
            import org.eclipse.jface.text.ITypedRegion;
            import org.eclipse.jface.text.Region;
            import org.eclipse.jface.text.TextPresentation;
            import org.eclipse.jface.text.presentation.IPresentationReconciler;
            import org.eclipse.jface.text.presentation.PresentationReconciler;
            import org.eclipse.jface.text.quickassist.IQuickAssistAssistant;
            import org.eclipse.jface.text.quickassist.QuickAssistAssistant;
            import org.eclipse.jface.text.rules.DefaultDamagerRepairer;
            import org.eclipse.jface.text.rules.ITokenScanner;
            import org.eclipse.jface.text.source.ISourceViewer;
            import org.eclipse.jface.text.source.SourceViewerConfiguration;

            import xy.ai.workbench.editor.spellcheck.SpellingQuickAssistProcessor;
        - id: AISourceViewerConfiguration
          excert: "public class AISourceViewerConfiguration extends SourceViewerConfiguration {"
          children:
          - id: AISourceViewerConfiguration.Wfhy4U|AhQFQe
            code: "\tprivate static final int LIMIT = 2 * 512 * 1024;\n\n\tprivate AIRuleScanner rules = new AIRuleScanner();\n\tprivate DefaultDamagerRepairer dmg;\n"
          - id: AISourceViewerConfiguration.AISourceViewerConfiguration
            code: "\tpublic AISourceViewerConfiguration(EditorManager updateManager) {\n\t\trules.setUpdateManager(updateManager);\n\t\tdmg = new DamagerRepairer(rules, updateManager);\n\t}\n"
          - id: AISourceViewerConfiguration.getQuickAssistAssistant
            code: "\t@Override\n\tpublic IQuickAssistAssistant getQuickAssistAssistant(ISourceViewer sourceViewer) {\n\t\tQuickAssistAssistant assistant = new QuickAssistAssistant();\n\t\tassistant.setQuickAssistProcessor(new SpellingQuickAssistProcessor());\n\t\treturn assistant;\n\t}\n"
          - id: AISourceViewerConfiguration.getPresentationReconciler
            code: "\t@Override\n\tpublic IPresentationReconciler getPresentationReconciler(ISourceViewer sourceViewer) {\n\t\tPresentationReconciler reconciler = new PresentationReconciler();\n\n\t\trules.init(sourceViewer.getTextWidget().getFont());\n\t\treconciler.setDamager(dmg, IDocument.DEFAULT_CONTENT_TYPE);\n\t\treconciler.setRepairer(dmg, IDocument.DEFAULT_CONTENT_TYPE);\n\n\t\treturn reconciler;\n\t}\n"
          - id: AISourceViewerConfiguration.getDamager
            code: "\tpublic DefaultDamagerRepairer getDamager() {\n\t\treturn dmg;\n\t}\n"
          - id: AISourceViewerConfiguration.DamagerRepairer
            excert: "private static class DamagerRepairer extends DefaultDamagerRepairer {"
            children:
            - id: AISourceViewerConfiguration.DamagerRepairer.Ehq9yQ|uKgHv3
              code: "\t\tprivate EditorManager updateManager;\n"
            - id: AISourceViewerConfiguration.DamagerRepairer.DamagerRepairer
              code: "\t\tprivate DamagerRepairer(ITokenScanner scanner, EditorManager updateManager) {\n\t\t\tsuper(scanner);\n\t\t\tthis.updateManager = updateManager;\n\t\t}\n"
            - id: AISourceViewerConfiguration.DamagerRepairer.createPresentation
              code: "\t\t@Override\n\t\tpublic void createPresentation(TextPresentation presentation, ITypedRegion region) {\n\t\t\tif (fDocument != null && fDocument.getLength() > LIMIT)\n\t\t\t\taddRange(presentation, region.getOffset(), region.getLength(), AIRuleScanner.DEFAULT_ATTR);\n\t\t\telse\n\t\t\t\tsuper.createPresentation(presentation, region);\n\t\t}\n"
            - id: AISourceViewerConfiguration.DamagerRepairer.getDamageRegion
              code: "\t\t@Override\n\t\tpublic IRegion getDamageRegion(ITypedRegion partition, DocumentEvent e, boolean documentPartitioningChanged) {\n\t\t\tvar node = updateManager.getAst().find(e.getOffset(), e.getOffset() + e.getLength()).getNode();\n\t\t\t// gate fo only update manager changes\n\t\t\tif (node != null && e.getOffset() == node.getOffset() && e.getLength() == node.length())\n\t\t\t\treturn new Region(node.getOffset(), node.length());\n\t\t\treturn new Region(0, 0);\n\t\t}\n"
      - path: /home/user/xyan/xy.ai.workbench/plugin.xml
        nodes:
        - id: Wyr38L|R5Uiaf
          code: <?eclipse version="3.4"?>
        - id: plugin
          excert: <plugin>
          children:
          - id: plugin.extension
            code: "<extension\n\t\tpoint=\"org.eclipse.ui.editors\">\n\t\t<editor\n\t\t\tclass=\"xy.ai.workbench.editor.AISessionEditor\"\n\t\t\tcontributorClass=\"xy.ai.workbench.editor.AISessionEditorContributor\"\n\t\t\textensions=\"md\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.editor.AISessionEditor\"\n\t\t\tname=\"XY.AI Editor\"\n\t\t\tsymbolicFontName=\"org.eclipse.jface.bannerfont\">\n\t\t</editor>\n\t</extension>"
          - id: plugin.extension_1
            code: "<extension\n\t\tpoint=\"org.eclipse.help.contexts\">\n\t\t<contexts\n\t\t\tfile=\"contexts.xml\">\n\t\t</contexts>\n\t</extension>"
          - id: plugin.extension_2
            code: "<extension\n\t\tpoint=\"org.eclipse.ui.views\">\n\t\t<category\n\t\t\tid=\"XY.AI.Workbench\"\n\t\t\tname=\"AI\">\n\t\t</category>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.AISessionView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.AISessionView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"XY.Ai Session Settings\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.AIBatchView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.AIBatchView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"XY.AI Batch List\">\n\t\t</view>\n\t\t<view\n\t\t\tname=\"AI Explorer\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.explorer.MarkdownExplorerView\"\n\t\t\tid=\"xy.ai.workbench.view.AIExplorer\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.ClaudeCodeSessionView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.ClaudeCodeSessionView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"CC Sessions\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.diff.DiffPanel\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.diff.DiffPanel\"\n\t\t\tname=\"AI Diff\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.view.approver.AutoApprovePanel\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.view.approver.AutoApprovePanel\"\n\t\t\tname=\"AI Auto Approve\">\n\t\t</view>\n\t</extension>"
          - id: plugin.extension_3
            excert: <extension point="org.eclipse.ui.navigator.navigatorContent">
            children:
            - id: plugin.extension_3.h9f3c8435
              code: "<navigatorContent\n\t\t\tname=\"AI Content\"\n            contentProvider=\"xy.ai.workbench.view.explorer.MarkdownContentProvider\"\n\t\t\tlabelProvider=\"xy.ai.workbench.view.explorer.MarkdownLabelProvider\"\n\t\t\tid=\"xy.ai.workbench.view.content.AIContent\"\n\t\t\tpriority=\"normal\">\n\t\t\t<triggerPoints>\n\t\t\t\t<instanceof value=\"org.eclipse.core.resources.IResource\" />\n\t\t\t</triggerPoints>\n\t\t\t<possibleChildren>\n\t\t\t\t<instanceof value=\"org.eclipse.core.resources.IResource\" />\n\t\t\t</possibleChildren>\n\t       <commonWizard\n\t            type=\"new\"\n\t            wizardId=\"XY.AI.Workbench.wizard.newfile.chat\"\n\t            menuGroupId=\"new.ext\">\n            <enablement>\n                  <instanceof value=\"org.eclipse.core.resources.IFolder\"/>\n            </enablement>\n\t       </commonWizard>\n\t       <commonWizard\n\t            type=\"new\"\n\t            wizardId=\"XY.AI.Workbench.wizard.newfile.markdown\"\n\t            menuGroupId=\"new.ext\">\n            <enablement>\n                  <instanceof value=\"org.eclipse.core.resources.IFolder\"/>\n            </enablement>\n\t       </commonWizard>\n\t       <commonWizard\n\t            type=\"new\"\n\t            wizardId=\"XY.AI.Workbench.wizard.newfile.context\"\n\t            menuGroupId=\"new.ext\">\n            <enablement>\n                  <instanceof value=\"org.eclipse.core.resources.IFolder\"/>\n            </enablement>\n\t       </commonWizard>\n\t\t</navigatorContent>"
            - id: plugin.extension_3.h07328659
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.view.filter.GitIgnore\"\n\t\t\tname=\"Git Ignored Resources\"\n\t\t\tdescription=\"Hides resources excluded by a &quot;.gitignore&quot; file.\"\n\t\t\tclass=\"xy.ai.workbench.view.explorer.GitIgnoreViewerFilter\"\n\t\t\tactiveByDefault=\"true\">\n\t\t</commonFilter>"
            - id: plugin.extension_3.hdca604ea
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.view.filter.Project\"\n\t\t\tname=\"Filter to Focused Project\"\n\t\t\tdescription=\"Limits the content to the project of the focused editor.\"\n\t\t\tclass=\"xy.ai.workbench.view.explorer.ProjectFilter\"\n\t\t\tactiveByDefault=\"true\"\n\t\t\tvisibleInUI=\"true\">\n\t\t</commonFilter>"
          - id: plugin.extension_4
            excert: <extension point="org.eclipse.ui.navigator.navigatorContent">
            children:
            - id: plugin.extension_4.h6fa6eee6
              code: "<navigatorContent\n\t\t\tname=\"Markdown Outline Content\"\n\t\t\tcontentProvider=\"xy.ai.workbench.editor.outline.MarkdownNavigatorContentProvider\"\n\t\t\tlabelProvider=\"xy.ai.workbench.editor.outline.MarkdownNavigatorLabelProvider\"\n\t\t\tid=\"xy.ai.workbench.editor.outline.content.MarkdownOutline\"\n\t\t\tpriority=\"normal\">\n\t\t\t<triggerPoints>\n\t\t\t\t<or>\n\t\t\t\t\t<instanceof value=\"xy.ai.workbench.editor.mdast.MarkdownDocument\" />\n\t\t\t\t\t<instanceof value=\"xy.ai.workbench.editor.outline.NodeElement\" />\n\t\t\t\t</or>\n\t\t\t</triggerPoints>\n\t\t\t<possibleChildren>\n\t\t\t\t<instanceof value=\"xy.ai.workbench.editor.outline.NodeElement\" />\n\t\t\t</possibleChildren>\n\t\t</navigatorContent>"
            - id: plugin.extension_4.hc4daf174
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.ScriptBlock\"\n\t\t\tname=\"Script Blocks\"\n\t\t\tdescription=\"Hides fenced script block nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.ScriptBlockFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.h8c84a729
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.LineComment\"\n\t\t\tname=\"Line Comments\"\n\t\t\tdescription=\"Hides line comment nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.LineCommentFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.hd7f81d80
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.Paragraph\"\n\t\t\tname=\"Paragraphs\"\n\t\t\tdescription=\"Hides plain paragraph nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.ParagraphFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.h51faddcc
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.Headings\"\n\t\t\tname=\"Headings\"\n\t\t\tdescription=\"Hides heading section nodes of all levels (and their substructure).\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.HeadingFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.h24ff59ec
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.Thinking\"\n\t\t\tname=\"Agent Thinking\"\n\t\t\tdescription=\"Hides agent 'thinking' block nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.ThinkingFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.h41a2acc4
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.Text\"\n\t\t\tname=\"Agent Text\"\n\t\t\tdescription=\"Hides agent 'text' block nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.TextBlockFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.h53009cd0
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.ToolUse\"\n\t\t\tname=\"Agent Tool Use\"\n\t\t\tdescription=\"Hides agent 'tool use' block nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.ToolUseFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.hbd971038
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.ToolResult\"\n\t\t\tname=\"Agent Tool Result\"\n\t\t\tdescription=\"Hides agent 'tool result' block nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.ToolResultFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.h7212a6bc
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.ReasoningToken\"\n\t\t\tname=\"Agent Reasoning Tokens\"\n\t\t\tdescription=\"Hides agent reasoning token block nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.ReasoningTokenFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.hbfd099c8
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.TokenStats\"\n\t\t\tname=\"Agent Token Stats\"\n\t\t\tdescription=\"Hides agent token statistics block nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.TokenStatsFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.he73d652e
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.SystemInit\"\n\t\t\tname=\"Agent System Init\"\n\t\t\tdescription=\"Hides agent system-init block nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.SystemInitFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.hf0e90869
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.Answer\"\n\t\t\tname=\"Tool Answers\"\n\t\t\tdescription=\"Hides tool answer block nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.AnswerFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.h791c8228
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.ControlRequest\"\n\t\t\tname=\"Control Requests\"\n\t\t\tdescription=\"Hides control request section nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.ControlRequestFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.h8e79a293
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.Page\"\n\t\t\tname=\"Page\"\n\t\t\tdescription=\"Hides the page section node.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.PageFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.hfadbbed0
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.UserSection\"\n\t\t\tname=\"User Sections\"\n\t\t\tdescription=\"Hides chat 'user' section nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.UserSectionFilter\">\n\t\t</commonFilter>"
            - id: plugin.extension_4.hd8dd56c0
              code: "<commonFilter\n\t\t\tid=\"xy.ai.workbench.editor.outline.filter.AgentSection\"\n\t\t\tname=\"Agent Sections\"\n\t\t\tdescription=\"Hides chat 'agent' section nodes.\"\n\t\t\tclass=\"xy.ai.workbench.editor.outline.filter.AgentSectionFilter\">\n\t\t</commonFilter>"
          - id: plugin.extension_5
            code: "<extension point=\"org.eclipse.ui.navigator.viewer\">\n\t\t<viewer\n\t\t\tviewerId=\"xy.ai.workbench.editor.outline\">\n\t\t</viewer>\n\t\t<viewerContentBinding viewerId=\"xy.ai.workbench.editor.outline\">\n\t\t\t<includes>\n\t\t\t\t<contentExtension pattern=\"xy\\.ai\\.workbench\\.editor\\.outline\\..*\" />\n\t\t\t</includes>\n\t\t\t<excludes>\n\t\t\t\t<contentExtension pattern=\"^(?!xy\\.ai\\.workbench\\.editor\\.outline\\.).*\" />\n\t\t\t</excludes>\n\t\t</viewerContentBinding>\n\t</extension>"
          - id: plugin.extension_6
            code: "<extension point=\"org.eclipse.ui.navigator.viewer\">\n\t\t<viewer\n        inheritBindingsFromViewer=\"org.eclipse.ui.navigator.ProjectExplorer\"\n        popupMenuId=\"org.eclipse.ui.navigator.ProjectExplorer#PopupMenu\"\n        viewerId=\"xy.ai.workbench.view.AIExplorer\">\n  \t\t</viewer>\n\t   <viewerContentBinding viewerId=\"xy.ai.workbench.view.AIExplorer\">\n\t      <includes>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.content.AIContent\"/>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.filter.GitIgnore\"/>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.filter.Project\"/>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.linkHelper.AILinkHelper\"/>\n\t      </includes>\n\t      <excludes>\n\t\t      <contentExtension\n\t\t          pattern=\"^(?!xy\\.ai\\.workbench\\.view\\.(content\\.AIContent|filter\\.(GitIgnore|Project)|linkHelper\\.AILinkHelper)$).*\">\n\t\t      </contentExtension>\n\t      </excludes>\n\t   </viewerContentBinding>\n\t</extension>"
          - id: plugin.extension_7
            code: "<extension point=\"org.eclipse.ui.navigator.linkHelper\">\n\t\t<linkHelper\n\t\t\tclass=\"xy.ai.workbench.view.explorer.MarkdownLinkHelper\"\n\t\t\tid=\"xy.ai.workbench.view.linkHelper.AILinkHelper\">\n\t\t\t<selectionEnablement>\n\t\t\t\t<instanceof value=\"org.eclipse.core.resources.IFile\"/>\n\t\t\t</selectionEnablement>\n\t\t\t<editorInputEnablement>\n\t\t\t\t<instanceof value=\"org.eclipse.ui.IFileEditorInput\"/>\n\t\t\t</editorInputEnablement>\n\t\t</linkHelper>\n\t</extension>"
          - id: plugin.extension_8
            code: "<extension\n\t\tpoint=\"org.eclipse.ui.commands\">\n\t\t<category\n\t\t\tid=\"XY.AI.Workbench.commands.category\"\n\t\t\tname=\"AI\">\n\t\t</category>\n\t\t<command\n\t\t\tcategoryId=\"XY.AI.Workbench.commands.category\"\n\t\t\tid=\"XY.AI.Workbench.commands.submitPromptCommand\"\n\t\t\tname=\"Submit prompt\">\n\t\t</command>\n\t\t<command\n\t\t\tcategoryId=\"XY.AI.Workbench.commands.category\"\n\t\t\tid=\"XY.AI.Workbench.commands.editor.comment\"\n\t\t\tname=\"Toggle AI Comment\">\n\t\t</command>\n\t\t<command\n\t\t\tcategoryId=\"XY.AI.Workbench.commands.category\"\n\t\t\tid=\"XY.AI.Workbench.commands.copyPath\"\n\t\t\tname=\"Copy Path\">\n\t\t</command>\n\t</extension>"
          - id: plugin.extension_9
            code: "<extension\n\t\tpoint=\"org.eclipse.ui.handlers\">\n\t\t<handler\n\t\t\tclass=\"xy.ai.workbench.handlers.SessionExecuteHandler\"\n\t\t\tcommandId=\"XY.AI.Workbench.commands.submitPromptCommand\">\n\t\t</handler>\n\t\t<handler\n\t\t\tclass=\"xy.ai.workbench.editor.CommentHandler\"\n\t\t\tcommandId=\"XY.AI.Workbench.commands.editor.comment\">\n\t\t\t<activeWhen>\n\t\t\t\t<and>\n\t\t\t\t\t<with variable=\"activeEditorId\">\n\t\t\t\t\t\t<equals\n\t\t\t\t\t\t\tvalue=\"xy.ai.workbench.editor.AISessionEditor\">\n\t\t\t\t\t\t</equals>\n\t\t\t\t\t</with>\n\t\t\t\t\t<with variable=\"activeEditor\">\n\t\t\t\t\t\t<instanceof\n\t\t\t\t\t\t\tvalue=\"xy.ai.workbench.editor.AISessionEditor\" />\n\t\t\t\t\t</with>\n\t\t\t\t</and>\n\t\t\t</activeWhen>\n\t\t</handler>\n\t\t<handler\n\t\t\tclass=\"xy.ai.workbench.handlers.CopyPathHandler\"\n\t\t\tcommandId=\"XY.AI.Workbench.commands.copyPath\">\n\t\t</handler>\n\t</extension>"
          - id: plugin.extension_10
            code: "<extension\n\t\tpoint=\"org.eclipse.ui.bindings\">\n\t\t<key\n\t\t\tcommandId=\"XY.AI.Workbench.commands.submitPromptCommand\"\n\t\t\tcontextId=\"org.eclipse.ui.contexts.window\"\n\t\t\tschemeId=\"org.eclipse.ui.defaultAcceleratorConfiguration\"\n\t\t\tsequence=\"M1+Enter\">\n\t\t</key>\n\t\t<key\n\t\t\tcommandId=\"XY.AI.Workbench.commands.editor.comment\"\n\t\t\tcontextId=\"org.eclipse.ui.textEditorScope\"\n\t\t\tschemeId=\"org.eclipse.ui.defaultAcceleratorConfiguration\"\n\t\t\tsequence=\"Shift+Ctrl+C\">\n\t\t</key>\n\t</extension>"
          - id: plugin.extension_11
            excert: <extension point="org.eclipse.ui.menus">
            children:
            - id: plugin.extension_11.menuContribution
              code: "<menuContribution\n\t\t\tlocationURI=\"menu:org.eclipse.ui.main.menu?after=additions\">\n\t\t\t<menu\n\t\t\t\tid=\"XY.AI.Workbench.menus.aiMenu\"\n\t\t\t\tlabel=\"AI\"\n\t\t\t\tmnemonic=\"A\">\n\t\t\t\t<command\n\t\t\t\t\tcommandId=\"XY.AI.Workbench.commands.submitPromptCommand\"\n\t\t\t\t\ticon=\"icons/sample.png\"\n\t\t\t\t\tid=\"XY.AI.Workbench.menus.submitPromptCommand\"\n\t\t\t\t\tmnemonic=\"S\">\n\t\t\t\t</command>\n\t\t\t</menu>\n\t\t</menuContribution>"
            - id: plugin.extension_11.menuContribution_1
              code: "<menuContribution\n\t\t\tlocationURI=\"toolbar:org.eclipse.ui.main.toolbar?after=additions\">\n\t\t\t<toolbar\n\t\t\t\tid=\"XY.AI.Workbench.toolbars.aiToolbar\">\n\t\t\t\t<command\n\t\t\t\t\tcommandId=\"XY.AI.Workbench.commands.submitPromptCommand\"\n\t\t\t\t\ticon=\"icons/sample.png\"\n\t\t\t\t\tid=\"XY.AI.Workbench.toolbars.submitPromptCommand\"\n\t\t\t\t\ttooltip=\"Submit the current prompt\">\n\t\t\t\t</command>\n\t\t\t</toolbar>\n\t\t</menuContribution>"
            - id: plugin.extension_11.menuContribution_2
              code: "<menuContribution\n\t\t\tlocationURI=\"popup:org.eclipse.ui.popup.any?after=additions\">\n\t\t\t<command\n\t\t\t\tcommandId=\"XY.AI.Workbench.commands.copyPath\"\n\t\t\t\tlabel=\"Copy Path\"\n\t\t\t\tmnemonic=\"P\"\n\t\t\t\tstyle=\"push\">\n\t\t\t\t<visibleWhen\n\t\t\t\t\tcheckEnabled=\"false\">\n\t\t\t\t\t<with\n\t\t\t\t\t\tvariable=\"selection\">\n\t\t\t\t\t\t<iterate\n\t\t\t\t\t\t\tifEmpty=\"false\">\n\t\t\t\t\t\t\t<adapt\n\t\t\t\t\t\t\t\ttype=\"org.eclipse.core.resources.IResource\">\n\t\t\t\t\t\t\t</adapt>\n\t\t\t\t\t\t</iterate>\n\t\t\t\t\t</with>\n\t\t\t\t</visibleWhen>\n\t\t\t</command>\n\t\t</menuContribution>"
            - id: plugin.extension_11.menuContribution_3
              code: "<menuContribution\n\t\t\tlocationURI=\"popup:org.eclipse.ui.popup.any?after=additions\">\n\t\t\t<command\n\t\t\t\tcommandId=\"org.eclipse.ui.edit.rename\"\n\t\t\t\tlabel=\"Rename\"\n\t\t\t\tmnemonic=\"R\"\n\t\t\t\tstyle=\"push\">\n\t\t\t\t<visibleWhen\n\t\t\t\t\tcheckEnabled=\"false\">\n\t\t\t\t\t<with\n\t\t\t\t\t\tvariable=\"activePartId\">\n\t\t\t\t\t\t<equals\n\t\t\t\t\t\t\tvalue=\"xy.ai.workbench.view.AIExplorer\">\n\t\t\t\t\t\t</equals>\n\t\t\t\t\t</with>\n\t\t\t\t</visibleWhen>\n\t\t\t</command>\n\t\t</menuContribution>"
            - id: plugin.extension_11.menuContribution_4
              code: "<menuContribution\n\t\t\tlocationURI=\"popup:#TextEditorContext?after=additions\">\n\t\t\t<command\n\t\t\t\tcommandId=\"XY.AI.Workbench.commands.copyPath\"\n\t\t\t\tlabel=\"Copy Path\"\n\t\t\t\tmnemonic=\"P\"\n\t\t\t\tstyle=\"push\">\n\t\t\t</command>\n\t\t</menuContribution>"
          - id: plugin.extension_12
            code: "<extension\n\t\tpoint=\"org.eclipse.ui.perspectiveExtensions\">\n\t\t<perspectiveExtension\n\t\t\ttargetID=\"org.eclipse.jdt.ui.JavaPerspective\">\n\t\t\t<view\n\t\t\t\tid=\"xy.ai.workbench.view.AIBatchView\"\n\t\t\t\tratio=\"0.5\"\n\t\t\t\trelationship=\"bottom\"\n\t\t\t\trelative=\"org.eclipse.ui.views.ProblemView\">\n\t\t\t</view>\n\t\t</perspectiveExtension>\n\t</extension>"
          - id: plugin.extension_13
            code: "<extension\n\t\tpoint=\"org.eclipse.ui.fontDefinitions\">\n\t\t<fontDefinition\n\t\t\tid=\"XY.AI.Workbench.fontDefinition1\"\n\t\t\tlabel=\"XY.AI Editor Font\"\n\t\t\tvalue=\"1|Arial|10.0|0|WINDOWS|1|0|0|0|0|0|0|0|0|3|0|0|0|Arial;\">\n\t\t</fontDefinition>\n\t</extension>"
          - id: plugin.extension_xy_ai_workbench_promptmarker
            code: "<extension\n\t\tid=\"xy.ai.workbench.promptmarker\"\n\t\tname=\"XY.AI Prompt Marker\"\n\t\tpoint=\"org.eclipse.core.resources.markers\">\n\t\t<persistent\n\t\t\tvalue=\"true\">\n\t\t</persistent>\n\t\t<super\n\t\t\ttype=\"org.eclipse.core.resources.taskmarker\">\n\t\t</super>\n\t</extension>"
          - id: plugin.extension_14
            code: "<extension\n\t\tpoint=\"org.eclipse.ui.newWizards\">\n\t\t<wizard\n\t\t\tid=\"XY.AI.Workbench.wizard.newfile.markdown\"\n\t\t\tname=\"Markdown File\"\n\t\t\tclass=\"xy.ai.workbench.handlers.NewMarkdownWizard\"\n\t\t\tcategory=\"XY.AI.Workbench.newfile.category\"\n\t\t\ticon=\"icons/sample.png\">\n\t\t\t<description>\n\t\t\t\tCreates a new Markdown file.\n\t\t\t</description>\n\t\t</wizard>\n\t\t<wizard\n\t\t\tid=\"XY.AI.Workbench.wizard.newfile.context\"\n\t\t\tname=\"Prompt Context\"\n\t\t\tclass=\"xy.ai.workbench.handlers.NewPromptContextWizard\"\n\t\t\tcategory=\"XY.AI.Workbench.newfile.category\"\n\t\t\ticon=\"icons/sample.png\">\n\t\t\t<description>\n\t\t\t\tCreates a new Prompt Context file.\n\t\t\t</description>\n\t\t</wizard>\n\t\t<wizard\n\t\t\tid=\"XY.AI.Workbench.wizard.newfile.chat\"\n\t\t\tname=\"AI Chat\"\n\t\t\tclass=\"xy.ai.workbench.handlers.NewChatWizard\"\n\t\t\tcategory=\"XY.AI.Workbench.newfile.category\"\n\t\t\ticon=\"icons/sample.png\">\n\t\t\t<description>\n\t\t\t\tCreates a new AI Chat file.\n\t\t\t</description>\n\t\t</wizard>\n\t\t<category\n\t\t\tid=\"XY.AI.Workbench.newfile.category\"\n\t\t\tname=\"AI Files\">\n\t\t</category>\n\t</extension>"
          - id: plugin.extension_15
            code: "<extension point=\"org.eclipse.ui.editors.annotationTypes\">\n\t\t<type name=\"xy.ai.workbench.editor.spellcheck.spelling\"/>\n\t</extension>"
          - id: plugin.extension_16
            code: "<extension point=\"org.eclipse.e4.workbench.model\">\n\t\t<processor\n\t\t\t\tclass=\"xy.ai.workbench.layout.FixedWidthPanelAddonProcessor\"\n\t\t\t\tbeforefragment=\"false\">\n\t\t</processor>\n\t</extension>"
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
        nodes:
        - id: DirectoryIncludeCommand
          type: ClassOrInterfaceDeclaration
          code: |
            public class DirectoryIncludeCommand<M> extends AdapterIncludeCommand<M> {
              private final PathIncludeCommand<M> pathCommand;

              public DirectoryIncludeCommand(IIncludeAdapter adapter, PathIncludeCommand<M> pathCommand) {
                super(adapter);
                this.pathCommand = pathCommand;
              }

              @Override
              public String kind() {
                return "directory";
              }

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
            }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/ContextPromptIncludeCommand.java
        nodes:
        - id: ContextPromptIncludeCommand
          type: ClassOrInterfaceDeclaration
          code: |
            public class ContextPromptIncludeCommand<M> extends AdapterIncludeCommand<M> {

              public ContextPromptIncludeCommand(IIncludeAdapter adapter) {
                super(adapter);
              }

              @Override
              public String kind() {
                return "contextprompt";
              }

              @Override
              public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                String text = requireAdapter().contextPrompt(arg);
                if (text != null && !text.isBlank())
                  out.add(call.message(Role.User, text.strip()));
              }
            }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FileIncludeCommand.java
        nodes:
        - id: FileIncludeCommand
          type: ClassOrInterfaceDeclaration
          code: |
            public class FileIncludeCommand<M> extends AdapterIncludeCommand<M> {

              public FileIncludeCommand(IIncludeAdapter adapter) {
                super(adapter);
              }

              @Override
              public String kind() {
                return "file";
              }

              @Override
              public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                String path = IncludeArgs.path(arg);
                String content = requireAdapter().file(path);
                if (content != null)
                  out.add(call.toolResult(new ToolResult(path, content)));
              }
            }
```
/answer 78b3-1f-1e allow
ReasoningToken: 176
Control Request:
```yaml
id: 78b3-20-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness
    pattern: INCLUDE_TAG|group\(1\)|commands\.execute|kind
    include:
    - IncludeProcessor.java
    intent: retrieve_context
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: class AISessionEditor|extends AITextEditor|contextprompt|ContextPrompt
    include:
    - '*.java'
    intent: locate_for_edit
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: contextPrompt|searchFiles|selectedDirectories
    include:
    - IIncludeAdapter.java
    intent: retrieve_context
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: jface\.text|org\.eclipse\.core\.resources|Bundle-SymbolicName
    include:
    - MANIFEST.MF
    exclude:
    - '**/target/**'
    - '**/bin/**'
    intent: survey
  reason: "Kind-Behandlung im IncludeProcessor, AISessionEditor, Adapter und Manifest prüfen"
```
/answer 78b3-20-0 allow
Control Request:
```yaml
id: 78b3-21-20
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness
        matches:
        - path: IncludeProcessor.java
          nodes:
          - id: IncludeProcessor.9ULz91|4YRM35
            matches:
            - "public static final Pattern INCLUDE_TAG = Pattern.compile(\"\\\\[include(?:\\\\s+(\\\\w+))?\\\\]\\\\(([^)]*)\\\\)\");"
          - id: IncludeProcessor.include
            matches:
            - "public void include(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {"
            - "commands.execute(kind, arg, chain, call, out, recursiv);"
          - id: IncludeProcessor.processText
            matches:
            - Matcher m = INCLUDE_TAG.matcher(line);
            - "while (m.find()) include(m.group(1), m.group(2).strip(), chain, call, out, recursiv);"
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
        matches:
        - path: AISessionEditorContributor.java
          nodes:
          - id: AISessionEditorContributor
            matches:
            - "public class AISessionEditorContributor extends MultiPageEditorActionBarContributor {"
        - path: AISessionEditor.java
          nodes:
          - id: AISessionEditor
            matches:
            - "public class AISessionEditor extends MultiPageEditorPart implements IResourceChangeListener {"
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: connector/src/xy/ai/conscious/harness/IIncludeAdapter.java
          nodes:
          - id: IIncludeAdapter.contextPrompt
            matches:
            - String contextPrompt(String dir);
          - id: IIncludeAdapter.selectedDirectories
            matches:
            - List<Entry> selectedDirectories();
          - id: IIncludeAdapter.searchFiles
            matches:
            - List<Entry> searchFiles();
      - directory:
        - /home/user/xyan/xy.ai.workbench
```
/answer 78b3-21-20 allow
Thinking:
Die Syntax ist `[include kind](arg)`, mit den Kinds directory, contextprompt, files, file, search und kind-losen Pfaden; `plugin.xml` registriert aber `AISessionEditor` statt `AITextEditor`. Als Nächstes prüfe ich Manifest und Editor-Einbindung.


ReasoningToken: 519
Control Request:
```yaml
id: 78b3-22-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: Require-Bundle|org\.eclipse\.jface\.text|org\.eclipse\.core\.resources
    include:
    - '*.MF'
    - build.properties
    - pom.xml
    - '*.bnd'
    exclude:
    - '**/bin/**'
    - '**/target/**'
    - connector/**
    intent: survey
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: AITextEditor
    include:
    - '*.java'
    intent: survey
  reason: Manifest/Build-Dateien und Einbindung von AITextEditor in AISessionEditor finden
```
/answer 78b3-22-0 allow
Control Request:
```yaml
id: 78b3-23-22
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
        matches:
        - path: editor/AITextEditor.java
          matches:
          - "public class AITextEditor extends TextEditor {"
          - "public AITextEditor() {"
        - path: editor/outline/MarkdownOutlinePage.java
          matches:
          - import xy.ai.workbench.editor.AITextEditor;
          - private final AITextEditor editor;
          - "public MarkdownOutlinePage(AITextEditor editor) {"
        - path: editor/AISessionEditor.java
          matches:
          - private AITextEditor editor;
          - "public AITextEditor getEditor() {"
          - editor = new AITextEditor();
```
/answer 78b3-23-22 allow
ReasoningToken: 6054
Control Request:
```yaml
id: 78b3-24-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
    ids:
    - Gwqkgi|FBrTeH
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/AdapterIncludeCommand.java
    node_type: class|interface
  reason: Imports der Registry und Konstruktor von AdapterIncludeCommand prüfen (null-Adapter für Kind-Abfrage)
```
/answer 78b3-24-0 allow
Control Request:
```yaml
id: 78b3-25-24
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/AdapterIncludeCommand.java
        nodes:
        - id: AdapterIncludeCommand
          type: ClassOrInterfaceDeclaration
          code: |
            abstract class AdapterIncludeCommand<M> implements IIncludeCommand<M> {

              private final IIncludeAdapter adapter;

              protected AdapterIncludeCommand(IIncludeAdapter adapter) {
                this.adapter = adapter;
              }

              protected IIncludeAdapter requireAdapter() {
                if (adapter == null)
                  throw new IllegalStateException("No IIncludeAdapter configured for this SessionProcessor");
                return adapter;
              }
            }
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
        nodes:
        - id: Gwqkgi|FBrTeH
          type: ImportGroup
          code: |
            import java.nio.file.Path;
            import java.util.HashMap;
            import java.util.List;
            import java.util.Map;

            import xy.ai.conscious.harness.IIncludeAdapter;
            import xy.ai.conscious.harness.ISessionIncludeProcessor;
            import xy.ai.conscious.harness.SessionCallbacks;
```
/answer 78b3-25-24 allow
Text:
 Ein `null`-Adapter ist erlaubt. Ich kann die Kinds also direkt aus der Registry lesen und muss sie nicht im Editor duplizieren. Jetzt lege ich die Klassen an.
ReasoningToken: 46
Control Request:
```yaml
id: 78b3-26-0
toolName: ast_edit
arguments:
  insert:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
    id: IncludeCommandRegistry.register
    position: after
    source: |2

        /** Registered kinds in alphabetical order, without the kind-less path command. */
        public Set<String> kinds() {
          Set<String> kinds = new TreeSet<>();
          for (String kind : commands.keySet()) if (kind != null)
            kinds.add(kind);
          return kinds;
        }
    addImport:
    - import java.util.Set;
    - import java.util.TreeSet;
  create:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeContext.java
    source: |
      package xy.ai.workbench.editor.suggest;

      /**
       * Caret position inside an include tag {@code [include kind](arg)}.
       *
       * @param part which part of the tag is being edited
       * @param kind kind of the tag, {@code null} for the kind-less path form
       * @param prefix text already typed in the edited part
       * @param start document offset where the edited part starts
       * @param rest text of the line after the caret
       */
      record IncludeContext(Part part, String kind, String prefix, int start, String rest) {

        enum Part {
          /** Directly after {@code [}, before the keyword {@code include} is complete. */
          TAG,
          /** After {@code [include }, while typing the kind. */
          KIND,
          /** Between {@code (} and {@code )}. */
          ARGUMENT
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeContextParser.java
    source: |
      package xy.ai.workbench.editor.suggest;

      import java.util.regex.Matcher;
      import java.util.regex.Pattern;
      import org.eclipse.jface.text.BadLocationException;
      import org.eclipse.jface.text.IDocument;
      import org.eclipse.jface.text.IRegion;
      import xy.ai.workbench.editor.suggest.IncludeContext.Part;

      /**
       * Locates the caret within the include syntax. Include tags are only recognised
       * on lines that consist of include tags only, as the include processor does.
       */
      class IncludeContextParser {

        private static final String KEYWORD = "include";
        private static final Pattern COMPLETE_TAGS = Pattern.compile("(?:\\s*\\[include(?:\\s+\\w+)?\\]\\([^)]*\\))*\\s*");
        private static final Pattern ARGUMENT = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]*)");
        private static final Pattern HEADER = Pattern.compile("\\[(\\S*)(\\s+(\\w*))?");

        IncludeContext parse(IDocument document, int offset) {
          try {
            IRegion line = document.getLineInformationOfOffset(offset);
            String before = document.get(line.getOffset(), offset - line.getOffset());
            String rest = document.get(offset, line.getOffset() + line.getLength() - offset);
            return parse(before, rest, line.getOffset());
          } catch (BadLocationException e) {
            return null;
          }
        }

        IncludeContext parse(String before, String rest, int lineOffset) {
          Matcher head = COMPLETE_TAGS.matcher(before);
          head.lookingAt();
          int tagStart = head.end();
          String tag = before.substring(tagStart);
          if (!tag.startsWith("["))
            return null;

          Matcher argument = ARGUMENT.matcher(tag);
          if (argument.matches())
            return new IncludeContext(Part.ARGUMENT, argument.group(1), argument.group(2), lineOffset + tagStart + argument.start(2), rest);

          Matcher header = HEADER.matcher(tag);
          if (!header.matches())
            return null;
          String word = header.group(1);
          if (header.group(2) == null)
            return KEYWORD.startsWith(word) ? new IncludeContext(Part.TAG, null, word, lineOffset + tagStart + 1, rest) : null;
          if (!KEYWORD.equals(word))
            return null;
          return new IncludeContext(Part.KIND, null, header.group(3), lineOffset + before.length() - header.group(3).length(), rest);
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/PathSuggestions.java
    source: |
      package xy.ai.workbench.editor.suggest;

      import java.io.IOException;
      import java.nio.file.Files;
      import java.nio.file.InvalidPathException;
      import java.nio.file.Path;
      import java.nio.file.Paths;
      import java.util.Comparator;
      import java.util.List;
      import java.util.Locale;
      import java.util.stream.Collectors;
      import java.util.stream.Stream;

      /** Lists file system entries for a partially typed include path. */
      class PathSuggestions {

        private static final int MAX_ENTRIES = 200;

        record Entry(Path path, boolean directory) {
          String name() {
            return path.getFileName() == null ? path.toString() : path.getFileName().toString();
          }
        }

        /**
         * @param typed path text typed so far, without a leading backtick
         * @param baseDirectory directory for relative input, may be {@code null}
         * @param directoriesOnly whether files are excluded
         */
        List<Entry> list(String typed, Path baseDirectory, boolean directoriesOnly) {
          int separator = Math.max(typed.lastIndexOf('/'), typed.lastIndexOf('\\'));
          String directoryText = typed.substring(0, separator + 1);
          String namePrefix = typed.substring(separator + 1).toLowerCase(Locale.ROOT);
          Path directory = resolve(directoryText, baseDirectory);
          if (directory == null || !Files.isDirectory(directory))
            return List.of();
          boolean hidden = namePrefix.startsWith(".");
          try (Stream<Path> children = Files.list(directory)) {
            return children.map(p -> new Entry(p.toAbsolutePath().normalize(), Files.isDirectory(p)))
                .filter(e -> !directoriesOnly || e.directory())
                .filter(e -> e.name().toLowerCase(Locale.ROOT).startsWith(namePrefix))
                .filter(e -> hidden || !e.name().startsWith("."))
                .sorted(Comparator.comparing((Entry e) -> !e.directory()).thenComparing(e -> e.name().toLowerCase(Locale.ROOT)))
                .limit(MAX_ENTRIES)
                .collect(Collectors.toList());
          } catch (IOException e) {
            return List.of();
          }
        }

        private Path resolve(String directoryText, Path baseDirectory) {
          try {
            if (directoryText.isEmpty())
              return baseDirectory;
            Path directory = Paths.get(directoryText);
            if (directory.isAbsolute())
              return directory;
            return baseDirectory == null ? null : baseDirectory.resolve(directory);
          } catch (InvalidPathException e) {
            return null;
          }
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeSyntax.java
    source: |
      package xy.ai.workbench.editor.suggest;

      import java.util.List;
      import java.util.Map;
      import java.util.Set;
      import xy.ai.conscious.commands.include.IncludeCommandRegistry;

      /** What the editor knows about the arguments of each include kind. */
      class IncludeSyntax {

        enum Argument {
          FILE, DIRECTORY, FIXED
        }

        private static final Map<String, Argument> ARGUMENTS = Map.of(
            "file", Argument.FILE,
            "files", Argument.FILE,
            "directory", Argument.DIRECTORY,
            "contextprompt", Argument.DIRECTORY,
            "search", Argument.FIXED);

        private static final Map<String, List<String>> FIXED_VALUES = Map.of("search", List.of("files", "matches"));

        private final Set<String> kinds = new IncludeCommandRegistry<Object>(null).kinds();

        Set<String> kinds() {
          return kinds;
        }

        /** Kinds not listed above, including the kind-less form, take a file path. */
        Argument argument(String kind) {
          return ARGUMENTS.getOrDefault(kind, Argument.FILE);
        }

        List<String> fixedValues(String kind) {
          return FIXED_VALUES.getOrDefault(kind, List.of());
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
    source: |
      package xy.ai.workbench.editor.suggest;

      import java.io.File;
      import java.nio.file.Path;
      import java.util.ArrayList;
      import java.util.List;
      import java.util.function.Supplier;
      import org.eclipse.jface.text.ITextViewer;
      import org.eclipse.jface.text.contentassist.CompletionProposal;
      import org.eclipse.jface.text.contentassist.ICompletionProposal;
      import org.eclipse.jface.text.contentassist.IContentAssistProcessor;
      import org.eclipse.jface.text.contentassist.IContextInformation;
      import org.eclipse.jface.text.contentassist.IContextInformationValidator;
      import xy.ai.workbench.editor.suggest.IncludeContext.Part;
      import xy.ai.workbench.editor.suggest.IncludeSyntax.Argument;

      /** Content assist for {@code [include kind](argument)} tags. */
      public class IncludeCompletionProcessor implements IContentAssistProcessor {

        private static final char[] ACTIVATION = { ' ', '(', '/', '\\' };

        private final IncludeContextParser parser = new IncludeContextParser();
        private final IncludeSyntax syntax = new IncludeSyntax();
        private final PathSuggestions paths = new PathSuggestions();
        private final Supplier<Path> baseDirectory;

        /** @param baseDirectory directory that relative paths are resolved against, may return {@code null} */
        public IncludeCompletionProcessor(Supplier<Path> baseDirectory) {
          this.baseDirectory = baseDirectory;
        }

        @Override
        public ICompletionProposal[] computeCompletionProposals(ITextViewer viewer, int offset) {
          IncludeContext context = parser.parse(viewer.getDocument(), offset);
          if (context == null)
            return null;
          List<ICompletionProposal> proposals = switch (context.part()) {
            case TAG -> tagProposals(context, offset);
            case KIND -> kindProposals(context, offset);
            case ARGUMENT -> argumentProposals(context, offset);
          };
          return proposals.isEmpty() ? null : proposals.toArray(new ICompletionProposal[0]);
        }

        private List<ICompletionProposal> tagProposals(IncludeContext context, int offset) {
          List<ICompletionProposal> result = new ArrayList<>();
          result.add(tag(context, offset, null));
          for (String kind : syntax.kinds()) result.add(tag(context, offset, kind));
          return result;
        }

        private ICompletionProposal tag(IncludeContext context, int offset, String kind) {
          String text = "include" + (kind == null ? "" : " " + kind) + "](" + closing(context.rest());
          String display = kind == null ? "include (path)" : "include " + kind;
          return new CompletionProposal(text, context.start(), offset - context.start(), text.indexOf('(') + 1, null, display, null, null);
        }

        private List<ICompletionProposal> kindProposals(IncludeContext context, int offset) {
          List<ICompletionProposal> result = new ArrayList<>();
          boolean closed = context.rest().startsWith("]");
          for (String kind : syntax.kinds()) {
            if (!kind.startsWith(context.prefix()))
              continue;
            String text = closed ? kind : kind + "](" + closing(context.rest());
            int cursor = closed ? text.length() : text.indexOf('(') + 1;
            result.add(new CompletionProposal(text, context.start(), offset - context.start(), cursor, null, kind, null, null));
          }
          return result;
        }

        private List<ICompletionProposal> argumentProposals(IncludeContext context, int offset) {
          String kind = context.kind();
          Argument argument = syntax.argument(kind);
          return argument == Argument.FIXED ? fixedProposals(context, offset) : pathProposals(context, offset, argument == Argument.DIRECTORY);
        }

        private List<ICompletionProposal> fixedProposals(IncludeContext context, int offset) {
          List<ICompletionProposal> result = new ArrayList<>();
          String typed = context.prefix().strip();
          int start = offset - context.prefix().length();
          for (String value : syntax.fixedValues(context.kind())) {
            if (!value.startsWith(typed))
              continue;
            String text = value + closing(context.rest());
            result.add(new CompletionProposal(text, start, context.prefix().length(), text.length(), null, value, null, null));
          }
          return result;
        }

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

        /** Whitespace and one backtick, which {@code IncludeArgs.path} strips as well. */
        private int leadingNoise(String prefix) {
          int i = 0;
          while (i < prefix.length() && Character.isWhitespace(prefix.charAt(i))) i++;
          if (i < prefix.length() && prefix.charAt(i) == '`')
            i++;
          return i;
        }

        private String closing(String rest) {
          return rest.startsWith(")") ? "" : ")";
        }

        @Override
        public IContextInformation[] computeContextInformation(ITextViewer viewer, int offset) {
          return null;
        }

        @Override
        public char[] getCompletionProposalAutoActivationCharacters() {
          return ACTIVATION;
        }

        @Override
        public char[] getContextInformationAutoActivationCharacters() {
          return null;
        }

        @Override
        public String getErrorMessage() {
          return null;
        }

        @Override
        public IContextInformationValidator getContextInformationValidator() {
          return null;
        }
      }
  reason: Registry um kinds() erweitern; neue Suggest-Klassen anlegen
```
/answer 78b3-26-0 allow
Control Request:
```yaml
id: 78b3-27-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeSyntax.java
    id: IncludeSyntax.argument
    old_text: "    return ARGUMENTS.getOrDefault(kind, Argument.FILE);"
    new_text: "    return kind == null ? Argument.FILE : ARGUMENTS.getOrDefault(kind, Argument.FILE);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeSyntax.java
    id: IncludeSyntax.fixedValues
    old_text: "    return FIXED_VALUES.getOrDefault(kind, List.of());"
    new_text: "    return kind == null ? List.of() : FIXED_VALUES.getOrDefault(kind, List.of());"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeContext.java
    id: IncludeContext
    old_text: "record IncludeContext(Part part,"
    new_text: "record IncludeContext(IncludeContext.Part part,"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
    id: IncludeCompletionProcessor
    old_text: |
      import xy.ai.workbench.editor.suggest.IncludeContext.Part;
    new_text: ""
  reason: "Null-sichere Lookups in IncludeSyntax, qualifizierter Enum-Typ im Record, ungenutzten Import entfernen"
```
/answer 78b3-27-0 allow
ReasoningToken: 1797
Control Request:
```yaml
id: 78b3-28-27
result:
  structuredContent:
    edit_block:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeSyntax.java
        id: IncludeSyntax.argument
        new_id: IncludeSyntax.argument
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeSyntax.java
        id: IncludeSyntax.fixedValues
        new_id: IncludeSyntax.fixedValues
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeContext.java
        id: IncludeContext
        new_id: IncludeContext
      errors:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
        id: IncludeCompletionProcessor
        error: Text not found in node.
        reason: not_found
        next_step: reread_node
```
/answer 78b3-28-27 allow
Control Request:
```yaml
id: 78b3-29-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: xy\.ai\.conscious\.commands
    include:
    - '*.java'
    - '*.MF'
    - '*.xml'
    - '*.properties'
    - .classpath
    exclude:
    - '**/bin/**'
    - '**/target/**'
    - connector/src/**
    intent: survey
    limit: 10
  reason: "Prüfen, ob das Paket commands.include aus dem Plugin bereits verwendet/exportiert wird"
```
/answer 78b3-29-0 allow
Control Request:
```yaml
id: 78b3-2a-29
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: src/xy/ai/workbench/editor/mdast/nodes/Elements.java
          matches:
          - import xy.ai.conscious.commands.AnswerCommand;
        - path: src/xy/ai/workbench/editor/suggest/IncludeSyntax.java
          matches:
          - import xy.ai.conscious.commands.include.IncludeCommandRegistry;
        - path: src/xy/ai/workbench/editor/AIRuleScanner.java
          matches:
          - import xy.ai.conscious.commands.AnswerCommand;
        - path: src/xy/ai/workbench/eclipse/GitSupport.java
          matches:
          - import xy.ai.conscious.commands.AnswerCommand;
          - import xy.ai.conscious.commands.Command;
        - path: connector/src/xy/ai/conscious/harness/SessionRenderer.java
          matches:
          - import xy.ai.conscious.commands.CallCommand;
        - path: connector/src/xy/ai/conscious/harness/SessionProcessor.java
          matches:
          - import xy.ai.conscious.commands.Command;
          - import xy.ai.conscious.commands.CommandRegistry;
        - path: connector/src/xy/ai/conscious/harness/PromptHandler.java
          matches:
          - import xy.ai.conscious.commands.AnswerCommand;
          - import xy.ai.conscious.commands.CallCommand;
        warnings:
        - "Limit of 10 matches reached (total 56); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 78b3-2a-29 allow
Control Request:
```yaml
id: 78b3-2b-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
    id: IncludeCompletionProcessor
    old_text: "private static final char[] ACTIVATION = { ' ', '(', '/', '\\\\' };"
    new_text: "private static final char[] ACTIVATION = { ' ', '(', '/', '\\\\' };"
    removeImport:
    - import xy.ai.workbench.editor.suggest.IncludeContext.Part;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    id: AISourceViewerConfiguration.AISourceViewerConfiguration
    old_text: "\tpublic AISourceViewerConfiguration(EditorManager updateManager) {\n\t\trules.setUpdateManager(updateManager);"
    new_text: "\tpublic AISourceViewerConfiguration(EditorManager updateManager, Supplier<Path> baseDirectory) {\n\t\tthis.baseDirectory = baseDirectory;\n\t\trules.setUpdateManager(updateManager);"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    id: AISourceViewerConfiguration.Wfhy4U|AhQFQe
    old_text: "\tprivate DefaultDamagerRepairer dmg;"
    new_text: "\tprivate DefaultDamagerRepairer dmg;\n\tprivate final Supplier<Path> baseDirectory;"
  insert:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    id: AISourceViewerConfiguration.getQuickAssistAssistant
    position: after
    source: "\n\t@Override\n\tpublic IContentAssistant getContentAssistant(ISourceViewer sourceViewer) {\n\t\tContentAssistant assistant = new ContentAssistant();\n\t\tassistant.setContentAssistProcessor(new IncludeCompletionProcessor(baseDirectory), IDocument.DEFAULT_CONTENT_TYPE);\n\t\tassistant.enableAutoActivation(true);\n\t\tassistant.setAutoActivationDelay(200);\n\t\tassistant.setInformationControlCreator(getInformationControlCreator(sourceViewer));\n\t\treturn assistant;\n\t}\n"
    addImport:
    - import java.nio.file.Path;
    - import java.util.function.Supplier;
    - import org.eclipse.jface.text.contentassist.ContentAssistant;
    - import org.eclipse.jface.text.contentassist.IContentAssistant;
    - import xy.ai.workbench.editor.suggest.IncludeCompletionProcessor;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.getUpdateManager
    position: after
    source: |2

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
    addImport:
    - import java.nio.file.Path;
    - import org.eclipse.core.runtime.IPath;
    - import org.eclipse.ui.IEditorInput;
    - import org.eclipse.ui.IFileEditorInput;
    - import org.eclipse.ui.IURIEditorInput;
  reason: Ungenutzten Part-Import im Processor entfernen; Content Assist in Viewer-Konfiguration und Editor einhängen
```
/answer 78b3-2b-0 allow
ReasoningToken: 110
Control Request:
```yaml
id: 78b3-2c-2b
result:
  structuredContent:
    insert:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
        id: AISourceViewerConfiguration.getQuickAssistAssistant
        inserted: 1
        ids:
        - AISourceViewerConfiguration.getContentAssistant
        imports:
          added:
          - import java.nio.file.Path;
          - import java.util.function.Supplier;
          - import org.eclipse.jface.text.contentassist.ContentAssistant;
          - import org.eclipse.jface.text.contentassist.IContentAssistant;
          - import xy.ai.workbench.editor.suggest.IncludeCompletionProcessor;
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        id: AITextEditor.getUpdateManager
        inserted: 1
        ids:
        - AITextEditor.baseDirectory
        imports:
          added:
          - import java.nio.file.Path;
          - import org.eclipse.core.runtime.IPath;
          - import org.eclipse.ui.IEditorInput;
          - import org.eclipse.ui.IFileEditorInput;
          - import org.eclipse.ui.IURIEditorInput;
    edit_block:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/suggest/IncludeCompletionProcessor.java
        id: IncludeCompletionProcessor
        new_id: IncludeCompletionProcessor
        imports:
          removed:
          - import xy.ai.workbench.editor.suggest.IncludeContext.Part;
      errors:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
        id: AISourceViewerConfiguration.AISourceViewerConfiguration
        error: Text not found in node; its content appears to have changed.
        reason: content_changed
        guess: |2-
            public AISourceViewerConfiguration(EditorManager updateManager) {
              rules.setUpdateManager(updateManager);
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
        id: AISourceViewerConfiguration.Wfhy4U|AhQFQe
        error: Text not found in node; its content appears to have changed.
        reason: content_changed
        guess: '  private DefaultDamagerRepairer dmg;'
```
/answer 78b3-2c-2b allow
Control Request:
```yaml
id: 78b3-2d-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.AITextEditor
    old_text: new AISourceViewerConfiguration(manager)
    new_text: "new AISourceViewerConfiguration(manager, this::baseDirectory)"
  reason: Konstruktor des Editors an neue Konfiguration anpassen
```
/answer 78b3-2d-0 allow
Text:
 Die Datei wurde beim Einfügen neu formatiert (Tabs zu Spaces). Deshalb wiederhole ich die beiden fehlgeschlagenen Edits.
Control Request:
```yaml
id: 78b3-2e-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    id: AISourceViewerConfiguration.AISourceViewerConfiguration
    old_text: "public AISourceViewerConfiguration(EditorManager updateManager) {"
    new_text: |-
      public AISourceViewerConfiguration(EditorManager updateManager, Supplier<Path> baseDirectory) {
          this.baseDirectory = baseDirectory;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    id: AISourceViewerConfiguration.Wfhy4U|AhQFQe
    old_text: private DefaultDamagerRepairer dmg;
    new_text: |-
      private DefaultDamagerRepairer dmg;
        private final Supplier<Path> baseDirectory;
  reason: Konstruktor und Feld in AISourceViewerConfiguration um baseDirectory erweitern
```
/answer 78b3-2e-0 allow Die AST normalisieren den Code
ReasoningToken: 92
Control Request:
```yaml
id: 78b3-2f-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    name: AISourceViewerConfiguration
    node_type: ClassOrInterfaceDeclaration
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: new AISourceViewerConfiguration\(
    include:
    - '*.java'
    intent: check_existence
  reason: "Ergebnis in Konfiguration, Editor und Registry prüfen sowie weitere Aufrufer des alten Konstruktors suchen"
```
/answer 78b3-2f-0 allow
Control Request:
```yaml
id: 78b3-30-2f
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src
        matches:
        - path: xy/ai/workbench/editor/AITextEditor.java
          matches:
          - "setSourceViewerConfiguration(new AISourceViewerConfiguration(manager, this::baseDirectory));"
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
        nodes:
        - id: AISourceViewerConfiguration
          type: ClassOrInterfaceDeclaration
          code: |
            public class AISourceViewerConfiguration extends SourceViewerConfiguration {
              private static final int LIMIT = 2 * 512 * 1024;
              private AIRuleScanner rules = new AIRuleScanner();
              private DefaultDamagerRepairer dmg;
              private final Supplier<Path> baseDirectory;

              public AISourceViewerConfiguration(EditorManager updateManager, Supplier<Path> baseDirectory) {
                this.baseDirectory = baseDirectory;
                rules.setUpdateManager(updateManager);
                dmg = new DamagerRepairer(rules, updateManager);
              }

              @Override
              public IQuickAssistAssistant getQuickAssistAssistant(ISourceViewer sourceViewer) {
                QuickAssistAssistant assistant = new QuickAssistAssistant();
                assistant.setQuickAssistProcessor(new SpellingQuickAssistProcessor());
                return assistant;
              }

              @Override
              public IContentAssistant getContentAssistant(ISourceViewer sourceViewer) {
                ContentAssistant assistant = new ContentAssistant();
                assistant.setContentAssistProcessor(new IncludeCompletionProcessor(baseDirectory), IDocument.DEFAULT_CONTENT_TYPE);
                assistant.enableAutoActivation(true);
                assistant.setAutoActivationDelay(200);
                assistant.setInformationControlCreator(getInformationControlCreator(sourceViewer));
                return assistant;
              }

              @Override
              public IPresentationReconciler getPresentationReconciler(ISourceViewer sourceViewer) {
                PresentationReconciler reconciler = new PresentationReconciler();
                rules.init(sourceViewer.getTextWidget().getFont());
                reconciler.setDamager(dmg, IDocument.DEFAULT_CONTENT_TYPE);
                reconciler.setRepairer(dmg, IDocument.DEFAULT_CONTENT_TYPE);
                return reconciler;
              }

              public DefaultDamagerRepairer getDamager() {
                return dmg;
              }

              private static class DamagerRepairer extends DefaultDamagerRepairer {
                private EditorManager updateManager;

                private DamagerRepairer(ITokenScanner scanner, EditorManager updateManager) {
                  super(scanner);
                  this.updateManager = updateManager;
                }

                @Override
                public void createPresentation(TextPresentation presentation, ITypedRegion region) {
                  if (fDocument != null && fDocument.getLength() > LIMIT)
                    addRange(presentation, region.getOffset(), region.getLength(), AIRuleScanner.DEFAULT_ATTR);
                  else
                    super.createPresentation(presentation, region);
                }

                @Override
                public IRegion getDamageRegion(ITypedRegion partition, DocumentEvent e, boolean documentPartitioningChanged) {
                  var node = updateManager.getAst().find(e.getOffset(), e.getOffset() + e.getLength()).getNode();
                  // gate fo only update manager changes
                  if (node != null && e.getOffset() == node.getOffset() && e.getLength() == node.length())
                    return new Region(node.getOffset(), node.length());
                  return new Region(0, 0);
                }
              }
            }
```
/answer 78b3-30-2f allow
Result Stats: id=9aaec37c-b8b8-4eb6-8c14-62003f1c5b07, total: 86770, in: 26, out: 23803, reason: 124, read: 399447, write: 62941, start: 1795813547252, end: 1795813558096, time: 10769, total: 160328, io: 0, control: 75, sent: 0, recv: 269
ReasoningToken: 124
Die Autocompletion für die Include-Syntax ist im Editor eingebaut. Kompiliert oder im Editor getestet habe ich sie nicht.

Sie wird mit Strg+Space ausgelöst. Automatisch erscheint sie nach Leerzeichen, `(` und `/`. Sie greift nur in Zeilen, die ausschließlich aus Include-Tags bestehen, so wie `IncludeProcessor.INCLUDE_LINE` es verlangt. Normale Markdown-Links werden also nicht beeinflusst.

Vorgeschlagen wird in drei Situationen:
- **Nach `[`:** `include (path)` und `include <kind>` für jedes Kind. Eingefügt wird `include kind](` und `)`, der Cursor steht zwischen den Klammern.
- **Nach `[include `:** die Kinds, gefiltert nach dem bereits Getippten.
- **In `( … )`:**
  - `search` schlägt `files` und `matches` vor.
  - `directory` und `contextprompt` schlagen Verzeichnisse vor.
  - `file`, `files` und das Tag ohne Kind schlagen Dateien und Verzeichnisse vor.

Bei Pfaden ersetzt ein Vorschlag auch einen relativ getippten Anfang durch den absoluten Pfad, weil `PathIncludeCommand` gegen das Arbeitsverzeichnis des Prozesses auflöst. Relativ getippte Pfade werden gegen das Verzeichnis der bearbeiteten Datei aufgelöst. Ein führender Backtick und Leerraum bleiben unberührt, das entspricht `IncludeArgs.path`.

**Neue Klassen** in `editor/suggest`:
- `IncludeCompletionProcessor` ist der Content-Assist-Processor.
- `IncludeContextParser` und `IncludeContext` bestimmen, wo der Cursor im Tag steht.
- `IncludeSyntax` hält die Argumenttypen pro Kind.
- `PathSuggestions` listet Dateisystemeinträge auf.

**Geänderte Dateien:**
- **`AISourceViewerConfiguration`:** Der Konstruktor nimmt jetzt zusätzlich einen `Supplier<Path>` für das Basisverzeichnis, und `getContentAssistant(...)` ist neu.
- **`AITextEditor`:** Der Konstruktor übergibt das Verzeichnis der Datei über `baseDirectory()`.
- **`IncludeCommandRegistry`:** Ich habe `kinds()` ergänzt. Der Editor liest die Kinds darüber aus der Registry, und neue Kinds erscheinen automatisch.

**Offene Punkte:**
- **`plugin.xml`:** Unverändert. Der Content Assist braucht keine Extension, und Strg+Space ist im Text-Editor-Kontext bereits gebunden. Ihre Zeile dazu war leer. Falls Sie dort etwas Bestimmtes erwartet haben, z. B. eine eigene Tastenbelegung, sagen Sie es mir.
- **Argumenttypen:** Sie stehen als kleine Tabelle in `IncludeSyntax`, weil `IIncludeCommand` sie nicht anbietet. Ein neues Kind mit Sonderargument müsste dort ergänzt werden. Sonst gilt "Dateipfad".
- **Manifest:** Die `MANIFEST.MF` habe ich nicht gefunden. Ich gehe davon aus, dass `org.eclipse.jface.text` und `org.eclipse.core.runtime` schon eingebunden sind, da der Editor sie bereits nutzt.
Kurze Frage, wie reagiert der Assist wenn die Editor Datei bewegt wird?
Result Stats: id=9aaec37c-b8b8-4eb6-8c14-62003f1c5b07, total: 89046, in: 28, out: 24589, reason: 245, read: 462388, write: 64429, start: 1795813813633, end: 1795813820284, time: 6594, total: 166922, io: 0, control: 57, sent: 1, recv: 129
SystemInit:  id=9aaec37c-b8b8-4eb6-8c14-62003f1c5b07, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 245
Der Assist liest das Basisverzeichnis bei jedem Aufruf neu. `baseDirectory()` fragt jedes Mal `getEditorInput()` ab und speichert nichts zwischen. Sobald der Editor nach dem Verschieben einen neuen Input hat, laufen relative Pfade gegen das neue Verzeichnis. Das ist der Standardweg von Eclipse, den `TextEditor` über seinen Resource-Listener selbst geht. Ein Neustart des Editors ist dafür nicht nötig.

Ob der Input im Editor wirklich aktualisiert wird, habe ich nicht geprüft. `AISessionEditor` ist ein `MultiPageEditorPart` mit eigenem `IResourceChangeListener` und hält den `AITextEditor` als Seite. Wie er Verschiebungen behandelt, habe ich nicht gelesen. Schließt er den Editor oder lädt er neu, ist das Verhalten trotzdem richtig. Bliebe der Input dagegen auf dem alten Stand, würden relative Pfade gegen das alte Verzeichnis aufgelöst.

Fehlt das Verzeichnis ganz, zum Beispiel weil die Datei gelöscht wurde, liefert `PathSuggestions` keine Vorschläge. Es gibt dann keine Exception. Tag- und Kind-Vorschläge sowie absolute Pfade funktionieren weiter.

Soll ich in `AISessionEditor` nachsehen, wie er Verschiebungen behandelt?
