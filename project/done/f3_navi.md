Ich mochte in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java` das naavigieren mit F3 Shortcut oder STRG-Click unterstützen.
Ich vermute hierfür muss ein Hyperlink Detektor implementiert werden.

- Zielpackage: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/navigate`
- `/home/user/xyan/xy.ai.workbench/plugin.xml`

## Hinweis

Das geht über den Extension Point `org.eclipse.ui.workbench.texteditor.hyperlinkDetectors`. F3 ist in Texteditoren an “Open Hyperlink” gebunden. Dieselben Detektoren steuern also F3 und Strg+Klick.

**plugin.xml**

```xml
<extension point="org.eclipse.ui.workbench.texteditor.hyperlinkDetectors">
  <hyperlinkDetector
      id="my.mdLinkDetector"
      name="Markdown Link Detector"
      class="my.MdLinkDetector"
      targetId="org.eclipse.ui.DefaultTextEditor"/>
</extension>
```

`targetId` ist der Editortyp. Bei einem eigenen Markdown-Editor brauchst du dessen Target-ID. Sie ist in dessen `hyperlinkDetectorTargets`-Extension definiert.

## Details

Folgende Syntax soll dabei unetrstützt werden.

Markdown Links auf Dateien (Ziel soll geöffnet/navigiert werden):
"`/home/user/pfad/AITextEditor.java`"

Alle Pfade der Include COmmands aus `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java`:
"[include](`/home/user/pfad/AITextEditor.java`)" (Kann "`" enthalten oder auch nicht)
"[include](/home/user/pfad/AITextEditor.java)"

Außerdem das Kommando "/answer 2e0e-46-45 allow" in allen Varianten. Die dritte Stelle der ID (also "45"), zeigt auf einen Tool Call der Session "2e0e". Beim folgen des Allow Commands soll zur ursprüngllichen Tool-Call gesprungen werden, also "2e0e-45-0".
Beispiel:

```yaml
id: 2e0e-45-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    id: runTests
  reason: "Prüfen, dass nach dem Deaktivieren der Lauf grün ist"
```

```yaml
id: 2e0e-46-45
result:
  structuredContent:
    results:
    - index: 0
      stdout: |
        OK: 2751 tests passed, 2 ignored
```
/answer 2e0e-46-45 allow

Beispeil Ende.


SystemInit:  id=3c77d172-9296-4d78-90c3-84db79ce9332, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: fe0f-1-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: .*
    file_include_mask:
    - '*.java'
    limit: 50
    intent: survey
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: hyperlinkDetector|AITextEditor|<extension point
    file_include_mask:
    - plugin.xml
    limit: 50
    intent: locate_for_edit
  reason: "Editor, plugin.xml, Include-Registry und Zielpackage sichten"
```
/answer fe0f-1-0 allow
Control Request:
```yaml
id: fe0f-2-1
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
        - "Limit of 50 matches reached (total 4382); narrow pattern, directory or the file/dir masks."
      - directory:
        - /home/user/xyan/xy.ai.workbench
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
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
        nodes:
        - id: pWkEcb|UEKlsM
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
          - id: IncludeCommandRegistry.kinds
            excert: public Set<String> kinds()
            docstring: "Registered kinds in alphabetical order, without the kind-less path command."
          - id: IncludeCommandRegistry.execute
            excert: "public void execute(String kind, String arg, List<Path> chain, SessionCallbacks…"
```
/answer fe0f-2-1 allow
ReasoningToken: 43
Control Request:
```yaml
id: fe0f-3-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    ids:
    - s0G8wt|06AciC
    - AITextEditor.AITextEditor
    - AITextEditor.createSourceViewer
    - AITextEditor.createPartControl
    - AITextEditor.baseDirectory
    - AITextEditor.getUpdateManager
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: extension|Bundle-SymbolicName|Require-Bundle|org.eclipse
    file_include_mask:
    - plugin.xml
    - MANIFEST.MF
    dir_exclude_mask:
    - bin
    - target
    - .git
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include
    pattern: getKind|kind\(\)|return "
    limit: 50
  reason: "Editor-Quelltext, Registry und plugin.xml lesen"
