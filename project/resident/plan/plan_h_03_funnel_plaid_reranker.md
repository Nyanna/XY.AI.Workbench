# Umsetzungsplan H3 · Funnel/PLAID & Cross-Encoder/Reranker

**Gruppe H — Ausblick/zurückgestellt. Status I/A** (Thesen-§7: über Addons + Centroid-ScopeResolver realisierbar). `resident_first.md`: Cross-Encoder „für die aktuelle Umsetzung nicht vorgesehen".

**Planungsauftrag:** NUR Erweiterungspunkte sichern. Weder PLAID-Funnel noch Reranker-Forward werden jetzt gebaut.

## 1. Was später kommt (Kurzriss, nicht umsetzen)

- **Funnel/Cascade:** Stufe 1 billiger Filter auf Centroid-Codes → Stufe 2 exaktes MaxSim auf der Spitze (`resident_subengine.md` §4.5, §11). Als **Addon** über die normale Queue, nicht als Kernfunktion (`resident_index.md` Addons).
- **Cross-Encoder/Reranker:** eigener **Inferenz**-Tasktyp bzw. Similarity mit anderem Scorer auf einem Subset; Modell lazy geladen, TTL entlädt (`resident_first.md`, `resident_subengine.md` §11).

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H3)

- **PLAID** (Santhanam et al., CIKM 2022, arXiv 2205.09707; Repo `stanford-futuredata/ColBERT`): (1) Kandidaten aus `nprobe` nächsten Zentroiden je Query-Token, (2) Centroid-Pruning `t_cs`, (3) Centroid-Interaction (ohne Residual-Dekompression, optional 2. Stufe `ndocs`), (4) Residual-Dekompression + volles MaxSim auf Top-Kandidaten. `next-plaid` unbelegt **[prüfen]**.
- **ColBERTv2-Kompression** (arXiv 2112.01488): K-Means → Zentroidtabelle (~`2^⌊log2(16·√N)⌋`), pro Token Zentroid-Index (Code) + Residual (Embedding−Zentroid) auf 1–2 Bit quantisiert (`nbits`).
- **Cross-Encoder `BAAI/bge-reranker-v2-m3`:** XLM-RoBERTa-Backbone (bge-m3, ~568 M), Eingabe `[CLS] query [SEP] doc [SEP]`, ein Logit. fp32 ~2,3 GB / fp16 ~1,1 GB (passt in 6 GB, aber nicht parallel zu großen Indizes). `candle-transformers` enthält XLM-RoBERTa inkl. Sequence-Classification **[prüfen]**; `model.safetensors` im HF-Repo **[prüfen]**.
- **Cascade-Steuerparameter:** `nprobe`, `t_cs`, `ndocs`, Top-k je Stufe, `nbits`; Stufentiefe = Recall/Latenz-Tradeoff, pro Query-Klasse konfigurierbar.

## 3. Zu sichernde Erweiterungspunkte (Checkliste — darf jetzt nicht verbaut werden)

1. **Centroid-ScopeResolver ist pluggable (Hook vorhanden).** `Scope::Centroid { k, radius }` und `AccessMethod::Centroid` existieren bereits; `Index::summary(Centroid)` liefert die Zentroidtabelle, `Index::members(cell)` die Inverted List (`resident_index.md`). Sicherstellen: E2 registriert Resolver pro `AccessMethod`, Kompatibilitätsprüfung „Verfahren deklariert UND Resolver vorhanden", sonst `UnsupportedScope` (kein stilles `All`).
2. **Optionales Centroid-Code-Feld pro Token im Slot.** `resident_subengine.md` §4.4: „Optionales Feld pro Token: Centroid-ID/Code (für späteren Funnel, vom Kernel ignoriert, wenn nicht genutzt)." Slot-/Segment-Layout muss dieses Feld reservieren können, ohne den aktuellen Kernel zu ändern.
3. **Scorer-Parameter ist erweiterbar.** `resident_first.md`: Scorer = Parameter `Cosine | CrossEncoder(model)`. Enum `#[non_exhaustive]`; Rerank ist **kein eigener Tasktyp**, sondern Similarity mit anderem Scorer auf einem Subset. Entscheidungsgrundlagen (Anzahl, Score-Abstand Top-1..Top-k, Streuung, Task-ID) werden mitgeliefert, damit die Oberschicht einen Folgetask bilden kann.
4. **Inferenz-Tasktyp + Lazy/TTL-Modellplatz.** Der Inferenz-Tasktyp der Fassade (`resident.md`) und der Ressourcen-Lifecycle `Unloaded→…→Idle→Evicting` mit TTL (`resident_subengine.md` §6) müssen ein zusätzliches Reranker-Modell (eigene Gewichte, Forward pro Paar, `resident_subengine.md` §11) lazy aufnehmen können, ohne Fassaden-/Task-Schema zu ändern.
5. **Addon-/Cascade-Pfad über die normale Queue.** `resident_index.md` Addons: Iteration/Kaskade/Rekursion sind **keine** Kernfunktion; ein Addon reiht Folgetasks mit kopierten/geänderten Parametern ein (z. B. Centroid-Task → Folgetask mit Zell-Mitgliedern als Scope). Nur das erweiterbare Parameterobjekt sichern; keine Eltern-Kind-Beziehung im Kern.
6. **Cascade-Parameter im Parameterobjekt vorsehen.** `nprobe`/`t_cs`/`ndocs`/`nbits`/Top-k-je-Stufe als spätere Felder des erweiterbaren Params — nicht jetzt definieren, nur nicht ausschließen.

## 4. Jetzt NICHT umsetzen

Kein K-Means/Zentroid-Training, keine Residual-Quantisierung, kein Funnel-Kernel, kein Cross-Encoder-Forward, kein Laden von bge-reranker. Der Standardpfad bleibt dense e5-small + Cosine/Top-k (`resident_first.md`).

## 5. Offene Verifikationen ([prüfen])

- `candle-transformers` XLM-RoBERTa-Classifier verfügbar + Gewichtsnamen.
- `next-plaid`-Repo/Lizenz/Status.
- VRAM-Budget: Reranker (fp16 ~1,1 GB) vs. residenter Index gleichzeitig.

## 6. Definition of Done

- Erweiterungspunkte 1–6 sind in E2 (ScopeResolver/Centroid), D7/§4.4 (Slot-Feld), C1/C3 (Scorer-Enum, Inferenz-Tasktyp), E9 (Addon) als Anforderung/offene Abstraktion vorhanden.
- Ein No-GPU-Test belegt: unbekannter/unregistrierter Centroid-Resolver liefert `UnsupportedScope`; Scorer-Enum und Params lassen sich um eine Variante/ein Feld erweitern, ohne bestehende Pfade zu brechen.
