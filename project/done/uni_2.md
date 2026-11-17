Folgendes wurd verändert. Prüfe nun die Tests.

Vordefinierte Tool Kommandos ("run_command")
- "runTests": Führt einen pyTest Testlauf durch.

## Was jetzt gilt

**Neues Modul `tools/results.py`**
- `ItemResult`, `ItemError` und `BatchResult` entsprechen dem Entwurf.
- `__init_subclass__` verbietet reservierte Feldnamen und das Überschreiben von `to_dict`.
- `BatchResult.__post_init__` prüft die Item-Typen zur Laufzeit.
- `all_warnings()` dedupliziert und hängt `derived_warnings()` an.
- `WARNINGS_SCHEMA` kommt dazu.

**`ToolResult`**
- Neu sind `warnings` und `ToolResult.failure(msg)`.
- Ein Fehler-Result darf weder Struktur noch Warnings tragen.
- Die Root-Keys `warnings` und `messageFromUser` sind reserviert, `to_dict` ist der einzige Schreiber.
- Die Registry hängt `warnings` an jedes Output-Schema.
- `normalize_result` akzeptiert nur noch `ToolResult`.
- Die Komfort-Rückgaben `str`, `dict` und `None` leben jetzt im Embedded-Adapter `FunctionTool`.

**`_tool_helpers`**
- `BatchError` ist entfernt, es war ungenutzt.
- `serialize_batch_result` prüft per `isinstance` statt per `getattr`.
- Neu ist `batch_tool_result(...)` als gemeinsamer Übergang zu `ToolResult` inklusive Root-Warnings.
- Alle ca. 20 `is_error=True`-Stellen laufen über `ToolResult.failure`, auch in `mcp_protocol`.

**Migrierte Tools**
- Item-Warnings: `list`, `search` (zwei Warnungen jetzt getrennt), `read` (`unchanged` bleibt typisiertes Feld), `ast_read`, `ast_outline`.
- Import-Warnung: `ast_replace`, `ast_delete`, `ast_insert`, `ast_edit_block` und `ast_edit_marks` leiten sie über die neue Basis `ImportCheckedBatchResult` aus den Items ab. `import_notice` und `notice` sind weg.
- Auch umgestellt: `ast_create`, `skills`, `edit_lines`, `edit_line`, `edit_block`, `edit_marks`, `write`, `file_stats`.
- `ast_read` läuft jetzt über `handle_batch_tool`. Der Outline-Fallback ist ein Item-Warning in der Fachfunktion, das doppelte `error` entfällt.
- Exa- und OpenAlex-`*_results` melden unbekannte oder abgelaufene ids jetzt als `ToolResult.warnings`, nicht mehr nur im Log.

**Tests**
- `tests/test_ast_imports.py` nutzt `all_warnings()`.
- Neu ist `tests/test_result_contract.py`. Er prüft die Klassen-Guards, die Serialisierungsregeln, die `ToolResult`-Regeln und die ids-Warnung. Zusätzlich parst er `tools/**/*.py` per `ast` und schlägt bei Feldern oder Dict-Keys `warning`, `notice` oder `hint` außerhalb von `results.py` an.

## Abweichungen vom Entwurf

- **`payload()` auch für Fehler:** `ItemError` hat ebenfalls einen `payload()`-Hook. `ast_edit_block` und `ast_edit_marks` brauchen ihn für `candidates`, `reason`, `guess` und weitere Zusatzfelder.
- **`to_dict(payload=...)`:** `to_dict` akzeptiert optional einen Payload. Das braucht `search` für den kontextabhängigen `intent`.
- **`ast/outcomes.py`:** Das neue Modul bündelt `NodeError`, `node_key` und `edit_payload`, damit nicht fünfmal derselbe Serializer steht.
- **Root-Warnings:** Sie schreibt nur `ToolResult.to_dict`, nicht mehr `serialize_batch_result`. Beides gleichzeitig hätte der Reservierungsregel widersprochen.
- **Zusätzlich migriert:** `file_stats`, `edit_line` und `write` standen nicht im Plan. Sie nutzen `handle_batch_tool` und wären sonst am neuen `isinstance`-Gate gescheitert.
- **`process.py`:** `process.py` ruft jetzt `.payload()` statt `asdict(...)` auf, damit `warnings` nicht in `file_stats` auftaucht.

## Was sich nach außen ändert

