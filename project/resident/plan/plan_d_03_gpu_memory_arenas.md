# D3 — GPU-Speicher & Arenen

**Zweck:** Arenen nach Lebensdauer (Weights/Index-Segment/Workspace/Staging/ParamBlock), VRAM-Budget via `cuMemGetInfo`, stream-geordnete Wiederverwendung, Pinned Staging, dtype pro Device. **Konsolidiert mit E3** (eine Arena-Spezifikation).

**Einordnung:** Abhängt von B2 (Region/Layout/MemoryManager), D2 (Lane/Stream/Events). Downstream: D4 (Weights/Workspace), D6 (Workspace/ParamBlock), D7 (Index-Segment-Arena/Staging). Überlappt E3 (Gruppe E) — **dort nur referenzieren, nicht neu beschreiben**.

**Quellen:** `resident_subengine.md` §4.2; `resident_index.md` (Arena/Manager-Disziplin, via E3); `context/context_d.md` D3 (#9–#13, #10 Pools, #32/#33).

---

## Festgelegte Entscheidungen

- **Region-Modell (B2):** `Region{arena, offset, len, gen}`, kein Pointer nach außen; `gen` entwertet Handles nach Evict/Reload. `Layout{shape, strides, dtype}` mit eigenem DType-Enum.
- **Arenen nach Lebensdauer (§4.2):** Weights (langlebig), Index-Segment (gestreamt/evictable), Workspace/Scratch pro Lane (kurzlebig), Staging (Pinned Host), ParamBlock pro Lane (klein, fest).
- **Budget (`[V/U]`, #12):** `cudarc::driver::result::mem_get_info() → (free,total)`; **nicht** fix 6 GB. Realistisch nutzbar ~5–5,5 GB (Context 250–400 MB, Compositor 100–500 MB). Vor Ort messen. **2 Generationen** einplanen (CoW-Spitze, E3/D7).
- **Kein malloc/free im Betrieb (`[V/U]`, #10):** cudarc ≥0.12 nutzt `cuMemAllocAsync`/`cuMemFreeAsync` bei Pool-Support (Turing: ja); `cuMemPoolSetAttribute(RELEASE_THRESHOLD = u64::MAX)` behält Speicher. Zusätzlich **Bump-Arena** (ein großer Alloc, Offsets) für eigene Scratch-Buffer.
- **Pinned Staging (`[U]`, #11):** `ctx.alloc_pinned::<T>(n) → PinnedHostSlice`, `stream.memcpy_htod/dtoh` async. **Ring von Regionen**, jede mit Event; Region erst nach `event.synchronize()`/Completion wiederverwenden.
- **Stream-geordnete Wiederverwendung (§4.2):** Region wird erst frei/überschrieben, wenn alle nutzenden Streams ihr Event erreicht haben — Logik im **MemoryManager**, nicht in Ops.
- **dtype (`[V/U]`, #13):** GPU `F16`, CPU `F32`. BF16 auf Turing meiden (nicht nativ). Attention-Maske f16-sicher: **−1e4 statt −inf**. Erwartete Cosinus-Abweichung f16↔f32 ≈ 1e-3.
- **Fremde Allokation in candle (`[U]`, kritisch, #9/#33):** `CudaSlice` ist **owning** → Drop gibt frei. Empfehlung (Default): **Arena-Speicher nur für eigene Kernel**; candle-Tensoren über candles eigene (stream-ordered Pool-)Allokation, Daten per **D2D-Copy** austauschen. `wrap_cuda_slice`/`upgrade_device_ptr`+`leak` nur nach erfolgreichem **G-WRAP**-Prototyp.

---

## Prototyp-Gate (zuerst, blockierend)

- **G-WRAP:** Arena-Sub-Range als `CudaStorage` in candle einbinden **ohne Doppel-Free beim Tensor-Drop**. Prüfen: `wrap_cuda_slice(CudaSlice, CudaDevice)` vs. `stream.upgrade_device_ptr` + `CudaSlice::leak()`; beachten, dass Sub-Range nur `CudaView` (borrowed) ergibt, das candle nicht als Storage nimmt.
  - **Positiv:** Zero-Copy `encode_into` direkt in Arena möglich (Phase 3+).
  - **Negativ (Fallback):** candle rechnet in eigenem Pool, `WriteSlotOp` kopiert D2D in die Arena. **Design darauf auslegen, dass der Fallback trägt.**

---

## Zu erstellende Artefakte (`rag/src/memory/`)

- `mod.rs`
  - `trait MemoryManager`: `alloc(dev, bytes, align) -> Region`, `free(Region)`, `resolve(Region) -> *internal*`, `budget() -> Budget`.
  - `enum ArenaKind { Weights, IndexSegment, Workspace, Staging, ParamBlock }`.
  - `struct Budget { free, total, reserved_for_generations }`.
- `arena.rs`
  - `struct ArenaAllocator` — Bump/Free-List, Offsets `u64`, `gen`-Entwertung; pro `ArenaKind` eine Instanz. **Gemeinsamer Vertrag für D3 und E3.**
  - Stream-geordnete Freigabe: `release_after(region, event)`; Rückgabe erst nach Event.
- `staging.rs` (hinter `cuda`)
  - `struct StagingRing { regions: Vec<PinnedRegion>, events: Vec<EventHandle> }`, `acquire()`/`publish(event)`.
- `cuda.rs` (hinter `cuda`)
  - `CudaMemoryManager` über cudarc: Pool-Config (`RELEASE_THRESHOLD`), `mem_get_info`, `alloc_pinned`, D2D-Copy-Helfer.
- `cpu.rs`
  - `CpuMemoryManager` — einfache RAM-Arena, kein Staging/Stream (CPU-Pfad).

---

## Arbeitsschritte

1. **G-WRAP** durchführen, Ergebnis + gewählten Pfad (Zero-Copy vs. D2D-Fallback) dokumentieren.
2. `MemoryManager`-Trait + `Region`/`Budget`; `gen`-Entwertung implementieren.
3. `ArenaAllocator` pro `ArenaKind`; stream-geordnete Freigabe (`release_after`).
4. CUDA-Backend: Pool-Konfiguration, `mem_get_info`, Budgetreservierung für 2 Generationen.
5. `StagingRing` (Pinned) mit Event-pro-Region.
6. dtype-Politik verankern (f16/f32), f16-sichere Maskenkonstante (−1e4) als Modulkonstante.
7. Abgleich mit **E3**: sicherstellen, dass E3 diesen `ArenaAllocator` referenziert (keine Dopplung).

## Fehler (Q1) & Logging (Q2)

- `enum MemoryError { ArenaExhausted{arena, requested, free}, TransferFailed{dir, source}, StaleRegion{gen}, BudgetProbe }`.
- Span pro Arena/Allokation (`arena = ?kind`, `offset`, `len`, `gen`); Budget-Probes auf `debug`, Alloc/Free auf `trace`.

## Feature-Flags (Q4)

- `staging.rs`/`cuda.rs` hinter `cuda`; `CpuMemoryManager` immer verfügbar. Factory wählt Backend.

## Tests (Q3)

- CPU-Arena: Alloc/Free/Offsets/`gen`-Entwertung, `ArenaExhausted`.
- Stream-geordnete Freigabe gegen Mock-Events (keine Freigabe vor Event).
- Staging-Ring: keine Wiederverwendung vor Event-Completion (Mock).
- Budget-Mock: Reservierung 2 Generationen respektiert.
- CUDA-D2D/Pinned nur unter `cuda` + GPU.

## Definition of Done

- G-WRAP dokumentiert; `MemoryManager`/`ArenaAllocator`/`StagingRing` implementiert; Budget inkl. 2-Generationen-Reserve; dtype-Politik verankert; mit E3 konsolidiert (eine Spezifikation).

## Result

D3 ist umgesetzt, aber nur teilweise geprüft. Die 220 Lib-Tests laufen grün (31 davon im Memory-Modul), und `cargo check --features cuda --tests` baut. Auf dieser Maschine gibt es kein `nvcc`. Die GPU-Tests (`#[ignore]`) und das G-WRAP-Gate sind deshalb nicht gelaufen. Beim `cuda`-Build bleiben Dead-Code-Warnungen, weil D4, D6 und D7 `copy_d2d`, `upload` und `download` noch nicht aufrufen.

**Hänger:** Mein Test-Helfer `Gate` hatte einen Deadlock, der den `cargo`-Lauf blockierte. Die Lane wartete beim Drop auf den Job, während der Sender noch lebte. Ich habe die Feldreihenfolge geändert. Danach habe ich ohne `--offline` und mit Timeout gebaut.

**Neu in `rag/src/resident/memory/`:**
- `error.rs`: `MemoryError` und `TransferDir`. `ResidentError::Memory` ist aufgenommen, ebenso `IsRetryable`.
- `budget.rs`: `ArenaKind`, `ArenaSizes`, `Budget`, `BudgetProbe` und `check_fit`.
  - Der Plan wird gegen freien VRAM abzüglich 512 MiB Reserve geprüft, bevor allokiert wird.
  - `generation_reserve(n)` hält den Index-Anteil für die inaktiven Generationen zurück, standardmäßig zwei Generationen.
- `arena.rs`: `ArenaAllocator` mit `release_after(region, event)`, `ArenaSet` und der Trait `DeviceMemory`.
  - `ArenaAllocator` baut auf dem vorhandenen `BookkeepingArena` auf, es gibt also keine zweite Offset-Logik.
  - Eine Region geht erst nach Abschluss ihres Events an die Free-List zurück.
- `staging.rs`: `StagingRing<S>` mit `acquire`, `acquire_wait`, `publish` und `release`. Ein Slot wird erst nach dem Event seines letzten Transfers wiederverwendet.
- `cpu.rs`: `CpuMemoryManager` als RAM-Arena mit `read`, `write` und `copy`.
- `cuda.rs` (hinter `cuda`): `CudaMemoryManager` mit je einer Backing-Allokation pro Arena-Klasse.
  - Dazu kommen die Pool-Konfiguration (`RELEASE_THRESHOLD = u64::MAX`), `mem_get_info`, ein pinned Staging-Ring und die Helfer `copy_d2d`, `upload` und `download`.
- `dtype.rs`: `F16_MASK_VALUE = -1e4`.
- `subengine/device.rs`: `DeviceHandle::cuda_context()` (nur `pub(crate)`).
- `rag/docs/architecture/gpu_memory.md`: Arena-Vertrag, Budget und Stand von G-WRAP.

**Abweichungen vom Plan:**
- Der bestehende Trait `MemoryManager` kennt keine Arena-Klasse. Deshalb gibt es daneben `DeviceMemory` mit `alloc_in`, `release_after` und `budget`. `MemoryManager` habe ich nicht geändert.
- `StagingRing` ist generisch und ohne `cuda` testbar. Nur die pinned Variante (`PinnedStaging`) liegt hinter `cuda`.
- `copy_d2d` verlangt, dass Quelle und Ziel in verschiedenen Arena-Klassen liegen.

**G-WRAP:** Offen, das Gate braucht ein CUDA-Gerät. Bis dahin gilt der Fallback: Arenen nur für eigene Kernel, Candle rechnet in eigenen Allokationen, die Ergebnisse kommen per D2D-Copy in die Arenen. Nichts darf von Zero-Copy abhängen, bevor das Gate bestanden ist.

Die Config setzt `CUDA_PATH` und `CUDA_HOME` auf `/usr/local/cuda-13.4`. Auf der GTX 1660 SUPER laufen alle 34 Memory-Tests inklusive der drei vorher ignorierten GPU-Tests grün:
- `upload_copy_download_roundtrip`: H2D, D2D und D2H über Pinned-Staging.
- `oversized_plan_is_rejected_before_allocating`: ein zu großer Plan wird vor dem Allokieren abgelehnt.
- `cuda_lanes_order_work_through_events` aus D2.

Die Aussage „kein `nvcc`“ in meiner Zusammenfassung ist damit hinfällig. Der `cuda`-Teil ist auf echter Hardware geprüft.

G-WRAP ist weiter offen. Dafür fehlt nicht die Hardware, sondern der Prototyp selbst: ein Test, der prüft, ob Candle einen Arena-Bereich beim Drop doppelt freigibt. Dieser Test existiert noch nicht. Soll ich ihn jetzt schreiben und laufen lassen?

Frei waren zuletzt nur etwa 4,9 GB von 6 GB. Das passt zur Budget-Annahme „nicht fix 6 GB“. Passen Sie den Hinweis in `gpu_memory.md` auf diesen Messwert an, wenn Sie das möchten.

G-WRAP ist durchlaufen und fällt negativ aus. Die drei Gate-Tests laufen auf der GTX 1660 SUPER grün, sie halten den negativen Befund fest.

Was die Tests zeigen:
- **Lesen geht:** Ein Arena-Bereich lässt sich über `upgrade_device_ptr` und `wrap_cuda_slice` als Candle-Tensor einbinden, und `to_vec1` liefert die richtigen Werte.
- **Drop geht nicht:** Beim Drop gibt Candle den Zeiger frei, und der nächste `synchronize` liefert `CUDA_ERROR_INVALID_VALUE`, der Kontext ist vergiftet. Candle bietet keinen Weg, die Storage vorher zurückzunehmen, `storage_mut_and_layout` ist nicht öffentlich.
- **`std::mem::forget` ist nur begrenzt brauchbar:** Es vermeidet den Fehler, leckt aber pro Tensor den Host-Wrapper und zwei CUDA-Events. Das taugt höchstens für wenige langlebige Lese-Views bis zum Shutdown.
- **Kein Zero-Copy-Output:** Candle-Ops legen ihren Output selbst an. Der einzige In-place-Weg, `slice_set`, ist selbst eine Kopie.

Damit gilt der D2D-Fallback endgültig: Arenen bedienen nur eigene Kernel, Candle rechnet in eigenen Allokationen, und Ergebnisse kommen per `copy_d2d` in die Arena. `gpu_memory.md` ist aktualisiert.

Nicht prüfbar war das In-place-Schreiben über einen gewrappten Tensor. Candle-Kernels laufen hier nicht: `CUDA_ERROR_UNSUPPORTED_PTX_VERSION`. Der Treiber (610.57.04) unterstützt CUDA 13.3, das Toolkit in der Config ist 13.4. Das betrifft auch D4 und D6, sobald sie Candle-Kernels ausführen. Dafür braucht es ein Toolkit höchstens in Treiberversion (`CUDA_PATH` und `CUDA_HOME` in `.cargo/config.toml`) oder einen neueren Treiber.

Tests, die Candle linken, brauchen `LD_LIBRARY_PATH=/usr/local/cuda-13.4/lib64`, sonst findet der Lader `libcurand.so.10` nicht. Der Befehl steht im Dokument.
