# Resident – Recherche-Kontext Gruppe B (B1–B6)

Stand: 10.10.2026. Zielhardware: GTX 1660 (TU116, sm75, 6 GB).
**Kennzeichnung:** `[belegt]` = per Web-Quelle geprüft (Link am Ende des Abschnitts). `[Fachwissen]` = Standardwissen/Eigenableitung, nicht in dieser Recherche belegt → in A3/Spezifikation gegenprüfen. Versionsnummern: nur genannt, wenn in Quellen gesehen; exakte Pins gehören zu A3.

---

## B1 · Queue & Task-Primitive

### 1. arc-swap: lock-freie Liste mit CAS `[belegt]` + Bewertung `[Fachwissen]`
- `ArcSwapOption<T>` = `ArcSwapAny<Option<Arc<T>>>`; API: `load`, `load_full`, `store`, `swap`, `compare_and_swap`, `rcu`.
- `compare_and_swap(current, new)` gibt den **vorherigen Wert** (als `Guard`) zurück; Swap hat stattgefunden, wenn dieser pointer-gleich `current` ist. `current` kann `&Arc`, `Guard`, `&Guard` oder Rohzeiger sein, **kein owned `Arc`**. Vergleich ist **Pointer-Identität**, nicht Wertgleichheit.
- `rcu(|old| new)` wiederholt die Closure bei Konflikt → nur billige, wiederholbare Arbeit hinein. Teures vorher erledigen.
- **Ohne `unsafe` realisierbar:** ja (alles safe API).
- Append-Skizze:
```rust
struct Node { state: AtomicU8, payload: Mutex<Option<Data>>, next: ArcSwapOption<Node>, result: OnceLock<Result<Out, Err>> }

fn append(tail: &Arc<Node>, new: Arc<Node>) -> Result<(), Arc<Node>> {
    // CAS None -> Some(new)
    let prev = tail.next.compare_and_swap(&None::<Arc<Node>>, Some(new.clone()));
    if prev.is_none() { Ok(()) } else { Err(new) } // Konflikt: Tail weiterlaufen und erneut
}
```
- **Fallstrick (wichtig):** Einfach verkettete Liste mit reinem CAS-Unlink (`prev.next: B→C`) ist **nicht korrekt** ohne Markierung (Harris-Problem): Wird parallel B ausgehängt und C hinter B ausgehängt bzw. nach B angehängt, geht eine Änderung verloren oder ein Knoten bleibt erreichbar. Empfehlung: **logisches Löschen** über `state` (`Cancelled`/`Taken`/`Done`), physisches Aushängen nur lazy durch Traversierer und **niemals den Tail-Knoten aushängen** (Sentinel/Dummy-Tail). Alternativ Unlink unter einem kurzen Mutex (Queue-Mutation selten, Durchlauf lock-frei per `load`).
- `compare_and_swap` mit `Guard` als `current` hält Debt-Slots; Guards nicht über Blockierendes halten (`swap` kann busy-waiten, solange Guards leben).
- Quellen: arc-swap Docs (ArcSwapAny) https://docs.rs/arc-swap · Repo https://github.com/vorner/arc-swap · Changelog (Miri/Stacked-Borrows-Fixes ab 1.6.0)

### 2. Speicherfreigabe mit `Arc<Node>` `[Fachwissen]`
- Solange ein Durchläufer ein `Arc<Node>` hält, kann die Adresse nicht freigegeben und **nicht wiederverwendet** werden → klassisches **Pointer-ABA** (Adresse recycelt) entfällt für CAS auf `Arc`-Identität; Use-after-free ist ausgeschlossen. Kein Epoch/Hazard-Pointer nötig.
- Bleibende Fallstricke:
  1. **Logisches ABA**: dieselbe Identität, geänderter Zustand → Zustand über `state`-Automat sichern, nicht über Pointer.
  2. **Kettenretention**: ausgehängter Knoten hält per `next` weiter Nachfolger am Leben, solange ein Durchläufer ihn hält. Harmlos, aber Speicher bleibt bis zum Drop des Durchläufers.
  3. **Zähler-Contention**: `Arc::clone` pro Schritt = Atomic-RMW auf geteilter Cacheline; `load()` (Guard) vermeidet das, `load_full()` nicht.
  4. **Rekursiver Drop** (siehe 6).
  5. Zyklen (`prev`-Zeiger als `Arc`) → Leck; nur `next` stark, Rückverweise `Weak` oder gar nicht.

