# Umsetzungsplan H2 · Gleitende Indizierung (Sliding Window)

**Gruppe H — Ausblick/zurückgestellt. Status A/I** (Thesen-§5: Datenmodell — Entry variabler Länge, Slot-Klassen — ist bereits der Hook; Kernel/Pipeline später).

**Planungsauftrag:** NUR Erweiterungspunkte sichern. Kein gleitender Kernel, keine Fenster-Pipeline wird jetzt gebaut.

## 1. Was später kommt (Kurzriss, nicht umsetzen)

- Embedding-Input ist ein großer Chunk; Parameter konfigurieren einen **gleitenden Embedding-Kernel**, der Fenster erzeugt, statt dass die Oberschicht vor-chunkt.
- Quellen: `resident.md` (Ausblick: gleitende Indizierung), `resident_planner.md` (Expansionsregel „Gleitende Indizierung (Ausblick)").

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H2)

- **Etablierte Verfahren:** überlappende Chunks mit Stride; Sliding-Window-Attention (Longformer 2020, BigBird, Mistral Rolling-Buffer-KV); **Late Chunking** (Günther et al., Jina 2024, arXiv 2409.04701: ganzes Dokument im Long-Context-Encoder kodieren, danach Mean-Pooling pro Chunk).
- **Praktisch mit BERT/e5-small (max 512):** Fenster = `max_seq_len − Spezialtokens`, Stride 25–50 % Überlappung. Pro Fenster ein Embedding; Aggregation Mean/Max **oder** getrennte Vektoren pro Fenster mit Dokument-ID (Retrieval-üblich, Score = max über Fenster). Präfixe `query: `/`passage: ` je Fenster. `tokenizers` unterstützt `truncation` mit `stride` + `return_overflowing_tokens` (`encoding.get_overflowing()`).
- **GPU-Technik:** Bei bidirektionalen Encodern ändert eine Fensterverschiebung die Attention aller Tokens → **keine etablierte Inkrementtechnik**. Wiederverwendbar ist nur Batching der Fenster und Token-Embedding-/Tokenisierungs-Wiederverwendung. (KV-Cache-Reuse nur bei kausalen Modellen.)

## 3. Zu sichernde Erweiterungspunkte (Checkliste — darf jetzt nicht verbaut werden)

1. **Embedding-Parameterobjekt ist erweiterbar für Fenster/Stride.** Das Params-Objekt (`resident.md`: „soll zukünftig weitere Einstellungen erlauben") muss ein späteres `sliding { window, stride, aggregate }`-Feld aufnehmen, ohne Bruch → `#[non_exhaustive]` oder Builder; der Query/Passage-Enum bleibt orthogonal.
2. **Planner-Regel-Registry nimmt eine Sliding-Variante auf.** Die Expansionsregel-Registry (C3, `resident_planner.md`: Registry-Eintrag pro Modell/Tasktyp) muss pro Tasktyp alternative Expansionen tragen können: statt `Tokenize → Embed → Pool+L2 → WriteIndex` eine Variante `Tokenize(Overflow+Stride) → Embed(sliding) → PoolPerWindow → WriteIndex(many)`. Kein hartkodiertes 1:1 Task→Pipeline.
3. **Entry-Modell trägt mehrere Vektoren pro Entry mit Fenster-Metadaten.** `resident_index.md`: Entry = Vektorbereich variabler Länge (Dense=1, Datei/Gleitfenster=viele). `resident_subengine.md` §4.4: Slot-Klassen (32/64/128 Tokens), optionales Feld pro Token. Sicherstellen, dass Entry↔Segment/Slot-Mapping (E/D7) „viele Fenster-Vektoren, eine Dokument-ID" abbilden kann und `KernelHit → EntryRef` die Dokument-Aggregation (max über Fenster) in der Oberschicht nicht verhindert.
4. **Tokenizer-Wrapper kann Overflow/Stride exponieren.** Der Tokenizer (D5, `resident_first.md`) ist CPU-Ressource; sein Wrapper darf `truncation`+`stride`+`return_overflowing_tokens` später aktivieren, ohne die `u32`-IDs+Attention-Masken-Ausgabe zu ändern. Keine Annahme „genau ein Encoding pro Eingabe".
5. **Op-Austausch hinter demselben Trait.** Der gleitende Embedding-Kernel ist eine weitere `Op`/Encoder-Implementierung hinter `encode_into`/demselben Op; der Scheduler bleibt unberührt.

## 4. Jetzt NICHT umsetzen

Keine Fenster-Erzeugung, kein gleitender Kernel, keine Pro-Fenster-Pooling-Logik, keine Late-Chunking-Pipeline. Nur Datenmodell- und Parameter-Offenheit absichern.

## 5. Offene Verifikationen ([prüfen])

- `tokenizers`-Overflow-API (`get_overflowing`, `stride`) in der Projektversion.
- Aggregationsstrategie (Mean/Max vs. getrennte Vektoren) — Entscheidung liegt bei der Oberschicht; Engine muss beide zulassen.

## 6. Definition of Done

- Erweiterungspunkte 1–5 sind in B-/C-/D-Paketen als Anforderung vermerkt bzw. als offene Abstraktion vorhanden.
- Ein Test belegt, dass ein Entry mit N>1 Vektoren und gemeinsamer Dokument-ID durch das Mapping läuft (Mock-Index), und dass die Params um ein Feld erweitert werden können, ohne bestehende Call-Sites zu brechen.
