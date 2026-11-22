# Spezifikation: Offene Punkte zum Abgleich bis 119 (Build-Order 119d)

Diese Spec hat kein eigenes Crate-Modul und keinen Code. Sie sammelt die Punkte aus 119b, 119c und dem Protokoll `konsolidierung_ab_120.md`, die sich noch nicht entscheiden lassen. Jeder Punkt nennt den Stand, die vorläufige Annahme in den Specs und den Weg zur Klärung.

## 1. Task, restrictions and boundaries

**Aufgabe**

- Offene Fragen festhalten, mit Beleg und mit der Annahme, auf der die Specs ab 125 heute aufbauen.
- Pro Punkt angeben, wodurch er sich klärt: Entscheidung, Test oder Lesen von Code.

**Grenzen**

- Keine Festlegung und keine Änderung von Specs oder Code. Wird ein Punkt entschieden, wandert die Festlegung in die betroffene Spec, und der Punkt hier erhält den Vermerk "erledigt" mit dem Verweis.
- Keine neuen Anforderungen.

## 2. Context

- 119b legt fest, wie die Specs bis 119 zu lesen sind. 119c setzt die Codeänderungen daraus um. Was dabei unentschieden blieb, steht hier.
- Die Specs 125, 137, 138, 139 und 140 enthalten für jeden Punkt eine Annahme. Sie sind unten aufgeführt.

## 3. Interface

Keine.

## 4. Behavior and invariants

Status je Punkt: **offen**, **Test** (durch einen Test zu klären) oder **Entscheidung** (braucht jemanden, der die Anforderung kennt).

**P1 Was ist ein `PerToken`-Entry? (Entscheidung)**

- Stand: `IndexHeader` kennt `HeaderPooling::{MaskedMean, PerToken}`. `vector_bytes()` rechnet für beide `dimension × 2`. Das Parquet-Format sagt nicht, ob ein `PerToken`-Entry eine Token-Matrix (Länge `k × dimension × 2`) oder ein einzelner Token-Vektor ist.
- Annahme: 138 akzeptiert `k` Zeilen je Entry (`k <= class.max_tokens()`). 137 akzeptiert nur `k = 1`. Bei einem Vektor je Entry gilt `SlotClass::custom(1)` (119c F1).
- Betrifft: 137 §4.6, 138 §4.4, 125 (`IndexSpec`).
- Klärung: Der Besitzer des Index-Formats legt fest, ob ein Dokument ein Entry mit mehreren Zeilen ist oder mehrere Entries (Abschnitte, `EntryRef.section`) hat. Bei mehreren Entries ist `section` mehr als 0 und 138 muss die Abschnitte vergeben.

**P2 Braucht der Scoring-Kernel das Root-Argument der Launch-Modi? (Test)**

- Stand: 108 übergibt Root und Epoche des `TableSwap` an normale Launches. 118 liest `KernelParams.segments_addr` (`SegmentEntry[]`) und keinen Root.
- Annahme: Der Scoring-Kernel braucht den Root nicht. 138 schreibt die `SegmentEntry`-Tabelle über `write_kernel_table`.
- Betrifft: 138 §4.10, 139.
- Klärung: Ein Test in 139 startet `dense_dot` und `fused_maxsim` mit einer von 138 geschriebenen Tabelle, ohne Root-Argument.

**P3 Reichen 114 und 119 für Treffer des Scoring-Kernels? (Test)**

- Stand: 114 löst `KernelHit{addr, score}` über Manifest und Segment-Generation auf. Der Scoring-Kernel meldet Positionen als globale Slot-Nummern.
- Annahme: Scoring-Treffer gehen über `GpuManager::entry_at_global_slot`. 114 bleibt für Treffer des Index-Aufbaus (`EmbeddingWrite`, `KernelHit` mit Adresse).
- Betrifft: 138 §4.2 und §4.12, 114, 119.
- Klärung: Ein Test, der einen Treffer des Scoring-Kernels durch beide Wege schickt und dasselbe `EntryRef` erwartet. Zeigt er einen Unterschied, entscheidet jemand, welcher Weg gilt.

**P4 Regionen nach `ManifestStore::reclaim()` genau einmal freigeben (Test)**

- Stand: 138 §4.12 zählt die Regionen aus `ManifestStore::reclaim()` und gibt sie nicht frei. `Eviction` und die Deferred-Return-Liste geben frei.
- Annahme: Es gibt keine Doppelfreigabe und keine vergessene Freigabe.
- Betrifft: 138 §4.8, §4.12.
- Klärung: Test mit Veröffentlichung, Eviction und anschließendem `reclaim_deferred`. Jede Region wird genau einmal am Allocator freigegeben. Mit C1 und C3 (119c) bleibt `SegmentArena` konsistent.

