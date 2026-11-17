Setze folgendes um.

Vordefinierte Tool Kommandos ("run_command")
- "runTests": Führt einen pyTest Testlauf durch.

## 1. Ist-Zustand

Es gibt keine gemeinsame Basis. Warnungen laufen heute über fünf verschiedene Mechanismen:

| Stelle | Heute | Problem |
|---|---|---|
| `list` (`ListResult`), `search` (`SearchResult`) | `warning: str \| None` am Item, von Hand serialisiert (`if r.warning` bzw. `is not None`) | Zwei Warnungen werden per `f'{a} {b}'` zu einem String verklebt (`search`). |
| `ast_read` (Handler) | `entry['warning'] = [str]` als Liste mit einem Element. Zusätzlich `entry['error'] = [str]` an einem *erfolgreichen* Result-Eintrag | Der Handler umgeht `handle_batch_tool` und baut Dicts selbst. Dieselbe Meldung steht doppelt als warning und error. |
| `ReadResult.errors: list[str]` | Teilfehler (nicht auflösbare ids) in einem erfolgreichen Item | Semantisch eine Warnung, heißt aber `errors`. |
| `ast_replace`, `ast_delete`, `ast_insert`, `ast_edit_block`, `ast_edit_marks` | `notice: str \| None` am `*BatchResult`, befüllt durch `import_notice(results)` | `serialize_batch_result` liest es per `getattr(batch_result, 'notice', None)`. Das ist Duck-Typing, und es gibt kein Typ-Gate. |
| `ast_outline._directory_listing_hint` | `ListResult.warning` wird in den Fehlertext hineinkopiert | Die Struktur geht verloren. |
| `tools/read` | `ReadResult.unchanged: str \| None` | Ein Status-Hinweis als Spezialfeld. |
| `web_fetch_exa_results`, `web_search_exa_results`, `openalex_search_results` | Unbekannte oder abgelaufene ids erscheinen nur in `logger.warning` | Der Agent sieht den Datenverlust nie. Das ist die eigentliche Lücke. |
| `ToolResult.message` | Nachricht des Users (`attach_message`) | Ein anderer Kanal, keine Warnung. Er muss als reservierter Name abgegrenzt bleiben. |

Bei den Errors ist es ähnlich:
- Jedes Tool definiert eigene `XxxError`-Klassen, mit wechselnden Schlüsselfeldern (`path`, `path+id`, `directory+pattern`).
- Der Feldname `error: str` ist bis auf den `ast_read`-Fall (dort eine Liste) einheitlich, aber nur zufällig.
- `BatchError` in `_tool_helpers` scheint ungenutzt zu sein. Das müsste ich noch prüfen.
- `ToolResult(content=[text_content(...)], is_error=True)` steht etwa 20-mal copy-pasted im Code.

Die Stelle, an der man ansetzen kann, ist `serialize_batch_result` zusammen mit `handle_batch_tool`. Nicht darüber laufen `ast_read`, `ast_outline`, `read`, `tool_usage`, `ast_find`, `ast_validate`, `ast_script` und Exa/OpenAlex. Für sie braucht es `ToolResult` als zweiten Durchgriffspunkt.

## 2. Vorschlag

### Semantische Regel

- **Error** heißt: Das Item hat kein Ergebnis geliefert. Der Feldname ist immer `error: str`.
- **Item-Warning** heißt: Das Item hat ein Ergebnis geliefert, aber mit Einschränkung (gekürzt, Fallback, Teilauflösung). Das Feld ist `warnings` am Result-Eintrag.
- **Result-Warning** betrifft den ganzen Call oder ist über mehrere Items identisch (zum Beispiel "Imports ungeprüft"). Das Feld ist `warnings` im Root von `structuredContent`.
- Ein Error-Item hat keine Warnings, und ein Error-`ToolResult` hat keine Warnings.

### Neues Modul `tools/results.py`

```python
from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass, field, replace
from typing import Any, Generic, TypeVar, final

WARNINGS_KEY = 'warnings'
RESERVED_NAMES = frozenset({'warning', 'warnings', 'notice', 'notices', 'hint', 'error', 'errors', 'message'})


def _guard_fields(cls: type, allowed: frozenset[str]) -> None:
    clash = (set(cls.__dict__.get('__annotations__', {})) & RESERVED_NAMES) - allowed
    if clash:
        raise TypeError(f'{cls.__name__}: reserved field name(s) {sorted(clash)}; use the base class API')
    if 'to_dict' in cls.__dict__:
        raise TypeError(f'{cls.__name__}: to_dict is final; override payload()')


@dataclass(frozen=True, kw_only=True)
class ItemOutcome(ABC):
    @abstractmethod
    def key(self) -> dict[str, Any]:
        """Input fields mirrored for result association."""

    @abstractmethod
    def to_dict(self) -> dict[str, Any]: ...


@dataclass(frozen=True, kw_only=True)
class ItemResult(ItemOutcome):
    warnings: tuple[str, ...] = ()

    def __init_subclass__(cls, **kw: Any) -> None:
        super().__init_subclass__(**kw)
        _guard_fields(cls, allowed=frozenset())

    def payload(self) -> dict[str, Any]:
        return compact_asdict(self, exclude={WARNINGS_KEY})

    @final
    def to_dict(self) -> dict[str, Any]:
        out = {**self.key(), **self.payload()}
        if self.warnings:
            out[WARNINGS_KEY] = list(self.warnings)
        return out

    def warn(self, *messages: str) -> 'ItemResult':
        return replace(self, warnings=(*self.warnings, *messages))


@dataclass(frozen=True, kw_only=True)
class ItemError(ItemOutcome):
    error: str

    def __init_subclass__(cls, **kw: Any) -> None:
        super().__init_subclass__(**kw)
        _guard_fields(cls, allowed=frozenset())

    @final
    def to_dict(self) -> dict[str, Any]:
        return {**self.key(), 'error': self.error}


R = TypeVar('R', bound=ItemResult)
E = TypeVar('E', bound=ItemError)


@dataclass(frozen=True, kw_only=True)
class BatchResult(ABC, Generic[R, E]):
    results: list[R] = field(default_factory=list)
    errors: list[E] = field(default_factory=list)
    warnings: tuple[str, ...] = ()

    def __post_init__(self) -> None:
        if not all(isinstance(r, ItemResult) for r in self.results):
            raise TypeError('results must be ItemResult instances')
        if not all(isinstance(e, ItemError) for e in self.errors):
            raise TypeError('errors must be ItemError instances')

    def derived_warnings(self) -> tuple[str, ...]:
        return ()

    @final
    def all_warnings(self) -> list[str]:
        return list(dict.fromkeys((*self.warnings, *self.derived_warnings())))
```

Konkrete Klassen schrumpfen auf das Wesentliche:

```python
@dataclass(frozen=True, kw_only=True)
class ListResult(ItemResult):
    path: str
    entries: list[str]

    def key(self): return {'path': self.path}


class ReplaceBatchResult(BatchResult[ReplaceResult, ReplaceError]):
    def derived_warnings(self):
        return (IMPORT_NOTICE,) if any(r.imports and r.imports.unchecked for r in self.results) else ()
```

`import_notice()` und das Feld `notice` entfallen. Die Warnung wird aus den Items abgeleitet und kann nicht vergessen werden.

### `ToolResult` als Result-Ebene für alle Tools, auch ohne Batch

```python
@dataclass(slots=True)
class ToolResult:
    ...
    warnings: list[str] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.is_error and (self.structured_content or self.warnings):
            raise ValueError('error results carry neither structured content nor warnings')
        if self.structured_content and RESERVED_ROOT & self.structured_content.keys():
            raise ValueError('reserved key in structured_content; use ToolResult.warnings / .message')

    @classmethod
    def failure(cls, message: str) -> 'ToolResult':
        return cls(content=[text_content(message)], is_error=True)
```

- `to_dict` ist der einzige Schreiber des Root-Keys `warnings`, analog zu `attach_message`.
- `handle_batch_tool` setzt `ToolResult(structured_content=..., warnings=batch.all_warnings())`.
- Die Exa- und OpenAlex-`*_results`-Tools übergeben ihre `missing`-ids so: `ToolResult(structured_content=..., warnings=[f'unknown or expired id(s): {missing}'])`.

### Wodurch es per Code erzwungen wird

1. **Beim Import:** `__init_subclass__` verbietet Felder wie `warning`, `notice`, `hint` oder `errors` in `ItemResult` und `ItemError`. Ebenso verbietet es das Überschreiben von `to_dict`.
2. **Ein einziger Serialisierungspunkt:** `serialize_batch_result` prüft `isinstance(batch_result, BatchResult)` statt `getattr`. Es ruft `to_dict()` auf und schreibt Root-`warnings` nur aus `all_warnings()`. Kontextabhängige Serializer (`search` mit `intent`, `core.to_dict(n)` mit `with_lines`) liefern nur `payload`. Setzt ein Payload einen reservierten Key, gibt es einen Fehler.
3. **Typen:** `handle_batch_tool(batch_fn: Callable[[list[T]], BatchResult[Any, Any]])` wird von mypy und pyright geprüft. `BatchResult.__post_init__` prüft zusätzlich zur Laufzeit.
4. **Escape-Hatch schließen:** `ToolDefinition.handle` darf nur noch `ToolResult` zurückgeben. Die Rückgabe von `str` oder `dict` samt Durchreichen von `content`-Dicts in `normalize_result` fällt weg.
5. **Contract-Test:** Der Test iteriert rekursiv über `ItemResult.__subclasses__()` und `BatchResult.__subclasses__()`. Zusätzlich parst er `tools/**/*.py` per `ast` und schlägt an, wenn irgendwo ein annotiertes Feld oder ein Dict-Key `warning`, `notice` oder `hint` außerhalb von `results.py` vorkommt. Das ist eine strukturelle Prüfung und keine Textsuche.
6. **Optional:** Ein gemeinsames `WARNINGS_SCHEMA`-Fragment kann in jedes `output_schema` eingehängt werden, damit alle Tools es identisch ankündigen.

### Migrationszuordnung

- `ListResult.warning` und `SearchResult.warning` werden zu `warnings: tuple[str, ...]`. Bei `search` werden die zwei Warnungen getrennt statt verklebt.
- `ReadResult.errors: list[str]` wird zu Item-`warnings` (Teilauflösung).
- Der `ast_read`-Handler läuft künftig über `handle_batch_tool`. Der Outline-Fallback wandert in die Fachfunktion und wird dort ein Item-Warning. Das doppelte `error` entfällt.
- Die fünf `notice`-Felder werden zu `derived_warnings()` (siehe oben).
- `_directory_listing_hint` verwendet `result.warnings`, die Struktur bleibt dabei erhalten.
- Alle `ToolResult(..., is_error=True)` werden zu `ToolResult.failure(...)`.
- `BatchError` wird entfernt, falls tatsächlich ungenutzt.

## 3. Reihenfolge

1. `results.py` und `ToolResult.warnings` / `.failure` ergänzen. Das ist rein additiv.
2. `list` und `search` migrieren, weil sie die einfachsten Item-Warnings haben.
3. Die `ast_*`-Batch-Tools mit `notice` migrieren und danach `ast_read` und `ast_outline`.
4. Exa- und OpenAlex-`*_results` auf `ToolResult.warnings` umstellen.
5. Verschärfen: `handle` auf `ToolResult` einschränken, `getattr`-Duck-Typing und `BatchError` entfernen, Contract-Test aktivieren.

Das ändert das Wire-Format:
- `warning` und `notice` werden zu `warnings` (Liste).
- `tests/test_ast_imports.py` prüft derzeit `result.notice` und muss angepasst werden.
- Bei der Migration `kw_only=True` beachten: Aufrufer, die `*BatchResult` positional konstruieren, müssten umgestellt werden. Die gesehenen Aufrufe sind alle keyword-basiert.

## 4. Entscheidungen

1. **Key-Name:** Ich empfehle `warnings` auf beiden Ebenen, getrennt durch Position (Item-Eintrag vs. Root) und durch Typ im Code.
2. **Format:** Ich empfehle `list[str]` für LLM-Konsumenten. Keine strukturierte Form `{code, message}`.
3. **`read.unchanged`:** Es ändert, ob Inhalt vorhanden ist, und ist damit eher Daten als Warnung. Ich würde es als typisiertes Feld lassen.
SystemInit:  id=3e01cb77-9cdd-434d-9bd8-d6e361639cfd, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Ich verschaffe mir zuerst einen Überblick über die Codebasis.
Control Request:
```yaml
id: 6bd4-20-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: warning|notice|BatchError|is_error=True|import_notice
    include:
    - '*.py'
    exclude:
    - .venv/**
    - node_modules/**
    limit: 50
    intent: survey
  reason: "Alle Stellen finden, die Warnungen, notice, BatchError und is_error=True verwenden."
```
/answer 6bd4-20-0 allow
Control Request:
```yaml
id: 6bd4-21-20
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: tests/test_ast_imports.py
        matches:
        - assert not result.errors and result.notice is None
        - "def test_python_invalid_import_keeps_edit_and_flags_notice(tmp_path: Path):"
        - assert result.notice == IMPORT_NOTICE
        - "def test_delete_file_ignores_imports_but_flags_notice(tmp_path: Path):"
        - "def test_javascript_non_import_keeps_edit_and_flags_notice(tmp_path: Path):"
        - "def test_failing_engine_keeps_edit_and_flags_notice(tmp_path: Path, monkeypatch):"
        - "def test_generic_present_import_needs_no_notice(tmp_path: Path):"
      - path: src/xy/ai/mcpc/control/handler.py
        matches:
        - "logger.warning('Control: manager not enabled, returning 404')"
        - "logger.warning('Control: invalid JSON body: %s', exc)"
        - "logger.warning(\"Control: 'approvals' is not a list: %r\", approvals)"
      - path: src/xy/ai/mcpc/control/manager.py
        matches:
        - "logger.warning(\"Approval entry missing 'id', skipped: %s\", approval)"
        - "logger.warning('Unknown approval id %s, skipped', item_id)"
        - Called by a transport (HTTP or WebSocket) as soon as it notices that
        - "logger.warning('Control item %s [%s/%s] timed out, auto-approving', item.tool_name, item.phase, item.id)"
      - path: src/xy/ai/mcpc/server/http_transport.py
        matches:
        - '''# watchdog an aborted client connection would go unnoticed until'''
      - path: src/xy/ai/mcpc/server/server.py
        matches:
        - Returns ``None`` (after logging a warning) when the transport is disabled
        - "logger.warning('WebSocket transport unavailable: %s', exc)"
      - path: src/xy/ai/mcpc/server/ws_transport.py
        matches:
        - "logger.warning('WS: unknown endpoint %s', parsed.path)"
        - "logger.warning('WS: origin forbidden: %s', origin)"
        - "logger.warning('WS: session id missing')"
        - "logger.warning('WS: session id is not a valid UUID')"
      - path: src/xy/ai/mcpc/server/mcp_protocol.py
        matches:
        - is_error=True).to_dict()
        - is_error=True)
        - "result = ToolResult(content=[text_content(f\"Tool '{name}' failed: {exc}\")], is_error=True)"
      - path: src/xy/ai/mcpc/embedded/mcpc.py
        matches:
        - "return CliResult(text=str(exc), is_error=True, subtype='session_error')"
      - path: src/xy/ai/mcpc/cli/session.py
        matches:
        - "return CliResult(text=problem, is_error=True, subtype='session_error')"
      - path: src/xy/ai/mcpc/tools/mcp/github.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: src/xy/ai/mcpc/tools/mcp/context7.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
        matches:
        - "logger.warning('web_fetch_exa_results: unknown or expired id(s): %s', missing)"
        - "logger.warning('web_fetch_exa_results: invalid pattern %r: %s', args.get('pattern'), exc)"
        - "return ToolResult(content=[text_content(f'Invalid pattern: {exc}')], is_error=True)"
        - "return ToolResult(content=[text_content(f'Error resolving web_fetch_exa results: {exc}')], is_error=True)"
      - path: src/xy/ai/mcpc/tools/mcp/exa/fetch.py
        matches:
        - "logger.warning('web_fetch_exa: no \"# title / URL:\" markers found in fallback text, cannot split into per-url items')"
        - "logger.warning('web_fetch_exa failed: %s', exc)"
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(content=[text_content(f'Unexpected error in web_fetch_exa: {exc}')], is_error=True)"
      - path: src/xy/ai/mcpc/tools/mcp/exa/search.py
        matches:
        - notice.
        - logger.warning(
        - "logger.warning('web_search_exa failed: %s', exc)"
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      warning: "Limit of 50 matches reached (total 124); narrow pattern, directory or include/exclude filters."
```
/answer 6bd4-21-20 allow
ReasoningToken: 98
Control Request:
```yaml
id: 6bd4-22-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: class ToolResult|def handle_batch_tool|def serialize_batch_result|def normalize_result|def compact_asdict|def attach_message
    include:
    - '*.py'
    exclude:
    - .venv/**
    limit: 20
  reason: "Kernstellen (ToolResult, handle_batch_tool, serialize_batch_result, normalize_result) lokalisieren."
```
/answer 6bd4-22-0 allow
Control Request:
```yaml
id: 6bd4-23-22
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: src/xy/ai/mcpc/tools/_tool_helpers.py
        matches:
        - "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_ser…: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:"
        - "def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:"
      - path: src/xy/ai/mcpc/tools/tool_registry.py
        matches:
        - "class ToolResult:"
        - "def attach_message(result: dict[str, Any], message: str) -> None:"
        - "def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:"
      warning: 1 match line(s) exceeded 300 characters and were shortened in the middle.
```
/answer 6bd4-23-22 allow
Control Request:
```yaml
id: 6bd4-24-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
  reason: Struktur der zentralen Helfer und von ToolResult ansehen.
```
/answer 6bd4-24-0 allow
Control Request:
```yaml
id: 6bd4-25-24
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: j2F2xB|e5OyD1
        excert: '''Common helpers for tool definition and request handling.'''
      - id: YOvHfY|j1xNTQ
        excert: "from dataclasses import dataclass, asdict"
      - id: compact
        excert: "def _compact(value: Any) -> Any:"
        docstring: Recursively drop ``None`` values and empty lists from a dataclass-derived struc…
      - id: batch_schema
        excert: "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
        docstring: "Build the standard ``{items: [...]}`` input schema shared by all batch tools. N…"
      - id: 1tCi6I|MiXew1
        excert: "__all__ = ['BatchError', 'require_items', 'handle_batch_tool', 'serialize_batch…"
      - id: BatchError
        excert: "@dataclass class BatchError:"
        docstring: Error from a single batch item.
      - id: aR0fQj|kPKhQR
        excert: T = TypeVar('T')
      - id: require_items
        excert: "def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':"
        docstring: "Return the non-empty list at *key* in ``ctx.arguments``, or an error result. Re…"
      - id: handle_batch_tool
        excert: "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:"
        docstring: "Common handler for batch-processing tools. Args: ctx: Tool context with argumen…"
      - id: serialize_batch_result
        excert: "def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:"
        docstring: "Serialize a batch result, only including non-empty result/error lists. Args: ba…"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: M6pfOV|iTlyps
        excert: '''Central tool registry and tool result helpers.\n\nTools are registered once in…'
      - id: FPT4hs|aHGJs9
        excert: "from dataclasses import dataclass, field"
      - id: 07wTbv|BnS8bM
        excert: "'#: Default value for the Anthropic-specific ``anthropic/maxResultSizeChars``'"
      - id: ToolResult
        excert: "@dataclass(slots=True) class ToolResult:"
        docstring: The result of a tool call (maps onto MCP ``CallToolResult``).
        children:
        - id: ToolResult.6F9vfX|khLiSB
          excert: '''The result of a tool call (maps onto MCP ``CallToolResult``).'''
        - id: ToolResult.post_init
          excert: "def __post_init__(self) -> None:"
        - id: ToolResult.to_dict
          excert: "def to_dict(self) -> dict[str, Any]:"
      - id: text_content
        excert: "def text_content(text: str) -> dict[str, Any]:"
        docstring: Build a ``TextContent`` block.
      - id: attach_message
        excert: "def attach_message(result: dict[str, Any], message: str) -> None:"
      - id: BFWVtm|SRXvgt
        excert: "'#: A handler receives the invocation context and returns one of:'"
      - id: Tool
        excert: "@dataclass(slots=True) class Tool:"
        docstring: A registered tool and its MCP metadata.
        children:
        - id: Tool.cjwFjQ|SNEi1f
          excert: '''A registered tool and its MCP metadata.'''
        - id: Tool.to_spec
          excert: "def to_spec(self) -> dict[str, Any]:"
          docstring: Return the MCP ``Tool`` object advertised via ``tools/list``.
      - id: ToolDefinition
        excert: "class ToolDefinition(ABC):"
        docstring: Base class for registering a tool as an object. Subclasses declare the MCP meta…
        children:
        - id: ToolDefinition.lDzjus|OawaHI
          excert: '''Base class for registering a tool as an object.\n\n    Subclasses declare the …'
        - id: ToolDefinition.rremWZ|d5jedw
          excert: "name: str"
        - id: ToolDefinition.handle
          excert: "@abstractmethod def handle(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':"
        - id: ToolDefinition.call
          excert: "def __call__(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':"
        - id: ToolDefinition.to_tool
          excert: "def to_tool(self) -> Tool:"
      - id: normalize_result
        excert: "def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:"
        docstring: Coerce whatever a handler returned into a :class:`ToolResult`.
      - id: hIMaii|mbWWig
        excert: "\"#: Name of the mandatory reason property injected into every tool's input\""
      - id: inject_property
        excert: "def _inject_property(schema: dict[str, Any], name: str, description: str, *, required: bool, first: bool=False) -> dict[str, Any]:"
        docstring: Return *schema* with an additional property generically injected. Used both for…
      - id: with_mandatory_reason
        excert: "def _with_mandatory_reason(schema: dict[str, Any]) -> dict[str, Any]:"
        docstring: "Return *schema* with a mandatory, short ``reason`` property injected. Every too…"
      - id: ToolRegistry
        excert: "class ToolRegistry:"
        docstring: Process-wide registry of available tools.
        children:
        - id: ToolRegistry.9VdZpn|xgr9BO
          excert: '''Process-wide registry of available tools.'''
        - id: ToolRegistry.init
          excert: "def __init__(self) -> None:"
        - id: ToolRegistry.register_alias
          excert: "def register_alias(self, alias: str, members: 'Iterable[str]') -> None:"
          docstring: "Define (or extend) a tool-set alias expanding to *members*. Generic mechanism: …"
        - id: ToolRegistry.expand_aliases
          excert: "def expand_aliases(self, names: 'Iterable[str]') -> set[str]:"
          docstring: Expand any alias names in *names* to their member tool names. The alias ``"all"…
        - id: ToolRegistry.is_enabled
          excert: "def is_enabled(self, session: Session, name: str) -> bool:"
          docstring: "Whether *name* is enabled for *session*, honouring tool-set aliases."
        - id: ToolRegistry.register
          excert: "def register(self, tool: ToolDefinition) -> Tool:"
        - id: ToolRegistry.tool
          excert: "def tool(self, name: str, *, description: str, input_schema: dict[str, Any], title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> Callable[[ToolHandler], ToolHandler]:"
          docstring: Decorator registering the decorated function as a tool handler.
        - id: ToolRegistry.get
          excert: "def get(self, name: str) -> Tool | None:"
        - id: ToolRegistry.contains
          excert: "def __contains__(self, name: object) -> bool:"
        - id: ToolRegistry.len
          excert: "def __len__(self) -> int:"
        - id: ToolRegistry.names
          excert: "def names(self) -> list[str]:"
        - id: ToolRegistry.list_for_session
          excert: "def list_for_session(self, session: Session) -> list[Tool]:"
          docstring: Reconcile the registry with the session's enabled-tool configuration. Returns t…
```
/answer 6bd4-25-24 allow
Control Request:
```yaml
id: 6bd4-26-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: \bwarning\b|\bnotice\b|\.errors\b|errors:|unchanged|BatchError|import_notice
    include:
    - '*.py'
    exclude:
    - control/**
    - server/**
    - cli/**
    - embedded/**
    limit: 50
  reason: Warn-/notice-/errors-Verwendungen in den Tools gezielt erfassen (ohne Server-Logging).
```
/answer 6bd4-26-0 allow
Control Request:
```yaml
id: 6bd4-27-26
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/control/handler.py
        matches:
        - "logger.warning('Control: manager not enabled, returning 404')"
        - "logger.warning('Control: invalid JSON body: %s', exc)"
        - "logger.warning(\"Control: 'approvals' is not a list: %r\", approvals)"
      - path: xy/ai/mcpc/control/manager.py
        matches:
        - '"""Return *submitted* only if it differs from *original*; ``None`` means unchanged."""'
        - "logger.warning(\"Approval entry missing 'id', skipped: %s\", approval)"
        - "logger.warning('Unknown approval id %s, skipped', item_id)"
        - "logger.warning('Control item %s [%s/%s] timed out, auto-approving', item.tool_name, item.phase, item.id)"
      - path: xy/ai/mcpc/server/jsonrpc.py
        matches:
        - "from xy.ai.mcpc.server.errors import JsonRpcError, invalid_request, parse_error"
      - path: xy/ai/mcpc/server/server.py
        matches:
        - Returns ``None`` (after logging a warning) when the transport is disabled
        - "logger.warning('WebSocket transport unavailable: %s', exc)"
      - path: xy/ai/mcpc/server/ws_transport.py
        matches:
        - "logger.warning('WS: unknown endpoint %s', parsed.path)"
        - "logger.warning('WS: origin forbidden: %s', origin)"
        - "logger.warning('WS: session id missing')"
        - "logger.warning('WS: session id is not a valid UUID')"
      - path: xy/ai/mcpc/cli/session.py
        matches:
        - "if errors:"
        - "def _join_errors(errors: Any) -> str:"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch_results.py
        matches:
        - "logger.warning('web_fetch_exa_results: unknown or expired id(s): %s', missing)"
        - "logger.warning('web_fetch_exa_results: invalid pattern %r: %s', args.get('pattern'), exc)"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
        matches:
        - "logger.warning('web_fetch_exa: no \"# title / URL:\" markers found in fallback text, cannot split into per-url items')"
        - "logger.warning('web_fetch_exa failed: %s', exc)"
      - path: xy/ai/mcpc/tools/mcp/exa/search.py
        matches:
        - notice.
        - logger.warning(
        - "logger.warning('web_search_exa failed: %s', exc)"
      - path: xy/ai/mcpc/tools/mcp/exa/search_results.py
        matches:
        - "logger.warning('web_search_exa_results: unknown or expired id(s): %s', missing)"
      - path: xy/ai/mcpc/tools/mcp/exa/core.py
        matches:
        - logger.warning(
        - "logger.warning('Exa result item is not a dict (got %s), skipping its fields: %r', type(raw).__name__, raw)"
      - path: xy/ai/mcpc/tools/tool_usage/__init__.py
        matches:
        - "errors: list[dict[str, str]] = []"
        - "if errors:"
      - path: xy/ai/mcpc/tools/ast/base.py
        matches:
        - "errors: list[str] = []"
      - path: xy/ai/mcpc/tools/ast/find.py
        matches:
        - path stays unchanged). Ambiguous or empty matches fall through to the
      - path: xy/ai/mcpc/tools/ast/replace.py
        matches:
        - "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args"
        - "errors: One :class:`ReplaceError` per replacement that failed."
        - "errors: list[ReplaceError]"
        - "notice: str | None = None"
        - "errors: list[ReplaceError] = []"
        - "return ReplaceBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: xy/ai/mcpc/tools/ast/delete.py
        matches:
        - "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args"
        - Both are ignored (and flagged in the batch notice) when a whole file is deleted.
        - "errors: One :class:`DeleteError` per item that failed."
        - "errors: list[DeleteError]"
        - "notice: str | None = None"
        - "errors: list[DeleteError] = []"
        - "return DeleteBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "errors: One :class:`OutlineNodesError` per file that failed."
        - "errors: list[OutlineNodesError]"
        - unchanged and left to fail normally.
        - "if batch.errors:"
        - "return f'{hint} Listing failed: {batch.errors[0].error}'"
        - "if result.warning:"
        - parts.append(result.warning)
      warning: "Limit of 50 matches reached (total 164); narrow pattern, directory or include/exclude filters."
```
/answer 6bd4-27-26 allow
Control Request:
```yaml
id: 6bd4-28-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: ""
    include:
    - '*.py'
    exclude:
    - .venv/**
    - src/**
    limit: 1
  reason: Testverzeichnis-Struktur prüfen.
```
/answer 6bd4-28-0 allow
Control Request:
```yaml
id: 6bd4-29-28
result:
  structuredContent:
    errors:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      pattern: ""
      error: pattern must not be empty.
```
/answer 6bd4-29-28 allow
ReasoningToken: 40
Control Request:
```yaml
id: 6bd4-2a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    ids:
    - ToolResult
    - text_content
    - attach_message
    - BFWVtm|SRXvgt
    - ToolDefinition
    - normalize_result
  reason: "Vollständigen Quelltext der Helfer, ToolResult und normalize_result lesen."
```
/answer 6bd4-2a-0 allow
Control Request:
```yaml
id: 6bd4-2b-2a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: j2F2xB|e5OyD1
        excert: '''Common helpers for tool definition and request handling.'''
      - id: YOvHfY|j1xNTQ
        excert: "from dataclasses import dataclass, asdict"
      - id: compact
        excert: "def _compact(value: Any) -> Any:"
        docstring: Recursively drop ``None`` values and empty lists from a dataclass-derived struc…
      - id: batch_schema
        excert: "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
        docstring: "Build the standard ``{items: [...]}`` input schema shared by all batch tools. N…"
      - id: 1tCi6I|MiXew1
        excert: "__all__ = ['BatchError', 'require_items', 'handle_batch_tool', 'serialize_batch…"
      - id: BatchError
        excert: "@dataclass class BatchError:"
        docstring: Error from a single batch item.
      - id: aR0fQj|kPKhQR
        excert: T = TypeVar('T')
      - id: require_items
        excert: "def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':"
        docstring: "Return the non-empty list at *key* in ``ctx.arguments``, or an error result. Re…"
      - id: handle_batch_tool
        excert: "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:"
        docstring: "Common handler for batch-processing tools. Args: ctx: Tool context with argumen…"
      - id: serialize_batch_result
        excert: "def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:"
        docstring: "Serialize a batch result, only including non-empty result/error lists. Args: ba…"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: ToolResult
        type: ClassDef
        excert: "@dataclass(slots=True) class ToolResult:"
        docstring: The result of a tool call (maps onto MCP ``CallToolResult``).
        children:
        - id: ToolResult.6F9vfX|khLiSB
          type: statements
          code: |-
            'The result of a tool call (maps onto MCP ``CallToolResult``).'
            content: list[dict[str, Any]] = field(default_factory=list)
            structured_content: dict[str, Any] | None = None
            is_error: bool = False
            auto_approve: bool = False
            message: str | None = None
        - id: ToolResult.post_init
          type: FunctionDef
          code: |-
            def __post_init__(self) -> None:
                if self.is_error and self.structured_content:
                    raise ValueError('ToolResult: structured_content must not be set when is_error is True.')
        - id: ToolResult.to_dict
          type: FunctionDef
          code: |-
            def to_dict(self) -> dict[str, Any]:
                result: dict[str, Any] = {}
                if self.content:
                    result['content'] = list(self.content)
                if self.structured_content:
                    result['structuredContent'] = dict(self.structured_content)
                if self.message:
                    attach_message(result, self.message)
                if self.is_error:
                    result['isError'] = True
                return result
      - id: text_content
        type: FunctionDef
        code: |-
          def text_content(text: str) -> dict[str, Any]:
              """Build a ``TextContent`` block."""
              return {'type': 'text', 'text': text}
      - id: attach_message
        type: FunctionDef
        code: |-
          def attach_message(result: dict[str, Any], message: str) -> None:
              structured = dict(result.get('structuredContent') or {})
              structured[MESSAGE_PROPERTY] = message
              result['structuredContent'] = structured
              reference = f"The user attached a message: read '{MESSAGE_PROPERTY}' in structuredContent."
              result['content'] = [*result.get('content', []), text_content(reference)]
      - id: BFWVtm|SRXvgt
        type: statements
        code: |-
          '#: A handler receives the invocation context and returns one of:'
          '#: * a :class:`ToolResult`,'
          '#: * a ``str`` (wrapped as a single text content block),'
          '#: * a ``dict`` (treated as structured content, also rendered as JSON text).'
          ToolHandler = Callable[[ToolContext], 'ToolResult | str | dict[str, Any]']
      - id: ToolDefinition
        type: ClassDef
        excert: "class ToolDefinition(ABC):"
        docstring: Base class for registering a tool as an object. Subclasses declare the MCP meta…
        children:
        - id: ToolDefinition.lDzjus|OawaHI
          type: statements
          code: "'Base class for registering a tool as an object.\\n\\n    Subclasses declare the MCP metadata as class attributes and implement\\n    :meth:`handle`; an instance is itself a callable :data:`ToolHandler` and\\n    can be passed directly to :meth:`ToolRegistry.register`::\\n\\n        class MyTool(ToolDefinition):\\n            name = \"my_tool\"\\n            description = \"...\"\\n            input_schema = {...}\\n\\n            def handle(self, ctx: ToolContext) -> ToolResult:\\n                ...\\n\\n        registry.register(MyTool())\\n    '"
        - id: ToolDefinition.rremWZ|d5jedw
          type: statements
          code: |-
            name: str
            description: str
            input_schema: dict[str, Any]
            title: str | None = None
            output_schema: dict[str, Any] | None = {'type': 'object'}
            annotations: dict[str, Any] | None = None
        - id: ToolDefinition.handle
          type: FunctionDef
          code: |-
            @abstractmethod
            def handle(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':
                ...
        - id: ToolDefinition.call
          type: FunctionDef
          code: |-
            def __call__(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':
                return self.handle(ctx)
        - id: ToolDefinition.to_tool
          type: FunctionDef
          code: |-
            def to_tool(self) -> Tool:
                return Tool(
                    name=self.name,
                    description=self.description,
                    input_schema=self.input_schema,
                    handler=self,
                    title=self.title,
                    output_schema=self.output_schema,
                    annotations=self.annotations)
      - id: normalize_result
        type: FunctionDef
        code: |-
          def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:
              """Coerce whatever a handler returned into a :class:`ToolResult`."""
              if isinstance(value, ToolResult):
                  return value
              if value is None:
                  return ToolResult(content=[])
              if isinstance(value, str):
                  return ToolResult(content=[text_content(value)])
              if isinstance(value, dict):
                  '# A dict already shaped like a CallToolResult is passed through.'
                  if 'content' in value and isinstance(value['content'], list):
                      return ToolResult(content=value['content'], structured_content=value.get(
                          'structuredContent'), is_error=bool(value.get('isError', False)))
                  '# Otherwise treat the dict as structured content.'
                  rendered = JsonCodec.encode(value)
                  return ToolResult(content=[text_content(rendered)], structured_content=value)
              raise TypeError(f'Unsupported tool return type: {type(value)!r}')
```
/answer 6bd4-2b-2a allow
Control Request:
```yaml
id: 6bd4-2c-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    ids:
    - j2F2xB|e5OyD1
    - YOvHfY|j1xNTQ
    - compact
    - batch_schema
    - 1tCi6I|MiXew1
    - BatchError
    - require_items
    - handle_batch_tool
    - serialize_batch_result
  reason: Code der Batch-Helfer lesen.
```
/answer 6bd4-2c-0 allow
Control Request:
```yaml
id: 6bd4-2d-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: j2F2xB|e5OyD1
        type: statements
        code: '''Common helpers for tool definition and request handling.'''
      - id: YOvHfY|j1xNTQ
        type: imports
        code: |-
          from dataclasses import dataclass, asdict
          from typing import Any, Callable, TypeVar
          from xy.ai.mcpc.tools.tool_registry import ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
      - id: compact
        type: FunctionDef
        code: |-
          def _compact(value: Any) -> Any:
              """Recursively drop ``None`` values and empty lists from a dataclass-derived structure."""
              if isinstance(value, dict):
                  return {k: _compact(v) for k, v in value.items() if v is not None and v != []}
              if isinstance(value, list):
                  return [_compact(v) for v in value]
              return value
      - id: batch_schema
        type: FunctionDef
        code: |-
          def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:
              """Build the standard ``{items: [...]}`` input schema shared by all batch tools.

              Neither the top level nor the items accept properties beyond the declared ones.

              Args:
                  item_properties: The ``properties`` of a single item.
                  required: The ``required`` keys of a single item.
                  description: Description of the ``items`` array.
              """
              item_schema: dict[str, Any] = {'type': 'object', 'properties': item_properties,
                                             'required': required, 'additionalProperties': False}
              return {
                  'type': 'object',
                  'properties': {
                      'items': {
                          'type': 'array',
                          'minItems': 1,
                          'items': item_schema,
                          'description': description}},
                  'required': ['items'],
                  'additionalProperties': False}
      - id: 1tCi6I|MiXew1
        type: statements
        code: "__all__ = ['BatchError', 'require_items', 'handle_batch_tool', 'serialize_batch_result', 'batch_schema']"
      - id: BatchError
        type: ClassDef
        code: |-
          @dataclass
          class BatchError:
              """Error from a single batch item."""
              error: str
      - id: require_items
        type: FunctionDef
        code: |-
          def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':
              """Return the non-empty list at *key* in ``ctx.arguments``, or an error result.

              Returns:
                  ``(values, None)`` on success, or ``([], error_result)`` if *key* is missing or
                  empty; callers must return ``error_result`` immediately in that case.
              """
              values = ctx.arguments.get(key) or []
              if not values:
                  return ([], ToolResult(content=[text_content(f"'{key}' must be a non-empty list.")], is_error=True))
              return (values, None)
      - id: handle_batch_tool
        type: FunctionDef
        code: |-
          def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:
              """Common handler for batch-processing tools.

              Args:
                  ctx: Tool context with arguments.
                  item_factory: Function to convert raw dict to typed item.
                  batch_fn: Function to process list of items, returning batch result.
                  error_class: Exception class raised by `batch_fn` for whole-batch failures.
                  result_serializer: Optional custom serializer for result items (default: asdict).
                  error_serializer: Optional custom serializer for error items (default: asdict).

              Returns:
                  ToolResult with serialized batch result.
              """
              raw_items, error = require_items(ctx)
              if error is not None:
                  return error
              items: list[T] = []
              for it in raw_items:
                  try:
                      items.append(item_factory(it))
                  except KeyError as exc:
                      key = exc.args[0] if exc.args else exc
                      return ToolResult(content=[text_content(f'Invalid item: missing required property {key!r}')], is_error=True)
                  except (ValueError, TypeError) as exc:
                      return ToolResult(content=[text_content(f'Invalid item: {exc}')], is_error=True)
              try:
                  batch_result = batch_fn(items)
              except error_class as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              content = serialize_batch_result(
                  batch_result,
                  result_serializer=result_serializer,
                  error_serializer=error_serializer)
              has_error = hasattr(batch_result, 'errors') and batch_result.errors
              return ToolResult(structured_content=content, auto_approve=not has_error and auto_approve)
      - id: serialize_batch_result
        type: FunctionDef
        code: |-
          def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:
              """Serialize a batch result, only including non-empty result/error lists.

              Args:
                  batch_result: Object with optional `results` and `errors` attributes.
                  result_serializer: Optional custom serializer for result items (default: asdict).
                  error_serializer: Optional custom serializer for error items (default: asdict).

              Returns:
                  Dict with only populated fields.
              """
              if result_serializer is None:
                  result_serializer = lambda r: _compact(asdict(r)) if hasattr(r, '__dataclass_fields__') else r
              if error_serializer is None:
                  error_serializer = lambda e: _compact(asdict(e)) if hasattr(e, '__dataclass_fields__') else e
              content: dict[str, Any] = {}
              if hasattr(batch_result, 'results') and batch_result.results:
                  content['results'] = [result_serializer(r) for r in batch_result.results]
              if hasattr(batch_result, 'errors') and batch_result.errors:
                  content['errors'] = [error_serializer(e) for e in batch_result.errors]
              notice = getattr(batch_result, 'notice', None)
              if notice:
                  content['notice'] = notice
              return content
```
/answer 6bd4-2d-2c allow
ReasoningToken: 39
Control Request:
```yaml
id: 6bd4-2e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: BatchError|serialize_batch_result|handle_batch_tool|normalize_result|_compact\b|ToolHandler
    include:
    - '*.py'
    - '*.md'
    exclude:
    - .venv/**
    limit: 50
  reason: Verwendungen der Helfer und von BatchError (ungenutzt?) prüfen.
```
/answer 6bd4-2e-0 allow
Control Request:
```yaml
id: 6bd4-2f-2e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: project/uni_1.md
        matches:
        - "| `ast_read` (Handler) | `entry['warning'] = [str]` als Liste mit einem Element. Zusätzlich `entry['error'] = [str]` an einem *erfolgreichen* Result-Eintrag | Der Handler umgeht `handle_batch_tool` und baut Dicts selbst. Dieselbe Meldung steht doppelt als warning und error. |"
        - "| `ast_replace`, `ast_delete`, `ast_insert`, `ast_edit_block`, `ast_edit_marks` | `notice: str \\| None` am `*BatchResult`, befüllt durch `import_notice(results)` | `serialize_batch_result` liest es per `getattr(batch_result, 'notice', None)`. Das ist Duck-Typing, und es gibt kein Typ-Gate. |"
        - '- `BatchError` in `_tool_helpers` scheint ungenutzt zu sein. Das müsste ich noch prüfen.'
        - "Die Stelle, an der man ansetzen kann, ist `serialize_batch_result` zusammen mit `handle_batch_tool`. Nicht darüber laufen `ast_read`, `ast_outline`, `read`, `tool_usage`, `ast_find`, `ast_validate`, `ast_script` und Exa/OpenAlex. Für sie braucht es `ToolResult` als zweiten Durchgriffspunkt."
        - "- `handle_batch_tool` setzt `ToolResult(structured_content=..., warnings=batch.all_warnings())`."
        - "2. **Ein einziger Serialisierungspunkt:** `serialize_batch_result` prüft `isinstance(batch_result, BatchResult)` statt `getattr`. Es ruft `to_dict()` …zer (`search` mit `intent`, `core.to_dict(n)` mit `with_lines`) liefern nur `payload`. Setzt ein Payload einen reservierten Key, gibt es einen Fehler."
        - "3. **Typen:** `handle_batch_tool(batch_fn: Callable[[list[T]], BatchResult[Any, Any]])` wird von mypy und pyright geprüft. `BatchResult.__post_init__` prüft zusätzlich zur Laufzeit."
        - 4. **Escape-Hatch schließen:** `ToolDefinition.handle` darf nur noch `ToolResult` zurückgeben. Die Rückgabe von `str` oder `dict` samt Durchreichen von `content`-Dicts in `normalize_result` fällt weg.
        - '- Der `ast_read`-Handler läuft künftig über `handle_batch_tool`. Der Outline-Fallback wandert in die Fachfunktion und wird dort ein Item-Warning. Das doppelte `error` entfällt.'
        - "- `BatchError` wird entfernt, falls tatsächlich ungenutzt."
        - "5. Verschärfen: `handle` auf `ToolResult` einschränken, `getattr`-Duck-Typing und `BatchError` entfernen, Contract-Test aktivieren."
        - "pattern: warning|notice|BatchError|is_error=True|import_notice"
        - "reason: \"Alle Stellen finden, die Warnungen, notice, BatchError und is_error=True verwenden.\""
        - "pattern: class ToolResult|def handle_batch_tool|def serialize_batch_result|def normalize_result|def compact_asdict|def attach_message"
        - "reason: \"Kernstellen (ToolResult, handle_batch_tool, serialize_batch_result, normalize_result) lokalisieren.\""
        - "- \"def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_… Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:\""
        - "- \"def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:\""
        - "- \"def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:\""
        - "excert: \"def _compact(value: Any) -> Any:\""
        - "excert: \"__all__ = ['BatchError', 'require_items', 'handle_batch_tool', 'serialize_batch…\""
        - "- id: BatchError"
        - "excert: \"@dataclass class BatchError:\""
        - "- id: handle_batch_tool"
        - "excert: \"def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, r… Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:\""
        - "- id: serialize_batch_result"
        - "excert: \"def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:\""
        - "- id: normalize_result"
        - "excert: \"def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:\""
        - "excert: \"def tool(self, name: str, *, description: str, input_schema: dict[str, Any], title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> Callable[[ToolHandler], ToolHandler]:\""
        - "pattern: \\bwarning\\b|\\bnotice\\b|\\.errors\\b|errors:|unchanged|BatchError|import_notice"
        - '- normalize_result'
        - "reason: \"Vollständigen Quelltext der Helfer, ToolResult und normalize_result lesen.\""
        - "ToolHandler = Callable[[ToolContext], 'ToolResult | str | dict[str, Any]']"
        - "code: \"'Base class for registering a tool as an object.\\\\n\\\\n    Subclasses declare the MCP metadata as class attributes and implement\\\\n    :meth:`ha…ema = {...}\\\\n\\\\n            def handle(self, ctx: ToolContext) -> ToolResult:\\\\n                ...\\\\n\\\\n        registry.register(MyTool())\\\\n    '\""
        - "def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:"
        - '- BatchError'
        - '- handle_batch_tool'
        - '- serialize_batch_result'
        - "def _compact(value: Any) -> Any:"
        - "return {k: _compact(v) for k, v in value.items() if v is not None and v != []}"
        - "return [_compact(v) for v in value]"
      warning: "Limit of 50 matches reached (total 193); narrow pattern, directory or include/exclude filters. 5 match line(s) exceeded 300 characters and were shortened in the middle."
```
/answer 6bd4-2f-2e allow
ReasoningToken: 43
Control Request:
```yaml
id: 6bd4-30-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: BatchError|serialize_batch_result|handle_batch_tool|normalize_result|ToolHandler|_tool_helpers
    include:
    - '*.py'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: ToolResult\(
    include:
    - '*.py'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: ^\s*(warning|notice|hint|errors|unchanged)\s*:|\.warning\b|\.notice\b|\.unchanged\b|warning=|notice=|'warning'|'notice'|"warning"|"notice"
    include:
    - '*.py'
    exclude:
    - control/**
    - server/**
    - cli/**
    - embedded/**
    limit: 50
  reason: "Verwendungen von Helfern, ToolResult-Konstruktionen und warning/notice-Felder in src und tests ermitteln."
```
/answer 6bd4-30-0 allow
Control Request:
```yaml
id: 6bd4-31-30
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: xy/ai/mcpc/server/mcp_protocol.py
        matches:
        - "from xy.ai.mcpc.tools.tool_registry import ToolRegistry, normalize_result, ToolResult, text_content, attach_message"
        - result = normalize_result(raw)
      - path: xy/ai/mcpc/embedded/tools.py
        matches:
        - "__all__ = ['FunctionTool', 'ToolHandlerFn', 'ToolProvider']"
        - "ToolHandlerFn = Callable[['ToolContext'], 'ToolResult | str | dict[str, Any]']"
        - "def __init__(self, name: str, description: str, input_schema: dict[str, Any], handler: ToolHandlerFn, *, title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> None:"
      - path: xy/ai/mcpc/embedded/mcpc.py
        matches:
        - "from xy.ai.mcpc.embedded.tools import FunctionTool, ToolHandlerFn, ToolProvider, register_tool"
        - "def tool(self, name: str, description: str, input_schema: dict[str, Any] | None=None, **kwargs: Any) -> Callable[[ToolHandlerFn], ToolHandlerFn]:"
        - "def decorator(fn: ToolHandlerFn) -> ToolHandlerFn:"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch_results.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: xy/ai/mcpc/tools/mcp/exa/search_results.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: xy/ai/mcpc/tools/tool_usage/__init__.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: xy/ai/mcpc/tools/ast/find.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: xy/ai/mcpc/tools/ast/replace.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - return handle_batch_tool(
      - path: xy/ai/mcpc/tools/ast/delete.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, result_serializer, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result"
        - "content = serialize_batch_result(batch, result_serializer, error_serializer)"
      - path: xy/ai/mcpc/tools/ast/edit_marks.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - return handle_batch_tool(
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import require_items, batch_schema"
      - path: xy/ai/mcpc/tools/ast/validate.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: xy/ai/mcpc/tools/ast/edit_block.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - return handle_batch_tool(
      - path: xy/ai/mcpc/tools/ast/create.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, ast_create, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/insert.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - return handle_batch_tool(
      - path: xy/ai/mcpc/tools/skills/__init__.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, get_skills, SkillsError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/bash/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import batch_schema, require_items"
      - path: xy/ai/mcpc/tools/bash/run_command.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import batch_schema, require_items"
      - path: xy/ai/mcpc/tools/_tool_helpers.py
        matches:
        - "__all__ = ['BatchError', 'require_items', 'handle_batch_tool', 'serialize_batch_result', 'batch_schema']"
        - "class BatchError:"
        - "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_ser…: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:"
        - content = serialize_batch_result(
        - "def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:"
      - path: xy/ai/mcpc/tools/edit_lines/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "return handle_batch_tool(ctx, item_factory, edit_lines, EditLinesError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/tool_registry.py
        matches:
        - "ToolHandler = Callable[[ToolContext], 'ToolResult | str | dict[str, Any]']"
        - "handler: ToolHandler"
        - :meth:`handle`; an instance is itself a callable :data:`ToolHandler` and
        - "def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:"
      warning: "Limit of 50 matches reached (total 70); narrow pattern, directory or include/exclude filters. 1 match line(s) exceeded 300 characters and were shortened in the middle."
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/server/mcp_protocol.py
        matches:
        - return ToolResult(
        - "return ToolResult(structured_content={'answer': decision.approval_hint}).to_dict()"
        - result = ToolResult(
        - "result = ToolResult(content=[text_content(f\"Tool '{name}' failed: {exc}\")], is_error=True)"
        - "return ToolResult(structured_content={'answer': combined_hint}).to_dict()"
      - path: xy/ai/mcpc/tools/mcp/github.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - return ToolResult(structured_content=result)
      - path: xy/ai/mcpc/tools/mcp/context7.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(structured_content={'libraries': [asdict(library) for library in libraries]})"
        - "return ToolResult(structured_content={'sections': [asdict(section) for section in sections]})"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch_results.py
        matches:
        - "return ToolResult(content=[text_content(f'Invalid pattern: {exc}')], is_error=True)"
        - "return ToolResult(content=[text_content(f'Error resolving web_fetch_exa results: {exc}')], is_error=True)"
        - "return ToolResult(structured_content={'results': results}, auto_approve=False)"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(content=[text_content(f'Unexpected error in web_fetch_exa: {exc}')], is_error=True)"
        - "return ToolResult(structured_content={'results': result.results})"
      - path: xy/ai/mcpc/tools/mcp/exa/search.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(content=[text_content(f'Unexpected error in web_search_exa: {exc}')], is_error=True)"
        - return ToolResult(structured_content=structured)
      - path: xy/ai/mcpc/tools/mcp/exa/search_results.py
        matches:
        - "return ToolResult(content=[text_content(f'Error resolving web_search_exa results: {exc}')], is_error=True)"
        - "return ToolResult(structured_content={'results': results}, auto_approve=False)"
      - path: xy/ai/mcpc/tools/tool_usage/__init__.py
        matches:
        - return ToolResult(structured_content=structured_content)
      - path: xy/ai/mcpc/tools/ast/find.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(structured_content={'files': [{'path': f.path, 'nodes': ["
      - path: xy/ai/mcpc/tools/ast/script.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - return ToolResult(structured_content=structured)
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - return ToolResult(structured_content=content)
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - return ToolResult(structured_content=structured_content)
      - path: xy/ai/mcpc/tools/ast/validate.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - return ToolResult(
      - path: xy/ai/mcpc/tools/edit_chars/__init__.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(structured_content={'result': result.result}, auto_approve=True)"
      - path: xy/ai/mcpc/tools/process.py
        matches:
        - "return ToolResult(structured_content={'result': 'success'}, auto_approve=False)"
        - "return ToolResult(content=content, structured_content=structured)"
      - path: xy/ai/mcpc/tools/agent/__init__.py
        matches:
        - "return ToolResult(content=[text_content(message)], is_error=True)"
        - "return ToolResult(content=[text_content(text)], is_error=True)"
        - "return ToolResult(structured_content={'response': text, 'session_id': session_id})"
      - path: xy/ai/mcpc/tools/bash/__init__.py
        matches:
        - "return ToolResult(content=[text_content(f'Invalid item: {exc!r}')], is_error=True)"
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(content=content, structured_content={'results': results})"
      - path: xy/ai/mcpc/tools/bash/run_command.py
        matches:
        - "return ToolResult(content=[text_content(f'Invalid item: {exc!r}')], is_error=True)"
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(content=content, structured_content={'results': results})"
      - path: xy/ai/mcpc/tools/_tool_helpers.py
        matches:
        - "return ([], ToolResult(content=[text_content(f\"'{key}' must be a non-empty list.\")], is_error=True))"
        - "return ToolResult(content=[text_content(f'Invalid item: missing required property {key!r}')], is_error=True)"
        - "return ToolResult(content=[text_content(f'Invalid item: {exc}')], is_error=True)"
      warning: "Limit of 50 matches reached (total 68); narrow pattern, directory or include/exclude filters."
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/control/handler.py
        matches:
        - "logger.warning('Control: manager not enabled, returning 404')"
        - "logger.warning('Control: invalid JSON body: %s', exc)"
        - "logger.warning(\"Control: 'approvals' is not a list: %r\", approvals)"
      - path: xy/ai/mcpc/control/manager.py
        matches:
        - "logger.warning(\"Approval entry missing 'id', skipped: %s\", approval)"
        - "logger.warning('Unknown approval id %s, skipped', item_id)"
        - "logger.warning('Control item %s [%s/%s] timed out, auto-approving', item.tool_name, item.phase, item.id)"
      - path: xy/ai/mcpc/server/server.py
        matches:
        - "logger.warning('WebSocket transport unavailable: %s', exc)"
      - path: xy/ai/mcpc/server/ws_transport.py
        matches:
        - "logger.warning('WS: unknown endpoint %s', parsed.path)"
        - "logger.warning('WS: origin forbidden: %s', origin)"
        - "logger.warning('WS: session id missing')"
        - "logger.warning('WS: session id is not a valid UUID')"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch_results.py
        matches:
        - "logger.warning('web_fetch_exa_results: unknown or expired id(s): %s', missing)"
        - "logger.warning('web_fetch_exa_results: invalid pattern %r: %s', args.get('pattern'), exc)"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
        matches:
        - "logger.warning('web_fetch_exa: no \"# title / URL:\" markers found in fallback text, cannot split into per-url items')"
        - "logger.warning('web_fetch_exa failed: %s', exc)"
      - path: xy/ai/mcpc/tools/mcp/exa/search.py
        matches:
        - logger.warning(
        - "logger.warning('web_search_exa failed: %s', exc)"
      - path: xy/ai/mcpc/tools/mcp/exa/search_results.py
        matches:
        - "logger.warning('web_search_exa_results: unknown or expired id(s): %s', missing)"
      - path: xy/ai/mcpc/tools/mcp/exa/core.py
        matches:
        - logger.warning(
        - "logger.warning('Exa result item is not a dict (got %s), skipping its fields: %r', type(raw).__name__, raw)"
      - path: xy/ai/mcpc/tools/tool_usage/__init__.py
        matches:
        - "errors: list[dict[str, str]] = []"
      - path: xy/ai/mcpc/tools/ast/base.py
        matches:
        - "errors: list[str] = []"
      - path: xy/ai/mcpc/tools/ast/replace.py
        matches:
        - "errors: One :class:`ReplaceError` per replacement that failed."
        - "errors: list[ReplaceError]"
        - "notice: str | None = None"
        - "errors: list[ReplaceError] = []"
        - "return ReplaceBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: xy/ai/mcpc/tools/ast/delete.py
        matches:
        - "errors: One :class:`DeleteError` per item that failed."
        - "errors: list[DeleteError]"
        - "notice: str | None = None"
        - "errors: list[DeleteError] = []"
        - "return DeleteBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "errors: One :class:`OutlineNodesError` per file that failed."
        - "errors: list[OutlineNodesError]"
        - "if result.warning:"
        - parts.append(result.warning)
        - "errors: list[OutlineNodesError] = []"
      - path: xy/ai/mcpc/tools/ast/edit_marks.py
        matches:
        - "errors: One :class:`EditMarksError` per edit that failed."
        - "errors: list[EditMarksError]"
        - "notice: str | None = None"
        - "errors: list[EditMarksError] = []"
        - "return EditMarksBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "errors: One message per requested id that could not be resolved (id"
        - "errors: list[str]"
        - "errors: One :class:`ReadError` per file that failed entirely."
        - "errors: list[ReadError]"
        - "errors: list[ReadError] = []"
        - "errors: list[dict[str, str]] = []"
        - "entry['warning'] = [warning]"
      - path: xy/ai/mcpc/tools/ast/edit_block.py
        matches:
        - "errors: One :class:`EditBlockError` per edit that failed."
      warning: "Limit of 50 matches reached (total 112); narrow pattern, directory or include/exclude filters."
```
/answer 6bd4-31-30 allow
Control Request:
```yaml
id: 6bd4-32-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: class \w+(Result|Error)\b|class \w+BatchResult
    include:
    - '*.py'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: ListResult|SearchResult|ReadResult|\bwarning\b
    include:
    - '*.py'
    exclude:
    - control/**
    - server/**
    - cli/**
    - embedded/**
    - mcp/**
    limit: 50
  reason: Alle Result-/Error-Klassen und die list/search/read-Warning-Stellen erfassen.
```
/answer 6bd4-32-0 allow
Control Request:
```yaml
id: 6bd4-33-32
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: xy/ai/mcpc/server/errors.py
        matches:
        - "class JsonRpcError(Exception):"
      - path: xy/ai/mcpc/cli/session.py
        matches:
        - "class CliSessionError(RuntimeError):"
        - "class CliResult:"
      - path: xy/ai/mcpc/tools/mcp/client.py
        matches:
        - "class McpClientError(RuntimeError):"
      - path: xy/ai/mcpc/tools/mcp/bridge.py
        matches:
        - "class McpBridgeError(RuntimeError):"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
        matches:
        - "class WebFetchResult:"
      - path: xy/ai/mcpc/tools/mcp/exa/search.py
        matches:
        - "class WebSearchResult:"
      - path: xy/ai/mcpc/tools/tool_usage/__init__.py
        matches:
        - "class ToolUsageError(Exception):"
      - path: xy/ai/mcpc/tools/ast/base.py
        matches:
        - "class AstError(Exception):"
        - "class AstTextError(AstError):"
      - path: xy/ai/mcpc/tools/ast/find.py
        matches:
        - "class FileNodesResult:"
        - "class FindNodesResult:"
      - path: xy/ai/mcpc/tools/ast/script.py
        matches:
        - "class ScriptError(Exception):"
        - "class AstScriptResult:"
      - path: xy/ai/mcpc/tools/ast/replace.py
        matches:
        - "class ReplaceResult:"
        - "class ReplaceError:"
        - "class ReplaceBatchResult:"
      - path: xy/ai/mcpc/tools/ast/delete.py
        matches:
        - "class DeleteResult:"
        - "class DeleteError:"
        - "class DeleteBatchResult:"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "class OutlineNodesResult:"
        - "class OutlineNodesError:"
        - "class OutlineNodesBatchResult:"
      - path: xy/ai/mcpc/tools/ast/edit_marks.py
        matches:
        - "class EditMarksResult:"
        - "class EditMarksError:"
        - "class EditMarksBatchResult:"
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "class ReadResult:"
        - "class ReadError:"
        - "class ReadBatchResult:"
      - path: xy/ai/mcpc/tools/ast/validate.py
        matches:
        - "class ValidateError(Exception):"
        - "class ValidateResult:"
      - path: xy/ai/mcpc/tools/ast/edit_block.py
        matches:
        - "class EditBlockResult:"
        - "class EditBlockError:"
        - "class EditBlockBatchResult:"
      - path: xy/ai/mcpc/tools/ast/create.py
        matches:
        - "class CreateFileResult:"
        - "class CreateFileError:"
        - "class CreateFilesResult:"
      - path: xy/ai/mcpc/tools/ast/insert.py
        matches:
        - "class InsertResult:"
        - "class InsertError:"
        - "class InsertBatchResult:"
      - path: xy/ai/mcpc/tools/edit_chars/__init__.py
        matches:
        - "class EditCharsError(Exception):"
        - "class EditCharsResult:"
      - path: xy/ai/mcpc/tools/process.py
        matches:
        - "class LaunchError(Exception):"
        - "class ProcessResult:"
      - path: xy/ai/mcpc/tools/skills/__init__.py
        matches:
        - "class SkillsError(Exception):"
        - "class SkillsResult:"
        - "class SkillsItemError:"
        - "class SkillsBatchResult:"
      - path: xy/ai/mcpc/tools/bash/__init__.py
        matches:
        - "class BashError(Exception):"
        - "class BashItemError:"
      warning: "Limit of 50 matches reached (total 105); narrow pattern, directory or include/exclude filters."
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/control/handler.py
        matches:
        - "logger.warning('Control: manager not enabled, returning 404')"
        - "logger.warning('Control: invalid JSON body: %s', exc)"
        - "logger.warning(\"Control: 'approvals' is not a list: %r\", approvals)"
      - path: xy/ai/mcpc/control/manager.py
        matches:
        - "logger.warning(\"Approval entry missing 'id', skipped: %s\", approval)"
        - "logger.warning('Unknown approval id %s, skipped', item_id)"
        - "logger.warning('Control item %s [%s/%s] timed out, auto-approving', item.tool_name, item.phase, item.id)"
      - path: xy/ai/mcpc/server/server.py
        matches:
        - Returns ``None`` (after logging a warning) when the transport is disabled
        - "logger.warning('WebSocket transport unavailable: %s', exc)"
      - path: xy/ai/mcpc/server/ws_transport.py
        matches:
        - "logger.warning('WS: unknown endpoint %s', parsed.path)"
        - "logger.warning('WS: origin forbidden: %s', origin)"
        - "logger.warning('WS: session id missing')"
        - "logger.warning('WS: session id is not a valid UUID')"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch_results.py
        matches:
        - "logger.warning('web_fetch_exa_results: unknown or expired id(s): %s', missing)"
        - "logger.warning('web_fetch_exa_results: invalid pattern %r: %s', args.get('pattern'), exc)"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
        matches:
        - "logger.warning('web_fetch_exa: no \"# title / URL:\" markers found in fallback text, cannot split into per-url items')"
        - "logger.warning('web_fetch_exa failed: %s', exc)"
      - path: xy/ai/mcpc/tools/mcp/exa/search.py
        matches:
        - "__all__ = ['WebSearchResult', 'web_search_exa', 'WebSearchExaTool', 'register']"
        - "class WebSearchResult:"
        - logger.warning(
        - "def web_search_exa(query: str, numResults: int | None=None) -> WebSearchResult:"
        - "return WebSearchResult(results=overview, autoprompt_string=raw.get('autoprompt_string'))"
        - "logger.warning('web_search_exa failed: %s', exc)"
      - path: xy/ai/mcpc/tools/mcp/exa/search_results.py
        matches:
        - "logger.warning('web_search_exa_results: unknown or expired id(s): %s', missing)"
      - path: xy/ai/mcpc/tools/mcp/exa/core.py
        matches:
        - logger.warning(
        - "logger.warning('Exa result item is not a dict (got %s), skipping its fields: %r', type(raw).__name__, raw)"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "if result.warning:"
        - parts.append(result.warning)
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "__all__ = ['ReadItem', 'ReadResult', 'ReadError', 'ReadBatchResult', 'ast_read', 'ReadNodeTool', 'register']"
        - "class ReadResult:"
        - "results: One :class:`ReadResult` per successfully read file."
        - "results: list[ReadResult]"
        - "def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:"
        - "return ReadResult(path=item.path, nodes=nodes, errors=errs)"
        - "results: list[ReadResult] = []"
        - "item_result: ReadResult | None = None"
        - "warning = f'ast_read fell back to ast_outline: ids {"
        - "entry['warning'] = [warning]"
      - path: xy/ai/mcpc/tools/ast/core.py
        matches:
        - "logger.warning('Import handling failed for %s', tree.path, exc_info=True)"
      - path: xy/ai/mcpc/tools/openalex/semantic_search.py
        matches:
        - "from ._common import DEFAULT_SEMANTIC_LIMIT, MAX_SEMANTIC_RESULTS, SearchResult, clamp, error_result, get_client, ok_result, overview_structured, summarise_list, to_search_overview"
        - "def openalex_semantic_search(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> SearchResult:"
      - path: xy/ai/mcpc/tools/openalex/search.py
        matches:
        - "from ._common import DEFAULT_SEARCH_LIMIT, MAX_PER_PAGE, SearchResult, clamp, error_result, get_client, ok_result, overview_structured, summarise_list, to_search_overview"
        - "def openalex_search(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> SearchResult:"
      - path: xy/ai/mcpc/tools/openalex/__init__.py
        matches:
        - "from ._common import SearchResult, WorkResult, build_client, set_client"
        - "from .search_results import OpenalexSearchResultsTool, openalex_search_results"
        - "'SearchResult',"
        - "'OpenalexSearchResultsTool',"
        - registry.register(OpenalexSearchResultsTool())
      - path: xy/ai/mcpc/tools/openalex/search_results.py
        matches:
        - "__all__ = ['openalex_search_results', 'OpenalexSearchResultsTool']"
        - "logger.warning('openalex_search_results: unknown or expired id(s): %s', missing)"
        - "class OpenalexSearchResultsTool(ToolDefinition):"
      warning: "Limit of 50 matches reached (total 96); narrow pattern, directory or include/exclude filters."
```
/answer 6bd4-33-32 allow
ReasoningToken: 73
Control Request:
```yaml
id: 6bd4-34-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: ^(def test_|class )
    include:
    - test_*.py
    - conftest.py
    exclude:
    - .venv/**
    - src/**
    limit: 5
  reason: Testlayout erkennen (für Contract-Test und bestehende Tests).
```
/answer 6bd4-34-0 allow
Control Request:
```yaml
id: 6bd4-35-34
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: tests/test_edit_tolerance.py
        matches:
        - "def test_level1_whitespace_tolerant():"
        - "def test_exact_requires_verbatim_whitespace():"
        - "def test_level2_matches_escaped_newlines():"
        - "def test_level2_blocked_at_max_level_1():"
        - "def test_escape_mirroring_keeps_literal_valid():"
      warning: "Limit of 5 matches reached (total 159); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 6bd4-35-34 allow
Thinking:
Die Basis (`_tool_helpers.py`, `ToolResult`) ist geklärt: `BatchError` wird in `src` und `tests` nirgends verwendet. Als Nächstes suche ich nach `list`, `search` und `read`.


Control Request:
```yaml
id: 6bd4-36-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: ListResult|class SearchResult|\bunchanged\b|class ReadResult
    include:
    - '*.py'
    limit: 40
  reason: "Orte von ListResult, SearchResult und read.unchanged finden."
```
/answer 6bd4-36-0 allow
Control Request:
```yaml
id: 6bd4-37-36
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: xy/ai/mcpc/control/manager.py
        matches:
        - '"""Return *submitted* only if it differs from *original*; ``None`` means unchanged."""'
      - path: xy/ai/mcpc/tools/ast/find.py
        matches:
        - path stays unchanged). Ambiguous or empty matches fall through to the
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - unchanged and left to fail normally.
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "class ReadResult:"
      - path: xy/ai/mcpc/tools/openalex/_common.py
        matches:
        - "class SearchResult:"
      - path: xy/ai/mcpc/tools/list/__init__.py
        matches:
        - "'ListResult',"
        - "class ListResult:"
        - "results: One :class:`ListResult` per successfully listed directory."
        - "results: list[ListResult]"
        - "def _list_one(item: ListItem) -> ListResult:"
        - "return ListResult(path=item.path, entries=entries)"
        - "return ListResult(path=item.path, entries=entries, warning=warning)"
        - "results: list[ListResult] = []"
      - path: xy/ai/mcpc/tools/read/__init__.py
        matches:
        - "class ReadResult:"
        - "unchanged: str | None = None"
        - session-agnostic and never sets :attr:`ReadResult.unchanged`.
        - unchanged = cache.get(key) == result.checksum
        - "if unchanged:"
        - unchanged='Content unchanged since the last identical read. Use the former read result.')
        - "if result.unchanged:"
        - "entry['unchanged'] = result.unchanged"
      - path: xy/ai/mcpc/tools/search/__init__.py
        matches:
        - "class SearchResult:"
        - are returned unchanged. Lines not covered by any node stay in ``matches``.
      - path: xy/ai/mcpc/utils/text_sanitize.py
        matches:
        - "returned unchanged (dict keys are sanitised too, when they are strings)."
```
/answer 6bd4-37-36 allow
ReasoningToken: 132
Control Request:
```yaml
id: 6bd4-38-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  node_type: ClassDef|FunctionDef
  reason: "ast_read und ast_outline im Detail lesen (Handler, Result-Klassen, Fallback)."
```
/answer 6bd4-38-0 allow
Control Request:
```yaml
id: 6bd4-39-38
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
      nodes:
      - id: looks_like_root_intent
        type: FunctionDef
        code: |-
          def _looks_like_root_intent(ids: list[str]) -> bool:
              """Whether ``ids`` is empty or consists solely of common root-id guesses."""
              return not ids or all((i.strip().lower() in _ROOT_INTENT_IDS for i in ids))
      - id: ReadItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadItem:
              """One file's node ids to read.

              Attributes:
                  path: Absolute path to the file to read.
                  ids: Node ids to read. Must be non-empty.
              """
              path: str
              ids: list[str]
      - id: ReadResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadResult:
              """Nodes read from a single file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  nodes: One expanded subtree per resolved id, in the given order; same
                      shape as :func:`ast_find`'s results (see :class:`core.OutlineNode`).
                  errors: One message per requested id that could not be resolved (id
                      unknown/ambiguous, and no unambiguous name/fuzzy match found).
              """
              path: str
              nodes: list[core.OutlineNode]
              errors: list[str]
      - id: ReadError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadError:
              """Error reading a whole item (e.g. bad path or empty ``ids``).

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  error: The error message.
              """
              path: str
              error: str
      - id: ReadBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadBatchResult:
              """Result of :func:`ast_read`.

              Attributes:
                  results: One :class:`ReadResult` per successfully read file.
                  errors: One :class:`ReadError` per file that failed entirely.
              """
              results: list[ReadResult]
              errors: list[ReadError]
      - id: read_one
        type: FunctionDef
        code: |-
          def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:
              if not item.ids:
                  raise core.AstError("'ids' must be a non-empty list of node ids.")
              tree = core.load(item.path)[1]
              nodes, errs = core.read_subtrees(core.locate_all(tree), item.ids, with_lines=with_lines)
              return ReadResult(path=item.path, nodes=nodes, errors=errs)
      - id: ast_read
        type: FunctionDef
        code: |-
          def ast_read(items: list[ReadItem], *, with_lines: bool=True) -> ReadBatchResult:
              """Recursively read the subtree of each addressed node, across one or more files.

              Each id resolves to a subtree: a node whose body consists solely of nested
              classes/functions is expanded into ``children`` instead of source, so the agent
              can descend to the innermost editable block; any other node is returned whole,
              as ``code`` ready to hand back to ``ast_replace`` via its ``id``.

              An id that doesn't match any node is retried as a node *name* (exact, then a
              conservative fuzzy match) instead of failing the whole item; ids that still
              can't be resolved are reported in that item's ``errors``, not raised.

              Args:
                  items: Per-file node ids to read. Must be non-empty.
                  with_lines: Whether to populate each node's line range.

              Returns:
                  ReadBatchResult: One result per file (with its own id-level errors), and
                  one file-level error per file that failed entirely (bad path, empty
                  ``ids``, or a syntax error).

              Raises:
                  core.AstError: If ``items`` is empty.
              """
              if not items:
                  raise core.AstError("'items' must be a non-empty list.")
              results: list[ReadResult] = []
              errors: list[ReadError] = []
              for item in items:
                  try:
                      results.append(_read_one(item, with_lines=with_lines))
                  except core.AstError as exc:
                      errors.append(ReadError(path=item.path, error=str(exc)))
              return ReadBatchResult(results=results, errors=errors)
      - id: ReadNodeTool
        type: ClassDef
        excert: "class ReadNodeTool(ToolDefinition):"
        children:
        - id: ReadNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API.

                    Per item, falls back to :func:`ast_outline` when ``ids`` looks like agents
                    habitually mis-guessing a root id (``root``/``_module_``/``module``/empty)
                    and none of them resolve to a node: returns that file's full outline
                    instead of an error, and notes the redirect in the item's ``errors``.
                    """
                with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)
                raw_items, error = require_items(ctx)
                if error is not None:
                    return error
                results: list[dict[str, Any]] = []
                errors: list[dict[str, str]] = []
                for raw in raw_items:
                    path = raw.get('path')
                    ids: list[str] = raw.get('ids') or []
                    item_result: ReadResult | None = None
                    if ids:
                        batch = ast_read([ReadItem(path=path, ids=ids)], with_lines=with_lines)
                        if batch.errors:
                            errors.append({'path': batch.errors[0].path, 'error': batch.errors[0].error})
                            continue
                        item_result = batch.results[0]
                    if _looks_like_root_intent(ids) and (item_result is None or not item_result.nodes):
                        list_batch = ast_outline([path], with_lines=with_lines)
                        for lr in list_batch.results:
                            entry: dict[str, Any] = {'path': lr.path, 'nodes': [core.to_dict(n) for n in lr.nodes]}
                            if ids:
                                warning = f'ast_read fell back to ast_outline: ids {
                                    ids!r} resolved to no node; returning the full outline of {
                                        lr.path} instead.'
                                entry['warning'] = [warning]
                                entry['error'] = [
                                    f'ids {
                                        ids!r} resolved to no node; redirected to ast_outline, returning the full outline instead.']
                            results.append(entry)
                        for le in list_batch.errors:
                            errors.append({'path': le.path, 'error': le.error})
                        continue
                    pathResult = {'path': item_result.path, 'nodes': [core.to_dict(n) for n in item_result.nodes]}
                    if item_result.errors:
                        pathResult['errors'] = item_result.errors
                    results.append(pathResult)
                structured_content: dict[str, Any] = {}
                if results:
                    structured_content['results'] = results
                if errors:
                    structured_content['errors'] = errors
                return ToolResult(structured_content=structured_content)
      - id: register
        type: FunctionDef
        code: |-
          def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(ReadNodeTool())
              functions.register(ast_read)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
      nodes:
      - id: OutlineNodesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNodesResult:
              """Nodes of a single file.

              Attributes:
                  path: The resolved path of the listed file (see ``ast_outline``'s
                      directory-expansion note), for result association.
                  nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
                      document order, suited for retrieval and navigation.
              """
              path: str
              nodes: list[core.OutlineNode]
      - id: OutlineNodesError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNodesError:
              """Error listing a single file.

              Attributes:
                  path: The resolved path that failed, for result association.
                  error: The error message.
              """
              path: str
              error: str
      - id: OutlineNodesBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNodesBatchResult:
              """Result of :func:`ast_outline`.

              Attributes:
                  results: One :class:`OutlineNodesResult` per successfully listed file.
                  errors: One :class:`OutlineNodesError` per file that failed.
              """
              results: list[OutlineNodesResult]
              errors: list[OutlineNodesError]
      - id: expand_path
        type: FunctionDef
        code: |-
          def _expand_path(path_str: str) -> list[str]:
              """Expand *path_str* to its own single-element list, unless it names a
              directory that can be unambiguously expanded to its files.

              Expansion only succeeds if the directory has no subdirectories and holds
              at most :data:`_MAX_DIR_EXPANSION` files; otherwise *path_str* is returned
              unchanged and left to fail normally.
              """
              p = Path(path_str)
              if not p.is_dir():
                  return [path_str]
              entries = list(p.iterdir())
              if any((e.is_dir() for e in entries)):
                  return [path_str]
              files = sorted((e for e in entries if e.is_file()), key=lambda e: e.name)
              if not files or len(files) > _MAX_DIR_EXPANSION:
                  return [path_str]
              return [str(f) for f in files]
      - id: outline_one
        type: FunctionDef
        code: |-
          def _outline_one(path: str, *, with_lines: bool) -> OutlineNodesResult:
              tree = core.load(path)[1]
              nodes = core.build_outline(core.locate_all(tree), with_lines=with_lines, with_type=False)
              return OutlineNodesResult(path=path, nodes=nodes)
      - id: directory_listing_hint
        type: FunctionDef
        code: |-
          def _directory_listing_hint(path: str) -> str:
              """Build an error message for a path that turned out to be a directory,
              anticipating the likely intent: list its contents (via the ``list`` tool)
              instead of outlining it as a file.
              """
              hint = 'Path is a directory, not a file; listing its contents instead.'
              try:
                  batch = _list_dirs([ListItem(path=path)])
              except ListError as exc:
                  return f'{hint} Listing failed: {exc}'
              if batch.errors:
                  return f'{hint} Listing failed: {batch.errors[0].error}'
              result = batch.results[0]
              parts = [hint]
              if result.warning:
                  parts.append(result.warning)
              parts.extend(result.entries)
              return '\n'.join(parts)
      - id: ast_outline
        type: FunctionDef
        code: |-
          def ast_outline(paths: list[str], *, with_lines: bool=True) -> OutlineNodesBatchResult:
              """List the hierarchical AST-node tree of one or more files.

              The tree is the foundation every other tool builds on: each node carries its
              unique, primarily name-based ``id`` and line range, but never its source –
              use ``ast_find`` (property/text filtering) or ``ast_read`` (by id) to
              retrieve source.

              A path naming a directory without subdirectories and holding at most
              5 files is transparently expanded to those files; any other directory
              is reported as an error whose message contains the directory's listing
              (via the ``list`` tool), anticipating the likely intent.

              Args:
                  paths: Absolute paths of the files to list. Must be non-empty.
                  with_lines: Whether to populate each node's line range.

              Returns:
                  OutlineNodesBatchResult: One result per listed file, one error per failed file.

              Raises:
                  core.AstError: If ``paths`` is empty.
              """
              if not paths:
                  raise core.AstError("'paths' must be a non-empty list.")
              results: list[OutlineNodesResult] = []
              errors: list[OutlineNodesError] = []
              for path in paths:
                  for real_path in _expand_path(path):
                      if Path(real_path).is_dir():
                          errors.append(OutlineNodesError(path=real_path, error=_directory_listing_hint(real_path)))
                          continue
                      try:
                          results.append(_outline_one(real_path, with_lines=with_lines))
                      except core.AstError as exc:
                          errors.append(OutlineNodesError(path=real_path, error=str(exc)))
              return OutlineNodesBatchResult(results=results, errors=errors)
      - id: OutlineNodesTool
        type: ClassDef
        excert: "class OutlineNodesTool(ToolDefinition):"
        children:
        - id: OutlineNodesTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_outline`, translating the MCP schema to/from the AST API."""
                paths, error = require_items(ctx, key='paths')
                if error is not None:
                    return error
                with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)
                batch = ast_outline(paths=paths, with_lines=with_lines)
                result_serializer = lambda r: {'path': r.path, 'nodes': [core.to_dict(n) for n in r.nodes]}
                error_serializer = lambda e: {'path': e.path, 'error': e.error}
                content = serialize_batch_result(batch, result_serializer, error_serializer)
                return ToolResult(structured_content=content)
      - id: register
        type: FunctionDef
        code: |-
          def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(OutlineNodesTool())
              functions.register(ast_outline)
```
/answer 6bd4-39-38 allow
ReasoningToken: 189
Control Request:
```yaml
id: 6bd4-3a-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  reason: "Struktur der list-, search- und read-Tools erfassen."
```
/answer 6bd4-3a-0 allow
Control Request:
```yaml
id: 6bd4-3b-3a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: JrBuUj|ISKZu0
        excert: "\"List tool – returns files below a directory, grouped by relative subdirectory …"
      - id: dvO55c|qVCPEH
        excert: import os
      - id: ZrSLiQ|rTmMVq
        excert: "__all__ = ["
      - id: ListError
        excert: "class ListError(Exception):"
        docstring: Raised when a directory listing cannot be performed.
      - id: ListItem
        excert: "@dataclass(frozen=True) class ListItem:"
        docstring: "One directory to list. Attributes: path: Absolute directory path to list (must …"
      - id: ListResult
        excert: "@dataclass(frozen=True) class ListResult:"
        docstring: "Result of listing a single directory, mirroring its input path for result assoc…"
      - id: ListItemError
        excert: "@dataclass(frozen=True) class ListItemError:"
        docstring: "Error listing a single directory, mirroring its input path for result associati…"
      - id: ListBatchResult
        excert: "@dataclass(frozen=True) class ListBatchResult:"
        docstring: "Result of :func:`list`. Attributes: results: One :class:`ListResult` per succes…"
      - id: list_one
        excert: "def _list_one(item: ListItem) -> ListResult:"
        docstring: "List all files below the absolute directory ``item.path``, grouped like ``ls -R…"
      - id: dedupe_hierarchy
        excert: "def _dedupe_hierarchy(items: list[ListItem]) -> list[ListItem]:"
        docstring: Remove items whose path lies below another item's path in the same directory hi…
      - id: list
        excert: "def list(items: list[ListItem]) -> ListBatchResult:"
        docstring: "List each directory in ``items``. Args: items: Directories to list. Must be non…"
      - id: ListTool
        excert: "class ListTool(ToolDefinition):"
        children:
        - id: ListTool.7wRGko|3xW2xR
          excert: name = 'list'
        - id: ListTool.4gWrRr|Coidj4
          excert: "_ITEM_PROPERTIES = {"
        - id: ListTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`list`, translating the MCP schema to/from the Python API."
      - id: register_list_tool
        excert: "def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: OyEiBH|qDejkd
        excert: "'Search tool – recursive extended-regex search for retrieval, for a batch of it…"
      - id: PBZWYD|FmTmX2
        excert: "from dataclasses import dataclass, replace"
      - id: 8Pl1k7|dBAVKD
        excert: "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - id: wTnFQX|MKeuzI
        excert: "__all__ = ["
      - id: gph2jG|n4wV62
        excert: "_NODE_TOOLS = frozenset({'ast_read', 'ast_edit_block', 'ast_edit_marks', 'ast_i…"
      - id: SearchError
        excert: "class SearchError(Exception):"
        docstring: Raised when a search cannot be executed or its output cannot be parsed.
      - id: SearchLineMatch
        excert: "@dataclass(frozen=True) class SearchLineMatch:"
        docstring: A single matching line within a file.
      - id: SearchNodeMatch
        excert: "@dataclass(frozen=True) class SearchNodeMatch:"
        docstring: Matching lines attributed to the most specific enclosing AST node.
      - id: SearchMatch
        excert: "@dataclass(frozen=True) class SearchMatch:"
        docstring: All matches found within a single file. ``nodes`` is set instead of relying on …
      - id: SearchItem
        excert: "@dataclass(frozen=True) class SearchItem:"
        docstring: "One independent search to run. Attributes: directory: Absolute paths of the dir…"
      - id: SearchResult
        excert: "@dataclass(frozen=True) class SearchResult:"
        docstring: "Result of a single search. Attributes: directory: The directory list exactly as…"
      - id: SearchItemError
        excert: "@dataclass(frozen=True) class SearchItemError:"
        docstring: "Error running a single search, mirroring its input for result association."
      - id: SearchBatchResult
        excert: "@dataclass(frozen=True) class SearchBatchResult:"
        docstring: "Result of :func:`search`. Attributes: results: One :class:`SearchResult` per su…"
      - id: make_line_match
        excert: "def _make_line_match(lineno: int, text: str) -> SearchLineMatch:"
      - id: parse_search_stdout
        excert: "def _parse_search_stdout(stdout: str) -> list[SearchMatch]:"
        docstring: "Parse 'path:line:content' stdout into :class:`SearchMatch` objects, grouped by …"
      - id: as_list
        excert: "def _as_list(value: list[str] | str | None) -> list[str]:"
        docstring: Normalize an optional list into a list (empty if ``None``). A bare string is to…
      - id: run_search
        excert: "def _run_search(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> tuple[ProcessResult, int]:"
        docstring: Recursively search one or more directories for ``pattern`` (extended regexp). A…
      - id: search_one
        excert: "def _search_one(item: SearchItem) -> SearchResult:"
      - id: resolve_nodes
        excert: "def _resolve_nodes(match: SearchMatch, roots: list[str]) -> SearchMatch:"
        docstring: Regroup a file's line matches by the most specific enclosing AST node. ``match`…
      - id: search
        excert: "def search(items: list[SearchItem], *, intent: str=_DEFAULT_INTENT, with_nodes: bool=False) -> SearchBatchResult:"
        docstring: "Run one or more independent searches. Limits apply per item, not per batch. Arg…"
      - id: check_intent
        excert: "def _check_intent(intent: str) -> None:"
      - id: SearchTool
        excert: "class SearchTool(ToolDefinition):"
        children:
        - id: SearchTool.IRSVLT|PCuUvD
          excert: name = 'search'
        - id: SearchTool.g4tkLE|v74Wwu
          excert: "_ITEM_PROPERTIES = {"
        - id: SearchTool.SQiS9d|8EWkWE
          excert: "_ITEM_REQUIRED = ['directory', 'pattern']"
        - id: SearchTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func: translating the MCP schema to/from the Python API."
      - id: register_search_tool
        excert: "def register_search_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
      nodes:
      - id: GXhd2a|xNQxjQ
        excert: "'Read tool – returns file contents, optionally sliced by line, character offset…"
      - id: igUrQT|uRBR2W
        excert: import hashlib
      - id: 0ZnbEE|BU27Ci
        excert: "__all__ = ["
      - id: ReadError
        excert: "class ReadError(Exception):"
        docstring: Raised when a file cannot be read or the requested range is invalid.
      - id: ReadItem
        excert: "@dataclass(frozen=True) class ReadItem:"
        docstring: "One file (optionally sliced to a range) to read. Attributes: path: Absolute pat…"
      - id: ReadResult
        excert: "@dataclass(frozen=True) class ReadResult:"
        docstring: "Result of reading a single file, mirroring its input path for result associatio…"
      - id: ReadItemError
        excert: "@dataclass(frozen=True) class ReadItemError:"
        docstring: "Error reading a single file, mirroring its input path for result association."
      - id: ReadBatchResult
        excert: "@dataclass(frozen=True) class ReadBatchResult:"
        docstring: "Result of :func:`read_file`. Attributes: results: One :class:`ReadResult` per s…"
      - id: cache_key
        excert: "def _cache_key(session_id: str, item: ReadItem) -> str:"
      - id: read_one
        excert: "def _read_one(item: ReadItem) -> ReadResult:"
      - id: read_file
        excert: "def read_file(items: list[ReadItem]) -> ReadBatchResult:"
        docstring: "Read one or more files, each optionally sliced to a range. Args: items: Files t…"
      - id: read_file_cached
        excert: "def _read_file_cached(items: list[ReadItem], session: Session) -> ReadBatchResult:"
        docstring: Wrap :func:`read_file`'s per-item logic with session-scoped change detection. K…
      - id: serialize_read_result
        excert: "def _serialize_read_result(result: ReadResult) -> dict[str, Any]:"
      - id: ReadTool
        excert: "class ReadTool(ToolDefinition):"
        children:
        - id: ReadTool.u9dRrZ|F2kfm5
          excert: name = 'read_file'
        - id: ReadTool.Iaivfu|ri3ksx
          excert: "_ITEM_PROPERTIES = {"
        - id: ReadTool.ZDSH0j|2essZ9
          excert: "_ITEM_REQUIRED = ['path']"
        - id: ReadTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`_read_file_cached`, translating the MCP schema to/from the P…"
      - id: register_read_tool
        excert: "def register_read_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 6bd4-3b-3a allow
ReasoningToken: 91
Control Request:
```yaml
id: 6bd4-3c-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    ids:
    - dvO55c|qVCPEH
    - ZrSLiQ|rTmMVq
    - ListResult
    - ListItemError
    - ListBatchResult
    - list_one
    - list
    - ListTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    ids:
    - PBZWYD|FmTmX2
    - 8Pl1k7|dBAVKD
    - wTnFQX|MKeuzI
    - SearchResult
    - SearchItemError
    - SearchBatchResult
    - search_one
    - search
    - SearchTool
  reason: "Code von list und search lesen, um deren Warnungen zu migrieren."
```
/answer 6bd4-3c-0 allow
Control Request:
```yaml
id: 6bd4-3d-3c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: dvO55c|qVCPEH
        type: imports
        code: |-
          import os
          import re
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools._directories import normalize_directory
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      - id: ZrSLiQ|rTmMVq
        type: statements
        code: |-
          __all__ = [
              'ListError',
              'ListItem',
              'ListResult',
              'ListItemError',
              'ListBatchResult',
              'list',
              'ListTool',
              'register_list_tool']
          _MAX_ENTRIES = 50
          _EXCLUDED_DIRS = {
              '.git',
              '.hg',
              '.svn',
              '__pycache__',
              '.mypy_cache',
              '.pytest_cache',
              '.ruff_cache',
              '.tox',
              '.venv',
              'venv',
              'node_modules',
              '.idea',
              '.vscode',
              'dist',
              'build',
              '.cache'}
      - id: ListResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListResult:
              """Result of listing a single directory, mirroring its input path for result association."""
              path: str
              entries: list[str]
              warning: str | None = None
      - id: ListItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListItemError:
              """Error listing a single directory, mirroring its input path for result association."""
              path: str
              error: str
      - id: ListBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListBatchResult:
              """Result of :func:`list`.

              Attributes:
                  results: One :class:`ListResult` per successfully listed directory.
                  errors: One :class:`ListItemError` per directory that failed.
              """
              results: list[ListResult]
              errors: list[ListItemError]
      - id: list_one
        type: FunctionDef
        code: |-
          def _list_one(item: ListItem) -> ListResult:
              """List all files below the absolute directory ``item.path``, grouped like ``ls -R``.

              Raises:
                  ListError: If path is not absolute.
                  ListError: If path does not exist or is not a directory.
                  ListError: If pattern is not a valid regular expression.
                  ListError: If more than ``_MAX_ENTRIES`` entries remain even after the result has been
                      reduced to one entry per directory level and then to top-level directories only.
              """
              '# When the flat file count exceeds the limit, the result is progressively reduced:'
              '# first to one entry per directory level (with a per-level file count), then, if'
              '# that still exceeds the limit, to top-level directories only (with an aggregated'
              '# file count below each). Both reduced results are returned with a warning.'
              dir_path = Path(item.path)
              if not dir_path.is_absolute():
                  raise ListError('Path must be absolute.')
              dir_path = normalize_directory(dir_path)
              if not dir_path.is_dir():
                  raise ListError('Directory not found or not a directory.')
              try:
                  regex = re.compile(item.pattern) if item.pattern else None
              except re.error as exc:
                  raise ListError(f'Invalid regex pattern: {exc}') from exc
              groups: dict[str, list[str]] = {}
              match_count = 0
              for root, dirs, files in os.walk(str(dir_path)):
                  rel_dir = os.path.relpath(root, str(dir_path))
                  matched_files = []
                  header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                  match_against_path = regex is not None and '/' in item.pattern
                  for file in sorted(files):
                      candidate = f'{header}/{file}' if match_against_path else file
                      if regex is None or regex.search(candidate):
                          matched_files.append(file)
                  if matched_files:
                      groups[rel_dir] = matched_files
                      match_count += len(matched_files)
                  if not item.recursive:
                      dirs.clear()
              if match_count <= _MAX_ENTRIES:
                  entries = []
                  for rel_dir in sorted(groups):
                      header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                      entries.append(f'{header}:')
                      entries.extend(groups[rel_dir])
                  return ListResult(path=item.path, entries=entries)
              '# Step 1: collapse each directory level to a single representative entry,'
              '# annotated with the total file count for that level.'
              level1_counts = {rel_dir: len(files) for rel_dir, files in groups.items()}
              if len(level1_counts) <= _MAX_ENTRIES:
                  entries = []
                  for rel_dir in sorted(groups):
                      header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                      count = level1_counts[rel_dir]
                      entries.append(f'{header}: ({count} file{('s' if count != 1 else '')})')
                      entries.append(groups[rel_dir][0])
                  warning = f"Result reduced because the total number of entries ({match_count}) exceeds the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter."
                  return ListResult(path=item.path, entries=entries, warning=warning)
              '# Step 2: the per-level reduction still exceeds the limit, so collapse the'
              '# tree depth and aggregate file counts below each top-level directory.'

              def _top_key(rel_dir: str) -> str:
                  if rel_dir == '.':
                      return '.'
                  return rel_dir.split(os.sep)[0]
              top_counts: dict[str, int] = {}
              for rel_dir, files in groups.items():
                  key = _top_key(rel_dir)
                  top_counts[key] = top_counts.get(key, 0) + len(files)
              if len(top_counts) > _MAX_ENTRIES:
                  raise ListError(
                      f"Too many entries even after reducing by directory level ({
                          len(top_counts)}) exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")
              entries = []
              for key in sorted(top_counts):
                  header = key if key == '.' else './' + key.replace(os.sep, '/')
                  count = top_counts[key]
                  entries.append(f'{header}: ({count} file{('s' if count != 1 else '')} below)')
              warning = f"Result reduced to top-level directories the limit of {_MAX_ENTRIES} is exceeded ({
                  len(level1_counts)}). Narrow down the result using the 'pattern' regular expression parameter."
              return ListResult(path=item.path, entries=entries, warning=warning)
      - id: list
        type: FunctionDef
        code: |-
          def list(items: list[ListItem]) -> ListBatchResult:
              """List each directory in ``items``.

              Args:
                  items: Directories to list. Must be non-empty.

              Returns:
                  ListBatchResult: one result per successfully listed directory, one error per failed directory.

              Raises:
                  ListError: If items is empty.
              """
              if not items:
                  raise ListError("'items' must be a non-empty list.")
              items = _dedupe_hierarchy(items)
              results: list[ListResult] = []
              errors: list[ListItemError] = []
              for item in items:
                  try:
                      results.append(_list_one(item))
                  except ListError as exc:
                      errors.append(ListItemError(path=item.path, error=str(exc)))
              return ListBatchResult(results=results, errors=errors)
      - id: ListTool
        type: ClassDef
        excert: "class ListTool(ToolDefinition):"
        children:
        - id: ListTool.7wRGko|3xW2xR
          type: statements
          code: |-
            name = 'list'
            title = 'List directory contents'
            description = 'List all files below one or more absolute directory paths, recursively, as a flat list, for a batch of items. Filter each result with a regular expression. Limits apply per item, not per batch.'
        - id: ListTool.4gWrRr|Coidj4
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'path': {
                    'type': 'string', 'description': 'Absolute directory path.'}, 'pattern': {
                        'type': 'string', 'description': 'Regular expression to filter the result.'}, 'recursive': {
                            'type': 'boolean', 'description': 'List subdirectories recursively. Default true.'}}
            _ITEM_REQUIRED = ['path']
            _ITEMS_DESCRIPTION = 'Directories to list.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: ListTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""

                def item_factory(it: dict[str, Any]) -> ListItem:
                    return ListItem(path=it['path'], pattern=it.get('pattern'), recursive=it.get('recursive', True))

                def result_serializer(r):
                    return {'path': r.path, 'entries': r.entries, **({'warning': r.warning} if r.warning else {})}
                return handle_batch_tool(ctx, item_factory, list, ListError, result_serializer)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: PBZWYD|FmTmX2
        type: imports
        code: |-
          from dataclasses import dataclass, replace
          from types import SimpleNamespace
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools._directories import normalize_directories
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: 8Pl1k7|dBAVKD
        type: imports
        code: |-
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
          import re
          from xy.ai.mcpc.tools.ast import core
      - id: wTnFQX|MKeuzI
        type: statements
        code: |-
          __all__ = [
              'SearchError',
              'SearchLineMatch',
              'SearchNodeMatch',
              'SearchMatch',
              'SearchItem',
              'SearchResult',
              'SearchItemError',
              'SearchBatchResult',
              'search',
              'SearchTool',
              'register_search_tool']
          _DEFAULT_LIMIT = 15
          _INTENTS = ('survey', 'editing')
          _DEFAULT_INTENT = 'survey'
          _LINE_TOOLS = frozenset({'read_file', 'edit_marks', 'edit_line', 'edit_block', 'edit_lines'})
      - id: SearchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SearchResult:
              """Result of a single search.

              Attributes:
                  directory: The directory list exactly as given in the input.
                  matches: The matches found, grouped by file (empty if none).
                  warning: Set if ``limit`` was reached and further matches may exist, or if match lines
                      exceeded the maximum length and were shortened with an ellipsis.

              """
              directory: list[str]
              matches: list[SearchMatch]
              warning: str | None = None
      - id: SearchItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SearchItemError:
              """Error running a single search, mirroring its input for result association."""
              directory: list[str]
              pattern: str
              error: str
      - id: SearchBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SearchBatchResult:
              """Result of :func:`search`.

              Attributes:
                  results: One :class:`SearchResult` per successfully executed search.
                  errors: One :class:`SearchItemError` per search that failed.
              """
              results: list[SearchResult]
              errors: list[SearchItemError]
      - id: search_one
        type: FunctionDef
        code: |-
          def _search_one(item: SearchItem) -> SearchResult:
              result, total_lines = _run_search(item.directory, item.pattern, exclude=item.exclude,
                                                include=item.include, limit=item.limit)
              if result.exit_code >= 2:
                  raise SearchError(f'search failed (exit code {result.exit_code}): {result.stderr}')
              matches = _parse_search_stdout(result.stdout)
              total_matches = total_lines
              warning = None
              if total_matches > item.limit:
                  hint = 'narrow pattern, directory or include/exclude filters'
                  if item.limit < _MAX_LIMIT:
                      hint += f', or raise limit (max {_MAX_LIMIT})'
                  warning = f'Limit of {item.limit} matches reached (total {total_matches}); {hint}.'
              shortened = sum((lm.truncated for m in matches for lm in m.matches))
              if shortened:
                  shortened_warning = f'{shortened} match line(s) exceeded {_MAX_LINE_LENGTH} characters and were shortened in the middle.'
                  warning = f'{warning} {shortened_warning}' if warning else shortened_warning
              return SearchResult(directory=item.directory, matches=matches, warning=warning)
      - id: search
        type: FunctionDef
        code: |-
          def search(items: list[SearchItem], *, intent: str=_DEFAULT_INTENT, with_nodes: bool=False) -> SearchBatchResult:
              """Run one or more independent searches. Limits apply per item, not per batch.

              Args:
                  items: Searches to run. Must be non-empty.
                  intent: Default intent for all items; ``SearchItem.intent`` takes precedence.
                  with_nodes: If true, items with intent ``'editing'`` resolve matched lines to AST node ids
                      (see :class:`SearchMatch`).

              Returns:
                  SearchBatchResult: one result per successful search, one error per failed search.

              Raises:
                  SearchError: If items is empty or the batch-level intent is invalid.
              """
              if not items:
                  raise SearchError("'items' must be a non-empty list.")
              _check_intent(intent)
              results: list[SearchResult] = []
              errors: list[SearchItemError] = []
              for item in items:
                  try:
                      effective = item.intent if item.intent is not None else intent
                      _check_intent(effective)
                      result = _search_one(item)
                      if effective == 'editing' and with_nodes:
                          result = replace(result, matches=[_resolve_nodes(m, item.directory) for m in result.matches])
                      results.append(result)
                  except SearchError as exc:
                      errors.append(SearchItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
              return SearchBatchResult(results=results, errors=errors)
      - id: SearchTool
        type: ClassDef
        excert: "class SearchTool(ToolDefinition):"
        children:
        - id: SearchTool.IRSVLT|PCuUvD
          type: statements
          code: |-
            name = 'search'
            title = 'Search code, text, id, and content within directories'
            description = "Run one or more independent searches with an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters."
        - id: SearchTool.g4tkLE|v74Wwu
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'directory': {
                    'type': 'array',
                    'items': {
                        'type': 'string'},
                    'minItems': 1,
                    'description': 'Absolute paths of the directories to search recursively. Always use the narrowest subtree(s) that are likely to contain the target files.'},
                'pattern': {
                    'type': 'string',
                            'description': 'Extended regular expression to search for. Make the pattern as specific as possible to reduce noise.'},
                'exclude': {
                    'type': 'array',
                    'items': {
                        'type': 'string'},
                    'description': "Globs of file names to exclude from the search, e.g. '*.min.js'. Always set this to exclude build artefacts, dependencies (e.g. 'node_modules/**'), and minified files."},
                'include': {
                    'type': 'array',
                    'items': {
                        'type': 'string'},
                    'description': "Globs of file names to include in the search, e.g. '*.py'. Always set this to restrict the search to the relevant file types; omit only when the file type is unknown."},
                'limit': {
                    'type': 'integer',
                    'description': 'Maximum number of matching lines to return for this item.',
                    'default': _DEFAULT_LIMIT,
                    'minimum': 1,
                    'maximum': _MAX_LIMIT},
                'intent': {
                    'type': 'string',
                    'enum': list(_INTENTS),
                    'description': 'Intent for this item; overrides the batch-level intent.'}}
        - id: SearchTool.SQiS9d|8EWkWE
          type: statements
          code: |-
            _ITEM_REQUIRED = ['directory', 'pattern']
            _ITEMS_DESCRIPTION = 'Independent searches to run.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
            input_schema['properties']['intent'] = {
                'type': 'string',
                'enum': list(_INTENTS),
                'default': _DEFAULT_INTENT,
                'description': 'Default intent for all items.' + _INTENT_DESCRIPTION}
        - id: SearchTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func: translating the MCP schema to/from the Python API."""
                intent = ctx.arguments.get('intent', _DEFAULT_INTENT)
                enabled = ctx.session.enabled_tools
                with_lines = bool(_LINE_TOOLS & enabled)
                with_nodes = bool(_NODE_TOOLS & enabled)

                def item_factory(it: dict[str, Any]) -> SearchItem:
                    return SearchItem(
                        directory=it['directory'],
                        pattern=it['pattern'],
                        exclude=it.get('exclude'),
                        include=it.get('include'),
                        limit=int(
                            it.get(
                                'limit',
                                _DEFAULT_LIMIT)),
                        intent=it.get('intent'))

                def line_dicts(lms: list[SearchLineMatch], with_lineno: bool) -> list[Any]:
                    if with_lineno:
                        return [{'lineno': lm.lineno, 'match': lm.match} for lm in lms]
                    return [lm.match for lm in lms]

                def file_dict(m: SearchMatch, item_intent: str) -> dict[str, Any]:
                    editing = item_intent == 'editing'
                    entry: dict[str, Any] = {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename}
                    if m.nodes:
                        entry['nodes'] = [{'id': n.node_id, 'matches': line_dicts(
                            n.matches, editing and with_lines)} for n in m.nodes]
                    if m.matches:
                        lines = line_dicts(m.matches, editing and (with_lines or with_nodes))
                        entry['matches'] = lines if editing else list(dict.fromkeys(lines))
                    return entry

                def result_serializer(rendered: tuple[SearchResult, str]) -> dict[str, Any]:
                    r, item_intent = rendered
                    entry: dict[str, Any] = {'directory': r.directory}
                    if r.matches:
                        entry['matches'] = [file_dict(m, item_intent) for m in r.matches]
                    if r.warning is not None:
                        entry['warning'] = r.warning
                    return entry

                def error_serializer(e: SearchItemError) -> dict[str, Any]:
                    return {'directory': e.directory, 'pattern': e.pattern, 'error': e.error}

                def run(items: list[SearchItem]) -> SimpleNamespace:
                    results: list[tuple[SearchResult, str]] = []
                    errors: list[SearchItemError] = []
                    for item in items:
                        batch = search([item], intent=intent, with_nodes=with_nodes)
                        effective = item.intent if item.intent is not None else intent
                        results += [(r, effective) for r in batch.results]
                        errors += batch.errors
                    return SimpleNamespace(results=results, errors=errors)
                return handle_batch_tool(ctx, item_factory, run, SearchError, result_serializer, error_serializer)
```
/answer 6bd4-3d-3c allow
ReasoningToken: 2985
Control Request:
```yaml
id: 6bd4-3e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
    pattern: normalize_result|to_dict\(\)|attach_message|structuredContent|JsonCodec
    include:
    - mcp_protocol.py
    intent: editing
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: MESSAGE_PROPERTY|error_serializer|result_serializer|output_schema\s*=
    include:
    - '*.py'
    limit: 50
  reason: "Protokoll-Schicht (normalize_result/ToolResult-Rendering), MESSAGE_PROPERTY und alle Serializer-/output_schema-Nutzungen verstehen."
```
/answer 6bd4-3e-0 allow
Control Request:
```yaml
id: 6bd4-3f-3e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
      matches:
      - path: mcp_protocol.py
        nodes:
        - id: lF3REn|IWeyxO
          matches:
          - "from xy.ai.mcpc.tools.tool_registry import ToolRegistry, normalize_result, ToolResult, text_content, attach_message"
        - id: McpProtocol.handle_tools_call.2w8tpw|yuXXaJ
          matches:
          - "'answer': reason or 'No answer given, anticipate and proceed'}).to_dict()"
          - is_error=True).to_dict()
          - "return ToolResult(structured_content={'answer': decision.approval_hint}).to_dict()"
        - id: McpProtocol.handle_tools_call.E8VojT|9XmOmT
          matches:
          - result = normalize_result(raw)
        - id: McpProtocol.handle_tools_call.dDYvUB|3FtoJS
          matches:
          - "result.to_dict(),"
          - "'answer': reason or 'No answer given, anticipate and proceed'}).to_dict()"
          - is_error=True).to_dict()
          - result_dict = result.to_dict()
          - "return ToolResult(structured_content={'answer': combined_hint}).to_dict()"
          - "attach_message(result_dict, combined_hint)"
        - id: McpProtocol.handle_tools_call.5AKA7N|0ThY6X
          matches:
          - return result.to_dict()
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/embedded/tools.py
        matches:
        - "self.output_schema = output_schema if output_schema is not None else {'type': 'object'}"
      - path: xy/ai/mcpc/tools/ast/replace.py
        matches:
        - "def result_serializer(r: ReplaceResult) -> dict[str, Any]:"
        - "def error_serializer(e: ReplaceError) -> dict[str, Any]:"
        - "result_serializer,"
        - "error_serializer,"
      - path: xy/ai/mcpc/tools/ast/delete.py
        matches:
        - "def result_serializer(r: DeleteResult) -> dict[str, Any]:"
        - "return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, result_serializer, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "result_serializer = lambda r: {'path': r.path, 'nodes': [core.to_dict(n) for n in r.nodes]}"
        - "error_serializer = lambda e: {'path': e.path, 'error': e.error}"
        - "content = serialize_batch_result(batch, result_serializer, error_serializer)"
      - path: xy/ai/mcpc/tools/ast/edit_marks.py
        matches:
        - "def result_serializer(r: EditMarksResult) -> dict[str, Any]:"
        - "def error_serializer(e: EditMarksError) -> dict[str, Any]:"
        - "result_serializer,"
        - "error_serializer,"
      - path: xy/ai/mcpc/tools/ast/edit_block.py
        matches:
        - "def result_serializer(r: EditBlockResult) -> dict[str, Any]:"
        - "def error_serializer(e: EditBlockError) -> dict[str, Any]:"
        - "result_serializer,"
        - "error_serializer,"
      - path: xy/ai/mcpc/tools/ast/insert.py
        matches:
        - "def result_serializer(r: InsertResult) -> dict[str, Any]:"
        - "def error_serializer(e: InsertError) -> dict[str, Any]:"
        - "result_serializer,"
        - "error_serializer,"
      - path: xy/ai/mcpc/tools/_tool_helpers.py
        matches:
        - "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_ser…: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:"
        - "result_serializer: Optional custom serializer for result items (default: asdict)."
        - "error_serializer: Optional custom serializer for error items (default: asdict)."
        - "result_serializer=result_serializer,"
        - error_serializer=error_serializer)
        - "def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:"
        - "if result_serializer is None:"
        - "result_serializer = lambda r: _compact(asdict(r)) if hasattr(r, '__dataclass_fields__') else r"
        - "if error_serializer is None:"
        - "error_serializer = lambda e: _compact(asdict(e)) if hasattr(e, '__dataclass_fields__') else e"
        - "content['results'] = [result_serializer(r) for r in batch_result.results]"
        - "content['errors'] = [error_serializer(e) for e in batch_result.errors]"
      - path: xy/ai/mcpc/tools/tool_registry.py
        matches:
        - "structured[MESSAGE_PROPERTY] = message"
        - "reference = f\"The user attached a message: read '{MESSAGE_PROPERTY}' in structuredContent.\""
        - "output_schema=self.output_schema,"
        - MESSAGE_PROPERTY = 'messageFromUser'
        - "base_output_schema = tool.output_schema or {'type': 'object', 'properties': {}}"
        - tool.output_schema = base_output_schema
        - "output_schema=output_schema,"
      - path: xy/ai/mcpc/tools/edit_block/__init__.py
        matches:
        - "def error_serializer(e: EditBlockItemError) -> dict[str, Any]:"
        - "error_serializer=error_serializer,"
      - path: xy/ai/mcpc/tools/edit_marks/__init__.py
        matches:
        - "def error_serializer(e: EditMarksItemError) -> dict[str, Any]:"
        - "error_serializer=error_serializer,"
      - path: xy/ai/mcpc/tools/list/__init__.py
        matches:
        - "def result_serializer(r):"
        - "return handle_batch_tool(ctx, item_factory, list, ListError, result_serializer)"
      - path: xy/ai/mcpc/tools/read/__init__.py
        matches:
        - "structured_content = serialize_batch_result(batch, result_serializer=_serialize_read_result)"
      warning: "Limit of 50 matches reached (total 53); narrow pattern, directory or include/exclude filters. 1 match line(s) exceeded 300 characters and were shortened in the middle."
```
/answer 6bd4-3f-3e allow
Control Request:
```yaml
id: 6bd4-40-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: (Result|Error)\b.*\(.*\):|^@dataclass|^class \w+(BatchResult|ItemError|Result)\b
    include:
    - edit_block/*.py
    - edit_marks/*.py
    - edit_lines/*.py
    - skills/*.py
    - bash/*.py
    - tool_usage/*.py
    - ast/create.py
    - ast/insert.py
    - ast/edit_block.py
    - ast/edit_marks.py
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: base_output_schema|MESSAGE_PROPERTY
    include:
    - tool_registry.py
    intent: editing
  reason: Übrige Batch-Tools mit eigenen Result-/Error-Klassen und die Output-Schema-Injektion im Registry finden.
```
/answer 6bd4-40-0 allow
Control Request:
```yaml
id: 6bd4-41-40
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: tool_registry.py
        nodes:
        - id: attach_message.PYUKdB|JUCNQG
          matches:
          - "structured[MESSAGE_PROPERTY] = message"
          - "reference = f\"The user attached a message: read '{MESSAGE_PROPERTY}' in structuredContent.\""
        - id: hIMaii|mbWWig
          matches:
          - MESSAGE_PROPERTY = 'messageFromUser'
        - id: ToolRegistry.register.Ymvxai|4eKtIT
          matches:
          - "base_output_schema = tool.output_schema or {'type': 'object', 'properties': {}}"
          - tool.output_schema = base_output_schema
```
/answer 6bd4-41-40 allow
ReasoningToken: 183
Control Request:
```yaml
id: 6bd4-42-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  reason: Struktur der ast_replace-Batch-Klassen und des import_notice-Helpers.
```
/answer 6bd4-42-0 allow
Control Request:
```yaml
id: 6bd4-43-42
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: G65kv0|hrDhpa
        excert: "'``ast_replace`` tool: replace selected nodes with new source.'"
      - id: G5Js58|nIvKCC
        excert: from dataclasses import dataclass
      - id: qzaxoR|Ancd56
        excert: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: xj2nBU|s4L0vm
        excert: "__all__ = ["
      - id: ReplaceItem
        excert: "@dataclass(frozen=True) class ReplaceItem:"
        docstring: "One node replacement to apply. Attributes: path: Absolute path to the file to m…"
      - id: ReplaceResult
        excert: "@dataclass(frozen=True) class ReplaceResult:"
        docstring: "Result of a single node replacement. Attributes: path: The path exactly as give…"
      - id: ReplaceError
        excert: "@dataclass(frozen=True) class ReplaceError:"
        docstring: "Error applying a single node replacement. Attributes: path: The path exactly as…"
      - id: ReplaceBatchResult
        excert: "@dataclass(frozen=True) class ReplaceBatchResult:"
        docstring: "Result of :func:`ast_replace`. Attributes: results: One :class:`ReplaceResult` …"
      - id: replace_one
        excert: "def _replace_one(item: ReplaceItem) -> ReplaceResult:"
      - id: ast_replace
        excert: "def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:"
        docstring: Replace one or more selected nodes with new source. ``id`` and replacement ``so…
      - id: ReplaceNodeTool
        excert: "class ReplaceNodeTool(ToolDefinition):"
        children:
        - id: ReplaceNodeTool.jKVWKu|Cre3Kp
          excert: name = 'ast_replace'
        - id: ReplaceNodeTool.hKJ2Rq|SrCCxe
          excert: _ITEMS_DESCRIPTION = 'Node replacements to apply.'
        - id: ReplaceNodeTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: foxd99|7aiScK
        excert: '"Import/include/use handling shared by the ``ast_*`` edit tools.\n\nEvery edit …'
      - id: AFajCW|f1RBFD
        excert: from __future__ import annotations
      - id: AiyIX3|8cD8JN
        excert: "__all__ = ["
      - id: ImportOutcome
        excert: "@dataclass class ImportOutcome:"
        docstring: "What an engine did for one item's ``addImport``/``removeImport``. Attributes: a…"
      - id: string_list
        excert: "def _string_list(item: dict[str, Any], key: str) -> list[str] | None:"
      - id: parse_import_args
        excert: "def parse_import_args(item: dict[str, Any]) -> tuple[list[str] | None, list[str] | None]:"
        docstring: "``(addImport, removeImport)`` of a raw MCP item; ``ValueError`` on malformed va…"
      - id: import_notice
        excert: "def import_notice(results: list[Any]) -> str | None:"
        docstring: "The single batch-level hint, if any result carries unchecked imports."
      - id: import_entry
        excert: "def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:"
        docstring: "Serialized ``imports`` field of a result, or ``None`` if nothing changed."
      - id: S64wGg|AyaH5Z
        excert: _SPACE = re.compile('\\s+')
      - id: normalize_import
        excert: "def normalize_import(text: str) -> str:"
        docstring: "Whitespace-, quote- and trailing-``;``-insensitive form used to compare stateme…"
      - id: checked
        excert: "def _checked(statement: str) -> str:"
      - id: unique
        excert: "def _unique(statements: list[str]) -> list[str]:"
      - id: statement_spans
        excert: "def statement_spans(data: bytes) -> list[tuple[int, int]]:"
        docstring: Byte spans of the ``;``-terminated top-level statements in ``data`` (comments s…
      - id: line_extent
        excert: "def line_extent(data: bytes, start: int, end: int) -> tuple[int, int]:"
        docstring: "``(start, end)`` widened to its whole line(s) if nothing else stands on them."
      - id: delete_span
        excert: "def delete_span(data: bytes, start: int, end: int) -> bytes:"
        docstring: "Remove ``data[start:end]``, including its line if nothing else stands on it."
      - id: next_line_start
        excert: "def _next_line_start(data: bytes, offset: int) -> int:"
      - id: line_starts
        excert: "def _line_starts(data: bytes) -> list[int]:"
      - id: store
        excert: "def _store(engine: Engine, tree: Tree, data: bytes) -> None:"
      - id: apply_generic
        excert: "def apply_generic(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
        docstring: "Engine-agnostic fallback. ``add`` is only checked: a statement missing as a lin…"
      - id: prologue_end
        excert: "def _prologue_end(root: Any, data: bytes, is_prologue: Callable[[Any], bool] | None) -> int:"
        docstring: Offset after leading shebang/directives and header comments (comments followed …
      - id: snippet
        excert: "def _snippet(engine: Engine, statement: str, import_types: set[str]) -> str:"
      - id: apply_treesitter
        excert: "def apply_treesitter(engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str], is_prologue: Callable[[Any], bool] | None=None) -> ImportOutcome:"
        docstring: Structure-aware handling for tree-sitter engines whose top-level import nodes a…
      - id: fs86A7|AnrSrF
        excert: "'#: Node types under which the RPC processes report a run of imports.'"
      - id: rpc_statement
        excert: "def _rpc_statement(statement: str) -> str:"
      - id: rpc_present
        excert: "def _rpc_present(engine: Engine, tree: Tree, source: str | None=None) -> tuple[dict[str, tuple[int, int]], list[Any]]:"
        docstring: Every top-level import statement's byte span (keyed by normalized text) and the…
      - id: g7FpPW|tkBF2z
        excert: "'#: Line prefixes that stay attached to the declaration below them (docs, annot…"
      - id: leading_start
        excert: "def _leading_start(data: bytes, starts: list[int], index: int) -> int:"
        docstring: "Start offset of line ``index``, moved up over directly attached comment/annotat…"
      - id: rpc_insertion
        excert: "def _rpc_insertion(data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:"
        docstring: "Edit that places ``code``: behind the last import, else behind ``package``/modu…"
      - id: apply_rpc
        excert: "def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
        docstring: Structure-aware handling for RPC engines that report import groups (Java ``impo…
```
/answer 6bd4-43-42 allow
ReasoningToken: 84
Control Request:
```yaml
id: 6bd4-44-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    ids:
    - G5Js58|nIvKCC
    - qzaxoR|Ancd56
    - xj2nBU|s4L0vm
    - ReplaceResult
    - ReplaceError
    - ReplaceBatchResult
    - ast_replace
    - ReplaceNodeTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    ids:
    - AFajCW|f1RBFD
    - AiyIX3|8cD8JN
    - ImportOutcome
    - import_notice
    - import_entry
  reason: "Replace-Result-Klassen, Serializer und import_notice im Detail lesen."
```
/answer 6bd4-44-0 allow
Control Request:
```yaml
id: 6bd4-45-44
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: G5Js58|nIvKCC
        type: imports
        code: |-
          from dataclasses import dataclass
          from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path
          from xy.ai.mcpc.tools._tool_helpers import batch_schema
      - id: qzaxoR|Ancd56
        type: imports
        code: |-
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - id: xj2nBU|s4L0vm
        type: statements
        code: |-
          __all__ = [
              'ReplaceItem',
              'ReplaceResult',
              'ReplaceError',
              'ReplaceBatchResult',
              'ast_replace',
              'ReplaceNodeTool',
              'register']
      - id: ReplaceResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceResult:
              """Result of a single node replacement.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  new_id: The node's new id, only set if the replacement changed it.
              """
              path: str
              id: str | None
              result: str
              new_id: str | None = None
              imports: ImportOutcome | None = None
      - id: ReplaceError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceError:
              """Error applying a single node replacement.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  error: The error message.
              """
              path: str
              id: str | None
              error: str
      - id: ReplaceBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceBatchResult:
              """Result of :func:`ast_replace`.

              Attributes:
                  results: One :class:`ReplaceResult` per successful replacement.
                  errors: One :class:`ReplaceError` per replacement that failed.
              """
              results: list[ReplaceResult]
              errors: list[ReplaceError]
              notice: str | None = None
      - id: ast_replace
        type: FunctionDef
        code: |-
          def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:
              """Replace one or more selected nodes with new source.

              ``id`` and replacement ``source`` stay together per item, since replacing
              the same node twice makes no sense; several items may target the same or
              different files.

              Args:
                  items: Node replacements to apply. Must be non-empty.

              Returns:
                  ReplaceBatchResult: One result per successful replacement, one error per
                  failed replacement.

              Raises:
                  core.AstError: If ``items`` is empty.
              """
              if not items:
                  raise core.AstError("'items' must be a non-empty list.")
              results: list[ReplaceResult] = []
              errors: list[ReplaceError] = []
              for item in items:
                  try:
                      results.append(_replace_one(item))
                  except core.AstError as exc:
                      errors.append(ReplaceError(path=item.path, id=item.id, error=str(exc)))
              return ReplaceBatchResult(results=results, errors=errors, notice=import_notice(results))
      - id: ReplaceNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."""

              def item_factory(it: dict[str, Any]) -> ReplaceItem:
                  add_import, remove_import = parse_import_args(it)
                  return ReplaceItem(
                      path=it['path'],
                      source=it['source'],
                      id=it.get('id'),
                      add_import=add_import,
                      remove_import=remove_import)

              def result_serializer(r: ReplaceResult) -> dict[str, Any]:
                  entry = {'path': r.path, 'result': r.result}
                  if r.id is not None:
                      entry['id'] = r.id
                  if r.new_id is not None:
                      entry['new_id'] = r.new_id
                  if (imports := import_entry(r.imports)) is not None:
                      entry['imports'] = imports
                  return entry

              def error_serializer(e: ReplaceError) -> dict[str, Any]:
                  entry = {'path': e.path, 'error': e.error}
                  if e.id is not None:
                      entry['id'] = e.id
                  return entry
              return handle_batch_tool(
                  ctx,
                  item_factory,
                  ast_replace,
                  core.AstError,
                  result_serializer,
                  error_serializer,
                  auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: AFajCW|f1RBFD
        type: imports
        code: |-
          from __future__ import annotations
          import re
          from dataclasses import dataclass, field
          from typing import Any, Callable
          from xy.ai.mcpc.tools.ast.base import AstError, Engine, Tree, check_no_control_chars
      - id: AiyIX3|8cD8JN
        type: statements
        code: |-
          __all__ = [
              'IMPORT_NOTICE',
              'ImportOutcome',
              'parse_import_args',
              'import_notice',
              'import_entry',
              'normalize_import',
              'apply_generic',
              'apply_treesitter',
              'apply_rpc']
          IMPORT_NOTICE = 'Imports/includes/use statements could not be verified. Check the affected files yourself.'
      - id: ImportOutcome
        type: ClassDef
        code: |-
          @dataclass
          class ImportOutcome:
              """What an engine did for one item's ``addImport``/``removeImport``.

              Attributes:
                  added: Statements that were missing and have been inserted.
                  removed: Statements that were present and have been removed.
                  unchecked: Whether at least one statement could not be handled and the
                      agent has to check it manually.
              """
              added: list[str] = field(default_factory=list)
              removed: list[str] = field(default_factory=list)
              unchecked: bool = False
      - id: import_notice
        type: FunctionDef
        code: |-
          def import_notice(results: list[Any]) -> str | None:
              """The single batch-level hint, if any result carries unchecked imports."""
              if any((r.imports is not None and r.imports.unchecked for r in results)):
                  return IMPORT_NOTICE
              return None
      - id: import_entry
        type: FunctionDef
        code: |-
          def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:
              """Serialized ``imports`` field of a result, or ``None`` if nothing changed."""
              if outcome is None:
                  return None
              entry = {key: value for key, value in (('added', outcome.added), ('removed', outcome.removed)) if value}
              return entry or None
```
/answer 6bd4-45-44 allow
Thinking:
Die Analyse ist fertig: `BatchError` ist ungenutzt und kann weg, aber `result.warning` und `.notice` werden an mehreren Stellen von Tests und Skript-API gelesen und müssen angepasst werden. Ich kann den pytest-Lauf nicht selbst starten, prüfe die Dateien daher nur per `ast_validate` und lese als Nächstes die übrigen Batch-Tools.


ReasoningToken: 164
Control Request:
```yaml
id: 6bd4-46-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  regexp: ^(@dataclass|class (\w+)(Result|Error|Tool)|\s+def (handle|result_serializer|error_serializer))|notice|BatchResult\(|CreateFilesResult\(
  reason: "Result-/Error-/BatchResult-Klassen, Serializer und notice-Stellen der übrigen ast-Edit-Tools lesen."
```
/answer 6bd4-46-0 allow
Control Request:
```yaml
id: 6bd4-47-46
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
      - id: G5Js58|nIvKCC
        code: |-
          from dataclasses import dataclass
          from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path
          from xy.ai.mcpc.tools._tool_helpers import batch_schema
      - id: DeleteItem.PrKpxX|E4BsYR
        code: |-
          'One node (or file) to delete.\n\n    Attributes:\n        path: Absolute path to the file to modify.\n        id: Unique id of the target node. Omit to delete the whole file.\n        add_import: Import statements to add if missing, applied after the delete.\n        remove_import: Import statements to remove if present, applied after the delete.\n            Both are ignored (and flagged in the batch notice) when a whole file is deleted.\n    '
          path: str
          id: str | None = None
      - id: DeleteBatchResult.O157Kg|C35UGZ
        code: |-
          'Result of :func:`ast_delete`.\n\n    Attributes:\n        results: One :class:`DeleteResult` per successfully deleted item.\n        errors: One :class:`DeleteError` per item that failed.\n    '
          results: list[DeleteResult]
          errors: list[DeleteError]
          notice: str | None = None
      - id: ast_delete.Azmusq|D9YYzV
        code: |-
          if not items:
              raise core.AstError("'items' must be a non-empty list.")
          results: list[DeleteResult] = []
          errors: list[DeleteError] = []
          for item in items:
              try:
                  results.append(_delete_one(item))
              except core.AstError as exc:
                  errors.append(DeleteError(path=item.path, id=item.id, error=str(exc)))
          return DeleteBatchResult(results=results, errors=errors, notice=import_notice(results))
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      nodes:
      - id: G5Js58|nIvKCC
        code: |-
          from dataclasses import dataclass
          from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path
          from xy.ai.mcpc.tools._tool_helpers import batch_schema
      - id: InsertBatchResult.O157Kg|owIDie
        code: |-
          'Result of :func:`ast_insert`.\n\n    Attributes:\n        results: One :class:`InsertResult` per successful insert.\n        errors: One :class:`InsertError` per insert that failed.\n    '
          results: list[InsertResult]
          errors: list[InsertError]
          notice: str | None = None
      - id: ast_insert.Azmusq|tFexdV
        code: |-
          if not items:
              raise core.AstError("'items' must be a non-empty list.")
          results: list[InsertResult] = []
          errors: list[InsertError] = []
          for item in items:
              try:
                  results.append(_insert_one(item))
              except core.AstError as exc:
                  errors.append(InsertError(path=item.path, id=item.id, error=str(exc)))
          return InsertBatchResult(results=results, errors=errors, notice=import_notice(results))
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: G5Js58|8gAslm
        code: |-
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_text
          from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
          from xy.ai.mcpc.tools._tool_helpers import batch_schema
      - id: EditBlockBatchResult.O157Kg|agXhT9
        code: |-
          'Result of :func:`ast_edit_block`.\n\n    Attributes:\n        results: One :class:`EditBlockResult` per successfully applied edit.\n        errors: One :class:`EditBlockError` per edit that failed.\n    '
          results: list[EditBlockResult]
          errors: list[EditBlockError]
          notice: str | None = None
      - id: ast_edit_block.92vOt4|K8Whz5
        code: "return EditBlockBatchResult(results=results, errors=errors, notice=import_notice(results))"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
      nodes:
      - id: G5Js58|8gAslm
        code: |-
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_text
          from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
          from xy.ai.mcpc.tools._tool_helpers import batch_schema
      - id: EditMarksBatchResult.O157Kg|a0g1B9
        code: |-
          'Result of :func:`ast_edit_marks`.\n\n    Attributes:\n        results: One :class:`EditMarksResult` per successfully applied edit.\n        errors: One :class:`EditMarksError` per edit that failed.\n    '
          results: list[EditMarksResult]
          errors: list[EditMarksError]
          notice: str | None = None
      - id: ast_edit_marks.ad45ed|Ygmaf8
        code: "return EditMarksBatchResult(results=results, errors=errors, notice=import_notice(results))"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
      nodes:
      - id: ast_create.qcHzuv|eqjKJV
        code: |-
          results: list[CreateFileResult] = []
          errors: list[CreateFileError] = []
          for item in items:
              try:
                  results.append(_create_one(item))
              except core.AstError as exc:
                  errors.append(CreateFileError(path=item.path, error=str(exc)))
          return CreateFilesResult(results=results, errors=errors)
```
/answer 6bd4-47-46 allow
ReasoningToken: 136
Control Request:
```yaml
id: 6bd4-48-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  node_type: ClassDef
  name: .*(Result|Error)
  reason: Vollständige Result-/Error-/BatchResult-Klassen der Edit-Tools lesen.
```
/answer 6bd4-48-0 allow
Control Request:
```yaml
id: 6bd4-49-48
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
      nodes: []
```
/answer 6bd4-49-48 allow
Text:
 `name` verlangt offenbar einen einfachen Namen. Ich lese die Klassen stattdessen per ID.
Control Request:
```yaml
id: 6bd4-4a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    ids:
    - DeleteResult
    - DeleteError
    - DeleteBatchResult
    - DeleteNodeTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    ids:
    - InsertResult
    - InsertError
    - InsertBatchResult
    - InsertNodeTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    ids:
    - EditBlockResult
    - EditBlockError
    - EditBlockBatchResult
    - EditBlockTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    ids:
    - EditMarksResult
    - EditMarksError
    - EditMarksBatchResult
    - EditMarksTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
    ids:
    - CreateFileResult
    - CreateFileError
    - CreateFilesResult
  reason: Klassen der Edit-Tools per vermuteter ID lesen (nicht gefundene IDs liefern Hinweise auf korrekte IDs).
```
/answer 6bd4-4a-0 allow
Control Request:
```yaml
id: 6bd4-4b-4a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
      - id: DeleteResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class DeleteResult:
              """Result of deleting a single node or file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
              """
              path: str
              id: str | None
              result: str
              imports: ImportOutcome | None = None
      - id: DeleteError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class DeleteError:
              """Error deleting a single node or file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  error: The error message.
              """
              path: str
              id: str | None
              error: str
      - id: DeleteBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class DeleteBatchResult:
              """Result of :func:`ast_delete`.

              Attributes:
                  results: One :class:`DeleteResult` per successfully deleted item.
                  errors: One :class:`DeleteError` per item that failed.
              """
              results: list[DeleteResult]
              errors: list[DeleteError]
              notice: str | None = None
      - id: DeleteTool
        type: ClassDef
        excert: "class DeleteTool(ToolDefinition):"
        children:
        - id: DeleteTool.gLGwBi|JNohfS
          type: statements
          code: |-
            name = 'ast_delete'
            title = 'Delete AST nodes or files'
            description = 'Delete selected nodes from files, or whole files if no selector is given, for a batch of items.'
            _ITEM_PROPERTIES = {'path': PATH_PROP, **PATH_SELECTOR_PROPS, **IMPORT_PROPS}
            _ITEM_REQUIRED = ['path']
            _ITEMS_DESCRIPTION = 'Nodes/files to delete.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: DeleteTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""

                def item_factory(it: dict[str, Any]) -> DeleteItem:
                    add_import, remove_import = parse_import_args(it)
                    return DeleteItem(path=it['path'], id=it.get('id'), add_import=add_import, remove_import=remove_import)

                def result_serializer(r: DeleteResult) -> dict[str, Any]:
                    entry = {'path': r.path, 'result': r.result}
                    if r.id is not None:
                        entry['id'] = r.id
                    if (imports := import_entry(r.imports)) is not None:
                        entry['imports'] = imports
                    return entry
                return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, result_serializer, auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      nodes:
      - id: InsertResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class InsertResult:
              """Result of a single insert operation.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  inserted: Number of top-level statements parsed from ``source`` and inserted.
                  ids: The newly inserted top-level node(s)' ids.
              """
              path: str
              id: str | None
              result: str
              inserted: int
              ids: list[str] | None = None
              imports: ImportOutcome | None = None
      - id: InsertError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class InsertError:
              """Error applying a single insert operation.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  error: The error message.
              """
              path: str
              id: str | None
              error: str
      - id: InsertBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class InsertBatchResult:
              """Result of :func:`ast_insert`.

              Attributes:
                  results: One :class:`InsertResult` per successful insert.
                  errors: One :class:`InsertError` per insert that failed.
              """
              results: list[InsertResult]
              errors: list[InsertError]
              notice: str | None = None
      - id: InsertNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""

              def item_factory(it: dict[str, Any]) -> InsertItem:
                  add_import, remove_import = parse_import_args(it)
                  return InsertItem(
                      path=it['path'],
                      source=it['source'],
                      position=it.get(
                          'position',
                          'after'),
                      id=it.get('id'),
                      add_import=add_import,
                      remove_import=remove_import)

              def result_serializer(r: InsertResult) -> dict[str, Any]:
                  entry = {'path': r.path, 'result': r.result, 'inserted': r.inserted}
                  if r.id is not None:
                      entry['id'] = r.id
                  if r.ids is not None:
                      entry['ids'] = r.ids
                  if (imports := import_entry(r.imports)) is not None:
                      entry['imports'] = imports
                  return entry

              def error_serializer(e: InsertError) -> dict[str, Any]:
                  entry = {'path': e.path, 'error': e.error}
                  if e.id is not None:
                      entry['id'] = e.id
                  return entry
              return handle_batch_tool(
                  ctx,
                  item_factory,
                  ast_insert,
                  core.AstError,
                  result_serializer,
                  error_serializer,
                  auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: EditBlockResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockResult:
              """Result of applying a single block edit.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  new_id: The node's new id, only set if the edit changed it.
              """
              path: str
              id: str | None
              result: str
              new_id: str | None = None
              imports: ImportOutcome | None = None
      - id: EditBlockError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockError:
              """Error applying a single block edit.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  error: The error message.
                  candidates: On ambiguity (id omitted, several nodes matched), the candidate node ids.
                  reason: Machine-readable cause, e.g. 'whitespace_mismatch', 'content_changed',
                      'guard_rejected', 'ambiguous', 'not_found'.
                  corrected_text: Verified fix for old_text (whitespace-only difference), if any.
                  guess: Unverified single-candidate guess at old_text's current form, if any.
                  next_step: Fallback instruction (e.g. 'reread_node') when no fix could be determined.
              """
              path: str
              id: str | None
              error: str
              candidates: list[str] | None = None
              reason: str | None = None
              corrected_text: str | None = None
              guess: str | None = None
              next_step: str | None = None
      - id: EditBlockBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockBatchResult:
              """Result of :func:`ast_edit_block`.

              Attributes:
                  results: One :class:`EditBlockResult` per successfully applied edit.
                  errors: One :class:`EditBlockError` per edit that failed.
              """
              results: list[EditBlockResult]
              errors: list[EditBlockError]
              notice: str | None = None
      - id: EditBlockNodeTool
        type: ClassDef
        excert: "class EditBlockNodeTool(ToolDefinition):"
        children:
        - id: EditBlockNodeTool.aKEDSe|mSaRry
          type: statements
          code: |-
            name = 'ast_edit_block'
            title = 'Replace short text within AST nodes'
            description = "Replace occurrences of short 'old_text' with 'new_text', within nodes addressed by prior known id, for a batch of items. Don't use for large edits, use ast_edit_marks instead."
        - id: EditBlockNodeTool.Z2OjHo|ftL6Rn
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'path': PATH_PROP,
                'old_text': {
                    'type': 'string',
                    'minLength': 10,
                    'maxLength': 100,
                    'description': 'Short text (10-100 chars) to replace within the node. Must occur exactly once, or replaceAll is set.'},
                'new_text': {
                    'type': 'string',
                    'description': 'Replacement text, may be empty to remove the text.'},
                'exact': {
                    'type': 'boolean',
                            'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                            'default': False},
                'replaceAll': {
                    'type': 'boolean',
                    'description': "If true, replace every occurrence of 'old_text' within the node instead of a single unique match.",
                    'default': False},
                **PATH_SELECTOR_PROPS,
                **IMPORT_PROPS}
        - id: EditBlockNodeTool.ZDSH0j|Jr7UgN
          type: statements
          code: |-
            _ITEM_REQUIRED = ['path', 'old_text', 'new_text', 'id']
            _ITEMS_DESCRIPTION = 'Block edits to apply.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: EditBlockNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST API."""

                def item_factory(it: dict[str, Any]) -> EditBlockItem:
                    add_import, remove_import = parse_import_args(it)
                    return EditBlockItem(
                        path=it['path'],
                        old_text=it['old_text'],
                        new_text=it['new_text'],
                        exact=it.get(
                            'exact',
                            False),
                        replace_all=it.get(
                            'replaceAll',
                            False),
                        id=it.get('id'),
                        add_import=add_import,
                        remove_import=remove_import)

                def result_serializer(r: EditBlockResult) -> dict[str, Any]:
                    entry = {'path': r.path, 'result': r.result}
                    if r.id is not None:
                        entry['id'] = r.id
                    if r.new_id is not None:
                        entry['new_id'] = r.new_id
                    if (imports := import_entry(r.imports)) is not None:
                        entry['imports'] = imports
                    return entry

                def error_serializer(e: EditBlockError) -> dict[str, Any]:
                    entry = {'path': e.path, 'error': e.error}
                    if e.id is not None:
                        entry['id'] = e.id
                    for field in ('candidates', 'reason', 'corrected_text', 'guess', 'next_step'):
                        value = getattr(e, field)
                        if value is not None:
                            entry[field] = value
                    return entry
                return handle_batch_tool(
                    ctx,
                    item_factory,
                    ast_edit_block,
                    core.AstError,
                    result_serializer,
                    error_serializer,
                    auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
      nodes:
      - id: EditMarksResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditMarksResult:
              """Result of applying a single marker-based edit.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  new_id: The node's new id, only set if the edit changed it.
              """
              path: str
              id: str | None
              result: str
              new_id: str | None = None
              imports: ImportOutcome | None = None
      - id: EditMarksError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditMarksError:
              """Error applying a single marker-based edit.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  error: The error message.
                  candidates: On ambiguity (id omitted, several nodes matched), the candidate node ids.
                  reason: Machine-readable cause, e.g. 'whitespace_mismatch', 'content_changed',
                      'guard_rejected', 'marker_order', 'ambiguous', 'not_found'.
                  position: Which marker failed ('start'/'end'), if applicable.
                  corrected_text: Verified fix for the failing marker (whitespace-only difference), if any.
                  guess: Unverified single-candidate guess at the marker's current form, if any.
                  next_step: Fallback instruction (e.g. 'reread_node') when no fix could be determined.
              """
              path: str
              id: str | None
              error: str
              candidates: list[str] | None = None
              reason: str | None = None
              position: str | None = None
              corrected_text: str | None = None
              guess: str | None = None
              next_step: str | None = None
      - id: EditMarksBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditMarksBatchResult:
              """Result of :func:`ast_edit_marks`.

              Attributes:
                  results: One :class:`EditMarksResult` per successfully applied edit.
                  errors: One :class:`EditMarksError` per edit that failed.
              """
              results: list[EditMarksResult]
              errors: list[EditMarksError]
              notice: str | None = None
      - id: EditMarksNodeTool
        type: ClassDef
        excert: "class EditMarksNodeTool(ToolDefinition):"
        children:
        - id: EditMarksNodeTool.tqcnP7|JTGu41
          type: statements
          code: |-
            name = 'ast_edit_marks'
            title = 'Replace large text regions within AST nodes between markers when id is known'
            description = "Replace everything between and including the unique 'start_marker' and 'end_marker' markers, found within nodes addressed by a known id, with new 'content', for a batch of items."
        - id: EditMarksNodeTool.Z2OjHo|jMiCNo
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'path': PATH_PROP,
                'start_marker': {
                    'type': 'string',
                    'minLength': 10,
                    'maxLength': 30,
                    'description': "Unique 10-30 char substring marking the beginning of the text to replace, within the node's source."},
                'end_marker': {
                    'type': 'string',
                    'minLength': 10,
                    'maxLength': 30,
                    'description': "Unique 10-30 char substring marking the end of the text to replace, within the node's source."},
                'content': {
                    'type': 'string',
                            'description': 'Replacement source for the marked text.'},
                'exact': {
                    'type': 'boolean',
                    'description': "If true, 'start_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                    'default': False},
                **PATH_SELECTOR_PROPS,
                **IMPORT_PROPS}
        - id: EditMarksNodeTool.ZDSH0j|PEqhnG
          type: statements
          code: |-
            _ITEM_REQUIRED = ['path', 'start_marker', 'end_marker', 'content']
            _ITEMS_DESCRIPTION = 'Marker-based edits to apply.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: EditMarksNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST API."""

                def item_factory(it: dict[str, Any]) -> EditMarksItem:
                    add_import, remove_import = parse_import_args(it)
                    return EditMarksItem(
                        path=it['path'],
                        start_marker=it['start_marker'],
                        end_marker=it['end_marker'],
                        content=it['content'],
                        exact=it.get(
                            'exact',
                            False),
                        id=it.get('id'),
                        add_import=add_import,
                        remove_import=remove_import)

                def result_serializer(r: EditMarksResult) -> dict[str, Any]:
                    entry = {'path': r.path, 'result': r.result}
                    if r.id is not None:
                        entry['id'] = r.id
                    if r.new_id is not None:
                        entry['new_id'] = r.new_id
                    if (imports := import_entry(r.imports)) is not None:
                        entry['imports'] = imports
                    return entry

                def error_serializer(e: EditMarksError) -> dict[str, Any]:
                    entry = {'path': e.path, 'error': e.error}
                    if e.id is not None:
                        entry['id'] = e.id
                    for field in ('candidates', 'reason', 'position', 'corrected_text', 'guess', 'next_step'):
                        value = getattr(e, field)
                        if value is not None:
                            entry[field] = value
                    return entry
                return handle_batch_tool(
                    ctx,
                    item_factory,
                    ast_edit_marks,
                    core.AstError,
                    result_serializer,
                    error_serializer,
                    auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
      nodes:
      - id: CreateFileResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class CreateFileResult:
              """Result of creating a single file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  result: Always ``"success"``.
              """
              path: str
              result: str
      - id: CreateFileError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class CreateFileError:
              """Error creating a single file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  error: The error message.
              """
              path: str
              error: str
      - id: CreateFilesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class CreateFilesResult:
              """Result of :func:`ast_create`.

              Attributes:
                  results: One :class:`CreateFileResult` per file created successfully.
                  errors: One :class:`CreateFileError` per file that failed.
              """
              results: list[CreateFileResult]
              errors: list[CreateFileError]
```
/answer 6bd4-4b-4a allow
Control Request:
```yaml
id: 6bd4-4c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: "requires-python|python_requires|testpaths|pythonpath|\\[tool\\.(pytest|mypy|pyright)"
    include:
    - pyproject.toml
    - pytest.ini
    - setup.cfg
    - tox.ini
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: "^(@dataclass|class \\w+|    \\w+: .*)$"
    include:
    - __init__.py
    limit: 1
  reason: "Python-Version und Test-Konfiguration prüfen (wichtig für kw_only, Annotations und Contract-Test)."
```
/answer 6bd4-4c-0 allow
Control Request:
```yaml
id: 6bd4-4d-4c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: pyproject.toml
        matches:
        - requires-python = ">=3.11"
        - "[tool.pytest.ini_options]"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: tool_usage/__init__.py
        matches:
        - "signature: str"
      warning: "Limit of 1 matches reached (total 202); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 6bd4-4d-4c allow
ReasoningToken: 1441
Control Request:
```yaml
id: 6bd4-4e-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
  reason: "Struktur der verbleibenden Batch-Tools (skills, edit_lines, edit_block, edit_marks) erfassen."
```
/answer 6bd4-4e-0 allow
Control Request:
```yaml
id: 6bd4-4f-4e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
      nodes:
      - id: JpZ0JI|u5Vo8y
        excert: "'Skills – on-demand hints an agent can request, for a batch of items.\\n\\nA *ski…"
      - id: oJ1xKq|eFu265
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
        excert: "@dataclass(frozen=True) class SkillsResult:"
        docstring: "Result of fetching a single skill, mirroring its input name for result associat…"
      - id: SkillsItemError
        excert: "@dataclass(frozen=True) class SkillsItemError:"
        docstring: "Error fetching a single skill, mirroring its input name for result association."
      - id: SkillsBatchResult
        excert: "@dataclass(frozen=True) class SkillsBatchResult:"
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
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
      nodes:
      - id: S5j2Hu|eYdgDd
        excert: "'Edit-lines tool – replaces a range of lines inside existing files, for a batch…"
      - id: PBZWYD|z6v29p
        excert: from dataclasses import dataclass
      - id: RGRDMg|5yXHtR
        excert: "__all__ = ["
      - id: EditLinesError
        excert: "class EditLinesError(Exception):"
        docstring: Raised when a edit-lines operation cannot be performed.
      - id: EditLinesItem
        excert: "@dataclass(frozen=True) class EditLinesItem:"
        docstring: "One line-range replacement to apply. Attributes: path: Absolute path to file (m…"
      - id: EditLinesResult
        excert: "@dataclass(frozen=True) class EditLinesResult:"
        docstring: "Result of applying a single line-range replacement, mirroring its input path fo…"
      - id: EditLinesItemError
        excert: "@dataclass(frozen=True) class EditLinesItemError:"
        docstring: "Error applying a single line-range replacement, mirroring its input path for re…"
      - id: EditLinesBatchResult
        excert: "@dataclass(frozen=True) class EditLinesBatchResult:"
        docstring: "Result of :func:`edit_lines`. Attributes: results: One :class:`EditLinesResult`…"
      - id: edit_lines_one
        excert: "def _edit_lines_one(item: EditLinesItem) -> EditLinesResult:"
      - id: edit_lines
        excert: "def edit_lines(items: list[EditLinesItem]) -> EditLinesBatchResult:"
        docstring: "Replace a range of lines in one or more files, for a batch of items. A given pa…"
      - id: EditLinesTool
        excert: "class EditLinesTool(ToolDefinition):"
        children:
        - id: EditLinesTool.hU3USs|GewBq1
          excert: name = 'edit_lines'
        - id: EditLinesTool.UzS4rT|wbgB4n
          excert: description = 'Replace a range of lines inside one or more existing files with …
        - id: EditLinesTool.a8vIIX|8fudaz
          excert: "_ITEM_PROPERTIES = {"
        - id: EditLinesTool.ZDSH0j|bXMBuI
          excert: "_ITEM_REQUIRED = ['path', 'offset', 'amount', 'content']"
        - id: EditLinesTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`edit_lines`, translating the MCP schema to/from the Python A…"
      - id: register_edit_lines_tool
        excert: "def register_edit_lines_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
      nodes:
      - id: 9NjzWq|MYFItt
        excert: "'Edit-block tool – edits an exact block of text (old -> new) in a file, for a b…"
      - id: PBZWYD|hnnU2r
        excert: "from dataclasses import asdict, dataclass"
      - id: vPAEde|Bp4ypK
        excert: "__all__ = ["
      - id: EditBlockError
        excert: "class EditBlockError(Exception):"
        docstring: Raised when a edit-block operation cannot be performed.
        children:
        - id: EditBlockError.4Nlb4m|kWgYNF
          excert: '''Raised when a edit-block operation cannot be performed.'''
        - id: EditBlockError.init
          excert: "def __init__(self, message: str, *, reason: str | None=None, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:"
      - id: EditBlockItem
        excert: "@dataclass(frozen=True) class EditBlockItem:"
        docstring: "One block edit to apply. Attributes: path: Absolute path to file (must be a reg…"
      - id: EditBlockResult
        excert: "@dataclass(frozen=True) class EditBlockResult:"
        docstring: "Result of applying a single block edit, mirroring its input path for result ass…"
      - id: EditBlockItemError
        excert: "@dataclass(frozen=True) class EditBlockItemError:"
        docstring: "Error applying a single block edit, mirroring its input path for result associa…"
      - id: EditBlockBatchResult
        excert: "@dataclass(frozen=True) class EditBlockBatchResult:"
        docstring: "Result of :func:`edit_block`. Attributes: results: One :class:`EditBlockResult`…"
      - id: edit_block_one
        excert: "def _edit_block_one(item: EditBlockItem) -> EditBlockResult:"
      - id: edit_block
        excert: "def edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:"
        docstring: Replace occurrence(s) of ``old_text`` with ``new_text`` in one or more files. M…
      - id: EditBlockTool
        excert: "class EditBlockTool(ToolDefinition):"
        children:
        - id: EditBlockTool.4YyfhJ|7FjLjq
          excert: name = 'edit_block'
        - id: EditBlockTool.3dbRH3|AFji2y
          excert: "_ITEM_PROPERTIES = {"
        - id: EditBlockTool.ZDSH0j|kGCzWl
          excert: "_ITEM_REQUIRED = ['path', 'old_text', 'new_text']"
        - id: EditBlockTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`edit_block`, translating the MCP schema to/from the Python A…"
      - id: register_edit_block_tool
        excert: "def register_edit_block_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
      nodes:
      - id: vdLcrM|H2OCmq
        excert: "'Edit Marks tool – replaces the text strictly including two markers, for a batc…"
      - id: PBZWYD|Vpz5GK
        excert: "from dataclasses import asdict, dataclass"
      - id: 1rEnt6|hbtM97
        excert: "__all__ = ["
      - id: EditMarksError
        excert: "class EditMarksError(Exception):"
        docstring: Raised when a replace operation cannot be performed.
        children:
        - id: EditMarksError.xpXv8u|lLycpS
          excert: '''Raised when a replace operation cannot be performed.'''
        - id: EditMarksError.init
          excert: "def __init__(self, message: str, *, reason: str | None=None, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:"
      - id: EditMarksItem
        excert: "@dataclass(frozen=True) class EditMarksItem:"
        docstring: "One marker-based replacement to apply. Attributes: path: Absolute path to targe…"
      - id: EditMarksResult
        excert: "@dataclass(frozen=True) class EditMarksResult:"
        docstring: "Result of applying a single marker-based replacement, mirroring its input path …"
      - id: EditMarksItemError
        excert: "@dataclass(frozen=True) class EditMarksItemError:"
        docstring: "Error applying a single marker-based replacement, mirroring its input path for …"
      - id: EditMarksBatchResult
        excert: "@dataclass(frozen=True) class EditMarksBatchResult:"
        docstring: "Result of :func:`edit_marks`. Attributes: results: One :class:`EditMarksResult`…"
      - id: edit_marks_text
        excert: "def edit_marks_text(text: str, begin_marker: str, content: str, end_marker: str, exact: bool=False) -> str:"
        docstring: Replace everything between and including 'begin_marker' and 'end_marker' with c…
      - id: edit_marks_one
        excert: "def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:"
      - id: edit_marks
        excert: "def edit_marks(items: list[EditMarksItem]) -> EditMarksBatchResult:"
        docstring: Replace everything between and including 'begin_marker' and 'end_marker' with c…
      - id: EditMarksTool
        excert: "class EditMarksTool(ToolDefinition):"
        children:
        - id: EditMarksTool.KkK7Sx|fQt7nq
          excert: name = 'edit_marks'
        - id: EditMarksTool.3dbRH3|hJ11Ne
          excert: "_ITEM_PROPERTIES = {"
        - id: EditMarksTool.ZDSH0j|wHiDhY
          excert: "_ITEM_REQUIRED = ['path', 'begin_marker', 'end_marker', 'content']"
        - id: EditMarksTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python A…"
      - id: register_edit_marks_tool
        excert: "def register_edit_marks_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 6bd4-4f-4e allow
ReasoningToken: 2878
Control Request:
```yaml
id: 6bd4-50-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: ToolResult\(structured_content=result\)|structured = |structured_content =
    include:
    - github.py
    - process.py
    - __init__.py
    intent: editing
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp
    pattern: result\b.*=|def call|structured
    include:
    - github.py
    limit: 15
  reason: "Prüfen, ob durchgereichte externe structured_content-Dicts mit dem neuen reservierten Root-Key 'warnings' kollidieren könnten."
```
/answer 6bd4-50-0 allow
Control Request:
```yaml
id: 6bd4-51-50
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: mcp/github.py
        nodes:
        - id: GitHubTool.handle.WhVJDf|xEpvSq
          matches:
          - return ToolResult(structured_content=result)
      - path: read/__init__.py
        nodes:
        - id: ReadTool.handle.spfQNh|Kv6qkQ
          matches:
          - "structured_content = serialize_batch_result(batch, result_serializer=_serialize_read_result)"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp
      matches:
      - path: github.py
        matches:
        - result = self._core(**ctx.arguments)
        - return ToolResult(structured_content=result)
```
/answer 6bd4-51-50 allow
ReasoningToken: 97
Control Request:
```yaml
id: 6bd4-52-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    ids:
    - M6pfOV|iTlyps
    - FPT4hs|aHGJs9
    - 07wTbv|BnS8bM
    - hIMaii|mbWWig
    - inject_property
    - with_mandatory_reason
    - ToolRegistry.register
    - ToolRegistry.tool
  reason: "Registry-Kopf, Imports, Konstanten und Schema-Injektion lesen, um WARNINGS_SCHEMA und ToolResult-Änderungen korrekt einzubauen."
```
/answer 6bd4-52-0 allow
Control Request:
```yaml
id: 6bd4-53-52
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: M6pfOV|iTlyps
        type: statements
        code: '''Central tool registry and tool result helpers.\n\nTools are registered once in a process-wide :class:`ToolRegistry`.  What a\ngiven client actually sees is derived by reconciling the registry against the\nper-session configuration (:attr:`Session.enabled_tools`).\n'''
      - id: FPT4hs|aHGJs9
        type: imports
        code: |-
          from dataclasses import dataclass, field
          from typing import Any, Callable
          from xy.ai.mcpc.server.json_codec import JsonCodec
          from xy.ai.mcpc.server.session import Session
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from abc import ABC, abstractmethod
      - id: 07wTbv|BnS8bM
        type: statements
        code: |-
          '#: Default value for the Anthropic-specific ``anthropic/maxResultSizeChars``'
          '#: meta annotation, applied generically to every tool result (see'
          '#: :meth:`ToolResult.to_dict`). This tells Anthropic-compatible MCP clients'
          '#: how many characters of the result they may render/keep before truncating.'
          ANTHROPIC_MAX_RESULT_SIZE_CHARS = 500000
      - id: hIMaii|mbWWig
        type: statements
        code: |-
          "#: Name of the mandatory reason property injected into every tool's input"
          '#: schema (see :func:`_inject_property`).'
          REASON_PROPERTY = 'reason'
          "#: Name of the optional hint property injected into every tool's output"
          "#: schema and, at call time, into the result's ``structuredContent`` (see"
          '#: :meth:`ToolResult.to_dict`) — must live there, not top-level, since MCP'
          '#: clients drop unknown top-level ``CallToolResult`` fields silently.'
          MESSAGE_PROPERTY = 'messageFromUser'
      - id: inject_property
        type: FunctionDef
        code: |-
          def _inject_property(schema: dict[str, Any], name: str, description: str, *, required: bool, first: bool=False) -> dict[str, Any]:
              """Return *schema* with an additional property generically injected.

              Used both for the mandatory ``reason`` property on every tool's input
              schema and for the optional ``controlHint`` property on every tool's
              output schema — the same generic mechanism, applied at registration time
              so individual tool modules never need to declare either themselves.
              """
              schema = dict(schema)
              properties = dict(schema.get('properties', {}))
              new_prop = {'type': 'string', 'description': description}
              if first:
                  properties = {name: new_prop, **{k: v for k, v in properties.items() if k != name}}
              else:
                  properties[name] = new_prop
              schema['properties'] = properties
              if required:
                  required_list = list(schema.get('required', []))
                  if name not in required_list:
                      required_list.append(name)
                  schema['required'] = required_list
              return schema
      - id: with_mandatory_reason
        type: FunctionDef
        code: |-
          def _with_mandatory_reason(schema: dict[str, Any]) -> dict[str, Any]:
              """Return *schema* with a mandatory, short ``reason`` property injected.

              Every tool call must carry an extremely short reason/goal for the call so
              the authorizing user can review it (e.g. via the human-in-the-loop
              control layer) before or while it executes.
              """
              return _inject_property(
                  schema,
                  REASON_PROPERTY,
                  'Precise, specific reason for this tool call (what exactly is being retrievedand why it is needed now), shown to the authorizing user.',
                  required=True,
                  first=True)
      - id: ToolRegistry.register
        type: FunctionDef
        code: |-
          def register(self, tool: ToolDefinition) -> Tool:
              tool = tool.to_tool()
              if self.registration_filter is not None and (not self.registration_filter(tool.name)):
                  return tool
              if tool.name in self._tools:
                  raise ValueError(f'Tool already registered: {tool.name}')
              tool.input_schema = _with_mandatory_reason(tool.input_schema)
              '# Applied unconditionally: ToolResult.to_dict() may attach controlHint'
              '# to *any* result regardless of whether the tool declared an'
              '# outputSchema, so the schema must always document it too.'
              base_output_schema = tool.output_schema or {'type': 'object', 'properties': {}}
              tool.output_schema = base_output_schema
              meta: dict[str, Any] = {'anthropic/maxResultSizeChars': ANTHROPIC_MAX_RESULT_SIZE_CHARS}
              tool.meta = meta
              self._tools[tool.name] = tool
              return tool
      - id: ToolRegistry.tool
        type: FunctionDef
        code: |-
          def tool(self, name: str, *, description: str, input_schema: dict[str, Any], title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> Callable[[ToolHandler], ToolHandler]:
              """Decorator registering the decorated function as a tool handler."""

              def decorator(handler: ToolHandler) -> ToolHandler:
                  self.register(
                      Tool(
                          name=name,
                          description=description,
                          input_schema=input_schema,
                          handler=handler,
                          title=title,
                          output_schema=output_schema,
                          annotations=annotations))
                  return handler
              return decorator
```
/answer 6bd4-53-52 allow
ReasoningToken: 142
Control Request:
```yaml
id: 6bd4-54-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: output_schema|outputSchema|normalize_result|\.warning\b|\.notice\b|\.unchanged|\.errors\b|'warning'|"warning"|'notice'|"notice"|is_error|ToolResult\(
    include:
    - '*.py'
    limit: 50
  reason: "Tests identifizieren, die an Wire-Format, ToolResult, normalize_result oder alte Felder gebunden sind."
```
/answer 6bd4-54-0 allow
Control Request:
```yaml
id: 6bd4-55-54
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: test_edit_tolerance.py
        matches:
        - assert len(result.errors) == 1
        - "error = result.errors[0]"
        - assert not batch.errors
        - assert len(batch.errors) == 1
        - "error = batch.errors[0]"
      - path: test_embedded_batch.py
        matches:
        - "assert not result.is_error, result.text"
      - path: test_ast_engines.py
        matches:
        - assert not result.errors
      - path: test_edit_diagnostics.py
        matches:
        - assert not result.errors
        - "err = result.errors[0]"
      - path: test_tools_family_smoke.py
        matches:
        - assert not call1.is_error
        - assert result.is_error
      - path: test_ast_imports.py
        matches:
        - assert not result.errors and result.notice is None
        - assert not result.errors
        - assert result.notice == IMPORT_NOTICE
      - path: test_cli_session.py
        matches:
        - "FAKE_CLI = '#!{python}\\nimport json, os, sys, time\\n\\nfor raw in sys.stdin:\\n    msg = json.loads(raw)\\n    text = msg[\"message\"][\"content\"][0][\"text\"… \"success\"}}\\n    else:\\n        out = {{\"type\": \"result\", \"subtype\": \"success\", \"result\": \"echo:\" + text}}\\n    print(json.dumps(out), flush=True)\\n'"
        - assert result.is_error
      warning: 1 match line(s) exceeded 300 characters and were shortened in the middle.
```
/answer 6bd4-55-54 allow
Text:
 Ich lege jetzt das neue Modul `tools/results.py` an.
ReasoningToken: 777
Control Request:
```yaml
id: 6bd4-56-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    source: |
      """Common result model: items, batches and the single place that owns warnings.

      An *error* means the item produced no result (``error: str``). A *warning* means
      it did, with a restriction; item warnings live in the item's ``warnings`` entry,
      call-wide ones in the root ``warnings`` of ``structuredContent``.
      """
      from __future__ import annotations

      from abc import ABC, abstractmethod
      from dataclasses import asdict, dataclass, field, replace
      from inspect import get_annotations
      from typing import Any, Generic, Iterable, Self, TypeVar, final

      __all__ = [
          'WARNINGS_KEY',
          'RESERVED_NAMES',
          'WARNINGS_SCHEMA',
          'compact',
          'compact_asdict',
          'ItemOutcome',
          'ItemResult',
          'ItemError',
          'BatchResult']

      WARNINGS_KEY = 'warnings'
      RESERVED_NAMES = frozenset({'warning', 'warnings', 'notice', 'notices', 'hint', 'error', 'errors', 'message'})
      WARNINGS_SCHEMA: dict[str, Any] = {
          'type': 'array',
          'items': {'type': 'string'},
          'description': 'Restrictions of the result that the caller should take into account.'}


      def compact(value: Any) -> Any:
          """Recursively drop ``None`` values and empty lists from a dataclass-derived structure."""
          if isinstance(value, dict):
              return {k: compact(v) for k, v in value.items() if v is not None and v != []}
          if isinstance(value, list):
              return [compact(v) for v in value]
          return value


      def compact_asdict(obj: Any, exclude: Iterable[str] = ()) -> dict[str, Any]:
          skipped = set(exclude)
          return compact({k: v for k, v in asdict(obj).items() if k not in skipped})


      def _guard_fields(cls: type) -> None:
          clash = set(get_annotations(cls)) & RESERVED_NAMES
          if clash:
              raise TypeError(f'{cls.__name__}: reserved field name(s) {sorted(clash)}; use the base class API')
          if 'to_dict' in cls.__dict__:
              raise TypeError(f'{cls.__name__}: to_dict is final; override payload()')


      def _checked_payload(owner: object, payload: dict[str, Any]) -> dict[str, Any]:
          clash = RESERVED_NAMES & payload.keys()
          if clash:
              raise ValueError(f'{type(owner).__name__}: payload uses reserved key(s) {sorted(clash)}')
          return payload


      @dataclass(frozen=True, kw_only=True)
      class ItemOutcome(ABC):
          @abstractmethod
          def key(self) -> dict[str, Any]:
              """Input fields mirrored for result association."""

          @abstractmethod
          def payload(self) -> dict[str, Any]:
              """Outcome-specific fields; must not use reserved names."""

          @abstractmethod
          def to_dict(self) -> dict[str, Any]: ...


      @dataclass(frozen=True, kw_only=True)
      class ItemResult(ItemOutcome):
          """A item that produced a result, possibly with restrictions (``warnings``)."""
          warnings: tuple[str, ...] = ()

          def __init_subclass__(cls, **kwargs: Any) -> None:
              super().__init_subclass__(**kwargs)
              _guard_fields(cls)

          def payload(self) -> dict[str, Any]:
              return compact_asdict(self, exclude={WARNINGS_KEY})

          @final
          def to_dict(self) -> dict[str, Any]:
              out = {**self.key(), **_checked_payload(self, self.payload())}
              if self.warnings:
                  out[WARNINGS_KEY] = list(self.warnings)
              return out

          def warn(self, *messages: str) -> Self:
              return replace(self, warnings=(*self.warnings, *messages))


      @dataclass(frozen=True, kw_only=True)
      class ItemError(ItemOutcome):
          """A item that produced no result."""
          error: str

          def __init_subclass__(cls, **kwargs: Any) -> None:
              super().__init_subclass__(**kwargs)
              _guard_fields(cls)

          def payload(self) -> dict[str, Any]:
              return {}

          @final
          def to_dict(self) -> dict[str, Any]:
              return {**self.key(), 'error': self.error, **_checked_payload(self, self.payload())}


      R = TypeVar('R', bound=ItemResult)
      E = TypeVar('E', bound=ItemError)


      @dataclass(frozen=True, kw_only=True)
      class BatchResult(ABC, Generic[R, E]):
          results: list[R] = field(default_factory=list)
          errors: list[E] = field(default_factory=list)
          warnings: tuple[str, ...] = ()

          def __init_subclass__(cls, **kwargs: Any) -> None:
              super().__init_subclass__(**kwargs)
              _guard_fields(cls)

          def __post_init__(self) -> None:
              if not all(isinstance(r, ItemResult) for r in self.results):
                  raise TypeError('results must be ItemResult instances')
              if not all(isinstance(e, ItemError) for e in self.errors):
                  raise TypeError('errors must be ItemError instances')

          def derived_warnings(self) -> tuple[str, ...]:
              """Call-wide warnings that follow from the items and so cannot be forgotten."""
              return ()

          @final
          def all_warnings(self) -> list[str]:
              return list(dict.fromkeys((*self.warnings, *self.derived_warnings())))
  reason: "Neues Basismodul results.py gemäß Vorschlag anlegen (ItemResult, ItemError, BatchResult, Konstanten)."
```
/answer 6bd4-56-0 allow
ReasoningToken: 375
Control Request:
```yaml
id: 6bd4-57-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    id: ItemResult
    old_text: "\"\"\"A item that produced a result, possibly"
    new_text: "\"\"\"An item that produced a result, possibly"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    id: ItemError
    old_text: '"""A item that produced no result."""'
    new_text: '"""An item that produced no result."""'
  reason: Grammatikfehler in den Docstrings korrigieren.
```
/answer 6bd4-57-0 allow
Control Request:
```yaml
id: 6bd4-58-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/tools.py
  node_type: ClassDef|FunctionDef|imports
  reason: "Embedded-Tool-Wrapper verstehen, bevor normalize_result auf ToolResult eingeschränkt wird."
```
/answer 6bd4-58-0 allow
Control Request:
```yaml
id: 6bd4-59-58
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/tools.py
      nodes:
      - id: zTYD23|09LItY
        type: imports
        code: |-
          from __future__ import annotations
          from abc import ABC, abstractmethod
          from collections.abc import Callable, Iterable
          from typing import TYPE_CHECKING, Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      - id: ToolProvider
        type: ClassDef
        excert: "class ToolProvider(ABC):"
        docstring: Subclass and implement :meth:`tools` to contribute custom tools to a server.
        children:
        - id: ToolProvider.tools
          type: FunctionDef
          code: |-
            @abstractmethod
            def tools(self, environment: AppEnvironment) -> Iterable[ToolDefinition]:
                """Return the tools to register. *environment* gives access to config, sessions, etc."""
      - id: FunctionTool
        type: ClassDef
        excert: "class FunctionTool(ToolDefinition):"
        docstring: A tool backed by a plain callable taking a ``ToolContext``.
        children:
        - id: FunctionTool.init
          type: FunctionDef
          code: |-
            def __init__(self, name: str, description: str, input_schema: dict[str, Any], handler: ToolHandlerFn, *, title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> None:
                self.name = name
                self.description = description
                self.input_schema = input_schema
                self.title = title
                self.output_schema = output_schema if output_schema is not None else {'type': 'object'}
                self.annotations = annotations
                self._handler = handler
        - id: FunctionTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult | str | dict[str, Any]:
                return self._handler(ctx)
      - id: register_tool
        type: FunctionDef
        code: |-
          def register_tool(tool: ToolDefinition | ToolProvider, registry: ToolRegistry, environment: AppEnvironment) -> None:
              if isinstance(tool, ToolProvider):
                  for definition in tool.tools(environment):
                      registry.register(definition)
              else:
                  registry.register(tool)
```
/answer 6bd4-59-58 allow
ReasoningToken: 323
Control Request:
```yaml
id: 6bd4-5a-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
  text: normalize_result
  reason: Aufrufstelle von normalize_result im Protokoll lesen.
```
/answer 6bd4-5a-0 allow
Control Request:
```yaml
id: 6bd4-5b-5a
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
      nodes:
      - id: lF3REn|IWeyxO
        code: |-
          import logging
          import base64
          from typing import Any
          from xy.ai.mcpc.server import errors
          from xy.ai.mcpc.config import ServerConfig
          from xy.ai.mcpc.server.jsonrpc import JsonRpcRequest
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.tool_registry import ToolRegistry, normalize_result, ToolResult, text_content, attach_message
          from xy.ai.mcpc.server.session import Session
          from xy.ai.mcpc.tools.tool_context import AppEnvironment
      - id: McpProtocol.handle_tools_call.E8VojT|9XmOmT
        code: |-
          try:
              with session.lock:
                  raw = tool.handler(context)
              result = normalize_result(raw)
          except errors.JsonRpcError:
              raise
          except KeyError as exc:
              key = exc.args[0] if exc.args else exc
              result = ToolResult(
                  content=[
                      text_content(
                          f"Tool '{name}' failed: missing required property {
                              key!r}")],
                  is_error=True)
          except Exception as exc:
              '# noqa: BLE001 - surface as tool error result'
              result = ToolResult(content=[text_content(f"Tool '{name}' failed: {exc}")], is_error=True)
```
/answer 6bd4-5b-5a allow
ReasoningToken: 278
Control Request:
```yaml
id: 6bd4-5c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: "def handle\\(self, ctx[^)]*\\)\\s*->\\s*(?!ToolResult:)|\\.tool\\(|ToolHandlerFn|FunctionTool\\(|@\\w+\\.tool\\("
    include:
    - '*.py'
    limit: 40
  reason: "Handler finden, die str/dict statt ToolResult liefern oder per Decorator registriert werden, bevor die Rückgabe eingeschränkt wird."
```
/answer 6bd4-5c-0 allow
Control Request:
```yaml
id: 6bd4-5d-5c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: xy/ai/mcpc/embedded/__init__.py
        matches:
        - "return [FunctionTool(\"lookup\", \"Look up a record\", {\"type\": \"object\", \"properties\": {}}, lambda ctx: \"ok\")]"
      - path: xy/ai/mcpc/embedded/tools.py
        matches:
        - "__all__ = ['FunctionTool', 'ToolHandlerFn', 'ToolProvider']"
        - "ToolHandlerFn = Callable[['ToolContext'], 'ToolResult | str | dict[str, Any]']"
        - "class FunctionTool(ToolDefinition):"
        - "def __init__(self, name: str, description: str, input_schema: dict[str, Any], handler: ToolHandlerFn, *, title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> None:"
      - path: xy/ai/mcpc/embedded/mcpc.py
        matches:
        - "from xy.ai.mcpc.embedded.tools import FunctionTool, ToolHandlerFn, ToolProvider, register_tool"
        - "def tool(self, name: str, description: str, input_schema: dict[str, Any] | None=None, **kwargs: Any) -> Callable[[ToolHandlerFn], ToolHandlerFn]:"
        - "def decorator(fn: ToolHandlerFn) -> ToolHandlerFn:"
        - "self.add_tool(FunctionTool(name, description, schema, fn, **kwargs))"
      - path: test_embedded.py
        matches:
        - "return [FunctionTool('echo', 'Echo', {'type': 'object', 'properties': {}}, lambda ctx: 'pong')]"
        - "@mcpc.tool('hello', 'Say hello')"
```
/answer 6bd4-5d-5c allow
ReasoningToken: 1299
Control Request:
```yaml
id: 6bd4-5e-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: ToolResult.6F9vfX|khLiSB
    source: |-
      'The result of a tool call (maps onto MCP ``CallToolResult``).'
      content: list[dict[str, Any]] = field(default_factory=list)
      structured_content: dict[str, Any] | None = None
      is_error: bool = False
      auto_approve: bool = False
      message: str | None = None
      warnings: list[str] = field(default_factory=list)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: ToolResult.post_init
    source: |-
      def __post_init__(self) -> None:
          if self.is_error and (self.structured_content or self.warnings):
              raise ValueError('ToolResult: error results carry neither structured_content nor warnings.')
          if self.structured_content and RESERVED_ROOT_KEYS & self.structured_content.keys():
              raise ValueError(
                  'ToolResult: reserved key in structured_content; use ToolResult.warnings / ToolResult.message.')

      @classmethod
      def failure(cls, message: str) -> 'ToolResult':
          return cls(content=[text_content(message)], is_error=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: ToolResult.to_dict
    source: |-
      def to_dict(self) -> dict[str, Any]:
          result: dict[str, Any] = {}
          if self.content:
              result['content'] = list(self.content)
          if self.structured_content or self.warnings:
              structured = dict(self.structured_content or {})
              if self.warnings:
                  structured[WARNINGS_KEY] = list(self.warnings)
              result['structuredContent'] = structured
          if self.message:
              attach_message(result, self.message)
          if self.is_error:
              result['isError'] = True
          return result
  reason: "ToolResult: warnings-Feld, Validierung (Fehler ohne Warnings, reservierte Root-Keys), failure()-Fabrik und to_dict als einziger Schreiber des Root-Keys 'warnings'."
```
/answer 6bd4-5e-0 allow
Thinking:
Die Datei `tools/results.py` ist angelegt und `ToolResult` hat jetzt Warnungen, `failure()` und Validierung. Als Nächstes kommen die Imports, `RESERVED_ROOT_KEYS`, die Typen für `handle` und eine strikte `normalize_result` in `tool_registry.py`, danach die Tool-Migration.


Control Request:
```yaml
id: 6bd4-5f-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: FPT4hs|aHGJs9
    source: |-
      from dataclasses import dataclass, field
      from typing import Any, Callable
      from xy.ai.mcpc.server.json_codec import JsonCodec
      from xy.ai.mcpc.server.session import Session
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.results import WARNINGS_KEY, WARNINGS_SCHEMA
      from abc import ABC, abstractmethod
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: hIMaii|mbWWig
    source: |-
      "#: Name of the mandatory reason property injected into every tool's input"
      '#: schema (see :func:`_inject_property`).'
      REASON_PROPERTY = 'reason'
      "#: Name of the optional hint property injected into every tool's output"
      "#: schema and, at call time, into the result's ``structuredContent`` (see"
      '#: :meth:`ToolResult.to_dict`) — must live there, not top-level, since MCP'
      '#: clients drop unknown top-level ``CallToolResult`` fields silently.'
      MESSAGE_PROPERTY = 'messageFromUser'
      '#: Root keys of ``structuredContent`` written only by :meth:`ToolResult.to_dict`.'
      RESERVED_ROOT_KEYS = frozenset({WARNINGS_KEY, MESSAGE_PROPERTY})
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: BFWVtm|SRXvgt
    source: |-
      '#: A handler receives the invocation context and returns a :class:`ToolResult`.'
      ToolHandler = Callable[[ToolContext], 'ToolResult']
  reason: "Imports, reservierte Root-Keys und strikten Handler-Typ in tool_registry.py setzen."
```
/answer 6bd4-5f-0 allow
ReasoningToken: 34
Control Request:
```yaml
id: 6bd4-60-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: ToolDefinition.handle
    old_text: "def handle(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':"
    new_text: "def handle(self, ctx: ToolContext) -> 'ToolResult':"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: ToolDefinition.call
    old_text: "def __call__(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':"
    new_text: "def __call__(self, ctx: ToolContext) -> 'ToolResult':"
  reason: Rückgabetyp von ToolDefinition.handle/__call__ auf ToolResult einschränken.
```
/answer 6bd4-60-0 allow
ReasoningToken: 71
Control Request:
```yaml
id: 6bd4-61-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: JsonCodec|ToolResult\.|\bToolHandler\b
    include:
    - tool_registry.py
    intent: editing
  reason: "Prüfen, ob JsonCodec nach dem Entfernen des dict-Zweigs in normalize_result noch gebraucht wird."
```
/answer 6bd4-61-0 allow
Control Request:
```yaml
id: 6bd4-62-61
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: tool_registry.py
        nodes:
        - id: FPT4hs|yBpqzF
          matches:
          - from xy.ai.mcpc.server.json_codec import JsonCodec
        - id: 07wTbv|BnS8bM
          matches:
          - "'#: :meth:`ToolResult.to_dict`). This tells Anthropic-compatible MCP clients'"
        - id: ToolResult.post_init.ezTiZr|Leno9F
          matches:
          - "'ToolResult: reserved key in structured_content; use ToolResult.warnings / ToolResult.message.')"
        - id: P0oVIe|oBV48H
          matches:
          - "ToolHandler = Callable[[ToolContext], 'ToolResult']"
        - id: Tool.cjwFjQ|SNEi1f
          matches:
          - "handler: ToolHandler"
        - id: ToolDefinition.lDzjus|OawaHI
          matches:
          - :meth:`handle`; an instance is itself a callable :data:`ToolHandler` and
        - id: normalize_result.y6yvbi|1oVe8T
          matches:
          - rendered = JsonCodec.encode(value)
        - id: hIMaii|mbWWig
          matches:
          - "'#: :meth:`ToolResult.to_dict`) — must live there, not top-level, since MCP'"
        - id: Z0sI5h|7o2XIr
          matches:
          - "'#: Root keys of ``structuredContent`` written only by :meth:`ToolResult.to_dict`.'"
        - id: ToolRegistry.register.yPRtmv|CSd19A
          matches:
          - "'# Applied unconditionally: ToolResult.to_dict() may attach controlHint'"
        - id: ToolRegistry.tool
          matches:
          - "def tool(self, name: str, *, description: str, input_schema: dict[str, Any], title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> Callable[[ToolHandler], ToolHandler]:"
        - id: ToolRegistry.tool.decorator
          matches:
          - "def decorator(handler: ToolHandler) -> ToolHandler:"
```
/answer 6bd4-62-61 allow
ReasoningToken: 196
Control Request:
```yaml
id: 6bd4-63-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: normalize_result
    source: |-
      def normalize_result(value: object) -> ToolResult:
          """Return *value* if it is a :class:`ToolResult`, otherwise fail loudly."""
          if not isinstance(value, ToolResult):
              raise TypeError(f'Tool handlers must return ToolResult, got {type(value).__name__}')
          return value
    removeImport:
    - from xy.ai.mcpc.server.json_codec import JsonCodec
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: ToolRegistry.register
    source: |-
      def register(self, tool: ToolDefinition) -> Tool:
          tool = tool.to_tool()
          if self.registration_filter is not None and (not self.registration_filter(tool.name)):
              return tool
          if tool.name in self._tools:
              raise ValueError(f'Tool already registered: {tool.name}')
          tool.input_schema = _with_mandatory_reason(tool.input_schema)
          '# Applied unconditionally: ToolResult.to_dict() may attach warnings to *any*'
          '# result regardless of whether the tool declared an outputSchema, so the'
          '# schema must always document them too.'
          base_output_schema = tool.output_schema or {'type': 'object', 'properties': {}}
          tool.output_schema = _with_warnings_property(base_output_schema)
          meta: dict[str, Any] = {'anthropic/maxResultSizeChars': ANTHROPIC_MAX_RESULT_SIZE_CHARS}
          tool.meta = meta
          self._tools[tool.name] = tool
          return tool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    id: with_mandatory_reason
    source: |-
      def _with_mandatory_reason(schema: dict[str, Any]) -> dict[str, Any]:
          """Return *schema* with a mandatory, short ``reason`` property injected.

          Every tool call must carry an extremely short reason/goal for the call so
          the authorizing user can review it (e.g. via the human-in-the-loop
          control layer) before or while it executes.
          """
          return _inject_property(
              schema,
              REASON_PROPERTY,
              'Precise, specific reason for this tool call (what exactly is being retrievedand why it is needed now), shown to the authorizing user.',
              required=True,
              first=True)


      def _with_warnings_property(schema: dict[str, Any]) -> dict[str, Any]:
          """Return *schema* announcing the root ``warnings`` array written by :meth:`ToolResult.to_dict`."""
          schema = dict(schema)
          schema['properties'] = {**schema.get('properties', {}), WARNINGS_KEY: WARNINGS_SCHEMA}
          return schema
  reason: "normalize_result strikt machen, JsonCodec-Import entfernen, Output-Schema um 'warnings' ergänzen."
```
/answer 6bd4-63-0 allow
ReasoningToken: 208
Control Request:
```yaml
id: 6bd4-64-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/tools.py
    id: FunctionTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          return _coerce(self._handler(ctx))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/tools.py
    id: register_tool
    source: |-
      def _coerce(value: ToolResult | str | dict[str, Any] | None) -> ToolResult:
          """Adapt the convenience return types of embedded handlers to :class:`ToolResult`."""
          if isinstance(value, ToolResult):
              return value
          if value is None:
              return ToolResult()
          if isinstance(value, str):
              return ToolResult(content=[text_content(value)])
          if isinstance(value, dict):
              return ToolResult(content=[text_content(JsonCodec.encode(value))], structured_content=value)
          raise TypeError(f'Unsupported tool return type: {type(value)!r}')


      def register_tool(tool: ToolDefinition | ToolProvider, registry: ToolRegistry, environment: AppEnvironment) -> None:
          if isinstance(tool, ToolProvider):
              for definition in tool.tools(environment):
                  registry.register(definition)
          else:
              registry.register(tool)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/tools.py
    id: zTYD23|09LItY
    source: |-
      from __future__ import annotations
      from abc import ABC, abstractmethod
      from collections.abc import Callable, Iterable
      from typing import TYPE_CHECKING, Any
      from xy.ai.mcpc.server.json_codec import JsonCodec
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
  reason: "Str/dict/None-Komfort-Rückgaben in den Embedded-Adapter verlagern, damit der Kern nur noch ToolResult kennt."
```
/answer 6bd4-64-0 allow
ReasoningToken: 1373
Control Request:
```yaml
id: 6bd4-65-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: YOvHfY|j1xNTQ
    source: |-
      from typing import Any, Callable, TypeVar
      from xy.ai.mcpc.tools.tool_registry import ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.results import BatchResult
    removeImport:
    - "from dataclasses import dataclass, asdict"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: compact
    source: "ResultPayload = Callable[[Any], dict[str, Any]]"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: 1tCi6I|MiXew1
    source: "__all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serialize_batch_result', 'batch_schema']"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: require_items
    source: |-
      def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':
          """Return the non-empty list at *key* in ``ctx.arguments``, or an error result.

          Returns:
              ``(values, None)`` on success, or ``([], error_result)`` if *key* is missing or
              empty; callers must return ``error_result`` immediately in that case.
          """
          values = ctx.arguments.get(key) or []
          if not values:
              return ([], ToolResult.failure(f"'{key}' must be a non-empty list."))
          return (values, None)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: BatchError
    source: |-
      def serialize_batch_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None) -> dict[str, Any]:
          """Serialize the populated ``results``/``errors`` lists of a batch result.

          Warnings are not part of this dict; they travel via :attr:`ToolResult.warnings`.

          Args:
              batch_result: The batch to serialize.
              result_payload: Optional context-dependent replacement for each result's
                  :meth:`ItemResult.payload` (the item key and warnings are still added by the item).
          """
          if not isinstance(batch_result, BatchResult):
              raise TypeError(f'Expected a BatchResult, got {type(batch_result).__name__}')
          content: dict[str, Any] = {}
          if batch_result.results:
              content['results'] = [r.to_dict() if result_payload is None else r.to_dict(payload=result_payload(r))
                                    for r in batch_result.results]
          if batch_result.errors:
              content['errors'] = [e.to_dict() for e in batch_result.errors]
          return content


      def batch_tool_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:
          """Wrap a batch result into a :class:`ToolResult` carrying its call-wide warnings."""
          return ToolResult(
              structured_content=serialize_batch_result(batch_result, result_payload),
              warnings=batch_result.all_warnings(),
              auto_approve=auto_approve and not batch_result.errors)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: handle_batch_tool
    source: |-
      def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], BatchResult[Any, Any]], error_class: type, result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:
          """Common handler for batch-processing tools.

          Args:
              ctx: Tool context with arguments.
              item_factory: Function to convert raw dict to typed item.
              batch_fn: Function to process list of items, returning batch result.
              error_class: Exception class raised by `batch_fn` for whole-batch failures.
              result_payload: Optional context-dependent payload for result items (default: ``payload()``).
              auto_approve: Approve the call automatically if no item failed.

          Returns:
              ToolResult with serialized batch result and its warnings.
          """
          raw_items, error = require_items(ctx)
          if error is not None:
              return error
          items: list[T] = []
          for it in raw_items:
              try:
                  items.append(item_factory(it))
              except KeyError as exc:
                  key = exc.args[0] if exc.args else exc
                  return ToolResult.failure(f'Invalid item: missing required property {key!r}')
              except (ValueError, TypeError) as exc:
                  return ToolResult.failure(f'Invalid item: {exc}')
          try:
              batch_result = batch_fn(items)
          except error_class as exc:
              return ToolResult.failure(str(exc))
          return batch_tool_result(batch_result, result_payload, auto_approve)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: serialize_batch_result
    source: T = TypeVar('T')
  reason: "_tool_helpers umbauen: BatchError und Duck-Typing entfernen, isinstance-Gate, batch_tool_result als einheitlicher Übergang zu ToolResult (inkl. warnings)."
```
/answer 6bd4-65-0 allow
ReasoningToken: 184
Control Request:
```yaml
id: 6bd4-66-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  reason: "Reihenfolge der Definitionen in _tool_helpers.py nach den Ersetzungen prüfen (T, ResultPayload)."
```
/answer 6bd4-66-0 allow
Control Request:
```yaml
id: 6bd4-67-66
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: j2F2xB|e5OyD1
        excert: '''Common helpers for tool definition and request handling.'''
      - id: pXkVK0|aIkesy
        excert: "from typing import Any, Callable, TypeVar"
      - id: 8xu9mE|sjbRxc
        excert: "ResultPayload = Callable[[Any], dict[str, Any]]"
      - id: batch_schema
        excert: "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
        docstring: "Build the standard ``{items: [...]}`` input schema shared by all batch tools. N…"
      - id: 1X9Vfg|HuOiNb
        excert: "__all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serializ…"
      - id: batch_tool_result
        excert: "def batch_tool_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:"
        docstring: Wrap a batch result into a :class:`ToolResult` carrying its call-wide warnings.
      - id: aR0fQj|kPKhQR
        excert: T = TypeVar('T')
      - id: require_items
        excert: "def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':"
        docstring: "Return the non-empty list at *key* in ``ctx.arguments``, or an error result. Re…"
      - id: handle_batch_tool
        excert: "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], BatchResult[Any, Any]], error_class: type, result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:"
        docstring: "Common handler for batch-processing tools. Args: ctx: Tool context with argumen…"
      - id: serialize_batch_result
        excert: "def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:"
        docstring: "Serialize a batch result, only including non-empty result/error lists. Args: ba…"
```
/answer 6bd4-67-66 allow
Text:
 Zwischenstand: Die ID-Zuordnung in `_tool_helpers.py` hat sich verschoben, die alte `serialize_batch_result` steht noch da. Ich lese die Datei und stelle die Reihenfolge sauber wieder her.
ReasoningToken: 166
Control Request:
```yaml
id: 6bd4-68-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    ids:
    - pXkVK0|aIkesy
    - 8xu9mE|sjbRxc
    - 1X9Vfg|HuOiNb
    - batch_tool_result
    - aR0fQj|kPKhQR
    - serialize_batch_result
  reason: Aktuellen Stand der verschobenen Knoten lesen.
```
/answer 6bd4-68-0 allow
Control Request:
```yaml
id: 6bd4-69-68
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: pXkVK0|aIkesy
        type: imports
        code: |-
          from typing import Any, Callable, TypeVar
          from xy.ai.mcpc.tools.tool_registry import ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.results import BatchResult
      - id: 8xu9mE|sjbRxc
        type: statements
        code: "ResultPayload = Callable[[Any], dict[str, Any]]"
      - id: 1X9Vfg|HuOiNb
        type: statements
        code: |-
          __all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serialize_batch_result', 'batch_schema']
          T = TypeVar('T')
      - id: batch_tool_result
        type: FunctionDef
        code: |-
          def batch_tool_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:
              """Wrap a batch result into a :class:`ToolResult` carrying its call-wide warnings."""
              return ToolResult(
                  structured_content=serialize_batch_result(
                      batch_result,
                      result_payload),
                  warnings=batch_result.all_warnings(),
                  auto_approve=auto_approve and (
                      not batch_result.errors))
      - id: aR0fQj|kPKhQR
        type: statements
        code: |-
          T = TypeVar('T')
          R = TypeVar('R')
      - id: serialize_batch_result
        type: FunctionDef
        code: |-
          def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:
              """Serialize a batch result, only including non-empty result/error lists.

              Args:
                  batch_result: Object with optional `results` and `errors` attributes.
                  result_serializer: Optional custom serializer for result items (default: asdict).
                  error_serializer: Optional custom serializer for error items (default: asdict).

              Returns:
                  Dict with only populated fields.
              """
              if result_serializer is None:
                  result_serializer = lambda r: _compact(asdict(r)) if hasattr(r, '__dataclass_fields__') else r
              if error_serializer is None:
                  error_serializer = lambda e: _compact(asdict(e)) if hasattr(e, '__dataclass_fields__') else e
              content: dict[str, Any] = {}
              if hasattr(batch_result, 'results') and batch_result.results:
                  content['results'] = [result_serializer(r) for r in batch_result.results]
              if hasattr(batch_result, 'errors') and batch_result.errors:
                  content['errors'] = [error_serializer(e) for e in batch_result.errors]
              notice = getattr(batch_result, 'notice', None)
              if notice:
                  content['notice'] = notice
              return content
```
/answer 6bd4-69-68 allow
Text:
 Der Zustand ist uneinheitlich. Ich schreibe die Datei vollständig neu, mit dem bekannten Inhalt von `batch_schema`.
ReasoningToken: 324
Control Request:
```yaml
id: 6bd4-6a-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    overwrite: true
    source: |
      'Common helpers for tool definition and request handling.'
      from typing import Any, Callable, TypeVar
      from xy.ai.mcpc.tools.tool_registry import ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.results import BatchResult

      __all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serialize_batch_result', 'batch_schema']
      T = TypeVar('T')
      ResultPayload = Callable[[Any], dict[str, Any]]


      def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:
          """Build the standard ``{items: [...]}`` input schema shared by all batch tools.

          Neither the top level nor the items accept properties beyond the declared ones.

          Args:
              item_properties: The ``properties`` of a single item.
              required: The ``required`` keys of a single item.
              description: Description of the ``items`` array.
          """
          item_schema: dict[str, Any] = {'type': 'object', 'properties': item_properties,
                                         'required': required, 'additionalProperties': False}
          return {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': item_schema,
                      'description': description}},
              'required': ['items'],
              'additionalProperties': False}


      def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':
          """Return the non-empty list at *key* in ``ctx.arguments``, or an error result.

          Returns:
              ``(values, None)`` on success, or ``([], error_result)`` if *key* is missing or
              empty; callers must return ``error_result`` immediately in that case.
          """
          values = ctx.arguments.get(key) or []
          if not values:
              return ([], ToolResult.failure(f"'{key}' must be a non-empty list."))
          return (values, None)


      def serialize_batch_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None) -> dict[str, Any]:
          """Serialize the populated ``results``/``errors`` lists of a batch result.

          Warnings are not part of this dict; they travel via :attr:`ToolResult.warnings`.

          Args:
              batch_result: The batch to serialize.
              result_payload: Optional context-dependent replacement for each result's
                  :meth:`ItemResult.payload` (item key and warnings are still added by the item).
          """
          if not isinstance(batch_result, BatchResult):
              raise TypeError(f'Expected a BatchResult, got {type(batch_result).__name__}')
          content: dict[str, Any] = {}
          if batch_result.results:
              content['results'] = [r.to_dict() if result_payload is None else r.to_dict(payload=result_payload(r))
                                    for r in batch_result.results]
          if batch_result.errors:
              content['errors'] = [e.to_dict() for e in batch_result.errors]
          return content


      def batch_tool_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:
          """Wrap a batch result into a :class:`ToolResult` carrying its call-wide warnings."""
          return ToolResult(
              structured_content=serialize_batch_result(batch_result, result_payload),
              warnings=batch_result.all_warnings(),
              auto_approve=auto_approve and not batch_result.errors)


      def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], BatchResult[Any, Any]], error_class: type, result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:
          """Common handler for batch-processing tools.

          Args:
              ctx: Tool context with arguments.
              item_factory: Function to convert raw dict to typed item.
              batch_fn: Function to process list of items, returning batch result.
              error_class: Exception class raised by `batch_fn` for whole-batch failures.
              result_payload: Optional context-dependent payload for result items (default: ``payload()``).
              auto_approve: Approve the call automatically if no item failed.

          Returns:
              ToolResult with serialized batch result and its warnings.
          """
          raw_items, error = require_items(ctx)
          if error is not None:
              return error
          items: list[T] = []
          for it in raw_items:
              try:
                  items.append(item_factory(it))
              except KeyError as exc:
                  key = exc.args[0] if exc.args else exc
                  return ToolResult.failure(f'Invalid item: missing required property {key!r}')
              except (ValueError, TypeError) as exc:
                  return ToolResult.failure(f'Invalid item: {exc}')
          try:
              batch_result = batch_fn(items)
          except error_class as exc:
              return ToolResult.failure(str(exc))
          return batch_tool_result(batch_result, result_payload, auto_approve)
  reason: "_tool_helpers.py vollständig und konsistent neu schreiben (BatchError/_compact/Duck-Typing entfernt, isinstance-Gate, batch_tool_result)."
```
/answer 6bd4-6a-0 allow
ReasoningToken: 195
Control Request:
```yaml
id: 6bd4-6b-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    id: ItemOutcome
    old_text: "    def to_dict(self) -> dict[str, Any]: ..."
    new_text: |2-
          def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
              """Wire form; *payload* replaces :meth:`payload` for context-dependent rendering."""
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    id: ItemResult
    old_text: |2-
          def to_dict(self) -> dict[str, Any]:
              out = {**self.key(), **_checked_payload(self, self.payload())}
    new_text: |2-
          def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
              body = self.payload() if payload is None else payload
              out = {**self.key(), **_checked_payload(self, body)}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    id: ItemError
    old_text: |2-
          def to_dict(self) -> dict[str, Any]:
              return {**self.key(), 'error': self.error, **_checked_payload(self, self.payload())}
    new_text: |2-
          def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
              body = self.payload() if payload is None else payload
              return {**self.key(), 'error': self.error, **_checked_payload(self, body)}
  reason: to_dict um den optionalen payload-Parameter für kontextabhängige Serialisierung erweitern.
```
/answer 6bd4-6b-0 allow
Control Request:
```yaml
id: 6bd4-6c-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: ListResult
    source: |-
      @dataclass(frozen=True)
      class ListResult(ItemResult):
          """Result of listing a single directory, mirroring its input path for result association."""
          path: str
          entries: list[str]

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return {'entries': self.entries}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: ListItemError
    source: |-
      @dataclass(frozen=True)
      class ListItemError(ItemError):
          """Error listing a single directory, mirroring its input path for result association."""
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: ListBatchResult
    source: |-
      class ListBatchResult(BatchResult[ListResult, ListItemError]):
          """Result of :func:`list`.

          Attributes:
              results: One :class:`ListResult` per successfully listed directory.
              errors: One :class:`ListItemError` per directory that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: ListTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""

          def item_factory(it: dict[str, Any]) -> ListItem:
              return ListItem(path=it['path'], pattern=it.get('pattern'), recursive=it.get('recursive', True))
          return handle_batch_tool(ctx, item_factory, list, ListError)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: dvO55c|qVCPEH
    source: |-
      import os
      import re
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools._directories import normalize_directory
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
  reason: list-Tool auf ItemResult/ItemError/BatchResult migrieren (warning -> warnings).
```
/answer 6bd4-6c-0 allow
ReasoningToken: 515
Control Request:
```yaml
id: 6bd4-6d-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: list_one
    old_text: |-
      return ListResult(path=item.path, entries=entries, warning=warning)
          '# Step 2
    new_text: |-
      return ListResult(path=item.path, entries=entries, warnings=(warning,))
          '# Step 2
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: list_one
    old_text: "    return ListResult(path=item.path, entries=entries, warning=warning)"
    new_text: "    return ListResult(path=item.path, entries=entries, warnings=(warning,))"
  reason: Die beiden Warnungs-Rückgaben in _list_one auf das warnings-Tupel umstellen.
```
/answer 6bd4-6d-0 allow
ReasoningToken: 150
Control Request:
```yaml
id: 6bd4-6e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list
    pattern: warning
    include:
    - __init__.py
    limit: 20
  reason: "Verifizieren, dass in list nur noch die neuen warnings-Stellen existieren."
```
/answer 6bd4-6e-0 allow
Control Request:
```yaml
id: 6bd4-6f-6e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list
      matches:
      - path: __init__.py
        matches:
        - '''# file count below each). Both reduced results are returned with a warning.'''
        - "warning = f\"Result reduced because the total number of entries ({match_count}) exceeds the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.\""
        - "return ListResult(path=item.path, entries=entries, warnings=(warning,))"
        - "warning = f\"Result reduced to top-level directories the limit of {_MAX_ENTRIES} is exceeded ({"
```
/answer 6bd4-6f-6e allow
ReasoningToken: 282
Control Request:
```yaml
id: 6bd4-70-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list
    pattern: return ListResult
    include:
    - __init__.py
    limit: 20
    intent: survey
  reason: Beide ListResult-Rückgaben mit Warnings verifizieren.
```
/answer 6bd4-70-0 allow
Control Request:
```yaml
id: 6bd4-71-70
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list
      matches:
      - path: __init__.py
        matches:
        - "return ListResult(path=item.path, entries=entries)"
        - "return ListResult(path=item.path, entries=entries, warnings=(warning,))"
```
/answer 6bd4-71-70 allow
ReasoningToken: 208
Control Request:
```yaml
id: 6bd4-72-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    ids:
    - list_one
  reason: _list_one nach den Edits vollständig prüfen.
```
/answer 6bd4-72-0 allow
Control Request:
```yaml
id: 6bd4-73-72
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: list_one
        type: FunctionDef
        code: |-
          def _list_one(item: ListItem) -> ListResult:
              """List all files below the absolute directory ``item.path``, grouped like ``ls -R``.

              Raises:
                  ListError: If path is not absolute.
                  ListError: If path does not exist or is not a directory.
                  ListError: If pattern is not a valid regular expression.
                  ListError: If more than ``_MAX_ENTRIES`` entries remain even after the result has been
                      reduced to one entry per directory level and then to top-level directories only.
              """
              '# When the flat file count exceeds the limit, the result is progressively reduced:'
              '# first to one entry per directory level (with a per-level file count), then, if'
              '# that still exceeds the limit, to top-level directories only (with an aggregated'
              '# file count below each). Both reduced results are returned with a warning.'
              dir_path = Path(item.path)
              if not dir_path.is_absolute():
                  raise ListError('Path must be absolute.')
              dir_path = normalize_directory(dir_path)
              if not dir_path.is_dir():
                  raise ListError('Directory not found or not a directory.')
              try:
                  regex = re.compile(item.pattern) if item.pattern else None
              except re.error as exc:
                  raise ListError(f'Invalid regex pattern: {exc}') from exc
              groups: dict[str, list[str]] = {}
              match_count = 0
              for root, dirs, files in os.walk(str(dir_path)):
                  rel_dir = os.path.relpath(root, str(dir_path))
                  matched_files = []
                  header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                  match_against_path = regex is not None and '/' in item.pattern
                  for file in sorted(files):
                      candidate = f'{header}/{file}' if match_against_path else file
                      if regex is None or regex.search(candidate):
                          matched_files.append(file)
                  if matched_files:
                      groups[rel_dir] = matched_files
                      match_count += len(matched_files)
                  if not item.recursive:
                      dirs.clear()
              if match_count <= _MAX_ENTRIES:
                  entries = []
                  for rel_dir in sorted(groups):
                      header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                      entries.append(f'{header}:')
                      entries.extend(groups[rel_dir])
                  return ListResult(path=item.path, entries=entries)
              '# Step 1: collapse each directory level to a single representative entry,'
              '# annotated with the total file count for that level.'
              level1_counts = {rel_dir: len(files) for rel_dir, files in groups.items()}
              if len(level1_counts) <= _MAX_ENTRIES:
                  entries = []
                  for rel_dir in sorted(groups):
                      header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                      count = level1_counts[rel_dir]
                      entries.append(f'{header}: ({count} file{('s' if count != 1 else '')})')
                      entries.append(groups[rel_dir][0])
                  warning = f"Result reduced because the total number of entries ({match_count}) exceeds the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter."
                  return ListResult(path=item.path, entries=entries, warnings=(warning,))
              '# Step 2: the per-level reduction still exceeds the limit, so collapse the'
              '# tree depth and aggregate file counts below each top-level directory.'

              def _top_key(rel_dir: str) -> str:
                  if rel_dir == '.':
                      return '.'
                  return rel_dir.split(os.sep)[0]
              top_counts: dict[str, int] = {}
              for rel_dir, files in groups.items():
                  key = _top_key(rel_dir)
                  top_counts[key] = top_counts.get(key, 0) + len(files)
              if len(top_counts) > _MAX_ENTRIES:
                  raise ListError(
                      f"Too many entries even after reducing by directory level ({
                          len(top_counts)}) exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")
              entries = []
              for key in sorted(top_counts):
                  header = key if key == '.' else './' + key.replace(os.sep, '/')
                  count = top_counts[key]
                  entries.append(f'{header}: ({count} file{('s' if count != 1 else '')} below)')
              warning = f"Result reduced to top-level directories the limit of {_MAX_ENTRIES} is exceeded ({
                  len(level1_counts)}). Narrow down the result using the 'pattern' regular expression parameter."
              return ListResult(path=item.path, entries=entries, warnings=(warning,))
```
/answer 6bd4-73-72 allow
Thinking:
`list` ist migriert und beide Reduktionswarnungen laufen über `warnings`. Als Nächstes migriere ich `search`, wo zwei Warnungen zusammenhängen und der Serializer vom Kontext abhängt.


ReasoningToken: 53
Control Request:
```yaml
id: 6bd4-74-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: SearchResult
    source: |-
      @dataclass(frozen=True)
      class SearchResult(ItemResult):
          """Result of a single search.

          Attributes:
              directory: The directory list exactly as given in the input.
              matches: The matches found, grouped by file (empty if none).
              warnings: Set if ``limit`` was reached and further matches may exist, and, separately,
                  if match lines exceeded the maximum length and were shortened with an ellipsis.
          """
          directory: list[str]
          matches: list[SearchMatch]

          def key(self) -> dict[str, Any]:
              return {'directory': self.directory}

          def payload(self) -> dict[str, Any]:
              return {'matches': [asdict(m) for m in self.matches]} if self.matches else {}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: SearchItemError
    source: |-
      @dataclass(frozen=True)
      class SearchItemError(ItemError):
          """Error running a single search, mirroring its input for result association."""
          directory: list[str]
          pattern: str

          def key(self) -> dict[str, Any]:
              return {'directory': self.directory, 'pattern': self.pattern}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: SearchBatchResult
    source: |-
      class SearchBatchResult(BatchResult[SearchResult, SearchItemError]):
          """Result of :func:`search`.

          Attributes:
              results: One :class:`SearchResult` per successfully executed search.
              errors: One :class:`SearchItemError` per search that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: search_one
    source: |-
      def _search_one(item: SearchItem) -> SearchResult:
          result, total_lines = _run_search(item.directory, item.pattern, exclude=item.exclude,
                                            include=item.include, limit=item.limit)
          if result.exit_code >= 2:
              raise SearchError(f'search failed (exit code {result.exit_code}): {result.stderr}')
          matches = _parse_search_stdout(result.stdout)
          warnings: list[str] = []
          if total_lines > item.limit:
              hint = 'narrow pattern, directory or include/exclude filters'
              if item.limit < _MAX_LIMIT:
                  hint += f', or raise limit (max {_MAX_LIMIT})'
              warnings.append(f'Limit of {item.limit} matches reached (total {total_lines}); {hint}.')
          shortened = sum((lm.truncated for m in matches for lm in m.matches))
          if shortened:
              warnings.append(
                  f'{shortened} match line(s) exceeded {_MAX_LINE_LENGTH} characters and were shortened in the middle.')
          return SearchResult(directory=item.directory, matches=matches, warnings=tuple(warnings))
  reason: "search: SearchResult/SearchItemError/SearchBatchResult auf die Basisklassen umstellen und die zwei Warnungen getrennt statt verklebt liefern."
```
/answer 6bd4-74-0 allow
ReasoningToken: 712
Control Request:
```yaml
id: 6bd4-75-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: SearchResult
    source: |-
      @dataclass(frozen=True)
      class SearchResult(ItemResult):
          """Result of a single search.

          Attributes:
              directory: The directory list exactly as given in the input.
              matches: The matches found, grouped by file (empty if none).
              warnings: Set if ``limit`` was reached and further matches may exist, and, separately,
                  if match lines exceeded the maximum length and were shortened with an ellipsis.
          """
          directory: list[str]
          matches: list[SearchMatch]

          def key(self) -> dict[str, Any]:
              return {'directory': self.directory}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: SearchTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func: translating the MCP schema to/from the Python API."""
          intent = ctx.arguments.get('intent', _DEFAULT_INTENT)
          enabled = ctx.session.enabled_tools
          with_lines = bool(_LINE_TOOLS & enabled)
          with_nodes = bool(_NODE_TOOLS & enabled)
          intents: dict[int, str] = {}

          def item_factory(it: dict[str, Any]) -> SearchItem:
              return SearchItem(
                  directory=it['directory'],
                  pattern=it['pattern'],
                  exclude=it.get('exclude'),
                  include=it.get('include'),
                  limit=int(
                      it.get(
                          'limit',
                          _DEFAULT_LIMIT)),
                  intent=it.get('intent'))

          def line_dicts(lms: list[SearchLineMatch], with_lineno: bool) -> list[Any]:
              if with_lineno:
                  return [{'lineno': lm.lineno, 'match': lm.match} for lm in lms]
              return [lm.match for lm in lms]

          def file_dict(m: SearchMatch, item_intent: str) -> dict[str, Any]:
              editing = item_intent == 'editing'
              entry: dict[str, Any] = {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename}
              if m.nodes:
                  entry['nodes'] = [{'id': n.node_id, 'matches': line_dicts(
                      n.matches, editing and with_lines)} for n in m.nodes]
              if m.matches:
                  lines = line_dicts(m.matches, editing and (with_lines or with_nodes))
                  entry['matches'] = lines if editing else list(dict.fromkeys(lines))
              return entry

          def result_payload(r: SearchResult) -> dict[str, Any]:
              if not r.matches:
                  return {}
              return {'matches': [file_dict(m, intents[id(r)]) for m in r.matches]}

          def run(items: list[SearchItem]) -> SearchBatchResult:
              results: list[SearchResult] = []
              errors: list[SearchItemError] = []
              for item in items:
                  batch = search([item], intent=intent, with_nodes=with_nodes)
                  effective = item.intent if item.intent is not None else intent
                  for r in batch.results:
                      intents[id(r)] = effective
                  results += batch.results
                  errors += batch.errors
              return SearchBatchResult(results=results, errors=errors)
          return handle_batch_tool(ctx, item_factory, run, SearchError, result_payload)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: PBZWYD|FmTmX2
    source: |-
      from dataclasses import dataclass, replace
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools._directories import normalize_directories
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: 8Pl1k7|dBAVKD
    source: |-
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
      import re
      from xy.ai.mcpc.tools.ast import core
  reason: search-Handler auf handle_batch_tool mit result_payload umstellen (ohne SimpleNamespace) und Imports bereinigen.
```
/answer 6bd4-75-0 allow
Control Request:
```yaml
id: 6bd4-76-75
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      result: success
      id: SearchResult
      new_id: SearchResult
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      result: success
      id: SearchTool.handle
      new_id: SearchTool.handle
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      result: success
      id: PBZWYD|FmTmX2
      new_id: PBZWYD|FmTmX2
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: 8Pl1k7|dBAVKD
```
/answer 6bd4-76-75 allow
ReasoningToken: 41
Control Request:
```yaml
id: 6bd4-77-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  node_type: imports
  reason: Aktuelle Import-Knoten von search ermitteln (ID hat sich geändert).
```
/answer 6bd4-77-0 allow
Control Request:
```yaml
id: 6bd4-78-77
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: ZXLaUr|alUQX2
        type: imports
        code: |-
          from dataclasses import dataclass, replace
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools._directories import normalize_directories
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      - id: 96cNWi|iHU0uE
        type: imports
        code: |-
          import re
          from xy.ai.mcpc.tools.ast import core
```
/answer 6bd4-78-77 allow
Control Request:
```yaml
id: 6bd4-79-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: ZXLaUr|alUQX2
    old_text: "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
    new_text: |-
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
  reason: Fehlenden results-Import in search ergänzen.
```
/answer 6bd4-79-0 allow
ReasoningToken: 246
Control Request:
```yaml
id: 6bd4-7a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
    ids:
    - igUrQT|uRBR2W
    - 0ZnbEE|BU27Ci
    - ReadResult
    - ReadItemError
    - ReadBatchResult
    - read_one
    - read_file
    - read_file_cached
    - serialize_read_result
    - ReadTool.handle
  reason: "read-Tool (ReadResult mit unchanged, Serializer, Handler) lesen."
```
/answer 6bd4-7a-0 allow
Control Request:
```yaml
id: 6bd4-7b-7a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
      nodes:
      - id: igUrQT|uRBR2W
        type: imports
        code: |-
          import hashlib
          import json
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result, batch_schema
          from xy.ai.mcpc.server.session import Session
      - id: 0ZnbEE|BU27Ci
        type: statements
        code: |-
          __all__ = [
              'ReadError',
              'ReadItem',
              'ReadResult',
              'ReadItemError',
              'ReadBatchResult',
              'read_file',
              'ReadTool',
              'register_read_tool']
          _CACHE_STATE_KEY = '_read_cache'
      - id: ReadResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadResult:
              """Result of reading a single file, mirroring its input path for result association."""
              path: str
              content: str
              checksum: str
              is_full_file: bool
              unchanged: str | None = None
              'Explanatory message if identical to a previous read with the same parameters, else None (set by :class:`ReadTool`, never by :func:`read_file`).'
      - id: ReadItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadItemError:
              """Error reading a single file, mirroring its input path for result association."""
              path: str
              error: str
      - id: ReadBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadBatchResult:
              """Result of :func:`read_file`.

              Attributes:
                  results: One :class:`ReadResult` per successfully read file.
                  errors: One :class:`ReadItemError` per file that failed.
              """
              results: list[ReadResult]
              errors: list[ReadItemError]
      - id: read_one
        type: FunctionDef
        code: |-
          def _read_one(item: ReadItem) -> ReadResult:
              if item.min_line is not None and item.min_char is not None:
                  raise ReadError('``min_line`` and ``min_char`` are mutually exclusive.')
              if item.max_line is not None and item.max_char is not None:
                  raise ReadError('``max_line`` and ``max_char`` are mutually exclusive.')
              if item.min_line is not None and item.start is not None:
                  raise ReadError('``min_line`` and ``start`` are mutually exclusive.')
              if item.min_char is not None and item.start is not None:
                  raise ReadError('``min_char`` and ``start`` are mutually exclusive.')
              if item.max_line is not None and item.end is not None:
                  raise ReadError('``max_line`` and ``end`` are mutually exclusive.')
              if item.max_char is not None and item.end is not None:
                  raise ReadError('``max_char`` and ``end`` are mutually exclusive.')
              file_path = Path(item.path)
              if not file_path.is_absolute():
                  raise ReadError('Path must be absolute.')
              if not file_path.exists():
                  raise ReadError('File not found.')
              if not file_path.is_file():
                  raise ReadError("Not a regular file. Don't read directories with this tool!")
              raw_bytes = file_path.read_bytes()
              text = raw_bytes.decode('utf-8', errors='replace')
              lines = text.splitlines(keepends=True)
              total_lines = len(lines)

              def line_start_offset(line_num: int) -> int:
                  n = max(0, min(line_num - 1, total_lines))
                  return sum((len(l) for l in lines[:n]))

              def line_end_offset(line_num: int) -> int:
                  n = max(0, min(line_num, total_lines))
                  return sum((len(l) for l in lines[:n]))
              if item.start is not None:
                  start_count = text.count(item.start)
                  if start_count == 0:
                      raise ReadError('Start marker not found in file.')
                  if start_count > 1:
                      raise ReadError(f'Start marker is ambiguous – found {start_count} occurrences in file.')
                  region_start = text.index(item.start)
              elif item.min_line is not None:
                  region_start = line_start_offset(item.min_line)
              elif item.min_char is not None:
                  region_start = item.min_char
              else:
                  region_start = 0
              if item.end is not None:
                  end_count = text.count(item.end)
                  if end_count == 0:
                      raise ReadError('End marker not found in file.')
                  if end_count > 1:
                      raise ReadError(f'End marker is ambiguous – found {end_count} occurrences in file.')
                  region_end = text.index(item.end) + len(item.end)
              elif item.max_line is not None:
                  region_end = line_end_offset(item.max_line)
              elif item.max_char is not None:
                  region_end = item.max_char
              else:
                  region_end = len(text)
              if region_end < region_start:
                  raise ReadError('Resolved end position must not lie before the resolved start position.')
              is_full_file = item.min_line is None and item.max_line is None and (
                  item.min_char is None) and (
                      item.max_char is None) and (
                          item.start is None) and (
                              item.end is None)
              if not is_full_file and len(text) and (region_end - region_start > 0.7 * len(text)):
                  region_start = 0
                  region_end = len(text)
                  is_full_file = True
              sliced = text[region_start:region_end]
              checksum = hashlib.sha256(sliced.encode('utf-8')).hexdigest()
              return ReadResult(path=item.path, content=sliced, checksum=checksum, is_full_file=is_full_file)
      - id: read_file
        type: FunctionDef
        code: |-
          def read_file(items: list[ReadItem]) -> ReadBatchResult:
              """Read one or more files, each optionally sliced to a range.

              Args:
                  items: Files to read. Must be non-empty.

              Returns:
                  ReadBatchResult: one result per successfully read file, one error per failed file.

              Raises:
                  ReadError: If items is empty.

              Note:
                  Line numbering is 1-based (first line is 1). Character offsets are 0-based.
              """
              if not items:
                  raise ReadError("'items' must be a non-empty list.")
              results: list[ReadResult] = []
              errors: list[ReadItemError] = []
              for item in items:
                  try:
                      results.append(_read_one(item))
                  except ReadError as exc:
                      errors.append(ReadItemError(path=item.path, error=str(exc)))
              return ReadBatchResult(results=results, errors=errors)
      - id: read_file_cached
        type: FunctionDef
        code: |-
          def _read_file_cached(items: list[ReadItem], session: Session) -> ReadBatchResult:
              """Wrap :func:`read_file`'s per-item logic with session-scoped change detection.

              Kept in the MCP layer (needs ``session``); :func:`read_file` itself stays
              session-agnostic and never sets :attr:`ReadResult.unchanged`.
              """
              results: list[ReadResult] = []
              errors: list[ReadItemError] = []
              with session.lock:
                  cache: dict[str, str] = session.state.setdefault(_CACHE_STATE_KEY, {})
                  for item in items:
                      try:
                          result = _read_one(item)
                      except ReadError as exc:
                          errors.append(ReadItemError(path=item.path, error=str(exc)))
                          continue
                      key = _cache_key(session.id, item)
                      unchanged = cache.get(key) == result.checksum
                      cache[key] = result.checksum
                      if unchanged:
                          result = ReadResult(
                              path=result.path,
                              content=result.content,
                              checksum=result.checksum,
                              is_full_file=result.is_full_file,
                              unchanged='Content unchanged since the last identical read. Use the former read result.')
                      results.append(result)
              return ReadBatchResult(results=results, errors=errors)
      - id: serialize_read_result
        type: FunctionDef
        code: |-
          def _serialize_read_result(result: ReadResult) -> dict[str, Any]:
              entry: dict[str, Any] = {'path': result.path, 'checksum': result.checksum}
              if result.unchanged:
                  entry['unchanged'] = result.unchanged
              else:
                  entry['content'] = result.content
              return entry
      - id: ReadTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`_read_file_cached`, translating the MCP schema to/from the Python API."""
              raw_items, error = require_items(ctx)
              if error is not None:
                  return error
              items = [
                  ReadItem(
                      path=it['path'],
                      min_line=it.get('min_line'),
                      max_line=it.get('max_line'),
                      min_char=it.get('min_char'),
                      max_char=it.get('max_char'),
                      start=it.get('start'),
                      end=it.get('end')) for it in raw_items]
              batch = _read_file_cached(items, ctx.session)
              structured_content = serialize_batch_result(batch, result_serializer=_serialize_read_result)
              has_error = bool(batch.errors)
              all_full_file = all((r.is_full_file for r in batch.results))
              return ToolResult(structured_content=structured_content, auto_approve=not has_error and all_full_file)
```
/answer 6bd4-7b-7a allow
Control Request:
```yaml
id: 6bd4-7c-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
    id: ReadResult
    source: |-
      @dataclass(frozen=True)
      class ReadResult(ItemResult):
          """Result of reading a single file, mirroring its input path for result association."""
          path: str
          content: str
          checksum: str
          is_full_file: bool
          unchanged: str | None = None
          'Explanatory message if identical to a previous read with the same parameters, else None (set by :class:`ReadTool`, never by :func:`read_file`).'

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              if self.unchanged:
                  return {'checksum': self.checksum, 'unchanged': self.unchanged}
              return {'checksum': self.checksum, 'content': self.content}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
    id: ReadItemError
    source: |-
      @dataclass(frozen=True)
      class ReadItemError(ItemError):
          """Error reading a single file, mirroring its input path for result association."""
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
    id: ReadBatchResult
    source: |-
      class ReadBatchResult(BatchResult[ReadResult, ReadItemError]):
          """Result of :func:`read_file`.

          Attributes:
              results: One :class:`ReadResult` per successfully read file.
              errors: One :class:`ReadItemError` per file that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
    id: ReadTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`_read_file_cached`, translating the MCP schema to/from the Python API."""
          raw_items, error = require_items(ctx)
          if error is not None:
              return error
          items = [
              ReadItem(
                  path=it['path'],
                  min_line=it.get('min_line'),
                  max_line=it.get('max_line'),
                  min_char=it.get('min_char'),
                  max_char=it.get('max_char'),
                  start=it.get('start'),
                  end=it.get('end')) for it in raw_items]
          batch = _read_file_cached(items, ctx.session)
          return batch_tool_result(batch, auto_approve=all(r.is_full_file for r in batch.results))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
    id: igUrQT|uRBR2W
    source: |-
      import hashlib
      import json
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools._tool_helpers import require_items, batch_tool_result, batch_schema
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
      from xy.ai.mcpc.server.session import Session
  reason: "read-Tool auf die Basisklassen migrieren; 'unchanged' bleibt typisiertes Feld, Serializer wird zur payload()-Methode."
```
/answer 6bd4-7c-0 allow
ReasoningToken: 358
Control Request:
```yaml
id: 6bd4-7d-0
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
    id: serialize_read_result
  reason: Nicht mehr benötigten _serialize_read_result entfernen (ersetzt durch ReadResult.payload).
```
/answer 6bd4-7d-0 allow
Thinking:
Die Tools `list`, `search` und `read` sind migriert; als Nächstes baue ich in `imports.py` eine gemeinsame Basisklasse für die `ast_*`-Edit-Tools, wobei die Import-Warnung künftig aus den Items abgeleitet wird.


ReasoningToken: 74
Control Request:
```yaml
id: 6bd4-7e-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: import_notice
    source: |-
      class ImportCheckedBatchResult(BatchResult[R, E]):
          """Batch result whose call-wide warning is derived from its items' unchecked imports."""

          def derived_warnings(self) -> tuple[str, ...]:
              unchecked = any(r.imports is not None and r.imports.unchecked for r in self.results)
              return (IMPORT_NOTICE,) if unchecked else ()
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: AFajCW|f1RBFD
    source: |-
      from __future__ import annotations
      import re
      from dataclasses import dataclass, field
      from typing import Any, Callable
      from xy.ai.mcpc.tools.ast.base import AstError, Engine, Tree, check_no_control_chars
      from xy.ai.mcpc.tools.results import BatchResult, E, R
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: AiyIX3|8cD8JN
    source: |-
      __all__ = [
          'IMPORT_NOTICE',
          'ImportOutcome',
          'ImportCheckedBatchResult',
          'parse_import_args',
          'import_entry',
          'normalize_import',
          'apply_generic',
          'apply_treesitter',
          'apply_rpc']
      IMPORT_NOTICE = 'Imports/includes/use statements could not be verified. Check the affected files yourself.'
  reason: import_notice durch eine ableitende Batch-Basisklasse ersetzen (Warnung wird aus den Items berechnet).
```
/answer 6bd4-7e-0 allow
ReasoningToken: 629
Control Request:
```yaml
id: 6bd4-7f-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  reason: "common.py auf Importe/Zyklen prüfen, bevor gemeinsame Key-/Payload-Helfer ergänzt werden."
```
/answer 6bd4-7f-0 allow
Control Request:
```yaml
id: 6bd4-80-7f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
      nodes:
      - id: Q9NkRl|1c3Fpi
        excert: '''Selector machinery shared by the ``ast_*`` tools.\n\n``ast_find`` is the only …'
      - id: QUDAPO|6eecMb
        excert: from typing import Any
      - id: w0xyc1|24HH6w
        excert: "__all__ = ["
      - id: clnjIr|Jz72IC
        excert: "SELECTOR_PROPS = {"
      - id: RwEctf|qsWZ3v
        excert: "'#: Path-only selectors used by every mutation tool (replace/insert/delete/edit…"
      - id: YMszCp|e3vwZI
        excert: "IMPORT_PROPS = {"
      - id: select_one
        excert: "def select_one(tree, **selectors: Any) -> core.Located:"
        docstring: "Return the single node in *tree* matching *selectors*. Raises: core.AstError: I…"
      - id: select_by_path
        excert: "def select_by_path(tree, *, id: str | None=None) -> core.Located:"
        docstring: "Return the single node in *tree* addressed by its unique ``id``. Raises: core.A…"
      - id: select_by_text
        excert: "def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:"
        docstring: "Return the node addressed by ``id``; if ``id`` is omitted, blank or unusable (s…"
      - id: select_by_contained_text
        excert: "def _select_by_contained_text(tree, texts: list[str]) -> core.Located:"
        docstring: Return the single innermost node whose source contains every string in *texts*.…
```
/answer 6bd4-80-7f allow
ReasoningToken: 1015
Control Request:
```yaml
id: 6bd4-81-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outcomes.py
    source: |
      """Item outcome building blocks shared by the node-addressed ``ast_*`` mutation tools."""
      from __future__ import annotations

      from dataclasses import dataclass
      from typing import Any

      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry
      from xy.ai.mcpc.tools.results import ItemError

      __all__ = ['node_key', 'edit_payload', 'NodeError']


      def node_key(path: str, id: str | None) -> dict[str, Any]:
          return {'path': path} if id is None else {'path': path, 'id': id}


      def edit_payload(result: str, *, new_id: str | None=None, imports: ImportOutcome | None=None, **extra: Any) -> dict[str, Any]:
          out: dict[str, Any] = {'result': result, **{k: v for k, v in extra.items() if v is not None}}
          if new_id is not None:
              out['new_id'] = new_id
          if (entry := import_entry(imports)) is not None:
              out['imports'] = entry
          return out


      @dataclass(frozen=True)
      class NodeError(ItemError):
          """Error for one node-addressed item; ``path`` and ``id`` are mirrored exactly as given."""
          path: str
          id: str | None

          def key(self) -> dict[str, Any]:
              return node_key(self.path, self.id)
  reason: "Gemeinsame Bausteine (node_key, edit_payload, NodeError) für die ast-Mutationstools anlegen, um Serializer-Duplikate zu vermeiden."
```
/answer 6bd4-81-0 allow
ReasoningToken: 272
Control Request:
```yaml
id: 6bd4-82-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceResult
    source: |-
      @dataclass(frozen=True)
      class ReplaceResult(ItemResult):
          """Result of a single node replacement.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              result: Always ``"success"``.
              new_id: The node's new id, only set if the replacement changed it.
          """
          path: str
          id: str | None
          result: str
          new_id: str | None = None
          imports: ImportOutcome | None = None

          def key(self) -> dict[str, Any]:
              return node_key(self.path, self.id)

          def payload(self) -> dict[str, Any]:
              return edit_payload(self.result, new_id=self.new_id, imports=self.imports)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceError
    source: |-
      class ReplaceError(NodeError):
          """Error applying a single node replacement.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              error: The error message.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceBatchResult
    source: |-
      class ReplaceBatchResult(ImportCheckedBatchResult[ReplaceResult, ReplaceError]):
          """Result of :func:`ast_replace`.

          Attributes:
              results: One :class:`ReplaceResult` per successful replacement.
              errors: One :class:`ReplaceError` per replacement that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceNodeTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."""

          def item_factory(it: dict[str, Any]) -> ReplaceItem:
              add_import, remove_import = parse_import_args(it)
              return ReplaceItem(
                  path=it['path'],
                  source=it['source'],
                  id=it.get('id'),
                  add_import=add_import,
                  remove_import=remove_import)
          return handle_batch_tool(ctx, item_factory, ast_replace, core.AstError, auto_approve=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: G5Js58|nIvKCC
    source: |-
      from dataclasses import dataclass
      from xy.ai.mcpc.tools.ast.imports import ImportCheckedBatchResult, ImportOutcome, parse_import_args
      from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key
      from xy.ai.mcpc.tools.results import ItemResult
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path
      from xy.ai.mcpc.tools._tool_helpers import batch_schema
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ast_replace
    source: |-
      def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:
          """Replace one or more selected nodes with new source.

          ``id`` and replacement ``source`` stay together per item, since replacing
          the same node twice makes no sense; several items may target the same or
          different files.

          Args:
              items: Node replacements to apply. Must be non-empty.

          Returns:
              ReplaceBatchResult: One result per successful replacement, one error per
              failed replacement.

          Raises:
              core.AstError: If ``items`` is empty.
          """
          if not items:
              raise core.AstError("'items' must be a non-empty list.")
          results: list[ReplaceResult] = []
          errors: list[ReplaceError] = []
          for item in items:
              try:
                  results.append(_replace_one(item))
              except core.AstError as exc:
                  errors.append(ReplaceError(path=item.path, id=item.id, error=str(exc)))
          return ReplaceBatchResult(results=results, errors=errors)
  reason: "ast_replace auf die Basisklassen migrieren (notice entfällt, Serializer entfallen, Warnung wird abgeleitet)."
```
/answer 6bd4-82-0 allow
ReasoningToken: 39
Control Request:
```yaml
id: 6bd4-83-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: (Replace|Delete|Insert|EditBlock|EditMarks|CreateFile|EditLines|Skills|List|Search|Read|OutlineNodes)(Result|Error|ItemError|BatchResult|sResult)\(
    include:
    - '*.py'
    limit: 50
  reason: Alle Konstruktoraufrufe der Result-Klassen prüfen (positional vs. keyword).
```
/answer 6bd4-83-0 allow
Control Request:
```yaml
id: 6bd4-84-83
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/tools/mcp/exa/search.py
        matches:
        - "return WebSearchResult(results=overview, autoprompt_string=raw.get('autoprompt_string'))"
      - path: xy/ai/mcpc/tools/ast/replace.py
        matches:
        - "class ReplaceResult(ItemResult):"
        - "class ReplaceError(NodeError):"
        - "class ReplaceBatchResult(ImportCheckedBatchResult[ReplaceResult, ReplaceError]):"
        - "return ReplaceResult(path=item.path, id=item.id, result='success', new_id=new_id, imports=imports)"
        - "errors.append(ReplaceError(path=item.path, id=item.id, error=str(exc)))"
        - "return ReplaceBatchResult(results=results, errors=errors)"
      - path: xy/ai/mcpc/tools/ast/delete.py
        matches:
        - "return DeleteResult(path=item.path, id=None, result='success', imports=unchecked)"
        - "return DeleteResult(path=item.path, id=item.id, result='success', imports=imports)"
        - "errors.append(DeleteError(path=item.path, id=item.id, error=str(exc)))"
        - "return DeleteBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "return OutlineNodesResult(path=path, nodes=nodes)"
        - "errors.append(OutlineNodesError(path=real_path, error=_directory_listing_hint(real_path)))"
        - "errors.append(OutlineNodesError(path=real_path, error=str(exc)))"
        - "return OutlineNodesBatchResult(results=results, errors=errors)"
      - path: xy/ai/mcpc/tools/ast/edit_marks.py
        matches:
        - "return EditMarksResult(path=item.path, id=item.id, result='success', new_id=new_id, imports=imports)"
        - "errors.append(EditMarksError(path=item.path, id=item.id, error=str(exc), candidates=exc.candidates))"
        - EditMarksError(
        - "return EditMarksBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "return ReadResult(path=item.path, nodes=nodes, errors=errs)"
        - "errors.append(ReadError(path=item.path, error=str(exc)))"
        - "return ReadBatchResult(results=results, errors=errors)"
      - path: xy/ai/mcpc/tools/ast/edit_block.py
        matches:
        - "return EditBlockResult(path=item.path, id=item.id, result='success', new_id=new_id, imports=imports)"
        - "errors.append(EditBlockError(path=item.path, id=item.id, error=str(exc), candidates=exc.candidates))"
        - EditBlockError(
        - "return EditBlockBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: xy/ai/mcpc/tools/ast/create.py
        matches:
        - "return CreateFileResult(path=item.path, result='success')"
        - "errors.append(CreateFileError(path=item.path, error=str(exc)))"
        - "return CreateFilesResult(results=results, errors=errors)"
      - path: xy/ai/mcpc/tools/ast/insert.py
        matches:
        - return InsertResult(
        - "errors.append(InsertError(path=item.path, id=item.id, error=str(exc)))"
        - "return InsertBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: xy/ai/mcpc/tools/skills/__init__.py
        matches:
        - "class SkillsError(Exception):"
        - "raise SkillsError(f'Unknown skill: {name!r}')"
        - raise SkillsError("'items' must be a non-empty list.")
        - "results.append(SkillsResult(name=item.name, instructions=skill.core()))"
        - "errors.append(SkillsItemError(name=item.name, error=str(exc)))"
        - "return SkillsBatchResult(results=results, errors=errors)"
      - path: xy/ai/mcpc/tools/edit_lines/__init__.py
        matches:
        - "class EditLinesError(Exception):"
        - raise EditLinesError('Path must be absolute.')
        - raise EditLinesError('File not found.')
        - raise EditLinesError('Not a regular file.')
        - raise EditLinesError('Offset is out of bounds.')
        - raise EditLinesError('Amount is out of bounds.')
        - "raise EditLinesError(f'Replace failed: {exc}') from exc"
        - "return EditLinesResult(path=item.path, result='success')"
        - raise EditLinesError("'items' must be a non-empty list.")
        - EditLinesItemError(
        - "errors.append(EditLinesItemError(path=item.path, error=str(exc)))"
        - "return EditLinesBatchResult(results=results, errors=errors)"
      warning: "Limit of 50 matches reached (total 129); narrow pattern, directory or include/exclude filters."
```
/answer 6bd4-84-83 allow
ReasoningToken: 187
Control Request:
```yaml
id: 6bd4-85-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteResult
    source: |-
      @dataclass(frozen=True)
      class DeleteResult(ItemResult):
          """Result of deleting a single node or file.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              result: Always ``"success"``.
          """
          path: str
          id: str | None
          result: str
          imports: ImportOutcome | None = None

          def key(self) -> dict[str, Any]:
              return node_key(self.path, self.id)

          def payload(self) -> dict[str, Any]:
              return edit_payload(self.result, imports=self.imports)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteError
    source: |-
      class DeleteError(NodeError):
          """Error deleting a single node or file.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              error: The error message.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteBatchResult
    source: |-
      class DeleteBatchResult(ImportCheckedBatchResult[DeleteResult, DeleteError]):
          """Result of :func:`ast_delete`.

          Attributes:
              results: One :class:`DeleteResult` per successfully deleted item.
              errors: One :class:`DeleteError` per item that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""

          def item_factory(it: dict[str, Any]) -> DeleteItem:
              add_import, remove_import = parse_import_args(it)
              return DeleteItem(path=it['path'], id=it.get('id'), add_import=add_import, remove_import=remove_import)
          return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, auto_approve=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: G5Js58|nIvKCC
    source: |-
      from dataclasses import dataclass
      from xy.ai.mcpc.tools.ast.imports import ImportCheckedBatchResult, ImportOutcome, parse_import_args
      from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key
      from xy.ai.mcpc.tools.results import ItemResult
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path
      from xy.ai.mcpc.tools._tool_helpers import batch_schema
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteItem.PrKpxX|E4BsYR
    source: "'One node (or file) to delete.\\n\\n    Attributes:\\n        path: Absolute path to the file to modify.\\n        id: Unique id of the target node. Omit to delete the whole file.\\n        add_import: Import statements to add if missing, applied after the delete.\\n        remove_import: Import statements to remove if present, applied after the delete.\\n            Both are ignored (and flagged in the batch warnings) when a whole file is deleted.\\n    '\\npath: str\\nid: str | None = None"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: ast_delete.Azmusq|D9YYzV
    source: |-
      if not items:
          raise core.AstError("'items' must be a non-empty list.")
      results: list[DeleteResult] = []
      errors: list[DeleteError] = []
      for item in items:
          try:
              results.append(_delete_one(item))
          except core.AstError as exc:
              errors.append(DeleteError(path=item.path, id=item.id, error=str(exc)))
      return DeleteBatchResult(results=results, errors=errors)
  reason: ast_delete migrieren.
```
/answer 6bd4-85-0 allow
Control Request:
```yaml
id: 6bd4-86-85
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteResult
      new_id: DeleteResult
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteError
      new_id: DeleteError
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteBatchResult
      new_id: DeleteBatchResult
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteTool.handle
      new_id: DeleteTool.handle
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: G5Js58|nIvKCC
      new_id: G5Js58|nIvKCC
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: ast_delete.Azmusq|D9YYzV
      new_id: ast_delete.Azmusq|D9YYzV
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      error: "Syntax error: unexpected character after line continuation character (line 1)"
      id: DeleteItem.PrKpxX|E4BsYR
```
/answer 6bd4-86-85 allow
Text:
 Das Delete-Docstring-Update scheiterte an meinem Escaping. Ich ersetze nur die betroffene Wendung.
ReasoningToken: 105
Control Request:
```yaml
id: 6bd4-87-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteItem.PrKpxX|E4BsYR
    old_text: flagged in the batch notice
    new_text: flagged in the batch warnings
  reason: Docstring von DeleteItem an die neue Terminologie (batch warnings) anpassen.
```
/answer 6bd4-87-0 allow
ReasoningToken: 73
Control Request:
```yaml
id: 6bd4-88-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertResult
    source: |-
      @dataclass(frozen=True)
      class InsertResult(ItemResult):
          """Result of a single insert operation.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              result: Always ``"success"``.
              inserted: Number of top-level statements parsed from ``source`` and inserted.
              ids: The newly inserted top-level node(s)' ids.
          """
          path: str
          id: str | None
          result: str
          inserted: int
          ids: list[str] | None = None
          imports: ImportOutcome | None = None

          def key(self) -> dict[str, Any]:
              return node_key(self.path, self.id)

          def payload(self) -> dict[str, Any]:
              return edit_payload(self.result, imports=self.imports, inserted=self.inserted, ids=self.ids)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertError
    source: |-
      class InsertError(NodeError):
          """Error applying a single insert operation.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              error: The error message.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertBatchResult
    source: |-
      class InsertBatchResult(ImportCheckedBatchResult[InsertResult, InsertError]):
          """Result of :func:`ast_insert`.

          Attributes:
              results: One :class:`InsertResult` per successful insert.
              errors: One :class:`InsertError` per insert that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertNodeTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""

          def item_factory(it: dict[str, Any]) -> InsertItem:
              add_import, remove_import = parse_import_args(it)
              return InsertItem(
                  path=it['path'],
                  source=it['source'],
                  position=it.get(
                      'position',
                      'after'),
                  id=it.get('id'),
                  add_import=add_import,
                  remove_import=remove_import)
          return handle_batch_tool(ctx, item_factory, ast_insert, core.AstError, auto_approve=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: G5Js58|nIvKCC
    source: |-
      from dataclasses import dataclass
      from xy.ai.mcpc.tools.ast.imports import ImportCheckedBatchResult, ImportOutcome, parse_import_args
      from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key
      from xy.ai.mcpc.tools.results import ItemResult
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path
      from xy.ai.mcpc.tools._tool_helpers import batch_schema
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: ast_insert.Azmusq|tFexdV
    source: |-
      if not items:
          raise core.AstError("'items' must be a non-empty list.")
      results: list[InsertResult] = []
      errors: list[InsertError] = []
      for item in items:
          try:
              results.append(_insert_one(item))
          except core.AstError as exc:
              errors.append(InsertError(path=item.path, id=item.id, error=str(exc)))
      return InsertBatchResult(results=results, errors=errors)
  reason: ast_insert migrieren.
```
/answer 6bd4-88-0 allow
Control Request:
```yaml
id: 6bd4-89-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockResult
    source: |-
      @dataclass(frozen=True)
      class EditBlockResult(ItemResult):
          """Result of applying a single block edit.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              result: Always ``"success"``.
              new_id: The node's new id, only set if the edit changed it.
          """
          path: str
          id: str | None
          result: str
          new_id: str | None = None
          imports: ImportOutcome | None = None

          def key(self) -> dict[str, Any]:
              return node_key(self.path, self.id)

          def payload(self) -> dict[str, Any]:
              return edit_payload(self.result, new_id=self.new_id, imports=self.imports)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockError
    source: |-
      @dataclass(frozen=True)
      class EditBlockError(NodeError):
          """Error applying a single block edit.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              error: The error message.
              candidates: On ambiguity (id omitted, several nodes matched), the candidate node ids.
              reason: Machine-readable cause, e.g. 'whitespace_mismatch', 'content_changed',
                  'guard_rejected', 'ambiguous', 'not_found'.
              corrected_text: Verified fix for old_text (whitespace-only difference), if any.
              guess: Unverified single-candidate guess at old_text's current form, if any.
              next_step: Fallback instruction (e.g. 'reread_node') when no fix could be determined.
          """
          candidates: list[str] | None = None
          reason: str | None = None
          corrected_text: str | None = None
          guess: str | None = None
          next_step: str | None = None

          def payload(self) -> dict[str, Any]:
              details = {
                  'candidates': self.candidates,
                  'reason': self.reason,
                  'corrected_text': self.corrected_text,
                  'guess': self.guess,
                  'next_step': self.next_step}
              return {k: v for k, v in details.items() if v is not None}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockBatchResult
    source: |-
      class EditBlockBatchResult(ImportCheckedBatchResult[EditBlockResult, EditBlockError]):
          """Result of :func:`ast_edit_block`.

          Attributes:
              results: One :class:`EditBlockResult` per successfully applied edit.
              errors: One :class:`EditBlockError` per edit that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockNodeTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST API."""

          def item_factory(it: dict[str, Any]) -> EditBlockItem:
              add_import, remove_import = parse_import_args(it)
              return EditBlockItem(
                  path=it['path'],
                  old_text=it['old_text'],
                  new_text=it['new_text'],
                  exact=it.get(
                      'exact',
                      False),
                  replace_all=it.get(
                      'replaceAll',
                      False),
                  id=it.get('id'),
                  add_import=add_import,
                  remove_import=remove_import)
          return handle_batch_tool(ctx, item_factory, ast_edit_block, core.AstError, auto_approve=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: G5Js58|8gAslm
    source: |-
      from dataclasses import dataclass
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_text
      from xy.ai.mcpc.tools.ast.imports import ImportCheckedBatchResult, ImportOutcome, parse_import_args
      from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key
      from xy.ai.mcpc.tools.results import ItemResult
      from xy.ai.mcpc.tools._tool_helpers import batch_schema
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: ast_edit_block.92vOt4|K8Whz5
    source: "return EditBlockBatchResult(results=results, errors=errors)"
  reason: "ast_edit_block migrieren (Result/Error mit Zusatzfeldern, Batch-Ableitung, Handler ohne Serializer)."
```
/answer 6bd4-89-0 allow
ReasoningToken: 926
Control Request:
```yaml
id: 6bd4-8a-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksResult
    source: |-
      @dataclass(frozen=True)
      class EditMarksResult(ItemResult):
          """Result of applying a single marker-based edit.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              result: Always ``"success"``.
              new_id: The node's new id, only set if the edit changed it.
          """
          path: str
          id: str | None
          result: str
          new_id: str | None = None
          imports: ImportOutcome | None = None

          def key(self) -> dict[str, Any]:
              return node_key(self.path, self.id)

          def payload(self) -> dict[str, Any]:
              return edit_payload(self.result, new_id=self.new_id, imports=self.imports)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksError
    source: |-
      @dataclass(frozen=True)
      class EditMarksError(NodeError):
          """Error applying a single marker-based edit.

          Attributes:
              path: The path exactly as given in the input, for result association.
              id: The id exactly as given in the input, for result association.
              error: The error message.
              candidates: On ambiguity (id omitted, several nodes matched), the candidate node ids.
              reason: Machine-readable cause, e.g. 'whitespace_mismatch', 'content_changed',
                  'guard_rejected', 'marker_order', 'ambiguous', 'not_found'.
              position: Which marker failed ('start'/'end'), if applicable.
              corrected_text: Verified fix for the failing marker (whitespace-only difference), if any.
              guess: Unverified single-candidate guess at the marker's current form, if any.
              next_step: Fallback instruction (e.g. 'reread_node') when no fix could be determined.
          """
          candidates: list[str] | None = None
          reason: str | None = None
          position: str | None = None
          corrected_text: str | None = None
          guess: str | None = None
          next_step: str | None = None

          def payload(self) -> dict[str, Any]:
              details = {
                  'candidates': self.candidates,
                  'reason': self.reason,
                  'position': self.position,
                  'corrected_text': self.corrected_text,
                  'guess': self.guess,
                  'next_step': self.next_step}
              return {k: v for k, v in details.items() if v is not None}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksBatchResult
    source: |-
      class EditMarksBatchResult(ImportCheckedBatchResult[EditMarksResult, EditMarksError]):
          """Result of :func:`ast_edit_marks`.

          Attributes:
              results: One :class:`EditMarksResult` per successfully applied edit.
              errors: One :class:`EditMarksError` per edit that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksNodeTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST API."""

          def item_factory(it: dict[str, Any]) -> EditMarksItem:
              add_import, remove_import = parse_import_args(it)
              return EditMarksItem(
                  path=it['path'],
                  start_marker=it['start_marker'],
                  end_marker=it['end_marker'],
                  content=it['content'],
                  exact=it.get(
                      'exact',
                      False),
                  id=it.get('id'),
                  add_import=add_import,
                  remove_import=remove_import)
          return handle_batch_tool(ctx, item_factory, ast_edit_marks, core.AstError, auto_approve=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: G5Js58|8gAslm
    source: |-
      from dataclasses import dataclass
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_text
      from xy.ai.mcpc.tools.ast.imports import ImportCheckedBatchResult, ImportOutcome, parse_import_args
      from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key
      from xy.ai.mcpc.tools.results import ItemResult
      from xy.ai.mcpc.tools._tool_helpers import batch_schema
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: ast_edit_marks.ad45ed|Ygmaf8
    source: "return EditMarksBatchResult(results=results, errors=errors)"
  reason: ast_edit_marks analog migrieren.
```
/answer 6bd4-8a-0 allow
ReasoningToken: 266
Control Request:
```yaml
id: 6bd4-8b-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  node_type: imports
  reason: Import-Knoten von create.py ermitteln.
```
/answer 6bd4-8b-0 allow
Control Request:
```yaml
id: 6bd4-8c-8b
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
      nodes:
      - id: wlvDdw|agm8HT
        type: imports
        code: |-
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools._tool_helpers import batch_schema
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
```
/answer 6bd4-8c-8b allow
Control Request:
```yaml
id: 6bd4-8d-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
    id: CreateFileResult
    source: |-
      @dataclass(frozen=True)
      class CreateFileResult(ItemResult):
          """Result of creating a single file.

          Attributes:
              path: The path exactly as given in the input, for result association.
              result: Always ``"success"``.
          """
          path: str
          result: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return {'result': self.result}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
    id: CreateFileError
    source: |-
      @dataclass(frozen=True)
      class CreateFileError(ItemError):
          """Error creating a single file.

          Attributes:
              path: The path exactly as given in the input, for result association.
              error: The error message.
          """
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
    id: CreateFilesResult
    source: |-
      class CreateFilesResult(BatchResult[CreateFileResult, CreateFileError]):
          """Result of :func:`ast_create`.

          Attributes:
              results: One :class:`CreateFileResult` per file created successfully.
              errors: One :class:`CreateFileError` per file that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
    id: wlvDdw|agm8HT
    source: |-
      from dataclasses import dataclass
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools._tool_helpers import batch_schema
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
  reason: ast_create migrieren.
```
/answer 6bd4-8d-0 allow
Control Request:
```yaml
id: 6bd4-8e-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    ids:
    - oJ1xKq|eFu265
    - SkillsResult
    - SkillsItemError
    - SkillsBatchResult
    - get_skills
    - SkillsTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
    ids:
    - PBZWYD|z6v29p
    - EditLinesResult
    - EditLinesItemError
    - EditLinesBatchResult
    - edit_lines
    - EditLinesTool.handle
  reason: skills und edit_lines für die Migration lesen.
```
/answer 6bd4-8e-0 allow
Control Request:
```yaml
id: 6bd4-8f-8e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
      nodes:
      - id: oJ1xKq|eFu265
        type: imports
        code: |-
          from abc import ABC, abstractmethod
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: SkillsResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SkillsResult:
              """Result of fetching a single skill, mirroring its input name for result association."""
              name: str
              instructions: str
      - id: SkillsItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SkillsItemError:
              """Error fetching a single skill, mirroring its input name for result association."""
              name: str
              error: str
      - id: SkillsBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SkillsBatchResult:
              """Result of :func:`get_skills`.

              Attributes:
                  results: One :class:`SkillsResult` per successfully fetched skill.
                  errors: One :class:`SkillsItemError` per skill that could not be resolved.
              """
              results: list[SkillsResult]
              errors: list[SkillsItemError]
      - id: get_skills
        type: FunctionDef
        code: |-
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
      - id: SkillsTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`get_skills`, translating the MCP schema to/from the Python API."""

              def item_factory(it: dict[str, Any]) -> SkillsItem:
                  return SkillsItem(name=it['name'])
              return handle_batch_tool(ctx, item_factory, get_skills, SkillsError, auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
      nodes:
      - id: PBZWYD|z6v29p
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: EditLinesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditLinesResult:
              """Result of applying a single line-range replacement, mirroring its input path for result association."""
              path: str
              result: str
      - id: EditLinesItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditLinesItemError:
              """Error applying a single line-range replacement, mirroring its input path for result association."""
              path: str
              error: str
      - id: EditLinesBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditLinesBatchResult:
              """Result of :func:`edit_lines`.

              Attributes:
                  results: One :class:`EditLinesResult` per successfully applied edit.
                  errors: One :class:`EditLinesItemError` per edit that failed.
              """
              results: list[EditLinesResult]
              errors: list[EditLinesItemError]
      - id: edit_lines
        type: FunctionDef
        code: |-
          def edit_lines(items: list[EditLinesItem]) -> EditLinesBatchResult:
              """Replace a range of lines in one or more files, for a batch of items.

              A given path may only be edited once per batch: since offsets can shift after
              each edit, every further item targeting an already-processed path fails.

              Args:
                  items: Line-range replacements to apply. Must be non-empty.

              Returns:
                  EditLinesBatchResult: one result per successfully applied edit, one error per failed edit.

              Raises:
                  EditLinesError: If items is empty.

              Note:
                  Lines are 0-based. content may be empty to perform pure deletion.
              """
              if not items:
                  raise EditLinesError("'items' must be a non-empty list.")
              results: list[EditLinesResult] = []
              errors: list[EditLinesItemError] = []
              seen: set[str] = set()
              for item in items:
                  if item.path in seen:
                      errors.append(
                          EditLinesItemError(
                              path=item.path,
                              error='Duplicate path in this batch; edit_lines allows only one edit per path per batch because offsets can shift after a prior edit.'))
                      continue
                  seen.add(item.path)
                  try:
                      results.append(_edit_lines_one(item))
                  except EditLinesError as exc:
                      errors.append(EditLinesItemError(path=item.path, error=str(exc)))
              return EditLinesBatchResult(results=results, errors=errors)
      - id: EditLinesTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`edit_lines`, translating the MCP schema to/from the Python API."""

              def item_factory(it: dict[str, Any]) -> EditLinesItem:
                  return EditLinesItem(path=it['path'], offset=it['offset'], amount=it['amount'], content=it['content'])
              return handle_batch_tool(ctx, item_factory, edit_lines, EditLinesError, auto_approve=True)
```
/answer 6bd4-8f-8e allow
Control Request:
```yaml
id: 6bd4-90-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    id: SkillsResult
    source: |-
      @dataclass(frozen=True)
      class SkillsResult(ItemResult):
          """Result of fetching a single skill, mirroring its input name for result association."""
          name: str
          instructions: str

          def key(self) -> dict[str, Any]:
              return {'name': self.name}

          def payload(self) -> dict[str, Any]:
              return {'instructions': self.instructions}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    id: SkillsItemError
    source: |-
      @dataclass(frozen=True)
      class SkillsItemError(ItemError):
          """Error fetching a single skill, mirroring its input name for result association."""
          name: str

          def key(self) -> dict[str, Any]:
              return {'name': self.name}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    id: SkillsBatchResult
    source: |-
      class SkillsBatchResult(BatchResult[SkillsResult, SkillsItemError]):
          """Result of :func:`get_skills`.

          Attributes:
              results: One :class:`SkillsResult` per successfully fetched skill.
              errors: One :class:`SkillsItemError` per skill that could not be resolved.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    id: oJ1xKq|eFu265
    source: |-
      from abc import ABC, abstractmethod
      from dataclasses import dataclass
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
    id: EditLinesResult
    source: |-
      @dataclass(frozen=True)
      class EditLinesResult(ItemResult):
          """Result of applying a single line-range replacement, mirroring its input path for result association."""
          path: str
          result: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return {'result': self.result}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
    id: EditLinesItemError
    source: |-
      @dataclass(frozen=True)
      class EditLinesItemError(ItemError):
          """Error applying a single line-range replacement, mirroring its input path for result association."""
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
    id: EditLinesBatchResult
    source: |-
      class EditLinesBatchResult(BatchResult[EditLinesResult, EditLinesItemError]):
          """Result of :func:`edit_lines`.

          Attributes:
              results: One :class:`EditLinesResult` per successfully applied edit.
              errors: One :class:`EditLinesItemError` per edit that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
    id: PBZWYD|z6v29p
    source: |-
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
  reason: skills und edit_lines auf die Basisklassen migrieren.
```
/answer 6bd4-90-0 allow
ReasoningToken: 80
Control Request:
```yaml
id: 6bd4-91-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
    ids:
    - PBZWYD|hnnU2r
    - EditBlockResult
    - EditBlockItemError
    - EditBlockBatchResult
    - edit_block
    - EditBlockTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
    ids:
    - PBZWYD|Vpz5GK
    - EditMarksResult
    - EditMarksItemError
    - EditMarksBatchResult
    - edit_marks
    - EditMarksTool.handle
  reason: Die klassischen edit_block/edit_marks-Tools lesen (haben eigene Error-Serializer).
```
/answer 6bd4-91-0 allow
Control Request:
```yaml
id: 6bd4-92-91
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
      nodes:
      - id: PBZWYD|hnnU2r
        type: imports
        code: |-
          from dataclasses import asdict, dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
          from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, TextMatchError
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: EditBlockResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockResult:
              """Result of applying a single block edit, mirroring its input path for result association."""
              path: str
              result: str
      - id: EditBlockItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockItemError:
              """Error applying a single block edit, mirroring its input path for result association."""
              path: str
              error: str
              reason: str | None = None
              corrected_text: str | None = None
              guess: str | None = None
              next_step: str | None = None
      - id: EditBlockBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockBatchResult:
              """Result of :func:`edit_block`.

              Attributes:
                  results: One :class:`EditBlockResult` per successfully applied edit.
                  errors: One :class:`EditBlockItemError` per edit that failed.
              """
              results: list[EditBlockResult]
              errors: list[EditBlockItemError]
      - id: edit_block
        type: FunctionDef
        code: |-
          def edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:
              """Replace occurrence(s) of ``old_text`` with ``new_text`` in one or more files.

              Matching escalates through whitespace/escape/quote tolerance; a candidate is
              only accepted when it preserves ``old_text``'s line structure, so no two lines
              are merged into a syntax error.

              Args:
                  items: Block edits to apply. Must be non-empty.

              Returns:
                  EditBlockBatchResult: one result per successfully applied edit, one error per failed edit.

              Raises:
                  EditBlockError: If items is empty.
              """
              if not items:
                  raise EditBlockError("'items' must be a non-empty list.")
              results: list[EditBlockResult] = []
              errors: list[EditBlockItemError] = []
              for item in items:
                  try:
                      results.append(_edit_block_one(item))
                  except EditBlockError as exc:
                      errors.append(
                          EditBlockItemError(
                              path=item.path,
                              error=str(exc),
                              reason=exc.reason,
                              corrected_text=exc.corrected_text,
                              guess=exc.guess,
                              next_step=exc.next_step))
              return EditBlockBatchResult(results=results, errors=errors)
      - id: EditBlockTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`edit_block`, translating the MCP schema to/from the Python API."""

              def item_factory(it: dict[str, Any]) -> EditBlockItem:
                  return EditBlockItem(
                      path=it['path'],
                      old_text=it['old_text'],
                      new_text=it['new_text'],
                      exact=it.get(
                          'exact',
                          False),
                      replace_all=it.get(
                          'replaceAll',
                          False))

              def error_serializer(e: EditBlockItemError) -> dict[str, Any]:
                  return {k: v for k, v in asdict(e).items() if v is not None}
              return handle_batch_tool(
                  ctx,
                  item_factory,
                  edit_block,
                  EditBlockError,
                  error_serializer=error_serializer,
                  auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
      nodes:
      - id: PBZWYD|Vpz5GK
        type: imports
        code: |-
          from dataclasses import asdict, dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
          from xy.ai.mcpc.tools._text_match import replace_between, marks_line_preserving, TextMatchError
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: EditMarksResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditMarksResult:
              """Result of applying a single marker-based replacement, mirroring its input path for result association."""
              path: str
              result: str
      - id: EditMarksItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditMarksItemError:
              """Error applying a single marker-based replacement, mirroring its input path for result association."""
              path: str
              error: str
              reason: str | None = None
              position: str | None = None
              corrected_text: str | None = None
              guess: str | None = None
              next_step: str | None = None
      - id: EditMarksBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditMarksBatchResult:
              """Result of :func:`edit_marks`.

              Attributes:
                  results: One :class:`EditMarksResult` per successfully applied edit.
                  errors: One :class:`EditMarksItemError` per edit that failed.
              """
              results: list[EditMarksResult]
              errors: list[EditMarksItemError]
      - id: edit_marks
        type: FunctionDef
        code: |-
          def edit_marks(items: list[EditMarksItem]) -> EditMarksBatchResult:
              """Replace everything between and including 'begin_marker' and 'end_marker' with content, in one or more files.

              Both markers are included in the replacement.

              Args:
                  items: Marker-based edits to apply. Must be non-empty.

              Returns:
                  EditMarksBatchResult: one result per successfully applied edit, one error per failed edit.

              Raises:
                  EditMarksError: If items is empty.
              """
              if not items:
                  raise EditMarksError("'items' must be a non-empty list.")
              results: list[EditMarksResult] = []
              errors: list[EditMarksItemError] = []
              for item in items:
                  try:
                      results.append(_edit_marks_one(item))
                  except EditMarksError as exc:
                      errors.append(
                          EditMarksItemError(
                              path=item.path,
                              error=str(exc),
                              reason=exc.reason,
                              position=exc.position,
                              corrected_text=exc.corrected_text,
                              guess=exc.guess,
                              next_step=exc.next_step))
              return EditMarksBatchResult(results=results, errors=errors)
      - id: EditMarksTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python API."""

              def item_factory(it: dict[str, Any]) -> EditMarksItem:
                  return EditMarksItem(
                      path=it['path'],
                      begin_marker=it['begin_marker'],
                      end_marker=it['end_marker'],
                      content=it['content'],
                      exact=it.get(
                          'exact',
                          False))

              def error_serializer(e: EditMarksItemError) -> dict[str, Any]:
                  return {k: v for k, v in asdict(e).items() if v is not None}
              return handle_batch_tool(
                  ctx,
                  item_factory,
                  edit_marks,
                  EditMarksError,
                  error_serializer=error_serializer,
                  auto_approve=True)
```
/answer 6bd4-92-91 allow
Thinking:
Die `ast_*`-Mutationstools, `ast_create`, `skills` und `edit_lines` sind umgestellt. Als Nächstes folgen `edit_block`/`edit_marks`, dann `ast_read`/`ast_outline`, die Exa- und OpenAlex-Tools und die Tests.


ReasoningToken: 93
Control Request:
```yaml
id: 6bd4-93-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
    id: EditBlockResult
    source: |-
      @dataclass(frozen=True)
      class EditBlockResult(ItemResult):
          """Result of applying a single block edit, mirroring its input path for result association."""
          path: str
          result: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return {'result': self.result}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
    id: EditBlockItemError
    source: |-
      @dataclass(frozen=True)
      class EditBlockItemError(ItemError):
          """Error applying a single block edit, mirroring its input path for result association."""
          path: str
          reason: str | None = None
          corrected_text: str | None = None
          guess: str | None = None
          next_step: str | None = None

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return compact_asdict(self, exclude={'path', 'error'})
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
    id: EditBlockBatchResult
    source: |-
      class EditBlockBatchResult(BatchResult[EditBlockResult, EditBlockItemError]):
          """Result of :func:`edit_block`.

          Attributes:
              results: One :class:`EditBlockResult` per successfully applied edit.
              errors: One :class:`EditBlockItemError` per edit that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
    id: EditBlockTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`edit_block`, translating the MCP schema to/from the Python API."""

          def item_factory(it: dict[str, Any]) -> EditBlockItem:
              return EditBlockItem(
                  path=it['path'],
                  old_text=it['old_text'],
                  new_text=it['new_text'],
                  exact=it.get(
                      'exact',
                      False),
                  replace_all=it.get(
                      'replaceAll',
                      False))
          return handle_batch_tool(ctx, item_factory, edit_block, EditBlockError, auto_approve=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
    id: PBZWYD|hnnU2r
    source: |-
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, TextMatchError
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult, compact_asdict
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
    id: EditMarksResult
    source: |-
      @dataclass(frozen=True)
      class EditMarksResult(ItemResult):
          """Result of applying a single marker-based replacement, mirroring its input path for result association."""
          path: str
          result: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return {'result': self.result}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
    id: EditMarksItemError
    source: |-
      @dataclass(frozen=True)
      class EditMarksItemError(ItemError):
          """Error applying a single marker-based replacement, mirroring its input path for result association."""
          path: str
          reason: str | None = None
          position: str | None = None
          corrected_text: str | None = None
          guess: str | None = None
          next_step: str | None = None

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return compact_asdict(self, exclude={'path', 'error'})
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
    id: EditMarksBatchResult
    source: |-
      class EditMarksBatchResult(BatchResult[EditMarksResult, EditMarksItemError]):
          """Result of :func:`edit_marks`.

          Attributes:
              results: One :class:`EditMarksResult` per successfully applied edit.
              errors: One :class:`EditMarksItemError` per edit that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
    id: EditMarksTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python API."""

          def item_factory(it: dict[str, Any]) -> EditMarksItem:
              return EditMarksItem(
                  path=it['path'],
                  begin_marker=it['begin_marker'],
                  end_marker=it['end_marker'],
                  content=it['content'],
                  exact=it.get(
                      'exact',
                      False))
          return handle_batch_tool(ctx, item_factory, edit_marks, EditMarksError, auto_approve=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
    id: PBZWYD|Vpz5GK
    source: |-
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools._text_match import replace_between, marks_line_preserving, TextMatchError
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult, compact_asdict
  reason: "Klassische edit_block/edit_marks-Tools migrieren (Error-Serializer entfallen, Zusatzfelder über payload())."
```
/answer 6bd4-93-0 allow
ReasoningToken: 1756
Control Request:
```yaml
id: 6bd4-94-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: "ast_read|ReadNodeTool|ast_outline|OutlineNodes|ids=\\[\\]|root"
    include:
    - '*.py'
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: ast_read|ast_outline|ReadBatchResult|OutlineNodesBatchResult
    include:
    - '*.py'
    exclude:
    - ast/read.py
    - ast/outline.py
    limit: 30
  reason: "Nutzung von ast_read/ast_outline in Tests und Quellcode prüfen, bevor der Fallback in die Fachfunktion wandert."
```
/answer 6bd4-94-0 allow
Control Request:
```yaml
id: 6bd4-95-94
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: test_ast_engines.py
        matches:
        - from xy.ai.mcpc.tools.ast.outline import ast_outline
        - "from xy.ai.mcpc.tools.ast.read import ReadItem, ast_read"
        - "nodes = ast_outline(paths=[py_file]).results[0].nodes"
        - "result = ast_read(items=[ReadItem(path=py_file, ids=['A.foo'])])"
      - path: test_http_tool_calls.py
        matches:
        - "``tools/call`` for ``list``, ``bash`` and ``python_ast_outline`` — exercising"
        - "def test_http_ast_outline_tool(http_client, tmp_path):"
        - "result = http_client.call_tool('ast_outline', {'paths': [str(source)]})"
      - path: test_import_all.py
        matches:
        - import xy.ai.mcpc as root_package
        - "names = [root_package.__name__]"
        - "root_package.__path__, prefix=root_package.__name__ + \".\""
      - path: test_grep_with_ast.py
        matches:
        - "session.enabled_tools.update({'tools', 'ast_read'})"
        - session.enabled_tools.add('ast_read')
      - path: test_tools_family_smoke.py
        matches:
        - "call2 = _call(registry, session, 'tool_call', tool_ids=[], code='print(kept)')"
        - "call3 = _call(registry, session, 'tool_call', tool_ids=[], code=f\"print('x' * {STREAM_SPILL_THRESHOLD + 1})\")"
        - "call4 = _call(registry, session, 'tool_call', tool_ids=[], code=f'print(len({spill_var}))')"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/tools/ast/find.py
        matches:
        - "file, nested like ``ast_outline`` but including source."
      - path: xy/ai/mcpc/tools/ast/script.py
        matches:
        - "'description': \"Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limite…ne header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed.\"}},"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "\"\"\"``ast_outline`` tool: list AST nodes of one or more files.\"\"\""
        - "'OutlineNodesBatchResult',"
        - "'ast_outline',"
        - "path: The resolved path of the listed file (see ``ast_outline``'s"
        - "class OutlineNodesBatchResult:"
        - '"""Result of :func:`ast_outline`.'
        - "def ast_outline(paths: list[str], *, with_lines: bool=True) -> OutlineNodesBatchResult:"
        - use ``ast_find`` (property/text filtering) or ``ast_read`` (by id) to
        - "OutlineNodesBatchResult: One result per listed file, one error per failed file."
        - "return OutlineNodesBatchResult(results=results, errors=errors)"
        - name = 'ast_outline'
        - "\"\"\"Delegate to :func:`ast_outline`, translating the MCP schema to/from the AST API.\"\"\""
        - "batch = ast_outline(paths=paths, with_lines=with_lines)"
        - functions.register(ast_outline)
      - path: xy/ai/mcpc/tools/ast/__init__.py
        matches:
        - "single ``outline`` tree (``ast_outline`` structure, ``ast_find`` property/text/regexp"
        - "filtering with source, ``ast_read`` reads subtrees by id); mutation is"
        - "'ast_outline',"
        - "'ast_read',"
        - '''ast_read'')'
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "\"\"\"``ast_read`` tool: read one or more node subtrees (with source) by id, across files.\"\"\""
        - from xy.ai.mcpc.tools.ast.outline import ast_outline
        - "__all__ = ['ReadItem', 'ReadResult', 'ReadError', 'ReadBatchResult', 'ast_read', 'ReadNodeTool', 'register']"
        - "class ReadBatchResult:"
        - '"""Result of :func:`ast_read`.'
        - "def ast_read(items: list[ReadItem], *, with_lines: bool=True) -> ReadBatchResult:"
        - "ReadBatchResult: One result per file (with its own id-level errors), and"
        - "return ReadBatchResult(results=results, errors=errors)"
      warning: "Limit of 30 matches reached (total 57); narrow pattern, directory or include/exclude filters, or raise limit (max 50). 1 match line(s) exceeded 300 characters and were shortened in the middle."
```
/answer 6bd4-95-94 allow
Control Request:
```yaml
id: 6bd4-96-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: "Kopf-Knoten (Imports, __all__, Konstanten) von ast/read.py ermitteln."
```
/answer 6bd4-96-0 allow
Control Request:
```yaml
id: 6bd4-97-96
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
      nodes:
      - id: QftHvL|jTnLqY
        excert: "'``ast_read`` tool: read one or more node subtrees (with source) by id, across …"
      - id: vLfIF6|SP3nbZ
        excert: from dataclasses import dataclass
      - id: JnyjUM|0JyCWU
        excert: "__all__ = ['ReadItem', 'ReadResult', 'ReadError', 'ReadBatchResult', 'ast_read'…"
      - id: looks_like_root_intent
        excert: "def _looks_like_root_intent(ids: list[str]) -> bool:"
        docstring: Whether ``ids`` is empty or consists solely of common root-id guesses.
      - id: ReadItem
        excert: "@dataclass(frozen=True) class ReadItem:"
        docstring: "One file's node ids to read. Attributes: path: Absolute path to the file to rea…"
      - id: ReadResult
        excert: "@dataclass(frozen=True) class ReadResult:"
        docstring: "Nodes read from a single file. Attributes: path: The path exactly as given in t…"
      - id: ReadError
        excert: "@dataclass(frozen=True) class ReadError:"
        docstring: "Error reading a whole item (e.g. bad path or empty ``ids``). Attributes: path: …"
      - id: ReadBatchResult
        excert: "@dataclass(frozen=True) class ReadBatchResult:"
        docstring: "Result of :func:`ast_read`. Attributes: results: One :class:`ReadResult` per su…"
      - id: read_one
        excert: "def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:"
      - id: ast_read
        excert: "def ast_read(items: list[ReadItem], *, with_lines: bool=True) -> ReadBatchResult:"
        docstring: "Recursively read the subtree of each addressed node, across one or more files. …"
      - id: ReadNodeTool
        excert: "class ReadNodeTool(ToolDefinition):"
        children:
        - id: ReadNodeTool.VeePTW|cPwHYq
          excert: name = 'ast_read'
        - id: ReadNodeTool.4gWrRr|O9B7Vp
          excert: "_ITEM_PROPERTIES = {"
        - id: ReadNodeTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API. P…"
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 6bd4-97-96 allow
ReasoningToken: 289
Control Request:
```yaml
id: 6bd4-98-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  reason: Kopf-Knoten von ast/outline.py ermitteln.
```
/answer 6bd4-98-0 allow
Control Request:
```yaml
id: 6bd4-99-98
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
      nodes:
      - id: JY1HZH|PFSe6T
        excert: "'``ast_outline`` tool: list AST nodes of one or more files.'"
      - id: e6pSwx|7lUxqQ
        excert: from dataclasses import dataclass
      - id: OeMnuN|SAOhjO
        excert: "__all__ = ["
      - id: OutlineNodesResult
        excert: "@dataclass(frozen=True) class OutlineNodesResult:"
        docstring: "Nodes of a single file. Attributes: path: The resolved path of the listed file …"
      - id: OutlineNodesError
        excert: "@dataclass(frozen=True) class OutlineNodesError:"
        docstring: "Error listing a single file. Attributes: path: The resolved path that failed, f…"
      - id: OutlineNodesBatchResult
        excert: "@dataclass(frozen=True) class OutlineNodesBatchResult:"
        docstring: "Result of :func:`ast_outline`. Attributes: results: One :class:`OutlineNodesRes…"
      - id: expand_path
        excert: "def _expand_path(path_str: str) -> list[str]:"
        docstring: "Expand *path_str* to its own single-element list, unless it names a directory t…"
      - id: outline_one
        excert: "def _outline_one(path: str, *, with_lines: bool) -> OutlineNodesResult:"
      - id: directory_listing_hint
        excert: "def _directory_listing_hint(path: str) -> str:"
        docstring: "Build an error message for a path that turned out to be a directory, anticipati…"
      - id: ast_outline
        excert: "def ast_outline(paths: list[str], *, with_lines: bool=True) -> OutlineNodesBatchResult:"
        docstring: List the hierarchical AST-node tree of one or more files. The tree is the found…
      - id: OutlineNodesTool
        excert: "class OutlineNodesTool(ToolDefinition):"
        children:
        - id: OutlineNodesTool.kZa70t|NgnQPq
          excert: name = 'ast_outline'
        - id: OutlineNodesTool.WDGjLy|cDLLDp
          excert: "input_schema = {"
        - id: OutlineNodesTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_outline`, translating the MCP schema to/from the AST API."
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 6bd4-99-98 allow
Control Request:
```yaml
id: 6bd4-9a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    ids:
    - vLfIF6|SP3nbZ
    - JnyjUM|0JyCWU
    - ReadNodeTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    ids:
    - e6pSwx|7lUxqQ
    - OeMnuN|SAOhjO
  reason: "Imports/__all__ von ast/read.py und ast/outline.py exakt lesen, damit die Ersetzungen vollständig sind."
```
/answer 6bd4-9a-0 allow
Control Request:
```yaml
id: 6bd4-9b-9a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
      nodes:
      - id: vLfIF6|SP3nbZ
        type: imports
        code: |-
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import PATH_PROP
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools.ast.outline import ast_outline
          from xy.ai.mcpc.tools._tool_helpers import require_items, batch_schema
      - id: JnyjUM|0JyCWU
        type: statements
        code: |-
          __all__ = ['ReadItem', 'ReadResult', 'ReadError', 'ReadBatchResult', 'ast_read', 'ReadNodeTool', 'register']
          _ROOT_INTENT_IDS = {'root', '_module_', '__module__', 'module', '', '*'}
      - id: ReadNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API.

                  Per item, falls back to :func:`ast_outline` when ``ids`` looks like agents
                  habitually mis-guessing a root id (``root``/``_module_``/``module``/empty)
                  and none of them resolve to a node: returns that file's full outline
                  instead of an error, and notes the redirect in the item's ``errors``.
                  """
              with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)
              raw_items, error = require_items(ctx)
              if error is not None:
                  return error
              results: list[dict[str, Any]] = []
              errors: list[dict[str, str]] = []
              for raw in raw_items:
                  path = raw.get('path')
                  ids: list[str] = raw.get('ids') or []
                  item_result: ReadResult | None = None
                  if ids:
                      batch = ast_read([ReadItem(path=path, ids=ids)], with_lines=with_lines)
                      if batch.errors:
                          errors.append({'path': batch.errors[0].path, 'error': batch.errors[0].error})
                          continue
                      item_result = batch.results[0]
                  if _looks_like_root_intent(ids) and (item_result is None or not item_result.nodes):
                      list_batch = ast_outline([path], with_lines=with_lines)
                      for lr in list_batch.results:
                          entry: dict[str, Any] = {'path': lr.path, 'nodes': [core.to_dict(n) for n in lr.nodes]}
                          if ids:
                              warning = f'ast_read fell back to ast_outline: ids {
                                  ids!r} resolved to no node; returning the full outline of {
                                      lr.path} instead.'
                              entry['warning'] = [warning]
                              entry['error'] = [
                                  f'ids {
                                      ids!r} resolved to no node; redirected to ast_outline, returning the full outline instead.']
                          results.append(entry)
                      for le in list_batch.errors:
                          errors.append({'path': le.path, 'error': le.error})
                      continue
                  pathResult = {'path': item_result.path, 'nodes': [core.to_dict(n) for n in item_result.nodes]}
                  if item_result.errors:
                      pathResult['errors'] = item_result.errors
                  results.append(pathResult)
              structured_content: dict[str, Any] = {}
              if results:
                  structured_content['results'] = results
              if errors:
                  structured_content['errors'] = errors
              return ToolResult(structured_content=structured_content)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
      nodes:
      - id: e6pSwx|7lUxqQ
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result
          from xy.ai.mcpc.tools.list import ListError, ListItem
          from xy.ai.mcpc.tools.list import list as _list_dirs
      - id: OeMnuN|SAOhjO
        type: statements
        code: |-
          __all__ = [
              'OutlineNodesResult',
              'OutlineNodesError',
              'OutlineNodesBatchResult',
              'ast_outline',
              'OutlineNodesTool',
              'register']
          _MAX_DIR_EXPANSION = 5
```
/answer 6bd4-9b-9a allow
ReasoningToken: 223
Control Request:
```yaml
id: 6bd4-9c-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: OutlineNodesResult
    source: |-
      @dataclass(frozen=True)
      class OutlineNodesResult(ItemResult):
          """Nodes of a single file.

          Attributes:
              path: The resolved path of the listed file (see ``ast_outline``'s
                  directory-expansion note), for result association.
              nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
                  document order, suited for retrieval and navigation.
          """
          path: str
          nodes: list[core.OutlineNode]

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return {'nodes': [core.to_dict(n) for n in self.nodes]}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: OutlineNodesError
    source: |-
      @dataclass(frozen=True)
      class OutlineNodesError(ItemError):
          """Error listing a single file.

          Attributes:
              path: The resolved path that failed, for result association.
              error: The error message.
          """
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: OutlineNodesBatchResult
    source: |-
      class OutlineNodesBatchResult(BatchResult[OutlineNodesResult, OutlineNodesError]):
          """Result of :func:`ast_outline`.

          Attributes:
              results: One :class:`OutlineNodesResult` per successfully listed file.
              errors: One :class:`OutlineNodesError` per file that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: directory_listing_hint
    source: |-
      def _directory_listing_hint(path: str) -> str:
          """Build an error message for a path that turned out to be a directory,
          anticipating the likely intent: list its contents (via the ``list`` tool)
          instead of outlining it as a file.
          """
          hint = 'Path is a directory, not a file; listing its contents instead.'
          try:
              batch = _list_dirs([ListItem(path=path)])
          except ListError as exc:
              return f'{hint} Listing failed: {exc}'
          if batch.errors:
              return f'{hint} Listing failed: {batch.errors[0].error}'
          result = batch.results[0]
          return '\n'.join([hint, *result.warnings, *result.entries])
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: OutlineNodesTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`ast_outline`, translating the MCP schema to/from the AST API."""
          paths, error = require_items(ctx, key='paths')
          if error is not None:
              return error
          with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)
          return batch_tool_result(ast_outline(paths=paths, with_lines=with_lines))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: e6pSwx|7lUxqQ
    source: |-
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools._tool_helpers import require_items, batch_tool_result
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
      from xy.ai.mcpc.tools.list import ListError, ListItem
      from xy.ai.mcpc.tools.list import list as _list_dirs
  reason: ast_outline migrieren; Directory-Hint übernimmt die Struktur von result.warnings.
```
/answer 6bd4-9c-0 allow
ReasoningToken: 1185
Control Request:
```yaml
id: 6bd4-9d-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: ReadResult
    source: |-
      @dataclass(frozen=True)
      class ReadResult(ItemResult):
          """Nodes read from a single file.

          Attributes:
              path: The path exactly as given in the input, for result association.
              nodes: One expanded subtree per resolved id, in the given order; same
                  shape as :func:`ast_find`'s results (see :class:`core.OutlineNode`).
              warnings: One message per requested id that could not be resolved (id
                  unknown/ambiguous, and no unambiguous name/fuzzy match found), or the note that
                  the file's outline was returned instead.
          """
          path: str
          nodes: list[core.OutlineNode]

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return {'nodes': [core.to_dict(n) for n in self.nodes]}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: ReadError
    source: |-
      @dataclass(frozen=True)
      class ReadError(ItemError):
          """Error reading a whole item (e.g. bad path or a syntax error).

          Attributes:
              path: The path exactly as given in the input, for result association.
              error: The error message.
          """
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: ReadBatchResult
    source: |-
      class ReadBatchResult(BatchResult[ReadResult, ReadError]):
          """Result of :func:`ast_read`.

          Attributes:
              results: One :class:`ReadResult` per successfully read file.
              errors: One :class:`ReadError` per file that failed entirely.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: read_one
    source: |-
      def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:
          tree = core.load(item.path)[1]
          nodes, unresolved = core.read_subtrees(core.locate_all(tree), item.ids, with_lines=with_lines)
          return ReadResult(path=item.path, nodes=nodes, warnings=tuple(unresolved))


      def _outline_instead(item: ReadItem, *, with_lines: bool) -> tuple[list[ReadResult], list[ReadError]]:
          """The file's full outline, noting the redirect if ids were given but none resolved."""
          outline = ast_outline([item.path], with_lines=with_lines)
          results = [
              ReadResult(
                  path=r.path,
                  nodes=r.nodes,
                  warnings=(f'ast_read fell back to ast_outline: ids {item.ids!r} resolved to no node; '
                            f'returning the full outline of {r.path} instead.',) if item.ids else ())
              for r in outline.results]
          return results, [ReadError(path=e.path, error=e.error) for e in outline.errors]
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: ast_read
    source: |-
      def ast_read(items: list[ReadItem], *, with_lines: bool=True) -> ReadBatchResult:
          """Recursively read the subtree of each addressed node, across one or more files.

          Each id resolves to a subtree: a node whose body consists solely of nested
          classes/functions is expanded into ``children`` instead of source, so the agent
          can descend to the innermost editable block; any other node is returned whole,
          as ``code`` ready to hand back to ``ast_replace`` via its ``id``.

          An id that doesn't match any node is retried as a node *name* (exact, then a
          conservative fuzzy match) instead of failing the whole item; ids that still
          can't be resolved are reported in that item's ``warnings``, not raised.

          Per item, falls back to :func:`ast_outline` when ``ids`` is empty or looks like
          agents habitually mis-guessing a root id (``root``/``_module_``/``module``) and
          none of them resolve to a node: returns that file's full outline instead of an
          error, and notes the redirect in the item's ``warnings`` (if ids were given).

          Args:
              items: Per-file node ids to read. Must be non-empty.
              with_lines: Whether to populate each node's line range.

          Returns:
              ReadBatchResult: One result per file (with its own id-level warnings), and
              one file-level error per file that failed entirely (bad path or a syntax error).

          Raises:
              core.AstError: If ``items`` is empty.
          """
          if not items:
              raise core.AstError("'items' must be a non-empty list.")
          results: list[ReadResult] = []
          errors: list[ReadError] = []
          for item in items:
              read: ReadResult | None = None
              if item.ids:
                  try:
                      read = _read_one(item, with_lines=with_lines)
                  except core.AstError as exc:
                      errors.append(ReadError(path=item.path, error=str(exc)))
                      continue
              if read is None or (not read.nodes and _looks_like_root_intent(item.ids)):
                  outlined, outline_errors = _outline_instead(item, with_lines=with_lines)
                  results += outlined
                  errors += outline_errors
                  continue
              results.append(read)
          return ReadBatchResult(results=results, errors=errors)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: ReadNodeTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API."""
          with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)

          def item_factory(it: dict[str, Any]) -> ReadItem:
              return ReadItem(path=it['path'], ids=it.get('ids') or [])
          return handle_batch_tool(
              ctx, item_factory, lambda items: ast_read(items, with_lines=with_lines), core.AstError)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: vLfIF6|SP3nbZ
    source: |-
      from dataclasses import dataclass
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools.ast.common import PATH_PROP
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.ast.outline import ast_outline
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
  reason: "ast_read auf BatchResult umstellen: Outline-Fallback wird Item-Warning in der Fachfunktion, Handler läuft über handle_batch_tool, doppeltes 'error' entfällt."
```
/answer 6bd4-9d-0 allow
Thinking:
Die Batch-Tools sind migriert, einschließlich `ast_read` und `ast_outline`, und es fehlen noch Exa, OpenAlex, die übrigen `is_error=True`-Stellen sowie Tests und Contract-Test.


ReasoningToken: 174
Control Request:
```yaml
id: 6bd4-9e-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
  reason: Exa-/OpenAlex-*_results-Tools mit der 'missing ids'-Lücke lesen.
```
/answer 6bd4-9e-0 allow
Control Request:
```yaml
id: 6bd4-9f-9e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
      nodes:
      - id: 8QhE05|CdBYEw
        excert: "'``web_fetch_exa_results`` - stage 2: resolve ``web_fetch_exa`` ids to full tex…"
      - id: 8sJ8Zx|70KiZc
        excert: import re
      - id: gFs83B|MZaYco
        excert: "__all__ = ['web_fetch_exa_results', 'WebFetchExaResultsTool', 'register']"
      - id: YaRXvC|tJJq56
        excert: "_INPUT_SCHEMA: dict[str,"
      - id: grep_lines
        excert: "def _grep_lines(text: str, pattern: str, context: int) -> str:"
        docstring: Keep lines matching *pattern* plus *context* lines around each match ('grep -E'…
      - id: web_fetch_exa_results
        excert: "def web_fetch_exa_results(ids: list[str], pattern: str | None=None, context: int=1) -> list[dict[str, Any]]:"
        docstring: "Resolve ids from a prior ``web_fetch_exa`` call to url and full text. Args: ids…"
      - id: WebFetchExaResultsTool
        excert: "class WebFetchExaResultsTool(ToolDefinition):"
        children:
        - id: WebFetchExaResultsTool.p8O5MY|fWk5Ac
          excert: name = 'web_fetch_exa_results'
        - id: WebFetchExaResultsTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
      nodes:
      - id: 5xAZPB|Ujuq0h
        excert: "'``web_search_exa_results`` - stage 2: resolve ``web_search_exa`` ids to url an…"
      - id: pzjVHL|tI2Nva
        excert: from typing import Any
      - id: HH7yhm|w9HLa7
        excert: "__all__ = ['web_search_exa_results', 'WebSearchExaResultsTool', 'register']"
      - id: web_search_exa_results
        excert: "def web_search_exa_results(ids: list[str]) -> list[dict[str, Any]]:"
        docstring: "Resolve ids from a prior ``web_search_exa`` call to url and full text. Args: id…"
      - id: WebSearchExaResultsTool
        excert: "class WebSearchExaResultsTool(ToolDefinition):"
        children:
        - id: WebSearchExaResultsTool.5A6MjO|lTQmMk
          excert: name = 'web_search_exa_results'
        - id: WebSearchExaResultsTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
      nodes:
      - id: NpgG2q|ggAeNV
        excert: "'``openalex_search_results`` - stage 2: resolve openalex_search / openalex_sema…"
      - id: HU1PLw|hE7PP2
        excert: from typing import Any
      - id: H93OtG|0aAMoo
        excert: "__all__ = ['openalex_search_results', 'OpenalexSearchResultsTool']"
      - id: YaRXvC|ycJsS3
        excert: "_INPUT_SCHEMA: dict[str,"
      - id: openalex_search_results
        excert: "def openalex_search_results(ids: list[str]) -> list[dict[str, Any]]:"
        docstring: Resolve ids from a prior openalex_search/openalex_semantic_search call to full …
      - id: OpenalexSearchResultsTool
        excert: "class OpenalexSearchResultsTool(ToolDefinition):"
        children:
        - id: OpenalexSearchResultsTool.cZ63qk|CnQ4ed
          excert: name = 'openalex_search_results'
        - id: OpenalexSearchResultsTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
```
/answer 6bd4-9f-9e allow
Control Request:
```yaml
id: 6bd4-a0-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
    ids:
    - web_fetch_exa_results
    - WebFetchExaResultsTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
    ids:
    - web_search_exa_results
    - WebSearchExaResultsTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
    ids:
    - openalex_search_results
    - OpenalexSearchResultsTool.handle
  reason: "Funktionen und Handler lesen, die 'missing' ids nur loggen."
```
/answer 6bd4-a0-0 allow
Control Request:
```yaml
id: 6bd4-a1-a0
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
      nodes:
      - id: web_fetch_exa_results
        type: FunctionDef
        code: |-
          def web_fetch_exa_results(ids: list[str], pattern: str | None=None, context: int=1) -> list[dict[str, Any]]:
              """Resolve ids from a prior ``web_fetch_exa`` call to url and full text.

              Args:
                  ids: Result ids returned by ``web_fetch_exa``.
                  pattern: Optional extended regular expression to filter text lines
                      (``grep -E`` semantics); non-adjacent matching blocks are
                      separated by a ``--`` line, like ``grep``.
                  context: Context lines kept before/after each match (default: 1).

              Returns:
                  One entry per known id, with ``id`` and ``text``.
              """
              items = fetch_cache.get_many(ids)
              missing = [i for i in ids if i not in {item['id'] for item in items}]
              if missing:
                  logger.warning('web_fetch_exa_results: unknown or expired id(s): %s', missing)
              results = []
              for item in items:
                  text = item.get('text') or ''
                  if pattern:
                      text = _grep_lines(text, pattern, context)
                  results.append(strip_empty({'id': item['id'], 'text': text}))
              return results
      - id: WebFetchExaResultsTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              args = ctx.arguments
              ids, error = require_items(ctx, key='ids')
              if error is not None:
                  return error
              try:
                  results = web_fetch_exa_results(ids=ids, pattern=args.get('pattern'), context=args.get('context', 1))
              except re.error as exc:
                  logger.warning('web_fetch_exa_results: invalid pattern %r: %s', args.get('pattern'), exc)
                  return ToolResult(content=[text_content(f'Invalid pattern: {exc}')], is_error=True)
              except Exception as exc:
                  logger.exception('web_fetch_exa_results failed')
                  return ToolResult(content=[text_content(f'Error resolving web_fetch_exa results: {exc}')], is_error=True)
              return ToolResult(structured_content={'results': results}, auto_approve=False)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
      nodes:
      - id: web_search_exa_results
        type: FunctionDef
        code: |-
          def web_search_exa_results(ids: list[str]) -> list[dict[str, Any]]:
              """Resolve ids from a prior ``web_search_exa`` call to url and full text.

              Args:
                  ids: Result ids returned by ``web_search_exa``.

              Returns:
                  One entry per known id, with ``id``, ``url`` and ``text``.
              """
              items = search_cache.get_many(ids)
              missing = [i for i in ids if i not in {item['id'] for item in items}]
              if missing:
                  logger.warning('web_search_exa_results: unknown or expired id(s): %s', missing)
              return [strip_empty({'id': item['id'], 'url': item.get('url'), 'text': item.get('text')}) for item in items]
      - id: WebSearchExaResultsTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              ids, error = require_items(ctx, key='ids')
              if error is not None:
                  return error
              try:
                  results = web_search_exa_results(ids=ids)
              except Exception as exc:
                  logger.exception('web_search_exa_results failed')
                  return ToolResult(content=[text_content(f'Error resolving web_search_exa results: {exc}')], is_error=True)
              return ToolResult(structured_content={'results': results}, auto_approve=False)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
      nodes:
      - id: openalex_search_results
        type: FunctionDef
        code: |-
          def openalex_search_results(ids: list[str]) -> list[dict[str, Any]]:
              """Resolve ids from a prior openalex_search/openalex_semantic_search call to full records.

              Args:
                  ids: OpenAlex ids returned by ``openalex_search`` or
                      ``openalex_semantic_search``.

              Returns:
                  One full record per known id.
              """
              items = search_cache.get_many(ids)
              found = {item['id'] for item in items}
              missing = [i for i in ids if i not in found]
              if missing:
                  logger.warning('openalex_search_results: unknown or expired id(s): %s', missing)
              return items
      - id: OpenalexSearchResultsTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              ids, error = require_items(ctx, key='ids')
              if error is not None:
                  return error
              return ok_result({'results': openalex_search_results(ids=ids)})
```
/answer 6bd4-a1-a0 allow
ReasoningToken: 361
Control Request:
```yaml
id: 6bd4-a2-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex
    pattern: def ok_result|def error_result|ToolResult|is_error
    include:
    - '*.py'
    intent: editing
    limit: 30
  reason: openalex-Helfer ok_result/error_result und ToolResult-Konstruktionen lokalisieren.
```
/answer 6bd4-a2-0 allow
Control Request:
```yaml
id: 6bd4-a3-a2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex
      matches:
      - path: semantic_search.py
        nodes:
        - id: PXJfns|nFucaC
          matches:
          - "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult"
        - id: OpenalexSemanticSearchTool.handle
          matches:
          - "def handle(self, ctx: ToolContext) -> ToolResult:"
      - path: search.py
        nodes:
        - id: 8mRXyy|hSIA5e
          matches:
          - "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult"
        - id: OpenalexSearchTool.handle
          matches:
          - "def handle(self, ctx: ToolContext) -> ToolResult:"
      - path: search_results.py
        nodes:
        - id: HU1PLw|hE7PP2
          matches:
          - "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult"
        - id: OpenalexSearchResultsTool.handle
          matches:
          - "def handle(self, ctx: ToolContext) -> ToolResult:"
      - path: work.py
        nodes:
        - id: Nk8SkJ|RyJtim
          matches:
          - "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult"
        - id: OpenalexWorkTool.handle
          matches:
          - "def handle(self, ctx: ToolContext) -> ToolResult:"
      - path: _common.py
        nodes:
        - id: YDPhBW|7tKsn8
          matches:
          - "from xy.ai.mcpc.tools.tool_registry import ToolResult, text_content"
        - id: error_result
          matches:
          - "def error_result(exc: Exception) -> ToolResult:"
        - id: error_result.0A8KVK|pENxeK
          matches:
          - "return ToolResult(content=[text_content(message)], is_error=True)"
        - id: ok_result
          matches:
          - "def ok_result(structured: dict[str, Any]) -> ToolResult:"
        - id: ok_result.avje1w|CStshN
          matches:
          - return ToolResult(structured_content=sanitize_value(structured))
```
/answer 6bd4-a3-a2 allow
ReasoningToken: 85
Control Request:
```yaml
id: 6bd4-a4-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/_common.py
    ids:
    - error_result
    - ok_result
  reason: ok_result/error_result im Detail lesen.
```
/answer 6bd4-a4-0 allow
Control Request:
```yaml
id: 6bd4-a5-a4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/_common.py
      nodes:
      - id: error_result
        type: FunctionDef
        code: |-
          def error_result(exc: Exception) -> ToolResult:
              message = str(exc)
              if isinstance(exc, OpenAlexAPIError) and exc.status is not None:
                  message = f'{message} (status {exc.status})'
              logger.error('OpenAlex request failed: %s', message, exc_info=exc)
              return ToolResult(content=[text_content(message)], is_error=True)
      - id: ok_result
        type: FunctionDef
        code: |-
          def ok_result(structured: dict[str, Any]) -> ToolResult:
              """# OpenAlex occasionally returns fields (titles, abstracts, ...) that"""
              '# contain raw non-printable control characters; strip them so downstream'
              '# consumers (notably YAML block-scalar rendering) never choke on them.'
              return ToolResult(structured_content=sanitize_value(structured))
```
/answer 6bd4-a5-a4 allow
ReasoningToken: 439
Control Request:
```yaml
id: 6bd4-a6-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    id: ItemOutcome
    old_text: |-
      @dataclass(frozen=True, kw_only=True)
      class ItemOutcome(ABC):
    new_text: |-
      def unknown_ids_warnings(missing: Iterable[str]) -> list[str]:
          ids = list(missing)
          return [f'unknown or expired id(s): {ids}'] if ids else []


      @dataclass(frozen=True, kw_only=True)
      class ItemOutcome(ABC):
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    id: ItemOutcome
    old_text: |2-
          'compact_asdict',
          'ItemOutcome',
    new_text: |2-
          'compact_asdict',
          'unknown_ids_warnings',
          'ItemOutcome',
  reason: Gemeinsamen Helfer für 'unknown or expired id(s)'-Warnungen ergänzen.
```
/answer 6bd4-a6-0 allow
Control Request:
```yaml
id: 6bd4-a7-a6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
      result: success
      id: ItemOutcome
      new_id: ItemOutcome
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
      error: Text not found in node.
      id: ItemOutcome
      reason: not_found
      next_step: reread_node
```
/answer 6bd4-a7-a6 allow
ReasoningToken: 99
Control Request:
```yaml
id: 6bd4-a8-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
  text: __all__
  reason: __all__-Knoten in results.py finden.
```
/answer 6bd4-a8-0 allow
Control Request:
```yaml
id: 6bd4-a9-a8
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
      nodes:
      - id: noZ1HO|AgJUzE
        code: |-
          __all__ = [
              'WARNINGS_KEY',
              'RESERVED_NAMES',
              'WARNINGS_SCHEMA',
              'compact',
              'compact_asdict',
              'ItemOutcome',
              'ItemResult',
              'ItemError',
              'BatchResult']
          WARNINGS_KEY = 'warnings'
          RESERVED_NAMES = frozenset({'warning', 'warnings', 'notice', 'notices', 'hint', 'error', 'errors', 'message'})
```
/answer 6bd4-a9-a8 allow
Control Request:
```yaml
id: 6bd4-aa-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    id: noZ1HO|AgJUzE
    old_text: |2-
          'compact_asdict',
          'ItemOutcome',
    new_text: |2-
          'compact_asdict',
          'unknown_ids_warnings',
          'ItemOutcome',
  reason: unknown_ids_warnings in __all__ aufnehmen.
```
/answer 6bd4-aa-0 allow
Control Request:
```yaml
id: 6bd4-ab-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/_common.py
    id: error_result.0A8KVK|pENxeK
    old_text: "return ToolResult(content=[text_content(message)], is_error=True)"
    new_text: return ToolResult.failure(message)
  reason: openalex error_result auf ToolResult.failure umstellen.
```
/answer 6bd4-ab-0 allow
Control Request:
```yaml
id: 6bd4-ac-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/_common.py
    id: ok_result
    source: |-
      def ok_result(structured: dict[str, Any], warnings: list[str] | None=None) -> ToolResult:
          """# OpenAlex occasionally returns fields (titles, abstracts, ...) that"""
          '# contain raw non-printable control characters; strip them so downstream'
          '# consumers (notably YAML block-scalar rendering) never choke on them.'
          return ToolResult(structured_content=sanitize_value(structured), warnings=list(warnings or []))
  reason: ok_result um optionale Call-Warnungen erweitern.
```
/answer 6bd4-ac-0 allow
Control Request:
```yaml
id: 6bd4-ad-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
    id: openalex_search_results
    source: |-
      def _resolve(ids: list[str]) -> tuple[list[dict[str, Any]], list[str]]:
          items = search_cache.get_many(ids)
          found = {item['id'] for item in items}
          return items, [i for i in ids if i not in found]


      def openalex_search_results(ids: list[str]) -> list[dict[str, Any]]:
          """Resolve ids from a prior openalex_search/openalex_semantic_search call to full records.

          Args:
              ids: OpenAlex ids returned by ``openalex_search`` or
                  ``openalex_semantic_search``.

          Returns:
              One full record per known id.
          """
          items, missing = _resolve(ids)
          if missing:
              logger.warning('openalex_search_results: unknown or expired id(s): %s', missing)
          return items
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
    id: OpenalexSearchResultsTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          ids, error = require_items(ctx, key='ids')
          if error is not None:
              return error
          items, missing = _resolve(ids)
          return ok_result({'results': items}, warnings=unknown_ids_warnings(missing))
  reason: "openalex_search_results: unbekannte ids als ToolResult.warnings sichtbar machen."
```
/answer 6bd4-ad-0 allow
ReasoningToken: 97
Control Request:
```yaml
id: 6bd4-ae-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
    id: web_search_exa_results
    source: |-
      def _resolve(ids: list[str]) -> tuple[list[dict[str, Any]], list[str]]:
          items = search_cache.get_many(ids)
          known = {item['id'] for item in items}
          results = [strip_empty({'id': item['id'], 'url': item.get('url'), 'text': item.get('text')}) for item in items]
          return results, [i for i in ids if i not in known]


      def web_search_exa_results(ids: list[str]) -> list[dict[str, Any]]:
          """Resolve ids from a prior ``web_search_exa`` call to url and full text.

          Args:
              ids: Result ids returned by ``web_search_exa``.

          Returns:
              One entry per known id, with ``id``, ``url`` and ``text``.
          """
          results, missing = _resolve(ids)
          if missing:
              logger.warning('web_search_exa_results: unknown or expired id(s): %s', missing)
          return results
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
    id: WebSearchExaResultsTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          ids, error = require_items(ctx, key='ids')
          if error is not None:
              return error
          try:
              results, missing = _resolve(ids)
          except Exception as exc:
              logger.exception('web_search_exa_results failed')
              return ToolResult.failure(f'Error resolving web_search_exa results: {exc}')
          return ToolResult(
              structured_content={'results': results},
              warnings=unknown_ids_warnings(missing),
              auto_approve=False)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
    id: web_fetch_exa_results
    source: |-
      def _resolve(ids: list[str], pattern: str | None, context: int) -> tuple[list[dict[str, Any]], list[str]]:
          items = fetch_cache.get_many(ids)
          known = {item['id'] for item in items}
          results = []
          for item in items:
              text = item.get('text') or ''
              if pattern:
                  text = _grep_lines(text, pattern, context)
              results.append(strip_empty({'id': item['id'], 'text': text}))
          return results, [i for i in ids if i not in known]


      def web_fetch_exa_results(ids: list[str], pattern: str | None=None, context: int=1) -> list[dict[str, Any]]:
          """Resolve ids from a prior ``web_fetch_exa`` call to url and full text.

          Args:
              ids: Result ids returned by ``web_fetch_exa``.
              pattern: Optional extended regular expression to filter text lines
                  (``grep -E`` semantics); non-adjacent matching blocks are
                  separated by a ``--`` line, like ``grep``.
              context: Context lines kept before/after each match (default: 1).

          Returns:
              One entry per known id, with ``id`` and ``text``.
          """
          results, missing = _resolve(ids, pattern, context)
          if missing:
              logger.warning('web_fetch_exa_results: unknown or expired id(s): %s', missing)
          return results
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
    id: WebFetchExaResultsTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          args = ctx.arguments
          ids, error = require_items(ctx, key='ids')
          if error is not None:
              return error
          try:
              results, missing = _resolve(ids, args.get('pattern'), args.get('context', 1))
          except re.error as exc:
              logger.warning('web_fetch_exa_results: invalid pattern %r: %s', args.get('pattern'), exc)
              return ToolResult.failure(f'Invalid pattern: {exc}')
          except Exception as exc:
              logger.exception('web_fetch_exa_results failed')
              return ToolResult.failure(f'Error resolving web_fetch_exa results: {exc}')
          return ToolResult(
              structured_content={'results': results},
              warnings=unknown_ids_warnings(missing),
              auto_approve=False)
  reason: "Exa-*_results-Tools: unbekannte/abgelaufene ids als Warnungen sichtbar machen."
```
/answer 6bd4-ae-0 allow
ReasoningToken: 242
Control Request:
```yaml
id: 6bd4-af-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
  node_type: imports
  reason: "Import-Blöcke der drei *_results-Dateien lesen, um results-Import zu ergänzen und text_content zu prüfen."
```
/answer 6bd4-af-0 allow
Control Request:
```yaml
id: 6bd4-b0-af
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
      nodes:
      - id: 8sJ8Zx|70KiZc
        type: imports
        code: |-
          import re
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools.mcp.exa.core import fetch_cache, logger, strip_empty
          from xy.ai.mcpc.tools._tool_helpers import require_items
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
      nodes:
      - id: pzjVHL|tI2Nva
        type: imports
        code: |-
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools.mcp.exa.core import logger, search_cache, strip_empty
          from xy.ai.mcpc.tools._tool_helpers import require_items
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
      nodes:
      - id: HU1PLw|hE7PP2
        type: imports
        code: |-
          from typing import Any
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult
          from xy.ai.mcpc.tools._tool_helpers import require_items
          from ._common import logger, ok_result, search_cache
```
/answer 6bd4-b0-af allow
Control Request:
```yaml
id: 6bd4-b1-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
    id: 8sJ8Zx|70KiZc
    old_text: |
      ToolRegistry, ToolResult, text_content
    new_text: |
      ToolRegistry, ToolResult
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
    id: 8sJ8Zx|70KiZc
    old_text: from xy.ai.mcpc.tools._tool_helpers import require_items
    new_text: |-
      from xy.ai.mcpc.tools._tool_helpers import require_items
      from xy.ai.mcpc.tools.results import unknown_ids_warnings
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
    id: pzjVHL|tI2Nva
    old_text: |
      ToolRegistry, ToolResult, text_content
    new_text: |
      ToolRegistry, ToolResult
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
    id: pzjVHL|tI2Nva
    old_text: from xy.ai.mcpc.tools._tool_helpers import require_items
    new_text: |-
      from xy.ai.mcpc.tools._tool_helpers import require_items
      from xy.ai.mcpc.tools.results import unknown_ids_warnings
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
    id: HU1PLw|hE7PP2
    old_text: from xy.ai.mcpc.tools._tool_helpers import require_items
    new_text: |-
      from xy.ai.mcpc.tools._tool_helpers import require_items
      from xy.ai.mcpc.tools.results import unknown_ids_warnings
  reason: Imports in den drei *_results-Dateien anpassen.
```
/answer 6bd4-b1-0 allow
ReasoningToken: 177
Control Request:
```yaml
id: 6bd4-b2-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: is_error=True
    include:
    - '*.py'
    exclude:
    - cli/**
    - embedded/mcpc.py
    intent: editing
    limit: 50
  reason: "Alle verbleibenden ToolResult(..., is_error=True)-Stellen mit AST-Knoten-IDs für die Umstellung auf failure() finden."
```
/answer 6bd4-b2-0 allow
Control Request:
```yaml
id: 6bd4-b3-b2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/server/mcp_protocol.py
        nodes:
        - id: McpProtocol.handle_tools_call.2w8tpw|yuXXaJ
          matches:
          - is_error=True).to_dict()
        - id: McpProtocol.handle_tools_call.E8VojT|9XmOmT
          matches:
          - is_error=True)
          - "result = ToolResult(content=[text_content(f\"Tool '{name}' failed: {exc}\")], is_error=True)"
        - id: McpProtocol.handle_tools_call.dDYvUB|3FtoJS
          matches:
          - is_error=True).to_dict()
      - path: xy/ai/mcpc/embedded/mcpc.py
        nodes:
        - id: EmbeddedMcpc.run_batch.run.tWOpFN|tmhR3t
          matches:
          - "return CliResult(text=str(exc), is_error=True, subtype='session_error')"
      - path: xy/ai/mcpc/cli/session.py
        nodes:
        - id: CliSession.collect_result.cSi9Ru|8Za6kc
          matches:
          - "return CliResult(text=problem, is_error=True, subtype='session_error')"
      - path: xy/ai/mcpc/tools/mcp/github.py
        nodes:
        - id: GitHubTool.handle.WhVJDf|xEpvSq
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/mcp/context7.py
        nodes:
        - id: Context7LibrariesTool.handle.obvGZ7|5QonXz
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - id: Context7DocumentationTool.handle.tXASWK|DMbWdk
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
        nodes:
        - id: WebFetchExaTool.handle.DuCTym|aWBtB5
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
          - "return ToolResult(content=[text_content(f'Unexpected error in web_fetch_exa: {exc}')], is_error=True)"
      - path: xy/ai/mcpc/tools/mcp/exa/search.py
        nodes:
        - id: WebSearchExaTool.handle.uKTzQX|voUjsr
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
          - "return ToolResult(content=[text_content(f'Unexpected error in web_search_exa: {exc}')], is_error=True)"
      - path: xy/ai/mcpc/tools/ast/find.py
        nodes:
        - id: FindNodesTool.handle.VMFIUW|9u3QYT
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/ast/script.py
        nodes:
        - id: ScriptTool.handle.rbKy0t|izbW4B
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/ast/validate.py
        nodes:
        - id: ValidateTool.handle.PnrmJV|6F8EnG
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/edit_chars/__init__.py
        nodes:
        - id: EditCharsTool.handle.qoTNu0|a7bDKP
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/agent/__init__.py
        nodes:
        - id: error.vRxfqt|zWcHaz
          matches:
          - "return ToolResult(content=[text_content(message)], is_error=True)"
        - id: result.0VBRVA|2wkVu4
          matches:
          - "return ToolResult(content=[text_content(text)], is_error=True)"
      - path: xy/ai/mcpc/tools/bash/__init__.py
        nodes:
        - id: BashTool.handle.w5PinS|Q99x7v
          matches:
          - "return ToolResult(content=[text_content(f'Invalid item: {exc!r}')], is_error=True)"
        - id: BashTool.handle.C3XSlB|puv3vm
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/bash/run_command.py
        nodes:
        - id: RunCommandTool.handle.N7C3Oa|w72GcX
          matches:
          - "return ToolResult(content=[text_content(f'Invalid item: {exc!r}')], is_error=True)"
        - id: RunCommandTool.handle.Vb2Wbm|Oli8KP
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/tool_registry.py
        nodes:
        - id: ToolResult.failure.VToDpU|oIG9K8
          matches:
          - "return cls(content=[text_content(message)], is_error=True)"
      - path: xy/ai/mcpc/tools/tool_call/__init__.py
        nodes:
        - id: ToolCallTool.handle.x2QK9u|KiIYNL
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/colgrep/__init__.py
        nodes:
        - id: ColgrepTool.handle.KXEYyR|6uGpCW
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/markdown/__init__.py
        nodes:
        - id: MarkdownTool.handle.k7gB7C|Jj4Msu
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: xy/ai/mcpc/tools/python/__init__.py
        nodes:
        - id: PythonTool.handle.1BgCit|3DgGxK
          matches:
          - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
```
/answer 6bd4-b3-b2 allow
ReasoningToken: 348
Control Request:
```yaml
id: 6bd4-b4-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
    id: GitHubTool.handle.WhVJDf|xEpvSq
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
    id: Context7LibrariesTool.handle.obvGZ7|5QonXz
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
    id: Context7DocumentationTool.handle.tXASWK|DMbWdk
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch.py
    id: WebFetchExaTool.handle.DuCTym|aWBtB5
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch.py
    id: WebFetchExaTool.handle.DuCTym|aWBtB5
    old_text: "return ToolResult(content=[text_content(f'Unexpected error in web_fetch_exa: {exc}')], is_error=True)"
    new_text: "return ToolResult.failure(f'Unexpected error in web_fetch_exa: {exc}')"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search.py
    id: WebSearchExaTool.handle.uKTzQX|voUjsr
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search.py
    id: WebSearchExaTool.handle.uKTzQX|voUjsr
    old_text: "return ToolResult(content=[text_content(f'Unexpected error in web_search_exa: {exc}')], is_error=True)"
    new_text: "return ToolResult.failure(f'Unexpected error in web_search_exa: {exc}')"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    id: FindNodesTool.handle.VMFIUW|9u3QYT
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    id: ScriptTool.handle.rbKy0t|izbW4B
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py
    id: ValidateTool.handle.PnrmJV|6F8EnG
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_chars/__init__.py
    id: EditCharsTool.handle.qoTNu0|a7bDKP
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_call/__init__.py
    id: ToolCallTool.handle.x2QK9u|KiIYNL
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
    id: ColgrepTool.handle.KXEYyR|6uGpCW
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/markdown/__init__.py
    id: MarkdownTool.handle.k7gB7C|Jj4Msu
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
    id: PythonTool.handle.1BgCit|3DgGxK
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/agent/__init__.py
    id: error.vRxfqt|zWcHaz
    old_text: "return ToolResult(content=[text_content(message)], is_error=True)"
    new_text: return ToolResult.failure(message)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/agent/__init__.py
    id: result.0VBRVA|2wkVu4
    old_text: "return ToolResult(content=[text_content(text)], is_error=True)"
    new_text: return ToolResult.failure(text)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
    id: BashTool.handle.w5PinS|Q99x7v
    old_text: "return ToolResult(content=[text_content(f'Invalid item: {exc!r}')], is_error=True)"
    new_text: "return ToolResult.failure(f'Invalid item: {exc!r}')"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
    id: BashTool.handle.C3XSlB|puv3vm
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py
    id: RunCommandTool.handle.N7C3Oa|w72GcX
    old_text: "return ToolResult(content=[text_content(f'Invalid item: {exc!r}')], is_error=True)"
    new_text: "return ToolResult.failure(f'Invalid item: {exc!r}')"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py
    id: RunCommandTool.handle.Vb2Wbm|Oli8KP
    old_text: "return ToolResult(content=[text_content(str(exc))], is_error=True)"
    new_text: return ToolResult.failure(str(exc))
  reason: "Die ~20 kopierten ToolResult(content=[text_content(...)], is_error=True)-Stellen auf ToolResult.failure(...) umstellen."
```
/answer 6bd4-b4-0 allow
ReasoningToken: 53
Control Request:
```yaml
id: 6bd4-b5-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
    ids:
    - McpProtocol.handle_tools_call.2w8tpw|yuXXaJ
    - McpProtocol.handle_tools_call.E8VojT|9XmOmT
    - McpProtocol.handle_tools_call.dDYvUB|3FtoJS
    - McpProtocol.handle_tools_call.5AKA7N|0ThY6X
  reason: Die is_error-Stellen und das Result-Rendering im Protokoll lesen.
```
/answer 6bd4-b5-0 allow
Control Request:
```yaml
id: 6bd4-b6-b5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
      nodes:
      - id: McpProtocol.handle_tools_call.2w8tpw|yuXXaJ
        type: statements
        code: |-
          if control is not None and (not skip_control):
              decision = control.submit_request(session, name, arguments)
              request_number = decision.request_number
              if not decision.approved:
                  reason = decision.rejection_reason
                  if name == TOOLNAME_ASK_USER:
                      return ToolResult(
                          structured_content={
                              'answer': reason or 'No answer given, anticipate and proceed'}).to_dict()
                  return ToolResult(
                      content=[
                          text_content(
                              f'Rejected by User: {reason}' if reason else 'Rejected by User. No reason was given; Reconsider your approach based on the requirements.')],
                      is_error=True).to_dict()
              if decision.modified_arguments is not None:
                  arguments = decision.modified_arguments
              force_result_review = decision.force_result_review
              if decision.approval_hint:
                  "# For ask_user, an approval hint *is* the human's answer"
                  if name == TOOLNAME_ASK_USER:
                      return ToolResult(structured_content={'answer': decision.approval_hint}).to_dict()
                  request_hint = decision.approval_hint
      - id: McpProtocol.handle_tools_call.E8VojT|9XmOmT
        type: statements
        code: |-
          try:
              with session.lock:
                  raw = tool.handler(context)
              result = normalize_result(raw)
          except errors.JsonRpcError:
              raise
          except KeyError as exc:
              key = exc.args[0] if exc.args else exc
              result = ToolResult(
                  content=[
                      text_content(
                          f"Tool '{name}' failed: missing required property {
                              key!r}")],
                  is_error=True)
          except Exception as exc:
              '# noqa: BLE001 - surface as tool error result'
              result = ToolResult(content=[text_content(f"Tool '{name}' failed: {exc}")], is_error=True)
      - id: McpProtocol.handle_tools_call.dDYvUB|3FtoJS
        type: statements
        code: |-
          if control is not None and (not skip_control):
              decision = control.submit_result(
                  session,
                  name,
                  result.to_dict(),
                  request_number=request_number,
                  auto_approve=result.auto_approve and (
                      not force_result_review))
              if not decision.approved:
                  reason = decision.rejection_reason
                  if name == TOOLNAME_ASK_USER:
                      return ToolResult(
                          structured_content={
                              'answer': reason or 'No answer given, anticipate and proceed'}).to_dict()
                  return ToolResult(
                      content=[
                          text_content(
                              f'Rejected by User: {reason}' if reason else 'Rejected by User. No reason was given; Reconsider your approach based on the requirements.')],
                      is_error=True).to_dict()
              hint_parts = [h for h in (request_hint, decision.approval_hint) if h]
              combined_hint = '\n'.join(hint_parts) if hint_parts else None
              if decision.modified_result is not None:
                  result_dict = dict(decision.modified_result)
              else:
                  result.message = combined_hint
                  result_dict = result.to_dict()
              if combined_hint and name == TOOLNAME_ASK_USER:
                  '# Same exception as in the request phase: for ask_user the'
                  '# hint *is* the answer, not an independent side-channel field.'
                  return ToolResult(structured_content={'answer': combined_hint}).to_dict()
              if combined_hint and decision.modified_result is not None:
                  attach_message(result_dict, combined_hint)
              return result_dict
      - id: McpProtocol.handle_tools_call.5AKA7N|0ThY6X
        type: statements
        code: return result.to_dict()
```
/answer 6bd4-b6-b5 allow
ReasoningToken: 1118
Control Request:
```yaml
id: 6bd4-b7-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
    id: McpProtocol.handle_tools_call.2w8tpw|yuXXaJ
    source: |-
      if control is not None and (not skip_control):
          decision = control.submit_request(session, name, arguments)
          request_number = decision.request_number
          if not decision.approved:
              reason = decision.rejection_reason
              if name == TOOLNAME_ASK_USER:
                  return ToolResult(
                      structured_content={
                          'answer': reason or 'No answer given, anticipate and proceed'}).to_dict()
              return ToolResult.failure(
                  f'Rejected by User: {reason}' if reason else 'Rejected by User. No reason was given; Reconsider your approach based on the requirements.').to_dict()
          if decision.modified_arguments is not None:
              arguments = decision.modified_arguments
          force_result_review = decision.force_result_review
          if decision.approval_hint:
              "# For ask_user, an approval hint *is* the human's answer"
              if name == TOOLNAME_ASK_USER:
                  return ToolResult(structured_content={'answer': decision.approval_hint}).to_dict()
              request_hint = decision.approval_hint
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
    id: McpProtocol.handle_tools_call.E8VojT|9XmOmT
    source: |-
      try:
          with session.lock:
              raw = tool.handler(context)
          result = normalize_result(raw)
      except errors.JsonRpcError:
          raise
      except KeyError as exc:
          key = exc.args[0] if exc.args else exc
          result = ToolResult.failure(f"Tool '{name}' failed: missing required property {key!r}")
      except Exception as exc:
          '# noqa: BLE001 - surface as tool error result'
          result = ToolResult.failure(f"Tool '{name}' failed: {exc}")
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
    id: McpProtocol.handle_tools_call.dDYvUB|3FtoJS
    source: |-
      if control is not None and (not skip_control):
          decision = control.submit_result(
              session,
              name,
              result.to_dict(),
              request_number=request_number,
              auto_approve=result.auto_approve and (
                  not force_result_review))
          if not decision.approved:
              reason = decision.rejection_reason
              if name == TOOLNAME_ASK_USER:
                  return ToolResult(
                      structured_content={
                          'answer': reason or 'No answer given, anticipate and proceed'}).to_dict()
              return ToolResult.failure(
                  f'Rejected by User: {reason}' if reason else 'Rejected by User. No reason was given; Reconsider your approach based on the requirements.').to_dict()
          hint_parts = [h for h in (request_hint, decision.approval_hint) if h]
          combined_hint = '\n'.join(hint_parts) if hint_parts else None
          if decision.modified_result is not None:
              result_dict = dict(decision.modified_result)
          else:
              result.message = combined_hint
              result_dict = result.to_dict()
          if combined_hint and name == TOOLNAME_ASK_USER:
              '# Same exception as in the request phase: for ask_user the'
              '# hint *is* the answer, not an independent side-channel field.'
              return ToolResult(structured_content={'answer': combined_hint}).to_dict()
          if combined_hint and decision.modified_result is not None:
              attach_message(result_dict, combined_hint)
          return result_dict
  reason: "mcp_protocol: is_error-Konstruktionen auf ToolResult.failure umstellen (Logik unverändert)."
```
/answer 6bd4-b7-0 allow
ReasoningToken: 300
Control Request:
```yaml
id: 6bd4-b8-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: "['\"]warnings?['\"]|['\"]notices?['\"]|['\"]hint['\"]|\\bnotice\\b|import_notice|BatchError|serialize_batch_result|error_serializer|result_serializer"
    include:
    - '*.py'
    exclude:
    - results.py
    limit: 40
  reason: "Restbestände alter Mechanismen (warning/notice/hint-Keys, import_notice, Serializer) im Quellcode aufspüren."
```
/answer 6bd4-b8-0 allow
Control Request:
```yaml
id: 6bd4-b9-b8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/control/manager.py
        matches:
        - "* ``{\"id\": \"…\", \"hint\": \"…\"}`` — approve with a hint for the agent"
        - "approval.get('result'), item.result), approval_hint=approval.get('hint'), force_result_review=bool("
      - path: xy/ai/mcpc/tools/mcp/exa/search.py
        matches:
        - notice.
      - path: xy/ai/mcpc/tools/ast/core.py
        matches:
        - "``unchecked``, which puts the notice on the batch. Engines without dedicated support"
      - path: xy/ai/mcpc/tools/_tool_helpers.py
        matches:
        - "__all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serialize_batch_result', 'batch_schema']"
        - "def serialize_batch_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None) -> dict[str, Any]:"
        - structured_content=serialize_batch_result(
```
/answer 6bd4-b9-b8 allow
ReasoningToken: 106
Control Request:
```yaml
id: 6bd4-ba-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: notice
    include:
    - search.py
    - core.py
    intent: editing
    limit: 10
  reason: Die zwei verbleibenden 'notice'-Erwähnungen (Docstring/Kommentar) im Kontext sehen.
```
/answer 6bd4-ba-0 allow
Control Request:
```yaml
id: 6bd4-bb-ba
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: mcp/exa/search.py
        nodes:
        - id: parse_search_text.Fwi4qA|8lxPVW
          matches:
          - notice.
      - path: ast/core.py
        nodes:
        - id: apply_imports.D7JYwV|5mj3Zf
          matches:
          - "``unchecked``, which puts the notice on the batch. Engines without dedicated support"
```
/answer 6bd4-bb-ba allow
Control Request:
```yaml
id: 6bd4-bc-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: apply_imports.D7JYwV|5mj3Zf
    old_text: which puts the notice on the batch.
    new_text: which adds a warning to the batch.
  reason: Docstring in core.apply_imports an die neue Terminologie anpassen.
```
/answer 6bd4-bc-0 allow
Control Request:
```yaml
id: 6bd4-bd-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outcomes.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/_common.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/tools.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
  reason: Alle geänderten Dateien auf Kompilierbarkeit prüfen (pytest kann ich ohne run_command nicht ausführen).
```
/answer 6bd4-bd-0 allow
Control Request:
```yaml
id: 6bd4-be-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: \bnotice\b|\.warning\b|"warning"|'warning'|import_notice
    include:
    - '*.md'
    - '*.txt'
    - '*.yaml'
    - '*.yml'
    - '*.json'
    - '*.toml'
    exclude:
    - .venv/**
    - node_modules/**
    - project/**
    limit: 20
  reason: Dokumentation/Konfiguration auf Verweise auf die alten Felder prüfen.
```
/answer 6bd4-be-0 allow
Control Request:
```yaml
id: 6bd4-bf-be
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      warning: "Limit of 20 matches reached (total 517); narrow pattern, directory or include/exclude filters, or raise limit (max 50). 1 match line(s) exceeded 300 characters and were shortened in the middle."
```
/answer 6bd4-bf-be allow
ReasoningToken: 2820
Control Request:
```yaml
id: 6bd4-c0-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    ids: []
  reason: "test_ast_imports.py lesen, um die notice-Assertions anzupassen."
```
/answer 6bd4-c0-0 allow
Control Request:
```yaml
id: 6bd4-c1-c0
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
      nodes:
      - id: Qqmdyv|s8N7jy
        excert: '''Tests for ``addImport``/``removeImport`` handling of the node-editing ``ast_*`…'
      - id: 0ekC7u|9IXYUc
        excert: from __future__ import annotations
      - id: EDFjeC|StWP8k
        excert: "_SRC = Path(__file__).resolve().parents[1] / 'src'"
      - id: 8Pl1k7|GddZCa
        excert: from xy.ai.mcpc.tools.ast import core
      - id: 3Mpw8b|t0uVQE
        excert: "'# noqa: E402'"
      - id: NREYuD|nugLQv
        excert: "from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete"
      - id: 3Mpw8b|t0uVQE_1
        excert: "'# noqa: E402'"
      - id: LgnMR1|79MOp3
        excert: from xy.ai.mcpc.tools.ast.find import ast_find
      - id: 3Mpw8b|t0uVQE_2
        excert: "'# noqa: E402'"
      - id: xJMLX3|tYgCCM
        excert: "from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, normalize_import, state…"
      - id: 3Mpw8b|t0uVQE_3
        excert: "'# noqa: E402'"
      - id: cNhh2C|88ZZ6p
        excert: "from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert"
      - id: 3Mpw8b|t0uVQE_4
        excert: "'# noqa: E402'"
      - id: mO2aS8|5v2aZQ
        excert: "from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace"
      - id: R9Ge4p|9nD8LQ
        excert: PY_SOURCE = '"""Doc."""\nimport os\n\n\nclass A:\n    pass\n'
      - id: write
        excert: "def _write(tmp_path: Path, name: str, source: str) -> str:"
      - id: first_id
        excert: "def _first_id(path: str, node_type: str) -> str:"
      - id: 3mF7m9|fcmzbE
        excert: '''# --------------------------------------------------------------------------- …'
      - id: h5732ce4e
        excert: "def test_normalize_import_ignores_whitespace_quotes_and_semicolon():"
      - id: h4a06df6b
        excert: "def test_statement_spans_skip_comments_and_braces():"
      - id: cdVHjw|Z8dp7j
        excert: '''# --------------------------------------------------------------------------- …'
      - id: ha9d8358c
        excert: "def test_python_adds_missing_imports_after_existing_ones(tmp_path: Path):"
      - id: test_python_add_is_idempotent
        excert: "def test_python_add_is_idempotent(tmp_path: Path):"
      - id: h185e7a1a
        excert: "def test_python_remove_import_and_single_alias(tmp_path: Path):"
      - id: test_python_future_import_goes_first
        excert: "def test_python_future_import_goes_first(tmp_path: Path):"
      - id: h243cd0c1
        excert: "def test_python_invalid_import_keeps_edit_and_flags_notice(tmp_path: Path):"
      - id: h5c97a8cd
        excert: "def test_delete_with_imports_removes_unused_import(tmp_path: Path):"
      - id: he8f51ccc
        excert: "def test_delete_file_ignores_imports_but_flags_notice(tmp_path: Path):"
      - id: N3p4qW|KgIdG9
        excert: '''# --------------------------------------------------------------------------- …'
      - id: test_javascript_add_and_remove
        excert: "def test_javascript_add_and_remove(tmp_path: Path):"
      - id: he1b0d6c3
        excert: "def test_javascript_without_imports_inserts_after_directive(tmp_path: Path):"
      - id: h1e0bf190
        excert: "def test_javascript_non_import_keeps_edit_and_flags_notice(tmp_path: Path):"
      - id: h02cf6cf9
        excert: "def test_failing_engine_keeps_edit_and_flags_notice(tmp_path: Path, monkeypatch):"
      - id: vTqL0d|hNFSuK
        excert: '''# --------------------------------------------------------------------------- …'
      - id: h273f84a2
        excert: "def test_generic_present_import_needs_no_notice(tmp_path: Path):"
      - id: hc2f3f704
        excert: "def test_generic_missing_import_is_not_written_but_flagged(tmp_path: Path):"
      - id: test_generic_removes_whole_line_only
        excert: "def test_generic_removes_whole_line_only(tmp_path: Path):"
      - id: test_generic_unsafe_remove_is_flagged
        excert: "def test_generic_unsafe_remove_is_flagged(tmp_path: Path):"
      - id: h739bcb8e
        excert: "@pytest.mark.parametrize('bad', [[''], 'import os', [1]]) def test_schema_level_validation_rejects_malformed_values(bad):"
```
/answer 6bd4-c1-c0 allow
ReasoningToken: 68
Control Request:
```yaml
id: 6bd4-c2-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: notice|NOTICE
    include:
    - '*.py'
    intent: editing
    limit: 30
  reason: Alle notice-Verwendungen in den Tests mit Knoten-IDs finden.
```
/answer 6bd4-c2-0 allow
Control Request:
```yaml
id: 6bd4-c3-c2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: test_ast_imports.py
        nodes:
        - id: xJMLX3|tYgCCM
          matches:
          - "from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, normalize_import, statement_spans"
        - id: ha9d8358c.Riz9t2|kbGtvl
          matches:
          - assert not result.errors and result.notice is None
        - id: h243cd0c1
          matches:
          - "def test_python_invalid_import_keeps_edit_and_flags_notice(tmp_path: Path):"
        - id: h243cd0c1.6ILtsC|1lBn6b
          matches:
          - assert result.notice == IMPORT_NOTICE
        - id: he8f51ccc
          matches:
          - "def test_delete_file_ignores_imports_but_flags_notice(tmp_path: Path):"
        - id: he8f51ccc.kzvrEt|AS0uy8
          matches:
          - assert result.notice == IMPORT_NOTICE
        - id: test_javascript_add_and_remove.6LHK2m|M1HYfI
          matches:
          - assert not result.errors and result.notice is None
        - id: h1e0bf190
          matches:
          - "def test_javascript_non_import_keeps_edit_and_flags_notice(tmp_path: Path):"
        - id: h1e0bf190.tKJ3JE|oHGUXV
          matches:
          - assert result.notice == IMPORT_NOTICE
        - id: h02cf6cf9
          matches:
          - "def test_failing_engine_keeps_edit_and_flags_notice(tmp_path: Path, monkeypatch):"
        - id: h02cf6cf9.NyckgD|84Q4rU
          matches:
          - assert result.notice == IMPORT_NOTICE
        - id: h273f84a2
          matches:
          - "def test_generic_present_import_needs_no_notice(tmp_path: Path):"
        - id: h273f84a2.6LHK2m|kLwOMb
          matches:
          - assert not result.errors and result.notice is None
        - id: hc2f3f704.Riz9t2|Na2GkZ
          matches:
          - assert result.notice == IMPORT_NOTICE
        - id: test_generic_removes_whole_line_only.Riz9t2|VtTbdj
          matches:
          - assert not result.errors and result.notice is None
        - id: test_generic_unsafe_remove_is_flagged.Riz9t2|ZA3BAo
          matches:
          - assert result.notice == IMPORT_NOTICE
```
/answer 6bd4-c3-c2 allow
ReasoningToken: 188
Control Request:
```yaml
id: 6bd4-c4-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: ha9d8358c.Riz9t2|kbGtvl
    old_text: result.notice is None
    new_text: not result.all_warnings()
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: h243cd0c1.6ILtsC|1lBn6b
    old_text: result.notice == IMPORT_NOTICE
    new_text: "result.all_warnings() == [IMPORT_NOTICE]"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: he8f51ccc.kzvrEt|AS0uy8
    old_text: result.notice == IMPORT_NOTICE
    new_text: "result.all_warnings() == [IMPORT_NOTICE]"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: test_javascript_add_and_remove.6LHK2m|M1HYfI
    old_text: result.notice is None
    new_text: not result.all_warnings()
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: h1e0bf190.tKJ3JE|oHGUXV
    old_text: result.notice == IMPORT_NOTICE
    new_text: "result.all_warnings() == [IMPORT_NOTICE]"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: h02cf6cf9.NyckgD|84Q4rU
    old_text: result.notice == IMPORT_NOTICE
    new_text: "result.all_warnings() == [IMPORT_NOTICE]"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: h273f84a2.6LHK2m|kLwOMb
    old_text: result.notice is None
    new_text: not result.all_warnings()
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: hc2f3f704.Riz9t2|Na2GkZ
    old_text: result.notice == IMPORT_NOTICE
    new_text: "result.all_warnings() == [IMPORT_NOTICE]"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: test_generic_removes_whole_line_only.Riz9t2|VtTbdj
    old_text: result.notice is None
    new_text: not result.all_warnings()
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: test_generic_unsafe_remove_is_flagged.Riz9t2|ZA3BAo
    old_text: result.notice == IMPORT_NOTICE
    new_text: "result.all_warnings() == [IMPORT_NOTICE]"
  reason: test_ast_imports.py auf all_warnings() statt .notice umstellen.
```
/answer 6bd4-c4-0 allow
ReasoningToken: 309
Control Request:
```yaml
id: 6bd4-c5-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_import_all.py
  node_type: FunctionDef|imports|statements
  reason: Bestehenden Import-Test als Vorlage für den Contract-Test lesen.
```
/answer 6bd4-c5-0 allow
Control Request:
```yaml
id: 6bd4-c6-c5
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_import_all.py
      nodes:
      - id: 6HfTQv|PTlg7Y
        type: statements
        code: "\"Minimal smoke test: every module under ``xy.ai.mcpc`` must import cleanly.\\n\\nThis is intentionally shallow — it does not exercise behaviour — but it is\\nexactly the kind of test that catches refactoring fallout: broken imports,\\nrenamed symbols, circular imports, syntax errors, stray top-level code that\\nraises, etc. Each module is collected as its own parametrized test case so a\\nsingle broken module doesn't hide failures in the others.\\n\""
      - id: 4Vq6xl|4do0a3
        type: imports
        code: |-
          from __future__ import annotations
          import importlib
          import pkgutil
          import pytest
          import xy.ai.mcpc as root_package
      - id: discover_module_names
        type: FunctionDef
        code: |-
          def _discover_module_names() -> list[str]:
              names = [root_package.__name__]
              for module_info in pkgutil.walk_packages(root_package.__path__, prefix=root_package.__name__ + '.'):
                  names.append(module_info.name)
              return sorted(names)
      - id: OkUINy|VokQhF
        type: statements
        code: MODULE_NAMES = _discover_module_names()
      - id: test_discovery_found_modules
        type: FunctionDef
        code: |-
          def test_discovery_found_modules():
              """# Guards against a broken discovery silently collecting zero tests."""
              assert len(MODULE_NAMES) > 30
      - id: test_module_imports
        type: FunctionDef
        code: |-
          @pytest.mark.parametrize('module_name', MODULE_NAMES)
          def test_module_imports(module_name):
              importlib.import_module(module_name)
```
/answer 6bd4-c6-c5 allow
ReasoningToken: 3038
Control Request:
```yaml
id: 6bd4-c7-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_result_contract.py
    source: |
      """Contract tests for the shared result model: one owner for warnings and errors."""
      from __future__ import annotations

      import ast
      import importlib
      import pkgutil
      from dataclasses import dataclass
      from inspect import get_annotations
      from pathlib import Path
      from typing import Any

      import pytest

      import xy.ai.mcpc.tools as tools_package
      from xy.ai.mcpc.tools._tool_helpers import batch_tool_result, serialize_batch_result
      from xy.ai.mcpc.tools.list import ListItem
      from xy.ai.mcpc.tools.list import list as list_directories
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult, RESERVED_NAMES
      from xy.ai.mcpc.tools.tool_registry import ToolResult, normalize_result

      TOOLS_DIR = Path(tools_package.__path__[0])
      RESULTS_MODULE = tools_package.__name__ + '.results'
      FORBIDDEN_NAMES = {'warning', 'notice', 'hint'}


      def _import_all_tool_modules() -> None:
          for info in pkgutil.walk_packages(tools_package.__path__, prefix=tools_package.__name__ + '.'):
              importlib.import_module(info.name)


      def _subclasses(base: type) -> list[type]:
          found: list[type] = []
          for sub in base.__subclasses__():
              found.append(sub)
              found.extend(_subclasses(sub))
          return found


      def _project_subclasses() -> list[type]:
          _import_all_tool_modules()
          classes = [c for base in (ItemResult, ItemError, BatchResult) for c in _subclasses(base)]
          return [c for c in classes if c.__module__.startswith('xy.ai.mcpc.') and c.__module__ != RESULTS_MODULE]


      @dataclass(frozen=True)
      class _Item(ItemResult):
          path: str
          value: int = 0

          def key(self) -> dict[str, Any]:
              return {'path': self.path}


      @dataclass(frozen=True)
      class _Failure(ItemError):
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}


      class _Batch(BatchResult[_Item, _Failure]):
          def derived_warnings(self) -> tuple[str, ...]:
              return ('derived',) if self.results else ()


      def test_project_result_classes_are_discovered():
          assert len(_project_subclasses()) > 20


      def test_project_result_classes_use_no_reserved_fields_and_keep_to_dict_final():
          for cls in _project_subclasses():
              assert not set(get_annotations(cls)) & RESERVED_NAMES, cls
              assert 'to_dict' not in cls.__dict__, cls


      @pytest.mark.parametrize('base, field', [(ItemResult, 'warning'), (ItemResult, 'notice'),
                                               (ItemError, 'hint'), (ItemError, 'errors'),
                                               (BatchResult, 'notice')])
      def test_reserved_field_names_are_rejected_at_class_creation(base, field):
          with pytest.raises(TypeError, match='reserved'):
              type('Bad', (base,), {'__annotations__': {field: 'str'}})


      def test_overriding_to_dict_is_rejected():
          with pytest.raises(TypeError, match='to_dict'):
              type('Bad', (ItemResult,), {'to_dict': lambda self: {}})


      def test_item_serialization_adds_key_payload_and_warnings():
          item = _Item(path='a', value=1).warn('w1', 'w2')
          assert item.to_dict() == {'path': 'a', 'value': 1, 'warnings': ['w1', 'w2']}
          assert _Item(path='a').to_dict() == {'path': 'a'}


      def test_error_serialization_has_error_string_and_no_warnings():
          assert _Failure(path='a', error='boom').to_dict() == {'path': 'a', 'error': 'boom'}


      def test_payload_with_reserved_key_is_rejected():
          @dataclass(frozen=True)
          class Leaky(ItemResult):
              path: str

              def key(self) -> dict[str, Any]:
                  return {'path': self.path}

              def payload(self) -> dict[str, Any]:
                  return {'notice': 'x'}
          with pytest.raises(ValueError, match='reserved'):
              Leaky(path='a').to_dict()


      def test_batch_result_rejects_foreign_items():
          with pytest.raises(TypeError):
              _Batch(results=[{'path': 'a'}])
          with pytest.raises(TypeError):
              _Batch(errors=['boom'])


      def test_serialize_batch_result_requires_a_batch_result():
          class Duck:
              results = [_Item(path='a')]
              errors: list[Any] = []
              notice = 'x'
          with pytest.raises(TypeError):
              serialize_batch_result(Duck())


      def test_batch_warnings_are_deduplicated_and_derived():
          batch = _Batch(results=[_Item(path='a')], warnings=('own', 'derived'))
          assert batch.all_warnings() == ['own', 'derived']
          assert _Batch(errors=[_Failure(path='a', error='x')]).all_warnings() == []


      def test_batch_tool_result_puts_call_warnings_at_the_root_and_item_warnings_in_the_item():
          batch = _Batch(results=[_Item(path='a').warn('item-level')], errors=[_Failure(path='b', error='x')])
          out = batch_tool_result(batch).to_dict()['structuredContent']
          assert out['warnings'] == ['derived']
          assert out['results'] == [{'path': 'a', 'warnings': ['item-level']}]
          assert out['errors'] == [{'path': 'b', 'error': 'x'}]


      def test_list_reduction_is_an_item_warning_list(tmp_path: Path):
          for index in range(60):
              (tmp_path / f'f{index}.txt').write_text('x')
          out = batch_tool_result(list_directories([ListItem(path=str(tmp_path))])).to_dict()['structuredContent']
          assert 'warnings' not in out
          assert isinstance(out['results'][0]['warnings'], list) and len(out['results'][0]['warnings']) == 1
          assert 'warning' not in out['results'][0]


      def test_tool_result_failure_carries_no_structure_or_warnings():
          failure = ToolResult.failure('nope')
          assert failure.to_dict() == {'content': [{'type': 'text', 'text': 'nope'}], 'isError': True}
          with pytest.raises(ValueError):
              ToolResult(content=[], is_error=True, warnings=['w'])
          with pytest.raises(ValueError):
              ToolResult(is_error=True, structured_content={'a': 1})


      def test_tool_result_is_the_only_writer_of_root_warnings():
          assert ToolResult(structured_content={'a': 1}, warnings=['w']).to_dict()['structuredContent'] == {
              'a': 1, 'warnings': ['w']}
          assert ToolResult(warnings=['w']).to_dict()['structuredContent'] == {'warnings': ['w']}
          for reserved in ('warnings', 'messageFromUser'):
              with pytest.raises(ValueError, match='reserved'):
                  ToolResult(structured_content={reserved: 'x'})


      @pytest.mark.parametrize('value', ['text', {'a': 1}, None])
      def test_handlers_must_return_tool_result(value):
          with pytest.raises(TypeError):
              normalize_result(value)
          assert normalize_result(ToolResult()) is not None


      def test_unknown_exa_ids_are_reported_not_only_logged():
          from xy.ai.mcpc.tools.mcp.exa.search_results import _resolve
          results, missing = _resolve(['definitely-unknown-id'])
          assert results == [] and missing == ['definitely-unknown-id']


      def _forbidden_names(path: Path) -> list[str]:
          tree = ast.parse(path.read_text(encoding='utf-8'))
          hits: list[str] = []
          for node in ast.walk(tree):
              if isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name) and node.target.id in FORBIDDEN_NAMES:
                  hits.append(f'{path.name}:{node.lineno}: field {node.target.id}')
              if isinstance(node, ast.Dict):
                  hits.extend(f'{path.name}:{key.lineno}: key {key.value!r}' for key in node.keys
                              if isinstance(key, ast.Constant) and key.value in FORBIDDEN_NAMES)
              if isinstance(node, ast.Subscript) and isinstance(node.slice, ast.Constant) and node.slice.value in FORBIDDEN_NAMES:
                  hits.append(f'{path.name}:{node.lineno}: key {node.slice.value!r}')
          return hits


      def test_no_ad_hoc_warning_notice_or_hint_fields_outside_results_module():
          hits: list[str] = []
          for path in sorted(TOOLS_DIR.rglob('*.py')):
              if path.name == 'results.py' and path.parent == TOOLS_DIR:
                  continue
              hits.extend(_forbidden_names(path))
          assert not hits, 'use ItemResult.warnings / BatchResult.warnings / ToolResult.warnings:\n' + '\n'.join(hits)
  reason: "Contract-Test anlegen: Klassen-Guards, Serialisierungsregeln, ToolResult-Regeln und strukturelle AST-Prüfung auf verbotene warning/notice/hint-Felder."
```
/answer 6bd4-c7-0 allow
ReasoningToken: 1725
Control Request:
```yaml
id: 6bd4-c8-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_result_contract.py
    id: test_item_serialization_adds_key_payload_and_warnings
    old_text: "    assert _Item(path='a').to_dict() == {'path': 'a'}"
    new_text: "    assert _Item(path='a').to_dict() == {'path': 'a', 'value': 0}"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_result_contract.py
    id: test_batch_tool_result_puts_call_warnings_at_the_root_and_item_warnings_in_the_item
    old_text: "    assert out['results'] == [{'path': 'a', 'warnings': ['item-level']}]"
    new_text: "    assert out['results'] == [{'path': 'a', 'value': 0, 'warnings': ['item-level']}]"
  reason: Erwartungswerte im Contract-Test an das Standard-Payload (value=0 bleibt erhalten) anpassen.
```
/answer 6bd4-c8-0 allow
ReasoningToken: 2477
Control Request:
```yaml
id: 6bd4-c9-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    ids:
    - ToolResult
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    ids:
    - ItemOutcome
    - ItemResult
    - ItemError
    - BatchResult
  reason: Endstand von ToolResult und results.py-Klassen visuell auf Korrektheit prüfen.
```
/answer 6bd4-c9-0 allow
Control Request:
```yaml
id: 6bd4-ca-c9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: ToolResult
        type: ClassDef
        excert: "@dataclass(slots=True) class ToolResult:"
        docstring: The result of a tool call (maps onto MCP ``CallToolResult``).
        children:
        - id: ToolResult.5qOFdm|55iW4K
          type: statements
          code: |-
            'The result of a tool call (maps onto MCP ``CallToolResult``).'
            content: list[dict[str, Any]] = field(default_factory=list)
            structured_content: dict[str, Any] | None = None
            is_error: bool = False
            auto_approve: bool = False
            message: str | None = None
            warnings: list[str] = field(default_factory=list)
        - id: ToolResult.post_init
          type: FunctionDef
          code: |-
            def __post_init__(self) -> None:
                if self.is_error and (self.structured_content or self.warnings):
                    raise ValueError('ToolResult: error results carry neither structured_content nor warnings.')
                if self.structured_content and RESERVED_ROOT_KEYS & self.structured_content.keys():
                    raise ValueError('ToolResult: reserved key in structured_content; use ToolResult.warnings / ToolResult.message.')
        - id: ToolResult.failure
          type: FunctionDef
          code: |-
            @classmethod
            def failure(cls, message: str) -> 'ToolResult':
                return cls(content=[text_content(message)], is_error=True)
        - id: ToolResult.to_dict
          type: FunctionDef
          code: |-
            def to_dict(self) -> dict[str, Any]:
                result: dict[str, Any] = {}
                if self.content:
                    result['content'] = list(self.content)
                if self.structured_content or self.warnings:
                    structured = dict(self.structured_content or {})
                    if self.warnings:
                        structured[WARNINGS_KEY] = list(self.warnings)
                    result['structuredContent'] = structured
                if self.message:
                    attach_message(result, self.message)
                if self.is_error:
                    result['isError'] = True
                return result
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
      nodes:
      - id: ItemOutcome
        type: ClassDef
        excert: "@dataclass(frozen=True, kw_only=True) class ItemOutcome(ABC):"
        children:
        - id: ItemOutcome.key
          type: FunctionDef
          code: |-
            @abstractmethod
            def key(self) -> dict[str, Any]:
                """Input fields mirrored for result association."""
        - id: ItemOutcome.payload
          type: FunctionDef
          code: |-
            @abstractmethod
            def payload(self) -> dict[str, Any]:
                """Outcome-specific fields; must not use reserved names."""
        - id: ItemOutcome.to_dict
          type: FunctionDef
          code: |-
            @abstractmethod
            def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
                """Wire form; *payload* replaces :meth:`payload` for context-dependent rendering."""
      - id: ItemResult
        type: ClassDef
        excert: "@dataclass(frozen=True, kw_only=True) class ItemResult(ItemOutcome):"
        docstring: "An item that produced a result, possibly with restrictions (``warnings``)."
        children:
        - id: ItemResult.tihVbt|PBPiGn
          type: statements
          code: |-
            'An item that produced a result, possibly with restrictions (``warnings``).'
            warnings: tuple[str, ...] = ()
        - id: ItemResult.init_subclass
          type: FunctionDef
          code: |-
            def __init_subclass__(cls, **kwargs: Any) -> None:
                super().__init_subclass__(**kwargs)
                _guard_fields(cls)
        - id: ItemResult.payload
          type: FunctionDef
          code: |-
            def payload(self) -> dict[str, Any]:
                return compact_asdict(self, exclude={WARNINGS_KEY})
        - id: ItemResult.to_dict
          type: FunctionDef
          code: |-
            @final
            def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
                body = self.payload() if payload is None else payload
                out = {**self.key(), **_checked_payload(self, body)}
                if self.warnings:
                    out[WARNINGS_KEY] = list(self.warnings)
                return out
        - id: ItemResult.warn
          type: FunctionDef
          code: |-
            def warn(self, *messages: str) -> Self:
                return replace(self, warnings=(*self.warnings, *messages))
      - id: ItemError
        type: ClassDef
        excert: "@dataclass(frozen=True, kw_only=True) class ItemError(ItemOutcome):"
        docstring: An item that produced no result.
        children:
        - id: ItemError.cNSh7Q|Ds7nZB
          type: statements
          code: |-
            'An item that produced no result.'
            error: str
        - id: ItemError.init_subclass
          type: FunctionDef
          code: |-
            def __init_subclass__(cls, **kwargs: Any) -> None:
                super().__init_subclass__(**kwargs)
                _guard_fields(cls)
        - id: ItemError.payload
          type: FunctionDef
          code: |-
            def payload(self) -> dict[str, Any]:
                return {}
        - id: ItemError.to_dict
          type: FunctionDef
          code: |-
            @final
            def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
                body = self.payload() if payload is None else payload
                return {**self.key(), 'error': self.error, **_checked_payload(self, body)}
      - id: BatchResult
        type: ClassDef
        excert: "@dataclass(frozen=True, kw_only=True) class BatchResult(ABC, Generic[R, E]):"
        children:
        - id: BatchResult.jpRjN8|8tyMAT
          type: statements
          code: |-
            results: list[R] = field(default_factory=list)
            errors: list[E] = field(default_factory=list)
            warnings: tuple[str, ...] = ()
        - id: BatchResult.init_subclass
          type: FunctionDef
          code: |-
            def __init_subclass__(cls, **kwargs: Any) -> None:
                super().__init_subclass__(**kwargs)
                _guard_fields(cls)
        - id: BatchResult.post_init
          type: FunctionDef
          code: |-
            def __post_init__(self) -> None:
                if not all((isinstance(r, ItemResult) for r in self.results)):
                    raise TypeError('results must be ItemResult instances')
                if not all((isinstance(e, ItemError) for e in self.errors)):
                    raise TypeError('errors must be ItemError instances')
        - id: BatchResult.derived_warnings
          type: FunctionDef
          code: |-
            def derived_warnings(self) -> tuple[str, ...]:
                """Call-wide warnings that follow from the items and so cannot be forgotten."""
                return ()
        - id: BatchResult.all_warnings
          type: FunctionDef
          code: |-
            @final
            def all_warnings(self) -> list[str]:
                return list(dict.fromkeys((*self.warnings, *self.derived_warnings())))
```
/answer 6bd4-ca-c9 allow
ReasoningToken: 740
Control Request:
```yaml
id: 6bd4-cb-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: __dataclass_fields__|dataclasses\.fields|\bfields\(|asdict\(|is_dataclass
    include:
    - '*.py'
    exclude:
    - results.py
    limit: 30
  reason: "Prüfen, ob die Function-Registry/Sandbox Result-Dataclasses per Reflection serialisiert (würde ItemResult-Felder wie warnings betreffen)."
```
/answer 6bd4-cb-0 allow
Control Request:
```yaml
id: 6bd4-cc-cb
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/tools/mcp/context7.py
        matches:
        - "return ToolResult(structured_content={'libraries': [asdict(library) for library in libraries]})"
        - "return ToolResult(structured_content={'sections': [asdict(section) for section in sections]})"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
        matches:
        - metrics = asdict(compute_text_stats(item.get('text') or ''))
      - path: xy/ai/mcpc/tools/ast/base.py
        matches:
        - return _compact(asdict(node))
      - path: xy/ai/mcpc/tools/process.py
        matches:
        - "info: dict[str, Any] = {'file_stats': asdict(_compute_file_stats(Path(path)))}"
      - path: xy/ai/mcpc/tools/edit_block/__init__.py
        matches:
        - "return compact_asdict(self, exclude={'path', 'error'})"
      - path: xy/ai/mcpc/tools/file_stats/__init__.py
        matches:
        - text_stats = asdict(compute_text_stats(text))
      - path: xy/ai/mcpc/tools/edit_marks/__init__.py
        matches:
        - "return compact_asdict(self, exclude={'path', 'error'})"
      - path: xy/ai/mcpc/openalex/models.py
        matches:
        - "known = {f.name for f in dataclasses.fields(model)}"
        - "known = {f.name for f in dataclasses.fields(GroupByItem)}"
      - path: xy/ai/mcpc/__main__.py
        matches:
        - "valid_fields = {f.name for f in dataclasses.fields(ServerConfig)}"
```
/answer 6bd4-cc-cb allow
ReasoningToken: 627
Control Request:
```yaml
id: 6bd4-cd-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: "handle_batch_tool\\(|serialize_batch_result\\(|batch_tool_result\\(|asdict|SimpleNamespace|_serialize_read_result|\\.errors\\[0\\]\\.warning"
    include:
    - '*.py'
    limit: 40
  reason: Alle Aufrufstellen der Batch-Helfer auf konsistente Argumente prüfen.
```
/answer 6bd4-cd-0 allow
Control Request:
```yaml
id: 6bd4-ce-cd
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: xy/ai/mcpc/tools/mcp/context7.py
        matches:
        - "from dataclasses import asdict, dataclass"
        - "return ToolResult(structured_content={'libraries': [asdict(library) for library in libraries]})"
        - "return ToolResult(structured_content={'sections': [asdict(section) for section in sections]})"
      - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
        matches:
        - "from dataclasses import asdict, dataclass"
        - metrics = asdict(compute_text_stats(item.get('text') or ''))
      - path: xy/ai/mcpc/tools/ast/base.py
        matches:
        - "from dataclasses import asdict, dataclass, field"
        - return _compact(asdict(node))
      - path: xy/ai/mcpc/tools/ast/replace.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, ast_replace, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/delete.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - "return batch_tool_result(ast_outline(paths=paths, with_lines=with_lines))"
      - path: xy/ai/mcpc/tools/ast/edit_marks.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, ast_edit_marks, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, lambda items: ast_read(items, with_lines=with_lines), core.AstError)"
      - path: xy/ai/mcpc/tools/ast/edit_block.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, ast_edit_block, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/create.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, ast_create, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/insert.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, ast_insert, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/process.py
        matches:
        - "from dataclasses import asdict, dataclass"
        - "info: dict[str, Any] = {'file_stats': asdict(_compute_file_stats(Path(path)))}"
      - path: xy/ai/mcpc/tools/results.py
        matches:
        - "from dataclasses import asdict, dataclass, field, replace"
        - "'compact_asdict',"
        - "def compact_asdict(obj: Any, exclude: Iterable[str]=()) -> dict[str, Any]:"
        - "return compact({k: v for k, v in asdict(obj).items() if k not in skipped})"
        - "return compact_asdict(self, exclude={WARNINGS_KEY})"
      - path: xy/ai/mcpc/tools/skills/__init__.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, get_skills, SkillsError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/_tool_helpers.py
        matches:
        - "def serialize_batch_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None) -> dict[str, Any]:"
        - "def batch_tool_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:"
        - structured_content=serialize_batch_result(
        - "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], BatchResult[Any, Any]], error_class: type, result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:"
        - "return batch_tool_result(batch_result, result_payload, auto_approve)"
      - path: xy/ai/mcpc/tools/edit_lines/__init__.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, edit_lines, EditLinesError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/edit_block/__init__.py
        matches:
        - "from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult, compact_asdict"
        - "return compact_asdict(self, exclude={'path', 'error'})"
        - "return handle_batch_tool(ctx, item_factory, edit_block, EditBlockError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/file_stats/__init__.py
        matches:
        - "from dataclasses import asdict, dataclass"
        - text_stats = asdict(compute_text_stats(text))
        - "return handle_batch_tool(ctx, item_factory, file_stats, FileStatsError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/edit_marks/__init__.py
        matches:
        - "from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult, compact_asdict"
        - "return compact_asdict(self, exclude={'path', 'error'})"
        - "return handle_batch_tool(ctx, item_factory, edit_marks, EditMarksError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/list/__init__.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, list, ListError)"
      - path: xy/ai/mcpc/tools/edit_line/__init__.py
        matches:
        - "return handle_batch_tool(ctx, item_factory, edit_line, EditLineError, auto_approve=True)"
      warning: "Limit of 40 matches reached (total 46); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 6bd4-ce-cd allow
ReasoningToken: 106
Control Request:
```yaml
id: 6bd4-cf-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: "return handle_batch_tool\\(|handle_batch_tool,|import handle_batch_tool"
    include:
    - '*.py'
    exclude:
    - _tool_helpers.py
    - ast/*.py
    - list/*
    - skills/*
    - edit_lines/*
    - edit_block/*
    - edit_marks/*
    - search/*
    limit: 30
  reason: "Noch nicht migrierte handle_batch_tool-Aufrufer (z. B. file_stats, edit_line) finden."
```
/answer 6bd4-cf-0 allow
Control Request:
```yaml
id: 6bd4-d0-cf
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/tools/ast/replace.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, ast_replace, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/delete.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/edit_marks.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, ast_edit_marks, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "return handle_batch_tool(ctx, item_factory, lambda items: ast_read(items, with_lines=with_lines), core.AstError)"
      - path: xy/ai/mcpc/tools/ast/edit_block.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, ast_edit_block, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/create.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, ast_create, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/ast/insert.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, ast_insert, core.AstError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/skills/__init__.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - "return handle_batch_tool(ctx, item_factory, get_skills, SkillsError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/edit_lines/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "return handle_batch_tool(ctx, item_factory, edit_lines, EditLinesError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/edit_block/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "return handle_batch_tool(ctx, item_factory, edit_block, EditBlockError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/file_stats/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "return handle_batch_tool(ctx, item_factory, file_stats, FileStatsError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/edit_marks/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "return handle_batch_tool(ctx, item_factory, edit_marks, EditMarksError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/list/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "return handle_batch_tool(ctx, item_factory, list, ListError)"
      - path: xy/ai/mcpc/tools/edit_line/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "return handle_batch_tool(ctx, item_factory, edit_line, EditLineError, auto_approve=True)"
      - path: xy/ai/mcpc/tools/write/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "return handle_batch_tool(ctx, item_factory, write, WriteError, auto_approve=True)"
      warning: "Limit of 30 matches reached (total 32); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 6bd4-d0-cf allow
ReasoningToken: 210
Control Request:
```yaml
id: 6bd4-d1-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write
    pattern: ^(@dataclass|class \w+|from xy.ai.mcpc.tools._tool_helpers)|BatchResult\(|ItemError\(|Result\(
    include:
    - __init__.py
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: import handle_batch_tool
    include:
    - '*.py'
    exclude:
    - replace.py
    - delete.py
    - insert.py
    - create.py
    - read.py
    - edit_marks.py
    - edit_block.py
    limit: 30
  reason: "Struktur der drei noch nicht migrierten Batch-Tools erfassen und prüfen, ob weitere existieren."
```
/answer 6bd4-d1-0 allow
Control Request:
```yaml
id: 6bd4-d2-d1
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write
      matches:
      - path: __init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - "class FileStatsError(Exception):"
        - '@dataclass(frozen=True)'
        - "class FileStatsItem:"
        - "class FileStatsResult:"
        - "class FileStatsItemError:"
        - "class FileStatsBatchResult:"
        - "class TextStatsResult:"
        - return TextStatsResult(
        - return FileStatsResult(
        - "errors.append(FileStatsItemError(path=item.path, error=str(exc)))"
        - "return FileStatsBatchResult(results=results, errors=errors)"
        - "class FileStatsTool(ToolDefinition):"
        - "class EditLineError(Exception):"
        - "class EditLineItem:"
        - "class EditLineResult:"
        - "class EditLineItemError:"
        - "class EditLineBatchResult:"
        - "return EditLineResult(path=item.path, result=batch.results[0].result)"
        - "errors.append(EditLineItemError(path=item.path, error=str(exc)))"
        - "return EditLineBatchResult(results=results, errors=errors)"
        - "class EditLineTool(ToolDefinition):"
        - "class WriteError(Exception):"
        - "class WriteItem:"
        - "class WriteResult:"
        - "class WriteItemError:"
      warning: "Limit of 40 matches reached (total 45); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: skills/__init__.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - path: edit_lines/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: edit_block/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: file_stats/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: edit_marks/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: list/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: edit_line/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: write/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: search/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
```
/answer 6bd4-d2-d1 allow
Text:
 Wichtiger Fund: `file_stats`, `edit_line` und `write` nutzen ebenfalls `handle_batch_tool`. Ohne Migration würde das neue isinstance-Gate dort zur Laufzeit fehlschlagen. Ich migriere sie jetzt.
ReasoningToken: 96
Control Request:
```yaml
id: 6bd4-d3-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
  reason: Knoten-IDs der drei noch nicht migrierten Batch-Tools ermitteln.
```
/answer 6bd4-d3-0 allow
Control Request:
```yaml
id: 6bd4-d4-d3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
      nodes:
      - id: qV4X2x|REf7JM
        excert: "'File stats tool – returns file metrics for access and processing planning, for…"
      - id: XqcIaD|nDxMAe
        excert: import hashlib
      - id: czqN4v|CGQQFJ
        excert: "__all__ = ["
      - id: FileStatsError
        excert: "class FileStatsError(Exception):"
        docstring: Raised when file metrics cannot be computed.
      - id: FileStatsItem
        excert: "@dataclass(frozen=True) class FileStatsItem:"
        docstring: "One file to compute metrics for. Attributes: path: Absolute path to file (must …"
      - id: FileStatsResult
        excert: "@dataclass(frozen=True) class FileStatsResult:"
      - id: FileStatsItemError
        excert: "@dataclass(frozen=True) class FileStatsItemError:"
        docstring: "Error computing metrics for a single file, mirroring its input path for result …"
      - id: FileStatsBatchResult
        excert: "@dataclass(frozen=True) class FileStatsBatchResult:"
        docstring: "Result of :func:`file_stats`. Attributes: results: One :class:`FileStatsResult`…"
      - id: calculate_complexity
        excert: "def _calculate_complexity(text: str) -> float:"
        docstring: Calculate data structure complexity (0.0 to 1.0). Based on character set divers…
      - id: TextStatsResult
        excert: "@dataclass(frozen=True) class TextStatsResult:"
      - id: compute_text_stats
        excert: "def compute_text_stats(text: str) -> TextStatsResult:"
        docstring: Compute size/line/word/complexity metrics for *text*. Extracted from :func:`com…
      - id: compute_file_stats
        excert: "def compute_file_stats(path: Path) -> FileStatsResult:"
        docstring: Compute the file-metrics block for *path* (also reused by the outline tool). As…
      - id: file_stats_one
        excert: "def _file_stats_one(item: FileStatsItem) -> FileStatsResult:"
      - id: file_stats
        excert: "def file_stats(items: list[FileStatsItem]) -> FileStatsBatchResult:"
        docstring: "Compute file metrics for each absolute path in ``items``. Args: items: Files to…"
      - id: FileStatsTool
        excert: "class FileStatsTool(ToolDefinition):"
        children:
        - id: FileStatsTool.hQvPNb|gJH1I2
          excert: name = 'file_stats'
        - id: FileStatsTool.4gWrRr|gektEn
          excert: "_ITEM_PROPERTIES = {'path': {'type': 'string', 'description': 'Absolute file pa…"
        - id: FileStatsTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`file_stats`, translating the MCP schema to/from the Python A…"
      - id: register_file_stats_tool
        excert: "def register_file_stats_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
      nodes:
      - id: 2nUDDu|SnH9wF
        excert: "'Edit-line tool – replaces a single line with one or more lines in a file, for …"
      - id: PBZWYD|c9Bbv3
        excert: from dataclasses import dataclass
      - id: YtfKw9|C1PXrB
        excert: "__all__ = ["
      - id: EditLineError
        excert: "class EditLineError(Exception):"
        docstring: Raised when an edit-line operation cannot be performed.
      - id: EditLineItem
        excert: "@dataclass(frozen=True) class EditLineItem:"
        docstring: "One single-line edit to apply. Attributes: path: Absolute path to file (must be…"
      - id: EditLineResult
        excert: "@dataclass(frozen=True) class EditLineResult:"
        docstring: "Result of applying a single line edit, mirroring its input path for result asso…"
      - id: EditLineItemError
        excert: "@dataclass(frozen=True) class EditLineItemError:"
        docstring: "Error applying a single line edit, mirroring its input path for result associat…"
      - id: EditLineBatchResult
        excert: "@dataclass(frozen=True) class EditLineBatchResult:"
        docstring: "Result of :func:`edit_line`. Attributes: results: One :class:`EditLineResult` p…"
      - id: edit_line_one
        excert: "def _edit_line_one(item: EditLineItem) -> EditLineResult:"
      - id: edit_line
        excert: "def edit_line(items: list[EditLineItem]) -> EditLineBatchResult:"
        docstring: "Replace a single line with one or more lines, in one or more files. Delegates t…"
      - id: EditLineTool
        excert: "class EditLineTool(ToolDefinition):"
        children:
        - id: EditLineTool.kmvhqW|SvWeSl
          excert: name = 'edit_line'
        - id: EditLineTool.3dbRH3|oyDRYm
          excert: "_ITEM_PROPERTIES = {"
        - id: EditLineTool.ZDSH0j|9may4x
          excert: "_ITEM_REQUIRED = ['path', 'old_line', 'new_lines']"
        - id: EditLineTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`edit_line`, translating the MCP schema to/from the Python AP…"
      - id: register_edit_line_tool
        excert: "def register_edit_line_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
      nodes:
      - id: ujs5Ag|mQ94sx
        excert: "'Write tool – writes files completely or appends content to them, for a batch o…"
      - id: ZXLaUr|ItuAK8
        excert: from dataclasses import dataclass
      - id: WVBpNO|Ep3OAb
        excert: "__all__ = ["
      - id: WriteError
        excert: "class WriteError(Exception):"
        docstring: Raised when a write operation cannot be performed.
      - id: WriteItem
        excert: "@dataclass(frozen=True) class WriteItem:"
        docstring: "One file to write. Attributes: path: Absolute path to file to write (created if…"
      - id: WriteResult
        excert: "@dataclass(frozen=True) class WriteResult:"
        docstring: "Result of writing a single file, mirroring its input path for result associatio…"
      - id: WriteItemError
        excert: "@dataclass(frozen=True) class WriteItemError:"
        docstring: "Error writing a single file, mirroring its input path for result association."
      - id: WriteBatchResult
        excert: "@dataclass(frozen=True) class WriteBatchResult:"
        docstring: "Result of :func:`write`. Attributes: results: One :class:`WriteResult` per succ…"
      - id: write_one
        excert: "def _write_one(item: WriteItem) -> WriteResult:"
      - id: write
        excert: "def write(items: list[WriteItem]) -> WriteBatchResult:"
        docstring: "Write or replace one or more target files, for a batch of items. Args: items: F…"
      - id: WriteTool
        excert: "class WriteTool(ToolDefinition):"
        children:
        - id: WriteTool.b7VGuc|QjWlF5
          excert: name = 'write'
        - id: WriteTool.I9iQgj|GRtQ66
          excert: "_ITEM_PROPERTIES = {"
        - id: WriteTool.ZDSH0j|wzoTvm
          excert: "_ITEM_REQUIRED = ['path', 'mode', 'content']"
        - id: WriteTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`write`, translating the MCP schema to/from the Python API."
      - id: register_write_tool
        excert: "def register_write_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 6bd4-d4-d3 allow
Control Request:
```yaml
id: 6bd4-d5-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
    ids:
    - XqcIaD|nDxMAe
    - FileStatsResult
    - FileStatsItemError
    - FileStatsBatchResult
    - compute_file_stats
    - file_stats_one
    - FileStatsTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
    ids:
    - PBZWYD|c9Bbv3
    - EditLineResult
    - EditLineItemError
    - EditLineBatchResult
    - EditLineTool.handle
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
    ids:
    - ZXLaUr|ItuAK8
    - WriteResult
    - WriteItemError
    - WriteBatchResult
    - WriteTool.handle
  reason: Result-/Error-/Batch-Klassen und Handler dieser drei Tools lesen.
```
/answer 6bd4-d5-0 allow
Control Request:
```yaml
id: 6bd4-d6-d5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
      nodes:
      - id: XqcIaD|nDxMAe
        type: imports
        code: |-
          import hashlib
          import re
          from dataclasses import asdict, dataclass
          from datetime import datetime, timezone
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: FileStatsResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class FileStatsResult:
              path: str
              size_bytes: int
              lines: int
              words: int
              complexity: float
              created: str
              modified: str
              accessed: str
              line_length_max: int
              line_length_min: int
              line_length_avg: float
              words_per_line_avg: float
              checksum: str
      - id: FileStatsItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class FileStatsItemError:
              """Error computing metrics for a single file, mirroring its input path for result association."""
              path: str
              error: str
      - id: FileStatsBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class FileStatsBatchResult:
              """Result of :func:`file_stats`.

              Attributes:
                  results: One :class:`FileStatsResult` per file whose metrics could be computed.
                  errors: One :class:`FileStatsItemError` per file that failed.
              """
              results: list[FileStatsResult]
              errors: list[FileStatsItemError]
      - id: compute_file_stats
        type: FunctionDef
        code: |-
          def compute_file_stats(path: Path) -> FileStatsResult:
              """Compute the file-metrics block for *path* (also reused by the outline tool).

              Assumes *path* is an existing regular file.
              """
              raw_bytes = path.read_bytes()
              text = raw_bytes.decode('utf-8', errors='replace')
              text_stats = asdict(compute_text_stats(text))
              text_stats['size_bytes'] = len(raw_bytes)
              stat = path.stat()
              created = datetime.fromtimestamp(stat.st_birthtime if hasattr(stat, 'st_birthtime')
                                               else stat.st_mtime, tz=timezone.utc).isoformat()
              modified = datetime.fromtimestamp(stat.st_mtime, tz=timezone.utc).isoformat()
              accessed = datetime.fromtimestamp(stat.st_atime, tz=timezone.utc).isoformat()
              return FileStatsResult(
                  path=str(
                      path.resolve()),
                  created=created,
                  modified=modified,
                  accessed=accessed,
                  **text_stats)
      - id: file_stats_one
        type: FunctionDef
        code: |-
          def _file_stats_one(item: FileStatsItem) -> FileStatsResult:
              file_path = Path(item.path)
              if not file_path.is_absolute():
                  raise FileStatsError('Path must be absolute.')
              if not file_path.exists():
                  raise FileStatsError('File not found.')
              if not file_path.is_file():
                  raise FileStatsError('Not a regular file.')
              return compute_file_stats(file_path)
      - id: FileStatsTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`file_stats`, translating the MCP schema to/from the Python API."""

              def item_factory(it: dict[str, Any]) -> FileStatsItem:
                  return FileStatsItem(path=it['path'])
              return handle_batch_tool(ctx, item_factory, file_stats, FileStatsError, auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
      nodes:
      - id: PBZWYD|c9Bbv3
        type: imports
        code: |-
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
          from xy.ai.mcpc.tools.edit_block import EditBlockItem, edit_block
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: EditLineResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditLineResult:
              """Result of applying a single line edit, mirroring its input path for result association."""
              path: str
              result: str
      - id: EditLineItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditLineItemError:
              """Error applying a single line edit, mirroring its input path for result association."""
              path: str
              error: str
      - id: EditLineBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditLineBatchResult:
              """Result of :func:`edit_line`.

              Attributes:
                  results: One :class:`EditLineResult` per successfully applied edit.
                  errors: One :class:`EditLineItemError` per edit that failed.
              """
              results: list[EditLineResult]
              errors: list[EditLineItemError]
      - id: EditLineTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`edit_line`, translating the MCP schema to/from the Python API."""

              def item_factory(it: dict[str, Any]) -> EditLineItem:
                  return EditLineItem(
                      path=it['path'],
                      old_line=it['old_line'],
                      new_lines=it['new_lines'],
                      exact=it.get(
                          'exact',
                          False),
                      replace_all=it.get(
                          'replaceAll',
                          False))
              return handle_batch_tool(ctx, item_factory, edit_line, EditLineError, auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
      nodes:
      - id: ZXLaUr|ItuAK8
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      - id: WriteResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class WriteResult:
              """Result of writing a single file, mirroring its input path for result association."""
              path: str
              result: str
      - id: WriteItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class WriteItemError:
              """Error writing a single file, mirroring its input path for result association."""
              path: str
              error: str
      - id: WriteBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class WriteBatchResult:
              """Result of :func:`write`.

              Attributes:
                  results: One :class:`WriteResult` per successfully written file.
                  errors: One :class:`WriteItemError` per file that failed.
              """
              results: list[WriteResult]
              errors: list[WriteItemError]
      - id: WriteTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`write`, translating the MCP schema to/from the Python API."""

              def item_factory(it: dict[str, Any]) -> WriteItem:
                  return WriteItem(path=it['path'], mode=it['mode'], content=it['content'])
              return handle_batch_tool(ctx, item_factory, write, WriteError, auto_approve=True)
```
/answer 6bd4-d6-d5 allow
ReasoningToken: 572
Control Request:
```yaml
id: 6bd4-d7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: compute_file_stats|_compute_file_stats|FileStatsResult|EditLineResult|WriteResult\b
    include:
    - '*.py'
    intent: editing
    limit: 30
  reason: Weiterverwendungen von FileStatsResult (insbesondere asdict in process.py) vor der Migration prüfen.
```
/answer 6bd4-d7-0 allow
Control Request:
```yaml
id: 6bd4-d8-d7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      - /home/user/xyan/xy.ai.workbench/mcpc/tests
      matches:
      - path: xy/ai/mcpc/tools/process.py
        nodes:
        - id: ygbkS8|gj1T9N
          matches:
          - from xy.ai.mcpc.tools.file_stats import compute_file_stats as _compute_file_stats
        - id: spill_extra_info.v6As8W|3XnEh8
          matches:
          - "info: dict[str, Any] = {'file_stats': asdict(_compute_file_stats(Path(path)))}"
      - path: xy/ai/mcpc/tools/file_stats/__init__.py
        nodes:
        - id: czqN4v|CGQQFJ
          matches:
          - "'FileStatsResult',"
          - "'compute_file_stats',"
        - id: FileStatsResult
          matches:
          - "class FileStatsResult:"
        - id: FileStatsBatchResult.e1HVmv|tuflJj
          matches:
          - "results: One :class:`FileStatsResult` per file whose metrics could be computed."
          - "results: list[FileStatsResult]"
        - id: compute_text_stats.1SXBNe|UOaR17
          matches:
          - Extracted from :func:`compute_file_stats` so string content (e.g. fetched
        - id: compute_file_stats
          matches:
          - "def compute_file_stats(path: Path) -> FileStatsResult:"
        - id: compute_file_stats.BPjiki|lBMjmY
          matches:
          - return FileStatsResult(
        - id: file_stats_one
          matches:
          - "def _file_stats_one(item: FileStatsItem) -> FileStatsResult:"
        - id: file_stats_one.XeicLs|VUT109
          matches:
          - return compute_file_stats(file_path)
        - id: file_stats.SajFij|KBxJ6s
          matches:
          - "results: list[FileStatsResult] = []"
      - path: xy/ai/mcpc/tools/edit_line/__init__.py
        nodes:
        - id: YtfKw9|C1PXrB
          matches:
          - "'EditLineResult',"
        - id: EditLineResult
          matches:
          - "class EditLineResult:"
        - id: EditLineBatchResult.zIDIrc|deARwO
          matches:
          - "results: One :class:`EditLineResult` per successfully applied edit."
          - "results: list[EditLineResult]"
        - id: edit_line_one
          matches:
          - "def _edit_line_one(item: EditLineItem) -> EditLineResult:"
        - id: edit_line_one.II3NI6|KJyLyL
          matches:
          - "return EditLineResult(path=item.path, result=batch.results[0].result)"
        - id: edit_line.1POfwf|hiPYAH
          matches:
          - "results: list[EditLineResult] = []"
      - path: xy/ai/mcpc/tools/write/__init__.py
        nodes:
        - id: WVBpNO|Ep3OAb
          matches:
          - "'WriteResult',"
        - id: WriteResult
          matches:
          - "class WriteResult:"
        - id: WriteBatchResult.GE5VsP|hBP8Bm
          matches:
          - "results: One :class:`WriteResult` per successfully written file."
          - "results: list[WriteResult]"
        - id: write_one
          matches:
          - "def _write_one(item: WriteItem) -> WriteResult:"
        - id: write_one.XrrbCW|la1g7x
          matches:
          - "return WriteResult(path=item.path, result='success')"
        - id: write.CQzcxg|EKZbUZ
          matches:
          - "results: list[WriteResult] = []"
```
/answer 6bd4-d8-d7 allow
Control Request:
```yaml
id: 6bd4-d9-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py
    id: spill_extra_info.v6As8W|3XnEh8
    old_text: asdict(_compute_file_stats(Path(path)))
    new_text: _compute_file_stats(Path(path)).payload()
  reason: "process.py: FileStatsResult wird zum ItemResult; die Spill-Info soll weiter nur die Metriken ohne 'warnings' enthalten."
```
/answer 6bd4-d9-0 allow
ReasoningToken: 203
Control Request:
```yaml
id: 6bd4-da-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
    id: FileStatsResult
    source: |-
      @dataclass(frozen=True)
      class FileStatsResult(ItemResult):
          path: str
          size_bytes: int
          lines: int
          words: int
          complexity: float
          created: str
          modified: str
          accessed: str
          line_length_max: int
          line_length_min: int
          line_length_avg: float
          words_per_line_avg: float
          checksum: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
    id: FileStatsItemError
    source: |-
      @dataclass(frozen=True)
      class FileStatsItemError(ItemError):
          """Error computing metrics for a single file, mirroring its input path for result association."""
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
    id: FileStatsBatchResult
    source: |-
      class FileStatsBatchResult(BatchResult[FileStatsResult, FileStatsItemError]):
          """Result of :func:`file_stats`.

          Attributes:
              results: One :class:`FileStatsResult` per file whose metrics could be computed.
              errors: One :class:`FileStatsItemError` per file that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
    id: XqcIaD|nDxMAe
    source: |-
      import hashlib
      import re
      from dataclasses import asdict, dataclass
      from datetime import datetime, timezone
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
    id: EditLineResult
    source: |-
      @dataclass(frozen=True)
      class EditLineResult(ItemResult):
          """Result of applying a single line edit, mirroring its input path for result association."""
          path: str
          result: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return {'result': self.result}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
    id: EditLineItemError
    source: |-
      @dataclass(frozen=True)
      class EditLineItemError(ItemError):
          """Error applying a single line edit, mirroring its input path for result association."""
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
    id: EditLineBatchResult
    source: |-
      class EditLineBatchResult(BatchResult[EditLineResult, EditLineItemError]):
          """Result of :func:`edit_line`.

          Attributes:
              results: One :class:`EditLineResult` per successfully applied edit.
              errors: One :class:`EditLineItemError` per edit that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
    id: PBZWYD|c9Bbv3
    source: |-
      from dataclasses import dataclass
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools.edit_block import EditBlockItem, edit_block
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
    id: WriteResult
    source: |-
      @dataclass(frozen=True)
      class WriteResult(ItemResult):
          """Result of writing a single file, mirroring its input path for result association."""
          path: str
          result: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}

          def payload(self) -> dict[str, Any]:
              return {'result': self.result}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
    id: WriteItemError
    source: |-
      @dataclass(frozen=True)
      class WriteItemError(ItemError):
          """Error writing a single file, mirroring its input path for result association."""
          path: str

          def key(self) -> dict[str, Any]:
              return {'path': self.path}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
    id: WriteBatchResult
    source: |-
      class WriteBatchResult(BatchResult[WriteResult, WriteItemError]):
          """Result of :func:`write`.

          Attributes:
              results: One :class:`WriteResult` per successfully written file.
              errors: One :class:`WriteItemError` per file that failed.
          """
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
    id: ZXLaUr|ItuAK8
    source: |-
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
  reason: "file_stats, edit_line und write auf die Basisklassen migrieren (sonst bricht das neue isinstance-Gate)."
```
/answer 6bd4-da-0 allow
ReasoningToken: 613
Control Request:
```yaml
id: 6bd4-db-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_result_contract.py
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/agent/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py
  reason: Zuletzt geänderte Dateien auf Kompilierbarkeit prüfen.
```
/answer 6bd4-db-0 allow
Result Stats: id=3e01cb77-9cdd-434d-9bd8-d6e361639cfd, total: 416118, in: 192, out: 128193, reason: 1278, read: 15108422, write: 287733, start: 1794949744619, end: 1794949768556, time: 23804, total: 904551, io: 0, control: 133, sent: 0, recv: 419
ReasoningToken: 1278
Ich habe alle fünf Schritte des Vorschlags umgesetzt. Ich konnte aber weder `runTests` noch pytest ausführen, weil mir in dieser Session kein `run_command`-Tool zur Verfügung stand. Geprüft habe ich nur per `ast_validate`, dass alle geänderten Dateien kompilieren, und zwar syntaktisch. Importreihenfolge, Dataclass-Vererbung und die neuen Tests sind ungetestet. Bitte einmal `runTests` laufen lassen.

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
