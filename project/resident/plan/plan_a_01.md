# Umsetzungsplan A1′ — Scope-Grenze Engine ↔ Suchschicht fixieren

> Teil von Gruppe A (Fundament & Kontext). Ersetzt das erledigte Paket A1 ("These destillieren").
> Quellen: `resident_planplan.md` (Abschnitt "Auswirkungen auf den Metaplan", Abgleich-Tabelle),
> `resident.md` (Ausblick/Klarstellung), `resident_index.md` ("Nicht Teil dieser Ergänzung"),
> `resident_precision.md` ("Nicht Teil"-Abschnitte).

## 1. Ziel & Ergebnistyp

Verbindliche **Scope-Charta** als eigenständiges Dokument: Was liefert die "Resident"-Engine,
und was bleibt bewusst außerhalb (Oberschicht/Suchschicht). Die Charta ist **blockierend** für
C1 (Fassaden-API) und E2 (Scope-Resolver) und verhindert, dass später Zuständigkeiten
verwischen (z. B. Qualität/Chunking in die Engine wandern).

**Ergebnistyp:** Markdown-Dokument (keine Code-Artefakte).

## 2. Eingaben & Quellen

- `resident_planplan.md` → Abschnitte **Kernbefund**, **Abgleich-Tabelle** (Status K/I/A/E/—),
  **Zentrale Divergenz**, **Auswirkungen auf den Metaplan** (A1′, H5, H6).
- `resident.md` → "Klarstellung" (Dateihandling/Persistenz außerhalb der Engine),
  "Ausblick" (gleitende Indizierung, Ringbuffer).
- `resident_index.md` → "Leitprinzipien" (approximative Vorfilter = Oberschicht),
  "Nicht Teil dieser Ergänzung".
- `resident_precision.md` → "Nicht Teil"-Abschnitte (Chunking, Query-Formulierung, Rerank-Policy).

## 3. Abhängigkeiten

- **Abhängt von:** — (keine; A1 These-Abgleich ist bereits erledigt).
- **Blockiert:** C1 (Fassaden-API), E2 (Scope-Resolver), und wirkt in G1/G3 (Standardpfad
  dense vs. ColBERT/MaxSim-Wechselpfad).

## 4. Vorannahmen / verifizierter Stand

Der These-Abgleich ist abgeschlossen (siehe Metaplan). Kernaussagen, die in die Charta
eingehen und nicht neu recherchiert werden müssen:

- Nebenläufigkeitsmodell Phase 1 = **Lock-free Pull-Queue + Processor/Planner/Subengine**;
  der SPSC-Ringbuffer (These §3) ist auf den Persistent-Kernel-Modus **zurückgestellt** (H1).
- Standardpfad ist **dense e5-small (Cosine)**; MaxSim/ColBERT existiert nur als
  Kernel-Infrastruktur/Wechselpfad (A/I).
- Nicht adressiert / außerhalb: Signalkompression (FFT/Wavelet, §6/§6.1 → H5),
  mutable weights / Online-Bandit (§9/§9.1 → H6).

## 5. Arbeitsschritte (für den Agenten)

1. **Grenztabelle aufstellen** mit den fünf Statusklassen aus dem Metaplan:
   **K** (Kern/umgesetzt), **I** (Hook vorbereitet), **A** (Ausblick in der Engine),
   **E** (bewusst außerhalb), **—** (nicht adressiert). Jede Zeile = ein Aspekt mit Status,
   Beleg (Dokument/§) und Zuständigkeit (Engine vs. Oberschicht).
2. **"Engine liefert"-Liste** ausformulieren: residenter Compute (Embedding/Similarity/Inferenz),
   Index-Substrat + Streaming-Sync RAM↔Index↔VRAM, modell-entkoppelte MaxSim-Geometrie,
   Residency/Eviction, Scope-Auflösung über deklarierte Verfahren + Resolver.
3. **"Außerhalb der Engine"-Liste** ausformulieren: Chunking, Query-Formulierung,
   Kandidaten-k/Rerank-Policy, Qualität/Vollständigkeit, approximative Vorfilter
   (Zentroid/FFT/Kompression) als Oberschicht-Entscheidung, Drift-Monitor,
   Signalkompression (H5), mutable weights/Bandit (H6), Index-Persistenzformat/-zeitpunkte.
4. **Zentrale Divergenz dokumentieren** (These §3 Ringbuffer vs. Phase-1-Pull-Queue) als
   bewusste Umsetzungsentscheidung, damit die These-Treue nachvollziehbar bleibt.
5. **Hook-Verpflichtungen festhalten** (was Phase 1 "nicht verbauen" darf): erweiterbares
   Parameterobjekt, `requires()`-Affinität, Op-Austausch hinter gleichem Trait,
   `AccessMethod`/`Scope`/`summary()` offen für spätere Frequenz-/Kompressionsdomäne (H5),
   Index nur **bidirektional mutabel**, Weights immutabel (H6).
6. **Verweise setzen:** explizit markieren, dass C1 und E2 gegen diese Charta zu planen sind.

## 6. Zu erzeugende Artefakte

- `rag/docs/architecture/scope_charta.md` (empfohlener Pfad; alternativ im Projektordner).
  Enthält: Grenztabelle, "Engine liefert"-/"Außerhalb"-Listen, Divergenz-Notiz, Hook-Liste.

## 7. Offene Punkte / Verifikation vor Abschluss

- Keine Recherche offen (A1 erledigt). Prüfen, dass **jede** H-Position (H1–H6) in der Charta
  einen eindeutigen Status und — falls Hook — eine benannte Hook-Stelle hat.
- Gegencheck: Jede "außerhalb"-Aussage muss durch einen "Nicht Teil"-Beleg in `resident_index.md`
  oder `resident_precision.md` oder durch die Abgleich-Tabelle gedeckt sein.

## 8. Akzeptanzkriterien (Definition of Done)

1. Charta enthält vollständige Grenztabelle (alle Aspekte der Metaplan-Tabelle abgebildet).
2. Jede Hook-Verpflichtung ist einer konkreten späteren Erweiterung (H1–H6) zugeordnet.
3. Divergenz Ringbuffer↔Pull-Queue ist als Entscheidung (nicht als Inkonsistenz) dokumentiert.
4. C1 und E2 referenzieren die Charta als Planungsgrundlage (Verweis gesetzt).

## 9. Querschnitt (Q1–Q4)

- **Q3:** Kein Testbedarf (Dokument). Charta definiert jedoch, welche Qualitäts-/Retrieval-Tests
  **nicht** Teil der Engine-Testsuite sind (Abgrenzung für spätere Testmatrizen).
- **Q1/Q2/Q4:** nicht betroffen.
