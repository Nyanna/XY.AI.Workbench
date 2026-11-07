Die Tabelle in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java` soll Zeilenweise Einfärbung in Abhängigkeit zu ihren Zustand haben.
Sessions die Open sind und deren Remaining Time abläuft, soll Gelb hinterlegt werden.
Erreicht die Lebenszeit der Session ihre letzte Minute soll sie Abwechseln Rot/Geld blinken. Das sollte sich einfach umsetzen lassen mit dem bestehenden Refresh Timer und einem Modulo auf der Restzeit. Die Folge wäre ein Blinken im Takt des Refreshs.

## Hinweise

SWT-`Table` bietet dafür mehrere Wege:

**Ganze Zeile (TableItem)**

```java
TableItem item = table.getItem(0);
item.setBackground(display.getSystemColor(SWT.COLOR_YELLOW));
item.setForeground(display.getSystemColor(SWT.COLOR_RED));
```

**Einzelne Zelle**

```java
item.setBackground(1, new Color(display, 200, 230, 255)); // Spalte 1
```

**Dynamisch per Paint-Listener (z. B. abwechselnde Zeilen, bedingte Färbung)**

```java
table.addListener(SWT.EraseItem, e -> {
    TableItem it = (TableItem) e.item;
    if (e.index == 0 && table.indexOf(it) % 2 == 0) {
        GC gc = e.gc;
        gc.setBackground(display.getSystemColor(SWT.COLOR_GRAY));
        gc.fillRectangle(0, e.y, table.getClientArea().width, e.height);
        e.detail &= ~SWT.HOT; // optional
    }
});
```

Für `SWT.EraseItem` ggf. `table.setData("SWT.EraseItem")`-Zeilenhöhe beachten; die Selektionsfarbe bleibt sonst plattformabhängig.

**Hinweise**

* `Color`-Objekte, die du selbst erzeugst (`new Color(...)`), müssen mit `dispose()` freigegeben werden (ab SWT 4.x/JFace: `LocalResourceManager` verwenden). Systemfarben nicht.
* Bei JFace `TableViewer`: `ColumnLabelProvider` bzw. `IColorProvider` (`getBackground`/`getForeground`) oder `StyledCellLabelProvider` verwenden.
* Unter Windows kann die Selektionsfarbe bzw. Hintergrundfarbe bei Themes (Dark Mode) abweichen; hier hilft `SWT.EraseItem`/`SWT.PaintItem`.

SystemInit:  id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: b725-1-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  reason: "Struktur der View-Klasse verstehen (Tabelle, Refresh Timer)"
```
/answer b725-1-0 allow
Control Request:
```yaml
id: b725-2-1
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
      nodes:
      - id: tNpOV8|HhEtWZ
        signature: import java.nio.file.Path;
      - id: vUKs95|zGNIPW
        signature: import org.eclipse.ui.part.ViewPart;
      - id: ClaudeCodeSessionView
        signature: /**  * Eclipse ViewPart that displays active Claude Code CLI sessions in real t…
        docstring: Eclipse ViewPart that displays active Claude Code CLI sessions in real time.
        children:
        - id: ClaudeCodeSessionView.SFR7nV|qt8Q3l
          signature: public static final String ID = "xy.ai.workbench.views.ClaudeCodeSessionView";
        - id: ClaudeCodeSessionView.tVJW11|RHWM5g
          signature: private long lastRefreshTime = 0;
        - id: ClaudeCodeSessionView.createPartControl
          signature: public void createPartControl(Composite parent)
        - id: ClaudeCodeSessionView.dispose
          signature: public void dispose()
        - id: ClaudeCodeSessionView.setFocus
          signature: public void setFocus()
        - id: ClaudeCodeSessionView.makeActions
          signature: private void makeActions()
        - id: ClaudeCodeSessionView.updateCurrentEditor
          signature: private void updateCurrentEditor()
        - id: ClaudeCodeSessionView.findAssociatedSession
          signature: private CCSession findAssociatedSession(List<CCSession> sessions)
        - id: ClaudeCodeSessionView.selectSession
          signature: private void selectSession(CCSession session)
        - id: ClaudeCodeSessionView.syncSelectionToCurrentFile
          signature: private void syncSelectionToCurrentFile()
        - id: ClaudeCodeSessionView.idLabel
          signature: private String idLabel(CCSession s)
        - id: ClaudeCodeSessionView.stateLabel
          signature: private String stateLabel(CCSession s)
        - id: ClaudeCodeSessionView.formatMMSS
          signature: private static String formatMMSS(long totalSeconds)
        - id: ClaudeCodeSessionView.detailLabel
          signature: private String detailLabel(CCSession s)
        - id: ClaudeCodeSessionView.abbreviateForDisplay
          signature: private static String abbreviateForDisplay(String s)
        - id: ClaudeCodeSessionView.fileNameOf
          signature: private static String fileNameOf(String path)
        - id: ClaudeCodeSessionView.refreshAsync
          signature: private void refreshAsync()
        - id: ClaudeCodeSessionView.scheduleDebouncedRefresh
          signature: private void scheduleDebouncedRefresh()
        - id: ClaudeCodeSessionView.refreshTable
          signature: private void refreshTable()
        - id: ClaudeCodeSessionView.createColumn
          signature: "private TableViewerColumn createColumn(String label, int weight)"
```
/answer b725-2-1 allow
Control Request:
```yaml
id: b725-3-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    ids:
    - ClaudeCodeSessionView.createPartControl
    - ClaudeCodeSessionView.dispose
    - ClaudeCodeSessionView.stateLabel
    - ClaudeCodeSessionView.formatMMSS
    - ClaudeCodeSessionView.refreshTable
    - ClaudeCodeSessionView.createColumn
    - ClaudeCodeSessionView.refreshAsync
  reason: Code der relevanten Methoden lesen
```
/answer b725-3-0 allow
Control Request:
```yaml
id: b725-4-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
      nodes:
      - id: ClaudeCodeSessionView.createPartControl
        type: MethodDeclaration
        code: |2
            @Override
            public void createPartControl(Composite parent) {
              sessionManager = Activator.getDefault().ccSessionManager;
              cfg = Activator.getDefault().cma.getManager();
              parent.setLayout(new FillLayout());
              // Table composite
              Composite tableComp = new Composite(parent, SWT.NONE);
              tableComp.setLayout(tableLayout = new TableColumnLayout());
              viewer = new TableViewer(tableComp, SWT.SINGLE | SWT.H_SCROLL | SWT.V_SCROLL | SWT.BORDER | SWT.FULL_SELECTION);
              Table table = viewer.getTable();
              table.setHeaderVisible(true);
              table.setLinesVisible(true);
              {
                createColumn("ID", 10).setLabelProvider(ColumnLabelProvider.createTextProvider(e -> idLabel((CCSession) e)));
                createColumn("State", 35).setLabelProvider(ColumnLabelProvider.createTextProvider(e -> stateLabel((CCSession) e)));
                createColumn("Detail", 55).setLabelProvider(ColumnLabelProvider.createTextProvider(e -> detailLabel((CCSession) e)));
              }
              viewer.setContentProvider(ArrayContentProvider.getInstance());
              viewer.setInput(new ArrayList<CCSession>());
              viewer.addSelectionChangedListener(event -> {
                IStructuredSelection sel = viewer.getStructuredSelection();
                if (sel.isEmpty())
                  sessionManager.setSelectedSessionUuid(null);
                else {
                  CCSession s = (CCSession) sel.getFirstElement();
                  sessionManager.setSelectedSessionUuid(s.getSessionUuid());
                }
              });
              viewer.addDoubleClickListener(event -> {
                IStructuredSelection sel = (IStructuredSelection) event.getSelection();
                if (!sel.isEmpty() && sel.getFirstElement() instanceof CCSession) {
                  CCSession s = (CCSession) sel.getFirstElement();
                  if (s != CNEW_LAUDE_CODE_SESSION)
                    new SessionDetailDialog(viewer.getControl().getShell(), s).open();
                }
              });
              sessionManager.addChangeListener(changeListener);
              // Toolbar
              makeActions();
              IActionBars bars = getViewSite().getActionBars();
              act.fillLocalToolBar(bars.getToolBarManager());
              act.fillLocalPullDown(bars.getMenuManager());
              bars.updateActionBars();
              Activator.getDefault().editorListener.addTextEditorObserver(e -> updateCurrentEditor());
              updateCurrentEditor();
              ttlRefreshRunnable = new Runnable() {

                @Override
                public void run() {
                  if (disposed)
                    return;
                  scheduleDebouncedRefresh();
                  Display.getCurrent().timerExec(TTL_REFRESH_INTERVAL_MS, this);
                }
              };
              Display.getDefault().timerExec(TTL_REFRESH_INTERVAL_MS, ttlRefreshRunnable);
            }
      - id: ClaudeCodeSessionView.dispose
        type: MethodDeclaration
        code: |2
            @Override
            public void dispose() {
              disposed = true;
              sessionManager.removeChangeListener(changeListener);
              Display.getDefault().timerExec(-1, ttlRefreshRunnable);
              super.dispose();
            }
      - id: ClaudeCodeSessionView.stateLabel
        type: MethodDeclaration
        code: |2
            private String stateLabel(CCSession s) {
              if (SessionState.Prompting.equals(s.getState()) && s.getLastSentAt() != null)
                return s.getState().name() + " (" + (Instant.now().toEpochMilli() - s.getLastSentAt().toEpochMilli()) + ")";
              if (SessionState.Open.equals(s.getState()))
                return formatMMSS(s.getRemainingTtlSeconds());
              return s.getState().name();
            }
      - id: ClaudeCodeSessionView.formatMMSS
        type: MethodDeclaration
        code: |2
            private static String formatMMSS(long totalSeconds) {
              if (totalSeconds < 0)
                return "--:--";
              long minutes = totalSeconds / 60;
              long seconds = totalSeconds % 60;
              return String.format("%02d:%02d", minutes, seconds);
            }
      - id: ClaudeCodeSessionView.refreshTable
        type: MethodDeclaration
        code: |2
            private void refreshTable() {
              if (viewer.getControl().isDisposed())
                return;
              List<CCSession> sessions = new ArrayList<>(sessionManager.getSessions());
              sessions.sort(Comparator.comparing(CCSession::getLastReceivedAt, Comparator.nullsLast(Comparator.reverseOrder())));
              List<CCSession> withDummy = new ArrayList<>();
              withDummy.add(CNEW_LAUDE_CODE_SESSION);
              withDummy.addAll(sessions);
              viewer.setInput(withDummy);
              viewer.refresh();
              if (syncAction.isChecked()) {
                IStructuredSelection currentSel = viewer.getStructuredSelection();
                Object selected = currentSel.getFirstElement();
                CCSession match = findAssociatedSession(sessions);
                Object desired = match != null ? match : CNEW_LAUDE_CODE_SESSION;
                if (selected != desired)
                  selectSession(match);
              }
            }
      - id: ClaudeCodeSessionView.createColumn
        type: MethodDeclaration
        code: |2
            private TableViewerColumn createColumn(String label, int weight) {
              TableColumn col = new TableColumn(viewer.getTable(), SWT.NONE);
              col.setText(label);
              col.setMoveable(true);
              col.setResizable(true);
              col.setWidth(50);
              tableLayout.setColumnData(col, new ColumnWeightData(weight));
              return new TableViewerColumn(viewer, col);
            }
      - id: ClaudeCodeSessionView.refreshAsync
        type: MethodDeclaration
        code: |2
            private void refreshAsync() {
              Display display = PlatformUI.getWorkbench().getDisplay();
              if (display != null && !display.isDisposed())
                display.asyncExec(this::scheduleDebouncedRefresh);
            }
```
/answer b725-4-3 allow
Control Request:
```yaml
id: b725-5-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: class CCSession\b|remaining|Remaining
    include:
    - '*.java'
    exclude:
    - bin/**
  reason: CCSession und Remaining-Time-Felder finden
```
/answer b725-5-0 allow
Control Request:
```yaml
id: b725-6-5
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/session/PresetList.java
        matches:
        - List<Raw> remaining = group;
        - "while (!remaining.isEmpty()) {"
        - "for (Raw raw : remaining) {"
        - remaining = stillColliding;
      - path: xy/ai/workbench/view/ClaudeCodeSessionView.java
        matches:
        - return formatMMSS(s.getRemainingTtlSeconds());
      - path: xy/ai/workbench/view/SessionDetailDialog.java
        matches:
        - long remaining = s.getRemainingTtlMinutes();
        - "return remaining < 0 ? \"—\" : remaining + \" min\";"
      - path: xy/ai/workbench/eclipse/EclipseProgress.java
        matches:
        - mon.setWorkRemaining(expectedWork);
      - path: xy/ai/workbench/eclipse/ConfigManagerAdapter.java
        matches:
        - "* be restored, and the model must be set before the remaining,"
```
/answer b725-6-5 allow
Control Request:
```yaml
id: b725-7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
    pattern: TTL_REFRESH_INTERVAL_MS|^import org.eclipse.(swt|jface.viewers)|getRemainingTtlSeconds|private .*(disposed|viewer);
    include:
    - ClaudeCodeSessionView.java
    exclude:
    - bin/**
    intent: editing
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: getRemainingTtlSeconds
    include:
    - CCSession.java
    exclude:
    - bin/**
  reason: "Konstanten, Imports und Felder prüfen"
```
/answer b725-7-0 allow
ReasoningToken: 142
Control Request:
```yaml
id: b725-8-7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
      matches:
      - path: ClaudeCodeSessionView.java
        nodes:
        - id: tNpOV8|HhEtWZ
          matches:
          - import org.eclipse.jface.viewers.ArrayContentProvider;
          - import org.eclipse.jface.viewers.ColumnLabelProvider;
          - import org.eclipse.jface.viewers.ColumnWeightData;
          - import org.eclipse.jface.viewers.IStructuredSelection;
          - import org.eclipse.jface.viewers.StructuredSelection;
          - import org.eclipse.jface.viewers.TableViewer;
          - import org.eclipse.jface.viewers.TableViewerColumn;
          - import org.eclipse.swt.SWT;
          - import org.eclipse.swt.layout.FillLayout;
          - import org.eclipse.swt.widgets.Composite;
          - import org.eclipse.swt.widgets.Display;
          - import org.eclipse.swt.widgets.Table;
          - import org.eclipse.swt.widgets.TableColumn;
        - id: ClaudeCodeSessionView.SFR7nV|qt8Q3l
          matches:
          - private static final int TTL_REFRESH_INTERVAL_MS = 1_000;
          - private TableViewer viewer;
      warning: "Limit of 15 matches reached (total 18); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
```
/answer b725-8-7 allow
Control Request:
```yaml
id: b725-9-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    ids:
    - ClaudeCodeSessionView.scheduleDebouncedRefresh
  reason: Debounce-Verhalten des Refresh prüfen
```
/answer b725-9-0 allow
Control Request:
```yaml
id: b725-a-9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
      nodes:
      - id: ClaudeCodeSessionView.scheduleDebouncedRefresh
        type: MethodDeclaration
        code: |2
            private void scheduleDebouncedRefresh() {
              if (disposed || viewer.getControl().isDisposed() || refreshScheduled)
                return;
              long elapsed = System.currentTimeMillis() - lastRefreshTime;
              int delay = (int) Math.max(0, REFRESH_DEBOUNCE_MS - elapsed);
              refreshScheduled = true;
              Display.getCurrent().timerExec(delay, () -> {
                refreshScheduled = false;
                lastRefreshTime = System.currentTimeMillis();
                refreshTable();
              });
            }
```
/answer b725-a-9 allow
ReasoningToken: 439
Control Request:
```yaml
id: b725-b-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: tNpOV8|HhEtWZ
    old_text: import org.eclipse.swt.SWT;
    new_text: |-
      import org.eclipse.swt.SWT;
      import org.eclipse.swt.graphics.Color;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: ClaudeCodeSessionView.createPartControl
    old_text: |2-
            createColumn("ID", 10).setLabelProvider(ColumnLabelProvider.createTextProvider(e -> idLabel((CCSession) e)));
            createColumn("State", 35).setLabelProvider(ColumnLabelProvider.createTextProvider(e -> stateLabel((CCSession) e)));
            createColumn("Detail", 55).setLabelProvider(ColumnLabelProvider.createTextProvider(e -> detailLabel((CCSession) e)));
    new_text: |2-
            createColumn("ID", 10).setLabelProvider(rowProvider(e -> idLabel(e)));
            createColumn("State", 35).setLabelProvider(rowProvider(e -> stateLabel(e)));
            createColumn("Detail", 55).setLabelProvider(rowProvider(e -> detailLabel(e)));
    exact: false
  reason: Color-Import ergänzen und Spalten-LabelProvider auf zeilenfärbenden Provider umstellen
```
/answer b725-b-0 allow
Control Request:
```yaml
id: b725-c-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: ClaudeCodeSessionView.formatMMSS
    position: before
    source: |2
        private ColumnLabelProvider rowProvider(java.util.function.Function<CCSession, String> text) {
          return new ColumnLabelProvider() {

            @Override
            public String getText(Object element) {
              return text.apply((CCSession) element);
            }

            @Override
            public Color getBackground(Object element) {
              return rowBackground((CCSession) element);
            }
          };
        }

        private Color rowBackground(CCSession s) {
          if (!SessionState.Open.equals(s.getState()))
            return null;
          long remaining = s.getRemainingTtlSeconds();
          if (remaining < 0)
            return null;
          Display display = viewer.getControl().getDisplay();
          boolean lastMinute = remaining < 60;
          boolean red = lastMinute && remaining % 2 == 0;
          return display.getSystemColor(red ? SWT.COLOR_RED : SWT.COLOR_YELLOW);
        }
  reason: "Hilfsmethoden für Zeilenfärbung (Gelb bei laufender TTL, Rot/Gelb-Blinken in letzter Minute) einfügen"
```
/answer b725-c-0 allow
ReasoningToken: 110
Control Request:
```yaml
id: b725-d-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  reason: Syntax prüfen
```
/answer b725-d-0 allow
Result Stats: id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, total: 19476, in: 12, out: 3818, reason: 0, read: 53661, write: 15646, start: 1794082056177, end: 1794082061226, time: 4987, total: 45206, io: 0, control: 62, sent: 0, recv: 126
Die Tabelle in `ClaudeCodeSessionView.java` färbt jetzt ganze Zeilen nach Zustand ein. Ich habe es nur auf Syntaxfehler geprüft (`ast_validate` meldet keine). Gebaut und in Eclipse angesehen habe ich es nicht.

