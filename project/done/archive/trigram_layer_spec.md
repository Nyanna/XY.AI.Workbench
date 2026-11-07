# Spezifikation: Trigramm-Layer

Das Dokument besteht aus zwei Teilen:

- **Teil A:** Verhalten des Layers (Pipeline, Normalisierung, Signatur, Index, Suchlauf).
- **Teil B:** Datenstruktur für Trigramm-Mengen (`CompactTrie`, `CellMask`, `QueryMask`), von Teil A genutzt.

## 1. Ziel

Ein Layer, der über die Query ein Textmuster in Dateien unterhalb eines Verzeichnisses sucht. Ein Trigramm-Index (Signatur pro Datei) schränkt vorab ein, welche Dateien tatsächlich durchsucht werden. Gematcht wird auf einer zeilentreuen, normalisierten Kopie der Datei.

**Referenzimplementierungen** (Struktur und Konventionen übernehmen):

| Zweck | Pfad |
|---|---|
| Layer-Vorbilder | `/home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs`, `.../glob_layer.rs` |
| Multithreading | `/home/user/xyan/xy.ai.workbench/rag/src/core/executor.rs` |
| Verzeichnis-Cache / rekursiver Lauf | `/home/user/xyan/xy.ai.workbench/rag/src/layers/dir_cache.rs` |

## 2. Ein- und Ausgabe

### Eingabe (Query)
- `Query`: Suchtext. Wird normalisiert (Abschnitt 4) und in eine Signatur umgewandelt (Abschnitt 6).
- `directory` (optional): Startverzeichnis der rekursiven Suche. Fallback auf den Document Root ist bereits vorhanden.

### Ausgabe (Query-Result-Objekt)
Pro Treffer-Datei ein Eintrag:

- `File`: Dateipfad relativ zum Document Root.
- `Lines`: assoziative, nach Zeilennummer **sortierte** Liste `Zeilennummer → Text der matchenden Zeile`.
  - Mehrere Treffer in derselben Zeile ergeben nur **einen** Eintrag.
  - Der Text ist die **Originalzeile** aus der Datei, nicht die normalisierte.

## 3. Ablauf eines Suchlaufs

Jede eintreffende Query startet einen eigenen Suchlauf:

1. **Query vorbereiten:** normalisieren, in Trigramm-Signatur umwandeln, `QueryMask` bauen.
2. **Rekursiver Lauf** über `dir_cache.rs` ab `directory` (bzw. Document Root). Pro Datei:
   - Größe > 5 MB: überspringen (weder indizieren noch durchsuchen).
   - Index-Eintrag fehlt oder `mtime` weicht ab: Datei neu indizieren (Abschnitt 7).
   - Index-Eintrag vorhanden, Datei existiert nicht mehr: Eintrag aus dem Index entfernen.
3. **Signaturabgleich:** `matches_at_least(file_mask, query_mask, t)`. Trifft die Signatur ausreichend stark zu, wird eine Dateisuche gestartet.
4. **Dateisuche (ein Task pro Datei):** Die normalisierte Query wird in der gespiegelten, normalisierten Datei gesucht. Die Trefferzeilen werden über die Zeilennummer auf die Originaldatei abgebildet (Newlines sind in der Normalisierung erhalten).
5. **Ergebnis** aus allen Dateisuchen zusammenführen.

**Schwelle `t`:** Die Spezifikation sagt nur „ausreichend stark". Default: konfigurierbares `min_match_ratio` mit Standardwert `0.8`. `t = ceil(ratio * n_keys)`, danach `corrected_threshold(t)`. Siehe Abschnitt 9, Punkt 1.

### Laufzeitlimits und Nebenläufigkeit
- Jede Dateisuche ist ein Task im Executor (`core/executor.rs`).
- Maximallaufzeit eines Suchlaufs: **1 Minute**.
- Ab **90 %** der Maximallaufzeit werden keine neuen Dateisuchen mehr gestartet.
- Bei Erreichen der Maximallaufzeit werden laufende Dateisuchen abgebrochen.

## 4. Normalisierung

Eingabe ist Dateitext (und identisch die Query). Ausgabe ist die normalisierte Form. Schritte in dieser Reihenfolge:

1. **Unicode-Normalisierung** (NFC/NFD/NFKC/NFKD). Default: NFKD, danach Combining Marks entfernen (siehe 9.3).
2. **Zeichen reduzieren:** Alles, was kein englisches alphanumerisches Zeichen ist (`[A-Za-z0-9]`), wird
   - zu einem Leerzeichen (Code und Syntax, z. B. `_ . ( ) { } -`), oder
   - umgewandelt, wenn es ein Buchstabe einer anderen Sprache ist (Umlaute usw., z. B. `ä→a`, `ß→ss`).
3. **Newlines bleiben erhalten.** Sie garantieren das Mapping Zeile in normalisierter Form ↔ Zeile in der Datei.
4. **CamelCase splitten**, danach alles lowercase. Beispiel: `getUserName → get user name`. Akronyme (`HTTPServer → http server`) siehe 9.4.
5. **Whitespace zusammenziehen:** Mehrere Leerzeichen werden zu einem. Dieses eine Leerzeichen ist der **Token-Trenner** (Vokabulartrenner). Zeilen, die nur aus Whitespace bestehen, kollabieren zu einem leeren Zeilenwechsel (Newline bleibt).

### Persistenz der normalisierten Form
Die normalisierte Datei wird in einer **gespiegelten Verzeichnisstruktur** unterhalb des Storage-Verzeichnisses gespeichert. Sie dient der Zeilenassoziation und dem Treffer-Matching.

## 5. Vokabular

- Mapping `Trigramm-String → ID (u32)`.
- IDs werden **zufällig verteilt** vergeben (kollisionsfrei, im Bereich `[0, 2^key_bits)`), damit Trigramme gleichmäßig im Schlüsselraum und damit im Trie liegen.
- **Lazy:** unbekannte Trigramme bekommen bei erstem Auftreten eine neue ID.
- Persistenz als Datei im Storage-Verzeichnis. Beim Start wird sie geladen, sonst neu angelegt. Die Datei muss nach Änderungen geschrieben werden (atomar: temp-Datei + rename).
- Der Seed bzw. die Vergabe muss reproduzierbar sein, sobald die Datei existiert. Sie ist die Quelle der Wahrheit.
- Ist der ID Raum erschöpft, wird ein Fehler geworfen und abgebrochen

### Nebenläufigkeit des Vokabulars (lazy)

Das Vokabular wird nicht vorab erzeugt, sondern beim ersten Auftreten eines Trigramms gefüllt. Mehrere Suchläufe und Indexierungs-Tasks greifen gleichzeitig darauf zu.

- **Zugriff:** Das Vokabular liegt hinter einem `RwLock`. Der Normalfall (Trigramm bekannt) braucht nur den Lesezugriff.
- **Neue Trigramme:** Bei einem Miss wird der Schreibzugriff geholt und erneut geprüft, ob ein anderer Task das Trigramm inzwischen angelegt hat. Erst danach wird eine zufällige, noch freie ID im Bereich `[0, 2^key_bits)` vergeben.
- **Query-Seite vergibt keine IDs:** Unbekannte Trigramme einer Query kommen in keiner indizierten Datei vor. Sie werden verworfen, das Vokabular wird nicht durch Query-Text gefüllt. Bleibt kein Trigramm übrig, ist das Ergebnis leer.
- **Persistenz:** Neue Einträge werden gesammelt und in einem Schritt geschrieben (temp-Datei + atomares rename). Reihenfolge: **zuerst das Vokabular persistieren, dann die Index-Einträge schreiben, die die neuen IDs verwenden.** Sonst referenziert ein Index nach einem Absturz IDs, die im Vokabular fehlen.
- **Quelle der Wahrheit:** Existiert die Vokabular-Datei, wird sie geladen und nie neu gewürfelt. Gespeicherte Signaturen bleiben dadurch gültig.

## 6. Signatur

Erzeugt aus dem normalisierten Text einer Datei bzw. der normalisierten Query.

