# Umsetzungsplan E6 · Synchronisation & Echo-Vermeidung (Hochrisiko)

> **Quellen:** `resident_index.md` (Synchronisation und Echo-Vermeidung). **Recherche:** `context_e.md` §E6 (Punkte 21–25, **Hochrisiko für Konsistenzmodell**). **Querschnitt:** Q1, Q2, Q3.
> **Abhängt von:** B4 (Stamps, `watch<u64>`, `apply`), E5 (Slot, Tracker). **Ergebnis:** Sync-Spezifikation + Testmatrix.

## 1. Ziel & Abgrenzung
Der **zustandsbasierte** Abgleich RAM/Index ↔ VRAM: Signal weckt nur, verglichen werden **Stamps nur residenter Entries**. Enthält: Signal-Quittierung **vor** dem Scan, Stamp-Lesereihenfolge beim Laden, **Echo-Unterdrückung** via `apply`-Rückgabe-Stamp, Zwei-Engines-Konvergenz, ABA-Vermeidung. Umsetzung des Konsistenzmodells aus `plan_e_overview.md` §1.

Nicht hier: Ladegranularität/Eviction (E7), Indexaufbau (E8).

## 2. Voraussetzungen
- B4: `stamp()`, `stamps()`, `apply() -> Stamp`, `changed() -> watch::Receiver<u64>`; Stamp-Vergleich nur `!=`.
- E5: `ResidencyTracker` (Mapping Entry ↔ Slot + bekannter Stamp), Slot-Phase 1 trägt `apply`-Stamp ein.

## 3. Zu bauende Artefakte
```rust
pub struct ResidencyTracker {
    known: HashMap<EntryId, (Slot, Stamp)>,  // nur residente Entries
}
pub struct SyncScanner { rx: watch::Receiver<u64> }
impl SyncScanner {
    fn scan(&mut self, index: &dyn Index, tr: &mut ResidencyTracker) -> ScanOutcome; // reloads/entfernt
}
pub struct Stamps; // globaler monotoner AtomicU64 (Vergabe bei jeder Mutation)
```

## 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
**`watch`-Semantik & Quittierung vor dem Scan** (context_e.md §E6 Punkt 21, `[V]`): `watch` hält nur den **letzten** Wert; mehrere Sends werden zusammengefasst; jeder Receiver verfolgt unabhängig, was er sah. **Vor** dem Scan: `let wm = *rx.borrow_and_update();` (markiert gesehen **und** liest Stand), dann Scan über alles `> wm`. Kommen Sends während des Scans, ist der Wert wieder „unseen" ⇒ nächstes `changed()` kehrt sofort zurück. Kein Verlust, nur ggf. redundanter Scan. **`borrow_and_update()` verwenden**, nicht `borrow()`.

**Nur residente Entries** (`resident_index.md`): Änderungen an nicht geladenen Entries brauchen keine Behandlung (Laden liest ohnehin aktuellen Stand). Tracker-Zustand skaliert mit dem geladenen Teil, nicht mit der Indexgröße.

**Stamp-Lesereihenfolge beim Laden** (`resident_index.md`): **Stamp → Daten → (bei mmap) Stamp erneut prüfen**; weicht er ab, wiederholen. Die falsche Reihenfolge (Daten dann Stamp) trägt einen neueren Stamp als bekannt ein, obwohl veraltete Daten geladen wurden — **die einzige unsichere Richtung**. Überzähliger Reload ist harmlos.

**Echo-Unterdrückung** (`resident_index.md`): Manager trägt den von `apply` **zurückgegebenen** Stamp des eigenen Writes als bekannt ein (Gleichheitsvergleich). Stamp muss aus dem **Rückgabewert** stammen, nicht aus späterem Lesen. Ohne Rückgabewert entsteht höchstens redundanter Upload (sichere Richtung).

**ABA / Stamps** (context_e.md §E6 Punkt 22, `[W]`): Globaler monotoner `AtomicU64` (`fetch_add`) vergibt Stamps bei jeder Mutation; Identität = (ID, Stamp); Vergleich nur `!=`. 64 Bit läuft praktisch nie über (Generation-Indices/Tagged Pointers, analog `slotmap`). Entfernen+Wiederanlegen derselben ID ergibt anderen Stamp.