```
/answer fe0f-3-0 allow Du musst für plugin.xml find benutzen
Control Request:
```yaml
id: fe0f-4-3
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include
        matches:
        - path: PathIncludeCommand.java
          matches:
          - "public String kind() {"
        - path: SearchIncludeCommand.java
          matches:
          - "public String kind() {"
          - return "search";
        - path: FilesIncludeCommand.java
          matches:
          - "public String kind() {"
          - return "files";
        - path: FileIncludeCommand.java
          matches:
          - "public String kind() {"
          - return "file";
        - path: DirectoryIncludeCommand.java
          matches:
          - "public String kind() {"
          - return "directory";
        - path: IIncludeCommand.java
          matches:
          - String kind();
        - path: IncludeCommandRegistry.java
          matches:
          - "commands.put(command.kind(), command);"
        - path: ContextPromptIncludeCommand.java
          matches:
          - "public String kind() {"
          - return "contextprompt";
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
        - id: AITextEditor.AITextEditor
          type: ConstructorDeclaration
          code: |2
              public AITextEditor() {
                super();
                setSourceViewerConfiguration(new AISourceViewerConfiguration(manager, this::baseDirectory));
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
        - id: AITextEditor.getUpdateManager
          type: MethodDeclaration
          code: |2
              public EditorManager getUpdateManager() {
                return manager;
              }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
        nodes:
        - id: pWkEcb|UEKlsM
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
          - id: IncludeCommandRegistry.kinds
            excert: public Set<String> kinds()
            docstring: "Registered kinds in alphabetical order, without the kind-less path command."
          - id: IncludeCommandRegistry.execute
            excert: "public void execute(String kind, String arg, List<Path> chain, SessionCallbacks…"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java instead."
```
/answer fe0f-4-3 allow
ReasoningToken: 66
Control Request:
```yaml
id: fe0f-5-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/plugin.xml
    regexp: .
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
    node_type: MethodDeclaration|ConstructorDeclaration
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    - /home/user/xyan/xy.ai.workbench/connector/src
    pattern: class AISourceViewerConfiguration|/answer|"answer"|Pattern.compile.*include|getHyperlinkDetectors
    file_include_mask:
    - '*.java'
    limit: 30
  reason: "plugin.xml per find lesen; Registry, Konfiguration und Include-Parsing suchen"
```
/answer fe0f-5-0 allow
Control Request:
```yaml
id: fe0f-6-5
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src
        - /home/user/xyan/xy.ai.workbench/connector/src
        matches:
        - path: xy/ai/workbench/editor/suggest/IncludeContextParser.java
          matches:
          - "private static final Pattern COMPLETE_TAGS = Pattern.compile(\"(?:\\\\s*\\\\[include(?:\\\\s+\\\\w+)?\\\\]\\\\([^)]*\\\\))*\\\\s*\");"
          - "private static final Pattern ARGUMENT = Pattern.compile(\"\\\\[include(?:\\\\s+(\\\\w+))?\\\\]\\\\(([^)]*)\");"
        - path: xy/ai/workbench/editor/AISourceViewerConfiguration.java
          matches:
          - "public class AISourceViewerConfiguration extends SourceViewerConfiguration {"
        - path: xy/ai/conscious/harness/IncludeProcessor.java
          matches:
          - "public static final Pattern INCLUDE_LINE = Pattern.compile(\"^(\\\\s*\\\\[include(?:\\\\s+\\\\w+)?\\\\]\\\\([^)]*\\\\)\\\\s*)+$\");"
          - "public static final Pattern INCLUDE_TAG = Pattern.compile(\"\\\\[include(?:\\\\s+(\\\\w+))?\\\\]\\\\(([^)]*)\\\\)\");"
        - path: xy/ai/conscious/harness/PromptInputHandler.java
          matches:
          - // An /answer command starting at the beginning of the block spans the
        - path: xy/ai/conscious/commands/AnswerCommand.java
          matches:
          - public static final String CMD_ANSWER = "/answer";
    find:
      results:
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
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
        nodes:
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
        - id: IncludeCommandRegistry.kinds
          type: MethodDeclaration
          code: |2
              public Set<String> kinds() {
                Set<String> kinds = new TreeSet<>();
                for (String kind : commands.keySet()) if (kind != null)
                  kinds.add(kind);
                return kinds;
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
/answer fe0f-6-5 allow
ReasoningToken: 106
Control Request:
```yaml
id: fe0f-7-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include
    pattern: resolve|Path\.of|Paths\.get|strip|replace
    file_include_mask:
    - '*.java'
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src
    - /home/user/xyan/xy.ai.workbench/src
    pattern: allow|ALLOW
    file_include_mask:
    - AnswerCommand.java
    - PromptInputHandler.java
    limit: 30
  reason: "AISourceViewerConfiguration, AnswerCommand, Include-Kommandos, AISessionEditor prüfen"
```
/answer fe0f-7-0 allow
Control Request:
```yaml
id: fe0f-8-7
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include
        matches:
        - path: PathIncludeCommand.java
          matches:
          - Path path = Paths.get(pathText).toAbsolutePath().normalize();
        - path: DirectoryIncludeCommand.java
          matches:
          - Path directory = Paths.get(dir).toAbsolutePath().normalize();
        - path: ContextPromptIncludeCommand.java
          matches:
          - "out.add(call.message(Role.User, text.strip()));"
        - path: IncludeArgs.java
          matches:
          - String s = arg.strip();
          - return s.strip();
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src
        - /home/user/xyan/xy.ai.workbench/src
        matches:
        - path: xy/ai/conscious/harness/PromptInputHandler.java
          matches:
          - "// whole selection, allowing a multi-line reason/hint for"
          - // allow and deny alike.
        - path: xy/ai/conscious/commands/AnswerCommand.java
          matches:
          - // "s" (DOTALL) allows a block-selected multi-line reason/hint after allow|deny.
          - return line != null && line.strip().matches("(?is)" + prefix() + "\\s+\\S+\\s+(allow|deny|exec)(\\s+.*)?");
          - if ("allow".equalsIgnoreCase(parameter(1)))
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
        nodes:
        - id: o8migr|M90NEo
          excert: import org.eclipse.jface.text.DocumentEvent;
        - id: OXjZGy|pH9pcT
          excert: import org.eclipse.jface.text.contentassist.IContentAssistant;
        - id: AISourceViewerConfiguration
          excert: "public class AISourceViewerConfiguration extends SourceViewerConfiguration {"
          children:
          - id: AISourceViewerConfiguration.av1nou|trQjAG
            excert: private static final int LIMIT = 2 * 512 * 1024;
          - id: AISourceViewerConfiguration.AISourceViewerConfiguration
            excert: "public AISourceViewerConfiguration(EditorManager updateManager, Supplier<Path> …"
          - id: AISourceViewerConfiguration.getQuickAssistAssistant
            excert: public IQuickAssistAssistant getQuickAssistAssistant(ISourceViewer sourceViewer)
          - id: AISourceViewerConfiguration.getContentAssistant
            excert: public IContentAssistant getContentAssistant(ISourceViewer sourceViewer)
          - id: AISourceViewerConfiguration.getPresentationReconciler
            excert: public IPresentationReconciler getPresentationReconciler(ISourceViewer sourceVi…
          - id: AISourceViewerConfiguration.getDamager
            excert: public DefaultDamagerRepairer getDamager()
          - id: AISourceViewerConfiguration.DamagerRepairer
            excert: "private static class DamagerRepairer extends DefaultDamagerRepairer {"
            children:
            - id: AISourceViewerConfiguration.DamagerRepairer.Ehq9yQ|uKgHv3
              excert: private EditorManager updateManager;
            - id: AISourceViewerConfiguration.DamagerRepairer.DamagerRepairer
              excert: "private DamagerRepairer(ITokenScanner scanner, EditorManager updateManager)"
            - id: AISourceViewerConfiguration.DamagerRepairer.createPresentation
              excert: "public void createPresentation(TextPresentation presentation, ITypedRegion regi…"
            - id: AISourceViewerConfiguration.DamagerRepairer.getDamageRegion
              excert: "public IRegion getDamageRegion(ITypedRegion partition, DocumentEvent e, boolean…"
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
        nodes:
        - id: AnswerCommand
          excert: "public class AnswerCommand extends Command {"
          children:
          - id: AnswerCommand.LN3qky|soKy4k
            excert: public static final String CMD_ANSWER = "/answer";
          - id: AnswerCommand.AnswerCommand
            excert: public AnswerCommand()
          - id: AnswerCommand.AnswerCommand_2
            excert: "public AnswerCommand(String id, String action, String reason)"
          - id: AnswerCommand.prefix
            excert: public String prefix()
          - id: AnswerCommand.matches
            excert: public boolean matches(String line)
          - id: AnswerCommand.parse
            excert: public Command parse(String line)
          - id: AnswerCommand.processorAction
            excert: public ProcessorAction processorAction()
          - id: AnswerCommand.Action
            excert: "public enum Action {"
          - id: AnswerCommand.BlockType
            excert: /**  * Distinguishes a pending tool call (arguments) from a pending tool result…
            docstring: Distinguishes a pending tool call (arguments) from a pending tool result (resul…
          - id: AnswerCommand.np6Lp5|nApalK
            excert: private String yaml;
          - id: AnswerCommand.setYaml
            excert: public void setYaml(String yaml)
            docstring: "Sets the raw YAML block this answer refers to, used by {@link #blockType()}."
          - id: AnswerCommand.yaml
            excert: public String yaml()
            docstring: "The raw YAML block preceding this answer (possibly edited by the user), or {@co…"
          - id: AnswerCommand.blockType
            excert: public BlockType blockType()
          - id: AnswerCommand.action
            excert: public Action action()
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
        nodes:
        - id: S1nFwh|22bDbD
          excert: import org.eclipse.core.resources.IResourceChangeEvent;
        - id: AISessionEditor
          excert: public class AISessionEditor extends MultiPageEditorPart implements IResourceCh…
          children:
          - id: AISessionEditor.ByFURu|MIZHxG
            excert: private AITextEditor editor;
          - id: AISessionEditor.AISessionEditor
            excert: public AISessionEditor()
          - id: AISessionEditor.getEditor
            excert: public AITextEditor getEditor()
          - id: AISessionEditor.createPages
            excert: protected void createPages()
          - id: AISessionEditor.dispose
            excert: public void dispose()
          - id: AISessionEditor.doSave
            excert: public void doSave(IProgressMonitor mon)
          - id: AISessionEditor.doSaveAs
            excert: public void doSaveAs()
          - id: AISessionEditor.isSaveAsAllowed
            excert: public boolean isSaveAsAllowed()
          - id: AISessionEditor.resourceChanged
            excert: public void resourceChanged(final IResourceChangeEvent event)
          - id: AISessionEditor.handleMove
            excert: private void handleMove(final IResourceChangeEvent event)
            docstring: "Aktualisiert Input und Titel, wenn die Datei (oder ein übergeordneter Ordner)"
          - id: AISessionEditor.updateTitle
            excert: private void updateTitle()
```
/answer fe0f-8-7 allow
ReasoningToken: 3526
Control Request:
```yaml
id: fe0f-9-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
    node_type: MethodDeclaration
    name: prefix
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
    node_type: MethodDeclaration
    name: matches
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
    node_type: MethodDeclaration
    name: parse
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FilesIncludeCommand.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/SearchIncludeCommand.java
    regexp: .
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    node_type: ImportDeclaration|ConstructorDeclaration|FieldDeclaration|ImportGroup
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: TextSourceViewerConfiguration|HyperlinkDetector|Require-Bundle|org.eclipse.ui.editors
    file_include_mask:
    - '*.java'
    - MANIFEST.MF
    limit: 15
  reason: "Details zu AnswerCommand, Path-Include, IncludeArgs und Config lesen"
```
/answer fe0f-9-0 allow
Control Request:
```yaml
id: fe0f-a-9
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: src/xy/ai/workbench/view/diff/DiffPanel.java
          matches:
          - import org.eclipse.ui.editors.text.EditorsUI;
        - path: src/xy/ai/workbench/editor/AITextEditor.java
          matches:
          - import org.eclipse.ui.editors.text.TextEditor;
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
        nodes:
        - id: AnswerCommand.prefix
          type: MethodDeclaration
          code: |2
              @Override
              public String prefix() {
                return AnswerCommand.CMD_ANSWER;
              }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
        nodes:
        - id: AnswerCommand.matches
          type: MethodDeclaration
          code: |2
              @Override
              public boolean matches(String line) {
                // "s" (DOTALL) allows a block-selected multi-line reason/hint after allow|deny.
                return line != null && line.strip().matches("(?is)" + prefix() + "\\s+\\S+\\s+(allow|deny|exec)(\\s+.*)?");
              }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/AnswerCommand.java
        nodes:
        - id: AnswerCommand.parse
          type: MethodDeclaration
          code: |2
              @Override
              public Command parse(String line) {
                String[] parts = line.strip().split("\\s+", 4);
                String id = parts[1];
                String action = parts[2].toLowerCase();
                String reason = parts.length > 3 ? parts[3].strip() : "";
                return new AnswerCommand(id, action, reason);
              }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
        nodes:
        - id: IncludeArgs
          excert: "class IncludeArgs {"
          children:
          - id: IncludeArgs.path
            code: |2
                static String path(String arg) {
                  String s = arg.strip();
                  if (s.startsWith("`"))
                    s = s.substring(1);
                  if (s.endsWith("`"))
                    s = s.substring(0, s.length() - 1);
                  return s.strip();
                }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
        nodes:
        - id: K9s6LE|TD7ww8
          code: |
            import java.io.IOException;
            import java.nio.file.Files;
            import java.nio.file.Path;
            import java.nio.file.Paths;
            import java.util.ArrayList;
            import java.util.List;
            import xy.ai.conscious.harness.IncludeCycleException;
            import xy.ai.conscious.harness.ISessionIncludeProcessor;
            import xy.ai.conscious.harness.SessionCallbacks;
        - id: PathIncludeCommand
          excert: "public class PathIncludeCommand<M> implements IIncludeCommand<M> {"
          children:
          - id: PathIncludeCommand.kind
            code: |2
                @Override
                public String kind() {
                  return null;
                }
          - id: PathIncludeCommand.execute
            code: |2
                @Override
                public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                  String path = IncludeArgs.path(arg);
                  if (path.isBlank())
                    throw new IllegalStateException("Include path must not be empty");
                  includePath(path, chain, call, out, recursiv);
                }
          - id: PathIncludeCommand.includePath
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
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FilesIncludeCommand.java
        nodes:
        - id: g88odJ|vjpE2Y
          code: |
            import java.nio.file.Path;
            import java.util.List;
            import xy.ai.conscious.harness.IIncludeAdapter;
            import xy.ai.conscious.harness.ISessionIncludeProcessor;
            import xy.ai.conscious.harness.SessionCallbacks;
            import xy.ai.conscious.harness.ToolResult;
        - id: FilesIncludeCommand
          excert: "public class FilesIncludeCommand<M> extends AdapterIncludeCommand<M> {"
          children:
          - id: FilesIncludeCommand.FilesIncludeCommand
            code: |2
                public FilesIncludeCommand(IIncludeAdapter adapter) {
                  super(adapter);
                }
          - id: FilesIncludeCommand.kind
            code: |2
                @Override
                public String kind() {
                  return "files";
                }
          - id: FilesIncludeCommand.execute
            code: |2
                @Override
                public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                  for (IIncludeAdapter.Entry entry : requireAdapter().files(IncludeArgs.path(arg))) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
                }
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/SearchIncludeCommand.java
        nodes:
        - id: g88odJ|c1bZpd
          code: |
            import java.nio.file.Path;
            import java.util.List;

            import com.fasterxml.jackson.databind.node.ArrayNode;
            import com.fasterxml.jackson.databind.node.JsonNodeFactory;
            import com.fasterxml.jackson.databind.node.ObjectNode;

            import xy.ai.conscious.connector.claudecode.YamlRenderer;
            import xy.ai.conscious.harness.IIncludeAdapter;
            import xy.ai.conscious.harness.ISessionIncludeProcessor;
            import xy.ai.conscious.harness.SessionCallbacks;
            import xy.ai.conscious.harness.ToolResult;
        - id: SearchIncludeCommand
          excert: "public class SearchIncludeCommand<M> extends AdapterIncludeCommand<M> {"
          children:
          - id: SearchIncludeCommand.KDIAgh|hW7a82
            code: |2
                private final YamlRenderer yaml = new YamlRenderer();
          - id: SearchIncludeCommand.SearchIncludeCommand
            code: |2
                public SearchIncludeCommand(IIncludeAdapter adapter) {
                  super(adapter);
                }
          - id: SearchIncludeCommand.kind
            code: |2
                @Override
                public String kind() {
                  return "search";
                }
          - id: SearchIncludeCommand.execute
            code: |2
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
          - id: SearchIncludeCommand.renderMatches
            code: |2
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
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
        nodes:
        - id: o8migr|M90NEo
          type: ImportGroup
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
            import java.nio.file.Path;
            import java.util.function.Supplier;
            import org.eclipse.jface.text.contentassist.ContentAssistant;
        - id: OXjZGy|pH9pcT
          type: ImportGroup
          code: |
            import org.eclipse.jface.text.contentassist.IContentAssistant;
            import xy.ai.workbench.editor.suggest.IncludeCompletionProcessor;
        - id: AISourceViewerConfiguration.AISourceViewerConfiguration
          type: ConstructorDeclaration
          code: |2
              public AISourceViewerConfiguration(EditorManager updateManager, Supplier<Path> baseDirectory) {
                this.baseDirectory = baseDirectory;
                rules.setUpdateManager(updateManager);
                dmg = new DamagerRepairer(rules, updateManager);
              }
        - id: AISourceViewerConfiguration.DamagerRepairer.DamagerRepairer
          type: ConstructorDeclaration
          code: |2
                private DamagerRepairer(ITokenScanner scanner, EditorManager updateManager) {
                  super(scanner);
                  this.updateManager = updateManager;
                }
