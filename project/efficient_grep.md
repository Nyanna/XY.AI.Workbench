Control Request:
```yaml
id: cc92-62
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: ArenaAllocator|IndexArena|struct Region|MemoryManager|ManagerError
    include:
    - '*.rs'
    limit: 40
  reason: Vorhandene Allocator-/Region-Typen und Fehler-Typen lokalisieren
```
/answer cc92-62 allow
Control Request:
```yaml
id: cc92-63
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/kernel/mod.rs
        matches:
        - match: "pub struct RegionSpan {"
      - path: resident/error.rs
        matches:
        - match: "pub(crate) enum ManagerError {"
        - match: "impl IsRetryable for ManagerError {"
        - match: "matches!(self, ManagerError::StaleRegion { .. })"
        - match: assert_send_sync_static::<ManagerError>();
        - match: "assert!(ManagerError::StaleRegion { arena : 1, gen : 2 } .is_retryable());"