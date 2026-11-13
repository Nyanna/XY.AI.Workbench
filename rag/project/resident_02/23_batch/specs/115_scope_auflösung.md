# Spezifikation: Scope-Auflösung (Build-Order 115)

Crate-Modul `scope_resolution`.

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe**
- Den `Scope` einer Operation über registrierte Resolver in ein `EntrySet` auflösen. `All` und `Set` löst das Modul selbst auf.
- Nicht unterstützte Scopes ablehnen. Es gibt kein stilles Ausweichen auf alle Entries.
- Standard-Resolver für Range, Subtree, Prefix und Centroid bereitstellen. Doppelte Registrierung wird abgewiesen.
- Aus einer Vorverarbeitung bekannte Kandidaten gezielt laden (`load_candidates`).

**Grenzen**
- Das Modul enthält keine Prefilter-Logik. Dazu gehören Centroid-Distanzrechnung, PLAID-Stufen, Pruning, Fourier und Kompression. Verlorene Randtreffer approximativer Prefilter sind Sache der äußeren Schichten.
- Der Index löst keine Scopes auf, das tut nur dieses Modul. Das Modul schreibt nie in den Index, hat keine Residency-Verwaltung und keinen Upload.
- Den Abgleich gegen den Ladezustand (fehlend, stale) macht der Manager mit `reconcile`. Das Modul liefert nur die Kandidatenmenge und das gezielte Laden.
- Kein `unsafe`. Abhängigkeiten: `std`, `tracing`, `scopes`, `index_contract`, `reconcile`.

## 2. Kontext

- `scopes` liefert `Scope`, `ScopeKind` und `EntrySet`. Dort ist `EntryId = u64`. `index_contract::EntryId(pub u64)` ist ein anderer Typ. Dieses Modul wandelt an der Grenze um, also `EntryId(x)` ↔ `x`. Im Folgenden steht `u64` für IDs in `EntrySet` und `index_contract::EntryId` für Index-IDs.
- Der Index deklariert Zugriffsmethoden (`access_methods()`). Der Manager ruft `ScopeResolverRegistry::resolve` auf und erhält ein kompaktes `EntrySet`.
- Bei Centroid macht der Manager die Summary-Struktur resident. Die Query wird gegen die Centroids gerechnet, und das übernimmt der `CellSelector` aus dem Kontext, den der Manager bereitstellt. Danach fragt der Resolver die Mitglieder der gewählten Zellen ab. Der Manager gleicht die Menge anschließend mit `Reconciler` und `ResidencyDirectory` gegen den Ladezustand ab.
- Ist die Kandidatenmenge schon aus einer Vorverarbeitung bekannt, ruft der Manager `load_candidates` direkt auf. Die Menge ist ein `EntrySet`. Es wird nur geladen, was darin steht.

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::{AccessMethod, CellId, EntryId, Index, IndexError};
use reconcile::LoadedEntry;
use scopes::{EntrySet, Scope, ScopeKind};

/// Name einer Zugriffsmethode für Fehler/Logs: "centroid" | "prefix" | "subtree" | "range".
pub const fn method_name(m: AccessMethod) -> &'static str;

#[derive(Debug, Clone, PartialEq)]
pub enum ResolveError {
    /// "Scope nicht unterstützt". `method` = Methodenname (siehe `method_name`),
    /// bzw. `scope.name()`, wenn der Scope-Art keine Methode zugeordnet ist.
    Unsupported { method: String },
    /// "Index nicht verfügbar"
    IndexUnavailable,
    /// Für diese `ScopeKind` ist schon ein Resolver registriert (auch All/Set sind belegt).
    DuplicateResolver { kind: ScopeKind },
    /// Fehler des Index (z. B. NotFound), unverändert durchgereicht.
    Index(IndexError),
}
impl std::fmt::Display for ResolveError { /* sprechender Text */ }
impl std::error::Error for ResolveError {}

/// Wählt die nächsten Zellen für die (im Selector gebundene) Query anhand der Summary-Struktur.
pub trait CellSelector: Send + Sync {
    /// Höchstens `count` Zellen, nächste zuerst; `radius` optional als Obergrenze der Distanz.
    fn select(&self, index: &dyn Index, count: u32, radius: Option<f32>)
        -> Result<Vec<CellId>, IndexError>;
}

#[derive(Clone, Copy, Default)]
pub struct ResolveContext<'a> {
    pub index: Option<&'a dyn Index>,
    pub cell_selector: Option<&'a dyn CellSelector>,
}