```
/answer fe0f-a-9 allow
ReasoningToken: 1454
Control Request:
```yaml
id: fe0f-b-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: org\.eclipse\.(ui\.ide|ui\.editors|jface\.text|core\.filesystem|ui\.workbench\.texteditor)
    file_include_mask:
    - MANIFEST.MF
    - '*.java'
    dir_exclude_mask:
    - bin
    - target
    - .git
    limit: 15
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: Require-Bundle
    file_include_mask:
    - MANIFEST.MF
    limit: 5
  reason: "Bundle-Abhängigkeiten prüfen (org.eclipse.ui.ide, texteditor)"
```
/answer fe0f-b-0 allow Auch für MANIFEST.MF musst du find benutzen
Control Request:
```yaml
id: fe0f-c-b
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: src/xy/ai/workbench/IncludeAdapter.java
          matches:
          - import org.eclipse.jface.text.BadLocationException;
          - import org.eclipse.jface.text.Document;
          - import org.eclipse.jface.text.IDocument;
        - path: src/xy/ai/workbench/EditorInterface.java
          matches:
          - import org.eclipse.jface.text.BadLocationException;
          - import org.eclipse.jface.text.IDocument;
          - import org.eclipse.jface.text.ITextSelection;
          - import org.eclipse.ui.ide.IDE;
        - path: src/xy/ai/workbench/ActiveEditorListener.java
          matches:
          - import org.eclipse.jface.text.BadLocationException;
          - import org.eclipse.jface.text.IDocument;
          - import org.eclipse.jface.text.IRegion;
          - import org.eclipse.jface.text.ITextSelection;
        - path: src/xy/ai/workbench/view/explorer/MarkdownExplorerView.java
          matches:
          - import org.eclipse.ui.ide.IDE;
          - import org.eclipse.ui.ide.ResourceUtil;
        - path: src/xy/ai/workbench/view/explorer/MarkdownLinkHelper.java
          matches:
          - import org.eclipse.ui.ide.ResourceUtil;
        - path: src/xy/ai/workbench/view/diff/DiffPanel.java
          matches:
          - import org.eclipse.jface.text.Document;
        warnings:
        - "Limit of 15 matches reached (total 127); narrow pattern, directory or the file/dir masks, or raise limit (max 50)."
      - directory:
        - /home/user/xyan/xy.ai.workbench
