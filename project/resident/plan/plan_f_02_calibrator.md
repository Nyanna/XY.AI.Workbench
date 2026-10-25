# Umsetzungsplan F2 — Calibrator (Messung, Pruning, Pareto)

> Gruppe F · Paket F2. Quelle: `resident_precision.md` (Abschnitte „Calibrator“, „Messmethodik“, „Kombinatorische Explosion begrenzen“). Kontext: `context/context_f.md` §F2 (Q6–Q14). Querschnitt: `plan_f0_overview.md` §4.
> **Hängt ab von:** F1. **Liefert an:** F3 (Pareto-Front), F5 (zu persistierende Rohergebnisse).

## 1. Zweck & Ergebnis
Der `Calibrator` zählt Kombinationen aus dem `CapabilitySet` auf, misst jede als **Mini-Benchmark** interleaved gegen die fp32-Referenz, verwirft ungültige, bildet die **Pareto-Front (Zeit, Abweichung)** und übergibt sie an F3. Ergebnis: Kalibrierungsablauf-Spezifikation + `Measure`-Trait + reale/mock Messung + Pareto-Auswahl.

## 2. Zieltypen (Skizze)
```rust
pub struct Cfg { pub combo: Vec<(AxisId, AxisValue)>, pub seq_lens: Vec<usize> }

pub struct MeasureResult {
    pub time: std::time::Duration, // robuste Zeit (Median oder Min)
    pub deviation: f64,            // Abweichung vs. fp32-Referenz desselben Testsets
    pub valid: Validity,           // Ok | Rejected(reason)
    pub stable: bool,              // false ⇒ als `unstable` markieren
}
pub enum Validity { Ok, NonFinite, Degenerate, OverMaxDeviation }

pub trait Measure { fn measure(&mut self, cfg: &Cfg) -> Result<MeasureResult, PrecisionError>; }

pub struct CalibratorConfig {
    pub max_deviation: f64,   // konfigurierbare Maximalabweichung (= inkompatibel)
    pub min_reps: u32,        // ≥ 3
    pub window_ms: f64,       // Zielfensterlänge je Messung (≥ ~1 ms)
    pub cv_threshold: f64,    // Streuungsschwelle (Variationskoeffizient)
    pub hard_cap: HardCap,    // max. Wiederholungen ODER Zeitbudget
    pub slice_budget_ms: f64, // ≤ ~5–10 ms je GPU-Launch (keine Preemption)
}

pub struct ParetoPoint { pub cfg: Cfg, pub time: Duration, pub deviation: f64, pub stable: bool }
```

## 3. Ablauf (verbindlich aus `resident_precision.md`)
1. **Aufzählen** der Kombinationen aus `CapabilitySet` (kartesisch) **mit Pruning**.
2. **Messen** jeder Kombination als Mini-Benchmark; **gleichzeitig** Abweichung vs. fp32-Referenz erfassen.
3. **Verwerfen**: NaN/Inf; degeneriert (konstante Vektoren, Norm außerhalb des plausiblen Bereichs); Abweichung > `max_deviation` (⇒ inkompatibel, **kein** Laufzeitfehler, nur nicht aktiviert).
4. **Pareto-Front** aus Zeit und Abweichung bilden.
5. Profile ableiten → **F3**.
6. Persistieren → **F5** (inkl. `unstable`, Maximalabweichungs-Verwerfungen, Pruning-Ergebnisse).

## 4. Kombinatorik begrenzen (verlustfrei bzgl. Pareto)
- **Separable Achsen getrennt messen** (z. B. Batch/Tile unabhängig von dtype), danach **nur Gewinner kombinieren** — OFAT, aber mit **Verifikationsmessung** der kombinierten Gewinner, weil Wechselwirkungen auftreten (typisch: Tile×dtype, Fusion×Akkumulator). [context §F2/Q12, W]
- **Dominierte Kombinationen früh abbrechen** (bereits langsamer **und** ungenauer als eine bekannte). Da **Abweichung früh billig** und **Zeit teuer** ist: **Abweichung zuerst** messen, dann bei Dominanz Zeit-Samples sparen. [§F2/Q11]
- Nur die Pareto-Front wird benötigt ⇒ Pruning ist verlustfrei bzgl. Ergebnis.