1. **Tokenisierung** am Leerzeichen.
2. **Trigramme mit Grenzmarkern** (kein Mindestlänge-Filter): Pro Token ein Marker `_` vorn und hinten, dann gleitendes Fenster der Länge 3.
   - `go → _go, go_`
   - `a → _a_`
   - `user → _us, use, ser, er_`
   - Ein Token der Länge L liefert genau L Trigramme, auch bei 1–2 Zeichen.
   - Wortanfang und -ende sind unterscheidbar (`_us ≠ us_`), das erhöht die Selektivität.
3. **Deduplizieren:** jedes Trigramm einmal pro Signatur.
4. **Häufige Trigramme entfernen:** Alle Trigramme der Datei zählen. Die häufigsten **10 %** (konfigurierbar) werden entfernt, aber nur solche, deren Zählwert **über 10** (konfigurierbar) liegt.
5. **ID-Mapping** über das Vokabular ergibt die Menge von `u32`-Schlüsseln. Diese ist Eingabe für `CompactTrie::build` (Datei) bzw. `QueryMask::new` (Query).

## 7. Index

- Pro Datei gespeichert: **Dateipfad**, **mtime**, **Signatur** (als `CompactTrie`-Payload ohne Header, gemeinsame `TrieParams` für den gesamten Index).
- Der Index liegt im Storage-Verzeichnis des Layers und ist persistent.
- **Aktualisierung während des Suchlaufs:**
  - Datei im Index, aber nicht mehr auf der Platte: Eintrag entfernen.
  - `mtime` im Index ≠ `mtime` der Datei: neu indizieren (normalisierte Spiegeldatei und Signatur neu schreiben).
  - Datei fehlt im Index: indizieren.
- Beim Laden wird jede Signatur zu einer `CellMask` aufgeklappt. Speicher: 8 KB pro Datei bei `b = 16`. Bei vielen Dateien ggf. LRU oder Lazy-Expand, siehe Teil B (`MaskTable`).

# Teil B: Datenstruktur für Trigramm-Mengen

Quantisierter Radix-4-Trie als Speicherform und Bitmaske als Memory-Form.

## 8.1 Ziel

Eine Datenstruktur für **Mengen ganzzahliger Schlüssel** (Trigramm-IDs, `u32`) mit

1. einer **kompakten, serialisierbaren Form** (Pre-Order-Radix-4-Trie, quantisiert) und
2. einer **In-Memory-Form als flache Bitmaske** (`u64`-Wörter), bei der der Abgleich gegen eine Anfrage nur aus `AND` + Popcount besteht.

Zweck: Pro Datei wird gespeichert, welche Gramme vorkommen. Zur Anfragezeit wird gezählt, wie viele Anfragegramme in der Datei vorkommen (`≥ T` ⇒ Datei wird durchsucht). Die Eingabe sind fertige Schlüssel.

## 8.2 Parameter (`TrieParams`)

| Parameter | Default | Bedeutung |
|---|---|---|
| `key_bits` | 16 | Breite des Schlüssels |
| `quant_bits` (q) | 0 | Zahl der entfernten unteren Schlüsselbits. Zelle = `key >> q`. Auflösung `b = key_bits − q` |
| `root_bits` (r) | 4 | Die obersten r Bit der Zelle bilden eine **direkte Wurzelmaske** mit `2^r` Flags |

- Darunter folgen `L = (b − r) / 2` Radix-4-Ebenen.
- **Gültig nur wenn:** `1 ≤ r ≤ min(8, b)`, `(b − r)` gerade, `b ≤ 24`. Sonst `Err(InvalidParams)`.
- Quantisierung ist einseitig: Mehrere Schlüssel teilen sich eine Zelle. Anwesende Schlüssel bleiben anwesend, es entstehen nur Falsch-Positive.

## 8.3 Kompaktform (Pre-Order-Bitstrom)

- **Bitreihenfolge LSB-first:** Stromposition `i` = Bit `i % 8` von Byte `i / 8`. Padding-Bits am Ende sind 0.
- **Gruppe** = Block von Flags. Die Wurzelgruppe hat `2^r` Flags, alle anderen Gruppen 4 Flags. Flag `j` gehört zum Präfixwert `j` (bei 4er-Gruppen: die nächsten 2 Schlüsselbits, MSB zuerst).
- **Aufbau:** Wurzelgruppe. Danach für jedes gesetzte Flag in aufsteigender Reihenfolge **sofort** dessen Untergruppe (rekursiv, Pre-Order), bevor das nächste Geschwister kommt.
- Die letzte Ebene (Gruppen auf Tiefe L) trägt direkt die Zellenflags. Darunter gibt es nichts mehr.
- **Kanonik:** Gesetzte Flags haben immer eine Untergruppe mit mindestens einem gesetzten Flag. Die Wurzelgruppe darf nur bei leerer Menge ganz null sein.
- Die Ausgabe ist **deterministisch**: unabhängig von Reihenfolge und Duplikaten der Eingabe, byte-identisch.

### Container
Header (little endian):

| Feld | Typ |
|---|---|
| magic `"GTRI"` | 4 Byte |
| version | `u8 = 1` |
| key_bits | `u8` |
| quant_bits | `u8` |
| root_bits | `u8` |
| payload_bits | `u32` |

Danach folgen `ceil(payload_bits/8)` Payload-Bytes.

Zusätzlich `to_payload()` / `from_payload(params, bytes, bits)` ohne Header, für die Verwendung mit gemeinsamen Parametern (Index, Abschnitt 7).

### Verbindlicher Testvektor
`key_bits=6, q=0, r=2` (⇒ b=6, L=2), Schlüssel `{13, 14, 48}`:

- Gruppen in Reihenfolge: `1001 | 0001 | 0110 | 1000 | 1000` (Flag 0 zuerst), 20 Bit.
- Payload-Bytes: `0x89 0x16 0x01`.
- Zellen-Bitmaske: ein Wort `0x0001_0000_0000_6000` (Bits 13, 14, 48 gesetzt).

## 8.4 In-Memory-Form

- `CellMask { words: Box<[u64]> }` mit `2^b` Bit, LSB-first: Zelle `c` = Bit `c % 64` in Wort `c / 64`. Für `b < 6` genau ein Wort, ungenutzte Bits 0.
- Die Kompaktform wird beim Laden in **einem Durchlauf** aufgeklappt. Dabei werden **nur Blattzellen** gesetzt (innere Flags nicht).

## 8.5 Anfrage (`QueryMask`)

- Gebaut aus den Anfrageschlüsseln mit denselben `TrieParams`.
- Speichert:
  - `n_keys`: Zahl verschiedener Anfrageschlüssel.
  - `n_cells`: Zahl verschiedener gesetzter Zellen.
  - eine **dünne Wortliste** `Vec<(word_idx, mask)>`, aufsteigend nach Index.
  - ein **Suffix-Array der Popcounts** dieser Masken (für den Frühabbruch).
- `corrected_threshold(T) = T.saturating_sub(n_keys − n_cells)`. Das gleicht Kollisionen innerhalb der Anfrage aus. Dateiseitige Kollisionen brauchen keine Korrektur.

## 8.6 API (Vorschlag)

```rust
TrieParams::new(key_bits, quant_bits, root_bits) -> Result<_, Error>

CompactTrie::build(params, keys: impl IntoIterator<Item = u32>) -> CompactTrie
CompactTrie::{to_bytes, from_bytes, to_payload, from_payload, expand() -> CellMask}

CellMask::{from_keys, contains_key, count_ones, union_with}

QueryMask::{new, corrected_threshold}

matched_cells(file: &CellMask, q: &QueryMask) -> u32
    // Summe über die dünne Wortliste von popcount(file.words[i] & mask)

matches_at_least(file, q, t_cells) -> bool
    // Frühabbruch: true, sobald Summe >= t;
    // false, sobald Summe + Rest-Popcount (Suffix-Array) < t
```

**Optional (Stretch):** `MaskTable` mit den Wörtern aller Dateien in einem zusammenhängenden `Vec<u64>` (fester Stride) und `scan(&QueryMask, t) -> Vec<u32>`. Parallelisierung optional per `rayon` hinter einem Feature-Flag.

## 8.7 Anforderungen

