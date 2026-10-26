Verbessere das Tool `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py`.

Im Beispiel verwendet der Agent das Wort "Use" und meint damit Import Sektionen in Rust. Dies kann antizipiert werden.
Liefert die Einschränkung mit Node Type keine Ergebnisse soll ein zweiter Lauf eine begrenzte Wortsuche durchführen und Segmente liefern die per Wortsuche (alleinstehend, case insensitive) "use" beinhalten. Für andere Sprachen sind andere wird der Agent andere Begriffe verwenden wie "include", "import", "source" etc.
Die Wortsuche muss mindestens 3 Zeichen haben und wird nur als ganzes vorkommen gesucht.

Beispiel:
```yaml
id: ed48-1f7
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
  node_type: Use
  reason: Alle pub-use-Exports der Fassade lesen
  
id: ed48-1f8
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
      nodes: []
```
SystemInit:  id=15b25c4d-49fd-4335-8be9-34e3b079c4c0, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 5d33-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: Struktur der Datei verstehen
```
/answer 5d33-1 allow
Control Request:
```yaml
id: 5d33-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
      nodes:
      - id: LjJRQf|jlFZZg
        signature: "'``ast_find`` tool: find AST nodes by type, name, id, line range or parent type…"
      - id: 6cUWvL|prGBvq
        signature: import re
      - id: rY0KYk|mmIbkP
        signature: "__all__ = ['FileNodesResult', 'FindNodesResult', 'ast_find', 'FindNodesTool', '…"
      - id: FileNodesResult
        signature: "@dataclass(frozen=True) class FileNodesResult:"
        docstring: "Nodes matching the given selectors within a single file. Attributes: path: The …"
      - id: FindNodesResult
        signature: "@dataclass(frozen=True) class FindNodesResult:"
        docstring: "Result of :func:`ast_find`. Attributes: files: One :class:`FileNodesResult` per…"
      - id: load_tolerant
        signature: "def _load_tolerant(path: str) -> core.Tree:"
        docstring: "Load *path*, tolerating a forgotten filename extension. If *path* does not exis…"
      - id: find_in_file
        signature: "def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool, with_type: bool) -> FileNodesResult:"
      - id: ast_find
        signature: "def ast_find(paths: list[str], *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:"
        docstring: "Find nodes by id, type, name, line range, parent type, text or regexp. ``ast_fi…"
      - id: FindNodesTool
        signature: "class FindNodesTool(ToolDefinition):"
        children:
        - id: FindNodesTool.4leSQE|KdRckz
          signature: name = 'ast_find'
        - id: FindNodesTool.WDGjLy|2g3uCW
          signature: "input_schema = {"
        - id: FindNodesTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 5d33-2 allow
