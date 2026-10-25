# Umsetzungsplan F1 — CapabilitySet-Erhebung

> Gruppe F · Paket F1. Quelle: `resident_precision.md` (Abschnitte „Capability-Listen statt Profile“, „Umsetzungsvorgaben“). Kontext: `context/context_f.md` §F1 (Q1–Q5). Querschnitt: `plan_f0_overview.md` §4.
> **Hängt ab von:** B6 (Typen `CapabilitySet`, `Profile`), D6 (`resident_subengine.md` §4.5 — reale Kernel-/Scoring-Pfade). **Liefert an:** F2.

## 1. Zweck & Ergebnis
Jede Subengine erhebt ihr `CapabilitySet`: **pro Parameterachse die tatsächlich verfügbaren Werte**. Ergebnis ist (a) ein erweiterbarer Achsen-/Wertekatalog, (b) ein `CapabilityProbe`-Trait mit CPU- und CUDA-Implementierung, (c) eine Abgleichregel gegen die D6-Kernel-Registry (nur Werte, für die ein Kernel existiert, werden gelistet).

## 2. Scope-Abgrenzung
- F1 **misst nicht** und bewertet nicht (das ist F2). F1 ist **optimistisch**: es listet, was Hardware + Kernel-Registry anbieten. Unbrauchbare Kombinationen verwirft F2 (NaN/Inf, Maximalabweichung, dominiert).
- F1 definiert **keine** Profile und keine Reihenfolge.
- Achsen/Werte müssen **pro Hardware erweiterbar** sein, **ohne** Änderung an Fassade (C1) oder Task-Schnittstelle (harte Vorgabe).

## 3. Zieltypen (Skizze; B6 ist formaler Eigentümer)
```rust
#[non_exhaustive]
pub enum EngineKind { Cpu, Cuda }

/// Achsen-Identität ist offen (String-Newtype), damit neue Achsen ohne
/// Enum-Änderung (= ohne API-Bruch) hinzukommen können.
#[derive(Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct AxisId(pub &'static str);

/// Ein Achsenwert ist getaggt, damit heterogene Achsen (dtype, Flag, Zahl)
/// in einem Katalog leben. Serde-fähig für F5-Persistenz.
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AxisValue {
    Dtype(DType),       // f32, f16, (bf16), int8 ...  — DType-Enum aus B2
    Accum(DType),       // Akkumulator-dtype
    Flag(bool),         // z. B. Fusion an/aus
    UInt(u32),          // Batch-, Tile-, Thread-Zahl
    Scoring(ScoringPath)// f32-FMA | f16x2-FMA/f32-Akku | int8-dp4a/i32-Akku | cpu-int8 ...
}

pub struct CapabilityAxis { pub id: AxisId, pub values: Vec<AxisValue> }

pub struct CapabilitySet { pub engine: EngineKind, pub axes: Vec<CapabilityAxis> }

pub trait CapabilityProbe { fn probe(&self) -> Result<CapabilitySet, PrecisionError>; }
```
`ScoringPath` ist ein erweiterbares Enum (`#[non_exhaustive]`), dessen Varianten **1:1 den in D6 registrierten Kernel-Varianten** entsprechen.

## 4. Achsen-/Wertekatalog (Default, aus `context_f.md` §F1)
### GPU / CUDA (Zielkarte GTX 1660, TU116, sm_75)
| Achse (`AxisId`) | Default-Werte | Belegt |
|---|---|---|
| `"operands"` | `{f32, f16}` | [V] native FP16 (2× FP32-Rate, `half2`) |
| `"accumulator"` | `{f32, f16}` | [V] |
| `"scoring"` | `{f32-FMA, f16x2-FMA/f32-Akku, int8-dp4a/i32-Akku}` | [V] `dp4a` ⇔ CC ≥ 6.1, sm_75 erfüllt |
| `"fusion"` | `{an, aus}` | — |
| `"batch"` | konfigurierbare Menge (z. B. `{1,8,32,…}`) | — |
| `"tile"` | konfigurierbare Menge | — |

**Harte Regeln (sm_75 / TU116):**
- **Keine Tensor-Cores** → **kein** WMMA/MMA-Matrixpfad in den Katalog aufnehmen. [V: TU116 ist eigenes Die ohne TC]
- **Tensor-Core-Erkennung NIE nur aus Compute Capability** ableiten: TU116 meldet CC 7.5, hat aber keine TC. Ein naiver `major >= 7`-Check (wie in pandrs/hive-gpu) wäre hier falsch-positiv. [V] → TC-Flag per **Whitelist/Mikrobenchmark**, nicht per CC.
- **bf16** ist auf sm_75 nicht nativ → als *emuliert/dominiert* behandeln, nicht als eigener Schnellpfad listen. [W]
- `dp4a`-**Durchsatz** (Rate vs. FP32) auf TU116 ist **unbelegt** → Wert in Katalog aufnehmen, aber dessen Nutzen **F2 messen lassen**, nicht annehmen.

