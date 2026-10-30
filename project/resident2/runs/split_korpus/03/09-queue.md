## Queue

### Zweck und Einsatz

- Die Queue ist eine lock-free MPMC-Struktur (Multi-Producer, Multi-Consumer) für Arbeitseinheiten mit eigenem Zustand, Ergebnis und Notification.
- Sie wird viermal eingesetzt: als Task-Queue und als Operations-Queue für CPU, GPU und gemeinsam. Jede arbeitet unabhängig mit eigenem Set von Notification-Channels.
- Typen von Metadaten, Payload, Ergebnis und Fehler legt der Aufrufer fest. Operations-Queues verwenden denselben Aufbau mit eigenen Metadaten und Werten als Payload.
- Bei Operationen wird die Fertigstellung nicht über das Ergebnis der Zustandseinheit verfolgt, sondern über den Zähler offener Operationen des Tasks.

### Struktur

- Jede Queue ist genau eine einfach verkettete Liste mit internem Sentinel-Knoten, ohne getrennte Listen je Priorität oder Ressource.
- Es gibt nur Append und Remove. Append geschieht per atomarem Compare-and-Swap am Next-Pointer des letzten Knotens, Unlink ebenfalls per Compare-and-Swap auf dem Pointer.
- Die Reihenfolge entspricht dem Alter, also dem erfolgreichen Append. Die Reihenfolge eines einzelnen Producers bleibt erhalten.
- Es gibt keine Version, keinen Snapshot und keinen Lock auf der Kette. Nur das Payload-Fach eines einzelnen Knotens ist kurzzeitig gegen gleichzeitigen Zugriff geschützt.
- Metadaten sind für die Queue opak. Sie werden nicht interpretiert, sondern Consumern zum Filtern bereitgestellt.

### Aufbau eines Knotens

- Immutable Parameter zum Filtern vor dem Claim.
- Gemeinsames Zustandsobjekt des Tasks, zugleich Handle des Aufrufers, mit atomarem Zustand, einmal setzbarem Ergebnis und Notification für Wartende.
- Geschütztes, optionales Payload-Fach, das beim Claim geleert wird.
- Atomar austauschbarer Pointer auf den Nachfolger.

### Enqueue

- Beliebig viele Einträge, jeder aus Metadaten, Payload und Zustandseinheit. Der Aufrufer erhält eine Referenz auf den neuen Eintrag.
- Enqueue blockiert nie. Mehrere Producer enqueuen gleichzeitig, ohne dass ein Eintrag verloren geht oder überschrieben wird.
- Ist der interne Tail-Hint veraltet, wird das tatsächliche Ende gesucht und die Reihenfolge gewahrt. Zu Testzwecken lässt sich das Aktualisieren des Tail-Hints gezielt unterdrücken.
- Beim Anlegen nimmt die Queue eine beliebige, auch leere Menge von Notification-Channels entgegen, je Subengine einen. Jedes Enqueue signalisiert alle Channels.

### Iteration

- Es gibt eine Iteration über alle Einträge in Altersreihenfolge und eine, die nur aktive Einträge liefert (weder completed noch cancelled). Der Sentinel erscheint nie.
- Iterationen blockieren nie und bleiben bei gleichzeitigem Enqueue, Claim und Completion gültig. Jeder gelieferte Eintrag bleibt gültig, auch nach logischem Entfernen.
- Die Iteration liefert exakt die Einträge in Enqueue-Reihenfolge, ohne Auslassung und Wiederholung. Eine leere Queue ergibt eine leere Iteration.
- Completed und cancelled Einträge gelten als logisch entfernt und werden in der aktiven Iteration übersprungen. Die vollständige Iteration liefert sie, solange sie nicht entlinkt sind.

### Zustandsmodell und Übergänge

