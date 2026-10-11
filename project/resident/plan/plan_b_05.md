# Umsetzungsplan B5 · Kernel-Abstraktion

| | |
|---|---|
| **Paket** | B5 (Gruppe B) |
| **Abhängt von** | B2 (`Region`/Spans) |
| **Blockiert** | D6 (Kernels & Scorer), E4 (Manager-Disziplin), E5 (Kommunikationsslot), E8 (Kernel schreibt Segmente) |
| **Ergebnis** | Kernel-Trait + Mock-Vertrag: `SegmentDesc`, `SegmentLayout`, `KernelHit`, Kernel-Trait (Batch-Start/-Ende, Ergebnisse, geschriebene Bereiche, Fortschritt) + Mock |
| **Quellen** | `../resident_index.md` (Kernel-Schnittstelle, Rückabbildung), `../resident_subengine.md` §4.5, `../context/context_b.md` §B5 (21–23) |

---

## 1. Zweck & Abgrenzung

Der **Kernel** kennt nur **Segmente, Adressen und Offsets** — keine Entry-Identität, keine Traversierung, keine Kommunikation während der Berechnung. Er liefert `(Adresse/Offset, Score)` bzw. geschriebene Bereiche. Die **Rückabbildung Adresse → Entry** macht der **Manager** (E), nicht der Kernel.

B5 liefert eine **adress-/offset-agnostische** Trait-Abstraktion plus einen **handgeschriebenen Mock** für Tests ohne GPU. Reale CUDA-Implementierung (cudarc/NVRTC) ist D6/E hinter `feature = "cuda"`.

**Namensklärung:** B5 benutzt `SegmentLayout` (nicht `Layout`, das ist B2) für `Fixed{stride}|Variable{offsets}`.

---

## 2. Zu implementierende Typen

```rust
pub enum SegmentLayout {
    Fixed    { stride: u32 },        // Entry-Grenzen per Arithmetik
    Variable { offsets: OffsetRef }, // Offset-Tabelle (Prefix-Summen), vom Manager gebaut
}

pub struct SegmentDesc { pub base: u64 /* Arena-Offset */, pub count: u32, pub layout: SegmentLayout }
pub struct KernelHit   { pub offset: u64 /* Arena-Offset */, pub score: f32 }
pub struct WrittenRange { pub offset: u64, pub len: u64 }
pub struct RegionSpan   { pub offset: u64, pub len: u64 } // adress-agnostisch; Pointer-Auflösung NUR im Backend
```
- `OffsetRef` ist im Logik-Build ein Host-Slice/Region-Handle; im `cuda`-Build ein Device-Pointer-Äquivalent. B5 bleibt agnostisch (Trait nimmt **Spans**, keine Pointer).
- Eine **Kandidatenmenge** ist ein `Variable`-Layout; der Kernel rechnet darüber, egal ob die Tabelle auf kompakte Kopien oder bestehende Segmente zeigt.

## 3. Kernel-Trait (real + Mock)

```rust
pub trait Kernel: Send + Sync {
    fn begin(&self, batch: &BatchDesc) -> Result<BatchId, Error>;
    fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput, Error>;
    fn progress(&self, batch: BatchId) -> u64;                   // monoton; Atomic/Event
    fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, Error>; // geschriebene Bereiche
}

pub struct BatchDesc { /* Segmente (SegmentDesc), Query-Batch, ParamBlock-Verweis, Zeitbudget */ }
pub struct BatchId(u64);
pub enum KernelOutput { Hits(Vec<KernelHit>), Partial /* Bereich erschöpft, Rest folgt */ }
```
- **Zwei Kernel-Rollen** (beide über denselben Trait):
  - **Scoring (MaxSim):** `run` liefert `Hits`. MaxSim aggregiert pro Entry über `stride`/Offset-Tabelle, **ohne** Entry-Identität.
  - **Indexaufbau (Embedding):** Kernel schreibt per **Cursor** in einen vorab vergebenen Bereich und **allokiert nie**; bei Bereichsmangel `Partial` → Manager vergibt nächsten Bereich. `end` meldet belegte Bereiche.
- **Fortschritt:** `progress` monoton (Atomic im Logik-Mock; im Ringbuffer-Modus Zähler in gemapptem Host-Speicher — H1/E).
- **Keine Kommunikation während der Berechnung**; Mutation nur im Kommunikationsslot (E5).

## 4. Rückabbildung-Helfer (Zuständigkeit: Manager)