- **Open mit laufender Restzeit:** Die Zeile ist gelb.
- **Letzte Minute (unter 60 Sekunden):** Die Zeile wechselt zwischen Rot und Gelb. Das hängt an der Parität der Restsekunden (`remaining % 2`). Bei 1 Sekunde Refresh-Intervall blinkt sie also jede Sekunde.
- **Alle anderen Zustände:** Die Zeile bleibt unverändert. Das gilt auch für Open-Sessions ohne bekannte Restzeit (`remaining < 0`, Anzeige `--:--`) und für die Dummy-Zeile "New Claude Code Session", solange sie nicht Open ist.

Die drei Spalten (ID, State, Detail) nutzen jetzt statt `ColumnLabelProvider.createTextProvider(...)` den neuen `rowProvider(...)`. Er liefert denselben Text und zusätzlich die Hintergrundfarbe. `rowBackground(CCSession)` enthält die Farblogik. Es werden nur Systemfarben benutzt, deshalb braucht es kein `dispose()`.

Das Blinken kommt vom bestehenden `ttlRefreshRunnable`. Es kann aber durch `REFRESH_DEBOUNCE_MS` unregelmäßig werden. Wenn der Debounce größer als 1 Sekunde ist, gehen einzelne Takte verloren. Ich habe den Wert nicht geprüft.
Hm das Select überlagert die Farbe. Kann man das Anpassen (nur beantworten)?
Result Stats: id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, total: 21045, in: 14, out: 4689, reason: 0, read: 69307, write: 16342, start: 1794082246931, end: 1794082254614, time: 7614, total: 52820, io: 0, control: 69, sent: 1, recv: 196
SystemInit:  id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Ja, das lässt sich anpassen. Die Selektionsfarbe zeichnet das Betriebssystem bzw. das Theme (unter Linux GTK) über den Hintergrund der Zeile. Das gilt besonders mit `SWT.FULL_SELECTION`. Es gibt mehrere Wege:

