"""Agent profiles: named presets binding a toolset, a description and a prompt.

A profile is an alias for a pre-configured toolset.  Each wrapper tool is bound
to exactly one profile; the wrapper advertises the profile's *description* and
pre-fills the profile's *system prompt* when delegating to the agent tool.  Not
every referenced tool is necessarily implemented yet.
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class AgentProfile:
    """A named preset for spawning a specialised sub-agent."""

    name: str
    #: Tool names activated for the sub-agent's session.
    tools: tuple[str, ...]
    #: Task description surfaced as the wrapper tool's description.
    description: str
    #: System prompt used to initialise the sub-agent.
    system_prompt: str


class ProfileRegistry:
    """Lookup table of the known agent profiles."""

    def __init__(self, profiles: "list[AgentProfile] | None" = None) -> None:
        self._profiles: dict[str, AgentProfile] = {}
        for profile in profiles or DEFAULT_PROFILES:
            self.register(profile)

    def register(self, profile: AgentProfile) -> AgentProfile:
        if profile.name in self._profiles:
            raise ValueError(f"Profile already registered: {profile.name}")
        self._profiles[profile.name] = profile
        return profile

    def get(self, name: str) -> AgentProfile | None:
        return self._profiles.get(name)

    def names(self) -> list[str]:
        return list(self._profiles)

    def __iter__(self):
        return iter(self._profiles.values())

    def __contains__(self, name: object) -> bool:
        return name in self._profiles


#: The four profiles required by the specification.  Each profile keeps its
#: tools, description and system prompt together in one block so a profile can
#: be edited or copied as a single self-contained unit.
DEFAULT_PROFILES: list[AgentProfile] = [
    AgentProfile(
        name="agt-python",
        tools=("python",),
        description=(
            "Python code tool that translates instructions, plans, and tasks "
            "into Python code, executes them inline, and handles errors "
            "autonomously."
        ),
        system_prompt=(
            "You are a Python coding agent. Translate the caller's "
            "instructions, plans and tasks into Python code, execute it "
            "inline, and handle errors autonomously until the task is "
            "complete. Report the outcome concisely."
        ),
    ),
    AgentProfile(
        name="agt-markdown",
        tools=("markdown",),
        description="Read, write, edit, and transform Markdown files.",
        system_prompt=(
            "You are a Markdown authoring agent. Read, write, edit and "
            "transform Markdown files exactly as requested, preserving "
            "structure and formatting."
        ),
    ),
    AgentProfile(
        name="agt-web-research",
        tools=("exa", "context7"),
        description=(
            "Conducts structured web research, internet-based lookups, and "
            "external queries; aggregates comprehensive, prioritized results "
            "using Context7 and Exa MCP tools."
        ),
        system_prompt=(
            "You are a web research agent. Conduct structured web research and "
            "external lookups using the Context7 and Exa tools, then aggregate "
            "comprehensive, prioritised results for the caller."
        ),
    ),
    AgentProfile(
        name="agt-github-research",
        tools=("github",),
        description=(
            "Conducts structured research on behalf of the caller and "
            "aggregates comprehensive, prioritized results using GitHub MCP "
            "tools for GitHub repositories."
        ),
        system_prompt=(
            "You are a GitHub research agent. Conduct structured research on "
            "GitHub repositories using the GitHub tools and aggregate "
            "comprehensive, prioritised results for the caller."
        ),
    ),
]
