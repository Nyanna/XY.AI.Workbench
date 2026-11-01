## Modell

### Modellbeschreibung und Validierung

- Ein Modell wird durch eine reine, ressourcenlose Beschreibung (Descriptor) dargestellt:
  - Model-ID und Architektur-ID,
  - Embedding-Dimension,
  - Pooling-Verfahren und Normalisierungsart,
  - Präfixe für Queries und Passages,
  - Storage-Datentyp der Embeddings, unabhängig von den Compute-Datentypen wählbar,
  - Compute-Datentypen des Hauptmodells, getrennt für CPU und GPU und nach Device-Klasse ermittelbar,
  - maximale Sequenzlänge,
  - optionale Projection-Layer mit Tensor-Name,
  - Pfade zur Konfigurationsdatei und zu den Weight-Dateien,
  - optionale Multi-Vector-Konfiguration.
- Default für „intfloat/multilingual-e5-small“ auf Basis eines Verzeichnisses und wählbaren Storage-Datentyps: Architektur „bert“, Dimension 384, Mean-Pooling über unmaskierte Token, L2-Normalisierung, Präfixe „query: “ und „passage: “, maximale Sequenzlänge 512, Compute-Datentyp FP32 auf beiden Device-Klassen, keine Projection-Layer, Konfigurationsdatei „config.json“ und Weight-Datei „model.safetensors“ im Verzeichnis.
- Validierung: Dimension und maximale Sequenzlänge größer null, ein nicht leeres Präfix endet mit einem Leerzeichen. Verstöße ergeben „inkompatible Modellkonfiguration“ mit Begründung. Die Validierung verhindert Konfigurationen, die die Embedding-Qualität unbemerkt verschlechtern.

### Modellregister und Laden

- Das Register bildet Architektur-IDs auf Konstruktor-Funktionen (Factories) für Encoder ab. Erneute Registrierung derselben ID ersetzt die bisherige.
- Ein Register mit allen in den Build einkompilierten Architekturen ist erzeugbar. „bert“ ist nur bei aktivierter Backend-Unterstützung (Feature-Flag) enthalten.
- Laden für eine Device-Klasse in dieser Reihenfolge: Beschreibung validieren, Factory suchen, Encoder konstruieren. Unbekannte Architekturen ergeben „nicht unterstützte Architektur“ mit ID.
- Ergebnis ist ein geladener Encoder aus Architektur (Code) und Gewichten (Daten) auf einem Device, der Beschreibung, Device-Klasse und Encoder bündelt und lesend zugänglich macht.
- Der geladene Encoder ist eine verwaltbare Ressource: Key aus Architektur-ID und Device-Klasse, Initialisierungskosten 100, Freigeben entlädt die Gewichte.

### Laden des Transformer-Hauptmodells

- Die Konfigurationsdatei wird als strukturierter Text (JSON) gelesen. Unlesbarkeit ergibt einen Load-Error mit Begründung.
- Der Modelltyp muss „bert“ lauten, sonst „inkompatibel“ mit gefundenem Typ.
- Die Hidden-Size muss der Beschreibung entsprechen. Die maximale Sequenzlänge darf die Anzahl der Position-Embeddings nicht überschreiten. Verstöße ergeben „inkompatibel“ mit beiden Werten.
- Auf der CPU ist das Modell stets ladbar. Auf der GPU nur bei einkompilierter Unterstützung, dann auf dem ersten GPU-Device, sonst „inkompatibel“ mit Hinweis auf die fehlende Unterstützung.
- Der Compute-Datentyp der Device-Klasse wird in den Backend-Datentyp übersetzt, sonst „inkompatibel“.
- Gewichte werden per mmap ohne vollständiges Kopieren im gewählten Datentyp auf das Device geladen. Ein fehlender Tensor ergibt „Gewichtsname passt nicht“ mit Name, sonstige Fehler einen Load-Error.
- Forward-Pass: aus Token-IDs und Mask (Zeilen × Sequenzlänge) die Hidden-States aller Token. Token-Type-Inputs sind null, die Mask wirkt als Attention-Mask. Ergebnis als flache Liste von FP32 (Zeilen × Sequenzlänge × Dimension), unabhängig vom Compute-Datentyp. Fehler ergeben „Encoding fehlgeschlagen“. Die Ausgabedimension ist abfragbar.
- Zwischen Hauptmodell und übriger Pipeline werden keine Typen der Tensor-Library ausgetauscht, nur einfache Zahlenlisten.

### Aufbau des Encoders

- Aus Beschreibung und Device-Klasse entsteht ein vollständiger Encoder aus Hauptmodell, optionaler Projection-Layer, Pooling, Normalisierung und Storage-Datentyp. Device-Klasse und Compute-Datentyp werden für das Tracing hinterlegt.
- Die Normalisierungsart wird übernommen. Ohne Angabe ist die L2-Normalisierung Default.
- Die Projection-Layer wird aus der ersten Weight-Datei geladen. Ohne Weight-Datei: Load-Error „keine Gewichtsdateien“. Ihre Eingabedimension muss der Ausgabedimension des Hauptmodells entsprechen, sonst „inkompatibel“.
- Für Multi-Vector-Modelle nach Art von ColBERT, für die die externe Library kein Modul hat, wird ein vorhandener Encoder-Backbone (BERT, ModernBERT und weitere) mit eigenem Projection-Head verwendet.

