Vielleicht hast du eine Idee. Ich habe an 2 Stellen eine Multiselect Liste. Das SWT Default verhalten musste ich überschreiben und zum Teil kompensieren, um das gewünschte zu erreichen.

Das verhalten soll sein:

- Per einfachem Click kann ich ein Element aktivieren/deaktivieren
- Beim Drücken von STRG, wird bei einem Click nur dieses Element gewählt (Das ist quasi das inverse Verhalten zu anderen Listen)
- Beim Drücken von Shift soll eine Range vom zuerst Selektierten bis zum zuletzt Selektierten markiert werden.   

Das Ganze hat jedoch seine Macken. Die größte ist das sich die Liste potenziell zurücksetzt beim ersten Click, wenn der interne Zustand abgewichen ist.

Der SelectTab `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java` hat eine zusätzliche Anforderung. Er ist darüber hinaus mit einem Freitextfeld kombiniert, indem sich Freitext und Bulkeingaben tätigen lassen.

Der ToolsTab ist hingegen einfacher.
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java`

Für das ToolsTab ist das Verhalten ausgelagert.
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java`

Vielleicht kannst du dir das ganze mal anschauen oder findet eine bessere Möglichkeit das Default SWT verhalten Robuster zu modifizieren.
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst die drei Dateien an.
ReasoningToken: 13
Control Request:
```yaml
id: 9c1b-1-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  reason: "Struktur der drei Dateien verschaffen, um das Multiselect-Verhalten zu analysieren"
```
/answer 9c1b-1-0 allow
Control Request:
```yaml
id: 9c1b-2-1
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
      nodes:
      - id: b14ok4|8NBSXT
        signature: import java.util.ArrayList;
      - id: MultiSelectListener
        signature: "public class MultiSelectListener implements Clearable {"
        children:
        - id: MultiSelectListener.tmr22J|qTgU5n
          signature: private ArrayList<Integer> selectedIndices = new ArrayList<>();
        - id: MultiSelectListener.MultiSelectListener
          signature: public MultiSelectListener(List component)
        - id: MultiSelectListener.setSelection
          signature: "public void setSelection(String[] items)"
        - id: MultiSelectListener.clear
          signature: public void clear()
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      nodes:
      - id: wZA8ZO|OsHsIe
        signature: import java.util.Arrays;
      - id: ToolsTab
        signature: "/**  * \"Tools\" tab: multi-select list of tools enabled for the current model.  …"
        docstring: "\"Tools\" tab: multi-select list of tools enabled for the current model."
        children:
        - id: ToolsTab.c4iLED|KWV8qC
          signature: private List toolsList;
        - id: ToolsTab.create
          signature: "public void create(TabFolder instr, ConfigManager cfg)"
        - id: ToolsTab.getSelection
          signature: "public String[] getSelection()"
        - id: ToolsTab.setSelection
          signature: "public void setSelection(String[] items)"
        - id: ToolsTab.clear
          signature: public void clear()
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      nodes:
      - id: k0tyZU|lKJhSS
        signature: import java.util.ArrayList;
      - id: SelectTab
        signature: "/**  * \"Select\" tab: toggles system-prompt lines on/off and holds the free-text…"
        docstring: "\"Select\" tab: toggles system-prompt lines on/off and holds the free-text"
        children:
        - id: SelectTab.FGxcw8|xBZMzM
          signature: private final FormToolkit toolkit;
        - id: SelectTab.SelectTab
          signature: "public SelectTab(FormToolkit toolkit, UpdateGuard guard, Runnable reflow)"
        - id: SelectTab.create
          signature: "public void create(TabFolder instr, ConfigManager cfg)"
        - id: SelectTab.createFreeTextArea
          signature: "private void createFreeTextArea(SashForm sash, ConfigManager cfg)"
        - id: SelectTab.updatePromptLines
          signature: "private String[] updatePromptLines(String[] cur)"
        - id: SelectTab.updateInstructionList
          signature: "private void updateInstructionList(String[] systemPrompt)"
        - id: SelectTab.clear
          signature: public void clear()
```
/answer 9c1b-2-1 allow
Control Request:
```yaml
id: 9c1b-3-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  node_type: ClassDeclaration|class_declaration
  reason: Vollständigen Quelltext der Klassen lesen
```
/answer 9c1b-3-0 allow
Control Request:
```yaml
id: 9c1b-4-3
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      nodes: []
```
/answer 9c1b-4-3 allow
Control Request:
```yaml
id: 9c1b-5-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
    ids:
    - MultiSelectListener
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    ids:
    - ToolsTab
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    ids:
    - SelectTab
  reason: Vollständigen Quelltext der drei Klassen lesen
```
/answer 9c1b-5-0 allow
Control Request:
```yaml
id: 9c1b-6-5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
      nodes:
      - id: MultiSelectListener
        type: ClassOrInterfaceDeclaration
        signature: "public class MultiSelectListener implements Clearable {"
        children:
        - id: MultiSelectListener.tmr22J|qTgU5n
          type: StatementGroup
          code: "\tprivate ArrayList<Integer> selectedIndices = new ArrayList<>();\n\tprivate final List component;\n"
        - id: MultiSelectListener.MultiSelectListener
          type: ConstructorDeclaration
          code: "\tpublic MultiSelectListener(List component) {\n\t\tthis.component = component;\n\t\tcomponent.addListener(SWT.MouseDown, event -> {\n\t\t\tint clickedIndex = component.getSelectionIndex();\n\t\t\tint topIdx = component.getTopIndex();\n\n\t\t\tif (selectedIndices.contains(clickedIndex))\n\t\t\t\tselectedIndices.remove(Integer.valueOf(clickedIndex));\n\t\t\telse\n\t\t\t\tselectedIndices.add(clickedIndex);\n\n\t\t\tint[] selection = selectedIndices.stream().mapToInt(Integer::intValue).sorted().toArray();\n\t\t\tcomponent.setSelection(selection);\n\t\t\tcomponent.setTopIndex(topIdx);\n\t\t});\n\t}\n"
        - id: MultiSelectListener.setSelection
          type: MethodDeclaration
          code: "\tpublic void setSelection(String[] items) {\n\t\tint topIdx = component.getTopIndex();\n\t\tjava.util.List<String> all = Arrays.asList(component.getItems());\n\t\tselectedIndices.clear();\n\t\tif (items != null)\n\t\t\tfor (String item : items) {\n\t\t\t\tint idx = all.indexOf(item);\n\t\t\t\tif (idx >= 0)\n\t\t\t\t\tselectedIndices.add(idx);\n\t\t\t}\n\t\tint[] selection = selectedIndices.stream().mapToInt(Integer::intValue).sorted().toArray();\n\t\tcomponent.setSelection(selection);\n\t\tcomponent.setTopIndex(topIdx);\n\t}\n"
        - id: MultiSelectListener.clear
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void clear() {\n\t\tselectedIndices.clear();\n\t\tcomponent.deselectAll();\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      nodes:
      - id: ToolsTab
        type: ClassOrInterfaceDeclaration
        signature: "/**  * \"Tools\" tab: multi-select list of tools enabled for the current model.  …"
        docstring: "\"Tools\" tab: multi-select list of tools enabled for the current model."
        children:
        - id: ToolsTab.c4iLED|KWV8qC
          type: StatementGroup
          code: "\tprivate List toolsList;\n\tprivate MultiSelectListener toolsMultiSelect;\n"
        - id: ToolsTab.create
          type: MethodDeclaration
          code: "\tpublic void create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem toolSel = new TabItem(instr, SWT.NONE);\n\t\ttoolSel.setText(\"Tools\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\ttoolSel.setControl(comp);\n\t\ttoolsList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\ttoolsList.setLayoutData(gridData);\n\t\ttoolsList.addMouseListener(MouseListener.mouseDownAdapter(m -> toolsList.setFocus()));\n\t\ttoolsList.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setEnabledTools(toolsList.getSelection())));\n\t\tcfg.addModelObs(m -> {\n\t\t\tif (m != null)\n\t\t\t\ttoolsList.setItems(m.cap.getTools());\n\t\t}, true);\n\t\ttoolsMultiSelect = new MultiSelectListener(toolsList);\n\t\tcfg.addEnabledToolsObs(t -> {\n\t\t\tif (!Arrays.equals(toolsList.getSelection(), t))\n\t\t\t\ttoolsMultiSelect.setSelection(t);\n\t\t}, true);\n\t}\n"
        - id: ToolsTab.getSelection
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String[] getSelection() {\n\t\treturn toolsList.getSelection();\n\t}\n"
        - id: ToolsTab.setSelection
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void setSelection(String[] items) {\n\t\ttoolsMultiSelect.setSelection(items);\n\t}\n"
        - id: ToolsTab.clear
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void clear() {\n\t\ttoolsMultiSelect.clear();\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      nodes:
      - id: SelectTab
        type: ClassOrInterfaceDeclaration
        signature: "/**  * \"Select\" tab: toggles system-prompt lines on/off and holds the free-text…"
        docstring: "\"Select\" tab: toggles system-prompt lines on/off and holds the free-text"
        children:
        - id: SelectTab.FGxcw8|xBZMzM
          type: StatementGroup
          code: "\tprivate final FormToolkit toolkit;\n\tprivate final UpdateGuard guard;\n\tprivate final Runnable reflow;\n\n\tprivate List instructionList;\n\tprivate Text instructionFree;\n\tprivate java.util.List<String> instructionSelection = new ArrayList<>();\n"
        - id: SelectTab.SelectTab
          type: ConstructorDeclaration
          code: "\tpublic SelectTab(FormToolkit toolkit, UpdateGuard guard, Runnable reflow) {\n\t\tthis.toolkit = toolkit;\n\t\tthis.guard = guard;\n\t\tthis.reflow = reflow;\n\t}\n"
        - id: SelectTab.create
          type: MethodDeclaration
          code: "\tpublic void create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem instrSel = new TabItem(instr, SWT.NONE);\n\t\tinstrSel.setText(\"Select\");\n\n\t\tSashForm sash = new SashForm(instr, SWT.VERTICAL);\n\t\tsash.setLayout(new GridLayout(1, false));\n\t\tGridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tscl2.heightHint = 100;\n\t\tscl2.widthHint = 1;\n\t\tsash.setLayoutData(scl2);\n\n\t\tComposite comp = new Composite(sash, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\tinstrSel.setControl(sash);\n\t\tinstructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\t\tcfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\tinstructionList.setLayoutData(gridData);\n\t\tinstructionList.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionList.setFocus()));\n\n\t\tinstructionList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tinstructionSelection = new ArrayList<>(Arrays.asList(instructionList.getSelection()));\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t}));\n\t\tinstructionList.addListener(SWT.MouseDown, event -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tString[] clickedIndex = instructionList.getSelection();\n\t\t\tLOG.info(Arrays.toString(clickedIndex));\n\t\t\tif (clickedIndex.length != 1)\n\t\t\t\treturn;\n\n\t\t\tif (!instructionSelection.remove(clickedIndex[0]))\n\t\t\t\tinstructionSelection.add(clickedIndex[0]);\n\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t});\n\n\t\tcreateFreeTextArea(sash, cfg);\n\t}\n"
        - id: SelectTab.createFreeTextArea
          type: MethodDeclaration
          code: "\tprivate void createFreeTextArea(SashForm sash, ConfigManager cfg) {\n\t\tinstructionFree = toolkit.createText(sash, \"\", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);\n\t\tcfg.addSystemFreeObs(p -> {\n\t\t\tif (!instructionFree.isFocusControl())\n\t\t\t\tguard.run(() -> {\n\t\t\t\t\tinstructionFree.setText(p != null ? p : \"\");\n\t\t\t\t\treflow.run();\n\t\t\t\t});\n\t\t}, true);\n\t\tinstructionFree\n\t\t\t\t.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setSystemFree(instructionFree.getText())));\n\t\tGridData gridData = new GridData(GridData.FILL_HORIZONTAL);\n\t\tgridData.widthHint = 1;\n\t\tinstructionFree.setLayoutData(gridData);\n\t\tinstructionFree.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionFree.setFocus()));\n\t\tsash.setWeights(3, 1);\n\t}\n"
        - id: SelectTab.updatePromptLines
          type: MethodDeclaration
          code: "\tprivate String[] updatePromptLines(String[] cur) {\n\t\tString[] upd = new String[cur.length];\n\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\tString line = cur[i];\n\t\t\tboolean isSelected = instructionSelection.contains(line);\n\t\t\tif (!isSelected && !line.startsWith(\"#\")) {\n\t\t\t\tline = \"#\" + line;\n\t\t\t} else if (isSelected && line.startsWith(\"#\")) {\n\t\t\t\tline = line.substring(1);\n\t\t\t}\n\t\t\tupd[i] = line;\n\t\t}\n\t\treturn upd;\n\t}\n"
        - id: SelectTab.updateInstructionList
          type: MethodDeclaration
          code: "\tprivate void updateInstructionList(String[] systemPrompt) {\n\t\tguard.run(() -> {\n\t\t\tinstructionList.setItems(systemPrompt);\n\t\t\tinstructionList.deselectAll();\n\t\t\tfor (int i = 0; i < systemPrompt.length; i++) {\n\t\t\t\tif (!systemPrompt[i].startsWith(\"#\"))\n\t\t\t\t\tinstructionList.select(i);\n\t\t\t}\n\t\t\treflow.run();\n\t\t});\n\t}\n"
        - id: SelectTab.clear
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void clear() {\n\t\tinstructionSelection.clear();\n\t}\n"
```
/answer 9c1b-6-5 allow
ReasoningToken: 3331
Control Request:
```yaml
id: 9c1b-7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
    pattern: ^(import|package)
    include:
    - MultiSelectListener.java
    - SelectTab.java
    - ToolsTab.java
    limit: 50
    intent: editing
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: MultiSelectListener|new List\(.*MULTI
    include:
    - '*.java'
    limit: 20
  reason: "Imports prüfen und feststellen, wo MultiSelectListener bzw. Multi-Listen sonst verwendet werden"
```
/answer 9c1b-7-0 allow
Control Request:
```yaml
id: 9c1b-8-7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
      matches:
      - path: MultiSelectListener.java
        nodes:
        - id: b14ok4|8NBSXT
          matches:
          - import java.util.ArrayList;
          - import java.util.Arrays;
          - import org.eclipse.swt.SWT;
          - import org.eclipse.swt.widgets.List;
          - import xy.ai.workbench.view.session.Clearable;
        matches:
        - lineno: 1
          match: package xy.ai.workbench.view;
      - path: session/SelectTab.java
        nodes:
        - id: k0tyZU|lKJhSS
          matches:
          - import java.util.ArrayList;
          - import java.util.Arrays;
          - import org.eclipse.swt.SWT;
          - import org.eclipse.swt.custom.SashForm;
          - import org.eclipse.swt.events.FocusListener;
          - import org.eclipse.swt.events.MouseListener;
          - import org.eclipse.swt.events.SelectionListener;
          - import org.eclipse.swt.layout.GridData;
          - import org.eclipse.swt.layout.GridLayout;
          - import org.eclipse.swt.widgets.Composite;
          - import org.eclipse.swt.widgets.List;
          - import org.eclipse.swt.widgets.TabFolder;
          - import org.eclipse.swt.widgets.TabItem;
          - import org.eclipse.swt.widgets.Text;
          - import org.eclipse.ui.forms.widgets.FormToolkit;
          - import xy.ai.conscious.core.ConfigManager;
          - import xy.ai.conscious.core.LOG;
        matches:
        - lineno: 1
          match: package xy.ai.workbench.view.session;
      - path: session/ToolsTab.java
        nodes:
        - id: wZA8ZO|OsHsIe
          matches:
          - import java.util.Arrays;
          - import org.eclipse.swt.SWT;
          - import org.eclipse.swt.events.MouseListener;
          - import org.eclipse.swt.events.SelectionListener;
          - import org.eclipse.swt.layout.GridData;
          - import org.eclipse.swt.layout.GridLayout;
          - import org.eclipse.swt.widgets.Composite;
          - import org.eclipse.swt.widgets.List;
          - import org.eclipse.swt.widgets.TabFolder;
          - import org.eclipse.swt.widgets.TabItem;
          - import xy.ai.conscious.core.ConfigManager;
          - import xy.ai.workbench.view.MultiSelectListener;
        matches:
        - lineno: 1
          match: package xy.ai.workbench.view.session;
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/MultiSelectListener.java
        matches:
        - "public class MultiSelectListener implements Clearable {"
        - "public MultiSelectListener(List component) {"
      - path: xy/ai/workbench/view/session/SelectTab.java
        matches:
        - "instructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);"
      - path: xy/ai/workbench/view/session/ToolsTab.java
        matches:
        - import xy.ai.workbench.view.MultiSelectListener;
        - private MultiSelectListener toolsMultiSelect;
        - "toolsList = new List(comp, SWT.MULTI | SWT.V_SCROLL);"
        - toolsMultiSelect = new MultiSelectListener(toolsList);
```
/answer 9c1b-8-7 allow
Thinking:
Die Ursache liegt darin, dass der Zustand doppelt geführt wird und beim Klick veraltet ist. Ich baue `MultiSelectListener` so um, dass das Widget die einzige Quelle ist, der Index aus den Mauskoordinaten kommt und das native Klickverhalten unterdrückt wird.


