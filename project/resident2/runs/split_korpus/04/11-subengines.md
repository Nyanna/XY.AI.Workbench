## Subengines

### Gemeinsame Regeln

- Die Ressourcenverwaltung arbeitet lazy: erst bei Bedarf initialisieren, bei Nichtnutzung evicten. Im VRAM gelten längere Fristen.
- Jede Subengine liefert ein Capability-Set und registriert kompatible Ressourcen samt Lanes in ihren eigenen und den gemeinsamen Queues.
- Jede Subengine kalibriert ihre Hardware per Benchmark und stellt daraus Precision-Profile bereit.
- Jede Subengine implementiert ihre Compute-Pfade hardwareoptimal und entscheidet selbst.
- Für die Batch-Verarbeitung gibt es eine austauschbare Schnittstelle (Trait) von Float-Batch zu Float-Batch:
  - Die CPU-Implementierung ist Default ohne einkompilierte GPU-Beschleunigung und Fallback, wenn die GPU-Beschleunigung nicht initialisierbar ist.
  - Die GPU-Implementierung ist die einzige Anbindungsstelle der GPU-Library. Ihr Anlegen scheitert ohne kompatible GPU, wodurch automatisch auf die CPU zurückgefallen wird.

### Invarianten

- In öffentlichen Signaturen gibt es keinen Tensor und keinen Raw-Pointer, nur Region und Layout.
- Jede GPU-Arbeit läuft über einen Execution-Context (Lane), nie auf dem Default-Stream.
- Kernel werden nur über das Kernel-Register geladen. Kernel der externen Library sind dort eine weitere Implementierung.
- Planung und Zuteilung kennen weder die externe Neural-Network-Library noch GPU-Execution-Graphs (CUDA Graphs).

### CPU-Subengine

- **Device:** die CPU, potenziell mehrere (Sockets). Ressourcen sind die Cores, jeweils mit Lanes.
- **Speicherverwaltung:** lädt Modelle und Indizes ins RAM bzw. streamt Daten hinein, kennt das verfügbare RAM und lagert Modelle, Kernel und Ressourcen bei Bedarf aus und ein.
- **Kernel:** eigene Implementierungen mit eigenem Register.
- **Index:** beim ersten Zugriff geladen, mit Entry, Datei und Manager.
- **Tokenizer und Normalizer:** Ressourcen dieser Subengine.
- **Besonderheiten:**
  - Mehr Speicher steht zur Verfügung, alles nutzt die CPU.
  - Der Scorer ist selbst implementiert und nutzt SIMD.
  - Der Encoder läuft über das CPU-Backend der Library.
  - Der Index ist per mmap lesbar.
  - Keine Streams, kein Staging-Memory für Kopien.
  - Default-Datentyp ist FP32.
  - Kalibrierung und Kernel laufen auf eigenen Threads niedriger Priorität, getrennt vom Async-Runtime.

### GPU-Subengine

- **Plattform:** CUDA. Device ist die GPU, potenziell mehrere, mit Compute Units als Ressourcen und Lanes darauf.
- **VRAM-Manager:**
  - lädt oder streamt Modelle und Indizes ins VRAM,
  - kennt das verfügbare VRAM und tauscht Modelle, Kernel und Ressourcen bei Bedarf aus und ein,
  - ist die Brücke zwischen dem Index im RAM und der im VRAM abgebildeten Darstellung für die Kernel,
  - lädt Kernel sowie Input-/Output-Arenen und Speicherbereiche,
  - synchronisiert dadurch den Index effektiv zwischen RAM und VRAM.
- **Kernel:** mit Register. Das Address-Mapping im VRAM erfolgt über Slots.
- **Index:** Segmenttabelle und Segmente.
- **Besonderheiten:**
  - Das VRAM ist begrenzt.
  - Es gibt Streams, Staging-Memory und Transfers.
  - Batching ist notwendig, damit Index-Segmente und Kernel effizient gestreamt und getauscht werden.
  - Default-Datentyp ist FP16.

### Gemeinsame Ressourcen

Die folgenden Ressourcen liegen CPU-seitig (Host) und werden von beiden Subengines genutzt.

- **Modell:** Die Modellbeschreibung stammt aus der Konfiguration, beschreibt eine Pipeline und besitzt nichts. Ein Register ordnet Architektur-IDs Encoder-Implementierungen und Scoring-Arten Scorer-Implementierungen zu. Neue Modelle bedeuten einen neuen Register-Eintrag. Multi-Vector-Parameter stammen aus der Checkpoint-Konfiguration, nicht aus hartkodiertem Code: maximale Query-Länge, maximale Dokumentlänge, Marker-Token, Dimension, Skiplist.
- **Normalizer:** gehört zum Tokenizer (Unicode-Normalisierung, Lowercasing und weitere Schritte), läuft nur auf der CPU und ist über Parameter konfiguriert.
- **Tokenizer:** läuft nur auf der CPU, modellspezifisch oder geteilt. Er liefert Token-IDs als u32. Mehr braucht die GPU nicht, also kein Gathering auf der CPU und kein Address-Mapping.
- **Weight-Source:** safetensors, per mmap und zero-copy auf Rohbytes. Die Gewichte werden pro Subengine in deren Arena kopiert.

