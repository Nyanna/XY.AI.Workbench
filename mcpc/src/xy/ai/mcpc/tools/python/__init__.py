"""Python tool – executes a Python script directly from context (no file)."""

from __future__ import annotations

import sys
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult
from ..process import run_capture


def register_python_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "python",
        title="Run Python script",
        description=(
            "Execute a Python script passed directly as content, without writing "
            "a script file. The script is fed to the interpreter on standard input. "
            "Returns the exit code, standard output and, if present, standard error output."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "script": {
                    "type": "string",
                    "description": "Python script content to execute.",
                },
            },
            "required": ["script"],
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
    def python(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        script: str = args["script"]

        return run_capture(
            [sys.executable, "-"],
            stdin=script,
            launch_error="Failed to launch Python",
        )
