Ich verwende in meinem Eclipse-Plug einen `org.eclipse.jface.text.rules.RuleBasedScanner` Scanner für Syntax-Highlighting. Bei großen Dateien ist dieser Ansatz zu langsam, es fehlt wohl eine Regionsoptimierung. 
#: SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=10f52522-82a8-474f-bf9e-f7ab0deff090 | model=claude-sonnet-5 | plugins=
#: ReasoningToken: 2436
Kurz vorab zwei Rückfragen, die die Antwort schärfer machen würden – aber ich gebe dir schon mal die üblichen Verdächtigen und Lösungsansätze, die in JFace-Text-basiertem Highlighting fast immer die Ursache sind:

- Nutzt du `PresentationReconciler` mit `DefaultDamagerRepairer` (bzw. `RuleBasedDamagerRepairer`)?
- Arbeitest du schon mit einer `IDocumentPartitioner` (z. B. `FastPartitioner`) oder scannt dein `RuleBasedScanner` den **gesamten Dokumenttext** in einer Partition?

## Warum es meist "zu viel" scannt

`RuleBasedScanner.setRange(document, offset, length)` unterstützt zwar bereits Bereichs-Scans – das Problem liegt fast nie im Scanner selbst, sondern darin, **welcher Bereich** ihm übergeben wird. Das steuert der `IPresentationDamager`:

```java
IRegion getDamageRegion(ITypedRegion partition, DocumentEvent event, boolean documentPartitioningChanged);
```

Wenn hier zu großzügig (oder im Extremfall "ganzes Dokument") zurückgegeben wird, scannt dein `RuleBasedScanner` bei jedem Tastendruck riesige Mengen Text neu – das ist typischerweise der eigentliche Performance-Killer, nicht die Regeln selbst.

## Die wichtigsten Hebel

### 1. Dokument-Partitionierung einführen (größter Effekt)

Wenn du bisher **eine** Partition für die ganze Datei hast, muss bei jeder Änderung potenziell der gesamte Kontext neu bewertet werden (z. B. "bin ich noch im Kommentar?"). Mit `FastPartitioner` + `IPredicateRule`s (Comments, Strings, Code-Blöcke als eigene Content-Types) wird nur die **betroffene Partition** neu berechnet:

```java
FastPartitioner partitioner = new FastPartitioner(
    new MyPartitionScanner(),
    LEGAL_CONTENT_TYPES);
partitioner.connect(document);
document.setDocumentPartitioner(partitioner);
```

Dann pro Content-Type einen eigenen `RuleBasedScanner` + Damager/Repairer im `PresentationReconciler` registrieren:

```java
reconciler.setDamager(dr, IDocument.DEFAULT_CONTENT_TYPE);
reconciler.setRepairer(dr, IDocument.DEFAULT_CONTENT_TYPE);
reconciler.setDamager(commentDr, MY_COMMENT_PARTITION);
reconciler.setRepairer(commentDr, MY_COMMENT_PARTITION);
```

### 2. Mehrzeilige Regeln ohne klares Ende vermeiden

Ein häufiger Grund, warum die "Beschädigung" (Damage) bis ans Dateiende propagiert: `PatternRule`/`MultiLineRule` ohne sauberes `endSequence` oder mit `breaksOnEOF = true` in Kombination mit nicht erkanntem Zeilenende. Dann muss der Reconciler im Zweifel alles ab der Änderung bis zum Ende neu scannen. Prüfe:

- Ist die Endsequenz eindeutig erkennbar?
- Wird bei `FastPartitioner` korrekt erkannt, dass eine Partitionsgrenze sich verschoben hat (`documentPartitioningChanged`)?

### 3. `DefaultDamagerRepairer` wirklich nutzen (nicht selbst "zu breit" überschreiben)

Falls du eine eigene `IPresentationDamager`-Implementierung hast, die z. B. immer `new Region(0, document.getLength())` zurückgibt – das ist der Klassiker für "funktioniert, aber lahm bei großen Dateien". `DefaultDamagerRepairer` erweitert den Schaden normalerweise nur auf Zeilengrenzen der betroffenen Partition, nicht auf das ganze Dokument.

### 4. Regelreihenfolge/-wahl im `RuleBasedScanner`

