# 06 — Java-Typabbildung

**Ziel:** Primitive und Container auf Java-Typen abbilden.
**Abhängig von:** 05

## Mapping (verbindlich)
| OpenAPI | Java |
|---|---|
| string | `String` |
| integer | `Long` |
| number | `Double` |
| boolean | `Boolean` |
| null | kein eigener Typ (siehe unten) |
| array `<T>` | Zugriff über generierte Listen-Klasse (`<Element>List`) |
| additionalProperties: `{schema}` | generierte Dictionary-Klasse |
| additionalProperties: `true`/`{}` | `com.fasterxml.jackson.databind.JsonNode` (Any) |

## Verbindliche Entscheidungen
- **Durchgängig Boxed-Typen** (`Long`/`Double`/`Boolean`/`String`) — nie `int`/`double`, da Felder absent/null sein können.
- `integer`→`Long`, `number`→`Double`. **Kein `BigDecimal`/`BigInteger` in Phase 1.**
- `null` ist kein Java-Typ: absent vs. explizit-null wird über den Getter der `AnyOf…Null`-View unterschieden (Segment 08).
- `format` und Validatoren werden **ignoriert** (Scope). Folge: `string` ≡ `string(format:uri)` derselbe Zustand — bewusst.

## Interpretations-Leitplanken
- Keine primitiven Java-Typen (Nullbarkeit).
- `format`/`minimum`/`maximum`/`pattern` beeinflussen weder Typ noch Identität.

## Acceptance (deepseek)
- `created_at` (number) → `Double`; `input_tokens` (integer) → `Long`; `parallel_tool_calls` (boolean) → `Boolean`.
- `FunctionTool.parameters` (`additionalProperties: {}`) → `JsonNode`.

## Nicht in diesem Segment
Template-Inhalte (07+).
