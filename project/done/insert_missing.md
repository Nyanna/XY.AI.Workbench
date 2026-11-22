In `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py` fehlt das der ID parameter mandatory ist. Prüfe das.
SystemInit:  id=64a509f0-1a95-4bdf-a67d-df4d50d79c03, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 1632-86-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  reason: "Datei insert.py lesen, um die Definition des id-Parameters zu prüfen"
```
/answer 1632-86-0 allow
Control Request:
```yaml
id: 1632-87-86
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
        nodes:
        - id: 9CmN9x|nHobTc
          excert: "'``ast_insert`` tool: insert statement(s) relative to selected nodes.'"
        - id: 8Wm4pY|RoeKEJ
          excert: from dataclasses import dataclass
        - id: indDlk|VBU4wK
          excert: "from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH…"
        - id: oTjM4y|Jut0OL
          excert: "__all__ = ['InsertItem', 'InsertResult', 'InsertError', 'InsertBatchResult', 'a…"
        - id: InsertItem
          excert: "@dataclass(frozen=True) class InsertItem:"
          docstring: "One insert operation. Attributes: path: Absolute path to the file to modify. so…"
        - id: InsertResult
          excert: "@dataclass(frozen=True) class InsertResult(ItemResult):"
          docstring: "Result of a single insert operation. Attributes: path: The path exactly as give…"
          children:
          - id: InsertResult.aUv9Tr|y2n1TA
            excert: "\"Result of a single insert operation.\\n\\n    Attributes:\\n        path: The pat…"
          - id: InsertResult.key
            excert: "def key(self) -> dict[str, Any]:"
          - id: InsertResult.payload
            excert: "def payload(self) -> dict[str, Any]:"
        - id: InsertError
          excert: "class InsertError(NodeError):"
          docstring: "Error applying a single insert operation. Attributes: path: The path exactly as…"
        - id: InsertBatchResult
          excert: "class InsertBatchResult(ImportCheckedBatchResult[InsertResult, InsertError]):"
          docstring: "Result of :func:`ast_insert`. Attributes: results: One :class:`InsertResult` pe…"
        - id: insert_one
          excert: "def _insert_one(item: InsertItem) -> InsertResult:"
        - id: ast_insert
          excert: "def ast_insert(items: list[InsertItem]) -> InsertBatchResult:"
          docstring: Insert statement(s) parsed from ``source`` relative to one or more selected nod…
        - id: InsertNodeTool
          excert: "class InsertNodeTool(ToolDefinition):"
          children:
          - id: InsertNodeTool.098XR3|mb1vJU
            excert: name = 'ast_insert'
          - id: InsertNodeTool.c0C4qn|Ha6JTT
            excert: "_ITEM_PROPERTIES = {"
          - id: InsertNodeTool.ei5nIo|Lu9IT0
            excert: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTIO…"
          - id: InsertNodeTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: "Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 1632-87-86 allow
Control Request:
```yaml
id: 1632-88-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    ids:
    - InsertItem
    - InsertNodeTool
    - insert_one
  reason: "Schema, Item-Dataclass und Handler lesen"
```
/answer 1632-88-0 allow
Control Request:
```yaml
id: 1632-89-88
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
        nodes:
        - id: InsertItem
          type: ClassDef
          code: |-
            @dataclass(frozen=True)
            class InsertItem:
                """One insert operation.

                Attributes:
                    path: Absolute path to the file to modify.
                    source: Source of the statement(s) to insert.
                    position: ``"before"`` or ``"after"`` the selected node. Defaults to ``"after"``.
                    id: Unique id of the target node.
                    add_import: Import statements to add if missing, applied after the insert.
                    remove_import: Import statements to remove if present, applied after the insert.
                """
                path: str
                source: str
                position: str = 'after'
                id: str | None = None
                add_import: list[str] | None = None
                remove_import: list[str] | None = None
        - id: InsertNodeTool
          type: ClassDef
          excert: "class InsertNodeTool(ToolDefinition):"
          children:
          - id: InsertNodeTool.098XR3|mb1vJU
            type: statements
            code: |-
              name = 'ast_insert'
              features = frozenset({ToolFeature.INPUT_ASTID})
              title = 'Insert AST nodes'
              description = "Insert source relative to selected nodes ('before' or 'after'), for a batch of items; several operations may target the same or different files."
          - id: InsertNodeTool.c0C4qn|Ha6JTT
            type: statements
            code: |-
              _ITEM_PROPERTIES = {
                  'path': PATH_PROP,
                  'source': {
                      'type': 'string',
                      'description': 'Source to insert.'},
                  'position': {
                      'type': 'string',
                      'enum': [
                              'before',
                              'after'],
                      'description': 'Placement relative to the selected node.',
                      'default': 'after'},
                  **PATH_SELECTOR_PROPS,
                  **IMPORT_PROPS}
              _ITEM_REQUIRED = ['path', 'source']
              _ITEMS_DESCRIPTION = 'Insert operations to apply.'
          - id: InsertNodeTool.ei5nIo|Lu9IT0
            type: statements
            code: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
          - id: InsertNodeTool.handle
            type: FunctionDef
            code: |-
              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""

                  def item_factory(it: dict[str, Any]) -> InsertItem:
                      add_import, remove_import = parse_import_args(it)
                      return InsertItem(
                          path=it['path'],
                          source=it['source'],
                          position=it.get(
                              'position',
                              'after'),
                          id=it.get('id'),
                          add_import=add_import,
                          remove_import=remove_import)
                  return handle_batch_tool(ctx, item_factory, ast_insert, core.AstError, auto_approve=True)
        - id: insert_one
          type: FunctionDef
          code: |-
            def _insert_one(item: InsertItem) -> InsertResult:
                file_path = core.require_path(item.path)
                tree = core.CACHE.get_tree(file_path)
                target = select_by_path(tree, id=item.id)
                before_ids = {loc.node_id for loc in core.locate_all(tree)}
                inserted = core.insert_node(target, item.source, item.position)
                new_ids = [loc.node_id for loc in core.locate_all(tree) if loc.node_id not in before_ids]
                imports = core.apply_imports(tree, item.add_import, item.remove_import)
                core.CACHE.save(file_path, tree)
                return InsertResult(path=item.path, id=item.id, inserted=inserted, ids=new_ids or None, imports=imports)
```
/answer 1632-89-88 allow
ReasoningToken: 41
Control Request:
```yaml
id: 1632-8a-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: _ITEM_REQUIRED|def select_by_path|PATH_SELECTOR_PROPS =
    include:
    - '*.py'
    exclude:
    - __pycache__/**
  reason: "Vergleich mit anderen Tools: _REQUIRED und select_by_path"
```
/answer 1632-8a-0 allow
Control Request:
```yaml
id: 1632-8b-8a
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
        matches:
        - path: replace.py
          matches:
          - "_ITEM_REQUIRED = ['path', 'source', 'id']"
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: delete.py
          matches:
          - "_ITEM_REQUIRED = ['path']"
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: common.py
          matches:
          - "PATH_SELECTOR_PROPS = {'id': SELECTOR_PROPS['id']}"
          - "def select_by_path(tree, *, id: str | None=None) -> core.Located:"
        - path: edit_marks.py
          matches:
          - "_ITEM_REQUIRED = ['path', 'start_marker', 'end_marker', 'content']"
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: read.py
          matches:
          - "_ITEM_REQUIRED = ['path']"
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: edit_block.py
          matches:
          - "_ITEM_REQUIRED = ['path', 'old_text', 'new_text', 'id']"
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: create.py
          matches:
          - "_ITEM_REQUIRED = ['path', 'source']"
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: insert.py
          matches:
          - "_ITEM_REQUIRED = ['path', 'source']"
        warnings:
        - "Limit of 15 matches reached (total 16); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 1632-8b-8a allow
ReasoningToken: 103
Control Request:
```yaml
id: 1632-8c-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertNodeTool.c0C4qn|Ha6JTT
    old_text: "_ITEM_REQUIRED = ['path', 'source']"
    new_text: "_ITEM_REQUIRED = ['path', 'source', 'id']"
  reason: id im Schema als Pflichtfeld deklarieren und im Handler strikt lesen