### 3. `tokio::sync::Notify` `[belegt]`
- `notify_one()`: weckt einen Wartenden; ist keiner da, wird **ein Permit gespeichert** (höchstens eins, Mehrfachaufrufe fallen zusammen).
- `notify_waiters()`: weckt **alle aktuell registrierten** `Notified`; **kein Permit**. Gemäß Docs werden Benachrichtigungen empfangen, wenn sie **nach dem Erzeugen** des `Notified`-Futures erfolgen – auch ohne vorheriges Polling/`enable()`.
- `Notified::enable()` registriert das Future für `notify_one` vor dem ersten Poll (relevant für `notify_one`-Reihenfolge); wirkt nicht auf `notify_waiters`.
- Korrektes Muster gegen verlorene Wakeups:
```rust
loop {
    let notified = notify.notified();      // 1. erzeugen (vor dem Prüfen!)
    tokio::pin!(notified);
    notified.as_mut().enable();            // 2. registrieren
    if state.load(Ordering::Acquire) == DONE { break; } // 3. Zustand prüfen
    notified.await;                        // 4. warten
}
```
- Verloren geht ein Wakeup nur, wenn `Notified` **nach** `notify_waiters()` erzeugt wird und der Zustand nicht danach geprüft wird. Zusammengefasst werden: mehrere `notify_one` ohne Wartende (1 Permit). Fusioniertes Future: nach Completion sofort wieder `Ready`.
- Quellen: https://docs.rs/tokio/latest/tokio/sync/futures/struct.Notified.html · tokio-Quelltext `sync/notify.rs` · Diskussion https://users.rust-lang.org/t/missing-notifications-with-tokio-notify-and-purpose-of-enable/137716

### 4. Set-once-Ergebnis `[belegt]` (once_cell-Vergleich) + `[Fachwissen]`
| Typ | Kontext | Mehrfach-Set | Blocking |
|---|---|---|---|
| `std::sync::OnceLock<T>` (stable seit 1.70) | sync, std | `set` → `Err(value)` beim zweiten Mal | `get_or_init`/`set` können kurz blockieren, wenn parallel initialisiert wird |
| `once_cell::sync::OnceCell<T>` (1.21.x gesehen) | sync, ältere MSRV | `set` → `Err(value)` | Docs: „assignable only once, may block the thread“ |
| `tokio::sync::OnceCell<T>` | async | `set` → `Err(SetError)`; `get_or_init` async | wartet async statt Thread-Blockade; **kein** „warte bis gesetzt“-API |
- once_cell-Doku empfiehlt `std`-Varianten, wenn diese ausreichen.
- **Empfehlung für `result`:** `std::sync::OnceLock<Result<Output, Error>>` + `Notify`/Zustand für async-Wartende. `set` genau einmal durch den Claim-Inhaber (durch Automat garantiert); `Err` beim zweiten Set als Invariantenverletzung loggen (`debug_assert!`).
- Quellen: https://docs.rs/once_cell (Abschnitt „Comparison with std“) · std-Docs `OnceLock`

### 5. Zustandsautomat `AtomicU8` `[Fachwissen]`
```rust
const QUEUED: u8 = 0; const TAKEN: u8 = 1; const DONE: u8 = 2; const CANCELLED: u8 = 3;

// claim: Queued -> Taken
state.compare_exchange(QUEUED, TAKEN, Ordering::AcqRel, Ordering::Acquire).is_ok();
// Fertigstellung: erst result setzen, dann Zustand veröffentlichen
result.set(out).ok();
state.store(DONE, Ordering::Release);      // alleiniger Eigentümer (Taken) -> Store genügt
// cancel: Queued -> Cancelled
state.compare_exchange(QUEUED, CANCELLED, Ordering::AcqRel, Ordering::Acquire);
// Leser/Waiter
state.load(Ordering::Acquire);
```
- Einzelne Übergänge ohne Schleife → **`compare_exchange` (strong)**; `compare_exchange_weak` nur in Retry-Schleifen.
- Release auf `DONE` + Acquire beim Lesen garantiert Sichtbarkeit von `result`. `SeqCst` nicht nötig.
- Fehlerfall: bei fehlgeschlagenem CAS aktuellen Wert (Rückgabe `Err(cur)`) auswerten: `Taken/Done` → Cancel abgelehnt, `Cancelled` → Claim überspringen.
- `Taken → Cancelled` bewusst **nicht** erlauben (laufende Arbeit); falls gewünscht, separates Flag `cancel_requested`.

