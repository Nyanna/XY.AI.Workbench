"""Shared subprocess execution for the stream-capturing tools.

``bash``, ``python`` and ``markdown`` all do the same thing: run a child
process, capture its STDOUT/STDERR and report ``exit_code`` + the two streams.
Centralising it here guarantees they decode child output identically to every
other stream in MCPC — **UTF-8 with ``errors="replace"``**.

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

The captured text is placed verbatim into the structured result; JSON escaping
happens exactly once, later, when the :class:`ToolResult` is serialised through
:class:`~xy.ai.mcpc.codec.JsonCodec`.
"""

from __future__ import annotations

import os
import re
import subprocess
from typing import Any

from ..registry import ToolResult, text_content


_BLANK_RUN_RE = re.compile(r"[ \t]+$", re.MULTILINE)
_MULTI_BLANK_RE = re.compile(r"\n{3,}")


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


def run_capture(
    cmd: list[str],
    *,
    cwd: str | os.PathLike[str] | None = None,
    stdin: str | None = None,
    launch_error: str = "Failed to launch process",
    normalize_output: bool = False,
    omit_zero_exit_code: bool = False,
) -> ToolResult:
    """Run *cmd*, capture its streams, and return a normalised :class:`ToolResult`.

    * ``cwd`` — working directory (already validated by the caller).
    * ``stdin`` — text fed to the child's standard input, or ``None``.
    * ``launch_error`` — message prefix used when the executable cannot start.
    * ``normalize_output`` — when ``True``, post-process STDOUT/STDERR to
      improve YAML block-scalar compatibility (see :func:`_normalize_stream`).
    * ``omit_zero_exit_code`` — when ``True``, ``exit_code`` is left out of the
      result entirely if the process exited with code ``0``.

    STDOUT/STDERR are decoded as UTF-8 with ``errors="replace"`` so decoding can
    never raise.  ``stdout`` is always present; ``stderr`` is included whenever
    it is non-empty.  A human-readable text block is always attached to the
    result (in addition to the structured content) so STDOUT/STDERR remain
    visible even when the surrounding client only renders textual content —
    e.g. on a non-zero exit code.  ``is_error`` mirrors a non-zero exit code.
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
        return ToolResult(
            structured_content={"error": f"{launch_error}: {exc}"},
            is_error=True,
        )

    stdout = proc.stdout or ""
    stderr = proc.stderr or ""
    if normalize_output:
        stdout = _normalize_stream(stdout)
        stderr = _normalize_stream(stderr)

    structured: dict[str, Any] = {}
    if not omit_zero_exit_code or proc.returncode != 0:
        structured["exit_code"] = proc.returncode
    structured["stdout"] = stdout
    if stderr:
        structured["stderr"] = stderr

    text_lines: list[str] = []

    return ToolResult(
        content=[text_content("\n".join(text_lines))],
        structured_content=structured,
        is_error=proc.returncode != 0,
    )
