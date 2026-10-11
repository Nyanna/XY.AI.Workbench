# D7 — Index-Segmente in der Subengine

**Zweck:** `SegmentView`/Slot/Slot-Klassen, Schreiben/Upsert/Gültigkeitsbit, Mapping `(segment,slot)→EntryRef`, Segment-Basistabelle, bidirektionale Sync. **Brücke zu Gruppe E.**

> Hinweis: „Index" meint hier den **Vektorindex** (RAG), keine Dateiliste. D7 überlappt stark mit Gruppe E (Index-Management); E referenziert die hier festgelegte Segment-Repräsentation.

**Einordnung:** Abhängt von B4 (Index-Trait), D3 (Index-Segment-Arena/Staging). Downstream: D6 (Kernel schreibt/liest Slots), Gruppe E (E1/E5/E8 bauen darauf). Querschnitt Q1–Q4.

**Quellen:** `resident_subengine.md` §4.4; `resident_index.md` (Segment/Slot/Sync, via Gruppe E); `context/context_d.md` D7 (#27–#29).

---

## Festgelegte Entscheidungen

- **Konsistenzmodell (Empfehlung festgelegt, in D7 bestätigen, `[V-Konzept]`, #27):** **immutable Segmente + Generations-Manifest** (Lucene/Tantivy-Prinzip). Host baut neues Segment/Manifest, publiziert per **`arc-swap`**; Leser **pinnen die Generation per Lease** (D1). Altes Segment wird erst freigegeben, wenn **Leases==0 UND alle Kernel-Stream-Events abgeschlossen** (D3 stream-geordnete Freigabe). Alternativen (nicht gewählt): Epoch-Reclamation (`crossbeam-epoch`), Double-Buffering. **VRAM-Budget für 2 Generationen** (CoW-Spitze, D3/E3).
- **SegmentView (§4.4):** Basisadresse als `Region`, Slot-Zahl, Stride, dtype, **Generation**.
- **Slot-Klassen (`[U]`, #28):** Adresse = `base + slot*stride(class)`; getrennte Längen-Arrays + **Valid-Bitmap**; Gültigkeit/Generationsnummer je Slot. Slot = feste Größe `T_max × dim × dtype`; Klassen (z. B. 32/64/128 Tokens) halten den Kernel **lookup-frei**.
- **Padding (§4.4):** gültige Token-Zahl je Slot; Padding mit **−inf/−1e4** maskiert (nicht 0).
- **Upsert (#28):** neuen Slot schreiben → **stream-geordnet** Valid-Bit setzen → altes Bit löschen (**Tombstone**). **Kein In-Place** während laufender Kernels. Kompaktierung durch **Segment-Rebuild** (vgl. vLLM-PagedAttention-Blöcke).
- **Optionales Feld pro Token:** Centroid-ID/Code (für späteren Funnel; vom Kernel ignoriert, wenn ungenutzt).
- **Kernel kennt nur Slots (§4.4):** Ergebnis `(segment_gen, slot, score)`. **Rückabbildung `(segment,slot)→EntryRef`** (Chunk/Zeile/Bereich/Datei) macht der **Manager auf der CPU**. Treffer aus entladenen Segmenten werden **verworfen oder remapped**. Segment-Basisadressen gehen als **kleine Tabelle über Segmente** (nicht Einträge) an den Kernel.
- **Bidirektionale Sync (`[U]`, #29):** Segment-Ereignisse („hinzugefügt/geändert") über `tokio::sync::mpsc`/`broadcast`; Kopie auf der **Copy-Lane** mit Pinned-Staging (D2/D3); **Publikation erst nach Event-Completion**; Version/Generation im Event.

---

## Zu erstellende Artefakte (`rag/src/segment/`)

- `mod.rs`
  - `struct SegmentView { base: Region, slots: u32, stride: u32, class: SlotClass, dtype: DType, gen: u64 }`.
  - `enum SlotClass { T32, T64, T128, .. }` (konfigurierbar) + `fn stride(class, dim, dtype)`.
  - `struct SegmentTable` — kleine Basistabelle über Segmente (an Kernel übergeben).
  - `struct Manifest` — Generations-Manifest, publiziert per `arc_swap::ArcSwap<Manifest>`.
- `slots.rs`
  - `struct SlotMeta { valid: bool, token_count: u16, gen: u64, centroid: Option<u32> }`; Valid-Bitmap; Upsert (write→set-valid→tombstone), stream-geordnet.
- `mapping.rs`
  - `struct SlotMap` — `(segment_gen, slot) → EntryRef`; Rückabbildung **Fixed** (Arithmetik) / **Variable** (Binärsuche über Offsets). Entladene Segmente → verwerfen/remappen.
- `sync.rs`
  - Segment-Ereigniskanal (`mpsc`/`broadcast`), Copy-Lane-Anbindung (Staging), Publikation nach Event, Generation im Event. Konvergenz mit Index-Objekt (Gruppe E).

---

## Arbeitsschritte

1. `SegmentView`/`SlotClass`/`SegmentTable` definieren; Generation durchgängig.
2. Konsistenzmodell (immutable Segmente + Generations-Manifest, `arc-swap`) implementieren; Leases pinnen Generation; Freigabe erst Leases==0 ∧ Events fertig (mit D3 koppeln).
3. Slot-Schreiben/Upsert/Tombstone **stream-geordnet**; Valid-Bitmap; Padding-Maskierung.
4. `SlotMap`-Rückabbildung (Fixed/Variable); Umgang mit entladenen Segmenten.
5. Segment-Basistabelle an Kernel (D6) übergeben.
6. Bidirektionale Sync über Ereigniskanal + Copy-Lane (D2) + Staging (D3); Publikation nach Event.
7. **Konsistenzmodell bestätigen** (offene Festlegung aus Metaplan/§4.4) und mit Gruppe E (E5/E6/E8) abstimmen.

## Fehler (Q1) & Logging (Q2)

- `enum SegmentError { StaleGeneration{gen}, SlotOutOfRange, MappingMissing, SyncFailed{source} }`; intern `StaleMapping` (`pub(crate)`), an API-Grenze abgebildet.
- Span pro Segment/Generation (`segment_gen`, `slot_class`); Events `segment_added/changed`, `upsert`, `tombstone`, `remap`. Sync-Pfad auf `debug`.

## Feature-Flags (Q4)

- Slot-/Mapping-/Manifest-Logik **GPU-frei** (RAM-Modell, testbar ohne GPU). Copy-Lane/Staging-Pfad hinter `cuda`.

## Tests (Q3)

- Upsert/Tombstone/Valid-Bitmap: korrekte Sichtbarkeit; kein In-Place während „laufender Kernel" (Mock-Events).
- `SlotMap`: Fixed-Arithmetik und Variable-Binärsuche; entladene Segmente verworfen/remappt.
- Generations-Reclamation: Freigabe erst Leases==0 ∧ Events fertig (Mock-Events, `loom`/`shuttle` für Race).
- Sync: Publikation erst nach Event-Completion; ABA/Generation-Wechsel zwischen Lesen und CAS (Failpoints).

## Definition of Done

- `SegmentView`/Slot-Klassen/Manifest implementiert; immutable-Segmente-Modell bestätigt und mit Gruppe E abgestimmt; Rückabbildung + bidirektionale Sync getestet (No-GPU-Modell grün).
