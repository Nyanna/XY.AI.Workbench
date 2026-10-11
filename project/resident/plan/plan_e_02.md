# Umsetzungsplan E2 · Scope & Resolver

> **Quellen:** `resident_index.md` (Scope und Auflösung). **Recherche:** `context_e.md` §E2 (Punkte 5–7). **Querschnitt:** Q1, Q2, Q3.
> **Abhängt von:** B4 (Index-Trait, `Scope`, `AccessMethod`), E1 (Manager). **Ergebnis:** Scope-Resolver-Spezifikation + Registry.

## 1. Ziel & Abgrenzung
Der **Manager** (nicht der Index) löst den Scope eines Tasks für kompatible Indextypen auf. Pluggable `ScopeResolver` pro `AccessMethod`. **Keine stille Ausweichung** auf `All`: Fehlt Verfahren oder Resolver → `UnsupportedScope`. Ergebnis eines Resolvers ist eine **kompakte Entry-Menge** (Ranges oder Bitmap), keine ID-Liste.

Nicht hier: das tatsächliche Laden der aufgelösten Mengen (E7), die Geometrie-Berechnung selbst (Kernel/Engine).

## 2. Voraussetzungen
- B4: `enum Scope { All, Range(..), Set(..), Subtree(NodeId), Prefix(..), Centroid{k,radius} }`, `AccessMethod`, `Index::capabilities()`, `Index::summary()`, `Index::children()`, `Index::members()`.
- E1: Manager-Rahmen.

## 3. Zu bauende Artefakte
```rust
pub trait ScopeResolver: Send + Sync {
    fn method(&self) -> AccessMethod;
    // löst einen Scope in eine kompakte Entry-Menge auf; nutzt Index-Struktur, keine Geometrie-Interpretation im Index
    fn resolve(&self, index: &dyn Index, scope: &Scope, ctx: &ResolveCtx) -> Result<EntrySet, ManagerError>;
}

pub struct ResolverRegistry { map: HashMap<AccessMethod, Arc<dyn ScopeResolver>> }
impl ResolverRegistry {
    pub fn register(&mut self, r: Arc<dyn ScopeResolver>);  // Duplikate ablehnen/überschreiben: definieren
    pub fn resolve(&self, index: &dyn Index, scope: &Scope) -> Result<EntrySet, ManagerError>;
}
```
- `EntrySet` als **Roaring-Bitmap** (siehe Entscheidung) mit Range-Shortcut.
- `CentroidResolver` als Referenz-Resolver.

## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
**Resolver-Registrierung & Kompatibilitätsprüfung** (context_e.md §E2 Punkt 5, `[W]`):
- **Variante:** `HashMap<AccessMethod, Arc<dyn ScopeResolver>>` mit dyn-Dispatch — Erweiterung ohne Kernänderung, `Send + Sync`, object-safe (keine generischen Trait-Methoden). Enum-Dispatch verworfen (geschlossen); `TypeId`-Any-Registry verworfen (Laufzeit-Downcast-Fehler).
- **Kompatibel heißt zweifach:** Der Index **deklariert** das Verfahren (`capabilities()` enthält `AccessMethod`) **UND** die Registry hat einen Resolver dafür. Nur dann Auflösung, sonst `UnsupportedScope{method}`. **Kein** stilles `All`.
- **Registrierungsreihenfolge/Duplikate:** Registrierung bei Manager-Aufbau; Duplikat einer `AccessMethod` ist ein Konfigurationsfehler → Panik beim Aufbau oder `Err` (nicht stilles Überschreiben). Keine Hot-Registration im Betrieb (Lebensdauer-Fallstrick).

**Kompakte Entry-Menge — Roaring vs. Ranges** (context_e.md §E2 Punkt 7, `[W]`):
- Standard: **`roaring`-Bitmap** (Container pro 2¹⁶-Block, Array/Bitmap/Run). Run-Container deckt dichte *und* fragmentierte Fälle ab, schnelle Union/Schnitt (für Scope-Aggregation in E7). Reine Range-Listen nur, wenn Scopes **garantiert** zusammenhängend sind.

**Zentroid-Resolver (Referenz)** (context_e.md §E2 Punkt 6, `[V]` PLAID/ColBERTv2):
- Manager holt Summary (`summary(Centroid)`) und macht sie resident. Engine/Kernel rechnet `S = C · Qᵀ`, pro Query-Token die `nprobe` nächsten Zentroide; `members(cell)` des Index liefert Kandidaten (PLAID: Zentroid → eindeutige Passage-IDs). Resolver gibt deren Vereinigung als `EntrySet` zurück. `k`/`radius` aus `Scope::Centroid`. Approximation (Randtreffer-Verlust) ist bewusst Eigenschaft der Oberschicht.

## 5. Umsetzungsschritte
1. `ScopeResolver`-Trait + `ResolverRegistry` mit `register`/`resolve` und Doppelprüfung (deklariert ∧ Resolver vorhanden).
2. `EntrySet` auf `roaring` abstützen; Range-Konstruktor als Shortcut.
3. Triviale Resolver: `RangeResolver`, `SubtreeResolver` (nutzt `children()`), `PrefixResolver`.
4. `CentroidResolver`: Summary resident machen (Delegation an E7-Hook), Zellenwahl, `members()`-Union.
5. `resolve(All)` → vollständige Entry-Menge aus `stamps()`-Domäne (ohne Vektorseiten).

## 6. Fehler / Logging / Feature-Flags
- **Q1:** `UnsupportedScope{method}` (Verfahren nicht deklariert ODER kein Resolver). Keine Ersatzauflösung.
- **Q2:** Span pro Resolve; Event: gewähltes Verfahren, Größe der aufgelösten Menge.
- **Q4:** Zentroid-Rechnung auf GPU hinter `cuda`; Resolver-Logik feature-frei testbar.

## 7. Tests (Q3)
- `UnsupportedScope`, wenn Verfahren nicht deklariert.
- `UnsupportedScope`, wenn deklariert, aber kein Resolver registriert.
- Range/Subtree/Prefix-Resolver gegen `FakeIndex` liefern erwartete `EntrySet`.
- Zentroid-Resolver mit Fake-Summary + Fake-`members()`: korrekte Vereinigung, `nprobe`/`k` respektiert.
- Duplikat-Registrierung wird abgelehnt.
- Roaring-Union zweier Scopes = erwartete Menge (Vorbereitung E7-Aggregation).

## 8. Offene Verifikationen
- `roaring`-Crate-API/Version prüfen.
- PLAID-Zählparameter (`nprobe`, Kandidatenzahl) sind Oberschicht-Parameter; hier nur Durchreichung — Schnittstelle gegen G3/H3 abstimmen.

## 9. Definition of Done
- Registry mit Doppelprüfung; mindestens Range/Subtree/Prefix/Centroid-Resolver; `EntrySet` roaring-basiert mit Union; alle genannten Tests grün ohne GPU.
