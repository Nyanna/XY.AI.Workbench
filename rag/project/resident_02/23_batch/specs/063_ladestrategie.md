# Spezifikation: Ladestrategie (Build-Order 63)

## 1. Aufgabe, Einschränkungen, Grenzen

Rust-Modul `load_strategy`. Es besteht aus drei Teilen:
1. **Planung:** Es übersetzt eine `EntrySet`, die Zugriffsart, die Geometrie und die Policy in Lade-Pläne (ganze Segmente, Gather, direktes Mapping/Zero-Copy). Die Planung ist eine reine Funktion.
2. **Segmentgröße:** Es bestimmt die Segmentgröße nach Budget.
3. **Tiering:** Der `LoadManager` hält Summary-Strukturen, Vektorsegmente und nicht geladene Bereiche gestaffelt. Er lädt lazy und scope-getrieben, also erst auf Anforderung eines Tasks und nur das Benötigte.

Nicht Teil des Moduls:
- Es führt keine echte Kopie, kein Mapping und keine Device-/Host-Allokation aus. Die Ausführung eines Plans delegiert es an einen Aufrufer-Closure.
- Es löst keine Scopes auf. Es bekommt fertige `EntrySet`s.
- Es definiert keine Index-Implementierung und kein Profil- oder Capability-Modell.
- Die Segmentgröße ist keine Eigenschaft des Index.

Abhängigkeiten: Scopes und Entry-Mengen (`EntrySet`), Allgemeiner Index-Vertrag, Arenen und Speicherbudget, Profilauswahl, Capability-Modell und `log`.

## 2. Kontext

- Retrieval- und Task-Module fordern für einen Scope die Entries an. Sie lösen den Scope vorher zu einer `EntrySet` auf.
- Der Manager plant daraus Ladeschritte, die ein Lader ausführt.
- Die Arena `IndexSegments` und das `Budget` (Modul `arenas`) liefern die verfügbare Menge für die Segmentgröße.
- Die kalibrierte Dichteschwelle kommt als `PrecisionProfile` (aus der Profilauswahl). `config` ist `BTreeMap<Axis, Value>`, und `Axis` und `Value` sind `Name` aus dem Capability-Modell.
- Summary-Strukturen kommen über `Index::summary(NodeId)` aus dem Index-Vertrag.
- Es gibt zwei `EntryId`-Typen. In diesem Modul gilt `u64` (die Scopes-Variante `EntryId = u64`). Die Geometrie und alle Pläne arbeiten mit rohen `u64`-IDs. `NodeId` stammt aus dem Index-Vertrag.

## 3. Interface

