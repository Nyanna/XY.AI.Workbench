# Umsetzungsplan B6 · CapabilitySet & Profile-Typen

| | |
|---|---|
| **Paket** | B6 (Gruppe B) |
| **Abhängt von** | A4 |
| **Blockiert** | C3 (`requires()`-Caps), F1 (Capability-Erhebung), F2/F3 (Calibrator/Profilbildung), F4 (Matching), F5 (Store) |
| **Ergebnis** | Präzisions-Datentypen: `CapabilitySet`, `Profile{position:f32, config}`, **keine** festen Profilnamen/-Enums |
| **Quellen** | `../resident_precision.md` (Capability-Listen, Profilbildung, Matching), `../context/context_b.md` §B6 (24–26) |

---

## 1. Zweck & Abgrenzung

B6 liefert die **Datentypen** für Hardware-Fähigkeiten und abgeleitete Profile. Leitprinzip (`resident_precision.md`): **keine vordefinierten Profilnamen, kein fest codiertes Profil-Enum.** Achsen und Werte sind **pro Hardware erweiterbar, ohne Änderung an Fassade oder Task-Schnittstelle**. Profile sind **unbenannte Indizes** mit Position `p ∈ [0,1]`.

**Nicht Teil von B6:** Erhebung (F1), Messung/Pareto/Pruning/Stabilität (F2), Positions-Berechnung aus der Pareto-Front (F3), Task-Matching `|t−p|` (F4), Persistenz/Fingerprint (F5). B6 definiert nur die **Typen + algebraischen Operationen** (`satisfies`), auf die F aufsetzt. Die Pareto-/Bogenlängen-Logik wird als **Helfer** bereitgestellt (von F3 genutzt), aber der Kalibrierungsablauf gehört zu F.

---

## 2. Zu implementierende Typen

### 2.1 Achsen & Werte (erweiterbar, string-basiert, interniert möglich)

```rust
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)] pub struct AxisId(Arc<str>);  // oder interniert als u32
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)] pub struct ValueId(Arc<str>);
```
- Neue Achsen/Werte = neue Strings → **keine** Änderung an Fassade/Task. Typsicherheit optional per Newtype-Konstanten über `LazyLock` oder Registry mit Validierung.
- `enum_dispatch` o. Ä. ist **ungeeignet** (löst nur geschlossene Varianten) — bewusst nicht verwenden.

### 2.2 CapabilitySet (Achsen → Wertemengen)

```rust
#[derive(Clone, Default)]
pub struct CapabilitySet(BTreeMap<AxisId, BTreeSet<ValueId>>);

impl CapabilitySet {
    pub fn satisfies(&self, req: &CapabilitySet) -> bool {       // req ⊆ self, achsenweise
        req.0.iter().all(|(a, vs)| self.0.get(a).map_or(false, |have| vs.is_subset(have)))
    }
}
```
- `BTreeMap`/`BTreeSet` ⇒ **stabile Ordnung** → reproduzierbare Serialisierung/Hashes/Profile (wichtig für F5-Fingerprint).
- `satisfies` ist die Grundlage für C3/`requires()` (B3): `Requirements.caps` wird gegen das Engine-`CapabilitySet` gematcht.

### 2.3 Profile (unbenannter Index + Position)

```rust
pub struct ProfileConfig(BTreeMap<AxisId, ValueId>);  // genau EIN Wert pro gewählter Achse
pub struct Profile {
    pub position: f32,       // p ∈ [0,1]: 0 = fast, 1 = precision; Einzelprofil = 0.5 (Konvention)
    pub config:   ProfileConfig,
}
```
- **Keine Namen**, nur Index + `position`. Reihenfolge/Herabstufung ergibt sich aus der Messung (F3): langsamer-und-ungenauer (dominierte/Emulations-Pfade) werden herabgestuft oder nicht aktiviert.
- Eine Engine **ohne aktives Profil** für ein Modell ist für dieses Modell gesperrt (Status/Logik in F, B6 stellt nur den Typ).

### 2.4 Pareto-/Positions-Helfer (von F3 genutzt)

