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

## 9. Offene Verifikation / Risiken
- `[W]` gesamter Abschnitt F3 ist Fachwissen ohne frische Quellenprüfung → Normierungs-/Bogenlängen-Formeln und die Single-Linkage-Schwelle an echten Fronten (aus F2-Läufen auf e5-small) validieren.
- Wahl „log(Zeit)“ vs. lineare Zeit beeinflusst Abstände/Positionen → fixieren und dokumentieren.
- Repräsentantenwahl (schnellster vs. mittlerer) konsistent festlegen (empfohlen: schnellster, damit `p`-Richtung monoton bleibt).

## 10. Definition of Done
- `derive_profiles` implementiert: Normierung (log-Zeit) → Bogenlänge → Linkage → Positionen, inkl. Sonderfälle (Einpunkt, identische Koordinaten).
- `spread_threshold` ist der einzige Konfigparameter, in normierter Skala.
- Emulations-/Herabstufungsregel umgesetzt; redundante Profile nicht aktiviert.
- Pflicht-Tests grün. Liefert `Vec<Profile>` an F4 und F5.
