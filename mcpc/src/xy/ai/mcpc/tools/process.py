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
import subprocess
from typing import Any

from ..registry import ToolResult


def run_capture(
    cmd: list[str],
    *,
    cwd: str | os.PathLike[str] | None = None,
    stdin: str | None = None,
    launch_error: str = "Failed to launch process",
) -> ToolResult:
    """Run *cmd*, capture its streams, and return a normalised :class:`ToolResult`.

    * ``cwd`` — working directory (already validated by the caller).
    * ``stdin`` — text fed to the child's standard input, or ``None``.
    * ``launch_error`` — message prefix used when the executable cannot start.

    STDOUT/STDERR are decoded as UTF-8 with ``errors="replace"`` so decoding can
    never raise.  The structured payload always carries ``exit_code`` and
    ``stdout``; ``stderr`` is included only when non-empty.  ``is_error`` mirrors
    a non-zero exit code.
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

    structured: dict[str, Any] = {
        "exit_code": proc.returncode,
        "stdout": proc.stdout,
    }
    if proc.stderr:
        structured["stderr"] = proc.stderr

    return ToolResult(
        structured_content=structured,
        is_error=proc.returncode != 0,
    )
