"""The agent tool and its profile-bound wrapper tools.

The **agent tool** spawns a sub-agent to offload complex or context-heavy work.
From the caller's point of view an agent call is indistinguishable from any other
tool call.  Each invocation mints a fresh session (with a pre-configured toolset)
and drives a CLI process through the :class:`CliSessionManager`; ``resume``
reattaches to a previously spawned sub-agent.

The **wrapper tools** (``agt-python``, ``agt-markdown``, …) bind a single profile
to its description and system prompt and delegate everything else to the agent
tool.  The agent tool itself is rarely called directly.
"""

from __future__ import annotations

import uuid
from typing import Any

from ...cli import CliParameters, CliSessionError, Effort, Model
from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from ...session import AgentSubSession
from .profiles import DEFAULT_PROFILES, AgentProfile, ProfileRegistry

_MODELS = tuple(m.value for m in Model)
_EFFORTS = tuple(e.value for e in Effort)


def _base_properties(*, include_system_prompt: bool) -> dict[str, Any]:
    props: dict[str, Any] = {
        "prompt": {
            "type": "string",
            "description": "The task or prompt to hand to the agent.",
        },
        "model": {
            "type": "string",
            "enum": list(_MODELS),
            "description": "Model the agent should run on.",
        },
        "effort": {
            "type": "string",
            "enum": list(_EFFORTS),
            "description": "Reasoning effort level.",
        },
        "resume": {
            "type": "string",
            "description": (
                "Optional session UUID of a previous agent call to continue. "
                "When set, the other fields are ignored."
            ),
        },
    }
    if include_system_prompt:
        props["system_prompt"] = {
            "type": "string",
            "description": "System prompt used to initialise the agent.",
        }
    return props


def _error(message: str) -> ToolResult:
    return ToolResult(content=[text_content(message)], is_error=True)


def _run_agent(
    ctx: ToolContext,
    *,
    profile: AgentProfile | None,
    system_prompt_override: str | None,
) -> ToolResult:
    """Shared implementation behind both the agent tool and its wrappers."""
    services = ctx.services
    if services is None:  # pragma: no cover - misconfiguration guard
        return _error("Agent tool is not wired to application services.")

    args = ctx.arguments
    prompt = args.get("prompt")
    resume = args.get("resume")

    if resume is not None:
        return _resume_agent(ctx, str(resume), prompt)

    if not isinstance(prompt, str) or not prompt:
        return _error('"prompt" is required.')

    model = args.get("model", Model.SONNET.value)
    if model not in _MODELS:
        return _error(f"Invalid model '{model}'. Expected one of: {', '.join(_MODELS)}.")

    effort = args.get("effort", Effort.MEDIUM.value)
    if effort not in _EFFORTS:
        return _error(f"Invalid effort '{effort}'. Expected one of: {', '.join(_EFFORTS)}.")

    # Resolve the profile (explicit for the raw agent tool, fixed for wrappers).
    if profile is None:
        profile_name = args.get("profile")
        if profile_name is not None:
            profile = services.profiles.get(str(profile_name))
            if profile is None:
                return _error(f"Unknown profile: {profile_name}")

    tools = list(profile.tools) if profile is not None else []
    system_prompt = (
        system_prompt_override
        or args.get("system_prompt")
        or (profile.system_prompt if profile is not None else "")
    )
    profile_name = profile.name if profile is not None else None

    # A fresh UUID identifies both the pre-created MCPC session (which carries the
    # sub-agent's toolset) and the CLI session (--session-id).  The sub-agent's
    # CLI connects back with this id and never sends X-MCPC-TOOLS itself.
    sub_id = str(uuid.uuid4())
    cc_profile = ctx.session.cc_profile
    services.sessions.precreate(sub_id, enabled_tools=set(tools), cc_profile = cc_profile)

    params = CliParameters(
        config=services.config,
        model=model,
        system_prompt=system_prompt,
        mcpc_session_id=sub_id,
        effort=effort,
        cc_profile=cc_profile,
    )

    try:
        cli = services.cli_manager.request(parameters=params, session_id=sub_id)
        result = cli.prompt(prompt)
    except CliSessionError as exc:
        return _error(f"Agent failed: {exc}")

    # Record the spawned sub-agent on the *calling* session, keyed by CLI id.
    ctx.session.register_agent_session(sub_id, model=model, profile=profile_name)

    return _result(result.text, sub_id, is_error=result.is_error)