B5 stellt die reine Funktion bereit, die E8 nutzt (Kernel selbst traversiert nie):
```rust
// Fixed: addr → index per Division (addr - base) / stride
// Variable: offsets.len() == n+1; Entry i belegt [offsets[i], offsets[i+1])
fn entry_of(offsets: &[u64], addr: u64) -> Option<usize> {
    let i = offsets.partition_point(|&o| o <= addr); // erstes o > addr
    (i > 0 && i < offsets.len()).then(|| i - 1)
}
```
- Aufbau per exklusiver Präfixsumme (`scan`) in O(n); Suche O(log n) via `slice::partition_point`. Dynamisch alternativ `BTreeMap<u64, EntryId>` (`range(..=addr).next_back()`).

## 5. cudarc-Bezug (nur Doku für D6, hinter `feature = "cuda"`)

- cudarc 0.19.x: `CudaContext::new(ordinal)` → `default_stream()`/`new_stream()`; `nvrtc::compile_ptx` → `load_module` → `load_function`; `stream.launch_builder(&func).arg(..).launch(cfg)` (`unsafe`). Daten `CudaSlice`/`CudaView(Mut)`; `clone_htod`/`memcpy_htod`/`clone_dtoh`; Events für Fortschritt; `LaunchConfig{grid_dim,block_dim,shared_mem_bytes}`.
- **Kritisch:** candle nutzt dieselbe cudarc-Basis → **Stream-/Kontext-Gleichheit** beachten (A3-Prototyp). Kernel-Args müssen `DeviceRepr` sein; Slice-Lebensdauer bis `synchronize`; Kernel-Fehler erscheinen **asynchron** beim späteren Sync.
- `sm_75` als `--gpu-architecture` beim NVRTC-Compile.

## 6. Mock-Kernel (Pflicht, CI ohne GPU)

Handgeschriebenes Fake statt Mock-Framework:
- Deterministische Ausgabe als **reine Funktion der Eingabe** (z. B. xxh3 der Bytes → Score).
- Geschriebene Bereiche aus der Eingabe berechnet; `Partial` simulierbar (Bereich künstlich knapp).
- Fortschritt als `AtomicU64`; Fehlerinjektion `fail_after(n)`; Aufruf-Log `Mutex<Vec<Call>>` für Assertions.
- Trait-Generics (`fn run<K: Kernel>`) vermeiden Dispatch-Kosten; `Arc<dyn Kernel>` für Laufzeitwahl. `mockall` nur für reine Interaktions-Tests.

## 7. Fehler & Logging

- Q1: `TransferFailed` (H2D/D2H), `ArenaExhausted` (Bereichsmangel → aber Default ist `Partial`, nicht Fehler), Kernel-/Launch-Fehler.
- Q2: `tracing`-Span pro Batch (`BatchId`): begin/run/end, Hits-Zahl, WrittenRanges, Partial-Ereignisse.

## 8. Teststrategie (Q3, ohne GPU)

- **entry_of:** Fixed (Division) und Variable (partition_point) inkl. Rändern (addr == base, letzter Offset, out-of-range → None).
- **Mock-Determinismus:** gleiche Eingabe → gleiche Hits/Ranges.
- **Partial-Pfad:** knapper Bereich → `Partial`; Manager vergibt Folgebereich; Summe der WrittenRanges deckt Eingabe.
- **Fortschritt:** `progress` monoton steigend über `run`.
- **Fehlerinjektion:** `fail_after(n)` liefert `Err`; Batch sauber beendbar.
- **Adress-Agnostik:** Trait-API nennt keine Pointer; Spans genügen (Compile-Check).

## 9. Offene Punkte (→ A3 / D6 / E)

1. `OffsetRef` im `cuda`-Build (Device-Pointer) vs. Logik-Build (Host-Slice) — Abstraktion final.
2. MaxSim: Kachel-Kette (Candle-Ops) vs. fused NVRTC-Kernel messen (D6/A3-Prototyp).
3. cudarc-Version == candle-interne cudarc-Version (A3, sonst inkompatible Typen).
4. Rückabbildung dynamisch: `partition_point` (statisch) vs. `BTreeMap`/Intervall-Map (bei laufenden Änderungen).

## 10. Definition of Done

- `SegmentDesc`, `SegmentLayout`, `KernelHit`, `WrittenRange`, `RegionSpan`, `Kernel`-Trait, `entry_of`-Helfer implementiert.
- `MockKernel` mit Determinismus, Partial, Fortschritt, Fehlerinjektion, Aufruf-Log.
- Trait ist adress-/offset-agnostisch (keine Pointer in öffentlicher API); Rückabbildung als Manager-Aufgabe dokumentiert.
- Testmatrix §8 grün; baut ohne `cuda`/`candle`.
