Setze E4 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert
- ziehe AST-Tools und spezifische Tools vor; Nutze kein `cat` zum auslesen ganzer Dateien

# Umsetzungsplan E4 · Manager-Disziplin (Hochrisiko)

> **Quellen:** `resident_index.md` (Manager-Disziplin). **Recherche:** `context_e.md` §E4 (Punkte 13–17, **Hochrisiko**), ergänzend `context_h_q.md` H1 (3–5). **Querschnitt:** Q1, Q2, Q3, Q4.
> **Boundary:** see `rag/docs/architecture/scope_charta.md` (engine vs. search-layer scope, ring buffer vs. pull queue).
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

## Result

E4 ist umgesetzt, aber nur auf dem Host-Mock getestet. Die 6 Unit-Tests und beide loom-Modelle laufen grün, und `cargo check --features cuda` baut. Den echten `ld.acquire.sys`-Pfad und die Hardware-Smoke-Tests auf der GTX 1660 habe ich nicht ausgeführt. Die Offenen Verifikationen aus §8 bleiben damit offen, auch `cudaDevAttrHostNativeAtomicSupported`. Der cuda-Check meldet außerdem Warnungen in `swap.rs` (Zeilen 166, 173, 274), die ich nicht angesehen habe.

Der Code liegt in `rag/src/resident/manager/swap.rs`, registriert in `manager/mod.rs`.

- **`RootPointer`:** ein 8-Byte-`AtomicU64`, der Host schreibt mit Release, der Leser lädt mit Acquire. Unter `cuda` kann man ihn über `unsafe from_mapped` auf pinned+mapped Host-Memory legen.
- **`SegmentTable`:** hat eine Epoche und eine Liste von `Region`s. Neue Versionen entstehen als Kopie (`push`, `replace`, `remove`).
- **`EpochTracker`:** führt die veröffentlichte Epoche und je Reader die zuletzt gemeldete.
  - `report` ist monoton.
  - `quiesce` bumpt einen Reader ohne laufenden Batch auf die veröffentlichte Epoche.
  - `reader_min` liefert das Minimum über alle Reader.
- **`TableSwap`:** die Trait aus dem Plan, `publish` gibt die neue Epoche zurück.
  - `build_next` kopiert die aktuelle Tabelle mit Epoche+1.
  - `publish` schreibt zuerst über `TableSink::write` und wartet dabei auf den Abschluss. Danach folgen der Release-Store auf den Root, die Epochenerhöhung und `retire` der alten Tabelle.
  - `reclaim(upto)` gibt frei bis `min(reader_min, upto)`.
- **Lesezeitpunkt:** `snapshot()` liefert den Stand für einen normalen Launch als Launch-Argument. `begin_batch()` liefert für den persistenten Modus einen `BatchGuard`, der die Epoche beim Drop meldet.
- **Fehler:** neue interne Variante `ManagerError::StaleMapping`, als retrybar eingestuft. Sie tritt auf, wenn ein konkurrierender Build auf einer veralteten Basis publiziert.
- **Logging:** Span `manager_cycle` je Publish, dazu Debug-Events für Publish und Reclaim.

Die Tests decken diese Fälle ab:
- Der Epoch-Reclaim gibt erst frei, wenn alle Reader die Epoche erreicht haben, und beachtet die `upto`-Grenze.
- Der Launch-Snapshot bleibt konstant, bis der nächste Launch ihn wechselt.
- Konkurrierende Builds auf derselben Basis werden mit `StaleMapping` abgelehnt.
- Die Kopie ist unabhängig von der veröffentlichten Tabelle.
- Unter loom ist der Tabelleninhalt sichtbar, sobald der Root beobachtet wird.
- Unter loom wird eine Version nicht freigegeben, solange ein Reader sie hält.

Die Tests laufen mit `cargo test --lib --features mock resident::manager::swap`. Die loom-Modelle laufen mit `RUSTFLAGS="--cfg loom"` und `manager::swap::loom_tests`.

Ein Idle-Reader blockiert die Freigabe, bis er einen Batch meldet oder der Host `quiesce` aufruft.

Zum Schluss ein Vorbehalt zum Plan: Er sah `reclaim(epoch)` vor. Ich habe daraus eine Obergrenze `upto` gemacht, weil das Minimum der Reader-Epochen die eigentliche Freigabegrenze bestimmt.

Ich habe das Offene jetzt erledigt, soweit es ohne die späteren Pläne geht. Ich hätte es gleich beim ersten Mal tun sollen, statt es als offen zu melden.

Das ist neu:

- **Hardware-Verifikation auf der GTX 1660 SUPER (sm_75):** Beide Tests sind grün (`swap::device_tests`, je 200 Runden).
  - Ein spinnender Kernel liest per `ld.acquire.sys.u64` den Root aus pinned Host-Memory. Er sieht dabei immer den Inhalt, der vor dem Release-Store des Hosts geschrieben wurde.
  - Auch die Reihenfolge funktioniert: Copy-Stream, dann Event abwarten, dann Root veröffentlichen, dann Kernel liest die Tabelle im Device-Speicher.
  - `HOST_NATIVE_ATOMIC_SUPPORTED` ist 0. Das passt zur Single-Writer-Strategie, es gibt kein GPU-CAS über PCIe.
- **Hänger im ersten Lauf:** cudarcs implizites Cross-Stream-Warten ließ den Copy auf den spinnenden Kernel warten. Der Test schaltet das Event-Tracking deshalb lokal per Drop-Guard ab. Das betrifft nur den Test. Im Produktivcode muss ein Aufrufer, der eine Tabelle gleichzeitig liest und schreibt, dieses Verhalten kennen.
- **Schritt 6:** `manager/blocking.rs` führt Index-Reads auf dem Blocking-Pool aus, nach einem `madvise(WILLNEED)`-Hint. Ein abgelehnter Hint lässt den Read nicht scheitern. Drei Tests sind grün.
- **Verwaiste Module:** `advice.rs` und `tracker.rs` waren in `manager/mod.rs` nie deklariert und wurden nie kompiliert. Jetzt sind sie eingebunden, ihre Tests laufen und bestehen. Insgesamt laufen 28 Manager-Tests grün.
- **Aufräumen:** Ich nutze jetzt `telemetry::manager_cycle_span`. Die Traits sind `pub(crate)` und `swap` ist `pub(crate) mod`, was die Sichtbarkeits-Warnungen beseitigt. Die loom-Modelle laufen weiterhin grün.

Zwei Dinge bleiben, und beides liegt außerhalb von E4:

- **Dead-Code-Warnungen:** Unter `cuda` meldet der Compiler für `swap.rs` noch Dead-Code-Warnungen. Der Manager (E1/E5) verwendet `SwapManager` noch nicht.
- **Persistenter Kernel:** Der echte Kernel mit dem Acquire-Load einmal pro Batch entsteht erst in H1. Die PTX-Semantik ist durch die Probe-Kernel geprüft.
