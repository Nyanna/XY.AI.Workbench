"""Skills – on-demand hints an agent can request, for a batch of items.

A *skill* is a small, easy-to-maintain unit combining metadata (name, title,
description, hint) with a real, named module-level function backing it as
``core`` (``FunctionRegistry`` requires actually existing functions with a
real signature/docstring). All skills are exposed through a single batching
tool so several skills can be fetched in one call.
"""
from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = [
    'Skill',
    'SKILLS',
    'MarkdownFormatSkill',
    'SkillsError',
    'SkillsItem',
    'SkillsResult',
    'SkillsItemError',
    'SkillsBatchResult',
    'get_skills',
    'SkillsTool',
    'register_skills',
    'markdown_format']

class Skill(ABC):
    """Metadata and backing function for one on-demand skill hint."""
    name: str
    title: str
    description: str
    hint: str

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
    return '* Use a line containing only `***` to insert a page break in PDF output.\n* Insert page breaks before top-level chapters (H1) at the start of each chapter.\n* Use `\\n---\\n` as a section separator before second-order chapters (H2) at the start of each chapter.\n* All files must end with an additional newline to prevent Markdown formatting errors on merge.\n* Use third-order headings and below only when necessary for navigation; use simple bold paragraph headings instead.\n* Chapter headings are numbered for H1–H3 only; lower-order headings do not contain numbering.\n* Use LaTeX (`$$`) for block mathematical expressions and inline LaTeX (`$`) for inline mathematical symbols, expressions, and formulas.'

class MarkdownFormatSkill(Skill):
    name = 'markdown_format'
    title = 'markdown-format'
    description = 'Preferred formatting rules for Pandoc-compatible Markdown documents. Load when formatting rules are requested or required.'
    hint = 'Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.'
    core = staticmethod(markdown_format)
'#: All declared skills. Append here to add a new one.'
SKILLS: list[Skill] = [MarkdownFormatSkill()]

class SkillsError(Exception):
    """Raised when a skill cannot be resolved."""

@dataclass(frozen=True)
class SkillsItem:
    """One skill to fetch.

    Attributes:
        name: Name of the skill to load.
    """
    name: str

@dataclass(frozen=True)
class SkillsResult:
    """Result of fetching a single skill, mirroring its input name for result association."""
    name: str
    instructions: str

@dataclass(frozen=True)
class SkillsItemError:
    """Error fetching a single skill, mirroring its input name for result association."""
    name: str
    error: str

@dataclass(frozen=True)
class SkillsBatchResult:
    """Result of :func:`get_skills`.

    Attributes:
        results: One :class:`SkillsResult` per successfully fetched skill.
        errors: One :class:`SkillsItemError` per skill that could not be resolved.
    """
    results: list[SkillsResult]
    errors: list[SkillsItemError]

def _skill_by_name(name: str) -> Skill:
    for skill in SKILLS:
        if skill.name == name:
            return skill
    raise SkillsError(f'Unknown skill: {name!r}')

def get_skills(items: list[SkillsItem]) -> SkillsBatchResult:
    """Fetch the instructions of one or more skills, for a batch of items.

    Args:
        items: Skills to fetch. Must be non-empty.

    Returns:
        SkillsBatchResult: one result per successfully fetched skill, one error per unknown skill.

    Raises:
        SkillsError: If items is empty.
    """
    if not items:
        raise SkillsError("'items' must be a non-empty list.")
    results: list[SkillsResult] = []
    errors: list[SkillsItemError] = []
    for item in items:
        try:
            skill = _skill_by_name(item.name)
            results.append(SkillsResult(name=item.name, instructions=skill.core()))
        except SkillsError as exc:
            errors.append(SkillsItemError(name=item.name, error=str(exc)))
    return SkillsBatchResult(results=results, errors=errors)

class SkillsTool(ToolDefinition):
    name = 'skills'
    title = 'Load skills'
    input_schema = {
        'type': 'object',
        'properties': {
            'items': {
                'type': 'array',
                'minItems': 1,
                'items': {
                    'type': 'object',
                    'additionalProperties': False,
                    'properties': {
                        'name': {
                            'type': 'string',
                            'enum': [
                                skill.name for skill in SKILLS],
                            'description': 'Name of the skill to load.'}},
                    'required': ['name']},
                'description': 'Skills to load.'}},
        'required': ['items']}
    output_schema = {
        'type': 'object', 'properties': {
            'results': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'name': {
                            'type': 'string'}, 'instructions': {
                                'type': 'string'}}, 'required': [
                                    'name', 'instructions']}}, 'errors': {
                                        'type': 'array', 'items': {
                                            'type': 'object', 'properties': {
                                                'name': {
                                                    'type': 'string'}, 'error': {
                                                        'type': 'string'}}, 'required': [
                                                            'name', 'error']}}}, 'required': [
                                                                'results', 'errors']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def __init__(self) -> None:
        catalog = '\n'.join((f'- {skill.name}: {skill.description}' for skill in SKILLS))
        self.description = f'Load one or more on-demand skill hints, for a batch of items. Available skills:\n{catalog}'

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`get_skills`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        raw_items = args.get('items') or []
        if not raw_items:
            return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
        items = [SkillsItem(name=it['name']) for it in raw_items]
        batch = get_skills(items)
        results = [{'name': r.name, 'instructions': r.instructions} for r in batch.results]
        errors = [{'name': e.name, 'error': e.error} for e in batch.errors]
        is_error = bool(batch.errors) and (not batch.results)
        return ToolResult(
            structured_content={
                'results': results,
                'errors': errors},
            is_error=False,
            auto_approve=not is_error)

def register_skills(registry: ToolRegistry, environment: AppEnvironment) -> None:
    """Register the batching skills tool and each skill's backing function."""
    registry.register(SkillsTool())
    for skill in SKILLS:
        environment.functions.register(skill.core)
    environment.functions.register(get_skills)