## Komponentenstruktur

Die Wurzel gliedert sich in die Engine und gemeinsam genutzte Bausteine.

- Engine: die Fassade (Facade) nach außen.
  - Processor: Orchestrierung mit Workern, Queues, Notification und Backpressure.
  - Planner: ordnet Operationen den Queues zu.
    - Expander: zerlegt Tasks in Operationsketten.
    - Operation: Operationsarten der Embedding- und Similarity-Ketten.
  - Subengine: Ressourcen- und Speicherverwaltung je Hardwareart.
    - CPU: Device mit Ressourcen (Cores) und Lanes. Speicher mit Kernel, Index (Entry, Datei im Parquet-Format, Manager), Tokenizer und Normalizer.
    - GPU: Device mit Ressourcen (Compute Units, Streaming Multiprocessors) und Lanes. Speicher mit Kernel und Index (Segmenttabelle, Segment, Manager).
    - Gemeinsam: Modell.
  - Task: Embedding-Task und Similarity-Task.
- Gemeinsam: Queue.

