## Tokenizer

### Laden

- Der Tokenizer wird aus seiner Konfigurationsdatei mit festem Dateinamen geladen, die im Verzeichnis der Modellkonfiguration liegt.
- Alternativ entsteht er aus einem bereits geladenen Tokenizer mit frei wählbarer ID, Schema für die Modus-Wahl und maximaler Sequenzlänge.
- Beim Laden aus einer Modellbeschreibung werden das Schema aus den hinterlegten Präfixen gebildet, die maximale Sequenzlänge übernommen und die Ressourcen-ID aus dem Dateipfad abgeleitet, sodass jeder geladene Tokenizer eindeutig identifizierbar ist.
- Fehlt die Datei oder ist sie unlesbar, bricht das Laden mit einem Load-Error ab, der Pfad und Ursache nennt.
- Das Pad-Token wird in dieser Reihenfolge bestimmt: im Tokenizer konfiguriertes Pad-Token, dann das Token `<pad>`, dann das Token `[PAD]`. Fehlt jedes, bricht das Laden mit einem Load-Error ab. Das Pad-Token ist nach außen abfragbar.
- Das eingebaute automatische Padding wird abgeschaltet. Padding ist ausschließlich Aufgabe der Batch-Bildung.
- Jede Eingabe wird auf die maximale Sequenzlänge trunkiert, wobei das abschließende Special-Token erhalten bleibt. Lehnt der Tokenizer die Truncation-Einstellung ab, bricht das Laden mit Ursache ab.

### Textvorbereitung

- Optional wird ein austauschbarer, nebenläufig nutzbarer Text-Normalizer übergeben. Ohne ihn bleibt der Text unverändert. Ein Identity-Normalizer wird mitgeliefert.
- Normalisierungen, die der Tokenizer selbst durchführt, werden nicht doppelt ausgeführt. Ein eigener Normalizer ergänzt nur zusätzliche Transformationen.
- Die Normalisierung läuft vor dem Voranstellen von Präfixen oder Markern und vor der Tokenisierung.

### Encoding-Modi und Schemata

- Zwei Encoding-Modi: Query und Passage (Dokument).
- **Präfix-Schema:** Je Modus wird der Query- bzw. Passage-Präfix vorangestellt. Die Tokenfolge bleibt danach unverändert.
- **Multi-Vector-Schema** (Late Interaction): Je Modus wird ein Query- bzw. Dokument-Marker vorangestellt. Je Modellvariante konfigurierbar sind Markertext für Queries und Dokumente, feste Query-Länge, maximale Dokumentlänge und ID des Mask-Tokens.
  - Query-Modus: Trunkieren auf die Query-Länge, Padding mit dem Mask-Token auf genau diese Länge. Die gepaddeten Positionen werden vom Modell berücksichtigt, nicht ausgeblendet (Query Augmentation).
  - Dokument-Modus: Trunkieren auf die maximale Dokumentlänge, ohne Padding. Pad-Tokens werden verworfen.
  - Diese Trunkierungen gelten zusätzlich zur allgemeinen Trunkierung.

### Encoding

- Eine Liste von Texten beliebiger als String lesbarer Typen wird in einem Modus in Token-IDs umgewandelt. Eine leere Liste wird mit „keine Eingabetexte“ abgelehnt.
- Je Text in dieser Reihenfolge: normalisieren, Präfix oder Marker voranstellen, tokenisieren mit den vorgesehenen Special-Tokens, schemaabhängig nachbearbeiten. Alle Texte werden gemeinsam als Batch tokenisiert.
- Scheitert die Tokenisierung, wird ein Encoding-Error mit Ursache gemeldet.
- Rückgabe in Eingabereihenfolge, ohne Padding außer dem festen Query-Padding im Multi-Vector-Schema.
- Geloggt werden Anzahl trunkierter Texte mit Modus und Textanzahl sowie auf Trace-Level die größte Sequenzlänge.
- Eine kombinierte Funktion kodiert und teilt danach in gepaddete, nach Länge gruppierte Batches gemäß Batch-Konfiguration. Jeder Batch ordnet seine Zeilen den Eingabe-Indizes zu. Encoding-Errors, auch die leere Eingabe, werden unverändert weitergegeben. Die Batch-Anzahl wird geloggt.

