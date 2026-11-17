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
import os
import re
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.ast import core as _ast_core
from xy.ai.mcpc.tools.ast.outline import ast_outline as _ast_outline
from xy.ai.mcpc.tools.file_stats import compute_file_stats as _compute_file_stats
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
_BLANK_RUN_RE = re.compile('[ \\t]+$', re.MULTILINE)
_MULTI_BLANK_RE = re.compile('\\n{3,}')

class LaunchError(Exception):
    """Raised when the child process could not be started."""

@dataclass(frozen=True)
class ProcessResult:
    exit_code: int
    stdout: str
    stderr: str

@dataclass(frozen=True)
class ProcessOutput(ItemResult):
    """Wire form of a finished process; ``index`` associates it with its input item in a batch."""
    index: int | None = None
    exit_code: int | None = None
    stdout: str | None = None
    stderr: str | None = None
    stdout_file: str | None = None
    stdout_file_stats: dict[str, Any] | None = None
    stdout_file_outline: list[dict[str, Any]] | None = None
    stderr_file: str | None = None
    stderr_file_stats: dict[str, Any] | None = None
    stderr_file_outline: list[dict[str, Any]] | None = None

    def key(self) -> dict[str, Any]:
        return {} if self.index is None else {'index': self.index}

@dataclass(frozen=True)
class ProcessItemError(ItemError):
    """An item that could not be run at all."""
    index: int

    def key(self) -> dict[str, Any]:
        return {'index': self.index}

class ProcessBatchResult(BatchResult[ProcessOutput, ProcessItemError]):
    """Outcomes of a batch of process runs, associated with the input by ``index``."""

def run_process(cmd: list[str], *, cwd: str | os.PathLike[str] | None=None, stdin: str | None=None) -> ProcessResult:
    """Run *cmd* to completion and return its captured result.

    Raises :class:`LaunchError` if the executable cannot be started.
    """
    try:
        proc = subprocess.run(cmd, input=stdin, cwd=os.fspath(cwd) if cwd is not None else None,
                              capture_output=True, encoding='utf-8', errors='replace')
    except OSError as exc:
        raise LaunchError(str(exc)) from exc
    return ProcessResult(exit_code=proc.returncode, stdout=proc.stdout or '', stderr=proc.stderr or '')

def _normalize_stream(text: str) -> str:
    """Improve compatibility with YAML block scalars.

    * Lines that contain only whitespace are reduced to a bare line break
      (trailing spaces/tabs on otherwise empty lines are stripped).
    * Successive blank lines are collapsed to a single blank line.
    """
    if not text:
        return text
    normalized = _BLANK_RUN_RE.sub('', text)
    normalized = _MULTI_BLANK_RE.sub('\n\n', normalized)
    return normalized

def _spill_to_file(text: str, label: str) -> str:
    """Write *text* to a fresh temp file and return its absolute path.

    Used as a safety limit: when a captured stream grows too large to be
    returned inline, it is persisted to disk instead so the caller can
    continue operating on it (e.g. via the ``read``/``bash`` tools) without
    the full content ever passing through the structured result.
    """
    fd, path = tempfile.mkstemp(prefix=f'mcpc-{label}-', suffix='.log')
    try:
        with os.fdopen(fd, 'w', encoding='utf-8') as fh:
            fh.write(text)
    except BaseException:
        os.close(fd)
        raise
    return path

def _spill_extra_info(path: str, ctx: ToolContext) -> tuple[dict[str, Any], list[dict[str, Any]] | None, list[str]]:
    """Describe a spilled-to-disk stream: file stats, plus the outline when ``ast_outline`` is enabled."""
    stats = _compute_file_stats(Path(path)).payload()
    outline: list[dict[str, Any]] | None = None
    warnings: list[str] = []
    if {'tools', 'ast_outline'} & ctx.session.enabled_tools:
        batch = _ast_outline(paths=[path])
        if batch.results:
            outline = [_ast_core.to_dict(n) for n in batch.results[0].nodes]
        elif batch.errors:
            warnings.append(f'ast_outline of `{path}` failed: {batch.errors[0].error}')
    return (stats, outline, warnings)

def pack_process_result(result: ProcessResult, ctx: ToolContext, *, normalize_output: bool=False, omit_zero_exit_code: bool=False, max_stream_chars: int | None=None, index: int | None=None) -> ProcessOutput:
    """Pack a :class:`ProcessResult` into a :class:`ProcessOutput`.

    * ``normalize_output`` — post-process STDOUT/STDERR to improve YAML
      block-scalar compatibility (see :func:`_normalize_stream`).
    * ``omit_zero_exit_code`` — leave ``exit_code`` out if the process exited with 0.
    * ``max_stream_chars`` — safety limit on the characters of STDOUT/STDERR
      returned inline. A longer stream is written to a temp file; the output then
      carries ``<stream>_file`` plus ``<stream>_file_stats`` (and
      ``<stream>_file_outline`` when the ``ast_outline`` tool is enabled) instead of
      the text, and a warning says so. ``None`` disables the limit.
    * ``index`` — position of the item in the input, for batch association.

    ``stdout`` is present unless spilled; ``stderr`` whenever it is non-empty. A
    non-zero ``exit_code`` is a normal outcome, not a failure of the tool call.
    """
    stdout = result.stdout
    stderr = result.stderr
    if normalize_output:
        stdout = _normalize_stream(stdout)
        stderr = _normalize_stream(stderr)
    fields: dict[str, Any] = {}
    warnings: list[str] = []

    def spill(label: str, text: str) -> None:
        path = _spill_to_file(text, label)
        stats, outline, notes = _spill_extra_info(path, ctx)
        fields[f'{label}_file'] = path
        fields[f'{label}_file_stats'] = stats
        fields[f'{label}_file_outline'] = outline
        warnings.append(f'{label.upper()} limit of characters exceeded, output saved to `{path}`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline).')
        warnings.extend(notes)
    if max_stream_chars is not None and len(stdout) > max_stream_chars:
        spill('stdout', stdout)
    else:
        fields['stdout'] = stdout
    if stderr:
        if max_stream_chars is not None and len(stderr) > max_stream_chars:
            spill('stderr', stderr)
        else:
            fields['stderr'] = stderr
    exit_code = None if omit_zero_exit_code and result.exit_code == 0 else result.exit_code
    return ProcessOutput(index=index, exit_code=exit_code, warnings=tuple(warnings), **fields)