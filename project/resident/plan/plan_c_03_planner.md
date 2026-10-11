# Umsetzungsplan C3 — Planner (reine Taskexpansion)

Stand: 10.10.2026 · Modul: `planner` · Zielsprache Rust
Quellen: `resident_planner.md` (maßgeblich) · `resident.md` (Tasktypen, Kontrollfluss) · Recherche `context/context_c.md` §C3 (Punkte 22–29), Querschnitt 30–32.

---

## 1. Ziel & Einordnung

Der Planner übersetzt einen **Task** (Daten + Parameter) in eine **geordnete Folge von Operationen**. Er ist eine **reine Funktion**: gleiche Eingabe (Task + Config-Snapshot + Capabilities + optionaler Hint) ⇒ gleiche Operationen. **Keine** Queues, Worker, Notify, kein veränderlicher Zustand.

Der Planner arbeitet nur mit **Config-Regeln** und den **Capabilities der Subengines**. Er setzt pro Operation `requires()` (Ressourcenart, Modell, Index, erlaubte Engine), `materialize` und Ein-/Ausgabe als `Value::Host|Device`.

---

## 2. Scope

**In:** Expander-Trait + explizite Registry (pro Modell/Tasktyp), Config-Format + Validierung + Hot-Reload-Snapshot, Expansionsregeln für Embedding/Similarity/Inferenz, `materialize`-Regel, aufgeschobene Device-Wahl, Op-Zähler-Vertrag (Definition; Zählung selbst führt C2 aus), Starved-Hint als reine Eingabe.

**Out:** Orchestrierung/Queues/Worker (C2), Capability-Erhebung und Präzisions-Matching (Gruppe F — der Planner liest `CapabilitySet`/`precision`, berechnet aber keine Profile), Kernel/Index-Interna (D/E), gleitende Indizierung (Ausblick H2 — nur Parameter-Hook).

---

## 3. Abhängigkeiten (Eingangsverträge)

| Vertrag | Herkunft | Nutzung |
|---|---|---|
| `Operation`, `requires()→Requirement`, `materialize`, `Value::Host\|Device` | B3 | Ausgabe-Typ des Planners. |
| `CapabilitySet` (welche Engine welche Ressourcenart anbietet) | B6 | Device-Wahl-Grundlage; `precision`-Feld nur durchgereicht. |
| Parameterobjekte `EmbeddingParams`/`SimilarityParams`/`InferenceParams`, `TextRole`, `PairInput` | C1 | Eingabe-Task. |
| Pipeline-Config pro Modell (Präfixe, Pooling, Normalisierung, `max_seq_len`, dtype) | C3 (hier definiert) | Expansionsregeln. |

---

## 4. Entscheidungen zu den offenen Punkten

Mappt die „Zu klären"-Punkte des Metaplans (Konfigurationsformat der Expansionsregeln als Registry pro Modell/Tasktyp; Gewichtung des Starved-Hints) auf verankerte Entscheidungen; adressiert `resident_planner.md` „Offen".

### 4.1 Konfigurationsformat der Expansionsregeln [W, context §22]
- **TOML** via `serde` + **Validierung nach dem Deserialisieren** (Registry-Check: unbekannte Ops/Ressourcenarten/Modelle ⇒ `PlannerError`/`ConfigError`). RON nur, falls Enums-mit-Daten häufig werden.
- Regeln, die **Logik** brauchen, bleiben **code-basiert** (Expander-Impl), nicht in Config. Config liefert nur Daten (Präfixe, Pooling, `max_seq_len`, dtype, Scorer-Auswahl).
- **Hot-Reload:** Datei-Watcher (`notify`-Crate) + atomarer Austausch (`ArcSwap` bzw. `RwLock<Arc<…>>`). Der Planner bleibt rein, weil er **pro Aufruf einen Snapshot** (`Arc<RuleSet>`) erhält.

### 4.2 Registry-Pattern [B/W, context §23]
- **Explizite Registry** als Standard: `HashMap<(ModelId, TaskKind), Box<dyn Expander>>` bzw. Enum-Dispatch → deterministisch, leicht mockbar.
- `inventory`/`linkme` (Life-before-main) **nur**, falls Fremdcrates sich selbst registrieren sollen — vorerst nicht.
- Expander als **`fn`-Pointer/Trait-Objekte**, **nicht** `Fn`-Closures mit Zustand (Reinheit erhalten).