```rust
use std::sync::Arc;
use arenas::{Budget, MIB};
use entry_sets::EntrySet;                    // "Scopes und Entry-Mengen"
use index_contract::{Index, IndexError, NodeId, Summary};
use pareto_profiles::PrecisionProfile;

pub const DEFAULT_DENSITY: f64 = 0.12;
pub const DEFAULT_SEGMENT_BYTES: u64 = 64 * MIB;
pub const DEFAULT_DIRECT_MAP_MAX_BYTES: u64 = 64 * 1024;

/// Austauschbare Geometrie.
pub trait Geometry: Send + Sync {
    fn segment_of(&self, entry: u64) -> u64;      // Segmentindex des Entries
    fn entry_size(&self, entry: u64) -> u64;      // Bytes
    fn segment_size(&self, segment: u64) -> u64;  // Bytes
}

/// Mitgeliefert: gleich große Entries in gleich großen Segmenten.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformGeometry { /* entry_bytes: u64, entries_per_segment: u64 */ }
impl UniformGeometry {
    /// entries_per_segment == 0 gilt als 1.
    pub fn new(entry_bytes: u64, entries_per_segment: u64) -> Self;
    /// Kapazität = policy.segment_bytes() / entry_bytes (Ganzzahl; entry_bytes == 0 oder Ergebnis 0 => 1).
    pub fn for_policy(entry_bytes: u64, policy: &LoadPolicy) -> Self;
}
impl Geometry for UniformGeometry {}
// segment_of = entry / cap; entry_size = entry_bytes; segment_size = cap.saturating_mul(entry_bytes)

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Access { Regular, Rare }   // Rare = kleine, selten gelesene Menge aus gemapptem Host-Memory

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadPolicy { /* density_threshold, segment_bytes, direct_map_max_bytes: private */ }
impl Default for LoadPolicy {}      // 0.12 / 64 MiB / 64 KiB
impl LoadPolicy {
    pub fn density_threshold(&self) -> f64;
    pub fn segment_bytes(&self) -> u64;
    pub fn direct_map_max_bytes(&self) -> u64;
    /// Nicht endlich (NaN, ±inf) => unverändert; sonst auf [0,1] begrenzt.
    pub fn with_density(self, d: f64) -> Self;
    pub fn with_segment_bytes(self, b: u64) -> Self;
    pub fn with_direct_map_max_bytes(self, b: u64) -> Self;
    /// Segmentgröße = segment_bytes_for_budget(budget.usable()).
    pub fn with_budget(self, budget: &Budget) -> Self;
    /// Übernimmt Dichte aus Achse "density" (Wert = ganze Promille, dezimal). true iff übernommen.
    pub fn apply_profile(&mut self, profile: &PrecisionProfile) -> bool;
}

/// available/8, auf [4 MiB, 256 MiB] begrenzt, auf ganze MiB abgerundet.
pub fn segment_bytes_for_budget(available: u64) -> u64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadPlan {
    /// Ganze Segmente (aufsteigend, ohne Duplikate); bytes = Summe segment_size.
    Segments { segments: Vec<u64>, bytes: u64 },
    /// Einzelne Entries (aufsteigend), gemeinsam gesendet; bytes = Summe entry_size.
    Gather { entries: Vec<u64>, bytes: u64 },
    /// Direktes Mapping ohne Kopie; bytes = Summe entry_size.
    DirectMap { entries: EntrySet, bytes: u64 },
}
impl LoadPlan { pub fn bytes(&self) -> u64; }

/// Reine Planung. Segment-Plan zuerst, dann Gather.
pub fn plan_loads(set: &EntrySet, access: Access, geometry: &dyn Geometry, policy: &LoadPolicy) -> Vec<LoadPlan>;

/// Führt Pläne in Reihenfolge aus. exec liefert die geladenen Bytes je Plan.
/// Bricht beim ersten Fehler ab (Fehler unverändert zurück); sonst Summe, saturierend.
pub fn execute_plans<E>(plans: &[LoadPlan], exec: impl FnMut(&LoadPlan) -> Result<u64, E>) -> Result<u64, E>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier { Summary, Vector, NotLoaded }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Resident { Summary(NodeId), Segment(u64) }

/// Thread-safe Tiering-Verwaltung (alle Methoden &self).
pub struct LoadManager { /* Mutex-geschützter Zustand */ }
impl LoadManager {
    pub fn new(index: Arc<dyn Index>, geometry: Arc<dyn Geometry>, policy: LoadPolicy) -> Self;
    pub fn policy(&self) -> LoadPolicy;
    /// Lädt fehlende Summaries per Index::summary (zuerst, vor Vektoren). Bricht beim ersten Fehler ab.
    /// Rückgabe: Anzahl neu geladener Summaries.
    pub fn load_summaries(&self, nodes: &[NodeId]) -> Result<usize, IndexError>;
    pub fn summary(&self, node: NodeId) -> Option<Summary>;   // nur Cache, lädt nicht
    /// Plant wie plan_loads, lässt aber bereits residente Segmente (und Entries darin) weg.
    pub fn plan(&self, set: &EntrySet, access: Access) -> Vec<LoadPlan>;
    /// Meldet erfolgreich geladene Vektorsegmente (Segmentindex, Bytes); aktualisiert LRU.
    pub fn mark_segment_loaded(&self, segment: u64, bytes: u64);
    pub fn touch_segment(&self, segment: u64);
    pub fn tier(&self, r: Resident) -> Tier;
    pub fn resident_bytes(&self) -> u64;
    /// Gibt mindestens `bytes` frei, soweit möglich: erst Vektorsegmente (LRU), dann Summaries (LRU).
    /// Rückgabe: tatsächlich freigegebene Bytes.
    pub fn evict(&self, bytes: u64) -> u64;
}
```

