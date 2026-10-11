# Umsetzungsplan B4 · Index-Trait & -Vertrag

| | |
|---|---|
| **Paket** | B4 (Gruppe B) — **eigenständig vom Kern**, parallel zu B1–B3 bearbeitbar |
| **Abhängt von** | A4 |
| **Blockiert** | C1 (Index-Übergabe), E1/E2/E5/E6 (Manager/Scope/Sync), G2 (konkretes Index-Schema) |
| **Ergebnis** | Index-Trait-Spezifikation: `Index`-Trait, Entry/Stamp-Semantik, `Scope`, `AccessMethod`, Schreibreihenfolge, Level-Signal `watch<u64>` |
| **Quellen** | `../resident_index.md` (Index-Vertrag, Scope, Synchronisation), `../context/context_b.md` §B4 (15–20) |

---

## 1. Zweck & Abgrenzung

Der **Index-Vertrag** ist bewusst vom Kern entkoppelt: Jeder Index implementiert sein **eigenes Modell** (Dense, ColBERT, Datei/Gleitfenster, AST, Chunks, kaskadierend). Der Kern kennt diese Modelle nicht. Der Index muss **nur Änderungen melden** und Struktur/Stamps liefern — **keine** Geometrie (kein Abstand, Radius, Top-k, Präfix-Match) und **keine** Scope-Auflösung.

Die einzige harte Bindung ist **Modell-ID und Dimension** im Index-Header. Persistenzformat und -zeitpunkte liegen **außerhalb** (beim Index-Objekt, siehe G2), nicht in B4.

**Nicht Teil von B4:** `StreamedIndexManager`, Scope-**Resolver**, Arena, Residency, Kommunikationsslot (alles Gruppe E); konkretes Parquet-Schema (G2). B4 = reiner **Trait + Datentypen + Stamp-Mechanik**.

---

## 2. Der Index-Trait

```rust
trait Index {
    fn stamp(&self, entry: EntryId) -> Option<Stamp>;            // None = entfernt
    fn stamps(&self) -> impl Iterator<Item = (EntryId, Stamp)>;  // OHNE Vektordaten zu lesen
    fn read(&self, entry: EntryId) -> EntryData<'_>;             // Referenz, keine Kopie
    fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;       // Stamp DES EIGENEN Writes
    fn changed(&self) -> watch::Receiver<u64>;                   // Level-Signal
    fn capabilities(&self) -> &[AccessMethod];                   // Centroid, Prefix, Subtree, Range, …
    fn children(&self, node: NodeId) -> EntrySet;                // reine Struktur
    fn members(&self, cell: CellId) -> EntrySet;                 // Inverted List o. Ä.
    fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>; // z. B. Zentroid-Tabelle
    fn prefetch(&self, set: &EntrySet);                          // optional, darf no-op sein
}
```

### 2.1 Begleittypen

```rust
pub struct EntryId(/* opak, u64 o. Ä. */);
pub struct Stamp(u64);                 // Vergleich NUR mit != , NIE mit <
#[non_exhaustive]
pub enum AccessMethod { Centroid, Prefix, Subtree, Range /* erweiterbar */ }
pub enum Scope { All, Range(/*..*/), Set(/*..*/), Subtree(NodeId), Prefix(/*..*/),
                 Centroid { k: u32, radius: Option<f32> } }
pub struct EntrySet(/* kompakt: Ranges oder Bitmap, NICHT ID-Liste */);
pub struct EntryData<'a>(/* Referenz auf Vektorbereich variabler Länge */);
pub struct SummaryRef(/* z. B. Zentroid-Tabelle als Vektordaten */);
```
- **Entry** = Vektorbereich **variabler Länge** (Dense = 1 Vektor, ColBERT/Datei/Fenster = viele). Entries dürfen überlappen / dieselben Vektoren referenzieren.
- `EntrySet` ist **kompakt** (Ranges/Bitmap), nicht ID-Liste.

---

## 3. Stamp-Mechanik (Kern von B4)

