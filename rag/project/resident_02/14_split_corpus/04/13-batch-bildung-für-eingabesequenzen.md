## Batch-Bildung für Eingabesequenzen

- Aus einer Liste von Tokenfolgen entsteht eine Liste von Batches. Jede Folge liegt in genau einem Batch. Jeder Batch führt die ursprünglichen Positionen seiner Zeilen mit. Leere Eingabe ergibt eine leere Liste.
- Konfiguration: Token-Budget je Batch (Zeilen mal Sequenzlänge), maximale Sequenzlänge, ID des Pad-Tokens.
- Length-Bucketing:
  - Zu lange Folgen werden auf die maximale Sequenzlänge trunkiert.
  - Die gepaddete Länge ist die kleinste Bucket-Größe aus 32, 64, 128, 256 und 512, die mindestens der trunkierten Länge entspricht. Oberhalb der größten Bucket-Größe gilt die maximale Sequenzlänge.
  - Die gepaddete Länge überschreitet nie die maximale Sequenzlänge und beträgt mindestens eins.
- Gruppierung:
  - Sortierung aufsteigend nach ursprünglicher Länge, bei Gleichheit stabil in Eingabereihenfolge.
  - Vor jedem Hinzufügen wird geprüft, ob das Token-Budget mit der neuen Zeile überschritten würde. Maßgeblich ist deren gepaddete Länge, wegen der Sortierung die größte im Batch. Dann wird ein neuer Batch begonnen.
  - Eine Folge, die das Budget allein überschreitet, bildet einen eigenen Batch.
  - Die Sequenzlänge eines Batches ist die gepaddete Länge seiner längsten Zeile. Alle Zeilen sind gleich lang.
- Inhalt: Token-IDs zeilenweise als zweidimensionales Array, am Zeilenende mit dem Pad-Token aufgefüllt, dazu eine gleich geformte Attention-Mask mit eins für echte Token und null für Padding. Die Zeilenanzahl ist abfragbar.
- Serialisierung: zuerst alle Token-IDs, dann die gesamte Mask, jeder Wert als u32 in Little-Endian. Das entspricht einem dreidimensionalen Array der Form 2 × Zeilen × Sequenzlänge.
- Der Encoder nutzt ein Token-Budget statt einer Task-Anzahl. Die Batch-Größe ergibt sich aus Token-Budget, Length-Bucket und Deadline je Klasse. Die Obergrenze der Batch-Dauer begrenzt die Query-Latenz.