- **Wire-Format:** Statt `warning`, `notice` und `ReadResult.errors` gibt es überall `warnings` als Liste.
- **Leere `ids` bei `ast_read`:** Das Python-API (`ast_read`) fällt jetzt wie das Tool auf den Outline zurück, statt einen Fehler zu liefern. Das folgt aus „Fallback in die Fachfunktion“.
- **Fehlendes `path`:** Beim `ast_read`-Tool gibt ein fehlendes `path` jetzt den allgemeinen Fehler „missing required property“ zurück.
- **Nicht migriert:** `bash`, `tool_usage`, `ast_find`, `ast_validate` und `ast_script` bauen weiter eigene Strukturen und nutzen `ToolResult` direkt. `BashItemError` und ähnliche Klassen sind bewusst unverändert. Sie laufen nicht über `handle_batch_tool`.

Wenn beim Testlauf etwas bricht, ist die wahrscheinlichste Stelle die Dataclass-Vererbung der neuen Basisklassen (frozen, kw_only, Generic).
SystemInit:  id=b1a49d4f-5984-42ed-bb5d-5e1e0d45f868, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Control Request:
```yaml
id: 6bd4-e3-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/mcpc
    id: runTests
  reason: Run the pytest suite after the results refactor
```
/answer 6bd4-e3-0 allow
Control Request:
```yaml
id: 6bd4-e4-e3
result:
  structuredContent:
    results:
    - exit_code: 1
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 310 items

        tests/test_ast_engines.py ..................                             [  5%]
        tests/test_ast_imports.py ....................                           [ 12%]
        tests/test_ast_imports_rpc.py ...........                                [ 15%]
        tests/test_ast_rust_bridge.py ....                                       [ 17%]
        tests/test_cli_session.py .................                              [ 22%]
        tests/test_edit_diagnostics.py .......................                   [ 30%]
        tests/test_edit_tolerance.py ...........................                 [ 38%]
        tests/test_embedded.py .............                                     [ 42%]
        tests/test_embedded_batch.py .                                           [ 43%]
        tests/test_grep_with_ast.py ...                                          [ 44%]
        tests/test_http_human_in_the_loop.py ..                                  [ 44%]
        tests/test_http_tool_calls.py ...                                        [ 45%]
        tests/test_import_all.py ............................................... [ 60%]
        ........................................................................ [ 84%]
        .                                                                        [ 84%]
        tests/test_result_contract.py ......................F                    [ 91%]
        tests/test_rpc_java_engine.py ..........                                 [ 95%]
        tests/test_smoke.py ..                                                   [ 95%]
        tests/test_tools_family_smoke.py ...                                     [ 96%]
        tests/test_validate_arguments.py ..........                              [100%]

        =================================== FAILURES ===================================
        _____ test_no_ad_hoc_warning_notice_or_hint_fields_outside_results_module ______

            def test_no_ad_hoc_warning_notice_or_hint_fields_outside_results_module():
                hits: list[str] = []
                for path in sorted(TOOLS_DIR.rglob('*.py')):
                    if path.name == 'results.py' and path.parent == TOOLS_DIR:
                        continue
                    hits.extend(_forbidden_names(path))
        >       assert not hits, 'use ItemResult.warnings / BatchResult.warnings / ToolResult.warnings:\n' + '\n'.join(hits)
        E       AssertionError: use ItemResult.warnings / BatchResult.warnings / ToolResult.warnings:
        E         __init__.py:36: field hint
        E       assert not ['__init__.py:36: field hint']

        tests/test_result_contract.py:177: AssertionError
        =========================== short test summary info ============================
        FAILED tests/test_result_contract.py::test_no_ad_hoc_warning_notice_or_hint_fields_outside_results_module
        ======================== 1 failed, 309 passed in 10.04s ========================
```
/answer 6bd4-e4-e3 allow
Text:
 Der Fund liegt in `tools/__init__.py` Zeile 36. Ich sehe mir die Stelle an.
Control Request:
```yaml
id: 6bd4-e5-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/tools/__init__.py
  lineno: 36
  reason: Inspect the flagged 'hint' field at line 36
```
/answer 6bd4-e5-0 allow
Control Request:
```yaml
id: 6bd4-e6-e5
result:
  content:
  - type: text
    text: File not found.
  isError: true
```
/answer 6bd4-e6-e5 allow
Control Request:
```yaml
id: 6bd4-e7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: def _forbidden_names|FORBIDDEN|forbidden
    include:
    - test_result_contract.py
  reason: Locate the checker logic in the contract test
```
/answer 6bd4-e7-0 allow
Control Request:
```yaml
id: 6bd4-e8-e7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: test_result_contract.py
        matches:
        - "FORBIDDEN_NAMES = {'warning', 'notice', 'hint'}"
        - "def _forbidden_names(path: Path) -> list[str]:"
        - "if isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name) and (node.target.id in FORBIDDEN_NAMES):"
        - "key, ast.Constant) and key.value in FORBIDDEN_NAMES))"
        - "if isinstance(node, ast.Subscript) and isinstance(node.slice, ast.Constant) and (node.slice.value in FORBIDDEN_NAMES):"
        - hits.extend(_forbidden_names(path))
```
/answer 6bd4-e8-e7 allow
Text:
 Der Pfad ist relativ zu `TOOLS_DIR`. Ich suche die Datei.