### 3.1 Monotoner Zähler
```rust
static NEXT: AtomicU64 = AtomicU64::new(1);
fn next_stamp() -> Stamp { Stamp(NEXT.fetch_add(1, Ordering::Relaxed)) }
```
- Eindeutigkeit genügt `Relaxed`. **Sichtbarkeit** der Daten zum Stamp über Release-Store/Lock beim Veröffentlichen (Stamp innerhalb der Schreib-Serialisierung vergeben).
- Entfernen + Wiederanlegen derselben ID ⇒ **neuer** Stamp (ABA-Schutz). Vergleich ausschließlich `!=`.
- Über Neustarts: High-Water-Mark im Index-Header persistieren, beim Laden `max(stored, file_max) + 1`.

### 3.2 Schreibreihenfolge (verbindlich)
**erst Daten, dann Stamp, dann Signal.** Falsche Reihenfolge trägt einen neueren Stamp ein als die Daten → veraltete Daten gelten als aktuell (einzige unsichere Richtung).

### 3.3 Echo-Rückgabe
`apply` gibt den Stamp **des eigenen Writes** zurück. Der Manager (E6) trägt genau diesen als bekannt ein (Gleichheitsvergleich) → Echo-Unterdrückung. **Der Rückgabewert, nicht ein späteres Lesen.**

### 3.4 Fallback-Stamp (dateigestützte mmap-Indizes)
Stufen: **Stamp → size+mtime → Content-Hash.**
- **Racy-Problem (git-Vorbild):** Liegt `mtime` innerhalb der Zeitauflösung **vor** dem letzten Scan (`mtime >= T_scan − Auflösung`), gilt der Entry als unsicher → Content-Hash-Prüfung. Stamp zusammen mit Referenzzeit `T_scan` merken.
- Konservatives Racy-Fenster **2 s** (FAT-Worst-Case) oder FS-spezifisch. Fallback-Stamp-Tupel: `(size, mtime_ns, ctime/inode falls verfügbar)` → bei Racy/Abweichung Content-Hash.
- **Hash:** `xxhash-rust` **xxh3_64** (schnell, nicht kryptografisch) als Default; `xxh3_128` bei Kollisionssorge; `blake3` nur bei Manipulationssicherheit. **Hash-Algorithmus + Version im Index-Header festhalten.**
  > `// TODO(A3): Racy-Fenster (FS-Auflösung) und xxh3_64 vs. 128 festlegen (context_b offener Punkt 5).`

---

## 4. Benachrichtigung — Level-Signal `watch<u64>`

- `tokio::sync::watch<u64>`: behält **nur den letzten Wert** (Watermark = Zählerwert), jeder Receiver verfolgt separat. Zusammengefasste/verlorene Signale sind **harmlos**, weil der Abgleich **zustandsbasiert** ist (E6 vergleicht Stamps).
- Konsumentenschleife mit `borrow_and_update()` (markiert als gesehen) — **nicht** `borrow()` (würde doppelt laufen):
```rust
loop {
    let stamp = *rx.borrow_and_update();
    reconcile(stamp).await;
    if rx.changed().await.is_err() { break; } // alle Sender gedroppt
}
```
- `wait_for`/Zwischenwerte: gehen verloren — für Ereignisströme ungeeignet, für zustandsbasierten Abgleich akzeptabel.

---

## 5. Mehrschreiber-Serialisierung

Der Index **serialisiert `apply`** und garantiert die Sequenz (Last-Writer-Wins = Dequeue-Reihenfolge). Zwei Engines dürfen denselben Entry gleichzeitig aktualisieren; wer zuerst kommt, wird ggf. überschrieben.

**Default-Muster:** Single-Writer-Task + `mpsc` + `oneshot`-Antwort:
```rust
// apply(cmd) -> oneshot::Receiver<Stamp>
// Stamp unmittelbar VOR Veröffentlichung vergeben, danach watch::Sender::send(stamp)
```
Alternative: kurzer sync-`Mutex` um `apply` (nie über `.await` halten; sonst `tokio::sync::Mutex`).

---

## 6. Stamp-Spalte ohne Vektorseiten (Scan-Billigkeit)

