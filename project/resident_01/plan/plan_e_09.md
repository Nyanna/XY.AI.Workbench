# Umsetzungsplan E9 · Addons (Iteration / Kaskade / Rekursion)

> **Quellen:** `resident_index.md` (Addons). **Recherche:** `context_e.md` §E9 (Punkte 34–36). **Querschnitt:** Q1, Q3.
> **Boundary:** see `rag/docs/architecture/scope_charta.md` (engine vs. search-layer scope, ring buffer vs. pull queue).
> **Abhängt von:** B1 (Queue/Task), C3 (Planner, erweiterbares Parameterobjekt). **Ergebnis:** Addon-Konzeptnotiz — **keine Kernänderung**.

## 1. Ziel & Abgrenzung
Iteration, Kaskaden und Rekursion sind **keine Kernfunktion**. Der Kern enthält **keine** Iteration/Kaskade/Rekursion/Eltern-Kind-Beziehung und keine Task-Zwischenzustände. Solche Muster entstehen **ausschließlich** durch ein **Addon**, das Tasks über die **normale Queue** einreiht (kopierte/geänderte Parameter). Ein Addon-Task ist **nicht unterscheidbar** von einem Anwendungs-Task.

**Auftrag dieses Pakets:** Nur den **Erweiterungspunkt sichern** — das **erweiterbare Parameterobjekt** — und nachweisen, dass Kaskaden ohne Kernänderung möglich sind. **Keine** neue Kernlogik.

## 2. Voraussetzungen
- B1: unveränderte Queue + `Task`-Primitive.
- C3: Planner expandiert Tasks; Parameterobjekt muss erweiterbar sein.
- E2: Scope eines Folgetasks kann aus Ergebnissen des Vortasks stammen (z. B. Zellen-Mitglieder).

## 3. Zu bauende/zu sichernde Artefakte
```rust
// Erweiterbares Parameterobjekt (der einzige Kernbedarf):
pub struct TaskParams {
    // feste Felder (Scope, precision, ...)
    ext: Extensions, // typsichere, offene Erweiterungen
}
// Erweiterungsmechanismus (Entscheidung unten):
pub struct Extensions(HashMap<TypeId, Box<dyn Any + Send + Sync>>);
```
- **Kein** Addon-Trait im Kern. Addons leben in Engine/Anwendung/Ressource und nutzen nur die Queue.

## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
**Typsichere Erweiterungen** (context_e.md §E9 Punkt 34, `[W]`): **`anymap`/`TypeMap`** (`HashMap<TypeId, Box<dyn Any + Send + Sync>>`) — flexibel, typsicher beim Zugriff, ohne Kernänderung erweiterbar. Für Serialisierung: Registry `name → (de)serialize` oder `serde_json::Value` als Parameter-Fallback. Extension-Traits (statisch) nur, wo zur Compile-Zeit bekannt. Builder nur bei festem Schema.

**Selbst-schedulende Tasks** (context_e.md §E9 Punkt 35, `[W]`): **Continuation-Passing** — ein Task gibt bei Abschluss Folgetasks an dieselbe Queue zurück (`Vec<Task>`); Work-Stealing (rayon/crossbeam-deque) für Lastverteilung. **Keine** Parent-Child-Struktur im Kern → Korrelation über **ID im Payload**. Pull-Filter (Priorität/Affinität) steuern, welche Engine den Folgetask zieht.

**Kaskaden-Beispiel (nur Illustration)** (context_e.md §E9 Punkt 36, `[V]` teilw.): Zentroid-Task gegen die Summary-Struktur → Folgetask mit den **Mitgliedern der gewählten Zellen als Scope**. Allgemeines Funnel-Muster (PLAID: Candidate Generation → Centroid Pruning → Centroid Interaction → Dekompression + exaktes MaxSim; BM25 → ColBERT-Rerank) ist so als Addon realisierbar — gehört aber in die **Oberschicht** (siehe H3), nicht in den Engine-Kern.