Control Request:
```yaml
id: 5d33-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    ids:
    - FileNodesResult
    - find_in_file
    - ast_find
  reason: Code lesen
```
/answer 5d33-3 allow
Control Request:
```yaml
id: 5d33-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
      nodes:
      - id: FileNodesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class FileNodesResult:
              """Nodes matching the given selectors within a single file.

              Attributes:
                  path: The path exactly as given in the input.
                  nodes: Outline-style node descriptions (see :class:`core.OutlineNode`)
                      matching the given selectors, suited for retrieval and navigation.
              """
              path: str
              nodes: list[core.OutlineNode]
      - id: find_in_file
        type: FunctionDef
        code: |-
          def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool, with_type: bool) -> FileNodesResult:
              tree = _load_tolerant(path)
              if no_selector:
                  nodes = core.build_outline(core.locate_all(tree), with_code=True, with_lines=with_lines, with_type=with_type)
                  return FileNodesResult(path=path, nodes=nodes)
              candidates = core.find(tree, **exact)
              if not candidates and exact.get('id') and (not any((v for k, v in exact.items() if k != 'id'))):
                  fallback = core.resolve_by_prefix(core.locate_all(tree), exact['id'])
                  candidates = [fallback] if fallback is not None else []
              if lineno is not None or end_lineno is not None:
                  start = lineno if lineno is not None else end_lineno
                  end = end_lineno if end_lineno is not None else lineno
                  hit = core.most_specific(candidates, start, end)
                  if hit is not None:
                      candidates = [hit]
                  else:
                      overlap = [c for c in candidates if c.lineno <= end and c.end_lineno >= start]
                      candidates = [
                          c for c in overlap if not any(
                              (o is not c and c.lineno <= o.lineno and (
                                  o.end_lineno <= c.end_lineno) for o in overlap))]
              if pattern is None:
                  nodes = core.build_outline(candidates, with_code=True, with_lines=with_lines, with_type=with_type)
                  return FileNodesResult(path=path, nodes=nodes)
              source = tree.source
              seen: set[str] = set()
              ordered: list[core.Located] = []
              for m in pattern.finditer(source):
                  start_line = source.count('\n', 0, m.start()) + 1
                  end_line = source.count('\n', 0, max(m.end() - 1, m.start())) + 1
                  loc = core.most_specific(candidates, start_line, end_line)
                  if loc is not None and loc.node_id not in seen:
                      seen.add(loc.node_id)
                      ordered.append(loc)
              nodes = core.build_outline(ordered, with_code=True, with_lines=with_lines, with_type=with_type)
              return FileNodesResult(path=path, nodes=nodes)
      - id: ast_find
        type: FunctionDef
        code: |-
          def ast_find(paths: list[str], *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:
              """Find nodes by id, type, name, line range, parent type, text or regexp.

              ``ast_find`` is the single retrieval point that restricts on node properties;
              every other tool addresses nodes purely by ``id``. ``text``/``regexp`` are
              matched against each file's whole source, and each match is attributed to the
              most specific (smallest) enclosing node rather than to every ancestor whose
              source happens to contain it. Matches are returned with their full source.
              Called with no selector at all, ``ast_find`` returns the whole node tree per
              file, nested like ``ast_outline`` but including source.

              Args:
                  paths: Absolute paths of the files to search. Must be non-empty.
                  id: Engine-independent unique node id (primarily name-based path).
                  name: Exact simple name a node's ``name`` must equal.
                  node_type: Regular expression (case-insensitive, ``re.search``) a
                      node's type name must match.
                  lineno: Line the target node must contain; selects the most specific
                      (smallest) matching node. Combined with ``end_lineno``, selects the
                      most specific node fully covering ``[lineno, end_lineno]``; if none
                      does, all innermost nodes overlapping the range are returned.
                  end_lineno: End line of the target range; see ``lineno``. May be given
                      alone to select the most specific node containing that single line.
                  parent_type: Node type name of the enclosing container (case-insensitive).
                  text: Case-insensitive substring to search for in each file.
                  regexp: Regular expression to search for in each file (``re.finditer``).
                  with_lines: Whether to populate each match's line range.

              Returns:
                  FindNodesResult: One :class:`FileNodesResult` per path, in order. Any
                  number of matches per file (including zero) is a normal, successful result.

              Raises:
                  core.AstError: If ``paths`` is empty, if any path is not absolute or does
                      not point to an existing regular file, if a file has a syntax error, or
                      if ``regexp`` is not a valid regular expression.
              """
              if not paths:
                  raise core.AstError("'paths' must be a non-empty list.")
              if node_type is not None:
                  try:
                      re.compile(node_type)
                  except re.error as exc:
                      raise core.AstError(f'Invalid regexp: {exc}') from exc
              exact = dict(id=id, name=name, node_type=node_type, parent_type=parent_type)
              structural = dict(exact, lineno=lineno, end_lineno=end_lineno)
              no_selector = not any(structural.values()) and text is None and (regexp is None)
              pattern: re.Pattern[str] | None = None
              if not no_selector and (text is not None or regexp is not None):
                  if regexp is not None:
                      try:
                          pattern = re.compile(regexp)
                      except re.error as exc:
                          raise core.AstError(f'Invalid regexp: {exc}') from exc
                  else:
                      pattern = re.compile(re.escape(text), re.IGNORECASE)
              with_type = node_type is not None
              files = [
                  _find_in_file(
                      p,
                      exact=exact,
                      lineno=lineno,
                      end_lineno=end_lineno,
                      no_selector=no_selector,
                      pattern=pattern,
                      with_lines=with_lines,
                      with_type=with_type) for p in paths]
              return FindNodesResult(files=files)
```
/answer 5d33-4 allow
ReasoningToken: 429
Control Request:
```yaml
id: 5d33-5
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    id: find_in_file
    old_text: |2-
          if lineno is not None or end_lineno is not None:
              start = lineno if lineno is not None else end_lineno
    new_text: |2-
          if not candidates and lineno is None and end_lineno is None and pattern is None:
              word = exact.get('node_type')
              if word and len(word) >= 3 and re.fullmatch(r'\w+', word):
                  rest = {k: v for k, v in exact.items() if k != 'node_type'}
                  pool = core.find(tree, **rest) if any(rest.values()) else core.locate_all(tree)
                  word_pattern = re.compile(rf'\b{re.escape(word)}\b', re.IGNORECASE)
                  hits: list[core.Located] = []
                  hit_ids: set[str] = set()
                  for m in word_pattern.finditer(tree.source):
                      first = tree.source.count('\n', 0, m.start()) + 1
                      last = tree.source.count('\n', 0, max(m.end() - 1, m.start())) + 1
                      loc = core.most_specific(pool, first, last)
                      if loc is not None and loc.node_id not in hit_ids:
                          hit_ids.add(loc.node_id)
                          hits.append(loc)
                  nodes = core.build_outline(hits, with_code=True, with_lines=with_lines, with_type=True)
                  return FileNodesResult(path=path, nodes=nodes)
          if lineno is not None or end_lineno is not None:
              start = lineno if lineno is not None else end_lineno
  reason: Fallback-Wortsuche einbauen
```
/answer 5d33-5 allow
Text:
 Dokumentation im Docstring ergänzen.