## 5. Messmethodik
### Allgemein (`resident_precision.md`)
- **≥ 3 Wiederholungen** je Kombination.
- **Interleaved** messen (nicht A komplett, dann B) → Lastschwankungen treffen alle gleich. Reihenfolgeeffekte per **ABBA**/Randomisierung dämpfen. [§F2/Q8]
- Nur **Relationen** zählen; Stabilitätsprüfung auf dem **Verhältnis** zwischen Kombinationen, nicht auf Einzelzeiten.
- Robuste Statistik: **Median oder Minimum** (Last verzerrt nur nach oben; Minimum = Untergrenze der Hardwarezeit). Mittelwert nur bei Gesamtdurchsatz. [§F2/Q9]
- **Adaptiv** nach Streuung (CV oder `(max−min)/median`): klein ⇒ stabil/fertig; intern verrauscht ⇒ mehr Samples je Wiederholung; Wiederholungen driften, Samples stabil ⇒ ganzen Durchlauf wiederholen.
- **Abbruch bei Stabilität, nicht nach fester Zeit.** Harte Obergrenze (max. Wiederholungen/Zeitbudget): besten Zwischenstand verwenden, als **`unstable`** loggen+persistieren, **nicht blockieren, nicht abbrechen**. `unstable` darf später (manuell) nachgemessen werden.
- **Niedrige Thread-Priorität**; Testset mit **Längenvarianz bis zur Maximalsequenzlänge** (prüft Verarbeitbarkeit: Speicher/Overflow), **nicht** Retrieval-Qualität.

### GPU-spezifisch (korrektes Benchmarking, context §F2/Q6 [V])
- Launches sind **asynchron**; ein Host-Timer ohne Sync misst nur die Submit-Zeit. **CUDA-Events auf dem Stream** stempeln und blenden Launch-Overhead aus.
- Vor Messung `synchronize`; **Warmup** (JIT, Caches, Clock-Ramp); **L2-Flush** zwischen Läufen; für **kurze** Kernel mehrere Launches/CUDA-Graph pro Messfenster und durch Anzahl teilen.
- Iterationszahl pro Kalibrierlauf auf **Zielfensterlänge** (`window_ms`, z. B. ≥ 1 ms) skalieren.
- **Clock-Fixierung** (`nvidia-smi -lgc`) braucht Admin und wird von GeForce teils nicht unterstützt [W] → **Fallback: Warmup + Streuungskriterium**.
- Fallstrick: implizite CUDA-Init verfälscht Host-Timer → vor der Messung einmal initialisieren.

### Keine GPU-Preemption (context §F2/Q7 [W])
- Zeitbudget je Launch (`slice_budget_ms` ≤ ~5–10 ms), kleine Problemgrößen, in **viele kurze Launches** slicen. Zwischen Slices Prüfpunkt (Atomic-Flag/Queue-Länge), ob ein Produktions-Task wartet → ggf. **abbrechen**. Gesamtbudget der Kalibrierung begrenzen.

## 6. Verwerfungskriterien (context §F2/Q10 [W])
- **NonFinite:** `!x.is_finite()` über alle Elemente (GPU-Reduktion).
- **Degenerate:** Varianz ≈ 0 (konstante Vektoren) oder L2-Norm außerhalb `[ε, K]`.
- **Abweichungsmetrik:** Embeddings `1 − cos(a, ref)`, zusätzlich relativer Fehler `‖a−ref‖/‖ref‖`; Scoring zusätzlich **Rang-Metrik** (Top-k-Overlap / Kendall τ).
- **Richtwerte (projektspezifisch festlegen):** fp16 ~1e-3…1e-2 Cosinus-Abweichung, int8 ~1e-2. In `CalibratorConfig.max_deviation` konfigurierbar.

## 7. Pareto-Auswahl
- 2D-Algorithmus: nach Zeit sortieren, linear durchlaufen, Punkt behalten, wenn Abweichung < bisheriges Minimum → `O(n log n)`. [§F2/Q11]
- Ergebnis: `Vec<ParetoPoint>` (sortiert), mit `stable`-Flag je Punkt.