### 6. Iterativer Drop `[Fachwissen]` (Muster aus „Learning Rust With Entirely Too Many Linked Lists“)
```rust
impl Drop for Node {
    fn drop(&mut self) {
        let mut cur = self.next.swap(None);
        while let Some(arc) = cur {
            match Arc::try_unwrap(arc) {
                Ok(mut node) => cur = node.next.swap(None), // node droppt hier mit leerem next
                Err(_shared) => break,                       // anderer Besitzer übernimmt den Rest
            }
        }
    }
}
```
- Ohne dies: Rekursion Tiefe = Kettenlänge → Stack-Overflow bei langen Ketten (Zehntausende Knoten).

### 7. Payload-Besitzübergang `Mutex<Option<Data>>` `[belegt]` + `[Fachwissen]`
- Muster: `let data = node.payload.lock().take();` (nur nach gewonnenem Claim-CAS; Lock kurz, Daten außerhalb des Locks verarbeiten).
- `std::sync::Mutex`: Poisoning (nach Panic `Err`); seit Rust 1.62 futex-basiert, in Praxis schnell. Poison behandeln mit `lock().unwrap_or_else(PoisonError::into_inner)`.
- `parking_lot::Mutex`: kein Poisoning, „eventual fairness“, kleiner; Herstellerangabe auf x86_64-Linux 1,5× schneller (unkontendiert) – seit std-Futex-Umbau deutlich geringerer Vorsprung; unter extrem hoher Thread-Zahl sind pathologische Fälle berichtet.
- Empfehlung: `parking_lot` (kein Poisoning, lock-freie Umgebung → kein Panic-Zustand im Datenfeld), alternativ std mit Poison-Recovery. Kein `await` unter diesen Locks.
- Quellen: https://docs.rs/parking_lot · https://users.rust-lang.org/t/which-mutex-to-use-parking-lot-or-std-sync/85060

---

## B2 · Region / Layout / Speicher-Modell

### 8. Generational Handles `[belegt]`
| Crate | Key-Größe | `Option<Key>` | Max. Elemente | Anmerkung |
|---|---|---|---|---|
| `slotmap` | 8 B (idx u32 + version u32) | 8 B | 2³² | typisierte Keys per Makro, DenseSlotMap/SecondaryMap |
| `thunderdome` | 8 B | 8 B (NonZero) | 2³² | schlank, keine typisierten Keys |
| `generational-arena` | 16 B | 24 B | 2⁶⁴ | größer |
| `thunderation` (Fork) | 8 B | – | – | typisierte Keys, `get_by_slot`, kein `unsafe` |
- Stale-Check = Vergleich `slot.generation == handle.generation` (ein `u32`-Vergleich plus Bounds-Check), O(1). Generationszähler erhöht sich beim Freigeben; bei Überlauf Slot stilllegen.
- **Für Region:** eigenes `struct RegionHandle { slot: u32, gen: u32 }` ist trivial und frei von Crate-Bindung; Crate nur für Slot-Verwaltung nutzen, wenn kein Offset-Bezug nötig.
- Quellen: https://docs.rs/thunderdome (Vergleichstabelle) · https://docs.rs/thunderation · https://docs.rs/slotmap

### 9. Arena-Allokator mit Free-List `[Fachwissen]`
- Muster: Offsets als `u64` in einen vom Allokator verwalteten Adressraum; Handles statt Zeiger; Free-List als `BTreeMap<u64 /*offset*/, u64 /*len*/>` (Coalescing durch Nachbarsuche) plus ggf. nach Größe geordneter Index. Bewährte Varianten: First-/Best-Fit, TLSF (O(1)), Buddy.
- Rust-Bausteine (nicht in dieser Recherche versionsgeprüft): `offset-allocator` (Port von Sebastian Aaltonens Allocator, genutzt in Bevy), `gpu-allocator`, `range-alloc`. In A3 prüfen.
- Invariante „No-Tensor/No-Pointer“: Allokator kennt nur `(offset, len, align)`; Abbildung auf Device-Pointer ausschließlich im Backend.

