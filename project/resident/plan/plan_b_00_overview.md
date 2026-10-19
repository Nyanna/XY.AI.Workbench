# Umsetzungspläne Gruppe B — Kern-Datentypen & Verträge

**Stufe 2 (Ausführung).** Diese Datei ist die Gruppenübersicht zu den Paketplänen `resident_plan_b1.md` … `resident_plan_b6.md`. Sie bündelt Reihenfolge, Abhängigkeiten, Modulzuordnung, gemeinsame Konventionen (Querschnitt Q) und Typ-Eigentümerschaft. Grundlagen: `../resident_planplan.md` (Metaplan Gruppe B) und `../context/context_b.md` (Recherche B1–B6, Stand 10.10.2026).

**Zielprojekt:** Rust-Workspace `/home/user/xyan/xy.ai.workbench/rag`. **Zielhardware:** GTX 1660 (TU116, sm75, 6 GB).

---

## 1. Zweck der Gruppe

Gruppe B legt die **Kern-Datentypen und Verträge** fest, bevor Orchestrierung (Gruppe C), Subengines (D), Index-Management (E) und Präzision (F) darauf aufsetzen. Alle Pakete liefern **Spezifikation + Rust-Typen/Traits + Tests**, aber **keine** GPU-/CUDA-Laufzeitlogik (die liegt in D/E hinter Feature-Flag `cuda`).

Oberste Invariante für alle B-Pakete (aus `resident_subengine.md` §2):
> **Kein `Tensor` und kein Raw-Pointer in öffentlichen Signaturen** — nur `Region` + `Layout` (B2) bzw. `Value`/`Lease` (B3). **Kein `unsafe` außerhalb von `arc-swap`.** Candle-Typen nur in `pub(crate)`-Adaptern hinter `feature = "candle"`.

---

## 2. Reihenfolge & Abhängigkeiten

Empfohlene Sequenz (aus Metaplan): **B1 → B2 → B3 → B4 → B5 → B6**.

| Paket | Titel | Abhängt von | Blockiert |
|---|---|---|---|
| B1 | Queue & Task-Primitive | A4 | C1, C2, D1, E9 |
| B2 | Region / Layout / Speicher-Modell | A4 | B3, B5, D2, D3 |
| B3 | Operation-/Op-Modell | B1, B2 | C1, C2, C3 |
| B4 | Index-Trait & -Vertrag | A4 | C1, E1, E2, E5, E6, G2 |
| B5 | Kernel-Abstraktion | B2 | D6, E4, E5, E8 |
| B6 | CapabilitySet & Profile-Typen | A4 | C3, F1–F5 |

B4 ist **eigenständig vom Kern** (Index-Trait) und kann parallel zu B1–B3 bearbeitet werden. B1/B2/B4/B6 brauchen nur A4 (Querschnittskonventionen).

---

## 3. Modulzuordnung (vorläufig — endgültig entscheidet A2)

Alle B-Typen bilden die kern­nahe Vertragsschicht. Vorschlag (A2 besitzt die finale Crate-/Modulgrenze):

```
rag/src/resident
└─ core/           # reine Verträge, kein CUDA, kein candle-Zwang
   ├─ queue.rs        (B1)  Node, ClaimState, TaskState, Queue
   ├─ mem.rs          (B2)  Region, Layout, DType, MemoryManager-Trait
   ├─ op.rs           (B3)  Op-Trait, OpParams, Requirements, Value, Lease
   ├─ index.rs        (B4)  Index-Trait, Scope, AccessMethod, Stamp, EntrySet
   ├─ kernel.rs       (B5)  Kernel-Trait, SegmentDesc, SegmentLayout, KernelHit, Mock
   ├─ caps.rs         (B6)  AxisId, ValueId, CapabilitySet, Profile
   └─ error.rs        (Q1)  gemeinsame Fehler-Taxonomie (von A4/Q1 geliefert)
```

Candle-/safetensors-Adapter liegen in einem separaten `pub(crate)`-Modul (`adapter_candle.rs`) hinter `feature = "candle"`; CUDA-Pfade hinter `feature = "cuda"`.

