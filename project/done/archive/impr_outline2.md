Verbessere das Fehlerverhalten von `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py`.
Wenn das Ziel irrtümlich ein Verzeichnis ist, soll intern das List Tool (`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py`) aufgerufen und dessen Ausgabe zurückgeliefert werden mit einem Hinweis. Der Intent wird hier antizipiert.

Beispiel:
```yaml
id: b24a-1a
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src
      error: Not a regular file.
```
SystemInit:  id=9da1fbdb-7fb4-4957-ada1-74f8b7645754, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 10e5-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  reason: "Struktur der beiden Dateien verstehen, um das Fehlerverhalten bei Verzeichnissen anzupassen"
```
/answer 10e5-1 allow
Control Request:
```yaml
id: 10e5-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
      nodes:
      - id: JY1HZH|PFSe6T
        signature: "'``ast_outline`` tool: list AST nodes of one or more files.'"
      - id: AIXUsP|EhRAaP
        signature: from dataclasses import dataclass
      - id: OeMnuN|SAOhjO
        signature: "__all__ = ["
      - id: OutlineNodesResult
        signature: "@dataclass(frozen=True) class OutlineNodesResult:"
        docstring: "Nodes of a single file. Attributes: path: The resolved path of the listed file …"
      - id: OutlineNodesError
        signature: "@dataclass(frozen=True) class OutlineNodesError:"
        docstring: "Error listing a single file. Attributes: path: The resolved path that failed, f…"
      - id: OutlineNodesBatchResult
        signature: "@dataclass(frozen=True) class OutlineNodesBatchResult:"
        docstring: "Result of :func:`ast_outline`. Attributes: results: One :class:`OutlineNodesRes…"
      - id: expand_path
        signature: "def _expand_path(path_str: str) -> list[str]:"
        docstring: "Expand *path_str* to its own single-element list, unless it names a directory t…"
      - id: outline_one
        signature: "def _outline_one(path: str, *, with_lines: bool) -> OutlineNodesResult:"
      - id: ast_outline
        signature: "def ast_outline(paths: list[str], *, with_lines: bool=True) -> OutlineNodesBatchResult:"
        docstring: List the hierarchical AST-node tree of one or more files. The tree is the found…
      - id: OutlineNodesTool
        signature: "class OutlineNodesTool(ToolDefinition):"
        children:
        - id: OutlineNodesTool.LOYPRC|AE1Pc0
          signature: name = 'ast_outline'
        - id: OutlineNodesTool.WDGjLy|WLS2rg
          signature: "input_schema = {"
        - id: OutlineNodesTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_list`, translating the MCP schema to/from the AST API."
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: JrBuUj|ISKZu0
        signature: "\"List tool – returns files below a directory, grouped by relative subdirectory …"
      - id: dvO55c|qVCPEH
        signature: import os
      - id: ZrSLiQ|rTmMVq
        signature: "__all__ = ["
      - id: ListError
        signature: "class ListError(Exception):"
        docstring: Raised when a directory listing cannot be performed.
      - id: ListItem
        signature: "@dataclass(frozen=True) class ListItem:"
        docstring: "One directory to list. Attributes: path: Absolute directory path to list (must …"
      - id: ListResult
        signature: "@dataclass(frozen=True) class ListResult:"
        docstring: "Result of listing a single directory, mirroring its input path for result assoc…"
      - id: ListItemError
        signature: "@dataclass(frozen=True) class ListItemError:"
        docstring: "Error listing a single directory, mirroring its input path for result associati…"
      - id: ListBatchResult
        signature: "@dataclass(frozen=True) class ListBatchResult:"
        docstring: "Result of :func:`list`. Attributes: results: One :class:`ListResult` per succes…"
      - id: list_one
        signature: "def _list_one(item: ListItem) -> ListResult:"
        docstring: "List all files below the absolute directory ``item.path``, grouped like ``ls -R…"
      - id: dedupe_hierarchy
        signature: "def _dedupe_hierarchy(items: list[ListItem]) -> list[ListItem]:"
        docstring: Remove items whose path lies below another item's path in the same directory hi…
      - id: list
        signature: "def list(items: list[ListItem]) -> ListBatchResult:"
        docstring: "List each directory in ``items``. Args: items: Directories to list. Must be non…"
      - id: ListTool
        signature: "class ListTool(ToolDefinition):"
        children:
        - id: ListTool.k0nI9s|nAr9WN
          signature: name = 'list'
        - id: ListTool.AYUjTN|SzH5Bg
          signature: _ITEMS_DESCRIPTION = 'Directories to list.'
        - id: ListTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`list`, translating the MCP schema to/from the Python API."
      - id: register_list_tool
        signature: "def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 10e5-2 allow
Control Request:
```yaml
id: 10e5-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    ids:
    - OutlineNodesResult
    - OutlineNodesError
    - OutlineNodesBatchResult
    - expand_path
    - outline_one
    - ast_outline
    - OutlineNodesTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    ids:
    - ListError
    - ListItem
    - ListResult
    - ListItemError
    - ListBatchResult
    - list_one
    - list
    - ListTool
  reason: "Vollständigen Code lesen, um Fehlerbehandlung bei Verzeichnissen zu implementieren"
