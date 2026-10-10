# Resident: Index, StreamedIndexManager und Synchronisation (Ergänzung)

Ergänzung zum Konzept der "Resident"-Engine (`resident.md`). Wird separat umgesetzt. Betrifft Index-Vertrag, Manager der Subengines, Kernel-Schnittstelle und Synchronisation RAM <> Index <> VRAM.

## Leitprinzipien

- Jeder Index implementiert sein **eigenes Modell** (ganze Dateien/Gleitfenster, AST-Knoten, Chunks, kaskadierende oder fraktale Strukturen). Der Kern kennt diese Modelle nicht.
- Der Index muss zunächst **nur Änderungen melden**. Was die Engine daraus macht, ist Sache von Manager und Engine.
- Die Richtungen sind agnostisch: RAM <> Index <> VRAM. Der Index ist die RAM-Primärinstanz, der VRAM ist ein abgeleiteter Spiegel.
- Der Index kennt weder Engine, Profil noch Kernel. Die einzige harte Index-Modell-Bindung ist Modell-ID und Dimension im Index-Header.
- Die Engine rechnet auf der ihr gegebenen Kandidatenmenge. Sie verbürgt weder Vollständigkeit noch Größe der Menge.
- Approximative Vorfilter (Zentroid, FFT, Kompression) verlieren potenziell Randtreffer. Das ist Eigenschaft der Oberschicht und nicht Teil der Engine.
- Veraltete Ergebnisse sind **akzeptierte Variablen**, kein Fehler. Es gibt keine Sperren zugunsten von Konsistenz.
- Der Kern enthält **keine Iteration, Kaskade, Rekursion oder Eltern-Kind-Beziehung** zwischen Tasks (siehe Abschnitt Addons).

## Drei Schichten

- **Index:** kennt Entries, deren Vektordaten und seine eigene Traversalstruktur. Meldet Änderungen, liefert Struktur und Stamps, sonst nichts.
- **StreamedIndexManager:** kennt Entry <-> Segment/Slot, Residency und Update-Queue. Verantwortlich für Upload, Eviction, Update, Rückabbildung von Adressen auf Entries.
- **Kernel:** kennt nur Segmente, Adressen und Offsets. Keine Entry-Identität, keine Traversierung.
- Der Manager ist eine Komponente der **CPU/GPU-Engine**, nicht der Fassade und nicht des Index. Er wird mit einer Engine und einem lazy aktiviertem Index konstruiert. Der Index bleibt unabhängig nutzbar.
- Der Manager der CPU-Engine liest den Index direkt (RAM ist Primärinstanz). Der Manager hat fast nichts zu tun. Ausnahme: Residency bei mmap-Indizes über RAM-Größe und Entladen nach Idle-Zeit (Prefetch, `madvise`).

## Index-Vertrag

- **Entry:** Vektorbereich mit variabler Länge (Dense = 1 Vektor, ColBERT/Datei/Gleitfenster = viele Vektoren). Entries können überlappen oder dieselben Vektoren referenzieren.
- **Stamp pro Entry:** Wert eines globalen, monotonen Zählers zum Zeitpunkt des Writes (letzte Änderung des Entries).
  - Eindeutig auch über Entfernen und Wiederanlegen derselben ID (ABA vermeiden).
  - Vergleich ausschließlich mit `!=`, nie mit `<`.
- **Fallback ohne eigenen Stamp** (z. B. dateigestützte mmap): Stufen Stamp -> size+mtime -> Content-Hash. Liegt der mtime innerhalb der Zeitauflösung vor dem letzten Scan, gilt der Entry als unsicher und wird per Hash geprüft (Racy-Problem).
- **Stamps in eigener Spalte.** Der Scan darf keine Vektorseiten einpagen.
- **Benachrichtigung:** Level-Signal ohne Inhalt, `tokio::sync::watch<u64>`. Es fasst zusammen, hat pro Receiver eigenen Zustand und trägt den Zählerwert als Watermark. Ein verlorenes oder zusammengefasstes Signal ist harmlos, weil der Abgleich zustandsbasiert ist.
- **Schreibreihenfolge im Index:** erst Daten, dann Stamp, dann Signal.
- **Zugriffsverfahren deklarieren:** Der Index meldet, welche Verfahren er unterstützt (Zentroid, Präfix, Subtree, Range, ...).
- **Nur Struktur liefern:** Kinder eines Knotens, Mitglieder einer Zelle (Inverted List), Summary-Einträge (z. B. Zentroid-Tabelle als Vektordaten), Präfix-Knoten. Der Index interpretiert keine Geometrie (kein Abstand, Radius, Top-k, Präfix-Match) und löst keinen Scope auf.
- **Mehrere Schreiber:** Der Index serialisiert `apply` und garantiert die Sequenz. Last-Writer-Wins. Zwei Engines dürfen gleichzeitig denselben Entry aktualisieren, wer zuerst kommt, wird potenziell vom anderen überschrieben.
- **Lazy:** Ein Index lädt lazy, `read` kann auf Page-Faults und Platten-I/O blockieren.

