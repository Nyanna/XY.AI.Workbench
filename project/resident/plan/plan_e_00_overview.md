# Umsetzungspläne Gruppe E — Index-Management & Synchronisation (Übersicht)

> **Zielhardware:** sm_75 (Turing, GTX 1660 / TU116, 22 SMs, 6 GB VRAM).
> **Projekt:** Rust-Crate unter `/home/user/xyan/xy.ai.workbench/rag`.
> **Primärquelle:** `resident_index.md`. **Recherche je Paket:** `context/context_e.md`. **Querschnitt:** `context/context_h_q.md` (Q1–Q4).
> **Hinweis:** `resident_index.md` ist die Spezifikation des **Vektorindex/Index-Managers**, keine Inhaltsübersicht. `resident_index.md` bezeichnet die reale Datei `resident_precision.md` fälschlich als `resident_precision_calibration.md`.

Diese Übersicht bündelt die gruppenweiten Entscheidungen und verweist auf die neun Einzelpläne `plan_e1.md … plan_e9.md`. Jeder Einzelplan ist isoliert abarbeitbar (eigene Quellen, Voraussetzungen, Artefakte, Tests, Definition of Done).

---

## 0. Scope-Grenze (verbindlich)

**In der Engine (Gruppe E baut dies):** `Index`-Trait + `StreamedIndexManager` (RAM ⇄ Index ⇄ VRAM-Spiegel), `ArenaAllocator` (GPU), `ScopeResolver` pro `AccessMethod`, Update-Queue, Residency-Tracker, Kommunikationsslot, zustandsbasierte Synchronisation/Echo-Vermeidung, Indexaufbau-Pfad (Kernel schreibt Segmente), Erweiterungspunkt für Addons.

**Nicht in der Engine (außerhalb / Oberschicht):** Persistenzformat & -zeitpunkte des Index (gehören dem Index-Objekt), Chunking, Query-Formulierung, Reranking, Kandidaten-`k`, Indexstruktur selbst, Qualität/Recall, Drift-Monitor, approximative Vorfilter (Zentroid/FFT/Kompression verlieren bewusst Randtreffer — Eigenschaft der Oberschicht). Iteration/Kaskade/Rekursion sind **keine** Kernfunktion (nur Addon, E9).

**Akzeptierte Invarianten:** Veraltete Ergebnisse sind Variablen, kein Fehler. Keine Sperren zugunsten Konsistenz. Stamp-Vergleich ausschließlich mit `!=`, nie mit `<`.

---

## 1. Konsistenzmodell Index-Sync (gruppenweite Festlegung)

Das in `resident_subengine.md` §4.4 und im Metaplan (D7/E) als „noch festzulegen" markierte Konsistenzmodell wird hier verbindlich fixiert:

1. **Source of Truth = Index (RAM-Primärinstanz).** VRAM ist ein abgeleiteter, verwerfbarer Spiegel. Richtung agnostisch RAM ⇄ Index ⇄ VRAM, aber bei Divergenz gewinnt der Index.
2. **Identität = (EntryId, Stamp).** Stamp ist der Wert eines globalen, monotonen `AtomicU64` zum Write-Zeitpunkt (siehe E6). Vergleich nur mit `!=`. 64 Bit laufen praktisch nie über → kein ABA.
3. **Zustandsbasierter Abgleich, nicht ereignisbasiert.** Das `watch<u64>`-Level-Signal weckt nur; verglichen werden Stamps **nur residenter Entries**. Verlorene/zusammengefasste Signale sind harmlos.
4. **Segment-/Tabellen-Mutation per Copy-on-Write + Epoche (RCU-artig).** Große Strukturen werden nie in place geändert; neu anlegen → atomarer 8-Byte-Root-Pointer-Swap → alte Version erst freigeben, wenn kein Reader mehr darauf zeigt. Epochenzähler: Kernel meldet pro Batch die gelesene Epoche; Host gibt frei, sobald alle Reader ≥ neue Epoche (siehe E4).
5. **Kein Epochen-/Sequenzfeld im Kernel-Ergebnisprotokoll.** Mutation nur in Lücken (Kommunikationsslot) ⇒ jedes Ergebnis sieht den Mapping-Stand seines Batches (siehe E5).
6. **Schreibreihenfolge im Index:** erst Daten → dann Stamp → dann Signal. **Lesereihenfolge beim Laden:** Stamp → Daten → (bei mmap) Stamp erneut prüfen.
7. **Last-Writer-Wins** über mehrere Schreiber; `apply` wird serialisiert. Zwei Engines konvergieren über den Stamp-Vergleich auf den Indexstand.

---

## 2. Konsolidierung D3 ↔ E3 (Arena)

D3 (Subengine-GPU-Speicher) und E3 (ArenaAllocator) beschreiben dieselbe Allokationsschicht. **Festlegung:** Es gibt **einen** `ArenaAllocator`/`MemoryManager` (aus Gruppe B/D), E3 ist seine **index-/segmentspezifische Nutzung** (Index-Segment-Arena, CoW-Doppelbedarf, Copy-Stream mit Pinned Staging). `plan_e3.md` spezifiziert ausschließlich die Index-Arena-Nutzung und verweist auf den gemeinsamen Allocator; keine zweite Allokator-Implementierung.

---

## 3. Voraussetzungen aus anderen Gruppen

