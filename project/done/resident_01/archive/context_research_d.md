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

Erstelle eine Liste aller in `resident_planplan.md` für Gruppe D zu beantwortenden Fragen für eine Web-Recherche. Ein anderer Agent wird diese Fragen recherchieren, beantworten und ein Markdown Kontext File als Ergebnis liefern.
SystemInit:  id=4fa46980-f724-437d-adc0-6d3665b1a51e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 I'll read all the documents in the resident project directory first.
Control Request:
```yaml
id: 0f9c-6
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_first.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  reason: Reading all documents in the resident project directory to understand the context and extract Group D questions from resident_planplan.md
```
/answer 0f9c-6 allow
Result Stats: id=4fa46980-f724-437d-adc0-6d3665b1a51e, total: 59261, in: 119, out: 7424, reason: 2551, read: 2836, write: 51718
ReasoningToken: 177
ReasoningToken: 2374
Ich habe alle acht Dokumente eingelesen. Nachfolgend die aus `resident_planplan.md` abgeleitete Liste aller für **Gruppe D (Subengine-Ressourcen)** zu beantwortenden Fragen, aufbereitet als Recherche-Auftrag für den nachgelagerten Web-Recherche-Agenten.

---

# Web-Recherche-Auftrag: Gruppe D — Subengine-Ressourcen (Resident-Engine)

## Fixer Stack-Kontext (gilt für alle Fragen)
Die Antworten müssen sich auf diesen Stack beziehen und möglichst konkrete Versionen/Crate-Namen/API-Signaturen nennen:
- **Compute:** `candle-core` / `candle-transformers` mit Feature `cuda`, optional `cudarc` für Custom-Kernel
- **Modell-Loading:** `safetensors`
- **Tokenizer:** `tokenizers`-Crate (CPU-only)
- **Index:** `parquet` + mmap, columnar
- **Concurrency:** `tokio`, `crossbeam`/`parking_lot`
- **Fehler/Logging:** `thiserror`, `tracing`
- **Zielhardware:** GTX 1660 (6 GB VRAM, Turing / **sm_75**)
- **Konkretes Modell:** `intfloat/multilingual-e5-small` (Dense Bi-Encoder, BERT-Backbone + XLM-R-Tokenizer, 384 Dim)
- **Ergebnisformat:** Ein Markdown-Kontextfile pro Paket-Abschnitt, mit Quellenangaben (Links), markiert nach *verifiziert* vs. *unsicher*.

---

