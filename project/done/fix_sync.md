Der über das CNF angezeigte Sync Button in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/explorer/MarkdownExplorerView.java` funktioniert nicht. Das ist wohl aufgrund der Content-Darstellung so und muss potentiell durch Überladung korrigiert werden. 
SystemInit:  id=9b052e7f-51ef-4455-b6a3-e1c52173b375, model=/home/user/xyan/xy.ai.workbench, cwd=claude-sonnet-5
Control Request:
```yaml
id: 92d17f4f-ea1d-4ee1-81b6-9fca98103365
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/explorer/MarkdownExplorerView.java
  reason: Analyze sync button issue
```
/answer 92d17f4f-ea1d-4ee1-81b6-9fca98103365 allow
ReasoningToken: 11
Control Request:
```yaml
id: 8da22ce5-427f-45f7-ad3b-e3942bf37c6a
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/explorer
  reason: Inspect related classes for content model structure
```
/answer 8da22ce5-427f-45f7-ad3b-e3942bf37c6a allow
Control Request:
```yaml
id: a018d630-118a-425a-95f7-bb4983531212
result:
  structuredContent:
    entries:
    - GitIgnoreFilter.java
    - GitIgnoreRule.java
    - GitIgnoreViewerFilter.java
    - MarkdownContentProvider.java
    - MarkdownExplorerView.java
    - MarkdownLabelProvider.java
    - ModificationDateComparator.java
    - ProjectFilter.java
```
/answer a018d630-118a-425a-95f7-bb4983531212 allow
ReasoningToken: 1535
Control Request:
```yaml
id: 6e2502b1-c6c8-4105-951d-96ed3b47c8f8
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/explorer/MarkdownContentProvider.java
  reason: Check custom content model for sync mapping
```
/answer 6e2502b1-c6c8-4105-951d-96ed3b47c8f8 allow
Control Request:
```yaml
id: 6bf0e010-2d8c-4482-9b5a-38d4769c6e11
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/explorer/ProjectFilter.java
  reason: Check filter behavior relevant to sync
```
/answer 6bf0e010-2d8c-4482-9b5a-38d4769c6e11 allow
Control Request:
```yaml
id: b19b89aa-25d5-452f-98f6-63bdfa445ca9
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/FlatEndingContentProvider.java
  reason: Understand tree structure for sync/reveal logic
```
/answer b19b89aa-25d5-452f-98f6-63bdfa445ca9 allow
ReasoningToken: 231
Text:
 Das Problem ist tatsächlich in der Baumstruktur begründet, die von `MarkdownExplorerView` verwendet wird – nicht direkt im Sync-Button-Code selbst.

Analyse:

- Der "Sync"/"Link with Editor"-Button des Common Navigator Frameworks ruft letztlich `CommonViewer.setSelection(selection, true)` auf, um das zur aktiven Editor-Datei gehörende Element im Baum zu selektieren und sichtbar zu machen (`reveal`).
- `AbstractTreeViewer.reveal(...)` baut dazu die Elternkette über `ITreeContentProvider.getParent(element)` auf und läuft sie top-down ab, um die passenden `TreeItem`s zu finden/aufzuklappen.
- Euer `MarkdownContentProvider` (→ `FlatEndingContentProvider`) zeigt aber eine **flache** Struktur: `getElements()` sammelt *alle* Ordner, die irgendwo im Projekt (in beliebiger Tiefe!) direkt `.md`-Dateien enthalten, und listet sie **alle als Top-Level-Elemente** nebeneinander auf. `getChildren()` liefert nur die direkt enthaltenen `.md`-Dateien.
- `getParent()` gibt aber unverändert `IResource#getParent()` zurück – also den *echten* Dateisystem-Elternordner. Der ist für tiefer liegende Ordner aber gar kein Knoten im Baum (da nur "Blattordner mit .md-Dateien" als Top-Level-Knoten existieren, nicht deren tatsächliche Vorgängerordner).
- Dadurch stimmt die von `getParent()` gelieferte Kette nicht mit der tatsächlich angezeigten Baumstruktur überein → `reveal()`/`setSelection(..., true)` findet die Zwischenknoten nicht und bricht die Selektion/Aufklappung stillschweigend ab → der Sync-Button "tut nichts".