**Seqlock / versioned read bei mmap** (context_e.md §E6 Punkt 23, `[V]` Prinzip): Reads nicht-atomarer mmap-Daten während Fremd-Schreibens sind in Rust formal UB (Data Race) → mmap-Bereich auf **Wortebene atomar** lesen (`AtomicU64`-Wörter oder `ptr::read_volatile` + Acquire-Fence vor dem zweiten Stamp-Read). Writer: Zähler ungerade → schreiben → gerade (+2). Reader wiederholt bei ungerade/Ungleichheit.

**Optionale Optimierungen (später, kein Vertrag)** (context_e.md §E6 Punkte 24–25, `[W]`): (a) **Baum mit max-Stamp pro Teilbaum** — Konsument besucht nur Teilbäume mit `max_stamp > watermark_c` (sub-linear); Watermark **pro Konsument**; gemeinsame „dirty"-Bitmap verliert Änderungen für den zweiten Konsumenten → **pro-Konsument-Bitmaps oder Stamps**. (b) **`changed_since(watermark)`** — bounded Ring (Stamp,EntryId); liegt `wm ≥ ältester Ringstamp` → Delta aus Ring, sonst Vollscan-Fallback. Analog Log-Offsets pro Consumer-Group (Kafka), LSN.

## 5. Umsetzungsschritte
1. Globaler `Stamps`-Zähler (`AtomicU64::fetch_add`), Identität (ID, Stamp), `!=`-Vergleich überall.
2. `SyncScanner::scan`: `borrow_and_update` vor Scan; `stamps()` iterieren (ohne Vektorseiten), nur residente vergleichen; Divergenz → Reload planen (an E7 delegiert), `None` → Entry entfernen.
3. Ladepfad: Stamp→Daten→(mmap)Stamp-Recheck; Seqlock-Wortlesung für mmap.
4. Echo: Slot-Phase 1 trägt `apply`-Rückgabe-Stamp in Tracker ein; Scanner erkennt ihn als bekannt (kein Reload).
5. Mehrere Subscriber (GPU-Manager, Persistenz-Timer, CPU-Manager) halten je eigenen Zustand/Receiver.
6. Optionale `changed_since`/Baum-Varianten als Feature-Erweiterung vorsehen (nicht Pflicht).

## 6. Fehler / Logging / Feature-Flags
- **Q1:** keine neuen öffentlichen Fehler; `StaleMapping` intern bei Recheck-Fehlschlag → Wiederholung.
- **Q2:** Events: Scan-Ergebnis (geladen/veraltet/entfernt je Zähler), Echo-Treffer (eigener Write erkannt), Recheck-Wiederholung.
- **Q4:** Logik komplett feature-frei (kern GPU-unabhängig); nur mmap-Seqlock kann realen Speicher brauchen — im Test über `FakeIndex` mit steuerbaren Stamps.

## 7. Tests (Q3 — Testmatrix, aus `resident_index.md`)
- **Echo:** eigener `apply` löst **keinen** Reload aus; fremder Stamp löst Reload aus; zwei Manager auf demselben Entry **konvergieren**.
- **Verlorenes/zusammengefasstes Signal:** nächster Abgleich findet **alle** Änderungen.
- **Signal-Quittierung vor dem Scan:** Änderung während des Scans geht **nicht** verloren (`borrow_and_update` vor Scan; Failpoint injiziert Send während Scan).
- **Stamp-Lesereihenfolge:** Änderung zwischen Stamp- und Datenlesen führt zu Wiederholung (Failpoint zwischen den Reads).
- **ABA:** Entfernen+Wiederanlegen derselben ID ergibt anderen Stamp; Vergleich `!=` erkennt es.
- `proptest` auf Scan-Sequenzen; `loom`/`shuttle` auf Echo-/Recheck-Interleavings; feste Seeds.

## 8. Offene Verifikationen
- `tokio::sync::watch` `borrow_and_update`-Verhalten gegen aktuelle Doku (context_e.md Punkt 21 verifiziert `[V]`).
- Seqlock-Korrektheit im Rust-/mmap-Kontext (Boehm MSPC'12, `atomic_load_per_byte_memcpy`) — Umsetzung mit Wort-Atomics abnicken; Crate `seqlock` Qualität `[?]`.

## 9. Definition of Done
- Zustandsbasierter Scanner mit Quittierung-vor-Scan, korrekter Lade-Lesereihenfolge, Echo-Unterdrückung über `apply`-Rückgabe; globale Stamps `!=`; vollständige Testmatrix oben grün ohne GPU.