Control Request:
```yaml
id: 6bd4-e9-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: "^\\s+hint\\s*:"
    include:
    - __init__.py
    exclude:
    - .venv/**
    - node_modules/**
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: TOOLS_DIR\s*=
    include:
    - test_result_contract.py
  reason: Find the file with the annotated 'hint' field and TOOLS_DIR definition
```
/answer 6bd4-e9-0 allow
Control Request:
```yaml
id: 6bd4-ea-e9
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: src/xy/ai/mcpc/tools/skills/__init__.py
        matches:
        - "hint: str"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: test_result_contract.py
        matches:
        - "TOOLS_DIR = Path(tools_package.__path__[0])"
```
/answer 6bd4-ea-e9 allow
Control Request:
```yaml
id: 6bd4-eb-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  lineno: 36
  reason: Read the class containing the flagged hint field
```
/answer 6bd4-eb-0 allow
Control Request:
```yaml
id: 6bd4-ec-eb
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
      nodes:
      - id: Skill.x7yrhO|2w2wOc
        code: |-
          'Metadata and backing function for one on-demand skill hint.'
          name: str
          title: str
          description: str
          hint: str
```
/answer 6bd4-ec-eb allow
ReasoningToken: 74
Control Request:
```yaml
id: 6bd4-ed-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: \.hint\b|hint=
    include:
    - '*.py'
  reason: See how widely Skill.hint is used
```
/answer 6bd4-ed-0 allow
Control Request:
```yaml
id: 6bd4-ee-ed
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/control/manager.py
        matches:
        - "approval.get('result'), item.result), approval_hint=approval.get('hint'), force_result_review=bool("
```
/answer 6bd4-ee-ed allow
Result Stats: id=b1a49d4f-5984-42ed-bb5d-5e1e0d45f868, total: 13633, in: 12, out: 1788, reason: 0, read: 51736, write: 11833, start: 1794950176924, end: 1794950182198, time: 5231, total: 31399, io: 0, control: 43, sent: 0, recv: 129
Testlauf: 309 von 310 Tests bestehen, 1 schlägt fehl.

**Fehlgeschlagen:** `tests/test_result_contract.py::test_no_ad_hoc_warning_notice_or_hint_fields_outside_results_module`

Der AST-Wächter meldet `tools/skills/__init__.py:36: field hint`. Das ist das Feld `hint: str` der Dataclass `Skill` (`src/xy/ai/mcpc/tools/skills/__init__.py`). Es ist Skill-Metadaten, nämlich ein Kurzhinweis zum Skill. Es ist keine Warnung im Result-Wire-Format. Der Test liefert hier also einen Fehlalarm. Die Result-Migration selbst ist nicht defekt.

Das Feld wird im Quellcode weder über `.hint` gelesen noch über `hint=` gesetzt. Die einzige Fundstelle ist `approval_hint` in `manager.py`, und die ist ein anderes Thema. Es nutzt es also niemand sichtbar im Code. Möglicherweise lesen Skill-Definitionen oder Templates es dynamisch.

Ich habe nichts geändert. Es gibt zwei Lösungen:
1. **`Skill.hint` umbenennen**, zum Beispiel in `summary` oder `usage_hint`. Das hält die Regel streng, ändert aber die Skill-Daten.
2. **Allowlist im Test**, nur für `skills/__init__.py` und `Skill.hint`. Das ist weniger invasiv.

