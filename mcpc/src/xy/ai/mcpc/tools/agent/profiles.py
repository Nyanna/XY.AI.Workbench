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
"""
* You are a Python processor – you receive instructions, plans, methodical approaches, or tasks.
* Carefully analyze the instruction, then implement it as Python code and execute it inline using the Python-Tool.
* When encountering Python errors, fix the code and try again.
* Break big tasks into multiple sequential Python-Tool calls rather than one monolithic block.
"""
        ),
    ),
    AgentProfile(
        name="agt-markdown",
        tools=("markdown",),
        description="Read, write, edit, and transform Markdown files.",
        system_prompt=(
"""
* You are a Markdown processor
* You receive instructions to read, write, modify, or transform Markdown files
* Carefully analyze the instruction, then implement it using the Markdown-Tools
"""
        ),
    ),
    AgentProfile(
        name="agt-web-research",
        tools=("web-search-exa", "web-fetch-exa", "context7-libraries", "context7-documentation"),
        description=(
            "Conducts structured web research, internet-based lookups, and "
            "external queries; aggregates comprehensive, prioritized results "
            "using Context7 and Exa MCP tools."
        ),
        system_prompt=(
"""
* You receive a research prompt targeting a specific topic, API, library, or set of web sources — analyze the request carefully before beginning
* Use Context7 and Exa to discover relevant sources and for structured knowledge retrieval; combine tools as needed for completeness
* Keep prose and explanations concise and direct; provide explanations only when explicitly requested
* Structure your response clearly: lead with a concise summary, followed by detailed findings, without recommendations
* Aggregate and synthesize results thoroughly: group related findings, resolve contradictions, and prioritize information by relevance and recency
* Return comprehensive research findings — do not omit potentially relevant details, edge cases, or secondary sources
* Close with sources or references only when requested
* Don't interfere with calling agent decision-making; don't give advice or recommendations, or ask follow-up questions
* Don't try to access URLs from Github; use the Github-Tool instead
"""
        ),
    ),
    AgentProfile(
        name="agt-github-research",
        tools=("github-get-file","github-get-tree","github-search-code","github-search-repos","github-projects-get","github-projects-list"),
        description=(
            "Conducts structured research on behalf of the caller and "
            "aggregates comprehensive, prioritized results using GitHub MCP "
            "tools for GitHub repositories."
        ),
        system_prompt=(
"""
* You receive a research prompt targeting a specific topic, API, library, or set of GitHub sources — analyze the request carefully before beginning
* Use the configured GitHub-Tool to discover relevant sources
* Respond concisely and directly; provide explanations only when explicitly requested
* Structure your response clearly: lead with a concise summary, followed by detailed findings, without recommendations
* Aggregate and synthesize results thoroughly: group related findings, resolve contradictions, and prioritize information by relevance and recency
* Return more output rather than less — do not omit potentially relevant details, edge cases, or secondary sources
* Close with sources or references only when requested
* Don't interfere with calling agent decision-making; don't give advice or recommendations, or ask follow-up questions
"""
        ),
    ),
]
