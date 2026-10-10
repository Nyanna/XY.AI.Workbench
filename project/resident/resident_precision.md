# Resident: Präzision, Kalibrierung und Profile (Ergänzung)

Ergänzung zum Konzept der "Resident"-Engine (`resident.md`). Wird separat umgesetzt. Betrifft Rechenpräzision, Kalibrierung und Task-Matching der Subengines (CPU, GPU/CUDA). Stack unverändert (candle, cudarc, tokenizers, tokio).

## Leitprinzipien

- Subengines, Kernel, Modelle und Indizes sind **nicht gekoppelt**.
- Varianz und Indeterminismus sind zugunsten von Auslastung und Performance erlaubt (Fusion, Atomics, unterschiedliche Rundung je Hardware).
- Ergebnisse von Embedding und Vergleich sind **akzeptierte Variablen**, keine Garantien.
- Jede Subengine implementiert Rechenpfade **hardwareoptimal und selbst entscheidend**. Es gibt keinen Rundungspunkt- oder dtype-Vertrag zwischen CPU und GPU.
- Akkumulierende Genauigkeitsverluste sind ein Problem der äußeren Schichten (Chunking, Query, Indexstruktur, Reranking, Kandidaten-k), nicht der Engine. Die Engine verbürgt keine Qualität.
- Der Task-Parameter für Präzision ist ein **Wunsch**, keine Zusicherung.
- Die Kalibrierung darf den Nutzer nie unterbrechen oder blockieren.

## Index und Modell: einzige harte Bindung

- Der Index ist ein eigenes Objekt. Seine RAM- und Persistenzrepräsentation ist dieselbe Instanz, VRAM ist ein abgeleiteter Spiegel.
- Der Index kennt weder Engine noch Profil.
- Die einzige Kompatibilitätsbedingung ist **Modell-ID und Dimension** im Index-Header. Vektoren verschiedener Modelle sind nicht vergleichbar.
- Ein Metadatum "erzeugendes Profil" ist höchstens optionale Diagnose, keine Pflicht.

## Capability-Listen statt Profile

- Jede Subengine liefert ein `CapabilitySet`: pro Parameterachse die verfügbaren Werte.
- Beispiele GPU: Operanden `{fp32, fp16}`, Akkumulator `{fp32, fp16}`, Scoring `{fp32, fp16, int8-dp4a}`, Fusion `{an, aus}`, Batchgröße, Tile-Größe.
- Beispiele CPU: Operanden `{fp32, fp16→fp32}`, Scoring `{fp32, int8}`, Threads, Tile-Größe.
- Es gibt **keine** vordefinierten Profilnamen und kein fest codiertes Profil-Enum.
- Achsen und Werte sind pro Hardware erweiterbar, ohne Änderung an Fassade oder Task-Schnittstelle.

## Calibrator (eigene Komponente)

Aufgaben: Aufzählung, Pruning, Messung, Pareto-Auswahl, Profilbildung, Persistenz.

### Ablauf

1. Kombinationen aus dem `CapabilitySet` aufzählen (kartesisch, mit Pruning).
2. Jede Kombination als Mini-Benchmark messen, dabei gleichzeitig Abweichung gegen die fp32-Referenz erfassen.
3. Ungültige Kombinationen verwerfen: NaN/Inf, degenerierte Ausgabe (konstante Vektoren, Norm außerhalb des plausiblen Bereichs), Abweichung über der konfigurierbaren **Maximalabweichung** (= inkompatibel, kein Fehler im Betrieb, nur nicht aktiviert).
4. Pareto-Front aus Zeit und Abweichung bilden.
5. Profile aus der Front ableiten (siehe unten).
6. Ergebnis persistieren.

### Messmethodik (jeder Check ist ein Mini-Benchmark)

