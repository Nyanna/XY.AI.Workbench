# Spezifikation: Abgleich der Spezifikationen bis 119 (Build-Order 119b)

Diese Spec hat kein eigenes Crate-Modul und keinen neuen Code. Die Codeänderungen, die sich daraus ergeben, setzt 119c um. Sie legt fest, wie die Specs bis 119 zu lesen sind, wo sie dem Code widersprechen und welche Lesart die Specs ab 125 voraussetzen.

## 1. Task, restrictions and boundaries

**Aufgabe**

- Abweichungen zwischen den Specs bis 119 und dem implementierten Code festhalten. Im Zweifel gilt der Code.
- Mehrdeutigkeiten zwischen den Specs bis 119 auflösen, auf die 125, 137, 138, 139 und 140 aufbauen.

**Grenzen**

- Die Specs bis 119 werden nicht verändert. Der Code bis 119 bleibt unverändert.
- Diese Spec definiert keine Typen und keine Funktionen.

## 2. Context

- Die Module bis 119 sind implementiert und getestet. Sie wurden parallel spezifiziert. Einzelne Aussagen stimmen deshalb nicht mehr überein.
- Die Specs ab 125 (`residency`, `manager`, `cpu_subengine`, `gpu_manager`, `gpu_subengine`, `index_ops`) hängen von den hier festgelegten Lesarten ab.

## 3. Interface

Keine. Diese Spec enthält keine öffentliche Schnittstelle.

## 4. Behavior and invariants

**K1 `SegmentId` und `SlotRef`.**

- `segments::SegmentId(pub u64)` und `segments::SlotRef { segment: SegmentId, slot: kernel_layout::Slot }` sind die einzigen Definitionen. `reconcile`, `manifest`, `address_map` und `query_batch` re-exportieren sie.
- `address_map::SegmentId(pub u32)` in 083 §3, 103 §5 und 119 §5 (Typ von `EmbeddingWrite.segment`) ist veraltet. Es gibt keine `u32`-Variante und keine Umsetzung zwischen den Typen.
- 109 §2 und 102 §3 beschreiben den Code richtig.

**K2 Segmenttabellen.**

- `TableSwap` (074) ist das Protokoll für Epochen, Reader und die Lebensdauer von Regionen. Seine Tabelle enthält `Region`s.
- `Manifest::kernel_table()` (102) ist implementiert, hat aber keinen Verbraucher außer Tests. Der Kernel liest sie nicht.
- Der Scoring-Kernel (118) liest `SegmentEntry[]` (40 Byte) über `KernelParams.segments_addr`. Der GPU-Manager (138 §4.10) erzeugt diese Tabelle aus den veröffentlichten Segmenten und schreibt sie per `sync_to_vram`.
- Die Launch-Modi (108) übergeben Root und Epoche des `TableSwap` an normale Launches. Der Scoring-Kernel (118) benutzt diese Argumente nicht.

**K3 Update-Queue.**

- `communication_slot::UpdateQueue` (Trait, 119 §3) und `update_queue::UpdateQueue` (Struktur, 109 §3) sind zwei verschiedene Typen mit demselben Namen.
- Der Trait hat `apply(&mut self, &ResidencyDirectory) -> UpdateOutcome`. Die Struktur hat `apply(&self, &ResidencyDirectory, &AddressMap) -> usize` und `take_warnings`.
- Der Adapter `SlotUpdates` (138 §4.12) verbindet beide. Er ist die einzige Stelle, die `apply` produktiv aufruft.

**K4 Bindings.**

- Eine bestehende Bindung (`SlotAssignment::bind`/`unbind`) ändert sich im Betrieb nur in `UpdateQueue::apply` in der Gap (083 §2, 109 §4).
- Ausnahme 1: `SegmentWriter::commit` bindet einen frisch reservierten, bisher ungebundenen Slot zu jedem Zeitpunkt. Hatte der Entry im selben Writer einen alten Slot, wird dieser dabei sofort gelöst und ausgeblendet. Seine Zuordnung bleibt über `resolve` für ältere Treffer auflösbar (101 §4).
- Ausnahme 2: In einem noch nicht veröffentlichten Segment ist jede Änderung unsichtbar.
- `UpdateQueue::apply` verändert `SlotStates` nie (109 §4). Das Ausblenden alter Slots (`SlotStates::invalidate`) übernimmt der GPU-Manager in `run_gap` (138 §4.9).

**K5 Epochen und Generationen.**

- Für Treffer zählt nur die Manifest-Generation (114, 119). `TableSwap`-Epoche und `MappingStore`-Epoche sind davon unabhängige Zähler.
- Die Manifest-Generation steigt bei jeder Veröffentlichung und bei jedem Entfernen eines Segments. `UpdateQueue::apply` bewegt sie nicht.
- Wer `ManifestStore::publish` aufruft, legt 138 fest (§4.6, §4.8). In den Specs bis 119 tut das niemand.

**K6 Freigabe von Regionen.**

- `Eviction::reclaim` (103), `TableSwap::reclaim` (074), `ManifestStore::reclaim` (102) und `SegmentArena::release` (087) sind vier getrennte Wege. Sie kümmern sich um verschiedene Regionen:
  - `TableSwap::reclaim` gibt abgelöste Tabellenregionen frei.
  - `Eviction::reclaim` gibt Regionen evicteter Segmente frei.
  - `SegmentArena::release` gibt Regionen frei, die der Aufrufer selbst zurückstellt.
  - `ManifestStore::reclaim` liefert Regionen, ohne sie freizugeben. Wer sie nutzt, gibt sie nicht ein zweites Mal frei.
- `Eviction` entfernt ein Segment nur aus Tabelle und Address-Map (103 §4). Manifest, Directory und Update-Queue räumt der Aufrufer auf (138 §4.8).

**K7 Schreiben in den Index.**

- Der Index serialisiert seine Writer selbst (086 §5). `index.write` direkt aufzurufen (119 §4, Schritt 1) ist zulässig.
- `IndexWriter` (086) liefert zusätzlich die Identität `(id, stamp)`. Er ist keine Voraussetzung für korrekte Writes.
- Der Stamp aus dem Rückgabewert eines Writes ist der einzige Stamp, der als Echo ins `ResidencyDirectory` eingetragen werden darf (100). 119 §4 tut das mit `set_known_stamp`. Das hat nur bei residenten Entries eine Wirkung. Neue GPU-Entries macht der gewrappte Index aus 138 §4.11 resident.

**K8 Bezeichner.**

- Es gibt drei Typen namens `IndexId`: `resource_requirements::IndexId` (`Clone + Eq + Hash`), `manager_lifecycle::IndexId(Arc<str>)` und der entfernte `residency::IndexId(u64)`. Die Specs ab 125 benutzen `resource_requirements::IndexId` als Schlüssel. Der `IndexManager` bekommt seinen eigenen `manager_lifecycle::IndexId` vom Aufrufer.
- `device_lane_event` ist der Modulname von `Event` und `Lane`. Die Kurzform `device` in 101, 102 und 074 meint dieses Modul.

## 5. Relied-on elements

Keine eigenen. Verwiesen wird auf die Specs 062, 074, 083, 086, 087, 100, 101, 102, 103, 108, 109, 114, 118 und 119 und auf deren Code.