- Vier Zustände: enqueued, claimed, completed, cancelled. Neue Einträge sind enqueued.
- Erlaubt sind ausschließlich diese atomaren Übergänge, bei gleichzeitigen Versuchen gelingt höchstens einer:
  - **Claim:** enqueued → claimed. Einzige Stelle, an der ein Abbruch relevant ist.
  - **Completion:** claimed → completed.
  - **Cancel:** enqueued → cancelled.
- Claim und Cancel schließen sich gegenseitig aus. Der Endzustand passt immer zum Gewinner. Der Zustand ist jederzeit als konsistenter Snapshot abfragbar. Ein ungültiger interner Zustandswert ist ein nicht behebbarer interner Fehler.
- Claim:
  - Der Claimer erhält die Payload per Ownership-Transfer, nie als Kopie, genau einmal.
  - Erneutes Claimen, Claim eines abgebrochenen Eintrags oder des Sentinels scheitert ohne Payload.
  - Bei gleichzeitigen Versuchen gewinnt genau einer. Der Claim blockiert nie und wartet nicht, während das Payload-Fach gesperrt ist.
  - Er ist auch über eine Funktion der Queue verfügbar, mit identischem Verhalten. Jeder Versuch wird mit Ausgang geloggt.
- Completion:
  - Nur der erfolgreiche Claimer darf abschließen, und nur einmal. Das Ergebnis ist Erfolgs- oder Fehlerwert.
  - Das Ergebnis ist gespeichert und sichtbar, bevor der Zustand „completed“ erkennbar wird.
  - Ein zweiter Versuch verändert das Ergebnis nicht, wird mit Warnung geloggt und fällt in Entwicklungsumgebungen als Programmierfehler auf. Bei „cancelled“ wird das Ergebnis verworfen.
  - Wartende werden geweckt. Ohne gesetztes Ergebnis liefert die Abfrage nichts, abgebrochene Einträge haben nie ein Ergebnis.
- Cancel:
  - Nur aus „enqueued“ möglich, mit Rückmeldung über Erfolg. Danach ist kein Claim mehr möglich.
  - In jedem anderen Zustand ändert sich nichts und es wird Misserfolg gemeldet.
  - Ein erfolgreicher Cancel weckt alle Wartenden. Jeder Versuch wird geloggt.

### Warten auf Abschluss

- Asynchrones Warten auf „completed“ oder „cancelled“, ohne einen Thread zu blockieren. Bei bereits erreichtem Endzustand endet es sofort.
- Keine Notification geht verloren, auch wenn der Abschluss zwischen Zustandsprüfung und Wartebeginn eintritt.
- Mehrere Wartende werden gleichzeitig unterstützt und alle geweckt. Ein Wakeup ohne Endzustand führt zu erneuter Prüfung.

### Ownership und Speicherfreigabe

- Ownership-Kette: Queue-Head, Kette, Iterator. Jeder Iterator hält pro Schritt eine Referenz auf den Knoten.
- Nach dem Claim bleibt die leere Hülle in der Kette, bis sie entlinkt wurde und der letzte Iterator sie verlassen hat. Dann wird sie automatisch freigegeben.
- Unlink ist optionale Aufräumarbeit: ein einfacher atomarer Swap auf dem Pointer des Vorgängers. Der entlinkte Knoten behält seinen Nachfolger, damit ein Iterator ohne Neustart weiterläuft. Nicht entlinkte Knoten werden spätestens beim Drop der Queue freigegeben.
- Freigabe über Reference Counting der Knoten. Epoch-based Reclamation, Slab-Verwaltung, Freelist und Generation-Counter entfallen für die Queue.
- Lange Ketten, auch über hunderttausend Einträge, werden iterativ statt rekursiv freigegeben, ohne Stack-Overflow. Von anderen Ownern gehaltene Kettenteile bleiben gültig, bis deren letzter Owner sie freigibt.
- Außerhalb der einen Komponente für den atomaren Pointer-Swap darf kein Code eingesetzt werden, der die Memory-Safety-Garantien der Sprache umgeht (kein `unsafe`).

