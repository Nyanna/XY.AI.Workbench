Lies zunächst alle Dokumente in `/home/user/xyan/xy.ai.workbench/project/resident` ein.

```json
{
  "id" : "call_fecd700b-dd1a-44b0-aa8a-cdcc668509d5",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident",
      "entries" : [ ".:", "resident.md", "resident_first.md", "resident_index.md", "resident_planner.md", "resident_precision.md", "resident_processor.md", "resident_subengine.md", "resident_planplan.md" ]
    } ]
  }
}
```

Erstelle eine Liste aller in `resident_planplan.md` für Gruppe E zu beantwortenden Fragen für eine Web-Recherche. Ein anderer Agent wird diese Fragen recherchieren, beantworten und ein Markdown Kontext File als Ergebnis liefern.
SystemInit:  id=25a99733-10ed-4374-b381-90c2572506d2, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 Ich lese zunächst alle Dokumente im Verzeichnis ein.
Control Request:
```yaml
id: 0f9c-7
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_first.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  reason: "Einlesen aller Projektdokumente im resident-Verzeichnis, um den Kontext zu verstehen und die Fragen für Gruppe E aus resident_planplan.md zu extrahieren."
```
/answer 0f9c-7 allow
Result Stats: id=25a99733-10ed-4374-b381-90c2572506d2, total: 63548, in: 119, out: 11791, reason: 6275, read: 2836, write: 51638
ReasoningToken: 53
ReasoningToken: 6222
Ich habe alle acht Dokumente gelesen. Nachfolgend die abgeleitete Fragenliste für Gruppe E (**Index-Management & Synchronisation**, Pakete E1–E9) aus `resident_planplan.md`, aufbereitet als Recherche-Auftrag für einen Web-Recherche-Agenten.

---

# Web-Recherche-Auftrag: Gruppe E — Index-Management & Synchronisation

## Kontext für den Recherche-Agenten

Diese Fragen entstammen dem Metaplan der „Resident"-Engine (Rust-RAG-Engine für ColBERT/dense-Retrieval mit residenten Modellen/Indizes). Relevanter technischer Rahmen für alle Antworten:

- **Sprache/Stack:** Rust; `candle`/`candle-core` (CUDA-Feature), `cudarc` (Custom-Kernel), `tokio` (Async), `crossbeam`/`parking_lot`, `tokenizers`, `parquet` + **mmap** (read-heavy).
- **Zielhardware:** GTX 1660 = **Turing, Compute Capability sm_75**, 6 GB VRAM → Antworten müssen auf dieser Architektur gelten (nicht nur Ampere/Hopper).
- **Architekturprinzip:** Der Index ist RAM-Primärinstanz, VRAM ist abgeleiteter Spiegel; ein `StreamedIndexManager` synchronisiert RAM ↔ Index ↔ VRAM; der GPU-Kernel wird **nie blockiert**, Mutation nur in „Lücken".
- **Ergebnisformat:** Ein Markdown-Kontext-File, pro Frage mit Antwort, Quellenangaben (URLs) und Kennzeichnung unsicherer/unverifizierter Aussagen.

---

## E1 · StreamedIndexManager-Grundgerüst (mmap-Residency, Prefetch)

1. Wie arbeiten `madvise`-Hinweise unter Linux (insb. `MADV_WILLNEED`, `MADV_DONTNEED`, `MADV_SEQUENTIAL`, `MADV_RANDOM`) für gezieltes Prefetchen bzw. Entladen von mmap-Regionen? Welche Garantien bzw. Grenzen gibt es (synchron/asynchron, Wirkung auf Page-Cache)?
2. Welche Rust-Crates (z. B. `memmap2`) bieten Zugriff auf `madvise`? Wie lässt sich die tatsächliche RAM-Residency einzelner Pages ermitteln/steuern (z. B. `mincore`)?
3. Wie begrenzt man die RAM-Residency eines großen mmap-Index gezielt über eine Soft-Grenze, ohne gegen den OS-Page-Cache zu arbeiten? Best Practices für read-heavy, großvolumige mmap-Dateien.
4. Wie trennt man blockierende `read`/Page-Fault-/Platten-I/O-Zugriffe sauber vom tokio-Runtime-Thread (Blocking-Pool, `spawn_blocking`)? Etablierte Muster und Fallstricke.

## E2 · Scope & Resolver (pluggable Resolver pro AccessMethod)

