# Umsetzungsplan G3 — Similarity / Cosine-Scorer

> Paket **G3** aus `resident_planplan.md`. Quellen: `resident_first.md` (Similarity), `context/context_g.md` (§G3). Abhängt von: **G1** (normierte Vektoren), **G2** (Index/Scope-Substrat), **C3** (Planner-Expansion). Ergebnis: Similarity-Konfiguration + Scorer.
>
> **Legende:** ✅ verifiziert · ⚠️ analysiert/zu messen · 🔌 Erweiterungspunkt.

---

## 1. Zweck

Standard-Ähnlichkeitspfad: **Cosine** über L2-normierte e5-Vektoren, optional eingeschränkt auf ein **Scope-Subset**, mit `top_k` und den **Entscheidungsgrundlagen** (Anzahl, Score-Abstand, Streuung, Task-ID), aus denen die **äußere Schicht** entscheidet, ob ein Folgetask (z. B. Rerank) eingereiht wird.

**Vertragslage (✅):**
- Scorer ist **Parameter**: `Cosine` | `CrossEncoder(modell)`. **CrossEncoder ist für die aktuelle Umsetzung nicht vorgesehen** (`resident_first.md`) → nur 🔌 Erweiterungspunkt (Gruppe H3).
- **Rerank ist kein eigener Tasktyp**, nur Similarity mit anderem Scorer auf einem Subset.
- Die Engine liefert **Rohwerte + Entscheidungsgrundlagen**; die **Entscheidungslogik liegt in der äußeren Schicht** (vor der Resident-Engine). Zurückgegeben werden **Zieldatei + Offsets + Entry-IDs** wie im Index hinterlegt (G2-Payload).

---

## 2. Cosine = Skalarprodukt (✅) + Fallstricke (⚠️)

Für `‖a‖=‖b‖=1` gilt `cos(a,b)=a·b` (Referenz: `embeddings @ embeddings.T`). Konkrete Umsetzungsregeln:

1. **f16-Speicherung zerstört exakte Norm 1** (Abweichung ~1e-3) → Dot-Product leicht ≠ Cosine. **Akkumulation in f32:** f16→f32 **upcasten vor** der Summe.
2. **Optional** nach Upcast re-normalisieren oder Norm beim Scoring mitführen, wenn exakte Cosine nötig.
3. **Zero-Vektor** (leerer Text) vermeiden → sonst Division durch 0 in `normalize_l2`. **Epsilon** ergänzen (greift bereits in G1; im Scorer defensiv prüfen).

**Score-Skala (✅):** e5-Scores clustern zwischen ~0,7 und 1,0 (InfoNCE-Temperatur 0,01). **Es zählt die Rangfolge, nicht der Absolutwert** → **absolute Schwellwerte sind schlecht übertragbar**; relative Maße verwenden.

---

## 3. Zu implementierende Artefakte

### 3.1 Scorer — `src/score/cosine.rs`
- Cosine über Scope-Kandidatenmenge; f32-Akkumulation; `top_k`-Reduktion (Heap/Partial-Sort).
- Rückgabe pro Treffer: `Hit { entry, score, outdated }` (B4-/Manager-Vertrag), angereichert um G2-Payload (Zieldatei + Zeichenoffsets + Entry-ID) auf der Ergebnis-Ebene.

### 3.2 Similarity-Parameterobjekt
- `scorer: Scorer` — Enum `Cosine` | `CrossEncoder(ModelId)` (letzteres 🔌, nicht aktiv).
- `scope: Scope` — optionales Index-Subset. Repräsentation als **kompakte Entry-Menge** (Range/Bitmap) gemäß `resident_index.md` (**nicht** als ID-Liste). Kann ID-Liste, Bereich oder **Ergebnis eines früheren Tasks** sein (Addon-Muster, kein Kern).
- `top_k: u32`.
- `precision: f32` (aus F4, optional) — durchreichen, Scorer selbst ist präzisionsneutral.
- Erweiterbar halten (`resident.md`: zukünftige Einstellungen).

### 3.3 Entscheidungsgrundlagen (für die äußere Schicht)
Die Engine berechnet und liefert (keine Entscheidung!):
- `count` — Kandidatenzahl.
- `s1`, `sk` — Top-1 und Top-k Score.
- `mean`, `std` — über die Top-k-Scores.
- `task_id`.
- Top-k-Liste mit `(entry, score, payload)`.