### 4.3 Impliziter Op-Graph [W, context §24, `resident_planner.md`]
- Ausgabe ist eine **geordnete `Vec<Operation>`** pro Task (Reihenfolge = Abhängigkeit). Operation trägt nur `requires()`.
- Verzweigung nur als **Folgeops beim Abschluss** erzeugen (keine automatische Parallelität zwischen unabhängigen Ops — bewusst, zugunsten Determinismus/Testbarkeit).

### 4.4 `materialize`-Regel [W, context §25, `resident_planner.md`]
- `materialize = false` (Device-Handle bleibt im VRAM), wenn ein **GPU-Nachfolger** folgt.
- `materialize = true` (Host-Kopie), wenn der **Nachfolger auf CPU** läuft, der **Task endet**, oder das Ergebnis in den **Index gespiegelt** werden muss.
- Entscheidung trifft der Planner anhand von Op-Metadaten; die Operation **führt** sie aus (nicht der Worker). D2H-Kopien sind stream-geordnet (Host-Daten erst nach Completion lesen — relevant für C2).

### 4.5 Aufgeschobene Device-Wahl [W, context §26, `resident_planner.md`]
- Op deklariert nur `ResourceKind` + optionale Affinität: `requires() -> Requirement { kind: ResourceKind, model, index, engine, affinity: Option<HandleId> }`, `#[non_exhaustive]`.
- Die **Subengine bindet beim Claim** ein konkretes Device. Vorerst keine Handle-Affinität: zwei Engines, jede Ressourcenart höchstens einmal pro Device, `requires()` bestimmt die Engine eindeutig.
- **Erweiterungspunkt:** `affinity`-Feld, falls dieselbe Art später auf beiden Devices existiert (Gruppe H / C2-Hook).

### 4.6 Op-Zähler & Abschluss-Vertrag [W, context §27, `resident_planner.md`]
- Planner **definiert** den Vertrag: Folgeoperationen werden **am Ende der Queue eingereiht, vor** dem Dekrement des Vorgängers. Die eigentliche atomare Zählung (`AtomicUsize pending`, `fetch_add` vor Enqueue, `fetch_sub==1 ⇒ Done`) führt C2 aus.
- Erster Fehler ⇒ `Cancelled` + `Err`, übrige Ops scheitern beim Claim.

### 4.7 Starved-Hint ohne Reinheitsverletzung [W, context §28, `resident_planner.md` Offen]
- Hint ist **Eingabe** der Expansion: `expand(task, &ExpandContext { starved, caps, rules })` — **kein** versteckter Zustand. Gleiche Eingabe + gleicher Hint ⇒ gleiche Ops.
- Der Hint darf **nur zwischen äquivalenten Varianten** wählen (Reihenfolge/Präferenz, z. B. CPU- vs. GPU-Encoder, wenn beide erlaubt), **nie** den Vertrag (Op-Menge/Ergebnis) ändern. → löst „Gewichtung des Starved-Hints" aus dem Metaplan.

---

## 5. Datentypen (Signatur-Skizze)

```rust
pub trait Expander: Send + Sync {
    // rein: kein &mut self, kein innerer Zustand
    fn expand(&self, task: &Task, ctx: &ExpandContext) -> Result<Vec<Operation>, PlannerError>;
}

pub struct Planner { rules: arc_swap::ArcSwap<RuleSet> }     // Snapshot pro Aufruf
impl Planner {
    pub fn new(rules: RuleSet) -> Self;
    pub fn expand(&self, task: &Task, starved: &StarvedSnapshot, caps: &CapabilitySet)
        -> Result<Vec<Operation>, PlannerError>;             // reine Funktion über Snapshot
    pub fn reload(&self, rules: RuleSet);                    // Hot-Reload: ArcSwap::store
}

pub struct RuleSet {
    expanders: std::collections::HashMap<(ModelId, TaskKind), Box<dyn Expander>>,
    pipelines: std::collections::HashMap<ModelId, PipelineConfig>, // aus TOML, validiert
}

pub struct ExpandContext<'a> {
    pub starved: &'a StarvedSnapshot,  // Präferenz, kein Vertrag
    pub caps: &'a CapabilitySet,       // B6
    pub pipelines: &'a std::collections::HashMap<ModelId, PipelineConfig>,
}

#[non_exhaustive]
pub struct PipelineConfig { /* prefixes(TextRole→String), pooling, normalize, max_seq_len, dtype, scorer */ }

pub enum TaskKind { Embedding, Similarity, Inference }
```

