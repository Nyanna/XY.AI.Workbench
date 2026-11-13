# Spezifikation: Write-Pointer-Bereich (Modul `write_pointer_region`, Build-Order 128)

## 1. Task, restrictions and boundaries

**Task.** Das Modul vergibt Byte-Claims aus einem Bereich fester Kapazität über einen atomaren Write-Pointer.
- Es hat eine Host-Implementierung für einzelne Claims und Gruppen-Claims.
- Es liefert den Device-Quelltext für Warp-Claims: Prefix-Sum über 32 Threads, eine Atomic-Aktualisierung je Warp.
- Es macht das Ergebnis für den Host sichtbar.
- Es meldet bei Überlauf ein Teilergebnis (`Partial`), damit der Manager den nächsten Bereich vergibt.

**Nicht Teil des Moduls:**
- Allokation von Bereichen und Vergabe des Folgebereichs (Manager).
- Auflösung von `RegionHandle` auf Device-Adressen (Memory-Backend).
- Kernel-Start, Doorbell, Event.
- Schreiben der Nutzdaten. Das Modul vergibt nur Offsets.
- Wiederholung oder Retry.

Es gibt keine Panics bei ungültigen Eingaben.

## 2. Context

- Ein Bereich ist durch ein `RegionHandle` (Arena, Offset, Länge, Generation) beschrieben. Seine Länge ist die Kapazität.
- Erzeuger-Kernels, etwa Ergebnis-Writer, rufen die Device-Funktionen `wp_claim` oder `wp_claim_warp` auf. Sie binden `WRITE_POINTER_DEVICE_SRC` in ihren JIT-Quelltext ein (`kernel_register`, Prelude des GPU-Kernel-Moduls). Das Modul startet selbst keinen Kernel.
- Der Zustand (`WriteState`, 32 Byte) liegt in host-sichtbarem Speicher, dessen Adresse das Backend liefert. Der Aufrufer übergibt dem Kernel diese Adresse in seinen eigenen Kernel-Parametern.
- Nach dem Abschluss des Batches liest der Host den Zustand. Der Abschluss wird über `Doorbell::poll` (`completed >= Batch-Nummer`) oder `Event::wait` festgestellt. Der Kernel inkrementiert `completed` mit Release, daher sind alle Atomics des Batches vorher sichtbar.
- Der Manager erhält `WriteResult`. Bei `Partial` vergibt er den nächsten Bereich und setzt dort ab `valid_len` fort. Das Modul macht das nicht selbst.

## 3. Interface

```rust
use region_handle::RegionHandle;
use std::sync::atomic::{AtomicU32, AtomicU64};

pub const INVALID_OFFSET: u64 = u64::MAX;  // Invalid-Marker eines gescheiterten Claims
pub const NO_OVERFLOW: u64 = u64::MAX;     // Anfangswert von overflow_min
pub const WARP_SIZE: u32 = 32;
pub const STATE_SIZE: usize = 32;

/// Zustand; repr(C), 32 Byte, 8-aligned. Device und Host nutzen dasselbe Layout.
#[repr(C)]
pub struct WriteState {
    ptr: AtomicU64,           // Byte 0: Write-Pointer (nur wachsend)
    overflow_min: AtomicU64,  // Byte 8: kleinste Overflow-Position (Grenze), Start NO_OVERFLOW
    capacity: u64,            // Byte 16: Kapazität in Bytes (nach Init konstant)
    overflow_flag: AtomicU32, // Byte 24: 0/1
    logged: AtomicU32,        // Byte 28: 0/1, Overflow wurde geloggt
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateSnapshot { pub ptr: u64, pub overflow_min: u64, pub capacity: u64, pub overflow: bool }

impl StateSnapshot {
    /// min(overflow_min, ptr, capacity)
    pub fn valid_len(&self) -> u64;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Claim { Ok { offset: u64 }, Invalid }   // Invalid entspricht INVALID_OFFSET

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimError { LengthMismatch { lens: usize, out: usize } }

impl WriteState {
    pub fn new(capacity: u64) -> WriteState;
    /// 32-Byte-Abbild (little endian) für Device-Init: ptr 0, overflow_min NO_OVERFLOW, Flags 0.
    pub fn init_image(capacity: u64) -> [u8; STATE_SIZE];
    /// Liest ein vom Device zurückkopiertes Abbild. Ergibt kein Live-Objekt.
    pub fn snapshot_from_image(image: &[u8; STATE_SIZE]) -> StateSnapshot;

    /// Einzel-Claim. Send + Sync-sicher, lock-frei.
    pub fn claim(&self, len: u64) -> Claim;
    /// Gruppen-Claim: eine einzige Pointer-Aktualisierung für alle `lens`.
    /// Schreibt je Mitglied den Offset oder INVALID_OFFSET nach `out`.
    pub fn claim_group(&self, lens: &[u64], out: &mut [u64]) -> Result<(), ClaimError>;

    /// Acquire-Lesung.
    pub fn snapshot(&self) -> StateSnapshot;
    /// Setzt den Zustand zurück (exklusiv, daher &mut).
    pub fn reset(&mut self);
}

/// Device-Quelltext (CUDA C++), vom Aufrufer in JIT-Bodies eingebunden.
/// Enthält struct WriteState (gleiches Layout), `wp_claim` und `wp_claim_warp`.
pub const WRITE_POINTER_DEVICE_SRC: &str;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteResult {
    /// Kein Overflow; `len` = gültige Bytes (= ptr).
    Complete { region: RegionHandle, len: u64 },
    /// Overflow: Präfix [0, valid_len) ist gültig, der Rest muss im nächsten Bereich vergeben werden.
    Partial { region: RegionHandle, valid_len: u64, overflow_pos: u64 },
}
impl WriteResult {
    pub fn from_snapshot(region: RegionHandle, s: &StateSnapshot) -> WriteResult;
    pub fn valid_len(&self) -> u64;
}

/// Host-Bereich: RegionHandle + Zustand. capacity = region.len().
pub struct WritePointerRegion { /* region: RegionHandle, state: WriteState */ }
impl WritePointerRegion {
    pub fn new(region: RegionHandle) -> WritePointerRegion;
    pub fn region(&self) -> RegionHandle;
    pub fn state(&self) -> &WriteState;
    pub fn result(&self) -> WriteResult;   // aus state().snapshot()
}
```

