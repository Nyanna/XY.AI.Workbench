# 02 — Schema-Ingest & Referenzindex

**Ziel:** OpenAPI-3.1-YAML laden, Referenzindex bauen, Operationen extrahieren. `$ref` bleibt atomar.
**Abhängig von:** 01

## Deliverables
- **Loader:** YAML → Python-Struktur.
- **RefIndex:** Map von `#/components/schemas/<Name>` und `#/components/responses/<Name>` auf Rohknoten; der **Original-Key** ist der kanonische Name.
- **Operationsliste:** aus `paths` je (path, method): `operationId`, `requestBody` (content→schema), `responses` (code→schema oder `$ref` auf `components.responses`), `parameters` (path/query).
- **`components.responses`-Auflösung:** die referenzierten wiederverwendbaren Responses (z.B. `InferenceRateLimited`, `InferenceServiceUnavailable`) auf ihren `content.<ct>.schema` reduzieren. `headers`/`examples` ignorieren.

## Verbindliche Entscheidungen
- **`$ref` NIE inline expandieren** (I4). Ref bleibt Verweis auf Ziel-Id.
- **`$ref` mit Geschwistern (3.1):** Siblings (`description`/`title`) gelten als **Edge-Metadaten-Override**, ändern NICHT Struktur/Ziel.
- **3.1-Null:** sowohl `type: 'null'` als auch `type: [x, 'null']` erkennen → Null-Zweig/-Kind.
- `security`/`servers`/`info` werden gelesen, aber nicht ins Model übernommen (Scope).
- Nur JSON-Content ist model-relevant; Content-Type-Kurzformen später (Segment 09).

## Interpretations-Leitplanken
- Wer `$ref` expandiert, bricht Dedup und Termination (I4) — verboten.
- **Parameter (path/query) gehören NICHT in den JSON-Body-Baum.** Sie sind Client-Methodensignatur (Segment 10).
- `components.responses` sind wiederverwendbare Responses, kein Schema — erst deren `content.schema` verwenden.

## Acceptance (deepseek)
- Genau 1 Operation: `POST /responses`, `operationId=createResponse`.
- requestBody → `$ref` `CreateResponse`.
- responses: `200`→`Response`, `429`→`InferenceRateLimited`→`ErrorResponse`, `503`→`InferenceServiceUnavailable`→`ErrorResponse`.
- RefIndex enthält alle `components.schemas` unter Original-Keys.

## Nicht in diesem Segment
IR-Knotenbau (Segment 03), Namen, Typen.
