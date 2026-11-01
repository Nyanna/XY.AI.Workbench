Die folgenden Module bilden einen Zyklus. Löse ihn auf, indem du mindestens eine Kante des Zyklus entfernst, und zwar ausschließlich dadurch, dass das auslösende Requirement in ein anderes Modul wandert.

Vorgehen:
1. Bestimme je Kante des Zyklus das auslösende Requirement.
2. Wähle die Kante, die sich mit der geringsten Verschiebung entfernen lässt, und prüfe in dieser Reihenfolge:
   a. Das Requirement gehört fachlich in das Ziel oder ein anderes Modul: dorthin verschieben.
   b. Das Requirement besteht aus einem Vertragsteil und einem Umsetzungsteil: Vertragsteil in ein neues Schnittstellenmodul auslagern, von dem beide abhängen. Bestehende Schnittstellenmodule im Baum zuerst verwenden.
   c. Nur wenn nichts davon geht: zusammenlegen.
3. Passe die Dependencies aller betroffenen Module an, passend zu den neuen Requirement-Positionen. Tiefsten Knoten nennen.

Regeln:
- Eine Dependency darf nur entfallen, wenn das Requirement, das sie begründet hat, das Modul verlassen hat. Löschen einer Dependency bei unverändertem Requirement ist verboten.
- Jede Dependency eines Moduls muss durch ein Requirement dieses Moduls begründet sein, und jedes Requirement mit Bezug auf ein anderes Modul braucht die zugehörige Dependency.
- Keine erfundenen Aussagen, Verantwortlichkeiten oder Begriffe. Neue Module tragen nur einen Namen, ihre Requirements stammen vollständig aus dem Bestand.
- Umformulieren ist erlaubt, wenn jede ursprüngliche Aussage erhalten bleibt.
- Jedes ursprüngliche Requirement steht danach in genau einem Modul.
- Verschiedene Verantwortungen nicht in einem Requirement mischen.
- Das Ergebnis darf keinen Zyklus enthalten.

Ausgabe: ausschließlich gültiges YAML, ohne Kommentare, ohne Begründungen, ohne Text davor oder danach. Geänderte und neue Module vollständig im Komponentenformat, unveränderte weglassen. Am Ende genau ein Eintrag:
- Entfernt: [<Name>, ...]

## Zyklen
Fehler: Zyklus: Dichtes Scoring braucht GPU-Kernel braucht Dichtes Scoring
Fehler: Zyklus: GPU-Manager braucht Index-Operations-Koordinator braucht GPU-Manager

## Komponentenbaum (nur Orientierung)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/modules_optimized.yml)

