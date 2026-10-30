Ich benötige ein Python Script das zwei YAML-Dateien als Input nimmt, die Struktur des Baums behält und alle Requirements auf die Knoten merged.

- Knotennamen können in den Anforderungen mehrfach vorkommen
- Gemerged werden kann anhand von "Name"
- Eingabedateien und Ausgabedatei werden als Konstante konfiguriert

Baum:
```yaml
- Name: Resident
  Responsibility:
    - Embedding und Similarity als dauerhaft laufende Engine für RAG-Anwendungen (Server, Daemon, CLI) bereitstellen
    - Selbstorganisation, Entkopplung, Nicht-Blockieren, schlanken Kern und Lazy-Ressourcen als Leitprinzipien durchsetzen
  Children:
    - Name: Engine-Fassade
      Responsibility:
        - Engine über Builder mit verpflichtendem Modell konfigurieren, starten und geordnet herunterfahren
        - Planner mit Subengines, Modellen und Indizes versorgen, Tasks entgegennehmen und Indizes ohne Laden unter eindeutiger ID registrieren
      Children:
        - Name: Task-Verwaltung
          Responsibility:
            - Tasks als Arbeitseinheiten mit Parametern, Zustand und Ergebnis modellieren
```

Anforderungen:
```yaml
- Name: GPU-Kernel
  Requirements:
    - Stellt zwei Kernel bereit, dichtes Dot-Product und fused Max-Similarity.
    - Verarbeitet Dokumente in FP32 oder FP16. FP16 wird beim Laden auf FP32 konvertiert, Queries und Ergebnisse liegen in FP32.
- Name: GPU-Kernel
  Requirements:
    - Stellt zwei Kernel bereit, dichtes Dot-Product und fused Max-Similarity.
    - Verarbeitet Dokumente in FP32 oder FP16. FP16 wird beim Laden auf FP32 konvertiert, Queries und Ergebnisse liegen in FP32.
```