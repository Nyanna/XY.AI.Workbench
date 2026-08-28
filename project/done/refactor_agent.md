Die Tools müssen vom Decorator auf die ABC Registrierung umgestellt werden, Beispiel: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py`.
Die registrierten Functions dürfen keine Locals sein, sondern müssen im Module Root direkt ansprechbar sein.

- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/agent`
SystemInit:  id=6054972b-9978-4cd4-aee1-d4835da7b1a9, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: b66c-1
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py
  reason: "Beispiel für ABC-Registrierung von Tools ansehen, um das agent-Tool umzustellen."
```
/answer b66c-1 allow
Control Request:
```yaml
id: b66c-2
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/agent/__init__.py
  reason: "Aktuellen Stand des agent-Tools lesen, um es auf ABC-Registrierung umzustellen."
```
/answer b66c-2 allow
ReasoningToken: 21
Control Request:
```yaml
id: b66c-3
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/registry.py
  reason: "ToolDefinition ABC und ToolRegistry API verstehen, um agent-Tool umzustellen."
```
/answer b66c-3 deny nicht notwendig
Control Request:
```yaml
id: b66c-4
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
  reason: "ToolContext und AppEnvironment verstehen, insbesondere wie environment übergeben wird."
```
/answer b66c-4 deny nicht notwendig
ReasoningToken: 116
Control Request:
```yaml
id: b66c-5
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py
  reason: "Prüfen, wie register_agent_tools aufgerufen wird, um die öffentliche Signatur beizubehalten."
```
/answer b66c-5 deny nicht notwendig
Control Request:
```yaml
id: b66c-6
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/agent/profiles.py
  reason: "AgentProfile und ProfileRegistry Definition verstehen, um die ABC-Registrierung korrekt umzusetzen."
```
/answer b66c-6 allow
ReasoningToken: 1664
Control Request:
```yaml
id: b66c-7
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/registry.py
  reason: "Muss die exakte ABC-Definition von ToolDefinition (Attribute, evtl. __slots__, abstrakte Methoden) kennen, um die agent-Tools korrekt und ohne Laufzeitfehler auf Klassen umzustellen, die pro Instanz Profil-spezifische Attribute (name, description, profile) benötigen."
```
/answer b66c-7 deny nicht notwendig
ReasoningToken: 2072
Control Request:
```yaml
id: b66c-8
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/agent/__init__.py
  mode: replace
  reason: "Umstellung des agent-Tools von Decorator-basierter Registrierung auf ABC-basierte ToolDefinition-Klassen (AgentTool, AgentProfileTool) auf Modulebene, gemäss dem Muster aus replace_block/__init__.py."
  content: |
    """The agent tool and its profile-bound wrapper tools.

    The **agent tool** spawns a sub-agent to offload complex or context-heavy work.
    From the caller's point of view an agent call is indistinguishable from any other
    tool call.

    The **wrapper tools** (``agt_python``, ``agt_markdown``, …) bind a single profile
    to its description and system prompt and delegate everything else to the agent
    tool.  The agent tool itself is rarely called directly.
    """

    from __future__ import annotations

    import uuid
    from typing import Any

    from xy.ai.mcpc.cli import CliParameters, CliSessionError, Effort, Model
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.server.session import AgentSubSession
    from xy.ai.mcpc.tools.agent.profiles import DEFAULT_PROFILES, AgentProfile, ProfileRegistry

    __all__ = [
        "AgentProfile",
        "ProfileRegistry",
        "AgentTool",
        "AgentProfileTool",
        "register_agent_tools",
    ]

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
        environment: AppEnvironment,
        profile: AgentProfile | None,
        system_prompt_override: str | None,
    ) -> ToolResult:
        """Shared implementation behind both the agent tool and its wrappers."""
        services = environment

        args = ctx.arguments
        prompt = args.get("prompt")
        resume = args.get("resume")

        if resume is not None:
            return _resume_agent(ctx, environment, str(resume), prompt)

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
        services.sessions.precreate(sub_id, enabled_tools=set(tools), cc_profile=cc_profile)

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


    def _resume_agent(
        ctx: ToolContext, environment: AppEnvironment, resume_id: str, prompt: Any
    ) -> ToolResult:
        services = environment

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


    class AgentTool(ToolDefinition):
        """Raw agent tool (rarely called directly); profile is selected via the ``profile`` argument."""

        name = "agent"
        title = "Run sub-agent"
        description = (
            "Delegate a task to a sub-agent. Sub-agents offload complex or "
            "context-heavy work to keep the main context lean or to use faster "
            "or more specialised models. Returns the agent's answer."
        )
        input_schema = {
            "type": "object",
            "properties": _base_properties(include_system_prompt=True),
            "required": ["prompt"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "response": {"type": "string"},
                "session_id": {"type": "string"},
            },
            "required": ["response", "session_id"],
        }
        annotations = {"readOnlyHint": False, "openWorldHint": True}

        def __init__(self, environment: AppEnvironment) -> None:
            self.environment = environment

        def handle(self, ctx: ToolContext) -> ToolResult:
            if self.environment is None:  # pragma: no cover - misconfiguration guard
                return _error("Agent tool is not wired to application services.")
            return _run_agent(ctx, environment=self.environment, profile=None, system_prompt_override=None)


    class AgentProfileTool(ToolDefinition):
        """Wrapper tool binding a single :class:`AgentProfile` to the agent tool."""

        output_schema = {
            "type": "object",
            "properties": {
                "response": {"type": "string"},
                "session_id": {"type": "string"},
            },
            "required": ["response", "session_id"],
        }
        annotations = {"readOnlyHint": False, "openWorldHint": True}

        def __init__(self, environment: AppEnvironment, profile: AgentProfile) -> None:
            self.environment = environment
            self.profile = profile
            self.name = profile.name
            self.title = profile.name
            # The wrapper surfaces the profile's task description.
            self.description = profile.description
            self.input_schema = {
                "type": "object",
                "properties": _base_properties(include_system_prompt=False),
                "required": ["prompt"],
            }

        def handle(self, ctx: ToolContext) -> ToolResult:
            if self.environment is None:  # pragma: no cover - misconfiguration guard
                return _error("Agent tool is not wired to application services.")
            # Profile and system prompt are pre-filled; everything else is delegated.
            return _run_agent(
                ctx,
                environment=self.environment,
                profile=self.profile,
                system_prompt_override=self.profile.system_prompt,
            )


    def register_agent_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
        """Register the agent tool together with all profile wrapper tools."""
        registry.register(AgentTool(environment))
        for profile in ProfileRegistry(DEFAULT_PROFILES):
            registry.register(AgentProfileTool(environment, profile))
```
/answer b66c-8 allow
Cannot read STDOUT: the CLI process is not started (uuid=6054972b-9978-4cd4-aee1-d4835da7b1a9, processAlive=false). A prompt must be sent (writeLine) before reading.