ReasoningToken: 443
Control Request:
```yaml
id: 9c1b-9-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
    overwrite: true
    source: "package xy.ai.workbench.view;\n\nimport java.util.Arrays;\nimport java.util.LinkedHashSet;\nimport java.util.Set;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.widgets.Event;\nimport org.eclipse.swt.widgets.List;\n\nimport xy.ai.workbench.view.session.Clearable;\n\n/**\n * Replaces the native click behaviour of a {@code SWT.MULTI} {@link List}:\n * <ul>\n * <li>plain click: toggles the clicked item</li>\n * <li>CTRL + click: selects only the clicked item</li>\n * <li>SHIFT + click: selects the range from the anchor (last clicked item) to\n * the clicked item</li>\n * </ul>\n * The widget itself is the single source of truth - no shadow state is kept, so\n * programmatic changes ({@code setItems}, {@code select}, keyboard, ...) can\n * never get out of sync. Only the range anchor is remembered, and it is\n * validated on every use.\n * <p>\n * Mouse changes are reported as regular {@link SWT#Selection} events, so\n * ordinary selection listeners keep working unchanged. Programmatic changes\n * (via {@link #setSelection(String[])}) do not fire events, like SWT itself.\n */\npublic class MultiSelectListener implements Clearable {\n\n\tprivate final List component;\n\tprivate int anchor = -1;\n\n\tpublic MultiSelectListener(List component) {\n\t\tthis.component = component;\n\t\tcomponent.addListener(SWT.MouseDown, this::onMouseDown);\n\t\t// keyboard (and any other native) selection moves the anchor, too;\n\t\t// our own synthetic events are marked with data == this\n\t\tcomponent.addListener(SWT.Selection, e -> {\n\t\t\tif (e.data != this)\n\t\t\t\tanchor = component.getFocusIndex();\n\t\t});\n\t}\n\n\tprivate void onMouseDown(Event event) {\n\t\tif (event.button != 1)\n\t\t\treturn;\n\n\t\t// we handle this click completely, suppress the native selection change\n\t\tevent.doit = false;\n\t\tcomponent.setFocus();\n\n\t\tint itemHeight = component.getItemHeight();\n\t\tint clicked = component.getTopIndex() + (itemHeight > 0 ? event.y / itemHeight : 0);\n\t\tif (clicked < 0 || clicked >= component.getItemCount())\n\t\t\treturn;\n\n\t\t// the widget state is the truth, read it fresh on every click\n\t\tSet<Integer> selection = new LinkedHashSet<>();\n\t\tfor (int i : component.getSelectionIndices())\n\t\t\tselection.add(i);\n\n\t\tboolean ctrl = (event.stateMask & SWT.MOD1) != 0;\n\t\tboolean shift = (event.stateMask & SWT.SHIFT) != 0;\n\n\t\tif (shift) {\n\t\t\tint from = validAnchor(selection, clicked);\n\t\t\tselection.clear();\n\t\t\tfor (int i = Math.min(from, clicked); i <= Math.max(from, clicked); i++)\n\t\t\t\tselection.add(i);\n\t\t\t// anchor stays, so the range can be adjusted by further shift-clicks\n\t\t} else if (ctrl) {\n\t\t\tselection.clear();\n\t\t\tselection.add(clicked);\n\t\t\tanchor = clicked;\n\t\t} else {\n\t\t\tif (!selection.remove(clicked))\n\t\t\t\tselection.add(clicked);\n\t\t\tanchor = clicked;\n\t\t}\n\n\t\tapply(selection.stream().mapToInt(Integer::intValue).sorted().toArray());\n\n\t\tEvent sel = new Event();\n\t\tsel.widget = component;\n\t\tsel.data = this;\n\t\tsel.index = clicked;\n\t\tsel.stateMask = event.stateMask;\n\t\tcomponent.notifyListeners(SWT.Selection, sel);\n\t}\n\n\t/** Anchor if still valid, otherwise first selected item, otherwise the click. */\n\tprivate int validAnchor(Set<Integer> selection, int clicked) {\n\t\tif (anchor >= 0 && anchor < component.getItemCount())\n\t\t\treturn anchor;\n\t\treturn selection.stream().min(Integer::compare).orElse(clicked);\n\t}\n\n\tprivate void apply(int[] indices) {\n\t\tint top = component.getTopIndex();\n\t\tcomponent.setSelection(indices);\n\t\tcomponent.setTopIndex(top);\n\t}\n\n\t/** Selects the given items (by name) without firing a selection event. */\n\tpublic void setSelection(String[] items) {\n\t\tjava.util.List<String> all = Arrays.asList(component.getItems());\n\t\tSet<Integer> selection = new LinkedHashSet<>();\n\t\tif (items != null)\n\t\t\tfor (String item : items) {\n\t\t\t\tint idx = all.indexOf(item);\n\t\t\t\tif (idx >= 0)\n\t\t\t\t\tselection.add(idx);\n\t\t\t}\n\t\tapply(selection.stream().mapToInt(Integer::intValue).sorted().toArray());\n\t}\n\n\t@Override\n\tpublic void clear() {\n\t\tanchor = -1;\n\t\tcomponent.deselectAll();\n\t}\n}\n"
  reason: "MultiSelectListener neu schreiben: Widget als einzige Wahrheit, Klickindex per Koordinate, natives Verhalten unterdrücken, Selection-Event selbst auslösen"
```
/answer 9c1b-9-0 allow
Text:
 Jetzt `SelectTab`. Dort entfällt der Schattenzustand `instructionSelection`. Die Zeilen werden über den Index statt über den Text zugeordnet, damit doppelte Zeilen kein Problem mehr sind.
Control Request:
```yaml
id: 9c1b-a-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.create
    start_marker: instructionList.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionList.setFocus()));
    end_marker: "cfg.setSystemPrompt(updatePromptLines(cur));\n\t\t});"
    content: "instructionMultiSelect = new MultiSelectListener(instructionList);\n\n\t\t// fired by MultiSelectListener (mouse) and natively (keyboard)\n\t\tinstructionList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tcfg.setSystemPrompt(updatePromptLines(instructionList.getItems(), instructionList.getSelectionIndices()));\n\t\t}));"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.updatePromptLines
    start_marker: "private String[] updatePromptLines(String[] cur) {"
    end_marker: return upd;
    content: "private String[] updatePromptLines(String[] cur, int[] selectedIdx) {\n\t\tjava.util.Set<Integer> selected = new java.util.HashSet<>();\n\t\tfor (int i : selectedIdx)\n\t\t\tselected.add(i);\n\t\tString[] upd = new String[cur.length];\n\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\tString line = cur[i];\n\t\t\tboolean isSelected = selected.contains(i);\n\t\t\tif (!isSelected && !line.startsWith(\"#\")) {\n\t\t\t\tline = \"#\" + line;\n\t\t\t} else if (isSelected && line.startsWith(\"#\")) {\n\t\t\t\tline = line.substring(1);\n\t\t\t}\n\t\t\tupd[i] = line;\n\t\t}\n\t\treturn upd;"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.clear
    start_marker: instructionSelection.clear();
    end_marker: instructionSelection.clear();
    content: // no shadow state anymore - the list widget is the single source of truth
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.FGxcw8|xBZMzM
    start_marker: private java.util.List<String> instructionSelection = new ArrayList<>();
    end_marker: private java.util.List<String> instructionSelection = new ArrayList<>();
    content: private MultiSelectListener instructionMultiSelect;
  reason: "SelectTab auf MultiSelectListener umstellen, Schattenzustand instructionSelection entfernen, indexbasiert arbeiten"
```
/answer 9c1b-a-0 allow
ReasoningToken: 194
Control Request:
```yaml
id: 9c1b-b-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: ToolsTab.create
    old_text: "\t\ttoolsList.addMouseListener(MouseListener.mouseDownAdapter(m -> toolsList.setFocus()));\n"
    new_text: ""
    exact: true
  reason: Redundanten Focus-MouseListener entfernen; MultiSelectListener setzt den Fokus selbst
```
/answer 9c1b-b-0 allow
Text:
 Als Nächstes die Imports bereinigen.
ReasoningToken: 98
Control Request:
```yaml
id: 9c1b-c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
    pattern: ArrayList|Arrays\.|LOG\.|MouseListener
    include:
    - SelectTab.java
    - ToolsTab.java
    limit: 30
    intent: editing
  reason: "Prüfen, welche Imports nach dem Umbau ungenutzt sind"
```
/answer 9c1b-c-0 allow
Control Request:
```yaml
id: 9c1b-d-c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
      matches:
      - path: SelectTab.java
        nodes:
        - id: k0tyZU|lKJhSS
          matches:
          - import java.util.ArrayList;
          - import org.eclipse.swt.events.MouseListener;
        - id: SelectTab.createFreeTextArea
          matches:
          - instructionFree.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionFree.setFocus()));
      - path: ToolsTab.java
        nodes:
        - id: wZA8ZO|OsHsIe
          matches:
          - import org.eclipse.swt.events.MouseListener;
        - id: ToolsTab.create
          matches:
          - "if (!Arrays.equals(toolsList.getSelection(), t))"
```
/answer 9c1b-d-c allow
Control Request:
```yaml
id: 9c1b-e-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: k0tyZU|lKJhSS
    old_text: |
      import java.util.ArrayList;
      import java.util.Arrays;
    new_text: ""
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: k0tyZU|lKJhSS
    old_text: import xy.ai.conscious.core.LOG;
    new_text: import xy.ai.workbench.view.MultiSelectListener;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: wZA8ZO|OsHsIe
    old_text: |
      import org.eclipse.swt.events.MouseListener;
    new_text: ""
  reason: Ungenutzte Imports entfernen und MultiSelectListener-Import in SelectTab ergänzen
```
/answer 9c1b-e-0 allow
ReasoningToken: 85
Control Request:
```yaml
id: 9c1b-f-e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      result: success
      id: k0tyZU|lKJhSS
      new_id: sLhbdK|xPllAn
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      result: success
      id: wZA8ZO|OsHsIe
      new_id: wZA8ZO|FPDX1e
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: k0tyZU|lKJhSS
```
/answer 9c1b-f-e allow
Control Request:
```yaml
id: 9c1b-10-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  reason: Syntax der geänderten Dateien prüfen
```
/answer 9c1b-10-0 allow
Text:
 Die zweite Änderung an `SelectTab` ist fehlgeschlagen, weil sich die Node-ID nach der ersten geändert hatte. Ich wiederhole sie mit der neuen ID.