`Geometry`, `UniformGeometry`, `LoadPolicy`, `LoadPlan` und `LoadManager` sind `Send + Sync`.

## 4. Behavior und Invarianten

**Policy**
- Default: Dichte 0,12 (Anteil der Segmentbytes), Segment 64 MiB, Direktmapping-Höchstgröße 64 KiB.
- `with_density` ignoriert NaN und ±inf (Wert bleibt). Endliche Werte werden auf [0,1] begrenzt.
- `apply_profile`:
  - Es liest `profile.config` bei Achse `axes::density()`. Der Wert muss als `u64` dezimal parsebar sein (Promille).
  - Dichte = Promille/1000, über `with_density` begrenzt (1500 ergibt 1,0).
  - Fehlt die Achse oder ist der Wert nicht parsebar, bleibt die Policy unverändert und die Rückgabe ist `false`.
- `segment_bytes_for_budget(a)` = `clamp(a/8, 4 MiB, 256 MiB)`, abgerundet auf ein Vielfaches von MiB. Beispiele:
  - `0` ergibt 4 MiB.
  - `u64::MAX` ergibt 256 MiB.
  - 100 MiB ergibt 12 MiB (12,5 abgerundet).

**plan_loads**
1. Eine leere `set` ergibt `vec![]`.
2. Bei `Access::Rare` und nicht leerer `set` wird die Gesamtgröße (Summe `entry_size`, saturierend) gebildet. Ist sie `<= direct_map_max_bytes`, ist das Ergebnis genau `[DirectMap{entries: set, bytes}]`. Gleichheit zählt. Ist sie größer, gilt Schritt 3 wie bei `Regular`.
3. Die Entries werden nach `segment_of` gruppiert. Je Segment `s` ist `r` die Summe der `entry_size` der angeforderten Entries (saturierend) und `S = segment_size(s)`.
   - Ganzes Segment, wenn `S > 0` und `r as f64 >= density_threshold * S as f64`. Bei Dichte 0 wird also jedes Segment mit angeforderten Entries ganz geladen.
   - Sonst werden die Entries des Segments gethert.
4. Das Ergebnis enthält höchstens einen `Segments`-Plan und höchstens einen `Gather`-Plan. Der `Segments`-Plan steht zuerst. Fehlt eine Art, gibt es keinen leeren Plan.
5. Segment- und Entry-Listen sind aufsteigend und duplikatfrei. Jeder Entry steckt in genau einem Plan, entweder direkt oder über sein Segment.
6. Jede Entscheidung wird geloggt, mit `log::debug!` je Segment (Segment, `r`, `S`, Entscheidung) und je Plan (Art, Bytes). Ein Fehler beim Loggen kann nicht auftreten.
7. Die Funktion ist deterministisch, panikfrei und überlaufsicher (saturierend). Die Kosten wachsen mit `set.len()`, denn sie iteriert die Entries.

**execute_plans**
- Die Pläne laufen in Slice-Reihenfolge, jeweils einmal.
- Beim ersten `Err` bricht die Funktion ab. Spätere Pläne werden nicht ausgeführt, und der Fehler wird unverändert zurückgegeben.
- Ohne Fehler ist das Ergebnis die saturierende Summe der von `exec` gemeldeten Bytes (nie Überlauf oder Wrapping). Leere Pläne ergeben `Ok(0)`.