### 10. candle-DTypes & Layout `[belegt]`
- `candle_core::DType` (aus 0.10.2/0.11.0-Quellen): `U8, U32, I16, I32, I64, BF16, F16, F32, F64, F8E4M3` sowie dummy-Typen `F6E2M3, F6E3M2, F4, F8E8M0` (`CpuStorage` führt 14 Varianten). Nicht-erschöpfend zu behandeln; Dummy-Formate sind opake Byte-Typen.
- `Layout { shape: Shape, stride: Vec<usize>, start_offset: usize }`; **Strides und `start_offset` in Elementen**, nicht Bytes. Konstruktoren: `Layout::new(shape, stride, start_offset)`, `contiguous_with_offset`. Views (`narrow`, `transpose`, `permute`, `broadcast_as`) ändern nur Layout.
- Eigenes `Layout { shape, strides, dtype }`: Strides in Elementen oder Bytes **explizit festlegen**; für Resident (Offset/Region-basiert) Bytes empfehlenswert, Umrechnung `elems * dtype.size_bytes()` an der candle-Grenze.
- Quellen: https://docs.rs/candle-core/0.10.2/src/candle_core/dtype.rs.html · https://docs.rs/candle-core/0.11.0/src/candle_core/layout.rs.html · https://docs.rs/candle-core/0.10.2/candle_core/cpu_backend/enum.CpuStorage.html

### 11. DType-Mapping ohne Leak `[Fachwissen]` (candle-Variantenliste belegt)
```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DType { F32, F16, Bf16, U8, U32, I64 /* … */ }
impl DType { pub const fn size_bytes(self) -> usize { /* … */ } }

// nur im Adapter-Modul (feature = "candle")
impl TryFrom<DType> for candle_core::DType { type Error = UnsupportedDType; /* … */ }
impl TryFrom<candle_core::DType> for DType { type Error = UnsupportedDType; /* … */ }
// safetensors: TryFrom<safetensors::Dtype> analog (Namen weichen ab: F16/BF16/F32/U8/I64 …)

pub fn compute_dtype(dev: DeviceKind) -> DType { match dev { DeviceKind::Gpu => DType::F16, DeviceKind::Cpu => DType::F32 } }
```
- `TryFrom` statt `From`, da candle neue/Dummy-Varianten hat (`#[non_exhaustive]` Wildcard-Arm Pflicht). Öffentliche Signaturen nennen nur das eigene Enum; `candle`-Typen nur in `pub(crate)` Adaptern.
- Praxisbeispiel für gleiche Aufgabe: `mistralrs-quant` (UQFF) bildet candle-DType explizit per Match auf u32-Codes ab und sichert gegen neue Varianten ab (Kommentar „Guard against candle adding new DType variants“).

---

## B3 · Operation-/Op-Modell

### 12. Typisierte Op/IR mit `requires()` `[Fachwissen]` (nicht per Suche belegt)
```rust
pub trait Op: Send + Sync {
    fn kind(&self) -> OpKind;                 // stabiler Bezeichner
    fn params(&self) -> &OpParams;            // unveränderlich, Clone/Hash/Eq
    fn requires(&self) -> Requirements;       // CapabilitySet + optional Affinity
}
#[non_exhaustive]
pub struct Requirements { pub caps: CapabilitySet, pub affinity: Option<Affinity> }
```
- Muster aus der Praxis: Op als **Daten** (Enum oder `Arc<dyn Op>` + `params`), Scheduler matcht `requires() ⊆ worker.capabilities()` ähnlich Kubernetes-Node-Selector/Taints. Vorbild-Prinzip candle: `UnaryOp`-/`BinaryOp`-Enums beschreiben Funktionen datenhaft.
- Task ≠ Op: Task = Ausführungsauftrag (Queue-Knoten) mit `Arc<Op>` + Payload; Op bleibt reine Beschreibung.

### 13. `Value::Host | Value::Device` `[Fachwissen]`
```rust
pub enum Value { Host(HostData), Device(DeviceHandle) }   // Handle = RegionHandle/Lease, kein Tensor
```
- Analog: candle `Storage::{Cpu, Cuda, Metal}`/`Device`, burn-Backends typisiert per Generics, wgpu Buffers als Handles. Für Resident: **Enum + Konvertierungs-Methoden** (`into_host(&self, ctx)`, explizite Transfers, keine impliziten Kopien). Fallstrick: unbeabsichtigter Device→Host-Rücktransfer in Hot-Paths; deshalb Transfer als eigene Op modellieren.

