# Kontext Gruppe E — Index-Management & Synchronisation (Resident-Engine)

**Stand:** 10.10.2026 · **Zielhardware:** sm_75 (Turing), 6 GB.
**Kennzeichnung:** `[V]` = in dieser Recherche per Quelle belegt · `[W]` = aus Fachwissen, **nicht** verifiziert (vor Verwendung gegenprüfen) · `[?]` = unsicher/offen.
Hinweis: Die Recherche war auf eine Auswahl der Kernquellen begrenzt; E3/E4/E6 sind am besten belegt, E1/E2/E5/E7–E9 überwiegend `[W]`.

---

## E1 · StreamedIndexManager (mmap, Prefetch)

**1. madvise (Linux)** `[W]`
- `MADV_WILLNEED`: stößt asynchrones Read-ahead an (Hint, kein Garantieversprechen, kehrt ohne Warten auf I/O zurück).
- `MADV_DONTNEED`: entfernt Seiten aus den Page Tables des Prozesses; bei file-backed Shared-Mappings bleibt der Page-Cache erhalten (kein echtes Evict aus dem Cache). Für Page-Cache-Eviction: `posix_fadvise(POSIX_FADV_DONTNEED)` oder `MADV_COLD`/`MADV_PAGEOUT` (Linux ≥ 5.4).
- `MADV_SEQUENTIAL`: aggressiveres Read-ahead; `MADV_RANDOM`: Read-ahead aus. Beides Hints.
- Die gefundenen Treffer waren BSD/Solaris-Manpages, die nur die generische Semantik bestätigen (WILLNEED = „bald benötigt", DONTNEED = „nicht bald benötigt") `[V]`: https://www.unix.com/man_page/opensolaris/3c/madvise · Linux-Manpage (nicht abgerufen): https://man7.org/linux/man-pages/man2/madvise.2.html

**2. Rust-Crates / Residency** `[W]`
- `memmap2`: `Mmap::advise(Advice::…)` (safe: WillNeed/Sequential/Random), `unsafe advise_range`/`UncheckedAdvice` (u. a. DontNeed).
- Residency: `mincore(2)` (via `libc`/`rustix`) liefert pro Page „im Page-Cache resident". Steuern: `mlock`/`MADV_WILLNEED`. `[?]` Genauigkeit unter Last nur Momentaufnahme.

**3. Soft-Grenze für Residency** `[W]`
- Kein direkter mmap-Soft-Limit-Mechanismus. Optionen: cgroup v2 `memory.high` (Throttling statt OOM); eigenes Fenster/Working-Set-Management mit WILLNEED für heiße, `MADV_COLD`/`PAGEOUT` für kalte Bereiche; `mlock` nur für kleine Summary-Strukturen. Mit dem Page-Cache arbeiten, nicht dagegen: kein eigener Cache über mmap.

**4. Blockierende Zugriffe vs. tokio** `[W]`
- Page-Faults in async-Tasks blockieren den Worker-Thread. Muster: mmap-Lesen/Prefetch in `spawn_blocking` oder dedizierten Thread-Pool (z. B. rayon), Ergebnis über Channel/oneshot.
- Fallstricke: `spawn_blocking`-Tasks sind nicht abbrechbar; Pool-Größe (Default 512) kann bei vielen Faults erschöpft werden; kein Page-Fault-Schutz bei Zugriff via `&[u8]` innerhalb von `async`-Code (unsichtbarer Block).
- tokio-Doku: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html (nicht abgerufen)

---

## E2 · Scope & Resolver

**5. Pluggable Resolver-Registry** `[W]`
- Varianten: `HashMap<AccessMethod, Arc<dyn ScopeResolver>>` (einfach, dyn-Dispatch, Erweiterung ohne Kernänderung); `HashMap<TypeId, Box<dyn Any>>`-Registry (typsicher via Downcast, Laufzeitfehler möglich); Enum-Dispatch (schnell, aber geschlossen).
- Fallstricke: `Send + Sync`-Bounds, Object-Safety (keine generischen Methoden), Registrierungsreihenfolge/Duplikate, Lebensdauer bei Hot-Registration.

