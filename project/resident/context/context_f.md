# Kontext Gruppe F — Präzision & Kalibrierung (Resident-Engine)

Stand: 2026-10-10. Kennzeichnung: **[V]** = per Web-Recherche belegt (Quelle genannt), **[W]** = Fachwissen ohne frische Quellenprüfung, vor Implementierung verifizieren.
Recherchiert wurden gezielt F1/Q1–2 und F2/Q6; alle übrigen Antworten sind **[W]**.

---

## F1 · CapabilitySet

### 1. Rechenpfade GTX 1660 (TU116, sm_75)
- **Keine Tensor Cores, keine RT Cores [V].** TU116 ist ein eigenes Die; Tensor-Zellen der RTX-Turing-Chips (TU106 etc.) fehlen. Quellen: Tom's Hardware, KitGuru, Igor's Lab, AnandTech (unten).
- **Nativer FP16-Durchsatz [V]:** Statt Tensor Cores besitzt TU116 dedizierte FP16-Einheiten, 2× FP32-Rate (GTX 1660: ~5 TFLOPS FP32, ~10 TFLOPS FP16 peak). Dual-Issue neben FP32/INT32.
- **Konsequenz:** FP16 = schneller *elementweiser/FMA*-Pfad (`half2`), aber **kein WMMA/MMA-Matrixpfad mit Tensor-Beschleunigung**. RTX 2060 erreicht FP16-GEMM über Tensor Cores (~51 TFLOPS) deutlich höher [V, Igor's Lab/Tom's Hardware]. AnandTech vermutet, dass Tensor-Ops auf TU116 (CC 7.5) über CUDA-Cores laufen, d. h. WMMA-Code kompiliert evtl., ist aber nicht schnell [V als Spekulation der Quelle, unverifiziert].
- **INT8 `dp4a`:** Voraussetzung CC ≥ 6.1 [V: ggml `MIN_CC_DP4A 610`, NVIDIA-Forum]. sm_75 erfüllt das → `__dp4a` verfügbar [V per Ableitung]. Konkreter Durchsatz auf TU116 (Rate relativ zu FP32) **[unverifiziert]** → Kalibrator soll ihn messen statt annehmen. INT8-Tensor-MMA nicht verfügbar.
- **Achsen-Vorschlag:** Operanden {f32, f16}, Scoring {f32-FMA, f16x2-FMA mit f32-Akku, int8-dp4a mit i32-Akku}; kein TC-Pfad auf dieser Karte. bf16: auf sm_75 keine native Hardwareunterstützung **[W]** → als emuliert/dominiert behandeln.
- Quellen: https://www.tomshardware.com/reviews/nvidia-geforce-gtx-1660-turing-tu116%2C6027.html · https://www.kitguru.net/components/graphic-cards/dominic-moass/msi-gtx-1660-ti-gaming-x-6g-review/ · https://www.igorslab.de/en/nvidia-geforce-gtx-1660-6gb-with-board-partner-cards-of-msi-it-moves-something-not-only-the-pixel/ · https://at-web1.www.anandtech.com/show/13973/nvidia-gtx-1660-ti-review-feat-evga-xc-gaming/2 · https://forums.developer.nvidia.com/t/performance-difference-dp4a-vs-vmin4-using-gtx1060/159538 · https://huggingface.co/spaces/Xenobd/whisper.cpp/blob/main/ggml/src/ggml-cuda/common.cuh