`stamps()` **darf keine Vektorseiten einpagen.** Umsetzung für Parquet/Arrow-gestützte Indizes:
- `ArrowReaderBuilder::with_projection(ProjectionMask::leaves(…, [stamp_idx]))` liest nur die Stamp-Spalte; Vektorspalte `FixedSizeList<f16,384>` bleibt ungelesen (Spaltenchunks pro Row-Group getrennt).
- **Robusteste Variante:** Stamp-Spalte als **separate Sidecar-Datei** (kleines Parquet/Arrow-IPC) → garantiert null Vektor-IO, entkoppelt Rewrites.
- mmap: Reader braucht `ChunkReader`; `memmap2`-Mapping als `bytes::Bytes` einhängen, nur Metadaten + Stamp-Chunk anfassen.
  > `// TODO(A3): parquet Float16/FixedSizeList-Support + Leaf-Index-Ermittlung über Spaltenpfad am Testfall prüfen (context_b offener Punkt 2).`

*(Das konkrete Schema ist G2; B4 definiert nur die Vertragsanforderung „Stamp ohne Vektor-IO“.)*

---

## 7. Lazy / blockierende Reads

`read` darf auf Page-Faults und Platten-I/O **blockieren**. Der Manager (E) ruft blockierende Reads im Blocking-/Worker-Pool auf, nie im async-Runtime-Thread, mit Prefetch-Hinweis (`madvise(WILLNEED)`). B4 dokumentiert dies im Trait-Kontrakt.

---

## 8. Fehler & Logging

- Q1: `IndexUnavailable` (Index nicht nutzbar), `StaleMapping` (intern), ABA-/Reihenfolge-Verletzungen als `debug_assert!`.
- Q2: `tracing` — `apply` (Entry, zurückgegebener Stamp), Signal-Sende-Watermark, Fallback-Hash-Trigger (racy).

## 9. Teststrategie (Q3, ohne GPU)

- **Stamp:** monoton/eindeutig; Entfernen+Neuanlage derselben ID ⇒ anderer Stamp (ABA); Vergleich nur `!=`.
- **Schreibreihenfolge:** Daten→Stamp→Signal; Test der unsicheren Richtung (Stamp vor Daten) als Negativfall dokumentiert.
- **Echo:** `apply`-Rückgabe-Stamp = bekannter Stamp (kein Reload-Trigger); fremder Stamp ⇒ Reload (Manager-Testanker für E6).
- **watch:** verlorenes/zusammengefasstes Signal → nächster Abgleich findet alle Änderungen; `borrow_and_update` läuft nicht doppelt.
- **Fallback:** mtime innerhalb Racy-Fenster ⇒ Hash-Pfad; size/mtime-Abweichung ⇒ Hash; stabiler Entry ⇒ Stat-Vergleich genügt.
- **stamps() ohne Vektor-IO:** mit Fake-Backend verifizieren, dass keine Vektorspalte gelesen wird (Zähl-Mock / Sidecar).
- **Mehrschreiber:** Single-Writer-Task serialisiert; Last-Writer-Wins = Dequeue-Reihenfolge; Stamps streng aufsteigend vergeben.

## 10. Offene Punkte (→ A3 / E / G2)

1. Racy-Fenster & Hash-Variante (xxh3_64/128) final.
2. parquet Float16/FixedSizeList + Leaf-Index am Testfall.
3. Stamp-Spalte inline (Projection) vs. Sidecar-Datei.
4. Mehrschreiber: Single-Writer-Task vs. kurzer Mutex (A3).

## 11. Definition of Done

- `Index`-Trait + alle Begleittypen (`EntryId`, `Stamp`, `Scope`, `AccessMethod`, `EntrySet`, `EntryData`, `SummaryRef`, `NodeId`, `CellId`) definiert.
- Stamp-Mechanik (monoton, ABA, Echo-Rückgabe, Fallback mit Racy+Hash, High-Water-Mark) implementiert + getestet.
- `watch<u64>`-Level-Signal-Kontrakt + `borrow_and_update`-Muster dokumentiert.
- Referenz-In-Memory-Index (Testdouble) implementiert; eigenständig vom Kern; baut ohne `cuda`/`candle`.
