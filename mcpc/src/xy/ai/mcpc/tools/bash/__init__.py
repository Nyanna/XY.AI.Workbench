"""Bash tool – executes a shell script inside a specified working directory."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from ..process import run_capture

#: Safety limit on inline STDOUT/STDERR size. Streams larger than this are
#: written to a temp file instead, and the absolute path is returned so the
#: caller can keep operating on the output (e.g. via the ``read`` tool).
_MAX_STREAM_CHARS = 2000


def register_bash_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "bash",
        title="Run Bash script",
        description=(
            "Execute a Bash script in the specified working directory. "
            "Returns the exit code, standard output and, if present, standard error output. "
            f"As a safety limit, STDOUT/STDERR longer than {_MAX_STREAM_CHARS} characters are "
            "written to a temp file instead; the absolute file path is returned "
            "(as 'stdout_file'/'stderr_file') so it can be inspected further."
        ),
        input_schema={
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
        },
        output_schema={
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
        },
        annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": True},
    )
    def bash(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        cwd_str: str = args["cwd"]
        script: str = args["script"]

        cwd = Path(cwd_str)
        if not cwd.is_absolute():
            return ToolResult(
                content=[text_content(f"cwd must be an absolute path.")],
                is_error=True,
            )
        if not cwd.is_dir():
            return ToolResult(
                content=[text_content(f"Working directory not found or not a directory.")],
                is_error=True,
            )

        return run_capture(
            ["bash", "-c", script],
            cwd=cwd,
            launch_error="Failed to launch bash",
            normalize_output=True,
            omit_zero_exit_code=True,
            max_stream_chars=_MAX_STREAM_CHARS,
        )
