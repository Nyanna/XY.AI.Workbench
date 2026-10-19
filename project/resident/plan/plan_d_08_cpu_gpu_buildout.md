Setze D8 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert
- ziehe AST-Tools und spezifische Tools vor

# D8 — CPU-vs-GPU-Unterschiede & Ausbaupfad

**Zweck:** Unterschiede (§8) konsolidieren und den API-bruchfreien Ausbaupfad (§9, Phasen 1–5) in konkrete Planungsstufen überführen.

**Einordnung:** Abhängt von D1–D7 (konsolidierend). Downstream: steuert die Reihenfolge der GPU-Optimierungen und sichert die Erweiterungspunkte (Gruppe H). Querschnitt Q1–Q4.

**Quellen:** `resident_subengine.md` §8, §9, §4.6, §10; `context/context_d.md` D8 (#30–#31), #32/#33; `context_h_q.md` (#15–#18 Graph-Capture, #1–#7 Ringbuffer).

---

## Festgelegte Entscheidungen

- **Unterschiede CPU/GPU (§8):**
  - **CPU:** mehr Speicher; alles CPU; Scorer selbst (SIMD, D6); Encoder über Candle-CPU; Index per mmap lesbar; **kein** Staging/Streams.
  - **GPU:** begrenzter VRAM; Streams/Staging/Transfers; Tokenizer/Normalizer CPU-gebunden (Bottleneck); Batching nötig zum Streamen/Swappen von Segmenten/Kernels.
  - **Gemeinsam:** dieselben Traits (`Op`, `Region`, `ExecContext`), dieselbe Lease-Logik, dieselben Task-Pipelines.
- **Numerik (`[U]`, #30/#31):** f32 (CPU) vs f16 (GPU) → Cosinus-Differenz 1e-3…1e-4, Ranking-Unterschiede nur bei Beinahe-Gleichstand. CPU-f16 in candle ist langsam → **CPU f32**. Gemischte Indizes nur mit Toleranz; idealerweise **dtype pro Index** festlegen. cuBLAS-Akkumulation ≠ candle-Ops → **Golden-Tests je Phase**.
- **Trait-Hygiene (#30):** `Op`/`Region`/`ExecContext` **nie** an `candle_core::Tensor` leaken, sonst bricht die API beim Ersetzen von candle. Event-Ordnung zwischen Candle- und eigenen Kerneln **explizit** (candle nutzt eigenen Allokator/Stream).
- **Ausbaupfad (§9, verbindliche Stufenreihenfolge):**
  1. **Phase 1:** `Region`/`Op`/`ExecContext` als dünne Hüllen um Candle; MaxSim als Kachel-Kette; Engine-Gerüst, Lazy-Init, Leases. *(D1–D7 Grundlevel)*
  2. **Phase 2:** eigene Streams, eigener MaxSim-Kernel (NVRTC), Slice-Scheduling. *(D2/D6)*
  3. **Phase 3:** eigene Arena für Gewichte/Workspace, `encode_into` ohne Copy (abhängig von G-WRAP). *(D3/D4)*
  4. **Phase 4:** Graph-Decorator, Persistent-Scorer mit Ring. *(§4.6, H4/H1 — nur Erweiterungspunkte sichern)*
  5. **Phase 5:** optional eigener Encoder-Forward (cuBLAS + wenige Kernels) hinter demselben `Op`. *(D4)*

---

## Erweiterungspunkte sichern (Gruppe H — nur Hooks, keine Detailplanung)

- **H1 Ringbuffer/Persistent-Kernel (§4.6):** Zustand (Head/Tail/Stop-Flag/Zähler) in eigener Region; Grid ≤ SMs×Occupancy; Stop-Flag nur am Scheibenende; Acquire-Load `ld.acquire.sys` (sm75 ok). **Hook:** eigener Stream + feste Ring-Slots im Manager vorsehen (D2/D7). Sinnvoll für Scorer, nicht Encoder.
- **H4 Graph-Capture (§4.6):** Launcher-Decorator `Direct`/`Capture`, Cache-Schlüssel `(Op, Shape-Bucket, Regions, dtype, Stream)`; Invalidierung bei `gen`-Wechsel; feste Ring-Slots. **Hook:** Launcher so bauen, dass Capture später eingehängt werden kann. **G-GRAPH** vor Aktivierung.
- Allgemein: Parameterobjekte erweiterbar, `requires()`-Affinität offen, Op-Austausch hinter gleichem Trait (gilt für alle H).

---

## Zu erstellende Artefakte

- `docs`/Code-Doku: **Stufen-/Phasenplan** (Phase 1–5) mit Zuordnung zu D-Paketen und Abnahmekriterien je Phase.
- `kernel/launcher.rs`: Launcher mit Decorator-Punkt (`Direct` jetzt, `Capture` als Hook).
- Golden-Test-Harness: Referenzwerte je Phase (f32-Referenz, f16-Toleranz), damit Phasen-Austausch (Candle→eigener Kernel) verifizierbar bleibt.
- `compat.rs` (oder Review-Checkliste): Prüfung, dass öffentliche Signaturen frei von `Tensor`/Raw-Pointern sind.

---

## Arbeitsschritte

1. §8-Unterschiede als CPU/GPU-Fähigkeitsmatrix festhalten (Speicher, Streams, Staging, Scorer-Impl, dtype).
2. Phasen 1–5 als Meilensteine mit Abnahmekriterien definieren; D-Pakete zuordnen.
3. Golden-Test-Harness (f32-Referenz + Toleranzen) aufsetzen; je Phase wiederverwendbar.
4. Launcher mit Decorator-Punkt (Capture-Hook) und festen Ring-Slots vorbereiten.
5. Trait-Hygiene-Review: keine `Tensor`/Pointer-Leaks; Event-Ordnung Candle↔eigene Kernel explizit.
6. H-Hooks (H1/H4) als Erweiterungspunkte dokumentieren; **keine** Detailimplementierung.
7. Prototyp-Gates verbuchen: G-WRAP (Phase 3), G-GRAPH (Phase 4), G-MAXSIM (Phase 2).

## Fehler (Q1) & Logging (Q2)

- Keine neuen Fehlertypen; D8 konsolidiert. Phasenübergänge als `info`-Events; Golden-Test-Abweichungen als strukturierte Diagnose (nicht als Panic).

## Feature-Flags (Q4)

- Phasen 2–5 sind GPU-spezifisch (hinter `cuda`). Phase 1 CPU-Pfad + Candle-CPU bleibt ohne `cuda` voll funktionsfähig und testbar.

## Tests (Q3)

- Golden-Tests je Phase (CPU-f32-Referenz; f16-Toleranz ~1e-3; kein exakter Gleichheitstest).
- API-Kompatibilitätstest: Austausch Candle-Op ↔ eigener Kernel ändert öffentliche Signatur nicht.
- Phasen-Regression: Phase-2-Kernel liefert gleiche Rangfolge wie Phase-1-Kette (innerhalb Toleranz).

## Definition of Done

- Phasenplan 1–5 dokumentiert und an D-Pakete gebunden; Golden-Test-Harness vorhanden; H1/H4-Hooks gesichert; Trait-Hygiene bestätigt; Prototyp-Gates den Phasen zugeordnet.

## Result

D8 ist umgesetzt: Die Tests laufen grün (306 Unit-Tests ohne Features, 41 Kernel-Tests mit `mock,candle`, 2 Hygiene-Tests). Der `cuda`-Build und die `#[ignore]`-GPU-Tests wurden nicht ausgeführt.

- **`kernel/launcher.rs`:** Enthält `LaunchMode` (`Direct`/`Capture`), `LaunchKey`, den Trait `Launcher`, `DirectLauncher` und `launcher_for`. Der Schlüssel besteht aus Op, Shape-Bucket, Regions (inklusive Generation), dtype und Lane. `Capture` liefert bis Phase 4 `KernelError::Unavailable`.
- **`kernel/golden.rs`:** Vergleicht einen Kandidaten-Scorer mit einem Referenz-Scorer über `Tolerance::F32` (1e-5) oder `Tolerance::F16` (1e-3). Rangvertauschungen sind nur bei Beinahe-Gleichstand erlaubt. Abweichungen werden per `tracing::warn` gemeldet und als `Deviation` zurückgegeben, ohne Panic.
- **`tests/api_hygiene.rs`:** Prüft, dass keine öffentliche Signatur in `src/resident` `Tensor`, rohe Pointer, `CudaSlice` oder `CUdeviceptr` enthält. Ausgenommen sind `model/candle_adapter.rs`, `kernel/cuda.rs` und `kernel/maxsim.rs`. Die Prüfung arbeitet mit Wortgrenzen, damit der eigene Typ `TensorRef` nicht fälschlich auffällt.
- **`docs/architecture/phase_plan.md`:** Enthält die CPU/GPU-Fähigkeitsmatrix und die Phasen 1–5 mit D-Paketen, Gates (G-MAXSIM, G-WRAP, G-GRAPH) und Abnahmekriterien. Außerdem stehen dort die Erweiterungspunkte und die Trait-Hygiene-Regeln.

- **Ring-Slots:** Die festen Ring-Slots im Manager (H1-Hook) habe ich nicht gebaut. Sie sind nur im Dokument als offener Erweiterungspunkt vermerkt.
- **Phasen-Regressionstest:** Phase 2 gegen Phase 1 ist noch nicht verdrahtet. Dafür fehlt ein Test, der `golden::check` mit der CPU-Referenz und dem fused-Kernel aufruft. Die vorhandenen Candle-Tests vergleichen schon gegen die CPU-Referenz.