### CPU
| Achse | Default-Werte | Belegt |
|---|---|---|
| `"operands"` | `{f32, f16→f32}` | [W] `half`-Crate, F16C auf x86 |
| `"scoring"` | `{f32, int8}` | [W] AVX2 `_mm256_maddubs_epi16` / VNNI |
| `"threads"` | `{1 … n_logical}` (sysinfo) | [W] Skalierung bis Speicherbandbreite |
| `"tile"` | konfigurierbare Menge (L1/L2-Passung) | [W] |

## 5. Umsetzungsschritte
1. **Typen/Trait** in `capability.rs` anlegen (ggf. B6 konsolidieren). `AxisId` als `&'static str`-Newtype, Werte getaggt, serde-fähig (für F5).
2. **CUDA-Probe** (`capability_cuda.rs`, `cfg(feature="cuda")`):
   - CC lesen: `cudarc::driver::CudaContext::new(ordinal)?.compute_capability() -> (major, minor)` (cudarc ≥ 0.19) [V]. Fallback roh: `result::device::get_attribute(dev, CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR/MINOR)` [V].
   - Ableitungen: FP16-schnell ⇔ CC ≥ 5.3; `dp4a` ⇔ CC ≥ 6.1 [V]. TC ⇔ CC ≥ 7.0 **UND** nicht in TU116-Whitelist.
   - Achsen aus einer **`hardware_table`** (CC/Device-Name → erlaubte Werte) erzeugen, danach mit D6-`KernelRegistry` schneiden (nur Werte mit existierendem Kernel).
3. **CPU-Probe** (`capability_cpu.rs`): `raw-cpuid` (Brand, Flags AVX2/F16C/VNNI), `sysinfo` (logische Kerne). Achsen entsprechend füllen (`int8` nur bei VNNI/AVX2, `f16→f32` nur bei F16C).
4. **Registry-Abgleich (D6):** Funktion `intersect_with_kernels(set, &KernelRegistry) -> CapabilitySet`. Entfernt Werte, für die kein Kernel registriert ist. Dokumentiert, dass F1 nie Werte liefert, die D6 nicht ausführen kann.
5. **Konfig-Achsen (batch/tile):** aus Engine-Config (nicht aus Hardware) befüllen; Defaults als Zweierpotenzen; Obergrenze an die konfigurierte Maximalsequenzlänge koppeln (F2 testet Verarbeitbarkeit bis dorthin).
6. **Factory** `make_probe(engine, cfg) -> Box<dyn CapabilityProbe>` — einzige `cfg`-Verzweigung für CUDA.

## 6. Querschnitt
- **Fehler (Q1):** `PrecisionError::Capability(String)` bei fehlgeschlagener Treiber-/CPUID-Abfrage. Kein Panic bei fehlender GPU — Probe liefert dann nur die CPU-Engine.
- **Logging (Q2):** `info!` mit `engine`, `device_name`, `cc`, Anzahl Achsen/Werte; `debug!` je Achse die gelisteten Werte und welche per Registry-Schnitt entfielen.
- **Feature-Flags (Q4):** CUDA-Probe komplett hinter `cuda`; ohne Feature existiert nur die CPU-Probe. `cudarc`-Safe-API liefert **keine** Runtime-/Driver-Version [V] → das ist F5 (NVML/`cuDriverGetVersion`), nicht F1.

## 7. Teststrategie (Q3)
- `FakeCapabilityProbe` mit festgelegtem Katalog (für F2/F3-Tests).
- CPU-Probe-Test: auf Host mit/ohne AVX2 korrekte Achsenmengen (Feature-Erkennung mocken via injizierbarer `CpuFeatures`-Quelle).
- Registry-Schnitt-Test: Wert ohne Kernel wird entfernt.
- Erweiterbarkeits-Test: neue Achse/neuer Wert lässt sich hinzufügen, ohne bestehende Signaturen zu ändern (Kompilationsnachweis).
- **Kein** echter GPU-Test im Default-Lauf.

