Der Titel (Text im Tab) des `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java` wird nicht aktualisiert wenn sich die Datei verschiebt oder der Dateiname ändert. Korrigiere das.
SystemInit:  id=758aff49-dd9f-4afc-a188-ee289d9be155, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 906f-13-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
  reason: Struktur des Editors ansehen
```
/answer 906f-13-0 allow
Control Request:
```yaml
id: 906f-14-13
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
        nodes:
        - id: 7o5txV|nc8A79
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
```
/answer 906f-14-13 allow
Control Request:
```yaml
id: 906f-15-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
  reason: Gesamten Quelltext lesen
```
/answer 906f-15-0 allow
Control Request:
```yaml
id: 906f-16-15
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
        nodes:
        - id: 7o5txV|nc8A79
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
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java instead."
```
/answer 906f-16-15 allow
Control Request:
```yaml
id: 906f-17-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
    ids:
    - AISessionEditor
  reason: Klasse vollständig lesen
```
/answer 906f-17-0 allow
Control Request:
```yaml
id: 906f-18-17
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
        nodes:
        - id: AISessionEditor
          type: ClassOrInterfaceDeclaration
          excert: public class AISessionEditor extends MultiPageEditorPart implements IResourceCh…
          children:
          - id: AISessionEditor.ByFURu|MIZHxG
            type: StatementGroup
            code: "\tprivate AITextEditor editor;\n"
          - id: AISessionEditor.AISessionEditor
            type: ConstructorDeclaration
            code: "\tpublic AISessionEditor() {\n\t\tsuper();\n\t\tResourcesPlugin.getWorkspace().addResourceChangeListener(this);\n\t}\n"
          - id: AISessionEditor.getEditor
            type: MethodDeclaration
            code: "\tpublic AITextEditor getEditor() {\n\t\treturn editor;\n\t}\n"
          - id: AISessionEditor.createPages
            type: MethodDeclaration
            code: "\t@Override\n\tprotected void createPages() {\n\t\ttry {\n\t\t\teditor = new AITextEditor();\n\t\t\teditor.setInitializationData(getConfigurationElement(), null, null);\n\t\t\tint index = addPage(editor, getEditorInput());\n\t\t\tsetPartName(getEditorInput().getName());\n\t\t\tsetPageText(index, editor.getTitle());\n\t\t\tvar folder = ((CTabFolder) getContainer());\n\t\t\tfolder.setTabHeight(0);\n\t\t} catch (PartInitException e) {\n\t\t\tErrorDialog.openError(getSite().getShell(), \"Error creating nested text editor\", null, e.getStatus());\n\t\t\tLOG.error(\"Error initing part\", e);\n\t\t}\n\t}\n"
          - id: AISessionEditor.dispose
            type: MethodDeclaration
            code: "\t@Override\n\tpublic void dispose() {\n\t\tResourcesPlugin.getWorkspace().removeResourceChangeListener(this);\n\t\tsuper.dispose();\n\t}\n"
          - id: AISessionEditor.doSave
            type: MethodDeclaration
            code: "\t@Override\n\tpublic void doSave(IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Save editor\", 1);\n\t\tgetEditor().doSave(sub.split(1));\n\t}\n"
          - id: AISessionEditor.doSaveAs
            type: MethodDeclaration
            code: "\t@Override\n\tpublic void doSaveAs() {\n\t\tIEditorPart editor = getEditor();\n\t\teditor.doSaveAs();\n\t\tsetPageText(0, editor.getTitle());\n\t\tsetInput(editor.getEditorInput());\n\t}\n"
          - id: AISessionEditor.isSaveAsAllowed
            type: MethodDeclaration
            code: "\t@Override\n\tpublic boolean isSaveAsAllowed() {\n\t\treturn true;\n\t}\n"
          - id: AISessionEditor.resourceChanged
            type: MethodDeclaration
            code: "\t@Override\n\tpublic void resourceChanged(final IResourceChangeEvent event) {\n\t\tif (event.getType() == IResourceChangeEvent.PRE_CLOSE) {\n\t\t\tDisplay.getDefault().asyncExec(() -> {\n\t\t\t\tIWorkbenchPage[] pages = getSite().getWorkbenchWindow().getPages();\n\t\t\t\tfor (int i = 0; i < pages.length; i++) {\n\t\t\t\t\tif (((FileEditorInput) editor.getEditorInput()).getFile().getProject()\n\t\t\t\t\t\t\t.equals(event.getResource())) {\n\t\t\t\t\t\tIEditorPart editorPart = pages[i].findEditor(editor.getEditorInput());\n\t\t\t\t\t\tpages[i].closeEditor(editorPart, true);\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t});\n\t\t}\n\t}\n"
