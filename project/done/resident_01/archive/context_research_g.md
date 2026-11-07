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

Erstelle eine Liste aller in `resident_planplan.md` für Gruppe G zu beantwortenden Fragen für eine Web-Recherche. Ein anderer Agent wird diese Fragen recherchieren, beantworten und ein Markdown Kontext File als Ergebnis liefern.
SystemInit:  id=20599445-7643-4274-9e7b-53c1922eea0f, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 Ich lese zunächst alle Dokumente in dem Verzeichnis ein.
Control Request:
```yaml
id: 0f9c-8
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
  reason: "Einlesen aller Dokumente im resident-Projektverzeichnis, um den Kontext zu verstehen und die für Gruppe G zu beantwortenden Web-Recherche-Fragen aus resident_planplan.md zu extrahieren."
```
/answer 0f9c-8 allow
Result Stats: id=20599445-7643-4274-9e7b-53c1922eea0f, total: 57572, in: 119, out: 5819, reason: 2235, read: 2836, write: 51634
ReasoningToken: 31
ReasoningToken: 2204
Ich habe alle acht Dokumente eingelesen. Für Gruppe G (Konkrete Modell-/Index-Integration) sind die Pakete **G1**, **G2** und **G3** relevant. Die zentrale Quelle ist `resident_first.md` (Modell-Notizen mit `[prüfen]`-Markierungen). Daraus und aus den „Zu klären"-Punkten im Metaplan habe ich alle **web-recherchierbaren Fragen** destilliert — also solche, die durch externe Fakten (HuggingFace, candle, sentence-transformers, Arrow/Parquet) beantwortbar sind, nicht durch interne Designentscheidungen.

---

# Web-Recherche-Auftrag: Gruppe G — Konkrete Modell-/Index-Integration