### 14. Lease/Handle (RAII vs. Zähler) & Affinität `[Fachwissen]`
- **RAII bevorzugen:** `Lease { handle: RegionHandle, pin: Arc<PinCount> }`, `Drop` dekrementiert; Evict nur bei `pin == 0`; Erzeugung prüft `gen`. Manuelle Use-Zähler sind fehleranfällig (Leaks bei Early-Return/Panic).
- Fallstrick: `Drop` darf nicht blockieren/awaiten; für async Freigabe Freigabe-Queue (Channel) statt Arbeit in `Drop`.
- Affinität zukunftssicher: `Requirements.affinity: Option<Affinity>` mit `#[non_exhaustive] enum Affinity { Handle(RegionHandle), Device(DeviceId) }`; Scheduler fragt nur `requires()`, neue Varianten ändern Fassade/Task nicht.

---

## B4 · Index-Trait & -Vertrag

### 15. `tokio::sync::watch<u64>` `[belegt]`
- Kanal behält **nur den letzten Wert**; jeder `Receiver` verfolgt separat, was er gesehen hat. Anfangswert gilt als gesehen.
- `changed().await`: kehrt sofort zurück, wenn ein ungesehener Wert existiert, sonst wartet er; markiert den neuen Wert beim Abschluss als gesehen. `Err` nur wenn alle `Sender` gedroppt.
- `borrow_and_update()` (verlangt `&mut self`) liest und markiert als gesehen → **in `changed()`-Schleifen bevorzugen**; `borrow()` kann Schleife doppelt mit gleichem Wert laufen lassen.
- Zwischenwerte gehen verloren, wenn schneller gesendet als gelesen wird – für **zustandsbasierten Abgleich** (Stamp vergleichen, dann Delta laden) akzeptabel, nicht für Ereignisströme.
```rust
loop {
    let stamp = *rx.borrow_and_update();
    reconcile(stamp).await;
    if rx.changed().await.is_err() { break; }
}
```
- `wait_for(pred)` verpasst ebenfalls Zwischenwerte (Docs).
- Quelle: https://docs.rs/tokio/latest/tokio/sync/watch/index.html

### 16. mtime-Racy-Problem `[belegt]` (git) + `[Fachwissen]` (Rest)
- **git** (`racy-git.txt`): Index-Eintrag gilt als „racy“, wenn Dateizeitstempel **nicht älter** als der Zeitstempel der Index-Datei ist (Datei könnte nach dem Stat-Capturing im selben Zeitfenster geändert worden sein) → dann Inhaltsvergleich statt Stat-Vergleich. Zusätzlich wird beim Schreiben des Index dessen mtime berücksichtigt (Truncation-Trick).
- Übertragbar: **Stamp merken zusammen mit Referenzzeit T_scan**; Entry mit `mtime >= T_scan − Auflösung` ist „racy“ → Hash-Fallback (und beim nächsten Scan neu bewerten).
- Zeitauflösungen `[Fachwissen]`: ext4 Nanosekunden im Format, effektiv durch Kernel-Zeitgeber (Jiffy, ~1–4 ms) grob; NTFS 100 ns (Schreibaktualisierung teils verzögert); APFS ns; ext3/HFS+ 1 s; FAT 2 s; Netz-/FUSE-Dateisysteme beliebig grob. Konservativ: Racy-Fenster **2 s** annehmen (FAT-Worst-Case) oder FS-spezifisch.
- make: vergleicht nur mtime (keine Racy-Behandlung); rsync: `--modify-window`; Bazel: Digest-Cache mit ctime/mtime/size/Inode und Digest bei Unsicherheit `[Fachwissen]`.
- Fallback-Stamp: (size, mtime_ns, ctime/inode falls verfügbar) → bei Racy/Abweichung Content-Hash.
- Quelle: https://github.com/git/git/blob/master/Documentation/technical/racy-git.txt