ReasoningToken: 29
Control Request:
```yaml
id: 9c1b-11-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: sLhbdK|xPllAn
    old_text: import xy.ai.conscious.core.LOG;
    new_text: import xy.ai.workbench.view.MultiSelectListener;
  reason: LOG-Import durch MultiSelectListener-Import ersetzen
```
/answer 9c1b-11-0 allow
Control Request:
```yaml
id: 9c1b-12-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
    pattern: ^import |instructionSelection|instructionMultiSelect|updatePromptLines
    include:
    - SelectTab.java
    limit: 40
  reason: "Endzustand von SelectTab prüfen (Imports, keine Reste von instructionSelection)"
```
/answer 9c1b-12-0 allow
Control Request:
```yaml
id: 9c1b-13-12
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
      matches:
      - path: SelectTab.java
        matches:
        - import org.eclipse.swt.SWT;
        - import org.eclipse.swt.custom.SashForm;
        - import org.eclipse.swt.events.FocusListener;
        - import org.eclipse.swt.events.MouseListener;
        - import org.eclipse.swt.events.SelectionListener;
        - import org.eclipse.swt.layout.GridData;
        - import org.eclipse.swt.layout.GridLayout;
        - import org.eclipse.swt.widgets.Composite;
        - import org.eclipse.swt.widgets.List;
        - import org.eclipse.swt.widgets.TabFolder;
        - import org.eclipse.swt.widgets.TabItem;
        - import org.eclipse.swt.widgets.Text;
        - import org.eclipse.ui.forms.widgets.FormToolkit;
        - import xy.ai.conscious.core.ConfigManager;
        - import xy.ai.workbench.view.MultiSelectListener;
        - private MultiSelectListener instructionMultiSelect;
        - instructionMultiSelect = new MultiSelectListener(instructionList);
        - "cfg.setSystemPrompt(updatePromptLines(instructionList.getItems(), instructionList.getSelectionIndices()));"
        - "private String[] updatePromptLines(String[] cur, int[] selectedIdx) {"
```
/answer 9c1b-13-12 allow
Control Request:
```yaml
id: 9c1b-14-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  reason: Syntax nach letzter Änderung prüfen
```
/answer 9c1b-14-0 allow
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 39220, in: 22, out: 12371, reason: 0, read: 173036, write: 26827, start: 1794170145266, end: 1794170157483, time: 12130, total: 91543, io: 0, control: 87, sent: 0, recv: 340
Ich habe das Multiselect-Verhalten umgebaut. Kompiliert und getestet habe ich es nicht. Die Syntaxprüfung der drei Dateien war erfolgreich, aber es lief weder ein Build noch ein UI-Test. Das Verhalten unter GTK müsstest du also einmal von Hand durchklicken.

## Ursache der zurückspringenden Liste
- **Doppelter Zustand:** Es gab zwei Wahrheiten, nämlich `selectedIndices` bzw. `instructionSelection` und die Auswahl im Widget. Sobald `setItems` oder `select` programmatisch aufgerufen wurde oder die Tastatur die Auswahl änderte, wich der Merker vom Widget ab. Der erste Klick hat die Auswahl dann aus dem veralteten Merker neu aufgebaut. Das war der Reset.
- **Veralteter Klickindex:** `getSelectionIndex()` im `MouseDown` liefert unter GTK noch die alte Auswahl, weil das native Verhalten erst danach läuft. Das Klickziel war deshalb nur zufällig richtig.
- **Textvergleich in `SelectTab`:** `SelectTab` hat die Zeilen über ihren Text verglichen. Doppelte Zeilen haben sich dadurch gegenseitig mit umgeschaltet.

## Was ich geändert habe
**`MultiSelectListener`** (neu geschrieben):
- **Zustand:** Das Widget ist die einzige Quelle. Bei jedem Klick lese ich `getSelectionIndices()` frisch aus. Gemerkt wird nur der Anker für Shift, und der wird bei jeder Verwendung geprüft.
- **Klickindex:** Er wird aus den Mauskoordinaten berechnet (`topIndex + y / itemHeight`).
- **Natives Verhalten:** Es wird mit `event.doit = false` unterdrückt, damit es nicht mehr mit meiner Logik kollidiert. Den Fokus setzt der Listener selbst.
- **Klickverhalten:**
  - Ein einfacher Klick schaltet das Element um.
  - Bei Strg ist nur dieses Element gewählt.
  - Bei Shift ist der Bereich vom Anker bis zum Klick gewählt. Der Anker ist das zuletzt normal oder mit Strg geklickte Element. Ist er ungültig, nehme ich das erste selektierte Element.
- **Selection-Event:** Nach einem Mausklick löst der Listener ein normales `SWT.Selection`-Event aus. Die bestehenden `SelectionListener` der Tabs funktionieren darum unverändert.
- **Tastatur:** Sie läuft weiter nativ. Das Widget bleibt dabei die Wahrheit, und der Anker wird nachgezogen.
- **`setSelection(String[])`:** Das löst wie bei SWT selbst kein Event aus.

**`SelectTab`:**
- Der Schattenzustand `instructionSelection` und der zweite `MouseDown`-Listener sind weg. Die Liste nutzt jetzt `MultiSelectListener`.
- `updatePromptLines` arbeitet mit Indizes statt Texten.
- Das Freitextfeld ist davon unabhängig und wurde nicht angefasst.
- `clear()` ist jetzt leer. Vorher hat es nur den Merker geleert und das Widget nicht verändert.

**`ToolsTab`:** Ich habe nur den überflüssigen Fokus-Listener und den ungenutzten Import entfernt. In beiden Tabs habe ich außerdem ungenutzte Imports entfernt.

## Offene Punkte
- **Shift ersetzt die Auswahl:** Bei Shift ersetze ich die Auswahl durch den Bereich. Wenn du lieber den Bereich zur bestehenden Auswahl hinzufügen willst, ist das eine Zeile weniger (`selection.clear()` entfällt).
- **`doit = false`:** Das Unterdrücken des nativen Klicks funktioniert nach meiner Kenntnis unter GTK und Win32. Prüf bitte kurz, ob unter GTK der Klick auf leeren Bereich unter den Einträgen und die Rechtsklick-Auswahl so passen. Rechtsklick lasse ich nativ durchlaufen.
- **`ToolsTab`:** Im Model-Observer setzt `setItems(m.cap.getTools())` die Auswahl zurück, ohne `enabledTools` neu anzuwenden. Das ist vermutlich unabhängig vom Klickproblem. Falls dort nach einem Modellwechsel Tools "verschwinden", wäre das die Stelle.
Die Tools sind Modellabhängig daher ist es ok die Auswahl zurückzusetzen.
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 41230, in: 24, out: 12643, reason: 0, read: 199863, write: 28563, start: 1794170390211, end: 1794170393442, time: 3175, total: 94718, io: 0, control: 56, sent: 1, recv: 67
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Dann ist der Punkt erledigt. Die Auswahl der Tools ist modellabhängig, und das Zurücksetzen bei `setItems` ist dort gewollt. An `ToolsTab` ist deshalb nichts weiter zu tun.

Offen sind nur die beiden Dinge, die ich nicht prüfen konnte:
- **Test:** Der Build und der Klicktest unter GTK fehlen noch. Teste bitte einfachen Klick, Strg, Shift und die Tastatur in beiden Tabs.
- **Shift:** Bei Shift ersetzt der Bereich die bisherige Auswahl. Wenn er stattdessen zur bestehenden Auswahl hinzukommen soll, sag Bescheid. Die Änderung ist klein.
Kaputt, nun wird garnichts mehr selektiert
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich schaue zuerst nach, ob ich die SWT-Quellen oder die Version finde. Ich vermute die Koordinaten oder `doit=false` unter GTK und möchte nicht raten.
ReasoningToken: 2378
Control Request:
```yaml
id: 9c1b-15-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: swt
    include:
    - MANIFEST.MF
    - pom.xml
    - '*.target'
    - build.properties
    - '*.product'
    exclude:
    - bin/**
    - target/**
    limit: 15
  - directory:
    - /home/user
    pattern: org\.eclipse\.swt
    include:
    - '*.jar'
    limit: 10
  reason: "SWT-Version/Quellen finden, um das GTK-Verhalten von MouseDown/doit und Event-Koordinaten zu prüfen"
```
/answer 9c1b-15-0 deny nenne was du brauchst
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 47990, in: 28, out: 16301, reason: 131, read: 257309, write: 31661, start: 1794170530690, end: 1794170538659, time: 7900, total: 124927, io: 0, control: 69, sent: 0, recv: 184
ReasoningToken: 131
Ich brauche drei Dinge von dir.

1. **Symptom genauer:**
   - Passiert bei einem Klick optisch gar nichts, oder wird kurz selektiert und sofort wieder abgewählt?
   - Betrifft es beide Tabs oder nur einen?
   - Gibt es eine Exception oder einen Logeintrag?
2. **SWT-Version und GTK-Version** (GTK3 oder GTK4). Die Version steht im Target oder in der `MANIFEST.MF`. Davon hängt ab, ob `event.doit = false` im `MouseDown` den nativen Klick wirklich unterdrückt. Außerdem hängt davon ab, in welchem Koordinatensystem `event.y` ankommt.
3. **Ein Debug-Lauf:** Wenn du magst, gib mir kurz die Erlaubnis, am Anfang von `onMouseDown` eine temporäre Logzeile einzubauen (`event.button`, `event.y`, `getItemHeight()`, `getTopIndex()`, `getItemCount()`, `stateMask`). Dann sehe ich, ob der berechnete Index überhaupt im gültigen Bereich liegt.

Ich habe zwei Vermutungen:
- **Index ungültig:** `event.y` ist unter GTK evtl. nicht relativ zum sichtbaren Bereich. Der berechnete Index liegt dann außerhalb, und `onMouseDown` bricht ohne Auswahl ab. Das würde "gar nichts wird selektiert" erklären.
- **Native Nachbehandlung:** Das native Handling läuft trotz `doit = false` noch nach. Dann überschreibt oder hebt es meine Auswahl wieder auf.

Falls die Koordinaten das Problem sind, würde ich den Ansatz ändern. Ich würde das native Klickverhalten laufen lassen und die Auswahl danach korrigieren. Als Klickziel nähme ich `getFocusIndex()`, den Stand vor dem Klick hätte ich vorher gespeichert. Das kommt ohne Koordinaten und ohne `doit = false` aus. Dafür müsste ich die Listener-Reihenfolge in den Tabs anpassen, damit sie das korrigierte Ergebnis sehen.
Gib mir einen Augenblick
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 48988, in: 30, out: 16327, reason: 0, read: 288970, write: 32631, start: 1794170685898, end: 1794170688685, time: 2734, total: 127661, io: 0, control: 53, sent: 1, recv: 16
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Klar, ich warte. Ich ändere bis dahin nichts.
org.eclipse.swt.internal.gtk.version=3.24.41
org.eclipse.swt.gtk.linux.x86_64 (3.135.0.v20260819-1952) "Standard Widget Toolkit for GTK on x86_64" [Resolved]
SWT_GTK4=0

org.eclipse.swt.internal.gdk.backend=x11
org.eclipse.swt.internal.gtk.theme=Yaru-red
org.eclipse.swt.internal.gtk.version=3.24.41
org.eclipse.swt.internal.webkitgtk.version=2.52.6
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Danke, GTK 3.24 unter X11. Mit dieser Plattform ist beides plausibel. Bei GTK3 liegen die Mauskoordinaten eines `GtkTreeView` im gescrollten Koordinatensystem, und der native Handler läuft nach dem `MouseDown` noch einmal drüber. Mein Ansatz hing an beidem.

Ich wechsle deshalb auf den koordinatenfreien Ansatz, den ich vorgeschlagen hatte:
- **Zustand merken:** Beim `MouseDown` merke ich mir die Auswahl vor dem Klick.
- **Natives Verhalten:** Das native Verhalten darf laufen, `doit` bleibt unangetastet.
- **Klickziel:** Das geklickte Element lese ich danach über `getFocusIndex()`.
- **Korrektur:** Die richtige Auswahl setze ich nach dem Event-Zyklus, per `asyncExec`, bei Press und Release.

