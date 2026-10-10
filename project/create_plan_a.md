Erstelle Umsetzungspläne für Agenten im Verzeichnis `/home/user/xyan/xy.ai.workbench/project/resident/plan` für Gruppe A aus `resident_planplan.md`.

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
SystemInit:  id=0135fe34-9ed9-4412-8c58-3a67ef879c9a, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 Ich werde zuerst die relevanten Dateien lesen, um zu verstehen, was für Gruppe A geplant werden soll.
Control Request:
```yaml
id: 0f9c-a
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_a.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  reason: "Lesen der Plan-Definition (resident_planplan.md), des Kontexts für Gruppe A (context_a.md) sowie der Übersichts-/Indexdateien, um die zu erstellenden Umsetzungspläne für Gruppe A zu verstehen."
```
/answer 0f9c-a allow
ReasoningToken: 232
Text:
 Ich habe die Metaplanung (`resident_planplan.md`), den verifizierten Recherchekontext für Gruppe A (`context_a.md`) sowie die Basiskonzepte (`resident.md`, `resident_index.md`) gelesen. 

Gruppe A besteht aus vier Paketen. Wichtig: `resident_planplan.md` löst **A1** ("These destillieren") auf und ersetzt es durch **A1′** ("Scope-Grenze Engine ↔ Suchschicht fixieren"). Ich erstelle daher Umsetzungspläne für **A1′, A2, A3, A4** plus eine Index-/Übersichtsdatei. Die Pläne stützen sich durchgängig auf die verifizierten Fakten aus `context_a.md`.
Control Request:
```yaml
id: 0f9c-b
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a_index.md
    mode: replace
    content: |
      # Umsetzungspläne — Gruppe A (Fundament & Kontext)

      > **Stufe 2 (Ausführungsplanung).** Erzeugt aus `resident_planplan.md` (Gruppe A) und dem
      > verifizierten Rechercheergebnis `context/context_a.md` (Verifizierungsdatum 2026-10-10).
      > Diese Datei ist die Einstiegs-/Übersichtsdatei für die vier Paketpläne der Gruppe A.

      ## Zweck der Gruppe A

      Gruppe A legt das Fundament, bevor Datentypen (B), Orchestrierung (C) usw. geplant/umgesetzt
      werden: Umfangsabgrenzung, Projekt-/Modulstruktur, verifizierter Abhängigkeits-Stack und die
      querschnittlichen Konventionen (Fehler/Logging/Test/Feature-Flags). Alle nachfolgenden Gruppen
      bauen hierauf auf.

      ## Zielumgebung (gilt für alle A-Pläne)

      - **Rust-Projekt (Code-Ausgabe):** `/home/user/xyan/xy.ai.workbench/rag`
      - **Zielhardware:** GTX 1660, `sm_75` (Turing), 6 GB VRAM.
      - **Modell:** `intfloat/multilingual-e5-small` (dense Bi-Encoder, BERT-Architektur, dim 384).
      - **CUDA-Linie:** CUDA **12.x** (nicht 13.x), Build mit `CUDA_COMPUTE_CAP=75`.
      - **Konsequenz sm_75:** bf16-Kernel in candle entfallen (`-DNO_BF16_KERNEL`) → nur **f16/f32** planen.

      ## Paketpläne & Reihenfolge

      Topologische Sequenz laut Metaplan: **A1 → A2 → A3 → A4**.

      | Plan | Paket | Ergebnis (Artefakt) | Abhängt von |
      |------|-------|---------------------|-------------|
      | [`plan_a1.md`](./plan_a1.md) | **A1′** Scope-Grenze Engine ↔ Suchschicht | Scope-Charta (Dokument) | — |
      | [`plan_a2.md`](./plan_a2.md) | **A2** Projekt- & Modulstruktur | Workspace-/Crate-Schnitt + Feature-Matrix (Code-Gerüst) | A1′ |
      | [`plan_a3.md`](./plan_a3.md) | **A3** Stack- & Versionskompatibilität | Verifizierte Abhängigkeitsliste + Risikoliste (gepinnt) | A2 |
      | [`plan_a4.md`](./plan_a4.md) | **A4** Querschnittskonventionen | Konventionsdokument + Basis-Scaffolding (speist Q1–Q4) | A2 |

      Hinweis zu A1: Das ursprüngliche Paket A1 ("These destillieren") ist laut `resident_planplan.md`
      **erledigt** (die These `resident_streaming.md` wurde eingelesen, der Abgleich steht im Metaplan).
      A1 wird deshalb durch **A1′** (Scope-Charta) ersetzt — siehe `plan_a1.md`.

      ## Querschnitt Q (begleitend, in Gruppe A initial zu fixieren)

      - **Q1 Fehler-Taxonomie** (`thiserror`) — Grundlage in A4, fortgeschrieben je Gruppe.
      - **Q2 Logging** (`tracing`) — Span-/Event-Konventionen in A4.
      - **Q3 Teststrategie & Mocks** — Konventionen in A4, No-GPU-Pfad in A2.
      - **Q4 Feature-Flags** — reale CUDA-Pfade hinter `cuda`-Flag, Logik GPU-frei testbar (A2).

      ## Definition of Done für Gruppe A (gesamt)

      1. Scope-Charta liegt vor und ist referenzierbar aus C1/E2 (A1′).
      2. Workspace baut mit **und** ohne `cuda`-Feature; `cargo build --no-default-features` benötigt
         kein CUDA-Toolkit (A2 + Q4).
      3. `Cargo.toml`/`Cargo.lock` enthalten den verifizierten, gepinnten Stack; offene Punkte aus
         `context_a.md` sind geklärt oder als Restrisiko dokumentiert (A3).
      4. Konventionsdokument (Fehler/Logging/Test/Feature) existiert; das Basis-Scaffolding
         (Fehler-Modul, `tracing`-Init, Mock-Konvention) kompiliert (A4, Q1–Q4).

      ## Verwendete Legende (aus context_a.md)

      **[V]** live verifiziert · **[W]** Fachwissen, nicht live verifiziert · **[?]** offen.
      Alle in den Plänen zitierten Versionen/Fakten tragen diese Herkunft.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a1.md
    mode: replace
    content: |
      # Umsetzungsplan A1′ — Scope-Grenze Engine ↔ Suchschicht fixieren

      > Teil von Gruppe A (Fundament & Kontext). Ersetzt das erledigte Paket A1 ("These destillieren").
      > Quellen: `resident_planplan.md` (Abschnitt "Auswirkungen auf den Metaplan", Abgleich-Tabelle),
      > `resident.md` (Ausblick/Klarstellung), `resident_index.md` ("Nicht Teil dieser Ergänzung"),
      > `resident_precision.md` ("Nicht Teil"-Abschnitte).

      ## 1. Ziel & Ergebnistyp

      Verbindliche **Scope-Charta** als eigenständiges Dokument: Was liefert die "Resident"-Engine,
      und was bleibt bewusst außerhalb (Oberschicht/Suchschicht). Die Charta ist **blockierend** für
      C1 (Fassaden-API) und E2 (Scope-Resolver) und verhindert, dass später Zuständigkeiten
      verwischen (z. B. Qualität/Chunking in die Engine wandern).

      **Ergebnistyp:** Markdown-Dokument (keine Code-Artefakte).

      ## 2. Eingaben & Quellen

      - `resident_planplan.md` → Abschnitte **Kernbefund**, **Abgleich-Tabelle** (Status K/I/A/E/—),
        **Zentrale Divergenz**, **Auswirkungen auf den Metaplan** (A1′, H5, H6).
      - `resident.md` → "Klarstellung" (Dateihandling/Persistenz außerhalb der Engine),
        "Ausblick" (gleitende Indizierung, Ringbuffer).
      - `resident_index.md` → "Leitprinzipien" (approximative Vorfilter = Oberschicht),
        "Nicht Teil dieser Ergänzung".
      - `resident_precision.md` → "Nicht Teil"-Abschnitte (Chunking, Query-Formulierung, Rerank-Policy).

      ## 3. Abhängigkeiten

      - **Abhängt von:** — (keine; A1 These-Abgleich ist bereits erledigt).
      - **Blockiert:** C1 (Fassaden-API), E2 (Scope-Resolver), und wirkt in G1/G3 (Standardpfad
        dense vs. ColBERT/MaxSim-Wechselpfad).

      ## 4. Vorannahmen / verifizierter Stand

      Der These-Abgleich ist abgeschlossen (siehe Metaplan). Kernaussagen, die in die Charta
      eingehen und nicht neu recherchiert werden müssen:

      - Nebenläufigkeitsmodell Phase 1 = **Lock-free Pull-Queue + Processor/Planner/Subengine**;
        der SPSC-Ringbuffer (These §3) ist auf den Persistent-Kernel-Modus **zurückgestellt** (H1).
      - Standardpfad ist **dense e5-small (Cosine)**; MaxSim/ColBERT existiert nur als
        Kernel-Infrastruktur/Wechselpfad (A/I).
      - Nicht adressiert / außerhalb: Signalkompression (FFT/Wavelet, §6/§6.1 → H5),
        mutable weights / Online-Bandit (§9/§9.1 → H6).

      ## 5. Arbeitsschritte (für den Agenten)

      1. **Grenztabelle aufstellen** mit den fünf Statusklassen aus dem Metaplan:
         **K** (Kern/umgesetzt), **I** (Hook vorbereitet), **A** (Ausblick in der Engine),
         **E** (bewusst außerhalb), **—** (nicht adressiert). Jede Zeile = ein Aspekt mit Status,
         Beleg (Dokument/§) und Zuständigkeit (Engine vs. Oberschicht).
      2. **"Engine liefert"-Liste** ausformulieren: residenter Compute (Embedding/Similarity/Inferenz),
         Index-Substrat + Streaming-Sync RAM↔Index↔VRAM, modell-entkoppelte MaxSim-Geometrie,
         Residency/Eviction, Scope-Auflösung über deklarierte Verfahren + Resolver.
      3. **"Außerhalb der Engine"-Liste** ausformulieren: Chunking, Query-Formulierung,
         Kandidaten-k/Rerank-Policy, Qualität/Vollständigkeit, approximative Vorfilter
         (Zentroid/FFT/Kompression) als Oberschicht-Entscheidung, Drift-Monitor,
         Signalkompression (H5), mutable weights/Bandit (H6), Index-Persistenzformat/-zeitpunkte.
      4. **Zentrale Divergenz dokumentieren** (These §3 Ringbuffer vs. Phase-1-Pull-Queue) als
         bewusste Umsetzungsentscheidung, damit die These-Treue nachvollziehbar bleibt.
      5. **Hook-Verpflichtungen festhalten** (was Phase 1 "nicht verbauen" darf): erweiterbares
         Parameterobjekt, `requires()`-Affinität, Op-Austausch hinter gleichem Trait,
         `AccessMethod`/`Scope`/`summary()` offen für spätere Frequenz-/Kompressionsdomäne (H5),
         Index nur **bidirektional mutabel**, Weights immutabel (H6).
      6. **Verweise setzen:** explizit markieren, dass C1 und E2 gegen diese Charta zu planen sind.

      ## 6. Zu erzeugende Artefakte

      - `rag/docs/architecture/scope_charta.md` (empfohlener Pfad; alternativ im Projektordner).
        Enthält: Grenztabelle, "Engine liefert"-/"Außerhalb"-Listen, Divergenz-Notiz, Hook-Liste.

      ## 7. Offene Punkte / Verifikation vor Abschluss

      - Keine Recherche offen (A1 erledigt). Prüfen, dass **jede** H-Position (H1–H6) in der Charta
        einen eindeutigen Status und — falls Hook — eine benannte Hook-Stelle hat.
      - Gegencheck: Jede "außerhalb"-Aussage muss durch einen "Nicht Teil"-Beleg in `resident_index.md`
        oder `resident_precision.md` oder durch die Abgleich-Tabelle gedeckt sein.

      ## 8. Akzeptanzkriterien (Definition of Done)

      1. Charta enthält vollständige Grenztabelle (alle Aspekte der Metaplan-Tabelle abgebildet).
      2. Jede Hook-Verpflichtung ist einer konkreten späteren Erweiterung (H1–H6) zugeordnet.
      3. Divergenz Ringbuffer↔Pull-Queue ist als Entscheidung (nicht als Inkonsistenz) dokumentiert.
      4. C1 und E2 referenzieren die Charta als Planungsgrundlage (Verweis gesetzt).

      ## 9. Querschnitt (Q1–Q4)

      - **Q3:** Kein Testbedarf (Dokument). Charta definiert jedoch, welche Qualitäts-/Retrieval-Tests
        **nicht** Teil der Engine-Testsuite sind (Abgrenzung für spätere Testmatrizen).
      - **Q1/Q2/Q4:** nicht betroffen.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a2.md
    mode: replace
    content: |
      # Umsetzungsplan A2 — Projekt- & Modulstruktur / Feature-Flags

      > Teil von Gruppe A (Fundament & Kontext).
      > Quellen: `resident_planplan.md` (Paket A2, Gruppenschnitt B–G), `context_a.md` (Paket A2,
      > Abschnitte 1–3 und Stack-Verifikation), `resident.md` (Stack, Schichten),
      > `resident_subengine.md` (Invarianten, Ausbaupfad §9).

      ## 1. Ziel & Ergebnistyp

      Festlegen und anlegen des **Workspace-/Crate-Schnitts** im Rust-Projekt `rag`, mit Modulgrenzen
      entlang der Schichten (Fassade / Processor / Planner / Subengine / IndexManager / Index /
      Kalibrierung) und einer **Feature-Matrix**, die reale CUDA-Pfade hinter `cuda` kapselt, sodass
      die Kernlogik ohne GPU baubar und testbar bleibt (Q4).

      **Ergebnistyp:** Workspace-Gerüst (Cargo-Dateien + leere Modulbäume) + Feature-Matrix-Dokument.

      ## 2. Eingaben & Quellen

      - `context_a.md` Paket A2 Nr. 1–3 (Feature-Flag-Idiome, `cuda`-Feature von candle-core 0.11.0,
        Schichtmuster) **[V]** für das candle-Feature, **[W]** für die Idiome.
      - `resident_planplan.md` Gruppen B–G: liefert die Komponentenliste, die 1:1 auf Module abbildet.
      - `resident.md` Stack-Abschnitt (Model-Loading, CUDA/GPU, Concurrency, Index).
      - A1′ Scope-Charta (für die öffentliche API-Grenze der Fassaden-Crate).

      ## 3. Abhängigkeiten

      - **Abhängt von:** A1′.
      - **Blockiert:** A3 (Dependency-Pinning braucht die Crate-Struktur), A4 (Konventionen/Scaffolding
        werden in diese Module gelegt), und alle Gruppen B–G (Modulziele).

      ## 4. Vorannahmen / verifizierter Stand (aus context_a.md)

      - candle-core 0.11.0 **[V]**: `cuda = ["cudarc", "dep:candle-kernels", "candle-ug?/cuda"]`,
        `default = []`. Ohne `cuda`-Feature kein cudarc/candle-kernels/nvcc nötig → No-GPU-Build möglich.
      - Idiom **[W]**: `candle-core` als **optionale** Dependency, `cuda = ["dep:candle-core",
        "candle-core/cuda"]`; reale Backends unter `#[cfg(feature = "cuda")]`; Default = CPU/Mock.
      - Idiom **[W]**: Crate-Grenze nur bei eigener Feature-/Dependency-Menge (CUDA) oder
        Wiederverwendung; sonst Modul. Pure Funktionsschicht separat, Adapter hinter Traits.

      ## 5. Arbeitsschritte (für den Agenten)

      ### 5.1 Workspace anlegen

      1. Workspace-Root `rag/Cargo.toml` mit `[workspace] resolver = "2"` und `members`.
      2. Crates anlegen (empfohlener Schnitt, dreigeteilt wie in `context_a.md` Nr. 1):
         - **`resident-core`** — reine Logik + alle Traits + CPU-Referenzimplementierungen + Mocks.
           Keine `candle`/`cudarc`-Abhängigkeit. GPU-frei kompilierbar und voll unit-testbar.
         - **`resident-cuda`** — CUDA/candle-Backend. Hängt von `resident-core` + `candle-core`
           (+ ggf. `cudarc`) ab. Implementiert die GPU-Traits aus `resident-core`.
         - **`resident`** — Fassaden-Bibliothek (`lib.rs`), re-exportiert die öffentliche API,
           wählt Backend per Feature (`cuda` → `dep:resident-cuda`, sonst CPU/Mock).
         - optional **`resident-cli`** / **`resident-server`** — Binary(s), verdrahten die Fassade
           (Server/Daemon/CLI laut `resident.md` Kontrollfluss).

      ### 5.2 Modulbaum in `resident-core` (Schichten → Module)

      Leere, dokumentierte Modulstümpfe anlegen (werden in B–G befüllt). Zuordnung zu Metaplan-Paketen:

      ```
      resident-core/src/
        lib.rs            // pub use Re-Exports, pub(crate)-Interna
        error.rs          // Q1 Fehler-Taxonomie-Basis (A4)
        telemetry.rs      // Q2 tracing-Konventionen/Init (A4)
        queue/            // B1 Queue & Task-Primitive (Node/Task/TaskState)
        memory/           // B2 Region/Layout/DType/MemoryManager-Trait
        op/               // B3 Operation/requires()/Value::Host|Device
        index/            // B4 Index-Trait, Scope, AccessMethod, Stamp
        kernel/           // B5 SegmentDesc/Layout/KernelHit, Kernel-Trait + Mock
        precision/        // B6 CapabilitySet/Profile (+ F1–F5 Kalibrierung)
        facade/           // C1 Resident-Fassade (öffentliche API)
        processor/        // C2 Orchestrierung (Task-Queue + Op-Queues)
        planner/          // C3 zustandslose Taskexpansion
        subengine/        // D1 Gemeinsames (Lifecycle, Pull), D5 Tokenizer (CPU)
        manager/          // E StreamedIndexManager, ScopeResolver, Update-Queue, Residency
      ```

      Regel: Traits und zustandslose Logik liegen in `resident-core`; alles mit echtem GPU-Zugriff
      (Device/Lane D2, GPU-Arena D3, LoadedEncoder D4, CUDA-Kernels D6, reale `ArenaAllocator` E3)
      kommt nach `resident-cuda` als Trait-Implementierung.

      ### 5.3 Backend-Trennung & CPU-Referenz

      1. GPU-berührende Fähigkeiten als schmale Traits in `resident-core` definieren (Namen als
         Platzhalter, finalisiert in B/D): z. B. `Kernel`, `Embedder`/Encoder, `MemoryManager`,
         `ArenaAllocator`.
      2. In `resident-core` eine **CPU-Referenz** und einen **Mock** je Trait vorsehen
         (Default-Pfad ohne GPU). Mock-Sichtbarkeit über `#[cfg(any(test, feature = "mock"))]`.
      3. In `resident-cuda` die realen Implementierungen anlegen (zunächst leer), gesamte Crate bzw.
         relevante Module unter `#[cfg(feature = "cuda")]` erreichbar.

      ### 5.4 Feature-Matrix festlegen

      `resident` (Fassade), empfohlene Features:

      | Feature | Zieht | Zweck |
      |---------|-------|-------|
      | `default` | `[]` | CPU + Mock, kein CUDA-Toolkit nötig |
      | `cuda` | `dep:resident-cuda` (→ `candle-core/cuda`) | reale GPU-Pfade |
      | `mock` | — (aktiviert Test-Fakes in core) | deterministische Fakes außerhalb von `#[test]` |
      | `server` | tokio-Runtime/Netz | Daemon/Server-Binary |
      | `cli` | CLI-Parsing | CLI-Binary |

      - `candle-core` in `resident-cuda`: `optional = true`, `cuda = ["dep:candle-core",
        "candle-core/cuda"]`. `dep:`-Syntax verwenden, damit kein impliziter Feature-Name entsteht.
      - `cuda` **niemals** in `default`.

      ### 5.5 CI-/Build-Profile

      1. CI-Job A: `cargo build --workspace --no-default-features` **ohne** CUDA-Toolkit (muss grün sein).
      2. CI-Job B: `cargo test -p resident-core` (GPU-frei, inkl. Mocks).
      3. CI-Job C (optional, GPU-Runner): `cargo build -p resident --features cuda` mit
         `CUDA_COMPUTE_CAP=75` und CUDA 12.x.
      4. GPU-Integrationstests mit `#[ignore]` oder eigenem Feature, damit CI ohne GPU grün bleibt.

      ## 6. Zu erzeugende Artefakte

      - `rag/Cargo.toml` (Workspace), `rag/crates/{resident-core,resident-cuda,resident}/Cargo.toml`.
      - Modulstümpfe gemäß 5.2 (mit Doku-Kommentar, der auf das jeweilige Metaplan-Paket verweist).
      - `rag/docs/architecture/feature_matrix.md` (Feature-Tabelle + CI-Matrix + No-GPU-Begründung).

      ## 7. Offene Punkte / Verifikation vor Abschluss

      - **[?]** Prüfen, ob die drei Tasktypen (Embedding/Similarity/Inferenz) je ein eigenes
        Parameter-Submodul in `facade/` brauchen (Feingranularität an C1 übergeben, Hook offen halten).
      - **[W]→verifizieren:** `dep:`-Syntax + `candle-core/cuda` baut im leeren Gerüst durch
        (`cargo build --no-default-features` muss ohne candle linken).
      - Entscheiden, ob `resident-cpu` als eigene Crate nötig ist oder CPU-Referenz in `resident-core`
        bleibt (Empfehlung: in core, bis D1/D-Paket Gegenteil zeigt).

      ## 8. Akzeptanzkriterien (Definition of Done)

      1. `cargo build --workspace --no-default-features` erfolgreich **ohne** installiertes CUDA-Toolkit.
      2. `cargo build -p resident --features cuda` kompiliert das leere CUDA-Gerüst (auf GPU-Runner).
      3. Jedes Metaplan-Paket aus B–G hat ein eindeutiges Zielmodul (keine Lücke, keine Dopplung).
      4. Feature-Matrix dokumentiert; `cuda` nicht in `default`.

      ## 9. Querschnitt (Q1–Q4)

      - **Q4:** Kernaufgabe dieses Pakets — CUDA-Pfade hinter `cuda`, Logik GPU-frei testbar.
      - **Q1/Q2:** Zielmodule `error.rs`/`telemetry.rs` werden hier angelegt, in A4 gefüllt.
      - **Q3:** Mock-Sichtbarkeit (`#[cfg(any(test, feature = "mock"))]`) und No-GPU-Testpfad verankert.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a3.md
    mode: replace
    content: |
      # Umsetzungsplan A3 — Stack- & Versionskompatibilität verifizieren

      > Teil von Gruppe A (Fundament & Kontext).
      > Quellen: `resident_planplan.md` (Paket A3), `context_a.md` (Versionsübersicht + Pakete A3
      > Nr. 4–15 + "Offene Punkte"), `resident.md` (Stack), `resident_subengine.md` §10.

      ## 1. Ziel & Ergebnistyp

      Harte Vorabprüfung aller Abhängigkeiten: **gepinnte, verifizierte Abhängigkeitsliste** in den
      Cargo-Dateien des Workspace (aus A2) plus eine **Risikoliste** mit den noch offenen Punkten aus
      `context_a.md`, entweder aufgelöst oder als dokumentiertes Restrisiko. Ziel ist, dass spätere
      Gruppen nicht an Versions-/Feature-Konflikten scheitern (insb. doppelte `cudarc`-Versionen).

      **Ergebnistyp:** gepinnte `Cargo.toml`(s) + `Cargo.lock` + Verifikations-/Risikodokument.

      ## 2. Eingaben & Quellen

      - `context_a.md` → **Versionsübersicht** (Tabelle, Stand 2026-10-10, **[V]**) und Pakete A3
        (Nr. 4–15) sowie die priorisierte **Offene-Punkte**-Liste (1–5).
      - A2-Artefakte (Workspace-Struktur, Feature-Matrix).
      - `resident.md` Stack, `resident_subengine.md` §10 (Prüf-Checkliste).

      ## 3. Abhängigkeiten

      - **Abhängt von:** A2.
      - **Blockiert:** alle Implementierungsgruppen (B–G), insb. D (candle/cudarc), E3/E7 (parquet/mmap),
        G1/G2 (Modell/Index-Schema).

      ## 4. Vorannahmen / verifizierter Stand (context_a.md, 2026-10-10)

      Verifizierte Stabilstände (**[V]**, crates.io API/Suche):

      | Crate | Version | Hinweis |
      |---|---|---|
      | candle-core / candle-transformers | 0.11.0 | candle `cuda = ["cudarc", "dep:candle-kernels", "candle-ug?/cuda"]` |
      | cudarc | 0.19.9 (max stable) | candle verlangt `^0.19.8`; candle-**main** nennt 0.19.10 **[?]** |
      | tokenizers | 0.23.2 | candle nutzt 0.23.1, `default-features = false`; 1.0.0-rc.2 meiden |
      | safetensors | 0.8.0 | candle hängt hart `^0.8.0`, nutzt `memmap2 ^0.9.3` |
      | parquet (arrow-rs) | 60.0.0 | schnelle Majors; `arrow`-Crates exakt gleiche Major |
      | tokio | 1.53.2 | Features: `rt-multi-thread,sync,macros,time` |
      | crossbeam / crossbeam-queue | 0.8.5 / 0.3.14 | `ArrayQueue`/`SegQueue` |
      | parking_lot | 0.12.5 | kurze kritische Abschnitte |
      | arc-swap | 1.9.2 | `ArcSwapOption` für Queue-Links |
      | thiserror | 2.0.20 | 1→2 Migrationsnotizen **[?]** |
      | tracing / tracing-subscriber | 0.1.44 / 0.3.23 | keine neue Major |
      | half | 2.7.1 | f16 |
      | mockall | 0.15.0 | Mocking |

      Hardware-Fakten **[V/W]**: sm_75 → bf16-Kernel entfallen (`-DNO_BF16_KERNEL`), nur f16/f32;
      CUDA **12.x** empfohlen, Build mit `CUDA_COMPUTE_CAP=75`; `model.safetensors` (471 MB f32) für
      `intfloat/multilingual-e5-small` **vorhanden** → keine Konvertierung nötig (f16 halbiert VRAM).

      ## 5. Arbeitsschritte (für den Agenten)

      ### 5.1 Dependencies pinnen

      1. In den Crates aus A2 die Versionen aus der Tabelle eintragen. `candle-core` in `resident-cuda`
         `optional = true` mit `features = ["cuda"]` hinter dem `cuda`-Feature.
      2. tokio-Features minimal halten (`rt-multi-thread`, `sync`, `macros`, `time`).
      3. tokenizers `default-features = false` (entfernt `onig`/`progressbar`) — passend zu candle.
      4. `arrow`/`parquet` auf identische Major (60.x) festnageln.
      5. `Cargo.lock` committen (reproduzierbare Builds).

      ### 5.2 cudarc-Dublettenprüfung (höchste Priorität)

      1. `cargo tree -i cudarc` und `cargo tree -d` ausführen → **genau eine** cudarc-Version im Baum.
      2. Klärung **[?]** 0.19.9 vs. 0.19.10: aufgelöste Version aus `Cargo.lock` dokumentieren.
      3. **Direkte** cudarc-Nutzung vermeiden; wenn nötig, prüfen ob candle `cudarc` re-exportiert
         (`candle_core::cuda_backend::cudarc`, **[?]** ungeprüft) oder exaktes Pinning
         (`cudarc = "=0.19.8"`) mit identischen Features + `cargo tree -d`.

      ### 5.3 CUDA-Toolkit/Treiber festlegen

      1. CUDA **12.x** wählen (nicht 13.x; Turing bleibt unterstützt, aber 12.x ist sicherste Wahl).
      2. Build-Umgebung: `CUDA_COMPUTE_CAP=75` setzen; candle nutzt
         `cuda-version-from-build-system` + `dynamic-linking`.
      3. Dokumentieren: bf16-Kernel entfallen → in D4/D6 nur f16/f32 einplanen.

      ### 5.4 Roundtrip-/Kompatibilitätstests (Minimalnachweise)

      1. **parquet f16** **[?]**: Roundtrip-Test `FixedSizeList<Float16, 384>` schreiben+lesen in
         parquet 60.0.0; mmap-Strategie prüfen (`memmap2` → `bytes::Bytes` →
         `ParquetRecordBatchReaderBuilder::try_new`). Echtes Zero-Copy nur bei unkomprimiert.
      2. **tokenizers** **[?]**: `Tokenizer::from_file` auf das XLM-R-`tokenizer.json`
         (Unigram/Metaspace) mit `default-features = false` — `encode`/`get_ids`/`get_attention_mask`
         funktionieren ohne `onig`.
      3. **BertModel/e5-small** **[V-Vorprüfung]**: `config.json` (model_type `bert`, hidden 384,
         12 Layer/Heads, `gelu`, vocab 250037) passt zu candle-`BertModel::load`. Verifizieren, dass
         `model.safetensors`-Tensornamen zur candle-Präfixlogik passen (ohne/mit `bert.`-Prefix).
      4. **safetensors mmap**: `VarBuilder::from_mmaped_safetensors` lädt (unsafe); Zero-Copy nur CPU,
         Kopie beim GPU-Transfer.

      ### 5.5 Restrisiken dokumentieren

      1. thiserror 1→2 Migrationsnotizen **[?]** aus dem Changelog nachziehen (Format-Syntax,
         Raw-Identifier `{r#type}`).
      2. arrow/parquet: schnelle Majors (~1–3 Monate) → Upgrade-Disziplin (Majors synchron ziehen).
      3. hf-hub 1.0.0 (candle-Workspace): falls direkt genutzt, gleiche Major wählen.

      ## 6. Zu erzeugende Artefakte

      - Gepinnte `Cargo.toml`(s) + committetes `Cargo.lock`.
      - `rag/docs/architecture/dependency_verification.md`: verifizierte Liste (mit [V]/[W]/[?]),
        `cargo tree`-Ausgaben, Toolkit-Entscheidung, Testresultate der Roundtrips, Risikoliste.
      - Minimale Verifikationstests (parquet-f16-Roundtrip, tokenizer-load) als `#[ignore]`- oder
        Feature-gaterte Tests im Workspace.

      ## 7. Offene Punkte / Verifikation vor Abschluss

      Die priorisierten offenen Punkte aus `context_a.md` müssen **alle** einen Status haben:

      1. cudarc 0.19.9 vs. 0.19.10 + Re-Export → aufgelöst via `cargo tree -i cudarc`.
      2. parquet-f16-Roundtrip + mmap-Strategie → Test grün oder Workaround dokumentiert.
      3. CUDA-Toolkit-Version (12.x) + `CUDA_COMPUTE_CAP=75` → im Build gesetzt.
      4. tokenizers `default-features = false` gegen XLM-R-`tokenizer.json` → Test grün.
      5. thiserror-2.0-Migrationsnotizen → nachgezogen.

      ## 8. Akzeptanzkriterien (Definition of Done)

      1. `cargo tree -d` zeigt keine problematischen Duplikate (insb. nur **eine** cudarc-Version).
      2. parquet-f16-Roundtrip und tokenizer-Load bestehen (oder dokumentierter, abgenommener Workaround).
      3. Toolkit-/Compute-Cap-Entscheidung im Build verankert (12.x, `CUDA_COMPUTE_CAP=75`).
      4. Verifikations-/Risikodokument vollständig; jeder [?]-Punkt aus context_a.md aufgelöst oder als
         Restrisiko mit Mitigationsplan markiert.

      ## 9. Querschnitt (Q1–Q4)

      - **Q3:** Roundtrip-/Load-Tests als GPU-freie bzw. `#[ignore]`-Tests (CI-tauglich).
      - **Q4:** Toolkit-/Feature-Entscheidungen sind Voraussetzung für die `cuda`-Feature-Pfade aus A2.
      - **Q1/Q2:** Fehlerfälle beim Laden (safetensors/tokenizer/parquet) fließen als Anforderungen in
        die A4-Fehler-Taxonomie ein (`Incompatible`, `TransferFailed`).
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a4.md
    mode: replace
    content: |
      # Umsetzungsplan A4 — Querschnittskonventionen festlegen

      > Teil von Gruppe A (Fundament & Kontext).
      > Quellen: `resident_planplan.md` (Paket A4 + Querschnitt Q1–Q4), `context_a.md` (Paket A4
      > Nr. 16–18), Fehler-/Logging-Abschnitte aller Dokumente (`resident.md`, `resident_index.md`,
      > `resident_processor.md`, `resident_precision.md`).

      ## 1. Ziel & Ergebnistyp

      Einheitliche Konventionen für **Fehler** (`thiserror`), **Logging** (`tracing`) und
      **Test/Mocks** festlegen, dazu passendes **Basis-Scaffolding** anlegen. Das Dokument speist die
      Querschnitte Q1–Q4 und wird je Gruppe fortgeschrieben.

      **Ergebnistyp:** Konventionsdokument + minimales, kompilierendes Basis-Scaffolding
      (`error.rs`, `telemetry.rs`, Mock-Konvention, Feature-Flag-Konvention).

      ## 2. Eingaben & Quellen

      - `context_a.md` Paket A4 Nr. 16–18 (alle **[W]**): Fehler-Taxonomie, tracing-Spans, Traits+Mocking.
      - `resident_planplan.md` Querschnitt **Q1** (konkrete Fehlervarianten-Liste), **Q2**, **Q3**, **Q4**.
      - Fehler-/Logging-Hinweise der Fachdokumente (z. B. `resident_index.md` → `UnsupportedScope`,
        `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` (intern), `TransferFailed`;
        `resident.md` → Init-/Launch-Fehler + Backoff + Logging mit Task-ID).
      - A2-Modulstruktur (Zielmodule `error.rs`/`telemetry.rs`, Crate-Grenzen).

      ## 3. Abhängigkeiten

      - **Abhängt von:** A2.
      - **Blockiert / speist:** alle Gruppen (Q1–Q4 begleiten durchgehend); unmittelbar relevant für
        C2 (Backpressure/Fehler), E (Sync-Fehler), F (Kalibrierungsfehler).

      ## 4. Vorannahmen / verifizierter Stand (context_a.md)

      - thiserror **2.0.20**, tracing **0.1.44**, tracing-subscriber **0.3.23**, mockall **0.15.0** **[V]**.
      - Konventionsempfehlungen **[W]** (nicht gegen Primärquellen geprüft) — vor Festschreibung kurz
        gegen aktuelle Crate-Doku gegenchecken (thiserror 2.0 Format-Syntax, tracing-instrument).

      ## 5. Arbeitsschritte (für den Agenten)

      ### 5.1 Q1 — Fehler-Taxonomie (`thiserror`)

      1. **Ein Fehler-Enum pro Schicht** (z. B. `AdapterError`, `OrchestratorError`, `ManagerError`,
         `CalibrationError`); obere Schichten wrappen untere mit `#[from]`/`#[source]`.
      2. Öffentliche Enums `#[non_exhaustive]`; alle Fehler **`Send + Sync + 'static`** (für
         `async`-Grenzen/`JoinHandle`/Channels). Kontext (z. B. `task_id`) als **Felder**, nicht String.
      3. **Gemeinsame Taxonomie-Basis** verankern (aus Q1), zunächst als Varianten/Platzhalter, in den
         jeweiligen Gruppen zu füllen:
         `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` (intern),
         `TransferFailed`, `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision`,
         plus Planner-/Config-/Capability-Fehler.
      4. **Retry-Klassifikation:** `is_retryable()` — OOM/kritisch vs. wiederholbar trennen
         (für C2-Backoff). Worker-Pool liefert `Result<T, E>` per Channel, nie `panic`; Panics via
         `JoinError` abfangen. `anyhow` nur in Binary/`main`.

      ### 5.2 Q2 — Logging (`tracing`)

      1. **Span-Granularität:** ein Span **pro Task** (Feld `task_id`), Child-Spans pro
         Verarbeitungszyklus/Batch und **pro Manager-Zyklus**; Ressource (Device/Index) als **Feld**,
         nicht als eigener Span.
      2. **Async-Regel:** `#[tracing::instrument(skip_all, fields(task_id = %id))]` bzw.
         `.instrument(span)` auf Futures; **kein** `span.enter()` über `.await`.
      3. **Thread/`spawn_blocking`-Regel:** Span klonen, im Closure `span.in_scope(...)` /
         `let _g = span.enter();` (Kontext propagiert nicht automatisch). GPU-Aufrufe über
         dedizierten Thread/`spawn_blocking`, nicht als async-Task.
      4. **Subscriber:** `tracing-subscriber` mit `EnvFilter`; hochfrequente Kernel-Level-Events auf
         `trace!`. Definieren, welche Ereignisse je Gruppe geloggt werden (z. B. Manager:
         Scan-Ergebnis, Lade-/Eviction-Entscheidung, verworfene/`outdated`-Ergebnisse, Arena-Auslastung).

      ### 5.3 Q3 — Teststrategie & Mocks

      1. **Schmale Traits** für GPU-/Mess-Schnittstellen (z. B. `Kernel` aus B5, Mess-Trait aus F2);
         Generics (statischer Dispatch) im Hot-Path, `dyn` nur an Rändern.
      2. **Mock-Strategie:** `mockall` (`#[automock]`) für Orchestrator-Tests; für deterministische
         Fakes (fester Embedding-Output, simulierter Fehler) handgeschriebene `FakeGpu`/`FakeKernel`
         (einfacher bei async/`Send`). Sichtbarkeit `#[cfg(any(test, feature = "mock"))]`.
      3. **Testmatrix-Vorgaben** je Paket referenzieren (B1 Queue-Semantik, E5/E6 Sync, F2–F5
         Kalibrierung) — als Konvention, dass jedes Paket seine Matrix mitliefert.
      4. **No-GPU-Pfad:** GPU-Integrationstests mit `#[ignore]` oder eigenem Feature; CI ohne GPU bleibt grün.

      ### 5.4 Q4 — Feature-Flags

      1. Konvention aus A2 übernehmen und festschreiben: reale CUDA-Pfade hinter `cuda`,
         Backend-Auswahl per `#[cfg]`:
         `#[cfg(feature = "cuda")] type DefaultBackend = CudaBackend;`
         `#[cfg(not(feature = "cuda"))] type DefaultBackend = CpuBackend;`
      2. `cuda` nie in `default`; `dep:`-Syntax für optionale GPU-Crates.

      ### 5.5 Basis-Scaffolding anlegen

      1. `resident-core/src/error.rs`: Taxonomie-Basis + Trait/Helfer `is_retryable()`, Re-Export in
         `lib.rs`.
      2. `resident-core/src/telemetry.rs`: Span-Helfer + Subscriber-Init (`EnvFilter`), Feld-Namen
         (`task_id`, `resource`) als Konstanten.
      3. Mock-Konventionsstub + `mock`-Feature in `Cargo.toml` verdrahten.

      ## 6. Zu erzeugende Artefakte

      - `rag/docs/architecture/conventions.md` (Q1–Q4 vollständig, mit den Regeln aus 5.1–5.4).
      - `rag/crates/resident-core/src/error.rs` + `telemetry.rs` (kompilierend, leere/Platzhalter-Varianten).
      - `mock`-Feature in `resident-core/Cargo.toml`.

      ## 7. Offene Punkte / Verifikation vor Abschluss

      - **[W]→verifizieren:** thiserror-2.0 Format-Syntax (Raw-Identifier `{r#type}`, Display-Bounds)
        gegen aktuelle Doku; `#[tracing::instrument]`-Signatur gegen tracing 0.1.44.
      - Prüfen, dass alle Q1-Varianten aus dem Metaplan im `error.rs`-Basisgerüst **adressiert** sind
        (als Variante oder dokumentierter TODO der Zielgruppe).
      - Gegencheck mit A3: Lade-/Kompatibilitätsfehler (`Incompatible`, `TransferFailed`) sind abgedeckt.

      ## 8. Akzeptanzkriterien (Definition of Done)

      1. `conventions.md` deckt Q1–Q4 vollständig ab (Fehler, Logging, Test/Mocks, Feature-Flags).
      2. `error.rs` + `telemetry.rs` kompilieren im GPU-freien Build (`--no-default-features`).
      3. Alle Q1-Fehlervarianten aus dem Metaplan sind verankert (Variante oder zugewiesener TODO).
      4. Mock-Konvention + `mock`-Feature funktionieren (Beispiel-Fake kompiliert außerhalb von `#[test]`).

      ## 9. Querschnitt (Q1–Q4)

      - Dieses Paket **ist** die initiale Fixierung von Q1–Q4; nachfolgende Gruppen schreiben die
        Taxonomie, Logging-Ereignisse und Testmatrizen in ihren Modulen fort.
  reason: "Erstellen der Umsetzungspläne (Stufe 2) für Gruppe A: Übersichts-/Indexdatei plus je ein Paketplan für A1′ (Scope-Charta), A2 (Projekt-/Modulstruktur), A3 (Stack-/Versionskompatibilität) und A4 (Querschnittskonventionen), gegründet auf resident_planplan.md und context_a.md."
