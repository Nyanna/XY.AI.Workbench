Erstelle Umsetzungspläne für Agenten im Verzeichnis `/home/user/xyan/xy.ai.workbench/project/resident/plan` für Gruppe H und Q aus `resident_planplan.md`.

* `resident_index.md` ist nur relevant für die Implementierung eines Vektorindexes und keine Indexdatei.

Projektpläne:
```json
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident",
      "entries" : [ ".:", "resident.md", "resident_first.md", "resident_index.md", "resident_planner.md", "resident_precision.md", "resident_processor.md", "resident_subengine.md", "resident_planplan.md" ]
    } ]
  }
}
Kontext Dateien für Gruppen:
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident/context",
      "entries" : [ ".:", "context_h_q.md", "context_a.md", "context_b.md", "context_c.md", "context_d.md", "context_e.md", "context_f.md", "context_g.md" ]
    } ]
  }
}
```
SystemInit:  id=aed6e3c8-93f9-4d67-a122-b8fb853e5d6d, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 I'll start by reading the relevant files to understand the project and the requirements for groups H and Q.
Control Request:
```yaml
id: b929-1
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_h_q.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  reason: "Reading the master planning file, the H/Q context file, and the project overview to understand what implementation plans for groups H and Q must contain."
```
/answer b929-1 allow
ReasoningToken: 234
Text:
 I have the metaplan and the H/Q research context. To write accurate implementation plans (especially the extension points for Group H and the cross-cutting conventions for Q), I need the exact trait/extension-point details from the source documents referenced in the packages. Let me read them.
Control Request:
```yaml
id: b929-2
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_first.md
  reason: "Reading the authoritative source documents (subengine, index, planner, processor, precision, first) to extract the exact traits and extension points (Op/requires(), Index trait, Kernel trait, Parameterobjekt, §4.6 Ringbuffer/Graph-Capture, §11 Funnel) needed to write precise H and Q implementation plans."
```
/answer b929-2 allow
ReasoningToken: 1839
Text:
 I now have the full picture: the metaplan (packages H1–H6, Q1–Q4), the researched answers in `context_h_q.md`, and the authoritative trait/extension-point details from the source docs. 

Key framing I'll apply:
- **Group H** = *Ausblick/zurückgestellt* → the plans must **only secure extension points** (Parameterobjekt, `requires()`-Affinität, Op-Austausch hinter gleichem Trait), **no detailed implementation**. H5/H6 are explicitly *außerhalb des Engine-Scopes* → only ensure nothing is precluded + document divergence.
- **Group Q** = cross-cutting conventions that other groups consume → concrete, binding conventions.