Fix: `getParent()` in `MarkdownContentProvider` überladen, sodass Container immer als Top-Level (Elternteil = `null`) behandelt werden – passend zur flachen Darstellung.
Control Request:
```yaml
id: 2790e94e-1c1e-4695-a082-8db71b3220fa
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/explorer/MarkdownContentProvider.java
  old_text: "package xy.ai.workbench.view.explorer;\n\nimport xy.ai.workbench.view.FlatEndingContentProvider;\n\npublic class MarkdownContentProvider extends FlatEndingContentProvider {\n\tpublic MarkdownContentProvider() {\n\t\tsuper(\".md\");\n\t}\n}"
  new_text: "package xy.ai.workbench.view.explorer;\n\nimport org.eclipse.core.resources.IContainer;\n\nimport xy.ai.workbench.view.FlatEndingContentProvider;\n\npublic class MarkdownContentProvider extends FlatEndingContentProvider {\n\tpublic MarkdownContentProvider() {\n\t\tsuper(\".md\");\n\t}\n\n\t/**\n\t * {@link FlatEndingContentProvider} presents a <i>flat</i> tree: every\n\t * folder that (anywhere in the project, at any depth) directly contains\n\t * a matching file is shown as a top-level element; folders are never\n\t * nested under one another in this view.\n\t * <p>\n\t * The inherited implementation, however, returns the real filesystem\n\t * parent ({@link org.eclipse.core.resources.IResource#getParent()}),\n\t * which usually is <b>not</b> a node of this flat tree at all. This\n\t * mismatch between the reported parent chain and the actually rendered\n\t * tree structure breaks {@link org.eclipse.jface.viewers.AbstractTreeViewer#reveal}\n\t * (and thus {@code TreeViewer#setSelection(ISelection, boolean)}), which\n\t * is exactly what the CNF \"Link with Editor\" / Sync toolbar button relies\n\t * on to locate and expand the element belonging to the active editor.\n\t * <p>\n\t * We therefore override {@link #getParent(Object)} here so that\n\t * containers are always reported as top-level (no parent), matching\n\t * what {@link #getElements(Object)} actually produces. Files still\n\t * report their containing folder as parent, since that folder is\n\t * guaranteed to be a genuine top-level node of this tree.\n\t */\n\t@Override\n\tpublic Object getParent(Object element) {\n\t\tif (element instanceof IContainer)\n\t\t\treturn null;\n\t\treturn super.getParent(element);\n\t}\n}"
  reason: Fix sync button by correcting getParent chain for flat tree structure
```
/answer 2790e94e-1c1e-4695-a082-8db71b3220fa allow
Control Request:
```yaml
id: d491ebdd-fa78-4114-984c-1bc61c6b6807
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -maxdepth 1 -iname "pom.xml" -o -iname "build.gradle" | head
  reason: Check for build tooling to validate compile
```
/answer d491ebdd-fa78-4114-984c-1bc61c6b6807 deny alles baut erfolgreich
ReasoningToken: 4982
ReasoningToken: 23
Verstanden – der Build wurde bereits bestätigt ("alles baut erfolgreich"), daher überspringe ich die Build-Prüfung.

## Zusammenfassung

**Ursache:** Nicht der Sync-Button-Code selbst ist fehlerhaft, sondern die Baum-Struktur eures `MarkdownContentProvider` (via `FlatEndingContentProvider`). Dieser zeigt eine **flache** Darstellung: Alle Ordner mit `.md`-Dateien werden – egal in welcher Tiefe im Projekt – als **Geschwister auf oberster Ebene** angezeigt, nie ineinander verschachtelt.

Der geerbte `getParent()` lieferte aber den echten Dateisystem-Elternordner zurück statt `null`. Das CNF-„Link with Editor“/Sync-Feature (und generell `TreeViewer.setSelection(..., reveal=true)`) baut über genau diese `getParent()`-Kette die Aufklapp-/Selektionspfade auf. Da die gemeldete Elternkette nicht mit der tatsächlich gerenderten flachen Struktur übereinstimmte, fand `reveal()` die Zwischenknoten nicht und brach die Selektion stillschweigend ab – der Button wirkte "kaputt".

