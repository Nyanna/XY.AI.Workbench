# Umsetzungsplan E6 · Synchronisation & Echo-Vermeidung (Hochrisiko)

> **Quellen:** `resident_index.md` (Synchronisation und Echo-Vermeidung). **Recherche:** `context_e.md` §E6 (Punkte 21–25, **Hochrisiko für Konsistenzmodell**). **Querschnitt:** Q1, Q2, Q3.
> **Boundary:** see `rag/docs/architecture/scope_charta.md` (engine vs. search-layer scope, ring buffer vs. pull queue).
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

## Result

E6 ist umgesetzt, aber noch nicht kompiliert und nicht getestet. Ich hatte kein Tool für `cargo`. Geprüft habe ich nur die Syntax der neuen Dateien mit `ast_validate`. Ein `cargo test` (und `RUSTFLAGS="--cfg loom" cargo test seqlock` für den Loom-Test) steht noch aus.

**Neue Dateien in `rag/src/resident/manager/`:**
- **`sync.rs`:**
  - `SyncScanner::scan` bestätigt das Signal mit `borrow_and_update` vor dem Vergleich. Verglichen werden nur Stamps residenter Entries, ausschließlich mit `!=`.
  - `ScanOutcome` meldet `reload` und `removed`. `enqueue_removals` legt die Entfernungen als `Update::Unbind` in die `UpdateQueue`.
  - `load_entry` liest erst den Stamp, dann die Daten und prüft den Stamp danach erneut. Bei Abweichung wiederholt es das Laden, höchstens 8-mal. Danach liefert es den zuerst gelesenen, älteren Stamp, was höchstens einen überzähligen Reload kostet.
  - Pro Subscriber gibt es einen eigenen Scanner mit eigenem Receiver.
- **`seqlock.rs`:** Seqlock mit Wort-Atomics für mmap-Daten, mit `from_raw` für gemappten Speicher.
- **`test/sync.rs`:**
  - Echo und fremder Write.
  - Zwei Manager konvergieren.
  - Zusammengefasste Signale.
  - Änderung während des Scans über Failpoint.
  - Änderung zwischen Stamp- und Datenlesen über Failpoint.
  - Entfernen und Wiederanlegen (ABA).
  - Entfernte Entries.
  - Proptest mit festem Seed.

`mod.rs` registriert `seqlock` und `sync`.

Abweichungen vom Plan:
- `scan` nimmt `&ResidencyTracker` statt `&mut`. Die Tracker- und Mapping-Änderungen laufen über die `UpdateQueue` und werden in der Lücke zwischen zwei Batches angewendet, damit der Scan das Mapping nicht verändert.
- Der Stamp-Zähler (`StampCounter`), `ResidencyTracker` und die Echo-Registrierung über den Rückgabewert von `apply` waren schon vorhanden. Ich habe sie nicht neu gebaut.
- Ein Echo-Treffer wird nicht einzeln gezählt. Er geht in `ScanOutcome::current` ein, zusammen mit allen anderen Entries, deren Stamp schon aktuell ist.
- Die Seqlock-Tests liegen inline in `seqlock.rs` statt unter `test/`, wie bei den Loom-Tests in `doorbell.rs`.

E6 ist jetzt kompiliert und getestet.

- **`cargo test --lib manager`:** 59 Tests grün. Dazu gehören alle Sync-Tests (Echo, Konvergenz, zusammengefasste Signale, Änderung während des Scans, Änderung zwischen Stamp- und Datenlesen, ABA, Entfernen, Proptest) und die Seqlock-Tests.
- **Loom (`RUSTFLAGS="--cfg loom" cargo test --lib seqlock`):** `reader_never_sees_a_torn_pair` ist grün.
- **Warnungen:** Der Testbuild meldet 40 Warnungen. Die gezeigten betreffen ungenutzte Methoden in bestehendem Code (`root`, `epochs`, `current`, `snapshot`, `begin_batch`). Mein Filter auf `sync.rs`, `seqlock.rs` und `test/sync.rs` hat keine Treffer geliefert. Ich habe nur den Ausschnitt gesehen, nicht die vollständige Liste.

Nicht gelaufen sind der Rest der Testsuite (nur `manager` wurde gefiltert) und `cargo clippy`.