Ich würde Variante 1 vorziehen, falls `hint` nicht nach außen sichtbar ist. Soll ich das umsetzen, und welche Variante?
Bist du sicher. Das klingt eher nach einem Rückgabefeld.
SystemInit:  id=b1a49d4f-5984-42ed-bb5d-5e1e0d45f868, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Das hatte ich nicht ausreichend geprüft. Ich lese das Modul.
ReasoningToken: 20
Control Request:
```yaml
id: 6bd4-ef-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills
    pattern: hint|Skill\b
    include:
    - '*.py'
    limit: 50
  reason: Check how Skill and hint are used in the skills tool
```
/answer 6bd4-ef-0 allow
Control Request:
```yaml
id: 6bd4-f0-ef
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills
      matches:
      - path: __init__.py
        matches:
        - "\"\"\"Skills – on-demand hints an agent can request, for a batch of items."
        - "description, hint) with a real, named module-level function backing it as"
        - "'Skill',"
        - "'MarkdownFormatSkill',"
        - "class Skill(ABC):"
        - '"""Metadata and backing function for one on-demand skill hint."""'
        - "hint: str"
        - "class MarkdownFormatSkill(Skill):"
        - "hint = 'Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.'"
        - "SKILLS: list[Skill] = [MarkdownFormatSkill()]"
        - "def _skill_by_name(name: str) -> Skill:"
        - "self.description = f'Load one or more on-demand skill hints, for a batch of items. Available skills:\\n{catalog}'"
```
/answer 6bd4-f0-ef allow
Control Request:
```yaml
id: 6bd4-f1-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  reason: See how the catalog and results use hint
```
/answer 6bd4-f1-0 allow
Control Request:
```yaml
id: 6bd4-f2-f1
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
      nodes:
      - id: JpZ0JI|u5Vo8y
        excert: "'Skills – on-demand hints an agent can request, for a batch of items.\\n\\nA *ski…"
      - id: 7gckXa|lXAsYV
        excert: "from abc import ABC, abstractmethod"
      - id: VuIsc0|6lIwaj
        excert: "__all__ = ["
      - id: Skill
        excert: "class Skill(ABC):"
        docstring: Metadata and backing function for one on-demand skill hint.
        children:
        - id: Skill.x7yrhO|2w2wOc
          excert: '''Metadata and backing function for one on-demand skill hint.'''
        - id: Skill.core
          excert: "@staticmethod @abstractmethod def core() -> str:"
          docstring: Real function backing this skill; also registered in the ``FunctionRegistry``.
      - id: markdown_format
        excert: "def markdown_format() -> str:"
        docstring: Preferred formatting rules for Pandoc-compatible Markdown documents. Load when …
      - id: MarkdownFormatSkill
        excert: "class MarkdownFormatSkill(Skill):"
      - id: E2VVDV|aWqT5h
        excert: "'#: All declared skills. Append here to add a new one.'"
      - id: SkillsError
        excert: "class SkillsError(Exception):"
        docstring: Raised when a skill cannot be resolved.
      - id: SkillsItem
        excert: "@dataclass(frozen=True) class SkillsItem:"
        docstring: "One skill to fetch. Attributes: name: Name of the skill to load."
      - id: SkillsResult
        excert: "@dataclass(frozen=True) class SkillsResult(ItemResult):"
        docstring: "Result of fetching a single skill, mirroring its input name for result associat…"
        children:
        - id: SkillsResult.uBv381|WLNQ3W
          excert: "'Result of fetching a single skill, mirroring its input name for result associa…"
        - id: SkillsResult.key
          excert: "def key(self) -> dict[str, Any]:"
        - id: SkillsResult.payload
          excert: "def payload(self) -> dict[str, Any]:"
      - id: SkillsItemError
        excert: "@dataclass(frozen=True) class SkillsItemError(ItemError):"
        docstring: "Error fetching a single skill, mirroring its input name for result association."
        children:
        - id: SkillsItemError.XSEIVv|Z0JayK
          excert: "'Error fetching a single skill, mirroring its input name for result association…"
        - id: SkillsItemError.key
          excert: "def key(self) -> dict[str, Any]:"
      - id: SkillsBatchResult
        excert: "class SkillsBatchResult(BatchResult[SkillsResult, SkillsItemError]):"
        docstring: "Result of :func:`get_skills`. Attributes: results: One :class:`SkillsResult` pe…"
      - id: skill_by_name
        excert: "def _skill_by_name(name: str) -> Skill:"
      - id: get_skills
        excert: "def get_skills(items: list[SkillsItem]) -> SkillsBatchResult:"
        docstring: "Fetch the instructions of one or more skills, for a batch of items. Args: items…"
      - id: SkillsTool
        excert: "class SkillsTool(ToolDefinition):"
        children:
        - id: SkillsTool.eOnl2A|xosk3y
          excert: name = 'skills'
        - id: SkillsTool.v23h4l|cBlE9D
          excert: "input_schema = {"
        - id: SkillsTool.init
          excert: "def __init__(self) -> None:"
        - id: SkillsTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`get_skills`, translating the MCP schema to/from the Python A…"
      - id: register_skills
        excert: "def register_skills(registry: ToolRegistry, environment: AppEnvironment) -> None:"
        docstring: Register the batching skills tool and each skill's backing function.
```
/answer 6bd4-f2-f1 allow
Control Request:
```yaml
id: 6bd4-f3-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    ids:
    - SkillsResult
    - SkillsTool.init
  reason: Check whether hint reaches the output or only the tool description
```
/answer 6bd4-f3-0 allow
Control Request:
```yaml
id: 6bd4-f4-f3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
      nodes:
      - id: SkillsResult
        type: ClassDef
        excert: "@dataclass(frozen=True) class SkillsResult(ItemResult):"
        docstring: "Result of fetching a single skill, mirroring its input name for result associat…"
        children:
        - id: SkillsResult.uBv381|WLNQ3W
          type: statements
          code: |-
            'Result of fetching a single skill, mirroring its input name for result association.'
            name: str
            instructions: str
        - id: SkillsResult.key
          type: FunctionDef
          code: |-
            def key(self) -> dict[str, Any]:
                return {'name': self.name}
        - id: SkillsResult.payload
          type: FunctionDef
          code: |-
            def payload(self) -> dict[str, Any]:
                return {'instructions': self.instructions}
      - id: SkillsTool.init
        type: FunctionDef
        code: |-
          def __init__(self) -> None:
              catalog = '\n'.join((f'- {skill.name}: {skill.description}' for skill in SKILLS))
              self.description = f'Load one or more on-demand skill hints, for a batch of items. Available skills:\n{catalog}'
```
/answer 6bd4-f4-f3 allow
ReasoningToken: 81
Control Request:
```yaml
id: 6bd4-f5-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: "getattr\\([^)]*hint|\\bhint\\b"
    include:
    - '*.py'
    - '*.md'
    - '*.json'
    exclude:
    - .venv/**
    - node_modules/**
    - .git/**
    limit: 30
  reason: Check for any dynamic consumer of Skill.hint
```
/answer 6bd4-f5-0 allow
Control Request:
```yaml
id: 6bd4-f6-f5
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: project/uni_2.md
        matches:
        - "- Neu ist `tests/test_result_contract.py`. Er prüft die Klassen-Guards, die Serialisierungsregeln, die `ToolResult`-Regeln und die ids-Warnung. Zusätzlich parst er `tools/**/*.py` per `ast` und schlägt bei Feldern oder Dict-Keys `warning`, `notice` oder `hint` außerhalb von `results.py` an."
        - "E         __init__.py:36: field hint"
        - "E       assert not ['__init__.py:36: field hint']"
        - "reason: Inspect the flagged 'hint' field at line 36"
        - "- \"FORBIDDEN_NAMES = {'warning', 'notice', 'hint'}\""
        - "pattern: \"^\\\\s+hint\\\\s*:\""
        - "reason: Find the file with the annotated 'hint' field and TOOLS_DIR definition"
        - "- \"hint: str\""
        - "reason: Read the class containing the flagged hint field"
        - '''Metadata and backing function for one on-demand skill hint.'''
        - "hint: str"
        - "pattern: \\.hint\\b|hint="
        - "reason: See how widely Skill.hint is used"
        - "- \"approval.get('result'), item.result), approval_hint=approval.get('hint'), force_result_review=bool(\""
        - "Der AST-Wächter meldet `tools/skills/__init__.py:36: field hint`. Das ist das Feld `hint: str` der Dataclass `Skill` (`src/xy/ai/mcpc/tools/skills/__i…nweis zum Skill. Es ist keine Warnung im Result-Wire-Format. Der Test liefert hier also einen Fehlalarm. Die Result-Migration selbst ist nicht defekt."
        - "Das Feld wird im Quellcode weder über `.hint` gelesen noch über `hint=` gesetzt. Die einzige Fundstelle ist `approval_hint` in `manager.py`, und die ist ein anderes Thema. Es nutzt es also niemand sichtbar im Code. Möglicherweise lesen Skill-Definitionen oder Templates es dynamisch."
        - "1. **`Skill.hint` umbenennen**, zum Beispiel in `summary` oder `usage_hint`. Das hält die Regel streng, ändert aber die Skill-Daten."
        - "2. **Allowlist im Test**, nur für `skills/__init__.py` und `Skill.hint`. Das ist weniger invasiv."
        - "Ich würde Variante 1 vorziehen, falls `hint` nicht nach außen sichtbar ist. Soll ich das umsetzen, und welche Variante?"
        - "pattern: hint|Skill\\b"
        - "reason: Check how Skill and hint are used in the skills tool"
        - "- \"description, hint) with a real, named module-level function backing it as\""
        - '- ''"""Metadata and backing function for one on-demand skill hint."""'''
        - "- \"hint = 'Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.'\""
        - "reason: See how the catalog and results use hint"
        - "docstring: Metadata and backing function for one on-demand skill hint."
        - "excert: '''Metadata and backing function for one on-demand skill hint.'''"
        - "reason: Check whether hint reaches the output or only the tool description"
        - "pattern: \"getattr\\\\([^)]*hint|\\\\bhint\\\\b\""
      warning: "Limit of 30 matches reached (total 202); narrow pattern, directory or include/exclude filters, or raise limit (max 50). 1 match line(s) exceeded 300 characters and were shortened in the middle."