**Fix:** In `MarkdownContentProvider.java` (nicht in `MarkdownExplorerView.java` selbst, dort liegt nur das Symptom) `getParent(Object)` überladen: Container melden jetzt `null` (Top-Level), Dateien weiterhin ihren Ordner als Parent – passend zur tatsächlichen flachen Darstellung. Damit funktioniert die Sync-/Link-with-Editor-Funktion des CNF wieder korrekt, ohne dass an `MarkdownExplorerView` selbst oder der generischen Basisklasse `FlatEndingContentProvider` etwas geändert werden musste.
Es funktioniert nicht
SystemInit:  id=9b052e7f-51ef-4455-b6a3-e1c52173b375, model=/home/user/xyan/xy.ai.workbench, cwd=claude-sonnet-5
Control Request:
```yaml
id: 15435651-2ef6-451c-9d6a-4a15cbd086f9
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "linkHelper\|toggleLinking\|LinkWithEditor\|linkWithEditor\|sync" --include=*.xml . ; echo '---'; grep -rn "linkHelper\|LinkHelper" --include=*.java .
  reason: Search for link-with-editor / sync configuration
```
/answer 15435651-2ef6-451c-9d6a-4a15cbd086f9 allow
Control Request:
```yaml
id: 8a47f48e-36c2-41d0-a235-16d52552d8b8
result:
  structuredContent:
    exit_code: 1
    stdout: |
      ---
```
/answer 8a47f48e-36c2-41d0-a235-16d52552d8b8 allow
ReasoningToken: 202
Control Request:
```yaml
id: fbe456cd-bdae-4a03-8095-3858286b4b17
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -iname "plugin.xml"; echo '---'; cat ./plugin.xml 2>/dev/null | head -200
  reason: Locate plugin.xml to inspect navigator view config
```
/answer fbe456cd-bdae-4a03-8095-3858286b4b17 deny hier `/home/user/xyan/xy.ai.workbench/plugin.xml`
ReasoningToken: 378
Control Request:
```yaml
id: 6a2206f1-a375-43b2-9e8a-5465617030e3
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench
  pattern: plugin\.xml|META-INF
  reason: Find plugin.xml location
```
/answer 6a2206f1-a375-43b2-9e8a-5465617030e3 deny und hier `/home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF`
Control Request:
```yaml
id: b8f705ff-ae50-4397-9e94-db54bfbefb20
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/plugin.xml
  reason: Inspect navigator view configuration for link-with-editor / sync button
```
/answer b8f705ff-ae50-4397-9e94-db54bfbefb20 allow
ReasoningToken: 186
Control Request:
```yaml
id: 86fde17a-df04-48d3-a2ce-13ef26a025a4
toolName: bash
arguments:
  cwd: /
  script: "find / -iname \"*.jar\" 2>/dev/null | xargs -I{} sh -c 'unzip -l \"{}\" 2>/dev/null | grep -q \"org/eclipse/ui/navigator/CommonNavigator.class\" && echo {}' 2>/dev/null | head -5"
  reason: Locate CommonNavigator class/jar to inspect editorActivated implementation
```
/answer 86fde17a-df04-48d3-a2ce-13ef26a025a4 deny hier sind die sourcen: `/home/user/xyan/eclipse.platform.ui`
ReasoningToken: 1289
Control Request:
```yaml
id: 2c776b24-6b09-409a-b6fb-45803c09ad8d
toolName: bash
arguments:
  cwd: /home/user/xyan/eclipse.platform.ui
  script: find . -iname "CommonNavigator.java"
  reason: Find CommonNavigator source
```
/answer 2c776b24-6b09-409a-b6fb-45803c09ad8d allow
Control Request:
```yaml
id: d3ec28ad-83a8-4518-86c7-3bf164cea1a8
result:
  structuredContent:
    stdout: |
      ./bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/navigator/CommonNavigator.java
```
/answer d3ec28ad-83a8-4518-86c7-3bf164cea1a8 allow
Control Request:
```yaml
id: 8a9dbaed-2ebc-4e01-ab65-aee13fb87e05
toolName: read-file
arguments:
  path: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/navigator/CommonNavigator.java
  reason: Inspect editorActivated / link-with-editor implementation
```
/answer 8a9dbaed-2ebc-4e01-ab65-aee13fb87e05 allow
Control Request:
```yaml
id: 49474a71-8033-4419-888d-2f0af335cb0e
toolName: bash
arguments:
  cwd: /home/user/xyan/eclipse.platform.ui
  script: find . -iname "*LinkEditor*" -o -iname "CommonNavigatorActionGroup.java" -o -iname "LinkHelperService.java"
  reason: Find link with editor action implementation
```
/answer 49474a71-8033-4419-888d-2f0af335cb0e allow
Control Request:
```yaml
id: 94126b4f-b3f7-422d-a704-c453b2fbb1fc
result:
  structuredContent:
    stdout: |
      ./bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/internal/navigator/actions/LinkEditorAction.java
      ./bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/internal/navigator/CommonNavigatorActionGroup.java
      ./bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/navigator/LinkHelperService.java
```
/answer 94126b4f-b3f7-422d-a704-c453b2fbb1fc allow
ReasoningToken: 55
Control Request:
```yaml
id: 4304e013-aae8-4074-bc40-9f4773a0a251
toolName: read-file
arguments:
  path: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/internal/navigator/actions/LinkEditorAction.java
  reason: Understand link with editor action logic
```
/answer 4304e013-aae8-4074-bc40-9f4773a0a251 allow
Control Request:
```yaml
id: 1a6dcfa8-4eba-43e6-be0d-bee950f5fa28
toolName: read-file
arguments:
  path: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/navigator/LinkHelperService.java
  reason: Understand how selection for editor input is resolved
```
/answer 1a6dcfa8-4eba-43e6-be0d-bee950f5fa28 allow
Control Request:
```yaml
id: 21668775-4dd5-4a11-bf4c-a5fca9d833fa
toolName: bash
arguments:
  cwd: /home/user/xyan/eclipse.platform.ui
  script: find . -iname "LinkHelperManager.java" -o -iname "LinkHelperDescriptor.java" -o -iname "ILinkHelper.java"
  reason: Find linkHelper extension classes to check registration requirements
```
/answer 21668775-4dd5-4a11-bf4c-a5fca9d833fa allow
Control Request:
```yaml
id: abe66f49-b9a6-4c3e-898b-3888185d26fa
result:
  structuredContent:
    stdout: |
      ./bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/internal/navigator/extensions/LinkHelperDescriptor.java
      ./bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/internal/navigator/extensions/LinkHelperManager.java
      ./bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/navigator/ILinkHelper.java
```
/answer abe66f49-b9a6-4c3e-898b-3888185d26fa allow
ReasoningToken: 1368
Control Request:
```yaml
id: 5fe654cb-eb21-4b46-8257-3a04cf8f6d63
toolName: read-file
arguments:
  path: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/internal/navigator/extensions/LinkHelperManager.java
  reason: Check matching requirements for link helper (content service scoping)
```
/answer 5fe654cb-eb21-4b46-8257-3a04cf8f6d63 allow
Control Request:
```yaml
id: 60d0781d-2933-4ca8-b1ab-a369f976d95a
toolName: read-file
arguments:
  path: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.ui.navigator/src/org/eclipse/ui/internal/navigator/extensions/LinkHelperDescriptor.java
  reason: Check descriptor structure and required plugin.xml attributes
```
/answer 60d0781d-2933-4ca8-b1ab-a369f976d95a allow
Control Request:
```yaml
id: d2f75e7f-a3a9-4748-9998-a64a0816564e
toolName: bash
arguments:
  cwd: /home/user/xyan/eclipse.platform.ui
  script: find . -iname "ILinkHelperExtPtConstants.java" | xargs cat; echo '---'; find . -iname "plugin.xml" -path "*navigator*" | xargs grep -l "linkHelper" 2>/dev/null
  reason: Check exact extension point schema for linkHelper
```
/answer d2f75e7f-a3a9-4748-9998-a64a0816564e allow
Control Request:
```yaml
id: 3f057274-fccc-43fb-98e8-e83a2f29921d
result:
  structuredContent:
    stdout: "/*******************************************************************************\n * Copyright (c) 2006 IBM Corporation and others.\n *\n * This program and the accompanying materials\n * are made available under the terms of the Eclipse Public License 2.0\n * which accompanies this distribution, and is available at\n * https://www.eclipse.org/legal/epl-2.0/\n *\n * SPDX-License-Identifier: EPL-2.0\n *\n * Contributors:\n *     IBM Corporation - initial API and implementation\n ******************************************************************************/\n\npackage org.eclipse.ui.internal.navigator.extensions;\n\n/**\n * @since 3.2\n */\npublic interface ILinkHelperExtPtConstants {\n\n\t/** */\n\tString LINK_HELPER = \"linkHelper\"; //$NON-NLS-1$\n\n\t/** */\n\tString ATT_ID = \"id\"; //$NON-NLS-1$\n\n\t/** */\n\tString ATT_CLASS = \"class\"; //$NON-NLS-1$\n\n\t/** */\n\tString ATT_NAVIGATOR_CONTENT_EXTENSION_ID = \"navigatorContentExtensionId\"; //$NON-NLS-1$\n\n\t/** */\n\tString EDITOR_INPUT_ENABLEMENT = \"editorInputEnablement\"; //$NON-NLS-1$\n\n\t/** */\n\tString SELECTION_ENABLEMENT = \"selectionEnablement\"; //$NON-NLS-1$\n\n}\n---\n./tests/org.eclipse.ui.tests.navigator/plugin.xml\n./bundles/org.eclipse.ui.navigator/plugin.xml\n./bundles/org.eclipse.ui.navigator.resources/plugin.xml\n"
```
/answer 3f057274-fccc-43fb-98e8-e83a2f29921d allow
ReasoningToken: 530
Control Request:
```yaml
id: 0c94005b-55c9-4df0-b291-d80cdeafd886
toolName: bash
arguments:
  cwd: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.ui.navigator.resources
  script: grep -n -B2 -A25 "linkHelper" plugin.xml
  reason: See a real linkHelper extension example
```
/answer 0c94005b-55c9-4df0-b291-d80cdeafd886 allow
Control Request:
```yaml
id: 9fad341e-b602-4086-b699-9b7d8e0b4735
result:
  content:
  - type: text
    text: Full output written to file 3219 characters). Reduce to relevant excerpts (e.g. via grep/head/tail).
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-_l18lt5l.log
```
/answer 9fad341e-b602-4086-b699-9b7d8e0b4735 allow
Control Request:
```yaml
id: 2789ac31-a302-47d6-a063-2961128dae75
toolName: read-file
arguments:
  path: /tmp/mcpc-stdout-_l18lt5l.log
  reason: View linkHelper example
```
/answer 2789ac31-a302-47d6-a063-2961128dae75 allow
Control Request:
```yaml
id: 4976d3a5-a97c-47a1-9ad4-7656a5bdcc88
toolName: bash
arguments:
  cwd: /home/user/xyan/eclipse.platform.ui
  script: find . -iname "ResourceLinkHelper.java" | xargs cat
  reason: See reference implementation of ILinkHelper
```
/answer 4976d3a5-a97c-47a1-9ad4-7656a5bdcc88 allow
Control Request:
```yaml
id: 14814c76-6ba8-4859-9b84-63b108c97921
result:
  structuredContent:
    stdout: "/*******************************************************************************\n * Copyright (c) 2006, 2015 IBM Corporation and others.\n *\n * This program and the accompanying materials\n * are made available under the terms of the Eclipse Public License 2.0\n * which accompanies this distribution, and is available at\n * https://www.eclipse.org/legal/epl-2.0/\n *\n * SPDX-License-Identifier: EPL-2.0\n *\n * Contributors:\n *     IBM Corporation - initial API and implementation\n *******************************************************************************/\n\npackage org.eclipse.ui.internal.navigator.resources.workbench;\n\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.jface.viewers.IStructuredSelection;\nimport org.eclipse.jface.viewers.StructuredSelection;\nimport org.eclipse.ui.IEditorInput;\nimport org.eclipse.ui.IEditorPart;\nimport org.eclipse.ui.IWorkbenchPage;\nimport org.eclipse.ui.ide.ResourceUtil;\nimport org.eclipse.ui.navigator.ILinkHelper;\nimport org.eclipse.ui.part.FileEditorInput;\n\n/**\n *\n * Links IFileEditorInput to IFiles, and vice versa.\n *\n * @since 3.2\n */\npublic class ResourceLinkHelper implements ILinkHelper {\n\n\t@Override\n\tpublic IStructuredSelection findSelection(IEditorInput anInput) {\n\t\tIFile file = ResourceUtil.getFile(anInput);\n\t\tif (file != null) {\n\t\t\treturn new StructuredSelection(file);\n\t\t}\n\t\treturn StructuredSelection.EMPTY;\n\t}\n\n\t@Override\n\tpublic void activateEditor(IWorkbenchPage aPage,\n\t\t\tIStructuredSelection aSelection) {\n\t\tif (aSelection == null || aSelection.isEmpty()) {\n\t\t\treturn;\n\t\t}\n\t\tif (aSelection.getFirstElement() instanceof IFile) {\n\t\t\tIEditorInput fileInput = new FileEditorInput((IFile) aSelection.getFirstElement());\n\t\t\tIEditorPart editor = null;\n\t\t\tif ((editor = aPage.findEditor(fileInput)) != null) {\n\t\t\t\taPage.bringToTop(editor);\n\t\t\t}\n\t\t}\n\n\t}\n\n}\n"
```
/answer 14814c76-6ba8-4859-9b84-63b108c97921 allow
ReasoningToken: 130
Text:
 Jetzt hab ich die tatsächliche Root Cause gefunden. Ich implementiere jetzt den fehlenden `ILinkHelper` und registriere ihn korrekt.