**6. PLAID / ColBERTv2 Candidate Generation** `[V]`
- Query-Embeddings `Q` gegen alle Zentroide `C`: `S = C · Qᵀ` (Matrixmultiplikation). Pro Query-Token werden die `nprobe` nächsten Zentroide gewählt; deren Inverted Lists liefern Kandidaten.
- Vanilla ColBERTv2: Inverted List Zentroid → Embedding-IDs. PLAID: Zentroid → **eindeutige Passage-IDs** (32-Bit möglich), kein hartes Limit der Kandidatenzahl; danach Stufen mit Centroid Interaction (Bag-of-Centroids-MaxSim) und Centroid Pruning, am Ende Residual-Dekompression nur für wenige Kandidaten.
- Quellen: https://arxiv.org/pdf/2205.09707 · Reproduzierbarkeit: https://arxiv.org/pdf/2404.14989v1

**7. Roaring vs. Range-Listen** `[W]`
- Roaring (`roaring` crate): Container pro 2¹⁶-Block (Array/Bitmap/Run), schnelle Vereinigung/Schnitt, speicherkompakt bei gemischter Dichte. Range-Listen: minimal bei wenigen, langen zusammenhängenden Intervallen; Union = Merge in O(n+m), degeneriert bei fragmentierten Mengen. Roaring mit Run-Container deckt beide Fälle ab; Ranges nur, wenn Scopes garantiert zusammenhängend sind.

---

## E3 · ArenaAllocator (GPU)

