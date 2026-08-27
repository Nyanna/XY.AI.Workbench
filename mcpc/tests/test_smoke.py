"""Minimal end-to-end smoke tests exercising the main construction paths.

These go one step beyond plain imports: they call the actual entry points
(``register_tools``, ``build_server``) so that wiring mistakes introduced by
refactoring (missing dependencies, wrong constructor args, broken registry
registration, etc.) surface even when every module imports fine in isolation.
"""
from __future__ import annotations

import socket

import pytest

from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.tools import register_tools
from xy.ai.mcpc.tools.registry import ToolRegistry
from xy.ai.mcpc.server.server import build_server


def _free_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def test_register_tools_populates_registry():
    registry = ToolRegistry()
    register_tools(registry)
    assert len(registry) > 0, "expected register_tools() to add at least one tool"
    assert len(registry.names()) == len(registry)


def test_build_server_constructs_without_error():
    config = ServerConfig(host="127.0.0.1", port=_free_port())
    server = build_server(config=config, enable_control=True)
    try:
        assert server.config is config
        assert server.protocol is not None
        assert server.sessions is not None
    finally:
        server.server_close()