## 8. Thread-Pool & Priorität (context §F2/Q13 [W])
- Eigener Pool via `std::thread::Builder`, `thread_priority::set_current_thread_priority(ThreadPriority::Min)` beim Start; **getrennt** von der tokio-Runtime (Vorgabe `resident_precision.md`).
- Hinweis: CPU-Priorität beeinflusst **GPU-Konkurrenz nicht** → deshalb zusätzlich die Slice-/Abbruch-Logik aus §5 (keine Preemption).

## 9. Querschnitt
- **Fehler (Q1):** `Incompatible { reason }` wird **nicht** geworfen — Verwerfung ist ein normaler Ausgang (`Validity`). `CalibrationUnstable` nur als Markierung/Log, nie als Betriebsabbruch.
- **Logging (Q2):** Span `calibrate{engine, model_id}`; je Kombination `debug!(cfg, time, deviation, stable)`; verworfene mit Grund (`info!`); am Ende Pareto-Größe. CUDA-Callbacks: Daten per Channel, **nicht** im Callback loggen.
- **Feature-Flags (Q4):** reale Messung (`measure_cuda.rs`) hinter `cuda`; Default-Tests nutzen `FakeMeasure`.

## 10. Teststrategie (Q3, `resident_precision.md` „Tests“)
- **`FakeMeasure`** hinter `Measure`: Basiswert + Rauschen aus **seeded RNG** (`StdRng::seed_from_u64`), Skripte: **stabil / Ausreißer / driftend / bimodal / nie stabil**; injizierter `Clock`-Trait.
- **Stabilitätslogik:** nie-stabil ⇒ Obergrenze erreicht ⇒ `unstable` + bester Zwischenstand (mehrere Seeds). Konvergenz bei geringem Rauschen. Ausreißer werden von Median/Min absorbiert.
- **Pareto:** konstruierte Punkte, dominierte werden verworfen; Monotonie der Front.
- **Verwerfung:** NaN/Inf, degeneriert, > `max_deviation`.
- **Pruning:** separable Achsen getrennt + Gewinner-Verifikation führt zum selben Front-Ergebnis wie Vollenumeration (kleiner Fall).
- **Kein** GPU nötig.

## 11. Verifikationsergebnisse

Messumgebung: GTX 1660 SUPER (cc 7.5, Treiber 610.57.04), CUDA 13.3, `multilingual-e5-small`, PyTorch 2.8 / transformers 4.55 als Messwerkzeug, cudarc `=0.19.9` (Quelltext). Testset: 104 mehrsprachige Texte (de/en/fr/ja/Code), Längen bis zur Trunkierung bei 512 Token. Referenz: fp32 auf CPU.

### Abweichungsschwellen (fp16/int8)

| Pfad | `1−cos` max | `1−cos` mean | rel. Fehler max | nicht-finite |
|------|-------------|--------------|-----------------|--------------|
| fp32 GPU vs. fp32 CPU | 1.2e-7 | 1.2e-8 | 6.7e-7 | 0 |
| fp16 GPU | 3.6e-7 | 1.7e-7 | 7.5e-4 | 0 |
| bf16 GPU | 2.3e-5 | 2.0e-5 | 6.9e-3 | 0 |
| int8 (dynamisch, Linear, CPU) | 3.2e-2 | 2.3e-2 | 0.25 | 0 |

- Die Richtwerte „fp16 1e-3…1e-2“ sind für e5-small um ca. 4 Größenordnungen zu locker. `1−cos` liegt bei fp16 an der fp32-Rauschgrenze. int8 (dynamisch) liegt dagegen über 1e-2.
- `1−cos` unterhalb von ca. 1e-6 ist Rundungsrauschen (Mittelwerte wurden teils negativ). Die Metrik wird auf `max(0, ·)` geklemmt, Werte < 1e-6 gelten als gleich.
- Entscheidung: `max_deviation` (Cosinus) Default **5e-2**. Das nimmt fp16/bf16 mit großem Abstand auf, int8 knapp. Die Unterscheidung der Pfade geschieht über die Pareto-Front, nicht über die Schwelle. Die Schwelle bleibt konfigurierbar, ein Modellwechsel erfordert Neumessung.
- Rangmetrik: Mit doppelten Texten im Testset fiel die Top-5-Überlappung schon bei fp32-GPU gegen fp32-CPU auf im Mittel 0.81 (Gleichstände). Das Testset muss **duplikatfrei** sein, sonst ist die Rangmetrik unbrauchbar.
- Degeneriert-Kriterium: Die Norm des gepoolten, unnormalisierten Embeddings liegt bei 4.53–5.02. Statt eines absoluten `[ε, K]` wird das **Normverhältnis zur Referenz** auf `[0.5, 2.0]` begrenzt. Konstante Vektoren bleiben über Varianz ≈ 0 erkannt.