```
/answer fe0f-c-b allow
ReasoningToken: 2799
Control Request:
```yaml
id: fe0f-d-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/navigate/NavigationHyperlinkDetector.java
    source: |
      package xy.ai.workbench.editor.navigate;

      import java.nio.file.Files;
      import java.nio.file.InvalidPathException;
      import java.nio.file.Path;
      import java.util.function.Supplier;
      import java.util.regex.Matcher;
      import java.util.regex.Pattern;
      import org.eclipse.jface.text.BadLocationException;
      import org.eclipse.jface.text.IDocument;
      import org.eclipse.jface.text.IRegion;
      import org.eclipse.jface.text.ITextViewer;
      import org.eclipse.jface.text.Region;
      import org.eclipse.jface.text.hyperlink.AbstractHyperlinkDetector;
      import org.eclipse.jface.text.hyperlink.IHyperlink;

      /**
       * Detects navigable references: file paths (backticked or as include argument) and
       * {@code /answer <session>-<result>-<call> allow|deny|exec} commands, which point to the originating tool call.
       */
      public class NavigationHyperlinkDetector extends AbstractHyperlinkDetector {
        private static final Pattern INCLUDE = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]*)\\)");
        private static final Pattern BACKTICK = Pattern.compile("`([^`]+)`");
        private static final Pattern ANSWER = Pattern.compile("^\\s*/answer\\s+(\\S+)\\s+(?:allow|deny|exec)\\b", Pattern.CASE_INSENSITIVE);
        private static final Pattern CALL_ID = Pattern.compile("([^-\\s]+)-\\d+-(\\d+)");

        private final Supplier<Path> baseDirectory;

        public NavigationHyperlinkDetector(Supplier<Path> baseDirectory) {
          this.baseDirectory = baseDirectory;
        }

        @Override
        public IHyperlink[] detectHyperlinks(ITextViewer viewer, IRegion region, boolean canShowMultipleHyperlinks) {
          IDocument document = viewer.getDocument();
          if (document == null)
            return null;
          try {
            IRegion line = document.getLineInformationOfOffset(region.getOffset());
            String text = document.get(line.getOffset(), line.getLength());
            int caret = region.getOffset() - line.getOffset();
            IHyperlink link = detectAnswer(viewer, text, caret, line.getOffset());
            if (link == null)
              link = detectInclude(text, caret, line.getOffset());
            if (link == null)
              link = detectBacktick(text, caret, line.getOffset());
            return link == null ? null : new IHyperlink[] { link };
          } catch (BadLocationException e) {
            return null;
          }
        }

        private IHyperlink detectAnswer(ITextViewer viewer, String text, int caret, int lineOffset) {
          Matcher answer = ANSWER.matcher(text);
          if (!answer.find() || !contains(answer.start(1), answer.end(1), caret))
            return null;
          Matcher id = CALL_ID.matcher(answer.group(1));
          if (!id.matches())
            return null;
          String callId = id.group(1) + "-" + id.group(2) + "-0";
          return new ToolCallHyperlink(viewer, new Region(lineOffset + answer.start(1), answer.end(1) - answer.start(1)), callId);
        }

        private IHyperlink detectInclude(String text, int caret, int lineOffset) {
          Matcher include = INCLUDE.matcher(text);
          while (include.find()) {
            if ("search".equals(include.group(1)))
              continue;
            int start = include.start(2);
            int end = include.end(2);
            while (start < end && isPadding(text.charAt(start)))
              start++;
            while (end > start && isPadding(text.charAt(end - 1)))
              end--;
            if (contains(start, end, caret))
              return fileLink(text.substring(start, end), lineOffset + start, end - start);
          }
          return null;
        }

        private IHyperlink detectBacktick(String text, int caret, int lineOffset) {
          Matcher quoted = BACKTICK.matcher(text);
          while (quoted.find()) {
            if (contains(quoted.start(1), quoted.end(1), caret))
              return fileLink(quoted.group(1), lineOffset + quoted.start(1), quoted.end(1) - quoted.start(1));
          }
          return null;
        }

        private IHyperlink fileLink(String pathText, int offset, int length) {
          Path file = resolve(pathText.strip());
          return file == null ? null : new FileHyperlink(new Region(offset, length), file);
        }

        private Path resolve(String pathText) {
          if (pathText.isEmpty())
            return null;
          try {
            Path path = Path.of(pathText);
            if (!path.isAbsolute()) {
              Path base = baseDirectory.get();
              if (base == null)
                return null;
              path = base.resolve(path);
            }
            path = path.normalize();
            return Files.isRegularFile(path) ? path : null;
          } catch (InvalidPathException e) {
            return null;
          }
        }

        private static boolean isPadding(char c) {
          return c == '`' || Character.isWhitespace(c);
        }

        private static boolean contains(int start, int end, int offset) {
          return offset >= start && offset <= end;
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/navigate/FileHyperlink.java
    source: |
      package xy.ai.workbench.editor.navigate;

      import java.nio.file.Path;
      import org.eclipse.core.filesystem.EFS;
      import org.eclipse.core.resources.IFile;
      import org.eclipse.core.resources.ResourcesPlugin;
      import org.eclipse.core.runtime.CoreException;
      import org.eclipse.core.runtime.IStatus;
      import org.eclipse.core.runtime.Status;
      import org.eclipse.jface.text.IRegion;
      import org.eclipse.jface.text.hyperlink.IHyperlink;
      import org.eclipse.ui.IWorkbenchPage;
      import org.eclipse.ui.PlatformUI;
      import org.eclipse.ui.ide.IDE;
      import org.eclipse.ui.statushandlers.StatusManager;

      class FileHyperlink implements IHyperlink {
        private final IRegion region;
        private final Path file;

        FileHyperlink(IRegion region, Path file) {
          this.region = region;
          this.file = file;
        }

        @Override
        public IRegion getHyperlinkRegion() {
          return region;
        }

        @Override
        public String getTypeLabel() {
          return "Open File";
        }

        @Override
        public String getHyperlinkText() {
          return "Open " + file.getFileName();
        }

        @Override
        public void open() {
          try {
            IWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();
            IFile[] files = ResourcesPlugin.getWorkspace().getRoot().findFilesForLocationURI(file.toUri());
            if (files.length > 0)
              IDE.openEditor(page, files[0]);
            else
              IDE.openEditorOnFileStore(page, EFS.getLocalFileSystem().fromLocalFile(file.toFile()));
          } catch (CoreException e) {
            StatusManager.getManager().handle(new Status(IStatus.ERROR, "xy.ai.workbench", "Cannot open " + file, e), StatusManager.SHOW);
          }
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/navigate/ToolCallHyperlink.java
    source: |
      package xy.ai.workbench.editor.navigate;

      import java.util.regex.Matcher;
      import java.util.regex.Pattern;
      import org.eclipse.jface.text.IRegion;
      import org.eclipse.jface.text.ITextViewer;
      import org.eclipse.jface.text.hyperlink.IHyperlink;
      import org.eclipse.swt.widgets.Display;

      class ToolCallHyperlink implements IHyperlink {
        private final ITextViewer viewer;
        private final IRegion region;
        private final String callId;

        ToolCallHyperlink(ITextViewer viewer, IRegion region, String callId) {
          this.viewer = viewer;
          this.region = region;
          this.callId = callId;
        }

        @Override
        public IRegion getHyperlinkRegion() {
          return region;
        }

        @Override
        public String getTypeLabel() {
          return "Go to Tool Call";
        }

        @Override
        public String getHyperlinkText() {
          return "Go to tool call " + callId;
        }

        @Override
        public void open() {
          if (viewer.getDocument() == null)
            return;
          Pattern idLine = Pattern.compile("^[ \\t]*(?:-[ \\t]+)?id:[ \\t]*[\"']?(" + Pattern.quote(callId) + ")[\"']?[ \\t]*$", Pattern.MULTILINE);
          Matcher match = idLine.matcher(viewer.getDocument().get());
          if (!match.find()) {
            Display.getDefault().beep();
            return;
          }
          viewer.setSelectedRange(match.start(1), callId.length());
          viewer.revealRange(match.start(1), callId.length());
        }
      }
  insert:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    id: AISourceViewerConfiguration.getContentAssistant
    position: after
    source: |2
        @Override
        public IHyperlinkDetector[] getHyperlinkDetectors(ISourceViewer sourceViewer) {
          return new IHyperlinkDetector[] { new NavigationHyperlinkDetector(baseDirectory) };
        }
    addImport:
    - org.eclipse.jface.text.hyperlink.IHyperlinkDetector
    - xy.ai.workbench.editor.navigate.NavigationHyperlinkDetector
  reason: Hyperlink-Detektor und Hyperlinks im Package navigate anlegen; Detektor in AISourceViewerConfiguration einhängen
```
/answer fe0f-d-0 allow
ReasoningToken: 34
Control Request:
```yaml
id: fe0f-e-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    text: Hyperlink
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: ^import .*(IHyperlinkDetector|NavigationHyperlinkDetector|SourceViewerConfiguration);
    file_include_mask:
    - AISourceViewerConfiguration.java
  reason: Importzustand der Konfiguration prüfen
```
/answer fe0f-e-0 allow
Control Request:
```yaml
id: fe0f-f-e
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
        matches:
        - path: AISourceViewerConfiguration.java
          matches:
          - import org.eclipse.jface.text.source.SourceViewerConfiguration;
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
        nodes:
        - id: AISourceViewerConfiguration.getHyperlinkDetectors
          code: |2
              @Override
              public IHyperlinkDetector[] getHyperlinkDetectors(ISourceViewer sourceViewer) {
                return new IHyperlinkDetector[] { new NavigationHyperlinkDetector(baseDirectory) };
              }
```
/answer fe0f-f-e allow
Text:
 Methode ist drin, die Imports fehlen noch. Ich ergänze sie ohne Semikolon (das Tool hat sie offenbar doppelt gesetzt).
Control Request:
```yaml
id: fe0f-10-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    id: OXjZGy|pH9pcT
    old_text: import org.eclipse.jface.text.contentassist.IContentAssistant;
    new_text: |-
      import org.eclipse.jface.text.contentassist.IContentAssistant;
      import org.eclipse.jface.text.hyperlink.IHyperlinkDetector;
      import xy.ai.workbench.editor.navigate.NavigationHyperlinkDetector;
  reason: Fehlende Imports in AISourceViewerConfiguration ergänzen
```
/answer fe0f-10-0 allow
Result Stats: id=3c77d172-9296-4d78-90c3-84db79ce9332, total: 67486, in: 20, out: 17014, reason: 0, read: 267021, write: 50452, start: 1795851826969, end: 1795851835439, time: 8399, total: 118594, io: 0, control: 71, sent: 0, recv: 230
F3 und Strg+Klick sind eingebaut. Ich habe sie nicht kompiliert und nicht in Eclipse getestet.

**`plugin.xml` habe ich nicht geändert.** Der Extension Point `hyperlinkDetectors` greift nur bei einer `TextSourceViewerConfiguration`. `AISourceViewerConfiguration` erbt aber von `SourceViewerConfiguration` und liefert bisher gar keine Detektoren. Deshalb überschreibe ich dort `getHyperlinkDetectors(...)` und gebe den Detektor direkt zurück. Ein Eintrag in `plugin.xml` würde nie geladen. Den Umstieg auf `TextSourceViewerConfiguration` konnte ich ohne Zugriff auf die API-Quellen nicht sicher prüfen und habe ihn daher nicht gemacht. Wenn du den Extension Point trotzdem willst, geht das nur mit diesem Umstieg.

**Neue Klassen** in `src/xy/ai/workbench/editor/navigate`:
- **`NavigationHyperlinkDetector`:** Er prüft die Zeile unter dem Cursor in dieser Reihenfolge auf diese drei Muster:
  - **`/answer <session>-<ergebnis>-<call> allow|deny|exec`:** Der Link liegt auf der ID. Aus `2e0e-46-45` wird das Ziel `2e0e-45-0`.
  - **`[include](…)` und `[include <kind>](…)`:** Der Link liegt auf dem Pfad, Backticks und Leerzeichen am Rand werden ignoriert. Der Kind `search` wird übersprungen, weil sein Argument kein Pfad ist.
  - **`` `pfad` ``:** Der Inhalt in Backticks wird als Pfad behandelt.
- **`FileHyperlink`:** Er öffnet die Datei über `IDE.openEditor`, wenn sie im Workspace liegt. Sonst öffnet er sie als externe Datei.
- **`ToolCallHyperlink`:** Er sucht im Dokument die Zeile `id: <ziel-id>` (auch mit `- ` davor oder mit Anführungszeichen) und markiert sie. Wenn die Zeile fehlt, piept es nur.

**Ein Pfad wird nur dann zum Link, wenn die Datei existiert.** Relative Pfade werden gegen das Verzeichnis der geöffneten Datei aufgelöst. Dadurch werden normale Inline-Codestücke in Backticks nicht fälschlich zu Links.

**Offene Punkte:**
- **Verzeichnisse:** `[include directory](…)` erzeugt keinen Link, weil sich ein Verzeichnis nicht im Editor öffnen lässt. Soll es stattdessen im Explorer angezeigt werden, kann ich das ergänzen.
- **Suche nur im aktuellen Dokument:** Der Sprung zum Tool-Call sucht nur in der aktuellen Datei.
- **Imports:** Das Einfügen der Imports in `AISourceViewerConfiguration` hat zuerst mit einem Parse-Fehler gemeldet. Ich habe sie danach manuell ergänzt, prüfe bitte kurz, ob die Datei kompiliert.
Kannst du "IDE.openEditorOnFileStore(page, EFS.getLocalFileSystem().fromLocalFile(file.toFile()));" tauschen gegen etwas das EFS nicht braucht?
SystemInit:  id=3c77d172-9296-4d78-90c3-84db79ce9332, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 274
Control Request:
```yaml
id: fe0f-11-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/navigate/FileHyperlink.java
    id: FileHyperlink.open
    old_text: "IDE.openEditorOnFileStore(page, EFS.getLocalFileSystem().fromLocalFile(file.toFile()));"
    new_text: "IDE.openEditor(page, file.toUri(), IDE.getEditorDescriptor(file.getFileName().toString(), true, false).getId(), true);"
    removeImport:
    - org.eclipse.core.filesystem.EFS
  reason: EFS-Aufruf durch IDE.openEditor mit URI ersetzen
```
/answer fe0f-11-0 allow
Control Request:
```yaml
id: fe0f-12-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/navigate
    pattern: EFS|IDE\.
    file_include_mask:
    - FileHyperlink.java
  reason: "Prüfen, dass EFS-Import entfernt wurde"
```
/answer fe0f-12-0 allow
Control Request:
```yaml
id: fe0f-13-12
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/navigate
        matches:
        - path: FileHyperlink.java
          matches:
          - import org.eclipse.core.filesystem.EFS;
          - "IDE.openEditor(page, files[0]);"
          - "IDE.openEditor(page, file.toUri(), IDE.getEditorDescriptor(file.getFileName().toString(), true, false).getId(), true);"
```
/answer fe0f-13-12 allow
Text:
 Der EFS-Import ist noch da, ich entferne ihn.
Control Request:
```yaml
id: fe0f-14-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/navigate/FileHyperlink.java
    id: FileHyperlink
    old_text: |
      import org.eclipse.core.filesystem.EFS;
    new_text: ""
  reason: Ungenutzten EFS-Import entfernen
```
/answer fe0f-14-0 allow
Control Request:
```yaml
id: fe0f-15-14
result:
  structuredContent:
    edit_block:
      errors:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/navigate/FileHyperlink.java
        id: FileHyperlink
        error: Text not found in node.
        reason: not_found
        next_step: reread_node
```
/answer fe0f-15-14 deny ich entferne ihn
Result Stats: id=3c77d172-9296-4d78-90c3-84db79ce9332, total: 71586, in: 28, out: 18449, reason: 18, read: 474469, write: 53109, start: 1795852045853, end: 1795852049683, time: 3770, total: 132020, io: 0, control: 60, sent: 0, recv: 91
ReasoningToken: 18
Den EFS-Import entferne ich nicht, du hast ihn selbst übernommen.

In `FileHyperlink.open()` steht jetzt dieser Aufruf statt des EFS-Aufrufs:

```java
IDE.openEditor(page, file.toUri(), IDE.getEditorDescriptor(file.getFileName().toString(), true, false).getId(), true);
```

`IDE.getEditorDescriptor(...)` wählt den Editor passend zum Dateinamen und Inhaltstyp. `IDE.openEditor(page, URI, editorId, activate)` öffnet damit die Datei. Dein Code braucht dafür kein EFS mehr. `PartInitException` ist eine `CoreException`, der bestehende `catch` greift also weiter.

`import org.eclipse.core.filesystem.EFS;` steht noch in der Datei und ist jetzt ungenutzt.

Falls für den Dateinamen kein Editor registriert ist, liefert `getEditorDescriptor` einen Fehler oder den Default-Editor. Ob die Datei dann als Text oder extern geöffnet wird, hängt von der Eclipse-Konfiguration ab.