1. **`SWT.EraseItem`-Listener (am robustesten):** Im Listener prüfst du, ob die Zeile selektiert ist (`(e.detail & SWT.SELECTED) != 0`). Dann füllst du den Hintergrund selbst mit der Statusfarbe, zum Beispiel per `gc.fillRectangle(...)`. Danach löschst du das Flag mit `e.detail &= ~SWT.SELECTED`, damit das System keine Selektionsfarbe darüberlegt. Der Text bleibt lesbar, weil die Vordergrundfarbe normal bleibt. Die Selektion kann man zusätzlich markieren, zum Beispiel mit einem dunkleren Farbton, einem Rahmen (`gc.drawRectangle`) oder fettem Text. Sonst sieht man nicht mehr, welche Zeile gewählt ist.

2. **Selektion nur bei nicht eingefärbten Zeilen:** Der Listener greift nur bei Zeilen, die gerade eine Statusfarbe haben (Open mit laufender Restzeit). Alle anderen Zeilen behalten das normale Selektionsverhalten.

3. **Weniger Aufwand, aber plattformabhängig:** Die Selektion bleibt wie sie ist. Die Statusfarbe wird dann zum Beispiel nur auf einer Spalte oder als Rahmen gezeigt. Das ist weniger auffällig, aber unproblematisch.