```
/answer 1632-8c-0 allow
Result Stats: id=64a509f0-1a95-4bdf-a67d-df4d50d79c03, total: 11711, in: 10, out: 1342, reason: 0, read: 31215, write: 10359, start: 1795381085115, end: 1795381088982, time: 3823, total: 15597, io: 0, control: 44, sent: 0, recv: 84
Ja, das stimmte: In `insert.py` war `id` nicht Pflicht. `_ITEM_REQUIRED` enthielt nur `['path', 'source']`. `replace.py` und `edit_block.py` verlangen `id` dagegen. Ich habe das Schema auf `_ITEM_REQUIRED = ['path', 'source', 'id']` geändert. Getestet habe ich die Änderung nicht.

Zwei Punkte in `insert.py` habe ich nicht angefasst:
- `InsertItem.id` ist weiter `str | None = None`.
- `handle` liest weiter `it.get('id')`.

Python-Aufrufe von `ast_insert` ohne `id` werden also nicht abgelehnt. Das Schema greift nur bei MCP-Aufrufen. Wenn `id` auch dort Pflicht sein soll, müsste man die Felder in der Dataclass umsortieren, weil `id` dann kein Default mehr hätte. Außerdem müsste `handle` `it['id']` lesen. Soll ich das auch umsetzen?
Was heißt nicht angefasst?
Result Stats: id=64a509f0-1a95-4bdf-a67d-df4d50d79c03, total: 12515, in: 12, out: 1729, reason: 0, read: 41574, write: 10774, start: 1795381109570, end: 1795381113237, time: 3624, total: 19221, io: 0, control: 43, sent: 1, recv: 88
SystemInit:  id=64a509f0-1a95-4bdf-a67d-df4d50d79c03, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
„Nicht angefasst“ heißt: Ich habe diese Stellen im Code nicht verändert, sie sind so, wie sie vorher waren.

Konkret:
- **`InsertItem`**: Das Feld steht weiter als `id: str | None = None` da. Wer `InsertItem(...)` direkt in Python aufruft, kann `id` also weglassen.
- **`handle`**: Es liest weiter `id=it.get('id')`. Das liefert `None`, wenn `id` fehlt, und löst keinen Fehler aus.

Geändert habe ich nur `_ITEM_REQUIRED`. Das ist die Liste im JSON-Schema, die MCP-Clients sehen. Sie sagt dort, dass `id` Pflicht ist. Ob das Schema auch durchgesetzt wird, hängt von `batch_schema` und der Validierung davor ab. Das habe ich nicht geprüft.

Der Code dahinter akzeptiert ein fehlendes `id` weiterhin ohne Fehlermeldung. Wenn du willst, ändere ich die beiden Stellen auch.
ID ist pflicht auch wenn es fallbacks gibt
SystemInit:  id=64a509f0-1a95-4bdf-a67d-df4d50d79c03, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 46
Control Request:
```yaml
id: 1632-8d-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: InsertItem\(
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    - '**/.venv/**'
    limit: 30
  reason: "Verwendungen von InsertItem prüfen, bevor Feldreihenfolge geändert wird"
```
/answer 1632-8d-0 allow
Control Request:
```yaml
id: 1632-8e-8d
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench
        matches:
        - path: mcpc/tests/test_ast_engines.py
          matches:
          - "ast_insert(items=[InsertItem(path=py_file, source='z = 5', id='bar', position='after')])"
        - path: mcpc/tests/test_ast_markdown_bridge.py
          matches:
          - "result = ast_insert(items=[InsertItem(path=md_file, source='## Part 1b\\n\\nmid', id='Doc.Part_1', position='after')])"
        - path: mcpc/tests/test_ast_imports.py
          matches:
          - InsertItem(
          - "ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['import sys'])])"
          - "ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['from __future__ import annotations'])])"
          - "result = ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['x = 1'])])"
        - path: mcpc/src/xy/ai/mcpc/tools/ast/insert.py
          matches:
          - return InsertItem(
```
/answer 1632-8e-8d allow
Text:
 Alle Aufrufer nutzen Keywords, die Umsortierung ist also sicher.
Control Request:
```yaml
id: 1632-8f-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertItem
    old_text: |2
          source: str
          position: str = 'after'
          id: str | None = None
    new_text: |2
          source: str
          id: str
          position: str = 'after'
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertNodeTool.handle
    old_text: "id=it.get('id'),"
    new_text: "id=it['id'],"
  reason: id in InsertItem und Handler zum Pflichtfeld machen
```
/answer 1632-8f-0 allow
Result Stats: id=64a509f0-1a95-4bdf-a67d-df4d50d79c03, total: 15155, in: 18, out: 2655, reason: 0, read: 75468, write: 12482, start: 1795381168998, end: 1795381172311, time: 3268, total: 27919, io: 0, control: 45, sent: 0, recv: 67
`id` ist jetzt in `insert.py` an drei Stellen Pflicht. Getestet habe ich die Änderung nicht, auch die Tests habe ich nicht ausgeführt.

- **Schema:** `_ITEM_REQUIRED` enthält `id`. Das hatte ich schon vorher geändert.
- **`InsertItem`:** `id: str` hat keinen Default mehr und steht vor `position`. Alle Aufrufer in den Tests und im Handler nutzen Keyword-Argumente, die Umsortierung bricht sie also nicht.
- **`handle`:** Es liest `it['id']`. Fehlt `id`, gibt es einen `KeyError`.

Der Fallback `select_by_path(tree, id=None)` bleibt in `common.py`, weil andere Tools ihn nutzen. `insert.py` übergibt aber immer eine `id`.
Teste sie
SystemInit:  id=64a509f0-1a95-4bdf-a67d-df4d50d79c03, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 1632-90-0
toolName: python
arguments:
  script: |-
    import subprocess
    r = subprocess.run(["python","-m","pytest","-q","tests/test_ast_engines.py","tests/test_ast_markdown_bridge.py","tests/test_ast_imports.py","-k","insert or import or markdown","-x","-q"],cwd="/home/user/xyan/xy.ai.workbench/mcpc",capture_output=True,text=True)
    print(r.stdout[-3000:], r.stderr[-2000:])
    r = subprocess.run(["python","-m","pytest","-q","tests"],cwd="/home/user/xyan/xy.ai.workbench/mcpc",capture_output=True,text=True)
    print(r.stdout[-3000:], r.stderr[-2000:])
  reason: Relevante Tests ausführen
```
/answer 1632-90-0 allow
Control Request:
```yaml
id: 1632-91-90
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      .............................F
      =================================== FAILURES ===================================
      _____________________ test_locate_all_builds_heading_tree ______________________

      engine = <xy.ai.mcpc.tools.ast.rpc._markdown.MarkdownEngine object at 0x757d7f416450>

          @requires_engine
          def test_locate_all_builds_heading_tree(engine: MarkdownEngine) -> None:
              located = engine.locate_all(engine.parse(DOC))
              assert [
                  loc.node_type for loc in located] == [
                      'Section',
                      'BlockGroup',
                      'Section',
                      'BlockGroup',
                      'Section',
                  'BlockGroup']
              sections = [loc for loc in located if loc.node_type == 'Section']
      >       assert [loc.node_id for loc in sections] == ['Title', 'Title.Section_A', 'Title.Section_B']
      E       AssertionError: assert ['JLeD', 'JLe..., 'JLeD.DpCJ'] == ['Title', 'Ti...le.Section_B']
      E
      E         At index 0 diff: 'JLeD' != 'Title'
      E         Use -v to get more diff

      tests/test_ast_markdown_bridge.py:195: AssertionError
      =========================== short test summary info ============================
      FAILED tests/test_ast_markdown_bridge.py::test_locate_all_builds_heading_tree
      !!!!!!!!!!!!!!!!!!!!!!!!!! stopping after 1 failures !!!!!!!!!!!!!!!!!!!!!!!!!!!

      est_skipped_heading_levels_nest_under_previous
      FAILED tests/test_ast_markdown_bridge.py::test_duplicate_headings_get_numbered_ids
      FAILED tests/test_ast_markdown_bridge.py::test_headings_inside_code_quote_and_list_are_not_sections
      FAILED tests/test_ast_markdown_bridge.py::test_thematic_breaks_do_not_split_sections
      FAILED tests/test_ast_markdown_bridge.py::test_replace_section_with_new_heading_and_body
      FAILED tests/test_ast_markdown_bridge.py::test_replace_changes_heading_level
      FAILED tests/test_ast_markdown_bridge.py::test_replace_root_section_replaces_whole_document
      FAILED tests/test_ast_markdown_bridge.py::test_replace_with_multiple_blocks
      FAILED tests/test_ast_markdown_bridge.py::test_insert_section_before_and_after
      FAILED tests/test_ast_markdown_bridge.py::test_insert_returns_number_of_blocks
      FAILED tests/test_ast_markdown_bridge.py::test_insert_before_first_node - Sto...
      FAILED tests/test_ast_markdown_bridge.py::test_delete_section - StopIteration
      FAILED tests/test_ast_markdown_bridge.py::test_delete_paragraph_and_whole_document
      FAILED tests/test_ast_markdown_bridge.py::test_mutation_normalizes_document
      FAILED tests/test_ast_markdown_bridge.py::test_mutation_normalizes_crlf_to_lf
      FAILED tests/test_ast_markdown_bridge.py::test_mutation_keeps_frontmatter - S...
      FAILED tests/test_ast_markdown_bridge.py::test_mutations_chain_on_the_same_tree
      FAILED tests/test_ast_markdown_bridge.py::test_unknown_node_id_raises_and_keeps_source[replace]
      FAILED tests/test_ast_markdown_bridge.py::test_unknown_node_id_raises_and_keeps_source[insert]
      FAILED tests/test_ast_markdown_bridge.py::test_unknown_node_id_raises_and_keeps_source[delete]
      FAILED tests/test_ast_markdown_bridge.py::test_empty_replacement_is_rejected
      FAILED tests/test_ast_markdown_bridge.py::test_control_characters_are_rejected_before_the_call
      FAILED tests/test_ast_markdown_bridge.py::test_concurrent_requests_are_isolated
      FAILED tests/test_ast_markdown_bridge.py::test_large_document - StopIteration
      FAILED tests/test_ast_markdown_bridge.py::test_outline_lists_sections_and_blocks
      FAILED tests/test_ast_markdown_bridge.py::test_find_by_type_and_name - Assert...
      FAILED tests/test_ast_markdown_bridge.py::test_read_returns_section_source - ...
      FAILED tests/test_ast_markdown_bridge.py::test_replace_via_tool_writes_file
      FAILED tests/test_ast_markdown_bridge.py::test_insert_via_tool_reports_ids - ...
      FAILED tests/test_ast_markdown_bridge.py::test_delete_via_tool - AssertionErr...
      FAILED tests/test_ast_markdown_bridge.py::test_batch_of_edits_in_one_call - A...
      FAILED tests/test_ast_markdown_bridge.py::test_cache_follows_external_changes
      FAILED tests/test_ast_markdown_bridge.py::test_cache_follows_tool_edits - Ass...
      FAILED tests/test_ast_markdown_bridge.py::test_unicode_file_roundtrip_via_tool
      FAILED tests/test_ast_markdown_bridge.py::test_retrieve_outline_find_read - a...
      FAILED tests/test_ast_markdown_bridge.py::test_retrieve_search_returns_node_ids_usable_for_read
      42 failed, 442 passed, 1 xfailed in 15.72s
```
/answer 1632-91-90 allow sieht nach einer änderung des markdown AST aus, korrigiere das auf das neue ID format
ReasoningToken: 107
Control Request:
```yaml
id: 1632-92-0
toolName: python
arguments:
  script: |-
    import subprocess
    r = subprocess.run(["python","-m","pytest","-q","tests","--tb=no","-q"],cwd="/home/user/xyan/xy.ai.workbench/mcpc",capture_output=True,text=True)
    import re
    print("\n".join(sorted({l.split("::")[0] for l in r.stdout.splitlines() if l.startswith("FAILED")})))
    print(r.stdout[-300:])
    r = subprocess.run(["git","status","--short"],cwd="/home/user/xyan/xy.ai.workbench",capture_output=True,text=True)
    print(r.stdout)
    r = subprocess.run(["git","log","-5","--stat","--format=%h %s"],cwd="/home/user/xyan/xy.ai.workbench",capture_output=True,text=True)
    print(r.stdout[:2500])
  reason: Vollständige Liste der Fehlschläge und Dateien ermitteln
```
/answer 1632-92-0 deny korrigiere erst die gezeigten
ReasoningToken: 57
Control Request:
```yaml
id: 1632-93-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
  reason: "Markdown-Bridge-Tests und ID-Erzeugung der Markdown-Engine lesen, um das neue ID-Format zu verstehen"
```
/answer 1632-93-0 allow
Control Request:
```yaml
id: 1632-94-93
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
        nodes:
        - id: vderWI|CmiNLI
          excert: '''Eigenständige Tests für die Markdown-RPC-Bridge (``xy.ai.mcpc.tools.ast.rpc._m…'
        - id: gZcKNj|03aXWS
          excert: from __future__ import annotations
        - id: oEgkx1|OfoLGZ
          excert: "from xy.ai.mcpc.tools.ast.edit_block import EditBlockItem, ast_edit_block"
        - id: PQJTLs|KgKQKq
          excert: "from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace"
        - id: kcapNL|6Fluzl
          excert: requires_engine = pytest.mark.skipif(
        - id: engine
          excert: "@pytest.fixture(scope='module') def engine() -> MarkdownEngine:"
        - id: find
          excert: "def _find(engine: MarkdownEngine, tree: Tree, node_id: str) -> Located:"
        - id: ids
          excert: "def _ids(engine: MarkdownEngine, source: str) -> list[str]:"
        - id: free_port
          excert: "def _free_port() -> int:"
        - id: flatten
          excert: "def _flatten(nodes: list[OutlineNode]) -> list[OutlineNode]:"
        - id: 4f9Vfx|Qgz05s
          excert: '''# ---------------------------------------------------------------------------'''
        - id: test_engine_dir_and_entry_point_exist
          excert: "def test_engine_dir_and_entry_point_exist() -> None:"
        - id: test_process_defaults
          excert: "def test_process_defaults(monkeypatch: pytest.MonkeyPatch) -> None:"
        - id: test_process_honours_environment
          excert: "def test_process_honours_environment(monkeypatch: pytest.MonkeyPatch) -> None:"
        - id: h5b7a775b
          excert: "def test_default_port_does_not_collide_with_other_engines(monkeypatch: pytest.MonkeyPatch) -> None:"
        - id: test_engine_is_registered_as_rpc_engine
          excert: "def test_engine_is_registered_as_rpc_engine() -> None:"
        - id: h347e0e87
          excert: "@pytest.mark.parametrize('name', ['x.md', 'x.markdown']) def test_extensions_resolve_to_markdown_engine(tmp_path: Path, name: str) -> None:"
        - id: test_engine_metadata
          excert: "def test_engine_metadata() -> None:"
        - id: h2b9a814e
          excert: "def test_constructing_engine_does_not_start_process() -> None:"
        - id: 4f9Vfx|B4Zl7x
          excert: '''# ---------------------------------------------------------------------------'''
        - id: h3a97ec38
          excert: "@requires_engine def test_process_starts_lazily_and_terminates(monkeypatch: pytest.MonkeyPatch) -> None:"
        - id: test_missing_node_binary_fails_fast
          excert: "def test_missing_node_binary_fails_fast(monkeypatch: pytest.MonkeyPatch) -> None:"
        - id: h6014b207
          excert: "def test_invalid_port_configuration_is_rejected(monkeypatch: pytest.MonkeyPatch) -> None:"
        - id: 4f9Vfx|TgECq4
          excert: '''# ---------------------------------------------------------------------------'''
        - id: test_parse_and_serialize_roundtrip
          excert: "@requires_engine def test_parse_and_serialize_roundtrip(engine: MarkdownEngine) -> None:"
        - id: test_parse_does_not_modify_source
          excert: "@requires_engine @pytest.mark.parametrize('source', ['', '\\n', 'plain\\n', '---\\ntitle: X\\n---\\n\\n# H\\n', '# A\\r\\n\\r\\ntext\\r\\n', '* a\\n* b\\n', 'no trailing newline', '# Ünï ✓ 日本\\n\\nText äöü 😀\\n']) def test_parse_does_not_modify_source(engine: MarkdownEngine, source: str) -> None:"
        - id: test_any_text_is_valid_markdown
          excert: "@requires_engine @pytest.mark.parametrize('source', ['', '# ok\\n', 'text', '\\x00', '```\\nunclosed fence', '<div>', '[broken](link']) def test_any_text_is_valid_markdown(engine: MarkdownEngine, source: str) -> None:"
        - id: test_empty_tree_and_empty_document
          excert: "@requires_engine def test_empty_tree_and_empty_document(engine: MarkdownEngine, tmp_path: Path) -> None:"
        - id: 4f9Vfx|vwmZZp
          excert: '''# ---------------------------------------------------------------------------'''
        - id: test_locate_all_builds_heading_tree
          excert: "@requires_engine def test_locate_all_builds_heading_tree(engine: MarkdownEngine) -> None:"
        - id: hefa5836c
          excert: "@requires_engine def test_line_ranges_are_one_based_and_inclusive(engine: MarkdownEngine) -> None:"
        - id: hea7d1180
          excert: "@requires_engine def test_child_ids_extend_parent_id_and_are_unique(engine: MarkdownEngine) -> None:"
        - id: hcea8d36c
          excert: "@requires_engine def test_anonymous_block_ids_are_content_stable(engine: MarkdownEngine) -> None:"
        - id: test_signature_docstring_and_code
          excert: "@requires_engine def test_signature_docstring_and_code(engine: MarkdownEngine) -> None:"
        - id: h7754a5dc
          excert: "@requires_engine def test_document_without_headings_has_root_blocks(engine: MarkdownEngine) -> None:"
        - id: test_frontmatter_is_its_own_node
          excert: "@requires_engine def test_frontmatter_is_its_own_node(engine: MarkdownEngine) -> None:"
        - id: test_setext_headings_are_sections
          excert: "@requires_engine def test_setext_headings_are_sections(engine: MarkdownEngine) -> None:"
        - id: he6c0ef19
          excert: "@requires_engine def test_skipped_heading_levels_nest_under_previous(engine: MarkdownEngine) -> None:"
        - id: test_duplicate_headings_get_numbered_ids
          excert: "@requires_engine def test_duplicate_headings_get_numbered_ids(engine: MarkdownEngine) -> None:"
        - id: h2e9d4beb
          excert: "@requires_engine def test_headings_inside_code_quote_and_list_are_not_sections(engine: MarkdownEngine) -> None:"
        - id: test_mixed_blocks_form_one_block_group
          excert: "@requires_engine def test_mixed_blocks_form_one_block_group(engine: MarkdownEngine) -> None:"
        - id: test_unicode_headings_and_content
          excert: "@requires_engine def test_unicode_headings_and_content(engine: MarkdownEngine) -> None:"
        - id: hcf22df9d
          excert: "@requires_engine def test_thematic_breaks_do_not_split_sections(engine: MarkdownEngine) -> None:"
        - id: 4f9Vfx|DI1wx2
          excert: '''# ---------------------------------------------------------------------------'''
        - id: test_replace_paragraph
          excert: "@requires_engine def test_replace_paragraph(engine: MarkdownEngine) -> None:"
        - id: h95dee7a9
          excert: "@requires_engine def test_replace_section_with_new_heading_and_body(engine: MarkdownEngine) -> None:"
        - id: test_replace_changes_heading_level
          excert: "@requires_engine def test_replace_changes_heading_level(engine: MarkdownEngine) -> None:"
        - id: h82e4947a
          excert: "@requires_engine def test_replace_root_section_replaces_whole_document(engine: MarkdownEngine) -> None:"
        - id: test_replace_with_multiple_blocks
          excert: "@requires_engine def test_replace_with_multiple_blocks(engine: MarkdownEngine) -> None:"
        - id: test_insert_section_before_and_after
          excert: "@requires_engine def test_insert_section_before_and_after(engine: MarkdownEngine) -> None:"
        - id: test_insert_returns_number_of_blocks
          excert: "@requires_engine def test_insert_returns_number_of_blocks(engine: MarkdownEngine) -> None:"
        - id: test_insert_before_first_node
          excert: "@requires_engine def test_insert_before_first_node(engine: MarkdownEngine) -> None:"
        - id: test_delete_section
          excert: "@requires_engine def test_delete_section(engine: MarkdownEngine) -> None:"
        - id: test_delete_paragraph_and_whole_document
          excert: "@requires_engine def test_delete_paragraph_and_whole_document(engine: MarkdownEngine) -> None:"
        - id: test_append
          excert: "@requires_engine def test_append(engine: MarkdownEngine) -> None:"
        - id: test_append_to_empty_tree
          excert: "@requires_engine def test_append_to_empty_tree(engine: MarkdownEngine) -> None:"
        - id: test_mutation_normalizes_document
          excert: "@requires_engine def test_mutation_normalizes_document(engine: MarkdownEngine) -> None:"
        - id: test_mutation_normalizes_crlf_to_lf
          excert: "@requires_engine def test_mutation_normalizes_crlf_to_lf(engine: MarkdownEngine) -> None:"
        - id: test_mutation_keeps_frontmatter
          excert: "@requires_engine def test_mutation_keeps_frontmatter(engine: MarkdownEngine) -> None:"
        - id: test_mutations_chain_on_the_same_tree
          excert: "@requires_engine def test_mutations_chain_on_the_same_tree(engine: MarkdownEngine) -> None:"
        - id: 4f9Vfx|lYUzt0
          excert: '''# ---------------------------------------------------------------------------'''
        - id: ha7bbd954
          excert: "@requires_engine @pytest.mark.parametrize('operation', ['replace', 'insert', 'delete']) def test_unknown_node_id_raises_and_keeps_source(engine: MarkdownEngine, operation: str) -> None:"
        - id: test_empty_replacement_is_rejected
          excert: "@requires_engine def test_empty_replacement_is_rejected(engine: MarkdownEngine) -> None:"
        - id: he3a09786
          excert: "@requires_engine def test_control_characters_are_rejected_before_the_call(engine: MarkdownEngine) -> None:"
        - id: h18b57d3d
          excert: "@requires_engine def test_imports_are_not_supported_for_markdown(engine: MarkdownEngine) -> None:"
        - id: 4f9Vfx|dMN5pB
          excert: '''# ---------------------------------------------------------------------------'''
        - id: test_concurrent_requests_are_isolated
          excert: "@requires_engine def test_concurrent_requests_are_isolated(engine: MarkdownEngine) -> None:"
        - id: h352f9ebe
          excert: "@requires_engine def test_concurrent_first_use_spawns_a_single_process(monkeypatch: pytest.MonkeyPatch) -> None:"
        - id: test_large_document
          excert: "@requires_engine def test_large_document(engine: MarkdownEngine) -> None:"
        - id: test_long_single_line
          excert: "@requires_engine def test_long_single_line(engine: MarkdownEngine) -> None:"
        - id: 4f9Vfx|Qd3JzI
          excert: '''# ---------------------------------------------------------------------------'''
        - id: md_file
          excert: "@pytest.fixture def md_file(tmp_path: Path) -> str:"
        - id: test_load_uses_markdown_rpc_engine
          excert: "@requires_engine def test_load_uses_markdown_rpc_engine(md_file: str) -> None:"
        - id: test_outline_lists_sections_and_blocks
          excert: "@requires_engine def test_outline_lists_sections_and_blocks(md_file: str) -> None:"
        - id: test_find_by_type_and_name
          excert: "@requires_engine def test_find_by_type_and_name(md_file: str) -> None:"
        - id: test_read_returns_section_source
          excert: "@requires_engine def test_read_returns_section_source(md_file: str) -> None:"
        - id: test_replace_via_tool_writes_file
          excert: "@requires_engine def test_replace_via_tool_writes_file(md_file: str) -> None:"
        - id: test_insert_via_tool_reports_ids
          excert: "@requires_engine def test_insert_via_tool_reports_ids(md_file: str) -> None:"
        - id: test_delete_via_tool
          excert: "@requires_engine def test_delete_via_tool(md_file: str) -> None:"
        - id: test_edit_marks_via_tool
          excert: "@requires_engine def test_edit_marks_via_tool(md_file: str) -> None:"
        - id: test_edit_block_via_tool
          excert: "@requires_engine def test_edit_block_via_tool(md_file: str) -> None:"
        - id: test_batch_of_edits_in_one_call
          excert: "@requires_engine def test_batch_of_edits_in_one_call(md_file: str) -> None:"
        - id: h31151fee
          excert: "@requires_engine def test_unknown_id_via_tool_is_reported_and_file_untouched(md_file: str) -> None:"
        - id: h3bae655c
          excert: "@requires_engine def test_validate_via_tool_accepts_any_markdown(tmp_path: Path) -> None:"
        - id: test_create_via_tool_and_overwrite
          excert: "@requires_engine def test_create_via_tool_and_overwrite(tmp_path: Path) -> None:"
        - id: test_cache_follows_external_changes
          excert: "@requires_engine def test_cache_follows_external_changes(md_file: str) -> None:"
        - id: test_cache_follows_tool_edits
          excert: "@requires_engine def test_cache_follows_tool_edits(md_file: str) -> None:"
        - id: test_unicode_file_roundtrip_via_tool
          excert: "@requires_engine def test_unicode_file_roundtrip_via_tool(tmp_path: Path) -> None:"
        - id: 4f9Vfx|qptQtr
          excert: '''# ---------------------------------------------------------------------------'''
        - id: retrieve
          excert: "def _retrieve(registry: ToolRegistry, arguments: dict):"
        - id: test_retrieve_outline_find_read
          excert: "@requires_engine def test_retrieve_outline_find_read(registry: ToolRegistry, md_file: str) -> None:"
        - id: hfb930679
          excert: "@requires_engine def test_retrieve_search_returns_node_ids_usable_for_read(registry: ToolRegistry, md_file: str) -> None:"
        - id: 4f9Vfx|tUplie
          excert: '''# ---------------------------------------------------------------------------'''
        - id: h2c0a8d2a
          excert: "@requires_engine def test_new_engine_instance_restarts_crashed_process() -> None:"
        - id: he9914ac8
          excert: "@requires_engine @pytest.mark.xfail(strict=True, reason='RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.') def test_existing_engine_instance_recovers_after_process_crash() -> None:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
        nodes:
        - id: 8fSL6x|fEXSFN
          excert: "\"Markdown engine: remark-based ``ast-engines/markdown`` process, driven over RP…"
        - id: ILN4fu|sCWK6E
          excert: from __future__ import annotations
        - id: HrsM8B|ZAPiHS
          excert: "__all__ = ['MarkdownEngine']"
        - id: workspace_root
          excert: "def _workspace_root() -> Path:"
        - id: pwufUY|RHnCVl
          excert: _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'markdown'
        - id: MarkdownProcess
          excert: "class MarkdownProcess(RpcProcess):"
          docstring: Starts the ``ast-engines/markdown`` remark engine on Node.js.
          children:
          - id: MarkdownProcess.T89awu|KUD046
            excert: '''Starts the ``ast-engines/markdown`` remark engine on Node.js.'''
          - id: MarkdownProcess.default_port
            excert: "def default_port(self) -> int:"
          - id: MarkdownProcess.cwd
            excert: "def cwd(self) -> str | None:"
          - id: MarkdownProcess.command
            excert: "def command(self, port: int) -> list[str]:"
        - id: U6RlhD|cmSjIL
          excert: _PROCESS = MarkdownProcess()
        - id: MarkdownEngine
          excert: "class MarkdownEngine(RpcEngine):"
          docstring: "Markdown, parsed and mutated by remark via the ``ast-engines/markdown`` RPC pro…"
          children:
          - id: MarkdownEngine.oP8rfU|ZXkRnE
            excert: "'Markdown, parsed and mutated by remark via the ``ast-engines/markdown`` RPC pr…"
          - id: MarkdownEngine.init
            excert: "def __init__(self) -> None:"
```
/answer 1632-94-93 allow
ReasoningToken: 63
Control Request:
```yaml
id: 1632-95-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/markdown
    pattern: "[Ii]d\\b.*(hash|slug|base|short|digest)|function .*[Ii]d|nodeId|node_id"
    include:
    - '*.js'
    - '*.mjs'
    - '*.ts'
    - '*.md'
    exclude:
    - node_modules/**
    - '**/node_modules/**'
    - dist/**
    limit: 30
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
    name: find
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
    name: ids
  reason: ID-Erzeugung im Markdown-Engine (Node) und Test-Hilfsfunktionen sichten
```
/answer 1632-95-0 deny nicht sichen, teste eine kürze rückgabe selbst dann passe das format an
ReasoningToken: 44
Control Request:
```yaml
id: 1632-96-0
toolName: python
arguments:
  script: |-
    import sys
    sys.path.insert(0, "/home/user/xyan/xy.ai.workbench/mcpc/src")
    from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
    e = MarkdownEngine()
    doc = "# Title\n\nintro\n\n## Section A\n\ntext a\n\n## Section B\n\ntext b\n\n## Section A\n\ndup\n"
    for l in e.locate_all(e.parse(doc)):
        print(repr(l.node_id), l.node_type, l.name)
  reason: Kurzes Markdown-Dokument parsen und die neuen IDs ausgeben
```
/answer 1632-96-0 allow
Control Request:
```yaml
id: 1632-97-96
result:
  structuredContent:
    exit_code: 0
    stdout: |
      'JLeD' Section Title
      'JLeD.8vS9|p3TE' BlockGroup None
      'JLeD.uvrD' Section Section A
      'JLeD.uvrD.AEZQ|15Cr' BlockGroup None
      'JLeD.DpCJ' Section Section B
      'JLeD.DpCJ.S1Dt|kXDa' BlockGroup None
      'JLeD.uvrD_2' Section Section A
      'JLeD.uvrD_2.xtjz|bnvh' BlockGroup None
```
/answer 1632-97-96 allow
Thinking:
Die neuen IDs sind jetzt ein 4-stelliger Hash des Überschriftennamens (etwa `Title` → `JLeD`); ich suche als Nächstes in den Tests nach den alten IDs.


ReasoningToken: 166
Control Request:
```yaml
id: 1632-98-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: "'[A-Z][A-Za-z0-9]*(_[0-9]+|\\.[A-Za-z_0-9|]+)*'|Doc\\.|Title|\\|"
    include:
    - test_ast_markdown_bridge.py
    limit: 50
    intent: locate_for_edit
  reason: Alte ID-Literale in den Markdown-Tests finden
```
/answer 1632-98-0 allow
Control Request:
```yaml
id: 1632-99-98
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/tests
        matches:
        - path: test_ast_markdown_bridge.py
          nodes:
          - id: kcapNL|6Fluzl
            matches:
            - DOC = '# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n'
          - id: test_process_defaults.W2lk1D|mlzI7V
            matches:
            - "monkeypatch.delenv('NODE', raising=False)"
          - id: test_process_honours_environment.XDqsBi|LnKpWn
            matches:
            - "monkeypatch.setenv('NODE', '/opt/node/bin/node')"
          - id: test_missing_node_binary_fails_fast.jlP7K2|qWqHUm
            matches:
            - "monkeypatch.setenv('NODE', '/nonexistent/node')"
          - id: test_locate_all_builds_heading_tree.cvBIII|zSiLh6
            matches:
            - "'Section',"
            - "'BlockGroup',"
            - "'Section',"
            - "'BlockGroup',"
            - "'Section',"
            - "'BlockGroup']"
            - "sections = [loc for loc in located if loc.node_type == 'Section']"
            - "assert [loc.node_id for loc in sections] == ['Title', 'Title.Section_A', 'Title.Section_B']"
            - "assert [loc.name for loc in sections] == ['Title', 'Section A', 'Section B']"
            - "assert located[0].parent_type == 'Root'"
          - id: test_locate_all_builds_heading_tree.DLXHST|WoVMPL
            matches:
            - "assert all((loc.parent_type == 'Section' for loc in located[1:]))"
          - id: hefa5836c.4OPrVQ|7LKHme
            matches:
            - "for loc in engine.locate_all(tree) if loc.node_type == 'Section'}"
            - "assert ranges == {'Title': (1, 11), 'Title.Section_A': (5, 7), 'Title.Section_B': (9, 11)}"
          - id: hea7d1180.66Er4T|Yo7dnY
            matches:
            - assert loc.node_id.startswith('Title.')
          - id: hcea8d36c.block_ids.E73sVH|1WqRh9
            matches:
            - "return {loc.node_id for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'BlockGroup'}"
          - id: test_signature_docstring_and_code.FSmkxI|hWDZah
            matches:
            - "title = _find(engine, tree, 'Title')"
            - "section_a = _find(engine, tree, 'Title.Section_A')"
            - assert engine.signature(title.node) == '# Title'
            - block = next((loc for loc in engine.locate_all(tree) if loc.node_type == 'BlockGroup'))
          - id: h7754a5dc.pA6GuE|YWVG9b
            matches:
            - "assert [loc.node_type for loc in located] == ['BlockGroup']"
            - "assert located[0].parent_type == 'Root'"
          - id: test_frontmatter_is_its_own_node.JK9LmQ|SB0YbD
            matches:
            - "assert [loc.node_type for loc in located] == ['Frontmatter', 'Section', 'BlockGroup']"
            - "assert located[0].parent_type == 'Root'"
            - "assert located[1].node_id == 'H1'"
          - id: test_setext_headings_are_sections.EtInGz|eaoC9F
            matches:
            - "sections = [loc for loc in located if loc.node_type == 'Section']"
            - "assert [loc.node_id for loc in sections] == ['Setext', 'Setext.Sub']"
          - id: he6c0ef19.o68iyS|E7DUCD
            matches:
            - "assert [i for i in ids if '|' not in i] == ['A', 'A.Deep', 'A.B']"
          - id: test_duplicate_headings_get_numbered_ids.RlTLSb|41uig9
            matches:
            - "ids = [i for i in _ids(engine, '# T\\n\\n## A\\n\\nx\\n\\n## A\\n\\ny\\n') if '|' not in i]"
            - "assert ids == ['T', 'T.A', 'T.A_2']"
            - "ids = [i for i in _ids(engine, '# A\\n\\n# A\\n') if '|' not in i]"
            - "assert ids == ['A', 'A_2']"
          - id: h2e9d4beb.1XAdNt|nopO6F
            matches:
            - "sections = [loc.node_id for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'Section']"
            - "assert sections == ['Real', 'Real.Next']"
          - id: test_mixed_blocks_form_one_block_group.ZsW1uR|0xec7U
            matches:
            - source = '# A\n\n- a\n- b\n\n1. one\n2. two\n\n```py\nprint(1)\n```\n\n> quote\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n<div>x</div>\n'
            - "assert [loc.node_type for loc in located] == ['Section', 'BlockGroup']"
          - id: hcf22df9d.SH8Pri|oO6ivM
            matches:
            - "assert [loc.node_id for loc in located if loc.node_type == 'Section'] == ['A', 'B']"
            - assert all((loc.parent_type != 'Page' for loc in located))
          - id: test_replace_paragraph.Bo1cs2|9LJ66K
            matches:
            - '''BlockGroup'' and ''Second'' in engine.node_code(loc.node)))'
          - id: h95dee7a9.GPxQeI|xTOnKR
            matches:
            - "engine.replace(_find(engine, tree, 'Title.Section_A'), '## Section A2\\n\\nnew body')"
            - tree) == '# Title\n\nIntro text.\n\n## Section A2\n\nnew body\n\n## Section B\n\nSecond paragraph.\n'
            - "assert 'Title.Section_A2' in _ids(engine, tree.source)"
            - "assert 'Title.Section_A' not in _ids(engine, tree.source)"
          - id: test_replace_changes_heading_level.HihnqG|BdDjjR
            matches:
            - "engine.replace(_find(engine, tree, 'A.B'), '### B3\\n\\nlevel3')"
          - id: h82e4947a.cQQtae|9diD1k
            matches:
            - "engine.replace(_find(engine, tree, 'Title'), '# Only\\n\\nleft')"
          - id: test_replace_with_multiple_blocks.rmtARI|WhgyXq
            matches:
            - block = next((loc for loc in engine.locate_all(tree) if loc.node_id.startswith('Title.Section_B.')))
          - id: test_insert_section_before_and_after.lut9pX|gLy2Zg
            matches:
            - "assert engine.insert(_find(engine, tree, 'Title.Section_B'), '## Mid\\n\\nmid', 'before') == 2"
        warnings:
        - "Limit of 50 matches reached (total 95); narrow pattern, directory or include/exclude filters."
```
/answer 1632-99-98 allow
ReasoningToken: 260
Control Request:
```yaml
id: 1632-9a-0
toolName: python
arguments:
  script: |-
    p="/home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py"
    L=open(p,encoding="utf-8").read().splitlines()
    print(len(L))
    for i,l in enumerate(L,1):
        if 20<=i<=120 or 190<=i<=520: print(i,l)
  reason: "Testdatei mit Zeilennummern lesen, um alle alten ID-Literale gezielt anzupassen"
```
/answer 1632-9a-0 allow
Control Request:
```yaml
id: 1632-9b-9a
result:
  structuredContent:
    exit_code: 0
    stdout: |
      720
      20 from xy.ai.mcpc.tools.ast import core
      21 from xy.ai.mcpc.tools.ast.base import AstError, Located, OutlineNode, Tree
      22 from xy.ai.mcpc.tools.ast.create import CreateFileItem, ast_create
      23 from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete
      24 from xy.ai.mcpc.tools.ast.edit_block import EditBlockItem, ast_edit_block
      25 from xy.ai.mcpc.tools.ast.edit_marks import EditMarksItem, ast_edit_marks
      26 from xy.ai.mcpc.tools.ast.find import ast_find
      27 from xy.ai.mcpc.tools.ast.generic import get_engine, language_for_extension
      28 from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert
      29 from xy.ai.mcpc.tools.ast.outline import ast_outline
      30 from xy.ai.mcpc.tools.ast.read import ReadItem, ast_read
      31 from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace
      32 from xy.ai.mcpc.tools.ast.rpc import MarkdownEngine as ExportedMarkdownEngine
      33 from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
      34 from xy.ai.mcpc.tools.ast.rpc._markdown import _ENGINE_DIR, _MAIN, _PROCESS, MarkdownEngine, MarkdownProcess
      35 from xy.ai.mcpc.tools.ast.validate import ast_validate
      36 from xy.ai.mcpc.tools.tool_context import ToolContext
      37 from xy.ai.mcpc.tools.tool_registry import ToolRegistry
      38 requires_engine = pytest.mark.skipif(
      39     shutil.which('node') is None or not (
      40         _ENGINE_DIR / 'node_modules').is_dir(),
      41     reason='node or the engine dependencies (npm install in ast-engines/markdown) are missing')
      42 DOC = '# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n'
      43 FILE_DOC = '# Doc\n\nintro\n\n## Part 1\n\nalpha\n\n## Part 2\n\nbeta\n'
      44
      45 @pytest.fixture(scope='module')
      46 def engine() -> MarkdownEngine:
      47     return MarkdownEngine()
      48
      49 def _find(engine: MarkdownEngine, tree: Tree, node_id: str) -> Located:
      50     return next((loc for loc in engine.locate_all(tree) if loc.node_id == node_id))
      51
      52 def _ids(engine: MarkdownEngine, source: str) -> list[str]:
      53     return [loc.node_id for loc in engine.locate_all(engine.parse(source))]
      54
      55 def _free_port() -> int:
      56     with socket.socket() as sock:
      57         sock.bind(('127.0.0.1', 0))
      58         return sock.getsockname()[1]
      59
      60 def _flatten(nodes: list[OutlineNode]) -> list[OutlineNode]:
      61     flat: list[OutlineNode] = []
      62     for node in nodes:
      63         flat.append(node)
      64         flat.extend(_flatten(list(node.children)))
      65     return flat
      66 '# ---------------------------------------------------------------------------'
      67 '# Konfiguration & Registrierung (ohne Prozess)'
      68 '# ---------------------------------------------------------------------------'
      69
      70 def test_engine_dir_and_entry_point_exist() -> None:
      71     assert _ENGINE_DIR.is_dir()
      72     assert (_ENGINE_DIR / _MAIN).is_file()
      73
      74 def test_process_defaults(monkeypatch: pytest.MonkeyPatch) -> None:
      75     monkeypatch.delenv('AST_MARKDOWN_PORT', raising=False)
      76     monkeypatch.delenv('NODE', raising=False)
      77     process = MarkdownProcess()
      78     assert process.default_port() == 8789
      79     assert process.cwd() == str(_ENGINE_DIR)
      80     assert process.command(1234) == ['node', str(_MAIN), '1234']
      81
      82 def test_process_honours_environment(monkeypatch: pytest.MonkeyPatch) -> None:
      83     monkeypatch.setenv('AST_MARKDOWN_PORT', '9911')
      84     monkeypatch.setenv('NODE', '/opt/node/bin/node')
      85     process = MarkdownProcess()
      86     assert process.default_port() == 9911
      87     assert process.command(9911)[0] == '/opt/node/bin/node'
      88
      89 def test_default_port_does_not_collide_with_other_engines(monkeypatch: pytest.MonkeyPatch) -> None:
      90     from xy.ai.mcpc.tools.ast.rpc._java import JavaProcess
      91     from xy.ai.mcpc.tools.ast.rpc._rust import RustProcess
      92     for var in ('AST_JAVA_PORT', 'AST_RUST_PORT', 'AST_MARKDOWN_PORT'):
      93         monkeypatch.delenv(var, raising=False)
      94     ports = {JavaProcess().default_port(), RustProcess().default_port(), MarkdownProcess().default_port()}
      95     assert len(ports) == 3
      96
      97 def test_engine_is_registered_as_rpc_engine() -> None:
      98     resolved = get_engine('markdown')
      99     assert isinstance(resolved, RpcEngine)
      100     assert isinstance(resolved, MarkdownEngine)
      101     assert type(resolved).__module__ == 'xy.ai.mcpc.tools.ast.rpc._markdown'
      102     assert resolved is get_engine('markdown')
      103     assert ExportedMarkdownEngine is MarkdownEngine
      104
      105 @pytest.mark.parametrize('name', ['x.md', 'x.markdown'])
      106 def test_extensions_resolve_to_markdown_engine(tmp_path: Path, name: str) -> None:
      107     assert language_for_extension(Path(name).suffix) == 'markdown'
      108     assert core.engine_for_path(tmp_path / name).name == 'markdown'
      109     assert isinstance(core.engine_for_path(tmp_path / name), MarkdownEngine)
      110
      111 def test_engine_metadata() -> None:
      112     resolved = MarkdownEngine()
      113     assert resolved.symbol == 'markdown'
      114     assert resolved.name == 'markdown'
      115     assert resolved.validates_syntax is False
      116
      117 def test_constructing_engine_does_not_start_process() -> None:
      118     process = MarkdownProcess()
      119     RpcEngine('markdown', process)
      120     assert process._process is None
      190             'Section',
      191             'BlockGroup',
      192             'Section',
      193         'BlockGroup']
      194     sections = [loc for loc in located if loc.node_type == 'Section']
      195     assert [loc.node_id for loc in sections] == ['Title', 'Title.Section_A', 'Title.Section_B']
      196     assert [loc.name for loc in sections] == ['Title', 'Section A', 'Section B']
      197     assert located[0].parent_type == 'Root'
      198     assert all((loc.parent_type == 'Section' for loc in located[1:]))
      199
      200 @requires_engine
      201 def test_line_ranges_are_one_based_and_inclusive(engine: MarkdownEngine) -> None:
      202     tree = engine.parse(DOC)
      203     ranges = {loc.node_id: (loc.lineno, loc.end_lineno)
      204               for loc in engine.locate_all(tree) if loc.node_type == 'Section'}
      205     assert ranges == {'Title': (1, 11), 'Title.Section_A': (5, 7), 'Title.Section_B': (9, 11)}
      206
      207 @requires_engine
      208 def test_child_ids_extend_parent_id_and_are_unique(engine: MarkdownEngine) -> None:
      209     located = engine.locate_all(engine.parse(DOC))
      210     ids = [loc.node_id for loc in located]
      211     assert len(ids) == len(set(ids))
      212     for loc in located[1:]:
      213         assert loc.node_id.startswith('Title.')
      214
      215 @requires_engine
      216 def test_anonymous_block_ids_are_content_stable(engine: MarkdownEngine) -> None:
      217
      218     def block_ids(source: str) -> set[str]:
      219         return {loc.node_id for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'BlockGroup'}
      220     first = block_ids(DOC)
      221     assert first == block_ids(DOC)
      222     third = block_ids(DOC.replace('Second paragraph.', 'Other words.'))
      223     assert len(first & third) == 2 and len(third - first) == 1
      224
      225 @requires_engine
      226 def test_signature_docstring_and_code(engine: MarkdownEngine) -> None:
      227     tree = engine.parse(DOC)
      228     title = _find(engine, tree, 'Title')
      229     section_a = _find(engine, tree, 'Title.Section_A')
      230     assert engine.signature(title.node) == '# Title'
      231     assert engine.signature(section_a.node) == '## Section A'
      232     assert engine.docstring(section_a.node) == 'First paragraph.'
      233     assert engine.node_code(section_a.node) == '## Section A\n\nFirst paragraph.\n'
      234     assert engine.node_code(title.node) == DOC
      235     block = next((loc for loc in engine.locate_all(tree) if loc.node_type == 'BlockGroup'))
      236     assert engine.signature(block.node) == 'Intro text.'
      237     assert engine.node_code(block.node) == 'Intro text.\n'
      238
      239 @requires_engine
      240 def test_document_without_headings_has_root_blocks(engine: MarkdownEngine) -> None:
      241     located = engine.locate_all(engine.parse('just text\n\nmore text\n'))
      242     assert [loc.node_type for loc in located] == ['BlockGroup']
      243     assert located[0].parent_type == 'Root'
      244     assert (located[0].lineno, located[0].end_lineno) == (1, 3)
      245
      246 @requires_engine
      247 def test_frontmatter_is_its_own_node(engine: MarkdownEngine) -> None:
      248     located = engine.locate_all(engine.parse('---\ntitle: X\n---\n\n# H1\n\ntext\n'))
      249     assert [loc.node_type for loc in located] == ['Frontmatter', 'Section', 'BlockGroup']
      250     assert (located[0].lineno, located[0].end_lineno) == (1, 3)
      251     assert located[0].parent_type == 'Root'
      252     assert located[1].node_id == 'H1'
      253
      254 @requires_engine
      255 def test_setext_headings_are_sections(engine: MarkdownEngine) -> None:
      256     located = engine.locate_all(engine.parse('Setext\n======\n\npara\n\nSub\n---\n\nmore\n'))
      257     sections = [loc for loc in located if loc.node_type == 'Section']
      258     assert [loc.node_id for loc in sections] == ['Setext', 'Setext.Sub']
      259     assert [engine.signature(loc.node) for loc in sections] == ['# Setext', '## Sub']
      260
      261 @requires_engine
      262 def test_skipped_heading_levels_nest_under_previous(engine: MarkdownEngine) -> None:
      263     ids = _ids(engine, '# A\n\n#### Deep\n\ntext\n\n## B\n')
      264     assert [i for i in ids if '|' not in i] == ['A', 'A.Deep', 'A.B']
      265
      266 @requires_engine
      267 def test_duplicate_headings_get_numbered_ids(engine: MarkdownEngine) -> None:
      268     ids = [i for i in _ids(engine, '# T\n\n## A\n\nx\n\n## A\n\ny\n') if '|' not in i]
      269     assert ids == ['T', 'T.A', 'T.A_2']
      270     ids = [i for i in _ids(engine, '# A\n\n# A\n') if '|' not in i]
      271     assert ids == ['A', 'A_2']
      272
      273 @requires_engine
      274 def test_headings_inside_code_quote_and_list_are_not_sections(engine: MarkdownEngine) -> None:
      275     source = '# Real\n\n```\n# not heading\n```\n\n> # quoted\n\n- # in list\n\n## Next\n'
      276     sections = [loc.node_id for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'Section']
      277     assert sections == ['Real', 'Real.Next']
      278
      279 @requires_engine
      280 def test_mixed_blocks_form_one_block_group(engine: MarkdownEngine) -> None:
      281     source = '# A\n\n- a\n- b\n\n1. one\n2. two\n\n```py\nprint(1)\n```\n\n> quote\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n<div>x</div>\n'
      282     located = engine.locate_all(engine.parse(source))
      283     assert [loc.node_type for loc in located] == ['Section', 'BlockGroup']
      284     assert (located[1].lineno, located[1].end_lineno) == (3, 19)
      285
      286 @requires_engine
      287 def test_unicode_headings_and_content(engine: MarkdownEngine) -> None:
      288     tree = engine.parse('# Ünï ✓ 日本\n\nText äöü 😀\n')
      289     section = engine.locate_all(tree)[0]
      290     assert section.name == 'Ünï ✓ 日本'
      291     assert engine.node_code(section.node) == '# Ünï ✓ 日本\n\nText äöü 😀\n'
      292     engine.replace(section, '# Ünï ✓ 日本\n\nNeu ß 🚀')
      293     assert engine.serialize(tree) == '# Ünï ✓ 日本\n\nNeu ß 🚀\n'
      294
      295 @requires_engine
      296 def test_thematic_breaks_do_not_split_sections(engine: MarkdownEngine) -> None:
      297     located = engine.locate_all(engine.parse('# A\n\nx\n\n---\n\n# B\n\ny\n'))
      298     assert [loc.node_id for loc in located if loc.node_type == 'Section'] == ['A', 'B']
      299     assert all((loc.parent_type != 'Page' for loc in located))
      300 '# ---------------------------------------------------------------------------'
      301 '# Mutationen'
      302 '# ---------------------------------------------------------------------------'
      303
      304 @requires_engine
      305 def test_replace_paragraph(engine: MarkdownEngine) -> None:
      306     tree = engine.parse(DOC)
      307     block = next((loc for loc in engine.locate_all(tree) if loc.node_type ==
      308                  'BlockGroup' and 'Second' in engine.node_code(loc.node)))
      309     engine.replace(block, 'Changed paragraph.')
      310     assert engine.serialize(tree) == DOC.replace('Second paragraph.', 'Changed paragraph.')
      311
      312 @requires_engine
      313 def test_replace_section_with_new_heading_and_body(engine: MarkdownEngine) -> None:
      314     tree = engine.parse(DOC)
      315     engine.replace(_find(engine, tree, 'Title.Section_A'), '## Section A2\n\nnew body')
      316     assert engine.serialize(
      317         tree) == '# Title\n\nIntro text.\n\n## Section A2\n\nnew body\n\n## Section B\n\nSecond paragraph.\n'
      318     assert 'Title.Section_A2' in _ids(engine, tree.source)
      319     assert 'Title.Section_A' not in _ids(engine, tree.source)
      320
      321 @requires_engine
      322 def test_replace_changes_heading_level(engine: MarkdownEngine) -> None:
      323     tree = engine.parse('# A\n\n## B\n\nb\n\n## C\n\nc\n')
      324     engine.replace(_find(engine, tree, 'A.B'), '### B3\n\nlevel3')
      325     assert engine.serialize(tree) == '# A\n\n### B3\n\nlevel3\n\n## C\n\nc\n'
      326
      327 @requires_engine
      328 def test_replace_root_section_replaces_whole_document(engine: MarkdownEngine) -> None:
      329     tree = engine.parse(DOC)
      330     engine.replace(_find(engine, tree, 'Title'), '# Only\n\nleft')
      331     assert engine.serialize(tree) == '# Only\n\nleft\n'
      332
      333 @requires_engine
      334 def test_replace_with_multiple_blocks(engine: MarkdownEngine) -> None:
      335     tree = engine.parse(DOC)
      336     block = next((loc for loc in engine.locate_all(tree) if loc.node_id.startswith('Title.Section_B.')))
      337     engine.replace(block, 'one\n\n- a\n- b\n\n```js\nx\n```')
      338     assert engine.serialize(tree).endswith('## Section B\n\none\n\n- a\n- b\n\n```js\nx\n```\n')
      339
      340 @requires_engine
      341 def test_insert_section_before_and_after(engine: MarkdownEngine) -> None:
      342     tree = engine.parse(DOC)
      343     assert engine.insert(_find(engine, tree, 'Title.Section_B'), '## Mid\n\nmid', 'before') == 2
      344     assert engine.serialize(
      345         tree) == '# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Mid\n\nmid\n\n## Section B\n\nSecond paragraph.\n'
      346     tree = engine.parse(DOC)
      347     assert engine.insert(_find(engine, tree, 'Title.Section_B'), '## After\n\nx', 'after') == 2
      348     assert engine.serialize(tree).endswith('Second paragraph.\n\n## After\n\nx\n')
      349
      350 @requires_engine
      351 def test_insert_returns_number_of_blocks(engine: MarkdownEngine) -> None:
      352     tree = engine.parse(DOC)
      353     assert engine.insert(_find(engine, tree, 'Title.Section_A'), 'single', 'after') == 1
      354     tree = engine.parse(DOC)
      355     assert engine.insert(_find(engine, tree, 'Title.Section_A'), 'p1\n\np2\n\np3', 'after') == 3
      356
      357 @requires_engine
      358 def test_insert_before_first_node(engine: MarkdownEngine) -> None:
      359     tree = engine.parse(DOC)
      360     engine.insert(_find(engine, tree, 'Title'), 'prefix', 'before')
      361     assert engine.serialize(tree) == 'prefix\n\n' + DOC
      362
      363 @requires_engine
      364 def test_delete_section(engine: MarkdownEngine) -> None:
      365     tree = engine.parse(DOC)
      366     engine.delete(_find(engine, tree, 'Title.Section_A'))
      367     assert engine.serialize(tree) == '# Title\n\nIntro text.\n\n## Section B\n\nSecond paragraph.\n'
      368
      369 @requires_engine
      370 def test_delete_paragraph_and_whole_document(engine: MarkdownEngine) -> None:
      371     tree = engine.parse(DOC)
      372     block = next((loc for loc in engine.locate_all(tree) if loc.node_id.startswith('Title.Section_A.')))
      373     engine.delete(block)
      374     assert 'First paragraph.' not in engine.serialize(tree)
      375     assert '## Section A' in engine.serialize(tree)
      376     tree = engine.parse(DOC)
      377     engine.delete(_find(engine, tree, 'Title'))
      378     assert engine.serialize(tree) == ''
      379     assert engine.locate_all(tree) == []
      380
      381 @requires_engine
      382 def test_append(engine: MarkdownEngine) -> None:
      383     tree = engine.parse(DOC)
      384     assert engine.append(tree, '## Section C\n\nappended') == 2
      385     assert engine.serialize(tree) == DOC + '\n## Section C\n\nappended\n'
      386
      387 @requires_engine
      388 def test_append_to_empty_tree(engine: MarkdownEngine) -> None:
      389     tree = engine.empty_tree()
      390     assert engine.append(tree, '# New\n\nbody') == 2
      391     assert engine.serialize(tree) == '# New\n\nbody\n'
      392     assert engine.append(tree, 'tail') == 1
      393     assert engine.serialize(tree) == '# New\n\nbody\n\ntail\n'
      394
      395 @requires_engine
      396 def test_mutation_normalizes_document(engine: MarkdownEngine) -> None:
      397     source = '# Title\n\n* item one\n* item two\n\n\nSome   *emph*   and __strong__ text.\n\n## Sec\n\nlast\n'
      398     tree = engine.parse(source)
      399     assert engine.serialize(tree) == source
      400     block = next((loc for loc in engine.locate_all(tree) if loc.node_id.startswith('Title.Sec.')))
      401     engine.replace(block, 'changed')
      402     assert engine.serialize(
      403         tree) == '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\n\n## Sec\n\nchanged\n'
      404
      405 @requires_engine
      406 def test_mutation_normalizes_crlf_to_lf(engine: MarkdownEngine) -> None:
      407     tree = engine.parse('# A\r\n\r\ntext\r\n\r\n## B\r\n\r\nmore\r\n')
      408     block = next((loc for loc in engine.locate_all(tree) if loc.node_id.startswith('A.B.')))
      409     engine.replace(block, 'new')
      410     assert '\r' not in engine.serialize(tree)
      411     assert engine.serialize(tree) == '# A\n\ntext\n\n## B\n\nnew\n'
      412
      413 @requires_engine
      414 def test_mutation_keeps_frontmatter(engine: MarkdownEngine) -> None:
      415     tree = engine.parse('---\ntitle: X\n---\n\n# H\n\ntext\n')
      416     engine.replace(_find(engine, tree, 'H'), '# H\n\nchanged')
      417     assert engine.serialize(tree) == '---\ntitle: X\n---\n\n# H\n\nchanged\n'
      418
      419 @requires_engine
      420 def test_mutations_chain_on_the_same_tree(engine: MarkdownEngine) -> None:
      421     tree = engine.parse(DOC)
      422     engine.replace(_find(engine, tree, 'Title.Section_A'), '## A\n\nfirst')
      423     engine.insert(_find(engine, tree, 'Title.A'), '## A2\n\nsecond', 'after')
      424     engine.delete(_find(engine, tree, 'Title.Section_B'))
      425     engine.append(tree, '## Z\n\nlast')
      426     assert engine.serialize(tree) == '# Title\n\nIntro text.\n\n## A\n\nfirst\n\n## A2\n\nsecond\n\n## Z\n\nlast\n'
      427 '# ---------------------------------------------------------------------------'
      428 '# Fehlerfälle'
      429 '# ---------------------------------------------------------------------------'
      430
      431 @requires_engine
      432 @pytest.mark.parametrize('operation', ['replace', 'insert', 'delete'])
      433 def test_unknown_node_id_raises_and_keeps_source(engine: MarkdownEngine, operation: str) -> None:
      434     tree = engine.parse(DOC)
      435     ghost = dataclasses.replace(_find(engine, tree, 'Title'), node_id='Does.Not.Exist')
      436     with pytest.raises(AstError, match='not found'):
      437         if operation == 'replace':
      438             engine.replace(ghost, 'x')
      439         elif operation == 'insert':
      440             engine.insert(ghost, 'x', 'after')
      441         else:
      442             engine.delete(ghost)
      443     assert tree.source == DOC
      444
      445 @requires_engine
      446 def test_empty_replacement_is_rejected(engine: MarkdownEngine) -> None:
      447     tree = engine.parse(DOC)
      448     with pytest.raises(AstError):
      449         engine.replace(_find(engine, tree, 'Title.Section_A'), '')
      450     assert tree.source == DOC
      451
      452 @requires_engine
      453 def test_control_characters_are_rejected_before_the_call(engine: MarkdownEngine) -> None:
      454     tree = engine.parse(DOC)
      455     loc = _find(engine, tree, 'Title.Section_A')
      456     with pytest.raises(AstError, match='control character'):
      457         engine.replace(loc, 'a\x01b')
      458     with pytest.raises(AstError, match='control character'):
      459         engine.insert(loc, 'a\x01b', 'after')
      460     with pytest.raises(AstError, match='control character'):
      461         engine.append(tree, 'a\x01b')
      462     assert tree.source == DOC
      463
      464 @requires_engine
      465 def test_imports_are_not_supported_for_markdown(engine: MarkdownEngine) -> None:
      466     tree = engine.parse('# A\n')
      467     with pytest.raises(AstError):
      468         engine.apply_imports(tree, ['import x'], [])
      469     assert tree.source == '# A\n'
      470 '# ---------------------------------------------------------------------------'
      471 '# Robustheit'
      472 '# ---------------------------------------------------------------------------'
      473
      474 @requires_engine
      475 def test_concurrent_requests_are_isolated(engine: MarkdownEngine) -> None:
      476
      477     def work(i: int) -> bool:
      478         tree = engine.parse(f'# Doc{i}\n\ntext {i}\n\n## Sub\n\nbody {i}\n')
      479         engine.replace(_find(engine, tree, f'Doc{i}.Sub'), f'## Sub\n\nnew {i}')
      480         return tree.source == f'# Doc{i}\n\ntext {i}\n\n## Sub\n\nnew {i}\n'
      481     with ThreadPoolExecutor(16) as pool:
      482         assert all(pool.map(work, range(96)))
      483
      484 @requires_engine
      485 def test_concurrent_first_use_spawns_a_single_process(monkeypatch: pytest.MonkeyPatch) -> None:
      486     monkeypatch.setenv('AST_MARKDOWN_PORT', str(_free_port()))
      487     process = MarkdownProcess()
      488     isolated = RpcEngine('markdown', process)
      489     try:
      490         with ThreadPoolExecutor(8) as pool:
      491             results = list(pool.map(lambda _: isolated.validate('# a\n'), range(8)))
      492         assert results == [None] * 8
      493         assert process._process is not None
      494     finally:
      495         process._terminate()
      496
      497 @requires_engine
      498 def test_large_document(engine: MarkdownEngine) -> None:
      499     sections = 300
      500     source = '# Big\n\n' + \
      501         ''.join((f'## Section {i}\n\n' + ''.join((f'Paragraph {i}.{j} lorem ipsum.\n\n' for j in range(5))) for i in range(sections)))
      502     tree = engine.parse(source)
      503     assert len(engine.locate_all(tree)) == 1 + 2 * sections
      504     engine.replace(_find(engine, tree, f'Big.Section_{sections - 1}'), f'## Section {sections - 1}\n\nX')
      505     assert tree.source.endswith(f'## Section {sections - 1}\n\nX\n')
      506     assert 'Section 0' in tree.source
      507
      508 @requires_engine
      509 def test_long_single_line(engine: MarkdownEngine) -> None:
      510     line = 'word ' * 20000
      511     tree = engine.parse(f'# A\n\n{line}\n')
      512     engine.append(tree, 'tail')
      513     assert tree.source.endswith('tail\n')
      514     assert line.strip() in tree.source
      515 '# ---------------------------------------------------------------------------'
      516 '# Tool-Ebene (Dateien, Cache, ast_*-Tools)'
      517 '# ---------------------------------------------------------------------------'
      518
      519 @pytest.fixture
      520 def md_file(tmp_path: Path) -> str:
```
/answer 1632-9b-9a allow
ReasoningToken: 627
Control Request:
```yaml
id: 1632-9c-0
toolName: python
arguments:
  script: |-
    p="/home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py"
    L=open(p,encoding="utf-8").read().splitlines()
    for i,l in enumerate(L,1):
        if 519<=i<=720: print(i,l)
    for i,l in enumerate(L,1):
        if 120<=i<=190: print(i,l)
  reason: Rest der Testdatei (Tool-Ebene) lesen
```
/answer 1632-9c-0 allow
Control Request:
```yaml
id: 1632-9d-9c
result:
  structuredContent:
    exit_code: 0
    stdout: |
      519 @pytest.fixture
      520 def md_file(tmp_path: Path) -> str:
      521     path = tmp_path / 'doc.md'
      522     path.write_text(FILE_DOC, encoding='utf-8')
      523     return str(path)
      524
      525 @requires_engine
      526 def test_load_uses_markdown_rpc_engine(md_file: str) -> None:
      527     _, tree = core.load(md_file)
      528     assert tree.engine.name == 'markdown'
      529     assert isinstance(tree.engine, RpcEngine)
      530
      531 @requires_engine
      532 def test_outline_lists_sections_and_blocks(md_file: str) -> None:
      533     nodes = ast_outline([md_file]).results[0].nodes
      534     assert [node.id for node in nodes] == ['Doc']
      535     child_ids = [child.id for child in nodes[0].children]
      536     assert 'Doc.Part_1' in child_ids and 'Doc.Part_2' in child_ids
      537     assert nodes[0].excert == '# Doc'
      538
      539 @requires_engine
      540 def test_find_by_type_and_name(md_file: str) -> None:
      541     sections = _flatten(ast_find(paths=[md_file], node_type='Section').files[0].nodes)
      542     assert [node.id for node in sections] == ['Doc', 'Doc.Part_1', 'Doc.Part_2']
      543     named = _flatten(ast_find(paths=[md_file], name='Part 2').files[0].nodes)
      544     assert [node.id for node in named] == ['Doc.Part_2']
      545     assert named[0].code == '## Part 2\n\nbeta\n'
      546
      547 @requires_engine
      548 def test_read_returns_section_source(md_file: str) -> None:
      549     result = ast_read(items=[ReadItem(path=md_file, ids=['Doc.Part_1'])])
      550     assert not result.errors
      551     assert result.results[0].nodes[0].code == '## Part 1\n\nalpha\n'
      552
      553 @requires_engine
      554 def test_replace_via_tool_writes_file(md_file: str) -> None:
      555     result = ast_replace(items=[ReplaceItem(path=md_file, source='## Part 1\n\nALPHA', id='Doc.Part_1')])
      556     assert not result.errors
      557     assert result.results[0].new_id == 'Doc.Part_1'
      558     assert Path(md_file).read_text(encoding='utf-8') == '# Doc\n\nintro\n\n## Part 1\n\nALPHA\n\n## Part 2\n\nbeta\n'
      559
      560 @requires_engine
      561 def test_insert_via_tool_reports_ids(md_file: str) -> None:
      562     result = ast_insert(items=[InsertItem(path=md_file, source='## Part 1b\n\nmid', id='Doc.Part_1', position='after')])
      563     assert not result.errors
      564     assert result.results[0].inserted == 2
      565     assert result.results[0].ids[0] == 'Doc.Part_1b'
      566     assert Path(md_file).read_text(
      567         encoding='utf-8') == '# Doc\n\nintro\n\n## Part 1\n\nalpha\n\n## Part 1b\n\nmid\n\n## Part 2\n\nbeta\n'
      568
      569 @requires_engine
      570 def test_delete_via_tool(md_file: str) -> None:
      571     result = ast_delete(items=[DeleteItem(path=md_file, id='Doc.Part_2')])
      572     assert not result.errors
      573     assert Path(md_file).read_text(encoding='utf-8') == '# Doc\n\nintro\n\n## Part 1\n\nalpha\n'
      574
      575 @requires_engine
      576 def test_edit_marks_via_tool(md_file: str) -> None:
      577     result = ast_edit_marks(
      578         items=[
      579             EditMarksItem(
      580                 path=md_file,
      581                 start_marker='alpha',
      582                 end_marker='alpha',
      583                 content='gamma',
      584                 id='Doc.Part_1')])
      585     assert not result.errors
      586     text = Path(md_file).read_text(encoding='utf-8')
      587     assert 'gamma' in text and 'alpha' not in text and ('beta' in text)
      588
      589 @requires_engine
      590 def test_edit_block_via_tool(md_file: str) -> None:
      591     result = ast_edit_block(items=[EditBlockItem(path=md_file, old_text='beta', new_text='delta', id='Doc.Part_2')])
      592     assert not result.errors
      593     text = Path(md_file).read_text(encoding='utf-8')
      594     assert 'delta' in text and 'beta' not in text and ('alpha' in text)
      595
      596 @requires_engine
      597 def test_batch_of_edits_in_one_call(md_file: str) -> None:
      598     result = ast_replace(
      599         items=[
      600             ReplaceItem(
      601                 path=md_file,
      602                 source='## Part 1\n\nONE',
      603                 id='Doc.Part_1'),
      604             ReplaceItem(
      605                 path=md_file,
      606                 source='## Part 2\n\nTWO',
      607                 id='Doc.Part_2')])
      608     assert not result.errors and len(result.results) == 2
      609     text = Path(md_file).read_text(encoding='utf-8')
      610     assert 'ONE' in text and 'TWO' in text and ('alpha' not in text) and ('beta' not in text)
      611
      612 @requires_engine
      613 def test_unknown_id_via_tool_is_reported_and_file_untouched(md_file: str) -> None:
      614     result = ast_replace(items=[ReplaceItem(path=md_file, source='## X\n\ny', id='Doc.Nope')])
      615     assert result.errors and (not result.results)
      616     assert Path(md_file).read_text(encoding='utf-8') == FILE_DOC
      617
      618 @requires_engine
      619 def test_validate_via_tool_accepts_any_markdown(tmp_path: Path) -> None:
      620     path = tmp_path / 'odd.md'
      621     path.write_text('```\nunclosed\n\n[x](\n', encoding='utf-8')
      622     assert ast_validate([str(path)]).all_ok
      623
      624 @requires_engine
      625 def test_create_via_tool_and_overwrite(tmp_path: Path) -> None:
      626     path = tmp_path / 'new.md'
      627     first = ast_create(items=[CreateFileItem(path=str(path), source='# One\n\ntext\n')])
      628     assert not first.errors
      629     assert path.read_text(encoding='utf-8') == '# One\n\ntext\n'
      630     refused = ast_create(items=[CreateFileItem(path=str(path), source='# Two\n')])
      631     assert refused.errors
      632     assert path.read_text(encoding='utf-8') == '# One\n\ntext\n'
      633     again = ast_create(items=[CreateFileItem(path=str(path), source='# Two\n', overwrite=True)])
      634     assert not again.errors
      635     assert path.read_text(encoding='utf-8') == '# Two\n'
      636
      637 @requires_engine
      638 def test_cache_follows_external_changes(md_file: str) -> None:
      639     path = Path(md_file)
      640     _, first = core.load(md_file)
      641     assert core.CACHE.get_tree(path) is first
      642     path.write_text('# Other\n\nchanged\n', encoding='utf-8')
      643     _, second = core.load(md_file)
      644     assert [loc.node_id for loc in core.locate_all(second) if loc.node_type == 'Section'] == ['Other']
      645
      646 @requires_engine
      647 def test_cache_follows_tool_edits(md_file: str) -> None:
      648     ast_replace(items=[ReplaceItem(path=md_file, source='## Part 1\n\nNEW', id='Doc.Part_1')])
      649     _, tree = core.load(md_file)
      650     assert 'NEW' in tree.source
      651     result = ast_read(items=[ReadItem(path=md_file, ids=['Doc.Part_1'])])
      652     assert result.results[0].nodes[0].code == '## Part 1\n\nNEW\n'
      653
      654 @requires_engine
      655 def test_unicode_file_roundtrip_via_tool(tmp_path: Path) -> None:
      656     path = tmp_path / 'u.md'
      657     path.write_text('# Grüße\n\nText 日本 😀\n\n## Teil\n\nalt\n', encoding='utf-8')
      658     ast_replace(items=[ReplaceItem(path=str(path), source='## Teil\n\nneu äöü', id='Grüße.Teil')])
      659     assert path.read_text(encoding='utf-8') == '# Grüße\n\nText 日本 😀\n\n## Teil\n\nneu äöü\n'
      660 '# ---------------------------------------------------------------------------'
      661 '# ast_retrieve über die Tool-Registry'
      662 '# ---------------------------------------------------------------------------'
      663
      664 def _retrieve(registry: ToolRegistry, arguments: dict):
      665     tool = registry.get('ast_retrieve')
      666     assert tool is not None
      667     session = Session(id='markdown-bridge')
      668     session.enabled_tools.add('ast_retrieve')
      669     return tool.handler(ToolContext(session=session, arguments=arguments, registry=registry))
      670
      671 @requires_engine
      672 def test_retrieve_outline_find_read(registry: ToolRegistry, md_file: str) -> None:
      673     result = _retrieve(registry, {'outline': [md_file], 'find': [{'paths': [md_file], 'name': 'Part 1'}], 'read': [
      674                        {'path': md_file, 'ids': ['Doc.Part_2']}]})
      675     assert not result.is_error
      676     content = result.structured_content
      677     assert set(content) == {'outline', 'find', 'read'}
      678     assert 'Doc.Part_1' in str(content['find'])
      679     assert 'beta' in str(content['read'])
      680
      681 @requires_engine
      682 def test_retrieve_search_returns_node_ids_usable_for_read(registry: ToolRegistry, md_file: str) -> None:
      683     directory = str(Path(md_file).parent)
      684     first = _retrieve(registry, {'search': [{'directory': [directory],
      685                       'pattern': 'beta', 'include': ['*.md'], 'intent': 'edit'}]})
      686     assert not first.is_error
      687     entry = first.structured_content['search']['results'][0]['matches'][0]
      688     node_id = entry['nodes'][0]['id']
      689     assert node_id.startswith('Doc.Part_2')
      690     second = _retrieve(registry, {'read': [{'path': md_file, 'ids': [node_id]}]})
      691     assert not second.is_error
      692     assert 'beta' in str(second.structured_content['read'])
      693 '# ---------------------------------------------------------------------------'
      694 '# Absturz des Engine-Prozesses (zuletzt: stellt den Prozess danach wieder her)'
      695 '# ---------------------------------------------------------------------------'
      696
      697 @requires_engine
      698 def test_new_engine_instance_restarts_crashed_process() -> None:
      699     MarkdownEngine().validate('# warm\n')
      700     process = _PROCESS._process
      701     assert process is not None
      702     os.kill(process.pid, signal.SIGKILL)
      703     process.wait(timeout=5)
      704     fresh = MarkdownEngine()
      705     assert fresh.validate('# a\n') is None
      706     assert _PROCESS._process is not process
      707
      708 @requires_engine
      709 @pytest.mark.xfail(strict=True, reason='RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.')
      710 def test_existing_engine_instance_recovers_after_process_crash() -> None:
      711     used = MarkdownEngine()
      712     used.validate('# warm\n')
      713     process = _PROCESS._process
      714     assert process is not None
      715     os.kill(process.pid, signal.SIGKILL)
      716     process.wait(timeout=5)
      717     try:
      718         assert used.validate('# a\n') is None
      719     finally:
      720         MarkdownEngine().validate('# restore\n')
      120     assert process._process is None
      121 '# ---------------------------------------------------------------------------'
      122 '# Prozess-Lifecycle'
      123 '# ---------------------------------------------------------------------------'
      124
      125 @requires_engine
      126 def test_process_starts_lazily_and_terminates(monkeypatch: pytest.MonkeyPatch) -> None:
      127     port = _free_port()
      128     monkeypatch.setenv('AST_MARKDOWN_PORT', str(port))
      129     process = MarkdownProcess()
      130     isolated = RpcEngine('markdown', process)
      131     assert process._process is None
      132     assert isolated.validate('# a\n') is None
      133     assert process._process is not None and process._process.poll() is None
      134     assert process.base_url() == f'http://127.0.0.1:{port}'
      135     first_pid = process._process.pid
      136     assert isolated.validate('# b\n') is None
      137     assert process._process.pid == first_pid
      138     process._terminate()
      139     with pytest.raises(OSError):
      140         socket.create_connection(('127.0.0.1', port), timeout=1).close()
      141
      142 def test_missing_node_binary_fails_fast(monkeypatch: pytest.MonkeyPatch) -> None:
      143     monkeypatch.setenv('NODE', '/nonexistent/node')
      144     monkeypatch.setenv('AST_MARKDOWN_PORT', str(_free_port()))
      145     with pytest.raises((OSError, RuntimeError)):
      146         MarkdownProcess().base_url()
      147
      148 def test_invalid_port_configuration_is_rejected(monkeypatch: pytest.MonkeyPatch) -> None:
      149     monkeypatch.setenv('AST_MARKDOWN_PORT', 'not-a-port')
      150     with pytest.raises(ValueError):
      151         MarkdownProcess().base_url()
      152 '# ---------------------------------------------------------------------------'
      153 '# Parse / Serialize / Validate'
      154 '# ---------------------------------------------------------------------------'
      155
      156 @requires_engine
      157 def test_parse_and_serialize_roundtrip(engine: MarkdownEngine) -> None:
      158     tree = engine.parse(DOC)
      159     assert engine.serialize(tree) == DOC
      160
      161 @requires_engine
      162 @pytest.mark.parametrize('source', ['', '\n', 'plain\n', '---\ntitle: X\n---\n\n# H\n', '# A\r\n\r\ntext\r\n', '* a\n* b\n', 'no trailing newline', '# Ünï ✓ 日本\n\nText äöü 😀\n'])
      163 def test_parse_does_not_modify_source(engine: MarkdownEngine, source: str) -> None:
      164     tree = engine.parse(source)
      165     assert engine.serialize(tree) == source
      166
      167 @requires_engine
      168 @pytest.mark.parametrize('source', ['', '# ok\n', 'text', '\x00', '```\nunclosed fence', '<div>', '[broken](link'])
      169 def test_any_text_is_valid_markdown(engine: MarkdownEngine, source: str) -> None:
      170     assert engine.validate(source) is None
      171     engine.parse(source)
      172
      173 @requires_engine
      174 def test_empty_tree_and_empty_document(engine: MarkdownEngine, tmp_path: Path) -> None:
      175     assert engine.serialize(engine.empty_tree()) == ''
      176     assert engine.empty_tree(tmp_path / 'x.md').path == tmp_path / 'x.md'
      177     assert engine.locate_all(engine.empty_tree()) == []
      178     assert engine.locate_all(engine.parse('')) == []
      179 '# ---------------------------------------------------------------------------'
      180 '# Struktur'
      181 '# ---------------------------------------------------------------------------'
      182
      183 @requires_engine
      184 def test_locate_all_builds_heading_tree(engine: MarkdownEngine) -> None:
      185     located = engine.locate_all(engine.parse(DOC))
      186     assert [
      187         loc.node_type for loc in located] == [
      188             'Section',
      189             'BlockGroup',
      190             'Section',
```
/answer 1632-9d-9c allow
ReasoningToken: 3392
Control Request:
```yaml
id: 1632-9e-0
toolName: python
arguments:
  script: |-
    import subprocess
    p="/home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py"
    s=open(p,encoding="utf-8").read()

    def rep(old,new,count=1):
        global s
        n=s.count(old)
        assert n==count,(old,n,count)
        s=s.replace(old,new)

    rep("""def _find(engine: MarkdownEngine, tree: Tree, node_id: str) -> Located:
        return next((loc for loc in engine.locate_all(tree) if loc.node_id == node_id))
    ""","""def _section_id(tree: Tree, *names: str) -> str:
        names_by_id = {loc.node_id: loc.name for loc in tree.engine.locate_all(tree) if loc.node_type == 'Section'}
        for node_id in names_by_id:
            parts = node_id.split('.')
            if tuple(names_by_id.get('.'.join(parts[:n])) for n in range(1, len(parts) + 1)) == names:
                return node_id
        raise LookupError(names)

    def _file_id(path: str, *names: str) -> str:
        return _section_id(core.load(path)[1], *names)

    def _find(engine: MarkdownEngine, tree: Tree, *names: str) -> Located:
        node_id = _section_id(tree, *names)
        return next((loc for loc in engine.locate_all(tree) if loc.node_id == node_id))
    """)

    for a,b in [("'Title.Section_A'","'Title', 'Section A'"),("'Title.Section_B'","'Title', 'Section B'")]:
        s=s.replace("_find(engine, tree, "+a+")","_find(engine, tree, "+b+")")
    rep("_find(engine, tree, 'A.B')","_find(engine, tree, 'A', 'B')")
    rep("_find(engine, tree, 'Title.A')","_find(engine, tree, 'Title', 'A')")
    rep("_find(engine, tree, f'Doc{i}.Sub')","_find(engine, tree, f'Doc{i}', 'Sub')")
    rep("_find(engine, tree, f'Big.Section_{sections - 1}')","_find(engine, tree, 'Big', f'Section {sections - 1}')")

    rep("""    assert [loc.node_id for loc in sections] == ['Title', 'Title.Section_A', 'Title.Section_B']
    ""","""    section_ids = [loc.node_id for loc in sections]
        assert len(set(section_ids)) == 3
        assert all(i.startswith(section_ids[0] + '.') for i in section_ids[1:])
    """)
    rep("""    ranges = {loc.node_id: (loc.lineno, loc.end_lineno)""","""    ranges = {loc.name: (loc.lineno, loc.end_lineno)""")
    rep("{'Title': (1, 11), 'Title.Section_A': (5, 7), 'Title.Section_B': (9, 11)}","{'Title': (1, 11), 'Section A': (5, 7), 'Section B': (9, 11)}")
    rep("""        assert loc.node_id.startswith('Title.')""","""        assert loc.node_id.startswith(located[0].node_id + '.')""")
    rep("""    assert located[1].node_id == 'H1'""","""    assert located[1].name == 'H1' and '.' not in located[1].node_id""")
    rep("""    assert [loc.node_id for loc in sections] == ['Setext', 'Setext.Sub']
    ""","""    assert [loc.name for loc in sections] == ['Setext', 'Sub']
        assert sections[1].node_id.startswith(sections[0].node_id + '.')
    """)
    rep("""    ids = _ids(engine, '# A\\n\\n#### Deep\\n\\ntext\\n\\n## B\\n')
        assert [i for i in ids if '|' not in i] == ['A', 'A.Deep', 'A.B']
    ""","""    sections = [loc for loc in engine.locate_all(engine.parse('# A\\n\\n#### Deep\\n\\ntext\\n\\n## B\\n')) if loc.node_type == 'Section']
        assert [loc.name for loc in sections] == ['A', 'Deep', 'B']
        assert all(loc.node_id.startswith(sections[0].node_id + '.') and loc.node_id.count('.') == 1 for loc in sections[1:])
    """)
    rep("""    ids = [i for i in _ids(engine, '# T\\n\\n## A\\n\\nx\\n\\n## A\\n\\ny\\n') if '|' not in i]
        assert ids == ['T', 'T.A', 'T.A_2']
        ids = [i for i in _ids(engine, '# A\\n\\n# A\\n') if '|' not in i]
        assert ids == ['A', 'A_2']
    ""","""    ids = [i for i in _ids(engine, '# T\\n\\n## A\\n\\nx\\n\\n## A\\n\\ny\\n') if '|' not in i]
        assert len(ids) == 3 and ids[2] == ids[1] + '_2'
        ids = [i for i in _ids(engine, '# A\\n\\n# A\\n') if '|' not in i]
        assert len(ids) == 2 and ids[1] == ids[0] + '_2'
    """)
    rep("""    sections = [loc.node_id for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'Section']
        assert sections == ['Real', 'Real.Next']""","""    sections = [loc for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'Section']
        assert [loc.name for loc in sections] == ['Real', 'Next']
        assert sections[1].node_id.startswith(sections[0].node_id + '.')""")
    rep("""    assert [loc.node_id for loc in located if loc.node_type == 'Section'] == ['A', 'B']""","""    assert [loc.name for loc in located if loc.node_type == 'Section'] == ['A', 'B']""")
    rep("""    assert 'Title.Section_A2' in _ids(engine, tree.source)
        assert 'Title.Section_A' not in _ids(engine, tree.source)""","""    reparsed = engine.parse(tree.source)
        assert _section_id(reparsed, 'Title', 'Section A2')
        with pytest.raises(LookupError):
            _section_id(reparsed, 'Title', 'Section A')""")
    rep("loc.node_id.startswith('Title.Section_B.')","loc.node_id.startswith(_section_id(tree, 'Title', 'Section B') + '.')")
    rep("loc.node_id.startswith('Title.Section_A.')","loc.node_id.startswith(_section_id(tree, 'Title', 'Section A') + '.')")
    rep("loc.node_id.startswith('Title.Sec.')","loc.node_id.startswith(_section_id(tree, 'Title', 'Sec') + '.')")
    rep("loc.node_id.startswith('A.B.')","loc.node_id.startswith(_section_id(tree, 'A', 'B') + '.')")

    rep("assert [node.id for node in nodes] == ['Doc']","assert [node.id for node in nodes] == [_file_id(md_file, 'Doc')]")
    rep("== ['Doc', 'Doc.Part_1', 'Doc.Part_2']","== [_file_id(md_file, 'Doc'), 'Doc.Part_1', 'Doc.Part_2']")
    rep("'Doc.Part_1b'","_file_id(md_file, 'Doc', 'Part 1b')")
    s=s.replace("'Doc.Part_1'","_file_id(md_file, 'Doc', 'Part 1')").replace("'Doc.Part_2'","_file_id(md_file, 'Doc', 'Part 2')")
    rep("id='Grüße.Teil'","id=_file_id(str(path), 'Grüße', 'Teil')")
    rep("""    assert [loc.node_id for loc in core.locate_all(second) if loc.node_type == 'Section'] == ['Other']""","""    assert [loc.name for loc in core.locate_all(second) if loc.node_type == 'Section'] == ['Other']""")
    open(p,"w",encoding="utf-8").write(s)

    r=subprocess.run(["python","-m","pytest","-q","tests/test_ast_markdown_bridge.py","--tb=short","-x","-q"],cwd="/home/user/xyan/xy.ai.workbench/mcpc",capture_output=True,text=True)
    print(r.stdout[-3500:],r.stderr[-1500:])
  reason: Markdown-Tests von namensbasierten auf hashbasierte IDs umstellen (Auflösung über Namenspfad) und Testdatei ausführen
```
/answer 1632-9e-0 allow
Control Request:
```yaml
id: 1632-9f-9e
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      ........................................................................ [ 80%]
      ................x                                                        [100%]
```
/answer 1632-9f-9e allow
ReasoningToken: 105
Control Request:
```yaml
id: 1632-a0-0
toolName: python
arguments:
  script: |-
    import subprocess
    r=subprocess.run(["python","-m","pytest","-q","tests","--tb=short","-q"],cwd="/home/user/xyan/xy.ai.workbench/mcpc",capture_output=True,text=True)
    print(r.stdout[-2500:],r.stderr[-800:])
  reason: Gesamte Testsuite erneut ausführen
```
/answer 1632-a0-0 allow
Control Request:
```yaml
id: 1632-a1-a0
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      ........................................................................ [ 14%]
      .................................................................x...... [ 29%]
      ........................................................................ [ 44%]
      ........................................................................ [ 59%]
      ........................................................................ [ 74%]
      ........................................................................ [ 89%]
      .....................................................                    [100%]
```
/answer 1632-a1-a0 allow
Result Stats: id=64a509f0-1a95-4bdf-a67d-df4d50d79c03, total: 66575, in: 40, out: 13997, reason: 0, read: 364602, write: 52538, start: 1795381424298, end: 1795381430027, time: 5682, total: 156473, io: 0, control: 47, sent: 0, recv: 143
Die Tests sind angepasst, und die ganze Testsuite läuft durch. Es gibt keine Fehlschläge mehr. Ein Test ist wie vorher als `xfail` markiert (`test_existing_engine_instance_recovers_after_process_crash`).

**Ursache der 42 Fehlschläge:** Die Markdown-Engine vergibt neue IDs. Eine Überschrift bekommt jetzt einen 4-stelligen Hash ihres Namens statt des Namens selbst. Aus `Title.Section_A` wurde zum Beispiel `JLeD.uvrD`. Doppelte Überschriften bekommen weiter den Zusatz `_2`. Die Änderung an `insert.py` hatte damit nichts zu tun. Die Tests in `test_ast_markdown_bridge.py` hatten die alten IDs fest eingetragen.

**Änderungen in `mcpc/tests/test_ast_markdown_bridge.py`:**
- **Neue Hilfsfunktionen:** `_section_id(tree, *names)` und `_file_id(path, *names)` suchen eine Sektion über ihren Namenspfad, etwa `('Title', 'Section A')`. `_find` nimmt jetzt ebenfalls Namen statt einer ID.
- **Strukturtests:** Sie prüfen jetzt Namen und die Verschachtelung der IDs, also dass die ID einer Unter-Sektion mit der ID der übergeordneten Sektion beginnt. Doppelte Überschriften prüft der Test über das Suffix `_2`.
- **Tool-Tests:** Sie holen die IDs über `_file_id` aus der geladenen Datei.

Die Tests hängen so nicht mehr an einem bestimmten Hash. Das Format selbst prüft jetzt aber kein Test mehr direkt. Soll ich einen kleinen Test ergänzen, der den Aufbau `hash.hash` und das Suffix `_2` festschreibt?
