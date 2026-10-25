# Umsetzungsplan F4 — Task-Matching (`precision → Profil`)

> Gruppe F · Paket F4. Quelle: `resident_precision.md` (Abschnitt „Task-Parameter und Matching“). Kontext: `context/context_f.md` §F4 (Q19–Q21). Querschnitt: `plan_f0_overview.md` §4.
> **Hängt ab von:** F3 (Profile), B6 (Typen), C1 (Fassade/`TaskParams`). **Liefert an:** Subengine-Pull (D1) / Scorer — welches Profil ein Task bekommt.

## 1. Zweck & Ergebnis
F4 bildet den **Wunsch** `TaskParams.precision: f32` (Skala **0 = fast, 1 = precision**) auf das **nächstliegende aktive Profil** der jeweiligen Subengine ab. Ergebnis: eine deterministische Matching-Funktion + Klemm-/NaN-Regeln + Tie-Break.

## 2. Verbindliche Semantik (`resident_precision.md`)
- `TaskParams.precision: f32`, Skala **0 = fast, 1 = precision**. Orientierung **muss** in Doku **und** Typ-Dokumentation ausdrücklich stehen.
- Werte außerhalb `[0,1]` werden **geklemmt**; **`NaN` ist ein Fehler** (`InvalidPrecision`).
- Matching: Profil mit **minimalem `|t − p|`**. Bei Gleichstand gewinnt **deterministisch das genauere** Profil (höheres `p`).
- Jede Subengine matcht gegen ihre **eigenen** Profile; die Skala ist **relativ zur Engine** (`t = 0.5` ist auf CPU und GPU nicht dasselbe Genauigkeitsniveau).
- Ein Wunsch ist **immer erfüllbar** (es gibt immer ein nächstliegendes aktives Profil). **Kein** Fehler und **kein** Warten wegen nicht verfügbarem Profil — **außer** die Engine hat für dieses Modell gar kein aktives Profil (dann `NoActiveProfile`, lazy; siehe §5).

## 3. Zielsignatur (Skizze)
```rust
/// Gibt den Index/das Profil mit minimalem |t - p| zurück.
/// Fehler nur bei NaN (InvalidPrecision) oder leerer aktiver Profilmenge (NoActiveProfile).
pub fn match_profile<'a>(
    precision: f32,
    profiles: &'a [Profile],   // aktive Profile EINER Engine für EIN Modell
    model_id: &str,
) -> Result<&'a Profile, PrecisionError>;
```

## 4. Implementierungsregeln (context §F4/Q20–Q21 [W])
1. **NaN zuerst abfangen:** `if precision.is_nan() { return Err(InvalidPrecision) }` — **vor** `clamp`, denn `f32::clamp` gibt bei NaN NaN zurück (Rust-Verhalten vor Implementierung gegen Doku **verifizieren**). `±∞` wird von `clamp` auf die Grenzen `[0,1]` geklemmt.
2. **Klemmen:** `let t = precision.clamp(0.0, 1.0);`.
3. **Leere aktive Profilmenge:** `Err(NoActiveProfile { model_id })`.
4. **Nächstes Profil:** `min_by` über `(t − p).abs()`, Vergleich mit `total_cmp`. **Distanz in f64 rechnen**, um f32-Rundungs-Gleichstände zu vermeiden.
5. **Tie-Break zum genaueren:** `d.total_cmp(&d2).then(p2.total_cmp(&p1))` (bei gleicher Distanz gewinnt höheres `p`).

```rust
let t = precision as f64; // bereits geklemmt
profiles.iter()
    .min_by(|a, b| {
        let da = (t - a.position as f64).abs();
        let db = (t - b.position as f64).abs();
        da.total_cmp(&db).then(b.position.total_cmp(&a.position))
    })
    .ok_or_else(|| PrecisionError::NoActiveProfile { model_id: model_id.into() })
```

## 5. Lazy-Sperre (aus `resident_precision.md` „Laufzeit-Fehlerbehandlung“)
- Ein inkompatibles/gesperrtes Modell fällt **erst beim ersten Task** auf. Der Fehler (`NoActiveProfile`) geht **sauber an den wartenden Task** zurück; die Subengine wird für dieses Modell **markiert**, damit Folgetasks die Prüfung nicht wiederholen.
- **Kein** Profil-Prädikat im Pull/Claim und **keine** Konsistenzbedingung zwischen Tasks (Vorgabe „Pull-Mechanismus“).