5. Welche etablierten Rust-Muster gibt es für eine pluggable Resolver-/Strategy-Registry (`AccessMethod → ScopeResolver`), z. B. per Trait-Objekten oder type-keyed Registry? Vor-/Nachteile, Fallstricke bei Erweiterbarkeit.
6. Wie funktioniert zentroidbasiertes Pruning/Scope-Auflösung in PLAID / ColBERTv2 konkret (Query → Zentroide → Zellenmitglieder / Inverted Lists / Candidate Generation)? Welche Schritte und Datenstrukturen?
7. Wie stellt man Scopes als kompakte Entry-Mengen dar? Vergleich Roaring Bitmaps (`roaring` crate) vs. Range-Listen hinsichtlich Speicher- und Laufzeitcharakteristik bei Set-Operationen (Vereinigung für Scope-Aggregation).

## E3 · ArenaAllocator (GPU) — VRAM-Budget, Free-List, CoW, Pinned Staging

8. Warum lösen `cudaMalloc`/`cudaFree` im laufenden Betrieb implizite Device-Synchronisation aus, und wie vermeidet man das mit einer vorab allozierten Arena? Wie verhält sich der stream-ordered Allocator (`cudaMallocAsync`/Memory Pools) dazu — und ist er auf Turing/dem verwendeten Treiber nutzbar?
9. Wie fragt man das verfügbare VRAM-Budget korrekt via `cuMemGetInfo` ab und plant Context-Overhead ein? Was belegt der Primary Context auf einer 6-GB-Turing-Karte typischerweise?
10. Welche Free-List-/Suballocator-Designs eignen sich für GPU-Arenen (Buddy, Slab, TLSF)? Fragmentierung, Alignment-Anforderungen für CUDA-Allokationen, u64-Offset-Adressierung.
11. Wie dimensioniert man ein Arena-Budget, das kurzzeitigen **Copy-on-Write-Doppelbedarf** (alte + neue Segmentkopie gleichzeitig) einplant? Übliche Heuristiken.
12. Pinned (page-locked) Host-Memory als Staging: Allokationskosten, Wiederverwendung von Staging-Puffern, Auswirkung auf H2D/D2H-Bandbreite; Nutzen eines separaten Copy-Streams getrennt vom Compute-Stream.

## E4 · Manager-Disziplin (Pointer-Swap, Acquire-Load, Fence-Sichtbarkeit)

13. CUDA/PTX `ld.acquire`/`st.release` und `__threadfence()`: Ab welcher Compute Capability verfügbar (gilt sm_75/Turing?), und wie korrekt einsetzen, damit ein vom Host getauschter Pointer vom persistenten Kernel sicher gelesen wird?
14. CUDA Memory Consistency Model: Wie garantiert man, dass der **Inhalt** einer neu angelegten Tabelle **vor** dem Pointer-Swap für den Kernel sichtbar ist (Events, Fences, `cudaStreamSynchronize`, Scopes)?
15. Wie realisiert man einen atomaren 8-Byte-Pointer-Swap, der von Host und GPU konsistent gesehen wird (system-scope atomics, mapped/unified memory)? Welche Voraussetzungen gelten auf Turing?
16. Welche Lock-free-Muster erlauben Readern (GPU-Kernel) sync-freies Weiterlaufen, während ein Writer (Host) Copy-on-Write über eine Pointer-Hierarchie (Root → Segmenttabelle → Segment) macht?
17. Unterschied Pointer-Lesezeitpunkt: „normale" Launches (Pointer als Kernel-Argument, wirkt ab nächstem Launch) vs. persistenter Kernel (Acquire-Load pro Work-Item). Fallstricke und Empfehlungen.

## E5 · Kommunikationsslot (feste Reihenfolge Host ↔ Kernel)

18. Welche etablierten Muster gibt es für einen „Kommunikationsslot" an Batch-Grenzen, in dem der Host nur in Kernel-Lücken mutiert (Batch-Ende-Event, Fortschrittszähler in gemapptem Host-Speicher), ohne den Kernel zu blockieren?
19. Wie bildet man host-seitig vom Kernel geschriebene VRAM-Adressbereiche effizient auf Entries zurück (Adresse → Entry)? Intervall-Maps / Binärsuche über Offset-Tabellen in Rust.
20. „Indexieren während der Suche": Welche Muster erlauben einem Stream, gleichzeitig neue Daten zu schreiben und zu lesen, mit pro-Batch konsistentem Mapping-Stand?

## E6 · Synchronisation & Echo-Vermeidung (zustandsbasiert, Stamps, ABA)

