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

Erstelle eine Liste aller in `resident_planplan.md` für Gruppe A zu beantwortenden Fragen für eine Web-Recherche. Ein anderer Agent wird diese Fragen recherchieren, beantworten und ein Markdown Kontext File als Ergebnis liefern.
SystemInit:  id=e89eb03f-5160-4194-a739-a2b0f964344b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 Ich lese zunächst alle Dokumente in dem Verzeichnis ein.
Control Request:
```yaml
id: 0f9c-3
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
  reason: "Einlesen aller Projektdokumente im resident-Verzeichnis, um den Kontext zu verstehen und die Gruppe-A-Fragen aus resident_planplan.md zu extrahieren."
```
/answer 0f9c-3 allow
Result Stats: id=e89eb03f-5160-4194-a739-a2b0f964344b, total: 59407, in: 119, out: 6653, reason: 2629, read: 1503, write: 52635
ReasoningToken: 76
ReasoningToken: 2553
Ich habe alle acht Dokumente eingelesen. Nachfolgend die aufbereitete Fragenliste **für Gruppe A (Fundament & Kontext)** aus `resident_planplan.md`, formuliert als Web-Recherche-Auftrag für den nachgelagerten Agenten.

Ich habe die „Zu klären"-Punkte der Pakete A1–A4 in konkrete, eigenständig recherchierbare Fragen übersetzt. Projekt-interne Prüfungen (die nicht per Web beantwortbar sind) sind am Ende separat ausgewiesen, damit der Recherche-Agent sie nicht fälschlich bearbeitet.

---

# Web-Recherche-Auftrag — Gruppe A (Fundament & Kontext)

## Kontext für den Recherche-Agenten (bitte beim Antworten berücksichtigen)

- **Zieldatum der Gültigkeit:** 2026-10-10 — bitte jeweils die zu diesem Zeitpunkt **aktuellen stabilen Crate-Versionen** und deren Kompatibilität ermitteln, mit Versionsnummer und Quelle/Datum.
- **Zielhardware:** NVIDIA GTX 1660 (Turing, **sm_75**, 6 GB VRAM).
- **Zielmodell:** `intfloat/multilingual-e5-small` (BERT-Architektur mit XLM-R-Tokenizer, 384 Dim, mehrsprachig inkl. Deutsch).
- **Stack laut Konzept:** `candle` / `candle-core` (Feature `cuda`), `cudarc`, `candle-transformers`, `tokenizers`, `safetensors`, `parquet` (+mmap), `tokio`, `crossbeam`/`parking_lot`, `arc-swap`, `thiserror`, `tracing`.
- **Ergebnisformat:** Markdown-Kontextdatei, pro Frage mit Antwort, Versionsangabe, Quelle und Verifizierungsdatum; Unsicherheiten explizit als solche markieren.

---

## Paket A2 — Projekt- & Modulstruktur / Feature-Flags

1. Wie strukturiert man einen Rust-Workspace/Crate so, dass GPU/CUDA-Pfade **hinter einem `cuda`-Cargo-Feature-Flag** liegen und die Kernlogik **ohne GPU** kompiliert und getestet werden kann? (Idiome für optionale Dependencies, `#[cfg(feature = "cuda")]`, Dummy-/Mock-Backends.)
2. Wie funktioniert das `cuda`-Feature von `candle-core` konkret (Aktivierung, transitiv aktivierte Dependencies wie `cudarc`), und wie isoliert man dieses so, dass ein No-GPU-Build möglich bleibt?
3. Bewährte Muster für **schichtbasierte Modulgrenzen** in größeren async-Rust-Projekten (Fassade / Orchestrator / reine Funktionsschicht / Ressourcen-Adapter): empfohlene Crate-vs-Modul-Aufteilung, Sichtbarkeits-/Re-Export-Konventionen.

## Paket A3 — Stack- & Versionskompatibilität (Hauptblock der Web-Recherche)

