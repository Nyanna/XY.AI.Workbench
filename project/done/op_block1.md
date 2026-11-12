Ich glaube, der Folgende Fehler hätte sicher antizipiert werden können, weil der "old_text" bestimmt nur an einer Stelle vorkam.
Die Node-ID wäre so sicher feststellbar gewesen und der Fehler unnötig. Prüfe die Implementierung auf sicheres Antizipieren des Änderungsersuches.

- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py`

```yaml
id: 3a25-c9-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/a/test/tokenizer.rs
    id: ""
    old_text: l020::a::model_descriptor::DType;
    new_text: l020::l010::layout::DType;
    
id: 3a25-ca-c9
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/a/test/tokenizer.rs
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: ""
```
SystemInit:  id=86622281-950f-4cac-8b28-8f12df3ec6b6, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Ich schaue mir zuerst die Implementierung an.
Control Request:
```yaml
id: 3a25-d0-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  reason: "Struktur von edit_block.py erfassen, um die Node-Auflösung bei leerer ID zu prüfen"
```
/answer 3a25-d0-0 allow
Control Request:
```yaml
id: 3a25-d1-d0
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: mQ7CrW|aoOqEy
        excert: "'``ast_edit_block`` tool: exact-block (old_text -> new_text) edits within selec…"
      - id: G5Js58|8gAslm
        excert: from dataclasses import dataclass
      - id: qzaxoR|dzOuQb
        excert: "from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, Tex…"
      - id: B1sD7y|298I9E
        excert: "__all__ = ["
      - id: EditBlockItem
        excert: "@dataclass(frozen=True) class EditBlockItem:"
        docstring: "One block edit to apply. Attributes: path: Absolute path to the file to modify.…"
      - id: EditBlockResult
        excert: "@dataclass(frozen=True) class EditBlockResult:"
        docstring: "Result of applying a single block edit. Attributes: path: The path exactly as g…"
      - id: EditBlockError
        excert: "@dataclass(frozen=True) class EditBlockError:"
        docstring: "Error applying a single block edit. Attributes: path: The path exactly as given…"
      - id: EditBlockBatchResult
        excert: "@dataclass(frozen=True) class EditBlockBatchResult:"
        docstring: "Result of :func:`ast_edit_block`. Attributes: results: One :class:`EditBlockRes…"
      - id: node_guard
        excert: "def _node_guard(engine, reference):"
        docstring: Guard for tolerant node edits. Engines that flag malformed edits on replace nee…
      - id: edit_block_one
        excert: "def _edit_block_one(item: EditBlockItem) -> EditBlockResult:"
      - id: ast_edit_block
        excert: "def ast_edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:"
        docstring: Replace occurrence(s) of ``old_text`` with ``new_text`` inside one or more node…
      - id: EditBlockNodeTool
        excert: "class EditBlockNodeTool(ToolDefinition):"
        children:
        - id: EditBlockNodeTool.aKEDSe|mSaRry
          excert: name = 'ast_edit_block'
        - id: EditBlockNodeTool.Z2OjHo|ftL6Rn
          excert: "_ITEM_PROPERTIES = {"
        - id: EditBlockNodeTool.ZDSH0j|Jr7UgN
          excert: "_ITEM_REQUIRED = ['path', 'old_text', 'new_text', 'id']"
        - id: EditBlockNodeTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST …"
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 3a25-d1-d0 allow
Control Request:
```yaml
id: 3a25-d2-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    ids:
    - node_guard
    - edit_block_one
    - ast_edit_block
    - EditBlockNodeTool.handle
  reason: "Kernlogik lesen: wie wird bei leerer id verfahren"
```
/answer 3a25-d2-0 allow
Control Request:
```yaml
id: 3a25-d3-d2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: node_guard
        type: FunctionDef
        code: |-
          def _node_guard(engine, reference):
              """Guard for tolerant node edits.

              Engines that flag malformed edits on replace need no extra check. Others
              (e.g. markup grammars) must not merge lines and must still re-parse cleanly.
              """
              if engine.validates_syntax:
                  return None
              keep_lines = line_preserving(reference)
              return lambda span, result: keep_lines(span, result) and engine.validate(result) is None
      - id: edit_block_one
        type: FunctionDef
        code: |-
          def _edit_block_one(item: EditBlockItem) -> EditBlockResult:
              file_path = core.require_path(item.path)
              tree = core.CACHE.get_tree(file_path)
              target = select_by_text(tree, [item.old_text], id=item.id)
              node_source = core.edit_node_source(target)
              try:
                  new_source = replace_in_block(
                      node_source,
                      item.old_text,
                      item.new_text,
                      exact=item.exact,
                      replace_all=item.replace_all,
                      accept=_node_guard(
                          tree.engine,
                          item.old_text),
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
              imports = core.apply_imports(tree, item.add_import, item.remove_import)
              core.CACHE.save(file_path, tree)
              return EditBlockResult(path=item.path, id=item.id, result='success', new_id=new_id, imports=imports)
      - id: ast_edit_block
        type: FunctionDef
        code: |-
          def ast_edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:
              """Replace occurrence(s) of ``old_text`` with ``new_text`` inside one or more nodes.

              Each addressed node's source is unparsed, its ``old_text`` block replaced
              (as with ``edit_block``), re-parsed, and used to replace the node.

              Args:
                  items: Block edits to apply. Must be non-empty.

              Returns:
                  EditBlockBatchResult: One result per successful edit, one error per failed edit.

              Raises:
                  core.AstError: If ``items`` is empty.
              """
              if not items:
                  raise core.AstError("'items' must be a non-empty list.")
              results: list[EditBlockResult] = []
              errors: list[EditBlockError] = []
              for item in items:
                  try:
                      results.append(_edit_block_one(item))
                  except core.AstAmbiguous as exc:
                      errors.append(EditBlockError(path=item.path, id=item.id, error=str(exc), candidates=exc.candidates))
                  except core.AstError as exc:
                      errors.append(
                          EditBlockError(
                              path=item.path, id=item.id, error=str(exc), reason=getattr(
                                  exc, 'reason', None), corrected_text=getattr(
                                      exc, 'corrected_text', None), guess=getattr(
                                          exc, 'guess', None), next_step=getattr(
                                              exc, 'next_step', None)))
              return EditBlockBatchResult(results=results, errors=errors, notice=import_notice(results))
      - id: EditBlockNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST API."""

              def item_factory(it: dict[str, Any]) -> EditBlockItem:
                  add_import, remove_import = parse_import_args(it)
                  return EditBlockItem(
                      path=it['path'],
                      old_text=it['old_text'],
                      new_text=it['new_text'],
                      exact=it.get(
                          'exact',
                          False),
                      replace_all=it.get(
                          'replaceAll',
                          False),
                      id=it.get('id'),
                      add_import=add_import,
                      remove_import=remove_import)

              def result_serializer(r: EditBlockResult) -> dict[str, Any]:
                  entry = {'path': r.path, 'result': r.result}
                  if r.id is not None:
                      entry['id'] = r.id
                  if r.new_id is not None:
                      entry['new_id'] = r.new_id
                  if (imports := import_entry(r.imports)) is not None:
                      entry['imports'] = imports
                  return entry

              def error_serializer(e: EditBlockError) -> dict[str, Any]:
                  entry = {'path': e.path, 'error': e.error}
                  if e.id is not None:
                      entry['id'] = e.id
                  for field in ('candidates', 'reason', 'corrected_text', 'guess', 'next_step'):
                      value = getattr(e, field)
                      if value is not None:
                          entry[field] = value
                  return entry
              return handle_batch_tool(
                  ctx,
                  item_factory,
                  ast_edit_block,
                  core.AstError,
                  result_serializer,
                  error_serializer,
                  auto_approve=True)
```
/answer 3a25-d3-d2 allow
Control Request:
```yaml
id: 3a25-d4-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: def (replace_in_block|line_preserving)|class TextMatch|def find_nodes|def .*by_text
    include:
    - '*.py'
    exclude:
    - __pycache__/**
  reason: Hilfsfunktionen für Textmatch und Knotensuche finden
```
/answer 3a25-d4-0 allow
Control Request:
```yaml
id: 3a25-d5-d4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: ast/common.py
        matches:
        - "def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:"
      - path: _text_match.py
        matches:
        - "class TextMatchError(Exception):"
        - "def line_preserving(reference: str) -> ReplaceGuard:"
        - "def replace_in_block(block: str, old_text: str, new_text: str, *, exact: bool, replace_all: bool=False, accept: ReplaceGuard | None=None, max_level: int=3, where: str='block') -> str:"
```
/answer 3a25-d5-d4 allow
Control Request:
```yaml
id: 3a25-d6-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  name: select_by_text
  reason: "select_by_text lesen: Verhalten bei leerer id und Textsuche"
```
/answer 3a25-d6-0 allow
Control Request:
```yaml
id: 3a25-d7-d6
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
      nodes:
      - id: select_by_text
        code: |-
          def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:
              """Return the node addressed by ``id``, or, if omitted, the single node whose
              source contains every string in *texts*.

              Nodes fully containing another matching node (an ancestor picked up merely
              because it contains the descendant) are dropped in favour of the innermost
              match; nesting alone is therefore not treated as ambiguity.

              Raises:
                  core.AstError: If ``id`` is given but matches zero/many nodes, or no
                      node contains all of *texts*.
                  core.AstAmbiguous: If several unrelated candidates remain after
                      dropping nested matches; carries their ids as ``candidates``.
              """
              if id is not None:
                  return select_one(tree, id=id)
              candidates = [loc for loc in core.locate_all(tree) if all((text in core.edit_node_source(loc) for text in texts))]
              if not candidates:
                  raise core.AstError('No node matched the given text; a node selector (id) is required.')
              candidates = [
                  c for c in candidates if not any(
                      (other is not c and c.lineno <= other.lineno and (
                          c.end_lineno >= other.end_lineno) and (
                              (c.lineno,
                               c.end_lineno) != (
                                  other.lineno,
                                  other.end_lineno)) for other in candidates))]
              if len(candidates) > 1:
                  raise core.AstAmbiguous(
                      'Multiple target nodes found; specify a node selector (id).', [
                          c.node_id for c in candidates])
              return candidates[0]
```
/answer 3a25-d7-d6 allow
ReasoningToken: 157
Control Request:
```yaml
id: 3a25-d8-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: select_by_text|def select_one
    include:
    - '*.py'
    exclude:
    - __pycache__/**
    intent: editing
  reason: Alle Aufrufer von select_by_text und select_one finden
```
/answer 3a25-d8-0 allow
Control Request:
```yaml
id: 3a25-d9-d8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/tools/ast/common.py
        nodes:
        - id: w0xyc1|24HH6w
          matches:
          - "'select_by_text']"
        - id: select_one
          matches:
          - "def select_one(tree, **selectors: Any) -> core.Located:"
        - id: select_by_text
          matches:
          - "def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:"
      - path: xy/ai/mcpc/tools/ast/edit_marks.py
        nodes:
        - id: G5Js58|8gAslm
          matches:
          - "from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_text"
        - id: edit_marks_one.BxRWv7|Y8OjRW
          matches:
          - "target = select_by_text(tree, [item.start_marker, item.end_marker], id=item.id)"
      - path: xy/ai/mcpc/tools/ast/edit_block.py
        nodes:
        - id: G5Js58|8gAslm
          matches:
          - "from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_text"
        - id: edit_block_one.yACvDQ|cNjCAq
          matches:
          - "target = select_by_text(tree, [item.old_text], id=item.id)"
```
/answer 3a25-d9-d8 allow
ReasoningToken: 104
Control Request:
```yaml
id: 3a25-da-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
    id: select_by_text
    old_text: |2-
          if id is not None:
              return select_one(tree, id=id)
    new_text: |2-
          if id is not None and id.strip():
              return select_one(tree, id=id)
    exact: false
  reason: "Leere/Whitespace-id wie fehlende id behandeln, damit die Textsuche greift"
```
/answer 3a25-da-0 allow
Result Stats: id=86622281-950f-4cac-8b28-8f12df3ec6b6, total: 15876, in: 12, out: 2249, reason: 0, read: 52352, write: 13615, start: 1794516733731, end: 1794516748019, time: 14209, total: 35332, io: 0, control: 79, sent: 0, recv: 170
Du hattest recht. Der Fehler war vermeidbar, und ich habe die Ursache behoben. Ich habe die Änderung nicht getestet, und `ast_validate` habe ich nicht laufen lassen.