**Tiering**
- `Tier::Summary` heißt, die Summary des Knotens ist gehalten. `Tier::Vector` heißt, das Segment ist als geladen markiert. Alles andere ist `Tier::NotLoaded`. Ein `Resident::Summary(_)` hat nie `Tier::Vector` und umgekehrt.
- Nichts wird beim Konstruieren geladen. Geladen wird nur durch `load_summaries` und `mark_segment_loaded`, die ein Task auslöst (lazy).
- `load_summaries` überspringt bereits gehaltene Knoten.
  - Der Index-Aufruf läuft ohne gehaltenen Lock.
  - Bei Fehler bleiben bereits geladene Summaries erhalten.
  - Bei gleichzeitigem Laden desselben Knotens gewinnt der erste Eintrag, es entsteht kein Doppelzustand.
- `plan` entfernt residente Segmente aus dem `Segments`-Plan und residente Entries aus Gather und DirectMap. Ein dadurch leerer Plan entfällt. Bytes werden entsprechend neu berechnet.
- `mark_segment_loaded` ersetzt die Bytes bei erneutem Aufruf und setzt das Segment an die jüngste LRU-Position. `touch_segment` tut das ebenfalls, aber nur für residente Segmente. Unbekannte Segmente werden ignoriert.
- `resident_bytes` = Summe der Segmentbytes plus der Summary-Bytes (`centroid.len()` plus 24 je Summary, saturierend).
- `evict(bytes)` gibt zuerst Vektorsegmente frei, ältester Zugriff zuerst. Erst wenn danach noch Bedarf besteht und keine Vektorsegmente mehr da sind, gibt es Summaries frei, ebenfalls LRU. `evict(0)` gibt nichts frei.
- Summaries werden also zuerst geladen und am längsten gehalten. Vektorsegmente sind warm und evictable.
- Bei überlappenden Entries (Vektoren einmal halten und mehrfach referenzieren, oder duplizieren) entscheidet der Manager intern. Es gibt dazu keine Vertragsaussage, und kein Test darf davon abhängen.

**Nebenläufigkeit und Fehler**
- Alle `LoadManager`-Methoden sind von beliebig vielen Threads aufrufbar. Der interne Lock ist poison-tolerant (`into_inner`) und wird nie über Index- oder Closure-Aufrufe gehalten, daher gibt es keine Deadlocks.
- `plan_loads` und `execute_plans` sind zustandslos.
- Der einzige Fehlerfall ist `IndexError` aus `load_summaries` (unverändert durchgereicht, etwa `Unsupported(Centroid)` oder `NotFound`). Sonst entstehen keine Fehler und keine Panics.

## 5. Relied-on elements

- Aus `entry_sets` (Scopes und Entry-Mengen):
  - `EntrySet` hat `is_empty()`, `len() -> u128`, `iter()` (lazy, streng aufsteigend, `u64`) und `Clone`.
  - Weiterhin hat es `Eq`, `Hash`, `Send + Sync`. `EntrySet::from_ids` und `from_ranges` bauen Sets.
- Aus `index_contract`:
  - `NodeId(pub u64)` (`Copy`, `Eq`, `Hash`, `Ord`).
  - `Summary { node: NodeId, centroid: SharedBytes, count: u64, stamp: Stamp }` (`Clone`).
  - `IndexError` mit `Unsupported(AccessMethod)`, `NotFound`, `Backend(String)` (`Clone`).
  - `Index: Send + Sync` mit `fn summary(&self, node: NodeId) -> Result<Summary, IndexError>`. Der Default ist `Err(Unsupported(Centroid))`.
- Aus `arenas`:
  - `MIB: u64 = 1024*1024`.
  - `Budget { free, total, generation_reserve }` mit `fn usable(&self) -> u64 = free.saturating_sub(generation_reserve)`.
- Aus `pareto_profiles`:
  - `PrecisionProfile { position: f64, config: BTreeMap<Axis, Value> }`.
- Aus dem Capability-Modell:
  - `Name` (`Axis` und `Value` sind Aliase) mit `as_str()` und `Ord`.
  - `axes::density()` ist `"density"`.
  - Numerische Achsenwerte sind Dezimalstrings (`Name::from_number`).