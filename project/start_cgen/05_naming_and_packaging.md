# 05 — Benennung & Packaging

**Ziel:** Aus Fingerprint-Klassen + Knotenrollen → Java-Klassennamen, Packages, gültige Java-Identifier.
**Abhängig von:** 04

## Namensableitung (Klassenname)
- **Benanntes Schema** → Key als Klassenname (sanitisiert). Behält Key auch bei Kompositions-Rumpf (z.B. `CreateResponse`, nicht `AllOf...`).
- **Inline-Komposition** → `<Keyword><Branch1><Branch2>…` (z.B. `OneOfDogCat`). Branchname = Ziel-Refname, sonst Primitive-Name (`String`/`Integer`/…), sonst bei anonymem Zweig `<Composite>Part<n>` (n 1-basiert, Zweigindex).
- **Liste** → `<Element>List` (`StringList`, `ToolList`). **LOCK Phase 1:** ausschließlich **struktureller** Elementname; kein key-basiertes Listennaming (Name MUSS Funktion der Struktur bleiben, sonst Falsch-Split, I7/Dedup).
- **Enum** → benannt: Key. Inline: `<Werte in PascalCase, deterministisch, ggf. gekappt>Enum` + Kollisionsnummerierung (Werte sind die Identität).
- **Dictionary** → `<Value>Dictionary`; **AnyDictionary** → `AnyDictionary`.

## Synthetische Transport-Namen
- **RequestNode:** bei `$ref`-Body der benannte Schema-Name (D4, z.B. `CreateResponse`). Nur inline/anonym → `<Path>Request<Method>`.
- **ResponseNode (root):** `<Path>Response` (z.B. `ResponsesResponse`). *Nicht* zu verwechseln mit dem benannten Body-Schema `Response`.
- **CodeNode/ContentTypeView:** `<Path>ResponseCode<code><Ct>` (z.B. `ResponsesResponseCode200Json`).

## Kollision (verschiedene Fingerprints, gleicher abgeleiteter Name)
- Kollidierende Fingerprints **kanonisch sortieren**, dann `Name`, `Name2`, `Name3` (D6). Die Nummer ist Funktion der Struktur, nicht des Parse-Zeitpunkts (I7).

## Packaging (Base konfigurierbar, Default `xy.api.codegen`)
- `.components` — benannte Klassen, die von **mehr als einem Pfad** referenziert werden.
- `.request.<path>.<method>.<ct>` — Request-Wurzel (Bsp. `.request.responses.post.json`).
- `.response.<path>.code<code>.<ct>` — Code/Content-Views (Bsp. `.response.responses.code200.json`).
- `.enums` / `.lists` / `.objects` / `.operators` / `.dictionaries` — **geteilte anonyme** Knoten je Art (`operators` = geteilte Kompositionen).
- Benannte Schemas, die nur von **einem** Pfad referenziert werden → in dessen request/response-Package (nicht `.components`).

## Identifier-Sanitizing
- Java-Keywords → Suffix `_`.
- Führender Unterstrich (`_MisalignmentErrorType`) → entfernen/ersetzen zu gültigem Identifier.
- JSON-Feldname bleibt via `@JsonProperty` erhalten; Java-Methodenname `snake_case→camelCase` (`top_logprobs`→`getTopLogprobs`).
- Deterministisch und kanonisch.

## Interpretations-Leitplanken
- **Name = Funktion der Struktur (Fingerprint), nie der referenzierenden Stelle** → sonst Falsch-Split, Dedup bricht.
- Nummerierung nach kanonischer Fingerprint-Reihenfolge (I7), nie Discovery.
- **Keine inneren Klassen** (D7); strikt das Package-Schema.

## Acceptance (deepseek)
- `CreateResponse` (benannt, single-path) → in `.request.responses.post.json`.
- `ResponsesResponse` (root) → in `.response.responses`; `ResponsesResponseCode200Json` → `.response.responses.code200.json`.
- Geteiltes `anyOf[string,null]` → `.operators`; geteiltes Status-Enum → `.enums`; `ToolList`/`ToolsArray` → `.lists`.
- Zwei Läufe erzeugen identische Namen.

## Nicht in diesem Segment
Java-Typabbildung (06), Templates (07+).
