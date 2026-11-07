## Pull-Ausführung der Subengines

### Loop und Iteration

- Pro Subengine läuft ein Loop mit genau einer Wartebedingung, der Notification. Danach folgt eine Iteration über die Queue.
- Die Iteration geht in Altersreihenfolge Knoten für Knoten vor:
  - Sind alle Ressourcen der Subengine ausgelastet, endet die Iteration.
  - Akzeptiert der Filter die Parameter nicht, wird der Knoten übersprungen. Prioritätspräferenz ergibt sich aus Filter und Reihenfolge, also aus Metadaten.
  - Der Ressourceneintrag wird über die Requirements der Operation bestimmt.
  - Ist die Ressource nicht initialisiert, wird die Lazy-Initialisierung angestoßen: atomarer Übergang von „uninitialized“ nach „loading“. Der Task bleibt in der Queue.
  - Ist die Ressource ready, hat freie Kapazität und gelingt der Claim, führt die Ressource den Knoten aus.
  - In allen anderen Zuständen (loading, busy, backoff) wird der Knoten übersprungen.
- Am Ende jeder Iteration werden ungenutzte Ressourcen evicted.

### Lanes, Batches und Ressourcen

- Die Lanes einer Ressource ziehen Operationen. Die Lane der CPU ist ein Thread des Processors.
- Lanes implementieren Batching: Der Batch beginnt mit der Ausgangsoperation. Die Ressource geht von dort vorwärts durch die Liste, übernimmt alles, was der Filter akzeptiert und in den Batch passt, und startet dann den Batch.
- Packing, Length-Bucketing und Streams sind reine Ressourcenlogik.
- Das Ergebnis löst die Notification der Subengine aus.
- Ressourcenzustände: uninitialized, loading, ready, backoff (nach Fehler).

### Notification-Topologie

- Append an eine Queue: Weiterleitung an die Notification jeder Subengine.
- Ergebnis, abgeschlossene Initialisierung und Eviction-Timer: eigene Notification der Subengine.
- Eine Iteration mit Fortschritt löst die eigene Notification aus. Der Loop ist seine eigene Notification.

### Regeln

- Keine Reservierung, kein Vorab-Claim, kein Head-of-Line-Blocking.
- Lazy-Initialisierung geschieht nur durch die Iteration, nie durch das Enqueue.
- Eviction ist nur erlaubt, wenn keine Operation läuft und die Lebensdauer (TTL) abgelaufen ist. Im VRAM ist sie deutlich länger als im RAM.
- Initialisierungs- oder Startfehler schreiben einen Fehler in die Task-Ergebnisse des Batches und versetzen die Ressource in den Backoff. Das wird mit Task-ID geloggt.

### Auslastungsmeldung

- Eine Subengine stellt Ressourcen und Execution-Kapazität eines Compute-Substrats bereit und ist aus mehreren Threads nutzbar.
- Sie gibt Auskunft über Art, Device, Kapazität und geglätteten Durchsatz in Entries pro Sekunde.
- Die Arbeit kommt aus den Queues des Processors. Die Subengine meldet nur, wie viel sie aufnehmen kann.

