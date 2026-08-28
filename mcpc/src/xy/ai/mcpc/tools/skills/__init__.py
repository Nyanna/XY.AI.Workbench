"""Skills – on-demand hints an agent can request.

A *skill* is a small, easy-to-maintain unit combining an MCP tool schema
(name, title, description, hint) with a real, named module-level function
that backs it as ``core`` (``FunctionRegistry`` requires actually existing
functions with a real signature/docstring.
"""

from __future__ import annotations

from abc import ABC, abstractmethod

from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["Skill", "SKILLS", "MarkdownFormatSkill", "register_skills", "markdown_format"]


class Skill(ToolDefinition, ABC):
    """Base class for skills: input-less tools returning maintainable hints."""

    hint: str
    input_schema = {"type": "object", "properties": {}}
    output_schema = {
        "type": "object",
        "properties": {
            "instructions": {"type": "string"},
        },
        "required": ["instructions"],
    }
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def __init__(self) -> None:
        self.description = f"{self.description}\n\n{self.hint}"

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Call ``core`` and wrap its result into the MCP output schema."""
        return ToolResult(structured_content={"instructions": self.core()})

    @staticmethod
    @abstractmethod
    def core() -> str:
        """Real function backing this skill; also registered in the ``FunctionRegistry``."""


def markdown_format() -> str:
    """Preferred formatting rules for Pandoc-compatible Markdown documents.

    Load when formatting rules are requested or required; apply proactively
    whenever creating, editing, or reviewing Markdown documents — even when
    formatting is not explicitly mentioned.

    Returns:
        The formatting instructions text.
    """
    return (
        "* Use a line containing only `***` to insert a page break in PDF output.\n"
        "* Insert page breaks before top-level chapters (H1) at the start of each chapter.\n"
        "* Use `\\n---\\n` as a section separator before second-order chapters (H2) at the start of each chapter.\n"
        "* All files must end with an additional newline to prevent Markdown formatting errors on merge.\n"
        "* Use third-order headings and below only when necessary for navigation; use simple bold paragraph headings instead.\n"
        "* Chapter headings are numbered for H1–H3 only; lower-order headings do not contain numbering.\n"
        "* Use LaTeX (`$$`) for block mathematical expressions and inline LaTeX (`$`) for inline mathematical symbols, expressions, and formulas."
    )


class MarkdownFormatSkill(Skill):
    name = "markdown_format"
    title = "markdown-format"
    description = (
        "Preferred formatting rules for Pandoc-compatible Markdown "
        "documents. Load when formatting rules are requested or required."
    )
    hint = (
        "Apply proactively whenever creating, editing, or reviewing "
        "Markdown documents — even when formatting is not explicitly "
        "mentioned."
    )
    core = staticmethod(markdown_format)


#: All declared skills.  Append here to add a new one.
SKILLS: list[Skill] = [
    MarkdownFormatSkill(),
]


def register_skills(
    registry: ToolRegistry,
    environment: AppEnvironment,
) -> None:
    """Register every skill in *skills* (defaults to :data:`SKILLS`) in both registries."""
    for skill in SKILLS:
        registry.register(skill)
        environment.functions.register(skill.core, id=skill.name)
