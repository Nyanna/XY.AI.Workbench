# Umsetzungsplan E5 · Kommunikationsslot (feste Reihenfolge)

> **Quellen:** `resident_index.md` (Kommunikationsslot). **Recherche:** `context_e.md` §E5 (Punkte 18–20). **Querschnitt:** Q1, Q2, Q3.
> **Boundary:** see `rag/docs/architecture/scope_charta.md` (engine vs. search-layer scope, ring buffer vs. pull queue).
> **Abhängt von:** E4 (Mutationsdisziplin), B4 (Index), B5 (Kernel/`KernelHit`). **Ergebnis:** Slot-Protokoll-Spezifikation.

## 1. Ziel & Abgrenzung
Der **Kommunikationsslot** zwischen den Batches mit **fester Reihenfolge**:
1. **Kernel-Writes lesen** → gemeldete Bereiche den Entries zuordnen → `Index::apply` aufrufen → eigene Stamps eintragen.
2. **Ergebnisse auflösen** → Adresse → Entry per Mapping.
3. **Updates einspielen** → Update-Queue anwenden (verändert das Mapping, daher **zuletzt**).

Weil nur in Lücken mutiert wird, sieht jedes Ergebnis den Mapping-Stand **seines** Batches — **keine** Epoche/Sequenznummer im Kernel-Protokoll. Ergebnisstatus: entfernt → verworfen; Update anstehend → `outdated`. Nach außen `Hit{entry, score, outdated}`.

Nicht hier: Echo-Vermeidung/Stamp-Abgleich (E6), Indexaufbau-Writes im Detail (E8).

## 2. Voraussetzungen
- E4: Mutation nur in Lücken, Pointer-Swap, Epoch-Reclaim.
- B5: Kernel meldet geschriebene Bereiche + `KernelHit{offset,score}` am Batch-Ende; `SegmentDesc`/`Layout`.
- B4: `Index::apply(entry, data) -> Stamp`, `Index::stamp(entry) -> Option<Stamp>`.

## 3. Zu bauende Artefakte
```rust
pub struct Hit { pub entry: EntryId, pub score: f32, pub outdated: bool }

pub struct UpdateQueue { /* ausstehende Mutationen pro Entry */ }

pub struct AddressMap { /* Adresse(Arena-Offset) -> EntryId */ }
impl AddressMap {
    fn resolve(&self, off: u64) -> Option<EntryId>; // Fixed: off/stride; Variable: Binärsuche
}

pub fn run_slot(
    writes: &[WrittenRange], hits: &[KernelHit],
    index: &dyn Index, map: &mut AddressMap, updates: &mut UpdateQueue,
    tracker: &mut ResidencyTracker,
) -> Vec<Hit>;
```

## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
**Reihenfolge-Invarianz (Mapping-Stand pro Batch)** (`resident_index.md`): Schritt 1 **vor** Schritt 2 garantiert, dass jede vom Kernel gemeldete Adresse auflösbar ist — auch Adressen frisch geschriebener Segmente. Schritt 3 **zuletzt**, weil Updates das Mapping ändern. Der Manager repräsentiert den VRAM → gemeldete Adresse ist aktuell; **keine** Epoche/Sequenz im Protokoll.

**Doorbell/Mailbox** (context_e.md §E5 Punkt 18, `[W]`): Ring in mapped pinned Memory; Kernel schreibt pro Batch Fortschrittszähler (Release), Host pollt (Acquire) oder wartet auf `cudaEvent`. Host mutiert nur zwischen „Batch-Ende" und „nächster Batch-Start-Quittung". **Nie** auf Kernel-Ende warten.

**Adresse → Entry** (context_e.md §E5 Punkt 19, `[W]`): Bei **Fixed-Stride** direkt `offset / stride`. Bei **Variable** sortierte Offset-Tabelle (Prefix-Summe der Entry-Größen) + `slice::partition_point`/`binary_search` (O(log n)); alternativ `BTreeMap<u64, EntryId>` mit `range(..=addr).next_back()`.

**Indexieren während Suche** (context_e.md §E5 Punkt 20, `[W]`): Versionierte Mapping-Tabelle (immutable Snapshot pro Batch, Pointer-Swap an Batch-Grenze aus E4); Writer hängt neue Segmente **append-only** an; Reader sehen den Snapshot des Batch-Starts.

**Ergebnisstatus beim Auflösen** (`resident_index.md`): ID im Manager vorhanden, im Index entfernt (`stamp(entry)==None`) → **verworfen**. Update für den Entry in der Queue → `outdated=true` (Diagnose, kein Fehler). Sonst normales `Hit`.

## 5. Umsetzungsschritte
1. `AddressMap` mit Fixed- und Variable-Pfad (Prefix-Summen-Tabelle aus Indexstruktur, Build in E8).
2. `run_slot` in exakt drei Phasen; Phase 1 ruft `Index::apply` und trägt Rückgabe-Stamp in den Tracker (Echo-Vorbereitung E6).
3. Phase 2: `resolve` + Status (entfernt/outdated/normal) → `Hit`.
4. Phase 3: `UpdateQueue` anwenden (Mapping ändern), erst nach Phase 2.
5. Doorbell-Ring + `cudaEvent`-Warten hinter `cuda`; ohne Feature treibt `FakeKernel` die Phasen synchron.

## 6. Fehler / Logging / Feature-Flags
- **Q1:** `StaleMapping` (intern, bei inkonsistentem Snapshot → interner Retry im nächsten Slot); kein nach außen sichtbarer Fehler für `outdated`/verworfen.
- **Q2:** Events: Anzahl applied Writes, aufgelöste/verworfene/`outdated` Hits, Update-Queue-Tiefe. Span pro Slot-Zyklus.
- **Q4:** Doorbell/Event hinter `cuda`; Protokolllogik feature-frei.

