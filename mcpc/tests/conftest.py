"""Test setup: make the ``src`` layout importable without installation."""
from __future__ import annotations
import sys
from pathlib import Path
SRC = Path(__file__).resolve().parent.parent / 'src'
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))
'# noqa: E402'
import pytest
'# noqa: E402'
from xy.ai.mcpc.cli.manager import CliSessionManager
'# noqa: E402'
from xy.ai.mcpc.config import ServerConfig
'# noqa: E402'
from xy.ai.mcpc.server.session import SessionStore
'# noqa: E402'
from xy.ai.mcpc.tools import register_tools
'# noqa: E402'
from xy.ai.mcpc.tools.agent.profiles import DEFAULT_PROFILES, ProfileRegistry
'# noqa: E402'
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
'# noqa: E402'
from xy.ai.mcpc.tools.tool_context import AppEnvironment
'# noqa: E402'
from xy.ai.mcpc.tools.tool_registry import ToolRegistry

@pytest.fixture
def registry() -> ToolRegistry:
    """A ``ToolRegistry`` populated by ``register_tools()``, wired like the real server."""
    config = ServerConfig()
    reg = ToolRegistry()
    environment = AppEnvironment(
        config=config,
        registry=reg,
        functions=FunctionRegistry(),
        sessions=SessionStore(),
        cli_manager=CliSessionManager(
            log_dir=config.cli_log_dir,
            ttl_seconds=config.agent_session_ttl_seconds,
            response_timeout=config.agent_response_timeout_seconds),
        profiles=ProfileRegistry(
            list(DEFAULT_PROFILES)))
    register_tools(reg, environment)
    return reg