I'll create one self-contained, agent-assignable plan per package (isolated per the metaplan), plus a short orientation overview.
Control Request:
```yaml
id: b929-3
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h_q_overview.md
    mode: replace
    content: |
      # Umsetzungspläne Gruppe H & Q — Überblick

      Stufe-2-Pläne zu `resident_planplan.md`. Je Paket ein eigenständiger, einzeln zuweisbarer Umsetzungsplan für einen Agenten. Zielprojekt: Rust-Crate unter `/home/user/xyan/xy.ai.workbench/rag`.

      Recherchegrundlage für alle Pläne: `project/resident/context/context_h_q.md` (Stand ca. Mitte 2026, Zielhardware GTX 1660 / TU116 / sm75 / 6 GB VRAM). Mit **[prüfen]** markierte Punkte sind vor der Implementierung gegen die aktuelle Crate-/CUDA-Version zu verifizieren.

      ## Grundregel Gruppe H (Ausblick / zurückgestellt)

      Für **alle** H-Pakete gilt: In Stufe 2 werden **nur Erweiterungspunkte gesichert** —
      1. erweiterbares **Parameterobjekt** (Task-Params, `#[non_exhaustive]` / Builder),
      2. **`requires()`-Affinität** als Erweiterungsfeld,
      3. **Op-Austausch hinter demselben Trait** (gleiche `Op`/`Launcher`/`Scorer`-Schnittstelle).

      **Keine Detailplanung, kein Kern-Workstream.** Jeder H-Plan liefert: (a) was später kommt, (b) die destillierte Recherche, (c) eine Checkliste „darf jetzt nicht verbaut werden", (d) explizit „jetzt NICHT umsetzen".

      Status-Legende (aus dem Thesen-Abgleich im Metaplan): **K** Kern · **I** Hook vorbereitet · **A** Ausblick/später in der Engine · **E** bewusst außerhalb der Engine · **—** nicht adressiert.

      ## Dateien

      | Paket | Datei | Status | Charakter |
      |---|---|---|---|
      | H1 GPU-Ringbuffer-Modus | `plan_h1_gpu_ringbuffer.md` | A | Erweiterungspunkte sichern |
      | H2 Gleitende Indizierung | `plan_h2_gleitende_indizierung.md` | A/I | Erweiterungspunkte sichern |
      | H3 Funnel/PLAID & Reranker | `plan_h3_funnel_plaid_reranker.md` | I/A | Erweiterungspunkte sichern |
      | H4 Graph-Capture & Persistent-Scorer | `plan_h4_graph_capture.md` | A | Erweiterungspunkte sichern |
      | H5 Signalkompression | `plan_h5_signalkompression.md` | E/— | nur Nicht-Ausschluss + Divergenz |
      | H6 Mutable Weights / Online-Bandit | `plan_h6_mutable_weights_bandit.md` | — | nur Nicht-Ausschluss + Divergenz |
      | Q1 Fehler-Taxonomie (`thiserror`) | `plan_q1_fehler_taxonomie.md` | Querschnitt | verbindliche Konvention |
      | Q2 Logging (`tracing`) | `plan_q2_logging.md` | Querschnitt | verbindliche Konvention |
      | Q3 Teststrategie & Mocks | `plan_q3_teststrategie.md` | Querschnitt | verbindliche Konvention |
      | Q4 Feature-Flags | `plan_q4_feature_flags.md` | Querschnitt | verbindliche Konvention |

      ## Reihenfolge / Abhängigkeiten

      - **Q zuerst fixieren** (nach A4): Q1→Q4 liefern die Konventionen, die alle anderen Gruppen konsumieren. Q4 (Feature-Flags) und Q3 (No-GPU-Pfad) sind Voraussetzung dafür, dass H-Erweiterungspunkte überhaupt GPU-frei testbar bleiben.
      - **H am Ende** (nur umreißen): H1/H4 hängen an E4 (Manager-Disziplin, Acquire-Load-Pointer) und §4.6 (Launcher-Decorator); H2 an C3 (Planner-Regel-Registry) und B4/§4.4 (Entry-/Slot-Modell); H3 an E2 (ScopeResolver/Centroid) und C1 (Scorer-/Inferenz-Tasktyp); H5/H6 nur an die Offenheit der Enums (`AccessMethod`, `Scope`, Task-Params).

      > Hinweis: `resident_index.md` ist die Spezifikation des **Vektorindex** (Index-Trait, StreamedIndexManager, Kernel-Schnittstelle), **keine** Inhalts-/Indexdatei.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h1_gpu_ringbuffer.md
    mode: replace
    content: |
      # Umsetzungsplan H1 · GPU-Ringbuffer-Modus (persistenter Kernel)

      **Gruppe H — Ausblick/zurückgestellt. Status A** (Thesen-§3: die eigentliche „zentrale Antwort" der These; in Phase 1 bewusst durch das Pull-Task-Queue-Modell ersetzt).

      **Planungsauftrag:** NUR Erweiterungspunkte sichern. Kein persistenter Kernel, kein SPSC-Ring wird jetzt gebaut. Ziel ist, dass der spätere Ringbuffer-Modus **ohne API-Bruch** nachgerüstet werden kann (Ausbaupfad `resident_subengine.md` §9, Phase 4).

      ## 1. Was später kommt (Kurzriss, nicht umsetzen)

      - Residenter Kernel mit begrenzter Lebensdauer (`idle_spin`, `max_run_time`), Neustart durch den Host; Modul, Gewichte, Index und Ring bleiben im VRAM.
      - **Getrennte SPSC-Ringe** für Embedding (+ Results) und MaxSim (+ Results), je ein Schreiber, zwei monotone Zeiger `w`/`r`. CAS nur für den Exit: `CAS(w, r, r | EXIT)`.
      - Manager kennt die eingereihten Requests und damit die **blockierten Segmente**; Freigabe erst, wenn kein vor dem Pointer-Swap eingereihter Request das Segment mehr referenziert.
      - Quellen: `resident.md` (Ausblick, GPU-Ringbuffer-Modus), `resident_index.md` (Ringbuffer-Modus; Manager-Disziplin, Pointer-Lesezeitpunkt), `resident_subengine.md` §4.6 (Persistent-Kernel/Ringbuffer), `resident_processor.md` (Nicht umgesetzt / Erweiterungen).

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H1)

      - **Persistenter Kernel:** Grid = `SMs × cudaOccupancyMaxActiveBlocksPerMultiprocessor(...)` (22 SMs auf der 1660), oder `cudaLaunchCooperativeKernel` (garantiert Co-Residenz, schlägt sonst fehl statt zu deadlocken). Fallstricke: Block-zu-Block-Wartebeziehungen → Deadlock bei nicht-residentem Block; Display-Watchdog (Linux/X11, Windows TDR ~2 s); belegt alle SMs → blockiert nachfolgende Launches.
      - **SPSC-Ring Host↔Kernel:** gemapptes Pinned-Memory (`cudaHostAlloc(..., cudaHostAllocMapped)` + `cudaHostGetDevicePointer`). `head`/`tail` auf getrennten Cachelines (≥128 B), Kapazität Zweierpotenz. **`cudaHostAllocWriteCombined` NICHT setzen** (macht Host-Reads extrem langsam). Device→Host sind posted writes → erst nach `__threadfence_system()` geordnet.
      - **Sichtbarkeit:** Device Release = Daten schreiben → `__threadfence_system()` → Index schreiben (oder `cuda::atomic_ref<T, thread_scope_system>::store(release)`). Device Acquire = `ld.acquire.sys` bzw. `load(acquire)`. `volatile` allein gibt **keine** Ordnung.
      - **Exit-CAS host-seitig** auf dem gemappten Speicher (`AtomicU64::compare_exchange`); GPU liest `w`/Flag nur per `ld.acquire.sys`. Vermeidet GPU-System-Atomics über PCIe (`cudaDevAttrHostNativeAtomicSupported` ist plattformabhängig **[prüfen]**).
      - **`ld.acquire.sys`** seit PTX 6.0 / sm_70 → auf Turing (sm75) verfügbar. Scope-Suffix `.sys` für Host↔Device nötig. Beim Austausch von Device-Tabellen zusätzlich Event/Stream-Sync.
      - **Zeitscheibe:** `clock64()`/`%globaltimer`, Idle via `__nanosleep(n)` (sm70+). Stop-Flag/Ring nur an Scheibengrenzen prüfen. Scheibe deutlich < Watchdog-Limit (z. B. ≤ 100 ms) **[Empfehlung]**.
      - **Crates:** Für langlaufende persistente Kernel **`cudarc` direkt** auf eigenem Stream nutzen (Driver-API, Cooperative-Launch über `result::*`), `candle` nur für Tensor-Ops. `candle` ist dafür nicht gedacht. Genaue API der aktuellen `cudarc`-Version **[prüfen]**.

      ## 3. Zu sichernde Erweiterungspunkte (Checkliste — darf jetzt nicht verbaut werden)

      1. **Lane/ExecContext kennt eine persistente Variante.** `ExecContext`/Lane (`resident_subengine.md` §4.1) darf nicht auf „ein Launch = ein Batch = ein Event" festverdrahtet werden. Die Lane-Klasse muss um einen Typ erweiterbar sein, dessen Lebensdauer über viele Work-Items reicht (Fortschrittszähler statt Batch-Event). → Lane-Klasse als offenes Enum/Trait, nicht als festes `{Query,Index,Copy}`-Match ohne Default.
      2. **MemoryManager kann eine langlebige Ring-/State-Region in gemapptem Pinned-Host-Speicher vergeben.** Die Arena-Klassen (`resident_subengine.md` §4.2: ParamBlock/Staging) müssen eine zusätzliche, langlebige Klasse „Ring-State (host-mapped)" aufnehmen können, ohne die `Region{arena,offset,len,gen}`-Signatur zu ändern. Kein Pointer in Signaturen.
      3. **Pointer-Lesezeitpunkt ist abstrahiert.** Die Manager-Disziplin (E4, `resident_index.md`) muss beide Modi tragen: (a) Pointer als Launch-Argument (Swap wirkt ab nächstem Launch) und (b) Acquire-Load im persistenten Modus. Die Swap-/CoW-Logik (atomarer 8-Byte-Pointer-Swap, Root→Segmenttabelle→Segment) darf keine Annahme „Pointer wird nur zwischen Launches gelesen" hartkodieren.
      4. **Residency-Tracker kann Segmente als „blockiert" markieren.** Der Residency-/Mapping-Zustand (E7/E5) muss ein Feld/Prädikat „durch eingereihte Ring-Requests gesperrt" aufnehmen können (Sperrmenge = nur Requests vor dem Swap; task-gebundene temporäre Segmente gelten als blockiert bis Auflösung). Eviction muss dieses Prädikat respektieren.
      5. **Launcher ist dekorierbar / austauschbar.** Der Op-Launch geht über eine Launcher-Abstraktion (siehe H4), sodass neben `Direct`/`Capture` später ein Persistent-Ring-Launcher als weitere Implementierung hinter demselben Trait eintritt. Der Scheduler kennt weder Ring noch Kernel (`resident_subengine.md` §2, Invarianten).
      6. **Getrennte Ringe Embedding/MaxSim als Konzept vorgesehen.** Dort, wo Op-/Ergebnis-Pfade definiert werden, keine Annahme „genau ein Ergebniskanal" treffen; Embedding-Ergebnisse (neue Segmente) und MaxSim-Ergebnisse (Hits) sind bereits getrennte Pfade (`resident_index.md`: „Kernel schreibt Segmente").

      ## 4. Jetzt NICHT umsetzen

      Kein persistenter Kernel, keine SPSC-Ring-Implementierung, kein `__threadfence_system`/`ld.acquire.sys`-Code, keine Cooperative-Launch-Konfiguration, keine Zeitscheiben-Logik. Diese gehören in Phase 4 des Ausbaupfads.

      ## 5. Offene Verifikationen ([prüfen])

      - `cudarc`-Cooperative-Launch-/Host-Alloc-Wrapper der aktuellen Version.
      - `cudaDevAttrHostNativeAtomicSupported` auf sm75 (entscheidet, dass Exit-CAS host-seitig bleiben muss).
      - Watchdog-Verhalten auf der Zielplattform (Linux/X11) → Zeitscheibengröße.

      ## 6. Definition of Done

      - Die sechs Erweiterungspunkte aus §3 sind in den jeweiligen Kern-Paketen (B2, D1/D2, E4, E5/E7, §4.6-Launcher) als Anforderung dokumentiert bzw. im Code als offene Abstraktion vorhanden (Trait/offenes Enum/Default-Zweig).
      - Ein kurzer ADR-Eintrag hält die Umsetzungsentscheidung fest: „Thesen-§3 Ringbuffer in Phase 1 durch Pull-Queue ersetzt; Ringbuffer = späterer Persistent-Scorer-Modus" (Nachvollziehbarkeit der Thesen-Treue).
      - Ein No-GPU-Test belegt, dass Lane-Klasse, Arena-Klasse und Pointer-Lese-Abstraktion erweiterbar sind (Mock), ohne reale CUDA-Pfade.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h2_gleitende_indizierung.md
    mode: replace
    content: |
      # Umsetzungsplan H2 · Gleitende Indizierung (Sliding Window)

      **Gruppe H — Ausblick/zurückgestellt. Status A/I** (Thesen-§5: Datenmodell — Entry variabler Länge, Slot-Klassen — ist bereits der Hook; Kernel/Pipeline später).

      **Planungsauftrag:** NUR Erweiterungspunkte sichern. Kein gleitender Kernel, keine Fenster-Pipeline wird jetzt gebaut.

      ## 1. Was später kommt (Kurzriss, nicht umsetzen)

      - Embedding-Input ist ein großer Chunk; Parameter konfigurieren einen **gleitenden Embedding-Kernel**, der Fenster erzeugt, statt dass die Oberschicht vor-chunkt.
      - Quellen: `resident.md` (Ausblick: gleitende Indizierung), `resident_planner.md` (Expansionsregel „Gleitende Indizierung (Ausblick)").

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H2)

      - **Etablierte Verfahren:** überlappende Chunks mit Stride; Sliding-Window-Attention (Longformer 2020, BigBird, Mistral Rolling-Buffer-KV); **Late Chunking** (Günther et al., Jina 2024, arXiv 2409.04701: ganzes Dokument im Long-Context-Encoder kodieren, danach Mean-Pooling pro Chunk).
      - **Praktisch mit BERT/e5-small (max 512):** Fenster = `max_seq_len − Spezialtokens`, Stride 25–50 % Überlappung. Pro Fenster ein Embedding; Aggregation Mean/Max **oder** getrennte Vektoren pro Fenster mit Dokument-ID (Retrieval-üblich, Score = max über Fenster). Präfixe `query: `/`passage: ` je Fenster. `tokenizers` unterstützt `truncation` mit `stride` + `return_overflowing_tokens` (`encoding.get_overflowing()`).
      - **GPU-Technik:** Bei bidirektionalen Encodern ändert eine Fensterverschiebung die Attention aller Tokens → **keine etablierte Inkrementtechnik**. Wiederverwendbar ist nur Batching der Fenster und Token-Embedding-/Tokenisierungs-Wiederverwendung. (KV-Cache-Reuse nur bei kausalen Modellen.)

      ## 3. Zu sichernde Erweiterungspunkte (Checkliste — darf jetzt nicht verbaut werden)

      1. **Embedding-Parameterobjekt ist erweiterbar für Fenster/Stride.** Das Params-Objekt (`resident.md`: „soll zukünftig weitere Einstellungen erlauben") muss ein späteres `sliding { window, stride, aggregate }`-Feld aufnehmen, ohne Bruch → `#[non_exhaustive]` oder Builder; der Query/Passage-Enum bleibt orthogonal.
      2. **Planner-Regel-Registry nimmt eine Sliding-Variante auf.** Die Expansionsregel-Registry (C3, `resident_planner.md`: Registry-Eintrag pro Modell/Tasktyp) muss pro Tasktyp alternative Expansionen tragen können: statt `Tokenize → Embed → Pool+L2 → WriteIndex` eine Variante `Tokenize(Overflow+Stride) → Embed(sliding) → PoolPerWindow → WriteIndex(many)`. Kein hartkodiertes 1:1 Task→Pipeline.
      3. **Entry-Modell trägt mehrere Vektoren pro Entry mit Fenster-Metadaten.** `resident_index.md`: Entry = Vektorbereich variabler Länge (Dense=1, Datei/Gleitfenster=viele). `resident_subengine.md` §4.4: Slot-Klassen (32/64/128 Tokens), optionales Feld pro Token. Sicherstellen, dass Entry↔Segment/Slot-Mapping (E/D7) „viele Fenster-Vektoren, eine Dokument-ID" abbilden kann und `KernelHit → EntryRef` die Dokument-Aggregation (max über Fenster) in der Oberschicht nicht verhindert.
      4. **Tokenizer-Wrapper kann Overflow/Stride exponieren.** Der Tokenizer (D5, `resident_first.md`) ist CPU-Ressource; sein Wrapper darf `truncation`+`stride`+`return_overflowing_tokens` später aktivieren, ohne die `u32`-IDs+Attention-Masken-Ausgabe zu ändern. Keine Annahme „genau ein Encoding pro Eingabe".
      5. **Op-Austausch hinter demselben Trait.** Der gleitende Embedding-Kernel ist eine weitere `Op`/Encoder-Implementierung hinter `encode_into`/demselben Op; der Scheduler bleibt unberührt.

      ## 4. Jetzt NICHT umsetzen

      Keine Fenster-Erzeugung, kein gleitender Kernel, keine Pro-Fenster-Pooling-Logik, keine Late-Chunking-Pipeline. Nur Datenmodell- und Parameter-Offenheit absichern.

      ## 5. Offene Verifikationen ([prüfen])

      - `tokenizers`-Overflow-API (`get_overflowing`, `stride`) in der Projektversion.
      - Aggregationsstrategie (Mean/Max vs. getrennte Vektoren) — Entscheidung liegt bei der Oberschicht; Engine muss beide zulassen.

      ## 6. Definition of Done

      - Erweiterungspunkte 1–5 sind in B-/C-/D-Paketen als Anforderung vermerkt bzw. als offene Abstraktion vorhanden.
      - Ein Test belegt, dass ein Entry mit N>1 Vektoren und gemeinsamer Dokument-ID durch das Mapping läuft (Mock-Index), und dass die Params um ein Feld erweitert werden können, ohne bestehende Call-Sites zu brechen.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h3_funnel_plaid_reranker.md
    mode: replace
    content: |
      # Umsetzungsplan H3 · Funnel/PLAID & Cross-Encoder/Reranker

      **Gruppe H — Ausblick/zurückgestellt. Status I/A** (Thesen-§7: über Addons + Centroid-ScopeResolver realisierbar). `resident_first.md`: Cross-Encoder „für die aktuelle Umsetzung nicht vorgesehen".

      **Planungsauftrag:** NUR Erweiterungspunkte sichern. Weder PLAID-Funnel noch Reranker-Forward werden jetzt gebaut.

      ## 1. Was später kommt (Kurzriss, nicht umsetzen)

      - **Funnel/Cascade:** Stufe 1 billiger Filter auf Centroid-Codes → Stufe 2 exaktes MaxSim auf der Spitze (`resident_subengine.md` §4.5, §11). Als **Addon** über die normale Queue, nicht als Kernfunktion (`resident_index.md` Addons).
      - **Cross-Encoder/Reranker:** eigener **Inferenz**-Tasktyp bzw. Similarity mit anderem Scorer auf einem Subset; Modell lazy geladen, TTL entlädt (`resident_first.md`, `resident_subengine.md` §11).

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H3)

      - **PLAID** (Santhanam et al., CIKM 2022, arXiv 2205.09707; Repo `stanford-futuredata/ColBERT`): (1) Kandidaten aus `nprobe` nächsten Zentroiden je Query-Token, (2) Centroid-Pruning `t_cs`, (3) Centroid-Interaction (ohne Residual-Dekompression, optional 2. Stufe `ndocs`), (4) Residual-Dekompression + volles MaxSim auf Top-Kandidaten. `next-plaid` unbelegt **[prüfen]**.
      - **ColBERTv2-Kompression** (arXiv 2112.01488): K-Means → Zentroidtabelle (~`2^⌊log2(16·√N)⌋`), pro Token Zentroid-Index (Code) + Residual (Embedding−Zentroid) auf 1–2 Bit quantisiert (`nbits`).
      - **Cross-Encoder `BAAI/bge-reranker-v2-m3`:** XLM-RoBERTa-Backbone (bge-m3, ~568 M), Eingabe `[CLS] query [SEP] doc [SEP]`, ein Logit. fp32 ~2,3 GB / fp16 ~1,1 GB (passt in 6 GB, aber nicht parallel zu großen Indizes). `candle-transformers` enthält XLM-RoBERTa inkl. Sequence-Classification **[prüfen]**; `model.safetensors` im HF-Repo **[prüfen]**.
      - **Cascade-Steuerparameter:** `nprobe`, `t_cs`, `ndocs`, Top-k je Stufe, `nbits`; Stufentiefe = Recall/Latenz-Tradeoff, pro Query-Klasse konfigurierbar.

      ## 3. Zu sichernde Erweiterungspunkte (Checkliste — darf jetzt nicht verbaut werden)

      1. **Centroid-ScopeResolver ist pluggable (Hook vorhanden).** `Scope::Centroid { k, radius }` und `AccessMethod::Centroid` existieren bereits; `Index::summary(Centroid)` liefert die Zentroidtabelle, `Index::members(cell)` die Inverted List (`resident_index.md`). Sicherstellen: E2 registriert Resolver pro `AccessMethod`, Kompatibilitätsprüfung „Verfahren deklariert UND Resolver vorhanden", sonst `UnsupportedScope` (kein stilles `All`).
      2. **Optionales Centroid-Code-Feld pro Token im Slot.** `resident_subengine.md` §4.4: „Optionales Feld pro Token: Centroid-ID/Code (für späteren Funnel, vom Kernel ignoriert, wenn nicht genutzt)." Slot-/Segment-Layout muss dieses Feld reservieren können, ohne den aktuellen Kernel zu ändern.
      3. **Scorer-Parameter ist erweiterbar.** `resident_first.md`: Scorer = Parameter `Cosine | CrossEncoder(model)`. Enum `#[non_exhaustive]`; Rerank ist **kein eigener Tasktyp**, sondern Similarity mit anderem Scorer auf einem Subset. Entscheidungsgrundlagen (Anzahl, Score-Abstand Top-1..Top-k, Streuung, Task-ID) werden mitgeliefert, damit die Oberschicht einen Folgetask bilden kann.
      4. **Inferenz-Tasktyp + Lazy/TTL-Modellplatz.** Der Inferenz-Tasktyp der Fassade (`resident.md`) und der Ressourcen-Lifecycle `Unloaded→…→Idle→Evicting` mit TTL (`resident_subengine.md` §6) müssen ein zusätzliches Reranker-Modell (eigene Gewichte, Forward pro Paar, `resident_subengine.md` §11) lazy aufnehmen können, ohne Fassaden-/Task-Schema zu ändern.
      5. **Addon-/Cascade-Pfad über die normale Queue.** `resident_index.md` Addons: Iteration/Kaskade/Rekursion sind **keine** Kernfunktion; ein Addon reiht Folgetasks mit kopierten/geänderten Parametern ein (z. B. Centroid-Task → Folgetask mit Zell-Mitgliedern als Scope). Nur das erweiterbare Parameterobjekt sichern; keine Eltern-Kind-Beziehung im Kern.
      6. **Cascade-Parameter im Parameterobjekt vorsehen.** `nprobe`/`t_cs`/`ndocs`/`nbits`/Top-k-je-Stufe als spätere Felder des erweiterbaren Params — nicht jetzt definieren, nur nicht ausschließen.

      ## 4. Jetzt NICHT umsetzen

      Kein K-Means/Zentroid-Training, keine Residual-Quantisierung, kein Funnel-Kernel, kein Cross-Encoder-Forward, kein Laden von bge-reranker. Der Standardpfad bleibt dense e5-small + Cosine/Top-k (`resident_first.md`).

      ## 5. Offene Verifikationen ([prüfen])

      - `candle-transformers` XLM-RoBERTa-Classifier verfügbar + Gewichtsnamen.
      - `next-plaid`-Repo/Lizenz/Status.
      - VRAM-Budget: Reranker (fp16 ~1,1 GB) vs. residenter Index gleichzeitig.

      ## 6. Definition of Done

      - Erweiterungspunkte 1–6 sind in E2 (ScopeResolver/Centroid), D7/§4.4 (Slot-Feld), C1/C3 (Scorer-Enum, Inferenz-Tasktyp), E9 (Addon) als Anforderung/offene Abstraktion vorhanden.
      - Ein No-GPU-Test belegt: unbekannter/unregistrierter Centroid-Resolver liefert `UnsupportedScope`; Scorer-Enum und Params lassen sich um eine Variante/ein Feld erweitern, ohne bestehende Pfade zu brechen.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h4_graph_capture.md
    mode: replace
    content: |
      # Umsetzungsplan H4 · Graph-Capture & Persistent-Scorer

      **Gruppe H — Ausblick/zurückgestellt. Status A** (Thesen-§3/§9; Ausbaupfad `resident_subengine.md` §9, Phase 4). Persistent-Scorer überlappt H1.

      **Planungsauftrag:** NUR Erweiterungspunkte sichern. Kein Graph-Capture, kein Persistent-Scorer wird jetzt gebaut.

      ## 1. Was später kommt (Kurzriss, nicht umsetzen)

      - **Graph-Decorator** auf dem Launcher: Modus `Direct` oder `Capture`, Cache-Schlüssel `(Op, Shape-Bucket, Regions)`, hält Handles (keine Besitzrechte), verwirft bei `gen`-Änderung; Ein-/Ausgaben in festen Ring-Slots des Managers (`resident_subengine.md` §4.6).
      - **Persistent-Scorer** mit Ring (siehe H1). „Sinnvoll für den Scorer, nicht für den Encoder."

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H4)

      - **Ablauf:** `cudaStreamBeginCapture(stream, mode)` → Ops im Stream → `cudaStreamEndCapture` → `cudaGraphInstantiate` → `cudaGraphLaunch` (Replay). Modi `Global`/`ThreadLocal`/`Relaxed`.
      - **Während Capture verboten/problematisch:** synchrone Aufrufe (`cudaStreamSynchronize`, `cudaDeviceSynchronize`, `cudaMemcpy` ohne Async), Legacy-Default-Stream, klassisches `cudaMalloc` (Global-Modus), Event-Queries. **Erlaubt:** Kernel, `cudaMemcpyAsync`, Events (Record/Wait), `cudaMallocAsync`.
      - **Capture über candle-Ops** nur, wenn keine Allokation/Sync im Capture ausgelöst wird; candle allokiert über cudarc — nur `cuMemAllocAsync` ist capture-fähig **[prüfen]**. Kein offizielles candle-Feature bekannt **[prüfen GitHub-Issues]**. Empfehlung: Warm-up-Lauf vor Capture, feste Buffer, Capture nur für eigene Kernel-Folgen.
      - **Cache-Schlüssel** = (Shape-Bucket, dtype, Kernel-Variante, Gerät/Stream). Feste Ein-/Ausgabe-Regionen; bei Pointer-Änderung (`gen`) Instanz invalidieren oder per `cudaGraphExecKernelNodeSetParams`/`cudaGraphExecUpdate` aktualisieren. Shapes in wenige Buckets (Zweierpotenzen) quantisieren (jede Instanz kostet VRAM).
      - **`cudarc`** hat Graph-Support auf Raw-API-Ebene (`result::graph`/`sys`) und ab neueren Versionen ein sicheres Wrapper-API **[prüfen Version/Beispiele]**.

      ## 3. Zu sichernde Erweiterungspunkte (Checkliste — darf jetzt nicht verbaut werden)

      1. **Launcher ist ein Trait mit Decorator-Fähigkeit.** Jeder Op-Launch geht über eine Launcher-Abstraktion, auf die ein Decorator mit Modus `Direct | Capture` (später `PersistentRing`) gelegt werden kann. Der Scheduler kennt keine Graphs/Kernel (`resident_subengine.md` §2, Invarianten).
      2. **Ein-/Ausgaben in festen Regionen / Ring-Slots.** Die Op-IO-Pfade müssen feste, vom Manager verwaltete Regionen nutzen können (keine impliziten Allokationen im Hot-Path). Das ist Voraussetzung für Capture **und** für den stream-geordneten Arena-Betrieb (D3/E3: `cudaMalloc/Free` im Betrieb vermeiden).
      3. **`gen`-basierte Invalidierung ist vorhanden.** `Region{...,gen}` entwertet Handles nach Evict/Reload (`resident_subengine.md` §4.2). Sicherstellen, dass eine spätere Graph-Instanz an `gen` gebunden/invalidierbar ist (Cache-Key enthält Region-Identität + `gen`).
      4. **Cache-Key-Struktur als offener Platzhalter.** Dort, wo Launches identifiziert werden, eine Key-Struktur `(Op, Shape-Bucket, Regions/gen, dtype, kernel-variant)` vorsehen (Shape-Bucket-Quantisierung als Hook), ohne jetzt einen Graph-Cache zu bauen.
      5. **Warm-up-Hook.** Der Lane-/Ressourcen-Lifecycle darf einen Warm-up-Lauf vor einem späteren Capture erlauben (keine Annahme „erster Launch = produktiver Launch").

      ## 4. Jetzt NICHT umsetzen

      Kein `cudaStreamBeginCapture`/`cudaGraphInstantiate`-Code, kein Graph-Cache, kein Persistent-Scorer, keine `cudaGraphExecUpdate`-Pfade.

      ## 5. Offene Verifikationen ([prüfen])

      - candle-Capture über eigenen Stream ohne Alloc/Sync (Issue-Recherche `huggingface/candle`).
      - `cudarc` Graph-Wrapper-API der Projektversion.
      - `cuMemAllocAsync`-Pfad in candle/cudarc (capture-fähig).

      ## 6. Definition of Done

      - Launcher-Trait + feste-Region-IO + `gen`-Invalidierung + Cache-Key-Platzhalter + Warm-up-Hook sind als Anforderung/offene Abstraktion in §4.6-nahen Paketen (D2/D6/E4) vorhanden.
      - Ein No-GPU-Test belegt, dass ein Launcher-Decorator (Mock `Direct` vs. Mock-`Capture`) hinter demselben Trait austauschbar ist, ohne Scheduler-/Op-Änderung.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h5_signalkompression.md
    mode: replace
    content: |
      # Umsetzungsplan H5 · Signalkompression (FFT/Wavelet, „Semantic Spectrogram")

      **Gruppe H — Status E/—** (Thesen-§6/§6.1). **Ausdrücklich NICHT Engine-Scope.**

      **Planungsauftrag:** Keine Engine-Arbeit. Es ist lediglich zu prüfen, dass `AccessMethod`/`Scope`/`summary()` eine spätere „Frequenz-/Kompressionsdomäne" **als Vorfilter der Oberschicht nicht ausschließen**. Divergenz zur These dokumentieren.

      ## 1. Einordnung / Divergenz

      - Die These skizziert Frequenz-/Signalkompression (JPEG/DCT, Semantic Spectrogram, Wavelet) als Kompressions-/Qualitätsschicht. Der Plan zieht diese **bewusst aus der Engine heraus**: Die Engine ist Rechensubstrat + Index-Substrat + MaxSim-Geometrie, **kein** Retrieval-/Kompressionsalgorithmus.
      - `resident_index.md` (Leitprinzipien): „Approximative Vorfilter (Zentroid, FFT, Kompression) verlieren potenziell Randtreffer. Das ist Eigenschaft der Oberschicht und nicht Teil der Engine."
      - `resident_precision.md`: Qualität/akkumulierende Genauigkeitsverluste sind Sache der äußeren Schichten.

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H5)

      - **Etabliert sind NICHT DCT/FFT**, sondern: Matryoshka Representation Learning (Kusupati et al., NeurIPS 2022), Product Quantization (Jégou et al., 2011), Binary/Scalar Quantization, PCA. DCT/Wavelet eher für Bild-/Zeitreihen-Hashing.
      - Zu „Semantic Spectrogram" für semantische Suche **keine etablierten Referenzen** bekannt **[prüfen Literatursuche]**.
      - **Erweiterbarkeit:** Ein Vorfilter ist ein Proxy-Score auf einer transformierten Repräsentation (nur Rangfolge-Treue nötig) + exakter Rerank. Passt als zusätzliche `Scope`/`AccessMethod`, solange die **Oberschicht** eine Methoden-Abstraktion hat; der Recall der Vorfilterstufe muss messbar sein.

      ## 3. Zu sichernde Nicht-Ausschlüsse (Checkliste)

      1. **`AccessMethod` ist offen/erweiterbar.** `Index::capabilities() -> &[AccessMethod]`; das Enum als `#[non_exhaustive]`, damit eine spätere `AccessMethod::CompressionDomain`/`Frequency` (durch die Oberschicht bereitgestellt) ergänzbar ist, ohne Kern-Bruch.
      2. **`Scope` ist erweiterbar.** `enum Scope { All, Range, Set, Subtree, Prefix, Centroid, … }` — als `#[non_exhaustive]`; eine Kandidatenmenge aus einem Kompressions-Vorfilter wird ohnehin als kompakte Entry-Menge (Range/Bitmap/Set) ausgedrückt und ist damit bereits abgedeckt.
      3. **`summary()` bleibt generisch.** `Index::summary(kind) -> Option<SummaryRef>` liefert beliebige Summary-Strukturen (Zentroidtabelle, Präfixtabelle, …) → eine „Frequenz-/Kompressions-Summary" ist formal zulässig, ohne dass der Kern Geometrie interpretiert.
      4. **ScopeResolver-Registry bleibt pluggable** (E2), sodass ein späterer Kompressions-Resolver (falls je in der Engine gewünscht) als weiterer Eintrag registrierbar wäre — ohne stilles Ausweichen (`UnsupportedScope`).

      ## 4. Jetzt NICHT umsetzen (und auch nicht als Engine-Workstream planen)

      Keine FFT/DCT/Wavelet-Transformationen, keine Quantisierung, kein Spectrogram, kein Vorfilter in der Engine. Diese gehören — falls überhaupt — in die Suchschicht **vor** der Resident-Engine.

      ## 5. Definition of Done

      - `AccessMethod` und `Scope` sind `#[non_exhaustive]` (B4); `summary()` ist generisch; ScopeResolver-Registry ist pluggable (E2).
      - Ein ADR-Eintrag dokumentiert die Divergenz: „Signal-/Frequenzkompression ist Oberschicht-Vorfilter, nicht Engine. Engine stellt nur generische Scope-/Summary-Hooks bereit."
      - Kein weiterer Code-Workstream.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_h6_mutable_weights_bandit.md
    mode: replace
    content: |
      # Umsetzungsplan H6 · Mutable Weights / Online-Bandit

      **Gruppe H — Status —** (Thesen-§9/§9.1). **Ausdrücklich NICHT Engine-Scope.**

      **Planungsauftrag:** Keine Engine-Arbeit. Divergenz zur These dokumentieren und sicherstellen, dass ein späterer eigener Workstream nicht verbaut wird („nicht verbauen").

      ## 1. Einordnung / Divergenz

      - These §9/§9.1: Parameter-/Aktivierungs-Dualität, **mutable weights**, Online-Adaption per Contextual Bandit als Meta-Router.
      - Plan-Divergenz (ausdrücklich festhalten): **Gewichte sind immutabel** (Weights-Arena langlebig, nur bei Entladung frei, `resident_subengine.md` §4.2/§4.3). **Nur der Index** ist bidirektional mutabel (`resident.md`, `resident_index.md`). Das ist die spekulativste Thesen-Idee und bewusst nicht adressiert.

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H6)

      - **Contextual Bandits als Meta-Router:** Arme = Retrieval-/Cascade-Strategien, Kontext = Query-Features, Reward = Klick/Zufriedenheit/latenz-adjustierte Qualität. Verfahren: LinUCB (Li et al., WWW 2010), Thompson Sampling, ε-greedy; Toolkit Vowpal Wabbit. Off-Policy-Evaluation (IPS/Doubly Robust) vor Ausrollen.
      - **Online-Adaption von Gewichten:** Online-Fine-Tuning, LoRA-Adapter-Updates, OGD auf Heads, kontinuierliches Lernen. Risiken: katastrophales Vergessen, Reward-Hacking, Feedback-Loops/Bias, Nicht-Reproduzierbarkeit. Praktikabel nur für kleine Teile (Router, Score-Gewichte), **nicht** für das Basismodell.
      - **Reward-Stream-Pipeline:** Inferenz loggt (Kontext, Aktion, Propensity) → Reward-Events über Join-Key → Lernkomponente (getrennter Prozess) → versionierter Snapshot → **atomarer Swap** im Serving. Inferenz und Update strikt getrennt, Rollback über Versionierung.

      ## 3. Zu sichernde Nicht-Ausschlüsse (Checkliste — „nicht verbauen")

      1. **Gewichts-Reload/Swap-Pfad nicht ausschließen.** Gewichte sind immutabel, aber die Ressource ist **reload-/swap-fähig** über den Lifecycle `Unloaded→Loading→Resident→…` und `Region.gen`-Entwertung (`resident_subengine.md` §4.2/§6). Ein späterer versionierter Snapshot-Swap (neue Gewichte als neue Region, Pointer-Swap wie bei Index-CoW) wäre damit prinzipiell möglich — diese Reload-Fähigkeit nicht hartkodiert deaktivieren (kein `const`/statisch eingebrannte Gewichte).
      2. **Parameterobjekt erweiterbar für spätere Logging-Felder.** Das Task-Params-Objekt (`#[non_exhaustive]`/Builder) darf später (Kontext, Aktion, Propensity) als Diagnose-/Log-Felder aufnehmen, ohne Bruch — jetzt nur nicht ausschließen.
      3. **Addon-Pfad trägt einen Meta-Router.** Ein Bandit-Meta-Router ist konzeptionell ein **Addon** (`resident_index.md` Addons), das Folgetasks unterschiedlich parametrisiert über die normale Queue einreiht — keine Kernänderung nötig. Sicherstellen, dass der Addon-/erweiterbare-Params-Pfad das zulässt.

      ## 4. Jetzt NICHT umsetzen (und auch nicht als Engine-Workstream planen)

      Kein Bandit, keine Reward-Queue, keine mutable Weights-Arena, kein Online-Lernen, keine Propensity-Logging-Implementierung. Falls je gewünscht: **eigener, getrennter Workstream** (zusätzliche Reward-Queue, mutable Weights-Arena, separater Lernprozess, versionierter Swap) — außerhalb dieser Engine-Phase.

      ## 5. Definition of Done

      - Ein ADR-Eintrag dokumentiert die Divergenz: „Gewichte immutabel, nur Index bidirektional mutabel; Bandit/Online-Adaption bewusst nicht adressiert."
      - Gewichts-Ressource ist reload-/swap-fähig (nicht statisch eingebrannt); Params sind erweiterbar; Addon-Pfad existiert.
      - Kein weiterer Code-Workstream.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q1_fehler_taxonomie.md
    mode: replace
    content: |
      # Umsetzungsplan Q1 · Fehler-Taxonomie (`thiserror`)

      **Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren, je Gruppe fortzuschreiben. Speist alle Gruppen (A–H).

      ## 1. Zweck

      Einheitliche, typisierte Fehlerbehandlung über alle Pakete. Konsolidiert die im Metaplan/den Quellen verstreuten Fehlerarten zu einer geschichteten Taxonomie mit stabiler öffentlicher API.

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q1)

      - **`thiserror` 2.x** **[prüfen Version]**. Muster: `#[derive(Error, Debug)]`, `#[error("…{field}")]` (Display), `#[from]` (auto-`From` + Source), `#[source]` (Ursachenkette ohne `From`), `#[error(transparent)]` (Durchreichen). Varianten mit **strukturierten Feldern**, keine Strings. `#[non_exhaustive]` auf öffentlichen Enums → spätere Varianten ohne Breaking Change.
      - **Geschichtet:** ein Enum **pro Komponente/Modul** (klein, präzise); ein Top-Level-Enum aggregiert per `#[from]`. Kein zentrales Mega-Enum (koppelt alle Module).
      - **Library vs. App:** Engine (öffentliche API) → `thiserror`, typisiert, matchbar. Anwendung/Binary/Tests → `anyhow` (`.context()`). **Die Engine gibt nie `anyhow::Error` in der öffentlichen API zurück.**
      - **Intern vs. extern:** öffentlicher Fehler als opaker Struct mit privatem `ErrorKind`-Enum (wie `std::io::Error`) + `kind()`-Accessor auf ein `#[non_exhaustive]` öffentliches Kind. Interne Varianten (z. B. `StaleMapping`) in `pub(crate)`-Enum, an der API-Grenze per `From`/`map_err` abgebildet oder intern per Retry behandelt.

      ## 3. Verbindliche Konvention (Umsetzung)

      **Pro-Komponenten-Enums** (`#[derive(Error, Debug)]`, `#[non_exhaustive]`), zugeordnet aus den Quellen:

      | Komponente / Modul | Enum | Varianten (aus Quellen) | Quelle |
      |---|---|---|---|
      | Index/Manager | `IndexError` | `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `TransferFailed` | `resident_index.md` (Fehler) |
      | Index/Manager (intern) | `ManagerError` *(pub(crate))* | `StaleMapping` | `resident_index.md` („nur intern") |
      | Präzision/Kalibrierung | `PrecisionError` | `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision`(NaN) | `resident_precision.md` |
      | Planner | `PlannerError` | `InvalidConfig`, `MissingCapability`, `UnknownModel` | `resident_planner.md` |
      | Config/Fassade | `ConfigError`, `CapabilityError` | ungültige Config, fehlende/unbekannte Capability | Metaplan A4/B6, `resident.md` |

      **Top-Level-Aggregat** `ResidentError` (`#[non_exhaustive]`), das die Komponenten-Enums per `#[from]` aggregiert:
      ```rust
      #[derive(Error, Debug)]
      #[non_exhaustive]
      pub enum ResidentError {
          #[error(transparent)] Index(#[from] IndexError),
          #[error(transparent)] Precision(#[from] PrecisionError),
          #[error(transparent)] Planner(#[from] PlannerError),
          #[error(transparent)] Config(#[from] ConfigError),
          #[error(transparent)] Capability(#[from] CapabilityError),
          // Transfer/IO/… nach Bedarf
      }
      ```

      **Regeln:**
      - Öffentliche API (Fassade, `Index`-Trait, Planner, Calibrator) gibt `Result<_, ResidentError>` bzw. das jeweilige Komponenten-Enum zurück — **nie** `anyhow`.
      - `StaleMapping` bleibt `pub(crate)` und wird intern per Retry (Kommunikationsslot) behandelt oder an der Grenze auf eine öffentliche Variante gemappt; es erscheint **nicht** nach außen.
      - Task-Zustände (`Queued|Taken|Done|Cancelled`) sind **keine** Fehler; `Cancelled` ist lokal abgeleitet (`resident.md` Queue-Logik). Fehler eines Batchs werden als `Err` in die betroffenen Task-Results geschrieben (`resident_processor.md`).
      - GPU-Initfehler → Fallback auf CPU-Engine, GPU als nicht verfügbar markiert (wiederholbar), als Fehlerzustand `Failed` der Ressource, nicht als API-Fehler des Nutzers (`resident_subengine.md` §6).
      - Fehlervarianten mit **strukturierten Feldern** (z. B. `UnsupportedScope { method: AccessMethod }`, `ArenaExhausted { requested, available }`), keine Strings.
      - Optional (falls API-Stabilität über interne Refactorings gefordert): die **Fassade** exponiert einen opaken `Error` + `kind() -> &ErrorKind` (`#[non_exhaustive]`), während intern die Komponenten-Enums genutzt werden.

      ## 4. Umsetzungsschritte

      1. `error`-Modul je Komponente anlegen; Komponenten-Enums mit `#[non_exhaustive]` + strukturierten Feldern.
      2. `ResidentError` als Aggregat mit `#[from]`/`transparent`.
      3. `pub(crate)`-interne Enums (`ManagerError`) + Mapping an der API-Grenze.
      4. Doku: Skala/Semantik dokumentieren (z. B. `InvalidPrecision` = NaN; `precision`-Klemmen ist kein Fehler, `resident_precision.md`).
      5. `anyhow` nur in Binärs/Tests als dev-/bin-dependency, nicht in der Lib-API.

      ## 5. Tests

      - `#[from]`-Konvertierungen: Komponenten-Enum → `ResidentError`.
      - `UnsupportedScope` entsteht bei nicht deklariertem Verfahren/fehlendem Resolver (Zusammenspiel mit E2); kein stilles `All`.
      - `InvalidPrecision` bei NaN; Klemmen `[0,1]` erzeugt **keinen** Fehler (F4).
      - Interne `StaleMapping` erscheint nicht in der öffentlichen Signatur (Kompilier-/Sichtbarkeitstest + API-Grenze-Mapping).
      - `#[non_exhaustive]`: Hinzufügen einer Variante bricht kein `match` in Tests mit `_`-Arm.

      ## 6. Offene Verifikationen ([prüfen])

      - `thiserror`-Version (2.x) in der Projekt-`Cargo.toml`.

      ## 7. Definition of Done

      - Komponenten-Enums + `ResidentError` existieren, alle gelisteten Varianten zugeordnet, `#[non_exhaustive]`, strukturierte Felder, keine `anyhow`-Leaks in der Lib-API, interne Varianten gekapselt; Tests aus §5 grün.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q2_logging.md
    mode: replace
    content: |
      # Umsetzungsplan Q2 · Logging (`tracing`)

      **Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren, je Gruppe fortzuschreiben.

      ## 1. Zweck

      Einheitliches strukturiertes Logging/Tracing: Spans pro Task-ID, Ressource und Manager-Zyklus; stabile Feldnamen; korrekte Propagation über async-/Thread-/CUDA-Callback-Grenzen; Initialisierung nur im Binary.

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q2)

      - **Spans/Felder:** `#[tracing::instrument(skip_all, fields(task_id = %id))]` auf Funktionen; `skip_all` verhindert Loggen großer Argumente, Felder explizit. Span pro Task/Ressource/Manager-Zyklus, **stabile Feldnamen**. Strukturierte Events `info!(bytes = n, "msg")`. Hochfrequente Pfade auf `trace`/`debug`.
      - **Propagation:** Futures `fut.instrument(span)`; gespawnte Tasks `tokio::spawn(fut.instrument(Span::current()))`; synchron `span.in_scope(|| …)`; **`Entered`-Guard nie über `.await` halten**. `spawn_blocking`/Threads übernehmen den Span nicht automatisch → `let span = Span::current(); spawn_blocking(move || span.in_scope(|| …))`.
      - **Subscriber:** `tracing-subscriber` mit `EnvFilter` (z. B. `RUST_LOG=engine=debug,engine::gpu=trace`), `fmt()` menschenlesbar, `.json()` (Feature `json`) für Produktion; `tracing-appender::non_blocking` (Guard am Leben halten). **Initialisierung im Binary, nie in der Library.**
      - **Nicht-tokio-Threads / CUDA-Callbacks:** `tracing` ist threadsicher bei globalem Subscriber; Span explizit übergeben. **CUDA-Host-Callbacks (`cuLaunchHostFunc`): keine CUDA-API darin, nur kurze Arbeit**; zum Loggen Daten per Channel/Atomic an einen Logging-Thread geben.

      ## 3. Verbindliche Konvention (Umsetzung)

      **Stabile Feldnamen (Span-Felder):** `task_id`, `resource_kind`, `model_id`, `index_id`, `segment_gen`, `slot`, `lane`, `manager_cycle`, `op`, `precision`, `profile`. Immer dieselben Namen, damit Filter/JSON-Pipelines stabil sind.

      **Span-Topologie:**
      - **Task-Span** pro Task-ID (Fassade/Planner/Processor; `resident_planner.md`/`resident_processor.md`: „Span pro Task-ID").
      - **Ressourcen-Span** pro geleaster Ressource (`resident_subengine.md` §6: „Span pro Task-ID und Ressource").
      - **Manager-Zyklus-Span** pro Kommunikationsslot/Scan-Zyklus (`resident_index.md`: „Spans pro Task und pro Manager-Zyklus").

      **Ereignis-Katalog (was pro Gruppe geloggt wird):**
      - **Queue/Processor (B1/C2):** Claim/Complete/Cancel, Batch-Bildung, Starved-Hint-Erzeugung/-Verfall, GPU-Completion (Callback/`cuEventQuery`) — hochfrequent → `trace`.
      - **Subengine/Lifecycle (D):** Lazy-Init-Start/-Ende, Lease-Zähler, Eviction-Entscheidung, `Failed`/Backoff, GPU-Init-Fallback auf CPU.
      - **Index/Manager (E):** Scan-Ergebnis (geladen/veraltet/entfernt), Lade-/Eviction-Entscheidungen, verworfene und `outdated`-Ergebnisse, Arena-Auslastung (`resident_index.md`).
      - **Präzision (F):** Kalibrierungsergebnis pro Kombination, verworfene Kombinationen mit Grund, abgeleitete Profile (Anzahl/Positionen), gewähltes Profil pro Task auf `debug` (`resident_precision.md`).

      **Propagations-Regeln (verbindlich):**
      - Worker-Pool/`spawn_blocking` (CPU-Kernel, Kalibrierung, blockierende Index-Reads): Span per `Span::current()` einfangen und `in_scope`. Kalibrierung/CPU-Kernel laufen im eigenen Thread-Pool (`resident_precision.md`).
      - GPU-Completion-Callback: **kein** CUDA-/Log-schweres Tun; nur Completion-Eintrag + Notify (`resident_processor.md`). Logging erst im aufräumenden Worker (Schritt 1 der Worker-Loop) mit dem dort übergebenen Span.
      - Kein `Entered`-Guard über `.await`.

      **Bibliotheks-Regel:** Die Engine-Crate setzt **keinen** globalen Subscriber und keine Appender; das macht ausschließlich das Binary/der Daemon/CLI (`resident.md`: Server/Daemon/CLI).

      ## 4. Umsetzungsschritte

      1. `#[tracing::instrument(skip_all, fields(...))]`-Konvention mit den stabilen Feldnamen etablieren (Helper/Doku).
      2. Span-Erzeugung an den drei Topologie-Punkten (Task/Ressource/Manager-Zyklus).
      3. Propagations-Helfer für `spawn_blocking`/Threads (`Span::current()` + `in_scope`).
      4. Completion-Callback: Channel/Atomic → Logging im Worker.
      5. Binary-Setup-Beispiel (EnvFilter + fmt/json + non_blocking) separat vom Lib-Code.
      6. Hochfrequente Ereignisse auf `trace`/`debug`.

      ## 5. Tests

      - `tracing-test` o. Ä.: Span-Felder vorhanden und korrekt benannt bei Task-/Manager-Pfaden.
      - `spawn_blocking`-Pfad trägt die Task-ID (Span-Propagation).
      - Lib initialisiert keinen globalen Subscriber (API-/Sichtbarkeits-Check).

      ## 6. Offene Verifikationen ([prüfen])

      - `tracing`/`tracing-subscriber`/`tracing-appender`-Versionen und `json`-Feature in der Projekt-`Cargo.toml`.

      ## 7. Definition of Done

      - Span-Topologie + stabile Feldnamen + Ereignis-Katalog + Propagations-Regeln sind umgesetzt und dokumentiert; Lib setzt keinen Subscriber; Completion-Callback loggt nicht selbst; Tests aus §5 grün.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q3_teststrategie.md
    mode: replace
    content: |
      # Umsetzungsplan Q3 · Teststrategie & Mocks

      **Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren, je Gruppe fortzuschreiben.

      ## 1. Zweck

      Einheitliche Test-/Mock-Strategie, die (a) die gesamte Logik **ohne GPU** testbar macht, (b) Nebenläufigkeit/CAS-Protokolle abdeckt, (c) deterministische Sync-/Kalibrierungs-Tests ermöglicht, und (d) je Paket eine Testmatrix vorgibt.

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q3)

      - **Mocks:** `mockall` (`#[automock]` auf Traits) für Erwartungs-/Aufruf-Verifikation; für **zustandsbehaftete** Kernel-/Mess-Mocks sind **manuelle Fakes** klarer (`FakeKernel` mit programmierbaren Ergebnissen, `FakeMeasure` mit Sequenz/RNG).
      - **Nebenläufigkeit:** `loom` (erschöpfende Interleavings unter C11, `loom::sync` via `cfg(loom)`) für kleine CAS-Protokolle (Append/Claim/Cancel/Unlink); `shuttle` (randomisierte Schedules) für größere Szenarien; ergänzend `proptest` für Sequenzen.
      - **GPU-frei:** Backend-Trait (`ComputeBackend`) mit CPU-/Mock-Impl; Feature `cuda` aktiviert nur die CUDA-Impl; Tests laufen **standardmäßig ohne** GPU.
      - **Deterministisch:** virtuelle Zeit (`tokio::time::pause`, Feature `test-util`) oder eigener `Clock`-Trait; Schritt-Hooks/Failpoints zum Erzwingen von Interleavings (ABA: Stamp/Generation gezielt zwischen Lesen und CAS ändern); zustandsbasierte Asserts statt Timing; feste Seeds.
      - **Verrauschte Messwerte:** Mess-Mock hinter Trait: Basiswert + Rauschen aus seeded RNG (`rand`, `StdRng::seed_from_u64`); Varianten konstant/Ausreißer/Drift. Tests: Abbruch an Obergrenze → `unstable`, Konvergenz bei geringem Rauschen, Verhalten bei Ausreißern; mehrere Seeds.

      ## 3. Verbindliche Konvention (Umsetzung)

      **Abstraktions-Traits (alle mit Mock/Fake):**
      - `ComputeBackend` — CPU/Mock vs. CUDA (hinter Feature `cuda`); die gesamte Orchestrierung läuft gegen das Trait.
      - **Kernel-Trait** (B5, `resident_index.md`/`resident_subengine.md` §4.5): Batch-Start, Batch-Ende, Ergebnisse, geschriebene Bereiche, Fortschrittszähler → `FakeKernel` mit skriptbaren Ergebnissen.
      - **Mess-Trait** (F2, `resident_precision.md`: „Kalibrierung über ein Trait für die Messung abstrahiert") → `FakeMeasure` (Basiswert + seeded Rauschen, Varianten konstant/Ausreißer/Drift).
      - `Clock`-Trait bzw. `tokio::time::pause` für TTL/Idle/Zeitscheiben.

      **Nebenläufigkeit:** CAS-Protokolle der Queue (B1) mit `loom` erschöpfend; größere Interleavings mit `shuttle`; Sequenzen/Properties mit `proptest`. Failpoints für gezielte ABA-Szenarien.

      **Testmatrizen je Paket (verbindlich):**
      - **B1 Queue:** `claim`=CAS `Queued→Taken`; Fertigstellung `Taken→Done`; `cancel` nur auf `Queued`; Unlink ohne Durchläufer-Restart; iterativer Drop (keine Stack-Überläufe); Notify-Topologie (loom).
      - **E5/E6 Index-Sync (Kernkandidaten):** Reihenfolge im Kommunikationsslot (Kernel-Writes → Auflösen → Updates), inkl. Ergebnisse auf frisch geschriebene Adressen; Ergebnisstatus (entfernt→verworfen, Update anstehend→`outdated`); Echo (eigener `apply` löst keinen Reload aus, fremder Stamp schon, zwei Manager konvergieren); verlorenes/zusammengefasstes Signal; Signal-Quittierung **vor** dem Scan; Stamp-Lesereihenfolge beim Laden; ABA (Entfernen+Neuanlage → anderer Stamp); CoW-Swap (altes Segment erst frei, wenn kein vor dem Swap eingereihter Request es referenziert); Gather-vs-Segment-Dichteschwelle; Single-Flight (ein Ladevorgang, Fehlerverteilung an alle Wartenden); `UnsupportedScope`.
      - **F2–F5 Kalibrierung:** Matching (Klemmen, NaN-Fehler, Tie-Break zum genaueren, Einzelprofil `p=0.5`); Profilbildung (Spread-Schwellwert, Zusammenfassen redundanter Punkte, Positionsberechnung über Bogenlänge); Pareto-Auswahl (dominierte Kombinationen verworfen); Persistenz (Fingerprint-Wechsel→`Stale`, fehlende Datei→`Missing`); Stabilitätslogik (verrauschte Mock-Messwerte, Abbruch an Obergrenze→`unstable`).
      - **C2/C3 Processor/Planner:** Worker-Loop-Priorität, Starved-Hint-Verfall, Callback+`cuEventQuery` ohne Doppelverarbeitung (gegen Mock-Backend), Planner-Expansion rein (gleiche Eingabe → gleiche Ops).

      **Keine Bit-Gleichheit:** Präzisions-/Scorer-Tests prüfen **Schranken/Relationen**, nicht Gleichheit (`resident_precision.md`).

      ## 4. Umsetzungsschritte

      1. Trait-Definitionen + Fakes (`ComputeBackend`, Kernel, Mess, `Clock`) in einem `testkit`/`test-support`-Modul (dev-dependency-sichtbar).
      2. `loom`/`shuttle`/`proptest` als dev-dependencies; `cfg(loom)`-Shim für `std::sync`→`loom::sync` in den betroffenen Modulen.
      3. Failpoint-/Step-Hooks an den Sync-Punkten (Kommunikationsslot, Lade-Pfad).
      4. Testmatrizen aus §3 als konkrete Test-Dateien je Paket.
      5. CI: Default-Lauf ohne `cuda` (alle Unit-Tests), siehe Q4.

      ## 5. Offene Verifikationen ([prüfen])

      - `loom`/`shuttle`/`proptest`/`mockall`/`rand`-Versionen in der Projekt-`Cargo.toml`.
      - `tokio` `test-util`-Feature aktiviert.

      ## 6. Definition of Done

      - Abstraktions-Traits + Fakes vorhanden; `loom`/`shuttle`/`proptest` eingebunden; Failpoints an Sync-Punkten; die Testmatrizen (B1, E5/E6, F2–F5, C2/C3) sind als Tests angelegt; alle Tests laufen **ohne** GPU im Default-Profil.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q4_feature_flags.md
    mode: replace
    content: |
      # Umsetzungsplan Q4 · Feature-Flags

      **Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren (hängt an A2 Modulstruktur). Voraussetzung dafür, dass die Logik ohne GPU testbar bleibt.

      ## 1. Zweck

      Reale CUDA-Pfade hinter ein Feature-Flag legen, sodass (a) der Default-Build **ohne** GPU/`nvcc` kompiliert und alle Unit-Tests ausführt, (b) `#[cfg]` nur an wenigen Modulgrenzen auftritt, (c) der Backend-Austausch über Traits/Factory statt `cfg`-Streuung läuft.

      ## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q4)

      - **Best Practices:** Features sind **additiv** (aktivieren nur Code), Default **ohne** `cuda`. Optionale Dependencies mit `dep:`-Syntax; `#[cfg(feature = "cuda")]` nur an wenigen Stellen (Modulgrenze). (Cargo Book „Features".)
      - **`candle-core` `cuda`:** aktiviert `cudarc`, `candle-kernels` u. a.; Bauen braucht CUDA-Toolkit/`nvcc` **[prüfen aktuelle Cargo.toml]**. Durchreichen:
        ```toml
        [features]
        default = []
        cuda = ["candle-core/cuda", "dep:cudarc"]
        [dependencies]
        candle-core = "…"
        cudarc = { version = "…", optional = true }
        ```
        Bei optionalem `candle-core`: Weak-Dependency `candle-core?/cuda`.
      - **No-GPU-Pfad:** Trait-Objekt-/Generics-Austausch statt `cfg`-Streuung: `cfg` nur beim **Konstruieren des Backends** (eine Factory), alles andere feature-unabhängig.
      - **CI:** Matrix **mit und ohne** `cuda`. No-GPU-Build muss ohne `nvcc` kompilieren und alle Unit-Tests ausführen; `cuda`-Build mindestens per `cargo check`.

      ## 3. Verbindliche Konvention (Umsetzung)

      - **`default = []`** (kein `cuda`). `cuda = ["candle-core/cuda", "dep:cudarc"]` (bzw. Weak-Syntax falls `candle-core` optional).
      - **`#[cfg(feature = "cuda")]` nur an der Backend-Factory** und an der einen Candle-Adapter-Stelle (Schicht 4, `resident_subengine.md` §2: „Candle ist ein Adapter an genau einer Stelle"). Keine `cfg`-Streuung in Scheduler/Planner/Index-Logik.
      - **Backend-Factory** konstruiert `ComputeBackend` (Q3): ohne `cuda` → CPU-Backend; mit `cuda` und verfügbarer GPU → CUDA-Backend, sonst Fallback CPU (`resident_subengine.md` §6: GPU-Initfehler → CPU). Der Rest der Engine arbeitet nur gegen das Trait.
      - **Reale CUDA-abhängige Pfade hinter `cuda`:** Acquire-Load/Pinned-Staging/Arena/Streams (`resident_index.md` Schlusssatz), persistenter Kernel (H1), Graph-Capture (H4). Deren **Logik** (Mapping, Disziplin, Scheduling) bleibt feature-unabhängig und gegen Mocks testbar.
      - **Invarianten bleiben feature-unabhängig** (`resident_subengine.md` §2): `Region`/`Op`/`ExecContext`-Signaturen ohne `Tensor`/Raw-Pointer — unabhängig von `cuda`.

      ## 4. Umsetzungsschritte

      1. `Cargo.toml`: `default`/`cuda`-Features, `cudarc` optional, Passthrough an `candle-core`.
      2. Backend-Factory mit dem einzigen `#[cfg(feature = "cuda")]`-Entscheidungspunkt.
      3. CUDA-spezifische Module hinter `#[cfg(feature = "cuda")]`; CPU/Mock-Pendants immer gebaut.
      4. CI-Matrix: Job A `--no-default-features`/default → `cargo build` + `cargo test` (ohne `nvcc`); Job B `--features cuda` → mindestens `cargo check`.
      5. Doku: „Default baut GPU-frei; `cuda` nur für reale CUDA-Pfade."

      ## 5. Tests

      - Default-Build kompiliert ohne `nvcc` und führt alle Unit-Tests aus (Q3).
      - Backend-Factory liefert ohne `cuda` das CPU-Backend; mit `cuda` ohne GPU erfolgt der dokumentierte Fallback.
      - Kein `#[cfg(feature = "cuda")]` außerhalb von Factory + Candle-Adapter (Lint/Grep-Check im CI).

      ## 6. Offene Verifikationen ([prüfen])

      - Aktuelle `candle-core`-`Cargo.toml`: was das `cuda`-Feature zieht; `cudarc`-Version = die von `candle` verwendete (`resident_subengine.md` §10: sonst inkompatible Typen).

      ## 7. Definition of Done

      - Feature-Matrix wie §3; einziger `cfg`-Entscheidungspunkt in der Factory; CI-Matrix mit/ohne `cuda` grün; No-GPU-Default führt alle Unit-Tests aus; `cfg`-Streuungs-Check besteht.
  reason: "Creating the Stufe-2 implementation plans for groups H (H1–H6, extension-points-only) and Q (Q1–Q4, binding cross-cutting conventions) plus a short orientation overview, in the plan directory, derived from resident_planplan.md, context_h_q.md and the authoritative source docs."
```
/answer b929-3 allow
Result Stats: id=aed6e3c8-93f9-4d67-a122-b8fb853e5d6d, total: 214416, in: 123, out: 42194, reason: 10334, read: 37673, write: 172099
ReasoningToken: 10313
ReasoningToken: 21
Fertig. Ich habe die Umsetzungspläne (Stufe 2) für Gruppe H und Q in `/home/user/xyan/xy.ai.workbench/project/resident/plan/` erstellt – ein eigenständiger, einzeln an Agenten zuweisbarer Plan pro Paket, plus eine kurze Orientierung.

**Erstellte Dateien (11):**

| Datei | Inhalt |
|---|---|
| `plan_h_q_overview.md` | Überblick, Grundregel für H, Statuslegende, Reihenfolge/Abhängigkeiten |
| `plan_h1_gpu_ringbuffer.md` | H1 – persistenter Kernel/SPSC-Ring |
| `plan_h2_gleitende_indizierung.md` | H2 – Sliding Window |
| `plan_h3_funnel_plaid_reranker.md` | H3 – Funnel/PLAID & Cross-Encoder |
| `plan_h4_graph_capture.md` | H4 – Graph-Capture & Persistent-Scorer |
| `plan_h5_signalkompression.md` | H5 – FFT/Wavelet (außerhalb Engine-Scope) |
| `plan_h6_mutable_weights_bandit.md` | H6 – Mutable Weights/Bandit (außerhalb Engine-Scope) |
| `plan_q1_fehler_taxonomie.md` | Q1 – `thiserror`-Taxonomie |
| `plan_q2_logging.md` | Q2 – `tracing` |
| `plan_q3_teststrategie.md` | Q3 – Tests & Mocks |
| `plan_q4_feature_flags.md` | Q4 – Feature-Flags |

**Angewandte Leitentscheidungen:**

- **Gruppe H** ist durchgängig als *„nur Erweiterungspunkte sichern, keine Detailplanung“* ausgeführt (Parameterobjekt, `requires()`-Affinität, Op-/Launcher-/Scorer-Austausch hinter gleichem Trait). Jeder H-Plan hat: Kurzriss des späteren Features, destillierte Recherche aus `context_h_q.md` (mit `[prüfen]`), eine Checkliste „darf jetzt nicht verbaut werden“, einen expliziten „jetzt NICHT umsetzen“-Abschnitt und eine Definition of Done.
- **H5/H6** sind klar als *außerhalb des Engine-Scopes* markiert (nur Nicht-Ausschluss über offene `AccessMethod`/`Scope`/Params + ADR-Divergenznotiz), passend zum Thesen-Abgleich im Metaplan.
- **Gruppe Q** ist als verbindliche, von allen anderen Gruppen konsumierte Konvention ausgeführt (konkrete Enum-/Span-/Trait-/Feature-Strukturen, Umsetzungsschritte, Tests, Cargo-Snippet).
- Recherchebasis durchgehend `context_h_q.md`; Trait-/Hook-Bezüge präzise aus `resident_subengine.md` (§2/§4.1–§4.6/§6/§9), `resident_index.md`, `resident_planner.md`, `resident_processor.md`, `resident_precision.md`, `resident_first.md`. `resident_index.md` wurde korrekt als Vektorindex-Spezifikation (nicht als Indexdatei) behandelt.

Offene `[prüfen]`-Punkte (Crate-Versionen, `cudarc`/`candle`-APIs, `next-plaid`, Host-Atomics sm75 usw.) sind in den jeweiligen Plänen als Verifikationen markiert.
