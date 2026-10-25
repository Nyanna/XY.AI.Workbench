# Umsetzungsplan F5 — CalibrationStore / Persistenz

> Gruppe F · Paket F5. Quelle: `resident_precision.md` (Abschnitt „Persistenz (CalibrationStore)“, „Laufzeit-Fehlerbehandlung“). Kontext: `context/context_f.md` §F5 (Q22–Q25). Querschnitt: `plan_f0_overview.md` §4.
> **Hängt ab von:** F3 (Profile), F1 (CapabilitySet für Rohdaten). **Liefert an:** Subengine-Init (lädt Profile beim ersten Task-Bedarf).

## 1. Zweck & Ergebnis
Der `CalibrationStore` persistiert das Kalibrierergebnis **pro `Modell × Subengine`** auf derselben Maschine (Datei), erkennt über einen **Fingerprint** die Zustände `Calibrated | Stale | Missing` und steuert das Verhalten (laden / im Hintergrund neu kalibrieren / konservativer Default). Ergebnis: Persistenz-Spezifikation + Store-API + Fingerprint + Serde-Schema.

## 2. Verbindliche Vorgaben (`resident_precision.md`)
- Kalibrierung läuft **initial einmal** und wird persistiert; **Erstkalibrierung darf lang sein**.
- Gilt **pro Modell × Subengine** (Verhalten hängt von Dimension und Sequenzlänge ab).
- **Fingerprint:** GPU-Modell, Treiber-/CUDA-Version, CPU-Modell, Modell-ID, Engine-Version.
- **Zustände:** `Calibrated` (laden/verwenden), `Stale` (Fingerprint abweichend: mit altem Ergebnis starten), `Missing` (konservativer Default = genaueste gültige Kombination + Warnung, **oder** explizites `calibrate`-Kommando).
- **Persistiert werden auch:** `unstable`-Markierung, Maximalabweichungs-**Verwerfungen** und **Pruning-Ergebnisse** (für Diagnose und Nachmessung).

## 3. Zieltypen (Skizze)
```rust
pub enum StoreState { Calibrated, Stale, Missing }

#[derive(PartialEq, Eq)]
pub struct Fingerprint {
    pub schema_version: u32,
    pub gpu_name: Option<String>,
    pub gpu_cc: Option<(u32,u32)>,
    pub cuda_driver_version: Option<u32>,
    pub cpu_features: Vec<&'static str>,  // fixe Reihenfolge, std-Detection
    pub model_id: String,
    pub engine_version: String,
}

pub struct CalibrationRecord {
    pub fingerprint: Fingerprint,
    pub profiles: Vec<Profile>,          // aus F3
    pub unstable: bool,                  // F2-Markierung
    #[serde(default)] pub rejections: Vec<RejectedCombo>,  // Maximalabweichung/degeneriert
    #[serde(default)] pub pruning: Vec<PrunedCombo>,       // Diagnose
}

pub trait CalibrationStore {
    fn load(&self, model_id: &str, engine: EngineKind) -> std::io::Result<Option<CalibrationRecord>>;
    fn state(&self, rec: &Option<CalibrationRecord>, current: &Fingerprint) -> StoreState;
    fn save_atomic(&self, rec: &CalibrationRecord) -> std::io::Result<()>; // Temp + rename
}
```

## 4. Fingerprint-Erhebung (minimal, ohne Zusatzabhängigkeiten)
- **GPU:** über `cudarc`, das die Engine ohnehin nutzt: `CudaContext::name()`, `compute_capability()` und `sys::cuDriverGetVersion` (u32, z. B. 13030). Ohne CUDA sind die GPU-Felder `None`.
- **CPU:** `std::arch::is_x86_feature_detected!` (z. B. `avx2`, `f16c`, `avx512vnni`) in fester Reihenfolge. Kein CPU-Modellname, keine Threadzahl.
- **Kein Treiber-String, kein VRAM, keine Hash-Funktion.** Ein Treiberwechsel ohne Änderung der CUDA-Driver-Version gilt nicht als Systemwechsel. Wer neu kalibrieren will, löscht die Kalibrierdatei.
- **Vergleich:** `Fingerprint` wird direkt per `==` mit dem gespeicherten verglichen. Zustände: `Calibrated` (gleich), `Stale` (abweichend), `Missing` (keine Datei). `schema_version` steht als Klartextfeld im Fingerprint. Ein anderer Wert ergibt `Stale`.

## 5. Serde-Format (context §F5/Q24 [W])
- **JSON** (debugbar) mit `schema_version`; `#[serde(default)]` für neue Felder (vorwärtskompatibel), **unbekannte Felder tolerieren**. (`postcard`/`bincode` wären kompakter, aber nicht selbstbeschreibend → verworfen für die Diagnosefähigkeit.)
- `unstable`, `rejections` (Maximalabweichungs-Verwerfungen) und `pruning` als **explizite Felder** (Pflicht für Diagnose/Nachmessung).
- Ablageort und Dateiname bestimmt die Applikation. Der Store bekommt den Pfad übergeben.