### 17. Content-Hash-Fallback `[belegt]` (Vergleichsangaben Dritter) + `[Fachwissen]`
- `xxhash-rust` (xxh3, 64/128): sehr schnell; Hersteller-/Drittangaben ~15–36 GB/s/Kern (stark CPU-abhängig, ARM-Messungen dabei), nicht kryptografisch. Bevy-Diskussion: `xxhash-rust` mit Compile-Time-SIMD (SSE2/AVX2), `twox-hash` damals ohne SIMD, `xxh3`-Crate mit Runtime-Detection.
- `blake3`: kryptografisch, SIMD mit Runtime-Detection, optional Multithreading (`rayon`-Feature); Drittangaben ~3–8 GB/s/Kern, ca. 5–20× langsamer als xxh3.
- `ahash`: dokumentiert **nicht** für Dateihashing/Persistenz geeignet (kein stabiler Standard).
- **Empfehlung:** `xxh3_64` (oder `xxh3_128` bei Kollisionssorge) als Stamp-Fallback; `blake3` nur, wenn Manipulationssicherheit/Content-Adressierung nötig. Hash-Algorithmus + Version im Index-Header festhalten.
- Quellen: https://github.com/bevyengine/bevy/pull/10208 · https://docs.rs/cachekit-core (xxh3-Messwerte, Herstellerangabe) · https://docs.rs/crate/axhash/0.12.0/source/doc/benchmark.md (Vergleichswerte, Herstellerangabe)

### 18. Monotoner Stamp-Zähler `[Fachwissen]`
```rust
static NEXT: AtomicU64 = AtomicU64::new(1);
fn next_stamp() -> u64 { NEXT.fetch_add(1, Ordering::Relaxed) }
```
- Eindeutigkeit genügt `Relaxed`; **Sichtbarkeit der Daten** zum Stamp über Release-Store/Lock beim Veröffentlichen regeln (Stamp wird innerhalb der Schreib-Serialisierung vergeben).
- Entfernen + Wiederanlegen derselben ID bekommt **neuen** Stamp → Vergleich nur per `!=` (keine Ordnungsannahme). `u64` läuft praktisch nie über.
- Über Neustarts: High-Water-Mark persistieren (Index-Header) und beim Laden `max(stored, file_max) + 1`.

### 19. Parquet/Arrow Spaltenlayout `[belegt]` + `[Fachwissen]`
- `parquet::arrow::arrow_reader::ArrowReaderBuilder`: `with_projection(ProjectionMask::leaves(&schema_desc, [idx]))` liest **nur die gewählten Spalten**; Spaltenchunks sind in Parquet pro Row-Group getrennt gespeichert → Vektorspalte `FixedSizeList<f16,384>` wird beim Stamp-Scan nicht gelesen. Weitere Hebel: `with_row_groups` (Row-Group-Pruning), `with_row_selection` (Page-Pruning), Page-Index (Schreiben und Lesen aktivieren, laut Docs empfohlen), `StatisticsConverter`.
- Fallstrick `[Fachwissen]`: Leaf-Indizes bei verschachtelten Spalten (FixedSizeList) – Maske über Spaltenpfad ermitteln, nicht über Arrow-Feldindex. Float16-Unterstützung in der eingesetzten `parquet`-Version in A3 prüfen.
- mmap: Reader benötigt `ChunkReader` (z. B. `bytes::Bytes`); mit `memmap2` gemappte Daten als `Bytes` einhängen und nur Metadaten + Stamp-Chunk anfassen; ungelesene Pages werden vom OS nicht eingepaged.
- **Robusteste Alternative:** Stamp-Spalte in **separater Datei/Sidecar** (kleines Parquet/Arrow-IPC) – garantiert null Vektor-IO, entkoppelt Rewrites.
- Quellen: https://arrow.apache.org/rust/parquet/arrow/arrow_reader/struct.ArrowReaderBuilder.html · https://docs.rs/parquet

### 20. Mehrschreiber-Serialisierung `[Fachwissen]`
| Muster | Vorteil | Nachteil |
|---|---|---|
| **Single-Writer-Task + `mpsc` + `oneshot`-Antwort** | klare Ordnung, Stamp = Reihenfolge, Backpressure | Task-/Channel-Overhead |
| `Mutex` (sync, kurz) um `apply` | einfach | blockiert Aufrufer; nie über `.await` halten (sonst `tokio::sync::Mutex`) |
| Command-Log + Replay | persistierbar | Aufwand |
- Empfehlung: Single-Writer-Task; `apply(cmd) -> oneshot::Receiver<Stamp>`; Last-Writer-Wins = Dequeue-Reihenfolge; Stamp unmittelbar vor Veröffentlichung vergeben; danach `watch::Sender::send(stamp)`.

---

## B5 · Kernel-Abstraktion

