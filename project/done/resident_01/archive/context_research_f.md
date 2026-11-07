Lies zunächst alle Dokumente in `/home/user/xyan/xy.ai.workbench/project/resident` ein.

```json
{
  "id" : "call_fecd700b-dd1a-44b0-aa8a-cdcc668509d5",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident",
      "entries" : [ ".:", "resident.md", "resident_first.md", "resident_index.md", "resident_planner.md", "resident_precision.md", "resident_processor.md", "resident_subengine.md", "resident_planplan.md" ]
    } ]
  }
}
```

Erstelle eine Liste aller in `resident_planplan.md` für Gruppe F zu beantwortenden Fragen für eine Web-Recherche. Ein anderer Agent wird diese Fragen recherchieren, beantworten und ein Markdown Kontext File als Ergebnis liefern.
Result Stats: id=25a99733-10ed-4374-b381-90c2572506d2, total: 137136, in: 121, out: 21007, reason: 5246, read: 2836, write: 116008
SystemInit:  id=25a99733-10ed-4374-b381-90c2572506d2, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
ReasoningToken: 5246
Die Dokumente liegen aus dem vorherigen Einlesen vollständig vor (identischer Ordnerstand, Checksummen unverändert); ich stütze mich auf `resident_planplan.md` (Gruppe F) und die Quelle `resident_precision.md`.

Nachfolgend der Recherche-Auftrag für Gruppe F (**Präzision & Kalibrierung**, Pakete F1–F5).

---

# Web-Recherche-Auftrag: Gruppe F — Präzision & Kalibrierung

## Kontext für den Recherche-Agenten

Diese Fragen entstammen dem Metaplan der „Resident"-Engine (Rust-RAG-Engine, residente Modelle/Indizes). Technischer Rahmen für alle Antworten:

- **Sprache/Stack:** Rust; `candle`/`candle-core` (Compute, CUDA-Feature), `cudarc` (Custom-Kernel/Device-Abfrage), `tokenizers`, `tokio`. Kalibrierung und CPU-Kernel laufen in **eigenem Thread-Pool**, nicht im tokio-Runtime-Thread.
- **Zielhardware:** GTX 1660 = **Turing TU116, Compute Capability sm_75**, 6 GB VRAM. Wichtig: diese Turing-Variante unterscheidet sich von RTX-2000-Turing → bitte explizit auf diese Karte beziehen.
- **Designprinzip:** Keine Bit-Reproduzierbarkeit als Ziel; Varianz/Indeterminismus zugunsten Performance erlaubt. Ergebnis der Kalibrierung ist eine **Pareto-Front (Zeit vs. Abweichung)** → daraus abgeleitete, **unbenannte Profile** mit Position `p ∈ [0,1]`. Der Task-Parameter `precision ∈ [0,1]` (0=fast, 1=precision) wird pro Subengine relativ gematcht.
- **Ergebnisformat:** Ein Markdown-Kontext-File, pro Frage mit Antwort, Quellen (URLs) und Kennzeichnung unsicherer/unverifizierter Aussagen.

---

## F1 · CapabilitySet-Erhebung (Achsen/Werte pro Subengine)

1. Welche numerischen Rechenpfade unterstützt eine **GTX 1660 (Turing TU116, sm_75)** tatsächlich: fp32, nativer fp16-Durchsatz, INT8-`dp4a`? **Besitzt die GTX 1660 Tensor Cores** (im Unterschied zu RTX-2060/Turing)? Konsequenz für erreichbare Operanden-/Akkumulator-/Scoring-Pfade.
2. Wie fragt man Device-Capabilities zur Laufzeit ab (Compute Capability, FP16-Support, `dp4a`/`__dp4a`-Verfügbarkeit) über `cudarc` bzw. CUDA Driver/Runtime API in Rust?
3. Welche dtype-/Mixed-Precision-Pfade bietet `candle` (candle-core) für CPU und CUDA (fp32, fp16, bf16, Akkumulationstyp)? Welche Operationen fallen intern auf fp32 zurück, und wie ist das steuerbar?
4. CPU-Seite in Rust: etablierte Umsetzung von fp16→fp32-Upcast-Rechnung (`half` crate) und int8-Scoring (SIMD, `wide`/`std::simd`); welche Achsen (Thread-Zahl, Tile-/Block-Größe) beeinflussen reproduzierbar den Durchsatz?
5. Gibt es etablierte Taxonomien/Achsenmodelle aus Auto-Tuning-Frameworks (cuBLAS/cuDNN-Heuristiken, CUTLASS, TVM/Ansor) für ein erweiterbares „CapabilitySet" (Operanden, Akkumulator, Scoring, Fusion, Batch, Tile)?

## F2 · Calibrator (Aufzählung, Messung, Pruning, Pareto)

