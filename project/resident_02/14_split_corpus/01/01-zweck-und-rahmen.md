## Zweck und Rahmen

- Resident ist eine dauerhaft laufende Engine für zwei Aufgabenarten:
  - Embedding: Texte werden in Vektoren umgewandelt.
  - Similarity: Ähnlichkeitssuche gegen einen Index.
- Eine RAG-Anwendung (Retrieval-Augmented Generation) startet die Engine. Sie ist als Server-Prozess, als Daemon oder über die CLI nutzbar.
- Gerechnet wird in zwei getrennten Subengines, eine für die CPU und eine für die GPU (CUDA). Beide holen ihre Arbeit selbstorganisiert aus Queues (Pull-Prinzip).
- Nach der Initialisierung ist die Engine bereit, verbraucht aber noch keine Ressourcen.

### Zielumgebung

- GPU: Nvidia GeForce GTX 1660 SUPER, Compute Capability 7.5 (Turing), Treiberversion 610.57.04.
- CUDA-Toolchain: Release 13.3, Version 13.3.73, Build 38244171_0.
- GPU-Kernel werden zur Laufzeit (JIT) für die tatsächlich vorhandene Architektur kompiliert, auf der Referenzkarte also für SM 7.5.
- Turing genügt für die geforderten Loads mit Acquire-Semantik, die ab Compute Capability 7.0 verfügbar sind.
- Die Referenzkarte gehört zur Baureihe ohne Tensor Cores. Ein Tensor-Core-Pfad und BF16 als Compute-Pfad werden daher nicht angeboten.

### Modelle, Formate und Abhängigkeiten

- Das Modell multilingual-e5-small liegt als Repository unter /home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small.
- Die Modelle bilden einen festen Satz und werden vor der Initialisierung der Engine konfiguriert.
- Modelle und Tokenizer werden in kompatiblen Formaten geladen bzw. zum Laden bereitgehalten. Die Gewichte liegen im Format safetensors vor.
- Persistente Indizes liegen im Spaltenformat Parquet mit Arrow-Schema vor.
- Alle Dependencies sind bereits geprüft und in der Manifest-Datei des Projekts eingetragen: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml. Die externe Tensor-/Neural-Network-Library ist Candle.

