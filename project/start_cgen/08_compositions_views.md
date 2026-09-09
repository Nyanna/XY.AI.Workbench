# 08 — Kompositionen & Proxy-Views

**Ziel:** allOf/anyOf/oneOf/discriminator/null als Proxy-Views über EINEN Node abbilden.
**Abhängig von:** 07

## Grundprinzip
Ein CompositionNode = eine Klasse, die genau EINEN JsonNode hält. Jeder Zweig wird als **Kind-View (Proxy) über denselben Node** exponiert. **Kein Merge, keine Kollisionsauflösung**: ein in mehreren Zweigen gleich benanntes Property liest über jede View dasselbe JSON-Feld.

## Semantik je Keyword (bestimmt nur Zugriffs-/Prüfmethoden)
- **allOf (Intersection):** alle Views gelten gleichzeitig → einfacher Getter je View (`getCat()`, `getDog()`).
- **anyOf (mind. eine):** je View Getter + `applies?()`-Prüfung.
- **oneOf (genau eine):** je View `applies?()`; Discriminator wählt deterministisch.
- Schreiben teilt denselben Node. Bei oneOf/anyOf kann fachlich widersprüchliches JSON entstehen; das wird **bewusst nicht geprüft** (Validatoren out of scope).

## Discriminator (D1)
- `discriminator.propertyName` deklariert → **value→branch-Map aus dem Const/Enum der Discriminator-Property jedes Zweigs** (z.B. `type: input_text` → `InputTextContent`).
- **Nicht** den Komponentennamen als Discriminator-Wert nutzen (die weichen ab: `input_text` ≠ `InputTextContent`).
- Fehlt der Const → struktureller `applies?`-Fallback (Präsenz/Typ der Pflichtfelder).
- `mapping` (falls vorhanden) hat Vorrang.

## Nullable (D3)
- `anyOf[X, null]` bleibt eigene Klasse (`AnyOfXNull`) mit Views `X` und `Null`.
- Getter unterscheidet **absent** (Feld fehlt) vs. **explizit null** (JSON `null`).

## Benennung anonymer Zweige
- Anonymer Inline-Zweig → `<Composite>Part<n>` (Segment 05).

## Verbindliche Entscheidungen
- `not`: keine View, unsupported (D10).
- Nested Kompositionen erlaubt: eine View kann selbst eine Komposition sein (rekursiv über Views).
- allOf erzeugt **kein** flaches Objekt; Zugriff über die View-Kette.

## Interpretations-Leitplanken
- Zugriff auf ein Feld aus einem allOf-Teil läuft über dessen View: `getResponseProperties().getModel()` — nicht direkt `getModel()`.
- Alle Views teilen denselben Node; keine separaten Datencontainer.

## Acceptance (deepseek)
- `InputContent` (oneOf, discr `type`) → `applies?` via `type ∈ {input_text, input_image, input_file}`.
- `CreateResponse` (allOf) → `getCreateModelResponseProperties()`, `getResponseProperties()`, `getCreateResponsePart3()`.
- `reasoning` (`anyOf[Reasoning, null]`) → eigene Klasse mit absent/null-Unterscheidung.
- `Tool` (oneOf, discr `type`, mapping implizit) → korrekte Zweigwahl über `type`-Const.

## Nicht in diesem Segment
Request/Response-Kapselung (09), Client (10).