6. GPU-Kernel-Benchmarking korrekt: Warum CUDA Events (`cudaEventRecord`/`cudaEventElapsedTime`) statt Host-Timern? Rolle von Warmup, Device-Sync und (fixiertem) Clock-State; wie misst man **kurze** Kernel robust?
7. Die GPU hat **keine Preemption**: Best Practices, um Mini-Benchmarks kurz und begrenzt zu halten (Zeitbudget, Slicing), damit wartende Produktions-Tasks nicht verzögert werden.
8. Warum reduziert **interleaved** Messung (A/B/A/B statt A…A dann B…B) den Bias durch Lastschwankungen? Etablierte Methodik und Fallstricke.
9. Robuste Laufzeitstatistik: Median vs. Minimum vs. Mittelwert; Variationskoeffizient / relative Spannweite als Stabilitätskriterium; adaptive Stopp-Regeln (mehr Samples je Wiederholung vs. ganzen Durchlauf wiederholen). Referenzverfahren (z. B. `criterion`, Benchmark-Literatur).
10. Verwerfungskriterien für ungültige Kombinationen: zuverlässige NaN/Inf-Erkennung; Erkennung „degenerierter" Ausgaben (konstante Vektoren, L2-Norm außerhalb plausiblem Bereich); geeignete Abweichungsmetrik gegen die fp32-Referenz (Cosinus-Abweichung, relativer/absoluter Fehler) und typische Maximalabweichungs-Schwellen.
11. Effiziente Berechnung der **Pareto-Front** (nicht-dominierte Menge über Zeit vs. Abweichung) und **frühes Pruning** dominierter Kombinationen (bereits langsamer *und* ungenauer).
12. Kombinatorische Explosion begrenzen: Unter welchen Annahmen dürfen Achsen als **unabhängig/separabel** getrennt gemessen und nur Gewinner kombiniert werden? Risiken (Wechselwirkungen) und Verfahren aus Auto-Tuning (OFAT vs. faktorielles/fraktionelles Design).
13. Niedrige Thread-Priorität für Hintergrund-Kalibrierung in Rust (OS-Thread-Priorität via `thread_priority` crate / nice-Werte; Windows vs. Linux) in einem eigenen Pool getrennt vom tokio-Runtime-Thread.
14. Abstraktion der Messung hinter einem **Trait (Mock für Tests)**: etablierte Muster, um verrauschte Messwerte zu simulieren und die Stabilitäts-/Abbruchlogik (Obergrenze → `unstable`) testbar zu machen.

## F3 · Profilbildung (Spread der Pareto-Front → Positionen)

15. Normierung zweier Zielachsen (Zeit, Abweichung) auf [0,1], Zeit ggf. **logarithmisch**: etablierte Verfahren und Fallstricke vor einer Bogenlängen-Parametrisierung.
16. **Bogenlängen-Parametrisierung** einer diskreten Pareto-Front: Berechnung kumulierter Distanz und Position `p ∈ [0,1]` je Punkt; warum ist das robuster/verzerrungsärmer als reine Rangnummerierung?
17. Zusammenfassen „redundanter" Punkte über einen **Spread-Schwellwert** (als einziger Konfigparameter) in normierter 1D-Skala entlang der Bogenlänge: geeignete Clustering-/Schwellwert-Verfahren.
18. Erkennung „umgekehrter" Fälle (reduzierte Präzision, die durch Emulation **langsamer** ist) zur Herabstufung/Nicht-Aktivierung: wie identifiziert man dominierte/emulierte Rechenpfade zuverlässig?

## F4 · Task-Matching (`precision: f32`)

19. Gibt es Referenzdesigns für eine kontinuierliche **Quality-/Latency-Stellschraube** (0=fast … 1=precision) in bestehenden Inference-/Retrieval-Systemen, an denen sich Skala und Semantik orientieren lassen?
20. Korrektes Klemmen auf [0,1] und Behandlung von `NaN` als Fehler in Rust (f32-Semantik, `is_nan`, `clamp`-Fallstricke bei NaN).
21. Nearest-Match `min|t − p|` mit **deterministischem Tie-Break** (zum genaueren Profil): robuste, reproduzierbare Implementierung bei Gleitkomma-Gleichstand.

## F5 · CalibrationStore / Persistenz (Fingerprint, Zustände)

22. Hardware-/Umgebungs-Fingerprinting in Rust: zuverlässige Ermittlung von **GPU-Modell, Treiberversion und CUDA-Runtime-/Driver-Version** (NVML via `nvml-wrapper`, `cudarc`, Driver API) sowie **CPU-Modell** (`raw-cpuid`, `sysinfo`).
23. Stabile Fingerprint-/Schlüsselbildung zur Invalidierung (Zustände `Calibrated | Stale | Missing`): welche Felder gehören in den Fingerprint (GPU/Treiber/CUDA/CPU/Modell-ID/Engine-Version), wie versioniert man den Schlüssel robust?
24. Persistenzformat für Kalibrierungsergebnisse inkl. `unstable`-Markierung, Maximalabweichungs-Verwerfungen und Pruning-Ergebnissen: Vergleich serde-Formate (JSON/TOML/`bincode`/`postcard`), Schema-Versionierung, Vorwärts-/Rückwärtskompatibilität.
25. Graceful Degradation: etablierte Muster für `Stale` (mit altem Ergebnis starten) und `Missing` (konservativer Default = genaueste gültige Kombination + Warnung) ohne Nutzer-Blockade; Trigger für manuelle Nachkalibrierung.

---

**Hinweis zur Priorisierung** (implementierungskritisch für Gruppe F):
- **F1 / Frage 1** (Tensor-Core- und `dp4a`-Verfügbarkeit der GTX 1660 TU116) ist die Grundlage für alle verfügbaren Rechenpfade → mit belastbaren, sm_75-/TU116-spezifischen Quellen klären.
- **F2 / Fragen 6–9** (korrektes, preemptionsfreies GPU-Benchmarking + robuste Statistik) bestimmen die Qualität der gesamten Kalibrierung → besonders gründlich belegen.

Soll ich diese Liste zusätzlich als Markdown-Datei im Projektordner ablegen (z. B. `resident_groupF_research.md`) — analog zum zuvor vorgeschlagenen `resident_groupE_research.md`?
