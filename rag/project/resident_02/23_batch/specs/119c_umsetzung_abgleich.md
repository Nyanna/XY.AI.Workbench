# Spezifikation: Umsetzung des Abgleichs bis 119 (Build-Order 119c)

Crate-Module `segment_arena` (087), `allocator` (`AllocatorSet`) und `gpu_kernel` (118), jeweils nur ergänzt. Diese Spec setzt die Lesarten aus 119b um. Die noch offenen Punkte stehen in 119d.

## 1. Task, restrictions and boundaries

**Aufgabe**
- **C1:** `SegmentArena` erkennt Regionen, die am Arena vorbei freigegeben wurden, und entfernt sie aus seinem Zustand.
- **C2:** `SegmentEntry` und die Segmenttabelle bekommen einen sicheren Byte-Serialisierer.
- **C3:** `AllocatorSet` gibt den `Arc<Allocator>` einer Arena heraus, damit `Eviction`, `SegmentArena` und `TableSwap` denselben Allocator benutzen.
- Die entschiedenen Punkte aus 119b als Festlegung ausformulieren (F1) und die Tests dazu (T1 bis T3).

**Grenzen**
- Keine bestehende öffentliche Signatur ändert sich. Es kommen nur neue Methoden und Funktionen hinzu.
- Kein anderes Modul bis 119 wird verändert. Insbesondere bleiben `update_queue`, `communication_slot`, `manifest`, `eviction`, `address_map`, `table_swap` und `segment_writer` unverändert.
- Kein `unsafe`.
- 119b bleibt die Lesart der übrigen Specs bis 119. Diese Spec widerspricht ihr nicht.
- Was sich heute nicht entscheiden lässt, steht in 119d und nicht hier.

## 2. Context

- `Eviction` (103) entfernt ein Segment aus Tabelle und Address-Map und gibt die Region später über `AllocatorSet::allocator_for(..).free(..)` frei. Es ruft `SegmentArena::release` nicht auf (103 §1). Das ist so vorgesehen.
- `SegmentArena` (087) führt eine eigene Tabelle `live` (Region → Segment). Aus ihr kommen `largest()`, `live_bytes()` und `stats().live_segments`. Die Belegung (`occupied`) liest er dagegen vom Allocator und ist deshalb richtig.
- `largest()` geht in die Headroom-Prüfung von `allocate` ein (`concurrent_replacements × largest`). Bleibt ein längst freigegebenes großes Segment in `live`, ist die Headroom dauerhaft zu hoch. `allocate` meldet dann "Arena erschöpft", obwohl Platz da ist.
- `KernelParams` hat in `gpu_kernel` bereits einen Serialisierer (`to_bytes`, Native-Endian, ohne `unsafe`). `SegmentEntry` hat keinen. Der GPU-Manager (138 §4.10) braucht ihn, darf aber kein `unsafe` nutzen und soll das Layout nicht ein zweites Mal beschreiben.
- `AllocatorSet::new(bookkeeping, &[(Arena, DeviceKind)])` baut seine Allocatoren selbst und gibt mit `allocator(kind)` und `allocator_for(&region)` nur `&Allocator` heraus. `SegmentArena` und `TableSwap` brauchen aber einen `Arc<Allocator>`. Ohne Zugriff auf den `Arc` des Sets müsste man für dieselbe Arena einen zweiten `Allocator` bauen, und `Eviction` und `SegmentArena` hätten verschiedene Instanzen mit getrennten `pending`-Listen.

## 3. Interface

```rust
// segment_arena: keine neue öffentliche API. Die Bereinigung ist intern (§4, C1).

// gpu_kernel
impl SegmentEntry {
    pub const BYTES: usize = 40;
    /// Native-Endian, in Feldreihenfolge: base, lens_addr, stride_bytes, first_slot, slot_count, pad_len, _reserved.
    pub fn to_bytes(&self) -> [u8; SegmentEntry::BYTES];
}

/// Segmenttabelle für den Kernel: die Einträge hintereinander. Leere Eingabe ergibt einen leeren Vec.
pub fn encode_segment_table(entries: &[SegmentEntry]) -> Vec<u8>;

// allocator
impl AllocatorSet {
    /// Der Allocator dieser Arena als `Arc`. None, wenn die Arena nicht im Set ist.
    pub fn shared(&self, arena: ArenaKind) -> Option<Arc<Allocator>>;
}
```

## 4. Behavior and invariants

**C1 Bereinigung von `SegmentArena`**
- Eine Region in `live` gilt als freigegeben, wenn `allocator.is_live(&region)` false ist und die Region nicht zur laufenden Ersetzung gehört (`replacement.old` und `replacement.new`).
- Die Bereinigung entfernt solche Einträge aus `live` und läuft unter dem State-Lock:
  - in `allocate`, direkt nach `reap()` und vor der Berechnung von `largest()`;
  - in `budget()` und `stats()`.
- Nach der Bereinigung gelten `largest()`, `live_bytes()`, `stats().live_segments` und die Headroom für die tatsächlich lebenden Segmente.
- `release` eines bereinigten Segments liefert weiter `StaleRegion` (die Region ist am Allocator nicht mehr live).
- Zurückgestellte Einträge (`deferred`, `Readers::After`) und die laufende Ersetzung bleiben unberührt.
- Die Bereinigung ruft nur `Allocator::is_live` auf. Sie gibt nichts frei, publiziert nichts und verändert keine Belegung.
- Sie ist idempotent. Ohne am Arena vorbei freigegebene Regionen ist sie ein No-op, und `allocate`, `release`, `budget` und `stats` verhalten sich wie zuvor.
- Ein `debug!`-Log nennt die Zahl entfernter Einträge, wenn sie größer als 0 ist.