```rust
trait Index {
    fn stamp(&self, entry: EntryId) -> Option<Stamp>;            // None = entfernt
    fn stamps(&self) -> impl Iterator<Item = (EntryId, Stamp)>;  // ohne Vektordaten zu lesen
    fn read(&self, entry: EntryId) -> EntryData<'_>;             // Referenz, keine Kopie
    fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;       // Stamp des eigenen Writes
    fn changed(&self) -> watch::Receiver<u64>;                   // Level-Signal
    fn capabilities(&self) -> &[AccessMethod];                   // Centroid, Prefix, Subtree, Range, ...
    fn children(&self, node: NodeId) -> EntrySet;                // reine Struktur
    fn members(&self, cell: CellId) -> EntrySet;                 // Inverted List o. Ä.
    fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>; // z. B. Zentroid-Tabelle
    fn prefetch(&self, set: &EntrySet);                          // optional, darf no-op sein
}
```

## Scope und Auflösung (IndexManager)

- Der **Manager** löst den Scope eines Tasks für kompatible Indextypen auf, nicht der Index.
- Kompatibel heißt: Der Index deklariert das Verfahren **und** der Manager hat einen Resolver dafür. Sonst Fehler `UnsupportedScope`, kein stilles Ausweichen auf `All`.
- Resolver sind pluggable pro Zugriffsverfahren (`AccessMethod`).
- Beispiel Zentroid: Der Manager holt die Summary-Struktur vom Index und macht sie resident. Die Engine/Kernel rechnet die Query gegen die Zentroide. Der Manager fragt den Index nach den Mitgliedern der gewählten Zellen und gleicht gegen den Ladezustand ab (fehlt etwas, ist etwas veraltet).
- Scope kann Teilbäume oder Bereiche einschränken. Ist die Kandidatenmenge schon aus dem Preprocess bekannt (PLAID-artig, Pruning, Präfix, FFT/Kompressionsdomäne), lädt der Manager nur diese Bereiche.
- Der Scope wird als kompakte Entry-Menge dargestellt (Ranges oder Bitmap), nicht als ID-Liste.

```rust
enum Scope { All, Range(..), Set(..), Subtree(NodeId), Prefix(..), Centroid { k: u32, radius: Option<f32> } }
```

## Kernel-Schnittstelle

- Der Kernel liefert nur `(Adresse/Offset, Score)`. Ob Segment-ID plus Offset, roher Pointer oder Arena-Offset, ist Implementierungsdetail. Es gibt keine Kommunikation während der Berechnung.
- **Layouts eines Segments:**
  - `Fixed { stride }`: Entry-Grenzen per Arithmetik.
  - `Variable { offsets }`: Offset-Tabelle (Prefix-Summen), vom Manager aus der Indexstruktur gebaut.
  - Der Kernel traversiert nie. MaxSim aggregiert pro Entry, dafür braucht er Stride oder Tabelle, aber nicht die Entry-Identität.
- **Rückabbildung** Adresse -> Entry macht der Manager: bei Fixed per Arithmetik, bei Variable per Binärsuche in seiner Tabelle (oder Intervall-Map über Arena-Offsets).
- Eine Kandidatenmenge ist ein `Variable`-Layout. Der Kernel rechnet darüber, egal ob die Tabelle auf kompakte Kopien oder bestehende Segmente zeigt.

```rust
enum Layout { Fixed { stride: u32 }, Variable { offsets: DevicePtr } }
struct SegmentDesc { base: u64 /* Arena-Offset */, count: u32, layout: Layout }
struct KernelHit  { offset: u64 /* Arena-Offset */, score: f32 }
```

## Manager-Disziplin (nie den Kernel blockieren)

