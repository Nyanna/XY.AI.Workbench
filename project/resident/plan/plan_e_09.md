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
