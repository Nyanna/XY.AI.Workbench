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

## 8. Offene Verifikation / Risiken
- `[prüfen]` `cudarc`-Version & genaue API (`compute_capability()` vs. `result::device::get_attribute`). Quellen: docs.rs prism-q/numr/pandrs/hive-gpu.
- `[unverifiziert]` `dp4a`-Durchsatz auf TU116, WMMA-Verhalten (AnandTech-Spekulation) → nur messen, nie annehmen.
- `[W]` candle-dtype-Pfade (f16 cuBLAS-Compute-Typ, bf16-Emulation auf sm_75) vor Nutzung im Katalog gegen `candle-kernels`/`cuda_backend` prüfen.
- Whitelist der TU116-Dies (fehlende TC) pflegbar halten; alternativ Mikrobenchmark-Hook für TC-Flag.

## 9. Definition of Done
- `CapabilityProbe` für CPU implementiert und getestet; CUDA-Probe hinter `cuda` baubar (`cargo check`).
- Default-Katalog GPU/CPU gemäß Tabelle, inkl. ausdrücklichem Ausschluss von Tensor-Cores/bf16 auf sm_75 mit Quellenkommentar.
- Registry-Schnitt gegen D6 implementiert; Achsen/Werte erweiterbar ohne API-Bruch (Test vorhanden).
- Liefert ein `CapabilitySet`, das F2 direkt aufzählen kann.