- Mindestens 3 Wiederholungen pro Kombination.
- Kombinationen **interleaved** messen (nicht A komplett, dann B), damit Lastschwankungen alle gleich treffen.
- Nur **Relationen** zählen, keine Absolutwerte. Stabilitätsprüfung auf dem Verhältnis zwischen Kombinationen, nicht auf Einzelzeiten.
- Robuste Statistik: Median oder Minimum statt Mittelwert (Last verzerrt nur nach oben).
- Adaptive Entscheidung nach Streuung (Variationskoeffizient oder relative Spannweite):
  - Streuung klein: stabil, fertig.
  - Samples intern verrauscht: mehr Samples pro Wiederholung.
  - Wiederholungen driften auseinander, Samples intern stabil: ganzen Durchlauf wiederholen.
- Abbruch bei Stabilität, nicht nach fester Zeit.
- Harte Obergrenze (maximale Wiederholungen oder Zeitbudget). Bei Erreichen: besten Zwischenstand verwenden, als `unstable` loggen und persistieren, nicht blockieren, nicht abbrechen.
- `unstable`-Ergebnisse dürfen bei nächster Gelegenheit (manuell aktivierte Kalibirerung) nachgemessen werden.
- Niedrige Thread-Priorität. Auf der GPU gibt es keine Preemption: Testset und Einzelläufe kurz halten, damit wartende Tasks nicht verzögert werden.
- Das Testset enthält Längenvarianz bis zur konfigurierten Maximalsequenzlänge (Prüfung der Verarbeitbarkeit: Speicher, Overflow), nicht der Retrieval-Qualität.

### Kombinatorische Explosion begrenzen

- Achsen mit unabhängigem Effekt getrennt messen (z. B. Batch- und Tile-Größe separat von dtype), dann nur Gewinner kombinieren.
- Dominierte Kombinationen früh abbrechen (bereits langsamer und ungenauer als eine bekannte).
- Es wird nur die Pareto-Front benötigt, Pruning ist daher verlustfrei bezogen auf das Ergebnis.

## Profilbildung

- Profile werden **allein aus dem Spread** der Pareto-Front abgeleitet. Die Anzahl ist variabel (1 bis n), nicht auf drei festgelegt.
- Liegen Punkte nicht ausreichend auseinander, werden sie zusammengefasst. Es werden keine künstlichen Profile erzeugt.
- Der Spread-Schwellwert ist die **einzige** Konfiguration, die die Profilanzahl bestimmt. Er ist in der **normierten Skala** definiert (modell- und hardwareunabhängig).
- Redundante Profile (praktisch identische Zeit und identisches Ergebnis, z. B. gleicher Emulationspfad) werden nicht aktiviert.
- Profile sind **unbenannte Indizes** mit einer Position `p ∈ [0,1]`.
- Positionsbestimmung:
  - Koordinaten normieren (Zeit und Abweichung jeweils auf 0..1, Zeit z. B. logarithmisch).
  - Bogenlänge entlang der Front berechnen.
  - Schnellstes Profil bei `p = 0`, genauestes bei `p = 1`, übrige proportional zur kumulierten Distanz (keine reine Rangnummer, da diese Abstände verzerrt).
  - Bei genau einem Profil gilt `p = 0.5` per Konvention.
- Profil-Ordnung ergibt sich aus der Messung (auch der umgekehrte Fall: reduzierte Genauigkeit, die durch Emulation langsamer ist, wird erkannt und herabgestuft oder nicht aktiviert).
- Eine Engine ohne aktives Profil für ein Modell ist für dieses Modell gesperrt.

## Task-Parameter und Matching

- `TaskParams.precision: f32`, Skala **0 = fast, 1 = precision**. Orientierung muss in Doku und Typ-Dokumentation ausdrücklich stehen.
- Werte außerhalb `[0,1]` werden **geklemmt**, `NaN` ist ein Fehler.
- Matching: Profil mit minimalem `|t − p|`. Bei Gleichstand gewinnt deterministisch das **genauere** Profil.
- Jede Subengine matcht gegen ihre **eigenen** Profile. Die Skala ist relativ zur Engine: `t = 0.5` ist nicht auf CPU und GPU dasselbe Genauigkeitsniveau.
- Ein Wunsch ist immer erfüllbar (es gibt immer ein nächstliegendes aktives Profil). Es gibt keinen Fehler und kein Warten wegen nicht verfügbarem Profil.