**C2 Serialisierer**
- `SegmentEntry::to_bytes` schreibt die Felder in dieser Reihenfolge und mit diesen Offsets: `base` bei 0, `lens_addr` bei 8, `stride_bytes` bei 16, `first_slot` bei 24, `slot_count` bei 28, `pad_len` bei 32, `_reserved` bei 36. Jeder Wert steht in Native-Endian (`to_ne_bytes`), wie bei `KernelParams`. Die Offsets sind dieselben wie die `offset_of!`-Prüfungen des Tests für `repr(C)`.
- `BYTES` ist 40 und gleich `size_of::<SegmentEntry>()`. Ein statischer Test prüft das.
- `encode_segment_table(entries)` liefert `entries.len() * 40` Bytes. Eintrag i belegt `[40 i, 40 i + 40)`. Die Funktion allokiert genau einmal.

**C3 `AllocatorSet::shared`**
- `shared(kind)` liefert einen Klon des `Arc`, den das Set intern für `kind` hält. Es ist dieselbe Instanz wie `allocator(kind)` und `allocator_for(&region)` für Regionen dieser Arena. Es entsteht kein zweiter `Allocator`.
- Eine Arena, die nicht im Set ist, ergibt `None`.
- Das Set bleibt nach der Konstruktion unveränderlich und `Send + Sync`. Das Klonen des `Arc` ändert weder Belegung noch `pending`.
- Der Aufrufer baut damit `SegmentArena::new(allocators.shared(kind)?, config)` und `TableSwap::new(allocators.shared(kind)?, sink, root)` und gibt dasselbe Set an `Eviction::new`. Alle drei sehen dieselbe Belegung und dieselbe `pending`-Liste.

**F1 Form eines Entries (entschiedener Teil).**
- Das Index-Format v1 hat einen Vektor je Entry: `IndexHeader::vector_bytes() == dimension × 2` (FP16).
- Für einen Index mit einem Vektor je Entry gilt im GPU-Manager (138) die Slot-Klasse `SlotClass::custom(1)` mit `tokens = 1`. Die Slot-Größe ist dann `dimension × Elementgröße`.
- Ein Vektor, dessen Länge ein Vielfaches `k × dimension × 2` ist, wird als Token-Matrix mit `k` Tokens behandelt, solange `k <= class.max_tokens()`. Die CPU-Subengine (137) bewertet dichte Vektoren und akzeptiert nur `k = 1`. Andere Längen stehen in `malformed`.
- Was ein `PerToken`-Entry genau ist, steht in 119d (P1).

**Tests**
- **T1 (C1):** Zwei Segmente anlegen (A groß, B klein). A direkt am Allocator freigeben. `stats().live_segments == 1`, `largest()` ist die Länge von B und die Headroom sinkt. Ein `allocate`, das vorher an der Headroom von A scheiterte, gelingt.
- **T1b (C1):** Ein Lauf mit `Eviction::make_room` und `reclaim` auf demselben Allocator, danach `allocate` im freien Bereich. Ergebnis: kein falsches "Arena erschöpft".
- **T1c (C1):** Ohne Freigabe am Arena vorbei bleiben alle bestehenden Tests unverändert grün.
- **T2 (C2):** `to_bytes` gegen die Offsets des `repr(C)`-Tests. Rundlauf: die Felder aus den Bytes mit `from_ne_bytes` zurückgelesen sind gleich dem Eintrag. `encode_segment_table(&[])` ist leer, bei drei Einträgen hat sie 120 Bytes.
- **T3 (C3):** `shared(kind)` und `allocator(kind)` zeigen auf dieselbe Instanz (`std::ptr::eq`). Eine über `shared` allokierte Region lässt sich über `allocator_for(&region)` freigeben, und `available()` stimmt danach. Eine unbekannte Arena ergibt `None`.

## 5. Relied-on elements

- **`allocator`:**
  - `Allocator::{new(Arena, DeviceKind, Arc<Bookkeeping>), allocate, free, free_deferred, reap, available, is_live, pending_count}`.
  - `AllocatorSet::{new(Arc<Bookkeeping>, &[(Arena, DeviceKind)]), allocator(ArenaKind) -> Option<&Allocator>, allocator_for(&Region) -> Result<&Allocator, AllocatorError>}`. Es hält je Arena einen `Arc<Allocator>` und ist unveränderlich.
  - `bookkeeping::Bookkeeping::new(Arc<ArenaSet>)`.
- **`segment_arena` (087):** der interne Zustand `live: map Region → Segment`, `deferred`, `replacement`, die Methoden `allocate`, `release`, `budget`, `stats` und `largest()`, und `SegmentArena::new(A: Borrow<Allocator>, SegmentArenaConfig)`.
- **`gpu_kernel` (118):** `SegmentEntry` (`repr(C)`, 40 Byte, `Clone, Copy, Debug, PartialEq, Eq`) und `KernelParams::to_bytes` als Vorbild für das Schreiben in Native-Endian.
- **`index_format`:** `IndexHeader::vector_bytes()`.
- **Specs:** 103 (Eviction gibt am Arena vorbei frei), 119b (Lesarten K1 bis K8), 119d (offene Punkte), 137 (CPU), 138 (GPU-Manager), 139 (GPU-Subengine).