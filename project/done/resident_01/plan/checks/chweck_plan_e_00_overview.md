Die ganze Gruppe E wurde erledigt, prüfe die Gruppen DoD.

- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_00_overview.md)

SystemInit:  id=16227a29-9d5e-41d9-b45d-4b0bf9302762, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich lese zuerst die DoD-Abschnitte der Einzelpläne E1–E9 und sehe mir den Code-Stand an.
ReasoningToken: 64
Control Request:
```yaml
id: c74e-1
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/project/resident/plan
    script:
    - "for f in plan_e_0[1-9].md; do echo \"=== $f\"; awk '/Definition of Done/{p=1} p' $f | head -40; done"
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - git status --short | head -50
    - find src -type f -name '*.rs' | sort
    - wc -l $(find src -name '*.rs') | tail -1
    - cat Cargo.toml
  reason: DoD-Abschnitte der Pläne E1-E9 lesen und Code-Struktur sowie Cargo.toml des Crates erfassen
```
/answer c74e-1 allow
Control Request:
```yaml
id: c74e-2
result:
  content:
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-t07kkidt.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-1q8d38os.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  structuredContent:
    results:
    - stdout_file: /tmp/mcpc-stdout-t07kkidt.log
      stdout_file_file_stats:
        path: /tmp/mcpc-stdout-t07kkidt.log
        size_bytes: 28505
        lines: 314
        words: 3749
        complexity: 0.634
        created: 2026-10-25T12:00:06.719845+00:00
        modified: 2026-10-25T12:00:06.719845+00:00
        accessed: 2026-10-25T12:00:06.719845+00:00
        line_length_max: 461
        line_length_min: 0
        line_length_avg: 89.04
        words_per_line_avg: 11.94
        checksum: c42fda56da07c7b950399c3ee499736d39536ab51d30780c4e28b817aeddc27a
      stdout_file_ast_outline:
      - id: file
        lines: 1-314
        signature: === plan_e_01.md
    - stdout_file: /tmp/mcpc-stdout-1q8d38os.log
      stdout_file_file_stats:
        path: /tmp/mcpc-stdout-1q8d38os.log
        size_bytes: 8753
        lines: 289
        words: 541
        complexity: 0.536
        created: 2026-10-25T12:00:06.721844+00:00
        modified: 2026-10-25T12:00:06.721844+00:00
        accessed: 2026-10-25T12:00:06.721844+00:00
        line_length_max: 97
        line_length_min: 0
        line_length_avg: 29.29
        words_per_line_avg: 1.87
        checksum: 3db1231ad37c5864dff9071a2848544b5f6b132c813c837e3c85ba986f01028a
      stdout_file_ast_outline:
      - id: file
        lines: 1-289
        signature: M ../project/resident/plan/runs/chweck_plan_e_00_overview.md
```
/answer c74e-2 deny nutze AST zum lesen der DoDs
Cannot process command: no active Claude Code session exists
Text:
 Dann nutze ich die AST-Tools.
Control Request:
```yaml
id: c74e-3
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_01.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_02.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_03.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_04.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_05.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_06.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_07.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_08.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_09.md
  text: Definition of Done
  reason: DoD-Abschnitte der Einzelpläne E1-E9 finden
```
/answer c74e-3 allow
Control Request:
```yaml
id: c74e-4
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_01.md
      nodes:
      - id: de0012.d9113e
        code: |+
          ## 9. Definition of Done
          - Lazy-Manager konstruierbar, CPU-Variante liest Index korrekt, mmap-Residency-Hooks getestet, GPU-Hülle kompiliert mit und ohne `cuda`. Keine Ressourcenbelegung vor erstem Zugriff.

    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_02.md
      nodes:
      - id: c65b93.d9113e
        code: |+
          ## 9. Definition of Done
          - Registry mit Doppelprüfung; mindestens Range/Subtree/Prefix/Centroid-Resolver; `EntrySet` roaring-basiert mit Union; alle genannten Tests grün ohne GPU.

    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_03.md
      nodes:
      - id: f79d9b.d9113e
        code: |+
          ## 9. Definition of Done
          - Eine Arena-Implementierung (konsolidiert mit D3), 256-B-Offset-Suballokation, budgetierte CoW-Heuristik, Pinned-Staging/Copy-Stream referenziert; Host-Mock-Pfad testbar ohne GPU.

    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_04.md
      nodes:
      - id: 6c1f83.d9113e
        code: |+
          ## 9. Definition of Done
          - CoW + atomarer Root-Swap + Epoch-Reclaim implementiert und per `loom`/Host-Mock verifiziert; Publish-Reihenfolge erzwungen; Pointer-Lesezeitpunkt beider Modi korrekt; reale Atomics hinter `cuda`, Logik ohne GPU grün.

    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_05.md
      nodes:
      - id: c7e2bc.8ff795
        code: |+
          ## 8. Offene Verifikationen
          - Doorbell-Ring-Sichtbarkeit (PCIe posted writes, `__threadfence_system`) auf Zielhardware (H1 Punkt 2/3).

    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_06.md
      nodes:
      - id: 5bafa5.d9113e
        code: |+
          ## 9. Definition of Done
          - Zustandsbasierter Scanner mit Quittierung-vor-Scan, korrekter Lade-Lesereihenfolge, Echo-Unterdrückung über `apply`-Rückgabe; globale Stamps `!=`; vollständige Testmatrix oben grün ohne GPU.

    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_07.md
      nodes:
      - id: 8e9063.d9113e
        code: |+
          ## 9. Definition of Done
          - `plan_load`, aggregation, single-flight and evictor are implemented; the density threshold is exported as a calibration axis; all tests are green through the host mock without a GPU. **Met**, except the concrete segment and zero-copy `LoadTransfer` implementations (see Result).

    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_08.md
      nodes:
      - id: 49ea49.8ff795
        code: |
          ## 8. Offene Verifikationen
          - Aggregierter `atomicAdd` pro Warp/Block und `__threadfence`-Sichtbarkeit auf GTX 1660: **verifiziert** (see implementation status).
          - CUB/Thrust `ExclusiveScan`-Verfügbarkeit im Build: **verifiziert**; the crate still uses the CPU prefix sum.
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_09.md
      nodes:
      - id: ee280f.d9113e
        code: |+
          ## 9. Definition of Done
          - Erweiterbares Parameterobjekt nachgewiesen; Referenz-Kaskade als Addon über die normale Queue demonstriert; Konzeptnotiz zur Kern/Addon/Oberschicht-Grenze vorhanden; **keine** Iteration/Kaskade/Rekursion im Kern.
```
/answer c74e-4 allow
Control Request:
```yaml
id: c74e-5
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src
    pattern: \.rs$
  reason: Code-Dateien des Crates auflisten
```
/answer c74e-5 allow
Control Request:
```yaml
id: c74e-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src
      entries:
      - ".: (1 file)"
      - lib.rs
      - "./bin: (1 file)"
      - xyrag.rs
      - "./core: (9 files)"
      - engine.rs
      - "./layers: (5 files)"
      - dir_cache.rs
      - "./layers/trigram: (7 files)"
      - index.rs
      - "./layers/trigram/test: (3 files)"
      - normalize.rs
      - "./resident: (6 files)"
      - backend.rs
      - "./resident/facade: (8 files)"
      - blocking.rs
      - "./resident/facade/test: (3 files)"
      - addon.rs
      - "./resident/index: (3 files)"
      - memory.rs
      - "./resident/index/test: (2 files)"
      - memory.rs
      - "./resident/kernel: (13 files)"
      - cosine.rs
      - "./resident/kernel/test: (10 files)"
      - cosine.rs
      - "./resident/manager: (16 files)"
      - advice.rs
      - "./resident/manager/test: (12 files)"
      - advice.rs
      - "./resident/memory: (13 files)"
      - arena.rs
      - "./resident/memory/test: (9 files)"
      - arena.rs
      - "./resident/model: (8 files)"
      - batch.rs
      - "./resident/model/test: (6 files)"
      - batch.rs
      - "./resident/op: (3 files)"
      - lease.rs
      - "./resident/op/test: (3 files)"
      - lease.rs
      - "./resident/planner: (6 files)"
      - adapter.rs
      - "./resident/planner/test: (2 files)"
      - adapter.rs
      - "./resident/precision: (6 files)"
      - axis.rs
      - "./resident/precision/test: (5 files)"
      - axis.rs
      - "./resident/processor: (11 files)"
      - completion.rs
      - "./resident/processor/test: (6 files)"
      - completion.rs
      - "./resident/queue: (5 files)"
      - mod.rs
      - "./resident/segment: (5 files)"
      - error.rs
      - "./resident/segment/test: (4 files)"
      - mapping.rs
      - "./resident/subengine: (6 files)"
      - device.rs
      - "./resident/subengine/test: (4 files)"
      - device.rs
      - "./resident/test: (2 files)"
      - error.rs
      - "./resident/tokenize: (4 files)"
      - error.rs
      - "./resident/tokenize/test: (1 file)"
      - mod.rs
      warning: Result reduced because the total number of entries (208) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer c74e-6 allow
Control Request:
```yaml
id: c74e-7
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
    pattern: \.rs$
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    pattern: \.rs$
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    pattern: \.rs$
  reason: "Manager-, Memory- und Facade-Module der Gruppe E auflisten"
```
/answer c74e-7 allow
ReasoningToken: 140
Control Request:
```yaml
id: c74e-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      entries:
      - ".:"
      - advice.rs
      - arena.rs
      - blocking.rs
      - build.rs
      - doorbell.rs
      - evict.rs
      - flight.rs
      - mod.rs
      - residency.rs
      - scope.rs
      - seqlock.rs
      - slot.rs
      - swap.rs
      - sync.rs
      - tracker.rs
      - upload.rs
      - "./test:"
      - advice.rs
      - arena.rs
      - blocking.rs
      - build.rs
      - evict.rs
      - flight.rs
      - residency.rs
      - scope.rs
      - slot.rs
      - sync.rs
      - tracker.rs
      - upload.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      entries:
      - ".:"
      - arena.rs
      - budget.rs
      - cpu.rs
      - cuda.rs
      - dtype.rs
      - error.rs
      - freelist.rs
      - layout.rs
      - manager.rs
      - mod.rs
      - region.rs
      - staging.rs
      - wrap_gate.rs
      - "./test:"
      - arena.rs
      - budget.rs
      - cpu.rs
      - cuda.rs
      - dtype.rs
      - freelist.rs
      - layout.rs
      - manager.rs
      - staging.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      entries:
      - ".:"
      - blocking.rs
      - error.rs
      - extensions.rs
      - handle.rs
      - mod.rs
      - params.rs
      - resident.rs
      - task.rs
      - "./test:"
      - addon.rs
      - extensions.rs
      - resident.rs
```
/answer c74e-8 allow
Control Request:
```yaml
id: c74e-9
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_05.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_08.md
  reason: "Struktur von E5 und E8 prüfen, da dort kein DoD-Abschnitt per Text gefunden wurde"
