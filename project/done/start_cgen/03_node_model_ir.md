# 03 — Knotenmodell (IR / Zustandsgraph)

**Ziel:** Rohschema → typisierte Knoten mit beschrifteten Kanten. Keine Namen, keine Java-Typen, keine Metadaten am Knoten.
**Abhängig von:** 02

- Übersicht: `/home/user/xyan/xy.ai.workbench/project/start_cgen/00_overview.md`

## Knotenarten (`kind`)
- **ObjectNode:** children = benannte Properties (label = property-name), `required`-Set.
- **ListNode:** element-Kante (label `element`) → Zieltyp. **MixedList:** mehrere Element-Typen.
- **EnumNode:** Wertemenge + Basis-Primitivtyp.
- **DictionaryNode:** value-Typ-Kante. **AnyDictionary** (`additionalProperties: true|{}`) → roher JsonNode.
- **PrimitiveNode:** string | integer | number | boolean | null.
- **CompositionNode:** `keyword ∈ {allOf, anyOf, oneOf}`, branch-Kanten (label = branch-Index/Rolle), optionaler `discriminator` (propertyName [+ mapping]).
- **RefNode:** atomarer Verweis auf einen benannten Zielknoten (Id-Token).

## Kante (Edge)
Trägt: `label`, `Ziel` (Knoten oder RefNode), **Edge-Metadaten** (`description`/`example`/`default` aus dem Schema an DIESER Stelle). Metadaten leben ausschließlich hier (I3).

## Wurzelknoten (Transport-Struktur)
- **RequestNode** je Operation: hüllt die Body-Struktur (bei `$ref` → RefNode auf benanntes Schema, siehe D4).
- **ResponseNode** je Operation: Kinder = CodeNodes.
- **CodeNode** je Statuscode: Kinder = ContentTypeViews.
- **ContentTypeView** je content-type: → Body-Schema-Knoten.

## Verbindliche Entscheidungen
- `anyOf[X, null]` = CompositionNode(anyOf) mit Branches `[X, NullPrimitive]`. Nicht kollabieren (D3).
- Ein `allOf`-Teil mit Inline-Objekt ist ein eigener **anonymer Branch-Knoten** (später `<Composite>Part<n>`). **Kein Merge** (I6).
- `required`, das kein vorhandenes Property nennt → ignorieren (D8).
- Discriminator wird am CompositionNode gespeichert.
- Metadaten NUR an Kanten, nie am Knoten (I3) — Voraussetzung für Dedup.

## Interpretations-Leitplanken
- Gleiche Struktur an zwei Stellen MUSS derselbe Knoten werden können → der Knoten darf nichts Stellenspezifisches (Metadaten, Key) tragen.
- `allOf` erzeugt hier kein flaches Objekt; nur die Branch-Liste.
- `oneOf`/`anyOf`-Zweigreihenfolge wird **beibehalten** (positionsrelevant für Discriminator/Benennung).

## Acceptance (deepseek)
- `CreateResponse` → CompositionNode(allOf, [Ref(CreateModelResponseProperties), Ref(ResponseProperties), ObjectNode(inline Part3)]).
- `Response` analog (allOf mit inline Part3, das die vielen `anyOf[...,null]`-Properties enthält).
- `InputContent` → CompositionNode(oneOf, [Ref×3], discriminator=type).
- `ToolsArray` → ListNode(element=Ref(Tool)).

## Nicht in diesem Segment
Fingerprint/Dedup (04), Namen/Packages (05).
