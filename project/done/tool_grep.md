Erstelle ein Tool auf Basis von Grep. Das Tool dient dem Retrieval.

- Beispiel für Tool mit Subprozess: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py`
- Eingabe ist ein absolutes Verzeichnis, eine Extended Regexp, exclude und include GLOB
- Es werden immer Dateien mit Treffern und Zeilennummern ausgegeben; Der Aufruf ist immer recursive
- STDERR und STDOUT werden entsprechend zurückgegeben; ebenfalls mit spill to file geschützt.
SystemInit:  id=eb70843c-e679-47bb-a55f-47ca63e33142, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: ea1b-1
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
  reason: "Referenzimplementierung für Subprozess-basiertes Tool ansehen, um Konventionen für neues Grep-Tool zu übernehmen."
```
/answer ea1b-1 allow
ReasoningToken: 27
Control Request:
```yaml
id: ea1b-2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py
  reason: Verstehen der run_process/pack_process_result API für die Grep-Tool-Implementierung.
```
/answer ea1b-2 allow
Control Request:
```yaml
id: ea1b-3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/registry.py
  reason: Verstehen der ToolDefinition/ToolResult/text_content API für die Grep-Tool-Implementierung.
```
/answer ea1b-3 deny das ist nicht notwendig
ReasoningToken: 47
Control Request:
```yaml
id: ea1b-4
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
  reason: "Konventionen für Subprozess-Tools mit Argumenten (z.B. cwd) ansehen, um Grep-Tool analog zu bauen."
```
/answer ea1b-4 allow
Control Request:
```yaml
id: ea1b-5
toolName: ask-user
arguments:
  question: "In welchem Verzeichnis soll das neue Grep-Tool angelegt werden (z. B. src/xy/ai/mcpc/tools/grep/__init__.py) und wo muss die Registrierung erfolgen, damit es im Server aktiv ist?"
  reason: "Ich möchte sicherstellen, dass Ablageort und Registrierungsstelle korrekt sind, bevor ich Dateien schreibe."
```
/answer ea1b-5 allow Der Vorschlag ist gut, ich importiere das Tool extern für die registrierung.
ReasoningToken: 156
Control Request:
```yaml
id: ea1b-6
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
  mode: replace
  reason: "Neues Grep-Retrieval-Tool anlegen, analog zu bash/python-Subprozess-Tools."
  content: |
    """Grep tool – recursive extended-regex search for retrieval."""
    from __future__ import annotations
    from pathlib import Path
    from typing import Any
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
    __all__ = ['GrepError', 'grep', 'GrepTool', 'register_grep_tool']
    _MAX_STREAM_CHARS = 3000

    class GrepError(Exception):
        """Raised when a grep search cannot be executed."""

    def grep(directory: str, pattern: str, *, exclude: str | None = None, include: str | None = None) -> ProcessResult:
        """Recursively search ``directory`` for ``pattern`` (extended regexp).

        Args:
            directory: Absolute path to the directory to search (must exist and be a directory).
            pattern: Extended regular expression (grep -E syntax).
            exclude: Glob of file names to exclude from the search, if given.
            include: Glob of file names to include in the search, if given.

        Returns:
            ProcessResult with:
                exit_code: 0 if matches were found, 1 if none were found, >=2 on grep error.
                stdout: Matching lines as 'path:line:content' (up to 3000 chars; see stdout_file if longer).
                stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
                stdout_file: Absolute path to temp file with full stdout if > 3000 chars.
                stderr_file: Absolute path to temp file with full stderr if > 3000 chars.

        Raises:
            GrepError: If directory is not absolute.
            GrepError: If directory does not exist or is not a directory.
            GrepError: If pattern is empty.
            GrepError: If grep binary cannot be launched.
        """
        directory_path = Path(directory)
        if not directory_path.is_absolute():
            raise GrepError('directory must be an absolute path.')
        if not directory_path.is_dir():
            raise GrepError('Directory not found or not a directory.')
        if not pattern:
            raise GrepError('pattern must not be empty.')

        cmd = ['grep', '--recursive', '--line-number', '--extended-regexp', '--binary-files=without-match', '--color=never']
        if include:
            cmd.append(f'--include={include}')
        if exclude:
            cmd.append(f'--exclude={exclude}')
        cmd += ['--', pattern, str(directory_path)]

        try:
            return run_process(cmd)
        except LaunchError as exc:
            raise GrepError(f'Failed to launch grep: {exc}') from exc

    class GrepTool(ToolDefinition):
        name = 'grep'
        title = 'Search files with grep'
        description = f"Recursively search a directory for lines matching an extended regular expression (grep -E). Returns matches as 'path:line:content', the exit code (0 = matches found, 1 = none found) and, if present, standard error output. As a safety limit, STDOUT/STDERR longer than {_MAX_STREAM_CHARS} characters are written to a temp file instead; the absolute file path is returned (as 'stdout_file'/'stderr_file') so it can be inspected further."
        input_schema = {
            'type': 'object',
            'properties': {
                'directory': {'type': 'string', 'description': 'Absolute path to the directory to search recursively.'},
                'pattern': {'type': 'string', 'description': 'Extended regular expression (grep -E syntax) to search for.'},
                'exclude': {'type': 'string', 'description': "Glob of file names to exclude from the search, e.g. '*.min.js'."},
                'include': {'type': 'string', 'description': "Glob of file names to include in the search, e.g. '*.py'."},
            },
            'required': ['directory', 'pattern'],
        }
        output_schema = {
            'type': 'object',
            'properties': {
                'exit_code': {'type': 'integer'},
                'stdout': {'type': 'string'},
                'stderr': {'type': 'string'},
                'stdout_file': {'type': 'string', 'description': 'Absolute path to a file containing the full STDOUT, present only if STDOUT exceeded the safety limit.'},
                'stderr_file': {'type': 'string', 'description': 'Absolute path to a file containing the full STDERR, present only if STDERR exceeded the safety limit.'},
            },
            'required': ['stdout'],
        }
        annotations = {'readOnlyHint': True, 'idempotentHint': True, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`grep` and pack the result into the MCP output schema."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = grep(args['directory'], args['pattern'], exclude=args.get('exclude'), include=args.get('include'))
            except GrepError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return pack_process_result(result, normalize_output=True, omit_zero_exit_code=True, max_stream_chars=_MAX_STREAM_CHARS)

    def register_grep_tool(registry: ToolRegistry) -> None:
        registry.register(GrepTool())
```
/answer ea1b-6 allow
ReasoningToken: 1059
Ich habe das Grep-Tool unter `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py` angelegt, analog zum bestehenden `bash`/`python`-Muster (Subprozess via `run_process`/`pack_process_result`).

Details:
- **Eingaben:** `directory` (absolut, muss existieren), `pattern` (Extended Regexp, `grep -E`), optional `include`/`exclude` (Glob je Dateiname).
- **Aufruf:** immer `--recursive --line-number --extended-regexp`, plus `--binary-files=without-match` (Binärdateien werden übersprungen statt Fehler/Müll zu liefern) und `--color=never`. `--` trennt Pattern von Pfad, damit Patterns mit führendem `-` nicht als Optionen interpretiert werden.
- **Ausgabe:** Treffer im Format `path:line:content` über `stdout`; `exit_code` 0 = Treffer, 1 = keine Treffer (kein Fehlerfall), ≥2 = grep-Fehler. STDOUT/STDERR werden bei >3000 Zeichen in eine Temp-Datei ausgelagert (`stdout_file`/`stderr_file`), gleiches Sicherheitsnetz wie bei `bash`.
- **Fehlerbehandlung:** `GrepError` bei relativem Pfad, fehlendem/kein Verzeichnis, leerem Pattern oder nicht startbarem `grep`-Binary.

Die Registrierung im globalen Tool-Registry hast du übernommen (externer Import), daher habe ich dort nichts geändert.
