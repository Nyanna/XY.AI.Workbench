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