## D1 — Subengine-Gemeinsames (Lifecycle & Pull)
1. Welche bewährten Rust-Muster/Primitiven gibt es für **Lazy-Init mit Fehlerwiederholung** (`get_or_try_init`) **plus Begrenzung paralleler Initialisierungen** per Semaphore (z. B. `tokio::sync::OnceCell` + `Semaphore`)? Verhalten bei Init-Fehler und erneutem Versuch?
2. Etablierte Muster für **Pull-basiertes, selbstorganisierendes Scheduling** mit Lastverteilung über **gleitenden Durchsatz-Mittelwert** (schnellere Engine holt mehr) — Referenzen/Trade-offs?
3. Übliche Realisierung von **Filterprädikaten beim Claim** (Priorität, Modell-/Index-Affinität, Verfügbarkeit/„Modell nicht gesperrt") in Lock-free-Task-Systemen?
4. Standard-Umsetzung von **TTL-/Hysterese-basierter Eviction** mit Bedingung `in_flight == 0` **und** offene Leases `== 0` **und** abgelaufener TTL — Referenzen für Lease-/Refcount-gesteuerte Ressourcenfreigabe.

## D2 — Device & Ausführung (Lane / ExecContext)
5. Kann `candle`'s `CudaDevice` mit einem **eigenen (Non-Default-)Stream** betrieben werden? Wie werden **Events auf dem Candle-Stream aufgezeichnet/abgewartet**? (konkrete API, candle-core Version)
6. Lässt sich der **Device-Pointer eines Candle-Tensors** im **Primary Context** von einem eigenen `cudarc`-Kernel lesen? Teilen candle und cudarc denselben CUDA-Context?
7. Realistische **CUDA-Context-Init-Kosten** (erste Context-Erzeugung, Sekundenbereich) und Konsequenzen für Lazy-Init-Timing — Messwerte/Erfahrungsberichte.
8. Üblicher Aufbau einer **Lane/ExecContext-Abstraktion** (Stream + Priorität + Events + ParamBlock + Scratch) über cudarc; Unterstützung für **Stream-Prioritäten** (Query-Lane hoch, Index-Lane Durchsatz, Copy-Lane) auf Turing?

## D3 — GPU-Speicher & Arenen
9. Wie bindet man eine **fremde (eigene) GPU-Allokation als `CudaStorage` in candle** ein, **ohne dass candle sie freigibt**? (API, Ownership-Fallstricke, Version)
10. Best Practices zur **Vermeidung von `cudaMalloc`/`cudaFree` im laufenden Betrieb** (implizite Device-Synchronisation) — Arena-/Pool-Allokator-Muster, verfügbare cudarc-Primitiven.
11. Umsetzung von **Pinned Host Memory (Staging)** für H2D/D2H über cudarc und **stream-geordneter Wiederverwendung** (Region erst frei nach erreichtem Event).
12. Laufzeit-Abfrage des **VRAM-Budgets via `cuMemGetInfo`** über cudarc; realistische Context-/Compositor-Overheads auf einer 6-GB-Turing-Karte.
13. Umgang mit **dtype pro Device** (GPU f16 / CPU f32) in candle: f16-Support des BERT-Forward, Toleranzen bei gemischter CPU/GPU-Indexierung.

## D4 — Modell-Ressourcen (Encoder)
14. Sind die **Gewichtsnamen von `intfloat/multilingual-e5-small` kompatibel mit candle's `BertModel`**? Mapping-Bedarf, bekannte Namenskonflikte (`candle-transformers::models::bert`).
15. Liegt `model.safetensors` im HF-Repo vor, oder ist eine **Konvertierung** (von `pytorch_model.bin`) nötig? Übliche Konvertierungswege.
16. Bestätigung, dass **candle-transformers kein ColBERT-Modul** besitzt — welcher Ansatz (Backbone + eigener Projektionskopf) ist Stand der Praxis?
17. Für den Wechselpfad: **Ort/Format des ColBERT-Projektionskopfs** (`Linear` z. B. 768→128, „Dense"-Modul) in Checkpoints wie `jina-colbert-v2` / `answerai-colbert-small-v1` (safetensors vs. separater Dense-Ordner)?
18. Gängige Umsetzung von **Token-Budget-Batching mit Längen-Bucketing und Padding** auf gemeinsame `seq_len` in candle-Encoder-Pipelines.

## D5 — Tokenizer & Normalizer (CPU, geteilt) — **Widerspruch auflösen**
19. **GPU-Tokenizer / Token-ID→Speicheradress-Mapping:** Ist der in `resident.md` angedachte Ansatz (CPU löst ID → VRAM-Adresse im residenten Modell auf, eigener cudarc-Gather-Kernel statt `index_select`) technisch realistisch, und mit welchem Aufwand/Nutzen? Gibt es Präzedenzfälle? → Grundlage, um den Widerspruch zu `resident_subengine.md` (dort nur „Ausblick") zu klären.
20. Liefert das `tokenizers`-Crate zuverlässig **`u32`-IDs + Attention-Maske** (`get_attention_mask`), und wo gehört die **Normalisierung** (Unicode/Lowercase) hin (Tokenizer-intern, CPU)?
21. Besonderheiten für **Query- vs. Dokument-Modus** (e5: Präfixe `query:` / `passage:`; ColBERT: Marker-Tokens, `[MASK]`-Padding, `doc_maxlen`-Kürzung) im `tokenizers`-Crate.

## D6 — Kernels & Scorer
22. Vergleich **MaxSim als Candle-Kachel-Kette (matmul → Maske → max → Summe)** vs. **fused NVRTC-Kernel**: Bandbreite/Latenz, bekannte Benchmarks, wann lohnt der fused Kernel?
23. Laufzeitkompilierung via **NVRTC über cudarc** (`--gpu-architecture=sm_75`): Vorgehen, CUBIN-Fallback bei zu altem Treiber, `KernelRegistry`-Muster `(name, dtype, sm_arch)`.
24. Effizienter **CPU-SIMD-Pfad für MaxSim** (laufendes Maximum je Query-Token, keine Matrix über Slots) in Rust — verfügbare Crates/Intrinsics.
25. Korrekte **Padding-Maskierung** bei MaxSim: Dokumentseite mit `-inf` maskieren, Query-`[MASK]`-Tokens mitzählen — bestätigte Referenzimplementierungen (ColBERT/PLAID).
26. Muster für **Work-Slice-Scheduling mit Zeitbudget** und **Preemption nur an Slice-Grenzen** auf CUDA (GPU hat keine Preemption) — Referenzen.

## D7 — Index-Segmente in der Subengine
27. Etablierte **Konsistenzmodelle für GPU-Index-Synchronisation** (Segment-Generationen / Epochen / Versionierung) bei nebenläufigem Lesen durch Kernel und Schreiben durch Host — Referenzen und Trade-offs. (Im Dokument ausdrücklich „noch festzulegen".)
28. Umsetzung von **Slot-Klassen fester Größe** (z. B. 32/64/128 Tokens) für lookup-freie Kernel bei variablen Längen; Gültigkeitsbit/Tombstone-Muster beim Upsert ohne In-Place während laufender Kernels.
29. Muster für **bidirektionale Sync** (Index-Objekt ↔ RAM/VRAM) über Segment-Ereignisse mit Async-Copy.

## D8 — CPU-vs-GPU-Unterschiede & Ausbaupfad
30. Belastbarkeit des **API-bruchfreien Ausbaupfads** (Phase 1 Candle-Hüllen → Phase 5 eigener cuBLAS-Forward): bekannte Fallstricke beim schrittweisen Ersetzen von Candle-Ops hinter stabilen Traits (`Op`, `Region`, `ExecContext`).
31. Vergleichspunkte CPU- vs. GPU-Subengine für denselben Encoder (candle-CPU-Backend vs. CUDA): erwartbare Qualitäts-/Toleranzunterschiede bei f32 vs. f16.

---

## Übergreifende Vorab-Verifikation (Gruppe-D-relevant, aus §10 „Prüfen" + `resident_first` „Prüfen")
32. **Version-Alignment:** Ist die `cudarc`-Version **innerhalb von candle identisch** mit der direkt im Projekt genutzten? (sonst inkompatible Typen) — welche candle/cudarc-Versionspaare sind kompatibel?
33. Lässt sich eine **fremde Allokation in candle einbinden, ohne dass candle sie freigibt** (Ownership-Grenze)? (ergänzt Frage 9)
34. **Mean-Pooling mit Maske + L2** gegen `sentence-transformers`-Referenz: bekannte zulässige Cosinus-Abweichung; Referenzwerte für e5-small.
35. **Präfix-Behandlung** (`query:` / `passage:`) — dokumentierter Qualitätsverlust bei falschem/fehlendem Präfix?

---

**Hinweis zur Priorisierung für den Recherche-Agenten:** Höchstes Risiko/Klärungsbedarf laut Metaplan: **Frage 19** (D5-Widerspruch GPU-Tokenizer), **Frage 27** (D7 Konsistenzmodell Index-Sync), **Fragen 5–6, 9, 32–33** (candle↔cudarc-Interop als Fundament der gesamten GPU-Subengine) sowie **Fragen 14–15, 34–35** (e5-small direkt implementierungskritisch).

Soll ich diese Liste zusätzlich als Markdown-Datei im Projektordner ablegen (z. B. `resident_groupD_research.md`), oder genügt die Ausgabe hier als Übergabe an den Recherche-Agenten?
