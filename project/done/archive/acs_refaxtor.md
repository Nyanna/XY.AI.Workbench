Refactor das Tool `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py` als Vorlage für alle weiteren.
Die Tool-Funktion soll später als Python Modul zur Verfügung gestellt werden. Dafür ist es notwendig die Funktion vom Ein- und Ausgabeschema zu abstrahieren. Es benötigt also eine Delegate Methode die die Funktion über eine korrekte Python Signatur und Typen anspricht, die Argumente weiterreicht, eine entsprechende Rückgabe macht und diese wieder in das MCP Ausgabeschema zurückpackt(Komplexes Resultobjekt, exportiert).
SystemInit:  id=debf8043-29a7-4c35-af71-f533e97ae5cf, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 613c-61
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py
  reason: "Aktuellen Code des insert-Tools lesen, um Refactoring als Vorlage zu planen"
```
/answer 613c-61 allow
Control Request:
```yaml
id: 613c-62
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  reason: "Überblick über andere Tools, um Konventionen und Struktur zu verstehen"
```
/answer 613c-62 allow
Control Request:
```yaml
id: 613c-63
result:
  structuredContent:
    entries:
    - CHECKLIST.md
    - __init__.py
    - _text_match.py
    - agent/__init__.py
    - agent/profiles.py
    - ask-user/__init__.py
    - ast/__init__.py
    - ast/convenience.py
    - ast/core.py
    - ast/crud.py
    - ast/file_ops.py
    - ast/layers.py
    - ast/node_replace_block.py
    - ast/outline.py
    - ast/script.py
    - ast/validate.py
    - bash/__init__.py
    - builtin.py
    - change/__init__.py
    - colgrep/__init__.py
    - file-stats/__init__.py
    - insert/__init__.py
    - list/__init__.py
    - markdown/__init__.py
    - mcp/__init__.py
    - mcp/bridge.py
    - mcp/client.py
    - mcp/context7.py
    - mcp/exa.py
    - mcp/github.py
    - openalex/__init__.py
    - process.py
    - python/__init__.py
    - read/__init__.py
    - replace_block/__init__.py
    - replace_chars/__init__.py
    - replace_lines/__init__.py
    - skills/__init__.py
    - write/__init__.py
```
/answer 613c-63 allow
ReasoningToken: 11
Control Request:
```yaml
id: 613c-64
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  reason: Registry/ToolContext/ToolResult Struktur verstehen für Delegate-Methode Design
```
/answer 613c-64 deny was du wissen musst ist die handle methode
ReasoningToken: 43
Control Request:
```yaml
id: 613c-65
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/CHECKLIST.md
  reason: Prüfen ob es Vorgaben/Konventionen für Tool-Refactoring gibt
```
/answer 613c-65 deny gibt es nicht
ReasoningToken: 60
Control Request:
```yaml
id: 613c-66
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py
  mode: replace
  reason: "Refactoring: Kernlogik in exportierte Funktion `insert` mit typisierter Signatur und Result-Dataclass auslagern, handle() als schlanke Delegate-Methode zum MCP-Schema"
  content: |
    """Insert tool – inserts text at a character offset inside an existing file."""

    from __future__ import annotations

    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = [
        "InsertError",
        "InsertResult",
        "insert",
        "InsertTool",
        "register_insert_tool",
    ]


    class InsertError(Exception):
        """Raised when an insert operation cannot be performed."""


    @dataclass(frozen=True)
    class InsertResult:
        result: str


    def insert(path: str, offset: int, content: str) -> InsertResult:
        """Insert ``content`` at the zero-based character ``offset`` of the file at ``path``."""
        file_path = Path(path)
        if not file_path.is_absolute():
            raise InsertError("Path must be absolute.")
        if not file_path.exists():
            raise InsertError("File not found.")
        if not file_path.is_file():
            raise InsertError("Not a regular file.")

        try:
            text = file_path.read_text(encoding="utf-8")
            if offset > len(text):
                raise InsertError("Offset is beyond end of file.")
            new_text = text[:offset] + content + text[offset:]
            file_path.write_text(new_text, encoding="utf-8")
        except OSError as exc:
            raise InsertError(f"Insert failed: {exc}") from exc

        return InsertResult(result="success")


    class InsertTool(ToolDefinition):
        name = "insert"
        title = "Insert into file"
        description = (
            "Insert text at a specific character offset inside an existing file. "
            "The offset is zero-based and refers to the UTF-8 decoded content of the file. "
            "All existing content at and after the offset is shifted right."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the file to modify.",
                },
                "offset": {
                    "type": "integer",
                    "description": "Zero-based character offset at which to insert the new content.",
                    "minimum": 0,
                },
                "content": {
                    "type": "string",
                    "description": "Text to insert at the given offset.",
                },
            },
            "required": ["path", "offset", "content"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
                    "description": "``success`` on success.",
                },
            },
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`insert`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = insert(
                    path=args["path"],
                    offset=args["offset"],
                    content=args["content"],
                )
            except InsertError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(
                structured_content={"result": result.result},
                auto_approve=True,
            )


    def register_insert_tool(registry: ToolRegistry) -> None:
        registry.register(InsertTool())
```
/answer 613c-66 allow
Control Request:
```yaml
id: 613c-67
toolName: python-ast-validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py
  reason: Syntaxprüfung des refaktorierten Tools