Control Request:
```yaml
id: 5d33-6
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    id: ast_find
    old_text: |2
                  node's type name must match.
    new_text: |2
                  node's type name must match. If a file yields no match and the
                  value is a single word of at least 3 characters (e.g. ``Use``,
                  ``import``), a second pass searches the file for that whole word
                  (case-insensitive) and returns the most specific enclosing nodes.
  reason: Docstring aktualisieren
```
/answer 5d33-6 allow
Result Stats: id=15b25c4d-49fd-4335-8be9-34e3b079c4c0, total: 13608, in: 10, out: 2330, reason: 0, read: 33242, write: 11268, start: 1793047144488, end: 1793047150446, time: 5889, total: 26472, io: 0, control: 69, sent: 0, recv: 130
`ast_find` führt jetzt eine zweite Suche aus, wenn `node_type` in einer Datei keinen Treffer liefert. Ich habe die Änderung weder ausgeführt noch getestet, auch nicht mit deinem Rust-Beispiel `mod.rs`.

**Ablauf der zweiten Suche** (in `_find_in_file` in `find.py`):
- Sie läuft nur, wenn der erste Lauf keine Treffer hat und weder `lineno`/`end_lineno` noch `text`/`regexp` angegeben sind.
- Der `node_type`-Wert muss ein einzelnes Wort mit mindestens 3 Zeichen sein, zum Beispiel `Use`, `import` oder `include`.
- Die Suche ist case-insensitive und findet das Wort nur als ganzes (`\bword\b`). Aus „use“ wird also kein Treffer in „reuse“ oder „user“.
- Jeder Treffer wird dem kleinsten umschließenden Knoten zugeordnet, und Duplikate werden entfernt.
- Andere Filter wie `name`, `id` und `parent_type` bleiben aktiv und schränken die Kandidaten weiter ein.
- In dieser zweiten Suche enthalten die Treffer das Feld `type`, damit der Agent sieht, welche Knotentypen die Sprache tatsächlich verwendet.

Im Docstring von `ast_find` steht das Verhalten jetzt beim Parameter `node_type`.

Ein Wort wie „use“ kann auch in Kommentaren oder Funktionskörpern vorkommen. Dann bekommt der Agent den umschließenden Knoten, also zum Beispiel eine ganze Funktion statt nur der Import-Zeile.
Teste es kurz
SystemInit:  id=15b25c4d-49fd-4335-8be9-34e3b079c4c0, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 53
Control Request:
```yaml
id: 5d33-7
toolName: python
arguments:
  script: |-
    import sys, tempfile, os
    sys.path.insert(0, '/home/user/xyan/xy.ai.workbench/mcpc/src')
    from xy.ai.mcpc.tools.ast.find import ast_find
    d = tempfile.mkdtemp()
    rs = os.path.join(d, 'mod.rs')
    open(rs, 'w').write("pub use a::B;\nuse c::D;\nfn reuse() {}\n")
    py = os.path.join(d, 'm.py')
    open(py, 'w').write("import os\n\ndef user():\n    pass\n")
    for p, t in [(rs, 'Use'), (py, 'Import'), (py, 'Us'), (py, 'FunctionDef')]:
        try:
            r = ast_find([p], node_type=t)
            print(t, os.path.basename(p), [(n.id, getattr(n, 'type', None), getattr(n, 'code', None)) for n in r.files[0].nodes])
        except Exception as e:
            print(t, p, 'ERR', repr(e))
  reason: Fallback-Wortsuche mit Rust- und Python-Dateien kurz testen
```
/answer 5d33-7 allow
Control Request:
```yaml
id: 5d33-8
result:
  structuredContent:
    exit_code: 0
    stdout: |
      Use mod.rs [('zKy0lt|4oylUv', 'ImportGroup', 'pub use a::B;\nuse c::D;\n')]
      Import m.py [('ZZzWzN|C42Mdx', 'imports', 'import os')]
      Us m.py []
      FunctionDef m.py [('user', 'FunctionDef', 'def user():\n    pass')]
```
/answer 5d33-8 allow
Result Stats: id=15b25c4d-49fd-4335-8be9-34e3b079c4c0, total: 15837, in: 14, out: 3212, reason: 0, read: 56430, write: 12611, start: 1793047219961, end: 1793047224196, time: 4177, total: 35784, io: 0, control: 58, sent: 0, recv: 85
Der Kurztest war erfolgreich. Ich habe ihn mit temporären Dateien in Python direkt gegen `ast_find` laufen lassen.

| Aufruf | Ergebnis |
|---|---|
| Rust-Datei mit `pub use a::B; use c::D; fn reuse(){}`, `node_type="Use"` | Die Suche liefert den Import-Block (`ImportGroup`) mit beiden `use`-Zeilen. `reuse` wird nicht gefunden. |
| Python-Datei mit `import os` und `def user()`, `node_type="Import"` | Die Suche liefert den Import-Knoten `import os`. |
| Python-Datei, `node_type="Us"` (zu kurz) | Leere Liste, also kein Fallback. |
| Python-Datei, `node_type="FunctionDef"` | Normaler Treffer im ersten Lauf, kein Fallback nötig. |

Nicht getestet habe ich die Kombination mit `name` oder `parent_type`, und ich habe keine Testdatei dafür angelegt.
