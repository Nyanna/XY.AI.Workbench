# Implementierung eines zeilenbasierten Spellchecks

Eine vorhergehende Syntax- und Grammatikprüfung verbessert die Effizienz eines Prompts. Implementiere einen Spellcheck auf Basis eines laufenden LanguageTool-Servers.

Das Herzstück ist das Zusammenspiel von **Reconciler** (Hintergrundthread), **IAnnotationModel** (Datenspeicher für Fehler) und **AnnotationPainter** (visuelle Darstellung).

## Kontext

* Die Implementierung findet innerhalb eines bestehenden Eclipse-Plugin-Projektes statt.
* Die Implementierung erfolgt typisch und einfach.
* Erweiterte Funktionen werden nicht innerhalb dieser Aufgabe implementiert.
* Keine Dokumentation und Tests
* Die verwendete Sprache im Code ist Englisch.
  

## Anforderungen

* Die Implementierung erfolgt innerhalb des Projektes `/home/user/xyan/xy.ai.workbench`.
* Das Rootpackage für alle Komponenten ist `xy.ai.workbench.editors.spellcheck`.

## Das Gesamtbild

1. Nutzer tippt und verändert eine Zeile.
2. Reconciler wartet ~500ms (debounce).
3. IReconcilingStrategy.reconcile(), Hintergrundthread, Spell-Checking läuft hier
4. Display.asyncExec(), zurück auf UI-Thread
5. IAnnotationModel.replaceAnnotations()
6. AnnotationPainter zeichnet Wellenlinie.

### 1. Reconciler – der Hintergrundthread-Manager

Der `MonoReconciler` startet einen BackgroundThread, der nach einer konfigurierbaren **Wartezeit nach dem letzten Tastendruck** eine Strategie aufruft:

Beispiel:

```java
SpellingStrategy strategy = new SpellingStrategy(sourceViewer);
MonoReconciler reconciler = new MonoReconciler(strategy, /*incremental=*/false);
reconciler.setDelay(500); // ms nach dem letzten Tastendruck
sourceViewer.setReconciler(reconciler);
```

### 2. IReconcilingStrategy – Logik im Hintergrund

Die Implementierung erhält hier entweder einen Change-Event für eine Zeile oder prüft die Zeile der aktuellen Cursor-Position.

Beispielhafte Logik:

```java
public class SpellingStrategy
        implements IReconcilingStrategy, IReconcilingStrategyExtension {

    private IDocument fDocument;
    private IProgressMonitor fMonitor;
    private final ISourceViewer fViewer;

    public SpellingStrategy(ISourceViewer viewer) { fViewer = viewer; }

    @Override public void setDocument(IDocument doc)         { fDocument = doc; }
    @Override public void setProgressMonitor(IProgressMonitor m) { fMonitor = m; }

    @Override
    public void initialReconcile() {
        // Einmal beim Start: ganzes Dokument prüfen
        reconcile(new Region(0, fDocument.getLength()));
    }

    @Override
    public void reconcile(IRegion partition) {
        if (fMonitor != null && fMonitor.isCanceled()) return;

        // ── Hier läuft die eigentliche Spell-Check-Logik ──
        List<SpellingProblem> problems = mySpellChecker.check(
            fDocument, partition.getOffset(), partition.getLength()
        );

        // Zurück auf den UI-Thread
        fViewer.getTextWidget().getDisplay().asyncExec(() ->
            applyAnnotations(problems, partition)
        );
    }

    @Override
    public void reconcile(DirtyRegion dirty, IRegion sub) {
        reconcile(sub); // inkrementell → nur geänderte Region
    }
}
```

#### 2.1. Implementierung des SpellChecker

Es gibt bereits eine Hook/Bash-Integration, die für den SpellChecker als Referenz dient.

`/home/user/xyan/xy.ai.workbench/claude-code/default/scripts/spell-check.sh`

Diese soll für die Ansteuerung mittels Java in Eclipse als Vorbild dienen.

* Ausschlussregeln für Backticks, Zeilen mit einem vorangestellten `@`-Zeichen usw. sind zu beachten. 

### 3. Annotations setzen (auf UI-Thread)

`IAnnotationModelExtension.replaceAnnotations()` entfernt alte und fügt neue **atomar** ein – wichtig für flickerfreie Aktualisierung:

Beispielhafte Logik:

```java
private void applyAnnotations(List<SpellingProblem> problems, IRegion region) {
    IAnnotationModel model = fViewer.getAnnotationModel();
    Object lock = (model instanceof ISynchronizable)
        ? ((ISynchronizable) model).getLockObject()
        : model;

    // Alte Annotations im betroffenen Bereich sammeln
    List<Annotation> toRemove = new ArrayList<>();
    synchronized (lock) {
        Iterator<Annotation> it = model.getAnnotationIterator();
        while (it.hasNext()) {
            Annotation a = it.next();
            if (SpellingAnnotation.TYPE.equals(a.getType())) {
                Position pos = model.getPosition(a);
                if (pos != null && region.getOffset() <= pos.offset
                        && pos.offset < region.getOffset() + region.getLength())
                    toRemove.add(a);
            }
        }
    }

    // Neue Annotations bauen
    Map<Annotation, Position> toAdd = new HashMap<>();
    for (SpellingProblem p : problems) {
        toAdd.put(
            new SpellingAnnotation(p),
            new Position(p.getOffset(), p.getLength())
        );
    }

    // Atomarer Tausch
    synchronized (lock) {
        ((IAnnotationModelExtension) model)
            .replaceAnnotations(toRemove.toArray(new Annotation[0]), toAdd);
    }
}
```

### 4. AnnotationPainter – die Wellenlinie zeichnen

Beispielhafte Implementierung:

```java
AnnotationPainter painter = new AnnotationPainter(
    sourceViewer,
    new DefaultAnnotationAccess()
);

// Moderne Variante: TextStyleStrategy (kein eigenes GC-Painting nötig)
painter.addTextStyleStrategy(
    "spelling",
    new AnnotationPainter.UnderlineStrategy(SWT.UNDERLINE_SQUIGGLE)
);
painter.addAnnotationType(SpellingAnnotation.TYPE, "spelling");
painter.setAnnotationTypeColor(
    SpellingAnnotation.TYPE,
    display.getSystemColor(SWT.COLOR_RED)
);
painter.install(sourceViewer);
```