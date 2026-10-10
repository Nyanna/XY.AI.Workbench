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
