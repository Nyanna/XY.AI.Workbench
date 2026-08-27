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
from xy.ai.mcpc.tools.registry import ToolResult, text_content
_BLANK_RUN_RE = re.compile('[ \\t]+$', re.MULTILINE)
_MULTI_BLANK_RE = re.compile('\\n{3,}')

class LaunchError(Exception):
    """Raised when the child process could not be started."""

@dataclass(frozen=True)
class ProcessResult:
    exit_code: int
    stdout: str
    stderr: str

def run_process(cmd: list[str], *, cwd: str | os.PathLike[str] | None=None, stdin: str | None=None) -> ProcessResult:
    """Run *cmd* to completion and return its captured result.

    Raises :class:`LaunchError` if the executable cannot be started.
    """
    try:
        proc = subprocess.run(cmd, input=stdin, cwd=os.fspath(cwd) if cwd is not None else None, capture_output=True, encoding='utf-8', errors='replace')
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

def pack_process_result(result: ProcessResult, *, normalize_output: bool=False, omit_zero_exit_code: bool=False, max_stream_chars: int | None=None) -> ToolResult:
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
        structured['exit_code'] = result.exit_code
    if max_stream_chars is not None and len(stdout) > max_stream_chars:
        stdout_file = _spill_to_file(stdout, 'stdout')
        content.append(text_content(f'Full output written to file ({len(stdout)} characters). Before loading the file, reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk) to extract only the relevant parts.Only load the file with `file-read` once the output is already narrowed down to the essential information.'))
        structured['stdout_file'] = stdout_file
    else:
        structured['stdout'] = stdout
    if stderr:
        if max_stream_chars is not None and len(stderr) > max_stream_chars:
            stderr_file = _spill_to_file(stderr, 'stderr')
            content.append(text_content(f'Full output written to file ({len(stdout)} characters). Before loading the file, reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk) to extract only the relevant parts.Only load the file with `file-read` once the output is already narrowed down to the essential information.'))
            structured['stderr_file'] = stderr_file
        else:
            structured['stderr'] = stderr
    '# Simple success with auto_approve when exit code is 0 and both streams are empty'
    if result.exit_code == 0 and (not stdout) and (not stderr):
        return ToolResult(structured_content={'result': 'success'}, auto_approve=True)
    return ToolResult(content=content, structured_content=structured, is_error=result.exit_code != 0 and bool(stderr))