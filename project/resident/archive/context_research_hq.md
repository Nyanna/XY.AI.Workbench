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

Erstelle eine Liste aller in `resident_planplan.md` für Gruppe H und Q zu beantwortenden Fragen für eine Web-Recherche. Ein anderer Agent wird diese Fragen recherchieren, beantworten und ein Markdown Kontext File als Ergebnis liefern.
SystemInit:  id=9c291187-b139-41f4-8823-2e4f69163d78, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 Ich lese zunächst alle Dokumente im Verzeichnis ein.
Control Request:
```yaml
id: 0f9c-9
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
  reason: "Einlesen aller Projektdokumente im resident-Verzeichnis, um den Kontext zu verstehen und die Fragen für Gruppe H und Q aus resident_planplan.md zu extrahieren."
```
/answer 0f9c-9 allow
Result Stats: id=9c291187-b139-41f4-8823-2e4f69163d78, total: 61122, in: 119, out: 9382, reason: 3934, read: 2839, write: 51621
ReasoningToken: 30
ReasoningToken: 3904
Ich habe alle acht Dokumente eingelesen. Aus `resident_planplan.md` sind für die **Gruppe H (Ausblick/zurückgestellt)** die Pakete H1–H4 sowie die in der Schärfung ergänzten H5/H6, und für den **Querschnitt Q** die Pakete Q1–Q4 relevant. Daraus habe ich die Punkte extrahiert, die sich für eine **externe Web-Recherche** eignen (Bibliotheks-/API-Dokumentation, CUDA-Programmiermodell, Fachpaper wie ColBERT/PLAID, Rust-Best-Practices). Rein interne Designentscheidungen habe ich – wo möglich – in recherchierbare technische Faktenfragen umformuliert und am Ende die nicht web-recherchierbaren Punkte abgegrenzt.

---

# Rechercheauftrag: Offene Fragen Gruppe H & Querschnitt Q („Resident"-Engine)

**Zielergebnis des Recherche-Agenten:** Ein Markdown-Kontextfile, das jede der folgenden Fragen mit belegten Antworten (Quellenangabe: Crate-Doc-Version, CUDA-Guide-Abschnitt, Paper, Repo) beantwortet.

**Technologie-Kontext (Stack):** Rust, `candle`/`candle-core` (Feature `cuda`), `cudarc`, `tokenizers`, `tokio`, `crossbeam`/`parking_lot`, `parquet` + mmap, `safetensors`. Zielhardware: GTX 1660 (Turing, sm75), 6 GB VRAM.

---

## Gruppe H — Ausblick / zurückgestellt (nur Erweiterungspunkte sichern)

### H1 · GPU-Ringbuffer-Modus (persistenter Kernel, SPSC-Ring)
1. Wie werden **persistente (resident laufende) CUDA-Kernel** umgesetzt? Welche etablierten Muster und Fallstricke gibt es (Grid-Größe ≤ SMs × Occupancy, damit alle Blöcke gleichzeitig resident sind; Vermeidung von Deadlocks bei nicht-resident schedulbaren Blöcken)?
2. Wie funktioniert **lock-freie SPSC-Ringbuffer-Kommunikation zwischen Host (CPU) und einem laufenden GPU-Kernel**? Welche Speichermechanismen (gemapptes Pinned/Host-Memory, `cudaHostAlloc`/mapped, Unified Memory) sind dafür geeignet?
3. Welche **CUDA-Synchronisations-/Sichtbarkeitsmechanismen** sind für Host↔Device-Signalisierung ohne Kernel-Beendigung nötig (`__threadfence_system`, `volatile`, System-Scope-Atomics, Memory Fences)?
4. Verfügbarkeit und Semantik von **atomaren Operationen (CAS) auf der GPU** für das Exit-Protokoll (`atomicCAS`, System-Scope-Atomics `atomic_system`) — speziell für das Muster `CAS(w, r, r | EXIT)`.
5. Unterstützung des **Acquire-Load (`ld.acquire`) ab sm70+ / auf Turing (sm75)**: Garantien, PTX-Syntax, Sichtbarkeitsvoraussetzungen (Event/Fence vor dem Lesen der neuen Tabelle).
6. Wie implementiert man **`idle_spin` / `max_run_time`** (zeitscheibenbasierter Kernel mit Neustart durch den Host) angesichts fehlender GPU-Preemption? Best Practices für Stop-Flag-Prüfung nur an Zeitscheibengrenzen.
7. Welche Unterstützung bieten **`cudarc` und `candle`** für das Laden/Starten eigener (persistenter) Kernel aus PTX/CUBIN und für langlaufende Launches?