**8. cudaMalloc/cudaFree und Stream-Ordered Allocator**
- NVIDIA: cudaMalloc/cudaFree führen zu Synchronisation über laufende Streams `[V]` (Aussage aus dem NVIDIA-Blog, chinesische Fassung: https://developer.nvidia.cn/blog/cuda-stream-order-memory-allocator-cn/). `cudaMallocAsync`/`cudaFreeAsync` (ab CUDA 11.2) vermeiden diese Synchronisation und arbeiten pool-basiert mit konfigurierbarem Release-Threshold `[V]`: https://developer.nvidia.com/blog/using-the-nvidia-cuda-stream-ordered-memory-allocator-part-1/ · Part 2: https://developer.nvidia.com/blog/using-the-nvidia-cuda-stream-ordered-memory-allocator-part-2/ · Programming Guide: https://docs.nvidia.com/cuda/cuda-programming-guide/04-special-topics/stream-ordered-memory-allocation.html
- Der Speicher wird bei `cudaFreeAsync` erst bei der nächsten Stream-Synchronisation wirklich freigegeben `[V]` (Part 2).
- Arena: einmal groß `cudaMalloc` beim Start, danach nur Suballokation → keine Runtime-Syncs.
- **Turing-Nutzbarkeit:** `[W]` Turing ist ab CUDA 11.2/Treiber ≥ 460 grundsätzlich unterstützt; verbindlich nur über `cudaDevAttrMemoryPoolsSupported` zur Laufzeit prüfen. `[?]` Unter WDDM (Windows) kann das Verhalten abweichen. Für eine feste Arena ist der Async-Allocator ohnehin nicht nötig.

**9. VRAM-Budget** `[W]`
- `cuMemGetInfo` (free/total) **nach** Context-Erzeugung und nach dem Laden von cuBLAS/cuDNN/Kernel-Modulen aufrufen; Budget = free − Sicherheitsreserve. Primary Context + Libraries auf Turing grob 300–600 MB `[?]` (Schätzwert, auf der Zielkarte messen). Display-GPU belegt zusätzlich VRAM.

**10. Suballocator-Designs** `[W]`
- Buddy: einfach, interne Fragmentierung bis 50 %. Slab: ideal bei wenigen festen Größen (Segmente fester Größe). TLSF: O(1), geringe Fragmentierung, gut für variable Größen. Alignment: `cudaMalloc` liefert ≥ 256 B ausgerichtet; Suballokationen auf 256 B (für Vektor-Loads/Coalescing mind. 16 B) alignen. Adressierung als `u64`-Offset relativ zur Arena-Basis (Pointer = Basis + Offset), 32-Bit nur bei < 4 GiB.

**11. CoW-Doppelbedarf** `[W]`
- Heuristik: Budget ≥ Live-Daten + (max. gleichzeitige CoW-Kopien × größtes Segment). Segmente klein halten (Granularität begrenzt den Spitzenbedarf), CoW seriell statt parallel, alte Kopie erst nach Epochenende freigeben. `[?]` kein zitierfähiges Standardverfahren gefunden.

**12. Pinned Staging / Copy-Stream** `[W]`
- `cudaHostAlloc` ist teuer (Page-Locking, ms-Bereich) → einmalig Pool anlegen und wiederverwenden. Pinned erlaubt echt asynchrone H2D/D2H und volle PCIe-Bandbreite; pageable Memory wird intern gestaged. Separater Copy-Stream (nicht-blockierend priorisiert) ermöglicht Überlappung Copy ↔ Compute (Turing: Copy-Engines vorhanden). Pinned-Gesamtmenge begrenzen (OS-Limit/RAM-Druck).

---

## E4 · Manager-Disziplin (Hochrisiko)

**13. ld.acquire / st.release / __threadfence auf sm_75** 
- PTX-Scoped-Memory-Modell mit `ld.acquire`, `st.release`, `fence.sc/acq_rel`: ab **sm_70 (Volta)**; Turing (sm_75) ist Nachfolger und gehört dazu `[V für sm_70+ bei fence.sc/membar]`: „On sm_70 and higher membar is a synonym for fence.sc" — https://docs.rs/crate/ptx-90-parser/latest/source/docs/instructions/fence.md (Spiegel der PTX-Doku). Die Verfügbarkeit von `ld.acquire` selbst ab sm_70 `[W]`, in der PTX-ISA-Doku gegenprüfen: https://docs.nvidia.com/cuda/parallel-thread-execution/
- `__threadfence()` entspricht device-scope-Fence (`membar.gl`, ab sm_70 = `fence.sc.gpu`) `[V]` (gleiche Quelle). Für Host↔GPU relevant ist Scope `.sys`.
- Offizielle ABI-Zuordnung C++ → PTX `[V]`: Release-Store = `st.release.<scope>` (empfohlen) oder `fence.release; st.relaxed`; Acquire-Load = `ld.acquire.<scope>` (empfohlen) oder `ld.relaxed; fence.acquire`: https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/atomic-abi.html · Formale Grundlage: Lustig et al., ASPLOS’19 (dort referenziert).
- Einsatz: Host publiziert Pointer mit Release (nach vollständigem Schreiben der Tabelle), Kernel liest Pointer mit `ld.acquire.sys` (Scope `sys`, weil Host beteiligt) bzw. `cuda::atomic_ref<…, thread_scope_system>`/`cuda::atomic<…>::load(memory_order_acquire)`. Anschließend sind alle Daten, die vor dem Release geschrieben wurden, sichtbar.

**14. Inhalt vor Pointer-Swap sichtbar** `[W, Prinzip aus V-Quellen ableitbar]`
- Reihenfolge: (1) Tabelle schreiben/kopieren; (2) Abschluss sicherstellen (`cudaStreamSynchronize`/Event bei Copy-Stream; bei Host-Schreibern in gemapptem Speicher Release-Store); (3) Pointer mit Release-Semantik publizieren; (4) Kernel liest Pointer mit Acquire (Scope sys). Scopes müssen die beteiligten Threads umfassen: Host-Thread ↔ GPU-Thread ⇒ `.sys`; `__threadfence()` (gpu-Scope) allein genügt für Host-Beteiligung **nicht** `[W]`.
- Wichtig: Eine `cudaMemcpyAsync` in einem anderen Stream wird für einen **laufenden** persistenten Kernel erst durch ein per Host-Event/Sync abgesichertes Flag/Pointer-Publish zu einem definierten Zeitpunkt sichtbar; die Streamordnung gilt nicht zwischen konkurrierenden Streams `[W]`.

**15. Atomarer 8-Byte-Pointer-Swap Host↔GPU** `[W/?]`
- 8-Byte-aligned `AtomicU64` in pinned+mapped Host-Speicher (`cudaHostAllocMapped`); Host: `store(Release)`; GPU: `ld.acquire.sys`. Reine Load/Store-Atomarität für naturally aligned 8 B ist gegeben. Echte RMW-Atomics (CAS) des GPU auf Host-Speicher über PCIe: Unterstützung abhängig von Plattform/OS `[?]`; Swap lässt sich meist als Single-Writer-Store (nur Host schreibt) ohne CAS lösen. Unified Memory mit gleichzeitigem CPU/GPU-Zugriff (`concurrentManagedAccess`) ist unter Windows nicht verfügbar `[W]` — mapped pinned verwenden.

**16. Lock-free-Muster, Reader=GPU, Writer=Host** `[W]`
- Immutable Segmente + Copy-on-Write: neue Segmenttabelle bauen, Root-Pointer einmal swappen (RCU-artig). Alte Version erst freigeben, wenn kein Reader mehr darauf zeigt → Epochenzähler: Kernel meldet pro Batch die gelesene Epoche (Fortschrittszähler); Host gibt frei, sobald alle Reader ≥ neue Epoche. Nie in-place mutieren.

**17. Pointer-Lesezeitpunkt: Launch-Argument vs. persistenter Kernel** `[W]`
- Launch-Argument: Wert wird bei Launch fixiert (konsistenter Snapshot pro Launch, Wechsel ab nächstem Launch). Persistenter Kernel: Acquire-Load pro Work-Item ⇒ Pointer kann mitten im Batch wechseln. Empfehlung: Pointer nur an Batch-Grenzen (Kommunikationsslot) tauschen bzw. einmal pro Batch lesen und im Registerfile halten; Mischzustände innerhalb eines Batchs vermeiden; Volatile/relaxed-Loads ohne Acquire vermeiden (Compiler-Caching).

---

## E5 · Kommunikationsslot

**18. Muster** `[W]`
- Doorbell-/Mailbox-Ring in mapped pinned Memory: Kernel schreibt pro Batch Fortschrittszähler (Release), Host pollt (Acquire) oder wartet auf `cudaEvent`. Host mutiert nur zwischen „Batch-Ende" und „nächster Batch-Start-Quittung". Nie auf Kernel-Ende warten.

**19. Adresse → Entry** `[W]`
- Sortierte Offset-Tabelle (Prefix-Summe der Entry-Größen) + `slice::partition_point`/`binary_search` (O(log n)); bei Fixed-Stride direkt `offset / stride`. Alternativ `BTreeMap<u64, EntryId>` mit `range(..=addr).next_back()`.

**20. Indexieren während Suche** `[W]`
- Versionierte Mapping-Tabelle (immutable Snapshot pro Batch, Pointer-Swap an Batch-Grenze); Writer hängt neue Segmente an (Append-only), Reader sehen Snapshot des Batch-Starts.

---

## E6 · Synchronisation & Echo-Vermeidung (Hochrisiko für Konsistenzmodell)

**21. `tokio::sync::watch`** `[V]`
- Hält nur den **letzten** Wert; mehrere Sends können zusammengefasst werden. Jeder Receiver verfolgt unabhängig, was er gesehen hat. `changed()` kehrt sofort zurück, wenn der aktuelle Wert „unseen" ist, sonst wartet es auf den nächsten Send; es markiert den Wert nach Abschluss als gesehen. Für Loops wird `borrow_and_update()` empfohlen, um Updates zwischen „changed bereit" und „Wert lesen" nicht zu verlieren. Quelle: https://www.rustmax.net/api/tokio/sync/watch/ · https://docs.rs/tokio/latest/tokio/sync/watch/
- „Signal vor dem Scan quittieren": `let wm = *rx.borrow_and_update();` **vor** dem Scan ausführen (markiert gesehen und liest den Stand), dann Scan über alles `> letzter Watermark`. Kommen während des Scans neue Sends, ist der Wert wieder „unseen" ⇒ nächstes `changed()` kehrt sofort zurück. Kein Verlust, nur ggf. redundanter Scan. Level-/Watermark-Semantik, kein Event-Zählen.

**22. ABA / Stamps** `[W]`
- Globaler monotoner `AtomicU64`-Zähler (`fetch_add`) vergibt Stamps bei jeder Mutation; Identität = (ID, Stamp). 64 Bit läuft praktisch nie über. Entspricht Generation-Indices/Tagged Pointers (z. B. `slotmap`, Version-Tags in lock-freien Strukturen).

**23. Seqlock / versioned read** `[V für Prinzip, Details W]`
- Writer: Zähler ungerade setzen → Daten schreiben → Zähler gerade (+2). Reader: Zähler lesen (gerade?) → Daten lesen → Zähler erneut lesen; bei Ungleichheit/ungerade wiederholen. Readers verändern keinen Speicher, Writer blockiert nicht `[V]` (https://github.com/htfy96/seqlock).
- Korrektheit im C++-Modell: Datenzugriffe müssen atomar (relaxed) oder per atomarem Byte-Memcpy erfolgen, Acquire-Fence vor dem zweiten Stamp-Read, Release beim Writer — Boehm, „Can Seqlocks Get Along with Programming Language Memory Models?", MSPC’12 `[V]`: https://unpaywall.org/10.1145%2F2247684.2247688 · Vorschlag atomic_load_per_byte_memcpy `[V]`: https://open-std.org/jtc1/sc22/wg21/docs/papers/2019/p1478r2.html · Produktions-Beispiel Abseil `[V]`: https://android.googlesource.com/platform/external/abseil-cpp/+/0740be76f23ee9f9d42f4a08552ec4722a6d8da6/absl/flags/internal/sequence_lock.h
- Rust: Reads von nicht-atomaren Daten während Schreibens sind formal UB (Data Race); mmap-Bereich daher über `AtomicU64`-Wörter oder `ptr::read_volatile` + Fences behandeln `[W]`; Crate `seqlock` existiert (Qualität `[?]`). Bei mmap + fremdem Schreibprozess kein Rust-Modell-Schutz ⇒ auf Wortebene atomar lesen.

**24. Baum mit max-Stamp pro Teilbaum + Per-Consumer-Watermark** `[W]`
- Jeder Knoten speichert `max_stamp` seines Teilbaums; Konsument besucht nur Teilbäume mit `max_stamp > watermark_c` (sub-linearer Scan). Watermark liegt **pro Konsument**.
- Gemeinsame Bitmap („dirty") verliert Änderungen: Löscht Konsument A ein Bit nach Verarbeitung, hat Konsument B es nie gesehen. Lösungen: pro-Konsument-Bitmaps, oder Stamps statt Flags (Konsument löscht nichts, merkt sich nur Watermark) — ähnlich Log-Offsets pro Consumer-Group (Kafka), LSN/Sequence-Numbers in Replikation.

**25. `changed_since(watermark)` mit Ring + Fallback** `[W]`
- Bounded Ring (Kapazität N) mit (Stamp, EntryId); Konsument fragt `changed_since(wm)`: liegt `wm` ≥ ältester Ringstamp → Delta aus Ring; sonst (Überlauf) → Vollscan über Baum/`max_stamp`. Ring in Overwrite-Variante (z. B. Disruptor-Prinzip, SPMC-Ring), Überlauf-Erkennung über „ältester verfügbarer Stamp".

---

## E7 · Residency & Ladestrategie

**26. Gather vs. Segment** `[W/?]`
- Keine allgemeingültige Dichteschwelle gefunden. Faustregel: Segmentladen gewinnt ab hoher Dichte (grob > ~10–25 % der Segmentbytes `[?]`), weil PCIe-Transfer pro Aufruf Fixkosten hat (~10 µs) und Kontiguität die Bandbreite ausreizt. Schwelle über Messung (Kosten ≈ Latenz·n_Transfers + Bytes/BW) auf der Zielkarte kalibrieren.

**27. Pinned Staging + ein DMA vs. viele kleine** `[W]`
- Gather per CPU in pinned Puffer, dann ein `cudaMemcpyAsync` ⇒ meist besser als hunderte kleiner Copies (Launch-Overhead). Alternative: GPU-Gather-Kernel aus mapped Host-Memory (Zero-Copy) bei sehr kleiner Menge.

**28. Single-Flight in Rust** `[W]`
- `tokio::sync::OnceCell::get_or_try_init` (ein Initialisierer, andere warten; bei Fehler darf der nächste erneut versuchen), `futures::future::Shared` (Fehlertyp muss `Clone` sein, gut für Fehlerverteilung an alle), Crate `async-singleflight`/`singleflight-async` `[?]`. Muster: `Mutex<HashMap<Key, Shared<BoxFuture<Result<Arc<T>, Arc<E>>>>>>`; Eintrag nach Abschluss entfernen.

**29. Eviction für VRAM** `[W]`
- Teure Reloads ⇒ LRU mit Pinning laufender Batches, TTL + Hysterese (High/Low-Watermark, gegen Thrashing), Summary-/Zentroid-Strukturen priorisiert halten (klein, hohe Trefferwirkung); Kostenbewusst: GDSF/Cost-aware LRU (Wert = Hitwahrscheinlichkeit × Reloadkosten/Größe). Unterschied zu RAM: kein transparentes Paging, Eviction ist explizit, Reload über PCIe, CoW-Spitzen.

**30. Zero-Copy aus mapped Host-Memory** `[W]`
- `cudaHostAlloc(cudaHostAllocMapped)` + `cudaHostGetDevicePointer`; UVA auf 64-Bit-Linux ⇒ Host-Pointer direkt nutzbar. Zugriffe laufen über PCIe (Latenz ~µs, Bandbreite < VRAM um ~Faktor 20–30 `[?]`), Coalescing wichtig. Sinnvoll nur für kleine, selten gelesene Daten (Flags, Zähler, wenige KB).

---

## E8 · Kernel schreibt Segmente

**31. Output-Arena mit atomarem Cursor** `[W]`
- Host vergibt Region (Offset, Kapazität); Kernel: `off = atomicAdd(&cursor, size)`; `if off + size > cap` ⇒ Überlauf-Flag setzen, nicht schreiben (Teilergebnis bleibt gültig, Host vergrößert/leert und wiederholt). Pro Warp/Block ein `atomicAdd` (aggregiert) statt pro Thread. Release-Fence/Flag nach dem Schreiben, bevor Host liest.

**32. Offset-Tabellen** `[W]`
- Variable Längen ⇒ Exklusive Prefix-Summe (z. B. Thrust/CUB `ExclusiveScan`) → Offset-Array; Lookup per Binärsuche (`partition_point`). Fixed-Stride lohnt, wenn Entry-Größen nahezu gleich (Padding-Verlust gering) — spart Tabelle und Suche.

**33. Getrennte Ringe** `[W]`
- Embedding-Ring (liefert neue Segmente) und MaxSim-Ring (liefert Hits) sind unabhängige SPSC-Pfade. „Writes lesen (Schritt 1) vor Ergebnisse auflösen (Schritt 2)": Erst werden neue Segment-Adressen/Mappings in den Snapshot übernommen, dann werden Hit-Adressen gegen diesen Snapshot aufgelöst ⇒ jede Hit-Adresse ist auflösbar, auch wenn zeitgleich indexiert wird.

---

## E9 · Addons

**34. Typsichere Erweiterungen** `[W]`
- `anymap`/`TypeMap` (`HashMap<TypeId, Box<dyn Any + Send + Sync>>`): flexibel, typsicher beim Zugriff, nicht serialisierbar ohne Zusatz (z. B. `typetag`/`erased-serde`). Extension-Traits: statisch, kein Laufzeit-Lookup, aber an Compile-Zeit gebunden. Builder: explizit, gut serialisierbar bei festem Schema. Für Serialisierung: Registry `name → (de)serialize-Funktion` oder `serde_json::Value` als Parameter-Fallback.

**35. Selbst-schedulende Tasks** `[W]`
- Continuation-Passing: Task gibt bei Abschluss Folgetasks an dieselbe Queue zurück (`Vec<Task>`); Work-Stealing (rayon/crossbeam-deque) für Lastverteilung. Keine Parent-Child-Struktur im Kern ⇒ Korrelation über ID im Payload.

**36. Cascaded Retrieval** `[V teilweise]`
- PLAID: Stufen 1 Candidate Generation (Zentroide) → 2 Centroid Pruning → 3 Centroid Interaction → 4 Dekompression + exaktes MaxSim `[V]` (https://arxiv.org/pdf/2205.09707). Allgemeines Funnel-Muster: billig/grob und breit → teuer/fein und schmal (z. B. BM25 → ColBERT-Reranking, SPLATE `[V]`: https://arxiv.org/html/2404.13950v1).

---

## Offene Punkte (Priorität)
1. **Turing-Verifikation** `ld.acquire.sys` und mapped-Atomics direkt in PTX-ISA (https://docs.nvidia.com/cuda/parallel-thread-execution/) bestätigen; Test auf der echten GTX 1660.
2. **Primary-Context-Größe** und `cudaDevAttrMemoryPoolsSupported` auf Zielsystem messen.
3. **Linux-madvise-Semantik** (man7) und `memmap2`-API gegen aktuelle Doku prüfen.
4. Gather-vs.-Segment-Schwelle empirisch messen.