Weniger relevant als 1–3, aber spürbar bei sehr langen Zeilen/Dateien:

- Häufigste/billigste Regeln zuerst (`WhitespaceRule`, einfache `WordRule` mit `IWordDetector`)
- Teure `PatternRule`/Regex-basierte Regeln möglichst zuletzt bzw. auf enge Partitionen beschränken
- `WordRule` mit `HashMap`-Lookup statt vielen einzelnen `PatternRule`s für Keywords

### 5. Falls das alles schon stimmt: Throttling/Async

`PresentationReconciler` reconciled standardmäßig synchron im UI-Thread bei jedem `DocumentEvent`. Bei wirklich sehr großen Dateien kann es sinnvoll sein, eine eigene `IPresentationReconcilingStrategy` mit Debounce (z. B. über `Job`/`UIJob` mit kurzer Verzögerung, ähnlich wie es Reconciler für Fehlerprüfung machen) zu bauen, statt bei jedem Tastendruck sofort zu reagieren.

---

Wenn du mir zeigst, wie dein `IPresentationDamager` (bzw. ob du `DefaultDamagerRepairer` direkt verwendest) und deine Partitionierung aktuell aussehen, kann ich gezielt sagen, wo genau der unnötig große Bereich entsteht.
Versuchen wir mal das billigste, an welcher Stelle kann ich das Scanning verhindern, wenn die Eingabemenge ein Limit überschreitet?
#: SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=10f52522-82a8-474f-bf9e-f7ab0deff090 | model=claude-sonnet-5 | plugins=
#: ReasoningToken: 1392
Der günstigste Punkt ist **nicht** im Scanner selbst, sondern **vor der Installation des Reconcilers** – dann fällt der komplette Overhead (DocumentListener, Damager/Repairer, Scanner) weg, statt bei jeder Änderung erneut geprüft zu werden.

## Variante 1 (empfohlen): Reconciler gar nicht erst installieren

In deiner `SourceViewerConfiguration`:

```java
private static final int MAX_HIGHLIGHT_LENGTH = 500_000; // Zeichen, anpassen

@Override
public IPresentationReconciler getPresentationReconciler(ISourceViewer sourceViewer) {
    IDocument document = sourceViewer.getDocument();
    if (document != null && document.getLength() > MAX_HIGHLIGHT_LENGTH) {
        return null; // kein Highlighting für riesige Dateien
    }
    return super.getPresentationReconciler(sourceViewer);
}
```

`SourceViewer.configure(SourceViewerConfiguration)` fragt diese Methode ab und installiert den Reconciler nur, wenn sie nicht `null` liefert. Damit wird **kein** `IDocumentListener` registriert, es gibt keine Damage-Berechnung, keinen Scan – der billigstmögliche Zustand.

⚠️ Achtung: Das ist eine einmalige Prüfung beim Öffnen/Konfigurieren. Wächst eine zunächst kleine Datei später über das Limit (z. B. durch Paste), greift das nicht automatisch – dafür müsstest du bei Bedarf `sourceViewer.unconfigure()`/`configure()` erneut aufrufen, wenn du das überwachen willst.

## Variante 2: Feingranularer, falls du nicht alles-oder-nichts willst

Falls du z. B. weiterhin die ersten X Zeilen highlighten willst oder dynamisch reagieren musst, ist der nächstgünstige Punkt der `IPresentationDamager`:

```java
@Override
public IRegion getDamageRegion(ITypedRegion partition, DocumentEvent event, boolean partitioningChanged) {
    if (event.getDocument().getLength() > MAX_HIGHLIGHT_LENGTH) {
        return null; // kein Schaden -> Repairer wird nicht aufgerufen
    }
    return super.getDamageRegion(partition, event, partitioningChanged);
}
```

Das ist zwar pro Edit eine Prüfung (billig – ein `getLength()`-Vergleich), aber der `DocumentListener` bleibt aktiv, nur die eigentliche Repair-Arbeit entfällt.

## Variante 3 (nicht empfohlen als "billigste" Lösung)

Ein Guard direkt in `RuleBasedScanner.setRange()` funktioniert auch, ist aber die schwächste Option, da Reconciler + Damager weiterhin pro Edit durchlaufen werden und nur der letzte Schritt (das eigentliche Scannen) übersprungen wird.