Ich würde Variante 1 mit einem Rahmen oder einem dunkleren Ton als Selektionsmarkierung empfehlen. Das Blinken bleibt dann auch bei selektierter Zeile sichtbar, was bei der Session-Auswahl wichtig ist, weil die View die Selektion für die aktuelle Session nutzt.

Unter GTK kann `EraseItem` je nach Theme etwas anders wirken. Das müsstest du bei dir ausprobieren.

Soll ich Variante 1 umsetzen? Dann bräuchte ich noch deine Wahl der Selektionsmarkierung: Rahmen, dunklerer Farbton oder fetter Text.
Ich verstehe nicht. Das Problem ist das die Selektion die Farbe verdeckt. Kann man die Selectionfarbe ändern ohne den wichtigen Selektion state zu ändern?
Result Stats: id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, total: 22764, in: 16, out: 5437, reason: 165, read: 85649, write: 17311, start: 1794082352802, end: 1794082360474, time: 7602, total: 60422, io: 0, control: 70, sent: 1, recv: 134
SystemInit:  id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 165
Ja. Die Selektion als Zustand und ihre Darstellung sind getrennt. SWT hat keine API, die Selektionsfarbe einer `Table` zu ändern. Sie kommt vom System-Theme. Man kann aber beim Zeichnen eingreifen.

