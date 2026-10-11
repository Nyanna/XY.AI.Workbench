# Umsetzungsplan E7 · Residency & Ladestrategie

> **Quellen:** `resident_index.md` (Residency und Ladestrategie); **überlappt Kalibrierung** (`resident_precision.md`, Verweis F). **Recherche:** `context_e.md` §E7 (Punkte 26–30). **Querschnitt:** Q1, Q2, Q3.
> **Abhängt von:** E3 (Arena/Staging), E5 (Slot). **Ergebnis:** Lade-/Eviction-Strategie.

## 1. Ziel & Abgrenzung
Die Manager-Implementierung von **Residency**: lazy/scope-getrieben laden, Ladegranularität (ganze Segmente vs. Gather vs. Zero-Copy), Tiering (Summaries langlebig), Aggregation über Scope-Union, Single-Flight, Eviction (VRAM strenger als RAM). Die **Dichteschwelle** Gather↔Segment ist Manager-Implementierung und kann als Kalibrierungsachse (F) exportiert werden.

Nicht hier: die Scope-Auflösung selbst (E2), der Stamp-Abgleich (E6).

## 2. Voraussetzungen
- E3: `IndexArena`, Pinned-Staging-Pool, Copy-Stream, CoW-Budget.
- E5: Slot-Reihenfolge (Laden passiert als Update/Residency-Änderung zwischen Batches).
- E2: aufgelöste `EntrySet` (roaring) als Ladeeingabe.

## 3. Zu bauende Artefakte
```rust
pub enum LoadPlan { Segments(Vec<SegmentId>), Gather(EntrySet /* in Staging */), ZeroCopy(EntrySet) }
pub struct LoadPolicy { density_threshold: f32 /* Kalibrierungsachse */, segment_bytes: u64 }
pub struct SingleFlight<K, T> { inflight: Mutex<HashMap<K, Shared<BoxFuture<Result<Arc<T>, Arc<ManagerError>>>>>> }
pub struct Evictor { /* LRU + TTL + Hysterese, cost-aware */ }
```

## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
**Gather vs. Segment / Dichteschwelle** (context_e.md §E7 Punkte 26–27, `[W/?]`): Keine allgemeingültige Schwelle. Faustregel: **Segmentladen gewinnt ab hoher Dichte** (grob > ~10–25 % der Segmentbytes `[?]`), weil PCIe pro Transfer Fixkosten (~10 µs) hat und Kontiguität die Bandbreite ausreizt. Kostenmodell `Kosten ≈ Latenz·n_Transfers + Bytes/BW`, **auf der Zielkarte kalibrieren** → `density_threshold` als **Achse in `resident_precision.md`** exportieren. **Gather:** CPU gathert in **pinned Puffer**, dann **ein** `cudaMemcpyAsync` (besser als hunderte kleine Copies). **Zero-Copy** (`cudaHostAllocMapped` + `cudaHostGetDevicePointer`, UVA 64-Bit-Linux) nur für **sehr kleine**, selten gelesene Mengen (Latenz ~µs, Bandbreite < VRAM um Faktor ~20–30 `[?]`).

**Tiering** (`resident_index.md`): Summary-Strukturen (Zentroide/Präfixtabellen) klein, zuerst und am längsten gehalten; Vektorsegmente warm/evictable; alles andere nicht geladen. Segmentgröße bestimmt der Manager nach VRAM-Budget (keine Index-Eigenschaft).

**Aggregation** (`resident_index.md`): Tasks gegen denselben Index über die **Vereinigung ihrer Scopes** (roaring-Union aus E2) bündeln und **einmal** laden.

**Single-Flight** (context_e.md §E7 Punkt 28, `[W]`): Gleichzeitige Ladeanforderungen auf denselben Bereich = **ein** Ladevorgang mit mehreren Wartenden. Umsetzung: `tokio::sync::OnceCell::get_or_try_init` (bei Fehler darf nächster erneut versuchen) oder `futures::future::Shared` (Fehlertyp `Clone`, verteilt Fehler an alle). Muster: `Mutex<HashMap<Key, Shared<BoxFuture<Result<Arc<T>, Arc<E>>>>>>`, Eintrag nach Abschluss entfernen. **Fehler geht an alle Wartenden** (kein stiller Retry-Sturm).