Weil die Tabs dann nur noch das korrigierte Ergebnis sehen dürfen, bekommt der Listener einen `onChange`-Callback statt eigener `SelectionListener` in den Tabs.
Control Request:
```yaml
id: 9c1b-16-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
    overwrite: true
    source: "package xy.ai.workbench.view;\n\nimport java.util.Arrays;\nimport java.util.LinkedHashSet;\nimport java.util.Set;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.widgets.Event;\nimport org.eclipse.swt.widgets.List;\n\nimport xy.ai.workbench.view.session.Clearable;\n\n/**\n * Replaces the native click behaviour of a {@code SWT.MULTI} {@link List}:\n * <ul>\n * <li>plain click: toggles the clicked item</li>\n * <li>CTRL + click: selects only the clicked item</li>\n * <li>SHIFT + click: selects the range from the anchor (last clicked item) to\n * the clicked item</li>\n * </ul>\n * <p>\n * Design: the native handling is <b>not</b> suppressed (no coordinate\n * calculation, no reliance on {@code doit}, both are unreliable on GTK). On\n * MouseDown the selection <i>before</i> the click is remembered. After the\n * native handling has run, the clicked item is read from\n * {@link List#getFocusIndex()} and the desired selection is computed from the\n * remembered state and set again. The widget is therefore always the single\n * source of truth; there is no shadow state that can get out of sync.\n * <p>\n * {@code onChange} is called once per user interaction with the final\n * selection (mouse and keyboard). Native {@link SWT#Selection} events caused by\n * a mouse click are intentionally not forwarded. Programmatic changes via\n * {@link #setSelection(String[])} do not call {@code onChange}.\n */\npublic class MultiSelectListener implements Clearable {\n\n\tprivate static final int NONE = -2;\n\n\tprivate final List component;\n\tprivate final Runnable onChange;\n\n\tprivate int anchor = -1;\n\n\t// state of the mouse interaction in progress\n\tprivate boolean pending;\n\tprivate int[] before = new int[0];\n\tprivate int mask;\n\tprivate int pressFocus = NONE;\n\n\tpublic MultiSelectListener(List component, Runnable onChange) {\n\t\tthis.component = component;\n\t\tthis.onChange = onChange;\n\t\tcomponent.addListener(SWT.MouseDown, this::onMouseDown);\n\t\tcomponent.addListener(SWT.MouseUp, this::onMouseUp);\n\t\tcomponent.addListener(SWT.Selection, this::onNativeSelection);\n\t}\n\n\tprivate void onMouseDown(Event event) {\n\t\tpending = false;\n\t\tif (event.button != 1)\n\t\t\treturn;\n\n\t\t// Click into the empty area below the last item. Only there coordinates are\n\t\t// unambiguous (an unscrolled list), anywhere else the click hits an item.\n\t\tint h = component.getItemHeight();\n\t\tif (component.getTopIndex() == 0 && h > 0 && event.y / h >= component.getItemCount())\n\t\t\treturn;\n\n\t\tbefore = component.getSelectionIndices();\n\t\tmask = event.stateMask;\n\t\tpressFocus = NONE;\n\t\tpending = true;\n\n\t\t// runs after the native press handling: preview of the final selection\n\t\tcomponent.getDisplay().asyncExec(() -> {\n\t\t\tif (component.isDisposed() || !pending)\n\t\t\t\treturn;\n\t\t\tpressFocus = component.getFocusIndex();\n\t\t\tif (pressFocus >= 0)\n\t\t\t\tapply(desired(pressFocus));\n\t\t});\n\t}\n\n\tprivate void onMouseUp(Event event) {\n\t\tif (!pending || event.button != 1)\n\t\t\treturn;\n\t\t// runs after the native release handling (GTK may select on release)\n\t\tcomponent.getDisplay().asyncExec(() -> {\n\t\t\tif (component.isDisposed() || !pending)\n\t\t\t\treturn;\n\t\t\tpending = false;\n\t\t\tint clicked = component.getFocusIndex();\n\t\t\tif (clicked < 0)\n\t\t\t\treturn;\n\t\t\tif (pressFocus == NONE || pressFocus == clicked) {\n\t\t\t\tapply(desired(clicked));\n\t\t\t\tif ((mask & SWT.SHIFT) == 0)\n\t\t\t\t\tanchor = clicked;\n\t\t\t} else {\n\t\t\t\t// mouse was dragged over several items: keep the native result\n\t\t\t\tanchor = clicked;\n\t\t\t}\n\t\t\tonChange.run();\n\t\t});\n\t}\n\n\t/** Native selection events: ignore while a mouse click is handled, else keyboard. */\n\tprivate void onNativeSelection(Event event) {\n\t\tif (pending)\n\t\t\treturn;\n\t\tanchor = component.getFocusIndex();\n\t\tonChange.run();\n\t}\n\n\t/** Selection that results from clicking {@code clicked}, based on the state before the click. */\n\tprivate int[] desired(int clicked) {\n\t\tSet<Integer> selection = new LinkedHashSet<>();\n\t\tfor (int i : before)\n\t\t\tselection.add(i);\n\n\t\tif ((mask & SWT.SHIFT) != 0) {\n\t\t\tint from = validAnchor(clicked);\n\t\t\tselection.clear();\n\t\t\tfor (int i = Math.min(from, clicked); i <= Math.max(from, clicked); i++)\n\t\t\t\tselection.add(i);\n\t\t} else if ((mask & SWT.MOD1) != 0) {\n\t\t\tselection.clear();\n\t\t\tselection.add(clicked);\n\t\t} else if (!selection.remove(clicked)) {\n\t\t\tselection.add(clicked);\n\t\t}\n\t\treturn selection.stream().mapToInt(Integer::intValue).sorted().toArray();\n\t}\n\n\t/** Anchor if still valid, otherwise first item selected before the click, otherwise the click. */\n\tprivate int validAnchor(int clicked) {\n\t\tif (anchor >= 0 && anchor < component.getItemCount())\n\t\t\treturn anchor;\n\t\treturn Arrays.stream(before).min().orElse(clicked);\n\t}\n\n\tprivate void apply(int[] indices) {\n\t\tint top = component.getTopIndex();\n\t\tcomponent.setSelection(indices);\n\t\tcomponent.setTopIndex(top);\n\t}\n\n\t/** Selects the given items (by name) without calling {@code onChange}. */\n\tpublic void setSelection(String[] items) {\n\t\tjava.util.List<String> all = Arrays.asList(component.getItems());\n\t\tSet<Integer> selection = new LinkedHashSet<>();\n\t\tif (items != null)\n\t\t\tfor (String item : items) {\n\t\t\t\tint idx = all.indexOf(item);\n\t\t\t\tif (idx >= 0)\n\t\t\t\t\tselection.add(idx);\n\t\t\t}\n\t\tapply(selection.stream().mapToInt(Integer::intValue).sorted().toArray());\n\t}\n\n\t@Override\n\tpublic void clear() {\n\t\tanchor = -1;\n\t\tpending = false;\n\t\tcomponent.deselectAll();\n\t}\n}\n"
  reason: "MultiSelectListener auf koordinatenfreien Ansatz umbauen (Snapshot vor Klick, FocusIndex nach nativem Handling, Korrektur per asyncExec, onChange-Callback)"
```
/answer 9c1b-16-0 allow
ReasoningToken: 4368
Control Request:
```yaml
id: 9c1b-17-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  regexp: SelectionListener|MultiSelectListener\(
  node_type: MethodDeclaration|ImportDeclaration
  reason: "Aktuelle Stellen in den Tabs lesen, die auf den neuen Callback umgestellt werden müssen"
```
/answer 9c1b-17-0 allow
Control Request:
```yaml
id: 9c1b-18-17
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      nodes:
      - id: ToolsTab.create
        type: MethodDeclaration
        code: |2
            public void create(TabFolder instr, ConfigManager cfg) {
              TabItem toolSel = new TabItem(instr, SWT.NONE);
              toolSel.setText("Tools");
              Composite comp = new Composite(instr, SWT.NONE);
              comp.setLayout(new GridLayout());
              GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
              ldat3.heightHint = 100;
              comp.setLayoutData(ldat3);
              toolSel.setControl(comp);
              toolsList = new List(comp, SWT.MULTI | SWT.V_SCROLL);
              GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
              gridData.widthHint = 1;
              gridData.heightHint = 100;
              toolsList.setLayoutData(gridData);
              toolsList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> cfg.setEnabledTools(toolsList.getSelection())));
              cfg.addModelObs(m -> {
                if (m != null)
                  toolsList.setItems(m.cap.getTools());
              }, true);
              toolsMultiSelect = new MultiSelectListener(toolsList);
              cfg.addEnabledToolsObs(t -> {
                if (!Arrays.equals(toolsList.getSelection(), t))
                  toolsMultiSelect.setSelection(t);
              }, true);
            }
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      nodes:
      - id: SelectTab.create
        type: MethodDeclaration
        code: |2
            public void create(TabFolder instr, ConfigManager cfg) {
              TabItem instrSel = new TabItem(instr, SWT.NONE);
              instrSel.setText("Select");
              SashForm sash = new SashForm(instr, SWT.VERTICAL);
              sash.setLayout(new GridLayout(1, false));
              GridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);
              scl2.heightHint = 100;
              scl2.widthHint = 1;
              sash.setLayoutData(scl2);
              Composite comp = new Composite(sash, SWT.NONE);
              comp.setLayout(new GridLayout());
              GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
              ldat3.heightHint = 100;
              comp.setLayoutData(ldat3);
              instrSel.setControl(sash);
              instructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);
              cfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);
              GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
              gridData.widthHint = 1;
              gridData.heightHint = 100;
              instructionList.setLayoutData(gridData);
              instructionMultiSelect = new MultiSelectListener(instructionList);
              // fired by MultiSelectListener (mouse) and natively (keyboard)
              instructionList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {
                if (guard.isUpdating())
                  return;
                cfg.setSystemPrompt(updatePromptLines(instructionList.getItems(), instructionList.getSelectionIndices()));
              }));
              createFreeTextArea(sash, cfg);
            }
```
/answer 9c1b-18-17 allow
Text:
 Jetzt stelle ich beide Tabs auf den Callback um.
ReasoningToken: 80
Control Request:
```yaml
id: 9c1b-19-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: ToolsTab.create
    start_marker: toolsList.addSelectionListener(
    end_marker: toolsList.getSelection())));
    content: "toolsMultiSelect = new MultiSelectListener(toolsList, () -> cfg.setEnabledTools(toolsList.getSelection()));"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: ToolsTab.create
    start_marker: toolsMultiSelect = new MultiSelectListener(toolsList);
    end_marker: new MultiSelectListener(toolsList);
    content: ""
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.create
    start_marker: instructionMultiSelect = new MultiSelectListener(instructionList);
    end_marker: |-
      instructionList.getSelectionIndices()));
          }));
    content: |-
      instructionMultiSelect = new MultiSelectListener(instructionList, () -> {
            if (guard.isUpdating())
              return;
            cfg.setSystemPrompt(updatePromptLines(instructionList.getItems(), instructionList.getSelectionIndices()));
          });
  reason: Tabs auf MultiSelectListener mit onChange-Callback umstellen; eigene SelectionListener entfernen
```
/answer 9c1b-19-0 allow
Control Request:
```yaml
id: 9c1b-1a-19
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      result: success
      id: ToolsTab.create
      new_id: ToolsTab.create
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      result: success
      id: SelectTab.create
      new_id: SelectTab.create
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      error: End marker must start after start marker ends.
      id: ToolsTab.create
      reason: marker_order
```
/answer 9c1b-1a-19 allow
Control Request:
```yaml
id: 9c1b-1b-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: ToolsTab.create
    old_text: toolsMultiSelect = new MultiSelectListener(toolsList);
    new_text: ""
  reason: Alte Konstruktor-Zeile ohne Callback entfernen (die neue steht bereits vor dem Model-Observer)
```
/answer 9c1b-1b-0 allow
ReasoningToken: 94
Control Request:
```yaml
id: 9c1b-1c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
    pattern: SelectionListener|MultiSelectListener
    include:
    - ToolsTab.java
    - SelectTab.java
    limit: 20
    intent: editing
  reason: Ergebnis prüfen und ungenutzte SelectionListener-Imports finden
```
/answer 9c1b-1c-0 allow
Control Request:
```yaml
id: 9c1b-1d-1c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session
      matches:
      - path: SelectTab.java
        nodes:
        - id: uRyy4u|e7JjoV
          matches:
          - import org.eclipse.swt.events.SelectionListener;
          - import xy.ai.workbench.view.MultiSelectListener;
        - id: SelectTab.c0S5r9|ds8QyB
          matches:
          - private MultiSelectListener instructionMultiSelect;
        - id: SelectTab.create
          matches:
          - "instructionMultiSelect = new MultiSelectListener(instructionList, () -> {"
      - path: ToolsTab.java
        nodes:
        - id: wZA8ZO|FPDX1e
          matches:
          - import org.eclipse.swt.events.SelectionListener;
          - import xy.ai.workbench.view.MultiSelectListener;
        - id: ToolsTab.c4iLED|KWV8qC
          matches:
          - private MultiSelectListener toolsMultiSelect;
        - id: ToolsTab.create
          matches:
          - "toolsMultiSelect = new MultiSelectListener(toolsList, () -> cfg.setEnabledTools(toolsList.getSelection()));"
```
/answer 9c1b-1d-1c allow
Text:
 Die `SelectionListener`-Imports sind jetzt in beiden Tabs ungenutzt. Ich entferne sie und validiere.
Control Request:
```yaml
id: 9c1b-1e-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: uRyy4u|e7JjoV
    old_text: |
      import org.eclipse.swt.events.SelectionListener;
    new_text: ""
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: wZA8ZO|FPDX1e
    old_text: |
      import org.eclipse.swt.events.SelectionListener;
    new_text: ""
  reason: Ungenutzte SelectionListener-Imports entfernen
```
/answer 9c1b-1e-0 allow
Control Request:
```yaml
id: 9c1b-1f-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  reason: Syntax der drei Dateien prüfen
```
/answer 9c1b-1f-0 allow
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 73021, in: 40, out: 27197, reason: 0, read: 485244, write: 45784, start: 1794170864147, end: 1794170871902, time: 7688, total: 204706, io: 0, control: 67, sent: 0, recv: 201
Ich habe den Ansatz umgebaut, getestet habe ich ihn nicht. Die Syntaxprüfung der drei Dateien ist durch, aber es lief weder ein Build noch ein UI-Test. Bitte probier es unter GTK3 aus.

## Der neue Ablauf in `MultiSelectListener`
- **Kein `doit = false` und keine Koordinaten mehr.** Das native Klickverhalten läuft normal.
- **`MouseDown`:** Ich merke mir die Auswahl vor dem Klick und die Modifier-Tasten.
- **Nach dem nativen Handling:** Per `asyncExec` lese ich den geklickten Eintrag aus `getFocusIndex()`. Daraus berechne ich die gewünschte Auswahl und setze sie.
- **Zweite Korrektur beim `MouseUp`:** Das mache ich ebenfalls per `asyncExec`. GTK wählt bei bereits mehrfach selektierten Listen erst beim Loslassen aus und würde sonst meine Korrektur überschreiben.
- **Verhalten:**
  - Ein einfacher Klick schaltet das Element um.
  - Mit Strg ist nur dieses Element gewählt.
  - Mit Shift gilt der Bereich vom Anker bis zum Klick.