### 2. Capability-Abfrage in Rust (cudarc)
- `cudarc::driver::CudaContext::new(ordinal)` → `ctx.compute_capability()` liefert `(major, minor)` (cudarc ≥ 0.19) [V: docs.rs prism-q, pandrs].
- Alternativ roh: `cudarc::driver::result::device::get_attribute(dev, CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR/MINOR)` [V: numr, hive-gpu]. Weitere Attribute (PCI-IDs, max threads/block, SM-Count) analog [V].
- cudarc-Safe-API liefert **keine** Runtime-/Driver-Version [V: Kommentar in pandrs] → `cudarc::driver::sys::cuDriverGetVersion` bzw. NVML nutzen **[W]**.
- Ableitung: FP16-schnell ⇔ CC ≥ 5.3 (voller Durchsatz-Vorteil auf 6.0/7.x); `dp4a` ⇔ CC ≥ 6.1 [V]; Tensor-Cores ⇔ CC ≥ 7.0 **und** nicht TU116 → **CC allein reicht nicht** (TU116 hat CC 7.5 ohne TC, vgl. AnandTech [V]). Daher Tensor-Core-Flag per Whitelist/Mikrobenchmark statt nur per CC (hive-gpu/pandrs prüfen nur `major >= 7` → würde auf der 1660 falsch positiv sein [V, Beobachtung am Code]).
- Quellen: https://docs.rs/crate/prism-q/0.11.2/source/src/gpu/device.rs · https://docs.rs/crate/numr/latest/source/src/runtime/cuda/device.rs · https://docs.rs/crate/pandrs/0.4.1/source/src/gpu/mod.rs · https://docs.rs/crate/hive-gpu/0.2.0/source/src/cuda/context.rs

### 3. candle dtype-Pfade **[W]**
- DType: F32, F16, BF16 (+ F64, U8, U32, I64) auf CPU und CUDA. Matmul auf CUDA über cuBLAS (f16 mit f32-Compute je nach Version); BF16 auf sm_75 nicht nativ (Fehler/Emulation möglich). Reductions/Softmax/LayerNorm intern teils f32-Akkumulation. Steuerung: explizit `to_dtype(F32)` vor sensiblen Ops; Custom-Kernel via cudarc für volle Kontrolle. Details im candle-Quellcode (`candle-kernels`, `candle-core/src/cuda_backend`) prüfen.

### 4. CPU-Seite **[W]**
- `half` crate (`f16`, `to_f32()`; Feature `use-intrinsics`/F16C auf x86). int8: `std::simd` (nightly) oder `wide`; x86 AVX2 `_mm256_maddubs_epi16`/VNNI. Reproduzierbare Durchsatz-Achsen: Thread-Zahl (Skalierung bis Speicherbandbreite), Tile-/Block-Größe (L1/L2-Passung), Batch, Speicherlayout. 

### 5. Taxonomien **[W]**
- CUTLASS: Threadblock-/Warp-/Instruction-Tile-Shape, Stages, Element-Typen (A/B/C/Accumulator), Epilogue-Fusion. cuBLAS: `cublasGemmEx` mit compute type + algo-Heuristik (`cublasLt` heuristics). TVM/Ansor: Sketch + Annotation (Tile, Unroll, Vectorize, Parallel). Passt auf Operand/Akku/Scoring/Fusion/Batch/Tile.

---

## F2 · Calibrator

### 6. Korrektes GPU-Benchmarking [V]
- Kernel-Launches sind asynchron; ein Host-Timer ohne Sync misst nur Submit-Zeit. Events werden auf dem GPU-Stream gestempelt und blenden Launch-Overhead aus [V: Speechmatics/PyTorch-Artikel, MLC-Handbuch]. Vor Messung `synchronize`; Warmup (JIT, Caches, Clock-Ramp); fixierte Clocks; L2-Cache-Flush zwischen Läufen; CUDA-Graphs/Mehrfach-Launch pro Messfenster für **kurze** Kernel [V].
- Einschränkung: Events nehmen an, dass zwischen Kernel und Events keine Lücke liegt; läuft die GPU der CPU davon, ist die Lücke mitgemessen → mehrere Iterationen zwischen zwei Events enqueuen, durch Anzahl teilen [V].
- Fallstrick: Implizite CUDA-Init verfälscht Host-Timer [V, NVIDIA-Forum].
- Robust kurze Kernel: Iterationszahl per Kalibrierlauf auf Ziel-Fensterlänge (z. B. ≥ 1 ms) skalieren (Muster im MLC-Handbuch, `warmup`/`repeat` als Millisekunden-Budget [V]).
- Quellen: https://www.speechmatics.com/company/articles-and-news/timing-operations-in-pytorch · https://mlc.ai/modern-gpu-programming-for-mlsys/appendix/benchmarking_gpu_kernels.html · https://forums.developer.nvidia.com/t/events-vs-timers-big-differences-measurung-kernel-execution-time/20271 · https://mcp.directory/skills/benchmark-kernel
- Clock-Fixierung auf Consumer-Karte: `nvidia-smi -lgc` benötigt Admin und wird von GeForce teils nicht unterstützt **[W]** → Fallback: Warmup + Streuungskriterium.