- Der Manager kennt den Laufzustand des Kernels und mutiert nur in **Lücken** (Batch-Ende bzw. Kommunikationsslot).
- Er ändert nichts unter dem Kernel und blockiert ihn nicht.
- **Atomarer Swap nur für einen 8-Byte-Pointer.** Größere Strukturen werden nie in place geändert, sondern als Copy-on-Write neu angelegt, danach wird der Pointer getauscht. Hierarchie: Root-Pointer -> Segmenttabelle -> Segment (Basis, Layout, Offset-Pointer).
- Gilt für Lookup-Tabellen und für ganze Segmente. Ein geändertes Segment wird komplett neu angelegt, der Pointer getauscht, sobald sicher ist, dass die Engine dort nicht mehr liest.
- **In-place-Update** ist in einer Lücke trivial. Im persistenten Modus kann der Kernel einen halb aktualisierten Vektor lesen (gemischte, aber gültige Werte). Das wird bewusst akzeptiert und dokumentiert.
- **Pointer-Lesezeitpunkt:**
  - Normale Launches: Pointer als Kernel-Argument, Swap wirkt ab dem nächsten Launch.
  - Persistenter Ringbuffer-Modus: Kernel liest den Pointer einmal pro Work-Item mit Acquire-Load (`ld.acquire`, sm70+, Turing reicht). Der Inhalt der neuen Tabelle muss vorher sichtbar sein (Event oder Fence).
- **Arena-Allokation:**
  - `cudaMalloc`/`cudaFree` im laufenden Betrieb vermeiden (implizite Device-Synchronisation).
  - Die Arena wird lazy beim ersten Laden nach VRAM-Budget angelegt, Vergabe per Free-List. Alle Adressen sind Arena-Offsets (`u64`).
  - Das Arena-Budget plant den kurzen Doppelbedarf durch Copy-on-Write ein.
- **Kopien** laufen über einen eigenen Copy-Stream mit gepinntem Staging, getrennt vom Compute-Stream.
- **Laufzustand:** ein Event pro Batch. Im persistenten Modus ein Fortschrittszähler in gemapptem Host-Speicher.
- Blockierende Reads vom Index (Page-Faults, I/O) laufen im Blocking-/Worker-Pool, nicht im async-Runtime-Thread. Prefetch-Hinweis (`madvise(WILLNEED)` o. Ä.), damit I/O und Kernel-Lauf überlappen.

## Kommunikationsslot: feste Reihenfolge

1. **Lesen, was der Kernel geschrieben hat:** gemeldete Bereiche den Entries zuordnen, `Index::apply` aufrufen, eigene Stamps eintragen.
2. **Ergebnisse auflösen:** Adresse -> Entry per Mapping.
3. **Updates einspielen:** Update-Queue anwenden (verändert das Mapping, daher zuletzt).

- Weil nur in Lücken mutiert wird, sieht jedes Ergebnis den Mapping-Stand seines Batches. Der Manager repräsentiert den VRAM, eine vom Kernel gemeldete Adresse ist daher aktuell. Keine Epoche und keine Sequenznummer im Kernel-Protokoll.
- **Ergebnisstatus beim Auflösen:**
  - ID im Manager vorhanden, im Index entfernt: löst nicht mehr auf, Ergebnis wird verworfen.
  - Für den Entry liegt ein Update in der Queue: Ergebnis wird mit `outdated` markiert (Diagnose, kein Fehler).
  - Sonst normales Ergebnis.
- Änderungen liegen in der Update-Queue, bis sie ausgeführt werden.
- Nach außen: `Hit { entry, score, outdated }`.

## Ringbuffer-Modus (persistent, geplant)

- **Getrennte Ringe** für Embedding (und dessen Results) und MaxSim (und dessen Results).
- Pro Ring ein Read- und ein Write-Pointer. Kein Pointer überholt den anderen.
- Die Engine und damit der Manager kennt die eingereihten Requests und somit die **blockierten Segmente**.
- Ein Segment wird erst freigegeben, wenn kein eingereihter Request es mehr referenziert. Die Sperrmenge besteht nur aus Requests, die **vor** dem Pointer-Swap eingereiht wurden. Spätere Requests lesen schon die neue Tabelle.
- Task-gebundene temporäre Segmente gelten als blockiert, bis die Ergebnisse aufgelöst sind.

## Kernel schreibt Segmente (Indexaufbau)

- Der Manager vergibt vorab einen beschreibbaren Bereich aus der Arena. Der Kernel schreibt per Cursor hinein und **allokiert nie**.
- Der Kernel meldet die belegten Bereiche am Batch-Ende. Reicht der Bereich nicht, liefert er ein Teilergebnis, der Manager vergibt den nächsten Bereich.
- Im nächsten Kommunikationsslot ordnet der Manager die Bereiche den Entries zu (Schritt 1 der Reihenfolge) und ruft `Index::apply` auf.
- Embedding-Ringe liefern neue Segmente, MaxSim-Ringe liefern Hits. Indexieren während der Suche funktioniert, weil Schritt 1 vor Schritt 2 liegt.