---

## 4. Typ-Eigentümerschaft (wer definiert was)

| Typ | Definiert in | Benutzt von |
|---|---|---|
| `Node`, `ClaimState`, `TaskState`, `Queue`, `Notify`-Relay | B1 | C1/C2 (Task-Queue + 3 Op-Queues), D1, E9 |
| `Region`, `Layout`, `DType`, `MemoryManager` (Trait), `DeviceKind` | B2 | B3 (`Value::Device`), B5 (Spans), D2/D3/E3 |
| `Op` (Trait), `OpKind`, `OpParams`, `Requirements`, `ResourceKind`, `Value`, `Lease`, `Affinity` | B3 | C3 (Planner-Ausgabe), C2, D | 
| `Index` (Trait), `EntryId`, `Stamp`, `Scope`, `AccessMethod`, `EntrySet`, `EntryData`, `SummaryRef` | B4 | E1/E2/E5/E6, G2 |
| `Kernel` (Trait), `SegmentDesc`, `SegmentLayout`, `KernelHit`, `WrittenRange`, `MockKernel` | B5 | D6, E4/E5/E8 |
| `AxisId`, `ValueId`, `CapabilitySet`, `Profile` | B6 | C3 (`requires()`-Caps), F1–F5 |

**Namenskollision vermeiden:** Es gibt zwei „Layout“-Begriffe. B2 `Layout{shape,strides,dtype}` (Tensor-Layout). B5 Segment-Layout heißt deshalb **`SegmentLayout`** (`Fixed{stride}|Variable{offsets}`).

---

## 5. Gemeinsame Konventionen (Querschnitt Q, aus A4)

- **Q1 Fehler (`thiserror`):** gemeinsame Taxonomie in `error.rs`. Für B relevant: `ArenaExhausted` (B2), `IndexUnavailable`/`StaleMapping`(intern) (B4), `TransferFailed` (B5), `Incompatible`/`InvalidPrecision` (B6), Config-/Capability-Fehler (B3). Jedes Paket trägt seine Varianten bei; kein stilles Ausweichen.
- **Q2 Logging (`tracing`):** Span pro Task-ID (B1/B3), pro Manager-Zyklus (B4), pro Batch/Kernel (B5). Invariantenverletzungen auf `debug_assert!` + `warn!`.
- **Q3 Tests & Mocks:** No-GPU-Pfad verpflichtend. Mocks: `MockKernel` (B5), kein GPU in CI. Testmatrizen je Paket (insb. B1 Queue-Semantik, B4 Stamp/Racy, B6 Pareto/Matching-Vorarbeit).
- **Q4 Feature-Flags:** reale CUDA-/candle-Pfade hinter `cuda`/`candle`; die reine Logik (Zustandsautomat, Free-List, Stamp-Vergleich, CapabilitySet) ist ohne GPU testbar.

Hier -> `/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_00_overview.md`

---

## 6. Definition of Done (Gruppe)

1. `resident-core` kompiliert **ohne** `cuda`/`candle`-Feature (reine Logik).
2. Alle öffentlichen Signaturen frei von `Tensor`/Raw-Pointer; kein `unsafe` außer via `arc-swap`.
3. Jedes Paket B1–B6 erfüllt seine eigene DoD (siehe Paketdatei) inkl. Tests.
4. Offene Punkte sind als `// TODO(A3): …` markiert und im jeweiligen Plan-Abschnitt „Offene Punkte“ gelistet.
5. Die drei Namens-/Konsistenz-Hinweise des Metaplans sind beachtet (Layout-vs-SegmentLayout, Stamp nur `!=`, DType `#[non_exhaustive]`).

## Result

Build und Tests laufen jetzt in der Standardkonfiguration und mit `--features mock` fehlerfrei durch (86 Unit-Tests und 4 E2E-Tests). Ich habe vier Dinge korrigiert:

- **`Cargo.toml`, `tokenizers`:** Das Feature `fancy-regex` fehlte, das Crate ließ sich deshalb nicht kompilieren.
- **`Cargo.toml`, `rand`:** `rand` ist jetzt eine optionale Dependency, die das Feature `mock` aktiviert. `src/resident/mock.rs` braucht es auch außerhalb von Tests.
- **`tests/trigram_e2e.rs`:** Der Helper liest `Lines` jetzt als Objekt `{Zeilennummer: Text}`, so wie der Layer es ausgibt. Der Test `reindexes_on_mtime_change` nutzt für die zweite Abfrage eine neue Engine. Der Verzeichnis-Cache vertraut Listings bis zu 5 s ohne I/O, das ist gewollt, daher hat der Test vorher nie die Neuindizierung geprüft.
- **`src/resident/queue/node.rs`:** Das überflüssige `mut` ist entfernt.

Noch offen sind Compiler-Warnungen zu ungenutztem Code: `ManagerError::StaleMapping` (wird nirgends erzeugt) und in `dtype.rs` die Funktionen `elems_to_bytes` und `bytes_to_elems`. Die letzten beiden zeigt nur der Build mit `candle`. Ich habe sie nicht angefasst, weil sie wohl für spätere Pläne gedacht sind.

Das Feature `cuda` habe ich nicht gebaut.

Beim Prüfen, ob die Fehlschläge schon vorher bestanden, habe ich meine Änderungen kurz per `git stash` weggelegt. Der `stash pop` scheiterte, weil Cargo die `Cargo.lock` neu geschrieben hatte. Ich habe die Lock-Datei zurückgesetzt und den Stash wiederhergestellt. Nichts ist verloren gegangen.

Ich habe die DoD von A, B und Q gegen den Code geprüft. Die meisten Punkte sind erfüllt, aber mehrere Tests und Dokumente fehlen noch. Die Details stehen in `project/resident/dod_check_a_b_q.md`, auf Englisch.

**Nicht prüfbar:** Der Build mit `cuda` scheitert hier, weil kein `nvcc` installiert ist. Das braucht einen GPU-Runner.

**Gruppe A:**
- **Erfüllt:** Die Dokumente Charta, Feature-Matrix und Konventionen existieren. Der Build ohne CUDA und der Mock-Build laufen durch.
- **Teilweise:**
  - Die Charta wird nur aus dem C1-Plan referenziert, aus keinem E-Plan.
  - `cargo tree -d` zeigt Duplikate (`rand`, `hashbrown`, `getrandom` und weitere), die niemand geprüft hat.
  - In `dependency_verification.md` stehen keine Restrisiko-Einträge.
- **Offen:**
  - Es gibt keine Tests für den parquet-f16-Roundtrip und das Laden des Tokenizers.
  - `CUDA_COMPUTE_CAP` ist nirgends im Build gesetzt.

**Gruppe B:**
- **Erfüllt:** B2, B3, B5 und B6 sind vollständig. Es gibt kein `unsafe`.
- **Offen:**
  - B1 hat keine Unlink-Operation und keinen `loom::model`-Test. Mit `--cfg loom` schlagen 10 Resident-Tests fehl.
  - B4 hat keine Tests für die Stamp-Mechanik und den In-Memory-Index.
  - Die vorgesehenen `TODO(A3)`-Markierungen fehlen im Code.

**Gruppe Q:**
- **Erfüllt:** Q1 (Fehler-Enums) und Q2 (Logging-Felder) sind vorhanden. Die Tests laufen ohne GPU.
- **Offen:**
  - `error.rs` hat keine Tests.
  - `ManagerError::StaleMapping` wird nirgends erzeugt.
  - `resident/mock.rs` definiert einen zweiten `Kernel`-Trait, der `kernel::Kernel` doppelt.
  - Failpoints fehlen. `shuttle` und `loom::model` sind im Resident-Code nicht eingebunden, `proptest` nur im Trigram-Code.
  - Es gibt keine CI-Konfiguration und keinen Check gegen verstreute `cfg`-Attribute. Zusätzlich steht `cfg(feature = "cuda")` auch in `kernel/mod.rs` und `subengine/mod.rs`, nicht nur in der Backend-Factory.
