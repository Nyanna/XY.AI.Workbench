# Prompt: Fixed-Width-Verhalten für Eclipse-Workbench-Views (e4)

## Kontext

Ich entwickle ein Eclipse-Plugin (RCP/e4-Workbench, kein reiner SWT-`SashForm`-Ansatz). Beim Resizen des Fensters skalieren aktuell alle Views proportional mit — ich möchte, dass bestimmte Views (z.B. der Project Explorer) ihre **Pixelbreite behalten**, während die Editor-Area den gesamten übrigen Platz absorbiert.

## Hintergrund / bereits bekannte Fakten (nicht erneut recherchieren)

- Es gibt **keine High-Level-API** (`IPageLayout.setFixed()` verhindert nur User-Drag&Drop/Zoom, nicht proportionales Resize-Verhalten bei Fenstergrößenänderung).
- Views/Editor-Area hängen im e4-Modell unter einem `MPartSashContainer`, dessen Kinder (`MPartSashContainerElement`) ein `containerData`-Gewicht tragen (String, analog zu `SashForm.setWeights`). Der `SashLayout` verteilt bei Resize proportional nach diesen Gewichten.
- Zwei mögliche Lösungswege wurden identifiziert:
  1. **Renderer-Austausch** über `org.eclipse.core.runtime.products`-Extension-Point mit Property `rendererFactoryUri`, die eine eigene `IRendererFactory` registriert (ersetzt `WorkbenchRendererFactory`), um `MPartSashContainer` mit einem eigenen `SashRenderer`/`SashLayout` zu rendern, der Fixed-Width-Logik direkt im Layout-Algorithmus umsetzt.
  2. **Add-on mit e4-Event-Bus** (leichtgewichtiger, bevorzugt für ersten Wurf): `@Inject @Optional void handler(@UIEventTopic(UIEvents.UIElement.TOPIC_WIDTH) Event e, ...)`, das nach jeder Breitenänderung `containerData` der fixierten View auf einen gecachten Pixelwert zurücksetzt und der Editor-Area (bzw. dem wachsenden Geschwisterknoten) die Restbreite zuweist.
- Es gibt keine Eclipse-Preference/Setting, das dieses Verhalten umschaltet.

## Aufgabe

Implementiere **Variante 2 (Add-on mit `@UIEventTopic`)** vollständig, produktionsreif, als Eclipse-Plugin-Bestandteil:

### 1. Modell-Traversierung

- Eine Methode, die den e4-Modellbaum ab `MWindow`/`MPerspective` rekursiv durchläuft und für Debug-Zwecke ausgibt: `MUIElement`-Typ, `elementId`, `containerData`, `tags`, verschachtelt eingerückt nach Tiefe.
- Eine Hilfsmethode `containsElementId(MUIElement, String)`, die prüft, ob eine gegebene `elementId` (z.B. Project Explorer) irgendwo unterhalb eines `MUIElement`-Knotens vorkommt (auch in verschachtelten `MPartStack`s).

### 2. Fixed-Width-Logik

- Konfigurierbare Liste von `elementId`s, die fix bleiben sollen (Default: `"org.eclipse.ui.navigator.ProjectExplorer"`). Alles andere, insbesondere die Editor-Area (`IPageLayout.ID_EDITOR_AREA`), soll frei skalieren.
- Beim ersten Event: Pixelbreite jedes zu fixierenden Kindes aus der aktuellen `Composite`-Breite cachen (nicht aus `containerData`, da das nur ein relatives Gewicht ist).
- Bei jedem `TOPIC_WIDTH`-Event (und optional `TOPIC_HEIGHT`, falls die Sash vertikal orientiert ist — `isHorizontal()`-Fall beider Achsen berücksichtigen):
  - Finde die gemeinsame `MPartSashContainer`-Elternebene von fixierter View und Editor-Area.
  - Berechne: `total = clientArea.width` (bzw. `.height`), `fixedSum = Summe der gecachten Fixed-Breiten`.
  - Setze `containerData` der fixierten Kinder auf ihre gecachten Werte, `containerData` des wachsenden Geschwisterknotens (der die Editor-Area enthält) auf `total - fixedSum` (unter Berücksichtigung der Sash-Breite zwischen den Elementen).
  - Wichtig: **nur die eine betroffene Sash-Ebene behandeln**, nicht den gesamten Baum — andere verschachtelte Sashes bleiben unangetastet.
- Guard gegen rekursive Trigger-Schleifen (das Setzen von `containerData` löst selbst wieder ein Layout-Event aus).

### 3. Registrierung

- Als e4-Add-on (`org.eclipse.e4.workbench.model`-Extension-Point mit `<processor>` oder Application-Model-Fragment mit `<addon>`), damit die Klasse automatisch instanziiert und per DI (`@Inject`) mit `EModelService`, `MWindow`/`MApplication` versorgt wird.
- Kein manuelles Hinzufügen eines `ControlListener` auf SWT-Ebene nötig — die Injection des `@UIEventTopic`-Handlers soll über den e4-Eventbus laufen.

### 4. Bekannte Einschränkung, die dokumentiert werden soll

- Liegt die zu fixierende View zusammen mit anderen Views in einem gemeinsamen `MPartStack` (Tab-Gruppe), wird die gesamte Stack-Breite fixiert, nicht nur der sichtbare Tab — das ist eine Eigenschaft des Sash-Modells (Gewichte gelten pro Sash-Kind, nicht pro Part innerhalb eines Stacks) und keine Einschränkung der Implementierung.

## Gewünschte Ausgabe

- Vollständiger, kompilierbarer Java-Code (Add-on-Klasse + ggf. Hilfsklassen).
- Kurze Erklärung, wie der Extension-Point-Eintrag (`plugin.xml` bzw. `Application.e4xmi`-Fragment) für die Add-on-Registrierung aussehen muss.
- Hinweis, an welcher Stelle die konfigurierbare Liste der fixen `elementId`s im Code anzupassen ist.
