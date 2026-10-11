# Umsetzungspläne — Gruppe A (Fundament & Kontext)

> **Stufe 2 (Ausführungsplanung).** Erzeugt aus `resident_planplan.md` (Gruppe A) und dem
> verifizierten Rechercheergebnis `context/context_a.md` (Verifizierungsdatum 2026-10-10).
> Diese Datei ist die Einstiegs-/Übersichtsdatei für die vier Paketpläne der Gruppe A.

## Zweck der Gruppe A

Gruppe A legt das Fundament, bevor Datentypen (B), Orchestrierung (C) usw. geplant/umgesetzt
werden: Umfangsabgrenzung, Projekt-/Modulstruktur, verifizierter Abhängigkeits-Stack und die
querschnittlichen Konventionen (Fehler/Logging/Test/Feature-Flags). Alle nachfolgenden Gruppen
bauen hierauf auf.

## Zielumgebung (gilt für alle A-Pläne)

- **Rust-Projekt (Code-Ausgabe):** `/home/user/xyan/xy.ai.workbench/rag`
- **Zielhardware:** GTX 1660, `sm_75` (Turing), 6 GB VRAM.
- **Modell:** `intfloat/multilingual-e5-small` (dense Bi-Encoder, BERT-Architektur, dim 384).
- **CUDA-Linie:** CUDA **12.x** (nicht 13.x), Build mit `CUDA_COMPUTE_CAP=75`.
- **Konsequenz sm_75:** bf16-Kernel in candle entfallen (`-DNO_BF16_KERNEL`) → nur **f16/f32** planen.

## Paketpläne & Reihenfolge

Topologische Sequenz laut Metaplan: **A1 → A2 → A3 → A4**.

| Plan | Paket | Ergebnis (Artefakt) | Abhängt von |
|------|-------|---------------------|-------------|
| [`plan_a1.md`](./plan_a1.md) | **A1′** Scope-Grenze Engine ↔ Suchschicht | Scope-Charta (Dokument) | — |
| [`plan_a2.md`](./plan_a2.md) | **A2** Projekt- & Modulstruktur | Workspace-/Crate-Schnitt + Feature-Matrix (Code-Gerüst) | A1′ |
| [`plan_a3.md`](./plan_a3.md) | **A3** Stack- & Versionskompatibilität | Verifizierte Abhängigkeitsliste + Risikoliste (gepinnt) | A2 |
| [`plan_a4.md`](./plan_a4.md) | **A4** Querschnittskonventionen | Konventionsdokument + Basis-Scaffolding (speist Q1–Q4) | A2 |

Hinweis zu A1: Das ursprüngliche Paket A1 ("These destillieren") ist laut `resident_planplan.md`
**erledigt** (die These `resident_streaming.md` wurde eingelesen, der Abgleich steht im Metaplan).
A1 wird deshalb durch **A1′** (Scope-Charta) ersetzt — siehe `plan_a1.md`.

## Querschnitt Q (begleitend, in Gruppe A initial zu fixieren)

- **Q1 Fehler-Taxonomie** (`thiserror`) — Grundlage in A4, fortgeschrieben je Gruppe.
- **Q2 Logging** (`tracing`) — Span-/Event-Konventionen in A4.
- **Q3 Teststrategie & Mocks** — Konventionen in A4, No-GPU-Pfad in A2.
- **Q4 Feature-Flags** — reale CUDA-Pfade hinter `cuda`-Flag, Logik GPU-frei testbar (A2).

## Definition of Done für Gruppe A (gesamt)

1. Scope-Charta liegt vor und ist referenzierbar aus C1/E2 (A1′).
2. Workspace baut mit **und** ohne `cuda`-Feature; `cargo build --no-default-features` benötigt
   kein CUDA-Toolkit (A2 + Q4).
3. `Cargo.toml`/`Cargo.lock` enthalten den verifizierten, gepinnten Stack; offene Punkte aus
   `context_a.md` sind geklärt oder als Restrisiko dokumentiert (A3).
4. Konventionsdokument (Fehler/Logging/Test/Feature) existiert; das Basis-Scaffolding
   (Fehler-Modul, `tracing`-Init, Mock-Konvention) kompiliert (A4, Q1–Q4).

## Verwendete Legende (aus context_a.md)

**[V]** live verifiziert · **[W]** Fachwissen, nicht live verifiziert · **[?]** offen.
Alle in den Plänen zitierten Versionen/Fakten tragen diese Herkunft.