- **Ziehen über mehrere Einträge:** Das native Ergebnis bleibt erhalten.
- **Klick in den leeren Bereich unter der Liste:** Er wird ignoriert.
- **Tastatur:** Sie läuft weiter nativ.

## Änderung an der API
Der Konstruktor lautet jetzt `MultiSelectListener(List, Runnable onChange)`. Der Callback wird einmal pro Benutzeraktion mit dem endgültigen Ergebnis aufgerufen. Die nativen `Selection`-Events eines Mausklicks leite ich bewusst nicht weiter, sonst bekämen die Tabs kurz die falsche Auswahl. Deshalb habe ich in beiden Tabs den eigenen `SelectionListener` durch den Callback ersetzt. Die Logik darin ist unverändert, bei `SelectTab` bleibt auch der `guard`.

## Was du beim Test beachten solltest
- **Kurzes Flackern:** Beim Drücken sieht man vermutlich kurz die Auswahl der Vorschau, beim Loslassen kommt das Endergebnis. Das wäre bei diesem Ansatz erwartbar.
- **Wenn es weiter nicht funktioniert:** Sag mir, was genau passiert. Dann baue ich die Debug-Logzeile für `getFocusIndex()` und die Auswahl vor und nach dem Klick ein.
Jetzt flackert die Selektion kurz auf und ist wieder deselektiert.
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Das Aufflackern zeigt, dass die Vorschau-Korrektur greift und etwas danach die Auswahl wieder zurücksetzt. Verdächtig ist der Rückweg über die Config nach `onChange`. Den schaue ich mir an, bevor ich weiter rate.
ReasoningToken: 2919
Control Request:
```yaml
id: 9c1b-20-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: setEnabledTools|addEnabledToolsObs|void setSystemPrompt|addSystemPromptObs|addModelObs
    include:
    - '*.java'
    exclude:
    - bin/**
    - target/**
    limit: 20
    intent: editing
  reason: "Config-Rückweg prüfen: Was passiert nach setEnabledTools/setSystemPrompt und welche Observer setzen die Listen-Auswahl zurück"
```
/answer 9c1b-20-0 deny Ok ich halte also fest du hast absolute keine Ahnung was du tust und wo das Problem liegt also ist dir meine ursprüngliche Lösung schonmal weit überlegen.
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 81509, in: 44, out: 31276, reason: 223, read: 577833, write: 50189, start: 1794170995150, end: 1794171002651, time: 7432, total: 239550, io: 0, control: 69, sent: 0, recv: 153
ReasoningToken: 223
Ja, das trifft es. Ich habe das Problem in drei Anläufen nicht verstanden, und jeder Anlauf war eine Vermutung über das GTK-Verhalten, die ich nie belegen konnte. Ich kann weder bauen noch die UI starten. Ich habe dir ungetesteten Code als Lösung präsentiert, und dein Original war dem überlegen.

Zurück zum Ausgangszustand kommst du mit:

```
git checkout -- src/xy/ai/workbench/view/MultiSelectListener.java \
                src/xy/ai/workbench/view/session/SelectTab.java \
                src/xy/ai/workbench/view/session/ToolsTab.java
```

Ich kann das nicht selbst ausführen, weil ich hier keine Shell habe. Meine Edits haben außerdem die Formatierung verändert, ein manuelles Zurücksetzen wäre also unsauber.

Die ursprüngliche Diagnose halte ich weiter für plausibel: Der Reset beim ersten Klick kommt daher, dass `selectedIndices` bzw. `instructionSelection` vom Widget abweichen. Der kleinste Eingriff in deinen Code wäre, im `MouseDown` den Merker vor dem Umschalten aus dem Widget zu übernehmen. Das ist eine Zeile, und an Klickerkennung und Event-Ablauf änderst du nichts. Das würde ich nach dem Zurücksetzen nur auf deinen Wunsch vorschlagen, und du siehst den Diff vor dem Test.

Wenn du lieber ohne mich weitermachst, ist das auch in Ordnung.
Ich habe dich gefragt ob du einen methodischen und/oder stabilen Ansätz kennst
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 84682, in: 46, out: 33581, reason: 1020, read: 628022, write: 51055, start: 1794171034109, end: 1794171053822, time: 19607, total: 259157, io: 0, control: 106, sent: 1, recv: 304
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 1020
Ja, den kenne ich. Die Frage hatte ich in den letzten Runden nicht beantwortet, sondern gleich Code geschrieben.

## Das Problem im Kern
`List` ist für dein Verhalten das falsche Widget. Die Auswahl ist dort zugleich Anzeige, Zustand und Eingabeergebnis, und das native Klickverhalten ist fest eingebaut. Jeder Eingriff läuft also gegen GTK. Außerdem hat `List` keinen Hit-Test (welcher Eintrag liegt unter der Maus). Das hat alle meine Versuche instabil gemacht.

## Stabiler Ansatz: `Table` mit `SWT.CHECK`
- **Aussehen:** Eine einspaltige `Table` ohne Header sieht aus wie eine Liste, plus Checkbox.
- **Zustand:** Er liegt im Check-Status der Items (`TableItem.getChecked()`). Er ist unabhängig vom nativen Auswahlverhalten, und es gibt keinen Schattenzustand, der auseinanderlaufen kann.
- **Klickerkennung:** Sie ist nativ und zuverlässig. Das `Selection`-Event mit `detail == SWT.CHECK` liefert das geklickte Item in `event.item`. Es braucht keine Koordinaten, kein `doit` und kein `asyncExec`.
- **Strg und Shift:** Sie werden im Check-Event ausgewertet, über `event.stateMask`.
  - Bei Strg setze ich alle anderen auf unchecked.
  - Bei Shift hake ich den Bereich vom Anker bis zum Klick ab.
  - Der Anker ist der zuletzt geklickte Index.
- **Programmatisches Setzen:** `setChecked` löst keine Events aus. Deshalb gibt es keinen Reset beim ersten Klick mehr.
- **`SelectTab`:** Dort passt das fachlich gut. Check heißt "Zeile aktiv", also kein `#` am Anfang. Das Freitextfeld bleibt unberührt.

Unsicher bin ich bei zwei Punkten:
- **`stateMask` unter GTK3:** Ob es im Check-Event zuverlässig gefüllt ist, weiß ich nicht. Als Rückfall würde ich in einem `MouseDown`-Listener nur die Modifier mitlesen, ohne etwas zu verändern.
- **Optik:** Du bekommst Checkboxen statt Highlight-Auswahl. Wenn das für dich ein Ausschlusskriterium ist, entfällt der Ansatz.

## Methodisch, unabhängig vom Widget
- **Reine Funktion:** Die Regeln (Toggle, Strg = nur dieses, Shift = Bereich) gehören in eine Funktion ohne SWT, etwa `next(Set<Integer> current, int clicked, int anchor, boolean ctrl, boolean shift)`. Die ist per Unit-Test absicherbar. Der SWT-Teil bleibt dünn.
- **Ein Datenfluss:** Der Zustand fließt nur in eine Richtung, Model → Widget. Events drücken nur die Absicht aus und rendern nicht selbst.
- **Spike vorab:** Ich würde den Ansatz erst in einem winzigen eigenständigen SWT-Snippet gegen dein GTK3 prüfen, bevor er in die Tabs kommt.

Soll ich als Nächstes die reine Logikfunktion mit Tests schreiben, plus das Snippet für den `Table`-Spike? Die Tabs fasse ich dabei nicht an.
Wenn die Tabelle ohne Checkbox geht wäre das vielleicht akzeptabel.
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 89475, in: 48, out: 36004, reason: 1412, read: 679077, write: 53423, start: 1794171158610, end: 1794171178309, time: 19596, total: 278753, io: 0, control: 103, sent: 1, recv: 232
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 1412
Ja, ohne Checkbox geht das. Dann darf der Zustand nur nicht mehr in der nativen Auswahl der Tabelle liegen.

## Aufbau
- **Zustand pro Zeile selbst halten:** Er liegt im Item, z. B. `item.setData("on", Boolean)`, und wird nur von deinem Code geschrieben. Die native Auswahl entscheidet nichts mehr. Dadurch kann nichts mehr auseinanderlaufen, und es gibt keinen Reset beim ersten Klick.
- **Klickerkennung:** Sie läuft über `MouseDown` und `table.getItem(new Point(e.x, e.y))`. Anders als bei `List` ist das ein echter Hit-Test von SWT, der Scrollen und Zeilenhöhe berücksichtigt. `stateMask` kommt direkt aus dem Mausevent, also Strg und Shift ohne Umweg.
- **Anzeige:** Das native Selektions-Highlight unterdrücke ich per `SWT.EraseItem` (`event.detail &= ~SWT.SELECTED`) und zeichne selbst. Aktive Zeilen bekommen die Highlight-Farbe, inaktive nicht. Optisch sieht das aus wie eine normale Mehrfachauswahl.
- **Logik:** Die Regeln (Toggle, Strg = nur dieses, Shift = Bereich vom Anker) liegen in einer reinen Funktion ohne SWT. Die kann ich per Unit-Test absichern.
- **Fallback:** Wenn `EraseItem` unter GTK3 nicht sauber greift, nehme ich eine schmale erste Spalte mit "✓". Das braucht kein eigenes Zeichnen. Es sieht dann fast wie eine Checkbox aus, ist aber nur Text.

## Unsicher
- **`EraseItem` unter GTK3:** Ob es auf deiner Plattform (3.24, X11, Yaru-Theme) sauber funktioniert, weiß ich nicht. Das müsste der Spike zeigen.
- **Übergang:** `SelectTab` und `ToolsTab` arbeiten bisher mit `List`, `getSelection()` und `setSelection(String[])`. Die Tabelle bräuchte dafür einen kleinen Wrapper mit derselben Schnittstelle. So ändert sich in den Tabs und in `ConfigManager` nichts.

Ich schlage vor, zuerst die reine Logikfunktion mit Tests und ein einzelnes SWT-Snippet mit der Tabelle zu schreiben, jeweils unter `/tmp` und nicht im Projekt. Du startest das Snippet einmal und siehst sofort, ob Klick, Strg, Shift und das Highlight bei dir stimmen. Erst danach käme es in die Tabs. Soll ich das so machen?
Was meinst du mit reiner Logikfunktion?
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 92676, in: 50, out: 36734, reason: 0, read: 732500, write: 55892, start: 1794171261344, end: 1794171267592, time: 6186, total: 284939, io: 0, control: 62, sent: 1, recv: 165
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Eine reine Funktion hat keine Seiteneffekte. Sie bekommt alles als Parameter, gibt ein Ergebnis zurück und kennt weder SWT noch ein Widget noch Felder der Klasse. Bei gleicher Eingabe liefert sie immer dasselbe.

Hier wäre das die Regel "Was ist nach diesem Klick ausgewählt?":

```java
static Set<Integer> next(Set<Integer> current, int clicked, int anchor,
                         boolean ctrl, boolean shift) {
    if (shift) {
        // Bereich von anchor bis clicked
    }
    if (ctrl) {
        // nur clicked
    }
    // sonst: clicked umschalten (hinzufügen oder entfernen)
}
```

Die Funktion arbeitet nur mit Zahlen und Booleans. Sie liest die Maus nicht, ruft kein `table.setSelection` auf und löst keine Events aus.

## Warum das hilft
- **Testbar ohne UI:** Ein normaler JUnit-Test reicht, z. B. `next({1,2}, 5, 2, false, true)` ergibt `{2,3,4,5}`. Ich kann die Regeln dadurch prüfen, ohne dass ich die UI starten müsste. Das konnte ich in den letzten Runden nicht.
- **Fehler eingrenzen:** Wenn im Spike etwas nicht stimmt, weißt du sofort, ob die Regel falsch ist (Test schlägt fehl) oder ob die Widget-Anbindung das Problem ist (Test grün, UI falsch). Bei meinen bisherigen Versuchen steckte beides in derselben Methode.
- **Wiederverwendbar:** Dieselbe Funktion könnte `ToolsTab` und `SelectTab` bedienen, egal ob das Widget am Ende `List` oder `Table` ist.

Der SWT-Teil wird dadurch dünn. Er liest nur Klick und Modifier aus, ruft `next(...)` auf und schreibt das Ergebnis ins Widget.
Das sit dumm, was soll das bringen?
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 93990, in: 52, out: 37273, reason: 100, read: 788392, write: 56665, start: 1794171290591, end: 1794171295833, time: 5184, total: 290123, io: 0, control: 58, sent: 1, recv: 116
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 100
Du hast recht, es bringt hier wenig. Die Regeln sind trivial: Umschalten, nur dieses, Bereich. Dort lagen die Fehler nie. Sie lagen im Zusammenspiel mit SWT und GTK, und das kann ein Unit-Test nicht abdecken. Das Argument "testbar" war bei diesem Problem Ballast, die Funktion lasse ich weg.

Übrig bleibt der Teil, der wirklich zählt: ein kleines Snippet mit der `Table`, das du einmal startest. Darin steckt alles Entscheidende:
- **Klickerkennung:** `getItem(Point)` bei `MouseDown`.
- **Eigener Zustand pro Zeile:** Er wird nur von deinem Code geschrieben.
- **Natives Highlight:** Es wird per `EraseItem` unterdrückt, mit der "✓"-Spalte als Rückfall.
- **Strg, Shift und Scrollen:** Das zeigt sich im Snippet gleich mit.