```
/answer 6bd4-f6-f5 allow
ReasoningToken: 19
Control Request:
```yaml
id: 6bd4-f7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: \bhint\b
    include:
    - '*.py'
    limit: 30
  reason: "Search code only, excluding notes, for any consumer of hint"
```
/answer 6bd4-f7-0 allow
Control Request:
```yaml
id: 6bd4-f8-f7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: xy/ai/mcpc/control/manager.py
        matches:
        - '''Human-readable hint for the agent when ``approved`` is False.'''
        - "'Optional human-written hint attached to an *approval* (``/allow <id> <hint>``).\\n\\n    Unlike :attr:`rejection_reason`, this never aborts the call — …   ``request`` and ``result`` phase; when a hint is supplied at both phases\\n    for the same call, the two are combined (see ``protocol.py``).\\n    '"
        - '``auto_approve`` is a hint set by the *tool itself* (via'
        - "* ``{\"id\": \"…\", \"hint\": \"…\"}`` — approve with a hint for the agent"
        - "(``/allow <id> <hint>``); does not reject the call, only enriches"
        - "approval.get('result'), item.result), approval_hint=approval.get('hint'), force_result_review=bool("
      - path: xy/ai/mcpc/server/mcp_protocol.py
        matches:
        - "\"# For ask_user, an approval hint *is* the human's answer\""
        - "'# hint *is* the answer, not an independent side-channel field.'"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "hint = 'Path is a directory, not a file; listing its contents instead.'"
        - "return f'{hint} Listing failed: {exc}'"
        - "return f'{hint} Listing failed: {batch.errors[0].error}'"
        - "return '\\n'.join([hint, *result.warnings, *result.entries])"
      - path: xy/ai/mcpc/tools/results.py
        matches:
        - "RESERVED_NAMES = frozenset({'warning', 'warnings', 'notice', 'notices', 'hint', 'error', 'errors', 'message'})"
      - path: xy/ai/mcpc/tools/skills/__init__.py
        matches:
        - "description, hint) with a real, named module-level function backing it as"
        - '"""Metadata and backing function for one on-demand skill hint."""'
        - "hint: str"
        - "hint = 'Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.'"
      - path: xy/ai/mcpc/tools/tool_registry.py
        matches:
        - "\"#: Name of the optional hint property injected into every tool's output\""
      - path: xy/ai/mcpc/tools/_text_match.py
        matches:
        - "a likely content change (unverified guess), or a genuine miss (re-read hint)."
      - path: xy/ai/mcpc/tools/search/__init__.py
        matches:
        - "hint = 'narrow pattern, directory or include/exclude filters'"
        - "hint += f', or raise limit (max {_MAX_LIMIT})'"
        - "warnings.append(f'Limit of {item.limit} matches reached (total {total_lines}); {hint}.')"
      - path: test_result_contract.py
        matches:
        - "FORBIDDEN_NAMES = {'warning', 'notice', 'hint'}"
        - "@pytest.mark.parametrize('base, field', [(ItemResult, 'warning'), (ItemResult, 'notice'), (ItemError, 'hint'), (ItemError, 'errors'), (BatchResult, 'notice')])"
      - path: test_tools_family_smoke.py
        matches:
        - "'# 2) inspect its usage; repeating the same request yields a hint, not the info again.'"
      - path: test_http_human_in_the_loop.py
        matches:
        - "then do the same for the result-phase item (attaching a human hint), and"
        - that the hint landed in ``structuredContent.messageFromUser``.
        - "'# -- phase 2: approve the result, attaching a human hint ---------------'"
        - "control_poll([{'id': result_item['id'], 'hint': 'looks fine'}])"
      warning: 1 match line(s) exceeded 300 characters and were shortened in the middle.
