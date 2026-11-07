Wir wollen das Fehlerverhalten im List Tool konstruktiver gestalten.

1. Wenn das Limit erreicht wird, soll zunächst versucht werden das Ergebnis auf einen Eintrag pro Verzeichnisebene zu reduzieren. Dieses wird dann mit einer Warnung auf zurückgegeben. Dabei wird pro Ebene ein Feld ergänzt, das die Gesamtanzahl Dateien pro Verzeichnisebene zeigt.

2. Überschreitet dies ebenfalls das Limit, wird der Baum in der tiefe reduziert. Die Warnung ändert sich und der Counter gibt die Anzahl sich unterhalb einer Verzeichnisebene Anzahl an Dateien aus. 

- List Tool: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py`
- Sprache ist englisch
- Der globale Hinweis zur Einschränkung und Nutzung von Patterns bleibt für beide Fälle erhalten.

Beispiel:
```yaml
id: b24a-2
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
      error: Too many entries (134) exceed the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/ast-engines
      error: Too many entries (135) exceed the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
SystemInit:  id=12563c56-3a6d-4bf7-a576-21d9a0f730af, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: b24a-25
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  reason: Struktur des List Tools verstehen
```
/answer b24a-25 allow
Control Request:
```yaml
id: b24a-26
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
/answer b24a-26 allow
ReasoningToken: 16
Control Request:
```yaml
id: b24a-27
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    ids:
    - JrBuUj|ISKZu0
    - ListError
    - ListItem
    - ListResult
    - ListItemError
    - ListBatchResult
    - list_one
    - list
    - ListTool
  reason: "Vollständigen Code des List Tools lesen, um Fehlerbehandlung und Limit-Logik zu verstehen"
```
/answer b24a-27 allow
Control Request:
```yaml
id: b24a-28
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: JrBuUj|ISKZu0
        type: statements
        code: "\"List tool – returns files below a directory, grouped by relative subdirectory like ``ls -R``, for a batch of items.\\n\\nWalks each given absolute directory recursively and returns all file paths\\n(files only, no directories), grouped by the relative directory they live in\\n(e.g. ``./src/pkg:`` followed by tab-indented file names), mirroring the\\noutput format of ``ls -R``. An optional regular expression can be supplied to\\nfilter the resulting files (matched against each file's path relative to the\\nrequested directory). Common VCS/build/cache directories (e.g. ``.git``) are\\nalways excluded. To keep results manageable, the number of matched files per\\nitem is capped; use ``pattern`` to narrow down large directories instead of\\nraising the limit.\\n\""
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
                  ListError: If more than ``_MAX_ENTRIES`` files match.
              """
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
              if match_count > _MAX_ENTRIES:
                  raise ListError(
                      f"Too many entries ({match_count}) exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")
              entries = []
              for rel_dir in sorted(groups):
                  header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                  entries.append(f'{header}:')
                  entries.extend(groups[rel_dir])
              return ListResult(path=item.path, entries=entries)
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
                result_serializer = lambda r: {'path': r.path, 'entries': r.entries}
                return handle_batch_tool(ctx, item_factory, list, ListError, result_serializer)
```
/answer b24a-28 allow
Control Request:
```yaml
id: b24a-29
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  text: _MAX_ENTRIES
  reason: Definition von _MAX_ENTRIES finden
```
/answer b24a-29 allow
Control Request:
```yaml
id: b24a-2a
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: ZrSLiQ|rTmMVq
        code: |-
          __all__ = [
              'ListError',
              'ListItem',
              'ListResult',
              'ListItemError',
              'ListBatchResult',
              'list',
              'ListTool',
              'register_list_tool']
          _MAX_ENTRIES = 50
          _EXCLUDED_DIRS = {
              '.git',
              '.hg',
              '.svn',
              '__pycache__',
              '.mypy_cache',
              '.pytest_cache',
              '.ruff_cache',
              '.tox',
              '.venv',
              'venv',
              'node_modules',
              '.idea',
              '.vscode',
              'dist',
              'build',
              '.cache'}
      - id: list_one.ultSa3|ec7R7F
        code: |-
          'List all files below the absolute directory ``item.path``, grouped like ``ls -R``.\n\n    Raises:\n        ListError: If path is not absolute.\n        ListError: If path does not exist or is not a directory.\n        ListError: If pattern is not a valid regular expression.\n        ListError: If more than ``_MAX_ENTRIES`` files match.\n    '
          dir_path = Path(item.path)
          if not dir_path.is_absolute():
              raise ListError('Path must be absolute.')
          dir_path = normalize_directory(dir_path)
      - id: list_one.nKyAmg|rqxOiX
        code: |-
          if match_count > _MAX_ENTRIES:
              raise ListError(
                  f"Too many entries ({match_count}) exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")
          entries = []
          for rel_dir in sorted(groups):
              header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
              entries.append(f'{header}:')
              entries.extend(groups[rel_dir])
          return ListResult(path=item.path, entries=entries)
```
/answer b24a-2a allow
ReasoningToken: 4794
Control Request:
```yaml
id: b24a-2b
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: ListResult
    source: |-
      @dataclass(frozen=True)
      class ListResult:
          """Result of listing a single directory, mirroring its input path for result association."""
          path: str
          entries: list[str]
          warning: str | None = None
  reason: Optionales warning Feld zu ListResult hinzufügen
```
/answer b24a-2b allow
Control Request:
```yaml
id: b24a-2c
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: list_one.nKyAmg|rqxOiX
    source: |-
      if match_count <= _MAX_ENTRIES:
          entries = []
          for rel_dir in sorted(groups):
              header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
              entries.append(f'{header}:')
              entries.extend(groups[rel_dir])
          return ListResult(path=item.path, entries=entries)

      # Step 1: collapse each directory level to a single representative entry,
      # annotated with the total file count for that level.
      level1_counts = {rel_dir: len(files) for rel_dir, files in groups.items()}
      if len(level1_counts) <= _MAX_ENTRIES:
          entries = []
          for rel_dir in sorted(groups):
              header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
              count = level1_counts[rel_dir]
              entries.append(f"{header}: ({count} file{'s' if count != 1 else ''})")
              entries.append(groups[rel_dir][0])
          warning = (
              f"Result reduced to one entry per directory level because the total number of entries "
              f"({match_count}) exceeds the limit of {_MAX_ENTRIES}. Each directory header now shows the "
              f"total file count for that level. Narrow down the result using the 'pattern' regular "
              f"expression parameter.")
          return ListResult(path=item.path, entries=entries, warning=warning)

      # Step 2: the per-level reduction still exceeds the limit, so collapse the
      # tree depth and aggregate file counts below each top-level directory.
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
              f"Too many entries even after reducing by directory level ({len(top_counts)}) exceed the "
              f"limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression "
              f"parameter.")

      entries = []
      for key in sorted(top_counts):
          header = key if key == '.' else './' + key.replace(os.sep, '/')
          count = top_counts[key]
          entries.append(f"{header}: ({count} file{'s' if count != 1 else ''} below)")
      warning = (
          f"Result reduced to top-level directories because grouping by directory level still exceeds "
          f"the limit of {_MAX_ENTRIES} ({len(level1_counts)} directories). Each header now shows the "
          f"total number of files below that directory. Narrow down the result using the 'pattern' "
          f"regular expression parameter.")
      return ListResult(path=item.path, entries=entries, warning=warning)
  reason: "Stufenweise Reduktion implementieren: erst eine Zeile pro Verzeichnisebene mit Zähler, dann Reduktion der Baumtiefe mit aggregiertem Zähler"
```
/answer b24a-2c allow
Control Request:
```yaml
id: b24a-2d
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    old_text: "result_serializer = lambda r: {'path': r.path, 'entries': r.entries}"
    new_text: "result_serializer = lambda r: {'path': r.path, 'entries': r.entries, **({'warning': r.warning} if r.warning else {})}"
  reason: "Warning-Feld im Tool-Ergebnis ausgeben, wenn gesetzt"
```
/answer b24a-2d allow
Control Request:
```yaml
id: b24a-2e
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  reason: Syntax prüfen nach den Änderungen
```
/answer b24a-2e allow
Control Request:
```yaml
id: b24a-2f
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    old_text: |2-
              ListError: If more than ``_MAX_ENTRIES`` files match.
          """
    new_text: |2-
              ListError: If more than ``_MAX_ENTRIES`` entries remain even after the result has been
                  reduced to one entry per directory level and then to top-level directories only.
          """
          # When the flat file count exceeds the limit, the result is progressively reduced:
          # first to one entry per directory level (with a per-level file count), then, if
          # that still exceeds the limit, to top-level directories only (with an aggregated
          # file count below each). Both reduced results are returned with a warning.
  reason: Docstring/Kommentar zur neuen Reduktionslogik ergänzen
```
/answer b24a-2f allow
Result Stats: id=12563c56-3a6d-4bf7-a576-21d9a0f730af, total: 27424, in: 147, out: 8311, reason: 105, read: 93804, write: 18966
ReasoningToken: 105
Text:
 Jetzt den Serializer im ListTool anpassen, damit `warning` mit ausgegeben wird.
Text:
 Jetzt validieren und die Docstring der `_list_one`-Funktion aktualisieren.
Fertig. Zusammenfassung der Änderungen in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py`:

1. `ListResult` hat ein neues optionales Feld `warning`.
2. `_list_one` reduziert bei Limitüberschreitung zunächst auf einen Eintrag pro Verzeichnisebene (mit Dateianzahl je Ebene) und gibt dies mit Warnung zurück.
3. Reicht das nicht, wird die Baumtiefe auf Top-Level-Verzeichnisse reduziert, mit aggregiertem Dateizähler je Teilbaum und entsprechend angepasster Warnung.
4. Beide Warnungen enthalten weiterhin den Hinweis auf das `pattern`-Parameter. Erst wenn auch die Top-Level-Reduktion das Limit überschreitet, wird wie bisher ein `ListError` ausgelöst.
5. `ListTool.handle` gibt `warning` im Ergebnis mit aus, falls gesetzt.