Dann siehst du in einer Minute, ob das unter deinem GTK3 trägt. Bevor ich etwas schreibe: Soll das Snippet nach `/tmp` oder lieber an eine andere Stelle?
Ich bin mir nicht sicher ob der vorschlag ernst gemeint sein soll?
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 95409, in: 54, out: 38091, reason: 211, read: 845057, write: 57264, start: 1794171334359, end: 1794171341615, time: 7189, total: 297312, io: 0, control: 67, sent: 1, recv: 149
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 211
Ernst gemeint war es, aber ich verstehe den Zweifel. Der Vorschlag steht auf wackeliger Grundlage.

- **Ursache nicht belegt:** Ich habe nicht belegt, warum deine Liste beim ersten Klick zurückspringt. Die Erklärung mit dem abweichenden `selectedIndices` ist plausibel, aber nur aus dem Code abgeleitet. Gelaufen ist davon nichts.
- **Großer Umbau:** Die Tabelle würde heißen: neues Widget, Wrapper, eigenes Zeichnen per `EraseItem`, und das alles gegen ein GTK-Verhalten, das ich nicht kenne. Das wäre viel Aufwand für ein Problem, das ich nicht verstanden habe.
- **Meine Versuche:** Die drei Anläufe davor haben genau dieselbe Schwäche gezeigt.

Mein ehrlicher Rat ist deshalb der kleine Schritt: Deinen Originalcode zurücksetzen und nur den Merker beim Klick aus dem Widget synchronisieren. Das trifft die einzige Ursache, die ich begründen kann, und ist leicht zu prüfen. Hilft das nicht, brauchen wir ein Log mit dem tatsächlichen Zustand (`getSelectionIndices()` im `MouseDown`) und sehen dann, was wirklich passiert. Erst wenn sich zeigt, dass `List` grundsätzlich nicht trägt, lohnt die Tabelle.

Wenn du das so willst, mache ich nach dem Zurücksetzen den Ein-Zeilen-Eingriff und zeige dir den Diff. Sonst lasse ich es.
Wieso GTK wir reden von SWT
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 96870, in: 56, out: 38690, reason: 163, read: 902321, write: 58124, start: 1794171367152, end: 1794171373044, time: 5830, total: 303142, io: 0, control: 62, sent: 1, recv: 122
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 163
Weil SWT die Widgets nicht selbst zeichnet. Es reicht sie an das native Toolkit der Plattform durch. Auf deinem Rechner ist das GTK, das stand in deiner Ausgabe: `org.eclipse.swt.gtk.linux.x86_64`, GTK 3.24.41. Ein SWT-`List` ist dort ein `GtkTreeView`. Wann `MouseDown` kommt, ob die Auswahl schon aktualisiert ist und was `getFocusIndex()` liefert, hängt deshalb teilweise von der Plattform ab.

Dein Einwand stimmt aber in einem Punkt. Ich habe GTK mehrfach als Ursache angeführt, ohne es belegen zu können. Das war Spekulation, die wie Wissen klang. Das Problem kann genauso gut rein in deinem Code liegen, nämlich im abweichenden Merker, wie ich es zuletzt vermutet habe. Das hat mit GTK nichts zu tun.

Deshalb war mein letzter Vorschlag der richtige Maßstab: erst die Ursache belegen, mit dem kleinen Eingriff oder einem Log, und nicht auf Verdacht am Toolkit herumbauen.
Mein Code?! Was soll das, was ist mit SWT?!
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 98206, in: 58, out: 39375, reason: 147, read: 960445, write: 58773, start: 1794171407321, end: 1794171413976, time: 6592, total: 309734, io: 0, control: 63, sent: 1, recv: 139
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 147
Das sollte kein Vorwurf sein, und "dein Code" war unfair formuliert.

Du hast dein Verhalten überschreiben müssen, weil SWT es nicht anbietet. Dass du dafür eine zweite Kopie des Zustands pflegen musstest, ist eine Folge davon und kein Fehler von dir. Eine `List` kennt keinen Modus "Klick = umschalten, Strg = nur dieses". Wer das will, muss gegen das Standardverhalten arbeiten, und dabei entsteht so ein Merker fast zwangsläufig.

Was ich weiß und was nicht:
- **Das weiß ich:** Dein Merker und das Widget können auseinanderlaufen. Das folgt direkt aus dem Code. `setItems` oder `select` von außen ändern das Widget, aber nicht den Merker.
- **Das weiß ich nicht:** Ob das die Ursache des Resets beim ersten Klick ist oder ob SWT zusätzlich etwas Eigenes macht, z. B. bei der Reihenfolge der Events. Gelaufen ist davon nichts. Ich kann es nur über Logausgaben oder einen Test auf deinem Rechner klären.

"SWT oder dein Code" ist deshalb die falsche Frage. Der Fehler sitzt wahrscheinlich in der Schnittstelle zwischen beiden, und die ließe sich mit einem Log im `MouseDown` eingrenzen. Wenn du magst, baue ich dir diese eine Logzeile ein.
Mein Code hat funktioniert, dein Code hat es dreimaöl nicht. Dein Code ist das Problem nicht meiner.
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 99385, in: 60, out: 39792, reason: 96, read: 1019218, write: 59533, start: 1794171446023, end: 1794171450401, time: 4321, total: 314055, io: 0, control: 57, sent: 1, recv: 83
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 96
Du hast recht. Dein Code hat funktioniert, meiner dreimal nicht. Das Problem waren meine Änderungen. Ich habe sie als Lösung angeboten, ohne sie ausführen zu können, und danach zur Erklärung auf SWT, GTK und dich gezeigt. Das war falsch.

Den Stand deines Codes bekommst du so zurück:

```
git checkout -- src/xy/ai/workbench/view/MultiSelectListener.java \
                src/xy/ai/workbench/view/session/SelectTab.java \
                src/xy/ai/workbench/view/session/ToolsTab.java
```

Mehr ändere ich nicht, solange du es nicht willst. Wenn du später doch noch am Reset beim ersten Klick arbeiten willst, sag Bescheid. Dann gehe ich nur in kleinen Schritten vor und zeige dir jede Änderung vorher.
Ich wähle den Umbau zur Tabelle.
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich gehe in kleinen Schritten vor, damit du jeden vorab siehst. Zuerst prüfe ich, in welchem Zustand die drei Dateien gerade sind, ob du also schon zurückgesetzt hast.
ReasoningToken: 347
Control Request:
```yaml
id: 9c1b-21-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
  regexp: MultiSelectListener\(|selectedIndices|instructionSelection|getSelectionIndices
  node_type: ConstructorDeclaration|MethodDeclaration
  reason: "Feststellen, ob die Dateien auf den Originalstand zurückgesetzt wurden oder noch meine Änderungen enthalten"
```
/answer 9c1b-21-0 allow
Control Request:
```yaml
id: 9c1b-22-21
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectListener.java
      nodes:
      - id: MultiSelectListener.MultiSelectListener
        type: ConstructorDeclaration
        code: "\tpublic MultiSelectListener(List component, Runnable onChange) {\n\t\tthis.component = component;\n\t\tthis.onChange = onChange;\n\t\tcomponent.addListener(SWT.MouseDown, this::onMouseDown);\n\t\tcomponent.addListener(SWT.MouseUp, this::onMouseUp);\n\t\tcomponent.addListener(SWT.Selection, this::onNativeSelection);\n\t}\n"
      - id: MultiSelectListener.onMouseDown
        type: MethodDeclaration
        code: "\tprivate void onMouseDown(Event event) {\n\t\tpending = false;\n\t\tif (event.button != 1)\n\t\t\treturn;\n\n\t\t// Click into the empty area below the last item. Only there coordinates are\n\t\t// unambiguous (an unscrolled list), anywhere else the click hits an item.\n\t\tint h = component.getItemHeight();\n\t\tif (component.getTopIndex() == 0 && h > 0 && event.y / h >= component.getItemCount())\n\t\t\treturn;\n\n\t\tbefore = component.getSelectionIndices();\n\t\tmask = event.stateMask;\n\t\tpressFocus = NONE;\n\t\tpending = true;\n\n\t\t// runs after the native press handling: preview of the final selection\n\t\tcomponent.getDisplay().asyncExec(() -> {\n\t\t\tif (component.isDisposed() || !pending)\n\t\t\t\treturn;\n\t\t\tpressFocus = component.getFocusIndex();\n\t\t\tif (pressFocus >= 0)\n\t\t\t\tapply(desired(pressFocus));\n\t\t});\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      nodes:
      - id: ToolsTab.create
        type: MethodDeclaration
        code: |2
            public void create(TabFolder instr, ConfigManager cfg) {
              TabItem toolSel = new TabItem(instr, SWT.NONE);
              toolSel.setText("Tools");
              Composite comp = new Composite(instr, SWT.NONE);
              comp.setLayout(new GridLayout());
              GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
              ldat3.heightHint = 100;
              comp.setLayoutData(ldat3);
              toolSel.setControl(comp);
              toolsList = new List(comp, SWT.MULTI | SWT.V_SCROLL);
              GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
              gridData.widthHint = 1;
              gridData.heightHint = 100;
              toolsList.setLayoutData(gridData);
              toolsMultiSelect = new MultiSelectListener(toolsList, () -> cfg.setEnabledTools(toolsList.getSelection()));
              cfg.addModelObs(m -> {
                if (m != null)
                  toolsList.setItems(m.cap.getTools());
              }, true);
              cfg.addEnabledToolsObs(t -> {
                if (!Arrays.equals(toolsList.getSelection(), t))
                  toolsMultiSelect.setSelection(t);
              }, true);
            }
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      nodes:
      - id: SelectTab.create
        type: MethodDeclaration
        code: |2
            public void create(TabFolder instr, ConfigManager cfg) {
              TabItem instrSel = new TabItem(instr, SWT.NONE);
              instrSel.setText("Select");
              SashForm sash = new SashForm(instr, SWT.VERTICAL);
              sash.setLayout(new GridLayout(1, false));
              GridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);
              scl2.heightHint = 100;
              scl2.widthHint = 1;
              sash.setLayoutData(scl2);
              Composite comp = new Composite(sash, SWT.NONE);
              comp.setLayout(new GridLayout());
              GridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);
              ldat3.heightHint = 100;
              comp.setLayoutData(ldat3);
              instrSel.setControl(sash);
              instructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);
              cfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);
              GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
              gridData.widthHint = 1;
              gridData.heightHint = 100;
              instructionList.setLayoutData(gridData);
              instructionMultiSelect = new MultiSelectListener(instructionList, () -> {
                if (guard.isUpdating())
                  return;
                cfg.setSystemPrompt(updatePromptLines(instructionList.getItems(), instructionList.getSelectionIndices()));
              });
              createFreeTextArea(sash, cfg);
            }
```
/answer 9c1b-22-21 deny ich habe zurückgesetzt
Thinking:
Ich lege zwei neue Dateien an: die Komponente `MultiSelectTable.java` mit unveränderter Schnittstelle plus `onChange`, und eine Demo `MultiSelectTableDemo.java` mit Knöpfen, die deinen Problemfall nachstellen.