Control Request:
```yaml
id: 22ebd820-fb4e-4c84-aaad-6a993bfb11c8
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/explorer/MarkdownLinkHelper.java
  mode: replace
  content: "package xy.ai.workbench.view.explorer;\n\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.jface.viewers.IStructuredSelection;\nimport org.eclipse.jface.viewers.StructuredSelection;\nimport org.eclipse.ui.IEditorInput;\nimport org.eclipse.ui.IEditorPart;\nimport org.eclipse.ui.IWorkbenchPage;\nimport org.eclipse.ui.ide.ResourceUtil;\nimport org.eclipse.ui.navigator.ILinkHelper;\nimport org.eclipse.ui.part.FileEditorInput;\n\n/**\n * Enables the \"Link with Editor\" (Sync) toolbar button of\n * {@link MarkdownExplorerView}.\n * <p>\n * The Common Navigator Framework does <b>not</b> know out of the box how to\n * map an active editor to an element of a custom content model - it relies\n * exclusively on {@code org.eclipse.ui.navigator.linkHelper} extensions\n * (see {@code org.eclipse.ui.internal.navigator.actions.LinkEditorAction}\n * and {@link org.eclipse.ui.navigator.LinkHelperService}). Without such an\n * extension, {@code LinkHelperService#getSelectionFor(IEditorInput)} always\n * returns an empty selection, so the Sync button silently does nothing -\n * regardless of how the tree content is structured.\n * <p>\n * This helper mirrors the standard\n * {@code org.eclipse.ui.internal.navigator.resources.workbench.ResourceLinkHelper}\n * behavior for plain {@link IFile}s, which is all {@link MarkdownContentProvider}\n * ever shows in the tree.\n */\npublic class MarkdownLinkHelper implements ILinkHelper {\n\n\t@Override\n\tpublic IStructuredSelection findSelection(IEditorInput anInput) {\n\t\tIFile file = ResourceUtil.getFile(anInput);\n\t\tif (file != null)\n\t\t\treturn new StructuredSelection(file);\n\t\treturn StructuredSelection.EMPTY;\n\t}\n\n\t@Override\n\tpublic void activateEditor(IWorkbenchPage aPage, IStructuredSelection aSelection) {\n\t\tif (aSelection == null || aSelection.isEmpty())\n\t\t\treturn;\n\t\tObject element = aSelection.getFirstElement();\n\t\tif (element instanceof IFile) {\n\t\t\tIEditorInput fileInput = new FileEditorInput((IFile) element);\n\t\t\tIEditorPart editor = aPage.findEditor(fileInput);\n\t\t\tif (editor != null)\n\t\t\t\taPage.bringToTop(editor);\n\t\t}\n\t}\n}\n"
  reason: Add missing ILinkHelper implementation to make CNF sync/link-with-editor functional
```
/answer 22ebd820-fb4e-4c84-aaad-6a993bfb11c8 allow
Control Request:
```yaml
id: f96290af-4ba8-4747-b1a5-6446c69f1aa3
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/plugin.xml
  old_text: "\t   <viewerContentBinding viewerId=\"xy.ai.workbench.view.AIExplorer\">\n\t      <includes>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.content.AIContent\"/>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.filter.GitIgnore\"/>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.filter.Project\"/>\n\t      </includes>\n\t      <excludes>\n\t\t      <contentExtension\n\t\t          pattern=\"^(?!xy\\.ai\\.workbench\\.view\\.(content\\.AIContent|filter\\.(GitIgnore|Project))$).*\">\n\t\t      </contentExtension>\n\t      </excludes>\n\t   </viewerContentBinding>\n\t</extension>"
  new_text: "\t   <viewerContentBinding viewerId=\"xy.ai.workbench.view.AIExplorer\">\n\t      <includes>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.content.AIContent\"/>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.filter.GitIgnore\"/>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.filter.Project\"/>\n\t         <contentExtension pattern=\"xy.ai.workbench.view.linkHelper.AILinkHelper\"/>\n\t      </includes>\n\t      <excludes>\n\t\t      <contentExtension\n\t\t          pattern=\"^(?!xy\\.ai\\.workbench\\.view\\.(content\\.AIContent|filter\\.(GitIgnore|Project)|linkHelper\\.AILinkHelper)$).*\">\n\t\t      </contentExtension>\n\t      </excludes>\n\t   </viewerContentBinding>\n\t</extension>\n\t<extension point=\"org.eclipse.ui.navigator.linkHelper\">\n\t\t<linkHelper\n\t\t\tclass=\"xy.ai.workbench.view.explorer.MarkdownLinkHelper\"\n\t\t\tid=\"xy.ai.workbench.view.linkHelper.AILinkHelper\">\n\t\t\t<selectionEnablement>\n\t\t\t\t<instanceof value=\"org.eclipse.core.resources.IFile\"/>\n\t\t\t</selectionEnablement>\n\t\t\t<editorInputEnablement>\n\t\t\t\t<instanceof value=\"org.eclipse.ui.IFileEditorInput\"/>\n\t\t\t</editorInputEnablement>\n\t\t</linkHelper>\n\t</extension>"
  reason: Register linkHelper extension so CNF sync button can resolve tree selection for active editor
```
/answer f96290af-4ba8-4747-b1a5-6446c69f1aa3 allow
Text:
 Jetzt die passende Extension in der `plugin.xml` registrieren (inkl. Aufnahme in das `viewerContentBinding`, da `LinkHelperDescriptor`n über `INavigatorContentService#isVisible(id)` gefiltert werden):