Optionale Kennzahlen (⚠️ keine Norm, als Diagnose anbieten; Schwellen bleiben außen):
- **Margin/Gap:** `s1 − s2`, `s1 − sk` (kleiner Gap → Reihenfolge unsicher).
- **Streuung:** Std/IQR der Top-k; **z-Score** von s1 gegen Top-k-Verteilung.
- **Softmax/Temperatur** über Top-k (T≈0,01–0,05 passend zur e5-Skala) → Masse Top-1 bzw. **Entropie**.
- **Elbow/Knick** in der sortierten Scorekurve.

---

## 4. Integration mit C3 (Planner)

- Der **Planner** expandiert einen Similarity-Task zustandslos in eine Op-Folge; G3 liefert die **Scorer-Regel** für die Expansion (`Cosine` → Dot-Product-Op über Scope-Segment, dann Top-k-Reduktion/Merge).
- **Rerank** entsteht nur dadurch, dass die äußere Schicht einen **zweiten Similarity-Task** mit anderem Scorer und eingeschränktem Scope einreiht — über den **normalen** Queue-Weg (Addon-Muster, `resident_index.md`). Kein neuer Tasktyp, keine Scheduler-seitige Task-Beziehung.
- Scorer-Austausch liegt **hinter demselben Op-/Scorer-Trait** → 🔌 CrossEncoder/MaxSim später ohne API-Bruch.

---

## 5. Scope-Behandlung

- Scope-Auflösung macht der **Manager** (E2), nicht G3. G3 bekommt eine aufgelöste Kandidatenmenge (`Variable`-Layout / Entry-Set).
- **Kein stilles Ausweichen:** nicht deklariertes Verfahren oder fehlender Resolver → `UnsupportedScope` (Q1).
- Ergebnisse auf entfernte Entries werden verworfen; Entries mit anstehendem Update → `outdated` (Diagnose).

---

## 6. Cross-Encoder — nur Erweiterungspunkt (🔌, NICHT implementieren)

Für spätere Umsetzung (H3) dokumentieren, Hooks sichern:
- Modell z. B. `BAAI/bge-reranker-v2-m3` (Cross-Encoder, XLM-R-`SequenceClassification`, multilingual, Apache-2.0, ~0,6B, F32).
- Query+Passage **gemeinsam** als Paar → **ein Relevanz-Logit pro Paar** (kein Embedding, kein Cosine), optional `sigmoid`→[0,1]. Höher = relevanter.
- Kosten O(Paare), kein Vorab-Index → nur auf kleinem Top-k-Subset; Scores **nicht** mit Cosine vergleichbar.
- Lazy laden, TTL entlädt. candle-Einbindung (XLM-R-SeqClass) ⚠️ nicht geprüft.
- **Hook:** Scorer-Enum-Variante `CrossEncoder(ModelId)` existiert im Parameterobjekt, bleibt in Phase 1 unimplementiert / liefert klaren Fehler.

---

## 7. Fehlerbehandlung (Q1) & Logging (Q2)

- **Fehler:** `UnsupportedScope`, `EmptyEmbedding`/Zero-Vektor, `InvalidPrecision` (NaN); CrossEncoder-Variante → klarer „nicht verfügbar"-Fehler.
- **Logging:** Span pro Similarity-Task mit `scope_kind`, `candidate_count`, `top_k`, `s1`, `sk`, `mean`, `std`, `scorer`, `discarded`/`outdated`-Zählern.

---

## 8. Teststrategie (Q3)

- **Cosine-Korrektheit:** gegen normierte Golden-Vektoren (aus G1); f32-Akkumulation; Ergebnis = Python-Referenz `embeddings @ embeddings.T` (×100 optional).
- **f16-Pfad:** Scoring mit f16-Storage → f32-Upcast; `cos ≥ 0.999` ggü. f32-Referenz; Top-k-Overlap.
- **Zero-Vektor:** kein NaN/Inf.
- **Entscheidungsgrundlagen:** gap/std/entropy auf deterministischem Mini-Korpus; Werte stabil.
- **Scope:** Subset schränkt Kandidaten korrekt ein; `UnsupportedScope` bei fehlendem Resolver.
- **Ergebnisstatus:** entfernt → verworfen; anstehendes Update → `outdated`.

---

## 9. Definition of Done

- Cosine-Scorer mit f32-Akkumulation und Epsilon-Schutz; `top_k`-Reduktion.
- Similarity-Parameterobjekt mit `scorer`/`scope`/`top_k`/`precision`, erweiterbar; Scope als kompakte Entry-Menge.
- Engine liefert Rohwerte + Entscheidungsgrundlagen (count, s1, sk, mean, std, task_id, Top-k-Liste mit Payload); **keine** Rerank-Entscheidung in der Engine.
- Rerank nur als zweiter Similarity-Task über die normale Queue (Addon), kein neuer Tasktyp.
- CrossEncoder/MaxSim als 🔌 Erweiterungspunkt gesichert, nicht implementiert.
- Q1/Q2/Q3 erfüllt; No-GPU-Tests grün.

