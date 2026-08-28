Refaktoriere `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py`.

- Es soll nicht über den Decorator registriert werden, sondern die ABC Variante, Beispiel: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py`
- Die Priorität muss auf die Core Function geändert werden und liegt nicht auf dem MCP Schema
- Mit Prioriät ist gemeint das die Core Function einen Teil des Schemas zurückgibt. Der primäre Aspekt liegt auf der Kernmethode. Die Gibt den Skill als String zurück und der Handler kann dieses in ein MCP Result verpacken.
Control Request:
```yaml
id: 7474-9
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/convenience.py
  reason: "Need BulkCrudTool class definition (fields, handle signature, priority attribute) from convenience.py, which layers.py already imports, to correctly rewrite layers.py using the ABC pattern."
```
/answer 7474-9 allow
SystemInit:  id=f3bdcdad-7c01-47fb-8053-e03e7699615d, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7474-a
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  reason: Aktuellen Inhalt der zu refaktorierenden Datei lesen
```
/answer 7474-a allow
Control Request:
```yaml
id: 7474-b
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py
  reason: Zielmuster (ABC-basierte Tool-Registrierung) als Vorlage lesen
```
/answer 7474-b allow
ReasoningToken: 22
Control Request:
```yaml
id: 7474-c
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/registry.py
  reason: ToolDefinition-Basisklasse und ToolRegistry API verstehen für ABC-Refactoring
```
/answer 7474-c deny Nicht notwendig
Control Request:
```yaml
id: 7474-d
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/function_registry.py
  reason: FunctionRegistry API verstehen für register-Aufruf
```
/answer 7474-d deny Nicht notwendig
ReasoningToken: 117
Control Request:
```yaml
id: 7474-e
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
  reason: "Prüfen, ob Skill.instructions oder register_skills/Skill von anderen Modulen referenziert wird, um Breaking Changes beim Refactoring zu vermeiden (AppEnvironment-Definition)"
```
/answer 7474-e deny Nicht notwendig
ReasoningToken: 2930
Control Request:
```yaml
id: 7474-f
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  mode: replace
  reason: "Refaktoriere die Skills-Registrierung von der Decorator-Variante auf die ABC-basierte ToolDefinition-Variante (Vorbild replace_block) und verlagert die Priorität auf die Core-Funktion, deren String-Ergebnis der Handler lediglich ins MCP-Schema verpackt."
  content: |
    """Skills – on-demand hints an agent can request.

    A *skill* is a small, easy-to-maintain unit bundling a name, a description, a
    hint and a core function. Every declared skill is automatically exposed as a
    tool: the tool's **description** is the skill's *description* plus its
    *hint*, the tool takes **no inputs**, and calling it wraps the skill's
    **core function** result into the MCP output schema.

    To add a skill, append a :class:`Skill` to :data:`SKILLS`, backed by a real,
    named module-level function (no parameters, returning the instructions text)
    — see :func:`markdown_format` for the pattern. This function is the skill's
    single source of truth; the tool merely delegates to it.
    """

    from __future__ import annotations

    from dataclasses import dataclass
    from typing import Callable

    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ["Skill", "SKILLS", "SkillTool", "register_skill", "register_skills", "markdown_format"]


    @dataclass(frozen=True, slots=True)
    class Skill:
        """A maintainable hint the agent can request on demand."""

        name: str
        description: str
        hint: str
        #: Real function backing this skill in the ``FunctionRegistry``; its
        #: return value is the skill's instructions.
        core: Callable[[], str]

        @property
        def tool_name(self) -> str:
            """MCP tool name: hyphens are not valid in the core function's name."""
            return self.name.replace("-", "_")

        @property
        def tool_description(self) -> str:
            """Description + hint, as advertised to the agent."""
            return f"{self.description}\n\n{self.hint}"


    class SkillTool(ToolDefinition):
        """Exposes a single :class:`Skill` as an input-less tool.

        The core function is the primary source: the tool schema merely wraps
        its string result into the MCP output format.
        """

        input_schema = {"type": "object", "properties": {}}
        output_schema = {
            "type": "object",
            "properties": {
                "instructions": {"type": "string"},
            },
            "required": ["instructions"],
        }
        annotations = {"readOnlyHint": True, "openWorldHint": False}

        def __init__(self, skill: Skill) -> None:
            self._skill = skill
            self.name = skill.tool_name
            self.title = skill.name
            self.description = skill.tool_description

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Call the skill's core function and wrap its result into the MCP schema."""
            return ToolResult(structured_content={"instructions": self._skill.core()})


    def register_skill(registry: ToolRegistry, skill: Skill, functions: "FunctionRegistry | None" = None) -> None:
        """Register a single *skill* as a tool, and its core function in *functions* if given."""
        registry.register(SkillTool(skill))
        if functions is not None:
            functions.register(skill.core, id=skill.tool_name)


    _MARKDOWN_FORMAT_INSTRUCTIONS = (
        "* Use a line containing only `***` to insert a page break in PDF output.\n"
        "* Insert page breaks before top-level chapters (H1) at the start of each chapter.\n"
        "* Use `\\n---\\n` as a section separator before second-order chapters (H2) at the start of each chapter.\n"
        "* All files must end with an additional newline to prevent Markdown formatting errors on merge.\n"
        "* Use third-order headings and below only when necessary for navigation; use simple bold paragraph headings instead.\n"
        "* Chapter headings are numbered for H1–H3 only; lower-order headings do not contain numbering.\n"
        "* Use LaTeX (`$$`) for block mathematical expressions and inline LaTeX (`$`) for inline mathematical symbols, expressions, and formulas."
    )


    def markdown_format() -> str:
        """Preferred formatting rules for Pandoc-compatible Markdown documents.

        Load when formatting rules are requested or required; apply proactively
        whenever creating, editing, or reviewing Markdown documents — even when
        formatting is not explicitly mentioned.

        Returns:
            The formatting instructions text.
        """
        return _MARKDOWN_FORMAT_INSTRUCTIONS


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
            core=markdown_format,
        ),
    ]


    def register_skills(
        registry: ToolRegistry,
        environment: "AppEnvironment | None" = None,
        skills: "list[Skill] | None" = None,
    ) -> None:
        """Register every skill in *skills* (defaults to :data:`SKILLS`) as a tool."""
        functions = environment.functions if environment is not None else None
        for skill in SKILLS if skills is None else skills:
            register_skill(registry, skill, functions)
```
/answer 7474-f allow
ReasoningToken: 354
Die Datei wurde erfolgreich refaktoriert. Zusammenfassung der Änderungen:

1. **ABC statt Decorator**: Statt `registry.tool(...)` als Decorator gibt es jetzt `SkillTool(ToolDefinition)`, analog zu `ReplaceBlockTool` in `replace_block/__init__.py`. Die Registrierung erfolgt via `registry.register(SkillTool(skill))`.

2. **Priorität auf der Core-Funktion**: `Skill.core` ist nun ein verpflichtendes Feld (kein `str`-Feld `instructions` mehr, das parallel gepflegt werden musste). Die Kernfunktion (z. B. `markdown_format()`) ist die einzige Quelle der Wahrheit und gibt den Skill-Inhalt als String zurück. `SkillTool.handle()` ruft lediglich `self._skill.core()` auf und verpackt das Ergebnis in `ToolResult(structured_content={"instructions": ...})` – das MCP-Schema ist nur noch Verpackung, keine eigene Quelle mehr.

3. **Redundanz entfernt**: Da `instructions` jetzt immer aus `core()` kommt, entfällt die doppelte Pflege von Text in `Skill.instructions` und in der Modulfunktion.
die register_skills function soll den skill direkt in beiden Registries registrieren. Das muss nicht durch eine extra Methode laufen. Die Function Registry kann direkt die echte/native Python Funktion bekommen.
SystemInit:  id=f3bdcdad-7c01-47fb-8053-e03e7699615d, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7474-10
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  reason: "Aktuellen Stand der Datei erneut lesen, um register_skill in register_skills zu inlinen"
```
/answer 7474-10 allow
ReasoningToken: 87
Control Request:
```yaml
id: 7474-11
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  mode: replace
  reason: "register_skill entfernen und dessen Logik direkt in register_skills inlinen, wie vom Nutzer gefordert."
  content: |
    """Skills – on-demand hints an agent can request.

    A *skill* is a small, easy-to-maintain unit bundling a name, a description, a
    hint and a core function. Every declared skill is automatically exposed as a
    tool: the tool's **description** is the skill's *description* plus its
    *hint*, the tool takes **no inputs**, and calling it wraps the skill's
    **core function** result into the MCP output schema.

    To add a skill, append a :class:`Skill` to :data:`SKILLS`, backed by a real,
    named module-level function (no parameters, returning the instructions text)
    — see :func:`markdown_format` for the pattern. This function is the skill's
    single source of truth; the tool merely delegates to it.
    """

    from __future__ import annotations

    from dataclasses import dataclass
    from typing import Callable

    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ["Skill", "SKILLS", "SkillTool", "register_skills", "markdown_format"]


    @dataclass(frozen=True, slots=True)
    class Skill:
        """A maintainable hint the agent can request on demand."""

        name: str
        description: str
        hint: str
        #: Real function backing this skill in the ``FunctionRegistry``; its
        #: return value is the skill's instructions.
        core: Callable[[], str]

        @property
        def tool_name(self) -> str:
            """MCP tool name: hyphens are not valid in the core function's name."""
            return self.name.replace("-", "_")

        @property
        def tool_description(self) -> str:
            """Description + hint, as advertised to the agent."""
            return f"{self.description}\n\n{self.hint}"


    class SkillTool(ToolDefinition):
        """Exposes a single :class:`Skill` as an input-less tool.

        The core function is the primary source: the tool schema merely wraps
        its string result into the MCP output format.
        """

        input_schema = {"type": "object", "properties": {}}
        output_schema = {
            "type": "object",
            "properties": {
                "instructions": {"type": "string"},
            },
            "required": ["instructions"],
        }
        annotations = {"readOnlyHint": True, "openWorldHint": False}

        def __init__(self, skill: Skill) -> None:
            self._skill = skill
            self.name = skill.tool_name
            self.title = skill.name
            self.description = skill.tool_description

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Call the skill's core function and wrap its result into the MCP schema."""
            return ToolResult(structured_content={"instructions": self._skill.core()})


    _MARKDOWN_FORMAT_INSTRUCTIONS = (
        "* Use a line containing only `***` to insert a page break in PDF output.\n"
        "* Insert page breaks before top-level chapters (H1) at the start of each chapter.\n"
        "* Use `\\n---\\n` as a section separator before second-order chapters (H2) at the start of each chapter.\n"
        "* All files must end with an additional newline to prevent Markdown formatting errors on merge.\n"
        "* Use third-order headings and below only when necessary for navigation; use simple bold paragraph headings instead.\n"
        "* Chapter headings are numbered for H1–H3 only; lower-order headings do not contain numbering.\n"
        "* Use LaTeX (`$$`) for block mathematical expressions and inline LaTeX (`$`) for inline mathematical symbols, expressions, and formulas."
    )


    def markdown_format() -> str:
        """Preferred formatting rules for Pandoc-compatible Markdown documents.

        Load when formatting rules are requested or required; apply proactively
        whenever creating, editing, or reviewing Markdown documents — even when
        formatting is not explicitly mentioned.

        Returns:
            The formatting instructions text.
        """
        return _MARKDOWN_FORMAT_INSTRUCTIONS


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
            core=markdown_format,
        ),
    ]


    def register_skills(
        registry: ToolRegistry,
        environment: "AppEnvironment | None" = None,
        skills: "list[Skill] | None" = None,
    ) -> None:
        """Register every skill in *skills* (defaults to :data:`SKILLS`) in both registries."""
        functions = environment.functions if environment is not None else None
        for skill in SKILLS if skills is None else skills:
            registry.register(SkillTool(skill))
            if functions is not None:
                functions.register(skill.core, id=skill.tool_name)