### 21. cudarc-Launch & Kernel-Trait `[belegt]` + `[Fachwissen]` (Trait-Entwurf)
- cudarc 0.19.x (zuletzt gesehen 0.19.9): `CudaContext::new(ordinal)` → `ctx.default_stream()` / `ctx.new_stream()`; Modul via `nvrtc::compile_ptx(src)` → `ctx.load_module(ptx)` → `module.load_function("name")`; Start:
```rust
let mut b = stream.launch_builder(&func);       // PushKernelArg
b.arg(&mut out).arg(&inp).arg(&n);
unsafe { b.launch(LaunchConfig::for_num_elems(n as u32)) }?;   // unsafe
```
- Daten: `CudaSlice<T>`, `CudaView/CudaViewMut` (Sub-Ranges), `stream.clone_htod/memcpy_htod/clone_dtoh/alloc_zeros`; `stream.synchronize()`; Events (`CudaEvent`) für Fortschritt. `LaunchConfig { grid_dim, block_dim, shared_mem_bytes }`. candle nutzt dieselbe cudarc-Basis (`ug-cuda`), d. h. Stream-/Kontext-Gleichheit beachten.
- Trait-Skizze (real + Mock):
```rust
pub trait Kernel: Send + Sync {
    fn begin(&self, batch: &BatchDesc) -> Result<BatchId>;
    fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput>;
    fn progress(&self, batch: BatchId) -> u64;                 // monoton, Atomic/Event
    fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>>; // geschriebene Bereiche (offset,len)
}
```
- Adress-/Offset-Agnostik: Trait nimmt **Spans (offset,len)**; Pointer-Auflösung nur in `CudaKernel`.
- Fallstrick: Kernel-Args müssen `DeviceRepr` sein; Lebensdauer der Slices bis `synchronize`; Fehler aus Kernel erscheinen asynchron bei späterem Sync.
- Quellen: https://docs.rs/crate/cudarc/latest · https://docs.rs/crate/cudarc/0.19.3/source/src/driver/safe/launch.rs · https://docs.rs/crate/cudarc/0.19.3/source/src/driver/mod.rs

### 22. Offset-Tabellen/Prefix-Summen `[Fachwissen]`
```rust
// offsets.len() == n + 1; Entry i belegt [offsets[i], offsets[i+1])
fn entry_of(offsets: &[u64], addr: u64) -> Option<usize> {
    let i = offsets.partition_point(|&o| o <= addr);   // erstes o > addr
    (i > 0 && i < offsets.len()).then(|| i - 1)
}
```
- Aufbau per `scan` (exklusive Präfixsumme) in O(n); Binärsuche O(log n) via `slice::partition_point`. Für dynamische Änderungen `BTreeMap<u64, EntryId>` (`range(..=addr).next_back()`); Intervall-Crates (`rangemap`, `intervaltree`) nicht geprüft.
- `Variable{offsets}` + `Fixed{stride}` als Enum: bei fester Länge reine Division.

### 23. Mock-Kernel `[Fachwissen]`
- Handgeschriebenes Fake statt Mock-Framework: deterministische Ausgabe als reine Funktion der Eingabe (z. B. Hash der Bytes), Schreibbereiche aus Eingabe berechnet, Fortschritt als `AtomicU64`, optional Fehlerinjektion (`fail_after(n)`), Aufruf-Log (`Mutex<Vec<Call>>`) für Assertions. `mockall` nur für Interaktions-Tests.
- Trait-Generics (`fn run<K: Kernel>`) statt `dyn` vermeidet Dispatch-Kosten, `Arc<dyn Kernel>` für Laufzeitwahl. Feature `cuda` trennt reale Implementierung; CI läuft ohne GPU.

---

## B6 · CapabilitySet & Profile-Typen

