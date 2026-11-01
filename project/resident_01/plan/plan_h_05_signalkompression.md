# Umsetzungsplan H5 · Signalkompression (FFT/Wavelet, „Semantic Spectrogram")

**Gruppe H — Status E/—** (Thesen-§6/§6.1). **Ausdrücklich NICHT Engine-Scope.**

**Planungsauftrag:** Keine Engine-Arbeit. Es ist lediglich zu prüfen, dass `AccessMethod`/`Scope`/`summary()` eine spätere „Frequenz-/Kompressionsdomäne" **als Vorfilter der Oberschicht nicht ausschließen**. Divergenz zur These dokumentieren.

## 1. Einordnung / Divergenz

- Die These skizziert Frequenz-/Signalkompression (JPEG/DCT, Semantic Spectrogram, Wavelet) als Kompressions-/Qualitätsschicht. Der Plan zieht diese **bewusst aus der Engine heraus**: Die Engine ist Rechensubstrat + Index-Substrat + MaxSim-Geometrie, **kein** Retrieval-/Kompressionsalgorithmus.
- `resident_index.md` (Leitprinzipien): „Approximative Vorfilter (Zentroid, FFT, Kompression) verlieren potenziell Randtreffer. Das ist Eigenschaft der Oberschicht und nicht Teil der Engine."
- `resident_precision.md`: Qualität/akkumulierende Genauigkeitsverluste sind Sache der äußeren Schichten.

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H5)

- **Etabliert sind NICHT DCT/FFT**, sondern: Matryoshka Representation Learning (Kusupati et al., NeurIPS 2022), Product Quantization (Jégou et al., 2011), Binary/Scalar Quantization, PCA. DCT/Wavelet eher für Bild-/Zeitreihen-Hashing.
- Zu „Semantic Spectrogram" für semantische Suche **keine etablierten Referenzen** bekannt **[prüfen Literatursuche]**.
- **Erweiterbarkeit:** Ein Vorfilter ist ein Proxy-Score auf einer transformierten Repräsentation (nur Rangfolge-Treue nötig) + exakter Rerank. Passt als zusätzliche `Scope`/`AccessMethod`, solange die **Oberschicht** eine Methoden-Abstraktion hat; der Recall der Vorfilterstufe muss messbar sein.

## 3. Zu sichernde Nicht-Ausschlüsse (Checkliste)

1. **`AccessMethod` ist offen/erweiterbar.** `Index::capabilities() -> &[AccessMethod]`; das Enum als `#[non_exhaustive]`, damit eine spätere `AccessMethod::CompressionDomain`/`Frequency` (durch die Oberschicht bereitgestellt) ergänzbar ist, ohne Kern-Bruch.
2. **`Scope` ist erweiterbar.** `enum Scope { All, Range, Set, Subtree, Prefix, Centroid, … }` — als `#[non_exhaustive]`; eine Kandidatenmenge aus einem Kompressions-Vorfilter wird ohnehin als kompakte Entry-Menge (Range/Bitmap/Set) ausgedrückt und ist damit bereits abgedeckt.
3. **`summary()` bleibt generisch.** `Index::summary(kind) -> Option<SummaryRef>` liefert beliebige Summary-Strukturen (Zentroidtabelle, Präfixtabelle, …) → eine „Frequenz-/Kompressions-Summary" ist formal zulässig, ohne dass der Kern Geometrie interpretiert.
4. **ScopeResolver-Registry bleibt pluggable** (E2), sodass ein späterer Kompressions-Resolver (falls je in der Engine gewünscht) als weiterer Eintrag registrierbar wäre — ohne stilles Ausweichen (`UnsupportedScope`).

## 4. Jetzt NICHT umsetzen (und auch nicht als Engine-Workstream planen)

Keine FFT/DCT/Wavelet-Transformationen, keine Quantisierung, kein Spectrogram, kein Vorfilter in der Engine. Diese gehören — falls überhaupt — in die Suchschicht **vor** der Resident-Engine.

## 5. Definition of Done

- `AccessMethod` und `Scope` sind `#[non_exhaustive]` (B4); `summary()` ist generisch; ScopeResolver-Registry ist pluggable (E2).
- Ein ADR-Eintrag dokumentiert die Divergenz: „Signal-/Frequenzkompression ist Oberschicht-Vorfilter, nicht Engine. Engine stellt nur generische Scope-/Summary-Hooks bereit."
- Kein weiterer Code-Workstream.
