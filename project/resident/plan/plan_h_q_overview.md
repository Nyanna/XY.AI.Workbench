# Umsetzungspläne Gruppe H & Q — Überblick

Stufe-2-Pläne zu `resident_planplan.md`. Je Paket ein eigenständiger, einzeln zuweisbarer Umsetzungsplan für einen Agenten. Zielprojekt: Rust-Crate unter `/home/user/xyan/xy.ai.workbench/rag`.

Recherchegrundlage für alle Pläne: `project/resident/context/context_h_q.md` (Stand ca. Mitte 2026, Zielhardware GTX 1660 / TU116 / sm75 / 6 GB VRAM). Mit **[prüfen]** markierte Punkte sind vor der Implementierung gegen die aktuelle Crate-/CUDA-Version zu verifizieren.

## Grundregel Gruppe H (Ausblick / zurückgestellt)

Für **alle** H-Pakete gilt: In Stufe 2 werden **nur Erweiterungspunkte gesichert** —
1. erweiterbares **Parameterobjekt** (Task-Params, `#[non_exhaustive]` / Builder),
2. **`requires()`-Affinität** als Erweiterungsfeld,
3. **Op-Austausch hinter demselben Trait** (gleiche `Op`/`Launcher`/`Scorer`-Schnittstelle).

**Keine Detailplanung, kein Kern-Workstream.** Jeder H-Plan liefert: (a) was später kommt, (b) die destillierte Recherche, (c) eine Checkliste „darf jetzt nicht verbaut werden", (d) explizit „jetzt NICHT umsetzen".

Status-Legende (aus dem Thesen-Abgleich im Metaplan): **K** Kern · **I** Hook vorbereitet · **A** Ausblick/später in der Engine · **E** bewusst außerhalb der Engine · **—** nicht adressiert.

## Dateien

| Paket | Datei | Status | Charakter |
|---|---|---|---|
| H1 GPU-Ringbuffer-Modus | `plan_h1_gpu_ringbuffer.md` | A | Erweiterungspunkte sichern |
| H2 Gleitende Indizierung | `plan_h2_gleitende_indizierung.md` | A/I | Erweiterungspunkte sichern |
| H3 Funnel/PLAID & Reranker | `plan_h3_funnel_plaid_reranker.md` | I/A | Erweiterungspunkte sichern |
| H4 Graph-Capture & Persistent-Scorer | `plan_h4_graph_capture.md` | A | Erweiterungspunkte sichern |
| H5 Signalkompression | `plan_h5_signalkompression.md` | E/— | nur Nicht-Ausschluss + Divergenz |
| H6 Mutable Weights / Online-Bandit | `plan_h6_mutable_weights_bandit.md` | — | nur Nicht-Ausschluss + Divergenz |
| Q1 Fehler-Taxonomie (`thiserror`) | `plan_q1_fehler_taxonomie.md` | Querschnitt | verbindliche Konvention |
| Q2 Logging (`tracing`) | `plan_q2_logging.md` | Querschnitt | verbindliche Konvention |
| Q3 Teststrategie & Mocks | `plan_q3_teststrategie.md` | Querschnitt | verbindliche Konvention |
| Q4 Feature-Flags | `plan_q4_feature_flags.md` | Querschnitt | verbindliche Konvention |

## Reihenfolge / Abhängigkeiten

- **Q zuerst fixieren** (nach A4): Q1→Q4 liefern die Konventionen, die alle anderen Gruppen konsumieren. Q4 (Feature-Flags) und Q3 (No-GPU-Pfad) sind Voraussetzung dafür, dass H-Erweiterungspunkte überhaupt GPU-frei testbar bleiben.
- **H am Ende** (nur umreißen): H1/H4 hängen an E4 (Manager-Disziplin, Acquire-Load-Pointer) und §4.6 (Launcher-Decorator); H2 an C3 (Planner-Regel-Registry) und B4/§4.4 (Entry-/Slot-Modell); H3 an E2 (ScopeResolver/Centroid) und C1 (Scorer-/Inferenz-Tasktyp); H5/H6 nur an die Offenheit der Enums (`AccessMethod`, `Scope`, Task-Params).

> Hinweis: `resident_index.md` ist die Spezifikation des **Vektorindex** (Index-Trait, StreamedIndexManager, Kernel-Schnittstelle), **keine** Inhalts-/Indexdatei.
