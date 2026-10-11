# Umsetzungsplan E3 · ArenaAllocator (GPU, Index-Segmente)

> **Quellen:** `resident_index.md` (Manager-Disziplin/Arena); **überlappt D3** (`resident_subengine.md` §4.2). **Recherche:** `context_e.md` §E3 (Punkte 8–12). **Querschnitt:** Q1, Q2, Q4.
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
- Primary-Context-Größe und `cudaDevAttrMemoryPoolsSupported` auf GTX 1660 messen (context_e.md offener Punkt 2).
- TLSF- vs. Slab-Entscheidung an realen Slot-Klassen (D7) überprüfen.

## 9. Definition of Done
- Eine Arena-Implementierung (konsolidiert mit D3), 256-B-Offset-Suballokation, budgetierte CoW-Heuristik, Pinned-Staging/Copy-Stream referenziert; Host-Mock-Pfad testbar ohne GPU.
