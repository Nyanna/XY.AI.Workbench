# Umsetzungsplan F3 — Profilbildung

> Gruppe F · Paket F3. Quelle: `resident_precision.md` (Abschnitt „Profilbildung“). Kontext: `context/context_f.md` §F3 (Q15–Q18). Querschnitt: `plan_f0_overview.md` §4.
> **Hängt ab von:** F2 (Pareto-Front). **Liefert an:** F4 (Profile zum Matchen), F5 (zu persistierende Profile).

## 1. Zweck & Ergebnis
Aus der Pareto-Front leitet F3 **unbenannte Profile** ab: `Profile { position: f32 ∈ [0,1], config }`. Die Anzahl ist variabel (1…n) und ergibt sich **allein aus dem Spread** der Front. Der **Spread-Schwellwert in normierter Skala** ist die **einzige** Konfiguration. Ergebnis: Funktion `pareto_front → Vec<Profile>` + Positionsberechnung über Bogenlänge.

## 2. Zieltypen (Skizze; B6 ist formaler Eigentümer von `Profile`)
```rust
pub struct Profile {
    pub position: f32,   // p ∈ [0,1], 0 = schnellstes, 1 = genauestes
    pub config: Cfg,     // die zugehörige Kombination (aus F1/F2)
}

pub struct ProfileConfig { pub spread_threshold: f64 } // EINZIGER Parameter, normierte Skala

pub fn derive_profiles(front: &[ParetoPoint], cfg: &ProfileConfig) -> Vec<Profile>;
```

## 3. Verfahren (verbindlich aus `resident_precision.md`)
1. **Normieren:** Zeit und Abweichung jeweils auf `[0,1]`. Zeit **logarithmisch** (`log(t)`), da Größenordnungsspannen. [context §F3/Q15]
   - Fallstricke absichern: Division durch 0 bei **Einpunkt-Front** (Konvention, siehe 4); Ausreißer dominieren Min/Max; Skalenwahl ändert Abstände → Skala fix dokumentieren.
2. **Bogenlänge** entlang der Front: `s_i = Σ_{j≤i} ‖P_j − P_{j−1}‖` (euklidisch im normierten Raum), `p_i = s_i / s_n`. **Robuster als Rang**, weil echte Tradeoff-Sprünge erhalten bleiben (Rang verzerrt Abstände). [§F3/Q16]
3. **Redundanz zusammenfassen:** 1D-**Single-Linkage entlang `p`** — Nachbarn mit `Δp < spread_threshold` verschmelzen. Repräsentant = schnellster (oder mittlerer) Punkt des Clusters. **Schwelle ist der einzige Parameter.** [§F3/Q17]
4. **Positionen setzen:** schnellstes Profil `p = 0`, genauestes `p = 1`, übrige proportional zur kumulierten Distanz. Bei genau **einem** Profil gilt `p = 0.5` per Konvention.
5. **Redundante Profile nicht aktivieren:** praktisch identische Zeit **und** identisches Ergebnis (z. B. derselbe Emulationspfad) → nicht aktivieren.
6. **Ordnung/Herabstufung:** Profil-Ordnung ergibt sich aus der Messung. Der **umgekehrte Fall** (reduzierte Genauigkeit, die durch Emulation **langsamer** ist) wird erkannt und **herabgestuft oder nicht aktiviert**.

## 4. Emulations-/Dominanzerkennung (context §F3/Q18 [W])
- Eine Kombination ist **dominiert**, wenn sie **langsamer und nicht genauer** als der fp32-Referenzpfad ist → aus der Front entfernen.
- Zusatz-Plausibilität: „**Zeit ≥ Zeit des höherpräzisen Pfads**“ bei geringerer Präzision ⇒ Emulation ⇒ herabstufen/nicht aktivieren.
- (Die eigentliche Pareto-Dominanz filtert F2 bereits; F3 wendet die Regel nochmals nach der Zusammenfassung an, um redundante Emulationspunkte sicher auszuschließen.)

