"""Rust engine: syn-based ``xy_ai_ast_rust`` process, driven over RPC/PIC.

Started lazily -- and kept running for the server's lifetime -- the same way
``ast-engines/rust/run.sh`` does, minus the shell: we invoke the prebuilt
release binary directly; building it (``run.sh --build``) stays a separate,
externally-guaranteed step.
"""
from __future__ import annotations
import os
from pathlib import Path
from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
__all__ = ['RustEngine']

def _workspace_root() -> Path:
    for ancestor in Path(__file__).resolve().parents:
        if (ancestor / 'ast-engines').is_dir():
            return ancestor
    raise RuntimeError(f"could not locate an 'ast-engines' directory above {__file__}")
_ENGINE_DIR = _workspace_root() / 'ast-engines' / 'rust'
_BINARY = _ENGINE_DIR / '.bin' / 'release' / 'xy_ai_ast_rust'

class RustProcess(RpcProcess):
    """Starts the prebuilt ``xy_ai_ast_rust`` syn engine (built via ``run.sh --build``)."""

    def default_port(self) -> int:
        return int(os.environ.get('AST_RUST_PORT', '8788'))

    def cwd(self) -> str | None:
        return str(_ENGINE_DIR)

    def command(self, port: int) -> list[str]:
        return [str(_BINARY), str(port)]
_PROCESS = RustProcess()

class RustEngine(RpcEngine):
    """Rust, parsed and mutated by ``syn`` via the ``xy_ai_ast_rust`` RPC process."""
    validates_syntax = True

    def __init__(self) -> None:
        super().__init__('rust', _PROCESS)