# Umsetzungsplan E4 · Manager-Disziplin (Hochrisiko)

> **Quellen:** `resident_index.md` (Manager-Disziplin). **Recherche:** `context_e.md` §E4 (Punkte 13–17, **Hochrisiko**), ergänzend `context_h_q.md` H1 (3–5). **Querschnitt:** Q1, Q2, Q3, Q4.
> **Abhängt von:** E1, E3, B5 (Kernel-Trait). **Ergebnis:** Mutations-/Swap-Disziplin.

## 1. Ziel & Abgrenzung
Die Disziplin, mit der der Manager **nie den Kernel blockiert**: Mutation nur in **Lücken** (Batch-Ende/Kommunikationsslot), **atomarer 8-Byte-Root-Pointer-Swap**, **Copy-on-Write-Hierarchie** (Root → Segmenttabelle → Segment), definierter **Pointer-Lesezeitpunkt** (Launch-Arg vs. Acquire-Load im Ringmodus), **Event/Fence-Sichtbarkeit** Host↔GPU, blocking Reads im Worker-Pool.

Nicht hier: die feste Slot-Reihenfolge selbst (E5), Residency (E7).

## 2. Voraussetzungen
- E1 Manager-Rahmen; E3 Arena (`u64`-Offsets, CoW-Budget); B5 Kernel-Trait (Batch-Start/-Ende, Fortschrittszähler).
- Konsistenzmodell aus `plan_e_overview.md` §1 (RCU + Epoche, Stamp-`!=`).

## 3. Zu bauende Artefakte
```rust
pub struct RootPointer(AtomicU64);          // 8-Byte, naturally aligned, in pinned+mapped Host-Memory
pub struct SegmentTable { /* CoW, immutable nach Publish */ }
pub struct EpochTracker { published: u64, reader_min: AtomicU64 } // Kernel meldet gelesene Epoche je Batch

pub trait TableSwap {
    fn build_next(&self) -> SegmentTable;            // CoW neue Tabelle
    fn publish(&self, table: SegmentTable);          // (1) Inhalt sichtbar machen, (2) Pointer mit Release swappen
    fn reclaim(&self, epoch: u64);                   // alte Version frei, wenn alle Reader >= epoch
}
```

## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
**`ld.acquire`/Fence auf sm_75** (context_e.md §E4 Punkte 13–15, H1 Punkt 5): PTX-Scoped-Memory-Modell mit `ld.acquire`/`st.release`/`fence.sc|acq_rel` ab **sm_70 (Volta)**; Turing (sm_75) gehört dazu. Syntax `ld.acquire.sys.u32 %r, [addr];`. Host↔Device erfordert Scope **`.sys`**. `__threadfence()` ist nur device-scope (`membar.gl` = `fence.sc.gpu` ab sm_70) — **genügt bei Host-Beteiligung nicht**. ABI: Release-Store = `st.release.<scope>`, Acquire-Load = `ld.acquire.<scope>` (empfohlen) bzw. `cuda::atomic_ref<…, thread_scope_system>`.

**Sichtbarkeits-Reihenfolge vor Pointer-Swap** (context_e.md §E4 Punkt 14, `[W]` aus `[V]` abgeleitet): (1) Tabelle schreiben/kopieren → (2) Abschluss sicherstellen (`cudaStreamSynchronize`/Event bei Copy-Stream; bei gemapptem Host-Memory Release-Store) → (3) Pointer mit **Release** publizieren → (4) Kernel liest Pointer mit **Acquire (.sys)**. Erst danach sind alle vor dem Release geschriebenen Daten sichtbar. Eine `cudaMemcpyAsync` in anderem Stream wird für einen **laufenden** persistenten Kernel erst durch Host-Event/Flag-Publish zu definiertem Zeitpunkt sichtbar (Streamordnung gilt nicht zwischen konkurrierenden Streams).

**Atomarer 8-Byte-Swap Host↔GPU** (context_e.md §E4 Punkt 15, `[W/?]`): **8-Byte-aligned `AtomicU64`** in **pinned+mapped** Host-Memory (`cudaHostAllocMapped`); Host `store(Release)`, GPU `ld.acquire.sys`. Reine Load/Store-Atomarität für naturally-aligned 8 B ist gegeben. **Single-Writer-Store (nur Host schreibt)** — **kein** GPU-seitiges RMW/CAS über PCIe nötig (PCIe-Atomics plattformabhängig). Unified/Managed mit gleichzeitigem CPU/GPU-Zugriff unter Windows nicht verfügbar → **mapped pinned** verwenden.

