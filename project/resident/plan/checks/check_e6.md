Erledige die offenen Punkte.

- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

# Übergabe: Stand nach der Arbeit an den offenen Punkten vor F1 nach E9

## Stand der Verifikation
- `cargo test --lib` (ohne GPU): **453 Tests grün**.
- `cargo check --lib --tests` und `cargo check --lib --tests --features cuda` kompilieren ohne Fehler und ohne `unused import`. Für `cuda` nutze ich `PATH=/usr/local/cuda-13.3/bin:$PATH CUDA_PATH=/usr/local/cuda-13.3`.
- Nicht ausgeführt: die `#[ignore]`-CUDA-Tests, also keine GPU-Läufe in dieser Session.

## Noch offen

**Punkt 6, E9:**
- Test, dass der Kern keine Parent-Child-Struktur hat.
- Das Referenz-Addon gibt `Vec<TaskHandle>` statt `Vec<Task>` zurück (Abweichung vom Plan). Entweder anpassen oder die Abweichung begründet festschreiben.
- Hardware-Smoke-Test für Extensions auf der GTX 1660.
- Die 22 ignorierten CUDA-Tests untersuchen und ausführen. Das schließt die neuen `device`-Tests ein. Besonders sinnvoll wäre ein CUDA-Test für den Bulk-Upload mit `CudaUploadPort`.

**Verifikationen (nicht begonnen):**
- **Punkt 8, E1:** `madvise` gegen die man-Seite lesen und das Verhalten auf dem Zielkernel (6.8.0-147) prüfen.
- **Punkt 9, E2:** Die DoD in `plan_e_02.md` Punkt für Punkt gegen den Code abgleichen. Das Result in `plan_e_02.md` ist durch die Umsetzung überholt.
- **Punkt 10, Loom:** Prüfen, ob `IndexArena::shared`/`AllocRef` ein eigenes loom-Modell braucht. Neu ist der `structural`-Mutex in `IndexRuntime`. Außerdem laufen `publish_pending`, `unpublish` und `complete_slot` nebenläufig, nur `complete_slot` nimmt nicht den Strukturlock.
- E3 bis E8 gegen ihre eigenen Verifikationspunkte prüfen. Das stand schon in der letzten Übergabe offen.

## Bekannte Einschränkungen und Risiken
- **Retry bei voller Arena:** Die Retry-Logik in `IndexRuntime::ensure` ist nur mit 0 Readern getestet. Mit echten Readern wird die evictete Region erst frei, wenn alle Reader die neue Epoche gemeldet haben. Dann kann der Retry trotzdem scheitern.
- **Bulk-Pfad:** Er hat keinen produktiven Index, der `extent` und `backing` liefert. Getestet ist er nur mit `FlatIndex` (Host-Mock-Port). Ein mmap-gestützter Index fehlt weiterhin.
- **Nicht betroffene Warnungen:** `drive_build`, `skip_bytes` und `Seqlock::from_raw` sind weiterhin „never used". `exec.rs:337` hat einen irrefutable `if let`. Das ist alles schon vorher da gewesen.
- **Pinning:** `Evictor::pin` (laufende Batches pinnen) wird von `IndexRuntime` nicht aufgerufen. Der Schutz läuft derzeit nur über die Epochen beim Reclaim.