```
/answer 10e5-3 allow
Control Request:
```yaml
id: 10e5-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
      nodes:
      - id: OutlineNodesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNodesResult:
              """Nodes of a single file.

              Attributes:
                  path: The resolved path of the listed file (see ``ast_outline``'s
                      directory-expansion note), for result association.
                  nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
                      document order, suited for retrieval and navigation.
              """
              path: str
              nodes: list[core.OutlineNode]
      - id: OutlineNodesError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNodesError:
              """Error listing a single file.

              Attributes:
                  path: The resolved path that failed, for result association.
                  error: The error message.
              """
              path: str
              error: str
      - id: OutlineNodesBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNodesBatchResult:
              """Result of :func:`ast_outline`.

              Attributes:
                  results: One :class:`OutlineNodesResult` per successfully listed file.
                  errors: One :class:`OutlineNodesError` per file that failed.
              """
              results: list[OutlineNodesResult]
              errors: list[OutlineNodesError]
      - id: expand_path
        type: FunctionDef
        code: |-
          def _expand_path(path_str: str) -> list[str]:
              """Expand *path_str* to its own single-element list, unless it names a
              directory that can be unambiguously expanded to its files.

              Expansion only succeeds if the directory has no subdirectories and holds
              at most :data:`_MAX_DIR_EXPANSION` files; otherwise *path_str* is returned
              unchanged and left to fail normally.
              """
              p = Path(path_str)
              if not p.is_dir():
                  return [path_str]
              entries = list(p.iterdir())
              if any((e.is_dir() for e in entries)):
                  return [path_str]
              files = sorted((e for e in entries if e.is_file()), key=lambda e: e.name)
              if not files or len(files) > _MAX_DIR_EXPANSION:
                  return [path_str]
              return [str(f) for f in files]
      - id: outline_one
        type: FunctionDef
        code: |-
          def _outline_one(path: str, *, with_lines: bool) -> OutlineNodesResult:
              tree = core.load(path)[1]
              nodes = core.build_outline(core.locate_all(tree), with_lines=with_lines, with_type=False)
              return OutlineNodesResult(path=path, nodes=nodes)
      - id: ast_outline
        type: FunctionDef
        code: |-
          def ast_outline(paths: list[str], *, with_lines: bool=True) -> OutlineNodesBatchResult:
              """List the hierarchical AST-node tree of one or more files.

              The tree is the foundation every other tool builds on: each node carries its
              unique, primarily name-based ``id`` and line range, but never its source –
              use ``ast_find`` (property/text filtering) or ``ast_read`` (by id) to
              retrieve source.

              A path naming a directory without subdirectories and holding at most
              5 files is transparently expanded to those files; any other directory
              fails normally.

              Args:
                  paths: Absolute paths of the files to list. Must be non-empty.
                  with_lines: Whether to populate each node's line range.

              Returns:
                  OutlineNodesBatchResult: One result per listed file, one error per failed file.

              Raises:
                  core.AstError: If ``paths`` is empty.
              """
              if not paths:
                  raise core.AstError("'paths' must be a non-empty list.")
              results: list[OutlineNodesResult] = []
              errors: list[OutlineNodesError] = []
              for path in paths:
                  for real_path in _expand_path(path):
                      try:
                          results.append(_outline_one(real_path, with_lines=with_lines))
                      except core.AstError as exc:
                          errors.append(OutlineNodesError(path=real_path, error=str(exc)))
              return OutlineNodesBatchResult(results=results, errors=errors)
      - id: OutlineNodesTool
        type: ClassDef
        signature: "class OutlineNodesTool(ToolDefinition):"
        children:
        - id: OutlineNodesTool.LOYPRC|AE1Pc0
          type: statements
          code: |-
            name = 'ast_outline'
            title = 'List AST nodes of files'
            description = "Hierarchical tree of one or more files' AST nodes (import/statement segments, classes, functions, sections) with id and optional line range – no source. A directory without subdirectories and with at most 5 files is transparently expanded to those files. " + core.OUTLINE_NODE_DESCRIPTION
        - id: OutlineNodesTool.WDGjLy|WLS2rg
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'paths': {
                        'type': 'array',
                        'minItems': 1,
                        'items': {
                            'type': 'string'},
                        'description': 'Absolute paths of the files to list.'}},
                'required': ['paths']}
        - id: OutlineNodesTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_list`, translating the MCP schema to/from the AST API."""
                paths, error = require_items(ctx, key='paths')
                if error is not None:
                    return error
                with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
                batch = ast_outline(paths=paths, with_lines=with_lines)
                result_serializer = lambda r: {'path': r.path, 'nodes': [core.to_dict(n) for n in r.nodes]}
                error_serializer = lambda e: {'path': e.path, 'error': e.error}
                content = serialize_batch_result(batch, result_serializer, error_serializer)
                return ToolResult(structured_content=content)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: ListError
        type: ClassDef
        code: |-
          class ListError(Exception):
              """Raised when a directory listing cannot be performed."""
      - id: ListItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListItem:
              """One directory to list.

              Attributes:
                  path: Absolute directory path to list (must exist and be a directory).
                  pattern: Optional regular expression to filter results. Only matching file paths are included.
              """
              path: str
              pattern: str | None = None
      - id: ListResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListResult:
              """Result of listing a single directory, mirroring its input path for result association."""
              path: str
              entries: list[str]
              warning: str | None = None
      - id: ListItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListItemError:
              """Error listing a single directory, mirroring its input path for result association."""
              path: str
              error: str
      - id: ListBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListBatchResult:
              """Result of :func:`list`.

              Attributes:
                  results: One :class:`ListResult` per successfully listed directory.
                  errors: One :class:`ListItemError` per directory that failed.
              """
              results: list[ListResult]
              errors: list[ListItemError]
      - id: list_one
        type: FunctionDef
        code: |-
          def _list_one(item: ListItem) -> ListResult:
              """List all files below the absolute directory ``item.path``, grouped like ``ls -R``.

              Raises:
                  ListError: If path is not absolute.
                  ListError: If path does not exist or is not a directory.
                  ListError: If pattern is not a valid regular expression.
                  ListError: If more than ``_MAX_ENTRIES`` entries remain even after the result has been
                      reduced to one entry per directory level and then to top-level directories only.
              """
              '# When the flat file count exceeds the limit, the result is progressively reduced:'
              '# first to one entry per directory level (with a per-level file count), then, if'
              '# that still exceeds the limit, to top-level directories only (with an aggregated'
              '# file count below each). Both reduced results are returned with a warning.'
              dir_path = Path(item.path)
              if not dir_path.is_absolute():
                  raise ListError('Path must be absolute.')
              dir_path = normalize_directory(dir_path)
              if not dir_path.is_dir():
                  raise ListError('Directory not found or not a directory.')
              try:
                  regex = re.compile(item.pattern) if item.pattern else None
              except re.error as exc:
                  raise ListError(f'Invalid regex pattern: {exc}') from exc
              groups: dict[str, list[str]] = {}
              match_count = 0
              for root, dirs, files in os.walk(str(dir_path)):
                  rel_dir = os.path.relpath(root, str(dir_path))
                  matched_files = []
                  for file in sorted(files):
                      rel_path = os.path.normpath(os.path.join(rel_dir, file))
                      if regex is None or regex.search(rel_path):
                          matched_files.append(file)
                  if matched_files:
                      groups[rel_dir] = matched_files
                      match_count += len(matched_files)
              if match_count <= _MAX_ENTRIES:
                  entries = []
                  for rel_dir in sorted(groups):
                      header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                      entries.append(f'{header}:')
                      entries.extend(groups[rel_dir])
                  return ListResult(path=item.path, entries=entries)
              '# Step 1: collapse each directory level to a single representative entry,'
              '# annotated with the total file count for that level.'
              level1_counts = {rel_dir: len(files) for rel_dir, files in groups.items()}
              if len(level1_counts) <= _MAX_ENTRIES:
                  entries = []
                  for rel_dir in sorted(groups):
                      header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                      count = level1_counts[rel_dir]
                      entries.append(f'{header}: ({count} file{('s' if count != 1 else '')})')
                      entries.append(groups[rel_dir][0])
                  warning = f"Result reduced to one entry per directory level because the total number of entries ({match_count}) exceeds the limit of {_MAX_ENTRIES}. Each directory header now shows the total file count for that level. Narrow down the result using the 'pattern' regular expression parameter."
                  return ListResult(path=item.path, entries=entries, warning=warning)
              '# Step 2: the per-level reduction still exceeds the limit, so collapse the'
              '# tree depth and aggregate file counts below each top-level directory.'

              def _top_key(rel_dir: str) -> str:
                  if rel_dir == '.':
                      return '.'
                  return rel_dir.split(os.sep)[0]
              top_counts: dict[str, int] = {}
              for rel_dir, files in groups.items():
                  key = _top_key(rel_dir)
                  top_counts[key] = top_counts.get(key, 0) + len(files)
              if len(top_counts) > _MAX_ENTRIES:
                  raise ListError(
                      f"Too many entries even after reducing by directory level ({
                          len(top_counts)}) exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")
              entries = []
              for key in sorted(top_counts):
                  header = key if key == '.' else './' + key.replace(os.sep, '/')
                  count = top_counts[key]
                  entries.append(f'{header}: ({count} file{('s' if count != 1 else '')} below)')
              warning = f"Result reduced to top-level directories because grouping by directory level still exceeds the limit of {_MAX_ENTRIES} ({
                  len(level1_counts)} directories). Each header now shows the total number of files below that directory. Narrow down the result using the 'pattern' regular expression parameter."
              return ListResult(path=item.path, entries=entries, warning=warning)
      - id: list
        type: FunctionDef
        code: |-
          def list(items: list[ListItem]) -> ListBatchResult:
              """List each directory in ``items``.

              Args:
                  items: Directories to list. Must be non-empty.

              Returns:
                  ListBatchResult: one result per successfully listed directory, one error per failed directory.

              Raises:
                  ListError: If items is empty.
              """
              if not items:
                  raise ListError("'items' must be a non-empty list.")
              items = _dedupe_hierarchy(items)
              results: list[ListResult] = []
              errors: list[ListItemError] = []
              for item in items:
                  try:
                      results.append(_list_one(item))
                  except ListError as exc:
                      errors.append(ListItemError(path=item.path, error=str(exc)))
              return ListBatchResult(results=results, errors=errors)
      - id: ListTool
        type: ClassDef
        signature: "class ListTool(ToolDefinition):"
        children:
        - id: ListTool.k0nI9s|nAr9WN
          type: statements
          code: |-
            name = 'list'
            title = 'List directory contents'
            description = 'List all files below one or more absolute directory paths, recursively, as a flat list, for a batch of items. Filter each result with a regular expression. Limits apply per item, not per batch.'
            _ITEM_PROPERTIES = {'path': {'type': 'string', 'description': 'Absolute directory path.'},
                                'pattern': {'type': 'string', 'description': 'Regular expression to filter the result.'}}
            _ITEM_REQUIRED = ['path']
        - id: ListTool.AYUjTN|SzH5Bg
          type: statements
          code: |-
            _ITEMS_DESCRIPTION = 'Directories to list.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)
        - id: ListTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""

                def item_factory(it: dict[str, Any]) -> ListItem:
                    return ListItem(path=it['path'], pattern=it.get('pattern'))

                def result_serializer(r):
                    return {'path': r.path, 'entries': r.entries, **({'warning': r.warning} if r.warning else {})}
                return handle_batch_tool(ctx, item_factory, list, ListError, result_serializer)
```
/answer 10e5-4 allow
ReasoningToken: 22
Control Request:
```yaml
id: 10e5-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    ids:
    - JY1HZH|PFSe6T
    - AIXUsP|EhRAaP
    - OeMnuN|SAOhjO
  reason: Imports und Modulkopf von outline.py prüfen für korrekten relativen Import des List-Tools
```
/answer 10e5-5 allow
Control Request:
```yaml
id: 10e5-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
      nodes:
      - id: JY1HZH|PFSe6T
        type: statements
        code: "'``ast_outline`` tool: list AST nodes of one or more files.'"
      - id: AIXUsP|EhRAaP
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result
      - id: OeMnuN|SAOhjO
        type: statements
        code: |-
          __all__ = [
              'OutlineNodesResult',
              'OutlineNodesError',
              'OutlineNodesBatchResult',
              'ast_outline',
              'OutlineNodesTool',
              'register']
          _MAX_DIR_EXPANSION = 5
```
/answer 10e5-6 allow
ReasoningToken: 1751
Text:
 Imports sind absolut. Ich füge den Import für das List-Tool hinzu und implementiere die Verzeichnis-Behandlung.
