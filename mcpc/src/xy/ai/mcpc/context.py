"""Process-wide services exposed to tool handlers.

Most tools only need the calling :class:`~xy.ai.mcpc.session.Session` (available
via :attr:`ToolContext.session`).  Tools that orchestrate *other* sessions — the
agent tool spawning sub-agents — additionally need the session store, the CLI
manager, and the agent-profile registry.  These are bundled here and threaded
through :class:`~xy.ai.mcpc.registry.ToolContext`.
"""

from __future__ import annotations

from dataclasses import dataclass, field
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
    #: Optional human-in-the-loop tool interceptor.  When set, every tool call
    #: is held for approval before execution and again before the result is
    #: returned to the agent.  ``None`` disables interception entirely.
    control_manager: "ToolControlManager | None" = None