```
/answer 613c-67 allow
ReasoningToken: 1139
Refactoring abgeschlossen und syntaktisch geprüft. Als Vorlage für alle weiteren Tools gilt jetzt folgendes Muster (am Beispiel `insert`):

**Struktur:**
1. **`InsertError`** – domänenspezifische Exception statt direkter `ToolResult`-Fehler in der Logik.
2. **`InsertResult`** (frozen `dataclass`) – typisiertes, exportiertes Rückgabeobjekt der Kernfunktion.
3. **`insert(path: str, offset: int, content: str) -> InsertResult`** – die eigentliche, schemafreie Funktion mit sauberer Python-Signatur/Typen, wirft `InsertError` statt MCP-`ToolResult`-Objekte zurückzugeben. Diese Funktion ist eigenständig importier- und nutzbar (`from xy.ai.mcpc.tools.insert import insert`).
4. **`InsertTool.handle()`** – reine Delegate-Methode: entpackt `ctx.arguments`, ruft `insert(...)` auf, fängt `InsertError` ab und packt Erfolg/Fehler zurück ins MCP-Ausgabeschema (`ToolResult`/`structured_content`).
5. **`__all__`** exportiert `InsertError`, `InsertResult`, `insert`, `InsertTool`, `register_insert_tool` – klar definierte öffentliche API des Moduls.

Damit ist die Trennung: I/O-Schema (JSON-Schema, `ToolContext`/`ToolResult`) lebt ausschließlich in `InsertTool`, die fachliche Logik lebt in der freien Funktion `insert` mit eigenem typisierten Result-Objekt. Dieses Muster lässt sich 1:1 auf die übrigen Tools (`change`, `bash`, `list`, `read`, `write`, `replace_block`, `replace_chars`, `replace_lines`, `file-stats`, `python`, `ast/*`, `mcp/*` etc.) übertragen.
Übertrage es auf alle übrigen Tools die ToolDefinition implementieren.
SystemInit:  id=debf8043-29a7-4c35-af71-f533e97ae5cf, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 613c-68
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  reason: "Alle Tool-Dateien auflisten, um ToolDefinition-Implementierungen zu identifizieren"
```
/answer 613c-68 deny verwende ein Grep nach ToolDefinition
ReasoningToken: 60
Control Request:
```yaml
id: 613c-69
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  script: grep -rl "ToolDefinition)" .
  reason: "Alle Dateien finden, die ToolDefinition implementieren, um sie nach dem insert-Muster zu refaktorieren"
```
/answer 613c-69 allow
Control Request:
```yaml
id: 613c-6a
result:
  structuredContent:
    stdout: |
      ./change/__init__.py
      ./ast/script.py
      ./ast/outline.py
      ./ast/file_ops.py
      ./ast/validate.py
      ./replace_chars/__init__.py
      ./file-stats/__init__.py
      ./insert/__init__.py
      ./bash/__init__.py
      ./replace_lines/__init__.py
      ./replace_block/__init__.py
      ./colgrep/__init__.py
      ./markdown/__init__.py
      ./ask-user/__init__.py
      ./python/__init__.py
      ./list/__init__.py
      ./write/__init__.py
      ./read/__init__.py
```
/answer 613c-6a allow
Control Request:
```yaml
id: 613c-6b
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  script: grep -rn "class .*ToolDefinition" . ; echo "---"; grep -rln "ToolDefinition" mcp/ ast/crud.py ast/layers.py ast/core.py ast/convenience.py ast/node_replace_block.py openalex/ agent/ skills/ 2>/dev/null
  reason: "Genaue Klassennamen und weitere ToolDefinition-Vorkommen in Unterordnern (mcp, ast helpers, openalex, agent, skills) finden"
```
/answer 613c-6b allow
Control Request:
```yaml
id: 613c-6c
result:
  structuredContent:
    exit_code: 1
    stdout: |
      ./change/__init__.py:12:class ChangeTool(ToolDefinition):
      ./ast/script.py:29:class ScriptTool(ToolDefinition):
      ./ast/outline.py:108:class OutlineTool(ToolDefinition):
      ./ast/file_ops.py:15:class CreateFileTool(ToolDefinition):
      ./ast/file_ops.py:53:class DeleteFileTool(ToolDefinition):
      ./ast/validate.py:26:class ValidateTool(ToolDefinition):
      ./replace_chars/__init__.py:11:class ReplaceCharsTool(ToolDefinition):
      ./file-stats/__init__.py:94:class FileStatsTool(ToolDefinition):
      ./insert/__init__.py:51:class InsertTool(ToolDefinition):
      ./bash/__init__.py:14:class BashTool(ToolDefinition):
      ./replace_lines/__init__.py:15:class ReplaceLinesTool(ToolDefinition):
      ./replace_block/__init__.py:12:class ReplaceBlockTool(ToolDefinition):
      ./colgrep/__init__.py:66:class ColgrepTool(ToolDefinition):
      ./markdown/__init__.py:65:class MarkdownTool(ToolDefinition):
      ./ask-user/__init__.py:25:class AskUserTool(ToolDefinition):
      ./python/__init__.py:12:class PythonTool(ToolDefinition):
      ./list/__init__.py:43:class ListTool(ToolDefinition):
      ./write/__init__.py:11:class WriteTool(ToolDefinition):
      ./read/__init__.py:32:class ReadTool(ToolDefinition):
      ---
```
/answer 613c-6c allow
ReasoningToken: 208
Control Request:
```yaml
id: 613c-6d
toolName: python-ast-outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/layers.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/convenience.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/node_replace_block.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  reason: "Struktur der weiteren ast-Hilfsdateien verstehen, um festzustellen ob dort weitere ToolDefinition-Subklassen (evtl. dynamisch generiert) existieren"
```
/answer 613c-6d deny nein existieren nicht
Control Request:
```yaml
id: 613c-6e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/change/__init__.py
  reason: Vollständigen Code des change-Tools lesen für Refactoring
```
/answer 613c-6e allow
ReasoningToken: 136
Control Request:
```yaml
id: 613c-6f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_chars/__init__.py
  reason: Vollständigen Code des replace_chars-Tools lesen für Refactoring
```
/answer 613c-6f allow
Control Request:
```yaml
id: 613c-70
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_lines/__init__.py
  reason: Vollständigen Code des replace_lines-Tools lesen für Refactoring
```
/answer 613c-70 allow
Control Request:
```yaml
id: 613c-71
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py
  reason: Vollständigen Code des replace_block-Tools lesen für Refactoring
```
/answer 613c-71 allow
Control Request:
```yaml
id: 613c-72
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
  reason: Vollständigen Code des write-Tools lesen für Refactoring
```
/answer 613c-72 allow
Control Request:
```yaml
id: 613c-73
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  reason: Vollständigen Code des read-Tools lesen für Refactoring
```
/answer 613c-73 allow
Control Request:
```yaml
id: 613c-74
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  reason: Vollständigen Code des list-Tools lesen für Refactoring
```
/answer 613c-74 allow
Control Request:
```yaml
id: 613c-75
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file-stats/__init__.py
  reason: Vollständigen Code des file-stats-Tools lesen für Refactoring
```
/answer 613c-75 allow
Control Request:
```yaml
id: 613c-76
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
  reason: Vollständigen Code des bash-Tools lesen für Refactoring
```
/answer 613c-76 allow
Control Request:
```yaml
id: 613c-77
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
  reason: Vollständigen Code des python-Tools lesen für Refactoring
```
/answer 613c-77 allow
Control Request:
```yaml
id: 613c-78
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  reason: Vollständigen Code des colgrep-Tools lesen für Refactoring
```
/answer 613c-78 allow
Control Request:
```yaml
id: 613c-79
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/markdown/__init__.py
  reason: Vollständigen Code des markdown-Tools lesen für Refactoring
```
/answer 613c-79 allow
Control Request:
```yaml
id: 613c-7a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ask-user/__init__.py
  reason: Vollständigen Code des ask-user-Tools lesen für Refactoring
```
/answer 613c-7a allow
Control Request:
```yaml
id: 613c-7b
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/file_ops.py
  reason: Vollständigen Code des ast/file_ops-Tools lesen für Refactoring
```
/answer 613c-7b allow
Control Request:
```yaml
id: 613c-7c
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py
  reason: Vollständigen Code des ast/validate-Tools lesen für Refactoring
```
/answer 613c-7c allow
Control Request:
```yaml
id: 613c-7d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  reason: Vollständigen Code des ast/outline-Tools lesen für Refactoring
```
/answer 613c-7d allow
Control Request:
```yaml
id: 613c-7e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: Vollständigen Code des ast/script-Tools lesen für Refactoring
```
/answer 613c-7e allow
Control Request:
```yaml
id: 613c-7f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py
  reason: "run_capture Signatur verstehen, um bash/python/markdown Tools korrekt zu refaktorieren"
```
/answer 613c-7f allow
Control Request:
```yaml
id: 613c-80
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: "core-Modul der AST-Tools verstehen (AstError, require_path, CACHE) für Refactoring von file_ops/validate/outline/script"
```
/answer 613c-80 allow
ReasoningToken: 114
ReasoningToken: 1326
Control Request:
```yaml
id: 613c-81
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py
  mode: replace
  reason: "process.py in eine schemafreie Ausführungsfunktion (run_process/ProcessResult/LaunchError) und eine separate MCP-Packfunktion (pack_process_result) aufteilen, damit bash/python/markdown darauf als Delegate aufbauen können"
  content: |
    """Shared subprocess execution for the stream-capturing tools.

    ``bash``, ``python`` and ``markdown`` all do the same thing: run a child
    process and report ``exit_code`` + its two streams. Centralising it here
    guarantees they decode child output identically to every other stream in
    MCPC — **UTF-8 with ``errors="replace"``**.

    Why this matters
    ----------------
    ``subprocess.run(..., text=True)`` alone decodes with the *ambient locale*
    encoding and the **strict** error handler.  A child that writes bytes which are
    not valid in that encoding (a stray ``\\xff``, latin-1 output, a truncated
    multibyte sequence) makes the *decode* raise :class:`UnicodeDecodeError` while
    capturing — after the work already ran — and the tool aborts with an internal
    error instead of returning what the process produced.  Forcing
    ``encoding="utf-8", errors="replace"`` makes stream capture total: undecodable
    bytes become U+FFFD and the exit code / output are always returned.

    The module is split in two layers:

    * :func:`run_process` — schema-free execution, returns a plain
      :class:`ProcessResult` or raises :class:`LaunchError`. This is the part a
      tool module exposes for programmatic (non-MCP) use.
    * :func:`pack_process_result` — MCP-specific packing of a
      :class:`ProcessResult` into a :class:`~xy.ai.mcpc.registry.ToolResult`
      (stream normalisation, spill-to-file safety limit, ``exit_code`` omission).
      This belongs to a tool's ``handle`` method, not its delegate function.
    """

    from __future__ import annotations

    import os
    import re
    import subprocess
    import tempfile
    from dataclasses import dataclass
    from typing import Any

    from ..registry import ToolResult, text_content

    _BLANK_RUN_RE = re.compile(r"[ \t]+$", re.MULTILINE)
    _MULTI_BLANK_RE = re.compile(r"\n{3,}")


    class LaunchError(Exception):
        """Raised when the child process could not be started."""


    @dataclass(frozen=True)
    class ProcessResult:
        exit_code: int
        stdout: str
        stderr: str


    def run_process(
        cmd: list[str],
        *,
        cwd: str | os.PathLike[str] | None = None,
        stdin: str | None = None,
    ) -> ProcessResult:
        """Run *cmd* to completion and return its captured result.

        Raises :class:`LaunchError` if the executable cannot be started.
        """
        try:
            proc = subprocess.run(
                cmd,
                input=stdin,
                cwd=os.fspath(cwd) if cwd is not None else None,
                capture_output=True,
                encoding="utf-8",
                errors="replace",
            )
        except OSError as exc:
            raise LaunchError(str(exc)) from exc

        return ProcessResult(exit_code=proc.returncode, stdout=proc.stdout or "", stderr=proc.stderr or "")


    def _normalize_stream(text: str) -> str:
        """Improve compatibility with YAML block scalars.

        * Lines that contain only whitespace are reduced to a bare line break
          (trailing spaces/tabs on otherwise empty lines are stripped).
        * Successive blank lines are collapsed to a single blank line.
        """
        if not text:
            return text
        normalized = _BLANK_RUN_RE.sub("", text)
        normalized = _MULTI_BLANK_RE.sub("\n\n", normalized)
        return normalized


    def _spill_to_file(text: str, label: str) -> str:
        """Write *text* to a fresh temp file and return its absolute path.

        Used as a safety limit: when a captured stream grows too large to be
        returned inline, it is persisted to disk instead so the caller can
        continue operating on it (e.g. via the ``read``/``bash`` tools) without
        the full content ever passing through the structured result.
        """
        fd, path = tempfile.mkstemp(prefix=f"mcpc-{label}-", suffix=".log")
        try:
            with os.fdopen(fd, "w", encoding="utf-8") as fh:
                fh.write(text)
        except BaseException:
            os.close(fd)
            raise
        return path


    def pack_process_result(
        result: ProcessResult,
        *,
        normalize_output: bool = False,
        omit_zero_exit_code: bool = False,
        max_stream_chars: int | None = None,
    ) -> ToolResult:
        """Pack a :class:`ProcessResult` into the MCP output schema.

        * ``normalize_output`` — when ``True``, post-process STDOUT/STDERR to
          improve YAML block-scalar compatibility (see :func:`_normalize_stream`).
        * ``omit_zero_exit_code`` — when ``True``, ``exit_code`` is left out of the
          result entirely if the process exited with code ``0``.
        * ``max_stream_chars`` — safety limit on the number of characters of
          STDOUT/STDERR returned inline.  When a stream exceeds this limit, its
          full content is written to a temp file instead and the structured
          result contains the absolute path (``stdout_file``/``stderr_file``) in
          place of the raw text, so the caller can keep operating on it (e.g.
          with the ``read`` tool) without the oversized content ever passing
          through the result payload.  ``None`` (the default) disables the
          limit.

        ``stdout`` is always present; ``stderr`` is included whenever it is
        non-empty. The result carries no separate text content block —
        ``structured_content`` alone conveys STDOUT/STDERR, avoiding duplication.
        ``is_error`` mirrors a non-zero exit code.
        """
        stdout = result.stdout
        stderr = result.stderr
        if normalize_output:
            stdout = _normalize_stream(stdout)
            stderr = _normalize_stream(stderr)

        content: list[dict[str, Any]] = []
        structured: dict[str, Any] = {}
        if not omit_zero_exit_code or result.exit_code != 0:
            structured["exit_code"] = result.exit_code

        if max_stream_chars is not None and len(stdout) > max_stream_chars:
            stdout_file = _spill_to_file(stdout, "stdout")
            content.append(
                text_content(
                    f"Full output written to file ({len(stdout)} characters). "
                    f"Before loading the file, reduce the content to what is strictly needed: "
                    f"use targeted commands (grep, head, tail, awk) to extract only the relevant parts."
                    f"Only load the file with `file-read` once the output is already narrowed down to the essential information."
                )
            )
            structured["stdout_file"] = stdout_file
        else:
            structured["stdout"] = stdout

        if stderr:
            if max_stream_chars is not None and len(stderr) > max_stream_chars:
                stderr_file = _spill_to_file(stderr, "stderr")
                content.append(
                    text_content(
                        f"Full output written to file ({len(stdout)} characters). "
                        f"Before loading the file, reduce the content to what is strictly needed: "
                        f"use targeted commands (grep, head, tail, awk) to extract only the relevant parts."
                        f"Only load the file with `file-read` once the output is already narrowed down to the essential information."
                    )
                )
                structured["stderr_file"] = stderr_file
            else:
                structured["stderr"] = stderr

        # Simple success with auto_approve when exit code is 0 and both streams are empty
        if result.exit_code == 0 and not stdout and not stderr:
            return ToolResult(structured_content={"result": "success"}, auto_approve=True)

        return ToolResult(
            content=content,
            structured_content=structured,
            is_error=result.exit_code != 0 and bool(stderr),
        )
```
/answer 613c-81 allow
Control Request:
```yaml
id: 613c-82
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
  mode: replace
  reason: "bash-Tool nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktion bash() + BashError + BashResult, handle() als Delegate"
  content: |
    """Bash tool – executes a shell script inside a specified working directory."""

    from __future__ import annotations

    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
    from ..process import LaunchError, ProcessResult, pack_process_result, run_process

    __all__ = ["BashError", "bash", "BashTool", "register_bash_tool"]

    _MAX_STREAM_CHARS = 3000


    class BashError(Exception):
        """Raised when a Bash script cannot be executed."""


    def bash(cwd: str, script: str) -> ProcessResult:
        """Run ``script`` with ``bash -c`` inside the absolute directory ``cwd``."""
        cwd_path = Path(cwd)
        if not cwd_path.is_absolute():
            raise BashError("cwd must be an absolute path.")
        if not cwd_path.is_dir():
            raise BashError("Working directory not found or not a directory.")

        try:
            return run_process(["bash", "-c", script], cwd=cwd_path)
        except LaunchError as exc:
            raise BashError(f"Failed to launch bash: {exc}") from exc


    class BashTool(ToolDefinition):
        name = "bash"
        title = "Run Bash script"
        description = (
            "Execute a Bash script in the specified working directory. "
            "Returns the exit code, standard output and, if present, standard error output. "
            f"As a safety limit, STDOUT/STDERR longer than {_MAX_STREAM_CHARS} characters are "
            "written to a temp file instead; the absolute file path is returned "
            "(as 'stdout_file'/'stderr_file') so it can be inspected further."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "cwd": {
                    "type": "string",
                    "description": "Absolute path to the working directory in which to run the script.",
                },
                "script": {
                    "type": "string",
                    "description": "Bash script content to execute.",
                },
            },
            "required": ["cwd", "script"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "exit_code": {"type": "integer"},
                "stdout": {"type": "string"},
                "stderr": {"type": "string"},
                "stdout_file": {
                    "type": "string",
                    "description": (
                        "Absolute path to a file containing the full STDOUT, "
                        "present only if STDOUT exceeded the safety limit."
                    ),
                },
                "stderr_file": {
                    "type": "string",
                    "description": (
                        "Absolute path to a file containing the full STDERR, "
                        "present only if STDERR exceeded the safety limit."
                    ),
                },
            },
            "required": ["stdout"],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": True}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`bash` and pack the result into the MCP output schema."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = bash(cwd=args["cwd"], script=args["script"])
            except BashError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return pack_process_result(
                result,
                normalize_output=True,
                omit_zero_exit_code=True,
                max_stream_chars=_MAX_STREAM_CHARS,
            )


    def register_bash_tool(registry: ToolRegistry) -> None:
        registry.register(BashTool())
```
/answer 613c-82 allow
Control Request:
```yaml
id: 613c-83
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
  mode: replace
  reason: "python-Tool nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktion run_python() + PythonError, handle() als Delegate"
  content: |
    """Python tool – executes a Python script directly from context (no file)."""

    from __future__ import annotations

    import sys
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
    from ..process import LaunchError, ProcessResult, pack_process_result, run_process

    __all__ = ["PythonError", "run_python", "PythonTool", "register_python_tool"]


    class PythonError(Exception):
        """Raised when a Python script cannot be executed."""


    def run_python(script: str) -> ProcessResult:
        """Feed ``script`` to a fresh Python interpreter on standard input."""
        try:
            return run_process([sys.executable, "-"], stdin=script)
        except LaunchError as exc:
            raise PythonError(f"Failed to launch Python: {exc}") from exc


    class PythonTool(ToolDefinition):
        name = "python"
        title = "Run Python script"
        description = (
            "Execute a Python script passed directly as content, without writing "
            "a script file. The script is fed to the interpreter on standard input. "
            "Returns the exit code, standard output and, if present, standard error output."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "script": {
                    "type": "string",
                    "description": "Python script content to execute.",
                },
            },
            "required": ["script"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "exit_code": {"type": "integer"},
                "stdout": {"type": "string"},
                "stderr": {"type": "string"},
            },
            "required": ["exit_code", "stdout"],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": True}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`run_python` and pack the result into the MCP output schema."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = run_python(args["script"])
            except PythonError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return pack_process_result(result)


    def register_python_tool(registry: ToolRegistry) -> None:
        registry.register(PythonTool())
```
/answer 613c-83 allow
Control Request:
```yaml
id: 613c-84
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/markdown/__init__.py
  mode: replace
  reason: "markdown-Tool nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktion run_markdown() + MarkdownError, handle() als Delegate inkl. Konfigurationsauflösung"
  content: |
    """Markdown tool – AST-based reading/writing/transforming of Markdown files.

    The tool runs a TypeScript (ESM) script inside a pre-provisioned Node.js package
    environment that exposes ``remark``, ``remark-behead`` and ``remark-frontmatter``
    (via a local ``createRemark`` helper).  The script is handed to
    ``node --input-type=module`` on standard input and executed with the environment
    directory as its working directory, so bare package imports resolve.
    """

    from __future__ import annotations

    from pathlib import Path
    from typing import Any

    from ...config import ServerConfig
    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
    from ..process import LaunchError, ProcessResult, pack_process_result, run_process

    __all__ = ["MarkdownError", "run_markdown", "MarkdownTool", "register_markdown_tool"]

    _EXAMPLE = """\
    import { read, write } from 'to-vfile';
    import { createRemark } from './remark.js';
    import { visit } from 'unist-util-visit';

    const processor = createRemark({
      // frontmatter: true, // if required
      // behead: { depth: 1 }, // if required
    });

    processor.use(() => (tree, file) => {
      // insert code here
    });

    // read file – replace 'path/to/file.md' with the actual file path
    const file = await read('path/to/file.md');

    // parse to AST
    const tree = await processor.run(processor.parse(file), file);

    // Extract headings
    const headings = [];
    visit(tree, 'heading', (node) => {
        headings.push({
        depth: node.depth,
        text: node.children.map(c => c.value || c.children?.map(x => x.value).join('') || '').join('').trim()
        });
    });

    // format output
    await processor.process(file);
    file.path = 'path/to/file.md';
    await write(file);

    console.log(String("Done"));
    """

    _DESCRIPTION = (
        "AST-based reading, writing, modifying and transforming of Markdown files. "
        "Provide a TypeScript (ESM) script that uses `remark` (with `remark-behead` "
        "and `remark-frontmatter` available) to operate on Markdown. "
        "Returns the exit code, standard output and, if present, standard error.\n\n"
        "Follow this pattern:\n\n```typescript\n" + _EXAMPLE + "```"
    )


    class MarkdownError(Exception):
        """Raised when a Markdown (remark) script cannot be executed."""


    def run_markdown(script: str, *, env_dir: str) -> ProcessResult:
        """Run ``script`` against the remark environment rooted at ``env_dir``."""
        cwd = Path(env_dir)
        if not cwd.is_dir():
            raise MarkdownError(f"Markdown environment not found: {cwd}")

        try:
            return run_process(["node", "--input-type=module"], cwd=cwd, stdin=script)
        except LaunchError as exc:
            raise MarkdownError(f"Failed to launch node: {exc}") from exc


    class MarkdownTool(ToolDefinition):
        name = "markdown"
        title = "Run Markdown (remark) script"
        description = _DESCRIPTION
        input_schema = {
            "type": "object",
            "properties": {
                "script": {
                    "type": "string",
                    "description": (
                        "TypeScript (ESM) script content to execute against the "
                        "remark environment."
                    ),
                },
            },
            "required": ["script"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "exit_code": {"type": "integer"},
                "stdout": {"type": "string"},
                "stderr": {"type": "string"},
            },
            "required": ["exit_code", "stdout"],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": True}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`run_markdown` and pack the result into the MCP output schema."""
            args: dict[str, Any] = ctx.arguments
            config = ctx.services.config if ctx.services is not None else ServerConfig()
            try:
                result = run_markdown(args["script"], env_dir=config.markdown_env_dir)
            except MarkdownError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return pack_process_result(result)


    def register_markdown_tool(registry: ToolRegistry) -> None:
        registry.register(MarkdownTool())
```
/answer 613c-84 allow
ReasoningToken: 3026
Text:
 Jetzt die drei darauf aufbauenden Tools:
Control Request:
```yaml
id: 613c-85
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/change/__init__.py
  mode: replace
  reason: "change-Tool nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktion change() + ChangeError/ChangeResult, handle() als Delegate"
  content: |
    """Change tool – replaces the block between start/end markers (both inclusive)."""

    from __future__ import annotations

    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
    from .._text_match import find as find_text

    __all__ = ["ChangeError", "ChangeResult", "change", "ChangeTool", "register_change_tool"]


    class ChangeError(Exception):
        """Raised when a change operation cannot be performed."""


    @dataclass(frozen=True)
    class ChangeResult:
        result: str


    def change(path: str, start: str, end: str, content: str, exact: bool = False) -> ChangeResult:
        """Replace the text between the unique markers ``start`` and ``end`` (both inclusive)."""
        file_path = Path(path)
        if not file_path.is_absolute():
            raise ChangeError("Path must be absolute.")
        if not file_path.exists():
            raise ChangeError("File not found.")
        if not file_path.is_file():
            raise ChangeError("Not a regular file.")

        text = file_path.read_text(encoding="utf-8")

        start_match = find_text(text, start, exact=exact)
        if start_match.count == 0:
            raise ChangeError("Start marker not found in file.")
        if start_match.count > 1:
            raise ChangeError(f"Start marker is ambiguous – found {start_match.count} occurrences in file.")

        end_match = find_text(text, end, exact=exact)
        if end_match.count == 0:
            raise ChangeError("End marker not found in file.")
        if end_match.count > 1:
            raise ChangeError(f"End marker is ambiguous – found {end_match.count} occurrences in file.")

        if end_match.start <= start_match.start:
            raise ChangeError("End marker must appear after start marker.")

        result_text = text[: start_match.start] + content + text[end_match.end :]

        try:
            file_path.write_text(result_text, encoding="utf-8")
        except OSError as exc:
            raise ChangeError(f"Write failed: {exc}") from exc

        return ChangeResult(result="success")


    class ChangeTool(ToolDefinition):
        name = "change"
        title = "Change file block"
        description = (
            "Replace the text between 'start' and 'end' (both included) with "
            "'content'. Each marker must occur exactly once in the file; "
            "'end' must come after 'start'. Repeat a marker inside 'content' "
            "to keep it. By default whitespace in 'start'/'end' is matched "
            "tolerantly; set 'exact' to require exact whitespace matching."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the target file.",
                },
                "start": {
                    "type": "string",
                    "description": "Unique substring marking the block's start (must occur exactly once).",
                },
                "end": {
                    "type": "string",
                    "description": "Unique substring marking the block's end (must occur exactly once, after 'start').",
                },
                "content": {
                    "type": "string",
                    "description": "Text that replaces the block, including where 'start'/'end' were.",
                },
                "exact": {
                    "type": "boolean",
                    "description": (
                        "If true, 'start'/'end' must match whitespace exactly. "
                        "If false (default), whitespace runs match any amount/kind of whitespace."
                    ),
                    "default": False,
                },
            },
            "required": ["path", "start", "end", "content"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
                    "description": "``success`` on success.",
                },
            },
            "required": [],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`change`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = change(
                    path=args["path"],
                    start=args["start"],
                    end=args["end"],
                    content=args["content"],
                    exact=args.get("exact", False),
                )
            except ChangeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(structured_content={"result": result.result}, auto_approve=True)


    def register_change_tool(registry: ToolRegistry) -> None:
        registry.register(ChangeTool())
```
/answer 613c-85 allow
Control Request:
```yaml
id: 613c-86
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_chars/__init__.py
  mode: replace
  reason: replace-chars-Tool nach dem insert-Vorlage-Muster refaktorieren
  content: |
    """Replace-chars tool – replaces a character range inside an existing file."""

    from __future__ import annotations

    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = [
        "ReplaceCharsError",
        "ReplaceCharsResult",
        "replace_chars",
        "ReplaceCharsTool",
        "register_replace_chars_tool",
    ]


    class ReplaceCharsError(Exception):
        """Raised when a replace-chars operation cannot be performed."""


    @dataclass(frozen=True)
    class ReplaceCharsResult:
        result: str


    def replace_chars(path: str, offset: int, length: int, content: str) -> ReplaceCharsResult:
        """Replace ``length`` characters starting at ``offset`` in the file at ``path`` with ``content``."""
        file_path = Path(path)
        if not file_path.is_absolute():
            raise ReplaceCharsError("Path must be absolute.")
        if not file_path.exists():
            raise ReplaceCharsError("File not found.")
        if not file_path.is_file():
            raise ReplaceCharsError("Not a regular file.")

        try:
            text = file_path.read_text(encoding="utf-8")
            file_len = len(text)
            if offset > file_len:
                raise ReplaceCharsError(
                    f"Offset {offset} is beyond end of file (file length: {file_len} characters)."
                )
            end = min(offset + length, file_len)
            result_text = text[:offset] + content + text[end:]
            file_path.write_text(result_text, encoding="utf-8")
        except OSError as exc:
            raise ReplaceCharsError(f"Replace failed: {exc}") from exc

        return ReplaceCharsResult(result="success")


    class ReplaceCharsTool(ToolDefinition):
        name = "replace-chars"
        title = "Replace characters in file"
        description = (
            "Replace a range of characters inside an existing file with new content. "
            "The range is defined by a zero-based character ``offset`` and a ``length`` "
            "(number of characters to remove starting at the offset). "
            "The supplied ``content`` is written in place of the removed range. "
            "To replace whole lines instead, use ``replace-lines``."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the file to modify.",
                },
                "offset": {
                    "type": "integer",
                    "description": "Zero-based character offset of the first character to replace.",
                    "minimum": 0,
                },
                "length": {
                    "type": "integer",
                    "description": "Number of characters to remove starting at ``offset``.",
                    "minimum": 0,
                },
                "content": {
                    "type": "string",
                    "description": "Replacement text (may be empty to perform a pure deletion).",
                },
            },
            "required": ["path", "offset", "length", "content"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
                    "description": "``success`` on success.",
                },
            },
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`replace_chars`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = replace_chars(
                    path=args["path"],
                    offset=args["offset"],
                    length=args["length"],
                    content=args["content"],
                )
            except ReplaceCharsError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(structured_content={"result": result.result}, auto_approve=True)


    def register_replace_chars_tool(registry: ToolRegistry) -> None:
        registry.register(ReplaceCharsTool())
```
/answer 613c-86 allow
Control Request:
```yaml
id: 613c-87
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_lines/__init__.py
  mode: replace
  reason: replace-lines-Tool nach dem insert-Vorlage-Muster refaktorieren
  content: |
    """Replace-lines tool – replaces a range of lines inside an existing file.

    This is the line-oriented analogue of ``replace-chars``: the range is given as a
    zero-based *line* offset and a *line* count instead of character offsets.
    """

    from __future__ import annotations

    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = [
        "ReplaceLinesError",
        "ReplaceLinesResult",
        "replace_lines",
        "ReplaceLinesTool",
        "register_replace_lines_tool",
    ]


    class ReplaceLinesError(Exception):
        """Raised when a replace-lines operation cannot be performed."""


    @dataclass(frozen=True)
    class ReplaceLinesResult:
        result: str


    def replace_lines(path: str, offset: int, length: int, content: str) -> ReplaceLinesResult:
        """Replace ``length`` lines starting at line ``offset`` in the file at ``path`` with ``content``."""
        file_path = Path(path)
        if not file_path.is_absolute():
            raise ReplaceLinesError("Path must be absolute.")
        if not file_path.exists():
            raise ReplaceLinesError("File not found.")
        if not file_path.is_file():
            raise ReplaceLinesError("Not a regular file.")

        try:
            text = file_path.read_text(encoding="utf-8")
            lines = text.splitlines(keepends=True)
            line_count = len(lines)
            if offset > line_count:
                raise ReplaceLinesError(
                    f"Offset {offset} is beyond end of file (file length: {line_count} lines)."
                )
            end = min(offset + length, line_count)
            result_text = "".join(lines[:offset]) + content + "".join(lines[end:])
            file_path.write_text(result_text, encoding="utf-8")
        except OSError as exc:
            raise ReplaceLinesError(f"Replace failed: {exc}") from exc

        return ReplaceLinesResult(result="success")


    class ReplaceLinesTool(ToolDefinition):
        name = "replace-lines"
        title = "Replace lines in file"
        description = (
            "Replace a range of lines inside an existing file with new content. "
            "The range is defined by a zero-based line ``offset`` and a ``length`` "
            "(number of lines to remove starting at the offset). "
            "The supplied ``content`` is written in place of the removed lines; "
            "it should include its own trailing newline if a line break is wanted. "
            "To replace an arbitrary character range instead, use ``replace-chars``."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the file to modify.",
                },
                "offset": {
                    "type": "integer",
                    "description": "Zero-based line offset of the first line to replace.",
                    "minimum": 0,
                },
                "length": {
                    "type": "integer",
                    "description": "Number of lines to remove starting at ``offset``.",
                    "minimum": 0,
                },
                "content": {
                    "type": "string",
                    "description": "Replacement text (may be empty to perform a pure deletion).",
                },
            },
            "required": ["path", "offset", "length", "content"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
                    "description": "``success`` on success.",
                },
            },
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`replace_lines`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = replace_lines(
                    path=args["path"],
                    offset=args["offset"],
                    length=args["length"],
                    content=args["content"],
                )
            except ReplaceLinesError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(structured_content={"result": result.result}, auto_approve=True)


    def register_replace_lines_tool(registry: ToolRegistry) -> None:
        registry.register(ReplaceLinesTool())
```
/answer 613c-87 allow
Control Request:
```yaml
id: 613c-88
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py
  mode: replace
  reason: replace-block-Tool nach dem insert-Vorlage-Muster refaktorieren
  content: |
    """Replace-block tool – replaces an exact block of text (old -> new) in a file."""

    from __future__ import annotations

    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
    from .._text_match import find as find_text

    __all__ = [
        "ReplaceBlockError",
        "ReplaceBlockResult",
        "replace_block",
        "ReplaceBlockTool",
        "register_replace_block_tool",
    ]


    class ReplaceBlockError(Exception):
        """Raised when a replace-block operation cannot be performed."""


    @dataclass(frozen=True)
    class ReplaceBlockResult:
        result: str


    def replace_block(path: str, old_text: str, new_text: str, exact: bool = False) -> ReplaceBlockResult:
        """Replace the unique occurrence of ``old_text`` in the file at ``path`` with ``new_text``."""
        file_path = Path(path)
        if not file_path.is_absolute():
            raise ReplaceBlockError("Path must be absolute.")
        if not file_path.exists():
            raise ReplaceBlockError("File not found.")
        if not file_path.is_file():
            raise ReplaceBlockError("Not a regular file.")
        if old_text == "":
            raise ReplaceBlockError("'old_text' must not be empty.")

        text = file_path.read_text(encoding="utf-8")

        match = find_text(text, old_text, exact=exact)
        if match.count == 0:
            raise ReplaceBlockError("Text not found in file.")
        if match.count > 1:
            raise ReplaceBlockError(f"Text is ambiguous – found {match.count} occurrences in file.")

        result_text = text[: match.start] + new_text + text[match.end :]

        try:
            file_path.write_text(result_text, encoding="utf-8")
        except OSError as exc:
            raise ReplaceBlockError(f"Write failed: {exc}") from exc

        return ReplaceBlockResult(result="success")


    class ReplaceBlockTool(ToolDefinition):
        name = "replace-block"
        title = "Replace text block in file"
        description = (
            "Replace a complete block of text inside an existing file. "
            "'old_text' must occur exactly once. By default whitespace "
            "(spaces, tabs, newlines) is matched tolerantly; set 'exact' to "
            "require exact whitespace matching."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the target file.",
                },
                "old_text": {
                    "type": "string",
                    "description": "Text to find and replace. Must occur exactly once.",
                },
                "new_text": {
                    "type": "string",
                    "description": "Text that replaces 'old_text'.",
                },
                "exact": {
                    "type": "boolean",
                    "description": (
                        "If true, 'old_text' must match whitespace exactly. "
                        "If false (default), whitespace runs match any amount/kind of whitespace."
                    ),
                    "default": False,
                },
            },
            "required": ["path", "old_text", "new_text"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
                },
            },
            "required": [],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`replace_block`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = replace_block(
                    path=args["path"],
                    old_text=args["old_text"],
                    new_text=args["new_text"],
                    exact=args.get("exact", False),
                )
            except ReplaceBlockError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(structured_content={"result": result.result}, auto_approve=True)


    def register_replace_block_tool(registry: ToolRegistry) -> None:
        registry.register(ReplaceBlockTool())
```
/answer 613c-88 allow
Control Request:
```yaml
id: 613c-89
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
  mode: replace
  reason: write-Tool nach dem insert-Vorlage-Muster refaktorieren
  content: |
    """Write tool – writes a file completely or appends lines to it."""

    from __future__ import annotations

    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = ["WriteError", "WriteResult", "write_file", "WriteTool", "register_write_tool"]


    class WriteError(Exception):
        """Raised when a write operation cannot be performed."""


    @dataclass(frozen=True)
    class WriteResult:
        result: str


    def write_file(path: str, mode: str, content: str) -> WriteResult:
        """Write ``content`` to ``path``; ``mode`` is ``replace`` or ``append``."""
        file_path = Path(path)
        if not file_path.is_absolute():
            raise WriteError("Path must be absolute.")

        try:
            file_path.parent.mkdir(parents=True, exist_ok=True)
            file_mode = "a" if mode == "append" else "w"
            with file_path.open(file_mode, encoding="utf-8") as fh:
                fh.write(content)
        except OSError as exc:
            raise WriteError(f"Write failed: {exc}") from exc

        return WriteResult(result="success")


    class WriteTool(ToolDefinition):
        name = "write"
        title = "Write file"
        description = (
            "Write content to a file. "
            "In ``replace`` mode the file is overwritten with the supplied content. "
            "In ``append`` mode the content is added at the end of the existing file "
            "(the file is created if it does not yet exist)."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the file to write.",
                },
                "mode": {
                    "type": "string",
                    "enum": ["replace", "append"],
                    "description": (
                        "``replace`` – overwrite the file with the new content. "
                        "``append`` – add the new content after the existing content."
                    ),
                },
                "content": {
                    "type": "string",
                    "description": "Text to write to the file.",
                },
            },
            "required": ["path", "mode", "content"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
                    "description": "``success`` on success.",
                },
            },
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`write_file`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = write_file(path=args["path"], mode=args["mode"], content=args["content"])
            except WriteError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(structured_content={"result": result.result}, auto_approve=True)


    def register_write_tool(registry: ToolRegistry) -> None:
        registry.register(WriteTool())
```
/answer 613c-89 allow
Control Request:
```yaml
id: 613c-8a
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  mode: replace
  reason: list-Tool nach dem insert-Vorlage-Muster refaktorieren
  content: |
    """List tool – returns a flat, sorted list of relative file paths below a directory.

    Walks the given absolute directory recursively and returns all file paths
    (files only, no directories) as an alphabetically sorted flat list of paths
    relative to the requested directory. An optional regular expression can be
    supplied to filter the resulting list (matched against each relative file
    path). Common VCS/build/cache directories (e.g. ``.git``) are always excluded.
    To keep results manageable, the number of returned entries is capped; use
    ``pattern`` to narrow down large directories instead of raising the limit.
    """

    from __future__ import annotations

    import os
    import re
    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = ["ListError", "ListResult", "list_files", "ListTool", "register_list_tool"]

    _MAX_ENTRIES = 50

    _EXCLUDED_DIRS = {
        ".git",
        ".hg",
        ".svn",
        "__pycache__",
        ".mypy_cache",
        ".pytest_cache",
        ".ruff_cache",
        ".tox",
        ".venv",
        "venv",
        "node_modules",
        ".idea",
        ".vscode",
        "dist",
        "build",
        ".cache",
    }


    class ListError(Exception):
        """Raised when a directory listing cannot be performed."""


    @dataclass(frozen=True)
    class ListResult:
        entries: list[str]


    def list_files(path: str, pattern: str | None = None) -> ListResult:
        """List all files below the absolute directory ``path``, optionally filtered by ``pattern``."""
        dir_path = Path(path)
        if not dir_path.is_absolute():
            raise ListError("Path must be absolute.")
        if not dir_path.exists():
            raise ListError("Directory not found.")
        if not dir_path.is_dir():
            raise ListError("Not a directory.")

        regex: re.Pattern[str] | None = None
        if pattern is not None:
            try:
                regex = re.compile(pattern)
            except re.error as exc:
                raise ListError(f"Invalid regular expression: {exc}") from exc

        root = dir_path.resolve()
        entries: list[str] = []
        for dirpath, dirnames, filenames in os.walk(root):
            dirnames[:] = [d for d in dirnames if d not in _EXCLUDED_DIRS]
            for filename in filenames:
                file_path = Path(dirpath) / filename
                rel_path = str(file_path.relative_to(root))
                if regex is not None and not regex.search(rel_path):
                    continue
                entries.append(rel_path)

        entries.sort()

        if len(entries) > _MAX_ENTRIES:
            raise ListError(
                f"Too many entries ({len(entries)}) exceed the limit of "
                f"{_MAX_ENTRIES}. Narrow down the result using the "
                "'pattern' regular expression parameter."
            )

        return ListResult(entries=entries)


    class ListTool(ToolDefinition):
        name = "list"
        title = "List directory"
        description = (
            "List all files below an absolute directory path, recursively, "
            "as a flat list. "
            "Optionally filter the result with a regular expression."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute directory path.",
                },
                "pattern": {
                    "type": "string",
                    "description": (
                        "Optional regular expression used to filter the result."
                    ),
                },
            },
            "required": ["path"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "entries": {
                    "type": "array",
                    "items": {"type": "string"},
                },
            },
            "required": ["entries"],
        }
        annotations = {"readOnlyHint": True, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`list_files`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = list_files(path=args["path"], pattern=args.get("pattern"))
            except ListError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(structured_content={"entries": result.entries})


    def register_list_tool(registry: ToolRegistry) -> None:
        registry.register(ListTool())
```
/answer 613c-8a allow
ReasoningToken: 59
Text:
 Nun die einfacheren Datei-Tools: change, replace_chars, replace_lines, replace_block, write.
Control Request:
```yaml
id: 613c-8b
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file-stats/__init__.py
  mode: replace
  reason: "file-stats-Tool nach dem insert-Vorlage-Muster refaktorieren, dabei compute_file_stats() für die bestehende Nutzung durch ast/outline.py erhalten"
  content: |
    """File stats tool – returns file metrics for access and processing planning.

    Provides compact metrics including complexity, timestamps, size, line/word
    counts, line length statistics, and average words per line.
    """

    from __future__ import annotations

    import hashlib
    import re
    from dataclasses import dataclass
    from datetime import datetime, timezone
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = [
        "FileStatsError",
        "FileStatsResult",
        "compute_file_stats",
        "file_stats",
        "FileStatsTool",
        "register_file_stats_tool",
    ]


    class FileStatsError(Exception):
        """Raised when file metrics cannot be computed."""


    @dataclass(frozen=True)
    class FileStatsResult:
        path: str
        size_bytes: int
        lines: int
        words: int
        complexity: float
        created: str
        modified: str
        accessed: str
        line_length_max: int
        line_length_min: int
        line_length_avg: float
        words_per_line_avg: float
        checksum: str


    def _calculate_complexity(text: str) -> float:
        """Calculate data structure complexity (0.0 to 1.0).

        Based on character set diversity and pattern variation.
        """
        if not text:
            return 0.0

        has_alpha = bool(re.search(r'[a-zA-Z]', text))
        has_digit = bool(re.search(r'\d', text))
        has_punct = bool(re.search(r'[^\w\s]', text))
        has_space = bool(re.search(r'\s', text))
        has_upper = bool(re.search(r'[A-Z]', text))
        has_lower = bool(re.search(r'[a-z]', text))

        char_type_score = sum([has_alpha, has_digit, has_punct, has_space, has_upper, has_lower]) / 6.0

        unique_chars = len(set(text))
        entropy_score = min(1.0, unique_chars / 256.0)

        complexity = (char_type_score * 0.4) + (entropy_score * 0.6)
        return round(complexity, 3)


    def compute_file_stats(path: Path) -> dict[str, Any]:
        """Compute the file-metrics block for *path* (also reused by the outline tool).

        Assumes *path* is an existing regular file.
        """
        raw_bytes = path.read_bytes()
        text = raw_bytes.decode("utf-8", errors="replace")
        lines = text.splitlines()

        size_bytes = len(raw_bytes)
        num_lines = len(lines)
        num_words = len(text.split())
        complexity = _calculate_complexity(text)
        checksum = hashlib.sha256(text.encode("utf-8")).hexdigest()

        line_lengths = [len(line) for line in lines] if lines else [0]
        line_length_max = max(line_lengths) if line_lengths else 0
        line_length_min = min(line_lengths) if line_lengths else 0
        line_length_avg = round(sum(line_lengths) / len(line_lengths), 2) if line_lengths else 0.0

        words_per_line = [len(line.split()) for line in lines]
        words_per_line_avg = (
            round(sum(words_per_line) / len(words_per_line), 2) if words_per_line else 0.0
        )

        stat = path.stat()
        created = datetime.fromtimestamp(
            stat.st_birthtime if hasattr(stat, "st_birthtime") else stat.st_mtime,
            tz=timezone.utc,
        ).isoformat()
        modified = datetime.fromtimestamp(stat.st_mtime, tz=timezone.utc).isoformat()
        accessed = datetime.fromtimestamp(stat.st_atime, tz=timezone.utc).isoformat()

        return {
            "path": str(path.resolve()),
            "size_bytes": size_bytes,
            "lines": num_lines,
            "words": num_words,
            "complexity": complexity,
            "created": created,
            "modified": modified,
            "accessed": accessed,
            "line_length_max": line_length_max,
            "line_length_min": line_length_min,
            "line_length_avg": line_length_avg,
            "words_per_line_avg": words_per_line_avg,
            "checksum": checksum,
        }


    def file_stats(path: str) -> FileStatsResult:
        """Compute file metrics for the absolute path ``path``."""
        file_path = Path(path)
        if not file_path.is_absolute():
            raise FileStatsError("Path must be absolute.")
        if not file_path.exists():
            raise FileStatsError("File not found.")
        if not file_path.is_file():
            raise FileStatsError("Not a regular file.")

        return FileStatsResult(**compute_file_stats(file_path))


    class FileStatsTool(ToolDefinition):
        name = "file-stats"
        title = "File stats"
        description = (
            "Get file metrics for access and processing planning: complexity, timestamps, "
            "size, line/word counts, and line length statistics."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute file path.",
                },
            },
            "required": ["path"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute file path.",
                },
                "size_bytes": {
                    "type": "integer",
                    "description": "File size in bytes.",
                },
                "lines": {
                    "type": "integer",
                    "description": "Total number of lines.",
                },
                "words": {
                    "type": "integer",
                    "description": "Total number of words.",
                },
                "complexity": {
                    "type": "number",
                    "description": "Data structure complexity (0.0 to 1.0).",
                },
                "created": {
                    "type": "string",
                    "description": "Creation timestamp (ISO 8601).",
                },
                "modified": {
                    "type": "string",
                    "description": "Last modification timestamp (ISO 8601).",
                },
                "accessed": {
                    "type": "string",
                    "description": "Last access timestamp (ISO 8601).",
                },
                "line_length_max": {
                    "type": "integer",
                    "description": "Maximum line length in characters.",
                },
                "line_length_min": {
                    "type": "integer",
                    "description": "Minimum line length in characters.",
                },
                "line_length_avg": {
                    "type": "number",
                    "description": "Average line length in characters.",
                },
                "words_per_line_avg": {
                    "type": "number",
                    "description": "Average number of words per line.",
                },
                "checksum": {
                    "type": "string",
                    "description": "sha256 checksum of the file content.",
                },
            },
            "required": [
                "path", "size_bytes", "lines", "words", "complexity",
                "created", "modified", "accessed",
                "line_length_max", "line_length_min", "line_length_avg",
                "words_per_line_avg", "checksum"
            ],
        }
        annotations = {"readOnlyHint": True, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`file_stats`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = file_stats(args["path"])
            except FileStatsError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(
                content=[],
                structured_content=result.__dict__,
                auto_approve=True,
            )


    def register_file_stats_tool(registry: ToolRegistry) -> None:
        registry.register(FileStatsTool())
```
/answer 613c-8b allow
Control Request:
```yaml
id: 613c-8c
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ask-user/__init__.py
  mode: replace
  reason: ask-user-Tool nach dem insert-Vorlage-Muster refaktorieren
  content: |
    """Ask-user tool – lets an agent ask the human a clarifying question.

    This tool exists to give agents a back-channel to the user so they can ask
    questions that improve session efficiency instead of, e.g., exhaustively
    searching whole directory hierarchies when the user might already know the
    answer or can find it far more easily.

    The permission system already allows the user to intercept tool calls and
    their outputs, so this implementation is intentionally a simple dummy: it
    always reports that the user did not answer, leaving it up to the agent to
    proceed on its own (e.g. by falling back to exploration). The main value of
    this module is exposing a well-defined API/MCP tool for the interaction, not
    an actual answering mechanism.
    """

    from __future__ import annotations

    from dataclasses import dataclass
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = ["AskUserError", "AskUserResult", "ask_user", "AskUserTool", "register_ask_user_tool"]

    _NOT_ANSWERED = "The user did not answer. Proceed on your own."


    class AskUserError(Exception):
        """Raised when a question cannot be asked."""


    @dataclass(frozen=True)
    class AskUserResult:
        answer: str


    def ask_user(question: str) -> AskUserResult:
        """Ask the user ``question``; always returns the "not answered" placeholder."""
        if not question or not question.strip():
            raise AskUserError("``question`` must not be empty.")

        return AskUserResult(answer=_NOT_ANSWERED)


    class AskUserTool(ToolDefinition):
        name = "ask-user"
        title = "Ask user"
        description = (
            "Ask the user a clarifying question, in the user's language, to "
            "improve session efficiency (e.g. instead of searching an entire "
            "file hierarchy when the user likely knows the answer already). "
        )
        input_schema = {
            "type": "object",
            "properties": {
                "question": {
                    "type": "string",
                    "description": "The question to ask the user, in the user's language.",
                },
            },
            "required": ["question"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "answer": {"type": "string"},
            },
            "required": ["answer"],
        }
        annotations = {"readOnlyHint": True, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ask_user`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = ask_user(args["question"])
            except AskUserError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(structured_content={"answer": result.answer})


    def register_ask_user_tool(registry: ToolRegistry) -> None:
        registry.register(AskUserTool())
```
/answer 613c-8c allow
Control Request:
```yaml
id: 613c-8d
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  mode: replace
  reason: "colgrep-Tool nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktion colgrep_search() + ColgrepError/ColgrepResult, handle() als Delegate"
  content: |
    """colgrep tool - semantic + keyword code search over a pre-built colgrep index.

    Search-only wrapper around the `colgrep` CLI. Never creates, initializes or
    otherwise modifies an index; that remains the user's responsibility (see the
    `colgrep-init.sh` setup script). Given a directory, the tool climbs up through
    parent directories until it finds a colgrep index (built with
    XDG_DATA_HOME/XDG_CONFIG_HOME pointed at the project root) and runs the
    search from there, scoped back to the originally requested directory.
    """
    from __future__ import annotations
    import json
    import os
    import subprocess
    from dataclasses import dataclass, field
    from pathlib import Path
    from typing import Any
    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = [
        "ColgrepError",
        "ColgrepResult",
        "colgrep_search",
        "ColgrepTool",
        "register_colgrep_tool",
    ]

    _COLGREP_BIN = '/home/user/.cargo/bin/colgrep'
    _CONTEXT_LINES = '2'
    _DEFAULT_RESULTS = 15
    _MAX_RESULTS = 50
    _MAX_CODE_LEN = 100
    _DROPPED_KEYS = frozenset({'language', 'signature', 'qualified_name', 'unit_type', 'complexity', 'has_loops', 'has_branches', 'has_error_handling', 'extends', 'parent_class', 'variables', 'name', 'return_type', 'calls', 'imports', 'parameters'})


    class ColgrepError(Exception):
        """Raised when a colgrep search cannot be performed."""


    @dataclass(frozen=True)
    class ColgrepResult:
        results: list[Any] = field(default_factory=list)
        count: int = 0


    def _find_index_root(start: Path) -> Path | None:
        """Climb from *start* up to the filesystem root looking for a colgrep index.

        A directory ``D`` is considered a colgrep project root if
        ``D/.colgrep/colgrep/indices`` exists and is non-empty - the layout
        produced when colgrep is run with ``XDG_DATA_HOME=D`` (see
        ``colgrep-init.sh``).
        """
        current = start
        while True:
            candidate = current / '.colgrep' / 'colgrep' / 'indices'
            if candidate.is_dir() and any(candidate.iterdir()):
                return current
            if current.parent == current:
                return None
            current = current.parent


    def _clean_result(value: Any) -> Any:
        """Recursively drop empty components (``False``, ``""``, ``None``, ``[]``) and
        unwanted keys (``score`` plus the fields listed in ``_DROPPED_KEYS``) from
        colgrep JSON output. The ``code`` field is truncated to ``_MAX_CODE_LEN``
        characters.
        """
        if isinstance(value, dict):
            cleaned = {}
            for key, item in value.items():
                if key == 'score' or key in _DROPPED_KEYS:
                    continue
                if key == 'code' and isinstance(item, str) and len(item) > _MAX_CODE_LEN:
                    item = item[:_MAX_CODE_LEN]
                cleaned_item = _clean_result(item)
                if cleaned_item is False or cleaned_item == '' or cleaned_item is None or cleaned_item == []:
                    continue
                cleaned[key] = cleaned_item
            return cleaned
        if isinstance(value, list):
            return [_clean_result(item) for item in value]
        return value


    def colgrep_search(
        path: str,
        query: str,
        results: int = _DEFAULT_RESULTS,
        semantic_only: bool = False,
        code_only: bool = False,
        files_only: bool = False,
        full_content: bool = False,
        include: list[str] | None = None,
        exclude: list[str] | None = None,
        exclude_dir: list[str] | None = None,
    ) -> ColgrepResult:
        """Search the colgrep index covering ``path`` for ``query``."""
        include = include or []
        exclude = exclude or []
        exclude_dir = exclude_dir or []

        if not query.strip():
            raise ColgrepError('query must not be empty.')
        search_path = Path(path)
        if not search_path.is_absolute():
            raise ColgrepError('path must be an absolute path.')
        if not search_path.is_dir():
            raise ColgrepError('Directory not found.')
        if files_only and full_content:
            raise ColgrepError('files_only and full_content are mutually exclusive.')
        if not 1 <= results <= _MAX_RESULTS:
            raise ColgrepError(f'results must be between 1 and {_MAX_RESULTS}.')

        search_dir = search_path.resolve()
        index_root = _find_index_root(search_dir)
        if index_root is None:
            raise ColgrepError('No colgrep index found for this directory or any parent directory.')

        cmd = [_COLGREP_BIN, query, str(search_dir), '--json', '-n', _CONTEXT_LINES, '-k', str(results)]
        if files_only:
            cmd.append('-l')
        if full_content:
            cmd.append('-c')
        if code_only:
            cmd.append('--code-only')
        if semantic_only:
            cmd.append('--semantic-only')
        for pattern in include:
            cmd.append(f'--include={pattern}')
        for pattern in exclude:
            cmd.append(f'--exclude={pattern}')
        for name in exclude_dir:
            cmd.append(f'--exclude-dir={name}')

        env = dict(os.environ)
        env['XDG_DATA_HOME'] = str(index_root)
        env['XDG_CONFIG_HOME'] = str(index_root)

        try:
            proc = subprocess.run(cmd, cwd=str(index_root), env=env, input='', capture_output=True, encoding='utf-8', errors='replace')
        except OSError as exc:
            raise ColgrepError(f'Failed to launch colgrep: {exc}') from exc

        if proc.returncode != 0:
            message = proc.stderr.strip() or proc.stdout.strip() or f'colgrep exited with code {proc.returncode}.'
            raise ColgrepError(message)

        try:
            parsed = json.loads(proc.stdout) if proc.stdout.strip() else []
        except json.JSONDecodeError as exc:
            raise ColgrepError('colgrep returned output that could not be parsed as JSON.') from exc

        parsed = _clean_result(parsed)
        if isinstance(parsed, list):
            return ColgrepResult(results=parsed, count=len(parsed))
        return ColgrepResult(results=[parsed], count=1)


    class ColgrepTool(ToolDefinition):
        name = 'colgrep'
        title = 'Search code with colgrep'
        description = "Search a project's codebase with colgrep."
        input_schema = {
            'type': 'object',
            'properties': {
                'path': {'type': 'string', 'description': 'Absolute directory to search in.'},
                'query': {'type': 'string', 'description': 'Search query: natural language and/or identifiers/keywords.'},
                'results': {'type': 'integer', 'minimum': 1, 'maximum': _MAX_RESULTS, 'default': _DEFAULT_RESULTS, 'description': 'Maximum number of results to return.'},
                'semantic_only': {'type': 'boolean', 'default': False, 'description': 'Disable keyword fusion; pure semantic ranking only.'},
                'code_only': {'type': 'boolean', 'default': False, 'description': 'Skip documentation/config files; search source code only.'},
                'files_only': {'type': 'boolean', 'default': False, 'description': 'Return matching file paths only, without snippets.'},
                'full_content': {'type': 'boolean', 'default': False, 'description': 'Return the full matched function/class body instead of a short snippet.'},
                'include': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Glob patterns a file must match, e.g. "*.py", "src/**/*.rs".'},
                'exclude': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Glob patterns of files to exclude, e.g. "*.test.ts".'},
                'exclude_dir': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Directory names to exclude, e.g. "vendor", "node_modules".'},
            },
            'required': ['path', 'query']
        }
        output_schema = {
            'type': 'object',
            'properties': {
                'results': {'type': 'array', 'items': {'type': 'object'}, 'description': 'Result objects as produced by `colgrep'},
                'count': {'type': 'integer'}
            },
            'required': ['results']
        }
        annotations = {'readOnlyHint': True, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`colgrep_search`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = colgrep_search(
                    path=args['path'],
                    query=args['query'],
                    results=args.get('results', _DEFAULT_RESULTS),
                    semantic_only=args.get('semantic_only', False),
                    code_only=args.get('code_only', False),
                    files_only=args.get('files_only', False),
                    full_content=args.get('full_content', False),
                    include=args.get('include') or [],
                    exclude=args.get('exclude') or [],
                    exclude_dir=args.get('exclude_dir') or [],
                )
            except ColgrepError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(structured_content={'results': result.results, 'count': result.count})


    def register_colgrep_tool(registry: ToolRegistry) -> None:
        registry.register(ColgrepTool())
```
/answer 613c-8d allow
Control Request:
```yaml
id: 613c-8e
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  mode: replace
  reason: "read-Tool nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktion read_file() liefert Inhalt+Checksumme, handle() übernimmt Session-Caching und MCP-Packing"
  content: |
    """Read tool – returns file contents, optionally sliced by line, character offset, or marker.

    Range: start = min_line | min_char | start-marker | file start;
    end = max_line | max_char | end-marker | file end (all inclusive).
    Markers must be unique substrings. Line and char ranges are mutually exclusive.

    Per-session cache (key ``_read_cache`` in ``Session.state``, keyed by the call
    arguments plus the session id): the sha256 checksum of every read is recorded.
    If a subsequent read with identical parameters yields the same checksum,
    ``content`` is omitted from ``structured_content`` and replaced by an
    explanatory text content block; only the checksum is still returned.
    ``structured_content`` always carries the ``checksum``.
    """

    from __future__ import annotations

    import hashlib
    import json
    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = ["ReadError", "ReadResult", "read_file", "ReadTool", "register_read_tool"]

    _CACHE_STATE_KEY = "_read_cache"


    class ReadError(Exception):
        """Raised when a file cannot be read or the requested range is invalid."""


    @dataclass(frozen=True)
    class ReadResult:
        content: str
        checksum: str
        is_full_file: bool


    def _cache_key(session_id: str, arguments: dict[str, Any]) -> str:
        payload = json.dumps({"session": session_id, "arguments": arguments}, sort_keys=True)
        return hashlib.sha256(payload.encode("utf-8")).hexdigest()


    def read_file(
        path: str,
        min_line: int | None = None,
        max_line: int | None = None,
        min_char: int | None = None,
        max_char: int | None = None,
        start: str | None = None,
        end: str | None = None,
    ) -> ReadResult:
        """Read the file at ``path``, optionally sliced to the given range."""
        if min_line is not None and min_char is not None:
            raise ReadError("``min_line`` and ``min_char`` are mutually exclusive.")
        if max_line is not None and max_char is not None:
            raise ReadError("``max_line`` and ``max_char`` are mutually exclusive.")
        if min_line is not None and start is not None:
            raise ReadError("``min_line`` and ``start`` are mutually exclusive.")
        if min_char is not None and start is not None:
            raise ReadError("``min_char`` and ``start`` are mutually exclusive.")
        if max_line is not None and end is not None:
            raise ReadError("``max_line`` and ``end`` are mutually exclusive.")
        if max_char is not None and end is not None:
            raise ReadError("``max_char`` and ``end`` are mutually exclusive.")

        file_path = Path(path)
        if not file_path.is_absolute():
            raise ReadError("Path must be absolute.")
        if not file_path.exists():
            raise ReadError("File not found.")
        if not file_path.is_file():
            raise ReadError("Not a regular file. Don't read directories with this tool!")

        raw_bytes = file_path.read_bytes()
        text = raw_bytes.decode("utf-8", errors="replace")
        lines = text.splitlines(keepends=True)
        total_lines = len(lines)

        def line_start_offset(line_num: int) -> int:
            n = max(0, min(line_num - 1, total_lines))
            return sum(len(l) for l in lines[:n])

        def line_end_offset(line_num: int) -> int:
            n = max(0, min(line_num, total_lines))
            return sum(len(l) for l in lines[:n])

        if start is not None:
            start_count = text.count(start)
            if start_count == 0:
                raise ReadError("Start marker not found in file.")
            if start_count > 1:
                raise ReadError(f"Start marker is ambiguous – found {start_count} occurrences in file.")
            region_start = text.index(start)
        elif min_line is not None:
            region_start = line_start_offset(min_line)
        elif min_char is not None:
            region_start = min_char
        else:
            region_start = 0

        if end is not None:
            end_count = text.count(end)
            if end_count == 0:
                raise ReadError("End marker not found in file.")
            if end_count > 1:
                raise ReadError(f"End marker is ambiguous – found {end_count} occurrences in file.")
            region_end = text.index(end) + len(end)
        elif max_line is not None:
            region_end = line_end_offset(max_line)
        elif max_char is not None:
            region_end = max_char
        else:
            region_end = len(text)

        if region_end < region_start:
            raise ReadError("Resolved end position must not lie before the resolved start position.")

        sliced = text[region_start:region_end]
        checksum = hashlib.sha256(sliced.encode("utf-8")).hexdigest()

        is_full_file = (
            min_line is None
            and max_line is None
            and min_char is None
            and max_char is None
            and start is None
            and end is None
        )

        return ReadResult(content=sliced, checksum=checksum, is_full_file=is_full_file)


    class ReadTool(ToolDefinition):
        name = "read-file"
        title = "Read file"
        description = (
            "Read a file as text, optionally sliced to a range. Don't use to read directories."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute file path.",
                },
                "min_line": {
                    "type": "integer",
                    "description": "Range start: line number, inclusive, 1-based. Excludes start and min_char.",
                    "minimum": 1,
                },
                "max_line": {
                    "type": "integer",
                    "description": "Range end: line number, inclusive, 1-based. Excludes end and max_char.",
                    "minimum": 1,
                },
                "min_char": {
                    "type": "integer",
                    "description": "Range start: character offset, inclusive, 0-based. Excludes min_line.",
                    "minimum": 0,
                },
                "max_char": {
                    "type": "integer",
                    "description": "Range end: character offset, exclusive, 0-based. Excludes max_line.",
                    "minimum": 0,
                },
                "start": {
                    "type": "string",
                    "description": "Range start: unique marker substring, inclusive. Excludes min_line and min_char.",
                },
                "end": {
                    "type": "string",
                    "description": "Range end: unique marker substring, inclusive. Excludes max_line and max_char.",
                },
            },
            "required": ["path"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "content": {"type": "string"},
                "checksum": {
                    "type": "string",
                    "description": "sha256 checksum of the read content.",
                },
                "unchanged": {
                    "type": "boolean",
                    "description": (
                        "True if the content is identical to a previous read with the "
                        "same parameters"
                    ),
                },
            },
            "required": ["checksum"],
        }
        annotations = {"readOnlyHint": True, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`read_file`, then apply session-level change detection and MCP packing."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = read_file(
                    path=args["path"],
                    min_line=args.get("min_line"),
                    max_line=args.get("max_line"),
                    min_char=args.get("min_char"),
                    max_char=args.get("max_char"),
                    start=args.get("start"),
                    end=args.get("end"),
                )
            except ReadError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            session = ctx.session
            key = _cache_key(session.id, args)
            with session.lock:
                cache: dict[str, str] = session.state.setdefault(_CACHE_STATE_KEY, {})
                previous_checksum = cache.get(key)
                cache[key] = result.checksum

            unchanged = previous_checksum == result.checksum

            structured: dict[str, Any] = {"checksum": result.checksum}
            if unchanged:
                structured["unchanged"] = True
            else:
                structured["content"] = result.content

            content: list[dict[str, Any]] = []
            if unchanged:
                content.append(
                    text_content(
                        "Content unchanged since the last identical read. Use the former read result."
                    )
                )

            return ToolResult(
                content=content,
                structured_content=structured,
                auto_approve=result.is_full_file,
            )


    def register_read_tool(registry: ToolRegistry) -> None:
        registry.register(ReadTool())
