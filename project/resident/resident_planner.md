# Resident – Planner (reine Taskexpansion)

Soft-Prompt-Notizen. Zustandslos, ohne Orchestrierung.

## Aufgabe
- Nimmt einen Task (Daten + Parameter) und expandiert ihn zu einer Folge von Operationen.
- Übersetzt Task-Parameter in konkrete Operation-Parameter-Konfiguration: welcher Index, welcher Tokenizer, welches Modell, Scorer, Pooling, Präfix.
- Arbeitet nur mit Config-Regeln und den Capabilities der Subengines.
- Reine Funktion: gleiche Eingabe, gleiche Operationen. Keine Queues, keine Worker, kein Notify.

## Eingaben
- Task: Daten + Parameterobjekt (Embedding, Similarity, Inferenz).
- Pipeline-Config pro Modell (Präfixe, Pooling, Normalisierung, `max_seq_len`, dtype).
- Subengine-Capabilities: welche Ressourcenarten jede Engine anbietet (z. B. Tokenizer nur CPU, Encoder CPU und GPU, Scorer CPU und GPU).
- Optional: Starved-Hint des Processors als Präferenz.

## Ausgabe: Operation
- `params` (unveränderlich, zum Filtern vor dem Claim) mit `requires()`: Ressourcenart, Modell, Index, erlaubte Engine.
- Priorität und Task-ID als Metadaten.
- `materialize: bool`: Ergebnis wird beim Fertigwerden in den Host zurückgeholt oder bleibt als Device-Handle liegen.
- Eingabe/Ausgabe als `Value::Host(data) | Value::Device(handle)`.
- Operation gehört zu genau einem Task. Der Graph ist implizit durch die Reihenfolge der Operationen eines Tasks gegeben.

## Expansionsregeln (Beispiele)
- Embedding: `Tokenize(CPU) → Embed(CPU|GPU) → Pool + L2 → optional WriteIndex`.
- Query-Similarity: `Tokenize → Embed → Similarity(Scope) → TopK`.
- Similarity mit CrossEncoder: Paare (Query, Text aus Payload) → `Infer(CrossEncoder)` → Scores, Scope aus früherem Ergebnis.
- Gleitende Indizierung (Ausblick): Embedding-Input ist großer Chunk, Parameter konfigurieren den gleitenden Kernel.

## Regel für `materialize`
- `true`, wenn der Nachfolger auf der CPU läuft, der Task endet oder das Ergebnis in den Index gespiegelt werden muss.
- `false`, wenn ein GPU-Nachfolger folgt, dann bleibt das Ergebnis als Handle im VRAM.
- Verantwortung liegt beim Planner, die Operation führt sie aus.

## Device-Wahl
- Stage ohne Device-Festlegung: Planner trägt nur die Ressourcenart ein, Zuordnung entscheidet der Claim der Subengine.
- Keine Affinität zu Handles vorerst: zwei Engines, jede Ressourcenart höchstens einmal pro Device, `requires()` bestimmt die Engine eindeutig.
- Erweiterungspunkt: Affinitätsfeld in `requires()`, falls dieselbe Art später auf beiden Devices existiert.

## Folgeoperationen und Abschluss
- Folgeoperationen werden am Ende der Queue eingereiht, vor dem Dekrement des Vorgängers.
- Task führt Zähler offener Operationen, bei 0 ist er `Done`.
- Erster Fehler setzt `Cancelled` und `Err`, übrige Operationen scheitern beim Claim.

## Fehler und Logging
- Strukturierte Fehler mit `thiserror` (ungültige Config, fehlende Capability, unbekanntes Modell).
- `tracing`-Span pro Task-ID.

## Offen
- Konfigurationsformat der Expansionsregeln (Registry-Eintrag pro Modell/Tasktyp).
- Wie der Starved-Hint die Expansion gewichtet (Präferenz, kein Vertrag).