**Ursache**

`_edit_block_one` in `edit_block.py` ruft `select_by_text(tree, [item.old_text], id=item.id)` auf. Diese Funktion in `tools/ast/common.py` hat bereits eine Rückfallebene. Ohne `id` sucht sie den einen Knoten, dessen Quelltext `old_text` enthält. Genau das hätte hier funktioniert.

Der Aufruf kam aber mit `id: ""`. Die Prüfung lautete `if id is not None`, und ein leerer String ist nicht `None`. Deshalb lief `select_one(tree, id="")`, und das lieferte „No node matched the selector“. Die Rückfallebene wurde nie erreicht.

**Änderung**

In `select_by_text` in `common.py` steht jetzt `if id is not None and id.strip():`. Eine leere oder nur aus Leerzeichen bestehende `id` zählt damit als nicht angegeben. Dann greift die Textsuche. Findet sie mehrere Knoten, kommt weiterhin die `AstAmbiguous`-Meldung mit den Kandidaten-IDs.

Die Änderung wirkt auch bei `ast_edit_marks`, weil das dieselbe Funktion nutzt.

**Zu beachten**

- Die Textsuche vergleicht per exaktem Substring (`text in source`). Weicht `old_text` in den Leerzeichen vom Quelltext ab, findet sie nichts, obwohl `edit_block` selbst tolerant arbeitet.
- Im Schema ist `id` weiterhin als Pflichtfeld markiert (`_ITEM_REQUIRED`). Das passt nicht ganz zu dem Rückfallverhalten. Ich habe es nicht angefasst, weil `id: ""` damit ein gültiger Weg für „bitte selbst suchen“ bleibt. Wenn die `id` offiziell optional sein soll, nimm sie aus `_ITEM_REQUIRED` heraus.
Es ist ein Notfall Rückfallverhalten. Was ist mit dem Fall die Node-ID wäre falsch gewesen?
SystemInit:  id=86622281-950f-4cac-8b28-8f12df3ec6b6, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
ReasoningToken: 194
Control Request:
```yaml
id: 3a25-db-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
    ids:
    - select_one
    - select_by_text
  reason: "select_one lesen: welche Fehler bei falscher id geworfen werden"
```
/answer 3a25-db-0 allow
Control Request:
```yaml
id: 3a25-dc-db
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
      nodes:
      - id: select_one
        type: FunctionDef
        code: |-
          def select_one(tree, **selectors: Any) -> core.Located:
              """Return the single node in *tree* matching *selectors*.

              Raises:
                  core.AstError: If no node matches, or more than one node matches.
              """
              hits = core.find(tree, **selectors)
              if not hits and set(selectors) == {'id'}:
                  fallback = core.resolve_by_prefix(core.locate_all(tree), selectors['id'])
                  hits = [fallback] if fallback is not None else []
              if not hits:
                  raise core.AstError('No node matched the selector. verify the node with `search` or `ast_find`')
              if len(hits) > 1:
                  raise core.AstError(f'Selector is ambiguous – {len(hits)} nodes matched.')
              return hits[0]
      - id: select_by_text
        type: FunctionDef
        code: |-
          def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:
              """Return the node addressed by ``id``, or, if omitted, the single node whose
              source contains every string in *texts*.

              Nodes fully containing another matching node (an ancestor picked up merely
              because it contains the descendant) are dropped in favour of the innermost
              match; nesting alone is therefore not treated as ambiguity.

              Raises:
                  core.AstError: If ``id`` is given but matches zero/many nodes, or no
                      node contains all of *texts*.
                  core.AstAmbiguous: If several unrelated candidates remain after
                      dropping nested matches; carries their ids as ``candidates``.
              """
              if id is not None and id.strip():
                  return select_one(tree, id=id)
              candidates = [loc for loc in core.locate_all(tree) if all((text in core.edit_node_source(loc) for text in texts))]
              if not candidates:
                  raise core.AstError('No node matched the given text; a node selector (id) is required.')
              candidates = [
                  c for c in candidates if not any(
                      (other is not c and c.lineno <= other.lineno and (
                          c.end_lineno >= other.end_lineno) and (
                              (c.lineno,
                               c.end_lineno) != (
                                  other.lineno,
                                  other.end_lineno)) for other in candidates))]
              if len(candidates) > 1:
                  raise core.AstAmbiguous(
                      'Multiple target nodes found; specify a node selector (id).', [
                          c.node_id for c in candidates])
              return candidates[0]
```
/answer 3a25-dc-db allow
ReasoningToken: 193
Control Request:
```yaml
id: 3a25-dd-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
    id: select_by_text
    start_marker: "    if id is not None and id.strip():"
    end_marker: "        return select_one(tree, id=id)"
    content: |2-
          if id is not None and id.strip():
              try:
                  return select_one(tree, id=id)
              except core.AstError as id_error:
                  # Emergency fallback: the id is unusable (stale/wrong/ambiguous).
                  # Try to locate the node unambiguously by text; if that fails too,
                  # report the original id error.
                  try:
                      return _select_by_contained_text(tree, texts)
                  except core.AstError:
                      raise id_error from None
          return _select_by_contained_text(tree, texts)


      def _select_by_contained_text(tree, texts: list[str]) -> core.Located:
          """Return the single innermost node whose source contains every string in *texts*."""
  reason: Fallback auf Textsuche auch bei falscher id; Textsuche in Hilfsfunktion auslagern
```
/answer 3a25-dd-0 allow
Control Request:
```yaml
id: 3a25-de-dd
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
      error: "Syntax error: unexpected indent (line 15)"
      id: select_by_text
```
/answer 3a25-de-dd allow
Control Request:
```yaml
id: 3a25-df-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
    id: select_by_text
    source: |-
      def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:
          """Return the node addressed by ``id``; if ``id`` is omitted, blank or unusable
          (stale, wrong, ambiguous), fall back to the single node whose source contains
          every string in *texts*.

          Raises:
              core.AstError: If no node can be determined. When a non-blank ``id`` was
                  given, the original id error is reported.
              core.AstAmbiguous: If the text fallback finds several unrelated candidates;
                  carries their ids as ``candidates``.
          """
          if id is not None and id.strip():
              try:
                  return select_one(tree, id=id)
              except core.AstError as id_error:
                  try:
                      return _select_by_contained_text(tree, texts)
                  except core.AstError:
                      raise id_error from None
          return _select_by_contained_text(tree, texts)


      def _select_by_contained_text(tree, texts: list[str]) -> core.Located:
          """Return the single innermost node whose source contains every string in *texts*.

          Nodes fully containing another matching node are dropped in favour of the
          innermost match; nesting alone is not treated as ambiguity.
          """
          candidates = [loc for loc in core.locate_all(tree) if all((text in core.edit_node_source(loc) for text in texts))]
          if not candidates:
              raise core.AstError('No node matched the given text; a node selector (id) is required.')
          candidates = [
              c for c in candidates if not any(
                  (other is not c and c.lineno <= other.lineno and (
                      c.end_lineno >= other.end_lineno) and (
                          (c.lineno,
                           c.end_lineno) != (
                              other.lineno,
                              other.end_lineno)) for other in candidates))]
          if len(candidates) > 1:
              raise core.AstAmbiguous(
                  'Multiple target nodes found; specify a node selector (id).', [
                      c.node_id for c in candidates])
          return candidates[0]
  reason: "select_by_text komplett neu schreiben: Fallback auch bei falscher id"
```
/answer 3a25-df-0 allow
ReasoningToken: 202
Control Request:
```yaml
id: 3a25-e0-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  reason: Syntax prüfen
```
/answer 3a25-e0-0 allow
Result Stats: id=86622281-950f-4cac-8b28-8f12df3ec6b6, total: 23554, in: 20, out: 5358, reason: 0, read: 112947, write: 18176, start: 1794516894572, end: 1794516901563, time: 6929, total: 61252, io: 0, control: 62, sent: 0, recv: 165
Jetzt greift die Textsuche auch dann, wenn die Node-ID falsch ist. Die Datei `common.py` ist syntaktisch gültig (`ast_validate`). Einen Funktionstest habe ich nicht gemacht.