## 5. Normierungs-Robustheit (Pflicht-Sonderfälle)
- **Einpunkt-Front:** direkt ein Profil mit `p = 0.5`, keine Normierung nötig.
- **Zwei identische Koordinaten:** Spannweite 0 in einer Achse → diese Achse trägt 0 zur Bogenlänge bei (keine Division durch 0 erzwingen; Guard einbauen).
- Normierte Skala ist **modell- und hardwareunabhängig** → `spread_threshold` gilt engineübergreifend gleich, obwohl Profile pro Engine gebildet werden.

## 6. Sperre ohne aktives Profil
- Bleibt nach Bildung/Aktivierung **kein** Profil übrig (alles verworfen/inaktiv), ist die Engine **für dieses Modell gesperrt** (`NoActiveProfile` tritt dann lazy im Matching/ersten Task auf, siehe F4).

## 7. Querschnitt
- **Fehler (Q1):** F3 selbst wirft i. d. R. nicht; eine leere aktive Profilmenge wird als Zustand an F4/F5 weitergegeben (→ `NoActiveProfile` dort).
- **Logging (Q2):** `info!` mit Anzahl abgeleiteter Profile und deren Positionen; `debug!` je Cluster (verschmolzene Punkte, Repräsentant, Grund für Nicht-Aktivierung).
- **Feature-Flags (Q4):** rein rechnerisch, **GPU-frei**, kein `cfg`.

## 8. Teststrategie (Q3, `resident_precision.md` „Tests“)
- **Spread-Schwellwert:** steuert Profilanzahl (kleine Schwelle → mehr Profile; große → Zusammenfassung).
- **Zusammenfassen redundanter Punkte:** nahe Punkte (Δp < Schwelle) ergeben ein Profil; Repräsentant korrekt gewählt.
- **Positionsberechnung über Bogenlänge:** konstruierte Front mit ungleichen Abständen → Positionen folgen kumulierter Distanz, nicht dem Rang.
- **Einzelprofil:** `p = 0.5`.
- **Emulation/Herabstufung:** langsamer-und-ungenauer-Punkt wird entfernt/nicht aktiviert.
- Alle Tests **deterministisch**, ohne GPU.

## 9. Verifikationsergebnisse

Geprüft per Codeinspektion (`precision/pareto.rs`), numerischem Experiment an konstruierten Fronten und realer Messung auf GTX 1660 SUPER (TU116, sm_75, Treiber 610.57.04, CUDA 13.3) sowie CPU. Die Messungen sind als `#[ignore]`-Gates im Repo abgelegt: `gate_encoder_dtype_front` (e5-small-Encoder, `model/test/candle_adapter.rs`; mit `GATE_CPU=1` auf CPU) `gate_cosine_dtype_front` (vorhandener Scoring-Kernel) und `gate_cosine_bandwidth_front` (Scoring-Kernel mit einem Warp pro Dokument für f32, f16 und int8 über `dp4a`), beide in `kernel/test/cuda.rs`.

### 9.1 Reale Fronten
Zeit = Median interleaved Runden, Abweichung gegen den fp32-Pfad derselben Eingaben.