## 8. Verifikationsergebnisse
Geprüft am 2026-10-25 auf GTX 1660 SUPER (TU116, CC 7.5, 22 SMs), Treiber 610.57.04, `nvcc` 13.3.73, `cudarc 0.19.9`, `candle-core 0.11.0`. Keine offenen Punkte.

### CUDA-Probe (`cudarc`)
Gelesen in `cudarc-0.19.9/src/driver/safe/core.rs`:
- `CudaContext::new(ordinal: usize) -> Result<Arc<Self>, DriverError>`
- `CudaContext::compute_capability(&self) -> Result<(i32, i32), DriverError>`
- `CudaContext::name(&self) -> Result<String, DriverError>`
- `CudaContext::attribute(&self, CUdevice_attribute) -> Result<i32, DriverError>`

Die Probe nutzt ausschließlich diese Safe-API; `result::device::get_attribute` ist nicht nötig. Typen sind `i32`, nicht `u32`. Das Projekt pinnt `cudarc =0.19.9` hinter dem Feature `cuda`; transitiv liegt zusätzlich `0.17.8` in der Lockfile.

### Durchsatz auf TU116 (Mikrobenchmark, `-arch=sm_75`, bestes von 5 Läufen)
| Pfad | Rate | Elemente/MACs je Instruktion | Spitzen-Durchsatz relativ zu f32 |
|---|---|---|---|
| f32 `fma` | 2,1–2,2 T Instr/s | 1 | 1× |
| f16x2 `hfma2` | 2,1–2,2 T Instr/s | 2 | 2× |
| int8 `dp4a` | 2,1–2,2 T Instr/s | 4 | 4× |
| WMMA f16/f32-Akku | 0,96 TFLOPS | – | ≈ 0,11× des f16x2-Peaks (≈ 8,6 TFLOPS) |

- `dp4a` und f16x2 sind auf TU116 schnell; der Wert `int8-dp4a/i32-Akku` bleibt im Katalog. Das ist Rechenspitze ohne Speichertraffic; der Nutzen im realen Scoring-Kernel bleibt Messgegenstand von F2.
- WMMA läuft zwar fehlerfrei, aber um rund eine Größenordnung langsamer als einfache f16x2-FMA. Das belegt, dass TU116 keine Tensor-Core-Rate liefert. Kein WMMA/MMA-Pfad im Katalog.
- Die erste f16x2-Messung (≈ 4×) war ein Artefakt: identische Startwerte ließen den Compiler die Ketten zusammenfassen. Mit verschiedenen Startwerten korrigiert.

### Tensor-Core-Flag
Whitelist/Negativliste über `CudaContext::name`, nie aus der CC. TU116 (GTX 1660/1660 Ti/1660 SUPER, 1650-Varianten) steht auf der Negativliste. Der WMMA-Durchsatztest ist als optionaler Mikrobenchmark-Hook für unbekannte CC-7.x-Geräte geeignet: Rate ≈ f16x2-Peak oder darunter ⇒ keine Tensor-Cores.

### candle-Pfade
- `candle-core 0.11.0`, `cuda_backend/mod.rs`: f16-Gemm wählt `CUBLAS_COMPUTE_16F`, wenn `gemm_reduced_precision_f16()` gesetzt ist, sonst `CUBLAS_COMPUTE_32F`. Der Default des Flags ist `false`. f16-Operanden akkumulieren also standardmäßig in f32; `accumulator=f16` ist über `set_gemm_reduced_precision_f16(true)` erreichbar. Beide Akkumulator-Werte stehen im Katalog; die Umschaltung ist ein globaler Prozess-Schalter, keine Pro-Aufruf-Option.
- bf16: `CUBLAS_COMPUTE_32F_FAST_16BF` nur bei gesetztem Flag; `candle-kernels/build.rs` deaktiviert die bf16-WMMA-Kernel unter sm_80. bf16 wird auf sm_75 nicht als Schnellpfad gelistet.
- f32: `CUBLAS_COMPUTE_32F_FAST_TF32` bei gesetztem Flag. TF32 existiert auf sm_75 nicht und wird nicht gelistet.

### CPU-Probe (Host, `__builtin_cpu_supports`)
| Feature | Verfügbar |
|---|---|
| AVX2 | ja |
| F16C | ja |
| FMA | ja |
| AVX512F / AVX512-VNNI / AVX-VNNI | nein |

Folge für den CPU-Katalog: `f16→f32` (F16C) und `int8` über AVX2 (`maddubs`) sind verfügbar; ein VNNI-Pfad wird nur bei erkanntem Flag gelistet und ist auf diesem Host nicht erreichbar. Die CPU-Probe-Tests müssen deshalb über die injizierbare `CpuFeatures`-Quelle auch Hosts mit VNNI abdecken.

