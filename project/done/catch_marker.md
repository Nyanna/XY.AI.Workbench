Erhöhe die Robustheit von `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py`. Wie im folgenden Beispiel ist die Verwendung eigentlich falsch, jedoch kann der Intent relativ sicher antizipiert werden. Wenn Start und Ende identisch ist, ist die Absicht eine Zeile mit einem Block zu tauschen.

Beispiel:
```yaml
id: e9ae-49
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    start_marker: //! Error taxonomy.
    end_marker: //! Error taxonomy.
    content: |
      //! Error taxonomy.
      //! ...
```
SystemInit:  id=108bdf12-787e-42d6-ae07-791764f237e1, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: 844b-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  reason: Inspect file structure to locate marker-matching logic
```
/answer 844b-1 allow
Control Request:
```yaml
id: 844b-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
      nodes:
      - id: zKe9Du|5qARzC
        signature: "'``ast_edit_marks`` tool: mark-based edits within the source of selected nodes.'"
      - id: 0NkjOX|Zg7YTi
        signature: from dataclasses import dataclass
      - id: qzaxoR|Ancd56
        signature: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: IklaUl|hqD6GH
        signature: "__all__ = ["
      - id: EditMarksItem
        signature: "@dataclass(frozen=True) class EditMarksItem:"
        docstring: "One marker-based edit to apply. Attributes: path: Absolute path to the file to …"
      - id: EditMarksResult
        signature: "@dataclass(frozen=True) class EditMarksResult:"
        docstring: "Result of applying a single marker-based edit. Attributes: path: The path exact…"
      - id: EditMarksError
        signature: "@dataclass(frozen=True) class EditMarksError:"
        docstring: "Error applying a single marker-based edit. Attributes: path: The path exactly a…"
      - id: EditMarksBatchResult
        signature: "@dataclass(frozen=True) class EditMarksBatchResult:"
        docstring: "Result of :func:`ast_edit_marks`. Attributes: results: One :class:`EditMarksRes…"
      - id: node_marks_guard
        signature: "def _node_marks_guard(engine, begin_marker, end_marker):"
        docstring: Guard for tolerant marker edits (see ``ast.edit_block._node_guard``).
      - id: edit_marks_one
        signature: "def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:"
      - id: ast_edit_marks
        signature: "def ast_edit_marks(items: list[EditMarksItem]) -> EditMarksBatchResult:"
        docstring: Replace everything between the start/end markers inside one or more nodes. Each…
      - id: EditMarksNodeTool
        signature: "class EditMarksNodeTool(ToolDefinition):"
        children:
        - id: EditMarksNodeTool.nfiMrL|YQq2sh
          signature: name = 'ast_edit_marks'
        - id: EditMarksNodeTool.MuT0pV|RNoeCK
          signature: "_ITEM_PROPERTIES = {"
        - id: EditMarksNodeTool.auCabR|Xg4Px7
          signature: "_ITEM_REQUIRED = ['path', 'start_marker', 'end_marker', 'content']"
        - id: EditMarksNodeTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST …"
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 844b-2 allow
Control Request:
```yaml
id: 844b-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    ids:
    - node_marks_guard
    - edit_marks_one
  reason: Read matching/replacement logic to add robustness for identical start/end marker
```
/answer 844b-3 allow
Control Request:
```yaml
id: 844b-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
      nodes:
      - id: node_marks_guard
        type: FunctionDef
        code: |-
          def _node_marks_guard(engine, begin_marker, end_marker):
              """Guard for tolerant marker edits (see ``ast.edit_block._node_guard``)."""
              if engine.validates_syntax:
                  return None
              keep_lines = marks_line_preserving(begin_marker, end_marker)
              return lambda begin, end, result: keep_lines(begin, end, result) and engine.validate(result) is None
      - id: edit_marks_one
        type: FunctionDef
        code: |-
          def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:
              file_path = core.require_path(item.path)
              tree = core.CACHE.get_tree(file_path)
              target = select_by_text(tree, [item.start_marker, item.end_marker], id=item.id)
              node_source = core.edit_node_source(target)
              begin, end = (item.start_marker, item.end_marker) if item.exact else (
                  item.start_marker.strip(), item.end_marker.strip())
              try:
                  new_source = replace_between(
                      node_source,
                      begin,
                      end,
                      item.content,
                      exact=item.exact,
                      accept=_node_marks_guard(
                          tree.engine,
                          begin,
                          end),
                      max_level=3 if tree.engine.validates_syntax else 2,
                      where='node')
              except TextMatchError as exc:
                  raise core.AstTextError(
                      str(exc), reason=getattr(
                          exc, 'reason', None), position=getattr(
                              exc, 'position', None), corrected_text=getattr(
                                  exc, 'corrected_text', None), guess=getattr(
                                      exc, 'guess', None), next_step=getattr(
                                          exc, 'next_step', None)) from exc
              new_id = core.replace_node(target, new_source)
              core.CACHE.save(file_path, tree)
              return EditMarksResult(path=item.path, id=item.id, result='success', new_id=new_id)
```
/answer 844b-4 allow
Control Request:
```yaml
id: 844b-5
toolName: grep
arguments:
  items:
  - pattern: def replace_between
    directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    include:
    - '*.py'
  reason: Locate replace_between implementation to understand start==end handling
```
/answer 844b-5 allow
Control Request:
```yaml
id: 844b-6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/tools/_text_match.py
        matches:
        - lineno: 239
          match: "def replace_between(block: str, begin_marker: str, end_marker: str, content: str, *, exact: bool, accept: MarksGuard | None=None, max_level: int=3, where: str='block') -> str:"
```
/answer 844b-6 allow
Control Request:
```yaml
id: 844b-7
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_text_match.py
  name: replace_between
  reason: Read replace_between implementation
```
/answer 844b-7 allow
Control Request:
```yaml
id: 844b-8
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_text_match.py
      nodes:
      - id: replace_between
        code: |-
          def replace_between(block: str, begin_marker: str, end_marker: str, content: str, *, exact: bool, accept: MarksGuard | None=None, max_level: int=3, where: str='block') -> str:
              """Replace the span between (and including) both markers with ``content``.

              Both markers are matched at the same escalating tolerance level; the first
              level whose unique markers yield an ``accept``-approved result wins. Raises
              :class:`TextNotFound`/:class:`TextAmbiguous`/:class:`TextMatchError`.
              """
              start_found = False
              guard_rejected = False
              for level in _levels(exact, max_level):
                  starts = _matches(block, begin_marker, level)
                  if not starts:
                      continue
                  if len(starts) > 1:
                      raise TextAmbiguous(f'Start marker is ambiguous – found {len(starts)} occurrences in {where}.', len(starts))
                  start_found = True
                  ends = _matches(block, end_marker, level)
                  if not ends:
                      continue
                  if len(ends) > 1:
                      raise TextAmbiguous(f'End marker is ambiguous – found {len(ends)} occurrences in {where}.', len(ends))
                  sm, em = (starts[0], ends[0])
                  if em.start < sm.end:
                      raise TextOrderError('End marker must start after start marker ends.')
                  result = block[:sm.start] + _mirror_escaping(block[sm.start:em.end], content) + block[em.end:]
                  if accept and (not accept(block[sm.start:sm.end], block[em.start:em.end], result)):
                      guard_rejected = True
                      continue
                  return result
              if not start_found:
                  raise _diagnose_not_found(
                      block,
                      begin_marker,
                      guard_rejected=False,
                      where=where,
                      label='Start marker',
                      position='start')
              raise _diagnose_not_found(
                  block,
                  end_marker,
                  guard_rejected=guard_rejected,
                  where=where,
                  label='End marker',
                  position='end')
```
/answer 844b-8 allow
ReasoningToken: 9
Control Request:
```yaml
id: 844b-9
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_text_match.py
    old_text: |2-
              sm, em = (starts[0], ends[0])
              if em.start < sm.end:
                  raise TextOrderError('End marker must start after start marker ends.')
    new_text: |2-
              sm, em = (starts[0], ends[0])
              same_span = sm.start == em.start and sm.end == em.end
              if not same_span and em.start < sm.end:
                  raise TextOrderError('End marker must start after start marker ends.')
  reason: Allow identical start/end marker spans (single-line swap intent) to be accepted instead of raising a spurious order error
```
/answer 844b-9 allow
Control Request:
```yaml
id: 844b-a
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_text_match.py
  reason: Verify the file still compiles after the fix
```
/answer 844b-a allow
Result Stats: id=108bdf12-787e-42d6-ae07-791764f237e1, total: 14318, in: 145, out: 2889, reason: 1559, read: 48253, write: 11284
ReasoningToken: 24
ReasoningToken: 1535
Text:
 Jetzt validieren:
Fix angewendet: In `replace_between` (`_text_match.py`) wurde die `TextOrderError`-Prüfung so angepasst, dass sie bei identischem Start-/End-Match-Span (gleicher `start`/`end`-Index) nicht mehr greift. Dadurch wird der im Beispiel gezeigte Intent – eine einzelne Zeile als Marker für den zu ersetzenden Block zu verwenden (`start_marker == end_marker`) – korrekt unterstützt, statt fälschlich einen Reihenfolgefehler zu werfen.
