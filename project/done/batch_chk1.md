Die Batch Ebene von `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py` soll einen Aufruf mit Unterverzeichnissen derselben Hierarchie erkennen und in diesem Fall alle Aufrufe bis auf den höchsten der Hierarchie entfernen.
Unverändert sind hingegen der Batch Aufruf von Unterverzeichnissen getrennter Hierarchien. 

Beispiel:
```yaml
id: b24a-1
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
  - path: /home/user/xyan/xy.ai.workbench/ast-engines
  - path: /home/user/xyan/xy.ai.workbench/ast-output
  reason: "Struktur des Java-Projekts und der ast-engines kennenlernen, um analoges Rust-Projekt zu erstellen"
```
In diesem Beispiel können sinnvoll 2 Aufrufe auf `/home/user/xyan/xy.ai.workbench/ast-engines` reduziert werden. `/home/user/xyan/xy.ai.workbench/ast-output` Bleibt unverändert.

SystemInit:  id=5780023f-1768-4c47-bdbc-7415714ce97a, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 6e34-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  reason: "Struktur der Batch-Ebene von list verstehen, um Deduplizierung von Unterverzeichnissen zu implementieren"
```
/answer 6e34-1 allow
Control Request:
```yaml
id: 6e34-2
result:
  structuredContent:
    results:
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
/answer 6e34-2 allow
ReasoningToken: 10
Control Request:
```yaml
id: 6e34-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    ids:
    - list
    - list_one
    - ListItem
  reason: "Implementierung der list()-Funktion ansehen, um Deduplizierung einzubauen"
```
/answer 6e34-3 allow
Control Request:
```yaml
id: 6e34-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
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
              results: list[ListResult] = []
              errors: list[ListItemError] = []
              for item in items:
                  try:
                      results.append(_list_one(item))
                  except ListError as exc:
                      errors.append(ListItemError(path=item.path, error=str(exc)))
              return ListBatchResult(results=results, errors=errors)
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
```
/answer 6e34-4 allow
Control Request:
```yaml
id: 6e34-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    ids:
    - dvO55c|qVCPEH
  reason: "Imports prüfen (Path, os) für Dedupe-Implementierung"
```
/answer 6e34-5 allow
Control Request:
```yaml
id: 6e34-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: dvO55c|qVCPEH
        type: imports
        code: |-
          import os
          import re
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools._directories import normalize_directory
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
```
/answer 6e34-6 allow
ReasoningToken: 1096
Text:
 Path ist importiert. Jetzt Implementierung einfügen.
Control Request:
```yaml
id: 6e34-7
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: list
    position: before
    source: |+
      def _dedupe_hierarchy(items: list[ListItem]) -> list[ListItem]:
          """Remove items whose path lies below another item's path in the same directory hierarchy.

          Only the topmost item of each hierarchy is kept; items for separate hierarchies
          are left untouched. Original relative order of the kept items is preserved.
          """
          resolved = [(item, Path(item.path).resolve()) for item in items]
          resolved.sort(key=lambda pair: len(pair[1].parts))
          kept_paths: list[Path] = []
          kept_items: list[ListItem] = []
          for item, path in resolved:
              if any(path == kept or kept in path.parents for kept in kept_paths):
                  continue
              kept_paths.append(path)
              kept_items.append(item)
          kept_ids = {id(item) for item in kept_items}
          return [item for item in items if id(item) in kept_ids]


  reason: Hilfsfunktion zur Deduplizierung hierarchischer Pfade vor der list()-Funktion einfügen
```
/answer 6e34-7 allow
Control Request:
```yaml
id: 6e34-8
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: list
    old_text: |2-
              raise ListError("'items' must be a non-empty list.")
          results
    new_text: |2-
              raise ListError("'items' must be a non-empty list.")
          items = _dedupe_hierarchy(items)
          results
  reason: "Dedupe-Aufruf in list() einbauen, bevor die Items verarbeitet werden"
```
/answer 6e34-8 allow
Control Request:
```yaml
id: 6e34-9
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  reason: Syntax nach den Änderungen prüfen
```
/answer 6e34-9 allow
Result Stats: id=5780023f-1768-4c47-bdbc-7415714ce97a, total: 15136, in: 145, out: 2713, reason: 0, read: 56817, write: 12278
Fertig. Die Batch-Ebene von `list()` entfernt nun vor der Verarbeitung alle Items, deren Pfad innerhalb der Hierarchie eines anderen Items liegt (behält jeweils nur den obersten), während Items in getrennten Hierarchien unverändert bleiben.