`WriteState`, `WritePointerRegion`, `WriteResult` und `StateSnapshot` sind `Send + Sync + 'static`.

## 4. Behavior and invariants

**Einzel-Claim (Host und Device identisch).**
- `old = ptr.fetch_add(len)`. Der Claim gelingt genau dann, wenn `old.checked_add(len)` existiert und `old + len <= capacity`.
- Bei Erfolg ist das Ergebnis `Ok{offset: old}`. Das Modul schreibt nichts in den Bereich.
- Ein Claim mit `len == 0` gelingt, wenn `old <= capacity`.
- Bei Misserfolg gilt:
  - Das Ergebnis ist `Invalid` (Device: `INVALID_OFFSET`).
  - `overflow_flag` wird auf 1 gesetzt.
  - `overflow_min = min(overflow_min, old)` per Atomic-Min.
  - Der Pointer bleibt vorangeschoben und wird nicht zurückgesetzt.
- Der Pointer schiebt bei jedem Claim voran, auch bei einem gescheiterten.
- Scheitert ein Claim, scheitern alle später ausgeführten (in Pointer-Reihenfolge), weil ihr `old` mindestens `old + len` des gescheiterten Claims ist. Die erfolgreichen Claims bilden damit ein zusammenhängendes Präfix `[0, valid_len)` ohne Lücken.
- Der Pointer darf `capacity` überschreiten. Der Bereich selbst wird nie überschritten, weil `valid_len` auf `capacity` begrenzt ist.
- Ein Pointer-Überlauf über `u64` wird als Overflow behandelt (`checked_add`, also `Invalid`). Der Pointer wird mit wrapping `fetch_add` fortgeschrieben. Falls er dabei wrappt, gilt `overflow_min <= alter Wert`, und `valid_len` bleibt begrenzt.

**Gruppen-Claim.**
- `total = Σ lens`, mit Sättigung bei `u64::MAX`. Es gibt eine einzige Aktualisierung `base = ptr.fetch_add(total)`.
- Mitglied `i` hat `offset_i = base + Σ_{j<i} lens[j]` (exklusiver Prefix) und gelingt genau dann, wenn `offset_i + lens[i] <= capacity` ohne Überlauf.
- Gescheiterte Mitglieder erhalten `INVALID_OFFSET`.
- Das erste gescheiterte Mitglied `k` setzt `overflow_flag` und `overflow_min = min(overflow_min, offset_k)`. Alle späteren Mitglieder scheitern ebenfalls.
- Ist `lens.len() != out.len()`, ergibt das `LengthMismatch`. Der Zustand bleibt dann unverändert.
- Eine leere Gruppe ändert nichts und gibt `Ok(())` zurück.

**Gültige Länge.** `valid_len = min(overflow_min, ptr, capacity)`. Ohne Overflow ist das `min(ptr, capacity)`, und wegen der Erfolgsbedingung gilt `ptr <= capacity`.

**Erstes Overflow-Logging.**
- Nur der Aufruf, der `logged` von 0 auf 1 setzt (Compare-Exchange), loggt: auf dem Host genau einmal mit `log::warn!`, mit capacity, Grenze und Claim-Länge.
- Auf dem Device wird `logged` im selben Schritt gesetzt, ohne Ausgabe.
- `WritePointerRegion::result()` loggt bei gesetztem `overflow_flag` und `logged == 0` einmalig und setzt `logged`.
- Pro Zustand erscheint höchstens ein Log-Eintrag.