## Synchronisation und Echo-Vermeidung (zustandsbasiert)

- Der Abgleich ist **zustandsbasiert statt ereignisbasiert**: Das Signal weckt nur, der Manager vergleicht Stamps.
- **Nur residente Entries** werden verglichen. Änderungen an nicht geladenen Entries brauchen keine Behandlung, weil das Laden ohnehin den aktuellen Stand liest. Der Manager-Zustand skaliert mit dem geladenen Teil, nicht mit der Indexgröße.
- **Signal vor dem Scan quittieren**, nicht danach. Sonst geht eine Änderung verloren, die während des Scans eintrifft.
- **Lesereihenfolge beim Laden:** Stamp lesen, dann Daten lesen, dann bei mmap den Stamp erneut prüfen. Weicht er ab, wiederholen. Die falsche Reihenfolge (erst Daten, dann Stamp) trägt einen neueren Stamp als bekannt ein, obwohl veraltete Daten geladen wurden. Das ist die einzige unsichere Richtung, ein überzähliger Reload ist harmlos.
- **Echo-Unterdrückung:** Der Manager trägt den von `apply` zurückgegebenen Stamp des eigenen Writes als bekannt ein (Gleichheitsvergleich). Der Stamp muss aus dem Rückgabewert stammen, nicht aus einem späteren Lesen. Ohne Rückgabewert entsteht höchstens ein redundanter Upload (sichere Richtung).
- **Zwei Engines, derselbe Entry:** Der andere Manager sieht einen abweichenden Stamp und lädt nach. Beide konvergieren auf den Indexstand.
- **Entfernen und Neuanlage:** Der Manager vergleicht sein Mapping mit `stamps()` bzw. `stamp(entry) -> None`.
- **Mehrere Subscriber** (GPU-Manager, Persistenz-Timer, CPU-Manager) haben jeweils eigenen Zustand.

### Optionale Optimierungen (später, kein Vertrag)

- `changed_since(watermark)` als begrenzter Recent-Change-Ring. Bei Überlauf Fallback auf Vollscan.
- Baumartige Indizes: max-Stamp pro Teilbaum, jeder Konsument mit eigenem Watermark. Bitmaps zum Eingrenzen von Teilbäumen müssen **pro Konsument** geführt werden (eine gemeinsame Bitmap verliert Änderungen für den zweiten Konsumenten).

## Residency und Ladestrategie (Manager-Implementierung)

- **Lazy und scope-getrieben:** Es wird nur geladen, was ein Task braucht.
- **Ladegranularität:**
  - Dichte Scopes: ganze Segmente laden.
  - Dünne Kandidatenmengen: Gather in einen temporären, kompakten Segmentpuffer im gepinnten Staging, ein DMA.
  - Sehr kleine Mengen: Zero-Copy aus gemapptem Host-Speicher möglich.
  - Die Dichteschwelle zwischen Gather und Segmentladen ist Manager-Implementierung. Sie kann als Achse in die Kalibrierung (`resident_precision_calibration.md`) aufgenommen werden.
- **Tiering:**
  - Summary-Strukturen (Zentroide, Präfixtabellen): klein, zuerst und am längsten gehalten (lange Entladezeit im VRAM).
  - Vektorsegmente: warm, per Eviction verdrängbar.
  - Alles andere: nicht geladen, kostet nichts.
- **Aggregation:** Tasks gegen denselben Index werden über die **Vereinigung ihrer Scopes** gebündelt und einmal geladen.
- **Single-Flight:** Gleichzeitige Ladeanforderungen auf denselben Bereich laufen als ein Ladevorgang mit mehreren Wartenden (analog zur Lazy-Init von Modellen).
- **Segmentgröße** bestimmt der Manager nach VRAM-Budget. Sie ist keine Eigenschaft des Index.
- **Überlappende Entries:** Der Manager entscheidet, ob Vektoren einmal gehalten und von mehreren Slots referenziert werden (spart VRAM, komplexer) oder dupliziert werden (einfach, teuer). Das ist Design-Entscheidung des Managers, kein Vertrag.
- **Entladen:** Idle-Zeiten für VRAM wesentlich länger als für RAM (wie im Hauptkonzept).

