"""Server configuration.

All runtime configuration lives in a single immutable :class:`ServerConfig`
dataclass so it can be passed explicitly through the object graph (transport →
protocol → registry) instead of relying on globals.
"""

from __future__ import annotations

import os
from dataclasses import dataclass, field, replace
from pathlib import Path

#: Protocol revisions understood by this server, newest first.  The first entry
#: is the version the server prefers when the client requests something it does
#: not know about.
SUPPORTED_PROTOCOL_VERSIONS: tuple[str, ...] = (
    "2025-11-25",
    "2025-06-18",
    "2025-03-26",
)


@dataclass(frozen=True, slots=True)
class ServerConfig:
    """Immutable server configuration."""

    host: str = "127.0.0.1"
    port: int = 9093
    #: The single MCP endpoint path.  ``basics.md`` specifies ``/mpc``.
    path: str = "/mpc"
    #: Path of the PreToolUse hook endpoint the spawned CLI calls back into.
    hook_path: str = "/hooks/tool"

    #: HTTP header the client uses to carry the session id (a UUID).  This is
    #: the primary key for every operation and must be present on every request.
    session_header: str = "X-MCPC-SESSION-ID"
    #: If true, the session id must be a syntactically valid UUID.
    require_uuid_session: bool = True

    #: HTTP header carrying the comma-separated list of tool names that are
    #: active for the session.  The header is re-evaluated on every request; a
    #: request that omits it leaves the session's tool configuration untouched
    #: (this is what lets a spawned sub-agent inherit a pre-configured toolset
    #: without ever sending the header itself).
    tools_header: str = "X-MCPC-TOOLS"

    #: Central directory into which per-session communication logs are written.
    log_dir: Path = field(default=Path("logs"))

    #: Directory into which the CLI-session manager replicates the input/output
    #: streams of every managed CLI process (one NDJSON file per CLI session).
    cli_log_dir: Path = field(default=Path("logs/cli"))

    #: Node.js package environment used by the ``markdown`` tool (provides
    #: ``remark``, ``remark-behead`` and ``remark-frontmatter``).  Used as the
    #: working directory for the ``node`` process.
    markdown_env_dir: Path = field(
        default=Path("/home/user/xyan/xy.ai.workbench/claude-code/markdown/remark")
    )

    #: Base URL of the Exa remote MCP server and the API key used to reach it.
    exa_mcp_url: str = "https://mcp.exa.ai/mcp"
    exa_api_key: str | None = None

    #: Time-to-live, in seconds, after which an idle agent / CLI session becomes
    #: invalid.  Measured from the timestamp of the last message sent to the CLI.
    agent_session_ttl_seconds: float = 3600.0

    #: How long, in seconds, to wait for a CLI process to answer a single prompt
    #: before giving up.
    agent_response_timeout_seconds: float = 24 * 60 * 60.0

    #: Advertised server identity (returned in the ``initialize`` result).
    server_name: str = "xy.ai.mcpc"
    server_title: str = "MCP Controller"
    server_version: str = "0.1.0"
    instructions: str | None = (
        "MCP Controller reference server. Tools are managed in a central "
        "registry and enabled per session."
    )

    supported_protocol_versions: tuple[str, ...] = SUPPORTED_PROTOCOL_VERSIONS

    #: Extra origins allowed in addition to localhost.  ``None`` means only the
    #: usual loopback origins (and the configured host) are accepted.
    allowed_origins: tuple[str, ...] | None = None

    #: Maximum number of tools returned per ``tools/list`` page.
    tools_page_size: int = 100

    #: Reject request bodies larger than this many bytes.
    max_body_bytes: int = 4 * 1024 * 1024

    @property
    def preferred_protocol_version(self) -> str:
        return self.supported_protocol_versions[0]

    def with_overrides(self, **changes) -> "ServerConfig":
        """Return a copy of this config with *changes* applied."""
        return replace(self, **changes)

    @classmethod
    def from_env(cls, environ: dict[str, str] | None = None) -> "ServerConfig":
        """Build a config from ``MCPC_*`` environment variables."""
        env = os.environ if environ is None else environ
        kwargs: dict[str, object] = {}
        if "MCPC_HOST" in env:
            kwargs["host"] = env["MCPC_HOST"]
        if "MCPC_PORT" in env:
            kwargs["port"] = int(env["MCPC_PORT"])
        if "MCPC_PATH" in env:
            kwargs["path"] = env["MCPC_PATH"]
        if "MCPC_LOG_DIR" in env:
            kwargs["log_dir"] = Path(env["MCPC_LOG_DIR"])
        if "MCPC_CLI_LOG_DIR" in env:
            kwargs["cli_log_dir"] = Path(env["MCPC_CLI_LOG_DIR"])
        if "MCPC_MARKDOWN_ENV_DIR" in env:
            kwargs["markdown_env_dir"] = Path(env["MCPC_MARKDOWN_ENV_DIR"])
        if "MCPC_EXA_MCP_URL" in env:
            kwargs["exa_mcp_url"] = env["MCPC_EXA_MCP_URL"]
        if "MCPC_EXA_API_KEY" in env:
            kwargs["exa_api_key"] = env["MCPC_EXA_API_KEY"]
        elif "EXA_API_KEY" in env:
            kwargs["exa_api_key"] = env["EXA_API_KEY"]
        if "MCPC_SESSION_HEADER" in env:
            kwargs["session_header"] = env["MCPC_SESSION_HEADER"]
        if "MCPC_TOOLS_HEADER" in env:
            kwargs["tools_header"] = env["MCPC_TOOLS_HEADER"]
        if "MCPC_AGENT_SESSION_TTL" in env:
            kwargs["agent_session_ttl_seconds"] = float(env["MCPC_AGENT_SESSION_TTL"])
        return cls(**kwargs)  # type: ignore[arg-type]