**Fazit:** Wenn "billig" wörtlich gemeint ist (minimaler Laufzeit-Overhead insgesamt), nimm **Variante 1** – sie verhindert das Problem an der Wurzel, statt es pro Tastendruck wegzuprüfen.
/resume 10f52522-82a8-474f-bf9e-f7ab0deff090
`getDamageRegion` wird nicht aufgerufen.
Session created
#: SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=10f52522-82a8-474f-bf9e-f7ab0deff090 | model=claude-sonnet-5 | plugins=
#: ReasoningToken: 3713
Das erklärt vermutlich auch dein ursprüngliches Performance-Problem: Wenn `getDamageRegion` nie aufgerufen wird, läuft irgendwo eine **Voll-Neuberechnung** statt des inkrementellen Damage/Repair-Mechanismus – genau die fehlende "Regionsoptimierung", von der du eingangs sprachst.

## Wann `getDamageRegion` normalerweise aufgerufen wird

Nur durch den **internen `IDocumentListener`**, den `PresentationReconciler.install(viewer)` beim Aufruf registriert – ausgelöst durch echte `DocumentEvent`s (Tippen, Einfügen, Löschen). Wichtig:

- Beim **ersten Öffnen** der Datei wird die initiale Hervorhebung **nicht** über den Damager berechnet, sondern direkt per `createPresentation()` für das gesamte Dokument. Das ist normales Verhalten, kein Bug – wenn du nur geöffnet, aber nichts editiert hast, ist "nicht aufgerufen" also erwartbar.
- Wenn aber auch **beim Tippen** `getDamageRegion` nie feuert, läuft etwas an der Standard-Mechanik vorbei.

## Wahrscheinliche Ursachen, wenn es auch bei Edits nicht feuert

1. **Es wird gar nicht der von dir überschriebene Damager verwendet** – z. B. weil `reconciler.setDamager(...)` mit einer anderen Instanz aufgerufen wird als der, die du angepasst hast (Kopier-/Cache-Fehler in `SourceViewerConfiguration`).
2. **Jemand ruft manuell `invalidateTextPresentation()`** oder einen eigenen `IDocumentListener`, der direkt `repairer.createPresentation(presentation, new Region(0, doc.getLength()))` für das ganze Dokument aufruft – das würde exakt euer Symptom erklären: volle Neuberechnung ohne je den Damager zu fragen.
3. **Reconciler ist gar nicht installiert** (`getPresentationReconciler()` liefert z. B. durch einen Bug immer `null` oder eine neue, nie installierte Instanz).
4. **Eigene `ITokenScanner`/Presentation-Logik** außerhalb von `PresentationReconciler` – manche Plugins bauen eigene Reconciling-Strategien, die den Standard-Damager komplett umgehen.

## Um das einzugrenzen

Kannst du mir zeigen, wie genau Reconciler/Damager/Repairer bei dir verdrahtet sind? Konkret:

- Der Code in `getPresentationReconciler(ISourceViewer)`
- Ob es einen eigenen `IDocumentListener` oder Aufrufe von `invalidateTextPresentation()` irgendwo im Editor/Plugin gibt

Damit finden wir die Stelle, an der tatsächlich das ganze Dokument gescannt wird – dort ist dann auch der richtige (und wirklich billige) Ort für die Größenprüfung, nicht in `getDamageRegion`, wenn diese Methode sowieso nie den relevanten Pfad darstellt.


Kann ich `createPresentation` überschreiben? Was muss ich zurückgeben?
#: SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=10f52522-82a8-474f-bf9e-f7ab0deff090 | model=claude-sonnet-5 | plugins=
#: ReasoningToken: 1965
Ja, `createPresentation` ist Teil des `IPresentationRepairer`-Interfaces (implementiert z. B. von `DefaultDamagerRepairer`/`RuleBasedDamagerRepairer`) und lässt sich problemlos überschreiben.

## Signatur

```java
void createPresentation(TextPresentation presentation, ITypedRegion region);
```

**Wichtig:** Die Methode ist `void` – es gibt keinen Rückgabewert. Das Ergebnis wird als **Seiteneffekt** auf das übergebene `presentation`-Objekt geschrieben, üblicherweise über:

```java
presentation.addStyleRange(new StyleRange(offset, length, foreground, background, style));
// oder
presentation.mergeStyleRange(styleRange);
```

## Warum du hier nicht einfach "nichts tun" solltest

Wenn du bei Größenüberschreitung komplett leer zurückkehrst, bekommt `presentation` **keine** StyleRange für den betroffenen Bereich – das kann je nach Aufrufer zu Inkonsistenzen führen (z. B. alte Styles bleiben stehen oder es flackert). Sauberer ist es, **eine einzige, billige StyleRange** mit dem Default-Attribut für den kompletten Bereich zu setzen – ganz ohne den teuren Scanner-Durchlauf:

```java
public class LimitedDamagerRepairer extends DefaultDamagerRepairer {

    private static final int MAX_LENGTH = 500_000; // anpassen

    public LimitedDamagerRepairer(ITokenScanner scanner, TextAttribute defaultTextAttribute) {
        super(scanner, defaultTextAttribute);
    }

    @Override
    public void createPresentation(TextPresentation presentation, ITypedRegion region) {
        if (fDocument != null && fDocument.getLength() > MAX_LENGTH) {
            // keine Rule-Auswertung, nur ein Default-Style für den ganzen Bereich
            addRange(presentation, region.getOffset(), region.getLength(), fDefaultTextAttribute);
            return;
        }
        super.createPresentation(presentation, region);
    }
}
```

`addRange(...)` ist bei `DefaultDamagerRepairer` `protected` und daher aus der Subklasse aufrufbar; `fDocument`/`fDefaultTextAttribute` ebenso als geschützte Felder vorhanden.

## Warum das die eigentliche Lösung für dein Symptom ist

Da bei dir `getDamageRegion` scheinbar gar nicht in den kritischen Pfad eingebunden ist (egal ob durch manuellen Voll-Scan-Aufruf oder Initial-Highlighting), ist `createPresentation` der **einzige garantiert durchlaufene Punkt**, bevor der Scanner tatsächlich über den Text läuft – unabhängig davon, wer die Methode aufruft (Reconciler-intern oder eigener Code). Das macht ihn zum robustesten Ort für den Größen-Guard, wenn du nicht sicher weißt, welcher Aufrufpfad bei dir aktiv ist.

**Tipp:** Setz an dieser Stelle testweise einen Breakpoint/Log-Ausdruck – so siehst du sofort, wie oft und mit welcher `region.getLength()` die Methode tatsächlich aufgerufen wird, und kannst bestätigen, ob hier wirklich der Vollscan passiert.

Mein Eingreifen funktioniert gut aber ich bin nicht das einzige Problem. Der Ganze Stack und die Basisimplementierung scheint ungeeignet für diese Dateigröße zu sein:

Thread [main] (Suspended)	
	OS.g_utf16_pointer_to_offset(long, long) line: not available [native method]	
	52 collapsed frames	
		TextLayout.computeRuns() line: 326	
		TextLayout.getBounds(int, int) line: 939	
		StyledText.getTextBounds(int, int) line: 4960	
		JFaceTextUtil.computeLineHeight(StyledText, int) line: 65	
		OverviewRuler$WidgetInfos.<init>(StyledText, Canvas) line: 361	
		OverviewRuler.doPaint(GC) line: 805	
		OverviewRuler.doPaint(GC, int, int) line: 721	
		OverviewRuler.doubleBufferPaint(GC) line: 706	
		OverviewRuler.lambda$1(PaintEvent) line: 604	
		Lambda.paintControl(PaintEvent) line: not available	
		TypedListener.handleEvent(Event) line: 272	
		EventTable.sendEvent(Event) line: 91	
		Display.sendEvent(EventTable, Event) line: 5865	
		Canvas(Widget).sendEvent(Event) line: 1656	
		Canvas(Widget).sendEvent(int, Event, boolean) line: 1682	
		Canvas(Widget).sendEvent(int, Event) line: 1665	
		Canvas(Control).gtk_draw(long, long) line: 3895	
		Canvas(Scrollable).gtk_draw(long, long) line: 394	
		Canvas(Composite).gtk_draw(long, long) line: 507	
		Canvas.gtk_draw(long, long) line: 173	
		Canvas(Widget).windowProc(long, long, long) line: 2629	
		Canvas(Control).windowProc(long, long, long) line: 6849	
		Display.windowProc(long, long, long) line: 6172	
		GTK3.gtk_main_do_event(long) line: not available [native method]	
		Display.eventProc(long, long) line: 1621	
		GTK3.gtk_main_iteration_do(boolean) line: not available [native method]	
		Display.readAndDispatch() line: 4512	

