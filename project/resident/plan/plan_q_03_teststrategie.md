Setze Q3 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Scope-Charta erstellt unter `/home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md`
- Projektdokumente immer in englisch ausgeben

# Umsetzungsplan Q3 · Teststrategie & Mocks

**Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren, je Gruppe fortzuschreiben.

## 1. Zweck

Einheitliche Test-/Mock-Strategie, die (a) die gesamte Logik **ohne GPU** testbar macht, (b) Nebenläufigkeit/CAS-Protokolle abdeckt, (c) deterministische Sync-/Kalibrierungs-Tests ermöglicht, und (d) je Paket eine Testmatrix vorgibt.

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q3)

- **Mocks:** `mockall` (`#[automock]` auf Traits) für Erwartungs-/Aufruf-Verifikation; für **zustandsbehaftete** Kernel-/Mess-Mocks sind **manuelle Fakes** klarer (`FakeKernel` mit programmierbaren Ergebnissen, `FakeMeasure` mit Sequenz/RNG).
- **Nebenläufigkeit:** `loom` (erschöpfende Interleavings unter C11, `loom::sync` via `cfg(loom)`) für kleine CAS-Protokolle (Append/Claim/Cancel/Unlink); `shuttle` (randomisierte Schedules) für größere Szenarien; ergänzend `proptest` für Sequenzen.
- **GPU-frei:** Backend-Trait (`ComputeBackend`) mit CPU-/Mock-Impl; Feature `cuda` aktiviert nur die CUDA-Impl; Tests laufen **standardmäßig ohne** GPU.
- **Deterministisch:** virtuelle Zeit (`tokio::time::pause`, Feature `test-util`) oder eigener `Clock`-Trait; Schritt-Hooks/Failpoints zum Erzwingen von Interleavings (ABA: Stamp/Generation gezielt zwischen Lesen und CAS ändern); zustandsbasierte Asserts statt Timing; feste Seeds.
- **Verrauschte Messwerte:** Mess-Mock hinter Trait: Basiswert + Rauschen aus seeded RNG (`rand`, `StdRng::seed_from_u64`); Varianten konstant/Ausreißer/Drift. Tests: Abbruch an Obergrenze → `unstable`, Konvergenz bei geringem Rauschen, Verhalten bei Ausreißern; mehrere Seeds.

## 3. Verbindliche Konvention (Umsetzung)

**Abstraktions-Traits (alle mit Mock/Fake):**
- `ComputeBackend` — CPU/Mock vs. CUDA (hinter Feature `cuda`); die gesamte Orchestrierung läuft gegen das Trait.
- **Kernel-Trait** (B5, `resident_index.md`/`resident_subengine.md` §4.5): Batch-Start, Batch-Ende, Ergebnisse, geschriebene Bereiche, Fortschrittszähler → `FakeKernel` mit skriptbaren Ergebnissen.
- **Mess-Trait** (F2, `resident_precision.md`: „Kalibrierung über ein Trait für die Messung abstrahiert") → `FakeMeasure` (Basiswert + seeded Rauschen, Varianten konstant/Ausreißer/Drift).
- `Clock`-Trait bzw. `tokio::time::pause` für TTL/Idle/Zeitscheiben.

**Nebenläufigkeit:** CAS-Protokolle der Queue (B1) mit `loom` erschöpfend; größere Interleavings mit `shuttle`; Sequenzen/Properties mit `proptest`. Failpoints für gezielte ABA-Szenarien.

**Testmatrizen je Paket (verbindlich):**
- **B1 Queue:** `claim`=CAS `Queued→Taken`; Fertigstellung `Taken→Done`; `cancel` nur auf `Queued`; Unlink ohne Durchläufer-Restart; iterativer Drop (keine Stack-Überläufe); Notify-Topologie (loom).
- **E5/E6 Index-Sync (Kernkandidaten):** Reihenfolge im Kommunikationsslot (Kernel-Writes → Auflösen → Updates), inkl. Ergebnisse auf frisch geschriebene Adressen; Ergebnisstatus (entfernt→verworfen, Update anstehend→`outdated`); Echo (eigener `apply` löst keinen Reload aus, fremder Stamp schon, zwei Manager konvergieren); verlorenes/zusammengefasstes Signal; Signal-Quittierung **vor** dem Scan; Stamp-Lesereihenfolge beim Laden; ABA (Entfernen+Neuanlage → anderer Stamp); CoW-Swap (altes Segment erst frei, wenn kein vor dem Swap eingereihter Request es referenziert); Gather-vs-Segment-Dichteschwelle; Single-Flight (ein Ladevorgang, Fehlerverteilung an alle Wartenden); `UnsupportedScope`.
- **F2–F5 Kalibrierung:** Matching (Klemmen, NaN-Fehler, Tie-Break zum genaueren, Einzelprofil `p=0.5`); Profilbildung (Spread-Schwellwert, Zusammenfassen redundanter Punkte, Positionsberechnung über Bogenlänge); Pareto-Auswahl (dominierte Kombinationen verworfen); Persistenz (Fingerprint-Wechsel→`Stale`, fehlende Datei→`Missing`); Stabilitätslogik (verrauschte Mock-Messwerte, Abbruch an Obergrenze→`unstable`).
- **C2/C3 Processor/Planner:** Worker-Loop-Priorität, Starved-Hint-Verfall, Callback+`cuEventQuery` ohne Doppelverarbeitung (gegen Mock-Backend), Planner-Expansion rein (gleiche Eingabe → gleiche Ops).

**Keine Bit-Gleichheit:** Präzisions-/Scorer-Tests prüfen **Schranken/Relationen**, nicht Gleichheit (`resident_precision.md`).

## 4. Umsetzungsschritte

1. Trait-Definitionen + Fakes (`ComputeBackend`, Kernel, Mess, `Clock`) in einem `testkit`/`test-support`-Modul (dev-dependency-sichtbar).
2. `loom`/`shuttle`/`proptest` als dev-dependencies; `cfg(loom)`-Shim für `std::sync`→`loom::sync` in den betroffenen Modulen.
3. Failpoint-/Step-Hooks an den Sync-Punkten (Kommunikationsslot, Lade-Pfad).
4. Testmatrizen aus §3 als konkrete Test-Dateien je Paket.
5. CI: Default-Lauf ohne `cuda` (alle Unit-Tests), siehe Q4.

## 5. Offene Verifikationen ([prüfen])

- `loom`/`shuttle`/`proptest`/`mockall`/`rand`-Versionen in der Projekt-`Cargo.toml`.
- `tokio` `test-util`-Feature aktiviert.

## 6. Definition of Done

- Abstraktions-Traits + Fakes vorhanden; `loom`/`shuttle`/`proptest` eingebunden; Failpoints an Sync-Punkten; die Testmatrizen (B1, E5/E6, F2–F5, C2/C3) sind als Tests angelegt; alle Tests laufen **ohne** GPU im Default-Profil.