## Pull-Mechanismus

- Filterprädikate beim Claim: Priorität, Modell-/Index-Affinität, Verfügbarkeit (Modell nicht gesperrt).
- Es gibt **kein** Profil-Prädikat und keine Konsistenzbedingung zwischen Tasks.
- Lastverteilung bleibt rein selbstorganisiert über den realen Fortschritt der Subengines. Es werden **keine Durchsatzwerte aus der Kalibrierung** für die Verteilung verwendet.

## Persistenz (CalibrationStore)

- Kalibrierung läuft **initial einmal** und wird auf derselben Maschine persistiert (Datei).
- Dadurch darf die Erstkalibrierung lang sein.
- Kalibrierung gilt pro **Modell × Subengine** (Verhalten hängt von Dimension und Sequenzlänge ab).
- Schlüssel (Fingerprint): GPU-Modell, Treiber-/CUDA-Version, CPU-Modell, Modell-ID, Engine-Version.
- Zustände: `Calibrated | Stale | Missing`.
  - `Calibrated`: laden und verwenden.
  - `Stale` (Fingerprint abweichend): mit altem Ergebnis starten
  - `Missing`: konservativer Default (genaueste gültige Kombination, Warnung) oder explizites `calibrate`-Kommando.
- Persistiert werden auch `unstable`-Markierung, Maximalabweichungs-Verwerfungen und Pruning-Ergebnisse (für Diagnose und Nachmessung).

## Laufzeit-Fehlerbehandlung

- Typisierte Fehler (`thiserror`): `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision` (NaN).
- Logging über `tracing`: Kalibrierungsergebnis pro Kombination, verworfene Kombinationen mit Grund, abgeleitete Profile (Anzahl, Positionen), gewähltes Profil pro Task auf Debug-Level.
- Lazy-Initialisierung: Ein inkompatibles oder gesperrtes Modell fällt erst beim ersten Task auf. Der Fehler geht sauber an den wartenden Task zurück, die Subengine wird für dieses Modell markiert, damit Folgetasks die Prüfung nicht wiederholen.
- Kein Laufzeitcheck pro Batch als Spezifikationsanforderung (ein Batch ändert weder Kernel, Index noch Gewichte).
- Keine stille Ersetzung durch andere Rechenpfade außerhalb der beschriebenen Profil-Abbildung.

## Nicht Teil dieser Ergänzung

- Chunking, Query-Formulierung, Reranking, Kandidaten-k, Indexstruktur: Aufgabe der Suchschicht.
- Index-Persistenzformat und VRAM-Spiegelung: unverändert im Hauptkonzept.
- Reproduzierbarkeit auf Bit-Ebene: ausdrücklich kein Ziel. Tests prüfen Schranken, nicht Gleichheit.

## Umsetzungsvorgaben für den Agenten

- Neue Komponenten: `CapabilitySet`, `Calibrator`, `CalibrationStore`, `Profile { position: f32, config }`, Matching-Funktion.
- Rust-Konventionen: `thiserror` für Fehler, `tracing` für Logging, tokio für Async. Kalibrierung und CPU-Kernel laufen im eigenen Thread-Pool, nicht im tokio-Runtime-Thread.
- Tests:
  - Matching: Klemmen, NaN-Fehler, Tie-Break zum genaueren, Einzelprofil.
  - Profilbildung: Spread-Schwellwert, Zusammenfassen redundanter Punkte, Positionsberechnung über Bogenlänge.
  - Pareto-Auswahl: dominierte Kombinationen werden verworfen.
  - Persistenz: Fingerprint-Wechsel führt zu `Stale`, fehlende Datei zu `Missing`.
  - Stabilitätslogik: simulierte verrauschte Messwerte (Mock-Messfunktion), Abbruch bei Obergrenze mit `unstable`.
- Die Kalibrierung wird über ein Trait für die Messung abstrahiert (Mock für Tests, reale Kernel für Betrieb).