### Projection-Layer

- Optionale lineare Abbildung jedes Token-Vektors von Input- auf Output-Dimension, etwa von 768 auf 128, insbesondere für Late Interaction.
- Weight-Matrix Output × Input, optionaler Bias der Länge Output-Dimension. Laden aus safetensors im Speicher oder per mmap aus einer Datei, über den Tensor-Namen der Beschreibung.
- Der Bias-Name entsteht durch Ersetzen der Endung „weight“ durch „bias“, sonst durch Anhängen von „bias“.
- Fehlender Tensor: „Gewichtsname passt nicht“. Falsche Shape: „inkompatibel“ mit tatsächlicher und erwarteter Shape. Werte in FP32, FP16 oder BF16 werden in FP32 konvertiert, andere Typen ergeben „inkompatibel“ mit Tensor und Typ. Dateifehler ergeben Load-Error.
- Anwendung: Dot-Product mit jeder Weight-Zeile plus Bias, Reihenfolge der Vektoren bleibt. Input- und Output-Dimension sind abfragbar.

### Pooling und Normalisierung

- Pooling-Verfahren: Mean über unmaskierte Token (ein Vektor je Sequenz) oder kein Pooling (ein Vektor je Token).
- Mean: Summe der Hidden-State-Vektoren mit Mask ungleich null geteilt durch deren Anzahl. Padding beeinflusst das Ergebnis nicht. Vollständig maskierte Sequenzen ergeben einen Nullvektor ohne Division durch null.
- Ohne Pooling werden alle Positionen unverändert weitergegeben, einschließlich Padding.
- L2-Normalisierung: jeder Vektor wird durch seine Norm geteilt, damit das Dot-Product dem Cosinus entspricht. Die Norm wird nach unten auf 10⁻¹² begrenzt (Epsilon-Clamp). Vektoren darunter gelten als degeneriert und bleiben praktisch Nullvektoren. Die Anzahl degenerierter Vektoren wird zurückgemeldet. Die Art „none“ lässt Vektoren unverändert.
- Die L2-Normalisierung wird einmal beim Schreiben am Ende des Encoders ausgeführt, nicht im Scorer. Query und Dokument werden gleich behandelt.

### Schreiben in die Output-Region

- Die Ergebnisvektoren werden bei Bedarf normalisiert, in den Output-Datentyp konvertiert und lückenlos in die Output-Region geschrieben.
- Zulässig sind FP32, FP16 und BF16 in Little-Endian, andere Typen ergeben „inkompatibel“.
- Degenerierte Vektoren erzeugen eine Warnung mit Anzahl, dass leere Embeddings als Nullvektor gespeichert wurden. Der Vorgang gilt als erfolgreich.
- Schreibfehler werden als Memory-Error weitergereicht.
- Auf der GPU kann ein dünner Write-Kernel den Device-Pointer des Ergebnis-Tensors lesen und normiert sowie datentypkonvertiert direkt in den vom Manager vorab vergebenen Slot schreiben. Der Tensor bleibt bis zum Event am Ende des Schreibens am Leben.

### Encoder-Schnittstelle

- Abfragbar sind Ausgabedimension (der Projection-Layer, sonst des Hauptmodells) und Storage-Datentyp.
- Input ist eine Region mit Layout im Serialisierungsformat der Batch-Bildung, Output ebenfalls eine Region mit Layout. Vorab wird geprüft, sonst „inkompatibel“ mit erwarteter und tatsächlicher Shape bzw. Datentyp:
  - Input genau 2 × Zeilen × Sequenzlänge als u32,
  - Output bei Mean-Pooling Zeilen × Dimension, sonst Zeilen × Sequenzlänge × Dimension,
  - Output-Datentyp gleich dem Storage-Datentyp.
- Das Encoding läuft asynchron auf einer übergebenen Lane. Der Aufruf kehrt nach dem Enqueue zurück und liefert ein Completion-Event.
- Ablauf: Input-Bytes lesen und in Token-IDs und Mask trennen, Hidden-States berechnen, Projection anwenden, Pooling ausführen, in die Output-Region schreiben.
- Scheitern Enqueue oder Event-Erzeugung, wird der Fehler als Device- oder Memory-Error zurückgegeben. Fehler während der Ausführung werden als Warnung „Embedding-Batch fehlgeschlagen“ geloggt und im Completion-Event hinterlegt. Die Lane selbst gilt als erfolgreich abgeschlossen.
- Das Ergebnis läuft über einen One-Shot-Channel je Task plus Event oder Notification. Der Batch merkt sich die Zeilen-Offsets je Task.
- Das Completion-Event ist nicht blockierend abfragbar (Polling) und blockierend abwartbar. Das Warten liefert einen hinterlegten Verarbeitungsfehler genau einmal. Fehler der Execution-Umgebung beim Warten werden als Device- oder Memory-Error gemeldet.

### Speicherzugriff

- Lese- und Schreibzugriff auf Regionen einer Memory-Domain laufen über eine einheitliche, nebenläufig nutzbare Schnittstelle: Bytes einer Region in einen Buffer lesen und Bytes in eine Region schreiben. Die CPU-Speicherverwaltung stellt sie bereit.
- Eine Tensor-Referenz verbindet Region und Layout und ist der einzige Weg, auf dem Tensoren die Encoder-Schnittstelle passieren.