## Result
Ich lag falsch: Die Produktions-Ausführung fehlt nicht ganz. Meine Suche hatte nur ein Treffer-Limit erreicht, und ich hatte `planner/adapter.rs` nicht gelesen.

**Was es gibt**
- `PlannerExpander` plant die Op-Kette `Similarity → TopK`. Am Ende wandelt `into_output` die Hits in `Output::Hits(Vec<(EntryId, f32)>)` um.
- Dieser Pfad ist in `planner/test/adapter.rs` getestet. Welche Engine ihn in Produktion tatsächlich bedient, habe ich nicht geprüft.

**Was fehlt**
- Der Report aus G3 ist an diesen Pfad nicht angeschlossen. `HostData::Hits` trägt nur `(u64, f32)`. Es fehlen:
  - `outdated`
  - die Anzahl verworfener Treffer
  - die Kandidatenzahl
  - die Payload (Zieldatei und Offsets)
  - die Task-ID
- `into_output` liefert deshalb weiterhin nur `Output::Hits`. Meine Aussage in der Zusammenfassung, der Report sei noch nirgends angeschlossen, stimmt, aber der Grund war falsch begründet.
- Wegen der Lücke ist die Plan-DoD „Engine liefert Rohwerte + Entscheidungsgrundlagen (count, s1, sk, mean, std, task_id, Top-k-Liste mit Payload)" für den Laufzeitpfad noch nicht erfüllt.

**Für den Anschluss nötig**
- Die Similarity-Op muss mehr als `(u64, f32)` weitergeben: das `Hit` des Managers (inklusive `outdated`), die Kandidaten- und Verworfen-Zähler und einen Payload-Resolver.
- `into_output` baut daraus `SimilarityReport::build` und gibt `Output::Similarity` zurück.
- Das berührt `HostData` in `op/` und den Engine-Pfad, den ich noch nicht gelesen habe.

Der Laufzeitpfad ist jetzt angeschlossen. `resident::`-Tests grün: 584 ohne und 598 mit `--features cuda` (34 ignoriert). Der Lib-Build zeigt keine Fehler oder Warnungen in den geänderten Dateien. Die zwei Warnungen im Testlauf stammen aus `subengine/exec.rs` und `manager/seqlock.rs`, die ich nicht angefasst habe.

**Neuer Datenvertrag (`op/mod.rs`)**
- `ScoredHit` trägt `entry`, `score`, `outdated` und die Quell-Payload (Zieldatei, Zeichenoffsets).
- `ScoredHits` enthält zusätzlich die Zähler `candidates` und `discarded`.
- Beide stecken in der neuen Variante `HostData::Scored`. Die Variante `HostData::Hits` bleibt unverändert.

**Adapter (`planner/adapter.rs`)**
- Der `Collect`-Closure bekommt jetzt die Task-ID und die `SimilarityParams` mit.
- `into_output` wandelt `Scored` über `SimilarityReport::build` in `Output::Similarity` um.
- Der Report enthält Top-k-Liste mit Payload, `s1`, `sk`, `mean`, `std`, `task_id` und die Zähler. Auch das Logging läuft darüber.
- Ohne Parameter schlägt der Task mit einem klaren Fehler fehl.
- Der alte Pfad `HostData::Hits` → `Output::Hits` bleibt unverändert, damit die Addon-Kaskade und ihre Tests weiter funktionieren.

**Tests**
- Der neue End-to-End-Test läuft durch Planner und Processor bis zum Output. Er prüft:
  - Sortierung, `top_k`-Kürzung und Payload
  - `task_id`, Kandidatenzahl, `discarded`, `s1` und `sk`
- `missing_result_is_an_error` deckt zusätzlich ab, dass `Scored` ohne Parameter scheitert.

Es gibt weiterhin keine produktive Engine, die `HostData::Scored` tatsächlich liefert. Im Repo erzeugt nur die Test-Engine `Scripted` die End-Daten der TopK-Op. Die echte Engine fehlt im Code, den ich gelesen habe. Sie müsste die Treffer des Managers (`run_slot` → `Hit`) und Payload und Zähler aus dem Index in `ScoredHits` übersetzen. Ich habe diesen Anschluss nicht gebaut, weil ich die Engine-Schicht dafür nicht gefunden habe. Falls sie anderswo liegt, sag mir wo.
