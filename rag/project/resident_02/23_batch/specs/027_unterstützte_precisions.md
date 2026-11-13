# Spezifikation: Unterstützte Precisions (Build-Order 27)

## 1. Aufgabe, Einschränkungen, Grenzen
Rust-Modul, das statisch festlegt, für welche Elementtypen (`DType`) je Device-Art Scoring-Kernel existieren. Es meldet die Capability-Achsen `operands` und `scoring` nur für diese Typen.

Einschränkungen:
- Das Modul ist reine Tabelle und reine Funktionen. Es hat keinen Zustand, keine I/O, keine Hardware-Erkennung und keine Kernel-Logik.
- Die Tabelle hängt nicht von aktivierter GPU-Unterstützung ab. Es gibt kein `cfg`/Feature-Gate und keine Laufzeitprüfung. `Gpu` liefert auch auf Hosts ohne GPU immer dieselbe Antwort.
- BF16 ist nie ein Compute-Pfad. Die Typ-Kennung `DType::BF16` bleibt in `layout` für Gewichte nutzbar. Dieses Modul meldet sie nie.
- Das Modul setzt keine anderen Achsen (`accumulator`, `fusion`, `batch_size`, `tile_size`, `thread_count`, `density`) und schränkt sie nicht ein.
- Keine Konvertierung, keine Profilnamen. Optionale Crates: keine.

## 2. Kontext
- `DeviceKind` (`Cpu`, `Gpu`, `#[non_exhaustive]`) stammt aus „Device und Lane und Event“.
- `DType` stammt aus „Layout und Elementtypen“.
- `CapabilitySet`, `Axis`, `Value`, `axes::*` und `values::*` stammen aus dem Capability-Modell.
- Backends und die Facade bilden die Capability-Menge eines Devices aus dem Ergebnis dieses Moduls und fügen eigene Achsen hinzu. Das Ergebnis liegt innerhalb von `cpu_reference()` bzw. `gpu_reference()`.

## 3. Interface
```rust
use crate::capability::{CapabilitySet, Value};
use crate::device::DeviceKind;
use crate::layout::DType;

/// Elementtypen mit Scoring-Kernel für die Device-Art, in fester Reihenfolge.
/// Cpu => [F32]; Gpu => [F32, F16]; unbekannte (künftige) Art => [].
pub fn scoring_dtypes(kind: DeviceKind) -> &'static [DType];

/// true iff `dtype` in `scoring_dtypes(kind)`.
pub fn has_scoring_kernel(kind: DeviceKind, dtype: DType) -> bool;

/// Capability-Wert-Name des Typs: F32 => Some(values::fp32()), F16 => Some(values::fp16()),
/// alle anderen (inkl. BF16) => None.
pub fn dtype_value(dtype: DType) -> Option<Value>;

/// Achsen `operands` und `scoring`, je mit den Werten aller Typen aus `scoring_dtypes(kind)`.
/// Nur diese beiden Achsen. Leere Menge, wenn kein Typ unterstützt ist.
pub fn precision_capabilities(kind: DeviceKind) -> CapabilitySet;
```
Alle Funktionen sind frei, rein und thread-sicher (`CapabilitySet: Send + Sync`). Es gibt kein `Result`, es kann kein Fehler auftreten.

## 4. Verhalten und Invarianten
- `scoring_dtypes(Cpu) == [F32]`.
- `scoring_dtypes(Gpu) == [F32, F16]`.
- Für jede künftige, hier nicht bekannte `DeviceKind`-Variante ist das Ergebnis leer (Wildcard-Zweig).
- `has_scoring_kernel(k, BF16) == false` für jedes `k`. `U8`, `U32` und `I64` haben nie einen Scoring-Kernel.
- `has_scoring_kernel(Cpu, F16) == false`. Die CPU-Variante „FP16 mit Konvertierung“ (`fp16_storage_fp32_compute`) wird hier nicht gemeldet.
- `precision_capabilities(Cpu)` ist genau `{operands:{fp32}, scoring:{fp32}}`.
- `precision_capabilities(Gpu)` ist genau `{operands:{fp32,fp16}, scoring:{fp32,fp16}}`.
- Operanden und Scoring haben immer dieselbe Wertemenge. Ein Typ ohne Kernel erscheint auf keiner der beiden Achsen, in keiner Konfiguration.
- Das Ergebnis enthält nie `bf16`, `accumulator`, `fusion`, `int8*`, `fp16_storage_fp32_compute` oder numerische Achsen.
- Das Ergebnis ist eine Teilmenge der jeweiligen Referenzmenge: `precision_capabilities(Cpu).is_within(&cpu_reference())` und `precision_capabilities(Gpu).is_within(&gpu_reference())` sind true.
- Aufrufer können das Ergebnis per `Extend`/`add` mit weiteren Achsen zusammenführen. Dieses Modul beeinflusst diese nicht.
- Jeder Aufruf liefert einen frischen, gleichen Wert. Es gibt keine Panics, keinen globalen Zustand und keine Abhängigkeit von Reihenfolge oder Threads.
- Für jedes `d` in `scoring_dtypes(k)` gilt `dtype_value(d).is_some()`. Für jedes `d` mit `dtype_value(d).is_none()` gilt `!has_scoring_kernel(k, d)`.

## 5. Relied-on elements
- `DeviceKind`: `#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] #[non_exhaustive] enum DeviceKind { Cpu, Gpu }`. Matches brauchen einen Wildcard-Zweig.
- `DType`: `enum DType { F32, F16, BF16, U8, U32, I64 }` (`Copy`, `Eq`, `Hash`).
- Capability-Modell:
  - `CapabilitySet::new()`.
  - `CapabilitySet::add(&mut self, axis: impl Into<Axis>, value: impl Into<Value>) -> bool`.
  - `CapabilitySet::with(self, axis, value) -> Self`.
  - `CapabilitySet::is_within(&self, upper: &CapabilitySet) -> bool`.
  - `axes::operands()` = "operands" und `axes::scoring()` = "scoring".
  - `values::fp32()` = "fp32" und `values::fp16()` = "fp16".
  - `cpu_reference()` und `gpu_reference()`.
  - Eine Achse existiert nur mit mindestens einem Wert. Deshalb entsteht bei leerer Typliste keine Achse.