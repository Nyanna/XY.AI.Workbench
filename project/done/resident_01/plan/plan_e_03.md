# Umsetzungsplan E3 · ArenaAllocator (GPU, Index-Segmente)

> **Quellen:** `resident_index.md` (Manager-Disziplin/Arena); **überlappt D3** (`resident_subengine.md` §4.2). **Recherche:** `context_e.md` §E3 (Punkte 8–12). **Querschnitt:** Q1, Q2, Q4.
> **Boundary:** see `rag/docs/architecture/scope_charta.md` (engine vs. search-layer scope, ring buffer vs. pull queue).
> **Abhängt von:** D3 (gemeinsamer Allocator/MemoryManager). **Ergebnis:** Index-Arena-Spezifikation, konsolidiert mit D3.

## 1. Ziel & Abgrenzung
Die **index-/segmentspezifische Nutzung** der gemeinsamen GPU-Arena: lazy Anlage nach VRAM-Budget, Free-List, `u64`-Arena-Offsets, eingeplanter Copy-on-Write-Doppelbedarf, Copy-Stream mit Pinned Staging. **Konsolidierung D3↔E3:** Es gibt genau **einen** `ArenaAllocator`/`MemoryManager` (aus D3). Dieser Plan definiert dessen Nutzung für Index-Segmente; **keine** zweite Allokator-Implementierung.

Nicht hier: Pointer-Swap-Disziplin & Acquire-Load (E4), Lade-/Eviction-Strategie (E7).

## 2. Voraussetzungen
- D3: `ArenaAllocator` mit `alloc/free/resolve`, `Region{arena,offset,len,gen}`, Budget via `cuMemGetInfo`, stream-geordnete Wiederverwendung, Copy-Stream + Pinned Staging.
- B2: `Region`-Vertrag (kein Pointer/Tensor in Signaturen).

## 3. Zu bauende Artefakte
```rust
pub struct IndexArena<'a> { inner: &'a ArenaAllocator, base: DeviceBase /* u64-Basis */ }
impl IndexArena<'_> {
    fn alloc_segment(&self, bytes: usize, align: usize) -> Result<Region, ManagerError>; // 256B-align
    fn reserve_writable(&self, cap: usize) -> Result<WritableRegion, ManagerError>;      // für E8-Kernel-Writes
    fn free(&self, r: Region);                                                           // stream-geordnet
    fn budget(&self) -> ArenaBudget;                                                      // live, reserve, cow_headroom
}
pub struct CowBudget { live: u64, max_concurrent_cow: u32, largest_segment: u64 }
```

## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
**VRAM-Budget** (context_e.md §E3 Punkt 9, `[W]`): `cuMemGetInfo` **nach** Context-Erzeugung und **nach** Laden von cuBLAS/cuDNN/Kernel-Modulen aufrufen; Budget = `free − Sicherheitsreserve`. Primary Context + Libraries auf Turing grob **300–600 MB** — auf der Zielkarte (GTX 1660) **messen**. Display-GPU belegt zusätzlich.

**Arena statt Runtime-Alloc** (context_e.md §E3 Punkt 8, `[V]`): `cudaMalloc`/`cudaFree` synchronisieren über laufende Streams → im Betrieb vermeiden. **Einmal groß** `cudaMalloc` beim ersten Laden, danach nur Suballokation (keine Runtime-Syncs). `cudaMallocAsync`-Pool (ab CUDA 11.2) ist pool-basiert und syncfrei, für eine **feste** Arena aber nicht nötig; Turing-Nutzbarkeit nur über `cudaDevAttrMemoryPoolsSupported` zur Laufzeit prüfen.

**Suballocator** (context_e.md §E3 Punkt 10, `[W]`): **TLSF** (O(1), geringe Fragmentierung) für variable Segmentgrößen; **Slab** nur falls Segmente feste Größe haben (Slot-Klassen aus D7). Buddy verworfen (bis 50 % interne Fragmentierung). **Alignment:** `cudaMalloc` liefert ≥ 256 B; Suballokationen auf **256 B** alignen (Coalescing/Vektor-Loads ≥ 16 B). Adressierung als **`u64`-Offset relativ zur Arena-Basis** (Pointer = Basis + Offset); 32-Bit nur bei < 4 GiB.

**CoW-Doppelbedarf** (context_e.md §E3 Punkt 11, `[W]`): Heuristik **Budget ≥ Live-Daten + (max. gleichzeitige CoW-Kopien × größtes Segment)**. Maßnahmen: Segmente **klein** halten (begrenzt Spitzenbedarf), CoW **seriell** statt parallel, alte Kopie erst nach **Epochenende** freigeben (Kopplung an E4). Kein zitierfähiges Standardverfahren — als Heuristik dokumentieren.

**Pinned Staging / Copy-Stream** (context_e.md §E3 Punkt 12, `[W]`): `cudaHostAlloc` ist teuer (Page-Locking, ms) → **einmalig** Pool anlegen, wiederverwenden. Pinned ermöglicht echt asynchrone H2D/D2H und volle PCIe-Bandbreite. **Separater Copy-Stream** (nicht-blockierend priorisiert) für Überlappung Copy ↔ Compute (Turing hat Copy-Engines). `cudaHostAllocWriteCombined` **nicht** setzen (macht Host-Reads extrem langsam). Pinned-Gesamtmenge begrenzen (OS-Limit/RAM-Druck).