ReasoningToken: 5228
Control Request:
```yaml
id: 9c1b-23-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectTable.java
    source: "package xy.ai.workbench.view;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.graphics.Point;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Event;\nimport org.eclipse.swt.widgets.Table;\nimport org.eclipse.swt.widgets.TableColumn;\nimport org.eclipse.swt.widgets.TableItem;\n\nimport xy.ai.workbench.view.session.Clearable;\n\n/**\n * List-like multi select with the following click behaviour:\n * <ul>\n * <li>plain click: toggles the clicked row</li>\n * <li>CTRL + click: only the clicked row is active</li>\n * <li>SHIFT + click: rows from the anchor (last clicked row) to the clicked row\n * are active</li>\n * <li>SPACE: toggles the row with the keyboard cursor</li>\n * </ul>\n * <p>\n * The \"active\" state of a row is stored by this class in the row itself\n * ({@link TableItem#setData(Object)}) and is never touched by the native\n * selection of the table. The native selection only serves as the cursor. So\n * there is no second copy of the state that could get out of sync, and\n * programmatic changes ({@link #setItems(String[])},\n * {@link #setSelection(String[])}) never influence the next click.\n * <p>\n * The clicked row is determined with {@link Table#getItem(Point)}.\n * <p>\n * Display: either the active rows are drawn like selected rows and the native\n * highlight is suppressed ({@code markerColumn == false}), or an additional\n * first column shows a check mark for active rows ({@code markerColumn == true},\n * no custom drawing).\n * <p>\n * {@code onChange} is called after each change by the user. Programmatic\n * changes do not call it.\n */\npublic class MultiSelectTable implements Clearable {\n\n\tprivate static final int MARKER_WIDTH = 28;\n\n\tprivate final Table table;\n\tprivate final Runnable onChange;\n\tprivate final boolean marker;\n\tprivate TableColumn markerCol;\n\tprivate final TableColumn textCol;\n\tprivate int anchor = -1;\n\n\tpublic MultiSelectTable(Composite parent, Runnable onChange) {\n\t\tthis(parent, false, onChange);\n\t}\n\n\tpublic MultiSelectTable(Composite parent, boolean markerColumn, Runnable onChange) {\n\t\tthis.onChange = onChange;\n\t\tthis.marker = markerColumn;\n\t\ttable = new Table(parent, SWT.SINGLE | SWT.V_SCROLL | SWT.FULL_SELECTION);\n\t\ttable.setHeaderVisible(false);\n\t\ttable.setLinesVisible(false);\n\t\tif (marker)\n\t\t\tmarkerCol = new TableColumn(table, SWT.CENTER);\n\t\ttextCol = new TableColumn(table, SWT.NONE);\n\n\t\ttable.addListener(SWT.Resize, e -> layoutColumns());\n\t\ttable.addListener(SWT.MouseDown, this::onMouseDown);\n\t\ttable.addListener(SWT.KeyDown, this::onKeyDown);\n\t\tif (!marker)\n\t\t\ttable.addListener(SWT.EraseItem, this::onEraseItem);\n\t}\n\n\t// ---------------------------------------------------------------- events\n\n\tprivate void onMouseDown(Event e) {\n\t\tif (e.button != 1)\n\t\t\treturn;\n\t\tTableItem item = table.getItem(new Point(e.x, e.y));\n\t\tif (item == null)\n\t\t\treturn;\n\t\tclick(table.indexOf(item), e.stateMask);\n\t}\n\n\tprivate void onKeyDown(Event e) {\n\t\tif (e.character != ' ')\n\t\t\treturn;\n\t\tint idx = table.getSelectionIndex();\n\t\tif (idx >= 0) {\n\t\t\tclick(idx, 0);\n\t\t\te.doit = false;\n\t\t}\n\t}\n\n\t/** Suppresses the native selection highlight, active rows are drawn by us. */\n\tprivate void onEraseItem(Event e) {\n\t\te.detail &= ~SWT.SELECTED;\n\t\tif (isOn((TableItem) e.item)) {\n\t\t\te.gc.setBackground(table.getDisplay().getSystemColor(SWT.COLOR_LIST_SELECTION));\n\t\t\te.gc.fillRectangle(e.x, e.y, e.width, e.height);\n\t\t}\n\t}\n\n\t// ----------------------------------------------------------------- logic\n\n\tprivate void click(int idx, int stateMask) {\n\t\tint count = table.getItemCount();\n\t\tif (idx < 0 || idx >= count)\n\t\t\treturn;\n\n\t\tif ((stateMask & SWT.SHIFT) != 0) {\n\t\t\tint from = validAnchor(idx);\n\t\t\tint lo = Math.min(from, idx);\n\t\t\tint hi = Math.max(from, idx);\n\t\t\tfor (int i = 0; i < count; i++)\n\t\t\t\tset(i, i >= lo && i <= hi);\n\t\t\t// anchor stays so the range can be adjusted by further shift clicks\n\t\t} else if ((stateMask & SWT.MOD1) != 0) {\n\t\t\tfor (int i = 0; i < count; i++)\n\t\t\t\tset(i, i == idx);\n\t\t\tanchor = idx;\n\t\t} else {\n\t\t\tset(idx, !isOn(table.getItem(idx)));\n\t\t\tanchor = idx;\n\t\t}\n\t\tonChange.run();\n\t}\n\n\t/** The anchor if valid, otherwise the first active row, otherwise the clicked row. */\n\tprivate int validAnchor(int clicked) {\n\t\tif (anchor >= 0 && anchor < table.getItemCount())\n\t\t\treturn anchor;\n\t\tfor (int i = 0; i < table.getItemCount(); i++)\n\t\t\tif (isOn(table.getItem(i)))\n\t\t\t\treturn i;\n\t\treturn clicked;\n\t}\n\n\t// ------------------------------------------------------------ rendering\n\n\tprivate static boolean isOn(TableItem item) {\n\t\treturn Boolean.TRUE.equals(item.getData());\n\t}\n\n\tprivate void set(int idx, boolean on) {\n\t\tTableItem item = table.getItem(idx);\n\t\titem.setData(on);\n\t\trender(item);\n\t}\n\n\tprivate void render(TableItem item) {\n\t\tboolean on = isOn(item);\n\t\tif (marker) {\n\t\t\titem.setText(0, on ? \"✓\" : \"\");\n\t\t} else {\n\t\t\tvar d = table.getDisplay();\n\t\t\titem.setBackground(on ? d.getSystemColor(SWT.COLOR_LIST_SELECTION) : null);\n\t\t\titem.setForeground(on ? d.getSystemColor(SWT.COLOR_LIST_SELECTION_TEXT) : null);\n\t\t}\n\t}\n\n\tprivate int textIndex() {\n\t\treturn marker ? 1 : 0;\n\t}\n\n\tprivate void layoutColumns() {\n\t\tint w = table.getClientArea().width;\n\t\tif (marker) {\n\t\t\tmarkerCol.setWidth(MARKER_WIDTH);\n\t\t\ttextCol.setWidth(Math.max(0, w - MARKER_WIDTH));\n\t\t} else {\n\t\t\ttextCol.setWidth(w);\n\t\t}\n\t}\n\n\t// ----------------------------------------------------------- public API\n\n\tpublic Table getControl() {\n\t\treturn table;\n\t}\n\n\tpublic void setLayoutData(Object layoutData) {\n\t\ttable.setLayoutData(layoutData);\n\t}\n\n\tpublic void setFocus() {\n\t\ttable.setFocus();\n\t}\n\n\t/** Replaces all rows; all rows are inactive afterwards. */\n\tpublic void setItems(String[] items) {\n\t\ttable.setRedraw(false);\n\t\ttry {\n\t\t\ttable.removeAll();\n\t\t\tanchor = -1;\n\t\t\tif (items != null)\n\t\t\t\tfor (String text : items) {\n\t\t\t\t\tTableItem item = new TableItem(table, SWT.NONE);\n\t\t\t\t\titem.setText(textIndex(), text);\n\t\t\t\t\titem.setData(Boolean.FALSE);\n\t\t\t\t\trender(item);\n\t\t\t\t}\n\t\t\tlayoutColumns();\n\t\t} finally {\n\t\t\ttable.setRedraw(true);\n\t\t}\n\t}\n\n\tpublic String[] getItems() {\n\t\tString[] result = new String[table.getItemCount()];\n\t\tfor (int i = 0; i < result.length; i++)\n\t\t\tresult[i] = table.getItem(i).getText(textIndex());\n\t\treturn result;\n\t}\n\n\t/** Texts of the active rows. */\n\tpublic String[] getSelection() {\n\t\treturn java.util.Arrays.stream(getSelectionIndices()).mapToObj(i -> table.getItem(i).getText(textIndex()))\n\t\t\t\t.toArray(String[]::new);\n\t}\n\n\t/** Indices of the active rows. */\n\tpublic int[] getSelectionIndices() {\n\t\treturn java.util.stream.IntStream.range(0, table.getItemCount()).filter(i -> isOn(table.getItem(i))).toArray();\n\t}\n\n\t/** Activates exactly the given rows (by index), without calling onChange. */\n\tpublic void setSelectionIndices(int[] indices) {\n\t\tjava.util.Set<Integer> on = new java.util.HashSet<>();\n\t\tif (indices != null)\n\t\t\tfor (int i : indices)\n\t\t\t\ton.add(i);\n\t\tfor (int i = 0; i < table.getItemCount(); i++)\n\t\t\tset(i, on.contains(i));\n\t}\n\n\t/** Activates exactly the rows with the given texts, without calling onChange. */\n\tpublic void setSelection(String[] items) {\n\t\tjava.util.List<String> all = java.util.Arrays.asList(getItems());\n\t\tjava.util.Set<Integer> on = new java.util.HashSet<>();\n\t\tif (items != null)\n\t\t\tfor (String item : items) {\n\t\t\t\tint idx = all.indexOf(item);\n\t\t\t\tif (idx >= 0)\n\t\t\t\t\ton.add(idx);\n\t\t\t}\n\t\tfor (int i = 0; i < table.getItemCount(); i++)\n\t\t\tset(i, on.contains(i));\n\t}\n\n\t/** Deactivates all rows. */\n\t@Override\n\tpublic void clear() {\n\t\tanchor = -1;\n\t\tfor (int i = 0; i < table.getItemCount(); i++)\n\t\t\tset(i, false);\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectTableDemo.java
    source: "package xy.ai.workbench.view;\n\nimport java.util.Arrays;\n\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Button;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.swt.widgets.Label;\nimport org.eclipse.swt.widgets.Shell;\n\n/**\n * Manual test window for {@link MultiSelectTable}. Delete after testing.\n * <p>\n * Run without arguments: active rows are drawn like selected rows (native\n * highlight suppressed). Run with the argument {@code marker}: additional check\n * mark column instead (fallback if the drawing does not work).\n * <p>\n * Things to check:\n * <ul>\n * <li>click toggles, CTRL+click = only this row, SHIFT+click = range</li>\n * <li>\"Select 0-2 programmatically\" / \"Replace items\" and then click right away:\n * the list must not reset (the original problem)</li>\n * <li>scroll the list (many rows) and click: the right row is hit</li>\n * <li>cursor keys + SPACE</li>\n * <li>no leftover native highlight, no flicker</li>\n * </ul>\n */\npublic class MultiSelectTableDemo {\n\n\tpublic static void main(String[] args) {\n\t\tboolean marker = Arrays.asList(args).contains(\"marker\");\n\n\t\tDisplay display = new Display();\n\t\tShell shell = new Shell(display);\n\t\tshell.setText(\"MultiSelectTable demo (\" + (marker ? \"marker column\" : \"custom highlight\") + \")\");\n\t\tshell.setLayout(new GridLayout(1, false));\n\n\t\tLabel state = new Label(shell, SWT.WRAP);\n\t\tstate.setLayoutData(new GridData(SWT.FILL, SWT.TOP, true, false));\n\n\t\tMultiSelectTable[] holder = new MultiSelectTable[1];\n\t\tRunnable onChange = () -> {\n\t\t\tstate.setText(\"onChange: \" + Arrays.toString(holder[0].getSelection()));\n\t\t\tshell.layout();\n\t\t};\n\t\tMultiSelectTable list = new MultiSelectTable(shell, marker, onChange);\n\t\tholder[0] = list;\n\t\tGridData gd = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgd.heightHint = 200;\n\t\tgd.widthHint = 300;\n\t\tlist.setLayoutData(gd);\n\t\tlist.setItems(items(30));\n\t\tlist.setSelection(new String[] { \"Item 1\", \"Item 4\" });\n\n\t\tButton sel = new Button(shell, SWT.PUSH);\n\t\tsel.setText(\"Select 0-2 programmatically\");\n\t\tsel.addListener(SWT.Selection, e -> list.setSelectionIndices(new int[] { 0, 1, 2 }));\n\n\t\tButton repl = new Button(shell, SWT.PUSH);\n\t\trepl.setText(\"Replace items (all inactive)\");\n\t\trepl.addListener(SWT.Selection, e -> list.setItems(items(30)));\n\n\t\tButton clr = new Button(shell, SWT.PUSH);\n\t\tclr.setText(\"clear()\");\n\t\tclr.addListener(SWT.Selection, e -> list.clear());\n\n\t\tshell.pack();\n\t\tshell.open();\n\t\twhile (!shell.isDisposed())\n\t\t\tif (!display.readAndDispatch())\n\t\t\t\tdisplay.sleep();\n\t\tdisplay.dispose();\n\t}\n\n\tprivate static String[] items(int n) {\n\t\tString[] r = new String[n];\n\t\tfor (int i = 0; i < n; i++)\n\t\t\tr[i] = \"Item \" + i;\n\t\treturn r;\n\t}\n}\n"
  reason: "Neue Komponente MultiSelectTable (Table-basiert, eigener Zustand pro Zeile, Hit-Test über getItem(Point)) und ein Demo-Fenster zum manuellen Test anlegen; bestehende Dateien bleiben unberührt"
```
/answer 9c1b-23-0 allow
Control Request:
```yaml
id: 9c1b-24-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectTable.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectTableDemo.java
  reason: Syntax der beiden neuen Dateien prüfen
```
/answer 9c1b-24-0 allow
Result Stats: id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, total: 125870, in: 68, out: 53204, reason: 0, read: 1271823, write: 72598, start: 1794171614153, end: 1794171622844, time: 8607, total: 399724, io: 0, control: 84, sent: 0, recv: 240
Die zwei neuen Dateien sind angelegt, `ToolsTab`, `SelectTab` und dein `MultiSelectListener` habe ich nicht angefasst. Die Syntaxprüfung ist grün, kompiliert und gestartet habe ich nichts.

## Was jetzt da ist
- **`MultiSelectTable`:** Die Komponente hat dieselbe Schnittstelle wie die bisherige Liste: `setItems`, `getItems`, `getSelection`, `getSelectionIndices`, `setSelection(String[])` und `clear`. Dazu kommen ein `onChange`-Callback und `getControl()` fürs Layout.
  - **Zustand:** Er liegt pro Zeile in `TableItem.setData`. Die native Auswahl der Tabelle dient nur als Cursor und beeinflusst nichts.
  - **Klickziel:** Es kommt von `table.getItem(Point)`.
  - **Klickverhalten:**
    - Ein einfacher Klick schaltet um.
    - Strg lässt nur diese Zeile aktiv.
    - Shift aktiviert den Bereich vom Anker bis zum Klick.
    - Die Leertaste schaltet die Cursorzeile um.
  - **Programmatische Änderungen:** `setSelection` und `setItems` lösen kein `onChange` aus und beeinflussen den nächsten Klick nicht.
- **`MultiSelectTableDemo`:** Das ist ein Testfenster mit 30 Zeilen. Es hat Knöpfe, die programmatisch die Auswahl ändern oder die Items ersetzen. Danach klickst du sofort, das ist dein Reset-Fall. Nach dem Test kannst du die Datei löschen.

## Bitte probier das aus
1. Starte `MultiSelectTableDemo` ohne Argument. Das ist der Modus mit eigenem Highlight.
2. Prüfe diese Punkte:
   - Klick, Strg und Shift.
   - Die Knöpfe "Select 0-2 programmatically" und "Replace items", jeweils mit einem Klick direkt danach. Die Liste darf nicht zurückspringen.
   - Scrollen und danach klicken. Es muss die richtige Zeile getroffen werden.
   - Pfeiltasten und Leertaste.
   - Es darf kein Rest des nativen Highlights und kein Flackern zu sehen sein.
3. Wenn das Highlight unter GTK nicht sauber ist, starte mit dem Argument `marker`. Dann zeigt eine zusätzliche Spalte ein "✓". Das kommt ohne eigenes Zeichnen aus.

