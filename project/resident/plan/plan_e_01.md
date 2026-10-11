# Umsetzungsplan E1 · StreamedIndexManager-Grundgerüst

> **Quellen:** `resident_index.md` (Drei Schichten, Leitprinzipien, Umsetzungsvorgaben). **Recherche:** `context_e.md` §E1 (Punkte 1–4). **Querschnitt:** Q1, Q2, Q4.
> **Abhängt von:** B4 (Index-Trait), D7 (Segment-Repräsentation). **Ergebnis:** Manager-Rollen-Spezifikation + lauffähiges Grundgerüst.

## 1. Ziel & Abgrenzung
Das Skelett des `StreamedIndexManager` als **Komponente der CPU/GPU-Engine** (nicht Fassade, nicht Index). Konstruktion mit einer Engine-Referenz und einem **lazy aktivierten** Index; der Index bleibt unabhängig nutzbar. Zwei Ausprägungen:
- **CPU-Manager:** trivial — liest den Index direkt (RAM ist Primärinstanz), fast keine Arbeit. Einzige Aufgabe: Residency bei mmap-Indizes über RAM-Größe (Prefetch via `madvise`, Entladen nach Idle).
- **GPU-Manager:** vollständiger Streaming-Manager (Fassade/Hülle; die konkrete Logik entsteht in E2–E8).

Nicht hier: Scope-Auflösung (E2), Arena (E3), Mutationsdisziplin (E4), Slot (E5), Sync (E6), Residency-Strategie (E7), Indexaufbau (E8).

## 2. Voraussetzungen
- B4-Trait `Index` inkl. `EntryId`, `Stamp`, `EntrySet`, `AccessMethod`, `watch<u64>`.
- D7-Typen `SegmentView`/Slot und Mapping `(segment,slot)→EntryRef` (als Signaturen).
- Gemeinsame Engine-Handle-Abstraktion (CPU/GPU) aus Gruppe D.

## 3. Zu bauende Artefakte
```rust
pub struct StreamedIndexManager<B: ComputeBackend> {
    engine: EngineHandle<B>,
    index: OnceCell<Arc<dyn Index>>,   // lazy aktiviert
    residency: ResidencyTracker,       // Mapping Entry <-> Slot + bekannte Stamps (Platzhalter, Ausbau E6)
    changed_rx: Option<watch::Receiver<u64>>,
}

pub trait IndexManager {               // gemeinsame Schnittstelle CPU/GPU
    fn attach_index(&self, index: Arc<dyn Index>);
    fn is_active(&self) -> bool;        // Index geladen?
}

pub struct CpuIndexManager { /* direkter Lesezugriff + mmap-Residency */ }
pub struct GpuIndexManager<B> { /* Streaming-Hülle */ }
```
- `ResidencyTracker` zunächst als Platzhalter (Map `EntryId → (Slot, Stamp)`), voll ausgebaut in E6/E7.

## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
**mmap-Residency über RAM-Größe & `madvise`-Prefetch** (context_e.md §E1, `[W]`):
- **Prefetch:** `memmap2::Mmap::advise(Advice::WillNeed)` (safe) für heiße Bereiche; `Advice::Sequential` bei dichten Segment-Scans, `Advice::Random` bei Gather. Alles nur Hints, kein Garantieversprechen.
- **Entladen/Kaltstellen:** `MADV_DONTNEED` entfernt nur Seiten aus den Page Tables, **nicht** aus dem Page-Cache (file-backed shared). Für echtes Kaltstellen `MADV_COLD`/`MADV_PAGEOUT` (Linux ≥ 5.4) bzw. `posix_fadvise(POSIX_FADV_DONTNEED)`. `mlock` **nur** für kleine Summary-Strukturen.
- **Residency-Messung:** `mincore(2)` (via `libc`/`rustix`) liefert pro Page „im Page-Cache resident" — nur Momentaufnahme unter Last.
- **Soft-Grenze:** Keinen eigenen Cache über mmap bauen — **mit** dem Page-Cache arbeiten. Working-Set-Fenster: `WILLNEED` für heiße, `COLD`/`PAGEOUT` für kalte Bereiche; optional cgroup v2 `memory.high` (Throttling statt OOM).
- **Blockierende Zugriffe:** Page-Faults in async-Tasks blockieren den Worker-Thread. mmap-Lesen/Prefetch daher in `spawn_blocking` oder dediziertem Thread-Pool; Ergebnis über `oneshot`/Channel. **Fallstricke:** `spawn_blocking`-Tasks sind nicht abbrechbar; Default-Pool (512) kann bei Fault-Stürmen erschöpfen; `&[u8]`-Zugriff in `async`-Code ist ein unsichtbarer Block.

**CPU- vs. GPU-Manager:** gemeinsame `IndexManager`-Schnittstelle; CPU-Variante delegiert Reads direkt an den Index + mmap-Residency; GPU-Variante hält zusätzlich die Streaming-Zustände (ab E3).

## 5. Umsetzungsschritte
1. `IndexManager`-Trait + `StreamedIndexManager`-Struct mit `OnceCell<Arc<dyn Index>>` (lazy, `get_or_try_init`).
2. `attach_index` / Lazy-Aktivierung; bei erstem Zugriff `changed()`-Receiver abonnieren.
3. `CpuIndexManager`: direkter `index.read(entry)` im Blocking-Pool; mmap-Residency-Hooks (`advise`).
4. `GpuIndexManager`: leere Hülle mit Feldern für Arena/Slot/Tracker (Platzhalter für E3–E8), hinter `#[cfg(feature = "cuda")]` für reale Pfade.
5. `ResidencyTracker`-Grundstruktur (Map + bekannter Stamp je Entry).

## 6. Fehler / Logging / Feature-Flags
- **Q1:** `IndexUnavailable` (Index noch nicht aktiv / Ladefehler). Modul-Enum `ManagerError`, per `#[from]` ins Top-Level.
- **Q2:** Span pro Manager-Zyklus (`#[instrument(skip_all, fields(index_id))]`); Events: Index aktiviert, Residency-Hint gesetzt/entfernt.
- **Q4:** mmap-Pfad feature-frei (CPU); GPU-Hülle hinter `cuda`.

## 7. Tests (Q3)
- Lazy-Aktivierung: Manager verbraucht ohne Index keine Ressourcen; erster Zugriff aktiviert.
- CPU-Manager liest Entry direkt; blockierender Read läuft im Blocking-Pool (kein async-Worker-Block — über instrumentierten Fake-Clock/Hook prüfen).
- mmap-Residency: `WillNeed`/`Cold` werden an der richtigen Stelle gesetzt (Fake-`Advise`-Sink zählt Aufrufe).
- `FakeIndex` ohne echte Datei für alle Logiktests.

## 8. Offene Verifikationen
- Linux-`madvise`-Semantik gegen man7 prüfen; `memmap2`-`Advice`-API gegen aktuelle Crate-Version (context_e.md offener Punkt 3).
- `MADV_COLD`/`PAGEOUT` erst ab Kernel ≥ 5.4 — Zielkernel bestätigen.

## 9. Definition of Done
- Lazy-Manager konstruierbar, CPU-Variante liest Index korrekt, mmap-Residency-Hooks getestet, GPU-Hülle kompiliert mit und ohne `cuda`. Keine Ressourcenbelegung vor erstem Zugriff.
