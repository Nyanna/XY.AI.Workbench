Das Herzstück ist das Zusammenspiel von **Reconciler** (Hintergrundthread), **IAnnotationModel** (Datenspeicher für Fehler) und **AnnotationPainter** (visuelle Darstellung).

---

## Das Gesamtbild

```
Nutzer tippt
    │
    ▼
Reconciler wartet ~500ms (debounce)
    │
    ▼
IReconcilingStrategy.reconcile()  ← Hintergrundthread
    │  Spell-Checking läuft hier
    ▼
Display.asyncExec()  ← zurück auf UI-Thread
    │
    ▼
IAnnotationModel.replaceAnnotations()
    │
    ▼
AnnotationPainter zeichnet Wellenlinie
```

---

## 1. Reconciler – der Hintergrundthread-Manager

Der `MonoReconciler` startet einen BackgroundThread, der nach einer konfigurierbaren **Wartezeit nach dem letzten Tastendruck** deine Strategie aufruft:

```java
SpellingStrategy strategy = new SpellingStrategy(sourceViewer);
MonoReconciler reconciler = new MonoReconciler(strategy, /*incremental=*/false);
reconciler.setDelay(500); // ms nach dem letzten Tastendruck
sourceViewer.setReconciler(reconciler);
```

---

## 2. IReconcilingStrategy – deine Logik im Hintergrund

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

---

## 3. Annotations setzen (auf UI-Thread)

`IAnnotationModelExtension.replaceAnnotations()` entfernt alte und fügt neue **atomar** ein – wichtig für Flicker-freie Aktualisierung:

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

---

## 4. AnnotationPainter – die Wellenlinie zeichnen

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

> Alternativ zu `UnderlineStrategy` kannst du eine eigene `IDrawingStrategy` implementieren und mit `gc.drawPolyline()` komplett frei zeichnen.

---

## 5. Alles nutzt Eclipses eingebauten SpellingService

Wenn du kein eigenes Wörterbuch willst, kannst du direkt `SpellingReconcileStrategy` nehmen – die erledigt Schritte 2–3 automatisch:

```java
SpellingReconcileStrategy strategy = new SpellingReconcileStrategy(
    sourceViewer,
    EditorsUI.getSpellingService()  // nutzt die registrierten ISpellingEngine-Plugins
);
MonoReconciler reconciler = new MonoReconciler(strategy, false);
reconciler.setDelay(500);
sourceViewer.setReconciler(reconciler);
```

---

## Zusammenfassung der Verantwortlichkeiten

| Klasse                 | Wofür                        | Thread            |
| ---------------------- | ---------------------------- | ----------------- |
| `MonoReconciler`       | Debounce + Hintergrundthread | verwaltet beide   |
| `IReconcilingStrategy` | eigentliche Prüflogik        | **Hintergrund**   |
| `Display.asyncExec()`  | Brücke zurück zur UI         | Hintergrund → UI  |
| `IAnnotationModel`     | Fehler-Datenspeicher         | **UI**            |
| `AnnotationPainter`    | Wellenlinien zeichnen        | UI (Paint-Events) |