```rust
// Front: minimiere (Zeit t, Abweichung d). Nach t aufsteigend, nur strikt fallendes d behalten. O(n log n).
fn pareto(mut pts: Vec<(f64 /*t*/, f64 /*d*/)>) -> Vec<(f64, f64)> {
    pts.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    let mut out: Vec<(f64, f64)> = Vec::new();
    for p in pts { if out.last().map_or(true, |l| p.1 < l.1) { out.push(p); } }
    out
}
// Position: Achsen min-max normieren (Zeit ggf. logarithmisch), kumulierte Bogenlänge s_i; p = s/S_total.
// Entartung: 1 Punkt → p = 0.5 (Konvention der Spec); konstante Achse → Division durch 0 abfangen.
```
- Eigenimplementierung (~50 Zeilen, `f64::total_cmp`); keine ungeprüfte Crate. **Spread-Schwellwert** (F3) ist die **einzige** Konfiguration, die die Profilanzahl bestimmt — in der **normierten Skala** definiert (modell-/hardwareunabhängig).

---

## 3. Well-known Achsen/Werte für Zielhardware (GTX 1660, TU116, sm75)

Als `LazyLock`-Konstanten bereitstellen (F1 erhebt die realen Werte; B6 liefert die Bezeichner, damit Typen/Tests stabil sind):

| Achse (GPU) | Werte | Hinweis (context_b §B6.24) |
|---|---|---|
| `operands` | `fp32`, `fp16` | **keine** Tensor Cores; FP16 ~2× FP32-Rate auf CUDA-Cores |
| `accumulator` | `fp32`, `fp16` | |
| `scoring` | `fp32`, `fp16`, `int8-dp4a` | `dp4a` ab sm61 vorhanden; **keine** int8-Tensor-Cores |
| `fusion` | `an`, `aus` | |
| `batch`, `tile` | numerisch | unabhängig messen, nur Gewinner kombinieren |

**`bf16` NICHT anbieten** (bf16-Rechnen erst ab sm80; auf TU116 nicht nativ → f16 wählen). fp8/fp4 nicht nutzbar.

| Achse (CPU) | Werte |
|---|---|
| `operands` | `fp32`, `fp16→fp32` |
| `scoring` | `fp32`, `int8` |
| `threads`, `tile` | numerisch |

> `// TODO(A3): candle Quantkernel/dp4a-Nutzung und bf16-Verhalten auf sm75 praktisch testen (context_b offener Punkt 4).`

---

## 4. Fehler & Logging

- Q1: `Incompatible` (Kombination über Maximalabweichung/degeneriert — aktiviert in F2), `NoActiveProfile` (F3/F4), `InvalidPrecision` (NaN — F4 beim Matching). B6 **definiert** die Varianten/Signaturen, die F benutzt; die Laufzeit-Logik liegt in F.
- Q2: `tracing` — abgeleitete Profile (Anzahl, Positionen) auf Debug (gefüllt in F3).

## 5. Teststrategie (Q3, ohne GPU)

- **satisfies:** `req ⊆ self` achsenweise true/false; fehlende Achse ⇒ false; leeres `req` ⇒ true.
- **Stabile Ordnung:** gleiche Mengen ⇒ identische Serialisierung/Hash (BTreeMap-Determinismus).
- **pareto():** dominierte Punkte verworfen; sortierte Front strikt fallendes `d`.
- **Position:** Bogenlänge korrekt; Einzelprofil ⇒ `p = 0.5`; konstante Achse ⇒ keine Division durch 0; schnellstes `p=0`, genauestes `p=1`.
- **Erweiterbarkeit:** neue Achse/neuer Wert ohne Signaturänderung aufnehmbar (Typ-Test).

*(Vollständige Matching-/Stabilitäts-/Persistenz-Tests sind F2–F5.)*

## 6. Offene Punkte (→ A3 / F)

1. `AxisId`/`ValueId` als `Arc<str>` vs. interniert (`u32`-Registry) — Performance/Ergonomie.
2. Zeit-Normierung (logarithmisch?) und Spread-Schwellwert-Default (gehört inhaltlich zu F3, hier nur Typ/Helfer).
3. candle dp4a/bf16 auf sm75 (A3-Prototyp).

## 7. Definition of Done

- `AxisId`, `ValueId`, `CapabilitySet` (+`satisfies`), `ProfileConfig`, `Profile{position, config}` implementiert.
- Pareto-/Positions-Helfer vorhanden (für F3), reine Logik, keine ungeprüfte Crate.
- Well-known sm75-Achsen/Werte als `LazyLock`-Konstanten (ohne `bf16`-Rechenpfad).
- Keine festen Profilnamen/-Enums; stabile (BTreeMap-)Serialisierung; baut ohne `cuda`/`candle`.