## 5. Umsetzungsschritte
1. `IndexArena` als dünne Hülle um `ArenaAllocator` mit 256-B-Alignment und `u64`-Offset-Adressierung.
2. Budgetermittlung: `cuMemGetInfo` nach Lib-Load, Reserve-Parameter, `CowBudget`-Berechnung; `ArenaExhausted` bei Unterschreitung.
3. Lazy-Anlage beim ersten Segment-Load; Free-List; stream-geordnete Freigabe (Event-gebunden, Kopplung D3).
4. `reserve_writable` für E8 (Cursor-Writes); Pinned-Staging-Pool + Copy-Stream als gemeinsame Ressource referenzieren.
5. CoW-Kopie seriell, Freigabe der alten Kopie an Epochenende (Callback aus E4).

## 6. Fehler / Logging / Feature-Flags
- **Q1:** `ArenaExhausted` (Budget inkl. CoW-Headroom unterschritten), `TransferFailed` (Copy-Stream).
- **Q2:** Events: Arena-Anlage (Größe, Budget, Reserve), Suballokation/Free, Arena-Auslastung pro Zyklus, CoW-Spitze.
- **Q4:** Gesamte CUDA-Mechanik hinter `cuda`; ohne Feature ein **Host-Arena-Mock** (Vec-Backed, gleiche Offset-Semantik) für Logiktests.

## 7. Tests (Q3)
- Suballokation/Free-List: Offsets 256-B-aligned, keine Überlappung, Wiederverwendung nach Free.
- Budget: `ArenaExhausted`, wenn Live + CoW-Headroom das Budget überschreitet.
- CoW seriell: zweite CoW wartet, alte Kopie wird erst nach Epochenende frei (Fake-Epoch-Hook, Kopplung E4).
- Host-Arena-Mock liefert deterministische Offsets für E4/E5/E8-Tests.

## 8. Offene Verifikationen

Status as of 2026-10-19, measured on a GTX 1660 SUPER (sm_75, 6 GiB, CUDA driver reports 5753 MiB total) with `cargo test --lib --features cuda -- --ignored` (`context_footprint_and_pool_support`, `index_arena_copy_on_write_on_device` in `memory/cuda.rs`).

| Item | Status | Result |
|---|---|---|
| `cudaDevAttrMemoryPoolsSupported` | verified | `1`; the stream-ordered pool path is usable on Turing. |
| Primary-context size | verified | about 72 MiB (5125 MiB free before the process, 5053 MiB after context creation). The 300–600 MB estimate does not hold with lazy module loading. |
| Library footprint | partially verified | cuBLAS adds 44 MiB after a 256×256 matmul via candle. Not covered: full kernel module set and cuDNN. Re-measure before lowering the safety reserve. |
| Copy-on-write on device | verified | `IndexArena` on the CUDA index-segment arena: alloc, CoW, pinned upload/download roundtrip, commit, epoch-gated reclaim. |
| TLSF vs. slab | decided, no code change | Slot classes (`SlotClass::T32/T64/T128/Custom`) are fixed-stride slots inside one segment and are already tracked by a bitmap (`segment/slots.rs`), so slab behaviour exists below the arena. The arena only sees few, large, variable-size segments (`slots × stride`); the existing first-fit coalescing `FreeList` is sufficient. TLSF is not needed until a fragmentation test with real segment mixes fails. |

## 9. Definition of Done
- Eine Arena-Implementierung (konsolidiert mit D3), 256-B-Offset-Suballokation, budgetierte CoW-Heuristik, Pinned-Staging/Copy-Stream referenziert; Host-Mock-Pfad testbar ohne GPU.

## Result

3 ist umgesetzt: `rag/src/resident/manager/arena.rs` ist neu und in `manager/mod.rs` als `pub(crate) mod arena` registriert. Die 8 neuen Tests und die übrigen 294 Resident-Tests laufen im No-GPU-Pfad durch, und `cargo check --features cuda` baut. Auf der GTX 1660 habe ich nichts getestet.

- **Kein zweiter Allocator:** `IndexArena` ist eine dünne Hülle um den vorhandenen `ArenaAllocator`. Segmente werden mit mindestens 256 B ausgerichtet. Ohne `cuda` dient ein `ArenaAllocator` mit `DeviceKind::Cpu` als Host-Mock mit gleicher Offset-Semantik.
- **Fehler:** `ManagerError` hat dafür zwei neue Varianten, `ArenaExhausted` und `Memory`, das eine `MemoryError` durchreicht (etwa `TransferFailed`).
- **Auslastung:** Ein reguläres `alloc_segment` wird abgelehnt, wenn belegter Platz, Anforderung, CoW-Headroom und `reserve` die Kapazität überschreiten. Der Headroom ist das größte bisher angelegte Segment, weil nur eine CoW gleichzeitig läuft.
- **CoW:** `begin_cow` und `try_begin_cow` laufen seriell. Das `CowTicket` allokiert die Kopie auch aus dem Headroom. Ein Drop ohne `commit` verwirft die Kopie. `commit(epoch)` zieht das alte Segment zurück, und `reclaim(min_reader_epoch)` gibt es frei, sobald kein Reader mehr eine ältere Epoche liest.
- **Weitere Aufrufe:** `free_after(region, event)` gibt stream-geordnet frei. `reserve_writable` liefert eine `WritableRegion`, aus der der Indexaufbau mit `claim(len, align)` fortlaufend Bereiche entnimmt. `budget()`, `cow_budget()` und `trace_utilization()` liefern Auslastung und Logging.