```
/answer 613c-8e allow
Control Request:
```yaml
id: 613c-8f
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/file_ops.py
  mode: replace
  reason: "ast/file_ops.py nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktionen create_python_file()/delete_python_file(), handle() als Delegate"
  content: |
    """Whole-file operations: ``python-ast-create-file`` and ``python-ast-delete-file``."""

    from __future__ import annotations

    from dataclasses import dataclass
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
    from . import core

    __all__ = [
        "AstFileResult",
        "create_python_file",
        "delete_python_file",
        "CreateFileTool",
        "DeleteFileTool",
        "register",
    ]


    @dataclass(frozen=True)
    class AstFileResult:
        result: str


    def create_python_file(path: str, code: str, overwrite: bool = False) -> AstFileResult:
        """Create a new Python file at ``path`` from ``code`` (validated by parsing it)."""
        file_path = core.require_path(path, must_exist=False)
        if file_path.exists() and not overwrite:
            raise core.AstError("File already exists.")
        tree = core.parse_source(code)
        file_path.parent.mkdir(parents=True, exist_ok=True)
        core.CACHE.save(file_path, tree)
        return AstFileResult(result="success")


    def delete_python_file(path: str) -> AstFileResult:
        """Delete the Python file at ``path`` and drop it from the AST cache."""
        file_path = core.require_path(path)
        try:
            file_path.unlink()
        except OSError as exc:
            raise core.AstError("Delete failed.") from exc
        core.CACHE.invalidate(file_path)
        return AstFileResult(result="success")


    class CreateFileTool(ToolDefinition):
        name = "python-ast-create-file"
        title = "Create Python file"
        description = "Create a new Python file from source text (validated by parsing it through the AST)."
        input_schema = {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path of the file to create."},
                "code": {"type": "string", "description": "Python source for the new file."},
                "overwrite": {
                    "type": "boolean",
                    "description": "Allow replacing an existing file.",
                    "default": False,
                },
            },
            "required": ["path", "code"],
        }
        output_schema = {
            "type": "object",
            "properties": {"result": {"type": "string"}},
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`create_python_file`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = create_python_file(
                    path=args["path"], code=args["code"], overwrite=args.get("overwrite", False)
                )
            except core.AstError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={"result": result.result}, auto_approve=True)


    class DeleteFileTool(ToolDefinition):
        name = "python-ast-delete-file"
        title = "Delete Python file"
        description = "Delete a Python file and drop it from the AST cache."
        input_schema = {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path of the file to delete."}
            },
            "required": ["path"],
        }
        output_schema = {
            "type": "object",
            "properties": {"result": {"type": "string"}},
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`delete_python_file`, translating the MCP schema to/from the Python API."""
            try:
                result = delete_python_file(ctx.arguments["path"])
            except core.AstError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={"result": result.result}, auto_approve=True)


    def register(registry: ToolRegistry) -> None:
        registry.register(CreateFileTool())
        registry.register(DeleteFileTool())
```
/answer 613c-8f allow
Control Request:
```yaml
id: 613c-90
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py
  mode: replace
  reason: "ast/validate.py nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktion validate_python_files(), handle() als Delegate"
  content: |
    """``python-ast-validate`` – compile a list of Python files and report results."""

    from __future__ import annotations

    from dataclasses import dataclass, field
    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

    __all__ = [
        "ValidateError",
        "FileCheck",
        "ValidateResult",
        "validate_python_files",
        "ValidateTool",
        "register",
    ]


    class ValidateError(Exception):
        """Raised when the validate operation cannot be performed at all."""


    @dataclass(frozen=True)
    class FileCheck:
        path: str
        ok: bool
        error: str | None


    @dataclass(frozen=True)
    class ValidateResult:
        all_ok: bool
        files: list[FileCheck] = field(default_factory=list)


    def _check(path_str: str) -> FileCheck:
        path = Path(path_str)
        if not path.is_absolute():
            return FileCheck(path=path_str, ok=False, error="Path must be absolute.")
        try:
            source = path.read_text(encoding="utf-8")
        except OSError:
            return FileCheck(path=path_str, ok=False, error="File not readable.")
        try:
            compile(source, str(path), "exec")
        except SyntaxError as exc:
            return FileCheck(path=path_str, ok=False, error=f"{exc.msg} (line {exc.lineno})")
        return FileCheck(path=path_str, ok=True, error=None)


    def validate_python_files(paths: list[str]) -> ValidateResult:
        """Compile each of ``paths`` and report success/error per file."""
        if not paths:
            raise ValidateError("'paths' must be a non-empty list.")
        files = [_check(p) for p in paths]
        return ValidateResult(all_ok=all(f.ok for f in files), files=files)


    class ValidateTool(ToolDefinition):
        name = "python-ast-validate"
        title = "Validate Python files"
        description = "Check that each of a list of Python files compiles; report success/error per file."
        input_schema = {
            "type": "object",
            "properties": {
                "paths": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Absolute paths of Python files to validate.",
                }
            },
            "required": ["paths"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "all_ok": {"type": "boolean"},
                "files": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "path": {"type": "string"},
                            "ok": {"type": "boolean"},
                            "error": {"type": ["string", "null"]},
                        },
                        "required": ["path", "ok", "error"],
                    },
                },
            },
            "required": ["all_ok", "files"],
        }
        annotations = {"readOnlyHint": True, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`validate_python_files`, translating the MCP schema to/from the Python API."""
            paths = ctx.arguments["paths"]
            if not isinstance(paths, list):
                return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
            try:
                result = validate_python_files(paths)
            except ValidateError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(
                structured_content={
                    "all_ok": result.all_ok,
                    "files": [f.__dict__ for f in result.files],
                },
                auto_approve=result.all_ok,
            )


    def register(registry: ToolRegistry) -> None:
        registry.register(ValidateTool())
```
/answer 613c-90 allow
Control Request:
```yaml
id: 613c-91
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  mode: replace
  reason: "ast/outline.py nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktion outline_python_files(), handle() als Delegate"
  content: |
    """``python-ast-outline`` – compact structural overview of Python files."""

    from __future__ import annotations

    import ast
    import importlib
    from dataclasses import dataclass, field
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
    from . import core

    __all__ = [
        "OutlineError",
        "FileOutline",
        "OutlineResult",
        "outline_python_files",
        "OutlineTool",
        "register",
    ]

    compute_file_stats = importlib.import_module(
        "xy.ai.mcpc.tools.file-stats"
    ).compute_file_stats


    class OutlineError(Exception):
        """Raised when the outline operation cannot be performed at all."""


    @dataclass(frozen=True)
    class FileOutline:
        path: str
        ok: bool
        error: str | None
        stats: dict[str, Any] | None = None
        imports: list[dict[str, Any]] = field(default_factory=list)
        classes: list[dict[str, Any]] = field(default_factory=list)
        functions: list[dict[str, Any]] = field(default_factory=list)


    @dataclass(frozen=True)
    class OutlineResult:
        all_ok: bool
        files: list[FileOutline] = field(default_factory=list)


    def _method_entry(loc: core.Located) -> dict[str, Any]:
        node = loc.node
        return {
            "name": loc.name,
            "qualified_name": loc.qualified_name,
            "lineno": node.lineno,
            "end_lineno": getattr(node, "end_lineno", node.lineno),
            "docstring": core.short_docstring(node),
        }


    def _build_outline(tree: ast.Module) -> dict[str, Any]:
        located = core.locate_all(tree)

        imports = [
            {"names": loc.name, "lineno": loc.node.lineno}
            for loc in located
            if isinstance(loc.node, core._IMPORT_TYPES)
        ]

        classes: list[dict[str, Any]] = []
        functions: list[dict[str, Any]] = []
        for loc in located:
            node = loc.node
            if isinstance(node, ast.ClassDef):
                methods = [
                    _method_entry(m)
                    for m in located
                    if m.parent is node and isinstance(m.node, (ast.FunctionDef, ast.AsyncFunctionDef))
                ]
                classes.append(
                    {
                        "name": loc.name,
                        "qualified_name": loc.qualified_name,
                        "lineno": node.lineno,
                        "end_lineno": getattr(node, "end_lineno", node.lineno),
                        "docstring": core.short_docstring(node),
                        "methods": methods,
                    }
                )
            elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and isinstance(
                loc.parent, ast.Module
            ):
                functions.append(_method_entry(loc))

        return {"imports": imports, "classes": classes, "functions": functions}


    def _outline_one(path_str: str) -> FileOutline:
        try:
            path, tree = core.load(path_str)
        except core.AstError as exc:
            return FileOutline(path=path_str, ok=False, error=str(exc))
        outline = _build_outline(tree)
        return FileOutline(
            path=path_str,
            ok=True,
            error=None,
            stats=compute_file_stats(path),
            **outline,
        )


    def outline_python_files(paths: list[str]) -> OutlineResult:
        """Build a structural outline (imports, classes, functions, stats) for each of ``paths``."""
        if not paths:
            raise OutlineError("'paths' must be a non-empty list.")
        files = [_outline_one(p) for p in paths]
        return OutlineResult(all_ok=all(f.ok for f in files), files=files)


    _OUTLINE_ITEM_SCHEMA = {
        "type": "object",
        "properties": {
            "path": {"type": "string"},
            "ok": {"type": "boolean"},
            "error": {"type": ["string", "null"]},
            "stats": {"type": "object", "description": "File-metrics block."},
            "imports": {
                "type": "array",
                "description": "Imports with line numbers.",
                "items": {
                    "type": "object",
                    "properties": {
                        "names": {"type": "string"},
                        "lineno": {"type": "integer"},
                    },
                    "required": ["names", "lineno"],
                },
            },
            "classes": {
                "type": "array",
                "description": "Top-level classes with nested methods.",
                "items": {"type": "object"},
            },
            "functions": {
                "type": "array",
                "description": "Top-level functions.",
                "items": {"type": "object"},
            },
        },
        "required": ["path", "ok", "error"],
    }


    class OutlineTool(ToolDefinition):
        name = "python-ast-outline"
        title = "Python outline"
        description = (
            "Token-efficient structural overview of Python files: file metrics, "
            "imports, and a class/function hierarchy with line ranges and short "
            "docstrings. Accepts one or several files at once."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "paths": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Absolute paths of Python files to outline.",
                }
            },
            "required": ["paths"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "all_ok": {"type": "boolean"},
                "files": {"type": "array", "items": _OUTLINE_ITEM_SCHEMA},
            },
            "required": ["all_ok", "files"],
        }
        annotations = {"readOnlyHint": True, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`outline_python_files`, translating the MCP schema to/from the Python API."""
            paths = ctx.arguments["paths"]
            if not isinstance(paths, list):
                return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
            try:
                result = outline_python_files(paths)
            except OutlineError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            return ToolResult(
                structured_content={
                    "all_ok": result.all_ok,
                    "files": [f.__dict__ for f in result.files],
                },
            )


    def register(registry: ToolRegistry) -> None:
        registry.register(OutlineTool())
```
/answer 613c-91 allow
Control Request:
```yaml
id: 613c-92
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  mode: replace
  reason: "ast/script.py nach dem insert-Vorlage-Muster refaktorieren: schemafreie Funktion run_ast_script(), handle() als Delegate"
  content: |
    """``python-ast-script`` – run restricted Python against the parsed AST.

    For complex reorganisation/optimisation the model can operate on the tree
    directly. The script runs with an empty ``__builtins__`` plus a small, curated
    set of safe names; the only capability handed in is the AST itself (``tree``)
    and the standard-library ``ast`` module. Any change to ``tree`` is persisted.
    """

    from __future__ import annotations

    import ast
    from dataclasses import dataclass
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
    from . import core

    __all__ = ["ScriptError", "AstScriptResult", "run_ast_script", "ScriptTool", "register"]

    _SAFE_BUILTINS = {
        name: getattr(__builtins__, name, None) if not isinstance(__builtins__, dict)
        else __builtins__.get(name)
        for name in (
            "isinstance", "issubclass", "getattr", "setattr", "hasattr", "delattr",
            "len", "list", "dict", "set", "tuple", "str", "int", "float", "bool",
            "enumerate", "range", "sorted", "reversed", "zip", "map", "filter",
            "any", "all", "min", "max", "sum", "type", "repr",
        )
    }


    class ScriptError(Exception):
        """Raised when an AST script cannot be run to completion."""


    @dataclass(frozen=True)
    class AstScriptResult:
        result: str
        value: str | None = None


    def run_ast_script(path: str, code: str) -> AstScriptResult:
        """Execute ``code`` in a restricted sandbox exposing the AST of ``path`` as ``tree``."""
        file_path = core.require_path(path)
        tree = core.CACHE.get_tree(file_path)
        env: dict[str, Any] = {"tree": tree, "ast": ast}
        sandbox_globals = {"__builtins__": _SAFE_BUILTINS}
        try:
            exec(compile(code, "<ast-script>", "exec"), sandbox_globals, env)  # noqa: S102
        except SyntaxError as exc:
            raise ScriptError(f"Script syntax error: {exc.msg}") from exc
        except Exception as exc:  # noqa: BLE001
            raise ScriptError(f"Script failed: {type(exc).__name__}: {exc}") from exc
        core.CACHE.save(file_path, tree)

        if "result" in env:
            return AstScriptResult(result="success", value=repr(env["result"]))
        return AstScriptResult(result="success")


    class ScriptTool(ToolDefinition):
        name = "python-ast-script"
        title = "Run AST script"
        description = (
            "Run restricted Python against a file's AST for complex/incremental "
            "transforms. Globals expose 'tree' (ast.Module) and 'ast'; assign "
            "'result' to return data. Changes to 'tree' are saved."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "code": {"type": "string", "description": "Python script operating on 'tree'."},
            },
            "required": ["path", "code"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "result": {"type": "string"},
                "value": {"description": "Repr of the script's 'result' variable, if set."},
            },
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`run_ast_script`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = run_ast_script(args["path"], args["code"])
            except (core.AstError, ScriptError) as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)

            structured: dict[str, Any] = {"result": result.result}
            if result.value is not None:
                structured["value"] = result.value
            return ToolResult(structured_content=structured)


    def register(registry: ToolRegistry) -> None:
        registry.register(ScriptTool())
```
/answer 613c-92 allow
Control Request:
```yaml
id: 613c-93
toolName: python-ast-validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/change/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_chars/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_lines/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file-stats/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/markdown/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ask-user/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/file_ops.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: Syntaxprüfung aller refaktorierten Tool-Dateien
```
/answer 613c-93 allow
Control Request:
```yaml
id: 613c-94
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |
    set -e
    python3 - <<'PY'
    import sys
    sys.path.insert(0, "src")
    import importlib

    modules = [
        "xy.ai.mcpc.tools.insert",
        "xy.ai.mcpc.tools.change",
        "xy.ai.mcpc.tools.replace_chars",
        "xy.ai.mcpc.tools.replace_lines",
        "xy.ai.mcpc.tools.replace_block",
        "xy.ai.mcpc.tools.write",
        "xy.ai.mcpc.tools.read",
        "xy.ai.mcpc.tools.list",
        "xy.ai.mcpc.tools.file-stats",
        "xy.ai.mcpc.tools.bash",
        "xy.ai.mcpc.tools.python",
        "xy.ai.mcpc.tools.markdown",
        "xy.ai.mcpc.tools.colgrep",
        "xy.ai.mcpc.tools.ask-user",
        "xy.ai.mcpc.tools.process",
        "xy.ai.mcpc.tools.ast.file_ops",
        "xy.ai.mcpc.tools.ast.validate",
        "xy.ai.mcpc.tools.ast.outline",
        "xy.ai.mcpc.tools.ast.script",
    ]
    for m in modules:
        importlib.import_module(m)
        print("OK", m)

    from xy.ai.mcpc.tools.insert import insert
    from xy.ai.mcpc.tools.write import write_file
    from xy.ai.mcpc.tools.read import read_file
    from xy.ai.mcpc.tools.change import change
    from xy.ai.mcpc.tools.replace_chars import replace_chars
    from xy.ai.mcpc.tools.replace_lines import replace_lines
    from xy.ai.mcpc.tools.replace_block import replace_block
    from xy.ai.mcpc.tools.python import run_python

    import tempfile, os
    tmp = tempfile.mktemp()
    write_file(tmp, "replace", "hello world\n")
    print(read_file(tmp))
    insert(tmp, 5, "!!!")
    print(read_file(tmp))
    change(tmp, "hello", "world", "HI THERE")
    print(read_file(tmp))
    os.remove(tmp)

    print(run_python("print(1+1)"))
    PY
  reason: "Rauchtest: Prüfen, dass die refaktorierten Module importierbar sind und die schemafreien Delegate-Funktionen (insert, write_file, read_file, change, run_python etc.) korrekt funktionieren"
```
/answer 613c-94 allow
Control Request:
```yaml
id: 613c-95
result:
  structuredContent:
    stdout: |
      OK xy.ai.mcpc.tools.insert
      OK xy.ai.mcpc.tools.change
      OK xy.ai.mcpc.tools.replace_chars
      OK xy.ai.mcpc.tools.replace_lines
      OK xy.ai.mcpc.tools.replace_block
      OK xy.ai.mcpc.tools.write
      OK xy.ai.mcpc.tools.read
      OK xy.ai.mcpc.tools.list
      OK xy.ai.mcpc.tools.file-stats
      OK xy.ai.mcpc.tools.bash
      OK xy.ai.mcpc.tools.python
      OK xy.ai.mcpc.tools.markdown
      OK xy.ai.mcpc.tools.colgrep
      OK xy.ai.mcpc.tools.ask-user
      OK xy.ai.mcpc.tools.process
      OK xy.ai.mcpc.tools.ast.file_ops
      OK xy.ai.mcpc.tools.ast.validate
      OK xy.ai.mcpc.tools.ast.outline
      OK xy.ai.mcpc.tools.ast.script
      ReadResult(content='hello world\n', checksum='a948904f2f0f479b8f8197694b30184b0d2ed1c1cd2a1ec0fb85d299a192a447', is_full_file=True)
      ReadResult(content='hello!!! world\n', checksum='5ceabd621332fa4e839c37717e0243494f1931f9e7e51c690161cbf7e7d001d8', is_full_file=True)
      ReadResult(content='HI THERE\n', checksum='20be25fb251547f6ad1530e703651912d31875bf271f281afe8cb9095e689a77', is_full_file=True)
      ProcessResult(exit_code=0, stdout='2\n', stderr='')
```
/answer 613c-95 allow
ReasoningToken: 91
ReasoningToken: 95
ReasoningToken: 9
ReasoningToken: 21
ReasoningToken: 163
ReasoningToken: 151
ReasoningToken: 12
ReasoningToken: 142
ReasoningToken: 156
Text:
 Jetzt validiere ich alle geänderten Dateien syntaktisch:
Text:
 Alle Dateien sind syntaktisch gültig. Nun noch ein funktionaler Rauchtest, um zu prüfen, dass die Registrierung/Imports weiterhin funktionieren (insbesondere der `importlib`-Import in `ast/outline.py` und die geteilten `process.py`-Funktionen):
Alle Module importieren erfolgreich und die schemafreien Delegate-Funktionen arbeiten korrekt zusammen (`write_file` → `insert` → `change` → `read_file`, `run_python`).

## Zusammenfassung

Alle 19 Tools, die `ToolDefinition` implementieren, wurden nach dem `insert`-Vorlagenmuster refaktoriert:

**Datei-Tools:** `change`, `replace_chars`, `replace_lines`, `replace_block`, `write`, `read`, `list`, `file-stats`
**Prozess-Tools:** `bash`, `python`, `markdown` (gemeinsame Basis in `process.py` in `run_process`/`ProcessResult`/`LaunchError` (schemafrei) und `pack_process_result` (MCP-Packing) aufgeteilt)
**Sonstige:** `colgrep`, `ask-user`
**AST-Tools:** `ast/file_ops.py`, `ast/validate.py`, `ast/outline.py`, `ast/script.py` (nutzen weiterhin `ast/core.py`s `AstError`/`CACHE` als gemeinsame Basis)

Jede Datei folgt jetzt konsequent dem Muster:

1. **`<Name>Error`** – schemafreie Exception statt direktem `ToolResult`.
2. **`<Name>Result`** (frozen `dataclass`) – typisiertes, exportiertes Rückgabeobjekt.
3. **freie Funktion** mit korrekter Python-Signatur/Typen (z. B. `insert(path, offset, content)`, `bash(cwd, script)`, `outline_python_files(paths)`) – eigenständig importierbar, unabhängig vom MCP-Schema.
4. **`<Name>Tool.handle()`** – reine Delegate-Methode: entpackt `ctx.arguments`, ruft die Funktion auf, fängt die Error-Klasse ab und packt das Ergebnis zurück ins MCP-Ausgabeschema.
5. **`__all__`** – definiert die öffentliche API des Moduls.

Besonderheiten:
- `file-stats`: `compute_file_stats(Path) -> dict` blieb unverändert bestehen (wird von `ast/outline.py` per `importlib` genutzt), zusätzlich neue typisierte `file_stats(str) -> FileStatsResult`.
- `read`: Kernfunktion `read_file()` liefert nur Inhalt/Checksumme; das sessionbezogene Change-Caching bleibt bewusst in `handle()`, da es reine MCP-Session-Logik ist.
- `bash`/`python`/`markdown`: Ausführung (`run_process`) und MCP-Packing (Stream-Normalisierung, Spill-to-file, Exit-Code-Unterdrückung) wurden sauber getrennt.

Alle Dateien wurden syntaktisch validiert und per Rauchtest (Import + Funktionsaufrufe) verifiziert.