**Expansionsregeln (aus `resident_planner.md`, verbindlich):**
- Embedding: `Tokenize(CPU) → Embed(CPU|GPU) → Pool + L2 → [optional WriteIndex]`.
- Query-Similarity: `Tokenize → Embed → Similarity(Scope) → TopK`.
- Similarity mit CrossEncoder: Paare `(Query, Text aus Payload)` → `Infer(CrossEncoder) → Scores`, Scope aus früherem Ergebnis.
- Gleitende Indizierung (Ausblick H2): nur Parameter-Hook, keine Kernel-Logik.

---

## 6. Umsetzungsschritte

1. **Modul `planner/`** anlegen; Platzhalter für B3-`Operation`/`Requirement`/`Value` und B6-`CapabilitySet`, falls noch nicht vorhanden.
2. **`Expander`-Trait + explizite Registry** (4.2): Enum-Dispatch/`HashMap`, `fn`-Pointer/Trait-Objekte.
3. **Config** (4.1): `PipelineConfig` + `RuleSet`-Deserialisierung (TOML/serde) + Post-Deser-Validierung (Registry-Check); `ArcSwap`-Snapshot; optionaler `notify`-Watcher für Hot-Reload.
4. **Expander-Implementierungen** für Embedding/Similarity/Inferenz gemäß §5.
5. **`requires()`/Device-Wahl** (4.5): `Requirement` mit `ResourceKind` + `affinity: Option<HandleId>` (`#[non_exhaustive]`), Engine aus Capabilities eindeutig.
6. **`materialize`-Regel** (4.4) in jeden Expander einbauen.
7. **Starved-Hint** (4.7): `ExpandContext.starved` als Präferenz zwischen äquivalenten Varianten.
8. **Abschluss-Vertrag dokumentieren** (4.6) für C2 (Enqueue vor Dekrement).
9. **Tests** (§9).

**Deliverables:** `planner`-Modul ohne Laufzeitabhängigkeiten, vollständig ohne GPU testbar; Golden-Expansionen pro Modell/Tasktyp.

---

## 7. Fehlerbehandlung (Q1) [B, context §30]
`PlannerError` (ungültige Config, fehlende Capability, unbekanntes Modell, unbekannte Op/Ressourcenart). `#[non_exhaustive]`, nach oben via `#[from]` zusammengeführt. Keine Panics bei Fehl-Config — Fehler beim Deserialisieren/Validieren.

## 8. Logging (Q2) [B/W, context §31]
`tracing`-Span pro Task-ID (vom aufrufenden Worker gesetzt). Expansionsentscheidungen nur als `debug`/`trace`-Events (gewähltes Device, `materialize`, Regel-Treffer). Planner ruft **nie** `set_global_default`.

## 9. Teststrategie & Mocks (Q3) [B/W, context §29]
- **`proptest`** für Invarianten:
  - „Op-Menge ist unabhängig vom Starved-Hint" (nur Reihenfolge/Präferenz ändert sich).
  - „Reine Funktion": gleiche Eingabe ⇒ identische `Vec<Operation>`.
  - „Zähler endet bei 0" (Vertrag, gegen Referenz-Zustandsautomat via `proptest-state-machine`).
- **Golden-Tests** (`insta`) für Expansionen pro Modell/Tasktyp.
- **Mock-Capabilities** als `CapabilitySet`-Impl (z. B. nur CPU-Encoder → Embed muss CPU wählen; CPU+GPU → Hint entscheidet zwischen äquivalenten Varianten).
- **Config-Tests:** unbekannte Op/Ressourcenart ⇒ Validierungsfehler; Hot-Reload tauscht Snapshot atomar.

## 10. Feature-Flags (Q4) [D/W, context §32]
`planner` ist vollständig ohne `cuda` übersetz- und testbar (keine CUDA-Berührung). Device-Wahl basiert nur auf `CapabilitySet`-Daten.

## 11. Offene Risiken / zu verifizieren
- Exakte Form von B3-`Operation`/`Requirement` und B6-`CapabilitySet` kann Feldnamen/Signaturen verschieben → nach B3/B6 angleichen.
- Config-Schema (`PipelineConfig`) muss mit der konkreten Modell-Integration (Gruppe G1, e5-small: Präfixe `query:`/`passage:`, Mean-Pooling+L2) kompatibel bleiben → Felder dort gegenprüfen.
- Starved-Hint-Gewichtung bleibt „Präferenz, kein Vertrag" (`resident_planner.md` Offen) — Invariante durch Property-Test absichern.
