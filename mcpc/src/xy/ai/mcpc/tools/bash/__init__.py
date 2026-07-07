"""Bash tool – executes a shell script inside a specified working directory."""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult


def register_bash_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "bash",
        title="Run Bash script",
        description=(
            "Execute a Bash script in the specified working directory. "
            "Returns the exit code, standard output and, if present, standard error output."
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
            },
            "required": ["exit_code", "stdout"],
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
                structured_content={"error": f"cwd must be an absolute path: {cwd_str}"},
                is_error=True,
            )
        if not cwd.is_dir():
            return ToolResult(
                structured_content={"error": f"Working directory not found or not a directory: {cwd_str}"},
                is_error=True,
            )

        try:
            proc = subprocess.run(
                ["bash", "-c", script],
                cwd=str(cwd),
                capture_output=True,
                text=True,
            )
        except OSError as exc:
            return ToolResult(
                structured_content={"error": f"Failed to launch bash: {exc}"},
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
