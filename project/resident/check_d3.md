D1 bis D4 wurden um gesetzt das folgende sind abweichungen aus D1-D3. Prüfe und korrigiere wo möglich.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert

I deviated from the plan in three places:
- **Time:** it is passed in as `now_ms: u64`, as in the existing `processor::ResourceSlot`. `Instant` and the `Clock` trait don't fit, because `Clock` only exists under `cfg(test)` or the `mock` feature. The eviction idle clock starts on the first `evict_idle` call after the last lease ends, so dropping a lease needs no clock.
- **`OnceCell`:** eviction replaces the whole `Arc<OnceCell>` inside an `RwLock`, because a `OnceCell` can't be reset in place. Taking a lease and evicting are therefore mutually exclusive, which also means the `Evicting` state is only briefly visible.
- **`Subengine` trait:** it has no `device()`, `resource()` or `run_ops()` yet, since D2 and the op types aren't wired in. There is also no `Subengine` implementation yet, only the trait.

D2 ist umgesetzt, aber ich konnte weder `cargo build` noch `cargo test` ausführen. Geprüft ist nur die Syntax der geänderten Dateien. Die Gates G-STREAM und G-PTR sind nicht durchlaufen, weil dafür eine GPU nötig ist.

- Der bestehende Trait `MemoryManager` kennt keine Arena-Klasse. Deshalb gibt es daneben `DeviceMemory` mit `alloc_in`, `release_after` und `budget`. `MemoryManager` habe ich nicht geändert.
- `StagingRing` ist generisch und ohne `cuda` testbar. Nur die pinned Variante (`PinnedStaging`) liegt hinter `cuda`.
- `copy_d2d` verlangt, dass Quelle und Ziel in verschiedenen Arena-Klassen liegen.