```
/answer 7474-11 allow
`register_skill` wurde entfernt; `register_skills` registriert jeden Skill jetzt direkt in beiden Registries (`registry.register(SkillTool(skill))` und `functions.register(skill.core, id=skill.tool_name)`), inklusive Anpassung von `__all__`.
Das ist nicht richtig. Dies macht keinen sinn `core=markdown_format,`. Die Function Registry soll direkt die markdown_format Function bekommen.
SystemInit:  id=f3bdcdad-7c01-47fb-8053-e03e7699615d, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7474-12
toolName: ask_user
arguments:
  question: "Um das richtig zu verstehen: Soll das `Skill`-Dataclass-Feld `core` (das die Funktion referenziert) ganz entfallen, und stattdessen soll `SKILLS` direkt eine Liste/ein Mapping der echten Funktionen sein (z. B. `[markdown_format]`), aus denen Name/Description/Hint abgeleitet werden – oder soll nur der Feldname `core` anders heißen (z. B. `function`), die Struktur mit description/hint aber bleiben?"
  reason: "Klärung, wie die Skill-Definition genau umgebaut werden soll, damit die Function Registry „direkt“ die markdown_format-Funktion erhält, ohne den bisherigen Ansatz falsch zu interpretieren."
```
/answer 7474-12 deny Skill kann auch eine ABC werden und bekommt eine konkrete instanz mit markdown_format die die MCP Schemata enthält.
ReasoningToken: 1710
Control Request:
```yaml
id: 7474-13
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  mode: replace
  reason: "Skill zu einer ABC (analog ToolDefinition) umbauen; konkrete Skills binden ihre reale Funktion direkt als core-staticmethod, sodass die FunctionRegistry unmittelbar die echte Funktion (z. B. markdown_format) erhält, ohne Umweg über ein separat gesetztes Dataclass-Feld."
  content: |
    """Skills – on-demand hints an agent can request.

    A *skill* is a small, easy-to-maintain unit combining an MCP tool schema
    (name, title, description, hint) with a real, named module-level function
    that backs it as ``core`` (``FunctionRegistry`` requires actually existing
    functions with a real signature/docstring — they cannot be generated
    dynamically). Every declared skill is automatically exposed as a tool: it
    takes **no inputs** and calling it wraps its ``core`` function's result into
    the MCP output schema.

    To add a skill, write a real, named module-level function (no parameters,
    returning the instructions text) and a :class:`Skill` subclass that sets
    ``core`` to that function directly — see :func:`markdown_format` and
    :class:`MarkdownFormatSkill` for the pattern — then append an instance to
    :data:`SKILLS`.
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
        environment: "AppEnvironment | None" = None,
        skills: "list[Skill] | None" = None,
    ) -> None:
        """Register every skill in *skills* (defaults to :data:`SKILLS`) in both registries."""
        functions = environment.functions if environment is not None else None
        for skill in SKILLS if skills is None else skills:
            registry.register(skill)
            if functions is not None:
                functions.register(skill.core, id=skill.name)
```
/answer 7474-13 allow
ReasoningToken: 6574
`Skill` ist jetzt eine ABC (analog `ToolDefinition`/`ReplaceBlockTool`):

- `Skill(ToolDefinition, ABC)` definiert Schema-Gemeinsamkeiten (`input_schema`, `output_schema`, `annotations`), kombiniert `description` + `hint` im `__init__`, und ruft in `handle()` einfach `self.core()` auf.
- `core` ist eine abstrakte `staticmethod`.
- `MarkdownFormatSkill(Skill)` ist die konkrete Instanz: sie trägt das MCP-Schema (`name`, `title`, `description`, `hint`) **und** setzt `core = staticmethod(markdown_format)` – eine direkte Bindung an die echte, benannte Funktion, keine Indirektion über ein separat befülltes Dataclass-Feld.
- In `register_skills` erhält `functions.register(skill.core, id=skill.name)` dadurch unmittelbar das reale `markdown_format`-Funktionsobjekt.