Im `SWT.EraseItem`-Listener entfernt man mit `e.detail &= ~SWT.SELECTED` das Flag. Das gilt nur für diesen einen Zeichenvorgang. Es sagt dem System: "Zeichne diese Zeile nicht mit der Selektionsfarbe." Danach füllt man den Hintergrund selbst mit der Statusfarbe.

Die eigentliche Selektion bleibt davon unberührt:
- `table.getSelection()` und `viewer.getStructuredSelection()` liefern weiter die gewählte Zeile.
- Der `SelectionChangedListener`, der `sessionManager.setSelectedSessionUuid(...)` aufruft, feuert wie bisher.
- Tastatur- und Mausbedienung bleiben gleich.

Es ändert sich nur das Aussehen. Die gewählte Zeile wäre deshalb nicht mehr als gewählt erkennbar, wenn man nichts ergänzt. Dafür braucht es eine eigene Markierung, etwa einen Rahmen um die Zeile oder einen dunkleren Farbton. Das gilt nur für Zeilen mit Statusfarbe. Alle anderen Zeilen behalten die normale Selektionsfarbe.

Soll ich das so umsetzen? Dann sag mir noch, ob Rahmen oder dunklerer Farbton.
Okay dann hätte ich gern das die Selection immer Hellgrau ist es sei denn die letzte Minute ist angebrochen dann soll es Hellgrau/Rot blinken.
SystemInit:  id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: b725-e-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: ClaudeCodeSessionView.rowBackground
    source: |2
        private boolean isLastMinute(CCSession s) {
          if (!SessionState.Open.equals(s.getState()))
            return false;
          long remaining = s.getRemainingTtlSeconds();
          return remaining >= 0 && remaining < 60;
        }

        private boolean isBlinkOn(CCSession s) {
          return s.getRemainingTtlSeconds() % 2 == 0;
        }

        private Color rowBackground(CCSession s) {
          if (!SessionState.Open.equals(s.getState()) || s.getRemainingTtlSeconds() < 0)
            return null;
          Display display = viewer.getControl().getDisplay();
          boolean red = isLastMinute(s) && isBlinkOn(s);
          return display.getSystemColor(red ? SWT.COLOR_RED : SWT.COLOR_YELLOW);
        }

        private Color selectionBackground(CCSession s) {
          boolean red = isLastMinute(s) && isBlinkOn(s);
          return red ? viewer.getControl().getDisplay().getSystemColor(SWT.COLOR_RED) : selectionGray;
        }

        private void paintSelection(Event e) {
          if ((e.detail & SWT.SELECTED) == 0 || !(e.item.getData() instanceof CCSession))
            return;
          GC gc = e.gc;
          gc.setBackground(selectionBackground((CCSession) e.item.getData()));
          gc.setForeground(gc.getDevice().getSystemColor(SWT.COLOR_BLACK));
          gc.fillRectangle(e.x, e.y, e.width, e.height);
          e.detail &= ~SWT.SELECTED;
        }
  reason: "Selektionsfarbe (Hellgrau, in letzter Minute Hellgrau/Rot blinkend) als eigene Zeichenlogik ergänzen"
