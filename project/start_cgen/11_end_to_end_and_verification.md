# 11 — Verdrahtung, Lauf & Verifikation

**Ziel:** Pipeline zusammenführen, gegen deepseek laufen, Kompilat prüfen.
**Abhängig von:** 01–10

- Übersicht: `/home/user/xyan/xy.ai.workbench/project/start_cgen/00_overview.md`

## Pipeline (Reihenfolge)
`ingest (02) → model/IR (03) → identity+dedup (04) → naming+packaging (05) → typemap (06) → emit model (07) → emit compositions (08) → emit request/response (09) → emit client (10)`

## Lauf
- Eingabe: `/home/user/xyan/xy.ai.workbench/libs/openapi/filters/deepseek.filtered.yaml`
- Ausgabe: konfiguriertes Verzeichnis, Root-Package `xy.api.codegen`.
- Aufruf: `python -m cgen --schema <yaml> --out <dir> --base-package xy.api.codegen`

## Verifikation (kein Test i.S. der Vorgabe)
- Der generierte Java-Code **kompiliert** (javac mit Jackson 2 + `java.net.http` auf dem Classpath).
- Der Kompilat-Check ist Akzeptanzkriterium, **keine** Unit-Tests.

## Definition of Done
- Alle Segmente 01–10 erfüllt.
- deepseek erzeugt:
  - Model-Klassen mit Kompositionen als Views (kein Merge).
  - Deduplizierte `.enums`/`.lists`/`.objects`/`.operators`/`.dictionaries`.
  - Request-Root `CreateResponse`; Response-Root `ResponsesResponse` mit `code200`/`code429`/`code503`.
  - Client `ResponsesClient` (Interface) + Impl.
- Generierter Code kompiliert.
- Invarianten-Stichprobe bestanden:
  - **I3:** keine Metadaten im Fingerprint (identische Struktur mit unterschiedlicher `description` → eine Klasse).
  - **I4:** keine Ref-Expansion; Fingerprinting terminiert.
  - **I5:** `ImageDetail`/`DetailEnum` bleiben getrennt.
  - **I7:** zwei Läufe erzeugen byte-identische Namen/Dateien.

## Interpretations-Leitplanken
- Emissionsreihenfolge beeinflusst Namen NICHT (I7 Determinismus).
- Kompilierfehler im jeweiligen Segment beheben, **nie** manuell im generierten Code.

## Nicht in diesem Segment
Neue Features außerhalb Scope (00).