## 6. Zustandsverhalten (context §F5/Q25 + `resident_precision.md`)
- **`Calibrated`:** laden und verwenden.
- **`Stale`:** alte Ergebnisse **sofort nutzen**, im **Hintergrund** neu kalibrieren, dann **atomar ersetzen** (Temp-Datei + `rename`). Nie den Nutzer blockieren.
- **`Missing`:** **konservativer Default** = genaueste gültige Kombination (fp32-Referenz), **Warnung loggen**, Kalibrierung im Hintergrund anstoßen; **manuelle** Nachkalibrierung via CLI/API-Flag (`calibrate`).
- Hintergrund-Kalibrierung läuft im **eigenen Thread-Pool** mit niedriger Priorität (F2), **nicht** im tokio-Runtime-Thread; sie darf den Betrieb nie blockieren.

## 7. Querschnitt
- **Fehler (Q1):** `PrecisionError::Store(std::io::Error)` für IO/Serde; `Stale`/`Missing` sind **Zustände**, keine Fehler. `NoActiveProfile` entsteht nicht hier, sondern im Matching (F4), wenn ein geladenes/Default-Record keine aktiven Profile hat.
- **Logging (Q2):** `warn!` bei `Missing` (konservativer Default aktiv); `info!` bei `Stale` (Hintergrund-Rekalibrierung gestartet) und beim atomaren Ersetzen; Fingerprint-Hash + `schema_version` im Log.
- **Feature-Flags (Q4):** GPU-Felder des Fingerprints (`cudarc`) hinter `cuda` optional; ohne GPU bleiben die GPU-Felder `None`, der CPU-Teil bleibt gültig. Store-Logik selbst ist GPU-frei testbar.

## 8. Teststrategie (Q3, `resident_precision.md` „Tests“)
- **Fingerprint-Wechsel → `Stale`:** abweichendes Feld (z. B. `cuda_driver_version`) ⇒ Fingerprints ungleich ⇒ `Stale`; altes Record bleibt nutzbar.
- **Fehlende Datei → `Missing`:** `load` liefert `None` ⇒ `state == Missing` ⇒ konservativer Default + Warnung.
- **Round-Trip:** `save_atomic` → `load` liefert identisches Record (inkl. `unstable`, `rejections`, `pruning`).
- **Vorwärtskompatibilität:** JSON mit zusätzlichem unbekanntem Feld lädt ohne Fehler (`#[serde(default)]`, unbekannte toleriert).
- **Atomarität:** `save_atomic` schreibt Temp + `rename`; ein abgebrochener Schreibvorgang lässt das alte Record intakt (simulierbar).
- **`unstable`-Persistenz:** ein in F2 als `unstable` markiertes Ergebnis wird gespeichert und wieder geladen.
- Alle Tests ohne GPU (GPU-Felder `None`).

## 9. Verifikationsergebnisse
Geprüft per Probe-Crate (Linux, GTX 1660 SUPER, CC 7.5, CUDA 13.3, `cudarc =0.19.9`), das kompiliert und ausgeführt wurde.

| Punkt | Ergebnis |
|---|---|
| GPU-Felder | `CudaContext::new(0)` liefert `name()` = `"NVIDIA GeForce GTX 1660 SUPER"` und `compute_capability()` = (7, 5). `sys::cuDriverGetVersion` liefert `CUDA_SUCCESS` und `13030`. `cudarc` reicht aus, `nvml-wrapper` entfällt. |
| CPU-Felder | `is_x86_feature_detected!` liefert `avx2`, `f16c` = true und `avx512vnni` = false. Es ist Teil von `std`. `raw-cpuid` und `sysinfo` entfallen. Auf Nicht-x86-Hosts ist das Makro nicht verfügbar, dort bleibt `cpu_features` leer (`cfg(target_arch)`). |
| Vergleich | Direkter `==`-Vergleich des Fingerprints. Hash und kanonische Serialisierung entfallen, damit auch `blake3`. |
| Serde | Ein unbekanntes Feld im JSON lädt fehlerfrei (ohne `deny_unknown_fields`), `#[serde(default)]` füllt fehlende Felder. `deny_unknown_fields` darf nicht gesetzt werden. |
| Atomarität | Temp-Datei im Zielverzeichnis, `sync_all()`, `rename()` funktioniert unter Linux. `rename` ist nur innerhalb desselben Dateisystems atomar, daher liegt die Temp-Datei im Verzeichnis des Ziels. |
| Fingerprint-Umfang | Bewusst minimal: ein Treiber-Update ohne neue CUDA-Driver-Version bleibt unbemerkt. Das ist akzeptiert, der Nutzer kann die Kalibrierdatei löschen. |

## 10. Definition of Done
- `CalibrationStore` (load/state/save_atomic), `Fingerprint`, Serde-Schema (JSON + `schema_version` + `#[serde(default)]`) implementiert.
- Fingerprint-Erhebung nur mit `cudarc` und `std` (keine Zusatzabhängigkeit); GPU-Felder optional hinter `cuda`.
- Zustandsverhalten `Calibrated/Stale/Missing` inkl. atomarem Ersetzen und Hintergrund-Rekalibrierung ohne Blockieren.
- `unstable`/`rejections`/`pruning` persistiert. Pflicht-Tests grün.
