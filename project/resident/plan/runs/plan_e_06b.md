Setze die verbleibenden Punkte von E6 um:

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


- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc ist im Projekt vorhanden und konfiguriert
- ziehe AST-Tools und spezifische Tools vor; Nutze kein `cat` zum auslesen ganzer Dateien
- Lasse keine Punkte ohne triftigen Grund aus

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_00_overview.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_06.md)
SystemInit:  id=0ead0eef-d612-4186-9ecc-d24b9c316f2f, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich kompiliere und teste zuerst.
ReasoningToken: 31
Control Request:
```yaml
id: fc3c-1
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib manager 2>&1 | tail -60
  reason: "Run cargo tests for the manager module, including the new sync and seqlock code, which have not been compiled yet."
```
/answer fc3c-1 allow
Control Request:
```yaml
id: fc3c-2
result:
  content:
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-n8qr20ec.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  structuredContent:
    results:
    - stdout_file: /tmp/mcpc-stdout-n8qr20ec.log
      stdout_file_file_stats:
        path: /tmp/mcpc-stdout-n8qr20ec.log
        size_bytes: 4950
        lines: 60
        words: 245
        complexity: 0.422
        created: 2026-10-23T18:17:55.889215+00:00
        modified: 2026-10-23T18:17:55.889215+00:00
        accessed: 2026-10-23T18:17:55.889215+00:00
        line_length_max: 113
        line_length_min: 0
        line_length_avg: 81.5
        words_per_line_avg: 4.08
        checksum: 3eb21fa591a0b70685e3694c0b49aa56b58e7ac77b444ffc33e4c6771185af12
      stdout_file_ast_outline:
      - id: file
        lines: 1-60
        signature: test resident::manager::arena::tests::admission_keeps_cow_headroom ... ok
```
/answer fc3c-2 allow
Control Request:
```yaml
id: fc3c-3
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - "grep -E 'FAILED|failed|panicked|error|warning: unused|test result' /tmp/mcpc-stdout-n8qr20ec.log | head -30"
    - grep -E 'sync|seqlock' /tmp/mcpc-stdout-n8qr20ec.log | head -30
  reason: Inspect the test output for failures and confirm the sync and seqlock tests ran.
```
/answer fc3c-3 allow
Control Request:
```yaml
id: fc3c-4
result:
  structuredContent:
    results:
    - stdout: |
        test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 301 filtered out; finished in 10.84s
        test resident::manager::seqlock::tests::read_returns_last_write ... ok
        test resident::manager::sync::tests::coalesced_signals_lose_no_change ... ok
        test resident::manager::sync::tests::changed_returns_immediately_for_an_unseen_signal ... ok
        test resident::manager::sync::tests::exhausted_rechecks_register_the_older_stamp ... ok
        test resident::manager::sync::tests::load_of_a_removed_entry_is_none ... ok
        test resident::manager::sync::tests::non_resident_changes_are_ignored ... ok
        test resident::manager::sync::tests::own_write_is_not_reloaded_but_foreign_write_is ... ok
        test resident::manager::sync::tests::remove_and_recreate_yields_a_different_stamp ... ok
        test resident::manager::sync::tests::removed_entries_are_reported_and_queued_for_unbinding ... ok
        test resident::manager::sync::tests::send_during_scan_stays_unseen_and_is_found_next ... ok
        test resident::manager::sync::tests::two_managers_converge_on_the_index_state ... ok
        test resident::manager::sync::tests::write_between_stamp_and_data_read_repeats_the_load ... ok
        test resident::manager::sync::tests::scan_sequences_converge_to_the_index_state ... ok
        test resident::manager::seqlock::tests::concurrent_reader_never_sees_a_torn_snapshot ... ok
```
/answer fc3c-4 allow
Text:
 Das normale Testlauf ist grün. Jetzt der Loom-Test und Warnungen/Clippy.
Control Request:
```yaml
id: fc3c-5
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - RUSTFLAGS="--cfg loom" cargo test --lib seqlock 2>&1 | tail -25
    - cargo build --lib 2>&1 | grep -E -A6 'warning|error' | grep -E 'manager/(sync|seqlock)|test/sync' | head
  reason: Run the loom seqlock test and check for compiler warnings in the new files.
```
/answer fc3c-5 allow
Control Request:
```yaml
id: fc3c-6
result:
  structuredContent:
    results:
    - stdout: |+
        ...
        215 |     pub fn root(&self) -> &RootPointer {
            |            ^^^^
        ...
        218 |     pub fn epochs(&self) -> &EpochTracker {
            |            ^^^^^^
        ...
        221 |     pub fn current(&self) -> Arc<SegmentTable> {
            |            ^^^^^^^
        ...
        226 |     pub fn snapshot(&self) -> Snapshot {
            |            ^^^^^^^^
        ...
        234 |     pub fn begin_batch(&self, reader: usize) -> BatchGuard<'_> {
            |            ^^^^^^^^^^^

        warning: `xy-ai-rag` (lib test) generated 40 warnings
            Finished `test` profile [unoptimized + debuginfo] target(s) in 9.22s
             Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-1a9f804c1a3d4e29)

        running 1 test
        test resident::manager::seqlock::loom_tests::reader_never_sees_a_torn_pair ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 341 filtered out; finished in 0.07s
```
/answer fc3c-6 allow
Result Stats: id=0ead0eef-d612-4186-9ecc-d24b9c316f2f, total: 20976, in: 8, out: 1151, reason: 0, read: 54835, write: 19817, start: 1792779529786, end: 1792779533660, time: 3820, total: 52647, io: 0, control: 54, sent: 0, recv: 77
E6 ist jetzt kompiliert und getestet.

- **`cargo test --lib manager`:** 59 Tests grün. Dazu gehören alle Sync-Tests (Echo, Konvergenz, zusammengefasste Signale, Änderung während des Scans, Änderung zwischen Stamp- und Datenlesen, ABA, Entfernen, Proptest) und die Seqlock-Tests.
- **Loom (`RUSTFLAGS="--cfg loom" cargo test --lib seqlock`):** `reader_never_sees_a_torn_pair` ist grün.
- **Warnungen:** Der Testbuild meldet 40 Warnungen. Die gezeigten betreffen ungenutzte Methoden in bestehendem Code (`root`, `epochs`, `current`, `snapshot`, `begin_batch`). Mein Filter auf `sync.rs`, `seqlock.rs` und `test/sync.rs` hat keine Treffer geliefert. Ich habe nur den Ausschnitt gesehen, nicht die vollständige Liste.

Nicht gelaufen sind der Rest der Testsuite (nur `manager` wurde gefiltert) und `cargo clippy`.
