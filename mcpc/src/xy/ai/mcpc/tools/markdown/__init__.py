"""Markdown tool – AST-based reading/writing/transforming of Markdown files.

The tool runs a TypeScript (ESM) script inside a pre-provisioned Node.js package
environment that exposes ``remark``, ``remark-behead`` and ``remark-frontmatter``
(via a local ``createRemark`` helper).  The script is handed to
``node --input-type=module`` on standard input and executed with the environment
directory as its working directory, so bare package imports resolve.
"""
from pathlib import Path
from typing import Any
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['MarkdownError', 'MarkdownRunner', 'MarkdownTool', 'register_markdown_tool']
_EXAMPLE = 'import { read, write } from \'to-vfile\';\nimport { createRemark } from \'./remark.js\';\nimport { visit } from \'unist-util-visit\';\n\nconst processor = createRemark({\n  // frontmatter: true, // if required\n  // behead: { depth: 1 }, // if required\n});\n\nprocessor.use(() => (tree, file) => {\n  // insert code here\n});\n\n// read file – replace \'path/to/file.md\' with the actual file path\nconst file = await read(\'path/to/file.md\');\n\n// parse to AST\nconst tree = await processor.run(processor.parse(file), file);\n\n// Extract headings\nconst headings = [];\nvisit(tree, \'heading\', (node) => {\n    headings.push({\n    depth: node.depth,\n    text: node.children.map(c => c.value || c.children?.map(x => x.value).join(\'\') || \'\').join(\'\').trim()\n    });\n});\n\n// format output\nawait processor.process(file);\nfile.path = \'path/to/file.md\';\nawait write(file);\n\nconsole.log(String("Done"));\n'
_DESCRIPTION = 'AST-based reading, writing, modifying and transforming of Markdown files. Provide a TypeScript (ESM) script that uses `remark` (with `remark-behead` and `remark-frontmatter` available) to operate on Markdown. Returns the exit code, standard output and, if present, standard error.\n\nFollow this pattern:\n\n```typescript\n' + _EXAMPLE + '```'

class MarkdownError(Exception):
    """Raised when a Markdown (remark) script cannot be executed."""

class MarkdownRunner:
    """Runs remark scripts against a pre-provisioned Node.js environment.

    ``env_dir`` (the remark environment root, containing ``node_modules`` and
    ``package.json``) is bound once, at construction, instead of being passed
    to every call: it is part of the environment, never an argument that
    varies between individual script invocations. Public callers of
    :meth:`run_markdown` therefore only ever supply what actually differs per
    call — the script itself.

    ``markdown`` keeps the name of the MCP tool it backs on purpose: it is
    the published, stable signature plugins can rely on and call on any
    runner instance they are handed, without needing to know about (or
    import) :class:`MarkdownRunner` itself.
    """

    def __init__(self, env_dir: Path | None=None) -> None:
        self._env_dir = env_dir or ServerConfig().markdown_env_dir

    def markdown(self, script: str) -> ProcessResult:
        """Run ``script`` against the bound remark environment.

        Args:
            script: JavaScript/remark script content to execute.

        Returns:
            ProcessResult with:
                exit_code: Exit code of remark process.
                stdout: Standard output (up to 3000 chars; see stdout_file if longer).
                stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
                stdout_file: Absolute path to temp file with full stdout if > 3000 chars.
                stderr_file: Absolute path to temp file with full stderr if > 3000 chars.

        Raises:
            MarkdownError: If remark/node cannot be launched.
        """
        try:
            return run_process(['node', '-'], input_text=script, cwd=self._env_dir)
        except LaunchError as exc:
            raise MarkdownError(f'Failed to launch remark: {exc}') from exc

class MarkdownTool(ToolDefinition):
    name = 'markdown'
    title = 'Run Markdown (remark) script'
    description = _DESCRIPTION
    input_schema = {
        'type': 'object',
        'properties': {
            'script': {
                'type': 'string',
                'description': 'TypeScript (ESM) script content to execute against the remark environment.'}},
        'required': ['script']}
    output_schema = {
        'type': 'object', 'properties': {
            'exit_code': {
                'type': 'integer'}, 'stdout': {
                    'type': 'string'}, 'stderr': {
                        'type': 'string'}}, 'required': [
                            'exit_code', 'stdout']}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': True}

    def __init__(self, env_dir: Path | None=None) -> None:
        """# The runner is bound once, at registration; MarkdownTool itself"""
        '# never touches ServerConfig again after construction.'
        self._runner = MarkdownRunner(env_dir)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to the bound :class:`MarkdownRunner` and pack the result into the MCP output schema."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = self._runner.markdown(args['script'])
        except MarkdownError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return pack_process_result(result)

def register_markdown_tool(registry: ToolRegistry, environment: AppEnvironment) -> None:
    tool = MarkdownTool(environment.config.markdown_env_dir)
    registry.register(tool)
    environment.functions.register(tool._runner.markdown)