def _resume_agent(ctx: ToolContext, resume_id: str, prompt: Any) -> ToolResult:
    services = ctx.services
    assert services is not None

    record: AgentSubSession | None = ctx.session.get_agent_session(resume_id)
    ttl = services.config.agent_session_ttl_seconds
    if record is None or not record.is_valid(ttl):
        return _error(
            f"Cannot resume agent session '{resume_id}': not found or expired."
        )

    if not isinstance(prompt, str) or not prompt:
        return _error('"prompt" is required to resume an agent session.')

    try:
        cli = services.cli_manager.request(resume=resume_id)
        result = cli.prompt(prompt)
    except CliSessionError as exc:
        return _error(f"Cannot resume agent session '{resume_id}': {exc}")

    record.touch()
    return _result(result.text, resume_id, is_error=result.is_error)


def _result(text: str, session_id: str, *, is_error: bool) -> ToolResult:
    if is_error:
        return ToolResult(content=[text_content(text)], is_error=True)
    return ToolResult(
        structured_content={"response": text, "session_id": session_id},
    )


def register_agent_tool(registry: ToolRegistry) -> None:
    """Register the raw agent tool (rarely called directly)."""

    @registry.tool(
        "agent",
        title="Run sub-agent",
        description=(
            "Delegate a task to a sub-agent. Sub-agents offload complex or "
            "context-heavy work to keep the main context lean or to use faster "
            "or more specialised models. Returns the agent's answer."
        ),
        input_schema={
            "type": "object",
            "properties": _base_properties(include_system_prompt=True),
            "required": ["prompt"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "response": {"type": "string"},
                "session_id": {"type": "string"},
            },
            "required": ["response", "session_id"],
        },
        annotations={"readOnlyHint": False, "openWorldHint": True},
    )
    def agent(ctx: ToolContext) -> ToolResult:
        return _run_agent(ctx, profile=None, system_prompt_override=None)


def register_wrapper_tools(
    registry: ToolRegistry, profiles: "ProfileRegistry | None" = None
) -> None:
    """Register one wrapper tool per agent profile."""
    profiles = profiles or ProfileRegistry(DEFAULT_PROFILES)
    for profile in profiles:
        _register_wrapper(registry, profile)


def _register_wrapper(registry: ToolRegistry, profile: AgentProfile) -> None:
    @registry.tool(
        profile.name,
        title=profile.name,
        # The wrapper surfaces the profile's task description.
        description=profile.description,
        input_schema={
            "type": "object",
            "properties": _base_properties(include_system_prompt=False),
            "required": ["prompt"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "response": {"type": "string"},
                "session_id": {"type": "string"},
            },
            "required": ["response", "session_id"],
        },
        annotations={"readOnlyHint": False, "openWorldHint": True},
    )
    def wrapper(ctx: ToolContext, _profile: AgentProfile = profile) -> ToolResult:
        # Profile and system prompt are pre-filled; everything else is delegated.
        return _run_agent(
            ctx, profile=_profile, system_prompt_override=_profile.system_prompt
        )


def register_agent_tools(
    registry: ToolRegistry, profiles: "ProfileRegistry | None" = None
) -> None:
    """Register the agent tool together with all profile wrapper tools."""
    register_agent_tool(registry)
    register_wrapper_tools(registry, profiles)


__all__ = [
    "AgentProfile",
    "ProfileRegistry",
    "register_agent_tool",
    "register_agent_tools",
    "register_wrapper_tools",
]