```
/answer b725-e-0 allow
ReasoningToken: 828
Control Request:
```yaml
id: b725-f-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: ClaudeCodeSessionView.createPartControl
    old_text: '    table.setLinesVisible(true);'
    new_text: |2-
          table.setLinesVisible(true);
          selectionGray = new Color(table.getDisplay(), 211, 211, 211);
          table.addListener(SWT.EraseItem, this::paintSelection);
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: ClaudeCodeSessionView.dispose
    old_text: '    super.dispose();'
    new_text: |2-
          if (selectionGray != null)
            selectionGray.dispose();
          super.dispose();
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: ClaudeCodeSessionView.tVJW11|RHWM5g
    old_text: private long lastRefreshTime = 0;
    new_text: |-
      private long lastRefreshTime = 0;
        private Color selectionGray;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: N7nmJS|xLvV7Z
    old_text: import org.eclipse.swt.graphics.Color;
    new_text: |-
      import org.eclipse.swt.graphics.Color;
      import org.eclipse.swt.graphics.GC;
      import org.eclipse.swt.widgets.Event;
  reason: "Listener registrieren, Hellgrau-Farbe anlegen/freigeben, Imports ergänzen"
```
/answer b725-f-0 allow
Control Request:
```yaml
id: b725-10-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  reason: Syntax nach den Änderungen prüfen
```
/answer b725-10-0 allow
Control Request:
```yaml
id: b725-11-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
    pattern: ^import .*\.(Event|GC|Color);
    include:
    - ClaudeCodeSessionView.java
    exclude:
    - bin/**
  reason: "Prüfen, dass keine doppelten Imports entstanden sind"
```
/answer b725-11-0 allow
Control Request:
```yaml
id: b725-12-11
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view
      matches:
      - path: ClaudeCodeSessionView.java
        matches:
        - import org.eclipse.swt.graphics.Color;
        - import org.eclipse.swt.graphics.GC;
        - import org.eclipse.swt.widgets.Event;
```
/answer b725-12-11 allow
Result Stats: id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, total: 30662, in: 22, out: 8794, reason: 0, read: 142303, write: 21846, start: 1794082487634, end: 1794082492218, time: 4524, total: 80057, io: 0, control: 60, sent: 0, recv: 119
Die selektierte Zeile ist jetzt hellgrau. In der letzten Minute blinkt sie zwischen Hellgrau und Rot. Die Datei hat die Syntaxprüfung bestanden, aber ich habe sie nicht gebaut oder in Eclipse ausprobiert.

- **Zeichnung:** Ein `SWT.EraseItem`-Listener (`paintSelection`) füllt selektierte Zeilen selbst und entfernt für diesen Zeichenvorgang das Flag `SWT.SELECTED`. Die eigentliche Selektion bleibt unverändert. `viewer.getStructuredSelection()` und der `SelectionChangedListener` mit `setSelectedSessionUuid` funktionieren wie vorher.
- **Blinktakt:** Rot erscheint bei geraden Restsekunden. Bei Zeilen, die nicht selektiert sind, ist der Takt derselbe (Rot/Gelb).
- **Textfarbe:** In der selektierten Zeile ist sie schwarz, damit sie auf dem hellen Grund lesbar bleibt.
- **Nicht selektierte Zeilen:** Sie sehen aus wie vorher (Gelb bei laufender Restzeit, in der letzten Minute Rot/Gelb blinkend).
- **Farbe Hellgrau:** Das ist ein eigenes `Color`-Objekt (211, 211, 211). Es wird in `dispose()` freigegeben.

Falls der Text unter GTK in der selektierten Zeile trotzdem weiß erscheint, schau dir das einmal an. Das Theme könnte die Vordergrundfarbe überschreiben, und dann müsste ich zusätzlich in `SWT.PaintItem` eingreifen.
Ein Fehler, die Selection (Hellgrau), hat vorrang vor Gelb aber nicht vor dem Blinkeffekt.
SystemInit:  id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 763
Control Request:
```yaml
id: b725-13-0
toolName: ask_user
arguments:
  question: "Ich bin unsicher, was du siehst und erwartest. Aktuell zeigt eine selektierte Zeile in der letzten Minute Hellgrau/Rot im Wechsel. Was passiert bei dir genau, und was soll stattdessen passieren? (a) Selektierte Zeile in der letzten Minute soll wie nicht selektierte blinken, also Rot/Gelb. (b) Es soll Hellgrau/Rot bleiben, aber bei dir ist die Zeile in der letzten Minute dauerhaft grau oder anders gefärbt. (c) Etwas anderes."
  reason: "Die Aussage \"Selection hat Vorrang vor Gelb, aber nicht vor dem Blinkeffekt\" lässt zwei Lesarten zu, die zu unterschiedlichem Code führen."
```
/answer b725-13-0 allow Okay, ich habe deine letzte änderung getestet. Eine amrkierte Zeile ist grau es sei denn sie ist open mit laufzeit, dann ist sie gelb (sollte aber doch grau sein). Gelb open gilt nur für nicht selektierte Zeilen. In der letzen Minute Blinkt eine selektierte Zeile Rot/Gelb statt Rot/Grau. Nur nicht selektierte Zeilen blinken in der letzen Minute Rot/Gelb
Control Request:
```yaml
id: b725-14-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: ClaudeCodeSessionView.rowBackground
    old_text: |2-
          if (!SessionState.Open.equals(s.getState()) || s.getRemainingTtlSeconds() < 0)
            return null;
    new_text: |2-
          if (viewer.getStructuredSelection().toList().contains(s))
            return selectionBackground(s);
          if (!SessionState.Open.equals(s.getState()) || s.getRemainingTtlSeconds() < 0)
            return null;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
    id: ClaudeCodeSessionView.createPartControl
    old_text: |2-
            IStructuredSelection sel = viewer.getStructuredSelection();
            if (sel.isEmpty())
              sessionManager.setSelectedSessionUuid(null);
    new_text: |2-
            viewer.refresh();
            IStructuredSelection sel = viewer.getStructuredSelection();
            if (sel.isEmpty())
              sessionManager.setSelectedSessionUuid(null);
  reason: "Selektierte Zeilen im LabelProvider selbst hellgrau/rot färben, damit Gelb sie nicht überdeckt; Neuzeichnen bei Selektionswechsel"
```
/answer b725-14-0 allow
ReasoningToken: 758
Control Request:
```yaml
id: b725-15-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  reason: Syntax prüfen
```
/answer b725-15-0 allow
Result Stats: id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, total: 37806, in: 28, out: 12013, reason: 0, read: 210565, write: 25765, start: 1794082852706, end: 1794082860463, time: 7690, total: 107062, io: 0, control: 67, sent: 0, recv: 125
Ich habe den Fehler behoben. Die Datei hat die Syntaxprüfung bestanden, ausprobiert habe ich es nicht.

**Ursache:** Der `LabelProvider` hat der Zeile die Hintergrundfarbe Gelb oder Rot/Gelb gesetzt. Diese Farbe wurde über das Hellgrau gezeichnet, das mein `EraseItem`-Listener vorher gefüllt hat. Der Listener allein reicht also nicht.

**Änderungen:**
- `rowBackground(...)` prüft jetzt zuerst, ob die Zeile selektiert ist. Dann liefert es direkt `selectionBackground(...)` zurück, also Hellgrau und in der letzten Minute im Wechsel Rot. Gelb gilt damit nur noch für nicht selektierte Zeilen.
- Im `SelectionChangedListener` steht jetzt `viewer.refresh()` als erste Zeile. Dadurch wechselt die Farbe sofort beim Markieren oder Abwählen. Sonst würde es bis zum nächsten Sekundentakt dauern.

**Ergebnis:**
- **Selektiert:** Die Zeile ist Hellgrau. In der letzten Minute wechselt sie zwischen Hellgrau und Rot.
- **Nicht selektiert:** Die Zeile ist Gelb. In der letzten Minute wechselt sie zwischen Gelb und Rot.

Falls beim Klicken etwas flackert oder die Selektion springt, liegt es am `viewer.refresh()` im Listener. Sag Bescheid, dann ersetze ich es durch ein gezieltes `viewer.update(...)` der betroffenen Zeilen.
Ok, lass mich das testen
Result Stats: id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, total: 38702, in: 30, out: 12262, reason: 0, read: 236330, write: 26410, start: 1794083006872, end: 1794083009793, time: 2868, total: 109930, io: 0, control: 53, sent: 1, recv: 60
SystemInit:  id=8b5f9449-0149-4fe8-b0a6-d316805a3c50, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Gern. Wenn du etwas Auffälliges siehst, schreib mir einfach, was dir aufgefallen ist. Mögliche Stellen wären:

- **Flackern oder springende Selektion beim Klicken:** Das liegt wahrscheinlich am `viewer.refresh()` im `SelectionChangedListener`.
- **Weißer Text auf der grauen Zeile:** Das GTK-Theme überschreibt dann die Vordergrundfarbe, und ich müsste zusätzlich in `SWT.PaintItem` eingreifen.
- **Blinken nicht im Sekundentakt:** Dann bremst `REFRESH_DEBOUNCE_MS` den Refresh aus.