### H2 · Gleitende Indizierung (Sliding Window)
8. Welche etablierten **Sliding-Window-Embedding-Verfahren für lange Dokumente** gibt es (überlappende Fenster, Sliding-Window-Attention) und welche Referenzimplementierungen existieren?
9. Wie werden Sliding-Window-Token-Embeddings mit Transformer-Encodern (BERT / e5-small) unter der `max_seq_len`-Grenze praktisch umgesetzt (Überlappung, Aggregation der Fenster)?
10. Gibt es **GPU-Kernel-Techniken für gleitende Embedding-Berechnung** (Fensterverschiebung ohne vollständige Neuberechnung) und entsprechende Referenzen?

### H3 · Funnel / PLAID & Cross-Encoder / Reranker
11. Wie funktioniert **PLAID** (und `next-plaid`) — mehrstufiges zentroidbasiertes Kandidaten-Pruning für ColBERT-Retrieval? (Originalpaper, Ablauf der Stufen, Referenz-Repo)
12. Wie werden **Centroid-Codes und Residual-Kompression in ColBERTv2 / PLAID** erzeugt und verwendet (Clustering, Quantisierung, Zentroid-Tabelle)?
13. Architektur und **Eingabeformat von Cross-Encoder-Rerankern** (z. B. `BAAI/bge-reranker-v2-m3`): gemeinsame Kodierung von (Query, Dokument)-Paaren, Scoring-Kopf, Ressourcenbedarf, `safetensors`-Verfügbarkeit.
14. Wie ist eine **Funnel-/Cascade-Retrieval-Pipeline** strukturiert (billiger Grobfilter auf Centroid-Codes → exaktes MaxSim nur auf der Spitze)? Welche Parameter steuern die Stufen?

### H4 · Graph-Capture & Persistent-Scorer
15. Wie funktioniert **CUDA-Graph-Capture** (`cudaStreamBeginCapture`/`cudaStreamEndCapture`, Instanziierung, Replay) und welche Einschränkungen gelten (erlaubte/verbotene Operationen während Capture)?
16. Lässt sich **CUDA-Graph-Capture über `candle`-Ops auf einem eigenen (nicht-Default-)Stream** durchführen? Welche bekannten Probleme/Issues gibt es dabei?
17. Best Practices für den **Cache-Schlüssel von Graph-Instanzen** (Shape-Buckets, feste Ein-/Ausgabe-Regionen, Invalidierung bei Pointer-/`gen`-Änderung).
18. Unterstützung von **CUDA-Graphs in `cudarc`** (API-Abdeckung, Beispiele).

### H5 · Signalkompression (FFT / Wavelet / „Semantic Spectrogram") — außerhalb Engine-Scope, nur State-of-the-Art
19. Welche Ansätze existieren für **Frequenz-/Transformationsdomänen-Kompression von Embedding-Vektoren** (DCT/FFT/Wavelet) als Retrieval-Vorfilter?
20. Gibt es Arbeiten zu **JPEG-/DCT-artiger Kompression von Embeddings** bzw. zu „Semantic Spectrogram"/Wavelet-Repräsentationen für semantische Suche? (Belege oder explizit: keine etablierten Referenzen)
21. Nur zur Erweiterbarkeitsprüfung: Lassen typische Vorfilter-Architekturen eine **„Frequenz-/Kompressionsdomäne" als Scope/AccessMethod der Oberschicht** grundsätzlich zu (konzeptioneller Stand der Technik)?

### H6 · Mutable Weights / Online-Bandit — außerhalb Engine-Scope, nur State-of-the-Art
22. Wie funktionieren **Contextual-Bandit-Ansätze als Meta-Router** über Retrieval-/Cascade-Strategien (Reward-Signal, Exploration/Exploitation)?
23. Welche Verfahren zur **Online-Adaption von Modellgewichten (mutable weights)** während des Betriebs existieren, und wie praktikabel sind sie (Risiken, Referenzen)?
24. Wie sind **Reward-Stream-basierte Online-Learning-Pipelines** aufgebaut (Datenfluss, Trennung von Inferenz und Update)?

---

## Querschnitt Q (begleitet alle Gruppen)