```
/answer c74e-9 allow
Control Request:
```yaml
id: c74e-a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_05.md
      nodes:
      - id: c7e2bc
        signature: Umsetzungsplan E5 · Kommunikationsslot (feste Reihenfolge)
        children:
        - id: c7e2bc.494bb5
          signature: 1. Ziel & Abgrenzung
          children:
          - id: c7e2bc.494bb5.uTaOvg|0pqs14
            signature: Der **Kommunikationsslot** zw…
          - id: c7e2bc.494bb5.hVZmNe|teqDHJ
            signature: Weil nur in Lücken mutiert wi…
          - id: c7e2bc.494bb5.7uW9ZJ|pm1PZj
            signature: "Nicht hier: Echo-Vermeidung/S…"
        - id: c7e2bc.12b1dd
          signature: 2. Voraussetzungen
        - id: c7e2bc.6ad869
          signature: 3. Zu bauende Artefakte
          children:
          - id: c7e2bc.6ad869.k1Sn8d|iHiCQS
            signature: |-
              ```rust
              pub struct Hit { pub …
        - id: c7e2bc.c9b059
          signature: 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
          children:
          - id: c7e2bc.c9b059.3Qut0X|4fo4Rg
            signature: '**Reihenfolge-Invarianz (Mapp…'
          - id: c7e2bc.c9b059.v2x4nx|fzjSJ0
            signature: '**Doorbell/Mailbox** (context…'
          - id: c7e2bc.c9b059.DvpzME|49I6W7
            signature: '**Adresse → Entry** (context_…'
          - id: c7e2bc.c9b059.WVDY3e|phc8KE
            signature: '**Indexieren während Suche** …'
          - id: c7e2bc.c9b059.H31wj8|WPy6HQ
            signature: '**Ergebnisstatus beim Auflöse…'
        - id: c7e2bc.720730
          signature: 5. Umsetzungsschritte
        - id: c7e2bc.0c3143
          signature: 6. Fehler / Logging / Feature-Flags
        - id: c7e2bc.c0dcae
          signature: 7. Tests (Q3)
        - id: c7e2bc.8ff795
          signature: 8. Offene Verifikationen
        - id: c7e2bc.d9113e
          signature: 9. Definition of Done
        - id: c7e2bc.5faa59
          signature: Result
          children:
          - id: c7e2bc.5faa59.Zkbas0|TVb2nb
            signature: E5 ist in `rag/src/resident/m…
          - id: c7e2bc.5faa59.RqoEPg|Oq5blk
            signature: '`run_slot` arbeitet in der fe…'
          - id: c7e2bc.5faa59.jH8S3c|XZezvN
            signature: Adressen werden bei festem La…
          - id: c7e2bc.5faa59.zL9jZQ|Xf1a0v
            signature: '**Abweichungen vom Plan:**'
          - id: c7e2bc.5faa59.nrCGYW|xeU33n
            signature: '**Nicht umgesetzt:**'
          - id: c7e2bc.5faa59.4qKS3Y|xD7z4S
            signature: Ich habe die ausgelassenen Pu…
          - id: c7e2bc.5faa59.x3ofkY|uETzwD
            signature: Beim Build erscheinen neue `d…
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_08.md
      nodes:
      - id: 49ea49
        signature: Umsetzungsplan E8 · Kernel schreibt Segmente (Indexaufbau)
        children:
        - id: 49ea49.494bb5
          signature: 1. Ziel & Abgrenzung
          children:
          - id: 49ea49.494bb5.Tr0TgE|IQYkAF
            signature: "Der Pfad, auf dem der **Kerne…"
          - id: 49ea49.494bb5.BZKNwn|xpysTU
            signature: "Nicht hier: der persistente R…"
        - id: 49ea49.12b1dd
          signature: 2. Voraussetzungen
        - id: 49ea49.6ad869
          signature: 3. Zu bauende Artefakte
          children:
          - id: 49ea49.6ad869.lM3cg8|Le9x8z
            signature: |-
              ```rust
              pub struct WriteRegio…
        - id: 49ea49.c9b059
          signature: 4. Entscheidungen (aufgelöste „Zu klären"-Punkte)
          children:
          - id: 49ea49.c9b059.rNvUVU|HrSXz9
            signature: '**Output-Arena mit atomarem C…'
          - id: 49ea49.c9b059.l0i5ir|QuAfIT
            signature: '**Offset-Tabellen / Rückabbil…'
          - id: 49ea49.c9b059.MTTBgE|jKxJT3
            signature: '**Getrennte Ringe Embedding v…'
          - id: 49ea49.c9b059.M0Z8gb|hHLqxP
            signature: '**Schreiben in D7-Slots:** Ma…'
        - id: 49ea49.720730
          signature: 5. Umsetzungsschritte
        - id: 49ea49.0c3143
          signature: 6. Fehler / Logging / Feature-Flags
        - id: 49ea49.c0dcae
          signature: 7. Tests (Q3)
        - id: 49ea49.8ff795
          signature: 8. Offene Verifikationen
        - id: 49ea49.d9113e
          signature: 9. Definition of Done
        - id: 49ea49.f79047
          signature: 10. Implementation status
          children:
          - id: 49ea49.f79047.UiZuT1|yM9XGf
            signature: Implemented in `rag/src/resid…
          - id: 49ea49.f79047.a3cffb
            signature: "Verification results (GTX 1660 SUPER, sm_75, CUDA 13.3)"
          - id: 49ea49.f79047.39fdec
            signature: Tests
        - id: 49ea49.5faa59
          signature: Result
          children:
          - id: 49ea49.5faa59.WyKRe9|kpSeVb
            signature: '**Status E8**'
          - id: 49ea49.5faa59.IimupV|mlVmAo
            signature: Der Code ist fertig und getes…
          - id: 49ea49.5faa59.ednjA8|CFYL9s
            signature: '**Umgesetzt**'
          - id: 49ea49.5faa59.XntaZl|yrsBVP
            signature: '**Tests**'
          - id: 49ea49.5faa59.UL4Ec9|HRkNj7
            signature: '**Offene Verifikationen**'
          - id: 49ea49.5faa59.cOjvCq|b8wufM
            signature: '**Noch offen:** Das Statusupd…'
          - id: 49ea49.5faa59.VSIMDa|CFWnw4
            signature: Jetzt ist E8 vollständig erle…
          - id: 49ea49.5faa59.pKIkvH|Cjqjkx
            signature: '**Nachgeholt**'
          - id: 49ea49.5faa59.qN21zj|Xk8WVj
            signature: '**Testlage**'
          - id: 49ea49.5faa59.tVgnrx|Oy4VSn
            signature: '**Nicht umgesetzt**'
```
/answer c74e-a allow
Control Request:
```yaml
id: c74e-b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_05.md
    ids:
    - c7e2bc.d9113e
    - c7e2bc.5faa59
    - c7e2bc.c0dcae
    - c7e2bc.8ff795
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_08.md
    ids:
    - 49ea49.d9113e
    - 49ea49.f79047
    - 49ea49.5faa59
    - 49ea49.c0dcae
  reason: "DoD, Tests und Ergebnisberichte von E5 und E8 lesen"
```
/answer c74e-b allow
ReasoningToken: 119
Control Request:
```yaml
id: c74e-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_05.md
      nodes:
      - id: c7e2bc.d9113e
        type: section
        code: |+
          ## 9. Definition of Done
          - `run_slot` hält die feste Reihenfolge; Adress-Rückabbildung für Fixed/Variable; Ergebnisstatus korrekt; frisch geschriebene Adressen auflösbar; alle Tests grün ohne GPU.

      - id: c7e2bc.5faa59
        type: section
        signature: Result
        children:
        - id: c7e2bc.5faa59.Zkbas0|TVb2nb
          type: paragraph
          code: |
            E5 ist in `rag/src/resident/manager/slot.rs` umgesetzt und in `manager/mod.rs` als `pub mod slot;` registriert. Die 6 neuen Tests laufen grün ohne GPU (`cargo test --lib resident::manager::slot`). Die übrigen 334 Tests habe ich nicht ausgeführt.
        - id: c7e2bc.5faa59.RqoEPg|Oq5blk
          type: paragraph
          code: |
            `run_slot` arbeitet in der festen Reihenfolge aus dem Plan:
        - id: c7e2bc.5faa59.jH8S3c|XZezvN
          type: paragraph
          code: |
            Adressen werden bei festem Layout mit `off / stride` aufgelöst, bei variablem Layout per Binärsuche über die Prefix-Summen. `AddressMap` ist append-only und per `clone()` als stabiler Snapshot nutzbar.
        - id: c7e2bc.5faa59.zL9jZQ|Xf1a0v
          type: paragraph
          code: |
            **Abweichungen vom Plan:**
        - id: c7e2bc.5faa59.nrCGYW|xeU33n
          type: paragraph
          code: |
            **Nicht umgesetzt:**
        - id: c7e2bc.5faa59.4qKS3Y|xD7z4S
          type: paragraph
          code: |
            Ich habe die ausgelassenen Punkte nachgezogen. Die gesamte Suite läuft grün: 346 Tests ohne GPU. Der `loom`-Test zum Doorbell läuft ebenfalls grün, und `cargo check --features cuda` läuft durch. Auf echter Hardware habe ich nichts getestet.
        - id: c7e2bc.5faa59.x3ofkY|uETzwD
          type: paragraph
          code: |
            Beim Build erscheinen neue `dead_code`-Warnungen für `snapshot_at`, `publish` und `run_slot_at`. Der Manager, der sie aufruft (E1), ist noch nicht verdrahtet. Das entspricht den vorhandenen Warnungen bei Arena und Swap.
      - id: c7e2bc.c0dcae
        type: section
        code: |+
          ## 7. Tests (Q3)
          - **Reihenfolge:** Ergebnis auf **frisch geschriebene** Adresse löst korrekt auf (Schritt 1 vor 2).
          - **Status:** entfernter Entry wird verworfen; ausstehendes Update markiert `outdated`.
          - Variable-Layout: Binärsuche liefert korrekten Entry an Segmentgrenzen; Fixed: `off/stride`.
          - Update-Last: Mapping-Änderung wirkt erst im nächsten Batch (Phase 3 zuletzt).
          - Indexieren während Suche: Snapshot des Batch-Starts bleibt stabil, während Writer append-only ergänzt.
          - `FakeKernel` liefert programmierte `writes`+`hits`; keine GPU.

      - id: c7e2bc.8ff795
        type: section
        code: |+
          ## 8. Offene Verifikationen
          - Doorbell-Ring-Sichtbarkeit (PCIe posted writes, `__threadfence_system`) auf Zielhardware (H1 Punkt 2/3).

    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_08.md
      nodes:
      - id: 49ea49.d9113e
        type: section
        code: |+
          ## 9. Definition of Done
          - Cursor-Write mit Teilergebnis, Offset-Tabellen/Fixed-Rückabbildung, getrennte Embedding/MaxSim-Ringe mit Reihenfolge-Garantie, D7-Upsert; H1-Hook offen; alle Tests grün ohne GPU.


      - id: 49ea49.f79047
        type: section
        signature: 10. Implementation status
        children:
        - id: 49ea49.f79047.UiZuT1|yM9XGf
          type: paragraph
          code: |
            Implemented in `rag/src/resident/manager/build.rs`; tests in `manager/test/build.rs`.
        - id: 49ea49.f79047.a3cffb
          type: section
          code: |+
            ### Verification results (GTX 1660 SUPER, sm_75, CUDA 13.3)
            - Warp-aggregated cursor: 1000 claims of 16 bytes with capacity 10000 yield exactly 625 contiguous offsets, overflow flag set, `limit == cap`.
            - Host visibility while the kernel is still running (mapped pinned memory, host polls the flag, no stream synchronization): 0 of 200 rounds with stale data for both `__threadfence()` and `__threadfence_system()`. Empirical only; the system-scope fence is the formally sufficient one and is used by the kernel. That the kernel was still running at read time is not measured.
            - CUB `DeviceScan::ExclusiveSum` builds with `nvcc -arch=sm_75` (path from `.cargo/config.toml`) and returns correct sums. Not integrated into the crate.

        - id: 49ea49.f79047.39fdec
          type: section
          code: |
            ### Tests
            - No-GPU: 13 E8 tests plus the existing `resident::` suite pass.
            - GPU (ignored by default): cursor test and two host-visibility tests pass.
      - id: 49ea49.5faa59
        type: section
        signature: Result
        children:
        - id: 49ea49.5faa59.WyKRe9|kpSeVb
          type: paragraph
          code: |
            **Status E8**
        - id: 49ea49.5faa59.IimupV|mlVmAo
          type: paragraph
          code: |
            Der Code ist fertig und getestet. Das Plandokument `plan_e_08.md` habe ich noch nicht aktualisiert.
        - id: 49ea49.5faa59.ednjA8|CFYL9s
          type: paragraph
          code: |
            **Umgesetzt**
        - id: 49ea49.5faa59.XntaZl|yrsBVP
          type: paragraph
          code: |
            **Tests**
        - id: 49ea49.5faa59.UL4Ec9|HRkNj7
          type: paragraph
          code: |
            **Offene Verifikationen**
        - id: 49ea49.5faa59.cOjvCq|b8wufM
          type: paragraph
          code: |
            **Noch offen:** Das Statusupdate im `plan_e_08.md` (auf Englisch, mit den Verifikationsständen). Soll ich das jetzt eintragen?
        - id: 49ea49.5faa59.VSIMDa|CFWnw4
          type: paragraph
          code: |
            Jetzt ist E8 vollständig erledigt. Sie hatten recht, vorher war noch einiges offen.
        - id: 49ea49.5faa59.pKIkvH|Cjqjkx
          type: paragraph
          code: |
            **Nachgeholt**
        - id: 49ea49.5faa59.qN21zj|Xk8WVj
          type: paragraph
          code: |
            **Testlage**
        - id: 49ea49.5faa59.tVgnrx|Oy4VSn
          type: paragraph
          code: |
            **Nicht umgesetzt**
      - id: 49ea49.c0dcae
        type: section
        code: |+
          ## 7. Tests (Q3)
          - Teilergebnis bei Bereichsmangel: Überlauf-Flag, Host vergibt neuen Bereich, erster Teil bleibt gültig.
          - Rückabbildung: Variable-`OffsetTable` Binärsuche korrekt an Grenzen; Fixed `off/stride`.
          - **Indexieren während Suche:** Embedding-Writes (Schritt 1) sind für MaxSim-Auflösung (Schritt 2) desselben Slots sichtbar → jede Hit-Adresse auflösbar.
          - Getrennte Ringe arbeiten unabhängig (SPSC-Fakes).
          - D7-Upsert: neuer Slot gültig, alter Slot-Bit gelöscht; kein In-Place.
```
/answer c74e-c allow
Control Request:
```yaml
id: c74e-d
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_01.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_02.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_03.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_04.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_06.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_07.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_09.md
  regexp: ^## (Result|Nicht umgesetzt|Offene Verifikationen|8\. Offene)
  reason: Ergebnis- und Offene-Verifikationen-Abschnitte der übrigen Pläne finden
```
/answer c74e-d deny diese Abschnitte würden deine Analyse verfälschen. Du sollst die DoDs unabhängig prüfen
Text:
 Die Ergebnisberichte der Pläne lasse ich beiseite und prüfe stattdessen gegen Code und Tests. Zuerst lasse ich die Build- und Test-Läufe laufen.
Control Request:
```yaml
id: c74e-e
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - export PATH=/usr/local/cuda-13.3/bin:$PATH
    - "cargo test --lib resident:: 2>&1 | tail -15"
    - echo ---CHECK; cargo check --features cuda 2>&1 | tail -5
    - echo ---CLIPPY-WARN; cargo build 2>&1 | grep -c '^warning'
  reason: "No-GPU-Testsuite, cuda-Feature-Check und Warnungsanzahl ausführen, um die DoD-Behauptungen unabhängig zu prüfen"
```
/answer c74e-e allow
ReasoningToken: 89
Control Request:
```yaml
id: c74e-f
result:
  structuredContent:
    results:
    - stdout: |
        test resident::manager::residency::tests::transfer_error_reaches_all_waiters ... ok
        test resident::manager::flight::tests::error_reaches_every_waiter_and_next_call_retries ... ok
        test resident::subengine::exec::tests::event_completes_after_prior_jobs ... ok
        test resident::queue::queue::tests::concurrent_pushes_are_all_linked ... ok
        test resident::planner::tests::starved_hint_changes_engines_not_ops ... ok
        test resident::subengine::exec::tests::wait_orders_work_across_lanes ... ok
        test resident::processor::tests::first_failing_op_fails_task_and_remaining_ops_are_dropped ... ok
        test resident::processor::completion::tests::concurrent_callback_and_pollers_process_once ... ok
        test resident::queue::queue::tests::iteration_matches_push_order ... ok
        test resident::tokenize::tests::e5_small_checkpoint_tokenizes_with_special_tokens ... ok
        test resident::queue::queue::tests::drop_long_chain_does_not_overflow_stack ... ok
        test resident::manager::seqlock::tests::concurrent_reader_never_sees_a_torn_snapshot ... ok

        test result: ok. 391 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 13.01s

        ---CHECK
        203 |         pub fn new(memory: Arc<CudaMemoryManager>, lane: CudaLane) -> Self {
            |                ^^^

        warning: `xy-ai-rag` (lib) generated 96 warnings (run `cargo fix --lib -p xy-ai-rag` to apply 1 suggestion)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.13s
        ---CLIPPY-WARN
        82
```
/answer c74e-f allow
Control Request:
```yaml
id: c74e-10
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/scope.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/arena.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/slot.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/sync.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/residency.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/build.rs
  reason: Struktur der Manager-Module E1-E8 erfassen
```
/answer c74e-10 allow
Control Request:
```yaml
id: c74e-11
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! StreamedIndexManager, ScopeResolver, update queue, residency."
      - id: advice
        signature: pub mod advice;
      - id: blocking
        signature: pub mod blocking;
      - id: doorbell
        signature: pub mod doorbell;
      - id: build
        signature: pub mod build;
      - id: arena
        signature: pub(crate) mod arena;
      - id: scope
        signature: pub mod scope;
      - id: slot
        signature: pub mod slot;
      - id: swap
        signature: pub(crate) mod swap;
      - id: seqlock
        signature: pub mod seqlock;
      - id: sync
        signature: pub mod sync;
      - id: tracker
        signature: pub mod tracker;
      - id: evict
        signature: pub(crate) mod evict;
      - id: flight
        signature: pub(crate) mod flight;
      - id: residency
        signature: pub(crate) mod residency;
      - id: upload
        signature: pub(crate) mod upload;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/scope.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Scope resolution: the manager turns a task scope into a compact entry set."
      - id: o7EVWj|HAvVMI
        signature: use std::collections::HashMap;
      - id: CellSelector
        signature: "pub trait CellSelector: Send + Sync {"
        docstring: "Picks the cells nearest to the query from a resident summary (e.g. `S = C·Qᵀ`, …"
        children:
        - id: CellSelector.select
          signature: "fn select(     &self,     summary: &SummaryRef,     k: u32,     radius: Option<…"
      - id: ResolveCtx
        signature: "pub struct ResolveCtx {"
      - id: ScopeResolver
        signature: "pub trait ScopeResolver: Send + Sync {"
        children:
        - id: ScopeResolver.method
          signature: fn method(&self) -> AccessMethod;
        - id: ScopeResolver.resolve
          signature: "fn resolve(     &self,     index: &dyn Index,     scope: &Scope,     ctx: &Reso…"
      - id: DuplicateResolver
        signature: pub struct DuplicateResolver(pub AccessMethod);
      - id: ResolverRegistry
        signature: "pub struct ResolverRegistry {"
      - id: impl_ResolverRegistry
        signature: "impl ResolverRegistry {"
        children:
        - id: impl_ResolverRegistry.new
          signature: "pub fn new() -> Self {"
        - id: impl_ResolverRegistry.with_defaults
          signature: "pub fn with_defaults() -> Self {"
        - id: impl_ResolverRegistry.register
          signature: "pub fn register(     &mut self,     resolver: Arc<dyn ScopeResolver>, ) -> Resu…"
        - id: impl_ResolverRegistry.resolve
          signature: "pub fn resolve(     &self,     index: &dyn Index,     scope: &Scope,     ctx: &…"
      - id: required_method
        signature: "fn required_method(scope: &Scope) -> Option<AccessMethod> {"
      - id: resolve_direct
        signature: "fn resolve_direct(index: &dyn Index, scope: &Scope) -> EntrySet {"
      - id: mismatch
        signature: "fn mismatch(method: AccessMethod) -> IndexError {"
      - id: RangeResolver
        signature: pub struct RangeResolver;
      - id: impl_ScopeResolver_for_RangeResolver
        signature: "impl ScopeResolver for RangeResolver {"
        children:
        - id: impl_ScopeResolver_for_RangeResolver.method
          signature: "fn method(&self) -> AccessMethod {"
        - id: impl_ScopeResolver_for_RangeResolver.resolve
          signature: "fn resolve(     &self,     _: &dyn Index,     scope: &Scope,     _: &ResolveCtx…"
      - id: SubtreeResolver
        signature: pub struct SubtreeResolver;
      - id: impl_ScopeResolver_for_SubtreeResolver
        signature: "impl ScopeResolver for SubtreeResolver {"
        children:
        - id: impl_ScopeResolver_for_SubtreeResolver.method
          signature: "fn method(&self) -> AccessMethod {"
        - id: impl_ScopeResolver_for_SubtreeResolver.resolve
          signature: "fn resolve(     &self,     index: &dyn Index,     scope: &Scope,     _: &Resolv…"
      - id: PrefixResolver
        signature: pub struct PrefixResolver;
        docstring: "Interprets the prefix as the leading big-endian bytes of the 64-bit entry id,"
      - id: impl_ScopeResolver_for_PrefixResolver
        signature: "impl ScopeResolver for PrefixResolver {"
        children:
        - id: impl_ScopeResolver_for_PrefixResolver.method
          signature: "fn method(&self) -> AccessMethod {"
        - id: impl_ScopeResolver_for_PrefixResolver.resolve
          signature: "fn resolve(     &self,     _: &dyn Index,     scope: &Scope,     _: &ResolveCtx…"
      - id: CentroidResolver
        signature: pub struct CentroidResolver;
      - id: impl_ScopeResolver_for_CentroidResolver
        signature: "impl ScopeResolver for CentroidResolver {"
        children:
        - id: impl_ScopeResolver_for_CentroidResolver.method
          signature: "fn method(&self) -> AccessMethod {"
        - id: impl_ScopeResolver_for_CentroidResolver.resolve
          signature: "fn resolve(     &self,     index: &dyn Index,     scope: &Scope,     ctx: &Reso…"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/arena.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index-segment use of the shared arena allocator: aligned segments, copy-on-…"
      - id: ku3dRm|3SW5gg
        signature: use std::ops::Range;
      - id: WRTBpG|CxUsUc
        signature: /// Minimum alignment of every segment offset.
      - id: CowBudget
        signature: "pub struct CowBudget {"
        docstring: Space that must stay free so a replaced segment can be rebuilt next to its pred…
      - id: impl_CowBudget
        signature: "impl CowBudget {"
        children:
        - id: impl_CowBudget.headroom
          signature: "pub fn headroom(&self) -> u64 {"
        - id: impl_CowBudget.required
          signature: "pub fn required(&self) -> u64 {"
      - id: ArenaBudget
        signature: "pub struct ArenaBudget {"
      - id: Retired
        signature: "struct Retired {"
      - id: State
        signature: "struct State {"
      - id: impl_State
        signature: "impl State {"
        children:
        - id: impl_State.is_retired
          signature: "fn is_retired(&self, region: Region) -> bool {"
      - id: IndexArena
        signature: "pub struct IndexArena<'a> {"
      - id: impl_IndexArena__a_
        signature: "impl<'a> IndexArena<'a> {"
        children:
        - id: impl_IndexArena__a_.new
          signature: "pub fn new(inner: &'a ArenaAllocator, reserve: u64) -> Self {"
          docstring: '`reserve` bytes of the arena are never handed out.'
        - id: impl_IndexArena__a_.alloc_segment
          signature: "pub fn alloc_segment(&self, bytes: u64, align: u64) -> Result<Region, ManagerEr…"
        - id: impl_IndexArena__a_.reserve_writable
          signature: "pub fn reserve_writable(&self, cap: u64) -> Result<WritableRegion, ManagerError…"
          docstring: "Segment the build path fills incrementally through [`WritableRegion::claim`]."
        - id: impl_IndexArena__a_.free
          signature: "pub fn free(&self, region: Region) -> Result<(), ManagerError> {"
          docstring: Immediate release of a region no reader or in-flight work can reference.
        - id: impl_IndexArena__a_.free_after
          signature: "pub fn free_after(     &self,     region: Region,     event: EventHandle, ) -> …"
          docstring: Release once all work recorded before `event` has finished.
        - id: impl_IndexArena__a_.retire
          signature: "pub fn retire(&self, region: Region, epoch: u64) -> Result<(), ManagerError> {"
          docstring: Marks `region` as replaced by the version published at `epoch`.
        - id: impl_IndexArena__a_.reclaim
          signature: "pub fn reclaim(&self, min_reader_epoch: u64) -> usize {"
          docstring: Frees every retired region that no reader at `min_reader_epoch` or later can st…
        - id: impl_IndexArena__a_.begin_cow
          signature: "pub fn begin_cow(&self, old: Region) -> Result<CowTicket<'_, 'a>, ManagerError>…"
          docstring: Starts a copy-on-write replacement of `old`; waits while another one is in prog…
        - id: impl_IndexArena__a_.try_begin_cow
          signature: "pub fn try_begin_cow(     &self,     old: Region, ) -> Result<Option<CowTicket<…"
          docstring: "Like [`begin_cow`](Self::begin_cow), but `Ok(None)` when another replacement is…"
        - id: impl_IndexArena__a_.cow_budget
          signature: "pub fn cow_budget(&self) -> CowBudget {"
        - id: impl_IndexArena__a_.budget
          signature: "pub fn budget(&self) -> ArenaBudget {"
        - id: impl_IndexArena__a_.trace_utilization
          signature: "pub fn trace_utilization(&self) {"
        - id: impl_IndexArena__a_.start_cow
          signature: "fn start_cow<'g>(     &'g self,     gate: MutexGuard<'g, ()>,     old: Region, …"
        - id: impl_IndexArena__a_.admit_and_alloc
          signature: "fn admit_and_alloc(     &self,     bytes: u64,     align: u64,     headroom: u6…"
        - id: impl_IndexArena__a_.ensure_owned
          signature: "fn ensure_owned(&self, st: &State, region: Region) -> Result<(), ManagerError> {"
      - id: CowTicket
        signature: "pub struct CowTicket<'a, 'b> {"
        docstring: Replacement segment under construction. Dropping it without `commit` discards t…
      - id: impl_CowTicket_______
        signature: "impl CowTicket<'_, '_> {"
        children:
        - id: impl_CowTicket_______.old_region
          signature: "pub fn old_region(&self) -> Region {"
        - id: impl_CowTicket_______.new_region
          signature: "pub fn new_region(&self) -> Region {"
        - id: impl_CowTicket_______.commit
          signature: "pub fn commit(mut self, epoch: u64) -> Result<Region, ManagerError> {"
          docstring: Publishes the copy as the version of `epoch`; the old segment is released by `r…
      - id: impl_Drop_for_CowTicket_______
        signature: "impl Drop for CowTicket<'_, '_> {"
        children:
        - id: impl_Drop_for_CowTicket_______.drop
          signature: "fn drop(&mut self) {"
      - id: WritableRegion
        signature: "pub struct WritableRegion {"
        docstring: Segment filled front to back by a single writer.
      - id: impl_WritableRegion
        signature: "impl WritableRegion {"
        children:
        - id: impl_WritableRegion.region
          signature: "pub fn region(&self) -> Region {"
        - id: impl_WritableRegion.written
          signature: "pub fn written(&self) -> u64 {"
        - id: impl_WritableRegion.remaining
          signature: "pub fn remaining(&self) -> u64 {"
        - id: impl_WritableRegion.claim
          signature: "pub fn claim(&mut self, len: u64, align: u64) -> Result<Range<u64>, ManagerErro…"
          docstring: Claims `len` bytes at `align`; returns the absolute arena offsets.
        - id: impl_WritableRegion.into_parts
          signature: "pub fn into_parts(self) -> (Region, u64) {"
      - id: map_memory
        signature: "fn map_memory(e: MemoryError) -> ManagerError {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/swap.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Mutation discipline of the index manager: copy-on-write segment tables, a s…"
      - id: 1gsLkl|IB2Xne
        signature: use std::sync::Arc;
      - id: P9fcuJ|yFCGCK
        signature: /// Root value before the first table is published.
      - id: Cell
        signature: "enum Cell {"
      - id: RootPointer
        signature: "pub struct RootPointer {"
        docstring: Naturally aligned 8-byte cell. Only the host stores; the device reads with `ld.…
      - id: impl_Send_for_RootPointer
        signature: "unsafe impl Send for RootPointer {"
      - id: impl_Sync_for_RootPointer
        signature: "unsafe impl Sync for RootPointer {"
      - id: impl_RootPointer
        signature: "impl RootPointer {"
        children:
        - id: impl_RootPointer.new
          signature: "pub fn new(initial: u64) -> Self {"
        - id: impl_RootPointer.from_mapped
          signature: "pub(crate) unsafe fn from_mapped(ptr: *mut u64) -> Self {"
          docstring: "Wraps a cell in pinned, mapped host memory."
        - id: impl_RootPointer.atomic
          signature: "fn atomic(&self) -> &AtomicU64 {"
        - id: impl_RootPointer.publish
          signature: "pub fn publish(&self, value: u64) {"
          docstring: Everything written before this call is visible to a reader that observes the va…
        - id: impl_RootPointer.load
          signature: "pub fn load(&self) -> u64 {"
      - id: SegmentTable
        signature: "pub struct SegmentTable {"
        docstring: Immutable once published; a new version is always a copy.
      - id: impl_SegmentTable
        signature: "impl SegmentTable {"
        children:
        - id: impl_SegmentTable.empty
          signature: "pub fn empty() -> Self {"
        - id: impl_SegmentTable.epoch
          signature: "pub fn epoch(&self) -> u64 {"
        - id: impl_SegmentTable.segments
          signature: "pub fn segments(&self) -> &[Region] {"
        - id: impl_SegmentTable.push
          signature: "pub fn push(&mut self, region: Region) {"
        - id: impl_SegmentTable.replace
          signature: "pub fn replace(&mut self, old: Region, new: Region) -> bool {"
          docstring: Swaps `old` for `new` in place of the copy; returns false when `old` is not lis…
        - id: impl_SegmentTable.remove
          signature: "pub fn remove(&mut self, region: Region) -> bool {"
        - id: impl_SegmentTable.encoded_len
          signature: "pub fn encoded_len(&self) -> u64 {"
          docstring: "Encoded size of the device-side table: epoch, count, then one entry per segment."
      - id: EpochTracker
        signature: "pub struct EpochTracker {"
        docstring: Versions published by the host and the epoch each reader reported having read.
      - id: impl_EpochTracker
        signature: "impl EpochTracker {"
        children:
        - id: impl_EpochTracker.new
          signature: "pub fn new(readers: usize) -> Self {"
        - id: impl_EpochTracker.published
          signature: "pub fn published(&self) -> u64 {"
        - id: impl_EpochTracker.advance
          signature: "fn advance(&self, epoch: u64) {"
        - id: impl_EpochTracker.report
          signature: "pub fn report(&self, reader: usize, epoch: u64) {"
          docstring: A finished batch read `epoch`; later batches of the reader read the same or a n…
        - id: impl_EpochTracker.quiesce
          signature: "pub fn quiesce(&self, reader: usize) {"
          docstring: "For a reader with no batch in flight: whatever it runs next reads the published…"
        - id: impl_EpochTracker.reader_min
          signature: "pub fn reader_min(&self) -> u64 {"
      - id: TableSink
        signature: "pub trait TableSink: Send + Sync {"
        docstring: Writes a table into device-visible memory.
        children:
        - id: TableSink.write
          signature: "fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError>;"
          docstring: "Returns only once the content is complete and visible to the device, i.e. after…"
      - id: TableSwap
        signature: "pub trait TableSwap {"
        children:
        - id: TableSwap.build_next
          signature: fn build_next(&self) -> SegmentTable;
          docstring: Copy of the current table with the next epoch.
        - id: TableSwap.publish
          signature: "fn publish(&self, table: SegmentTable) -> Result<u64, ManagerError>;"
          docstring: "Makes the content visible, then swaps the root with release semantics. Returns …"
        - id: TableSwap.reclaim
          signature: "fn reclaim(&self, upto: u64) -> usize;"
          docstring: Frees versions retired up to `upto` once every reader has reached them.
      - id: Snapshot
        signature: "pub struct Snapshot {"
        docstring: Root and epoch seen by one launch or one persistent batch.
      - id: BatchGuard
        signature: "pub struct BatchGuard<'a> {"
        docstring: Pins the version read at batch start; reports its epoch when dropped.
      - id: impl_BatchGuard____
        signature: "impl BatchGuard<'_> {"
        children:
        - id: impl_BatchGuard____.snapshot
          signature: "pub fn snapshot(&self) -> Snapshot {"
      - id: impl_Drop_for_BatchGuard____
        signature: "impl Drop for BatchGuard<'_> {"
        children:
        - id: impl_Drop_for_BatchGuard____.drop
          signature: "fn drop(&mut self) {"
      - id: Current
        signature: "struct Current {"
      - id: SwapManager
        signature: "pub struct SwapManager<'a, 'b> {"
      - id: impl_SwapManager__a__b_
        signature: "impl<'a, 'b> SwapManager<'a, 'b> {"
        children:
        - id: impl_SwapManager__a__b_.new
          signature: "pub fn new(     arena: &'a IndexArena<'b>,     sink: Box<dyn TableSink + 'a>,  …"
        - id: impl_SwapManager__a__b_.with_root
          signature: "pub fn with_root(     arena: &'a IndexArena<'b>,     sink: Box<dyn TableSink + …"
        - id: impl_SwapManager__a__b_.root
          signature: "pub fn root(&self) -> &RootPointer {"
        - id: impl_SwapManager__a__b_.epochs
          signature: "pub fn epochs(&self) -> &EpochTracker {"
        - id: impl_SwapManager__a__b_.current
          signature: "pub fn current(&self) -> Arc<SegmentTable> {"
        - id: impl_SwapManager__a__b_.snapshot
          signature: "pub fn snapshot(&self) -> Snapshot {"
          docstring: "Normal launches: the root is passed as a launch argument, so the version stays …"
        - id: impl_SwapManager__a__b_.begin_batch
          signature: "pub fn begin_batch(&self, reader: usize) -> BatchGuard<'_> {"
          docstring: "Persistent kernel: one acquire load per batch, never per work item."
      - id: impl_TableSwap_for_SwapManager_______
        signature: "impl TableSwap for SwapManager<'_, '_> {"
        children:
        - id: impl_TableSwap_for_SwapManager_______.build_next
          signature: "fn build_next(&self) -> SegmentTable {"
        - id: impl_TableSwap_for_SwapManager_______.publish
          signature: "fn publish(&self, table: SegmentTable) -> Result<u64, ManagerError> {"
        - id: impl_TableSwap_for_SwapManager_______.reclaim
          signature: "fn reclaim(&self, upto: u64) -> usize {"
      - id: tests
        signature: "mod tests {"
        children:
        - id: tests.flTjQt|5qRwZw
          signature: use super::*;
        - id: tests.HostSink
          signature: "struct HostSink<'a, 'b>(&'a IndexArena<'b>);"
        - id: tests.impl_TableSink_for_HostSink_______
          signature: "impl TableSink for HostSink<'_, '_> {"
          children:
          - id: tests.impl_TableSink_for_HostSink_______.write
            signature: "fn write(&self, table: &SegmentTable) -> Result<Region, ManagerError> {"
        - id: tests.alloc
          signature: "fn alloc() -> ArenaAllocator {"
        - id: tests.seg
          signature: "fn seg(offset: u64) -> Region {"
        - id: tests.publish_swaps_root_and_advances_epoch
          signature: "fn publish_swaps_root_and_advances_epoch() {"
        - id: tests.concurrent_builds_on_the_same_base_are_rejected
          signature: "fn concurrent_builds_on_the_same_base_are_rejected() {"
        - id: tests.old_table_is_freed_only_after_all_readers_reach_the_new_epoch
          signature: "fn old_table_is_freed_only_after_all_readers_reach_the_new_epoch() {"
        - id: tests.reclaim_respects_the_requested_bound
          signature: "fn reclaim_respects_the_requested_bound() {"
        - id: tests.launch_snapshot_is_constant_until_the_next_launch
          signature: "fn launch_snapshot_is_constant_until_the_next_launch() {"
        - id: tests.copy_is_independent_of_the_published_table
          signature: "fn copy_is_independent_of_the_published_table() {"
      - id: loom_tests
        signature: "mod loom_tests {"
        children:
        - id: loom_tests.aCv6R1|7NCwfi
          signature: use super::*;
        - id: loom_tests.content_is_visible_once_the_root_is_observed
          signature: "fn content_is_visible_once_the_root_is_observed() {"
        - id: loom_tests.retired_version_is_not_reclaimed_while_a_reader_holds_it
          signature: "fn retired_version_is_not_reclaimed_while_a_reader_holds_it() {"
      - id: device_tests
        signature: "mod device_tests {"
        children:
        - id: device_tests.XiB8kc|gxjU93
          signature: use super::*;
        - id: device_tests.obwXJ0|6OEnCW
          signature: "const PROBE: &str = r#\""
        - id: device_tests.device_sees_content_written_before_the_host_release_store
          signature: "fn device_sees_content_written_before_the_host_release_store() {"
        - id: device_tests.xreCHs|1RgSqu
          signature: "const PROBE_DEVICE: &str = r#\""
        - id: device_tests.copy_stream_event_orders_device_table_before_the_root
          signature: "fn copy_stream_event_orders_device_table_before_the_root() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/slot.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Communication slot between kernel batches. Phases run in a fixed order: ker…"
      - id: tduEXu|95xQKg
        signature: use std::collections::HashMap;
      - id: Hit
        signature: "pub struct Hit {"
      - id: MappedSegment
        signature: "struct MappedSegment {"
      - id: Located
        signature: "struct Located {"
      - id: AddressMap
        signature: "pub struct AddressMap {"
        docstring: "Arena address -> entry. The segment set is copy-on-write: a clone is the stable…"
      - id: impl_AddressMap
        signature: "impl AddressMap {"
        children:
        - id: impl_AddressMap.new
          signature: "pub fn new() -> Self {"
        - id: impl_AddressMap.epoch
          signature: "pub fn epoch(&self) -> u64 {"
        - id: impl_AddressMap.successor
          signature: "pub fn successor(&self) -> Self {"
          docstring: Copy for the next table epoch.
        - id: impl_AddressMap.add_segment
          signature: "pub fn add_segment(&mut self, segment: u32, map: SlotMap) -> bool {"
          docstring: "Returns false when the segment is empty, its id is taken or its range overlaps …"
        - id: impl_AddressMap.remove_segment
          signature: "pub fn remove_segment(&mut self, segment: u32) -> bool {"
        - id: impl_AddressMap.resolve
          signature: "pub fn resolve(&self, off: u64) -> Option<EntryId> {"
        - id: impl_AddressMap.locate
          signature: "fn locate(&self, off: u64) -> Option<Located> {"
        - id: impl_AddressMap.segment
          signature: "fn segment(&self, id: u32) -> Option<&MappedSegment> {"
        - id: impl_AddressMap.bind
          signature: "fn bind(&self, slot: SlotRef, entry: EntryId) -> bool {"
        - id: impl_AddressMap.unbind_if
          signature: "fn unbind_if(&self, slot: SlotRef, entry: EntryId) {"
      - id: MappingStore
        signature: "pub struct MappingStore {"
        docstring: "Versioned mapping, advanced together with the segment table epoch of the swap m…"
      - id: impl_MappingStore
        signature: "impl MappingStore {"
        children:
        - id: impl_MappingStore.new
          signature: "pub fn new() -> Self {"
        - id: impl_MappingStore.snapshot
          signature: "pub fn snapshot(&self) -> AddressMap {"
        - id: impl_MappingStore.snapshot_at
          signature: "pub(crate) fn snapshot_at(&self, at: &Snapshot) -> Result<AddressMap, ManagerEr…"
          docstring: The mapping a batch started on `at` must resolve against.
        - id: impl_MappingStore.publish
          signature: "pub(crate) fn publish(&self, next: AddressMap) -> Result<(), ManagerError> {"
      - id: Update
        signature: "pub enum Update {"
      - id: impl_Update
        signature: "impl Update {"
        children:
        - id: impl_Update.entry
          signature: "fn entry(&self) -> EntryId {"
      - id: UpdateQueue
        signature: "pub struct UpdateQueue {"
        docstring: "Pending mutations, latest per entry; writes that could not be attributed wait f…"
      - id: impl_UpdateQueue
        signature: "impl UpdateQueue {"
        children:
        - id: impl_UpdateQueue.new
          signature: "pub fn new() -> Self {"
        - id: impl_UpdateQueue.push
          signature: "pub fn push(&mut self, update: Update) {"
        - id: impl_UpdateQueue.contains
          signature: "pub fn contains(&self, entry: EntryId) -> bool {"
        - id: impl_UpdateQueue.depth
          signature: "pub fn depth(&self) -> usize {"
        - id: impl_UpdateQueue.deferred_writes
          signature: "pub fn deferred_writes(&self) -> usize {"
        - id: impl_UpdateQueue.apply
          signature: "fn apply(&mut self, map: &AddressMap, tracker: &mut ResidencyTracker) -> usize {"
      - id: RangeSource
        signature: "pub trait RangeSource {"
        docstring: Reads back the bytes a kernel wrote into a device range.
        children:
        - id: RangeSource.read
          signature: "fn read(&self, range: WrittenRange) -> Vec<u8>;"
      - id: run_slot_at
        signature: "pub(crate) fn run_slot_at(     store: &MappingStore,     at: &Snapshot,     wri…"
        docstring: Runs one slot against the mapping of the batch epoch `at`. A mapping that moved…
      - id: run_slot
        signature: "pub fn run_slot(     writes: &[WrittenRange],     hits: &[KernelHit],     sourc…"
      - id: apply_writes
        signature: "fn apply_writes(     writes: &[WrittenRange],     source: &dyn RangeSource,    …"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/sync.rs
      nodes:
      - id: ModuleDoc
        signature: //! State-based reconciliation between the index and a resident mirror. The cha…
      - id: zmL2Dd|imV305
        signature: use std::collections::HashMap;
      - id: 5kSs3n|PSpC6v
        signature: /// Bound on stamp-recheck repetitions per load. Exhaustion falls back to the s…
      - id: ScanOutcome
        signature: "pub struct ScanOutcome {"
      - id: impl_ScanOutcome
        signature: "impl ScanOutcome {"
        children:
        - id: impl_ScanOutcome.is_clean
          signature: "pub fn is_clean(&self) -> bool {"
        - id: impl_ScanOutcome.enqueue_removals
          signature: "pub fn enqueue_removals(&self, queue: &mut UpdateQueue) {"
          docstring: Removals are mutations of the mapping and therefore go through the update queue.
      - id: SyncScanner
        signature: "pub struct SyncScanner {"
        docstring: "One scanner per subscriber (device manager, persistence timer, host mirror); ea…"
      - id: impl_SyncScanner
        signature: "impl SyncScanner {"
        children:
        - id: impl_SyncScanner.new
          signature: "pub fn new(index: &dyn Index) -> Self {"
        - id: impl_SyncScanner.changed
          signature: "pub async fn changed(&mut self) -> bool {"
          docstring: Resolves once an unseen signal exists; `false` when the index is gone.
        - id: impl_SyncScanner.has_pending
          signature: "pub fn has_pending(&self) -> bool {"
          docstring: A signal arrived since the last scan acknowledged the channel.
        - id: impl_SyncScanner.scan
          signature: "pub fn scan(     &mut self,     index: &dyn Index,     tracker: &ResidencyTrack…"
      - id: Loaded
        signature: "pub struct Loaded<'a> {"
      - id: load_entry
        signature: "pub fn load_entry(index: &dyn Index, entry: EntryId) -> Option<Loaded<'_>> {"
        docstring: "Reads stamp, then data, then the stamp again. Registering a stamp read after th…"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/residency.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Load strategy: how a resolved entry set reaches the device (whole segments,…"
      - id: sbI38B|uLAjNU
        signature: use std::collections::BTreeMap;
      - id: SegmentId
        signature: pub type SegmentId = u64;
      - id: 7MWN2b|eoQwYL
        signature: "const MIB: u64 = 1 << 20;"
      - id: LoadGeometry
        signature: "pub trait LoadGeometry: Send + Sync {"
        docstring: Where segment boundaries and entry sizes come from; implemented by the segment …
        children:
        - id: LoadGeometry.segment_of
          signature: "fn segment_of(&self, entry: EntryId) -> SegmentId;"
        - id: LoadGeometry.entry_bytes
          signature: "fn entry_bytes(&self, entry: EntryId) -> u64;"
        - id: LoadGeometry.segment_bytes
          signature: "fn segment_bytes(&self, segment: SegmentId) -> u64;"
      - id: UniformGeometry
        signature: "pub struct UniformGeometry {"
        docstring: Equal-sized entries in consecutive segments of equal size.
      - id: impl_LoadGeometry_for_UniformGeometry
        signature: "impl LoadGeometry for UniformGeometry {"
        children:
        - id: impl_LoadGeometry_for_UniformGeometry.segment_of
          signature: "fn segment_of(&self, entry: EntryId) -> SegmentId {"
        - id: impl_LoadGeometry_for_UniformGeometry.entry_bytes
          signature: "fn entry_bytes(&self, _: EntryId) -> u64 {"
        - id: impl_LoadGeometry_for_UniformGeometry.segment_bytes
          signature: "fn segment_bytes(&self, _: SegmentId) -> u64 {"
      - id: Access
        signature: "pub enum Access {"
      - id: LoadPlan
        signature: "pub enum LoadPlan {"
      - id: impl_LoadPlan
        signature: "impl LoadPlan {"
        children:
        - id: impl_LoadPlan.kind
          signature: "fn kind(&self) -> &'static str {"
      - id: LoadPolicy
        signature: "pub struct LoadPolicy {"
      - id: impl_Default_for_LoadPolicy
        signature: "impl Default for LoadPolicy {"
        children:
        - id: impl_Default_for_LoadPolicy.default
          signature: "fn default() -> Self {"
      - id: impl_LoadPolicy
        signature: "impl LoadPolicy {"
        children:
        - id: impl_LoadPolicy.with_density_threshold
          signature: "pub fn with_density_threshold(mut self, threshold: f32) -> Self {"
        - id: impl_LoadPolicy.sized_for
          signature: "pub fn sized_for(mut self, budget: &ArenaBudget) -> Self {"
          docstring: "Segment size from the free device budget: one eighth, within 4..=256 MiB, whole…"
        - id: impl_LoadPolicy.apply_profile
          signature: "pub fn apply_profile(self, config: &ProfileConfig) -> Self {"
          docstring: Takes the threshold from the calibrated profile when it carries the `density` a…
        - id: impl_LoadPolicy.density_value
          signature: "pub fn density_value(&self) -> ValueId {"
          docstring: "Value of the `density` calibration axis, in permille."
      - id: plan_load
        signature: "pub fn plan_load(     set: &EntrySet,     geometry: &dyn LoadGeometry,     poli…"
        docstring: Chooses the transfer shape. Segments whose requested share reaches the threshol…
      - id: LoadTransfer
        signature: "pub trait LoadTransfer: Send + Sync {"
        docstring: Device-side execution of a plan; each method returns the bytes moved.
        children:
        - id: LoadTransfer.load_segments
          signature: "fn load_segments(&self, segments: &[SegmentId]) -> Result<u64, ManagerError>;"
        - id: LoadTransfer.load_gather
          signature: "fn load_gather(&self, set: &EntrySet) -> Result<u64, ManagerError>;"
        - id: LoadTransfer.map_zero_copy
          signature: "fn map_zero_copy(&self, set: &EntrySet) -> Result<u64, ManagerError>;"
      - id: LoadOutcome
        signature: "pub struct LoadOutcome {"
      - id: execute
        signature: "fn execute(     transfer: &dyn LoadTransfer,     plans: &[LoadPlan], ) -> Resul…"
      - id: Aggregated
        signature: "pub struct Aggregated<K> {"
      - id: LoadAggregator
        signature: "pub struct LoadAggregator<K> {"
        docstring: Collects the scopes of tasks against the same index so each index loads once ov…
      - id: impl_Default_for_LoadAggregator_K_
        signature: "impl<K> Default for LoadAggregator<K> {"
        children:
        - id: impl_Default_for_LoadAggregator_K_.default
          signature: "fn default() -> Self {"
      - id: impl_LoadAggregator_K_
        signature: "impl<K: Ord> LoadAggregator<K> {"
        children:
        - id: impl_LoadAggregator_K_.new
          signature: "pub fn new() -> Self {"
        - id: impl_LoadAggregator_K_.add
          signature: "pub fn add(&mut self, index: K, set: &EntrySet) {"
        - id: impl_LoadAggregator_K_.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_LoadAggregator_K_.drain
          signature: "pub fn drain(&mut self) -> Vec<Aggregated<K>> {"
      - id: ResidencyLoader
        signature: "pub struct ResidencyLoader<K> {"
      - id: impl_ResidencyLoader_K_
        signature: "impl<K> ResidencyLoader<K> where     K: Clone + Eq + Hash + Ord + Send + Sync +…"
        children:
        - id: impl_ResidencyLoader_K_.new
          signature: "pub fn new(     policy: LoadPolicy,     geometry: Arc<dyn LoadGeometry>,     tr…"
        - id: impl_ResidencyLoader_K_.policy
          signature: "pub fn policy(&self) -> &LoadPolicy {"
        - id: impl_ResidencyLoader_K_.inflight
          signature: "pub fn inflight(&self) -> usize {"
        - id: impl_ResidencyLoader_K_.ensure
          signature: "pub async fn ensure(     &self,     index: K,     set: EntrySet,     access: Ac…"
          docstring: Identical requests in flight share one transfer; an error reaches every waiter.
        - id: impl_ResidencyLoader_K_.flush
          signature: "pub async fn flush(     &self,     aggregator: &mut LoadAggregator<K>,     acce…"
          docstring: Drains the aggregator and loads every index once over the union of its requests.
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/build.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index build path: the kernel appends segments to a range granted up front a…"
      - id: tduEXu|aT45gV
        signature: use std::collections::VecDeque;
      - id: BuildRing
        signature: "pub enum BuildRing {"
      - id: WriteCursor
        signature: "pub struct WriteCursor {"
        docstring: Host model of the device cursor. A claim always advances the cursor; once one c…
      - id: impl_WriteCursor
        signature: "impl WriteCursor {"
        children:
        - id: impl_WriteCursor.new
          signature: "pub fn new(base: u64, cap: u64) -> Self {"
        - id: impl_WriteCursor.claim
          signature: "pub fn claim(&self, size: u64) -> Option<Range<u64>> {"
          docstring: "Absolute arena range, or `None` after setting the overflow state; nothing is wr…"
        - id: impl_WriteCursor.claim_group
          signature: "pub fn claim_group(&self, sizes: &[u64]) -> Vec<Option<Range<u64>>> {"
          docstring: "One cursor update for a whole group of claims (warp or block), in group order."
        - id: impl_WriteCursor.settle
          signature: "fn settle(&self, off: u64, size: u64) -> Option<Range<u64>> {"
        - id: impl_WriteCursor.overflowed
          signature: "pub fn overflowed(&self) -> bool {"
        - id: impl_WriteCursor.valid_len
          signature: "pub fn valid_len(&self) -> u64 {"
          docstring: Bytes holding successful claims.
      - id: exclusive_prefix_sum
        signature: "pub fn exclusive_prefix_sum(base: u64, lens: &[u64]) -> Vec<u64> {"
        docstring: "Exclusive prefix sum shifted to `base`: `n + 1` ascending offsets."
      - id: OffsetTable
        signature: "pub struct OffsetTable {"
      - id: impl_OffsetTable
        signature: "impl OffsetTable {"
        children:
        - id: impl_OffsetTable.new
          signature: "pub fn new(base: u64, items: &[(EntryId, u64)]) -> Self {"
        - id: impl_OffsetTable.offsets
          signature: "pub fn offsets(&self) -> &[u64] {"
        - id: impl_OffsetTable.len
          signature: "pub fn len(&self) -> usize {"
        - id: impl_OffsetTable.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_OffsetTable.addr_to_entry
          signature: "pub fn addr_to_entry(&self, off: u64) -> Option<EntryId> {"
      - id: BuiltLayout
        signature: "pub enum BuiltLayout {"
        docstring: Addressing of a freshly built segment.
      - id: impl_BuiltLayout
        signature: "impl BuiltLayout {"
        children:
        - id: impl_BuiltLayout.plan
          signature: "pub fn plan(base: u64, items: &[(EntryId, u64)], max_padding_permille: u32) -> …"
          docstring: Fixed stride when the padding to the largest entry stays within `max_padding_pe…
        - id: impl_BuiltLayout.addr_to_entry
          signature: "pub fn addr_to_entry(&self, off: u64) -> Option<EntryId> {"
        - id: impl_BuiltLayout.into_slot_map
          signature: "pub fn into_slot_map(self) -> SlotMap {"
          docstring: "Slot map with every slot bound to its entry, ready for `AddressMap::add_segment…"
      - id: RingFeed
        signature: "pub trait RingFeed {"
        docstring: Consumer side of the two build rings.
        children:
        - id: RingFeed.drain_writes
          signature: fn drain_writes(&self) -> Vec<WrittenRange>;
        - id: RingFeed.drain_hits
          signature: fn drain_hits(&self) -> Vec<KernelHit>;
      - id: SpscRing
        signature: "pub struct SpscRing<T> {"
        docstring: "Single-producer, single-consumer by contract."
      - id: impl_Default_for_SpscRing_T_
        signature: "impl<T> Default for SpscRing<T> {"
        children:
        - id: impl_Default_for_SpscRing_T_.default
          signature: "fn default() -> Self {"
      - id: impl_SpscRing_T_
        signature: "impl<T: Copy> SpscRing<T> {"
        children:
        - id: impl_SpscRing_T_.push
          signature: "pub fn push(&self, items: &[T]) {"
        - id: impl_SpscRing_T_.drain
          signature: "pub fn drain(&self) -> Vec<T> {"
        - id: impl_SpscRing_T_.len
          signature: "pub fn len(&self) -> usize {"
        - id: impl_SpscRing_T_.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
      - id: BuildRings
        signature: "pub struct BuildRings {"
      - id: impl_BuildRings
        signature: "impl BuildRings {"
        children:
        - id: impl_BuildRings.new
          signature: "pub fn new() -> Self {"
        - id: impl_BuildRings.push_writes
          signature: "pub fn push_writes(&self, writes: &[WrittenRange]) {"
        - id: impl_BuildRings.push_hits
          signature: "pub fn push_hits(&self, hits: &[KernelHit]) {"
        - id: impl_BuildRings.pending
          signature: "pub fn pending(&self, ring: BuildRing) -> usize {"
      - id: impl_RingFeed_for_BuildRings
        signature: "impl RingFeed for BuildRings {"
        children:
        - id: impl_RingFeed_for_BuildRings.drain_writes
          signature: "fn drain_writes(&self) -> Vec<WrittenRange> {"
        - id: impl_RingFeed_for_BuildRings.drain_hits
          signature: "fn drain_hits(&self) -> Vec<KernelHit> {"
      - id: run_build_slot
        signature: "pub fn run_build_slot(     feed: &dyn RingFeed,     source: &dyn RangeSource,  …"
        docstring: "A hit is only pushed after the write it reads, so draining writes first leaves …"
      - id: BuildOutcome
        signature: "pub struct BuildOutcome {"
      - id: drive_build
        signature: "pub(crate) fn drive_build(     kernel: &dyn Kernel,     arena: &IndexArena<'_>,…"
        docstring: "Runs an embedding batch to completion. A `Partial` answer is not an error: the …"
      - id: skip_bytes
        signature: "fn skip_bytes(input: &[RegionSpan], mut done: u64) -> Vec<RegionSpan> {"
      - id: device
        signature: "pub mod device {"
      - id: tests
        signature: mod tests;
```
/answer c74e-11 allow
Text:
 Im Outline fehlt der `StreamedIndexManager` (E1). Ich suche gezielt nach ihm und den anderen DoD-Punkten.
ReasoningToken: 81
Control Request:
```yaml
id: c74e-12
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag/src
    script:
    - echo '--- StreamedIndexManager'; grep -rn 'StreamedIndexManager' . | head
    - echo '--- error variants'; grep -rnE 'UnsupportedScope|IndexUnavailable|ArenaExhausted|StaleMapping|TransferFailed' resident/manager/*.rs resident/*.rs | grep -v '/test/' | head -30
    - "echo '--- tracing'; grep -rnE 'tracing::|info_span|debug_span|#\\[instrument|(info|debug|warn|trace)!\\(' resident/manager/*.rs | wc -l; grep -rnE 'info_span|debug_span|instrument|span!' resident/ | grep -v /test/ | head"
    - echo '--- addon'; grep -rniE 'cascade|kaskade' resident --include=*.rs -l | head
    - echo '--- loom feature'; grep -n loom ../Cargo.toml
  reason: "Existenz des StreamedIndexManager, Fehler-Taxonomie, tracing-Spans und Addon-Kaskade im Code prüfen"
```
/answer c74e-12 allow
Control Request:
```yaml
id: c74e-13
result:
  content:
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-qdbmz_t2.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  structuredContent:
    results:
    - stdout_file: /tmp/mcpc-stdout-qdbmz_t2.log
      stdout_file_file_stats:
        path: /tmp/mcpc-stdout-qdbmz_t2.log
        size_bytes: 3696
        lines: 47
        words: 271
        complexity: 0.557
        created: 2026-10-25T12:04:44.785100+00:00
        modified: 2026-10-25T12:04:44.785100+00:00
        accessed: 2026-10-25T12:04:44.785100+00:00
        line_length_max: 126
        line_length_min: 2
        line_length_avg: 77.64
        words_per_line_avg: 5.77
        checksum: 0f913720c5c1fec0966c6898f16059b89f930a9ddef52cbddd9b1a63f40b0c39
      stdout_file_ast_outline:
      - id: file
        lines: 1-47
        signature: '--- StreamedIndexManager'
```
/answer c74e-13 allow
Control Request:
```yaml
id: c74e-14
toolName: ast_read
arguments:
  items:
  - path: /tmp/mcpc-stdout-qdbmz_t2.log
    ids:
    - file
  reason: Ausgabe der Grep-Prüfung lesen
```
/answer c74e-14 allow
Control Request:
```yaml
id: c74e-15
result:
  structuredContent:
    results:
    - path: /tmp/mcpc-stdout-qdbmz_t2.log
      nodes:
      - id: file
        type: file
        code: |
          --- StreamedIndexManager
          ./resident/manager/mod.rs:1://! StreamedIndexManager, ScopeResolver, update queue, residency.
          --- error variants
          resident/manager/arena.rs:231:            return Err(ManagerError::ArenaExhausted {
          resident/manager/arena.rs:305:            return Err(ManagerError::ArenaExhausted {
          resident/manager/arena.rs:319:        MemoryError::ArenaExhausted { requested, free, .. } => {
          resident/manager/arena.rs:320:            ManagerError::ArenaExhausted {
          resident/manager/build.rs:277:                return Err(ManagerError::ArenaExhausted {
          resident/manager/evict.rs:231:    /// `ArenaExhausted` when pinned entries and summaries leave too little to free.
          resident/manager/evict.rs:251:            return Err(ManagerError::ArenaExhausted {
          resident/manager/residency.rs:273:                                ManagerError::Memory(MemoryError::TransferFailed {
          resident/manager/scope.rs:75:        let unsupported = || IndexError::UnsupportedScope {
          resident/manager/scope.rs:103:    IndexError::UnsupportedScope {
          resident/manager/scope.rs:189:            .ok_or_else(|| IndexError::IndexUnavailable {
          resident/manager/slot.rs:143:            return Err(ManagerError::StaleMapping {
          resident/manager/slot.rs:153:            return Err(ManagerError::StaleMapping {
          resident/manager/slot.rs:230:/// batch started is reported as `StaleMapping` without side effects; the caller retries the slot
          resident/manager/swap.rs:255:            return Err(ManagerError::StaleMapping {
          resident/manager/swap.rs:325:        assert!(matches!(mgr.publish(b), Err(ManagerError::StaleMapping { .. })));
          resident/manager/upload.rs:27:        self.as_mut_slice().map_err(|e| MemoryError::TransferFailed {
          resident/manager/upload.rs:111:                return Err(ManagerError::ArenaExhausted {
          resident/manager/upload.rs:214:            lane.record_event().map_err(|e| MemoryError::TransferFailed {
          resident/error.rs:18:    UnsupportedScope { method: String },
          resident/error.rs:20:    IndexUnavailable { scope: String },
          resident/error.rs:22:    ArenaExhausted { requested: usize, available: usize },
          resident/error.rs:24:    TransferFailed { reason: String },
          resident/error.rs:33:            self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. }
          resident/error.rs:47:    ArenaExhausted { requested: u64, available: u64 },
          resident/error.rs:49:    StaleMapping { base: u64, published: u64 },
          resident/error.rs:58:            ManagerError::StaleRegion { .. } | ManagerError::ArenaExhausted { .. } => {
          resident/error.rs:61:            ManagerError::StaleMapping { .. } => true,
          --- tracing
          33
          resident/processor/worker.rs:74:            let span = tracing::info_span!("worker", ? side, index);
          resident/kernel/launcher.rs:39:    #[tracing::instrument(
          resident/tokenize/mod.rs:71:    #[tracing::instrument(level = "debug", skip_all, fields(?mode, n_texts = texts.len()))]
          resident/tokenize/mod.rs:107:    #[tracing::instrument(level = "debug", skip_all, fields(?mode, n_texts = texts.len()))]
          resident/subengine/device.rs:158:            let span = tracing::info_span!(
          resident/subengine/exec.rs:293:        let span = tracing::info_span!("lane", lane = ?class, device = "cpu");
          resident/manager/sync.rs:65:        let _span = tracing::info_span!("sync_scan", watermark).entered();
          resident/manager/scope.rs:73:        let span = tracing::debug_span!("scope_resolve", ? method);
          resident/manager/slot.rs:255:    let _span = tracing::info_span!("slot_cycle", epoch = map.epoch).entered();
          resident/memory/arena.rs:54:    #[tracing::instrument(level = "trace", skip(self), fields(arena = ?self.kind))]
          --- addon
          resident/facade/test/addon.rs
          --- loom feature
          68:loom = "0.7"
```
/answer c74e-15 allow
Control Request:
```yaml
id: c74e-16
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_01.md
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_e_09.md
  regexp: ^## (3\.|7\.)
  reason: "Artefakt- und Test-Abschnitte von E1 und E9 lesen (nicht die Ergebnisberichte), um DoD gegen Code zu prüfen"
```
/answer c74e-16 deny nicht relevant, Ziel sind nur ein Sekundärer Aspekt. Fehlende Tests sind kein Problem.
ReasoningToken: 198
Control Request:
```yaml
id: c74e-17
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/advice.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/blocking.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/tracker.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/test/addon.rs
  reason: "Prüfen, ob E1-Manager-Hüllen (advice/blocking) und E9-Addon-Test existieren"
```
/answer c74e-17 allow
Control Request:
```yaml
id: c74e-18
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/advice.rs
      nodes:
      - id: ModuleDoc
        signature: //! Page-cache hints for mmap-backed index data. Hints only; the kernel may ign…
      - id: 57n2KO|N0ZncL
        signature: use std::io;
      - id: Hint
        signature: "pub enum Hint {"
      - id: AdviceSink
        signature: "pub trait AdviceSink: Send + Sync {"
        children:
        - id: AdviceSink.advise
          signature: "fn advise(&self, range: Range<usize>, hint: Hint) -> io::Result<()>;"
      - id: MmapAdvice
        signature: "pub struct MmapAdvice {"
      - id: impl_MmapAdvice
        signature: "impl MmapAdvice {"
        children:
        - id: impl_MmapAdvice.new
          signature: "pub fn new(map: Mmap) -> Self {"
      - id: impl_AdviceSink_for_MmapAdvice
        signature: "impl AdviceSink for MmapAdvice {"
        children:
        - id: impl_AdviceSink_for_MmapAdvice.advise
          signature: "fn advise(&self, range: Range<usize>, hint: Hint) -> io::Result<()> {"
      - id: advise_cold
        signature: "fn advise_cold(map: &Mmap, offset: usize, len: usize) -> io::Result<()> {"
      - id: advise_cold_2
        signature: "fn advise_cold(_map: &Mmap, _offset: usize, _len: usize) -> io::Result<()> {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/blocking.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index reads that may page-fault or hit disk run on the blocking pool, never…"
      - id: 9d1TOl|KCxEXs
        signature: use std::ops::Range;
      - id: BlockingReads
        signature: "pub struct BlockingReads {"
      - id: impl_BlockingReads
        signature: "impl BlockingReads {"
        children:
        - id: impl_BlockingReads.new
          signature: "pub fn new(handle: Handle) -> Self {"
        - id: impl_BlockingReads.with_advice
          signature: "pub fn with_advice(mut self, sink: Arc<dyn AdviceSink>) -> Self {"
        - id: impl_BlockingReads.prefetch
          signature: "pub fn prefetch(&self, range: Range<usize>) {"
          docstring: Asks the OS to read `range` ahead without waiting for it. Hints may be ignored.
        - id: impl_BlockingReads.read
          signature: "pub async fn read<T, F>(&self, range: Range<usize>, read: F) -> Result<T, JoinE…"
          docstring: "Hints `range`, then runs `read` on the blocking pool inside the caller's span."
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/tracker.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Mapping between index entries and resident slots, with the stamp known per …"
      - id: rSubyT|KeAo9U
        signature: use std::collections::HashMap;
      - id: SlotRef
        signature: "pub struct SlotRef {"
      - id: Resident
        signature: "pub struct Resident {"
      - id: ResidencyTracker
        signature: "pub struct ResidencyTracker {"
      - id: impl_ResidencyTracker
        signature: "impl ResidencyTracker {"
        children:
        - id: impl_ResidencyTracker.new
          signature: "pub fn new() -> Self {"
        - id: impl_ResidencyTracker.insert
          signature: "pub fn insert(     &mut self,     entry: EntryId,     slot: SlotRef,     stamp:…"
        - id: impl_ResidencyTracker.remove
          signature: "pub fn remove(&mut self, entry: EntryId) -> Option<Resident> {"
        - id: impl_ResidencyTracker.get
          signature: "pub fn get(&self, entry: EntryId) -> Option<Resident> {"
        - id: impl_ResidencyTracker.known_stamp
          signature: "pub fn known_stamp(&self, entry: EntryId) -> Option<Stamp> {"
        - id: impl_ResidencyTracker.len
          signature: "pub fn len(&self) -> usize {"
        - id: impl_ResidencyTracker.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_ResidencyTracker.iter
          signature: "pub fn iter(&self) -> impl Iterator<Item = (EntryId, Resident)> + '_ {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/test/addon.rs
      nodes:
      - id: ModuleDoc
        signature: //! Reference cascade built only on the public submit API and parameter extensi…
      - id: r0Oc1C|5QlE27
        signature: use super::*;
      - id: Correlation
        signature: struct Correlation(u64);
      - id: resident
        signature: "fn resident() -> Resident {"
      - id: claim_and_complete
        signature: "fn claim_and_complete(r: &Resident, out: Output) -> (TaskMeta, Payload) {"
      - id: cascade
        signature: "fn cascade(     r: &Resident,     hits: &Output,     summary: &SimilarityParams…"
        docstring: "Continuation step: derives follow-up tasks from a finished one through the norm…"
      - id: foreign_extension_travels_through_the_queue
        signature: "fn foreign_extension_travels_through_the_queue() {"
      - id: cascade_follow_up_is_a_regular_task_with_derived_scope
        signature: "fn cascade_follow_up_is_a_regular_task_with_derived_scope() {"
      - id: follow_up_equals_application_task_without_addon_marker
        signature: "fn follow_up_equals_application_task_without_addon_marker() {"
```
/answer c74e-18 allow
ReasoningToken: 220
Control Request:
```yaml
id: c74e-19
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - echo '--- manager structs/lazy'; grep -rnEi 'lazy|OnceCell|OnceLock|struct .*Manager' src/resident --include=*.rs | grep -v /test/ | grep -v 'memory/' | head -20
    - echo '--- impl LoadTransfer'; grep -rn 'impl.*LoadTransfer for' src | head
    - echo '--- roaring'; grep -n roaring Cargo.toml; grep -rn 'RoaringBitmap' src/resident --include=*.rs -l | head -3
    - echo '--- E9 note'; grep -rliE 'addon' docs ../project/resident/*.md 2>/dev/null | head; grep -n 'Kern.*Addon.*Oberschicht\|Addon.*Oberschicht' -r docs | head -3
    - echo '--- Doorbell/loom'; RUSTFLAGS='--cfg loom' cargo test --lib --release loom 2>&1 | tail -6
  reason: "E1-Manager-Existenz, E7-Transfer-Implementierungen, roaring, E9-Konzeptnotiz und loom-Tests prüfen"
```
/answer c74e-19 allow
Control Request:
```yaml
id: c74e-1a
result:
  structuredContent:
    results:
    - stdout: |+
        --- manager structs/lazy
        src/resident/processor/hub.rs:2:use std::sync::{Arc, OnceLock};
        src/resident/processor/hub.rs:109:    region_check: OnceLock<RegionCheck>,
        src/resident/processor/hub.rs:128:            region_check: OnceLock::new(),
        src/resident/precision/known.rs:3:use std::sync::LazyLock;
        src/resident/precision/known.rs:8:        $(pub static $name : LazyLock <$ty > = LazyLock::new(|| $ty ::new($s));)+
        src/resident/queue/queue.rs:1://! Generic lock-free queue: sentinel head, CAS append, lazy logical unlink.
        src/resident/queue/state.rs:3:use std::sync::OnceLock;
        src/resident/queue/state.rs:36:    result: OnceLock<Result<O, E>>,
        src/resident/queue/state.rs:48:            result: OnceLock::new(),
        src/resident/subengine/lifecycle.rs:6:use tokio::sync::{OnceCell, Semaphore};
        src/resident/subengine/lifecycle.rs:65:    slot: RwLock<Arc<OnceCell<Arc<R>>>>,
        src/resident/subengine/lifecycle.rs:79:            slot: RwLock::new(Arc::new(OnceCell::new())),
        src/resident/subengine/lifecycle.rs:201:        let old = std::mem::replace(&mut *slot, Arc::new(OnceCell::new()));
        src/resident/subengine/device.rs:55:            std::env::set_var("CUDA_MODULE_LOADING", "LAZY");
        src/resident/subengine/device.rs:89:    /// Time spent creating the context, reported to the lazy-init accounting.
        src/resident/subengine/device.rs:93:    /// Forces driver-side lazy work (first allocation, stream creation) so it does not land on the
        src/resident/manager/swap.rs:180:pub struct SwapManager<'a, 'b> {
        src/resident/failpoint.rs:9:    use std::sync::LazyLock;
        src/resident/failpoint.rs:13:    static ARMED: LazyLock<Mutex<HashMap<(&'static str, u64), u32>>> =
        src/resident/failpoint.rs:14:        LazyLock::new(|| Mutex::new(HashMap::new()));
        --- impl LoadTransfer
        src/resident/manager/test/residency.rs:140:impl LoadTransfer for Recorder {
        --- roaring
        --- E9 note
        docs/architecture/scope_charta.md
        docs/architecture/addons.md
        ../project/resident/index.md
        ../project/resident/resident_index.md
        ../project/resident/resident_planplan.md
        --- Doorbell/loom
        test resident::queue::state::loom_tests::concurrent_claims_have_one_winner ... ok
        test resident::manager::swap::loom_tests::retired_version_is_not_reclaimed_while_a_reader_holds_it ... ok
        test resident::manager::seqlock::loom_tests::reader_never_sees_a_torn_pair ... ok

        test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 393 filtered out; finished in 0.02s
```
/answer c74e-1a allow
ReasoningToken: 45
Control Request:
```yaml
id: c74e-1b
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - grep -rn 'pub struct EntrySet' -A6 src | head -12
    - grep -n -i 'roaring\|bitmap' Cargo.toml src/resident/index/*.rs | head -5
    - grep -n -i 'union' src/resident/index/*.rs | head -3
  reason: EntrySet-Implementierung auf roaring und Union prüfen (E2-DoD)
```
/answer c74e-1b allow
Control Request:
```yaml
id: c74e-1c
result:
  structuredContent:
    results:
    - stdout: |
        src/resident/index/mod.rs:59:pub struct EntrySet {
        src/resident/index/mod.rs-60-    ranges: Vec<Range<u64>>,
        src/resident/index/mod.rs-61-}
        src/resident/index/mod.rs-62-impl EntrySet {
        src/resident/index/mod.rs-63-    pub fn new() -> Self {
        src/resident/index/mod.rs-64-        Self::default()
        src/resident/index/mod.rs-65-    }
        src/resident/index/mod.rs:100:    pub fn union(&self, other: &Self) -> Self {
```
/answer c74e-1c allow
Result Stats: id=16227a29-9d5e-41d9-b45d-4b0bf9302762, total: 58863, in: 22, out: 8175, reason: 0, read: 311855, write: 50666, start: 1792929995914, end: 1792930013131, time: 17121, total: 104884, io: 0, control: 96, sent: 0, recv: 452
# DoD-Prüfung Gruppe E

**Gesamtergebnis:** E1 ist nicht erledigt, und E2 und E9 sind teilweise offen. Die Gruppen-DoD ist damit nicht erfüllt. Die Prüfung stützt sich auf Code und Testläufe. Die „Result"-Abschnitte der Pläne habe ich nicht ausgewertet.

**Läufe**
- `cargo test --lib resident::` ergibt 391 passed, 0 failed, alles ohne GPU.
- Die `loom`-Tests (7) laufen mit `RUSTFLAGS='--cfg loom'` grün. Darunter sind die Swap-Reclaim- und Seqlock-Tests.
- `cargo check --features cuda` läuft durch.
- Der Build erzeugt 82 Warnungen. Ich habe sie nicht einzeln geprüft.
- Auf der GPU habe ich nichts ausgeführt, Hardwarebefunde sind deshalb ungeprüft.

## Einzelpläne

| Plan | Status | Befund |
|---|---|---|
| **E1** | **Nicht erfüllt** | Es gibt kein `StreamedIndexManager`. Der Name steht nur im Moduldoc von `manager/mod.rs`. Eine Lazy-Konstruktion, eine CPU-Variante, die den Index liest, und eine GPU-Hülle fehlen. Vorhanden sind nur `advice.rs` und `blocking.rs` (mmap-Hints, Blocking-Pool). Auch `loader`, `run_slot` und `drive_build` hängen an keinem Manager. |
| **E2** | **Teilweise** | `ResolverRegistry` mit Doppelprüfung (`DuplicateResolver`) und Range-, Subtree-, Prefix- und Centroid-Resolver sind vorhanden. `EntrySet` ist aber `Vec<Range<u64>>`, nicht roaring-basiert. `roaring` fehlt in `Cargo.toml`. `union` existiert. |
| **E3** | Erfüllt | `IndexArena` auf dem gemeinsamen `ArenaAllocator` mit Alignment, `CowBudget`/`begin_cow`/`commit`, `retire`/`reclaim` und `WritableRegion`. Es gibt keine zweite Allokator-Implementierung. Staging und Copy-Stream stecken in `upload.rs`. |
| **E4** | Erfüllt | `SegmentTable` (CoW), `RootPointer::publish` (Release), `EpochTracker`, `BatchGuard` und `TableSink`, das vor dem Root-Swap vollständig schreibt. Die loom-Tests laufen grün. Es gibt zusätzlich ignorierte Device-Tests. |
| **E5** | Erfüllt (Code) | `run_slot` in fester Reihenfolge, `AddressMap` mit Fixed- und Variable-Auflösung, `StaleMapping` bei Epochen-Mismatch. Die offene Hardwarefrage zu Doorbell und posted writes bleibt unverifiziert. |
| **E6** | Erfüllt | `SyncScanner` hat Quittierung vor dem Scan. `load_entry` liest Stamp, Daten, Stamp. Das Modul hat Span `sync_scan`. |
| **E7** | Weitgehend | `plan_load`, `LoadAggregator`, `ResidencyLoader` mit Single-Flight, `evict.rs` und die Density-Schwelle als Kalibrierachse (`apply_profile`) sind vorhanden. Es gibt keine konkrete `LoadTransfer`-Implementierung, nur den Test-`Recorder`. Segment-, Gather- und Zero-Copy-Pfad sind damit nicht real angebunden. |
| **E8** | Erfüllt (Code) | `WriteCursor` mit Überlauf und Teilergebnis, `OffsetTable` und `BuiltLayout`, getrennte `BuildRings`, `drive_build`. Der Plan erwähnt Hardware-Verifikationen, die ich nicht nachgeprüft habe. Das CUB-`ExclusiveScan` ist nicht in den Crate integriert (CPU-Präfixsumme). |
| **E9** | Teilweise | Die Referenz-Kaskade als Addon läuft in `facade/test/addon.rs` über die normale Queue. `docs/architecture/addons.md` und `scope_charta.md` sind vorhanden. Ich habe sie nicht inhaltlich gelesen. Dass der Kern keine Kaskade enthält, habe ich nicht geprüft. |

## Gruppen-DoD (Abschnitt 7 der Übersicht)

- **Tests grün ohne GPU:** erfüllt für die vorhandenen Module. Weil E1 fehlt, ist die Aussage nur für diese Teile belegt.
- **Konsistenzmodell in Doku und Tests:** großteils erfüllt (CoW, Epoche, Stamp-Reihenfolge, `!=`). Der Punkt „Source of Truth = Index, bei Divergenz gewinnt der Index" hängt am fehlenden Manager.
- **Fehler-Taxonomie:** erfüllt. `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` (intern) und `TransferFailed` sind definiert und werden verwendet.
- **tracing-Spans pro Task und Manager-Zyklus:** teilweise. Es gibt Spans in `sync_scan`, `slot_cycle` und `scope_resolve`. Einen Span für den Manager-Zyklus kann es ohne Manager nicht geben. Das Logging von Lade- und Eviction-Entscheidungen sowie der Arena-Auslastung habe ich nicht im Detail geprüft. `IndexArena` hat `trace_utilization`.
- **Offene Verifikationen mit Status dokumentiert:** nicht geprüft.

## Empfohlene Nacharbeit
1. E1 umsetzen: `StreamedIndexManager` mit Lazy-Init, CPU-Variante und `cuda`-Hülle. Er sollte `ScopeResolver`, `SyncScanner`, `ResidencyLoader`, `SwapManager` und `run_slot` verdrahten und den Manager-Zyklus-Span setzen.
2. Bei E2 entscheiden, ob `roaring` eingeführt oder die DoD auf Range-basiert angepasst wird.
3. Bei E7 eine konkrete `LoadTransfer`-Implementierung hinter dem `cuda`-Feature nachziehen.
4. Bei E9 die Konzeptnotiz gegen die DoD lesen.
