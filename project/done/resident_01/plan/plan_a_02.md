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
