# D2 — Device & Ausführung (`DeviceHandle`, `ExecContext`/Lane)

**Zweck:** Ausführungskontext je Subengine: `DeviceHandle`, `ExecContext`/Lane (Stream, Priorität, Events, ParamBlock, Scratch), Lane-Klassen (Query/Index/Copy), CUDA-Init-Kosten.

**Einordnung:** Abhängt von D1. Downstream: D3 (Arenen/Staging an Lanes), D4 (Encoder läuft auf Lane), D6 (Kernel-Launch auf Lane), D7 (Copy-Lane für Segment-Sync). Querschnitt Q1–Q4.

**Quellen:** `resident_subengine.md` §4.1, §2 (ExecContext-Invariante); `context/context_d.md` D2 (#5–#8); `context_h_q.md` (#7 cudarc/candle).

---

## Festgelegte Entscheidungen

- **Invariante:** Jede GPU-Arbeit läuft über einen `ExecContext`, **nie** auf dem Default-Stream (§2).
- **Eigener Stream (`[U]`, #5):** candle-core ≥0.9 nutzt cudarc ≥0.16 (`CudaContext`/`CudaStream`). `CudaDevice::new_with_stream(ordinal)` **prüfen** (G-STREAM). Events über cudarc: `stream.record_event(flags) → CudaEvent`, `event.synchronize()`, `other_stream.wait(&event)`. Candle-Ops laufen auf dem **Device-Stream** — nicht annehmen, dass sie auf dem Default-Stream bleiben.
- **Primary Context / Pointer (`[V/U]`, #6):** cudarc retained den Primary Context; candle teilt ihn **bei identischer cudarc-Version** (siehe G-ALIGN). Pointer: `CudaStorage → CudaSlice<T> → DevicePtr::device_ptr(&stream)`. Stream-Sync **manuell per Event**.
- **Init-Kosten (`[U]`, #7):** erste Context-Erzeugung 0,2–2 s (kalt mehr). Context beim Start **warmlaufen** (Dummy-Alloc), `CUDA_MODULE_LOADING=LAZY` setzen, Modell-Init separat messen. Lazy-Init (D1) muss diese Kosten einplanen.
- **Lane/Prioritäten (`[U]`, #8):** cudarc-safe hat **keine** Prioritäts-API → `sys::cuStreamCreateWithPriority` + `cuCtxGetStreamPriorityRange`. Turing unterstützt Stream-Prioritäten (z. B. 0…−5), wirken nur aufs **Block-Scheduling**, keine Preemption laufender Blöcke.
- **ExecContext =** `{ stream, events, pinned staging (D3), ParamBlock (Device-Buffer), scratch-Arena }`.
- **CPU-Lane:** Lane = Worker-Gruppe im eigenen Pool, **gleiche Schnittstelle**, ohne Streams/Staging.

---

## Zu erstellende Artefakte (`rag/src/device/`)

- `mod.rs`
  - `struct DeviceHandle` — Ordinal bzw. CPU; hält (hinter `cuda`) `CudaContext`/Device.
  - `trait ExecContext` (objektsicher, GPU-frei): `submit(op)`, `record_event() -> EventHandle`, `wait(&EventHandle)`, `param_block() -> &mut ParamBlock`, `scratch() -> &ScratchArena`.
  - `enum LaneClass { Query, Index, Copy }` + Prioritäts-Mapping.
  - `struct EventHandle` (opak; kein Raw-Handle nach außen).
  - `struct ParamBlock` (kleine feste Region für dynamische Kernel-Parameter; aus D3-Arena).
- `cpu.rs`: `CpuLane` (Worker-Gruppe) implementiert `ExecContext` ohne Streams; Events = no-op/Join-Barrieren.
- `cuda.rs` (hinter `#[cfg(feature = "cuda")]`):
  - `CudaLane { stream, priority, events }` über cudarc; Prioritäts-Stream via `sys::cuStreamCreateWithPriority`.
  - `fn warmup(&self)` (Dummy-Alloc, Modul-Load).
  - Event-Record/Wait-Mapping; Device-Pointer-Zugriff nur **intern**.

---

## Prototyp-Gates (zuerst)

1. **G-ALIGN:** `cargo tree -i cudarc` / `-d` → identische cudarc-Version candle↔Projekt; sonst `candle_core::cuda_backend::cudarc` re-exportieren.
2. **G-STREAM:** `CudaDevice`/`new_with_stream` + Event record/wait auf candles Stream verifizieren.
3. **G-PTR:** Device-Pointer eines Candle-Tensors in eigenem Trivial-Kernel lesen (Primary Context geteilt).

Ergebnis jedes Gates dokumentieren; bei Fehlschlag Fallback (eigener Context/manuelle Pointer-Pflege) festhalten.

---

## Arbeitsschritte

1. Gates G-ALIGN/G-STREAM/G-PTR durchführen und dokumentieren.
2. `DeviceHandle` für CPU und CUDA; Context-Warmup + `CUDA_MODULE_LOADING=LAZY`.
3. `ExecContext`-Trait definieren (GPU-frei), `CpuLane` + `CudaLane` implementieren.
4. Lane-Klassen Query/Index/Copy mit Prioritäten anlegen; Übergaben **nur** per Event.
5. `ParamBlock`/`ScratchArena` an Lane binden (Regions aus D3).
6. Init-Kosten messen (Context, Modell separat) und als Metrik an D1-Lazy-Init melden.

## Fehler (Q1) & Logging (Q2)

- `enum DeviceError { Init{source}, StreamCreate, EventRecord, PriorityUnsupported, VersionMismatch }`.
- Span pro Lane (`lane = ?class`, `stream_prio = n`); Events `warmup`, `event_record/wait`. **Keine CUDA-API in Host-Callbacks** (#32).

## Feature-Flags (Q4)

- `cuda.rs` komplett hinter `cuda`. Backend-Factory wählt `CpuLane`/`CudaLane`. No-GPU-Build nutzt nur `CpuLane`.

## Tests (Q3)

- `CpuLane` voll gegen `ExecContext`-Trait (Reihenfolge, Event-Barrieren als Join).
- CUDA-Tests nur unter `cuda`-Feature + vorhandener GPU (ignorierbar in CI ohne GPU).
- Mock-Lane für Downstream (D4/D6) ohne echten Stream.

## Definition of Done

- Gates dokumentiert; `ExecContext`-Trait + CPU/CUDA-Lanes; Prioritäts-Streams gesetzt; Warmup vorhanden; keine Raw-Pointer/`Tensor` in öffentlichen Signaturen.

## Result

D2 ist umgesetzt, aber ich konnte weder `cargo build` noch `cargo test` ausführen. Geprüft ist nur die Syntax der geänderten Dateien. Die Gates G-STREAM und G-PTR sind nicht durchlaufen, weil dafür eine GPU nötig ist.

Das Ergebnis liegt in `rag/src/resident/subengine/`: `exec.rs` ist neu, `device.rs`, `error.rs` und `mod.rs` sind erweitert. In `resident/error.rs` kommt `DeviceError` als neue Variante in `ResidentError` hinzu.

- **Pfade:** Die Pfade weichen vom Plan ab (`device/mod.rs`, `cpu.rs`, `cuda.rs`). Ich habe die Struktur genommen, die schon im Crate liegt.
- **`submit`:** `submit` nimmt einen Closure (`LaneJob`) statt eines `Op`, weil es noch keine Ausführungsschicht für Ops gibt.
- **`param_block`:** `param_block()` braucht `&mut self`, weil eine Lane exklusiv einem Worker gehört.
- **Warteregel:** Eine CUDA-Lane, die auf ein CPU-Event wartet, blockiert den Aufrufer, weil ein Stream kein Host-Event abwarten kann.
- **Nicht gebaut:** `ParamBlock` und `ScratchArena` arbeiten nur auf `Region`-Buchhaltung und sind noch nicht mit der GPU-Arena aus D3 verbunden.

Zum Verhalten:
- **`CpuLane`:** Sie läuft auf einem eigenen Worker-Thread und führt Jobs in Einreichungsreihenfolge aus.
- **Fehler:** Jobfehler und Panics werden festgehalten, und `synchronize` meldet den ersten davon.
- **`CudaLane`:** Sie nutzt `new_stream_with_priority` aus cudarc 0.19.9 und begrenzt die Priorität auf den Bereich des Geräts. Query ist am dringendsten, Copy mittel, Index normal.
- **`DeviceHandle`:** Der Handle setzt `CUDA_MODULE_LOADING=LAZY`, misst die Init-Kosten und hat `warmup()`. Ohne das Feature `cuda` gibt `cuda()` `CudaUnavailable` zurück.
- **Tests:**
  - Neue Unit-Tests liegen in `exec.rs` (Reihenfolge, Events, Fehler, Param/Scratch) und in `device.rs`.
  - Ein cuda-Test prüft G-ALIGN über `TypeId`-Gleichheit der cudarc-Typen von candle und Projekt.
  - Ein weiterer cuda-Test ist als `#[ignore]` markiert, weil er eine GPU braucht.
- **Mock:** `RecordingLane` ist als Lane-Mock für D4 und D6 gedacht.
