## Processor

### Struktur

- Der Processor besitzt die Task-Queue, drei Operations-Queues (CPU, GPU, gemeinsam), den Parkplatz der Worker (Worker-Pool), die Notification und die Idle-Notifications.
- Er ruft den Planner für die Expansion.
- Der interne Executor startet bei der Initialisierung eine Mindestzahl idle Threads (Worker).
- Eine Subengine prüft zuerst ihre eigene Operations-Queue, danach die gemeinsame.
- Subengines besitzen ihre Ressourcen: Tokenizer, Encoder, Scorer, Index-Segmente. Der Tokenizer ist eine Ressource der CPU-Subengine.

### Worker

- Die Zahl der Worker der gesamten Engine ist die Zahl der CPU-Cores minus eins. Das ist eine Obergrenze für Context-Switches, keine Core-Zuordnung (kein Pinning).
- Worker parken im Processor und wachen bei jeder Zustandsänderung per Notification auf.
- CPU: Der Worker führt die Operation synchron aus und kommt mit dem Ergebnis zurück.
- GPU: Der Worker startet die Operation bzw. den Kernel und kehrt zurück. Nach der Completion-Notification verarbeitet ein freier Worker das Ergebnis.
- Blockiert ein Worker, etwa in der Compute-Library oder deren Thread-Pool, ist das das natürliche Backpressure-Signal ohne Sonderbehandlung.

### Worker-Loop

Nach jeder Operation wird in dieser Prioritätsreihenfolge neu geprüft:

- GPU-Ergebnisse verarbeiten und in den Index zurückspiegeln (Mirror-Back).
- GPU-Ressourcen füttern: Index ins VRAM synchronisieren, bei freier Lane starten.
- CPU-Ressourcen füttern, ausführen, Ergebnis zurückgeben, auch an Indizes.
- Neue Tasks expandieren (Planner).

### Completion auf der GPU

- Der Callback am Ende des Streams legt nur einen Completion-Eintrag ab und löst die Notification aus. Im Callback erfolgt kein Aufruf der GPU-API.
- Der Schritt zur Ergebnisverarbeitung prüft zusätzlich per Event-Query und übernimmt eine fertige Lane per atomarem Swap. Das deckt verpasste Callbacks ab.

### Backpressure und Expansion

- Eine Iteration ohne Treffer bei freier Ressource erzeugt eine Idle-Notification mit Ressourcenart und Parametern.
- Die Notifications liegen als Set je Subengine-Art vor, nicht als Queue. Sie verfallen bei einem Treffer oder wenn die Ressource belegt ist.
- Expandiert wird nur bei einer Idle-Notification oder leerer Operations-Queue. Bevorzugt werden Tasks, deren erste Operation die unterversorgte Ressource braucht.

