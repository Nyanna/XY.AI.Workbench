# Umsetzungsplan B2 · Region / Layout / Speicher-Modell

| | |
|---|---|
| **Paket** | B2 (Gruppe B) |
| **Abhängt von** | A4 |
| **Blockiert** | B3 (`Value::Device`), B5 (Spans), D2/D3 (Device/Arena), E3 (ArenaAllocator) |
| **Ergebnis** | Speicher-Abstraktionsvertrag: `Region`, `Layout`, eigenes `DType`, `MemoryManager`-Trait, Arena-Konzept (ohne GPU-Details) |
| **Quellen** | `../resident_subengine.md` §4.2, `../context/context_b.md` §B2 (8–11) |

---

## 1. Zweck & Abgrenzung

Einheitliches, **pointer- und tensorfreies** Speichermodell. Öffentliche Signaturen kennen nur `Region` (Handle) und `Layout` (Form/Strides/dtype). Die Auflösung Handle → Device-Pointer liegt **ausschließlich im Backend** (D2/D3/E3), nie in B2.

**Nicht Teil von B2:** konkrete GPU-Arenen, `cuMemGetInfo`-Budget, Pinned Staging, Stream-geordnete Wiederverwendung (alles D3/E3). B2 definiert die **Schnittstelle** und das **Free-List-Konzept** generisch und ohne GPU.

**Invariante (No-Tensor/No-Pointer):** Der Allokator kennt nur `(offset, len, align)`; `Region` ist ein Handle, kein Zeiger.

---

## 2. Zu implementierende Typen

### 2.1 DeviceKind & dtype-Policy

```rust
#[non_exhaustive]
pub enum DeviceKind { Cpu, Gpu }

pub fn compute_dtype(dev: DeviceKind) -> DType {
    match dev { DeviceKind::Gpu => DType::F16, DeviceKind::Cpu => DType::F32 }
}
```
Policy: **GPU f16 / CPU f32** (passt zu 6 GB VRAM; f16 halbiert Bedarf). Index-dtype ist pro Index fest; gemischte CPU/GPU-Indexierung nur mit definierter Toleranz (F/Index-Kalibrierung).

### 2.2 Eigenes DType-Enum (kein candle-Leak)

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DType { F32, F16, Bf16, U8, U32, I64 /* erweiterbar */ }
impl DType { pub const fn size_bytes(self) -> usize { /* … */ } }
```
- `#[non_exhaustive]`, weil candle (0.10.2/0.11.0) **14** Varianten inkl. Dummy-Typen führt (`F8E4M3`, `F6E2M3`, …). Öffentliche Signaturen nennen **nur** das eigene Enum.
- Adapter **nur** in `pub(crate)`-Modul hinter `feature = "candle"`:
  ```rust
  impl TryFrom<DType> for candle_core::DType { type Error = UnsupportedDType; /* … */ }
  impl TryFrom<candle_core::DType> for DType { type Error = UnsupportedDType; /* Wildcard-Arm Pflicht */ }
  ```
  `TryFrom` (nicht `From`), da candle neue/Dummy-Varianten hat. Analog `TryFrom<safetensors::Dtype>` (Namen weichen ab: F16/BF16/F32/U8/I64).
- **sm75-Hinweis (aus B6-Recherche):** `Bf16` existiert als Typ, ist aber auf TU116 **nicht nativ**; dtype-Wahl für GPU-Rechnen = `F16`.

### 2.3 Region (Handle) & generationale Entwertung

```rust
pub struct Region {
    pub arena:  ArenaId,   // welche Arena
    pub offset: u64,       // Arena-Offset in BYTES
    pub len:    u64,       // Länge in BYTES
    pub gen:    u32,       // entwertet Handle nach Evict/Reload
}
```
- `gen` wird beim Freigeben/Reload des Slots erhöht; ein veraltetes Handle erkennt der Manager per `slot.gen == handle.gen` (O(1)-Vergleich). Stale-Zugriff ist intern `StaleMapping` (Q1), nach außen nie ein roher Fehlzugriff.
- Handle-Verwaltung optional über `slotmap`/`thunderdome` (8 B Key, `gen` als `u32`); bei Überlauf Slot stilllegen. Bei Offset-Bezug reicht das **eigene** `Region`-Struct (keine Crate-Bindung).

