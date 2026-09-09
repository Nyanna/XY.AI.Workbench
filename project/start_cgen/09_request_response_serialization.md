# 09 — Request/Response-Serialisierung

**Ziel:** Wurzelobjekte kapseln (De-)Serialisierung selbst.
**Abhängig von:** 07, 08

## Request
- `toString()` → JSON-Body; `fromString(body)` → Instanz.
- Der Client ruft mit dem Root-Request-Objekt; dieses serialisiert sich selbst.

## Response
- Statuscode & Content-Type sind **Transport-Metadaten** und stehen NICHT im Body.
- **Konstruktion:** `from(body, statusCode, contentType)` (D5). `fromString(body)` allein genügt NICHT.
- **Statuscode = Discriminator** für das zutreffende CodeNode-Kind: `getCode200()` liefert den Subtree, `getCode429()` bleibt null.
- **Content-Type-Header = Discriminator** für die Content-Type-View (`*Json`/`*Txt`).
- Response-Codes sind mögliche Kinder, per `applies?` gelesen.

## Verbindliche Entscheidungen
- **Content-Type→Kurzname-Tabelle:** `application/json`→`Json`, `text/plain`→`Txt` (erweiterbar). Bei mehreren gleicher Kurzform deterministisch disambiguieren.
- `components.responses` (429/503) fließen als CodeNodes mit ihrem `content.schema` ein (hier `ErrorResponse`).

## Interpretations-Leitplanken
- Der Body kennt weder Status noch Content-Type; diese kommen ausschließlich über die `from(...)`-Signatur.
- Root-Objekte kapseln Serialisierung selbst; **kein externer Serializer im Client**.
- HTTP-Fehlercodes sind Views, keine Exceptions (siehe 10).

## Acceptance (deepseek)
- `ResponsesResponse.from(body, 200, "application/json")` → `getCode200().getJson()` = `Response`-Body.
- `ResponsesResponse.from(body, 429, "application/json")` → `getCode429().getJson()` = `ErrorResponse`.
- `CreateResponse.toString()`/`fromString(...)` roundtrippen den Body.

## Nicht in diesem Segment
Client-Transport (10), Verdrahtung (11).
