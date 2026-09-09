# 04 — Strukturelle Identität & Dedup

**Ziel:** Fingerprint pro Knoten; Äquivalenzklassen; geteilte Knoten. Dies ist die **Zustandsminimierung** der State Engine.
**Abhängig von:** 03

- Übersicht: `/home/user/xyan/xy.ai.workbench/project/start_cgen/00_overview.md`

## Kanonische Normalform (Identitätsumfang)
**Enthalten:** `kind`; Kinder als `(label, Ziel-Fingerprint)`; `required`-Set (sortiert); Enum-Wertemenge; `discriminator` (propertyName + mapping); Primitivtyp.
**NICHT enthalten:** `description`, `example`, `default`, `title`, `format` und alle Validatoren (I3).

Kanonisierung:
- Objekt-Properties nach Property-Name sortieren (`{a,b}` ≡ `{b,a}`).
- Union-Zweige (`anyOf`/`oneOf`): **Reihenfolge beibehalten, NICHT sortieren** (Discriminator/Branch-Index sind positionsrelevant; identische Unions matchen nur bei gleicher Reihenfolge). Begründung im Code dokumentieren.

## Fingerprint-Berechnung
- **RefNode-Fingerprint = Id-Token des Ziels** (Zielname), NICHT dessen Expansion (I4).
- Dadurch bilden anonyme Knoten über Ref-Kanten einen **DAG** → **bottom-up Hash, memoisiert**, terminiert ohne Sonderfall.
- Kanonische Serialisierung (sortierte Felder) → stabiler, deterministischer Hash (I7).

## Dedup-Regeln
- Anonyme Knoten mit gleichem Fingerprint → **ein geteilter Typ**.
- **Benannte `components.schemas`: NIE dedupliziert**, behalten Key (I5) — auch bei struktureller Gleichheit mit anderem benannten Schema.
- Geteilte anonyme Knoten → typspezifisches Package (Segment 05).

## Verbindliche Entscheidungen
- **Keine Bisimulation, kein Partition-Refinement.** Begründung: Zyklen laufen in OpenAPI stets über benannte `$ref`, die als Id-Token atomar sind; benannte Schemas werden ohnehin nie gemerged → der einzige Fall, der Bisimulation verlangen würde, ist gar nicht erwünscht.
- Der Fingerprint ist die WAHRE Identität; der freundliche Name (05) ist nur Darstellung.

## Interpretations-Leitplanken
- Metadaten in den Fingerprint zu ziehen zersplittert Dedup (I3) — verboten.
- Refs zu expandieren riskiert Nicht-Termination und Falsch-Splits (I4) — verboten.
- „Identisch" heißt: gleicher Fingerprint. Nicht: gleicher Name, nicht: gleiche Beschreibung.

## Acceptance (deepseek)
- Alle inline `anyOf[string, null]` → **ein** Fingerprint/Klasse.
- Alle inline Status-Enums `{in_progress, completed, incomplete}` → **ein** Fingerprint/Klasse.
- `ImageDetail` und `DetailEnum` (beide benannt, gleiche Werte) → **zwei** Klassen (I5).
- Fingerprints sind über zwei Läufe identisch (Determinismus).

## Nicht in diesem Segment
Namen/Packages (05), Java-Typen (06).
