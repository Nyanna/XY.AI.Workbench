## Devices, Lanes und Events

- Device-Arten sind CPU und GPU, erweiterbar. Das Device-Handle besteht aus dem Device-Ordinal bzw. der CPU.
- Bei der GPU gibt es einen Prozess, einen Primary Context und mehrere Streams.
- Ein Execution-Context (Lane) besteht aus Stream, Priorität, Events, eigenem Parameter-Block und Workspace.
- Beispiel-Lanes: Query (hohe Priorität, kleine Batches), Insert und Update im Index (Throughput), Kopieren zwischen RAM und VRAM in beide Richtungen (H2D und D2H).
- Übergaben zwischen Lanes laufen ausschließlich über die Queue.
- Ein Event ist ein opaker Completion-Marker für alle vor seiner Aufzeichnung auf einer Lane eingereichte Arbeit:
  - Kopien beziehen sich auf dasselbe Event.
  - Der Abschluss ist nicht blockierend abfragbar und blockierend abwartbar. Bei GPU-Events meldet das Warten Fehler als Event-Error mit Ursache.
  - Ein CPU-Event bleibt nach dem Signalisieren dauerhaft abgeschlossen und weckt alle Wartenden.
  - Die Textdarstellung (Display) zeigt den Completion-Status.
- Der Laufzustand wird über ein Event je Batch erfasst. Im Persistent-Modus dient ein Progress-Counter in gemapptem Host-Memory.

