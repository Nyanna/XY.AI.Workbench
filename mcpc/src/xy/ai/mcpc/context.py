"""Process-wide services exposed to tool handlers.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .cli import CliSessionManager
    from .config import ServerConfig
    from .control import ToolControlManager
    from .registry import ToolRegistry
    from .session import SessionStore
    from .tools.agent.profiles import ProfileRegistry


@dataclass(slots=True)
class AppServices: 
    """Container for the shared components a tool handler may need."""

    config: "ServerConfig"
    registry: "ToolRegistry"
    sessions: "SessionStore"
    cli_manager: "CliSessionManager"
    profiles: "ProfileRegistry"
    control_manager: "ToolControlManager | None" = None
