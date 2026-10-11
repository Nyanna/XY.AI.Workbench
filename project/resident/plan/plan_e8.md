# Umsetzungsplan E8 · Kernel schreibt Segmente (Indexaufbau)

> **Quellen:** `resident_index.md` (Kernel schreibt Segmente, Rückabbildung). **Recherche:** `context_e.md` §E8 (Punkte 31–33). **Querschnitt:** Q1, Q2, Q3.
> **Abhängt von:** E5 (Slot-Reihenfolge), B5 (Kernel-Trait, `SegmentDesc`/`Layout`). **Ergebnis:** Indexaufbau-Pfad-Spezifikation.

## 1. Ziel & Abgrenzung
Der Pfad, auf dem der **Kernel neue Segmente schreibt**: Der Manager vergibt **vorab** einen beschreibbaren Bereich aus der Arena; der Kernel schreibt per **Cursor** und **allokiert nie**; bei Bereichsmangel **Teilergebnis**; Zuordnung im Slot (Schritt 1 aus E5); **Rückabbildung** Adresse→Entry (Fixed-Arithmetik / Variable-Binärsuche). Getrennte Ringe: **Embedding** liefert neue Segmente, **MaxSim** liefert Hits.

Nicht hier: der persistente Ringbuffer-Modus selbst (H1, nur Hook), die Slot-Reihenfolge (E5).

## 2. Voraussetzungen
- E5: feste Slot-Reihenfolge (Schritt 1 „Kernel-Writes lesen" vor Schritt 2 „Ergebnisse auflösen").
- E3: `reserve_writable(cap)`-Bereich, Pinned Staging, Copy-Stream.
- B5: Kernel-Trait meldet geschriebene Bereiche + Fortschrittszähler am Batch-Ende.

## 3. Zu bauende Artefakte
```rust
pub struct WriteRegion { base: u64, cap: u64 }        // vorab vergeben
pub struct WriteCursor; // GPU-seitig: off = atomicAdd(&cursor, size); off+size>cap => overflow
pub struct OffsetTable { offsets: Vec<u64> }           // exklusive Prefix-Summe (Variable-Layout)
impl OffsetTable { fn addr_to_entry(&self, off: u64) -> EntryId; } // partition_point
pub enum BuildRing { Embedding, MaxSim }
```

## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
**Output-Arena mit atomarem Cursor** (context_e.md §E8 Punkt 31, `[W]`): Host vergibt Region `(base, cap)`. Kernel: `off = atomicAdd(&cursor, size)`; `if off + size > cap` → **Überlauf-Flag** setzen, **nicht** schreiben (Teilergebnis bleibt gültig, Host vergibt nächsten Bereich und wiederholt). **Pro Warp/Block ein aggregierter `atomicAdd`** statt pro Thread. **Release-Fence/Flag nach dem Schreiben**, bevor der Host liest.

**Offset-Tabellen / Rückabbildung** (context_e.md §E8 Punkt 32, `[W]`; `resident_index.md`): **Variable** Längen → **exklusive Prefix-Summe** (`Thrust/CUB ExclusiveScan` auf GPU oder CPU) → Offset-Array; Lookup per **Binärsuche** (`partition_point`). **Fixed-Stride** lohnt, wenn Entry-Größen nahezu gleich (geringer Padding-Verlust) — dann `offset / stride`, keine Tabelle. Die Tabelle baut der **Manager** aus der Indexstruktur (E5 `AddressMap`).

**Getrennte Ringe Embedding vs. MaxSim** (context_e.md §E8 Punkt 33, `[W]`): Zwei unabhängige **SPSC**-Pfade. **Embedding-Ring** liefert neue Segment-Adressen/Mappings, **MaxSim-Ring** liefert Hits. Weil in E5 **Schritt 1 (neue Segment-Adressen in den Snapshot übernehmen) vor Schritt 2 (Hit-Adressen gegen diesen Snapshot auflösen)** liegt, ist jede Hit-Adresse auflösbar — **auch während zeitgleich indexiert wird**.

**Schreiben in D7-Slots:** Manager vergibt Slot vorab; Encoder/Kernel schreibt direkt; **Gültigkeitsbit erst nach dem Event**. Upsert = neuer Slot, dann Bit des alten löschen (kein In-Place während laufender Kernel). Zuordnung `(segment, slot) → EntryRef` macht der Manager auf der CPU.

## 5. Umsetzungsschritte
1. `reserve_writable(cap)` aus E3; `WriteRegion` an den Kernel übergeben.
2. Cursor-Protokoll (GPU): aggregierter `atomicAdd`, Überlauf-Flag, Release-Fence vor Host-Read (hinter `cuda`; Host-Mock simuliert Cursor + Überlauf).
3. Teilergebnis-Handling: bei Überlauf nächste Region vergeben und wiederholen; Teilergebnis behält Gültigkeit.
4. Im Slot (E5, Schritt 1): gemeldete Bereiche den Entries zuordnen, `OffsetTable`/Fixed bauen, `Index::apply` aufrufen.
5. Getrennte `BuildRing::Embedding`/`MaxSim`-Pfade; Embedding-Writes vor MaxSim-Auflösung sichtbar machen (Reihenfolge-Garantie aus E5).
6. D7-Slot-Schreiben: Gültigkeitsbit nach Event; Upsert über neuen Slot.
7. **H1-Hook:** `BuildRing`-Schnittstelle so lassen, dass getrennte SPSC-Ringe (persistenter Modus) ohne API-Bruch andocken.

## 6. Fehler / Logging / Feature-Flags
- **Q1:** `ArenaExhausted` (kein Folgebereich), `TransferFailed`. Überlauf ist **kein** Fehler (Teilergebnis).
- **Q2:** Events: vergebene Region (base/cap), Überlauf + Folgereservierung, applied Segmente, Ring-Typ.
- **Q4:** Cursor/atomicAdd/Fence hinter `cuda`; Offset-Tabellen-Bau und Rückabbildung feature-frei; Host-Mock simuliert Writes+Überlauf.

## 7. Tests (Q3)
- Teilergebnis bei Bereichsmangel: Überlauf-Flag, Host vergibt neuen Bereich, erster Teil bleibt gültig.
- Rückabbildung: Variable-`OffsetTable` Binärsuche korrekt an Grenzen; Fixed `off/stride`.
- **Indexieren während Suche:** Embedding-Writes (Schritt 1) sind für MaxSim-Auflösung (Schritt 2) desselben Slots sichtbar → jede Hit-Adresse auflösbar.
- Getrennte Ringe arbeiten unabhängig (SPSC-Fakes).
- D7-Upsert: neuer Slot gültig, alter Slot-Bit gelöscht; kein In-Place.

## 8. Offene Verifikationen
- Aggregierter `atomicAdd` pro Warp/Block und `__threadfence`-Sichtbarkeit auf GTX 1660.
- CUB/Thrust `ExclusiveScan`-Verfügbarkeit im Build (sonst CPU-Prefix-Summe).

## 9. Definition of Done
- Cursor-Write mit Teilergebnis, Offset-Tabellen/Fixed-Rückabbildung, getrennte Embedding/MaxSim-Ringe mit Reihenfolge-Garantie, D7-Upsert; H1-Hook offen; alle Tests grün ohne GPU.
