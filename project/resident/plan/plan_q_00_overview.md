# Umsetzungspläne Gruppe Q — Überblick

Stufe-2-Pläne zu `resident_planplan.md`. Je Paket ein eigenständiger, einzeln zuweisbarer Umsetzungsplan für einen Agenten. Zielprojekt: Rust-Crate unter `/home/user/xyan/xy.ai.workbench/rag`.

Recherchegrundlage für alle Pläne: `project/resident/context/context_h_q.md` (Stand ca. Mitte 2026, Zielhardware GTX 1660 / TU116 / sm75 / 6 GB VRAM). Mit **[prüfen]** markierte Punkte sind vor der Implementierung gegen die aktuelle Crate-/CUDA-Version zu verifizieren.

## Dateien

| Paket | Datei | Status | Charakter |
|---|---|---|---|
| Q1 Fehler-Taxonomie (`thiserror`) | `plan_q1_fehler_taxonomie.md` | Querschnitt | verbindliche Konvention |
| Q2 Logging (`tracing`) | `plan_q2_logging.md` | Querschnitt | verbindliche Konvention |
| Q3 Teststrategie & Mocks | `plan_q3_teststrategie.md` | Querschnitt | verbindliche Konvention |
| Q4 Feature-Flags | `plan_q4_feature_flags.md` | Querschnitt | verbindliche Konvention |

## Reihenfolge / Abhängigkeiten

- **Q zuerst fixieren** (nach A4): Q1→Q4 liefern die Konventionen, die alle anderen Gruppen konsumieren. Q4 (Feature-Flags) und Q3 (No-GPU-Pfad) sind Voraussetzung dafür, dass H-Erweiterungspunkte überhaupt GPU-frei testbar bleiben.