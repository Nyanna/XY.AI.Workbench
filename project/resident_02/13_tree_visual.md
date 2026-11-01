📄 all_reqs_dedup_merged_tree.yml
└── Resident
    ├── Engine-Fassade
    │   ├── Task-Verwaltung
    │   │   ├── Task-Modell
    │   │   │   ├── Task-Handle
    │   │   │   ├── Erweiterbare Zusatzparameter
    │   │   │   ├── Embedding-Task
    │   │   │   └── Similarity-Task
    │   │   └── Ergebnisarten
    │   │       └── Similarity-Report
    │   └── Addons
    ├── Processor
    │   ├── Planner
    │   │   ├── Expander
    │   │   │   └── Schrittweise Übergabe und Ergebnisaufbereitung
    │   │   └── Operation
    │   │       ├── Resource-Requirements
    │   │       └── Operationsknoten
    │   │           ├── Materialisierung
    │   │           └── Task-Kontext
    │   ├── Queue
    │   │   ├── Queue-Struktur und Enqueue
    │   │   │   ├── Queue-Iteration
    │   │   │   └── Queue-Speicherfreigabe
    │   │   └── Queue-Zustandsmodell
    │   │       └── Abschluss-Warten
    │   └── Worker-Pool
    │       └── Worker-Loop
    │           └── Backpressure und Expansionssteuerung
    ├── Subengine
    │   ├── Gemeinsame Subengine-Regeln
    │   │   ├── Device, Lane und Event
    │   │   ├── Ressourcen-Lebenszyklus
    │   │   │   └── Auslastungsmeldung
    │   │   ├── Precision, Kalibrierung und Profile
    │   │   │   ├── Capability-Modell
    │   │   │   │   └── Capability-Erkennung
    │   │   │   ├── Kalibrierungslauf
    │   │   │   │   ├── Benchmark-Workload und Abweichungsmaße
    │   │   │   │   ├── Messung
    │   │   │   │   │   └── Hintergrundausführung und Statistik
    │   │   │   │   └── Pareto-Front und Profilableitung
    │   │   │   └── Profilauswahl
    │   │   │       └── Kalibrierungs-Persistenz
    │   │   └── Kernel und Scoring
    │   │       ├── Unterstützte Precisions
    │   │       ├── Batch-Compute-Schnittstelle
    │   │       ├── Query-Batch und Dokumentquelle
    │   │       │   └── Dichtes Scoring
    │   │       │       ├── Max-Similarity-Operation
    │   │       │       ├── Top-k-Selektion
    │   │       │       └── Referenz-Scorer-Vergleich
    │   │       └── Kernel-Batchlauf-Schnittstelle
    │   │           └── Work-Chunks
    │   ├── Pull-Ausführung
    │   │   ├── Subengine-Loop
    │   │   ├── Lane und Batching
    │   │   └── Notification-Topologie
    │   ├── CPU-Subengine
    │   │   ├── CPU-Backend
    │   │   ├── CPU-Compute-Primitive
    │   │   └── CPU-Manager
    │   │       └── Page-Cache-Hinweise und blockierende Lesezugriffe
    │   └── GPU-Subengine
    │       ├── GPU-Completion
    │       ├── GPU-Backend
    │       │   └── Staging-Ring
    │       ├── GPU-Kernel
    │       │   ├── Kernel-Register
    │       │   └── Launch-Modi
    │       └── GPU-Manager
    │           ├── Device-Transfer
    │           └── Kernel-Synchronisation
    │               ├── Mutationsdisziplin
    │               │   └── Table-Swap
    │               ├── Doorbell-Synchronisation
    │               │   └── Communication-Slot
    │               │       ├── Address-Map und Mapping-Store
    │               │       └── Update-Queue
    │               └── Index-Aufbau durch Kernel
    │                   ├── Write-Pointer-Bereich
    │                   └── Embedding-Batch-Ausführung
    │                       └── Write- und Treffer-Queues
    ├── Gemeinsame Host-Ressourcen
    │   ├── Modell
    │   │   ├── Modellbeschreibung
    │   │   ├── Modellregister
    │   │   │   ├── Gewichtsquelle
    │   │   │   └── Encoder-Aufbau
    │   │   │       ├── Transformer-Hauptmodell
    │   │   │       ├── Projection-Layer
    │   │   │       └── Pooling und Normalisierung
    │   │   └── Encoder-Schnittstelle
    │   │       └── Embedding-Ausgabe
    │   ├── Tokenizer
    │   │   ├── Tokenizer-Laden
    │   │   │   └── Textnormalisierung
    │   │   └── Encoding-Schemata
    │   │       └── Text-Encoding
    │   │           └── Batch-Bildung
    │   ├── Speicherverwaltung
    │   │   ├── Arenen und Speicherbudget
    │   │   ├── Free-Space-Verwaltung
    │   │   │   └── Allocator mit Deferred Free
    │   │   ├── Region-Handle
    │   │   │   ├── Bookkeeping und Gültigkeit
    │   │   │   ├── Layout und Elementtypen
    │   │   │   └── Speicherzugriff
    │   │   │       └── Region-Pinning
    │   │   └── Library-Adapter
    │   │       └── Zielumgebung und Abhängigkeiten
    │   └── Index
    │       ├── Allgemeiner Index-Vertrag
    │       │   ├── Change-Stamp
    │       │   └── Scopes und Entry-Mengen
    │       ├── Referenz-Index
    │       └── Dateigestützter Index
    │           ├── Index-Öffnung und Lazy Loading
    │           │   └── Dateiformat und Modellbindung
    │           ├── Entry-Verwaltung im Dateiindex
    │           │   ├── Nutzdaten und Checksum
    │           │   └── Persistentes Sichern
    │           └── Erkennung von Dateiänderungen
    └── Manager
        ├── Manager-Lebenszyklus
        ├── Index-Konsistenz
        │   ├── Konsistenzmodell
        │   │   └── Konsistente Snapshots
        │   └── Zustandsbasierter Abgleich
        ├── Segmentverwaltung
        │   ├── Kernel-Schnittstelle und Layouts
        │   ├── Segmente und Slots
        │   │   ├── Slot-Layout und Segment-View
        │   │   ├── Segment-Schreibpfad
        │   │   └── Treffer-Auflösung
        │   ├── Manifeste und Publication
        │   └── Segment-Arena
        └── Residency-Steuerung
            ├── Scope-Auflösung
            ├── Ladestrategie
            │   └── Lade-Aggregation und Single-Flight
            └── Index-Operations-Koordinator
                └── Eviction