## 7. Tests (Q3)
- **Reihenfolge:** Ergebnis auf **frisch geschriebene** Adresse löst korrekt auf (Schritt 1 vor 2).
- **Status:** entfernter Entry wird verworfen; ausstehendes Update markiert `outdated`.
- Variable-Layout: Binärsuche liefert korrekten Entry an Segmentgrenzen; Fixed: `off/stride`.
- Update-Last: Mapping-Änderung wirkt erst im nächsten Batch (Phase 3 zuletzt).
- Indexieren während Suche: Snapshot des Batch-Starts bleibt stabil, während Writer append-only ergänzt.
- `FakeKernel` liefert programmierte `writes`+`hits`; keine GPU.

## 8. Offene Verifikationen
- Doorbell-Ring-Sichtbarkeit (PCIe posted writes, `__threadfence_system`) auf Zielhardware (H1 Punkt 2/3).

## 9. Definition of Done
- `run_slot` hält die feste Reihenfolge; Adress-Rückabbildung für Fixed/Variable; Ergebnisstatus korrekt; frisch geschriebene Adressen auflösbar; alle Tests grün ohne GPU.

## Result

E5 ist in `rag/src/resident/manager/slot.rs` umgesetzt und in `manager/mod.rs` als `pub mod slot;` registriert. Die 6 neuen Tests laufen grün ohne GPU (`cargo test --lib resident::manager::slot`). Die übrigen 334 Tests habe ich nicht ausgeführt.

`run_slot` arbeitet in der festen Reihenfolge aus dem Plan:
1. **Kernel-Writes:** Jede geschriebene Adresse wird einem Entry zugeordnet, `Index::apply` wird aufgerufen und der Rückgabe-Stamp landet im `ResidencyTracker`. Schreibbereiche, die noch nicht zuordenbar sind, werden im nächsten Slot erneut versucht.
2. **Ergebnisse auflösen:** Ein Treffer wird verworfen, wenn seine Adresse nicht auflösbar ist oder `Index::stamp` den Wert `None` liefert. Steht für den Entry ein Update an, ist er `outdated`.
3. **Updates einspielen:** Die Queue wird zuletzt angewendet, erst Unbind, dann Bind. Eine Mapping-Änderung wirkt deshalb erst im nächsten Batch.

Adressen werden bei festem Layout mit `off / stride` aufgelöst, bei variablem Layout per Binärsuche über die Prefix-Summen. `AddressMap` ist append-only und per `clone()` als stabiler Snapshot nutzbar.

**Abweichungen vom Plan:**
- **`RangeSource`:** `run_slot` hat einen zusätzlichen Parameter `source: &dyn RangeSource`. Ohne ihn fehlen die geschriebenen Bytes für `Index::apply`. Im Test liefert ein Stub Nullbytes.
- **`StaleMapping`:** Ich habe keine neue Fehlervariante eingeführt. Der Retry-Mechanismus für nicht zuordenbare Writes ersetzt sie.

**Nicht umgesetzt:**
- **Doorbell-Ring und `cudaEvent`-Warten** hinter `cuda`: bleiben offen. Sie hängen an der offenen Hardware-Verifikation (PCIe posted writes, `__threadfence_system`).
- **Provisorische Mapping-Parameter:** Die Zuordnung der Segmente (`add_segment`, `Update`) ist als einfache API gebaut. E8 muss sie beim Indexaufbau noch anbinden.

Ich habe die ausgelassenen Punkte nachgezogen. Die gesamte Suite läuft grün: 346 Tests ohne GPU. Der `loom`-Test zum Doorbell läuft ebenfalls grün, und `cargo check --features cuda` läuft durch. Auf echter Hardware habe ich nichts getestet.

- **`FakeKernel`-Test:** `kernel_fake_drives_writes_then_hits` treibt `run_slot` mit dem vorhandenen `MockKernel`. Der Embedding-Modus liefert die Writes, der Scoring-Modus die Hits.
- **`SlotMap` statt Doppelung:** `AddressMap` baut jetzt auf `SlotMap` aus `segment/mapping.rs` auf. Meine eigene Layout-Implementierung ist entfernt. `SlotMap` hat dafür zwei neue Methoden, `slot_span` und `bounds`.
- **Einbindung in E4:**
  - `AddressMap` trägt eine Epoche und ist per `clone()` ein Snapshot des Batch-Starts.
  - Neue Segmente kommen per Copy-on-Write dazu.
  - `MappingStore` veröffentlicht nur die jeweils nächste Epoche.
  - `run_slot_at` prüft die Epoche gegen den `Snapshot` aus dem `SwapManager`.
- **`StaleMapping`:** Stimmen die Epochen nicht überein, kommt `ManagerError::StaleMapping` zurück, ohne Seiteneffekte. Der Aufrufer wiederholt den Slot dann mit frischem Snapshot.
- **Doorbell:** Neu in `manager/doorbell.rs`.
  - Das Gerät meldet fertige Batches per Release (`complete`).
  - Der Host fragt ohne zu blockieren ab (`poll`) und bekommt ein `Gap`.
  - Mit `close` quittiert er, und erst danach darf der nächste Batch starten (`may_start`).
  - `wait_gap` wartet auf ein Batch-Event, nie auf das Kernel-Ende.
  - Der Konstruktor für gemappten Pinned-Speicher liegt hinter `cuda`.
  - Die Sichtbarkeit über PCIe ist als offene Verifikation im Moduldoc vermerkt.

Beim Build erscheinen neue `dead_code`-Warnungen für `snapshot_at`, `publish` und `run_slot_at`. Der Manager, der sie aufruft (E1), ist noch nicht verdrahtet. Das entspricht den vorhandenen Warnungen bei Arena und Swap.