Sag mir, was du siehst, und zwar auch dann, wenn es nur "sieht komisch aus" ist. Erst danach stelle ich `ToolsTab` und `SelectTab` auf die Tabelle um. Ich zeige dir die Änderungen vorher.

## Offene Punkte
- **`EraseItem` unter GTK3:** Dass es dort sauber funktioniert, ist nicht belegt, deshalb gibt es den `marker`-Fallback.
- **Native Selection-Events:** Die Tabelle löst weiterhin welche aus, wenn man klickt. Ich höre sie bewusst nicht ab. `onChange` kommt nur von meiner Logik.
Äh nein, die Demo ist Bullshit. Die Tabelle muss in Eclipse laufen.
SystemInit:  id=d9026a6c-0a72-4ad5-86e2-e90ef50169bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Verstanden, die Demo war überflüssig. Ich lösche sie und baue die Tabelle direkt in die beiden Tabs ein, damit du es in Eclipse testest. Zuerst lese ich den aktuellen Stand der Tabs.
Control Request:
```yaml
id: 9c1b-25-0
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/MultiSelectTableDemo.java
  reason: "Selbst angelegte Demo-Datei wieder entfernen, der Test soll direkt in Eclipse in den Tabs erfolgen"
```
/answer 9c1b-25-0 allow
ReasoningToken: 232
Control Request:
```yaml
id: 9c1b-26-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    ids:
    - ToolsTab
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    ids:
    - SelectTab
  reason: "Aktuellen (zurückgesetzten) Stand der beiden Tabs lesen, bevor sie auf MultiSelectTable umgestellt werden"
```
/answer 9c1b-26-0 allow
Control Request:
```yaml
id: 9c1b-27-26
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
      nodes:
      - id: ToolsTab
        type: ClassOrInterfaceDeclaration
        signature: "/**  * \"Tools\" tab: multi-select list of tools enabled for the current model.  …"
        docstring: "\"Tools\" tab: multi-select list of tools enabled for the current model."
        children:
        - id: ToolsTab.c4iLED|KWV8qC
          type: StatementGroup
          code: "\tprivate List toolsList;\n\tprivate MultiSelectListener toolsMultiSelect;\n"
        - id: ToolsTab.create
          type: MethodDeclaration
          code: "\tpublic void create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem toolSel = new TabItem(instr, SWT.NONE);\n\t\ttoolSel.setText(\"Tools\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\ttoolSel.setControl(comp);\n\t\ttoolsList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\ttoolsList.setLayoutData(gridData);\n\t\ttoolsList.addMouseListener(MouseListener.mouseDownAdapter(m -> toolsList.setFocus()));\n\t\ttoolsList.addSelectionListener(\n\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setEnabledTools(toolsList.getSelection())));\n\t\tcfg.addModelObs(m -> {\n\t\t\tif (m != null)\n\t\t\t\ttoolsList.setItems(m.cap.getTools());\n\t\t}, true);\n\t\ttoolsMultiSelect = new MultiSelectListener(toolsList);\n\t\tcfg.addEnabledToolsObs(t -> {\n\t\t\tif (!Arrays.equals(toolsList.getSelection(), t))\n\t\t\t\ttoolsMultiSelect.setSelection(t);\n\t\t}, true);\n\t}\n"
        - id: ToolsTab.getSelection
          type: MethodDeclaration
          code: "\t@Override\n\tpublic String[] getSelection() {\n\t\treturn toolsList.getSelection();\n\t}\n"
        - id: ToolsTab.setSelection
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void setSelection(String[] items) {\n\t\ttoolsMultiSelect.setSelection(items);\n\t}\n"
        - id: ToolsTab.clear
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void clear() {\n\t\ttoolsMultiSelect.clear();\n\t}\n"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
      nodes:
      - id: SelectTab
        type: ClassOrInterfaceDeclaration
        signature: "/**  * \"Select\" tab: toggles system-prompt lines on/off and holds the free-text…"
        docstring: "\"Select\" tab: toggles system-prompt lines on/off and holds the free-text"
        children:
        - id: SelectTab.FGxcw8|xBZMzM
          type: StatementGroup
          code: "\tprivate final FormToolkit toolkit;\n\tprivate final UpdateGuard guard;\n\tprivate final Runnable reflow;\n\n\tprivate List instructionList;\n\tprivate Text instructionFree;\n\tprivate java.util.List<String> instructionSelection = new ArrayList<>();\n"
        - id: SelectTab.SelectTab
          type: ConstructorDeclaration
          code: "\tpublic SelectTab(FormToolkit toolkit, UpdateGuard guard, Runnable reflow) {\n\t\tthis.toolkit = toolkit;\n\t\tthis.guard = guard;\n\t\tthis.reflow = reflow;\n\t}\n"
        - id: SelectTab.create
          type: MethodDeclaration
          code: "\tpublic void create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem instrSel = new TabItem(instr, SWT.NONE);\n\t\tinstrSel.setText(\"Select\");\n\n\t\tSashForm sash = new SashForm(instr, SWT.VERTICAL);\n\t\tsash.setLayout(new GridLayout(1, false));\n\t\tGridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tscl2.heightHint = 100;\n\t\tscl2.widthHint = 1;\n\t\tsash.setLayoutData(scl2);\n\n\t\tComposite comp = new Composite(sash, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\tinstrSel.setControl(sash);\n\t\tinstructionList = new List(comp, SWT.MULTI | SWT.V_SCROLL);\n\t\tcfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\tinstructionList.setLayoutData(gridData);\n\t\tinstructionList.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionList.setFocus()));\n\n\t\tinstructionList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tinstructionSelection = new ArrayList<>(Arrays.asList(instructionList.getSelection()));\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t}));\n\t\tinstructionList.addListener(SWT.MouseDown, event -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tString[] clickedIndex = instructionList.getSelection();\n\t\t\tLOG.info(Arrays.toString(clickedIndex));\n\t\t\tif (clickedIndex.length != 1)\n\t\t\t\treturn;\n\n\t\t\tif (!instructionSelection.remove(clickedIndex[0]))\n\t\t\t\tinstructionSelection.add(clickedIndex[0]);\n\n\t\t\tString[] cur = instructionList.getItems();\n\t\t\tcfg.setSystemPrompt(updatePromptLines(cur));\n\t\t});\n\n\t\tcreateFreeTextArea(sash, cfg);\n\t}\n"
        - id: SelectTab.createFreeTextArea
          type: MethodDeclaration
          code: "\tprivate void createFreeTextArea(SashForm sash, ConfigManager cfg) {\n\t\tinstructionFree = toolkit.createText(sash, \"\", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);\n\t\tcfg.addSystemFreeObs(p -> {\n\t\t\tif (!instructionFree.isFocusControl())\n\t\t\t\tguard.run(() -> {\n\t\t\t\t\tinstructionFree.setText(p != null ? p : \"\");\n\t\t\t\t\treflow.run();\n\t\t\t\t});\n\t\t}, true);\n\t\tinstructionFree\n\t\t\t\t.addFocusListener(FocusListener.focusLostAdapter(e -> cfg.setSystemFree(instructionFree.getText())));\n\t\tGridData gridData = new GridData(GridData.FILL_HORIZONTAL);\n\t\tgridData.widthHint = 1;\n\t\tinstructionFree.setLayoutData(gridData);\n\t\tinstructionFree.addMouseListener(MouseListener.mouseDownAdapter(m -> instructionFree.setFocus()));\n\t\tsash.setWeights(3, 1);\n\t}\n"
        - id: SelectTab.updatePromptLines
          type: MethodDeclaration
          code: "\tprivate String[] updatePromptLines(String[] cur) {\n\t\tString[] upd = new String[cur.length];\n\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\tString line = cur[i];\n\t\t\tboolean isSelected = instructionSelection.contains(line);\n\t\t\tif (!isSelected && !line.startsWith(\"#\")) {\n\t\t\t\tline = \"#\" + line;\n\t\t\t} else if (isSelected && line.startsWith(\"#\")) {\n\t\t\t\tline = line.substring(1);\n\t\t\t}\n\t\t\tupd[i] = line;\n\t\t}\n\t\treturn upd;\n\t}\n"
        - id: SelectTab.updateInstructionList
          type: MethodDeclaration
          code: "\tprivate void updateInstructionList(String[] systemPrompt) {\n\t\tguard.run(() -> {\n\t\t\tinstructionList.setItems(systemPrompt);\n\t\t\tinstructionList.deselectAll();\n\t\t\tfor (int i = 0; i < systemPrompt.length; i++) {\n\t\t\t\tif (!systemPrompt[i].startsWith(\"#\"))\n\t\t\t\t\tinstructionList.select(i);\n\t\t\t}\n\t\t\treflow.run();\n\t\t});\n\t}\n"
        - id: SelectTab.clear
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void clear() {\n\t\tinstructionSelection.clear();\n\t}\n"
```
/answer 9c1b-27-26 allow
Text:
 Beide Tabs sind auf dem Originalstand. Jetzt stelle ich sie um. Den Feldnamen `toolsList` bzw. `instructionList` behalte ich bei, nur der Typ wird `MultiSelectTable`. Das hält den Diff klein.
Control Request:
```yaml
id: 9c1b-28-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: ToolsTab.c4iLED|KWV8qC
    source: "\tprivate MultiSelectTable toolsList;\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: ToolsTab.create
    source: "\tpublic void create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem toolSel = new TabItem(instr, SWT.NONE);\n\t\ttoolSel.setText(\"Tools\");\n\n\t\tComposite comp = new Composite(instr, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\ttoolSel.setControl(comp);\n\t\ttoolsList = new MultiSelectTable(comp, () -> cfg.setEnabledTools(toolsList.getSelection()));\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\ttoolsList.setLayoutData(gridData);\n\t\tcfg.addModelObs(m -> {\n\t\t\tif (m != null)\n\t\t\t\ttoolsList.setItems(m.cap.getTools());\n\t\t}, true);\n\t\tcfg.addEnabledToolsObs(t -> {\n\t\t\tif (!Arrays.equals(toolsList.getSelection(), t))\n\t\t\t\ttoolsList.setSelection(t);\n\t\t}, true);\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: ToolsTab.setSelection
    source: "\t@Override\n\tpublic void setSelection(String[] items) {\n\t\ttoolsList.setSelection(items);\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/ToolsTab.java
    id: ToolsTab.clear
    source: "\t@Override\n\tpublic void clear() {\n\t\ttoolsList.clear();\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.FGxcw8|xBZMzM
    source: "\tprivate final FormToolkit toolkit;\n\tprivate final UpdateGuard guard;\n\tprivate final Runnable reflow;\n\n\tprivate MultiSelectTable instructionList;\n\tprivate Text instructionFree;\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.create
    source: "\tpublic void create(TabFolder instr, ConfigManager cfg) {\n\t\tTabItem instrSel = new TabItem(instr, SWT.NONE);\n\t\tinstrSel.setText(\"Select\");\n\n\t\tSashForm sash = new SashForm(instr, SWT.VERTICAL);\n\t\tsash.setLayout(new GridLayout(1, false));\n\t\tGridData scl2 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tscl2.heightHint = 100;\n\t\tscl2.widthHint = 1;\n\t\tsash.setLayoutData(scl2);\n\n\t\tComposite comp = new Composite(sash, SWT.NONE);\n\t\tcomp.setLayout(new GridLayout());\n\t\tGridData ldat3 = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tldat3.heightHint = 100;\n\t\tcomp.setLayoutData(ldat3);\n\t\tinstrSel.setControl(sash);\n\t\tinstructionList = new MultiSelectTable(comp, () -> {\n\t\t\tif (guard.isUpdating())\n\t\t\t\treturn;\n\t\t\tcfg.setSystemPrompt(\n\t\t\t\t\tupdatePromptLines(instructionList.getItems(), instructionList.getSelectionIndices()));\n\t\t});\n\t\tcfg.addSystemPromptObs(p -> updateInstructionList(p.systemPrompt), true);\n\n\t\tGridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);\n\t\tgridData.widthHint = 1;\n\t\tgridData.heightHint = 100;\n\t\tinstructionList.setLayoutData(gridData);\n\n\t\tcreateFreeTextArea(sash, cfg);\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.updatePromptLines
    source: "\tprivate String[] updatePromptLines(String[] cur, int[] selectedIdx) {\n\t\tjava.util.Set<Integer> selected = new java.util.HashSet<>();\n\t\tfor (int i : selectedIdx)\n\t\t\tselected.add(i);\n\t\tString[] upd = new String[cur.length];\n\t\tfor (int i = 0; i < cur.length; i++) {\n\t\t\tString line = cur[i];\n\t\t\tboolean isSelected = selected.contains(i);\n\t\t\tif (!isSelected && !line.startsWith(\"#\")) {\n\t\t\t\tline = \"#\" + line;\n\t\t\t} else if (isSelected && line.startsWith(\"#\")) {\n\t\t\t\tline = line.substring(1);\n\t\t\t}\n\t\t\tupd[i] = line;\n\t\t}\n\t\treturn upd;\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.updateInstructionList
    source: "\tprivate void updateInstructionList(String[] systemPrompt) {\n\t\tguard.run(() -> {\n\t\t\tinstructionList.setItems(systemPrompt);\n\t\t\tinstructionList.setSelectionIndices(java.util.stream.IntStream.range(0, systemPrompt.length)\n\t\t\t\t\t.filter(i -> !systemPrompt[i].startsWith(\"#\")).toArray());\n\t\t\treflow.run();\n\t\t});\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/session/SelectTab.java
    id: SelectTab.clear
    source: "\t@Override\n\tpublic void clear() {\n\t\tinstructionList.clear();\n\t}\n"
  reason: "ToolsTab und SelectTab von List/MultiSelectListener auf die neue MultiSelectTable umstellen; Schattenzustand instructionSelection entfällt, Zeilen werden per Index statt per Text zugeordnet"
```
/answer 9c1b-28-0 allow