- Stabiles Rust, `#![forbid(unsafe_code)]`, **keine Rekursion** beim Decodieren (expliziter Stack), `u64::count_ones()` für Popcount.
- **Fehler statt Panic** bei: falscher Magic/Version, ungültigen Parametern, abgeschnittenem Strom, Überhang nach dem Strom, Padding ≠ 0, leerer Untergruppe, Schlüssel ≥ `2^key_bits`.
- Zähler laufen nie über innere Ebenen: nur Zellen werden gezählt.
- Abhängigkeiten minimal (`thiserror` optional). Dev-Dependencies: `proptest`, `criterion`.
- Ort im Repo: eigenes Modul unter `rag/src/layers/` (z. B. `trigram/trie.rs`)

# Tests und Benchmarks

## Teil B (Datenstruktur)

- **Testvektor** aus 8.3 (Bytes und Maske exakt).
- **Property-Tests:**
  - `expand(build(keys))` ≡ `CellMask::from_keys(keys)`.
  - `contains_key(k)` für alle `k ∈ keys`.
  - Gleichheit mit naivem `HashSet`-Zählen für q = 0.
  - Für q > 0 gilt `matched_cells ≥` echte Trefferzahl.
- **Randfälle:** leere Menge, ein Schlüssel, alle Schlüssel (volle Menge), Schlüssel 0 und `2^key_bits − 1`, Duplikate, jede gültige (r, q)-Kombination.
- **Korrupte Eingaben:** Abschneiden an jeder Byte-Grenze, gekippte Bits ⇒ immer `Err`, nie Panic.
- **Reihenfolge-Invarianz** der Bytes (Eingabe gemischt).

## Teil A (Layer)

Ergänzung, in der Vorlage nicht genannt:

- Normalisierung: Beispiele für CamelCase, Umlaute, Whitespace-Kollaps, reine Whitespace-Zeilen, Zeilenanzahl bleibt erhalten.
- Signatur: `go`, `a`, `user` laut 6.2, Dedup, Entfernen häufiger Trigramme.
- Index: neu, veraltet (mtime), Datei gelöscht, Datei > 5 MB.
- End-to-End: Treffer mit Mehrfachvorkommen in einer Zeile ergeben einen Eintrag. `Lines` ist sortiert. Das Timeout verhält sich wie in Abschnitt 3.

## Benchmarks (`criterion`)

- **Aufklappen:** `b = 16`, n ∈ {100, 500, 5000}.
- **Abgleich:** 100 000 Masken zu 8 KB (b = 16) und zu 128 B (b = 10) gegen Anfragen mit 20 bzw. 100 Schlüsseln. Ausgabe: ns pro Datei, getrennt für `matched_cells` und `matches_at_least`.

# 9. Annahmen

4. **CamelCase-Regeln:** Split bei `[a-z0-9][A-Z]` und bei `[A-Z][A-Z][a-z]` (`HTTPServer → http server`). Ziffern bleiben am Token haften, außer bei Groß-Übergang.
5. **Vokabular vs. Schlüsselraum:** Gibt es mehr verschiedene Trigramme als `2^key_bits`, kollidieren IDs. Das wirkt wie Quantisierung (nur Falsch-Positive, unkritisch). Für die Zeichenmenge `[a-z0-9_]` (37³ ≈ 50 653) reicht `key_bits = 16` fast aus. Kollisionen sind akzeptiert, kein Fehler.
6. **Parameter-Konstanz:** `TrieParams` sind pro Index fix. Ändern Sie sich, wird der Index verworfen und neu aufgebaut. Dies kann festgestellt werden, wenn der Erstellungszeitpunkt des Vokabulars (Creation Timestamp persistieren), aktueller ist als die Signaturen. Das wird analog einer veralteten Signatur betrachtet und beim Besuch wird diese aktualisiert. Dies kann dann optimiert auf Basis des normalisierten Spiegels erfolgen.
7. **Binärdateien:** Dateien mit NUL-Bytes oder ungültigem UTF-8 werden übersprungen (nicht indiziert). Dies kann bei der Normalisierung durch einen Fehler abgebildet werden. Es wird dann eine Signatur ohne Trigramme gespeichert.Welches 

# 10. Umsetzungsreihenfolge

1. **Teil B:** `TrieParams`, `CompactTrie::build` und `expand`, Testvektor, Fehlerfälle, Property-Tests.
2. **Teil B:** `QueryMask`, `matched_cells`, `matches_at_least`, Benchmarks. (`MaskTable` zuletzt, optional.)
3. **Normalisierung** mit Tests und zeilentreuem Mapping.
4. **Vokabular** (lazy, persistent) und **Signatur**.
5. **Index** (Speicherung, mtime-Logik, Bereinigung) und gespiegelte normalisierte Dateien.
6. **Layer:** Suchlauf über `dir_cache.rs`, Dateisuchen über `executor.rs`, Timeouts (90 % / 100 %), Ergebnisobjekt.
7. End-to-End-Tests und Dokumentation der Entscheidungen aus Abschnitt 9.

