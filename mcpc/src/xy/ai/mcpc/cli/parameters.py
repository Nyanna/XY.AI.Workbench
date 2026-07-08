"""Construction of the ``claude`` CLI command line and process environment.

This is a Python port of the reference ``buildBaseCommand`` /
``buildEnvironment`` from ``project/sessionmanager.md``.  The CLI is launched in
stream-json mode with no built-in tools; its only tool source is a self-hook
back into this MCPC server, which is what lets the pre-configured per-session
toolset take effect.
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from enum import Enum

from ..config import ServerConfig


class Model(str, Enum):
    """Models the agent tool may request."""

    HAIKU = "haiku"
    SONNET = "sonnet"
    OPUS = "opus"


class Effort(str, Enum):
    """Reasoning-effort levels the agent tool may request."""

    DISABLED = "disabled"
    MINIMAL = "minimal"
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    XHIGH = "xhigh"


#: Environment variable through which the CLI learns which MCPC session id to
#: present when it connects back for tools/hooks.
MCPC_SESSION_ENV = "MCPC_SESSION_ID"


@dataclass(slots=True)
class CliParameters:
    """Everything needed to (re)start a single CLI process.

    The MCP-server and tool-hook endpoints the spawned CLI connects back to are
    taken straight from this server's own :class:`ServerConfig` (host, port, MCP
    endpoint path and hook path), so there is a single source of truth.
    """

    #: The running server's configuration; the CLI's MCP + hook endpoints are
    #: derived from it.
    config: ServerConfig
    model: str
    system_prompt: str
    #: MCPC session id the CLI connects back with (its tools are pre-configured
    #: on that session).  Exposed to the CLI via the ``MCPC_SESSION_ID`` env var.
    mcpc_session_id: str
    effort: str = Effort.MEDIUM.value
    #: Selects the ``CLAUDE_CONFIG_DIR`` (``~/.claude-<profile>``) so different
    #: agent profiles keep isolated credentials/caches.
    cc_profile: str = "none"

    executable: str = "claude"
    extra_env: dict[str, str] = field(default_factory=dict)

    # -- URLs ---------------------------------------------------------------
    @property
    def base_url(self) -> str:
        return f"http://{self.config.host}:{self.config.port}"

    @property
    def mcp_url(self) -> str:
        return f"{self.base_url}{self.config.path}"

    @property
    def hook_url(self) -> str:
        return f"{self.base_url}{self.config.hook_path}"

    # -- Command ------------------------------------------------------------
    def build_base_command(self, cli_session_id: str) -> list[str]:
        """Build the command line shared by initial-start and resume."""
        session_header = self.config.session_header
        ccprofile_header = self.config.ccprofile_header

        settings = {
            "hooks": {
                "PreToolUse": [
                    {
                        "type": "http",
                        "url": self.hook_url,
                        "headers": {session_header: cli_session_id},
                        "timeout": 86400,
                    }
                ]
            }
        }
        mcp_config = {
            "mcpServers": {
                "mcpc": {
                    "type": "http",
                    "url": self.mcp_url,
                    "timeout": 86400000,
                    "headers": {session_header: cli_session_id, ccprofile_header: self.cc_profile},
                }
            }
        }

        cmd = [
            self.executable,
            "--system-prompt", self.system_prompt,
            "--tools", "",  # no built-in tools; tools come from MCPC
            "--settings", json.dumps(settings, ensure_ascii=False),
            "--mcp-config", json.dumps(mcp_config, ensure_ascii=False),
            "--verbose",
            "--include-hook-events",
            "--include-partial-messages",
            "--input-format", "stream-json",
            "--output-format", "stream-json",
            "--replay-user-messages",
            "--model", self.model,
        ]
        if self.effort != Effort.DISABLED.value:
            cmd += ["--effort", self.effort]
        # Until interactive permission prompts are handled, skip them.
        cmd.append("--dangerously-skip-permissions")
        return cmd

    def build_command(self, cli_session_id: str, *, resume: bool) -> list[str]:
        """Return the full command line for a given CLI session id.

        ``resume`` selects ``--resume`` over ``--session-id`` to reattach to an
        existing CLI-side session while reusing its prompt cache.
        """
        cmd = self.build_base_command(cli_session_id)
        cmd.append("--resume" if resume else "--session-id")
        cmd.append(cli_session_id)
        return cmd

    # -- Environment --------------------------------------------------------
    def build_environment(self, env: dict[str, str]) -> dict[str, str]:
        """Populate *env* (mutated in place) for the CLI process."""
        import os

        home = os.path.expanduser("~")
        env["CLAUDE_CONFIG_DIR"] = f"{home}/.claude-{self.cc_profile}"
        env[MCPC_SESSION_ENV] = self.mcpc_session_id
        env["CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS"] = "1"
        env["CLAUDE_CODE_DISABLE_SPELLCHECK"] = "true"
        # Thinking is only forcibly disabled when reasoning effort is disabled.
        if self.effort == Effort.DISABLED.value:
            env["CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING"] = "1"
            env["MAX_THINKING_TOKENS"] = "0"
        env["CLAUDE_CODE_DISABLE_AGENT_VIEW"] = "1"
        env["CLAUDE_CODE_DISABLE_BACKGROUND_TASKS"] = "1"
        env["CLAUDE_CODE_DISABLE_BUNDLED_SKILLS"] = "1"
        env["CLAUDE_CODE_DISABLE_CLAUDE_MDS"] = "1"
        env["CLAUDE_CODE_DISABLE_CRON"] = "1"
        env["CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS"] = "1"
        env["CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS"] = "1"
        env["CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC"] = "1"
        env["CLAUDE_CODE_DISABLE_POLICY_SKILLS"] = "1"
        env["CLAUDE_CODE_DISABLE_WORKFLOWS"] = "1"
        env["CLAUDE_CODE_ENABLE_AWAY_SUMMARY"] = "0"
        env["CLAUDE_CODE_ENABLE_BACKGROUND_PLUGIN_REFRESH"] = "1"
        env["CLAUDE_CODE_FORK_SUBAGENT"] = "0"
        env["CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY"] = "1"
        env["ENABLE_TOOL_SEARCH"] = "false"
        env["CLAUDE_CODE_DISABLE_ADVISOR_TOOL"] = "1"
        env.update(self.extra_env)
        return env
