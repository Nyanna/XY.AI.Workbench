Setze E2 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert
- ziehe AST-Tools und spezifische Tools vor; Nutze kein `cat` zum auslesen ganzer Dateien

# Umsetzungsplan E2 · Scope & Resolver

> **Quellen:** `resident_index.md` (Scope und Auflösung). **Recherche:** `context_e.md` §E2 (Punkte 5–7). **Querschnitt:** Q1, Q2, Q3.
> **Boundary:** see `rag/docs/architecture/scope_charta.md` (engine vs. search-layer scope, ring buffer vs. pull queue).
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

## Result

E2 ist umgesetzt, und alle 8 neuen Tests laufen grün (286 bestanden im gesamten `resident::`-Lauf). Der Code ist in `rag/src/resident/manager/scope.rs`.

Von der Spezifikation weiche ich an drei Stellen ab:
- **Kein `roaring`:** Die Crate ist weder in `Cargo.lock` noch in der lokalen Registry vorhanden. Ich habe die vorhandene range-basierte `EntrySet` (sortierte, verschmolzene Ranges) behalten und um `union` und `intersection` erweitert. Für E7 reicht das. Falls du `roaring` trotzdem willst, ist das ein Austausch der Datenstruktur in `index/mod.rs`.
- **Fehlertyp:** Die Methoden liefern `IndexError` statt `ManagerError`. `UnsupportedScope` und `IndexUnavailable` liegen nur dort, und `ManagerError` hat bisher nur `StaleRegion`.
- **Prefix-Auflösung:** Der `Index`-Trait bietet dafür keine Abfrage. Ich lese das Präfix deshalb als führende Big-Endian-Bytes der 64-Bit-Entry-ID und bekomme eine zusammenhängende Range. Präfixe über 8 Bytes liefern eine leere Menge. Bei leerem Präfix endet die Range bei `u64::MAX`, die ID `u64::MAX` selbst ist also nicht enthalten. Wenn Präfixe in deinem Index etwas anderes bedeuten, muss `PrefixResolver` angepasst werden.

Umgesetzt ist sonst:
- **Trait und Registry:** `ScopeResolver` und `ResolverRegistry` mit Doppelprüfung. Das Verfahren muss in `capabilities()` deklariert sein und ein Resolver muss registriert sein, sonst kommt `UnsupportedScope`. Ein doppeltes `register` gibt einen Fehler zurück.
- **Resolver:** `RangeResolver`, `SubtreeResolver`, `PrefixResolver` und `CentroidResolver`. `All` kommt aus `stamps()`, `Set` wird durchgereicht.
- **Zentroid:** Der Resolver holt die Summary und lässt über das Trait `CellSelector` die Zellen wählen. Er schneidet bei `k` ab und vereinigt die `members()`.
- **Tracing:** Ein Span pro Auflösung und ein Event mit der Größe der aufgelösten Menge.

`manager/mod.rs` enthält jetzt nur `pub mod scope;`. `advice.rs` und `tracker.rs` sind dort noch nicht eingebunden. Das habe ich so gelassen.