**Device (`WRITE_POINTER_DEVICE_SRC`).**
- `wp_claim(WriteState*, len)` ist die Einzel-Variante mit `atomicAdd` und `atomicMin` (u64).
- `wp_claim_warp(WriteState*, len)` gilt für die aktiven Lanes eines Warps (`__activemask`).
  - Die Lanes bilden per `__shfl_up_sync` einen Inklusiv-Prefix-Sum und daraus den exklusiven Prefix. Lanes mit `len == 0` nehmen teil.
  - Die letzte aktive Lane führt mit der Warp-Summe genau ein `atomicAdd` auf `ptr` aus. Pro Warp gibt es keine weitere Pointer-Atomic.
  - `base` wird per `__shfl_sync` verteilt. Die Semantik je Lane ist die des Gruppen-Claims (`base + exklusiver Prefix`, Erfolgsbedingung wie oben).
  - Nur die Lane mit dem kleinsten Index unter den gescheiterten Lanes setzt Flag und `atomicMin(overflow_min)` mit ihrem Offset.
  - Das Ergebnis ist je Lane der Offset oder `INVALID_OFFSET`.
- Die Atomics sind relaxed. Die Sichtbarkeit für den Host kommt aus dem Release-Inkrement von `completed` am Batch-Ende, das der Aufrufer-Kernel nach dem GPU-Kernel-Modul ausführt (siehe Abschnitt 5).
- Dynamischer Shared Memory wird nicht verwendet.

**Sichtbarkeit für den Host.**
- Der Host liest erst nach dem Abschluss-Signal des Batches (Doorbell oder Event).
- `snapshot()` nutzt Acquire-Loads.
- `snapshot_from_image` liest ein Device-Abbild: `ptr` @0, `overflow_min` @8, `capacity` @16, `overflow_flag` @24 (little endian). Layout und Offsets sind per Test prüfbar (`size_of == 32`, `align_of == 8`).

**Teilergebnis.**
- `from_snapshot` ergibt `Complete{region, len: valid_len}`, wenn `overflow == false`.
- Sonst ergibt es `Partial{region, valid_len, overflow_pos: overflow_min}`.
- `Partial` bedeutet: Bytes `[0, valid_len)` sind gültig, es wurde nichts darüber hinaus geschrieben. Der Manager vergibt den nächsten Bereich.
- Das Modul ändert die Generation des Handles nicht und gibt das übergebene `RegionHandle` unverändert zurück.

**Nebenläufigkeit.**
- Alle `&self`-Methoden sind lock-frei und aus beliebig vielen Threads aufrufbar. Das Ergebnis entspricht einer Serialisierung in Pointer-Reihenfolge.
- `reset(&mut self)` setzt `ptr = 0`, `overflow_min = NO_OVERFLOW` und beide Flags auf 0. Es ist erst nach der Quittierung des Batches erlaubt.
- Das Modul hat keine globalen Variablen.

**Tests.**
- Einzel-Claims bis zum Überlauf: Das Präfix ist lückenlos, spätere Claims sind `Invalid`.
- Ein Claim, der exakt `capacity` füllt, gelingt.
- Ein Gruppen-Claim, der mitten in der Gruppe überläuft: Die ersten Mitglieder sind gültig, `overflow_min` ist der Offset des ersten ungültigen Mitglieds.
- Mehrere Threads claimen parallel: Die Intervalle sind disjunkt.
- Das Layout ist korrekt (32 Byte, Offsets).
- Es gibt genau einen Log-Eintrag.
- `valid_len` ist das Minimum der drei Werte.
- Eine Quelle ohne `extern __shared__`.

## 5. Relied-on elements

- `region_handle::RegionHandle`: `Copy + Eq`, `len() -> u64` (= Kapazität), Rückgabe unverändert. Es gibt keine Auflösung auf Device-Adressen.
- GPU-Kernel:
  - Die Kernel-Quellen laufen über `kernel_register` (JIT-Prelude `T`, `load_f`, `warp_reduce_sum`, `warp_reduce_max`, `DIM`). Block 128 Threads, also 4 Warps, und kein dynamischer Shared Memory.
  - Der letzte Block eines Kernels erhöht `completed` per `atom.add.release.sys.u64`. Vorher geschriebene Atomics und Outputs sind damit sichtbar, sobald der Host `Doorbell::poll` mit `last >= Batch-Nummer` sieht oder `Event::wait` zurückkehrt.
  - Der Host schreibt Parameter und Ergebnisse erst danach neu und quittiert per `Gap::close`.
  - Dieses Modul führt keinen eigenen Launch aus und ändert nichts an `KernelParams`.