### Q1 · Fehler-Taxonomie (`thiserror`)
25. Best Practices für **Fehler-Taxonomie-Design mit `thiserror`** (Error-Enums, `#[from]`, `#[error(...)]`, Source-Chaining, `#[source]`).
26. Wie strukturiert man **geschichtete/hierarchische Fehlertypen über mehrere Module** (pro Komponente ein Enum vs. ein zentrales Enum; Aggregation/Weiterreichen)?
27. Abgrenzung **Library- vs. Application-Fehler** in Rust; Zusammenspiel `thiserror` (Bibliothek) und `anyhow` (Anwendung) — Empfehlung für eine Engine mit öffentlicher API.
28. Muster für **intern-sichtbare vs. nach außen exponierte Fehler** (z. B. `StaleMapping` nur intern) — wie API-stabil kapseln?

### Q2 · Logging (`tracing`)
29. Best Practices für **`tracing`-Spans und strukturiertes Logging in async Rust** (`#[instrument]`, Span-Felder, Span pro Task-ID / Ressource / Manager-Zyklus).
30. Wie **propagiert/korreliert man Spans über async-Grenzen und Task-/Thread-Wechsel** (Task-ID-Weitergabe, `Span::current`, `in_scope`, `Instrument`)?
31. Empfohlene **Subscriber-Konfiguration mit `tokio`** (`tracing-subscriber`, Env-Filter, Log-Level pro Modul/Target, JSON vs. human-readable).
32. Wie loggt man aus **CPU-Thread-Pools / Nicht-tokio-Threads** (Kalibrierung, CUDA-Host-Callbacks) korrekt mit `tracing`? Einschränkungen in Callbacks (keine CUDA-API, kurze Arbeit).

### Q3 · Teststrategie & Mocks
33. Best Practices zum **Mocken von Traits in Rust** (`mockall` vs. manuelle Mocks) — konkret für Kernel-Mock und Mess-/Benchmark-Mock mit Trait-Abstraktion.
34. Werkzeuge und Vorgehen zum **Testen lock-freier/nebenläufiger Datenstrukturen** (`loom`, `shuttle`) — für die CAS-basierte Queue-Semantik (Append/Claim/Cancel/Unlink).
35. Wie strukturiert man **GPU-freie Tests** über Feature-Flags/Mock-Backends, sodass die gesamte Logik ohne CUDA testbar bleibt?
36. Verfahren für **deterministisches Testen von Synchronisations-/Zustandslogik** (z. B. zustandsbasierter Stamp-Abgleich, Signal-vor-Scan-Quittierung, ABA-Szenarien).
37. Best Practices für **simulierte verrauschte Messwerte** (Mock-Messfunktion) zum Test der Kalibrierungs-/Stabilitätslogik (Abbruch bei Obergrenze → `unstable`).

### Q4 · Feature-Flags
38. Best Practices für **Cargo-Feature-Flags zur Kapselung von CUDA-Pfaden** (optionale Dependencies, `#[cfg(feature = "cuda")]`, additive Features, Default-Features).
39. Wie ist das **`cuda`-Feature von `candle-core`** aufgebaut und wie reicht man es sauber durch das eigene Crate (Feature-Forwarding, `cudarc` als optionale Dependency)?
40. Muster für **bedingte Kompilierung**, die sicherstellen, dass der No-GPU-Pfad vollständig kompiliert und getestet werden kann (Trait-Objekt-Austausch statt `cfg`-Streuung).

---

## Nicht web-recherchierbar (rein interne Designentscheidungen — als Hinweis)

Folgende in H/Q berührte Punkte sind **projektinterne Festlegungen** und keine Rechercheaufgabe; der Recherche-Agent sollte sie nicht beantworten, sondern höchstens durch obige Fakten stützen:

- Konkrete Konsolidierung der projekteigenen Fehler-Liste (`UnsupportedScope`, `ArenaExhausted`, `TransferFailed`, `Incompatible`, `NoActiveProfile` usw.) zu einer Taxonomie (Q1 — nutzt aber Frage 25–28).
- Welche Ereignisse pro Gruppe konkret geloggt werden (Q2 — Projektentscheidung).
- Die konkreten Testmatrizen je Paket (Q3 — ergibt sich aus den Dokumenten E5/E6, F2–F5, B1).
- Die Entscheidung, H1–H6 in Phase 1 nur als Erweiterungspunkte zu sichern (Projekt-Scope).

---

Möchtest du, dass ich diese Liste direkt als Markdown-Datei (z. B. `resident_research_HQ.md`) in das `resident`-Verzeichnis schreibe, damit sie dem Recherche-Agenten als Auftrag übergeben werden kann? (Dafür bräuchte ich eine Schreibfreigabe bzw. den gewünschten Zielpfad.)