```
/answer 906f-18-17 allow
ReasoningToken: 808
Control Request:
```yaml
id: 906f-19-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
    node_type: Import.*
  reason: Vorhandene Imports prüfen
```
/answer 906f-19-0 allow
Control Request:
```yaml
id: 906f-1a-19
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
        nodes:
        - id: 7o5txV|nc8A79
          type: ImportGroup
          code: |
            import org.eclipse.core.resources.IResourceChangeEvent;
            import org.eclipse.core.resources.IResourceChangeListener;
            import org.eclipse.core.resources.ResourcesPlugin;
            import org.eclipse.core.runtime.IProgressMonitor;
            import org.eclipse.core.runtime.SubMonitor;
            import org.eclipse.jface.dialogs.ErrorDialog;
            import org.eclipse.swt.custom.CTabFolder;
            import org.eclipse.swt.widgets.Display;
            import org.eclipse.ui.IEditorPart;
            import org.eclipse.ui.IWorkbenchPage;
            import org.eclipse.ui.PartInitException;
            import org.eclipse.ui.part.FileEditorInput;
            import org.eclipse.ui.part.MultiPageEditorPart;

            import xy.ai.conscious.core.LOG;
```
/answer 906f-1a-19 allow
ReasoningToken: 96
Text:
 Die Imports müssen pro Edit angegeben werden. Ich wiederhole den Aufruf entsprechend.
ReasoningToken: 20
Control Request:
```yaml
id: 906f-1c-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
    id: AISessionEditor.resourceChanged
    addImport:
    - import org.eclipse.core.resources.IFile;
    - import org.eclipse.core.resources.IResource;
    - import org.eclipse.core.resources.IResourceDelta;
    - import org.eclipse.ui.IEditorInput;
    source: "\t@Override\n\tpublic void resourceChanged(final IResourceChangeEvent event) {\n\t\tif (event.getType() == IResourceChangeEvent.PRE_CLOSE) {\n\t\t\tDisplay.getDefault().asyncExec(() -> {\n\t\t\t\tIWorkbenchPage[] pages = getSite().getWorkbenchWindow().getPages();\n\t\t\t\tfor (int i = 0; i < pages.length; i++) {\n\t\t\t\t\tif (((FileEditorInput) editor.getEditorInput()).getFile().getProject()\n\t\t\t\t\t\t\t.equals(event.getResource())) {\n\t\t\t\t\t\tIEditorPart editorPart = pages[i].findEditor(editor.getEditorInput());\n\t\t\t\t\t\tpages[i].closeEditor(editorPart, true);\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t});\n\t\t} else if (event.getType() == IResourceChangeEvent.POST_CHANGE) {\n\t\t\thandleMove(event);\n\t\t}\n\t}\n\n\t/**\n\t * Aktualisiert Input und Titel, wenn die Datei (oder ein übergeordneter Ordner)\n\t * verschoben oder umbenannt wurde.\n\t */\n\tprivate void handleMove(final IResourceChangeEvent event) {\n\t\tif (!(getEditorInput() instanceof FileEditorInput fileInput)) {\n\t\t\treturn;\n\t\t}\n\t\tIResourceDelta rootDelta = event.getDelta();\n\t\tif (rootDelta == null) {\n\t\t\treturn;\n\t\t}\n\t\tIResourceDelta delta = rootDelta.findMember(fileInput.getFile().getFullPath());\n\t\tif (delta == null || (delta.getFlags() & IResourceDelta.MOVED_TO) == 0\n\t\t\t\t|| (delta.getKind() & IResourceDelta.REMOVED) == 0) {\n\t\t\treturn;\n\t\t}\n\t\tIResource newResource = ResourcesPlugin.getWorkspace().getRoot().findMember(delta.getMovedToPath());\n\t\tif (!(newResource instanceof IFile newFile)) {\n\t\t\treturn;\n\t\t}\n\t\tDisplay display = Display.getDefault();\n\t\tdisplay.asyncExec(() -> {\n\t\t\tif (display.isDisposed() || getContainer() == null || getContainer().isDisposed()) {\n\t\t\t\treturn;\n\t\t\t}\n\t\t\tsetInput(new FileEditorInput(newFile));\n\t\t\tupdateTitle();\n\t\t});\n\t}\n\n\tprivate void updateTitle() {\n\t\tIEditorInput input = getEditorInput();\n\t\tsetPartName(input.getName());\n\t\tsetTitleToolTip(input.getToolTipText());\n\t\tif (editor != null && getPageCount() > 0) {\n\t\t\tsetPageText(0, editor.getTitle());\n\t\t}\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
    id: AISessionEditor.doSaveAs
    source: "\t@Override\n\tpublic void doSaveAs() {\n\t\tIEditorPart editor = getEditor();\n\t\teditor.doSaveAs();\n\t\tsetInput(editor.getEditorInput());\n\t\tupdateTitle();\n\t}\n"
  reason: Auf Verschieben/Umbenennen reagieren und Titel aktualisieren