Control Request:
```yaml
id: 10e5-7
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: AIXUsP|EhRAaP
    old_text: "from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result"
    new_text: |-
      from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result
      from xy.ai.mcpc.tools.list import ListError, ListItem
      from xy.ai.mcpc.tools.list import list as _list_dirs
  reason: Import des List-Tools für die interne Verzeichnis-Behandlung ergänzen
```
/answer 10e5-7 allow
Control Request:
```yaml
id: 10e5-8
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: outline_one
    position: after
    source: |2-


      def _directory_listing_hint(path: str) -> str:
          """Build an error message for a path that turned out to be a directory,
          anticipating the likely intent: list its contents (via the ``list`` tool)
          instead of outlining it as a file.
          """
          hint = 'Path is a directory, not a file; listing its contents instead (intent anticipated).'
          try:
              batch = _list_dirs([ListItem(path=path)])
          except ListError as exc:
              return f'{hint} Listing failed: {exc}'
          if batch.errors:
              return f'{hint} Listing failed: {batch.errors[0].error}'
          result = batch.results[0]
          parts = [hint]
          if result.warning:
              parts.append(result.warning)
          parts.extend(result.entries)
          return '\n'.join(parts)
  reason: "Hilfsfunktion ergänzen, die bei Verzeichnis-Zielen das List-Tool aufruft und dessen Ausgabe mit Hinweis formatiert"
```
/answer 10e5-8 allow
Control Request:
```yaml
id: 10e5-9
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: ast_outline
    old_text: |2-
                  try:
                      results.append(_outline_one(real_path, with_lines=with_lines))
                  except core.AstError as exc:
                      errors.append(OutlineNodesError(path=real_path, error=str(exc)))
    new_text: |2-
                  if Path(real_path).is_dir():
                      errors.append(OutlineNodesError(path=real_path, error=_directory_listing_hint(real_path)))
                      continue
                  try:
                      results.append(_outline_one(real_path, with_lines=with_lines))
                  except core.AstError as exc:
                      errors.append(OutlineNodesError(path=real_path, error=str(exc)))
  reason: "Verzeichnis-Erkennung in ast_outline einbauen, bevor der normale Outline-Versuch unternommen wird"
```
/answer 10e5-9 allow
ReasoningToken: 19
Text:
 Jetzt die Schleife in `ast_outline` anpassen, um Verzeichnisse zu erkennen:
Control Request:
```yaml
id: 10e5-a
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: ast_outline
    old_text: |2-
          A path naming a directory without subdirectories and holding at most
          5 files is transparently expanded to those files; any other directory
          fails normally.
    new_text: |2-
          A path naming a directory without subdirectories and holding at most
          5 files is transparently expanded to those files; any other directory
          is reported as an error whose message contains the directory's listing
          (via the ``list`` tool), anticipating the likely intent.
  reason: Docstring an das neue Verzeichnis-Fehlerverhalten anpassen
```
/answer 10e5-a allow
Text:
 Docstring-Hinweis auf das neue Verhalten ergänzen:
Control Request:
```yaml
id: 10e5-b
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: OutlineNodesTool.LOYPRC|AE1Pc0
    old_text: A directory without subdirectories and with at most 5 files is transparently expanded to those files.
    new_text: A directory without subdirectories and with at most 5 files is transparently expanded to those files; any other directory is reported as an error containing its listing.
  reason: Tool-Beschreibung konsistent zum neuen Verzeichnis-Fehlerverhalten aktualisieren
```
/answer 10e5-b allow
Control Request:
```yaml
id: 10e5-c
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  reason: Syntaktische Korrektheit der geänderten Datei prüfen
```
/answer 10e5-c allow
Result Stats: id=9da1fbdb-7fb4-4957-ada1-74f8b7645754, total: 24192, in: 151, out: 4875, reason: 0, read: 133310, write: 19166
Text:
 Auch die Tool-Beschreibung aktualisieren und validieren:
Fertig. Bei einem irrtümlich übergebenen Verzeichnis (das nicht trivial zu Dateien expandiert werden kann) wird nun intern das `list`-Tool aufgerufen; dessen Ausgabe (inkl. eventueller Warnung) wird zusammen mit einem Hinweis auf den antizipierten Intent in das `error`-Feld geschrieben.