**Lock-free-Muster (Reader=GPU, Writer=Host)** (context_e.md §E4 Punkt 16, `[W]`): **Immutable Segmente + CoW** (RCU-artig): neue Segmenttabelle bauen, Root-Pointer **einmal** swappen; alte Version erst freigeben, wenn kein Reader mehr darauf zeigt. **Epochenzähler:** Kernel meldet pro Batch die gelesene Epoche (Fortschrittszähler); Host gibt frei, sobald **alle Reader ≥ neue Epoche**. **Nie in-place mutieren** (Ausnahme: dokumentierter persistenter In-Place-Update, der halb aktualisierte, aber gültige Werte erlaubt).

**Pointer-Lesezeitpunkt** (context_e.md §E4 Punkt 17, `[W]`): **Normale Launches:** Pointer als **Launch-Argument** → konsistenter Snapshot pro Launch, Wechsel ab nächstem Launch. **Persistenter Kernel:** Acquire-Load **einmal pro Batch** in ein Register, nicht pro Work-Item mitten im Batch; Mischzustände innerhalb eines Batchs vermeiden; `volatile`/relaxed-Loads ohne Acquire vermeiden (Compiler-Caching).

## 5. Umsetzungsschritte
1. `RootPointer` als `AtomicU64` in pinned+mapped Region (hinter `cuda`); ohne Feature als normaler `AtomicU64` im Host-Mock.
2. `SegmentTable` immutable nach Publish; `build_next` erzeugt CoW-Kopie (nutzt E3-Arena, serielle CoW).
3. `publish`: Reihenfolge (1)–(4) strikt; Copy-Stream-Event abwarten, dann Release-Store.
4. `EpochTracker`: Kernel-Batch meldet gelesene Epoche (B5-Fortschrittszähler); `reclaim(epoch)` gibt alte Tabelle/Segmente frei, wenn `reader_min ≥ epoch`.
5. Normale Launches: Pointer als Launch-Arg; persistenter Modus (Hook zu H1): Acquire-Load einmal pro Batch.
6. Blockierende Index-Reads (Page-Faults/I/O) in Blocking-/Worker-Pool; `madvise(WILLNEED)` für Überlappung.

## 6. Fehler / Logging / Feature-Flags
- **Q1:** `StaleMapping` (nur intern; interner Retry), `TransferFailed`, `ArenaExhausted` (CoW-Spitze).
- **Q2:** Span pro Manager-Zyklus; Events: Publish (Epoche, Tabellengröße), Reclaim (freigegebene Epoche), gelesene Reader-Epoche. CUDA-Host-Callbacks: **keine** CUDA-API/kein Logging darin — Daten per Channel/Atomic an Logging-Thread (H1 Punkt 32/Q2).
- **Q4:** Acquire-Load/Release-Store/pinned-mapped hinter `cuda`; Host-Mock bildet Release/Acquire mit `std::sync::atomic` nach (x86 TSO) für Logiktests.

## 7. Tests (Q3)
- **CoW-Swap:** altes Segment wird erst frei, wenn kein **vor** dem Swap eingereihter Request es mehr referenziert (Epoch-Reclaim), `FakeKernel` meldet Epochen.
- Pointer-Lesezeitpunkt: Launch-Arg-Snapshot bleibt konstant während eines Batchs; Swap wirkt ab nächstem Launch.
- Reihenfolge Publish (1)–(4): Inhalt vor Pointer sichtbar (Host-Mock mit erzwungenem Interleaving / `loom`).
- Epoch-Reclaim: Freigabe erst bei `reader_min ≥ published`.
- `loom`/`shuttle` auf dem Swap-/Reclaim-Protokoll (kleines CAS-/Store-Protokoll).

## 8. Offene Verifikationen (zuerst, Hochrisiko)
- **`ld.acquire.sys` und mapped-Atomics** direkt in der PTX-ISA (parallel-thread-execution) bestätigen **und auf der echten GTX 1660 testen** (context_e.md offener Punkt 1).
- `cudaDevAttrHostNativeAtomicSupported` prüfen (H1 Punkt 4) — bestätigt die Single-Writer-Store-Strategie (kein GPU-CAS über PCIe).
- Event-/Fence-Sichtbarkeit zwischen Copy-Stream und persistentem Kernel auf Zielhardware messen.

## 9. Definition of Done
- CoW + atomarer Root-Swap + Epoch-Reclaim implementiert und per `loom`/Host-Mock verifiziert; Publish-Reihenfolge erzwungen; Pointer-Lesezeitpunkt beider Modi korrekt; reale Atomics hinter `cuda`, Logik ohne GPU grün.