#: SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=10f52522-82a8-474f-bf9e-f7ab0deff090 | model=claude-sonnet-5 | plugins=
#: ReasoningToken: 3745
Guter Fund – das ist tatsächlich ein bekanntes strukturelles Problem, das nichts mehr mit deinem `RuleBasedScanner` zu tun hat, sondern mit dem **Overview Ruler** (die Miniatur-Leiste rechts neben dem Scrollbalken).

## Was hier passiert

`OverviewRuler.doPaint()` baut bei jedem Repaint über `WidgetInfos` eine Zuordnung von Annotationen/Modellzeilen zu Pixel-Positionen auf. Dafür ruft `JFaceTextUtil.computeLineHeight()` pro betroffener Zeile `StyledText.getTextBounds()` auf – und das landet in `TextLayout.computeRuns()`, also einem **nativen GTK-Textlayout-Aufruf** (Shaping, Bidi, Glyph-Berechnung). Das ist:

- pro Zeile relativ teuer (native Aufrufe, kein gecachtes Layout für nicht sichtbare Zeilen),
- wird bei **jedem Repaint** der Ruler-Leiste neu berechnet (Scrollen, Resize, Annotation-Änderungen, sogar Fokuswechsel),
- skaliert schlecht mit der Anzahl der Annotationen bzw. Zeilen, die der Ruler abbilden muss.

Das ist eine bekannte Schwachstelle von JFace/SWT bei sehr großen Dateien – es gibt dafür keine "Regionsoptimierung" wie beim Scanner, weil der Ruler grundsätzlich versucht, das gesamte Dokument proportional darzustellen.

## Cheapster Fix: Overview Ruler für große Dateien deaktivieren

Analog zu deinem bisherigen Ansatz – einfach **nicht erzeugen**, statt zu optimieren:

```java
@Override
protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {
    IDocument document = getDocumentProvider() != null
        ? getDocumentProvider().getDocument(getEditorInput())
        : null;

    boolean tooLarge = document != null && document.getNumberOfLines() > MAX_LINES; // Schwelle wählen

    IOverviewRuler overviewRuler = tooLarge ? null : createOverviewRuler(getSharedColors());

    ISourceViewer viewer = new SourceViewer(parent, ruler, overviewRuler,
        !tooLarge /* showAnnotationsOverview */, styles);

    getSourceViewerDecorationSupport(viewer);
    return viewer;
}
```

`SourceViewer` akzeptiert `null` als Overview-Ruler-Parameter – dann wird `doPaint()`/`WidgetInfos` gar nicht erst instanziiert, und die teuren `TextLayout`-Aufrufe entfallen komplett.

## Alternativen, falls du den Ruler behalten willst

1. **Annotationen reduzieren**: Falls dein Plugin viele Annotationen pro Zeile erzeugt (z. B. eigene Marker statt reinem `StyleRange`-Highlighting), ist das ein Multiplikator für die Anzahl teurer `computeLineHeight`-Aufrufe. Prüfen, ob du Highlighting wirklich über Annotationen statt über den Presentation-Mechanismus machst.
2. **Zeilenumbruch (Word Wrap) deaktivieren**, falls aktiv – variable Zeilenhöhen zwingen SWT zu echtem Layout statt fixer Zeilenhöhe (`fixedLineHeight`-Fastpath entfällt dann).
3. **Preference abschalten**: Nutzer können es selbst über *Window → Preferences → General → Editors → Text Editors → "Show overview ruler"* deaktivieren – für dich als Plugin-Autor aber nicht deterministisch genug, wenn du es an Dateigröße koppeln willst.

## Kurze Rückfrage

Erbt dein Editor von `TextEditor`/`AbstractDecoratedTextEditor`, oder hast du eine eigene `SourceViewer`-Erzeugung? Dann sage ich dir die exakte Override-Stelle für dein Setup – die Struktur variiert leicht je nach Basisklasse.