## Addons (Iteration, Kaskaden, Rekursion)

- Die Queue ist bereits designt und bleibt **unverändert**.
- Der Kern kennt keine Iteration, Kaskade, Rekursion, Eltern-Kind-Beziehung oder Zwischenzustände von Tasks.
- Ein Task kann den Scope eines Folgetasks treiben. Iteration und Rekursion entstehen allein durch ein **Addon** in einer Engine, in der Anwendung oder in einer konkreten Ressourcenimplementierung (Index, Modell).
- Ein Addon reiht Tasks über den normalen Weg ein (kopierte oder geänderte Parameter). Die Engine kann sie selbst oder eine andere Engine kann sie ziehen, gesteuert über die vorhandenen Pull-Filter (Priorität, Affinität).
- Ein von einem Addon erzeugter Task ist **nicht unterscheidbar** von einem Anwendungs-Task.
- Der Kern benötigt dafür nur das bereits geforderte **erweiterbare Parameterobjekt**.
- Beispiel (nur Illustration): Zentroid-Task gegen die Summary-Struktur, danach Folgetask mit den Mitgliedern der gewählten Zellen als Scope.

## Fehlerbehandlung und Logging

- Typisierte Fehler (`thiserror`), u. a. `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` (nur intern), `TransferFailed`.
- `tracing`-Spans pro Task und pro Manager-Zyklus. Geloggt werden: Scan-Ergebnis (geladen/veraltet/entfernt), Lade- und Eviction-Entscheidungen, verworfene und `outdated`-Ergebnisse, Arena-Auslastung.
- Fehler beim Laden eines Bereichs gehen an die wartenden Tasks zurück. Single-Flight verteilt den Fehler an alle Wartenden, kein stiller Retry-Sturm.
- Kein stilles Ausweichen auf andere Verfahren oder Scopes.

## Nicht Teil dieser Ergänzung

- Persistenzformat und Persistenzzeitpunkte des Index (liegen beim Index-Objekt, außerhalb der Engine).
- Präzision, Profile, Kalibrierung: siehe `resident_precision_calibration.md`.
- Chunking, Query-Formulierung, Reranking, Kandidaten-k, Indexstruktur selbst.
- Iteration und Kaskaden als Kernfunktion (nur als Addon).
- Fan-out mit Join und Scheduler-seitiger Task-Beziehung.

## Umsetzungsvorgaben für den Agenten

- Neue Komponenten: `Index`-Trait, `StreamedIndexManager` (GPU-Engine), `ArenaAllocator`, `ScopeResolver` pro Zugriffsverfahren, `SegmentDesc`/`Layout`, Update-Queue, Residency-Tracker (Mapping Entry <-> Slot mit bekannten Stamps).
- Der Manager arbeitet gegen eine **Kernel-Abstraktion** (Trait) mit Mock für Tests: Batch-Start, Batch-Ende, Ergebnisse, geschriebene Bereiche, Fortschrittszähler.
- CPU-Engine: ein trivialer Manager, der den Index direkt liest.
- Tests:
  - Reihenfolge im Kommunikationsslot (Kernel-Writes, Auflösen, Updates), inklusive Ergebnisse auf frisch geschriebene Adressen.
  - Ergebnisstatus: entfernt wird verworfen, ausstehendes Update markiert `outdated`.
  - Echo: eigener `apply` löst keinen Reload aus, fremder Stamp löst Reload aus, zwei Manager auf demselben Entry konvergieren.
  - Verlorenes oder zusammengefasstes Signal: nächster Abgleich findet alle Änderungen.
  - Signal-Quittierung vor dem Scan (Änderung während des Scans geht nicht verloren).
  - Stamp-Lesereihenfolge beim Laden (Änderung zwischen Stamp- und Datenlesen führt zu Wiederholung).
  - ABA: Entfernen und Wiederanlegen derselben ID ergibt einen anderen Stamp.
  - Copy-on-Write-Swap: altes Segment wird erst frei, wenn kein vor dem Swap eingereihter Request es mehr referenziert.
  - Gather-versus-Segment-Entscheidung über Dichteschwelle.
  - Single-Flight: ein Ladevorgang bei mehreren gleichzeitigen Anforderungen, Fehlerverteilung an alle Wartenden.
  - `UnsupportedScope` bei nicht deklariertem Verfahren oder fehlendem Resolver.
- Reale CUDA-Pfade (Acquire-Load, Pinned Staging, Arena) hinter einem Feature-Flag, damit die Logik ohne GPU testbar bleibt.