```
/answer 0f9c-b allow
Result Stats: id=0135fe34-9ed9-4412-8c58-3a67ef879c9a, total: 92965, in: 121, out: 24564, reason: 5598, read: 45819, write: 68280
ReasoningToken: 5583
ReasoningToken: 15
Fertig. Ich habe die Umsetzungspläne für Gruppe A in `/home/user/xyan/xy.ai.workbench/project/resident/plan` erstellt.

## Erstellte Dateien

| Datei | Inhalt |
|-------|--------|
| `plan_a_index.md` | Übersicht: Zielumgebung (GTX 1660/sm_75, e5-small, CUDA 12.x), Reihenfolge A1→A2→A3→A4, Querschnitt Q1–Q4, Gesamt-DoD |
| `plan_a1.md` | **A1′ Scope-Charta** (ersetzt das erledigte „These destillieren") – Grenze Engine ↔ Suchschicht, Status K/I/A/E/—, Hook-Verpflichtungen |
| `plan_a2.md` | **A2 Projekt-/Modulstruktur** – Workspace-Schnitt (`resident-core`/`resident-cuda`/`resident`), Modulbaum entlang B–G, Feature-Matrix, No-GPU-CI |
| `plan_a3.md` | **A3 Stack-/Versionskompatibilität** – gepinnte Versionen, cudarc-Dublettenprüfung, parquet-f16/tokenizer-Roundtrips, Risikoliste |
| `plan_a4.md` | **A4 Querschnittskonventionen** – Fehler-Taxonomie, tracing-Spans, Test/Mocks, Feature-Flags + Basis-Scaffolding (`error.rs`/`telemetry.rs`) |

## Wichtige Entscheidungen, auf die ich dich hinweisen will

1. **A1 → A1′:** `resident_planplan.md` erklärt A1 („These destillieren") ausdrücklich für **erledigt** und ersetzt es durch **A1′ (Scope-Charta)**. Ich habe deshalb A1′ geplant, nicht die These-Destillation. Falls du doch ein separates Plan-Artefakt für die ursprüngliche A1-Formulierung willst, sag Bescheid.

2. **Grundlage:** Alle Pläne sind an den verifizierten Fakten aus `context_a.md` (Stand 2026-10-10) verankert – inkl. der Herkunfts-Markierungen **[V]/[W]/[?]**, damit die offenen Punkte (cudarc 0.19.9↔0.19.10, parquet-f16, tokenizers-`default-features`, CUDA-Toolkit, thiserror-2.0) explizit als Verifikationsschritte in A3/A4 auftauchen.

3. **Code-Artefakte** habe ich konsequent nach `/home/user/xyan/xy.ai.workbench/rag` (bzw. `rag/docs/architecture/`) verortet, da das Plan-Verzeichnis nur die Pläne enthalten soll. Die konkreten Pfade sind als Empfehlung markiert.

Soll ich als Nächstes die Pläne für **Gruppe B** (Kern-Datentypen, `context_b.md`) erstellen?
