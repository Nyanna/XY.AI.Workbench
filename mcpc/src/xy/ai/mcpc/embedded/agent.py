"""Agents driven from a script against an embedded MCPC."""
from __future__ import annotations
from collections.abc import Iterable
from enum import Enum
from typing import TYPE_CHECKING
from xy.ai.mcpc.cli.parameters import CacheMode, CliParameters, Effort, Model
from xy.ai.mcpc.cli.session import CliResult, CliSession, CliSessionError
if TYPE_CHECKING:
    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.embedded.mcpc import EmbeddedMcpc
__all__ = ['Agent', 'CacheMode', 'CliResult', 'CliSessionError', 'Effort', 'Model']

def _value(value: Enum | str) -> str:
    return value.value if isinstance(value, Enum) else value

def agent_parameters(config: ServerConfig, mcpc_session_id: str, system_prompt: str, *, model: Model | str, effort: Effort | str, cc_profile: str, cache_mode: CacheMode | str, executable: str, extra_env: dict[str, str] | None) -> CliParameters:
    return CliParameters(
        config=config,
        model=_value(model),
        system_prompt=system_prompt,
        mcpc_session_id=mcpc_session_id,
        effort=_value(effort),
        cc_profile=cc_profile,
        executable=executable,
        extra_env=dict(
            extra_env or {}),
        cache_mode=_value(cache_mode))

class Agent:
    """A Claude CLI agent wired to an :class:`EmbeddedMcpc` session.

    Created via :meth:`EmbeddedMcpc.start_agent` or :func:`launch_agent`. The
    conversation persists across :meth:`prompt` calls. Closing releases the CLI
    process and the MCPC session (and the server itself when the agent owns it).
    """

    def __init__(self, mcpc: EmbeddedMcpc, cli: CliSession, mcpc_session_id: str, *, owns_mcpc: bool=False) -> None:
        self.mcpc = mcpc
        self.cli = cli
        self.mcpc_session_id = mcpc_session_id
        self.owns_mcpc = owns_mcpc

    @property
    def id(self) -> str:
        return self.cli.id

    def prompt(self, text: str) -> CliResult:
        return self.cli.prompt(text)

    def ask(self, text: str) -> str:
        """Like :meth:`prompt` but return only the text; raise :class:`CliSessionError` on an error result."""
        result = self.prompt(text)
        if result.is_error:
            raise CliSessionError(result.text)
        return result.text

    def close(self) -> None:
        self.cli.terminate()
        if self.owns_mcpc:
            self.mcpc.stop()
        elif self.mcpc.running:
            self.mcpc.server.environment.cli_manager.discard(self.cli.id)
            self.mcpc.close_session(self.mcpc_session_id)

    def __enter__(self) -> Agent:
        return self

    def __exit__(self, *exc_info: object) -> None:
        self.close()