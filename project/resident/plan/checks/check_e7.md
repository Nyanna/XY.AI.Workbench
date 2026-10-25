Erledige die offenen Punkte.

- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

# Übergabe

## Erledigt in dieser Session
- **Kein Parent-Child im Kern:** Der neue Test `core_task_types_carry_no_parent_child_structure` steht in `facade/test/addon.rs`. Er zerlegt `TaskMeta` und `Payload::Similarity` mit erschöpfenden Mustern. Er bricht beim Kompilieren, sobald ein Feld dazukommt, zum Beispiel ein Link auf einen Parent. `TaskMeta` hat nur `id` und `kind`.
- **`Vec<TaskHandle>` statt `Vec<Task>`:** Ich habe die Abweichung nicht angepasst, sondern festgeschrieben. Das Addon reiht die Folgetasks selbst über die öffentliche Submit-API ein, deshalb gibt es Handles zurück. Der Test `cascade_returns_handles_of_independent_tasks` hält das fest. Er prüft, dass das Handle die Task-ID des eingereihten Tasks trägt.
- **Ignorierte CUDA-Tests:** Alle 24 laufen mit `--features cuda -- --ignored --test-threads=1` grün auf der GTX 1660 SUPER (Treiber 610.57.04, nvcc 13.3). Das schließt die `device`-Tests ein. Die Session-Übergabe sprach von 22. Es sind 22 Bestandstests plus die zwei neuen CUDA-Tests unten.
- **Bulk-Upload mit `CudaUploadPort`:** Der neue Test `bulk_segment_reaches_the_device_in_one_direct_copy` steht in `manager/test/transfer.rs`. Er nutzt `FlatIndex`, lädt ein lückenloses Segment per Direktkopie, liest es vom Device zurück und prüft jedes Slot-Muster. Er ist grün.
- **Hardware-Smoke-Test für Extensions:** Der neue Test `extension_task_runs_through_workers_with_device_round_trips` steht in `planner/test/adapter.rs`. Planner, Processor und Worker-Thread laufen mit einem echten Device-Roundtrip pro Op. Der Task mit Extension und der ohne liefern dieselbe Op-Kette und dasselbe Ergebnis, die Kette stimmt auch mit dem CPU-Lauf überein. Dafür habe ich `Scripted` um ein optionales `echo` erweitert und `run_with` eingeführt. `run` bleibt unverändert nutzbar.
- **Tests ohne GPU:** Die 5 Addon-Tests sind grün. Die Gesamtsuite habe ich nach den Änderungen nicht neu laufen lassen. Vorher waren es 453 Tests, es kommen 2 dazu.

## Noch offen
- **Punkt 8, E1 (`madvise`):** Nicht begonnen. Der letzte Befehl (grep nach `madvise` in `rag/src`, `rag/docs` und `plan_e_01.md`, dazu `man -w madvise`) wurde abgelehnt. Zu tun ist, die Aufrufe gegen die man-Seite zu lesen und das Verhalten auf Kernel 6.8.0-147 zu prüfen.
- **Punkt 9, E2:** Die DoD in `plan_e_02.md` Punkt für Punkt gegen den Code abgleichen. Das Result dort ist überholt.
- **Punkt 10, Loom:** Prüfen, ob `IndexArena::shared` und `AllocRef` ein eigenes loom-Modell brauchen. Neu ist der `structural`-Mutex in `IndexRuntime`. Außerdem laufen `publish_pending`, `unpublish` und `complete_slot` nebenläufig, und nur `complete_slot` nimmt den Strukturlock nicht.
- **E3 bis E8:** Gegen ihre eigenen Verifikationspunkte prüfen.
- **E9-Zusammenfassung:** In `plan_e_09.md` sind die Abschnitte „Abweichungen und Hinweise“ und „Verifikation“ veraltet. Sie sagen noch, es gebe keinen Parent-Child-Test, keinen Hardware-Smoke-Test und keine Untersuchung der CUDA-Tests. Sie müssen auf den Stand oben gebracht werden. Das habe ich noch nicht gemacht.

## Hinweise für die nächste Session
- **Tool-Einschränkung:** Der Controller hat einzelne Bash-Aufrufe abgelehnt. Das waren `cat`, `grep` und `man`, jeweils mit dem Hinweis, AST- oder Suchwerkzeuge zu nutzen. Dateien lese ich besser mit `ast_read`, `ast_find` und `list`. Ausgewählte `grep`-Aufrufe und `cargo test` gingen durch.
- **CUDA-Umgebung:** `export PATH=/usr/local/cuda-13.3/bin:$PATH CUDA_PATH=/usr/local/cuda-13.3`

## Bekannte Einschränkungen und Risiken (unverändert)
- **Retry bei voller Arena:** Die Retry-Logik in `IndexRuntime::ensure` ist nur mit 0 Readern getestet. Mit echten Readern wird die evictete Region erst frei, wenn alle Reader die neue Epoche gemeldet haben. Dann kann der Retry trotzdem scheitern.
- **Kein produktiver Index mit `extent` und `backing`:** Der Bulk-Pfad ist jetzt auch auf dem Device getestet, aber nur mit `FlatIndex`. Ein mmap-gestützter Index fehlt weiterhin.
- **Keine produktive GPU-Engine in der Fassade:** Der Smoke-Test nutzt deshalb eine skriptgesteuerte Engine mit echtem Device-Roundtrip. Ein End-to-End-Lauf mit dem e5-small-Modell über `Resident` ist damit nicht abgedeckt.
- **Alte Warnungen:** `drive_build`, `skip_bytes` und `Seqlock::from_raw` sind weiterhin „never used“. In `dtype.rs` sind `elems_to_bytes` und `bytes_to_elems` ebenfalls ungenutzt. `Doorbell::from_mapped` meldet dasselbe. Außerdem hat `exec.rs:337` einen irrefutable `if let`.
- **Pinning:** `Evictor::pin` wird von `IndexRuntime` nicht aufgerufen. Der Schutz läuft derzeit nur über die Epochen beim Reclaim.