```
/answer 906f-1c-0 allow
Result Stats: id=758aff49-dd9f-4afc-a188-ee289d9be155, total: 14761, in: 14, out: 5484, reason: 0, read: 61649, write: 9263, start: 1795643107368, end: 1795643112873, time: 5483, total: 39897, io: 0, control: 22, sent: 0, recv: 132
Ich habe `AISessionEditor.java` angepasst, sodass der Tab-Titel nach einem Verschieben oder Umbenennen aktualisiert wird. Kompiliert oder getestet habe ich nichts.

**Ursache:** `resourceChanged` hat nur `PRE_CLOSE` behandelt. Auf Verschieben oder Umbenennen hat der Editor nicht reagiert. Seine Eingabe und der Part-Name blieben deshalb auf dem alten Stand.

**Änderungen:**
- **Neuer Zweig für `POST_CHANGE`:** `resourceChanged` ruft jetzt `handleMove(event)` auf.
- **Neue Methode `handleMove`:** Sie sucht im Resource-Delta nach der Datei des Editors. Ist sie mit `MOVED_TO` entfernt worden, setzt sie im UI-Thread per `asyncExec` die neue Eingabe (`setInput(new FileEditorInput(newFile))`) und aktualisiert den Titel. Das gilt auch, wenn ein übergeordneter Ordner verschoben oder umbenannt wurde. Die Methode prüft, ob der Editor schon disposed ist.
- **Neue Methode `updateTitle`:** Sie setzt Part-Name, Tooltip und Seitentext aus der aktuellen Eingabe.
- **`doSaveAs`:** Ruft jetzt ebenfalls `updateTitle()` auf. Vorher wurde dort nur der Seitentext gesetzt, nicht der Part-Name, der den Tab-Titel bestimmt.
- **Imports:** `IFile`, `IResource`, `IResourceDelta` und `IEditorInput` sind ergänzt.

Zum Testen: Öffne eine Session-Datei und benenne sie um. Verschiebe sie danach in einen anderen Ordner. Der Tab-Titel sollte sich jeweils ändern.
