Ordne alle Anforderungen aus dem Material den Modulen des Modulbaums zu.

Ziel:
- Jedes Modul ist unabhängig entwickelbar, auf Basis definierter Schnittstellen und Verantwortlichkeiten.
- Eine Anforderung gehört in das Modul, das sie ohne Kenntnis fremder Interna umsetzen kann.

Regeln:
- Jede Anforderung kommt in mindestens ein Modul.
- Erlaubt: Aufsplitten, Umformulieren, Übertragen, Zusammenführen, Anpassen.
- Keine erfundenen Anforderungen.
- Module ohne Anforderungen entfallen.

Zuordnung:
- Wähle das spezifischste Modul, dessen Responsibility die Anforderung abdeckt. Ein Elternmodul nur, wenn sie das Modul als Ganzes betrifft und kein Kind zuständig ist.
- Betrifft eine Anforderung mehrere Module, splitte sie je Modul. Jede Teilanforderung muss allein verständlich sein.
  Beispiel: "Subengines sind nicht an Kernel, Modelle oder Indizes gekoppelt" → GPU-Engine und CPU-Engine je: "Ist nicht an Kernel, Modelle oder Indizes gekoppelt."
- Prüfe bei Modulen mit Kindern jede Anforderung auf Zuordnung oder Aufteilung an ein Kind.

Beziehungen:
- Löse Beziehungen zwischen Modulen je Modul auf und benenne die Schnittstelle (Operation, Daten).
- Ein Modul hält nur Daten, die zu seiner Responsibility passen. Verschiebe die Verantwortung zum passenden Modul, statt ein generisches Modul zu erweitern.
  Beispiel: "Subengine registriert ihre Capabilities in Queues" →
  - Subengine: "Meldet ihre kompatiblen Capabilities beim Planner an."
  - Planner: "Verwaltet die Zuordnung von Capabilities zu Queues und leitet Operationen entsprechend weiter."
  - Queue: ohne Capability-Kenntnis.

Formulierung:
- Das Modul aus Name ist implizit das Subjekt. Nenne es nicht erneut.
  Beispiel: "Liefert Embeddings ohne Determinismusgarantie."

Ausgabe:
- Genau eine YAML-Liste, kein Text davor oder danach, keine YAML-Kommentare.
- Pro Modul nur die Felder Name (identisch zum Modulbaum) und Requirements (Liste).

Form:
```yaml
- Name: Modulname
  Requirements:
    - ...
```

Modulbaum:
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/modules_optimized.yml)

Material:

# Requirements
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/split_korpus/20-precision-kalibrierung-und-profile.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/split_korpus/19-kernel-und-scoring.md)
[include directory](/home/user/xyan/xy.ai.workbench/project/resident2/runs/split_korpus/06)