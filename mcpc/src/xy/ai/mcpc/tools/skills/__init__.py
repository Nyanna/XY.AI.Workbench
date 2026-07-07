"""Skills – on-demand hints an agent can request.

A *skill* is a small, easy-to-maintain unit bundling a name, a description, a
hint and a set of instructions.  Every declared skill is automatically exposed
as a tool:

* the tool's **description** is the skill's *description* plus its *hint*,
* the tool takes **no inputs**, and
* calling it simply **returns the skill's instructions**.

To add a skill, append a :class:`Skill` to :data:`SKILLS`.
"""

from __future__ import annotations

from dataclasses import dataclass

from ...registry import ToolContext, ToolRegistry, ToolResult


@dataclass(frozen=True, slots=True)
class Skill:
    """A maintainable hint the agent can request on demand."""

    name: str
    description: str
    hint: str
    instructions: str

    @property
    def tool_description(self) -> str:
        """Description + hint, as advertised to the agent."""
        return f"{self.description}\n\n{self.hint}"


def register_skill(registry: ToolRegistry, skill: Skill) -> None:
    """Register a single *skill* as an input-less tool returning its instructions."""

    @registry.tool(
        skill.name,
        title=skill.name,
        description=skill.tool_description,
        input_schema={"type": "object", "properties": {}},
        output_schema={
            "type": "object",
            "properties": {
                "instructions": {"type": "string"},
            },
            "required": ["instructions"],
        },
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def skill_tool(ctx: ToolContext, _skill: Skill = skill) -> ToolResult:
        return ToolResult(structured_content={"instructions": _skill.instructions})


#: All declared skills.  Append here to add a new one.
SKILLS: list[Skill] = [
    Skill(
        name="markdown-format",
        description=(
            "Preferred formatting rules for Pandoc-compatible Markdown "
            "documents. Load when formatting rules are requested or required."
        ),
        hint=(
            "Apply proactively whenever creating, editing, or reviewing "
            "Markdown documents — even when formatting is not explicitly "
            "mentioned."
        ),
        instructions=(
            "* Use a line containing only `***` to insert a page break in PDF output.\n"
            "* Insert page breaks before top-level chapters (H1) at the start of each chapter.\n"
            "* Use `\\n---\\n` as a section separator before second-order chapters (H2) at the start of each chapter.\n"
            "* All files must end with an additional newline to prevent Markdown formatting errors on merge.\n"
            "* Use third-order headings and below only when necessary for navigation; use simple bold paragraph headings instead.\n"
            "* Chapter headings are numbered for H1–H3 only; lower-order headings do not contain numbering.\n"
            "* Use LaTeX (`$$`) for block mathematical expressions and inline LaTeX (`$`) for inline mathematical symbols, expressions, and formulas."
        ),
    ),
]


def register_skills(registry: ToolRegistry, skills: "list[Skill] | None" = None) -> None:
    """Register every skill in *skills* (defaults to :data:`SKILLS`) as a tool."""
    for skill in SKILLS if skills is None else skills:
        register_skill(registry, skill)


__all__ = ["Skill", "SKILLS", "register_skill", "register_skills"]