**P5 Eviction-Budget bei mehreren Indizes (Entscheidung)**

- Stand: Jeder Index hat eine eigene `Eviction` mit demselben `TierBudget` auf der Arena `IndexSegments`. `make_room` eines Index sieht nur dessen Segmente. `EvictKey(pub u64)` trägt keinen Index. 138 §4.6 bildet ihn als `EvictKey(id.0)` aus der `SegmentId`, und `next_id` zählt je `GpuManager` ab 0 (138 §4.1). Zwei Indizes vergeben also dieselben Keys.
- Annahme: Die Arena ist die harte Grenze. Das Budget kann überzeichnet werden. Der Koordinator eines Index macht nur Platz mit den eigenen Segmenten.
- Betrifft: 139 §4.5, 140 §4.2.
- Klärung: Entscheidung, ob das Budget je Index aufgeteilt wird oder eine gemeinsame `Eviction` je Sub-Engine entsteht. Bei einer gemeinsamen `Eviction` müssen `EvictKey`s über Indizes hinweg eindeutig sein.

**P6 Drei Typen `IndexId` (Entscheidung)**

- Stand: `resource_requirements::IndexId(String)` (Schlüssel in 125, 126, 137, 139), `manager_lifecycle::IndexId(Arc<str>)` (im `IndexManager`) und `similarity_task::IndexId(u64)` (in `SimilarityParams`). Nur 065 wandelt: `IndexId::new(params.index.0.to_string())`.
- Annahme: Der Aufrufer hält alle drei und ordnet sie zu. Außer 065 (Similarity nach `resource_requirements`) wandelt kein Modul um.
- Betrifft: 065, 125, 139, 140.
- Klärung: Entscheidung, ob ein Typ die anderen ersetzt oder eine Umwandlung bekommt.

**P7 Kein Rebuild von Segmenten (Entscheidung)**

- Stand: Slots werden nie wiederverwendet. 101 bietet `compaction_plan` und `rebuild`, 087 `begin_replace` und `finish_replace`. Kein Modul bindet sie an: `SegmentWriter::rebuild`, `compaction_plan`, `SegmentArena::begin_replace` und `finish_replace` existieren, werden aber außerhalb der Tests nirgends aufgerufen.
- Annahme: Segmente mit vielen ausgeblendeten Slots werden erst durch die Eviction entfernt.
- Betrifft: 138 §1 (Grenzen).
- Klärung: Entscheidung, ab wann und durch wen ein Segment neu gebaut wird.

**P8 CPU ohne gepolstertes Layout (Test)**

- Stand: 137 liest Vektoren direkt vom Index, ohne Slots und ohne Padding (137 §4.4: kein Slot-Manager, Lesen über `Index::read`).
- Annahme: Die CPU-Kernel (`score_dot`, `score_negl2`) brauchen kein einheitliches Layout.
- Betrifft: 137 §4.6.
- Klärung: Die Tests von `CpuScoreKernel` laufen auf Zeilen aus `Index::read`. Bei `PerToken`-Indizes hängt das von P1 ab.

**P9 Wie kommt ein vom Kernel gebauter Entry in Index und Directory? (Entscheidung)**

- Stand: 133 führt den Embedding-Batch aus und gibt Bereiche zurück (133 §1: Freigabe und Weiterverwendung bleiben beim Aufrufer). Es schreibt weder in den Index noch ins Directory. 138 deckt nur den Weg über `mirrored_index` und `write_back` ab.
- Annahme: Der Aufrufer von 133 schreibt das Ergebnis über `write_back` (mit `placed`) oder über den gewrappten Index.
- Betrifft: 133, 138 §4.11, 119.
- Klärung: Entscheidung, wer den Schritt "Bereich gelesen → Slot reserviert → Index geschrieben" ausführt.

**P10 Prosa mit "dem Manager" (offen)**

- Stand: 124 (§2) und 128 (§1, §2, §4) nennen "den Manager" ohne Bezug auf `GpuManager` oder `IndexOperations`.
- Annahme: Gemeint ist der Speichermanager des Index (138 mit 140).
- Betrifft: 124, 128.
- Klärung: Beim nächsten Überarbeiten dieser Specs den Namen eintragen.

## 5. Relied-on elements

Keine eigenen. Verwiesen wird auf 101, 108, 114, 118, 119b, 119c, 124, 125, 128, 133, 137, 138, 139 und 140.