## 6. Einordnung zum Task-Parameterobjekt (C1)
- `precision: f32` ist ein Feld des erweiterbaren `TaskParams`-Objekts der Fassade (C1). F4 liefert nur die Matching-Funktion; die Fassade/Planner reicht den Wert durch. **Keine** Zusicherung nach außen (Wunsch-Semantik).
- Analogien zur Orientierung (keine 1:1-Vorlage): HNSW `ef_search` / IVF `nprobe`, Faiss-Autotuning, TensorRT-Präzisionsmodi, llama.cpp-Quantisierungsstufen. [context §F4/Q19]

## 7. Querschnitt
- **Fehler (Q1):** `InvalidPrecision` (NaN), `NoActiveProfile { model_id }` (keine aktiven Profile). Sonst nie Fehler/Warten.
- **Logging (Q2):** **gewähltes Profil pro Task auf `debug`** (Vorgabe): `debug!(task_id, t, chosen_p = profile.position)`.
- **Feature-Flags (Q4):** rein rechnerisch, GPU-frei.

## 8. Teststrategie (Q3, `resident_precision.md` „Tests“)
- **Klemmen:** `-0.3 → 0.0`, `1.7 → 1.0`, `+∞ → 1.0`, `−∞ → 0.0`.
- **NaN-Fehler:** `NaN → Err(InvalidPrecision)` (vor Klemmen).
- **Tie-Break zum genaueren:** zwei Profile gleich weit von `t` → höheres `p` gewinnt, deterministisch (mehrfach ausgeführt gleich).
- **Einzelprofil:** jedes `t` matcht das einzige Profil (`p = 0.5`).
- **Leere Profilmenge:** `Err(NoActiveProfile)`.
- **f64-Distanz:** konstruierte Fälle, in denen f32-Distanz einen Gleichstand erzeugen würde, f64 aber nicht (und umgekehrt der Tie-Break greift).

## 9. Verifikationsergebnisse
Geprüft mit rustc 1.99.0 (Testprogramm, `-O`).
- **`f32::clamp` bei NaN:** gibt NaN zurück (kein Panic). Die NaN-Prüfung muss daher vor `clamp` stehen. `clamp` panikt nur, wenn `min > max` oder eine Grenze NaN ist. Das ist bei den festen Grenzen `0.0`/`1.0` ausgeschlossen.
- **Klemmen:** `-0.3 → 0.0`, `1.7 → 1.0`, `+∞ → 1.0`, `−∞ → 0.0`. Das stimmt mit den Testvorgaben in §8 überein.
- **Tie-Break:** Bei Profilen `{0.0, 0.25, 0.75}` und `t = 0.5` gewinnt in drei Läufen immer `0.75`. Das Ergebnis hängt nicht von der Eingabereihenfolge ab (auch nicht bei `[0.75, 0.25]`). `min_by` mit `total_cmp` und nachgeschaltetem Vergleich auf `p` ist also deterministisch.
- **Leere Profilmenge:** `min_by` liefert `None`, das auf `NoActiveProfile` abgebildet wird.
- **f64-Distanz:** Die Differenz zweier f32-Werte, nach f64 gehoben, ist praktisch exakt. Gleichstände entstehen daher nur bei echter Gleichheit der Distanzen. Das Beispiel `t = 0.3` mit `p = 0.1` und `p = 0.5` zeigt Distanzen von `0.20000001…` und `0.19999998…`. Beide Rechnungen unterscheiden die Distanzen. Der Tie-Break greift nur bei exakt gleichen f64-Distanzen, etwa bei dyadischen Werten wie `0.25/0.75` um `0.5`.
- **Restrisiko:** Keines offen. Die Tests aus §8 decken die geprüften Fälle ab, auch mit mehreren gleich weit entfernten Profilen.



## 10. Definition of Done
- `match_profile` implementiert mit NaN-Fehler-vor-Klemmen, f64-Distanz, Tie-Break zum genaueren.
- `precision`-Skala-Semantik (0=fast, 1=precision) in Typ-Doku dokumentiert.
- Lazy-`NoActiveProfile`-Pfad vorbereitet (Markierung des gesperrten Modells an der Subengine).
- Pflicht-Tests grün.
