## Typischer Anwendungsfall

Der Ablauf folgt dieser Reihenfolge:

- Die Anwendung startet, die Engine wird instanziiert und konfiguriert. Indizes können registriert werden, werden aber weder geladen noch verwendet.
- Die Engine wird initialisiert. Der interne Executor des Processors startet die Mindestzahl idle Threads.
- Die Engine ist bereit, verbraucht aber noch keine Modell- oder Device-Ressourcen.
- Tasks für verschiedene Aufgaben werden über die Engine enqueued.
- Der Expander im Planner zerlegt die Tasks in Operationsketten. Der Planner stellt fest, welche Subengines konfiguriert sind, und startet sie.
- Benötigt eine Operation bestimmte Ressourcen, wird das den Subengines mitgeteilt. Diese registrieren kompatible Ressourcen (Capabilities) in den ihnen zugewiesenen und in den gemeinsamen Queues.
- Der Planner weist die Operationen anhand der kompatiblen Capabilities den Queues zu, vorzugsweise der gemeinsamen.
- Sobald eine aktivierte Lane frei ist, zieht sie eine Operation und initialisiert die Ressourcen:
  - Tokenizer und Normalisierung werden lazy initialisiert.
  - Kernel werden lazy kompiliert und geladen.
  - Indizes und Modelle werden geladen.
  - Wenn sinnvoll, werden weitere Operationen, die mit demselben Index oder Modell laufen können oder müssen, zu einem Batch aggregiert.
- Ergebnisse werden bereitgehalten und können abgeholt werden, per Event oder Notification.
- Nach einer bestimmten Zeit werden unbenutzte Ressourcen entladen, aber nicht deaktiviert. Die Unload-Fristen für Modelle und Indizes im VRAM sind wesentlich länger als im RAM.
- Normalisierung bezeichnet die zum Tokenizer gehörende. Zusätzlich kann es eine externe, vorgeschaltete Textnormalisierung für die Chunks geben.