```
/answer 6bd4-f8-f7 allow
Result Stats: id=b1a49d4f-5984-42ed-bb5d-5e1e0d45f868, total: 22412, in: 24, out: 3676, reason: 218, read: 136505, write: 18712, start: 1794950262202, end: 1794950269441, time: 7194, total: 50675, io: 0, control: 45, sent: 0, recv: 141
ReasoningToken: 218
Nach genauerem Prüfen stimmt das nur teilweise. Als Rückgabefeld wird `hint` derzeit nicht verwendet.

**Was dagegen spricht:**
- `SkillsResult` liefert nur `name` und `instructions`. `payload()` enthält kein `hint`.
- Die Tool-Beschreibung baut den Katalog nur aus `skill.name` und `skill.description`.
- Im Code und in den Tests liest niemand `Skill.hint`. Die einzigen Treffer sind die Deklaration, der Wert in `MarkdownFormatSkill` und der Modul-Docstring.

**Was für Ihre Sicht spricht:**
- Der Wert von `MarkdownFormatSkill.hint` ist an den Agenten gerichtet: „Apply proactively whenever creating, editing, or reviewing Markdown documents …“. Das ist eine Nutzungsempfehlung.
- Wahrscheinlich sollte er in den Katalog oder die Ausgabe gelangen und tut es nicht. Das ist ein toter Wert, keine Warnung im Sinne der Result-Regel. Ich weiß nicht, ob das schon vor der Migration so war. `git diff` oder `git log` dazu habe ich nicht geprüft.

**Mein Vorschlag:**
- **Entweder** benennen wir das Feld in `usage` oder `when_to_use` um und nehmen es in den Katalog auf (`- name: description — usage`). Dann ist es ein echtes Feld, und der Wächter hat keinen Grund mehr zu schlagen.
- **Oder** wir lassen das Feld, wie es ist, und tragen es im Test als Ausnahme ein.

Ich würde die erste Variante nehmen. Soll ich das so umsetzen?
Dann klingt das so als sollte der Agent das als usage hint bekommen.
SystemInit:  id=b1a49d4f-5984-42ed-bb5d-5e1e0d45f868, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
ReasoningToken: 77
Control Request:
```yaml
id: 6bd4-f9-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    ids:
    - JpZ0JI|u5Vo8y
    - Skill
    - MarkdownFormatSkill
  reason: Read the exact source before renaming hint to usage