Ja einen TextEditor. Gar nicht erst erzeugen, deckt nicht den Fall ab das die Datei wächst. Daher brauche ich dynamische Abschaltungen oder Hook Methoden die, die Verarbeitung übergehen können.
#: SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=10f52522-82a8-474f-bf9e-f7ab0deff090 | model=claude-sonnet-5 | plugins=
#: ReasoningToken: 5779
Genau richtig erkannt – dafür gibt es eine Laufzeit-Hook-Methode, du musst den `SourceViewer` nicht neu bauen.

## Dynamisches Ein-/Ausschalten des Overview Rulers

`org.eclipse.jface.text.source.SourceViewer` bietet dafür direkt:

```java
public void showAnnotationsOverview(boolean show)
```

Das erzeugt/entfernt zur Laufzeit den `Control` des Overview Rulers (inkl. Relayout), ohne dass du den ganzen `SourceViewer` neu instanziieren musst. Genau der Hook, den du suchst.

## Einbindung: Größe laufend überwachen

```java
private boolean fOverviewRulerCurrentlyVisible = true;
private static final int MAX_LINES = 50_000; // Schwelle anpassen

@Override
protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {
    ISourceViewer viewer = super.createSourceViewer(parent, ruler, styles);

    viewer.getDocument().addDocumentListener(new IDocumentListener() {
        @Override
        public void documentChanged(DocumentEvent event) {
            updateOverviewRulerVisibility(event.getDocument());
        }
        @Override
        public void documentAboutToBeChanged(DocumentEvent event) { }
    });

    return viewer;
}

private void updateOverviewRulerVisibility(IDocument document) {
    boolean shouldShow = document.getNumberOfLines() <= MAX_LINES;
    if (shouldShow != fOverviewRulerCurrentlyVisible) {
        fOverviewRulerCurrentlyVisible = shouldShow;
        if (getSourceViewer() instanceof SourceViewer sourceViewer) {
            sourceViewer.showAnnotationsOverview(shouldShow);
        }
    }
}
```

Wichtig dabei:

- **Zustand cachen** (`fOverviewRulerCurrentlyVisible`), damit du nicht bei jedem Tastendruck unnötig `dispose()`/`createControl()` auslöst, sondern nur beim tatsächlichen Über-/Unterschreiten der Schwelle.
- Läuft der `documentChanged`-Callback im UI-Thread (Standard bei interaktiver Bearbeitung), ist kein `Display.asyncExec` nötig. Falls du Inhalte programmatisch aus einem Hintergrund-Thread setzt, wrappe den Aufruf entsprechend.
- Initial direkt nach `createSourceViewer` einmal `updateOverviewRulerVisibility(viewer.getDocument())` aufrufen, damit auch schon beim Öffnen einer bereits großen Datei korrekt reagiert wird.

## Falls `showAnnotationsOverview` in deiner Zielversion nicht verfügbar/zugänglich ist

Fallback über die `IOverviewRuler`-Control direkt:

```java
IOverviewRuler overviewRuler = getSourceViewer().getOverviewRuler();
if (overviewRuler != null) {
    Control control = overviewRuler.getControl();
    if (control != null && !control.isDisposed()) {
        control.setVisible(shouldShow);
        control.getParent().layout(true, true);
    }
}
```

Ein unsichtbarer `Control` bekommt in SWT **keine** `Paint`-Events mehr – `doPaint()` (und damit `WidgetInfos`/`TextLayout.computeRuns()`) wird dann schlicht nicht mehr aufgerufen, ganz ohne den Ruler zu zerstören.

## Zusätzlicher Hinweis

Da dieselbe `IDocumentListener`-Logik jetzt an zwei Stellen relevant ist (Scanner-Guard + Ruler-Guard), lohnt es sich, eine gemeinsame kleine Klasse `LargeFileGuard` zu bauen, die den Schwellenwert zentral hält und beide Mechanismen (Presentation-Skip, Ruler-Visibility) von einer Stelle aus synchron schaltet – vermeidet Inkonsistenzen, falls du später weitere teure Features (z. B. Folding, Minimap, Code-Lens) genauso drosseln willst.