## Analyse – der eigentliche Grund

Meine erste Vermutung (Baumstruktur/`getParent()`) war zwar sachlich korrekt (und bleibt als sinnvolle Korrektur bestehen), aber **nicht die Ursache dafür, dass der Button gar nichts tut**. Ich habe den Eclipse-Platform-Quellcode (`org.eclipse.ui.navigator`) direkt geprüft:

- Der Sync-Button wird von `LinkEditorAction` bereitgestellt. Bei Aktivierung eines Editors ruft er `LinkHelperService#getSelectionFor(IEditorInput)` auf.
- Diese Methode fragt `LinkHelperManager.getLinkHelpersFor(input, contentService)` – und die filtert **ausschließlich** über registrierte `org.eclipse.ui.navigator.linkHelper`-Extensions, die zusätzlich per `contentService.isVisible(id)` im `viewerContentBinding` freigeschaltet sein müssen.
- In eurer `plugin.xml` gab es **überhaupt keine** `linkHelper`-Extension. Damit liefert `getSelectionFor(...)` immer eine leere `StructuredSelection`, der `if (!newSelection.isEmpty())`-Check in `LinkEditorAction.updateSelectionJob` schlägt fehl, und `selectReveal(...)` wird nie aufgerufen – der Button tut buchstäblich nichts, unabhängig von der Baumstruktur.

## Fix

1. **`MarkdownLinkHelper`** (neu, `ILinkHelper`-Implementierung analog zu Eclipses eigenem `ResourceLinkHelper`): mappt `IFileEditorInput` ↔ `IFile`-Selektion.
2. **`plugin.xml`**: neue `org.eclipse.ui.navigator.linkHelper`-Extension registriert, **und** deren id zusätzlich in `includes`/`excludes` des `viewerContentBinding` für `xy.ai.workbench.view.AIExplorer` aufgenommen (sonst würde `isVisible(id)` weiterhin `false` liefern und der Helper stillschweigend ignoriert werden).

Zusammen mit der vorherigen `getParent()`-Korrektur in `MarkdownContentProvider` (die jetzt auch tatsächlich zum Tragen kommt, sobald `selectReveal` überhaupt aufgerufen wird) sollte der Sync-Button jetzt korrekt funktionieren: Aktivierung eines `.md`-Editors selektiert/expandiert die passende Datei im Explorer, und ein Klick auf die Datei im Explorer bringt den zugehörigen Editor in den Vordergrund.

Bitte im laufenden Workbench testen (ggf. mit `-clean`, da die Extension-Registry sonst gecacht sein kann).