21. `tokio::sync::watch` als Level-/Watermark-Signal: Semantik bei zusammengefassten/verlorenen Benachrichtigungen; wie implementiert man korrekt „Signal **vor** dem Scan quittieren" für einen zustandsbasierten Abgleich?
22. ABA-Problem bei Entfernen/Wiederanlegen derselben ID: Wie verhindern global eindeutige, monotone 64-Bit-Stamps/Generationszähler das ABA-Problem? Etablierte Techniken.
23. Lesereihenfolge „Stamp → Daten → Stamp erneut" bei mmap: Wie funktioniert ein **Seqlock**/versioned read für lock-freies, konsistentes Lesen bei nebenläufigen Schreibern? Korrektheitsbedingungen.
24. Baumartige Indizes mit max-Stamp pro Teilbaum und **pro-Konsument-Watermark**: Wie entwirft man das korrekt, und warum verliert eine gemeinsame Bitmap Änderungen für den zweiten Konsumenten? Etablierte per-consumer change-tracking-Lösungen.
25. `changed_since(watermark)` als begrenzter Recent-Change-Ring mit Fallback auf Vollscan bei Überlauf: Übliche Designs für bounded change logs / Ring-Buffer.

## E7 · Residency & Ladestrategie (Gather vs. Segment, Single-Flight, Eviction)

26. Gather (DMA vieler kleiner, verstreuter Elemente in einen kompakten Puffer) vs. Laden ganzer Segmente: Ab welcher Dichte lohnt sich welche Variante? Bekannte Heuristiken/Messgrößen für Scatter/Gather vs. kontiguösen PCIe-Transfer.
27. Pinned Staging + ein einzelner DMA für Gather vs. viele kleine Transfers: Performance-Charakteristik und Best Practices.
28. Single-Flight / Request-Coalescing in Rust (ein Ladevorgang, mehrere Wartende, Fehlerverteilung an alle): etablierte Crates/Muster (z. B. `singleflight`, `futures::Shared`, `OnceCell::get_or_try_init`).
29. Tiering-/Eviction-Policies für VRAM-Caches mit teuren Reloads (LRU, TTL mit Hysterese, Summary-Strukturen lange halten): Was eignet sich, und wie unterscheidet sich das von RAM-Eviction?
30. Zero-Copy aus gemapptem Host-Speicher für sehr kleine Mengen (`cudaHostGetDevicePointer`, mapped/pinned memory): Voraussetzungen, Grenzen und Performance auf Turing.

## E8 · Kernel schreibt Segmente (Indexaufbau)

31. Muster für GPU-Kernel, die in einen vorab vergebenen Bereich per Cursor schreiben und **nie selbst allozieren** (Output-Arena + atomarer Cursor via `atomicAdd` auf Offset): etablierte Techniken, Umgang mit Überlauf und Teilergebnis.
32. Fixed-Stride- vs. Variable-Layout-Rückabbildung: effiziente Konstruktion von Offset-Tabellen (Prefix-Summen) für variabel lange Entries und Binärsuche darauf; wann lohnt Fixed-Stride.
33. Getrennte Ringe für Embedding (liefert neue Segmente) vs. MaxSim (liefert Hits): wie entwirft man getrennte Producer/Consumer-Pfade, und warum ermöglicht „Schritt 1 (Writes lesen) vor Schritt 2 (Ergebnisse auflösen)" gleichzeitiges Indexieren und Suchen?

## E9 · Addons (Iteration/Kaskade/Rekursion — nur Erweiterungspunkt)

34. Erweiterbare, typsichere Parameterobjekte in Rust ohne Kernänderung: Vergleich von type-maps (`anymap`), `HashMap<TypeId, Box<dyn Any>>`, Extension-Traits und Builder-Mustern — Vor-/Nachteile, Serialisierbarkeit.
35. Iteration/Kaskade als „Addon", das Folgetasks über die normale Queue einreiht (nicht unterscheidbar von App-Tasks): etablierte Muster für selbst-schedulende Task-Graphen ohne kern-seitige Eltern-Kind-Beziehung (z. B. continuation-passing, work-stealing).
36. (These-Bezug) Cascaded/fractal refinement und Funnel (PLAID/ColBERT): Wie sind kaskadierende Retrieval-Stufen (coarse → fine) in der Literatur üblicherweise aufgebaut?

---

**Hinweise zur Priorisierung** (laut Metaplan höchste Unschärfe/Risiko in Gruppe E):
- **E4** (CUDA Acquire-Load/Fence-Sichtbarkeit auf Turing, Fragen 13–17) und **E3** (CoW-Budget/`cudaMallocAsync`, Fragen 8, 11) sind hardwarenah und am risikoreichsten → besonders gründlich mit sm_75-spezifischen Quellen belegen.
- **E6** (Seqlock, ABA, per-Konsument-Watermark, Fragen 22–24) betrifft das noch offene Konsistenzmodell der Index-Synchronisation (D7/E) → etablierte, zitierfähige Muster bevorzugen.

Soll ich die Liste zusätzlich als Markdown-Datei im Projektordner ablegen (z. B. `resident_groupE_research.md`)?
