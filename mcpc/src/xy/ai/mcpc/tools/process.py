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
import tempfile
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


def run_capture(
    cmd: list[str],
    *,
    cwd: str | os.PathLike[str] | None=None,
    stdin: str | None=None,
    launch_error: str="Failed to launch process",
    normalize_output: bool=False,
    omit_zero_exit_code: bool=False,
    max_stream_chars: int | None=None,
) -> ToolResult:
    """Run *cmd*, capture its streams, and return a normalised :class:`ToolResult`.

    * ``cwd`` — working directory (already validated by the caller).
    * ``stdin`` — text fed to the child's standard input, or ``None``.
    * ``launch_error`` — message prefix used when the executable cannot start.
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

    STDOUT/STDERR are decoded as UTF-8 with ``errors="replace"`` so decoding can
    never raise.  ``stdout`` is always present; ``stderr`` is included whenever
    it is non-empty.  The result carries no separate text content block —
    ``structured_content`` alone conveys STDOUT/STDERR, avoiding duplication.
    ``is_error`` mirrors a non-zero exit code.
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
            content=[text_content(f"{launch_error}: {exc}")],
            is_error=True,
        )

    stdout = proc.stdout or ""
    stderr = proc.stderr or ""
    if normalize_output:
        stdout = _normalize_stream(stdout)
        stderr = _normalize_stream(stderr)

    content: list[dict[str, Any]] = []
    structured: dict[str, Any] = {}
    if not omit_zero_exit_code or proc.returncode != 0:
        structured["exit_code"] = proc.returncode

    if max_stream_chars is not None and len(stdout) > max_stream_chars:
        stdout_file = _spill_to_file(stdout, "stdout")
        content.append(
            text_content(
                f"Full output written to file {len(stdout)} characters). "
                f"Read only relevant excerpts (e.g. via grep/head/tail)."
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
                    f"Full output written to file {len(stderr)} characters). "
                    f"Read only relevant excerpts (e.g. via grep/head/tail)."
                )
            )
            structured["stderr_file"] = stderr_file
        else:
            structured["stderr"] = stderr

    return ToolResult(
        content=content,
        structured_content=structured,
        is_error=proc.returncode != 0 and stderr,
    )