| Pfad | Variante | Median | Abweichung |
|------|----------|--------|------------|
| GPU-Encoder (e5-small, 64 Texte, seq 209) | F32 | 621 ms | 0 (Referenz) |
| | F16 | 1528 ms (2,46× langsamer) | L2 mittel 5,3e-4, max 6,9e-4 |
| | BF16 | nicht verfügbar (`CUDA_ERROR_NOT_FOUND`, Kernel-Symbol fehlt auf sm_75) | – |
| CPU-Encoder (2 Texte, Debug-Build, nur Verhältnis aussagekräftig) | F32 | 2280 ms | 0 (Referenz) |
| | F16 | 4208 ms (1,85× langsamer) | L2 mittel 6,9e-4 |
| | BF16 | nicht verfügbar (`unsupported dtype BF16 for op matmul`) | – |
| GPU-Scoring `cosine_dot` (200 000 Dokumente, dim 384, 8 Queries) | F32 | 6,78 ms | max 1,0e-7 (f32-Rundung) |
| | F16 | 6,79 ms (kein Unterschied; min 6,02 vs. 6,13 ms) | max 4,8e-5, mittel 7,1e-6 |
| GPU-Scoring, Warp pro Dokument (200 000 Dokumente, dim 384, 8 Queries) | F32 | 2,000 ms (154 GB/s gelesen) | max 6,7e-8 (f32-Rundung) |
| | F16 | 1,923 ms (4 % schneller) | max 4,8e-5, mittel 7,1e-6 |
| | int8 (`dp4a`, Skala pro Vektor) | 1,177 ms (1,70× schneller) | max 5,5e-4, mittel 1,2e-4 |

Befunde:
- **Encoder:** F16 ist auf beiden Engines langsamer **und** ungenauer als F32 und damit dominiert; BF16 fällt aus. Die Emulations-/Dominanzregel aus §4 tritt mit realen Daten ein, nicht nur im Test. Die Front besteht jeweils aus genau dem fp32-Pfad.
- **Scoring, vorhandener Kernel:** Ein Block pro Dokument ist latenzlimitiert (Reduktion und Synchronisation je Query); F16 ist dort nicht schneller und ungenauer, also dominiert. Die Zeitdifferenz liegt innerhalb des Rauschens (Spanne min–max beim Lauf 6,0–9,0 ms).
- **Scoring, Warp pro Dokument:** Die Zeiten streuen hier um unter 1 % (min 1,919 bis Median 1,923 ms bei F16), die Differenzen sind echt. Es entsteht eine echte Dreipunkt-Front: int8 (1,177 ms, 5,5e-4), F16 (1,923 ms, 4,8e-5), F32 (2,000 ms, Referenz). `dp4a` läuft auf TU116 und ist 1,70× schneller als F32; die Messung belegt damit den Durchsatz, den die Fähigkeitserhebung bisher nur annahm. Der Kernel erreicht nur 66 GB/s (int8) bzw. 154 GB/s (f32) gelesen, ist also nicht bandbreitenlimitiert.
- **Konsequenz Frontgröße:** Die Frontgröße hängt vom Pfad ab: Encoder (candle) auf TU116 einen Punkt, Scoring mit `dp4a`-Kernel drei Punkte. Der Einpunkt-Fall (`p = 0.5`, F4 liefert für jeden Wunsch dieses Profil) ist ein Regelfall und kein Randfall und wird als solcher getestet.
- **Konsequenz Rauschen:** Zeitgleiche Punkte (wie F16 im vorhandenen Scoring-Kernel) können durch Rauschen knapp „schneller“ gemessen werden und dann fälschlich auf der Front landen. Die Dominanzprüfung vergleicht Zeiten deshalb mit der von F2 gelieferten Messunsicherheit (Zeiten innerhalb dieser Spanne gelten als gleich); ein ungenauerer Punkt ohne signifikant geringere Zeit wird verworfen. Die Unsicherheit stammt aus F2 und ist kein F3-Parameter.
- **Konsequenz Verfügbarkeit:** Ein nicht ausführbarer Pfad (BF16) wird erst bei der Messung sichtbar, nicht bei der Erhebung der Fähigkeiten; er gilt als verworfene Kombination (`Incompatible`), nicht als Fehler der Kalibrierung.
- **Abweichungsmetrik:** `1 − cos` liegt bei f32-Ausgabe im Rauschen (≈1e-7, quadratisch in der Winkelabweichung) und trennt F16 nicht von F32. Als Abweichung dient die L2-Distanz normierter Vektoren, in f64 berechnet (linear, hier 5e-4 für F16).