**Eviction** (context_e.md §E7 Punkt 29, `[W]`): Teure Reloads ⇒ **LRU mit Pinning laufender Batches**, **TTL + Hysterese** (High/Low-Watermark gegen Thrashing), Summary-/Zentroid-Strukturen priorisiert halten, **cost-aware** (GDSF: Wert = Hitwahrscheinlichkeit × Reloadkosten/Größe). Unterschied zu RAM: kein transparentes Paging, Eviction **explizit**, Reload über PCIe, CoW-Spitzen. Idle-Zeiten für VRAM **deutlich länger** als für RAM.

**Überlappende Entries** (`resident_index.md`): Manager entscheidet zwischen einmaligem Halten mit Mehrfach-Referenz (spart VRAM, komplex) und Duplizieren (einfach, teuer) — Design-Entscheidung, kein Vertrag. Startwahl: **Duplizieren** (einfach), Referenz als Optimierung später.

## 5. Umsetzungsschritte
1. `LoadPolicy` mit `density_threshold` (Default aus Faustregel, überschreibbar durch Kalibrierung F).
2. `plan_load(EntrySet) -> LoadPlan`: Dichte aus EntrySet vs. Segmentbytes → Segments/Gather/ZeroCopy.
3. Aggregation: offene Ladeanforderungen gegen denselben Index per Scope-Union zusammenziehen.
4. `SingleFlight` um den Ladevorgang; Fehlerverteilung an alle Wartenden.
5. Gather in pinned Puffer + ein `cudaMemcpyAsync`; Segmentladen direkt; Zero-Copy für Mini-Mengen.
6. `Evictor`: LRU+TTL+Hysterese, cost-aware, Summaries pinnen, laufende Batches pinnen; Trigger bei VRAM-Druck (`cuMemGetInfo`).

## 6. Fehler / Logging / Feature-Flags
- **Q1:** `TransferFailed` (Copy), `ArenaExhausted` (kein Platz trotz Eviction). Single-Flight verteilt Fehler an alle Wartenden.
- **Q2:** Events: Lade-Entscheidung (Segments/Gather/ZeroCopy, Dichte), Single-Flight-Coalescing (Anzahl Wartende), Eviction-Entscheidung (Opfer, Grund), Arena-Auslastung.
- **Q4:** Alle CUDA-Copies hinter `cuda`; Entscheidungslogik (Plan/Aggregation/SingleFlight/Eviction) feature-frei testbar über Host-Arena-Mock + virtuelle Zeit.

## 7. Tests (Q3)
- **Gather-vs-Segment** über Dichteschwelle: niedrige Dichte → Gather, hohe → Segments, winzig → ZeroCopy.
- **Single-Flight:** ein Ladevorgang bei mehreren gleichzeitigen Anforderungen; **Fehlerverteilung an alle** Wartenden; Eintrag danach entfernt.
- Aggregation: zwei Tasks mit überlappenden Scopes → ein Ladevorgang über die Union.
- Eviction: TTL + Hysterese kein Thrashing (virtuelle Zeit); Summaries werden nicht evictet; laufender Batch gepinnt.

## 8. Offene Verifikationen
- Gather-vs-Segment-Schwelle **empirisch auf GTX 1660 messen** (context_e.md offener Punkt 4) und als Kalibrierungsachse an F übergeben.
- `OnceCell::get_or_try_init` vs. `Shared`-Verhalten bei Fehlern gegen aktuelle Crate-Version prüfen.

## 9. Definition of Done
- `plan_load`, Aggregation, Single-Flight, Evictor implementiert; Dichteschwelle als Kalibrierungsachse exportiert; alle Tests grün über Host-Mock ohne GPU.
