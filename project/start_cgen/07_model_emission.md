# 07 — Model-Emission (Proxy State Engine)

**Ziel:** Pro Knoten ein Template → eine Java-Klasse, die als Proxy über JsonNode liest/schreibt.
**Abhängig von:** 05, 06

## Klassenaufbau (pro Knoten)
- **Feld:** Referenz auf ihren JsonNode. Für Schreiben mutable Knoten (`ObjectNode`/`ArrayNode` via `JsonNodeFactory`).
- **Interne, generierte Kindliste:** erlaubte `(label, Zieltyp)` — statisch aus dem IR (die Transitionstabelle).
- **Lazy Read:** Getter prüft den nächsten lesbaren Datentyp im Node, instanziiert das Kind mit dessen (Sub-)Node.
- **Write:** Setter / `add`/`remove` (Listen) — hängt Teilbaum/Primitive an, wenn laut Kindliste zulässig.
- **Keine Kopien** primitiver Werte; nur Referenzen (Proxy).

## Methodenarten
- **Primitiv:** Getter liest direkt aus dem JsonNode; Setter schreibt direkt.
- **Komplex:** Getter instanziiert die Kindklasse (gebunden an den Kindknoten); Setter/`add`/`remove`.
- Getter/Setter tragen `description`+`example` (Edge-Metadaten) als knappes Javadoc.

## Templating
- **Ein Template pro Knotenart.** Variablen: Klassenname, FQN, Kindliste (label + Typ), JSON-Feldname, Edge-Metadaten.
- Jackson 2 (`com.fasterxml.jackson.databind.*`).

## Verbindliche Entscheidungen
- Klasse hält KEINE Daten außer Node-Referenz + (statischer) Kindliste — reiner Zustand (I3).
- Metadaten NUR an Getter/Setter, nicht als Klassen-Javadoc geteilter Typen (I3).
- Schreiben über mutable Nodes; Lesen über den vorhandenen Node.

## Interpretations-Leitplanken
- Der Proxy dupliziert keine JSON-Daten; er navigiert sie.
- Kind wird erst bei Bedarf erzeugt (lazy), nicht im Konstruktor.
- Kein Merge, keine Feldflachlegung (Kompositionen siehe 08).

## Acceptance (deepseek)
- `OutputMessage`-Klasse: `id`/`type`/`role`/`status` als Primitive; `content` als Liste von `OutputMessageContent`; symmetrisches Schreiben; keine Datenkopie.
- Erzeugter Node beim Schreiben ist ein `ObjectNode`/`ArrayNode`.

## Nicht in diesem Segment
Kompositions-Views (08), Request/Response-Kapselung (09), Client (10).