```
/answer 6bd4-f9-0 allow
Control Request:
```yaml
id: 6bd4-fa-f9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
      nodes:
      - id: JpZ0JI|u5Vo8y
        type: statements
        code: "'Skills – on-demand hints an agent can request, for a batch of items.\\n\\nA *skill* is a small, easy-to-maintain unit combining metadata (name, title,\\ndescription, hint) with a real, named module-level function backing it as\\n``core`` (``FunctionRegistry`` requires actually existing functions with a\\nreal signature/docstring). All skills are exposed through a single batching\\ntool so several skills can be fetched in one call.\\n'"
      - id: Skill
        type: ClassDef
        excert: "class Skill(ABC):"
        docstring: Metadata and backing function for one on-demand skill hint.
        children:
        - id: Skill.x7yrhO|2w2wOc
          type: statements
          code: |-
            'Metadata and backing function for one on-demand skill hint.'
            name: str
            title: str
            description: str
            hint: str
        - id: Skill.core
          type: FunctionDef
          code: |-
            @staticmethod
            @abstractmethod
            def core() -> str:
                """Real function backing this skill; also registered in the ``FunctionRegistry``."""
      - id: MarkdownFormatSkill
        type: ClassDef
        code: |-
          class MarkdownFormatSkill(Skill):
              name = 'markdown_format'
              title = 'markdown-format'
              description = 'Preferred formatting rules for Pandoc-compatible Markdown documents. Load when formatting rules are requested or required.'
              hint = 'Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.'
              core = staticmethod(markdown_format)
```
/answer 6bd4-fa-f9 allow
Control Request:
```yaml
id: 6bd4-fb-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    id: JpZ0JI|u5Vo8y
    old_text: "description, hint) with"
    new_text: "description, usage) with"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    id: Skill.x7yrhO|2w2wOc
    old_text: |-
      description: str
      hint: str
    new_text: |-
      description: str
      usage: str
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    id: MarkdownFormatSkill
    old_text: '    hint = ''Apply proactively'
    new_text: '    usage = ''Apply proactively'
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    id: SkillsTool.init
    old_text: "{skill.name}: {skill.description}'"
    new_text: "{skill.name}: {skill.description} Usage: {skill.usage}'"
  reason: Rename Skill.hint to usage and include it in the tool catalog shown to the agent
```
/answer 6bd4-fb-0 allow
Control Request:
```yaml
id: 6bd4-fc-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/mcpc
    id: runTests
  reason: Re-run tests after the rename
```
/answer 6bd4-fc-0 allow
Control Request:
```yaml
id: 6bd4-fd-fc
result:
  structuredContent:
    results:
    - stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 310 items

        tests/test_ast_engines.py ..................                             [  5%]
        tests/test_ast_imports.py ....................                           [ 12%]
        tests/test_ast_imports_rpc.py ...........                                [ 15%]
        tests/test_ast_rust_bridge.py ....                                       [ 17%]
        tests/test_cli_session.py .................                              [ 22%]
        tests/test_edit_diagnostics.py .......................                   [ 30%]
        tests/test_edit_tolerance.py ...........................                 [ 38%]
        tests/test_embedded.py .............                                     [ 42%]
        tests/test_embedded_batch.py .                                           [ 43%]
        tests/test_grep_with_ast.py ...                                          [ 44%]
        tests/test_http_human_in_the_loop.py ..                                  [ 44%]
        tests/test_http_tool_calls.py ...                                        [ 45%]
        tests/test_import_all.py ............................................... [ 60%]
        ........................................................................ [ 84%]
        .                                                                        [ 84%]
        tests/test_result_contract.py .......................                    [ 91%]
        tests/test_rpc_java_engine.py ..........                                 [ 95%]
        tests/test_smoke.py ..                                                   [ 95%]
        tests/test_tools_family_smoke.py ...                                     [ 96%]
        tests/test_validate_arguments.py ..........                              [100%]

        ============================= 310 passed in 10.04s =============================