**Kontext für den recherchierenden Agenten:** Es geht um eine Rust-RAG-Engine („Resident") auf Basis des Stacks `candle`/`candle-transformers`, `safetensors`, `tokenizers`, `parquet`/Arrow, `tokio`. Zielmodell ist `intfloat/multilingual-e5-small` (dense Bi-Encoder). Alle Antworten sollen mit **Quellenangabe (URL)** und, wo möglich, **Codebeleg** belegt werden. Unverifizierte Angaben sind als solche zu kennzeichnen. Ergebnis: ein Markdown-Kontext-File.

---

## G1 — Modell `intfloat/multilingual-e5-small` & Pipeline-Konfiguration

**Modell-Grunddaten & Dateien**
1. Welche Dateien liegen im HuggingFace-Repo `intfloat/multilingual-e5-small` vor? Ist eine `model.safetensors` **direkt vorhanden**, oder liegt nur `pytorch_model.bin` vor (→ Konvertierung nötig)? Sind `tokenizer.json` und `config.json` vorhanden?
2. Bestätige die Architektur-Eckdaten: **118M Parameter**, **384 Embedding-Dimensionen**, XLM-R-Vokabular (~250k Tokens), mehrsprachig (inkl. Deutsch). Was steht konkret in `config.json` (`hidden_size`, `vocab_size`, `max_position_embeddings`, `model_type`)?
3. Bestätige: Das Modell ist architektonisch ein **BERT** mit XLM-R-Tokenizer. Ist daher in `candle-transformers` die Klasse `BertModel` (nicht `xlm_roberta`) zu verwenden? Welche candle-Beispiele/Referenzen nutzen dieses Modell?

**Gewichtskompatibilität mit candle**
4. Welche **Tensor-/Gewichtsnamen** (Key-Schema) verwendet die `model.safetensors` von e5-small, und sind diese **kompatibel mit dem `BertModel` in `candle-transformers`** (Namenskonvention, Präfixe wie `embeddings.`, `encoder.layer.`, Pooler vorhanden/ignoriert)? Ist ein Remapping der Keys nötig?
5. Gibt es bekannte Fallstricke beim Laden von XLM-R-basierten BERT-Gewichten in candle (z. B. `position_embedding_type`, `token_type_embeddings`, fehlender Pooler)?

**Pipeline: Präfixe, Pooling, Normalisierung**
6. Was sind die **exakten, offiziell geforderten Präfix-Strings**? Bestätige `"query: "` und `"passage: "` (inkl. Leerzeichen) laut Modellkarte. Wie stark verschlechtert ein falsches/fehlendes Präfix die Qualität (Dokumentationshinweise)?
7. Wie lautet die **offizielle Mean-Pooling-Formel mit Attention-Maske** für e5 (Σ(h·mask)/Σ(mask))? Wie implementiert es `sentence-transformers` genau (Referenz-Code), um einen Abgleich der Rust-Implementierung zu ermöglichen?
8. Bestätige: Nach dem Mean-Pooling folgt **L2-Normalisierung**, danach genügt das Skalarprodukt als Cosine-Similarity. Wie macht das die Referenz (sentence-transformers / `intfloat`-Beispielcode)?

**Sequenzlänge & Präzision**
9. Was ist die **maximale Sequenzlänge** des Modells (`max_position_embeddings`)? Bestätige 512 als Maximum. Gibt es offizielle Empfehlungen zur sinnvollen `max_seq_len` für Sätze (Plan nennt 64–128)?
10. Gibt es belastbare Angaben/Erfahrungswerte zur **f16-vs-f32-Toleranz** bei e5-small (GPU f16 beim Embedding vs. CPU f32)? Welche Cosine-Abweichung ist bei gemischter CPU/GPU-Indexierung zu erwarten, und welche Toleranzschwelle ist üblich/empfohlen?
11. Gibt es eine **offizielle Referenz-Testsuite / erwartete Beispiel-Embeddings** (z. B. STS-Benchmark-Werte, MTEB-Scores), gegen die man Mean-Pooling+L2 verifizieren kann?

---

## G2 — Index-Schema (Parquet / Arrow)

12. Unterstützt das Rust-`parquet`/`arrow`-Ökosystem einen **`FixedSizeList<Float16, 384>`**-Spaltentyp? Wie wird **f16 (Arrow `Float16`/`half::f16`)** in Parquet gespeichert und gelesen (Encoding, Kompatibilität, Reifegrad)?
13. Wie realisiert man **mmap-basierten, read-heavy Zugriff** auf Parquet-Dateien in Rust (geeignete Crates/APIs, Zero-Copy-Grenzen, Page-Fault-Verhalten)?
14. Welche **Checksum-Verfahren** (z. B. CRC32/xxHash/SHA) eignen sich zur Verifizierung der Payload (Zieldatei + Zeichenoffsets einer indizierten Zeile)? Was ist Stand der Praxis bzgl. Geschwindigkeit vs. Kollisionssicherheit?
15. Best Practices für **Schema-Versionierung** in Parquet (Modell-ID, Dim, Pooling, Präfixe, dtype als Metadaten): Wo/wie werden solche Header-/Schema-Metadaten in Parquet abgelegt (`key_value_metadata`)?

---

## G3 — Similarity / Scorer-Konfiguration

16. Bestätige die mathematische Äquivalenz: Bei **L2-normalisierten Vektoren** entspricht die Cosine-Similarity dem **Skalarprodukt**. Gibt es numerische Fallstricke (f16-Rundung, Normierungstoleranz)?
17. Welche etablierten Verfahren/Metriken gibt es für die vom Plan geforderten **Entscheidungsgrundlagen** einer Top-k-Ergebnismenge — insbesondere **Score-Abstand Top-1 zu Top-k** und **Streuung/Verteilung** der Scores — zur Steuerung eines optionalen Rerank-Schritts durch die äußere Schicht?
18. (Wechselpfad-Kontext, optional) Falls später ein Cross-Encoder-Reranker gewünscht ist: Welche Modelle (z. B. `BAAI/bge-reranker-v2-m3`) sind gängig, und wie unterscheidet sich deren Scoring-API grundsätzlich vom Bi-Encoder-Cosine-Pfad? (Nur zur Einordnung — in der aktuellen Umsetzung nicht vorgesehen.)

---

**Hinweise zur Abgrenzung (nicht Teil der Web-Recherche, da interne Designentscheidungen):**
- Persistenz-Verantwortung (Index-Objekt besitzt Mutex/Persistenz) und „Modellwechsel = neue Schema-Version + Re-Embedding" sind **Architekturfestlegungen** des Projekts, keine Rechercheaufgaben.
- Dass Rerank kein eigener Tasktyp ist (nur Similarity mit anderem Scorer auf Subset) ist eine **interne Vorgabe**.

Soll ich diese Liste als eigenständiges Markdown-Datei (z. B. `resident_G_research_brief.md`) im `resident`-Ordner ablegen, damit der recherchierende Agent sie direkt als Eingabe nutzen kann?