## Komponentenliste
```yaml
- Name: Dichtes Scoring
  Requirements:
    - Liefert Scores der Similarity-Operation, aus denen der Similarity-Report seine Kennzahlen ableitet.
    - Führt keine L2-Normalisierung aus, da diese einmal am Ende des Encoders erfolgt.
    - Berechnet auf Anforderung des Managers die Query gegen die Centroids der Summary-Struktur eines Index und liefert die Scores, aus denen die nächsten Zellen gewählt werden.
    - Berechnet Cosine Similarity als Dot-Product, gültig für Einheitsvektoren.
    - Teilt im exakten Modus durch das Normprodukt, für abweichende Längen etwa durch FP16-Speicherung. Ein nicht positiver oder nicht endlicher Nenner ergibt null.
    - Verarbeitet Queries mit genau einem Token.
    - Lehnt Queries als leer ab, wenn das Norm-Quadrat nicht endlich oder unter 10⁻¹⁸ ist, mit Fehler "leeres Embedding" und Query-Index. Die Prüfung läuft für alle Queries vor Beginn.
    - Meldet falsche Slot-Dimension oder leere Slots als Shape-Error.
    - Liefert je Query einen Score je Slot in Slot-Reihenfolge.
    - Liefert in beiden Modi (Dot-Product und exakt) die Top-k-Treffer.
    - Ist vom Launch-Modus getrennt.
  Dependencies:
    - Kernel-Batchlauf-Schnittstelle
    - Unterstützte Precisions
    - CPU-Compute-Primitive
    - GPU-Kernel
    - Slot-Layout und Segment-View
    - Allgemeiner Index-Vertrag
- Name: GPU-Kernel
  Requirements:
    - Varianz und Indeterminismus durch Fusion oder Atomics sind zugunsten von Auslastung und Performance erlaubt.
    - Publiziert die Zahl abgeschlossener Batches mit Release-Semantik.
    - Prüft vor jedem weiteren Batch, ob das Ack den Vorgänger abdeckt. Der erste Batch darf immer starten.
    - Stellt zwei Kernel bereit, dichtes Dot-Product und fused Max-Similarity.
    - Verarbeitet Dokumente in FP32 oder FP16. FP16 wird beim Laden auf FP32 konvertiert, Queries und Ergebnisse liegen in FP32.
    - 'Dot-Product: ein Thread-Block je Slot, Teilsummen über Threads und Warps reduziert, Output Queries × Slots.'
    - 'Max-Similarity: ein Block je Dokument für genau eine Query, bei der die Zwischenmatrix nie im VRAM liegt.'
    - Begrenzt bei der Max-Similarity gepaddete Dokumente über eine Längenliste auf die Padding-Länge.
    - Setzt bei der Max-Similarity für Token ohne gültiges Dokument-Token den Wert −10 000.
    - Legt die Dimension bei der Kompilierung fest.
    - Startet mit 128 Threads je Block und einem Block je Slot (mindestens einer), ohne dynamischen Shared Memory.
    - Meldet Launch-Fehler mit Ursache.
    - Erhält Slot-Indizes, Zähler, Segmenttabelle, Query-Länge, Head und Tail dynamisch über den Parameter-Block.
  Dependencies:
    - Doorbell-Synchronisation
    - Kernel und Scoring
    - Dichtes Scoring
    - Max-Similarity-Operation
    - Unterstützte Precisions
    - Query-Batch und Dokumentquelle
    - Kernel-Batchlauf-Schnittstelle
    - Kernel-Schnittstelle und Layouts
    - Layout und Elementtypen
    - Device und Lane und Event
- Name: GPU-Manager
  Requirements:
    - Hält den GPU-Index als Segmenttabelle mit Segmenten und verwaltet ihn.
    - Synchronisiert den Index effektiv zwischen RAM und VRAM, indem er die RAM-Darstellung auf die VRAM-Darstellung für die Kernel abbildet. Synchronisiert ihn ins VRAM, bevor eine Operation auf einer freien Lane startet.
    - Spiegelt verarbeitete GPU-Ergebnisse in den Index zurück (Mirror-Back).
    - Ist Komponente der GPU-Subengine, weder der Fassade noch des Index. Wird mit der GPU-Subengine und einem lazy aktivierten Index konstruiert.
    - Löst Scopes über die Resolver-Registrierung der Scope-Auflösung auf.
    - Ensure einer Menge setzt einen angehängten Index voraus, lädt über den Loader, markiert danach alle Segmente in der Eviction als benutzt und loggt die bewegten Bytes.
    - Eine Reconcile-Iteration legt den Reconciler beim ersten Mal an, vergleicht residente Entries mit dem Index, stellt Entfernte als Delete-Updates ein, lädt Geänderte ohne ausstehendes Update regulär nach und gibt das Ergebnis zurück.
    - Publish eines geladenen Segments vergibt eine fortlaufende Segment-ID und nimmt die Region in eine neue Tabellenversion und die Segmentzuordnung in eine neue Mapping-Epoche auf. Veröffentlicht zuerst die Tabelle, dann die Zuordnung. Weist die Zuordnung ab (etwa bei Überlappung), ergibt das einen Transfer-Error. Bei Fehlern wird die Region zurückgegeben.
    - Publish nimmt das Segment bei Erfolg in die Eviction auf (Größe, Tier Vektor, Reload-Kosten gleich Größe, Zeitpunkt jetzt). Die mitgelieferten Bindings mit Segment-ID und Stamp werden als Updates in die Update-Queue eingestellt und wirken in der nächsten Gap.
    - Meldet Segmentzahl, residente Bytes aus Eviction-Sicht und ausstehende Updates.
    - Freigeben eines Segments gibt die Region zurück, entfernt das Segment aus der Eviction und löscht die Residency aller Entries. Noch anstehende Bindings in dieses Segment scheitern schadlos.
    - Stellt Eviction-Schritt und Platzbeschaffung bereit.
  Dependencies:
    - Manager
    - Allgemeiner Index-Vertrag
    - Zustandsbasierter Abgleich
    - Konsistenzmodell
    - Scope-Auflösung
    - Eviction
    - Segmente und Slots
    - Manifeste und Publication
    - Segment-Arena
    - Kernel-Schnittstelle und Layouts
    - Ladestrategie
    - Lade-Aggregation und Single-Flight
    - Index-Operations-Koordinator
    - GPU-Backend
    - GPU-Kernel
    - Region-Handle
    - Allocator mit Deferred Free
    - Arenen und Speicherbudget
- Name: Index-Operations-Koordinator
  Requirements:
    - Ist die Operations-Unit je Index-Operation und verbindet Manager, Table-Swap, Mapping-Store und Queue geladener Segmente.
    - Publiziert jedes vom Laden gemeldete Segment selbständig über das Publish des GPU-Managers.
    - Serialisiert strukturelle Änderungen.
    - Meldet ein Laden „Arena erschöpft“, publiziert er ausstehende Segmente, schafft über die Eviction Platz und wiederholt genau einmal.
    - Reconcile-Iterationen publizieren Nachgeladenes vor ihrer Rückkehr.
    - Publish versucht alle Segmente und meldet danach den ersten Fehler oder die Anzahl.
    - Verwaltungsfehler werden zu Index-Fehlern. „Arena erschöpft“ bleibt, alle anderen werden Transfer-Error.
  Dependencies:
    - GPU-Manager
    - Table-Swap
    - Address-Map und Mapping-Store
    - Update-Queue
    - Manifeste und Publication
    - Segment-Arena
    - Zustandsbasierter Abgleich
    - Ladestrategie
    - Manager-Lebenszyklus

```