java.lang.NullPointerException: Cannot invoke "org.eclipse.jface.text.IDocument.addDocumentListener(org.eclipse.jface.text.IDocumentListener)" because the return value of "org.eclipse.jface.text.source.ISourceViewer.getDocument()" is null
#: SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=10f52522-82a8-474f-bf9e-f7ab0deff090 | model=claude-sonnet-5 | plugins=
#: ReasoningToken: 1042
Der Grund: `createSourceViewer()` wird während `createPartControl()` aufgerufen – zu diesem Zeitpunkt ist das Dokument dem Viewer **noch nicht** zugewiesen. Das passiert erst später in `doSetInput()`, wenn der `DocumentProvider` das Dokument liefert und `getSourceViewer().setDocument(...)` aufgerufen wird. `viewer.getDocument()` ist an der Stelle also zwangsläufig `null`.

## Richtiger Hook: `ITextInputListener`

`ISourceViewer` erbt von `ITextViewer`, das genau für diesen Fall `addTextInputListener` anbietet – er feuert immer dann, wenn dem Viewer ein (neues) Dokument zugewiesen wird, auch beim allerersten Öffnen:

```java
private final IDocumentListener fDocumentListener = new IDocumentListener() {
    @Override
    public void documentChanged(DocumentEvent event) {
        updateOverviewRulerVisibility(event.getDocument());
    }
    @Override
    public void documentAboutToBeChanged(DocumentEvent event) { }
};

@Override
protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {
    ISourceViewer viewer = super.createSourceViewer(parent, ruler, styles);

    viewer.addTextInputListener(new ITextInputListener() {
        @Override
        public void inputDocumentAboutToBeChanged(IDocument oldInput, IDocument newInput) {
            if (oldInput != null) {
                oldInput.removeDocumentListener(fDocumentListener);
            }
        }

        @Override
        public void inputDocumentChanged(IDocument oldInput, IDocument newInput) {
            if (newInput != null) {
                newInput.addDocumentListener(fDocumentListener);
                updateOverviewRulerVisibility(newInput); // initialer Check
            }
        }
    });

    return viewer;
}
```

## Warum das robuster ist

- Kein `null`-Zugriff mehr, weil du nicht mehr davon ausgehst, dass beim Erzeugen des Viewers schon ein Dokument existiert.
- `inputDocumentAboutToBeChanged` sorgt dafür, dass du beim **Wechsel** des Dokuments (z. B. wenn ein Editor-Teil wiederverwendet oder das Backing-Dokument ausgetauscht wird) den alten Listener sauber abmeldest – sonst würdest du bei jedem Input-Wechsel einen weiteren `IDocumentListener` anhäufen (Memory Leak + mehrfaches Feuern).
- Der initiale `updateOverviewRulerVisibility(newInput)`-Aufruf in `inputDocumentChanged` deckt auch den Fall ab, dass eine bereits **beim Öffnen** zu große Datei sofort korrekt behandelt wird, nicht erst nach der ersten Änderung.

Damit sollte die NPE verschwinden und dein Guard funktioniert unabhängig davon, wann/wie das Dokument tatsächlich gebunden wird.

Schon viel besser. Eine sache war dennoch bei sehr langen Zeilen:
Thread [main] (Suspended)	
	OS.g_utf16_pointer_to_offset(long, long) line: not available [native method]	
	39 collapsed frames	
		TextLayout.computeRuns() line: 326	
		TextLayout.getOffset(int, int, int[]) line: 1455	
		StyledText.getOffsetAtPoint(int, int, int[], boolean) line: 4248	
		StyledText.getOffsetAtPoint(Point) line: 4187	
		StyledText.setCaretLocations(Point[], int) line: 8544	
		StyledText.setCaretLocations() line: 8500	
		StyledText.scrollVertical(int, boolean) line: 8012	
		StyledText.handleVerticalScroll(Event) line: 6252	
		Lambda.handleEvent(Event) line: not available	
		EventTable.sendEvent(Event) line: 91	
		Display.sendEvent(EventTable, Event) line: 5865	
		ScrollBar(Widget).sendEvent(Event) line: 1656	
		Display.runDeferredEvents() line: 5080	
		Display.readAndDispatch() line: 4515	


