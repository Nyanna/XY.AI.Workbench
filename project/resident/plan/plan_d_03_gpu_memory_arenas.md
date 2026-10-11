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
