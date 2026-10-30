Aufgabe: Leite aus dem Material eine Modulliste ab, das alle darin enthaltenen Anforderungen aufnehmen kann.

- Ein Modul ist ein Verantwortungsbereich
- Name: Nominalphrase der Fachsprache, eindeutig, in der Sprache des Materials.
- Responsibility: nur aus dem Material ableiten.
- Jede Anforderung muss sich mindestens einem Modul zuordnen lassen. Lege Module an, bis das gilt, und fasse zusammen, was vereinbare Verantwortung hat.

Ausgabe: genau eine YAML-Liste, kein Text davor oder danach, keine YAML-Kommentare.
Struktur: Jedes Modul hat genau die Felder Name, Responsibility.
- Name: Zeichenkette.
- Responsibility: Liste von Verbphrasen (1-2), kompakt.

Form:
```yaml
- Name: Modulename
  Responsibility:
    - ...
```

Material:
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/plan7.md)
Result Stats: id=d7c52633-9dcb-49ad-b2e2-e76331a8141d, total: 68146, in: 65409, out: 2737, reason: 0, read: 0, write: 0, start: 1793394282585, end: 1793394301498, time: 18895, total: 18895, io: 335, control: 18, sent: 1, recv: 471
SystemInit:  id=d7c52633-9dcb-49ad-b2e2-e76331a8141d, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
```yaml
- Name: Engine-Fassade
  Responsibility:
    - Konfiguriert und startet die Engine über einen Builder mit Pflichtmodell und registriert Indizes
    - Stellt Enqueue-Methoden bereit und fährt die Engine geordnet herunter
- Name: Task-Verwaltung
  Responsibility:
    - Kapselt Daten, Parameter, Zustand und Ergebnis von Embedding- und Similarity-Tasks samt Handle, Polling, Timeout und Cancel
    - Verwaltet erweiterbare typindizierte Zusatzparameter und Ergebnisarten
- Name: Similarity-Report
  Responsibility:
    - Erzeugt aus gescorten Treffern eine sortierte Rangliste mit Kennzahlen als reine Fakten
    - Berechnet Gap, Spread, z-Score, Softmax, Entropie und Knee-Point
- Name: Processor
  Responsibility:
    - Orchestriert Worker-Pool, Worker-Loop, Notifications, Idle-Notifications und Backpressure
    - Verarbeitet GPU-Completions und expandiert neue Tasks bedarfsgesteuert
- Name: Planner
  Responsibility:
    - Ordnet Operationen anhand der Capabilities den Queues zu und aktiviert Subengines
    - Legt Materialization-Flags fest und prüft Modellverfügbarkeit
- Name: Expander
  Responsibility:
    - Zerlegt Tasks zustandslos in geordnete Operationsketten mit Konfiguration aus Modell-Pipeline
    - Bereitet Inputs auf, übergibt Operationen schrittweise und sammelt das Ergebnis ein
- Name: Operationsmodell
  Responsibility:
    - Beschreibt Operationen, Parameter, Resource-Requirements und Operationsknoten mit Continuation-Handlern
    - Verwaltet Task-Kontext, Zähler offener Operationen, ersten Fehler und Pins von Device-Regionen
- Name: Queue
  Responsibility:
    - Stellt lock-free MPMC-Queue mit atomarem Zustandsmodell aus Claim, Completion und Cancel bereit
    - Bietet nicht blockierende Iteration, asynchrones Warten, Notification-Channels und Reference-Counting-Freigabe
- Name: Pull-Ausführung
  Responsibility:
    - Steuert je Subengine den Loop mit Filter, Lazy-Initialisierung, Claim, Batching und Eviction
    - Verwaltet Ressourcenzustände, Lanes und meldet Auslastung und Durchsatz
- Name: Subengine-Basis
  Responsibility:
    - Liefert Capability-Set, registriert Ressourcen und Lanes und wählt Compute-Pfade und CPU-Fallback
    - Erzwingt Invarianten für Kernel-Register, Lanes und Schnittstellen ohne Tensoren und Pointer
- Name: CPU-Subengine
  Responsibility:
    - Verwaltet RAM-Ressourcen, Cores, Lanes, Kernel und Indizes der CPU
    - Führt Scorer mit SIMD und Encoder über das CPU-Backend aus
- Name: GPU-Subengine
  Responsibility:
    - Verwaltet CUDA-Devices, VRAM, Streams, Lanes, Staging und Transfers
    - Führt Kernel und Encoder auf der GPU aus mit Batching
- Name: Modellverwaltung
  Responsibility:
    - Beschreibt und validiert Modelle, registriert Encoder-Architekturen und lädt Encoder je Device-Klasse
    - Lädt Gewichte aus safetensors und verwaltet Weight-Source und Modellkonfiguration
- Name: Encoder
  Responsibility:
    - Berechnet Hidden-States und wendet Projection-Layer, Pooling und L2-Normalisierung an
    - Schreibt Embeddings in Output-Regionen und liefert asynchron Completion-Events
- Name: Tokenizer
  Responsibility:
    - Lädt Tokenizer, normalisiert Texte und kodiert sie in Query- und Passage-Modus mit Präfix- oder Multi-Vector-Schema
    - Trunkiert Eingaben und bestimmt das Pad-Token
- Name: Batch-Bildung
  Responsibility:
    - Bildet längengruppierte, gepaddete Batches nach Token-Budget mit Attention-Mask
    - Serialisiert Batches und begrenzt die Batch-Dauer
- Name: Device-, Lane- und Event-Modell
  Responsibility:
    - Definiert Device-Arten, Execution-Contexts (Lanes) und Completion-Events
    - Erfasst den Laufzustand von Batches
- Name: Speicherverwaltung
  Responsibility:
    - Verwaltet Arenen, Budget, Regionen, Layouts, Elementtypen und Free-Space-Allocators mit Deferred Free
    - Stellt CPU- und GPU-Backend, Staging-Ring und Tensor-Wrapping bereit
- Name: Index
  Responsibility:
    - Hält Entries, Change-Stamps, Change-Notifications und Entry-Mengen als Source of Truth
    - Persistiert dateigestützt im Parquet-Format mit Header, Checksums, Verifikation, atomarem Save und Erkennung von Dateiänderungen
- Name: Index-Manager
  Responsibility:
    - Löst Scopes auf und verwaltet Residency, Update-Queue und zustandsbasierten Abgleich mit Echo-Suppression
    - Steuert Ladestrategie, Transfer ins VRAM, CPU-Page-Cache-Hinweise und Eviction
- Name: Segment- und Manifest-Verwaltung
  Responsibility:
    - Verwaltet Segmente, Slots, Manifeste, Segmenttabellen, Epochen und Table-Swap per Copy-on-Write
    - Löst Kernel-Treffer über Address-Map auf Entries auf und synchronisiert Batches per Doorbell und Communication-Slot
- Name: Kernel-Index-Aufbau
  Responsibility:
    - Schreibt Embeddings durch Kernel per Write-Pointer in vorab vergebene Bereiche
    - Verwaltet Partial Results und getrennte Queues für Writes und Treffer
- Name: Kernel-Register
  Responsibility:
    - Kompiliert und lädt Kernel lazy zur Laufzeit für die Zielarchitektur
    - Meldet unterstützte Precisions je Device-Art
- Name: Scoring
  Responsibility:
    - Berechnet dichtes Scoring und Max-Similarity auf CPU und GPU mit Query-Batches und Top-k-Selektion
    - Plant Work-Chunks, Launch-Modi und vergleicht mit einem Referenz-Scorer
- Name: Kalibrierung
  Responsibility:
    - Erkennt Capabilities, misst Kombinationen im Benchmark und bildet Pareto-Front und Precision-Profile
    - Wählt Profile nach Precision-Wunsch, persistiert Ergebnisse und kalibriert im Hintergrund
- Name: Hintergrundausführung
  Responsibility:
    - Stellt Low-Priority-Pools mit Futures und Statistik-Schätzern bereit
```
