"""Built-in example tools.

These demonstrate the registry, per-session configuration and stateful
sessions.  Register them onto a :class:`ToolRegistry` with
:func:`register_builtin_tools`.
"""

from __future__ import annotations

from ..registry import ToolRegistry
from .builtin import register_builtin_tools

__all__ = ["ToolRegistry", "register_builtin_tools"]