### 24. Präzisionspfade GTX 1660 (TU116, sm75) `[belegt]` + `[Fachwissen]`
- **Keine Tensor Cores, keine RT-Cores** (TU116); stattdessen dedizierte FP16-Einheiten: **FP16 mit doppelter FP32-Rate** (~10–10,9 TFLOPS FP16 vs. ~5 TFLOPS FP32, je nach Modell 1660/1660 Ti). Fp16-GEMM läuft daher auf CUDA-Cores, nicht per `wmma`/Tensor-Core-Pfaden (HGEMM-Tensor-Variante dort ohne Vorteil).
- sm75 unterstützt `dp4a` (ab sm61) → int8-Skalarprodukte auf CUDA-Cores; **keine** int8-Tensor-Core-Pfade. bf16 nativ **nicht** (bf16-Rechnen ab sm80) → f16 wählen. fp8/fp4 nicht nutzbar.
- candle: `F16`-CUDA-Pfade vorhanden (cuBLAS), `BF16` auf sm75 ohne native Unterstützung problematisch; quantisierte Kernel (ggml-Portierung) nutzen `dp4a` `[Fachwissen]`. cudarc: Wrapper für cuBLAS/cuBLASLt, NVRTC; gewünschte `sm_75`-Arch beim NVRTC-Compile (`arch`-Option) setzen.
- 6 GB VRAM: f16 halbiert Bedarf gegenüber f32; Policy GPU f16 / CPU f32 passt.
- Quellen: https://www.tomshardware.com/reviews/nvidia-geforce-gtx-1660-turing-tu116,6027.html · https://www.anandtech.com/show/13973/nvidia-gtx-1660-ti-review-feat-evga-xc-gaming/2 · https://www.igorslab.de/en/turing-light-nvidia-geforce-gtx-1660-ti-launch-with-the-msi-gtx-1060-ti-gaming-x-and-gtx-1660-ti-ventus-xs/

### 25. Erweiterbare Capability-Modellierung `[Fachwissen]`
```rust
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)] pub struct AxisId(Arc<str>);   // oder interniert (u32)
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)] pub struct ValueId(Arc<str>);

#[derive(Clone, Default)]
pub struct CapabilitySet(BTreeMap<AxisId, BTreeSet<ValueId>>);
impl CapabilitySet {
    pub fn satisfies(&self, req: &CapabilitySet) -> bool {     // req ⊆ self, achsenweise
        req.0.iter().all(|(a, vs)| self.0.get(a).map_or(false, |have| vs.is_subset(have)))
    }
}
```
- Neue Achsen/Werte = neue Strings, **keine Änderung an Fassade/Task**. Typsicherheit nachrüsten per Newtype-Konstanten (`pub const PRECISION: AxisId = …` über `LazyLock`) oder Registry mit Validierung.
- `enum_dispatch` löst **geschlossene** Varianten-Dispatch-Probleme (Compile-Zeit-Enum) und ist für offene Erweiterbarkeit ungeeignet. Alternative bei Typisierung: Achsen als Traits mit assoziiertem Wert + `dyn Any`-Registry (komplexer).
- Serialisierung: BTreeMap → stabil geordnet (reproduzierbare Hashes/Profile).

### 26. Pareto-Front & normierte Position `p ∈ [0,1]` `[Fachwissen]` (keine geprüfte Crate)
- Front (minimiere Zeit `t` und Abweichung `d`): nach `t` aufsteigend sortieren, nur Punkte mit strikt fallendem `d` behalten – O(n log n).
```rust
fn pareto(mut pts: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    pts.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    let mut out: Vec<(f64, f64)> = Vec::new();
    for p in pts { if out.last().map_or(true, |l| p.1 < l.1) { out.push(p); } }
    out
}
// Position: Achsen min-max normieren, kumulierte Bogenlänge s_i; p = s(proj(x)) / s_total
```
- Position eines Messpunkts: auf nächstes Polyline-Segment projizieren, `p = (s_i + t_seg·len_i) / S_total`. Entartungsfälle: Front mit 1 Punkt (`p = 0`), konstante Achse (Division durch 0 abfangen), Rauschen (Glättung/Monotonie erzwingen).
- Crates (`pareto_front`, `moo-*`) nicht verifiziert; Eigenimplementierung (~50 Zeilen, `f64::total_cmp`) ist unkritisch.

---

## Offene Punkte für A3 / Spezifikation
1. Exakte Versionen/MSRV aller Crates (arc-swap, tokio, parking_lot, slotmap/thunderdome, candle 0.10.x/0.11.x, cudarc 0.19.x, parquet/arrow) festlegen.
2. `parquet` ↔ Float16/FixedSizeList-Unterstützung und Leaf-Index-Ermittlung am Testfall prüfen.
3. Lock-freier Unlink: Entscheidung logisches Löschen + Dummy-Tail (Empfehlung) vs. Mutex-geschützte Mutation.
4. candle-Quantkernel/`dp4a`-Nutzung und bf16-Verhalten auf sm75 praktisch testen.
5. Racy-Fenster (Dateisystem-Auflösung) und Hash-Algorithmus (xxh3_64 vs. 128) festlegen.