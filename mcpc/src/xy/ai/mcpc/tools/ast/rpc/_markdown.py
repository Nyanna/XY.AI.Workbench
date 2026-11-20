"""Markdown engine: remark-based ``ast-engines/markdown`` process, driven over RPC/PIC.

Replaces the old tree-sitter :class:`~xy.ai.mcpc.tools.ast.generic._markdown.
MarkdownEngine` (kept around, but no longer wired up). The Node.js engine is
started lazily -- and kept running for the server's lifetime -- the same way
``ast-engines/markdown/run.sh`` does, minus the shell: we invoke ``node``
directly on the engine's entry point; installing its dependencies
(``npm install``) stays a separate, externally-guaranteed step.
"""
from __future__ import annotations
import os
from pathlib import Path
from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
__all__ = ['MarkdownEngine']

def _workspace_root() -> Path:
    for ancestor in Path(__file__).resolve().parents:
        if (ancestor / 'ast-engines').is_dir():
            return ancestor
    raise RuntimeError(f"could not locate an 'ast-engines' directory above {__file__}")
_ENGINE_DIR = _workspace_root() / 'ast-engines' / 'markdown'
_MAIN = Path('src') / 'xy' / 'ai' / 'mcpc' / 'ast' / 'engine' / 'Main.ts'

class MarkdownProcess(RpcProcess):
    """Starts the ``ast-engines/markdown`` remark engine on Node.js."""

    def default_port(self) -> int:
        return int(os.environ.get('AST_MARKDOWN_PORT', '8789'))

    def cwd(self) -> str | None:
        return str(_ENGINE_DIR)

    def command(self, port: int) -> list[str]:
        return [os.environ.get('NODE', 'node'), str(_MAIN), str(port)]
_PROCESS = MarkdownProcess()

class MarkdownEngine(RpcEngine):
    """Markdown, parsed and mutated by remark via the ``ast-engines/markdown`` RPC process."""

    def __init__(self) -> None:
        super().__init__('markdown', _PROCESS)