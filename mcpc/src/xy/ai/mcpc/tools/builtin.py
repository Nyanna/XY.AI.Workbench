"""Compatibility shim – the built-in example tools have been removed.

Use :func:`register_tools` from the parent package instead.
"""

from __future__ import annotations

from ..registry import ToolRegistry
from . import register_tools


def register_builtin_tools(registry: ToolRegistry) -> None:
    """Deprecated alias for :func:`register_tools`."""
    register_tools(registry)
