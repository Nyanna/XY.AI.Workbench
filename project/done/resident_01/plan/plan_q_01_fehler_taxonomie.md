# Umsetzungsplan Q1 · Fehler-Taxonomie (`thiserror`)

**Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren, je Gruppe fortzuschreiben. Speist alle Gruppen (A–H).

## 1. Zweck

Einheitliche, typisierte Fehlerbehandlung über alle Pakete. Konsolidiert die im Metaplan/den Quellen verstreuten Fehlerarten zu einer geschichteten Taxonomie mit stabiler öffentlicher API.

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q1)

- **`thiserror` 2.x** **[prüfen Version]**. Muster: `#[derive(Error, Debug)]`, `#[error("…{field}")]` (Display), `#[from]` (auto-`From` + Source), `#[source]` (Ursachenkette ohne `From`), `#[error(transparent)]` (Durchreichen). Varianten mit **strukturierten Feldern**, keine Strings. `#[non_exhaustive]` auf öffentlichen Enums → spätere Varianten ohne Breaking Change.
- **Geschichtet:** ein Enum **pro Komponente/Modul** (klein, präzise); ein Top-Level-Enum aggregiert per `#[from]`. Kein zentrales Mega-Enum (koppelt alle Module).
- **Library vs. App:** Engine (öffentliche API) → `thiserror`, typisiert, matchbar. Anwendung/Binary/Tests → `anyhow` (`.context()`). **Die Engine gibt nie `anyhow::Error` in der öffentlichen API zurück.**
- **Intern vs. extern:** öffentlicher Fehler als opaker Struct mit privatem `ErrorKind`-Enum (wie `std::io::Error`) + `kind()`-Accessor auf ein `#[non_exhaustive]` öffentliches Kind. Interne Varianten (z. B. `StaleMapping`) in `pub(crate)`-Enum, an der API-Grenze per `From`/`map_err` abgebildet oder intern per Retry behandelt.

## 3. Verbindliche Konvention (Umsetzung)

**Pro-Komponenten-Enums** (`#[derive(Error, Debug)]`, `#[non_exhaustive]`), zugeordnet aus den Quellen:

| Komponente / Modul | Enum | Varianten (aus Quellen) | Quelle |
|---|---|---|---|
| Index/Manager | `IndexError` | `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `TransferFailed` | `resident_index.md` (Fehler) |
| Index/Manager (intern) | `ManagerError` *(pub(crate))* | `StaleMapping` | `resident_index.md` („nur intern") |
| Präzision/Kalibrierung | `PrecisionError` | `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision`(NaN) | `resident_precision.md` |
| Planner | `PlannerError` | `InvalidConfig`, `MissingCapability`, `UnknownModel` | `resident_planner.md` |
| Config/Fassade | `ConfigError`, `CapabilityError` | ungültige Config, fehlende/unbekannte Capability | Metaplan A4/B6, `resident.md` |

**Top-Level-Aggregat** `ResidentError` (`#[non_exhaustive]`), das die Komponenten-Enums per `#[from]` aggregiert:
```rust
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ResidentError {
    #[error(transparent)] Index(#[from] IndexError),
    #[error(transparent)] Precision(#[from] PrecisionError),
    #[error(transparent)] Planner(#[from] PlannerError),
    #[error(transparent)] Config(#[from] ConfigError),
    #[error(transparent)] Capability(#[from] CapabilityError),
    // Transfer/IO/… nach Bedarf
}
```

**Regeln:**
- Öffentliche API (Fassade, `Index`-Trait, Planner, Calibrator) gibt `Result<_, ResidentError>` bzw. das jeweilige Komponenten-Enum zurück — **nie** `anyhow`.
- `StaleMapping` bleibt `pub(crate)` und wird intern per Retry (Kommunikationsslot) behandelt oder an der Grenze auf eine öffentliche Variante gemappt; es erscheint **nicht** nach außen.
- Task-Zustände (`Queued|Taken|Done|Cancelled`) sind **keine** Fehler; `Cancelled` ist lokal abgeleitet (`resident.md` Queue-Logik). Fehler eines Batchs werden als `Err` in die betroffenen Task-Results geschrieben (`resident_processor.md`).
- GPU-Initfehler → Fallback auf CPU-Engine, GPU als nicht verfügbar markiert (wiederholbar), als Fehlerzustand `Failed` der Ressource, nicht als API-Fehler des Nutzers (`resident_subengine.md` §6).
- Fehlervarianten mit **strukturierten Feldern** (z. B. `UnsupportedScope { method: AccessMethod }`, `ArenaExhausted { requested, available }`), keine Strings.
- Optional (falls API-Stabilität über interne Refactorings gefordert): die **Fassade** exponiert einen opaken `Error` + `kind() -> &ErrorKind` (`#[non_exhaustive]`), während intern die Komponenten-Enums genutzt werden.

## 4. Umsetzungsschritte

1. `error`-Modul je Komponente anlegen; Komponenten-Enums mit `#[non_exhaustive]` + strukturierten Feldern.
2. `ResidentError` als Aggregat mit `#[from]`/`transparent`.
3. `pub(crate)`-interne Enums (`ManagerError`) + Mapping an der API-Grenze.
4. Doku: Skala/Semantik dokumentieren (z. B. `InvalidPrecision` = NaN; `precision`-Klemmen ist kein Fehler, `resident_precision.md`).
5. `anyhow` nur in Binärs/Tests als dev-/bin-dependency, nicht in der Lib-API.

## 5. Tests

- `#[from]`-Konvertierungen: Komponenten-Enum → `ResidentError`.
- `UnsupportedScope` entsteht bei nicht deklariertem Verfahren/fehlendem Resolver (Zusammenspiel mit E2); kein stilles `All`.
- `InvalidPrecision` bei NaN; Klemmen `[0,1]` erzeugt **keinen** Fehler (F4).
- Interne `StaleMapping` erscheint nicht in der öffentlichen Signatur (Kompilier-/Sichtbarkeitstest + API-Grenze-Mapping).
- `#[non_exhaustive]`: Hinzufügen einer Variante bricht kein `match` in Tests mit `_`-Arm.

## 6. Offene Verifikationen ([prüfen])

- `thiserror`-Version (2.x) in der Projekt-`Cargo.toml`.

## 7. Definition of Done

- Komponenten-Enums + `ResidentError` existieren, alle gelisteten Varianten zugeordnet, `#[non_exhaustive]`, strukturierte Felder, keine `anyhow`-Leaks in der Lib-API, interne Varianten gekapselt; Tests aus §5 grün.

## Result

The structure now matches the Q1 spec exactly: `IndexError` (UnsupportedScope, IndexUnavailable, ArenaExhausted, TransferFailed), internal `pub(crate) ManagerError` (StaleMapping), `PrecisionError` (Incompatible, CalibrationUnstable, NoActiveProfile, InvalidPrecision), `PlannerError` (InvalidConfig, MissingCapability, UnknownModel), `CapabilityError` + `ConfigError`, and `ResidentError` aggregating all public enums via `#[from]`/`transparent`. Validated to compile syntactically.
