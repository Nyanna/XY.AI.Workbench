# 00 — Überblick & verbindliche Grundlagen

Quelle: `/home/user/xyan/xy.ai.workbench/project/sttub_gen.md` + geklärte Design-Entscheidungen.
Dieses Verzeichnis ist der **Umsetzungsplan**, kein Code. Jedes Segment ist eigenständig
interpretierbar. Der Agent arbeitet die Segmente in numerischer Reihenfolge ab.

Vor dem Start jedes Segments: `_context.md` in diesem Verzeichnis lesen (discovered Pfade,
Repo-Layout, bereits umgesetzte API, Konventionen) — spart erneutes Discovery. Nach Abschluss
eines Segments dort den Stand nachführen.

## Auftrag
Eigener Codegenerator (Python) erzeugt aus OpenAPI-3.1-YAML typsicheren Java-Code.
- Ziel-Python-Projekt: `/home/user/xyan/xy.ai.workbench/codegen`
- Beispielschema: `/home/user/xyan/xy.ai.workbench/libs/openapi/filters/deepseek.filtered.yaml`
- Java-Root-Package: `xy.api.codegen` (konfigurierbar)
- Vorerst keine Tests. Kommentare knapp und signifikant (immer englisch).
- Keine Referenzen auf den Umsetzungsplan oder direkte Anforderungen in Kommentaren

## Kernkonzept (verbindlich)
Das generierte Model ist eine **typsichere State Engine**:
- **Zustand** = (generierter Typ × gebundener JsonNode).
- **Transition** = interne Kindliste eines Knotens (Label + Zieltyp).
- **Typsicherheit** = ungültige Transitionen existieren nicht als Methode → der Compiler ist der Validator.
- **(De-)Serialisierung** = Traversierung dieses Graphen; JsonNode ist der Speicher; Klassen halten nur Referenzen (Proxy).
- **Symmetrie** = derselbe Graph wird lesend (deserialisieren) und schreibend (serialisieren) traversiert.

## Globale Invarianten (dürfen NIE verletzt werden)
- **I1** Der Generator schreibt nie direkt Java. Er baut ein IR und rendert pro Knoten ein Template.
- **I2** Das Model ist unabhängig von HTTP-Client/Server. JSON ausschließlich via Jackson 2. Der Client hängt vom Model ab, nie umgekehrt.
- **I3** Identität ist strukturell (Fingerprint). Metadaten (description/example/default/title/format) sind NIE Teil der Identität — sie leben an der **Kante** (Getter/Setter/Property), nicht am geteilten Typ.
- **I4** `$ref` ist ein **atomares Id-Token**, wird nie expandiert. Dadurch bilden anonyme Knoten einen DAG → Fingerprint bottom-up, terminiert. **Keine Bisimulation.**
- **I5** Benannte `components.schemas` behalten ihren Key und werden NIE dedupliziert/gemerged. Nur anonyme/abgeleitete Knoten werden geteilt.
- **I6** Kompositionen (allOf/anyOf/oneOf) werden NICHT gemerged: ein Knoten, mehrere Proxy-Views.
- **I7** Determinismus: alles sortieren. Namen/Nummerierung nach **kanonischer Fingerprint-Reihenfolge**, nie nach Entdeckungsreihenfolge.
- **I8** Kommentare im generierten Code knapp und signifikant.

## Decisions Register (gelöste Streitpunkte — NICHT neu interpretieren)
- **D1** Discriminator ohne `mapping`: value→branch-Map durch Lesen des **Const/Enum der Discriminator-Property jedes Zweigs** (z.B. `type: input_text`). Fehlt der Const → struktureller `applies?`-Fallback (Pflichtfeld-Präsenz/-Typ).
- **D2** Open-Enum-Idiom `anyOf[string, {string+enum}]`: KEIN Sonderfall. Als anyOf-Komposition (Proxy-View) behandeln.
- **D3** `anyOf[X, null]`: eigene Klasse (`AnyOfXNull`), nicht zu nullable X kollabieren. Getter unterscheidet **absent** vs. **explizit null**.
- **D4** Request-Root: ist der requestBody ein einzelner `$ref` auf ein benanntes Schema, IST dieses die Request-Wurzel (behält Key, z.B. `CreateResponse`). Der synthetische Name `<Path>Request<Method>` gilt nur für inline/anonyme Bodies.
- **D5** Response pro **Operation** (Path+Method); bei genau einer Methode je Pfad entfällt das Method-Segment (wie im Dokument). Konstruktion aus `body + statusCode + contentType`.
- **D6** Namenskollision: kollidierende Fingerprints kanonisch sortieren, dann `Name`, `Name2`, `Name3`.
- **D7** Innere Klassen: in Phase 1 NICHT verwenden. Packaging strikt nach Dokument-Schema. (Innere Klassen sind eine bewusst ausgeschlossene spätere Optimierung.)
- **D8** `required`, das ein in diesem Knoten nicht vorhandenes Property nennt (cross-allOf oder gefiltert), wird ignoriert (Robustheit).
- **D9** `additionalProperties: true|{}` → Any-Dictionary (roher JsonNode-Durchgriff). `additionalProperties: {schema}` → typisiertes Dictionary.
- **D10** `not`: unsupported → Knoten wird übersprungen/als Fehler markiert, keine View.
- **D11** Auth/Base-URL: Kern generiert Hooks + überschreibbare Client-Basisklasse; keine konkrete Auth-Implementierung im Kern.

## Scope
**In:** OpenAPI 3.1 `components.schemas` + `components.responses` + `paths`; JSON content; oneOf/anyOf/allOf; discriminator; enums; arrays/mixed-arrays; dictionaries; primitives; Client (java.net.http.HttpClient) als Interface + Impl.
**Out:** Validatoren (min/max/pattern/`format`), `info`/`servers`/`security` als Funktion, Header, Server-Stack, `not`, Tests, konkrete Auth.

## Glossar
- **Knoten/Node**: IR-Element, entspricht einem Zustand.
- **Kante/Transition**: benannter Übergang (Property/Element/Branch) zum Zieltyp; trägt Edge-Metadaten.
- **Fingerprint**: kanonischer struktureller Identitätsschlüssel eines Knotens.
- **Geteilt (shared)**: mehrere Stellen mit gleichem Fingerprint → ein Typ.
- **Benannt (named)**: `components.schemas`-Eintrag; behält Key, nie gemerged.
- **View**: Proxy-Sicht eines Kompositions-Zweigs über denselben Node.
- **`applies?`**: Laufzeitprüfung, ob eine Union-View auf den aktuellen Node zutrifft.

## Arbeitsweise für den Agenten
1. Segmente 01→11 in Reihenfolge; „Abhängig von" im Kopf jedes Segments beachten. Keine Vorgriffe.
2. Pro Segment nur den dort definierten Scope umsetzen; am Ende die Acceptance-Kriterien erfüllen, bevor es weitergeht.
3. Konfliktregel: Bei Widerspruch zwischen diesem Plan und `sttub_gen.md` gilt das **Decisions Register**; sonst `sttub_gen.md`.
4. Bei echter Unklarheit **nicht raten** — als offenen Punkt markieren und eskalieren.

## Zielartefakte
Python-Projekt in `/codegen`; generierter Java-Code im Ausgabeverzeichnis; Lauf gegen `deepseek.filtered.yaml`; generierter Code kompiliert (Segment 11).