## 9. Definition of Done
- `CapabilityProbe` für CPU implementiert und getestet; CUDA-Probe hinter `cuda` baubar (`cargo check`).
- Default-Katalog GPU/CPU gemäß Tabelle, inkl. ausdrücklichem Ausschluss von Tensor-Cores/bf16 auf sm_75 mit Quellenkommentar.
- Registry-Schnitt gegen D6 implementiert; Achsen/Werte erweiterbar ohne API-Bruch (Test vorhanden).
- Liefert ein `CapabilitySet`, das F2 direkt aufzählen kann.

## Result

F1 ist umgesetzt. Die Tests laufen grün: 477 ohne GPU, und 36 `precision`-Tests mit `--features cuda` (nvcc 13.3). `cargo check --features cuda` zeigt keine Fehler. Der Test gegen die echte GTX 1660 SUPER (`real_device_probe_matches_hardware_table`) lief mit. Die Tests prüfen Schranken und Katalogmengen, keine Bit-Gleichheit.

Das Modul `resident/precision` hatte `CapabilitySet`, `AxisId`/`ValueId` (String-Newtypes), `known` und `PrecisionError` schon. Ich habe F1 deshalb daran angepasst und nicht die Typen aus der Planskizze (`AxisValue`, `EngineKind`) neu gebaut. Stattdessen nutze ich `SubengineKind`.

Neu ist Folgendes:
- **Probe-Trait und Konfiguration** (`probe.rs`): `CapabilityProbe`, `ProbeConfig` mit `batches`, `tiles` und `max_seq_len`, dazu `FakeCapabilityProbe` für die F2- und F3-Tests.
- **Kernel-Schnitt** (`probe.rs`): `intersect_with_kernels` und `KernelFilteredProbe`. Sie entfernen Werte ohne Kernel. Ein Kernel-Katalog liegt neu in `kernel/catalog.rs`.
- **CPU-Probe** (`probe_cpu.rs`): Die CPU-Features kommen über das injizierbare Trait `CpuFeatureSource`. Die Tests decken damit auch einen VNNI-Host ab, den dieser Rechner nicht hat. `int8-vnni` ist ein neuer Wert in `known`.
- **CUDA-Probe** (`probe_cuda.rs`): Die Hardware-Tabelle (CC 5.3 für fp16, CC 6.1 für dp4a) ist eine reine Funktion und braucht keine GPU. Tensor-Cores werden nur über eine Namensliste erkannt, nie aus der CC, und es gibt weder bf16 noch einen WMMA-Pfad.
- **Factory** (`make_probe`): Sie ist die einzige Stelle mit einer `cfg`-Verzweigung für `cuda`. Ohne GPU oder ohne das Feature liefert sie einen Fehler und paniert nicht.
- **Fehlervariante**: `PrecisionError::Capability { reason }`.

Zwei Abweichungen vom Plan:
- **dp4a fällt im Endergebnis weg:** Der Schnitt mit dem Kernel-Katalog entfernt `int8-dp4a` in der gefilterten Probe, weil es dafür noch keinen Kernel gibt. Der NVRTC-Loader kompiliert nur F32 und F16. `known::gpu_sm75()` und die Hardware-Tabelle behalten den Wert. Sobald ein dp4a-Kernel existiert, erscheint er automatisch. Dasselbe gilt für die CPU: Die gefilterte CPU-Probe liefert nur fp32, weil `cpu_simd` nur f32 rechnet. Die Hardware-Probe listet f16-to-f32 und int8 trotzdem korrekt.
- **Kein neues Crate:** `make_probe` bekommt zusätzlich das `DeviceHandle`, weil die CUDA-Probe den Kontext braucht. Statt `raw-cpuid` und `sysinfo` nutze ich `std::is_x86_feature_detected!` und `available_parallelism`, damit keine neue Abhängigkeit nötig ist.

Zwei Mal habe ich versehentlich eine Platzhalterdatei mit erfundenem Import angelegt (`kernel/catalog.rs` und `test/probe_cpu.rs`). Beide sind sofort durch die richtigen Versionen überschrieben, und der Build ist sauber.

Die Bindung zwischen Kernel-Katalog und NVRTC-Loader ist nur eine Konstante (`CUDA_KERNEL_DTYPES`). Der Loader wurde nicht umgebaut, daher kann beides auseinanderlaufen, wenn jemand einen Typ nur an einer Stelle ergänzt.