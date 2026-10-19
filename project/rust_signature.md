`/home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine` liefert in einigen Fällen dasselbe für signature und docstring. Idealerweise sollte dort die Funktion stehen und wenn dies nicht möglich is sollte kein docstring zurückgegeben werden wenn dieser bereits die Signatur ist.

Beispiel, `/home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs`:
```yaml
        - id: ExecContext.record_event
          signature: /// Completes once everything submitted so far has finished.
          docstring: Completes once everything submitted so far has finished.
        - id: ExecContext.wait
          signature: /// Orders all later work on this lane after `event`.
          docstring: Orders all later work on this lane after `event`.
        - id: ExecContext.synchronize
          signature: /// Blocks until the lane has drained; returns the first job failure since the …
          docstring: Blocks until the lane has drained; returns the first job failure since the last…
        - id: ExecContext.param_block
          signature: fn param_block(&mut self) -> &mut ParamBlock;
        - id: ExecContext.scratch
          signature: fn scratch(&self) -> &ScratchArena;
```