### 2.4 Layout (Tensor-Layout)

```rust
pub struct Layout { pub shape: Vec<usize>, pub strides: Vec<usize>, pub dtype: DType }
```
- **Entscheidung:** `strides` und (impliziter) Offset werden in **Bytes** geführt (Region ist offset-/byte-basiert). Umrechnung an der candle-Grenze: candle `Layout` nutzt **Elemente** → `bytes = elems * dtype.size_bytes()` im Adapter.
  > `// TODO(A3): Bytes- vs. Elemente-Konvention final bestätigen (context_b §B2.10). candle narrow/transpose/permute/broadcast ändern nur Layout.`
- Views (narrow/transpose/permute/broadcast) sind reine Layout-Operationen, keine Kopien.

### 2.5 MemoryManager-Trait + Arena-Konzept

```rust
pub trait MemoryManager {
    fn alloc(&self, dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, Error>; // Error::ArenaExhausted
    fn free(&self, region: Region);
    // resolve(): NUR intern/pub(crate), gibt Backend-spezifische Adresse — nie in öffentlicher API
}
```
- **Arenen nach Lebensdauer** (Benennung/Belegung in D3/E3): Weights (langlebig), Index-Segment (gestreamt/evictable), Workspace/Scratch pro Lane (kurzlebig), Staging (Pinned, nur GPU), ParamBlock pro Lane (klein, fest).
- **Free-List-Konzept (B2, backend-agnostisch):** Offsets als `u64`, Handles statt Zeiger, Free-List als `BTreeMap<u64 /*offset*/, u64 /*len*/>` mit Coalescing (Nachbarsuche). Varianten First-/Best-Fit, TLSF (O(1)), Buddy. Konkrete GPU-Arena (VRAM-Budget via `cuMemGetInfo`, CoW-Doppelbedarf, Copy-Stream) = D3/E3.
- Rust-Bausteine (A3-geprüft): `offset-allocator`, `gpu-allocator`, `range-alloc`.

---

## 3. Fehler & Logging

- Q1: `ArenaExhausted` (B2), `UnsupportedDType` (Adapter), `StaleMapping` (intern, bei gen-Mismatch).
- Q2: Arena-Auslastung und alloc/free als Debug-Events (die lauten Betriebslogs liegen in D3/E3).

## 4. Teststrategie (Q3, ohne GPU)

- **DType:** `size_bytes` für alle Varianten; `TryFrom`-Roundtrip candle↔eigen (hinter `candle`-Feature) inkl. Fehler bei Dummy-Variante.
- **Region/gen:** Alloc→Free→Realloc erhöht `gen`; altes Handle wird als stale erkannt.
- **Free-List:** Alloc/Free-Sequenzen, Coalescing benachbarter Blöcke, Fragmentierung, `ArenaExhausted` bei Überbelegung (reine Logik, mit Fake-Backend).
- **Layout:** Byte-Strides-Berechnung; Adapter-Umrechnung Elemente↔Bytes.
- No-Pointer-Invariante: Compile-Test, dass keine öffentliche Signatur candle-/Pointer-Typen nennt (Doc-Test/`trybuild` optional).

## 5. Offene Punkte (→ A3)

1. Byte- vs. Elemente-Konvention für `Layout::strides` final.
2. Eigenes `Region`-Struct vs. `slotmap`/`thunderdome` für Slot-Verwaltung.
3. Free-List-Variante (BTreeMap/TLSF/Buddy) und Crate-Pin (`offset-allocator` u. a.).
4. candle-Version (0.10.x/0.11.x) für den Adapter.

## 6. Definition of Done

- `Region`, `Layout`, `DType` (+`size_bytes`, +`TryFrom`-Adapter hinter `candle`), `DeviceKind`, `compute_dtype`, `MemoryManager`-Trait + backend-agnostische Free-List implementiert.
- Kein `Tensor`/Pointer in öffentlicher API; gen-Entwertung + `ArenaExhausted` getestet.
- Baut ohne `cuda`/`candle`-Feature.