### Clock-Fixierung auf GeForce

- `nvidia-smi -lgc 1000,1000` ohne Admin: „does not have permission to change clocks“. Die unterstützten Taktstufen werden gelistet (2145 MHz abwärts), das Locking ist mit Admin also prinzipiell möglich.
- Application Clocks sind auf diesem Treiber „deprecated“. Im Ruhezustand liegt der SM-Takt bei 1185 MHz, das Maximum bei 2145 MHz, der Takt rampt also.
- Ergebnis: Locking steht im Normalbetrieb nicht zur Verfügung. **Warmup + Streuungskriterium ist der verbindliche Pfad**, Locking höchstens optional.

### Separabilität (OFAT)

Vorhersage „unabhängige Faktoren multiplizieren sich“ gegenüber Messung (Quotient 1.0 = separabel), Zeit je Element:

| Kombination | vorhergesagt / gemessen |
|-------------|-------------------------|
| fp32 · sdpa · bs1 | 1.16 |
| fp32 · sdpa · bs32 | 1.03 |
| fp16 · eager/sdpa · bs1 | 2.9 |
| fp16 · eager/sdpa · bs32 | 1.16–1.17 |

- Die Annahme gilt nicht uneingeschränkt: Die Batch-Achse wechselwirkt stark mit dtype (bs1 bei fp16 um Faktor ≈ 3 falsch vorhergesagt). Die Attention-Achse wechselwirkt schwach.
- Konsequenz: Gewinner der separablen Achsen werden **je Wert der Leitachse (dtype)** bestimmt, nicht global. Die Verifikationsmessung der kombinierten Gewinner ist Pflicht.
- Auffällig und nicht untersucht: fp16 war mit 11–16 ms/Element deutlich langsamer als fp32 (2.5–9 ms/Element). Die Zeitordnung der Pfade darf daher nie angenommen werden und muss gemessen werden. Ursache vor der Interpretation der Zeitwerte separat klären.

### cudarc-Event-/Graph-API (0.19.9)

- `CudaStream::record_event(Option<CUevent_flags>)`. Der Default ist `CU_EVENT_DISABLE_TIMING`. Für die Zeitmessung **muss `CU_EVENT_DEFAULT` übergeben werden**, `launch.rs` macht das intern genauso.
- `CudaEvent::elapsed_ms(&end)` synchronisiert beide Events (blockierend), prüft denselben Kontext und liefert `f32` ms.
- `CudaEvent::is_complete()` ist ein `cuEventQuery`-Wrapper und liefert `bool`. **Jeder** Fehler ergibt `false`, nicht nur „not ready“. Es eignet sich für das Polling im Slice-Prüfpunkt, Fehler werden darüber aber nicht sichtbar.
- Graph: `begin_capture(CUstreamCaptureMode)`, `end_capture(..)`, `CudaGraph::launch()` sind vorhanden. Damit sind Mehrfach-Launches pro Messfenster möglich.

## 12. Definition of Done
- `Measure`-Trait + `FakeMeasure` + reale CUDA-Messung (hinter `cuda`) implementiert.
- Ablauf 1–4 + Pruning + Pareto umgesetzt; `unstable`-Pfad ohne Blockieren.
- Slice-/Abbruch-Logik (keine Preemption) und eigener Thread-Pool mit niedriger Priorität.
- Pflicht-Tests (Stabilität, Pareto, Verwerfung, Pruning) grün mit ≥ 3 Seeds.
- Liefert `Vec<ParetoPoint>` an F3 und Rohergebnisse (inkl. Verwerfungen/Pruning/`unstable`) an F5.
