"""``Engine`` back-ends delegating to external parsers over the RPC/PIC HTTP
protocol (``ast-engines/openapi.yaml``), each running as a lazily-started,
long-lived subprocess.

:mod:`xy.ai.mcpc.tools.ast.rpc._process` and :mod:`._engine` provide the
reusable process-supervision/``Engine`` plumbing; per-language modules like
:mod:`._java` only add the concrete subprocess command and static metadata.
"""
from __future__ import annotations
from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
__all__ = ['RpcEngine', 'RpcProcess', 'JavaEngine', 'RustEngine']