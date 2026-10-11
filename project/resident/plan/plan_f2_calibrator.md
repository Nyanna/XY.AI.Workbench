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

## 11. Offene Verifikation / Risiken
- `[V, aber projektspezifisch]` Abweichungsschwellen (fp16/int8) müssen gegen reale e5-small-Embeddings festgelegt werden (Verweis G1).
- `[W]` Clock-Locking auf GeForce nicht verlässlich → Fallback-Streuungskriterium ist Pflichtpfad.
- `[W]` Separabilitätsannahme (OFAT): Wechselwirkungen Tile×dtype / Fusion×Akku → Verifikationsmessung der Gewinner nicht weglassen.
- `cudarc` Event-/Graph-API & `cuEventQuery` vor realer Messung prüfen (vgl. `context_h_q.md`).

## 12. Definition of Done
- `Measure`-Trait + `FakeMeasure` + reale CUDA-Messung (hinter `cuda`) implementiert.
- Ablauf 1–4 + Pruning + Pareto umgesetzt; `unstable`-Pfad ohne Blockieren.
- Slice-/Abbruch-Logik (keine Preemption) und eigener Thread-Pool mit niedriger Priorität.
- Pflicht-Tests (Stabilität, Pareto, Verwerfung, Pruning) grün mit ≥ 3 Seeds.
- Liefert `Vec<ParetoPoint>` an F3 und Rohergebnisse (inkl. Verwerfungen/Pruning/`unstable`) an F5.