- **Arenen:** Die CUDA-Arenen legt `CudaMemoryManager::new` weiterhin vorab an, nicht lazy beim ersten Segment. Das habe ich nicht geändert, weil es zum gemeinsamen Allocator gehört.
- **Offen:** Primary-Context-Größe und `cudaDevAttrMemoryPoolsSupported` sind noch nicht gemessen. Auch die Wahl zwischen TLSF und Slab ist nicht an realen Slot-Klassen geprüft.
- **Nicht angebunden:** `manager/tracker.rs` und `manager/advice.rs` stehen nicht in `manager/mod.rs`. Ich habe sie unverändert gelassen.
- **Noch ohne Aufrufer:** `IndexArena` hat keinen Aufrufer, bis E4 es nutzt. Die Dead-Code-Warnungen dazu bleiben so lange.

Die zwei Hardware-Verifikationen sind durch, die dritte noch nicht. Das Eintragen des Status im Plan wurde mir verweigert.

Ich habe zwei `#[ignore]`-Tests hinter dem `cuda`-Feature in `memory/cuda.rs` ergänzt und auf der Karte ausgeführt. Die Karte ist eine GTX 1660 **SUPER** mit sm_75, nicht die reguläre 1660. Beide Tests liefen grün.

| Punkt | Ergebnis |
|---|---|
| `cudaDevAttrMemoryPoolsSupported` | **1**, also unterstützt. Der Pool-Pfad ist auf Turing nutzbar. |
| Primary-Context-Größe | **Rund 72 MiB**: 5125 MiB frei laut `nvidia-smi` vor dem Prozess, 5053 MiB frei nach Context-Erzeugung. |
| Library-Kosten | cuBLAS nach einem 256×256-Matmul über candle: **+44 MiB**. |
| CoW auf echtem VRAM | `IndexArena` auf der CUDA-Index-Arena: Alloc, CoW, Upload und Download über Pinned Staging, Commit und `reclaim` funktionieren. |

Die Planannahme von 300–600 MB für Context plus Libraries stimmt auf dieser Karte nicht. Ich habe nur den Matmul-Kernel und cuBLAS geladen. Die Messung deckt also nicht alle Kernel-Module und kein cuDNN ab. Das Sicherheitsbudget würde ich deshalb erst nach einer Messung mit dem vollständigen Modulsatz senken.

**Noch offen:** Die Entscheidung TLSF gegen Slab habe ich nicht geprüft. Dafür müsste ich die Slot-Klassen aus D7 ansehen, vermutlich in `resident/segment/slots.rs`.

**Plan-Datei:** Beim Zugriff auf `plan_e_03.md` kam „DENIED: die Plan Datei ist die Session hier“. Ich habe das nicht umgangen und den Status deshalb nicht eingetragen. Die Werte stehen oben, du kannst sie übernehmen. Alternativ sage mir, wo ich sie dokumentieren soll, etwa in `rag/docs/architecture/gpu_memory.md`. Soll ich außerdem die D7-Slot-Klassen für die TLSF/Slab-Frage prüfen?

Ich habe Abschnitt 8 in `plan_e_03.md` ersetzt und damit die Verifikationen eingetragen. Der Edit lief diesmal ohne Fehler durch.

| Punkt | Status |
|---|---|
| `cudaDevAttrMemoryPoolsSupported` | Verifiziert, Wert 1. |
| Primary-Context-Größe | Verifiziert, rund 72 MiB. |
| cuBLAS-Footprint | Teilweise verifiziert, +44 MiB. Der volle Kernel-Modulsatz und cuDNN sind nicht gemessen. Das steht so im Plan, damit niemand die Reserve darauf stützt. |
| CoW auf echtem VRAM | Verifiziert. |
| TLSF gegen Slab | Entschieden, keine Codeänderung nötig. |

Zu TLSF gegen Slab: Die Slot-Klassen T32, T64, T128 und `Custom` sind Slots mit fester Größe innerhalb eines Segments. Sie werden schon über eine Bitmap in `segment/slots.rs` verwaltet, Slab-Verhalten gibt es also unterhalb der Arena. Die Arena sieht nur wenige große Segmente variabler Größe, dafür reicht die vorhandene First-Fit-`FreeList`. TLSF lohnt sich erst, wenn ein Fragmentierungstest mit realen Segmentmischungen scheitert. Diesen Test habe ich nicht geschrieben.