**Definition of Done:** alle Tests grün, `cargo clippy` ohne Warnungen, `#![forbid(unsafe_code)]` im Trie-Modul, Benchmarks laufen, Entscheidungen aus Abschnitt 9 im Code oder README dokumentiert.
SystemInit:  id=c65d6e76-6919-4452-ad29-9a3aac3f4417, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 I'll start by exploring the repo structure and conventions before implementing Part B (step 1).
Control Request:
```yaml
id: 4afe-2b
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: echo "=== tree src ==="; find src -type f | head -100; echo "=== Cargo.toml ==="; cat Cargo.toml
  reason: "Understand repo layout, dependencies, module conventions before implementing Part B"
```
/answer 4afe-2b allow
Control Request:
```yaml
id: 4afe-2c
result:
  structuredContent:
    stdout: |
      === tree src ===
      src/lib.rs
      src/bin/xyrag.rs
      src/core/mod.rs
      src/core/query.rs
      src/core/registry.rs
      src/core/topology.rs
      src/core/engine.rs
      src/core/result.rs
      src/core/persistence.rs
      src/core/layer.rs
      src/core/executor.rs
      src/layers/mod.rs
      src/layers/glob_layer.rs
      src/layers/grep_layer.rs
      src/layers/dir_cache.rs
      === Cargo.toml ===
      [package]
      name = "xy-ai-rag"
      version = "0.1.0"
      edition = "2021"
      description = "xy.ai.rag - Layered Anytime Retrieval Engine"

      [lib]
      name = "xy_ai_rag"
      path = "src/lib.rs"

      [[bin]]
      name = "xyrag"
      path = "src/bin/xyrag.rs"

      [dependencies]
      tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }
      tokio-util = "0.7"
      async-trait = "0.1"
      futures = "0.3"
      serde = { version = "1", features = ["derive"] }
      serde_json = "1"
      rusqlite = { version = "0.31", features = ["bundled"] }
      clap = { version = "4", features = ["derive"] }
      anyhow = "1"
      rayon = "1"
      glob = "0.3"
      lru = "0.12"
      regex = "1"
```
/answer 4afe-2c allow
Control Request:
```yaml
id: 4afe-2d
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs
  reason: Learn layer structure/conventions to follow in trigram module
```
/answer 4afe-2d allow
Control Request:
```yaml
id: 4afe-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Grep-style, regex based file-content search layer."
      - id: D7wDOB|Llqlbo
        signature: "use std::collections::{HashMap, VecDeque};"
      - id: bxxdmX|Ap2kWL
        signature: /// Max number of directory listings kept in memory at once.
      - id: FileCandidate
        signature: "/// One file discovered below the search root, prior to content search. struct …"
        docstring: "One file discovered below the search root, prior to content search."
      - id: join_rel
        signature: "fn join_rel(prefix: &str, name: &str) -> String {"
      - id: collect_files
        signature: "/// Collects every regular file below `root`, via `cache`. /// /// Traverses th…"
        docstring: "Collects every regular file below `root`, via `cache`."
      - id: CompiledToken
        signature: "/// A compiled search token together with a rough specificity score. #[derive(C…"
        docstring: A compiled search token together with a rough specificity score.
      - id: regex_specificity
        signature: "fn regex_specificity(pattern: &str) -> usize {"
      - id: LineMatch
        signature: "/// One matching line found in one file. #[derive(Clone)] struct LineMatch {"
        docstring: One matching line found in one file.
      - id: build_excerpt
        signature: /// Builds a combined excerpt for a line given all of its match spans /// (byte…
        docstring: Builds a combined excerpt for a line given all of its match spans
      - id: search_file
        signature: "/// Searches one file for all `tokens`, line by line; CPU-bound, runs on /// th…"
        docstring: "Searches one file for all `tokens`, line by line; CPU-bound, runs on"
      - id: GrepLayer
        signature: "/// Grep-style, regex based file-content search layer. /// /// See the module d…"
        docstring: "Grep-style, regex based file-content search layer."
      - id: impl_GrepLayer
        signature: "impl GrepLayer {"
        children:
        - id: impl_GrepLayer.new
          signature: "pub fn new() -> Self {"
      - id: impl_Default_for_GrepLayer
        signature: "impl Default for GrepLayer {"
        children:
        - id: impl_Default_for_GrepLayer.default
          signature: "fn default() -> Self {"
      - id: impl_Layer_for_GrepLayer
        signature: "#[async_trait] impl Layer for GrepLayer {"
        children:
        - id: impl_Layer_for_GrepLayer.id
          signature: "fn id(&self) -> &str {"
        - id: impl_Layer_for_GrepLayer.stage
          signature: "fn stage(&self) -> LayerStage {"
        - id: impl_Layer_for_GrepLayer.applies
          signature: "fn applies(&self, query: &Query) -> bool {"
        - id: impl_Layer_for_GrepLayer.run
          signature: "async fn run(     &self,     query: &Query,     result_set: &ResultSet,     ctx…"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs
      nodes:
      - id: ModuleDoc
        signature: //! Glob-pattern based directory/file search layer.
      - id: 1dAG1t|vdDQQo
        signature: "use std::path::{Path, PathBuf};"
      - id: hXrAAw|YF28qS
        signature: /// Max number of directory listings kept in memory at once.
      - id: Candidate
        signature: "/// One candidate found below the search root, prior to glob matching. #[derive…"
        docstring: "One candidate found below the search root, prior to glob matching."
      - id: join_rel
        signature: "fn join_rel(prefix: &str, name: &str) -> String {"
      - id: walk
        signature: "/// Collects every file/directory below `root`, via `cache`. /// /// Traverses …"
        docstring: "Collects every file/directory below `root`, via `cache`."
      - id: wildcard_count
        signature: /// Number of glob meta characters in a pattern - a rough specificity /// signa…
        docstring: Number of glob meta characters in a pattern - a rough specificity
      - id: GlobLayer
        signature: /// Glob-pattern based file/directory search layer. /// /// See the module docu…
        docstring: Glob-pattern based file/directory search layer.
      - id: impl_GlobLayer
        signature: "impl GlobLayer {"
        children:
        - id: impl_GlobLayer.new
          signature: "pub fn new() -> Self {"
      - id: impl_Default_for_GlobLayer
        signature: "impl Default for GlobLayer {"
        children:
        - id: impl_Default_for_GlobLayer.default
          signature: "fn default() -> Self {"
      - id: impl_Layer_for_GlobLayer
        signature: "#[async_trait] impl Layer for GlobLayer {"
        children:
        - id: impl_Layer_for_GlobLayer.id
          signature: "fn id(&self) -> &str {"
        - id: impl_Layer_for_GlobLayer.stage
          signature: "fn stage(&self) -> LayerStage {"
        - id: impl_Layer_for_GlobLayer.applies
          signature: "fn applies(&self, query: &Query) -> bool {"
        - id: impl_Layer_for_GlobLayer.run
          signature: "async fn run(     &self,     query: &Query,     result_set: &ResultSet,     _ct…"
```
/answer 4afe-2e allow
ReasoningToken: 336
Control Request:
```yaml
id: 4afe-2f
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cat src/layers/mod.rs
  reason: See how layer modules are declared to add trigram module
```
/answer 4afe-2f allow
Control Request:
```yaml
id: 4afe-30
result:
  structuredContent:
    stdout: |
      //! Concrete layer implementations (grep, BM25, AST, cache, ...).
      pub mod dir_cache;
      pub mod glob_layer;
      pub mod grep_layer;
```
/answer 4afe-30 allow
ReasoningToken: 136
Control Request:
```yaml
id: 4afe-31
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cat src/lib.rs; echo "=== cargo version ==="; cargo --version 2>/dev/null; ls benches 2>/dev/null || echo "no benches dir"
  reason: Check lib module exposure and whether benches dir exists
```
/answer 4afe-31 allow
Control Request:
```yaml
id: 4afe-32
result:
  structuredContent:
    stdout: |
      //! xy.ai.rag - Layered Anytime Retrieval Engine.
      //!
      //! Used both as a library (`xy_ai_rag::core::...`) and as an on-demand CLI
      //! utility (`xyrag`, see `src/bin/xyrag.rs`) - no daemon or server process.

      pub mod core;
      pub mod layers;

      pub const VERSION: &str = env!("CARGO_PKG_VERSION");
      === cargo version ===
      cargo 1.99.0 (5f94df478 2026-08-27)
      no benches dir
```
/answer 4afe-32 allow
ReasoningToken: 15203
Text:
 I'll implement Part B (steps 1 & 2): the `trigram/trie.rs` module with `TrieParams`, `CompactTrie`, `CellMask`, `QueryMask`, plus tests and benchmark.