4. Welche **`cudarc`-Version** zieht die aktuelle `candle` / `candle-core` (mit `cuda`-Feature) transitiv herein? Wie stellt man sicher, dass eine im Projekt direkt verwendete `cudarc`-Version **exakt** mit der von Candle verwendeten übereinstimmt (Typinkompatibilität vermeiden)?
5. Welche **CUDA-Toolkit-/Treiberversionen** werden von der aktuellen Candle-/`cudarc`-Kombination unterstützt, und ist die **GTX 1660 (sm_75, Turing)** damit nutzbar (inkl. f16-Support, `dp4a`/int8, Acquire-Load `ld.acquire` ab sm_70)?
6. Aktuelle stabile Version und relevante API des **`tokenizers`**-Crates: Laden von `tokenizer.json`, Abruf der Attention-Maske (`get_attention_mask()`), Padding/Truncation, Batch-Encoding, Ausgabe von `u32`-IDs.
7. Unterstützt das **`parquet`**-Crate (bzw. `arrow`/`arrow-rs`) **memory-mapped (mmap) Lesezugriff**, und wie liest/schreibt man eine `FixedSizeList<f16, 384>`-Spalte? Gibt es vollständigen **f16/half-float**-Support in Arrow/Parquet (Rust)?
8. Aktuelle stabile Version von **`tokio`** und die für eine Executor-/Worker-Pool-Architektur relevanten Features (Multi-Thread-Runtime, `spawn_blocking`/Blocking-Pool, `sync::watch`, `sync::Notify`, `OnceCell`).
9. Vergleich **`crossbeam`** (insb. `crossbeam-queue`) vs. **`parking_lot`** für einen Lock-free Ring-Buffer bzw. eine CAS-basierte verkettete Liste: aktuelle Versionen, jeweilige Eignung und Einschränkungen.
10. Aktuelle Version und API von **`arc-swap`** (`ArcSwapOption`): CAS-Semantik, Nutzung für Append/Unlink in einer einfach verketteten Lock-free-Liste, Speicherfreigabe-Verhalten (als Ersatz für `crossbeam-epoch`).
11. Aktuelle Version des **`safetensors`**-Crates: mmap/Zero-Copy-Laden von Gewichten, Kompatibilität mit Candle-Tensor-Laden.
12. Stellt **`candle-transformers`** einen `BertModel` bereit, der zum Laden von `multilingual-e5-small` (XLM-R-Tokenizer, aber BERT-Architektur) geeignet ist? Welche **Gewichtsnamen-/Layerbezeichnungskonventionen** erwartet dieser, und wie weicht das von einem `xlm_roberta`-Modell ab?
13. Wird **`intfloat/multilingual-e5-small`** auf Hugging Face als `model.safetensors` verteilt, oder liegt nur `pytorch_model.bin` vor (→ Konvertierungsbedarf)? Welche Dateien (`config.json`, `tokenizer.json`) sind vorhanden?
14. Aktuelle stabile Versionen von **`thiserror`** und **`tracing`** (inkl. `tracing-subscriber`) und etwaige Breaking-Changes/Migrationshinweise der jeweils letzten Major-Versionen.
15. Gibt es bekannte **Versionskonflikte/Inkompatibilitäten** zwischen den oben genannten Crates (z. B. `candle` ↔ `cudarc` ↔ CUDA-Toolkit, Arrow-/Parquet-Major-Versionen), die vorab als Risiko zu dokumentieren sind?

## Paket A4 — Querschnittskonventionen (Fehler, Logging, Tests/Mocks)

16. Best Practices für eine **Fehler-Taxonomie mit `thiserror`** in einem geschichteten, nebenläufigen System: Fehler-Enums pro Schicht vs. zentral, Fehler-Wrapping/`#[from]`, Umgang mit Fehlern über `async`-Grenzen und in Worker-Pools.
17. Best Practices für **`tracing`-Span-Granularität** (Spans pro Task-ID / pro Ressource / pro Verarbeitungszyklus), korrektes Instrumentieren von async-Code und Thread-übergreifendes Span-Kontext-Propagieren.
18. Bewährte Muster für **Trait-Abstraktionen + Mocking** in Rust zum Testen ohne GPU: Eignung von `mockall` o. Ä., Abstraktion von Kernel-/Mess-Schnittstellen hinter Traits, Feature-Flag-gesteuerte Mock- vs. Real-Implementierung.

---

## Nicht für Web-Recherche geeignet (projekt-intern — bitte NICHT recherchieren)

Diese „Zu klären"-Punkte aus Gruppe A sind nur aus dem Projektkontext/den lokalen Dateien beantwortbar und gehören nicht in die Web-Recherche:

- **A1 (These destillieren):** Laut `resident_planplan.md` **bereits aufgelöst** (Abgleich These↔Plan liegt am Dokumentende vor). Die referenzierte Datei `/home/user/xyan/xy.ai.workbench/docs/rag/resident_streaming.md` ist eine **lokale, fehlende Datei** — nicht per Web beschaffbar. Ersatzpaket **A1′ (Scope-Grenze Engine ↔ Suchschicht)** ist eine rein interne Dokumentationsaufgabe.
- **A2:** Endgültiger Crate-/Modulschnitt und die konkrete Feature-Matrix des Projekts (Entwurfsentscheidung, keine Recherche — Frage 1–3 liefern nur die *Idiome* dazu).
- **A3:** Tatsächliches Vorhandensein von `model.safetensors` **im lokalen Repo** (vs. Frage 13, die die **Verfügbarkeit auf Hugging Face** klärt).
- **A4:** Festlegung der konkreten, projektspezifischen Fehler-Varianten und Span-Namen (Entscheidung, keine Recherche).

---

Soll ich diese Liste als eigenständige Markdown-Datei (z. B. `resident_groupA_research.md`) im Projektordner ablegen, oder möchtest du vorab noch Fragen ergänzen/streichen?
