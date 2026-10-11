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

pub struct Fingerprint {
    pub schema_version: u32,     // Klartext, NICHT nur im Hash
    pub gpu_name: Option<String>,
    pub gpu_cc: Option<(u32,u32)>,
    pub gpu_vram: Option<u64>,
    pub driver_version: Option<String>,
    pub cuda_driver_version: Option<String>,
    pub cuda_runtime_version: Option<String>,
    pub cpu_brand: String,
    pub cpu_features: Vec<String>,  // AVX2/F16C/VNNI ...
    pub cpu_threads: usize,
    pub model_id: String,           // + Hash
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

## 4. Fingerprint-Erhebung (context §F5/Q22–Q23)
- **GPU/Treiber/CUDA:** NVML via `nvml-wrapper` (`device.name()`, `sys_driver_version()`, `sys_cuda_driver_version()`); **CC** via `cudarc` (F1). Fallback bei fehlendem NVML (z. B. Windows ohne DLL): `cudarc::driver::sys::cuDriverGetVersion`.
- **CPU:** `raw-cpuid` (Brand-String, Feature-Flags AVX2/F16C/VNNI), `sysinfo` (Kernzahl).
- **Schlüssel** = Hash (empfohlen **blake3**) über **kanonisch serialisierte** Felder, **plus `schema_version` im Klartext** (nicht nur im Hash). Zustände: `Calibrated` (Hash gleich), `Stale` (Hash weicht ab), `Missing` (keine Datei). [§F5/Q23]

## 5. Serde-Format (context §F5/Q24 [W])
- **JSON** (debugbar) mit `schema_version`; `#[serde(default)]` für neue Felder (vorwärtskompatibel), **unbekannte Felder tolerieren**. (`postcard`/`bincode` wären kompakter, aber nicht selbstbeschreibend → verworfen für die Diagnosefähigkeit.)
- `unstable`, `rejections` (Maximalabweichungs-Verwerfungen) und `pruning` als **explizite Felder** (Pflicht für Diagnose/Nachmessung).
- Ablageort: pro Maschine, Dateiname/Pfad enthält `model_id` + `engine` + `schema_version`.

## 6. Zustandsverhalten (context §F5/Q25 + `resident_precision.md`)
- **`Calibrated`:** laden und verwenden.
- **`Stale`:** alte Ergebnisse **sofort nutzen**, im **Hintergrund** neu kalibrieren, dann **atomar ersetzen** (Temp-Datei + `rename`). Nie den Nutzer blockieren.
- **`Missing`:** **konservativer Default** = genaueste gültige Kombination (fp32-Referenz), **Warnung loggen**, Kalibrierung im Hintergrund anstoßen; **manuelle** Nachkalibrierung via CLI/API-Flag (`calibrate`).
- Hintergrund-Kalibrierung läuft im **eigenen Thread-Pool** mit niedriger Priorität (F2), **nicht** im tokio-Runtime-Thread; sie darf den Betrieb nie blockieren.

## 7. Querschnitt
- **Fehler (Q1):** `PrecisionError::Store(std::io::Error)` für IO/Serde; `Stale`/`Missing` sind **Zustände**, keine Fehler. `NoActiveProfile` entsteht nicht hier, sondern im Matching (F4), wenn ein geladenes/Default-Record keine aktiven Profile hat.
- **Logging (Q2):** `warn!` bei `Missing` (konservativer Default aktiv); `info!` bei `Stale` (Hintergrund-Rekalibrierung gestartet) und beim atomaren Ersetzen; Fingerprint-Hash + `schema_version` im Log.
- **Feature-Flags (Q4):** NVML/`cudarc`-Felder des Fingerprints hinter `cuda` optional; ohne GPU bleiben die GPU-Felder `None`, der CPU-Teil bleibt gültig. Store-Logik selbst ist GPU-frei testbar.

## 8. Teststrategie (Q3, `resident_precision.md` „Tests“)
- **Fingerprint-Wechsel → `Stale`:** abweichendes Feld (z. B. Treiberversion) ⇒ Hash weicht ab ⇒ `Stale`; altes Record bleibt nutzbar.
- **Fehlende Datei → `Missing`:** `load` liefert `None` ⇒ `state == Missing` ⇒ konservativer Default + Warnung.
- **Round-Trip:** `save_atomic` → `load` liefert identisches Record (inkl. `unstable`, `rejections`, `pruning`).
- **Vorwärtskompatibilität:** JSON mit zusätzlichem unbekanntem Feld lädt ohne Fehler (`#[serde(default)]`, unbekannte toleriert).
- **Atomarität:** `save_atomic` schreibt Temp + `rename`; ein abgebrochener Schreibvorgang lässt das alte Record intakt (simulierbar).
- **`unstable`-Persistenz:** ein in F2 als `unstable` markiertes Ergebnis wird gespeichert und wieder geladen.
- Alle Tests ohne GPU (GPU-Felder `None`).

## 9. Offene Verifikation / Risiken
- `[prüfen]` `nvml-wrapper`-API (`sys_driver_version`, `sys_cuda_driver_version`) und Verfügbarkeit (Windows ohne DLL → Fallback `cuDriverGetVersion`).
- `[W]` gesamter F5-Abschnitt ist Fachwissen → Fingerprint-Feldauswahl/Serde-Schema vor Festschreibung bestätigen.
- Kanonische Serialisierung für den Hash festlegen (Feldreihenfolge stabil!), sonst instabile Hashes.
- Dateirechte/Ablageort pro OS; `rename` muss auf dem Zielvolume atomar sein (gleiches Verzeichnis für Temp-Datei).

## 10. Definition of Done
- `CalibrationStore` (load/state/save_atomic), `Fingerprint`, Serde-Schema (JSON + `schema_version` + `#[serde(default)]`) implementiert.
- Fingerprint-Erhebung mit NVML/`cudarc`/`raw-cpuid`/`sysinfo` + NVML-Fallback; GPU-Felder optional hinter `cuda`.
- Zustandsverhalten `Calibrated/Stale/Missing` inkl. atomarem Ersetzen und Hintergrund-Rekalibrierung ohne Blockieren.
- `unstable`/`rejections`/`pruning` persistiert. Pflicht-Tests grün.