#: SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=10f52522-82a8-474f-bf9e-f7ab0deff090 | model=claude-sonnet-5 | plugins=
#: ReasoningToken: 6637
Das ist eine andere Kategorie von Problem als die vorherigen – hier bist du tief in der **nativen SWT-Widget-Ebene** (`StyledText`/Pango via GTK), nicht mehr in JFace-Text-Bereichen, die du über Damager/Repairer/Reconciler steuern kannst.

## Was hier passiert

Beim Scrollen ruft `StyledText.scrollVertical()` → `setCaretLocations()` → `getOffsetAtPoint()` auf, um die Caret-Position nach dem Scrollen neu zu bestimmen. Das erfordert eine Pixel→Offset-Umrechnung, wofür Pango über `TextLayout.computeRuns()` die **komplette Zeile** itemisieren/shapen muss (Font-Fallback, Bidi-Reordering, Ligaturen). Bei einer pathologisch langen Zeile (z. B. minifiziertes JS, generierter Code, Log-Zeile mit Hunderttausenden Zeichen) ist das pro Aufruf teuer – und es gibt dafür **keinen JFace-Hook**, weil es Teil der internen Caret-Verwaltung von `StyledText` selbst ist.

## Praktikabelster Trick: Zeile künstlich in Segmente zerlegen

SWT bietet dafür eigentlich ein Bidi-Feature, das sich hier zweckentfremden lässt: `BidiSegmentListener`. Damit kannst du Pango zwingen, eine sehr lange Zeile in kleinere, unabhängig verarbeitete Abschnitte aufzuteilen, statt sie als einen einzigen Shaping-Lauf zu behandeln:

```java
StyledText textWidget = sourceViewer.getTextWidget();

textWidget.addBidiSegmentListener(event -> {
    int length = event.lineText.length();
    if (length <= SEGMENT_THRESHOLD) {
        return; // normale Zeilen unangetastet lassen
    }

    int segmentSize = 200; // experimentell ermitteln
    int segmentCount = length / segmentSize;
    int[] segments = new int[segmentCount + 2];
    segments[0] = 0;
    for (int i = 1; i <= segmentCount; i++) {
        segments[i] = i * segmentSize;
    }
    segments[segments.length - 1] = length;
    event.segments = segments;
});
```

Das reduziert die Kosten pro `computeRuns()`-Aufruf drastisch, weil Pango jedes Segment unabhängig verarbeitet statt die ganze Zeile am Stück.

**Nebenwirkungen, die du in Kauf nimmst:**
- Echte Bidi-Texte (RTL-Sprachen) könnten an den künstlichen Segmentgrenzen leicht falsch dargestellt werden – für Code/Log-Dateien (überwiegend ASCII/LTR) i. d. R. unkritisch.
- Ligaturen/Kerning exakt an Segmentgrenzen können minimal abweichen – optisch meist nicht wahrnehmbar bei Monospace-Editor-Fonts.

## Ergänzend, kein Plugin-Fix, aber relevant

- Prüfe, ob eine **neuere Eclipse-/SWT-Plattformversion** dieses Problem bereits mildert – es gab in der Vergangenheit mehrere Bugzilla-Einträge zu genau diesem Muster ("very long line" + `computeRuns`/`getOffsetAtPoint`), teils mit Upstream-Fixes in späteren GTK/SWT-Versionen.
- Falls reproduzierbar und bei dir noch nicht gefixt: lohnt sich ein Bugzilla-Report mit Stacktrace + Repro-Datei, da das eher ein Plattform- als ein Plugin-Problem ist.
- Als Nutzer-seitiger Workaround (nicht code-technisch): Navigation per Tastatur (Page-Down/End) statt Scrollbar-Drag vermeidet manche kontinuierlichen `scrollVertical`-Trigger, ist aber keine echte Lösung.

**Kurz gesagt:** Bei den vorherigen Problemen (Scanner, Overview Ruler) hattest du saubere JFace-Hooks. Hier bist du an der Grenze dessen, was man ohne SWT-interne Tricks beeinflussen kann – der Bidi-Segment-Ansatz ist der pragmatischste mir bekannte Hebel, ist aber ein Workaround, kein offizieller Performance-Schalter.