### 7. Keine Preemption **[W]**
- Zeitbudget pro Mini-Benchmark (z. B. ≤ 5–10 ms/Kernel-Launch), kleine Problemgrößen, Slicing in viele kurze Launches; zwischen Slices Prüfpunkt, ob Produktions-Task wartet (Atomic-Flag/Queue-Länge) und ggf. abbrechen. Gesamtbudget pro Kalibrierung begrenzen.

### 8. Interleaved A/B/A/B **[W]**
- Langsame Drift (Thermik, Boost-Clocks, Fremdlast) wirkt auf beide Varianten gleich statt systematisch auf die zweite. Fallstrick: Reihenfolge-Effekte (Cache/Clock) → Reihenfolge randomisieren oder alternierend (ABBA).

### 9. Statistik **[W]**
- Median als Hauptwert (robust gegen Ausreißer), Minimum als Untergrenze der Hardware-Zeit (gut bei Rauschen, das nur addiert), Mittelwert nur bei Gesamtdurchsatz. Stabilität: Variationskoeffizient oder (max−min)/median; adaptiv: erst mehr Samples pro Wiederholung, bei Nichterreichen Gesamtdurchlauf wiederholen bis Obergrenze → `unstable`. Referenz: `criterion` (Bootstrap-Konfidenzintervalle, Ausreißerklassifikation).

### 10. Verwerfungskriterien **[W]**
- NaN/Inf: `!x.is_finite()` über alle Elemente (GPU-Reduktion). Degeneriert: Varianz ≈ 0, L2-Norm außerhalb [ε, K]. Metrik: 1 − cos(a, ref) für Embeddings; relativer Fehler `‖a−ref‖/‖ref‖`; für Scoring zusätzlich Rang-Metrik (Top-k-Overlap/Kendall τ). Schwellen (Richtwerte, projektspezifisch festlegen): fp16 ~1e-3…1e-2 Cosinus-Abweichung, int8 ~1e-2.

### 11. Pareto-Front **[W]**
- 2D: nach Zeit sortieren, linear durchlaufen, Punkt behalten, wenn Abweichung < bisheriges Minimum → O(n log n). Frühes Pruning: sobald eine Kombination bereits gemessen ist, die in Zeit **und** Abweichung ≤ ist, restliche Samples abbrechen (Abweichung früh billig, Zeit teuer → Abweichung zuerst messen).

### 12. Separabilität **[W]**
- OFAT zulässig, wenn Achsen näherungsweise additiv wirken (kein Interaktionsterm). Typische Wechselwirkungen: Tile × Datentyp, Fusion × Akkumulator. Gegenmaßnahme: fraktionelles faktorielles Design oder OFAT + Verifikationsmessung der kombinierten Gewinner; Auto-Tuner (Ansor/Kernel Tuner) nutzen zusätzlich Cost-Model/Search statt Vollenumeration.

### 13. Thread-Priorität **[W]**
- `thread_priority` crate (`set_current_thread_priority(ThreadPriority::Min)`): Linux nice-/Scheduler-Policy (niedrige Priorität ohne Root nur absenkbar), Windows `SetThreadPriority`. Eigener Pool via `std::thread::Builder` mit Prioritätssetzung beim Start, getrennt vom tokio-Runtime.
- Hinweis: CPU-Priorität beeinflusst GPU-Konkurrenz nicht → Q7 bleibt notwendig.

### 14. Mess-Trait **[W]**
- `trait Measure { fn time(&mut self, cfg:&Cfg) -> Result<Duration>; }` Mock mit deterministischem Rausch-Generator (seeded RNG, Sequenz-Replay, Skripte: stabil / driftend / bimodal / nie stabil) → Stabilitäts- und `unstable`-Logik ohne GPU testbar. Zeit via injizierter Clock abstrahieren.

---