**Neues Verhalten von `select_by_text`**

| Fall | Ergebnis |
|---|---|
| `id` fehlt, ist leer oder besteht nur aus Leerzeichen | Textsuche, wie bisher |
| `id` ist gültig | Dieser Knoten wird verwendet |
| `id` ist falsch, veraltet oder nicht eindeutig | Textsuche als Notfall-Rückfall |
| Auch die Textsuche findet keinen eindeutigen Knoten | Der ursprüngliche ID-Fehler wird gemeldet, damit die Meldung zur Eingabe passt |

Ich habe die Textsuche in die Hilfsfunktion `_select_by_contained_text` ausgelagert. Sie nimmt weiterhin den innersten passenden Knoten. Mehrere unabhängige Treffer ergeben `AstAmbiguous` mit den Kandidaten-IDs.

**Grenzen**

- **Gültige ID, falscher Knoten:** Zeigt die ID auf einen existierenden Knoten, der `old_text` nicht enthält, gibt es keinen Rückfall. Dann kommt weiterhin der Fehler aus `replace_in_block`. Ich habe das bewusst nicht geändert, weil die ID dann eindeutig eine andere Absicht ausdrückt und ein stiller Wechsel des Knotens überraschen könnte. Sag Bescheid, falls du dort auch einen Rückfall willst.
- **Stille Umleitung bei falscher ID:** Die Antwort enthält nur die `new_id` des tatsächlich bearbeiteten Knotens. Ein Hinweis, dass auf die Textsuche ausgewichen wurde, fehlt. Dafür müsste ich ein Feld in `EditBlockResult` ergänzen, etwa `notice`. Soll ich das machen?
- **`ast_edit_marks`:** Der Rückfall gilt auch dort. Das passt, denn dort sucht die Textsuche nach `start_marker` und `end_marker`.