### 9.2 Normierungs- und Bogenlängenformeln — bestätigt
- Min-max-Normierung mit Guard (`range > 0`, sonst 0-Vektor), Einpunkt-Front → `[0.5]`, Bogenlänge via `hypot`, `s = 0` → gleichmäßige Rangpositionen: alle Sonderfälle aus §5 sind abgedeckt, keine Division durch 0, `p_0 = 0`, `p_n = 1`.
- Die Front aus `pareto()` ist nach Zeit aufsteigend und Abweichung strikt fallend sortiert; die Positionen sind monoton, `p = 0` ist das schnellste Profil.
- Bogenlänge statt Rang bestätigt: Front `(0,1.0) (1,0.9) (10,0.0)` liefert für den Mittelpunkt `p < 0.5`; ein Rangverfahren lieferte exakt 0.5.
- Invariant gegen gleichmäßige Skalierung aller Zeiten, also unabhängig von der absoluten Hardwaregeschwindigkeit.

### 9.3 Skala: `log(Zeit)`, Abweichung linear — festgelegt
- Zeit logarithmisch: Abstände bilden Faktoren ab (2× schneller zählt überall gleich). Fällt auf linear zurück, wenn eine Zeit `≤ 0` ist. Beispiel realer Größenordnung: F16-Encoder 2,46× und F16-CPU 1,85× liegen im selben Maßstab wie Sprünge auf konstruierten Fronten.
- Abweichung linear: Die fp32-Referenz hat Abweichung 0, `log` ist dort undefiniert; ein Epsilon-Offset wäre eine zusätzliche Konstante.
- Konstruierte Front `(4.0,2e-2) (7.5,1.5e-3) (14.0,0)`: mittleres Profil bei `p = 0.602` (linear) bzw. `0.676` (log). Log ist nicht ausreißerrobuster (ein 10× langsamerer Referenzpfad verschiebt den Mittelpunkt linear um 0.116, log um 0.143); die Wahl begründet sich allein mit der Verhältnissemantik. Zeitausreißer fängt F2 ab.
- Die Skala ist fix; einziger Parameter bleibt `spread_threshold`.

### 9.4 Single-Linkage-Schwelle — Default 0.1, an realer Front bestätigt
Reale Dreipunkt-Front des Scorings (Zeit logarithmisch, Abweichung linear, Referenz mit Abweichung 0):
- Positionen: int8 `p = 0.000`, F16 `p = 0.919`, F32 `p = 1.000` (linear in der Zeit: 0.910; die Wahl der Zeitskala ändert das Ergebnis nicht).
- Abstand F16–F32: `Δp = 0.081`. Für `spread_threshold > 0.081` verschmelzen F16 und F32 zu einem Cluster (Repräsentant F32 nach §9.5), es bleiben zwei Profile: schnell (int8, `p = 0`) und genau (F32, `p = 1`). Für `spread_threshold < 0.081` bleiben drei Profile.
- F16 ist nur 4 % schneller als F32 bei 4,8e-5 Abweichung, bietet also keinen nennenswerten Trade-off; die Verschmelzung ist das sinnvolle Ergebnis. `0.1` liegt über 0.081 und liefert es, `0.2` liefert dasselbe (konstruierte dichte Front: ebenfalls drei Cluster bei 0.1 und 0.2). Der Default bleibt `0.1`.
- Der Abstand 0.081 liegt nahe an 0.1; kleine Änderungen der Messwerte können F16 als eigenes Profil erscheinen lassen. Das ist hinnehmbar: Beide Ausgänge sind gültige Profilmengen, und F4 liefert in beiden Fällen für jeden Wunsch ein Profil.
- Single-Linkage kann dichte Ketten zu einem Cluster verbinden, auch wenn dessen Ausdehnung die Schwelle überschreitet. Das ist gewollt; Test mit gleichmäßig dichter Front (Erwartung: ein Profil).