Control Request:
```yaml
id: 4afe-33
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/trigram/mod.rs
    source: |
      //! Trigram layer: text-pattern search gated by a per-file trigram signature.
      //!
      //! Part B (this submodule tree) provides the set data structure used by the
      //! layer: a quantised radix-4 trie (`CompactTrie`) as the serialisable form and
      //! a flat bitmask (`CellMask`) as the in-memory form.

      pub mod trie;
  reason: Create trigram module declaring the trie submodule (Part B)
```
/answer 4afe-33 allow
ReasoningToken: 58
Control Request:
```yaml
id: 4afe-34
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/trigram/trie.rs
    source: |
      //! Quantised radix-4 trie for sets of integer keys (trigram IDs).
      //!
      //! Two representations of the same set of `u32` keys:
      //!
      //! * [`CompactTrie`] - a compact, serialisable pre-order radix-4 bitstream.
      //! * [`CellMask`] - a flat `u64` bitmask; matching against a query is `AND` +
      //!   popcount only.
      //!
      //! Keys are quantised to *cells* (`cell = key >> quant_bits`); several keys may
      //! share a cell, which only ever produces false positives at query time.

      #![forbid(unsafe_code)]

      use std::fmt;

      /// Errors raised by the trie; the decoder never panics on malformed input.
      #[derive(Debug, Clone, PartialEq, Eq)]
      pub enum Error {
          /// Parameter combination violates the constraints in `TrieParams::new`.
          InvalidParams,
          /// Container magic was not `"GTRI"`.
          BadMagic,
          /// Container version was not `1`.
          BadVersion,
          /// The bitstream ended before a group could be read in full.
          Truncated,
          /// Bits remained after decoding finished.
          Overhang,
          /// A padding bit past `payload_bits` was set.
          Padding,
          /// A set flag pointed at a child group with no set flags.
          EmptySubgroup,
          /// A key (or cell) was `>= 2^key_bits` (resp. out of the cell range).
          KeyOutOfRange,
      }

      impl fmt::Display for Error {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              let s = match self {
                  Error::InvalidParams => "invalid trie parameters",
                  Error::BadMagic => "bad container magic",
                  Error::BadVersion => "unsupported container version",
                  Error::Truncated => "truncated bitstream",
                  Error::Overhang => "unexpected trailing bits",
                  Error::Padding => "non-zero padding bit",
                  Error::EmptySubgroup => "empty child subgroup",
                  Error::KeyOutOfRange => "key out of range",
              };
              f.write_str(s)
          }
      }

      impl std::error::Error for Error {}

      /// Trie shape parameters; fixed per index.
      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct TrieParams {
          /// Width of a key in bits.
          pub key_bits: u8,
          /// Number of removed low key bits (`q`); `cell = key >> q`.
          pub quant_bits: u8,
          /// Top `r` bits of the cell form the direct root mask (`2^r` flags).
          pub root_bits: u8,
      }

      impl TrieParams {
          /// Builds parameters, validating the constraints from the spec:
          /// `1 <= r <= min(8, b)`, `(b - r)` even, `b <= 24`, where
          /// `b = key_bits - quant_bits`.
          pub fn new(key_bits: u8, quant_bits: u8, root_bits: u8) -> Result<Self, Error> {
              if key_bits == 0 || quant_bits > key_bits {
                  return Err(Error::InvalidParams);
              }
              let b = (key_bits - quant_bits) as u32;
              let r = root_bits as u32;
              if b == 0 || b > 24 {
                  return Err(Error::InvalidParams);
              }
              if r < 1 || r > b.min(8) {
                  return Err(Error::InvalidParams);
              }
              if (b - r) % 2 != 0 {
                  return Err(Error::InvalidParams);
              }
              Ok(Self {
                  key_bits,
                  quant_bits,
                  root_bits,
              })
          }

          /// Cell resolution `b = key_bits - quant_bits`.
          #[inline]
          pub fn cell_bits(&self) -> u32 {
              (self.key_bits - self.quant_bits) as u32
          }

          /// Number of radix-4 levels `L = (b - r) / 2`.
          #[inline]
          pub fn levels(&self) -> u32 {
              (self.cell_bits() - self.root_bits as u32) / 2
          }

          /// Highest valid key, inclusive.
          #[inline]
          fn max_key(&self) -> u64 {
              if self.key_bits >= 64 {
                  u64::MAX
              } else {
                  (1u64 << self.key_bits) - 1
              }
          }

          /// Number of `u64` words needed for a [`CellMask`] (`2^b` bits).
          #[inline]
          fn mask_words(&self) -> usize {
              let cells = 1u64 << self.cell_bits();
              (((cells + 63) / 64) as usize).max(1)
          }

          /// Maps a key to its cell, erroring if the key is out of range.
          #[inline]
          fn cell_of(&self, key: u32) -> Result<u64, Error> {
              if key as u64 > self.max_key() {
                  return Err(Error::KeyOutOfRange);
              }
              Ok((key as u64) >> self.quant_bits)
          }
      }

      #[inline]
      fn get_bit(bytes: &[u8], i: usize) -> bool {
          (bytes[i / 8] >> (i % 8)) & 1 == 1
      }

      /// Collects the distinct, sorted cells for a set of keys.
      fn sorted_cells(params: &TrieParams, keys: impl IntoIterator<Item = u32>) -> Result<Vec<u32>, Error> {
          let mut cells = Vec::new();
          for k in keys {
              cells.push(params.cell_of(k)? as u32);
          }
          cells.sort_unstable();
          cells.dedup();
          Ok(cells)
      }

      /// Compact, serialisable pre-order radix-4 bitstream of a key set.
      #[derive(Clone, Debug, PartialEq, Eq)]
      pub struct CompactTrie {
          params: TrieParams,
          payload: Vec<u8>,
          payload_bits: u32,
      }

      impl CompactTrie {
          /// Builds the canonical compact form from a set of keys.
          ///
          /// Output is deterministic: independent of input order and duplicates.
          pub fn build(params: TrieParams, keys: impl IntoIterator<Item = u32>) -> Result<Self, Error> {
              let cells = sorted_cells(&params, keys)?;
              let mut bits: Vec<bool> = Vec::new();
              emit(&mut bits, &cells, params.cell_bits() - params.root_bits as u32, 1usize << params.root_bits);
              let payload_bits = bits.len() as u32;
              let mut payload = vec![0u8; (bits.len() + 7) / 8];
              for (i, &bit) in bits.iter().enumerate() {
                  if bit {
                      payload[i / 8] |= 1 << (i % 8);
                  }
              }
              Ok(Self {
                  params,
                  payload,
                  payload_bits,
              })
          }

          /// Parameters this trie was built with.
          pub fn params(&self) -> TrieParams {
              self.params
          }

          /// Serialises to the self-describing container (header + payload).
          pub fn to_bytes(&self) -> Vec<u8> {
              let mut out = Vec::with_capacity(12 + self.payload.len());
              out.extend_from_slice(b"GTRI");
              out.push(1); // version
              out.push(self.params.key_bits);
              out.push(self.params.quant_bits);
              out.push(self.params.root_bits);
              out.extend_from_slice(&self.payload_bits.to_le_bytes());
              out.extend_from_slice(&self.payload);
              out
          }

          /// Parses a self-describing container and validates its structure.
          pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
              if bytes.len() < 12 {
                  return Err(Error::Truncated);
              }
              if &bytes[0..4] != b"GTRI" {
                  return Err(Error::BadMagic);
              }
              if bytes[4] != 1 {
                  return Err(Error::BadVersion);
              }
              let params = TrieParams::new(bytes[5], bytes[6], bytes[7])?;
              let payload_bits = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
              let payload = bytes[12..].to_vec();
              Self::from_payload(params, payload, payload_bits)
          }

          /// Payload bytes and bit length, without a header (shared `TrieParams`).
          pub fn to_payload(&self) -> (Vec<u8>, u32) {
              (self.payload.clone(), self.payload_bits)
          }

          /// Builds from a headerless payload with externally supplied parameters,
          /// validating the structure immediately.
          pub fn from_payload(params: TrieParams, bytes: Vec<u8>, payload_bits: u32) -> Result<Self, Error> {
              let trie = Self {
                  params,
                  payload: bytes,
                  payload_bits,
              };
              trie.decode()?; // validate
              Ok(trie)
          }

          /// Expands the compact form into a flat [`CellMask`] in one pass.
          ///
          /// Infallible: every constructor validates the structure beforehand.
          pub fn expand(&self) -> CellMask {
              let cells = self.decode().expect("trie structure validated at construction");
              let mut mask = CellMask::zeros(&self.params);
              for c in cells {
                  mask.words[(c / 64) as usize] |= 1u64 << (c % 64);
              }
              mask
          }

          /// Decodes the bitstream to the set of set leaf cells (explicit stack,
          /// no recursion), validating canonicity and framing.
          fn decode(&self) -> Result<Vec<u32>, Error> {
              let payload_bits = self.payload_bits as usize;
              let total_bytes = (payload_bits + 7) / 8;
              if self.payload.len() < total_bytes {
                  return Err(Error::Truncated);
              }
              for i in payload_bits..total_bytes * 8 {
                  if get_bit(&self.payload, i) {
                      return Err(Error::Padding);
                  }
              }

              let b = self.params.cell_bits();
              let r = self.params.root_bits as u32;
              let mut pos = 0usize;
              let mut cells = Vec::new();
              // stack entry: (shift of this group's flag bits, prefix bits above, n_flags)
              let mut stack: Vec<(u32, u64, usize)> = vec![(b - r, 0u64, 1usize << r)];
              let mut is_root = true;

              while let Some((shift, prefix, nflags)) = stack.pop() {
                  if pos + nflags > payload_bits {
                      return Err(Error::Truncated);
                  }
                  let mut set: Vec<usize> = Vec::new();
                  for j in 0..nflags {
                      if get_bit(&self.payload, pos + j) {
                          set.push(j);
                      }
                  }
                  pos += nflags;

                  if is_root {
                      is_root = false;
                  } else if set.is_empty() {
                      return Err(Error::EmptySubgroup);
                  }

                  if shift == 0 {
                      for &k in &set {
                          cells.push((prefix | k as u64) as u32);
                      }
                  } else {
                      for &k in set.iter().rev() {
                          let child_prefix = prefix | ((k as u64) << shift);
                          stack.push((shift - 2, child_prefix, 4));
                      }
                  }
              }

              if pos != payload_bits {
                  return Err(Error::Overhang);
              }
              Ok(cells)
          }
      }

      /// Emits the canonical pre-order bitstream of `cells` (sorted, within range).
      fn emit(bits: &mut Vec<bool>, cells: &[u32], shift: u32, nflags: usize) {
          let mask = (nflags as u32) - 1;
          let mut present = vec![false; nflags];
          let mut ranges = vec![(0usize, 0usize); nflags];
          let mut i = 0;
          while i < cells.len() {
              let v = ((cells[i] >> shift) & mask) as usize;
              let start = i;
              while i < cells.len() && ((cells[i] >> shift) & mask) as usize == v {
                  i += 1;
              }
              present[v] = true;
              ranges[v] = (start, i);
          }
          for &p in &present {
              bits.push(p);
          }
          if shift == 0 {
              return;
          }
          for v in 0..nflags {
              if present[v] {
                  let (s, e) = ranges[v];
                  emit(bits, &cells[s..e], shift - 2, 4);
              }
          }
      }

      /// Flat in-memory bitmask over `2^b` cells, LSB-first.
      #[derive(Clone, Debug, PartialEq, Eq)]
      pub struct CellMask {
          words: Box<[u64]>,
          params: TrieParams,
      }

      impl CellMask {
          fn zeros(params: &TrieParams) -> Self {
              Self {
                  words: vec![0u64; params.mask_words()].into_boxed_slice(),
                  params: *params,
              }
          }

          /// Builds a mask directly from keys.
          pub fn from_keys(params: TrieParams, keys: impl IntoIterator<Item = u32>) -> Result<Self, Error> {
              let mut mask = Self::zeros(&params);
              for k in keys {
                  let c = params.cell_of(k)?;
                  mask.words[(c / 64) as usize] |= 1u64 << (c % 64);
              }
              Ok(mask)
          }

          /// Whether the cell of `key` is set.
          pub fn contains_key(&self, key: u32) -> bool {
              let c = match self.params.cell_of(key) {
                  Ok(c) => c,
                  Err(_) => return false,
              };
              (self.words[(c / 64) as usize] >> (c % 64)) & 1 == 1
          }

          /// Number of set cells.
          pub fn count_ones(&self) -> u32 {
              self.words.iter().map(|w| w.count_ones()).sum()
          }

          /// In-place union with another mask of the same parameters.
          pub fn union_with(&mut self, other: &CellMask) {
              for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
                  *a |= *b;
              }
          }

          /// Raw words (LSB-first).
          pub fn words(&self) -> &[u64] {
              &self.words
          }
      }

      /// Query side: sparse word list of set cells plus a popcount suffix array for
      /// the early-abort in [`matches_at_least`].
      #[derive(Clone, Debug)]
      pub struct QueryMask {
          /// Distinct query keys.
          pub n_keys: u32,
          /// Distinct set cells.
          pub n_cells: u32,
          /// `(word_idx, mask)`, ascending by index.
          sparse: Vec<(usize, u64)>,
          /// `suffix_pop[i]` = sum of popcounts of masks `i..`.
          suffix_pop: Vec<u32>,
      }

      impl QueryMask {
          /// Builds a query mask from the query keys with the given parameters.
          pub fn new(params: TrieParams, keys: impl IntoIterator<Item = u32>) -> Result<Self, Error> {
              let mut distinct_keys = std::collections::BTreeSet::new();
              for k in keys {
                  params.cell_of(k)?; // range-check
                  distinct_keys.insert(k);
              }
              let mut words: std::collections::BTreeMap<usize, u64> = std::collections::BTreeMap::new();
              for &k in &distinct_keys {
                  let c = (k as u64) >> params.quant_bits;
                  *words.entry((c / 64) as usize).or_insert(0) |= 1u64 << (c % 64);
              }
              let sparse: Vec<(usize, u64)> = words.into_iter().collect();
              let n_cells: u32 = sparse.iter().map(|(_, m)| m.count_ones()).sum();
              let mut suffix_pop = vec![0u32; sparse.len() + 1];
              for i in (0..sparse.len()).rev() {
                  suffix_pop[i] = suffix_pop[i + 1] + sparse[i].1.count_ones();
              }
              Ok(Self {
                  n_keys: distinct_keys.len() as u32,
                  n_cells,
                  sparse,
                  suffix_pop,
              })
          }

          /// Corrects a cell threshold for query-internal collisions.
          pub fn corrected_threshold(&self, t: u32) -> u32 {
              t.saturating_sub(self.n_keys - self.n_cells)
          }
      }

      /// Number of query cells also present in `file`.
      pub fn matched_cells(file: &CellMask, q: &QueryMask) -> u32 {
          q.sparse
              .iter()
              .map(|&(w, m)| (file.words[w] & m).count_ones())
              .sum()
      }

      /// Whether at least `t_cells` query cells are present in `file`, with early
      /// abort from both sides.
      pub fn matches_at_least(file: &CellMask, q: &QueryMask, t_cells: u32) -> bool {
          if t_cells == 0 {
              return true;
          }
          let mut sum = 0u32;
          for (i, &(w, m)) in q.sparse.iter().enumerate() {
              sum += (file.words[w] & m).count_ones();
              if sum >= t_cells {
                  return true;
              }
              if sum + q.suffix_pop[i + 1] < t_cells {
                  return false;
              }
          }
          sum >= t_cells
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use std::collections::HashSet;

          #[test]
          fn test_vector_bytes_and_mask() {
              let params = TrieParams::new(6, 0, 2).unwrap();
              let trie = CompactTrie::build(params, [13u32, 14, 48]).unwrap();
              let (payload, bits) = trie.to_payload();
              assert_eq!(bits, 20);
              assert_eq!(payload, vec![0x89, 0x16, 0x01]);
              let mask = trie.expand();
              assert_eq!(mask.words(), &[0x0001_0000_0000_6000u64]);
          }

          #[test]
          fn container_roundtrip() {
              let params = TrieParams::new(6, 0, 2).unwrap();
              let trie = CompactTrie::build(params, [13u32, 14, 48]).unwrap();
              let bytes = trie.to_bytes();
              let back = CompactTrie::from_bytes(&bytes).unwrap();
              assert_eq!(trie, back);
          }

          #[test]
          fn order_and_dup_invariance() {
              let params = TrieParams::new(6, 0, 2).unwrap();
              let a = CompactTrie::build(params, [48u32, 14, 13, 13, 48]).unwrap();
              let b = CompactTrie::build(params, [13u32, 14, 48]).unwrap();
              assert_eq!(a.to_bytes(), b.to_bytes());
          }

          #[test]
          fn edge_cases() {
              for (kb, q, r) in [(6u8, 0u8, 2u8), (6, 0, 6), (16, 0, 4), (16, 4, 4), (10, 0, 2)] {
                  let params = TrieParams::new(kb, q, r).unwrap();
                  let maxk = (1u32 << kb) - 1;
                  for keys in [vec![], vec![0u32], vec![maxk], vec![0, maxk]] {
                      let trie = CompactTrie::build(params, keys.clone()).unwrap();
                      let expanded = trie.expand();
                      let direct = CellMask::from_keys(params, keys.clone()).unwrap();
                      assert_eq!(expanded, direct, "kb={kb} q={q} r={r} keys={keys:?}");
                      for k in &keys {
                          assert!(expanded.contains_key(*k));
                      }
                  }
              }
          }

          #[test]
          fn full_set() {
              let params = TrieParams::new(6, 0, 2).unwrap();
              let all: Vec<u32> = (0..64).collect();
              let trie = CompactTrie::build(params, all.clone()).unwrap();
              let mask = trie.expand();
              assert_eq!(mask.count_ones(), 64);
              assert_eq!(mask.words(), &[u64::MAX]);
          }

          #[test]
          fn key_out_of_range() {
              let params = TrieParams::new(6, 0, 2).unwrap();
              assert_eq!(CompactTrie::build(params, [64u32]), Err(Error::KeyOutOfRange));
          }

          #[test]
          fn invalid_params() {
              assert!(TrieParams::new(6, 0, 3).is_err()); // (b-r) odd
              assert!(TrieParams::new(6, 0, 0).is_err()); // r < 1
              assert!(TrieParams::new(6, 0, 7).is_err()); // r > min(8,b)
              assert!(TrieParams::new(26, 0, 2).is_err()); // b > 24
          }

          #[test]
          fn corrupt_inputs_never_panic() {
              let params = TrieParams::new(16, 0, 4).unwrap();
              let trie = CompactTrie::build(params, [1u32, 2, 1000, 50000, 65535]).unwrap();
              let good = trie.to_bytes();

              // Truncate at every byte boundary.
              for n in 0..good.len() {
                  let _ = CompactTrie::from_bytes(&good[..n]); // must not panic
              }
              // Flip every single bit.
              for byte in 0..good.len() {
                  for bit in 0..8 {
                      let mut c = good.clone();
                      c[byte] ^= 1 << bit;
                      let _ = CompactTrie::from_bytes(&c); // must not panic
                  }
              }
          }

          #[test]
          fn bad_magic_and_version() {
              let params = TrieParams::new(6, 0, 2).unwrap();
              let mut bytes = CompactTrie::build(params, [1u32]).unwrap().to_bytes();
              let mut bad = bytes.clone();
              bad[0] = b'X';
              assert_eq!(CompactTrie::from_bytes(&bad), Err(Error::BadMagic));
              bytes[4] = 2;
              assert_eq!(CompactTrie::from_bytes(&bytes), Err(Error::BadVersion));
          }

          #[test]
          fn query_matching_basic() {
              let params = TrieParams::new(16, 0, 4).unwrap();
              let file = CellMask::from_keys(params, [1u32, 2, 3, 100, 200]).unwrap();
              let q = QueryMask::new(params, [2u32, 3, 999]).unwrap();
              assert_eq!(matched_cells(&file, &q), 2);
              assert!(matches_at_least(&file, &q, 2));
              assert!(!matches_at_least(&file, &q, 3));
          }

          #[test]
          fn corrected_threshold_for_collisions() {
              // q = 2 collapses keys 0..3 into one cell -> n_keys > n_cells.
              let params = TrieParams::new(6, 2, 2).unwrap();
              let q = QueryMask::new(params, [0u32, 1, 2, 3]).unwrap();
              assert_eq!(q.n_keys, 4);
              assert_eq!(q.n_cells, 1);
              assert_eq!(q.corrected_threshold(4), 1);
          }

          // ---- Property tests ----------------------------------------------------

          use proptest::prelude::*;

          proptest! {
              #[test]
              fn prop_expand_equals_from_keys(keys in prop::collection::vec(0u32..65536, 0..200)) {
                  let params = TrieParams::new(16, 0, 4).unwrap();
                  let trie = CompactTrie::build(params, keys.clone()).unwrap();
                  prop_assert_eq!(trie.expand(), CellMask::from_keys(params, keys).unwrap());
              }

              #[test]
              fn prop_contains_all_keys(keys in prop::collection::vec(0u32..65536, 0..200)) {
                  let params = TrieParams::new(16, 0, 4).unwrap();
                  let mask = CompactTrie::build(params, keys.clone()).unwrap().expand();
                  for k in keys {
                      prop_assert!(mask.contains_key(k));
                  }
              }

              #[test]
              fn prop_matched_equals_naive_q0(
                  fkeys in prop::collection::vec(0u32..4096, 0..200),
                  qkeys in prop::collection::vec(0u32..4096, 0..60),
              ) {
                  let params = TrieParams::new(12, 0, 4).unwrap();
                  let file = CellMask::from_keys(params, fkeys.clone()).unwrap();
                  let q = QueryMask::new(params, qkeys.clone()).unwrap();
                  let fset: HashSet<u32> = fkeys.into_iter().collect();
                  let qset: HashSet<u32> = qkeys.into_iter().collect();
                  let naive = qset.iter().filter(|k| fset.contains(k)).count() as u32;
                  prop_assert_eq!(matched_cells(&file, &q), naive);
              }

              #[test]
              fn prop_matched_upper_bounds_real_qgt0(
                  fkeys in prop::collection::vec(0u32..65536, 0..200),
                  qkeys in prop::collection::vec(0u32..65536, 0..60),
              ) {
                  let params = TrieParams::new(16, 4, 4).unwrap();
                  let file = CellMask::from_keys(params, fkeys.clone()).unwrap();
                  let q = QueryMask::new(params, qkeys.clone()).unwrap();
                  let fset: HashSet<u32> = fkeys.into_iter().collect();
                  let qset: HashSet<u32> = qkeys.into_iter().collect();
                  let real = qset.iter().filter(|k| fset.contains(k)).count() as u32;
                  prop_assert!(matched_cells(&file, &q) >= real);
              }

              #[test]
              fn prop_bytes_order_invariant(mut keys in prop::collection::vec(0u32..65536, 0..200)) {
                  let params = TrieParams::new(16, 0, 4).unwrap();
                  let a = CompactTrie::build(params, keys.clone()).unwrap().to_bytes();
                  keys.reverse();
                  keys.extend_from_within(..); // duplicates
                  let b = CompactTrie::build(params, keys).unwrap().to_bytes();
                  prop_assert_eq!(a, b);
              }

              #[test]
              fn prop_matches_at_least_agrees(
                  fkeys in prop::collection::vec(0u32..65536, 0..200),
                  qkeys in prop::collection::vec(0u32..65536, 0..60),
                  t in 0u32..40,
              ) {
                  let params = TrieParams::new(16, 0, 4).unwrap();
                  let file = CellMask::from_keys(params, fkeys).unwrap();
                  let q = QueryMask::new(params, qkeys).unwrap();
                  prop_assert_eq!(matches_at_least(&file, &q, t), matched_cells(&file, &q) >= t);
              }
          }
      }
  reason: Implement Part B core data structure with full unit + property + corruption tests
```
/answer 4afe-34 allow
Control Request:
```yaml
id: 4afe-35
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/mod.rs
    old_text: pub mod grep_layer;
    new_text: |-
      pub mod grep_layer;
      pub mod trigram;
  reason: Register the new trigram module
```
/answer 4afe-35 allow
Control Request:
```yaml
id: 4afe-36
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    old_text: |
      regex = "1"
    new_text: |
      regex = "1"

      [dev-dependencies]
      proptest = "1"
      criterion = "0.5"

      [[bench]]
      name = "trie_bench"
      harness = false
  reason: Add dev-dependencies and bench target for Part B
```
/answer 4afe-36 allow
Control Request:
```yaml
id: 4afe-37
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/benches/trie_bench.rs
    source: |
      //! Benchmarks for the trigram trie (Part B, section 8 "Benchmarks").
      //!
      //! * Expansion: `b = 16`, n in {100, 500, 5000}.
      //! * Matching: 100 000 masks of 8 KB (b = 16) and 128 B (b = 10) against
      //!   queries of 20 and 100 keys; reported per file for `matched_cells` and
      //!   `matches_at_least`.

      use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
      use xy_ai_rag::layers::trigram::trie::{
          matched_cells, matches_at_least, CellMask, CompactTrie, QueryMask, TrieParams,
      };

      /// Cheap deterministic PRNG (xorshift64) to avoid extra dependencies.
      struct Rng(u64);
      impl Rng {
          fn next(&mut self) -> u64 {
              let mut x = self.0;
              x ^= x << 13;
              x ^= x >> 7;
              x ^= x << 17;
              self.0 = x;
              x
          }
          fn key(&mut self, bits: u32) -> u32 {
              (self.next() & ((1u64 << bits) - 1)) as u32
          }
      }

      fn keys(rng: &mut Rng, n: usize, bits: u32) -> Vec<u32> {
          (0..n).map(|_| rng.key(bits)).collect()
      }

      fn bench_expand(c: &mut Criterion) {
          let params = TrieParams::new(16, 0, 4).unwrap();
          let mut group = c.benchmark_group("expand_b16");
          for &n in &[100usize, 500, 5000] {
              let mut rng = Rng(0x1234_5678_9abc_def0);
              let trie = CompactTrie::build(params, keys(&mut rng, n, 16)).unwrap();
              group.bench_function(format!("n{n}"), |bch| {
                  bch.iter_batched(|| &trie, |t| black_box(t.expand()), BatchSize::SmallInput)
              });
          }
          group.finish();
      }

      fn bench_match(c: &mut Criterion) {
          const FILES: usize = 100_000;
          for &(bits, label) in &[(16u32, "b16_8k"), (10u32, "b10_128b")] {
              let params = TrieParams::new(bits as u8, 0, 4).unwrap();
              let mut rng = Rng(0xdead_beef_cafe_babe);
              let masks: Vec<CellMask> = (0..FILES)
                  .map(|_| CellMask::from_keys(params, keys(&mut rng, 40, bits)).unwrap())
                  .collect();
              for &qn in &[20usize, 100] {
                  let q = QueryMask::new(params, keys(&mut rng, qn, bits)).unwrap();
                  let t = q.corrected_threshold((qn as u32 * 8) / 10);
                  let mut group = c.benchmark_group(format!("match_{label}_q{qn}"));
                  group.throughput(criterion::Throughput::Elements(FILES as u64));
                  group.bench_function("matched_cells", |bch| {
                      bch.iter(|| {
                          let mut acc = 0u64;
                          for m in &masks {
                              acc += black_box(matched_cells(m, &q)) as u64;
                          }
                          acc
                      })
                  });
                  group.bench_function("matches_at_least", |bch| {
                      bch.iter(|| {
                          let mut acc = 0u64;
                          for m in &masks {
                              acc += black_box(matches_at_least(m, &q, t)) as u64;
                          }
                          acc
                      })
                  });
                  group.finish();
              }
          }
      }

      criterion_group!(benches, bench_expand, bench_match);
      criterion_main!(benches);
  reason: Add criterion benchmark for expansion and matching per spec
```
/answer 4afe-37 allow
Control Request:
```yaml
id: 4afe-38
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo test --lib layers::trigram 2>&1 | tail -40
  reason: Compile and run the trie unit + property tests
```
/answer 4afe-38 allow
Control Request:
```yaml
id: 4afe-39
result:
  structuredContent:
    stdout: |2+
         Compiling rusty-fork v0.3.1
         Compiling serde_json v1.0.151
         Compiling zerocopy-derive v0.8.59
         Compiling tinytemplate v1.2.1
         Compiling ahash v0.8.12
         Compiling half v2.7.1
         Compiling ppv-lite86 v0.2.21
         Compiling hashbrown v0.14.5
         Compiling ciborium-ll v0.2.2
         Compiling rand_chacha v0.9.0
         Compiling ciborium v0.2.2
         Compiling proptest v1.11.0
         Compiling hashlink v0.9.1
         Compiling criterion v0.5.1
         Compiling rusqlite v0.31.0
         Compiling xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
          Finished `test` profile [unoptimized + debuginfo] target(s) in 12.88s
           Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-0156d17e3f129464)

      running 17 tests
      test layers::trigram::trie::tests::bad_magic_and_version ... ok
      test layers::trigram::trie::tests::container_roundtrip ... ok
      test layers::trigram::trie::tests::corrected_threshold_for_collisions ... ok
      test layers::trigram::trie::tests::full_set ... ok
      test layers::trigram::trie::tests::invalid_params ... ok
      test layers::trigram::trie::tests::key_out_of_range ... ok
      test layers::trigram::trie::tests::edge_cases ... ok
      test layers::trigram::trie::tests::order_and_dup_invariance ... ok
      test layers::trigram::trie::tests::query_matching_basic ... ok
      test layers::trigram::trie::tests::test_vector_bytes_and_mask ... ok
      test layers::trigram::trie::tests::corrupt_inputs_never_panic ... ok
      test layers::trigram::trie::tests::prop_matches_at_least_agrees ... ok
      test layers::trigram::trie::tests::prop_matched_equals_naive_q0 ... ok
      test layers::trigram::trie::tests::prop_matched_upper_bounds_real_qgt0 ... ok
      test layers::trigram::trie::tests::prop_contains_all_keys ... ok
      test layers::trigram::trie::tests::prop_expand_equals_from_keys ... ok
      test layers::trigram::trie::tests::prop_bytes_order_invariant ... ok

      test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
```
/answer 4afe-39 allow
Control Request:
```yaml
id: 4afe-3a
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo clippy --all-targets 2>&1 | grep -E "warning|error|trigram|trie" | head -40; echo "---exit---"; cargo bench --no-run 2>&1 | tail -5
  reason: Verify clippy clean and benchmark compiles
```
/answer 4afe-3a allow
Control Request:
```yaml
id: 4afe-3b
result:
  structuredContent:
    stdout: |2
          Checking quick-error v1.2.3
      warning: manual implementation of `.is_multiple_of()`
        --> src/layers/trigram/trie.rs:82:12
      warning: manually reimplementing `div_ceil`
         --> src/layers/trigram/trie.rs:118:10
      warning: manually reimplementing `div_ceil`
         --> src/layers/trigram/trie.rs:164:37
      warning: manually reimplementing `div_ceil`
         --> src/layers/trigram/trie.rs:245:27
      warning: `xy-ai-rag` (lib) generated 4 warnings (run `cargo clippy --fix --lib -p xy-ai-rag -- ` to apply 4 suggestions)
      warning: `xy-ai-rag` (lib test) generated 4 warnings (4 duplicates)
      ---exit---
         Compiling xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
          Finished `bench` profile [optimized] target(s) in 16.03s
        Executable benches src/lib.rs (.bin/release/deps/xy_ai_rag-9358ae97e26ac572)
        Executable benches src/bin/xyrag.rs (.bin/release/deps/xyrag-15242d43ce49fcac)
        Executable benches/trie_bench.rs (.bin/release/deps/trie_bench-31dbca1f31f59a38)
```
/answer 4afe-3b allow
Control Request:
```yaml
id: 4afe-3c
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/trigram/trie.rs
    old_text: "        if (b - r) % 2 != 0 {"
    new_text: "        if !(b - r).is_multiple_of(2) {"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/trigram/trie.rs
    old_text: '        (((cells + 63) / 64) as usize).max(1)'
    new_text: '        ((cells.div_ceil(64)) as usize).max(1)'
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/trigram/trie.rs
    old_text: "        let mut payload = vec![0u8; (bits.len() + 7) / 8];"
    new_text: "        let mut payload = vec![0u8; bits.len().div_ceil(8)];"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/trigram/trie.rs
    old_text: '        let total_bytes = (payload_bits + 7) / 8;'
    new_text: '        let total_bytes = payload_bits.div_ceil(8);'
  reason: Resolve clippy warnings (is_multiple_of / div_ceil)
```
/answer 4afe-3c allow
Control Request:
```yaml
id: 4afe-3d
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo clippy --all-targets 2>&1 | grep -E "warning:|error" | head; echo "exit=$?"
  reason: Confirm clippy is now clean
```
/answer 4afe-3d allow
Control Request:
```yaml
id: 4afe-3e
result:
  structuredContent:
    stdout: |
      exit=0
```
/answer 4afe-3e allow
Result Stats: id=c65d6e76-6919-4452-ad29-9a3aac3f4417, total: 80895, in: 139, out: 30938, reason: 237, read: 400846, write: 49818
ReasoningToken: 13
Text:
 Now wire up the module and add dev-dependencies plus the benchmark.
Text:
 Now let me build and run the tests and clippy.
Text:
 All 17 tests pass. Now clippy and a bench compile check.
Text:
 Benchmark compiles. Let me fix the 4 clippy warnings.
ReasoningToken: 224
Part B (Umsetzungsreihenfolge steps 1–2) is complete and green:

- `src/layers/trigram/trie.rs` — `TrieParams`, `CompactTrie` (`build`/`to_bytes`/`from_bytes`/`to_payload`/`from_payload`/`expand`), `CellMask`, `QueryMask`, `matched_cells`, `matches_at_least`. `#![forbid(unsafe_code)]`, iterative decode (explicit stack), `Error` enum (no `thiserror`), errors instead of panics.
- Test vector exact (`0x89 0x16 0x01`, mask `0x0001_0000_0000_6000`), property tests, corruption/truncation fuzz, edge cases — 17 tests pass.
- `benches/trie_bench.rs` (criterion) for expand + matching; compiles.
- `cargo clippy --all-targets` clean.

Remaining (steps 3–7: normalization, vocabulary, index, layer, E2E) not yet started.