| Abhängigkeit | Liefert | Genutzt in |
|---|---|---|
| **B4** Index-Trait & -Vertrag | `Index`, `EntryId`, `Stamp`, `EntrySet`, `Scope`, `AccessMethod`, `SummaryRef`, `watch<u64>` | E1, E2, E5, E6, G2 |
| **B5** Kernel-Abstraktion | `SegmentDesc`, `Layout{Fixed/Variable}`, `KernelHit`, Kernel-Trait + Mock | E4, E5, E8 |
| **B2 / D2 / D3** Speicher/Device/Arena | `Region{arena,offset,len,gen}`, `MemoryManager`/`ArenaAllocator`, Copy-Stream, Pinned Staging, `cuMemGetInfo`-Budget | E3, E4, E7 |
| **D7** Index-Segmente in der Subengine | `SegmentView`/Slot/Slot-Klassen, `(segment,slot)→EntryRef` | E1, E5, E8 |
| **C3** Planner | Op-Expansion, erweiterbares Parameterobjekt | E9 |
| **Q1–Q4** Querschnitt | Fehler-Taxonomie, `tracing`-Spans, Mocks/No-GPU, `cuda`-Feature-Flag | alle E |

Fehlt eine Voraussetzung noch, baut der Agent gegen die in B4/B5 vereinbarten Trait-Signaturen und einen Mock.

---

## 4. Empfohlene Modulstruktur (Rust)

```
index/            # B4: Index-Trait, EntryId, Stamp, EntrySet, Scope, AccessMethod (fremd)
manager/
  mod.rs         # StreamedIndexManager (E1)
  scope.rs       # ScopeResolver-Registry + Resolver (E2)
  arena.rs       # Index-Segment-Arena-Nutzung (E3, nutzt gemeinsamen Allocator)
  swap.rs        # Manager-Disziplin: CoW, Pointer-Swap, Acquire-Load-Bindung (E4)
  slot.rs        # Kommunikationsslot, feste Reihenfolge (E5)
  sync.rs        # Zustandsabgleich, Echo-Vermeidung, Residency-Tracker (E6)
  residency.rs   # Lade-/Eviction-Strategie, Single-Flight, Gather/Segment (E7)
  build.rs       # Kernel schreibt Segmente, Rückabbildung Adresse→Entry (E8)
addons/          # Erweiterungspunkt (E9) — nur Konzept, kein Kern
```

Reale CUDA-Pfade (Acquire-Load, Pinned Staging, Arena, Copy-Stream) liegen hinter `#[cfg(feature = "cuda")]`; die gesamte Logik läuft gegen Traits (`ComputeBackend`, Kernel-Trait) und ist ohne GPU testbar (Q4).

---

## 5. Reihenfolge & Abhängigkeitsgraph

```
E1 (Grundgerüst)
 ├─ E2 (Scope & Resolver)            [braucht E1, B4]
 ├─ E3 (Index-Arena)                 [braucht D3]
 │   └─ E4 (Manager-Disziplin)       [braucht E1, E3, B5]
 │       └─ E5 (Kommunikationsslot)  [braucht E4, B4, B5]
 │           ├─ E6 (Sync/Echo)       [braucht B4, E5]
 │           ├─ E7 (Residency)       [braucht E3, E5]
 │           └─ E8 (Indexaufbau)     [braucht E5, B5]
E9 (Addons)                          [braucht B1, C3 — unabhängig]
```

Sequenz für den Lauf: **E1 → E2 → E3 → E4 → E5 → E6 → E7 → E8 → E9**.

**Hochrisiko (zuerst prototypisch absichern):** E4 (Acquire-Load/Fence-Sichtbarkeit Host↔GPU) und E6 (Konsistenzmodell). Siehe die „Offene Verifikationen" der jeweiligen Pläne.

---

## 6. Gemeinsame Testinfrastruktur (Q3)

- **`FakeKernel`** (manueller Fake, nicht `mockall`): programmierbare Ergebnisse, geschriebene Bereiche, Fortschrittszähler; implementiert das Kernel-Trait (B5). Treibt E4/E5/E8-Tests ohne GPU.
- **`FakeIndex`**: `Index`-Trait mit steuerbaren Stamps/`apply`-Rückgaben, programmierbarem `changed()`-Signal; treibt E1/E2/E6.
- **Nebenläufigkeit:** `loom`/`shuttle` für die CoW-/Epoch-/Slot-Protokolle (E4/E5), `proptest` für Scan-Sequenzen (E6). Virtuelle Zeit (`tokio::time::pause`) für Idle/Eviction (E7). Failpoints zum Erzwingen von Interleavings (Stamp zwischen Stamp- und Datenlesen ändern, E6).
- Alle Tests laufen **ohne** `cuda`-Feature (No-GPU-Pfad). CUDA-Pfade nur per `cargo check` + Hardware-Smoke-Test.

Hier -> `/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_00_overview.md`

---

## 7. Gruppenweite Definition of Done

- Alle in E1–E8 genannten Tests aus `resident_index.md` („Tests"-Liste) grün im No-GPU-Pfad.
- Konsistenzmodell (Abschnitt 1) in Code-Doku und Tests nachweisbar verankert.
- Fehler-Taxonomie (`UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` intern, `TransferFailed`) vollständig; kein stilles Ausweichen auf andere Verfahren/Scopes.
- `tracing`-Spans pro Task und Manager-Zyklus; geloggt: Scan-Ergebnis (geladen/veraltet/entfernt), Lade-/Eviction-Entscheidung, verworfene & `outdated` Ergebnisse, Arena-Auslastung.
- Offene Verifikationen (sm_75-Atomics, Primary-Context-Größe, madvise-Semantik, Gather/Segment-Schwelle) dokumentiert mit Status (verifiziert/offen).