## F3 · Profilbildung **[W]**
15. Normierung min-max auf [0,1]; Zeit `log(t)` wegen Größenordnungsspannen. Fallstricke: Division durch 0 bei Einpunkt-Front, Ausreißer dominieren Min/Max, Skalenwahl ändert Abstände.
16. Bogenlänge: `s_i = Σ‖P_j−P_{j−1}‖`, `p_i = s_i / s_n`. Robuster als Rang, weil Abstände zwischen Punkten (echte Tradeoff-Sprünge) erhalten bleiben; Rang behandelt dicht und weit entfernte Punkte gleich.
17. Redundanz: 1D-Single-Linkage entlang `p` (Nachbarn mit Δp < Schwelle verschmelzen), Repräsentant = schnellster oder mittlerer Punkt. Schwelle ist einziger Parameter.
18. Emulationserkennung: Kombination ist **dominiert** (langsamer und nicht genauer als f32-Referenz-Pfad) → aus Front entfernt; zusätzlich Plausibilitätscheck „Zeit ≥ Zeit des höherpräzisen Pfads“.

## F4 · Task-Matching **[W]**
19. Analoga: HNSW `ef_search` / IVF `nprobe` (Recall↔Latenz), Faiss-Parameter-Autotuning, TensorRT/ONNX-Runtime-Präzisionsmodi, llama.cpp-Quantisierungsstufen. Als Orientierung für Semantik, keine 1:1-Kontinuitätsvorlage.
20. `if t.is_nan() { Err }` vor `clamp`; `f32::clamp` gibt bei NaN NaN zurück (Rust-Docs-Verhalten, verifizieren) → NaN explizit abfangen. `±∞` wird von `clamp` auf Grenzen geklemmt.
21. `min_by` mit `(t−p).abs()` über `total_cmp`; Tie-Break auf höheres `p` (genaueres Profil): Vergleich `d.total_cmp(&d2).then(p2.total_cmp(&p1))`. Gleichstand wegen f32-Rundung möglich → Distanz in f64 rechnen.

## F5 · CalibrationStore **[W]**
22. GPU/Treiber/CUDA: NVML (`nvml-wrapper`: `device.name()`, `sys_driver_version()`, `sys_cuda_driver_version()`), `cudarc` für CC; CPU: `raw-cpuid` (Brand String, Features AVX2/F16C/VNNI), `sysinfo` (Kernzahl). Fallback bei fehlendem NVML (z. B. Windows ohne DLL): `cuDriverGetVersion`.
23. Fingerprint-Felder: GPU-Name + CC + VRAM, Treiberversion, CUDA-Driver-/Runtime-Version, CPU-Brand + Feature-Flags + Threadzahl, Modell-ID/Hash, Engine-Version, Kalibrier-Schema-Version. Schlüssel = Hash (z. B. blake3) über kanonisch serialisierte Felder, plus `schema_version` im Klartext. Zustände: `Calibrated` (Hash gleich), `Stale` (Hash weicht ab), `Missing` (keine Datei).
24. Format: JSON/TOML für Debugbarkeit, `postcard`/`bincode` kompakt aber ohne Selbstbeschreibung. Empfehlung: JSON mit `schema_version`, `#[serde(default)]` für neue Felder (vorwärtskompatibel), unbekannte Felder tolerieren; `unstable`, Verwerfungen und Pruning-Ergebnisse als explizite Felder.
25. `Stale`: alte Ergebnisse nutzen, im Hintergrund neu kalibrieren, atomar ersetzen (Temp-Datei + rename). `Missing`: konservativ = genaueste gültige Kombination (f32-Referenz), Warnung loggen, Kalibrierung im Hintergrund anstoßen; manuelle Nachkalibrierung via CLI/API-Flag.

---

## Unsicherheiten / offene Verifikation
- dp4a-Durchsatz (Rate vs. FP32) und WMMA-Verhalten auf TU116 nicht belegt.
- Verhalten von candle bei BF16/F16 auf sm_75 und interne f32-Fallbacks nicht geprüft.
- `f32::clamp`-NaN-Verhalten und Clock-Locking auf GeForce nicht gegen Doku geprüft.
- Alle [W]-Abschnitte (F2 Q7–14, F3–F5) enthalten keine belegten Quellen.