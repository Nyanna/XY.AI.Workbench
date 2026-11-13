# Spezifikation: Page-Cache-Hinweise und blockierende Lesezugriffe (`page_advice`, Build-Order 7)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe**
- Advice (`WillNeed`, `Sequential`, `Random`, `Cold`) je Byte-Bereich unverbindlich setzen.
- Bereiche prüfen.
- Blockierende Lesezugriffe im Blocking-Pool der Async-Runtime (tokio) ausführen.
- Abbruch oder Panic als Fehler melden.

**Grenzen**
- Keine Abhängigkeiten zu anderen Modulen.
- Kein Mapping-Lebenszyklus: Das Modul öffnet oder erzeugt keine Mappings. `MmapAdvisor` umhüllt ein vorhandenes `memmap2::Mmap`.
- Kein Caching, keine Lese-Semantik. Der Lesevorgang ist eine vom Aufrufer gelieferte Closure.
- Keine Retries, keine Verwerfung von Seiten-Inhalten.
- Alle Hinweise sind unverbindlich.

**Crates:** `tokio` (rt), `memmap2`, `libc`, `tracing`, `thiserror`.

## 2. Kontext

Das Modul liegt unter den Lese-Modulen. Sie lesen aus Memory-Mappings, und Page-Faults und Disk-I/O dürfen nie im Async-Runtime-Thread laufen. Das Modul liefert dafür zwei Dinge:
- ein austauschbares Hinweisziel (Trait, in Tests mockbar),
- einen Wrapper, der eine blockierende Closure im Blocking-Pool ausführt, optional mit vorherigem Hinweis.

## 3. Interface

```rust
use std::{io, ops::Range, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Advice { WillNeed, Sequential, Random, Cold }

/// Half-open byte range [start, end).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange { pub start: u64, pub end: u64 }

#[derive(Debug, thiserror::Error)]
pub enum AdviceError {
    #[error("range start {start} is behind end {end}")]
    StartAfterEnd { start: u64, end: u64 },
    #[error("range end {end} exceeds mapping length {len}")]
    EndBeyondLen { end: u64, len: u64 },
    #[error("os error: {0}")]
    Os(#[from] io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum BlockingError {
    #[error("blocking task was cancelled")]
    Cancelled,
    #[error("blocking task panicked")]
    Panicked,
}

/// Replaceable hint target. Implementations must not block for I/O.
pub trait AdviceTarget: Send + Sync {
    /// Length of the target in bytes.
    fn len(&self) -> u64;
    /// Apply advice. The range is already validated and non-empty.
    fn advise(&self, advice: Advice, range: ByteRange) -> Result<(), AdviceError>;
}

/// Validates the range, then forwards it to the target.
pub fn advise(target: &dyn AdviceTarget, advice: Advice, range: ByteRange) -> Result<(), AdviceError>;

/// Own prefetch function: equivalent to `advise(target, Advice::WillNeed, range)`.
pub fn prefetch(target: &dyn AdviceTarget, range: ByteRange) -> Result<(), AdviceError>;

/// Validation only: start<=end, end<=len.
pub fn check_range(range: ByteRange, len: u64) -> Result<(), AdviceError>;

/// Concrete target over a memory mapping.
pub struct MmapAdvisor { /* Arc<memmap2::Mmap> */ }
impl MmapAdvisor { pub fn new(map: Arc<memmap2::Mmap>) -> Self; }
impl AdviceTarget for MmapAdvisor { /* ... */ }

/// Read without advice: runs `f` in the blocking pool.
pub async fn read_blocking<T, F>(f: F) -> Result<T, BlockingError>
where F: FnOnce() -> T + Send + 'static, T: Send + 'static;

/// Advises first, then reads in the blocking pool.
/// `advice: None` means the default `Advice::WillNeed`.
/// `source: None` means the hint has no effect.
pub async fn read_with_advice<T, F>(
    source: Option<Arc<dyn AdviceTarget>>,
    range: ByteRange,
    advice: Option<Advice>,
    f: F,
) -> Result<T, BlockingError>
where F: FnOnce() -> T + Send + 'static, T: Send + 'static;
```

## 4. Verhalten und Invarianten

**Bereichsprüfung**
- `check_range` liefert `StartAfterEnd` bei `start > end`.
- Es liefert `EndBeyondLen` bei `end > len`.
- Die Prüfung `start > end` hat Vorrang.
- Ein leerer Bereich (`start == end`, `end <= len`) ist ein erfolgreicher No-op. `target.advise` wird dann nicht aufgerufen.
- `advise` ruft zuerst `check_range(range, target.len())`. Bei Erfolg und nicht-leerem Bereich ruft es `target.advise` genau einmal und reicht dessen Ergebnis unverändert durch.

**`MmapAdvisor`**
- `len()` ist die Mapping-Länge.
- `WillNeed`, `Sequential` und `Random` werden mit `madvise` über `memmap2::Advice` oder `libc` auf den exakten Bereich gesetzt.
- `Cold` richtet auf Page-Grenzen aus: `aligned_start = start - start % page`, `len' = (end - start) + (start - aligned_start)`.
  - Die Seitengröße kommt aus `sysconf(_SC_PAGESIZE)`.
  - Die Länge wird nicht über das Mapping-Ende hinaus verlängert.
  - Auf Linux ist das `MADV_COLD` (Wert 20). Inhalte werden nicht verworfen; nur die Reclaim-Priorität sinkt.
  - Auf anderen Plattformen (`cfg`) ist `Cold` ein erfolgreicher No-op.
- OS-Fehler (`madvise` gibt einen Fehler zurück, z. B. `EINVAL`) werden als `AdviceError::Os` weitergereicht, ohne Retry.
- Kein Hinweis verändert je die gelesenen Daten.

**`read_blocking`**
- `f` läuft per `tokio::task::spawn_blocking`, nie im aufrufenden Async-Thread.
- Wird die Blocking-Aufgabe abgebrochen, ergibt das `BlockingError::Cancelled`.
- Paniert `f`, ergibt das `BlockingError::Panicked`. Die Panic propagiert nicht in den Aufrufer.
- Die Zuordnung erfolgt über `JoinError::is_panic()`.
- Der Ergebniswert von `f` wird unverändert als `Ok(T)` geliefert.

**`read_with_advice`**
- Reihenfolge: Erst wird der Hinweis gesetzt (synchron, kurz, ohne I/O-Warten) und dann `f` im Blocking-Pool gestartet. Der Kernel kann so I/O und die Ausführung von `f` überlappen.
- `source == None`: kein Hinweis, `f` läuft trotzdem.
- Lehnt `advise` den Hinweis ab (ungültiger Bereich oder OS-Fehler), wird nur mit `tracing::warn!` geloggt. `f` läuft trotzdem, und der Fehler wird nicht zurückgegeben.
- Ein leerer Bereich führt zu keinem Hinweis und `f` läuft.

**Nebenläufigkeit**
- Alle Funktionen sind reentrant.
- `AdviceTarget: Send + Sync`, mehrere Tasks dürfen gleichzeitig Hinweise auf dasselbe Ziel setzen.
- Es gibt keinen globalen Zustand.

## 5. Relied-on elements

Keine; das Modul hat keine Abhängigkeiten zu anderen Modulen.