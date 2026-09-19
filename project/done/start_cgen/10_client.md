# 10 — Client (Interface + Implementierung)

**Ziel:** Zweiteiliger Client via `java.net.http.HttpClient`.
**Abhängig von:** 09

- Übersicht: `/home/user/xyan/xy.ai.workbench/project/start_cgen/00_overview.md`

## Aufbau
- **Interface:** eine Methode je **Operation** (Path+Method). Übernimmt `description`/`example` des Path als Javadoc.
- **Impl:** implementiert mit `java.net.http.HttpClient` gegen das Interface.
- **Methode:** bildet URL-Parameter (path/query) ab; **Header nicht unterstützt**. Aufruf mit dem Root-Request-Objekt; liefert IMMER das Response-Objekt (Ausnahme: Exceptions bei Transportfehlern).
- **Ablauf Impl:** Request serialisieren (gekapselt, 09) → senden → Response via `from(body, statusCode, contentType)` konstruieren.

## Auth / Base-URL (D11)
- **Base-URL** als Konstruktor-Parameter/Config.
- **Auth** als überschreibbarer Hook (z.B. Request-Customizer) UND die Impl als **ableitbare Basisklasse** (Subklasse setzt `Authorization`).
- **Kein** konkreter API-Key-/Bearer-Code im Kern.

## Verbindliche Entscheidungen
- Methodenname: aus `operationId` (`createResponse`), sonst `<method><Path>`.
- `java.net.http.HttpClient`, **synchrones** `send` in Phase 1.
- Genau ein Interface + eine Impl-Klasse.

## Interpretations-Leitplanken
- Der Client hängt vom Model ab, nie umgekehrt (I2).
- HTTP-Fehlercodes (429/503) sind **keine** Exceptions, sondern Response-Views; nur Transport-/IO-Fehler werfen.
- Serialisierung liegt in den Root-Objekten, nicht im Client.

## Acceptance (deepseek)
- `ResponsesClient` (Interface) mit `createResponse(CreateResponse) : ResponsesResponse`.
- Impl mit HttpClient, Base-URL konfigurierbar, Auth über Subklasse/Hook injizierbar.

## Nicht in diesem Segment
Verdrahtung/Lauf/Verifikation (11).