### 9.5 Repräsentantenwahl — festgelegt
- Repräsentant ist der schnellste Punkt des Clusters; das Cluster mit dem genauesten Frontpunkt wird durch diesen Punkt vertreten, sonst könnte der fp32-Pfad in einem Cluster aufgehen und `precision = 1` die höchste Genauigkeit verfehlen. Ein „mittlerer“ Punkt hätte keinen Vorteil, da der Repräsentant ein real gemessener Punkt sein muss.
- Positionen werden nach dem Zusammenfassen über die Bogenlänge der Repräsentanten neu berechnet, damit `p = 0` das schnellste und `p = 1` das genaueste verbleibende Profil ist.

### 9.6 Codebestand
- `positions` und `pareto` arbeiten auf `(f64, f64)`-Tupeln. `derive_profiles` bildet Tupel auf Konfigurationen zurück und erzeugt `Profile::new(position, config)` mit `ProfileConfig` aus `precision/profile.rs`.
- Noch nicht vorhanden: Linkage, Repräsentantenwahl, Emulations-/Dominanzregel mit Messunsicherheit und `derive_profiles` selbst.

## 10. Definition of Done
- `derive_profiles` implementiert: Normierung (log-Zeit) → Bogenlänge → Linkage → Positionen, inkl. Sonderfälle (Einpunkt, identische Koordinaten).
- `spread_threshold` ist der einzige Konfigparameter, in normierter Skala.
- Emulations-/Herabstufungsregel umgesetzt; redundante Profile nicht aktiviert.
- Pflicht-Tests grün. Liefert `Vec<Profile>` an F4 und F5.

## Result

F3 ist umgesetzt. Die 105 Tests im `precision`-Modul laufen durch, darunter 11 neue für F3. `cargo check --features cuda` läuft ebenfalls durch. Ich habe die Gates auf der GPU nicht neu ausgeführt und die Clippy-Warnungen nicht geprüft.

**`profile.rs`**
- `derive_profiles(front, &ProfilingConfig) -> Vec<Profile>` läuft in dieser Reihenfolge:
  1. **Dominanzprüfung:** Ein Punkt fällt weg, wenn ein genauerer Punkt innerhalb der Messunsicherheit gleich schnell ist.
  2. **Positionen:** Sie kommen aus `positions` mit logarithmischer Zeit und Bogenlänge.
  3. **Single-Linkage:** Nachbarn mit `Δp < spread_threshold` verschmelzen zu einem Cluster.
  4. **Repräsentant:** Es wird der schnellste Punkt des Clusters gewählt. Das Cluster mit dem genauesten Punkt wird durch diesen vertreten.
  5. **Neuberechnung:** Die Positionen werden über die Repräsentanten neu berechnet.
- Ein einzelnes Profil bekommt `p = 0.5`, eine leere Front liefert einen leeren Vektor.
- `ProfilingConfig { spread_threshold }` mit Default `0.1` ist der einzige Parameter. Ich habe es so genannt, weil `ProfileConfig` schon die Achsenkombination bezeichnet.
- Geloggt wird mit `info!` (Anzahl und Positionen) und `debug!` (je Cluster und je verworfenem Punkt).
- Der Export steht in `mod.rs`.

**F2-Ergänzung für die Messunsicherheit**
- Das neue Feld `uncertainty` steht in `MeasureResult` und `ParetoPoint`.
- Es ist die relative Streuung der Zeitreihe. Bei zu wenigen Samples gilt `dominance_margin` als Ersatz.

**Neue Tests in `test/profile.rs`**
- Sie decken ab:
  - leere Front
  - Einzelprofil
  - Schwelle steuert die Profilanzahl
  - die reale Dreipunkt-Front (int8, F16, F32)
  - Repräsentantenwahl
  - dichte Kette
  - Bogenlänge statt Rang
  - Rauschen
  - identische Koordinaten
  - nicht endliche Punkte