## 5. Umsetzungsschritte
1. **Verifizieren**, dass `TaskParams` ein erweiterbares `Extensions`-Feld besitzt (ggf. mit C3 abstimmen/ergänzen — dies ist die einzige erlaubte „Änderung", und sie liegt in B/C, nicht im E-Kern).
2. **Referenz-Addon** (nur als Test/Doku, nicht im Kern): Zentroid→Mitglieder-Kaskade, die Folgetasks mit kopiertem/geändertem `Scope` einreiht.
3. Nachweisen: Addon-erzeugte Tasks durchlaufen Queue/Planner/Subengine **identisch** zu App-Tasks; keine Sonderbehandlung.
4. Konzeptnotiz schreiben: Was ist Kern (Queue + Parameterobjekt), was ist Addon (alles Iterative/Kaskadierende), wo liegt die Grenze zur Oberschicht.

## 6. Fehler / Logging / Feature-Flags
- **Q1:** keine neuen Fehler.
- **Q3-Relevanz:** Korrelation über Payload-ID testbar.
- Kein Feature-Flag nötig (GPU-unabhängig).

## 7. Tests (Q3)
- Erweiterbares Parameterobjekt: fremdes Extension-Feld wird durch Queue/Planner transportiert, ohne Kernänderung.
- Referenz-Addon: Folgetask mit abgeleitetem Scope wird regulär eingereiht und ist von einem App-Task nicht unterscheidbar.
- Continuation-Passing: Task gibt `Vec<Task>` zurück; Korrelation über ID bleibt erhalten; keine Parent-Child-Struktur im Kern.

## 8. Offene Verifikationen
- `anymap`-Crate vs. eigenes `TypeMap`; Serialisierbarkeit nur falls Persistenz von Parametern gewünscht (`typetag`/`erased-serde`).
- Mit C3 abstimmen, dass das Parameterobjekt `#[non_exhaustive]`/offen bleibt.

## 9. Definition of Done
- Erweiterbares Parameterobjekt nachgewiesen; Referenz-Kaskade als Addon über die normale Queue demonstriert; Konzeptnotiz zur Kern/Addon/Oberschicht-Grenze vorhanden; **keine** Iteration/Kaskade/Rekursion im Kern.

## Result

# Zusammenfassung E9 (Addons)

## Erledigt
- **`Extensions` (neu, `facade/extensions.rs`):** typsichere Map `TypeId → Arc<dyn Any + Send + Sync>` mit `insert`, `get`, `remove`, `len` und `is_empty`. Sie ist `Clone`, `Debug`, `Send + Sync`.
- **Parameterobjekte:** `EmbeddingParams`, `SimilarityParams` und `InferenceParams` haben ein `ext`-Feld sowie `with_ext<T>()` und `ext()`. Sie waren schon `#[non_exhaustive]`. Queue und Planner reichen sie unverändert durch. `Extensions` ist über `facade` exportiert.
- **Kern unverändert:** keine Iteration, Kaskade, Rekursion oder Parent-Child-Struktur im Kern.
- **Konzeptnotiz:** `rag/docs/architecture/addons.md` beschreibt die Grenze zwischen Kern, Addon und Oberschicht.
- **Tests (neu):**
  - vier Unit-Tests für `Extensions`
  - Transport einer fremden Extension durch die Queue
  - Referenz-Kaskade als reiner Test (`facade/test/addon.rs`): Der Folgetask hat einen abgeleiteten `Scope::Set` und trägt die Korrelations-ID weiter.
  - Addon-Task und App-Task sind in `TaskKind` und Payload-Typ gleich.
  - Mit und ohne Extension durchläuft derselbe Task denselben Planner-Ablauf.
- **`api_hygiene` repariert:** `Doorbell::from_mapped`, `RootPointer::from_mapped` und `Seqlock::from_raw` sind jetzt `pub(crate)` statt `pub`. Der Fehler bestand schon vor E9 und gehört zu E4. Die Allowlist des Tests habe ich nicht verändert.

## Verifikation
- Alle Test-Binaries laufen grün: 419 Lib-Tests, `api_hygiene` (2), `deps_smoke` (2) und `trigram_e2e` (4).
- Mit dem Feature `cuda` und `nvcc` aus `/usr/local/cuda-13.3/bin` im PATH: `cargo check --features cuda --all-targets` baut. `cargo test --features cuda --lib` ergibt 431 Tests grün und 22 ignoriert.
- `nvcc` 13.3 ist vorhanden und über `.cargo/config.toml` konfiguriert, aber nicht im Shell-PATH.
- Clippy meldet in den geänderten Dateien keine Warnungen. Im übrigen Projekt gibt es bestehende Warnungen, die ich nicht angefasst habe.

## Abweichungen und Hinweise
- **Eigenes `Extensions` statt `anymap`-Crate:** Das vermeidet eine neue Abhängigkeit, und die Wahl ist begründet.
- **Serialisierung nicht umgesetzt:** Sie war im Plan nur bedingt gefordert. Als Option steht sie in der Konzeptnotiz.
- **Kein einheitliches `TaskParams`:** Der Plan ging davon aus, im Code gibt es drei Param-Structs. Das `ext`-Feld sitzt deshalb in allen dreien.
- **Rückgabetyp des Addons:** Das Referenz-Addon reicht `Vec<TaskHandle>` statt `Vec<Task>` zurück, weil es die Folgetasks selbst einreiht. Einen eigenen Test dafür, dass der Kern keine Parent-Child-Struktur hat, gibt es nicht.
- **Keine Abstimmung mit der Planner-Gruppe:** Die Params-Structs waren bereits `#[non_exhaustive]`. Den Zustand habe ich im Code geprüft.
- **22 ignorierte CUDA-Tests nicht untersucht:** Vermutlich brauchen sie echte GPU-Hardware. Einen Hardware-Smoke-Test auf der GTX 1660 habe ich nicht gesehen.
