"""Java engine: JavaParser-based ``ast-java`` process, driven over RPC/PIC.

Replaces the old tree-sitter :class:`~xy.ai.mcpc.tools.ast.generic._java.
JavaEngine` (kept around, but no longer wired up) with the real Java parser
under ``ast-engines/java``, started lazily -- and kept running for the
server's lifetime -- the same way ``ast-engines/java/run.sh`` does, minus the
shell: we invoke ``java`` (from ``$JAVA_HOME`` if set) directly against the
prebuilt classpath; building it (``run.sh --build``) stays a separate,
externally-guaranteed step.
"""
from __future__ import annotations
import os
from pathlib import Path
from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
__all__ = ['JavaEngine']

def _workspace_root() -> Path:
    for ancestor in Path(__file__).resolve().parents:
        if (ancestor / 'ast-engines').is_dir():
            return ancestor
    raise RuntimeError(f"could not locate an 'ast-engines' directory above {__file__}")
_ENGINE_DIR = _workspace_root() / 'ast-engines' / 'java'
_SRC_DIR = _ENGINE_DIR / 'src'
_BIN_DIR = _ENGINE_DIR / '.bin'
_LIBS_DIR = _ENGINE_DIR / 'libs'
_MAIN_CLASS = 'xy.ai.mcpc.ast.engine.Main'

class JavaProcess(RpcProcess):
    """Starts the prebuilt ``ast-java`` JavaParser engine (built via ``run.sh --build``)."""

    def default_port(self) -> int:
        return int(os.environ.get('AST_JAVA_PORT', '8787'))

    def cwd(self) -> str | None:
        return str(_ENGINE_DIR)

    def command(self, port: int) -> list[str]:
        return [self._java_tool('java'), '-cp', self._classpath(), _MAIN_CLASS, str(port)]

    def _java_tool(self, name: str) -> str:
        java_home = os.environ.get('JAVA_HOME')
        if java_home:
            candidate = Path(java_home) / 'bin' / name
            if candidate.exists():
                return str(candidate)
        return name

    def _classpath(self) -> str:
        jars = sorted((str(p) for p in _LIBS_DIR.glob('*.jar')
                      if not p.name.endswith(('-sources.jar', '-javadoc.jar'))))
        return os.pathsep.join([str(_BIN_DIR), *jars])
_PROCESS = JavaProcess()

class JavaEngine(RpcEngine):
    """Java, parsed and mutated by JavaParser via the ``ast-java`` RPC process."""
    validates_syntax = True

    def __init__(self) -> None:
        super().__init__('java', _PROCESS)