```
/answer 6bd4-fd-fc allow
Result Stats: id=b1a49d4f-5984-42ed-bb5d-5e1e0d45f868, total: 27433, in: 30, out: 4969, reason: 0, read: 195417, write: 22434, start: 1794950362549, end: 1794950365714, time: 3119, total: 73996, io: 0, control: 46, sent: 0, recv: 75
Alle 310 Tests bestehen jetzt.

In `tools/skills/__init__.py` habe ich zwei Dinge geändert:
- **Umbenennung:** `Skill.hint` heißt jetzt `usage`. Das betrifft die Deklaration, den Wert in `MarkdownFormatSkill` und den Modul-Docstring.
- **Katalog:** Der Agent bekommt den Hinweis jetzt in der Tool-Beschreibung. Der Katalogeintrag lautet `- name: description Usage: usage`.

Ich habe den Test nicht angepasst. Der Wächter hat auch keine Ausnahme bekommen.

Ob der Katalogtext so gut lesbar ist, habe ich nicht geprüft. Zwischen `description` und `Usage:` steht nur ein Leerzeichen, weil `description` bereits mit einem Punkt endet. Wenn Sie eine eigene Zeile oder ein anderes Format wollen, passe ich das an.
Ist eine Zeilenumbruch nicht besser?
SystemInit:  id=b1a49d4f-5984-42ed-bb5d-5e1e0d45f868, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Control Request:
```yaml
id: 6bd4-fe-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    id: SkillsTool.init
    old_text: "{skill.description} Usage: {skill.usage}'"
    new_text: "{skill.description}\\n  Usage: {skill.usage}'"
  reason: Put the usage hint on its own indented line in the catalog
```
/answer 6bd4-fe-0 allow
ReasoningToken: 36
Control Request:
```yaml
id: 6bd4-ff-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/mcpc
    id: runTests
  reason: Re-run tests after the catalog format change
```
/answer 6bd4-ff-0 allow
Control Request:
```yaml
id: 6bd4-100-ff
result:
  structuredContent:
    results:
    - stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 310 items

        tests/test_ast_engines.py ..................                             [  5%]
        tests/test_ast_imports.py ....................                           [ 12%]
        tests/test_ast_imports_rpc.py ...........                                [ 15%]
        tests/test_ast_rust_bridge.py ....                                       [ 17%]
        tests/test_cli_session.py .................                              [ 22%]
        tests/test_edit_diagnostics.py .......................                   [ 30%]
        tests/test_edit_tolerance.py ...........................                 [ 38%]
        tests/test_embedded.py .............                                     [ 42%]
        tests/test_embedded_batch.py .                                           [ 43%]
        tests/test_grep_with_ast.py ...                                          [ 44%]
        tests/test_http_human_in_the_loop.py ..                                  [ 44%]
        tests/test_http_tool_calls.py ...                                        [ 45%]
        tests/test_import_all.py ............................................... [ 60%]
        ........................................................................ [ 84%]
        .                                                                        [ 84%]
        tests/test_result_contract.py .......................                    [ 91%]
        tests/test_rpc_java_engine.py ..........                                 [ 95%]
        tests/test_smoke.py ..                                                   [ 95%]
        tests/test_tools_family_smoke.py ...                                     [ 96%]
        tests/test_validate_arguments.py ..........                              [100%]

        ============================= 310 passed in 10.09s =============================
```
/answer 6bd4-100-ff allow
Result Stats: id=b1a49d4f-5984-42ed-bb5d-5e1e0d45f868, total: 29654, in: 34, out: 5533, reason: 0, read: 240679, write: 24087, start: 1794950429294, end: 1794950431148, time: 1810, total: 90281, io: 0, control: 44, sent: 0, recv: 37
Ja, das ist besser lesbar. Der Katalogeintrag steht jetzt auf zwei Zeilen, und die zweite ist eingerückt:

```
- markdown_format: Preferred formatting rules for Pandoc-compatible Markdown documents. Load when formatting rules are requested or required.
  Usage: Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.
```

Alle 310 Tests bestehen weiterhin.