pub trait ScopeResolver: Send + Sync {
    /// Scope-Art, für die der Resolver zuständig ist (Registry-Schlüssel).
    fn kind(&self) -> ScopeKind;
    /// Vom Index zu deklarierende Methode.
    fn access_method(&self) -> AccessMethod;
    fn resolve(&self, scope: &Scope, ctx: &ResolveContext<'_>) -> Result<EntrySet, ResolveError>;
}

pub struct RangeResolver;     // ScopeKind::Range,    AccessMethod::Range
pub struct SubtreeResolver;   // ScopeKind::Subtree,  AccessMethod::Subtree
pub struct PrefixResolver;    // ScopeKind::Prefix,   AccessMethod::Prefix
pub struct CentroidResolver;  // ScopeKind::Centroid, AccessMethod::Centroid
// alle: Default + Clone + Send + Sync, impl ScopeResolver

pub struct ScopeResolverRegistry { /* private */ }   // Send + Sync
impl ScopeResolverRegistry {
    /// Leer (nur All und Set eingebaut).
    pub fn new() -> Self;
    /// Mit den vier Standard-Resolvern.
    pub fn with_defaults() -> Self;
    pub fn register(&mut self, r: Arc<dyn ScopeResolver>) -> Result<(), ResolveError>;
    pub fn resolve(&self, scope: &Scope, ctx: &ResolveContext<'_>) -> Result<EntrySet, ResolveError>;
}
impl Default for ScopeResolverRegistry {}   // == with_defaults()

/// Lädt genau die Mitglieder von `candidates`, aufsteigend, je Entry per `reconcile::load_entry`.
/// `f(id, None)` für Entries, die im Index fehlen. Liest keine Entries außerhalb der Menge.
pub fn load_candidates<F>(index: &dyn Index, candidates: &EntrySet, f: F)
where F: FnMut(EntryId, Option<LoadedEntry>);
```

## 4. Behavior und Invarianten

**Registry und Dispatch (`resolve`)**
1. `Scope::Set(s)` ergibt `Ok(s.clone())`. Es braucht weder Index noch Resolver.
2. Alle anderen Arten brauchen `ctx.index`. Fehlt er, kommt `IndexUnavailable`.
3. `Scope::All` ergibt die IDs aus `index.stamps()`, also ein `EntrySet` aller vorhandenen Entries. Es werden keine Vektordaten gelesen. Es braucht keine Methode.
4. Für die übrigen Arten sucht die Registry den Resolver zur `scope.kind()`.
   - Gibt es keinen, kommt `Unsupported { method: scope.name() }`. Das gilt auch für künftige, unbekannte Scope-Arten.
   - Deklariert `index.access_methods()` die `access_method()` des Resolvers nicht, kommt `Unsupported { method: method_name(..) }`.
   - Sonst ruft die Registry `resolver.resolve` auf.
5. Es gibt kein stilles Ausweichen auf alle Entries. Jeder Fehlerfall liefert `Err`.
6. `register` weist eine zweite Registrierung derselben `ScopeKind` mit `DuplicateResolver { kind }` ab, der Zustand bleibt unverändert. `All` und `Set` sind immer belegt, `register` dafür ergibt ebenfalls `DuplicateResolver`.
7. `resolve` läuft in einem `tracing`-Span (Name `scope_resolve`) mit dem Feld `method`, belegt mit `scope.name()`. Nach Erfolg loggt es auf debug-Level die Anzahl (`count = set.len()`). Bei Fehlern wird keine Anzahl geloggt.
8. Jeder Standard-Resolver, der mit einem Scope der falschen Art direkt aufgerufen wird, liefert `Unsupported { method: method_name(seine Methode) }`. Das ist unabhängig davon, ob `ctx.index` gesetzt ist.

**Range**
- `Scope::Range{start,end}` ergibt genau `EntrySet::from_ranges([start..end])`, also halboffen. `start >= end` ergibt die leere Menge. Es werden keine Index-Aufrufe zur Bildung gemacht. Die Menge wird nicht gegen vorhandene Entries gefiltert.

**Subtree**
- Ergebnis sind die Kinder des Knotens: `index.children(NodeId(root))`, die `NodeId.0` als IDs in `EntrySet::from_ids`. Der Knoten selbst gehört nicht dazu.
- Ein Index-Fehler (z. B. `NotFound`) kommt als `ResolveError::Index(e)` zurück.

**Prefix**
- Die Bytes sind die führenden Bytes der 64-Bit-ID in Big-Endian und bilden einen zusammenhängenden Bereich. Sei n = `bytes.len()`.
  - n = 0: alle IDs `0..=u64::MAX`.
  - n > 8: leere Menge.
  - 1 ≤ n ≤ 8: `lo` sind die Bytes, rechts mit 0x00 auf 8 Byte aufgefüllt (Big-Endian). `hi` sind die Bytes, rechts mit 0xFF aufgefüllt. Das Ergebnis ist `lo..=hi`. Die Obergrenze ist auf `u64::MAX` begrenzt, und `u64::MAX` ist enthalten, wenn `hi == u64::MAX`. Weil ein halboffenes `Range` `u64::MAX` nicht enthalten kann, entsteht das Ergebnis als `from_ranges([lo..hi])` vereinigt mit `from_ids([hi])`.
- Beispiele: `[0x12]` ergibt `0x1200_0000_0000_0000..=0x12FF_FFFF_FFFF_FFFF`. `[0xFF; 8]` ergibt die Menge `{u64::MAX}`. `[0xFF]` ergibt `0xFF00…..=u64::MAX`.
- Es gibt keinen Index-Aufruf zur Bildung. Die Methode `Prefix` muss aber deklariert sein (Dispatch Schritt 4).

**Centroid**
- Prüfreihenfolge: `ctx.index` fehlt → `IndexUnavailable`. Dann `Centroid` nicht deklariert → `Unsupported{"centroid"}` (Registry). Dann `ctx.cell_selector` fehlt → `Unsupported{"centroid"}`.
- `count == 0` ergibt die leere Menge, der Selector wird nicht aufgerufen.
- Sonst ruft der Resolver `selector.select(index, count, radius)` auf und gibt `radius` unverändert weiter. Ein `Err(e)` des Selectors ergibt `Index(e)`.
- Die Zellliste wird unter Beibehaltung der Reihenfolge dedupliziert und auf höchstens `count` Zellen gekürzt, auch wenn der Selector mehr liefert.
- Ergebnis ist die Union von `index.cell_entries(cell)` aller gewählten Zellen. Ein Fehler (z. B. `NotFound`) ergibt `Index(e)`, ohne Teilergebnis.

**load_candidates**
- Iteriert `candidates.iter()` aufsteigend und ruft je ID `reconcile::load_entry(index, EntryId(id))` auf. Jede ID erhält genau einen Aufruf von `f`.
- Es werden keine Entries außerhalb von `candidates` gelesen. Eine leere Menge ergibt keinen Index-Zugriff.
- Der Stamp-vor-Daten-Ablauf und die Retries gelten wie in `load_entry`. Das Modul hält keine Locks.

**Nebenläufigkeit und Fehler**
- `ScopeResolverRegistry`, die Resolver und `ResolveError` sind `Send + Sync`. `resolve` ist `&self` und von beliebig vielen Threads aufrufbar. `register` braucht `&mut self`, also Registrierung vor dem geteilten Einsatz.
- Das Modul paniciert nicht bei gültigen Eingaben. Alle Fehler kommen als `ResolveError`.
- Resolver halten keinen Zustand zwischen Aufrufen.

## 5. Relied-on elements

**scopes**
- `Scope` (`#[non_exhaustive]`): `All`, `Range{start:u64,end:u64}` (halboffen, ggf. leer/umgekehrt), `Set(EntrySet)`, `Subtree{root:u64}`, `Prefix{bytes:Vec<u8>}`, `Centroid{count:u32, radius:Option<f32>}`. Dazu `kind() -> ScopeKind` und `name() -> &'static str` ("all","range","set","subtree","prefix","centroid").
- `ScopeKind` (`non_exhaustive`, `Copy + Eq + Hash + Debug`).
- `EntrySet` (unveränderlich, `Clone + Eq + Hash + Send + Sync`): `empty()`, `from_ranges(IntoIterator<Item=Range<u64>>)` (leere/umgekehrte werden ignoriert), `from_ids(IntoIterator<Item=u64>)` (`u64::MAX` erlaubt), `len() -> u128`, `iter()` (aufsteigend, lazy), `union`, `is_empty`.
- Ein halboffenes `Range` kann `u64::MAX` nicht enthalten.

**index_contract**
- `EntryId(pub u64)`, `NodeId(pub u64)`, `CellId(pub u64)` sind getrennte Typen. `AccessMethod { Centroid, Prefix, Subtree, Range }`. `IndexError { InvalidVectorLength, Unsupported(AccessMethod), NotFound, Backend(String) }` ist `Clone + PartialEq`.
- `Index: Send + Sync` mit `access_methods(&self) -> &[AccessMethod]` und `stamps(&self) -> Vec<(EntryId, Stamp)>` (Snapshot, aufsteigend, ohne Vektordaten).
- `children(&self, NodeId) -> Result<Vec<NodeId>, IndexError>` (Subtree, aufsteigend) und `cell_entries(&self, CellId) -> Result<Vec<EntryId>, IndexError>` (aufsteigend). Nicht deklarierte Methoden liefern `Err(Unsupported(..))`. Unbekannte Knoten oder Zellen liefern `NotFound`.
- `summary(&self, NodeId)` und `root()` werden vom `CellSelector` des Managers genutzt, nicht von diesem Modul.

**reconcile**
- `pub fn load_entry(index: &dyn Index, id: EntryId) -> Option<LoadedEntry>`. Der Ablauf ist stamp → read → stamp mit höchstens 8 Wiederholungen, `None` bei fehlendem Entry. Der zurückgegebene Stamp ist nie neuer als die Daten.
- `LoadedEntry { stamp: Stamp, payload: Payload, retries: u32 }` ist `Clone + Send + Sync`.
- `Reconciler`/`ResidencyDirectory` sind für den Abgleich der Kandidaten durch den Manager zuständig, und dieses Modul nutzt sie nicht.