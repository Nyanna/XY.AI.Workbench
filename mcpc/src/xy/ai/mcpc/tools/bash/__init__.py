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
