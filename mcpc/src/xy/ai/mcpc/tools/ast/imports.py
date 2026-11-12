"""Import/include/use handling shared by the ``ast_*`` edit tools.

Every edit tool applies ``addImport``/``removeImport`` through
:meth:`Engine.apply_imports` after the item's node edit. Support is purely
structural (no compilation). Engines without dedicated support fall back to
:func:`apply_generic`, which only verifies and removes whole lines.
"""
from __future__ import annotations
import re
from dataclasses import dataclass, field
from typing import Any, Callable
from xy.ai.mcpc.tools.ast.base import AstError, Engine, Tree, check_no_control_chars
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

def _string_list(item: dict[str, Any], key: str) -> list[str] | None:
    value = item.get(key)
    if value is None:
        return None
    if not isinstance(value, list) or not all((isinstance(v, str) and v.strip() for v in value)):
        raise ValueError(f"'{key}' must be a list of non-empty strings")
    return value or None

def parse_import_args(item: dict[str, Any]) -> tuple[list[str] | None, list[str] | None]:
    """``(addImport, removeImport)`` of a raw MCP item; ``ValueError`` on malformed values."""
    return (_string_list(item, 'addImport'), _string_list(item, 'removeImport'))

def import_notice(results: list[Any]) -> str | None:
    """The single batch-level hint, if any result carries unchecked imports."""
    if any((r.imports is not None and r.imports.unchecked for r in results)):
        return IMPORT_NOTICE
    return None

def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:
    """Serialized ``imports`` field of a result, or ``None`` if nothing changed."""
    if outcome is None:
        return None
    entry = {key: value for key, value in (('added', outcome.added), ('removed', outcome.removed)) if value}
    return entry or None
_SPACE = re.compile('\\s+')
_PUNCT = re.compile('\\s*([{},:;.*()])\\s*')

def normalize_import(text: str) -> str:
    """Whitespace-, quote- and trailing-``;``-insensitive form used to compare statements."""
    squashed = _SPACE.sub(' ', text.strip()).replace('"', "'")
    return _PUNCT.sub('\\1', squashed).replace(',}', '}').rstrip(';').strip()

def _checked(statement: str) -> str:
    check_no_control_chars(statement)
    return statement.strip()

def _unique(statements: list[str]) -> list[str]:
    return list(dict.fromkeys(statements))

def statement_spans(data: bytes) -> list[tuple[int, int]]:
    """Byte spans of the ``;``-terminated top-level statements in ``data`` (comments skipped)."""
    spans: list[tuple[int, int]] = []
    depth, start, i, n = (0, None, 0, len(data))
    while i < n:
        if data.startswith(b'//', i):
            end = data.find(b'\n', i)
            i = n if end < 0 else end
            continue
        if data.startswith(b'/*', i):
            end = data.find(b'*/', i + 2)
            i = n if end < 0 else end + 2
            continue
        char = data[i:i + 1]
        if start is None and (not char.isspace()):
            start = i
        if char == b'{':
            depth += 1
        elif char == b'}':
            depth = max(depth - 1, 0)
        elif char == b';' and depth == 0:
            spans.append((start, i + 1))
            start = None
        i += 1
    if start is not None:
        spans.append((start, n))
    return spans

def line_extent(data: bytes, start: int, end: int) -> tuple[int, int]:
    """``(start, end)`` widened to its whole line(s) if nothing else stands on them."""
    line_start = data.rfind(b'\n', 0, start) + 1
    line_end = data.find(b'\n', end)
    line_end = len(data) if line_end < 0 else line_end
    if not data[line_start:start].strip() and (not data[end:line_end].strip()):
        return (line_start, min(line_end + 1, len(data)))
    return (start, end)

def delete_span(data: bytes, start: int, end: int) -> bytes:
    """Remove ``data[start:end]``, including its line if nothing else stands on it."""
    start, end = line_extent(data, start, end)
    return data[:start] + data[end:]

def _next_line_start(data: bytes, offset: int) -> int:
    newline = data.find(b'\n', offset)
    return len(data) if newline < 0 else newline + 1

def _line_starts(data: bytes) -> list[int]:
    starts = [0]
    for line in data.splitlines(keepends=True):
        starts.append(starts[-1] + len(line))
    return starts

def _store(engine: Engine, tree: Tree, data: bytes) -> None:
    source = data.decode('utf-8')
    error = engine.validate(source) if engine.validates_syntax else None
    if error:
        raise AstError(f'The import change would leave the file invalid: {error}')
    tree.raw = engine.parse(source, tree.path).raw
    tree.source = source

def apply_generic(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
    """Engine-agnostic fallback.

    ``add`` is only checked: a statement missing as a line is reported as unchecked
    and never written. ``remove`` deletes lines that equal the statement; if the text
    occurs anywhere else (or spans several lines) nothing is touched and the batch
    is flagged instead.
    """
    outcome = ImportOutcome()
    source = tree.source
    present = {normalize_import(line) for line in source.splitlines()}
    outcome.unchecked = any((normalize_import(_checked(s)) not in present for s in add))
    lines = source.splitlines(keepends=True)
    for statement in _unique([_checked(s) for s in remove]):
        wanted = normalize_import(statement)
        kept = [line for line in lines if normalize_import(line) != wanted]
        if len(kept) != len(lines):
            lines = kept
            outcome.removed.append(statement)
        elif statement.rstrip(';').strip() in ''.join(lines):
            outcome.unchecked = True
    if outcome.removed:
        _store(engine, tree, ''.join(lines).encode('utf-8'))
    return outcome

def _prologue_end(root: Any, data: bytes, is_prologue: Callable[[Any], bool] | None) -> int:
    """Offset after leading shebang/directives and header comments (comments followed by a blank line)."""
    nodes = root.named_children
    pos, i = (0, 0)
    while i < len(nodes):
        node = nodes[i]
        if node.type == 'hash_bang_line' or (is_prologue is not None and is_prologue(node)):
            pos, i = (_next_line_start(data, node.end_byte), i + 1)
            continue
        if node.type != 'comment':
            break
        j = i
        while j + 1 < len(nodes) and nodes[j + 1].type == 'comment' and (nodes[j + 1].start_point[0] - nodes[j].end_point[0] <= 1):
            j += 1
        following = nodes[j + 1] if j + 1 < len(nodes) else None
        if following is None or following.start_point[0] - nodes[j].end_point[0] <= 1:
            break
        pos, i = (_next_line_start(data, nodes[j].end_byte), j + 1)
    return pos

def _snippet(engine: Engine, statement: str, import_types: set[str]) -> str:
    text = _checked(statement)
    root = engine.parse(text).raw.root_node
    nodes = root.named_children
    if root.has_error or len(nodes) != 1 or nodes[0].type not in import_types:
        raise AstError(f'Not a single import statement: {text!r}')
    return text

def apply_treesitter(engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str], is_prologue: Callable[[Any], bool] | None=None) -> ImportOutcome:
    """Structure-aware handling for tree-sitter engines whose top-level import nodes are ``import_types``."""
    adds = _unique([_snippet(engine, s, import_types) for s in add])
    removes = {normalize_import(_snippet(engine, s, import_types)): s.strip() for s in remove}
    outcome = ImportOutcome()
    data = tree.source.encode('utf-8')

    def imports(root: Any) -> list[Any]:
        return [n for n in root.named_children if n.type in import_types]

    def text_of(node: Any, buffer: bytes) -> str:
        return buffer[node.start_byte:node.end_byte].decode('utf-8')
    doomed = [(n, key) for n in imports(tree.raw.root_node) if (key := normalize_import(text_of(n, data))) in removes]
    for node, _ in sorted(doomed, key=lambda hit: hit[0].start_byte, reverse=True):
        data = delete_span(data, node.start_byte, node.end_byte)
    outcome.removed = _unique([removes[key] for _, key in doomed])
    root = engine.parse(data.decode('utf-8'), tree.path).raw.root_node if doomed else tree.raw.root_node
    present = {normalize_import(text_of(n, data)) for n in imports(root)}
    missing = [s for s in adds if normalize_import(s) not in present]
    if missing:
        existing = imports(root)
        block = '\n'.join(missing).encode('utf-8')
        if existing:
            pos = _next_line_start(data, existing[-1].end_byte)
            text = block + b'\n'
            if pos == len(data) and (not data.endswith(b'\n')):
                text = b'\n' + block
        else:
            pos = _prologue_end(root, data, is_prologue)
            text = block + b'\n'
            if data[pos:].strip() and (not data[pos:].startswith((b'\n', b'\r\n'))):
                text += b'\n'
            if pos > 0 and data[pos - 1:pos] != b'\n':
                text = b'\n' + text
        data = data[:pos] + text + data[pos:]
        outcome.added = missing
    if doomed or missing:
        _store(engine, tree, data)
    return outcome
'#: Node types under which the RPC processes report a run of imports.'
_RPC_GROUP_TYPES = {'ImportGroup', 'imports'}
'#: Top-level node types that must stay in front of any import (Rust ``//!`` module docs).'
_RPC_PROLOGUE_TYPES = {'ModuleAttrs'}

def _rpc_statement(statement: str) -> str:
    text = _checked(statement)
    if len(statement_spans(text.encode('utf-8'))) != 1:
        raise AstError(f'Not a single import statement: {text!r}')
    return text if text.endswith(';') else text + ';'

def _rpc_present(engine: Engine, tree: Tree, source: str | None=None) -> tuple[dict[str, tuple[int, int]], list[Any]]:
    """Every top-level import statement's byte span (keyed by normalized text) and the top-level nodes.

    ``source`` inspects a candidate text instead of ``tree.source``, without touching the tree.
    """
    '# A candidate has no file on disk: passing the path makes the engine reject it as stale.'
    view = tree if source is None else Tree(engine, None, source, None)
    data = view.source.encode('utf-8')
    starts = _line_starts(data)
    top = [loc for loc in engine.locate_all(view) if '.' not in loc.node_id]
    present: dict[str, tuple[int, int]] = {}
    for loc in top:
        if loc.node_type not in _RPC_GROUP_TYPES:
            continue
        low = starts[min(loc.lineno - 1, len(starts) - 1)]
        high = starts[min(loc.end_lineno, len(starts) - 1)]
        for begin, end in statement_spans(data[low:high]):
            present.setdefault(normalize_import(data[low + begin:low + end].decode('utf-8')), (low + begin, low + end))
    return (present, top)
'#: Line prefixes that stay attached to the declaration below them (docs, annotations, attributes).'
_ATTACHED_PREFIXES = (b'//', b'/*', b'*', b'#[', b'@')

def _leading_start(data: bytes, starts: list[int], index: int) -> int:
    """Start offset of line ``index``, moved up over directly attached comment/annotation lines."""
    while index > 0:
        previous = data[starts[index - 1]:starts[index]].strip()
        if previous.startswith(_ATTACHED_PREFIXES) and (not previous.startswith(b'//!')):
            index -= 1
        else:
            break
    return starts[index]

def _rpc_insertion(data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:
    """Edit that places ``code``: behind the last import, else behind ``package``/module docs, else before the first node."""
    block = code.encode('utf-8')
    if present:
        pos = _next_line_start(data, max((end for _, end in present.values())))
        text = b'\n' + block if pos == len(data) and (not data.endswith(b'\n')) else block + b'\n'
        return (pos, pos, text)
    spans = statement_spans(data)
    if spans and data[spans[0][0]:spans[0][1]].startswith(b'package'):
        pos = _next_line_start(data, spans[0][1])
        return (pos, pos, b'\n' + block + b'\n')
    starts = _line_starts(data)
    prologue = [n for n in top if n.node_type in _RPC_PROLOGUE_TYPES]
    body = [n for n in top if n.node_type not in _RPC_PROLOGUE_TYPES]
    if body:
        pos = _leading_start(data, starts, min(body[0].lineno - 1, len(starts) - 1))
        return (pos, pos, block + b'\n\n')
    if prologue:
        pos = starts[min(prologue[-1].end_lineno, len(starts) - 1)]
        return (pos, pos, b'\n' + block + b'\n')
    pos = len(data)
    return (pos, pos, (b'' if not data or data.endswith(b'\n') else b'\n') + block + b'\n')

def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
    """Structure-aware handling for RPC engines that report import groups (Java ``import``, Rust ``use``).

    The engine only reads: its locate result yields every import statement's span, all
    removals and the one insertion are computed as edits on the original text and applied
    together. The candidate is validated and re-located before ``tree.source`` is written once,
    so a failure leaves the tree untouched.
    """
    adds = _unique([_rpc_statement(s) for s in add])
    removes = _unique([_rpc_statement(s) for s in remove])
    data = tree.source.encode('utf-8')
    present, top = _rpc_present(engine, tree)
    outcome = ImportOutcome()
    edits: list[tuple[int, int, bytes]] = []
    removed_keys: set[str] = set()
    for statement in removes:
        key = normalize_import(statement)
        if key in present:
            edits.append((*line_extent(data, *present[key]), b''))
            removed_keys.add(key)
            outcome.removed.append(statement)
    missing = [s for s in adds if normalize_import(s) not in present.keys() - removed_keys]
    if missing:
        edits.append(_rpc_insertion(data, present, top, '\n'.join(missing)))
        outcome.added = missing
    if not edits:
        return outcome
    for start, end, text in sorted(edits, key=lambda edit: (edit[0], edit[1]), reverse=True):
        data = data[:start] + text + data[end:]
    source = data.decode('utf-8')
    error = engine.validate(source)
    if error:
        raise AstError(f'The import change would leave the file invalid: {error}')
    after, _ = _rpc_present(engine, tree, source)
    if any((normalize_import(s) not in after for s in outcome.added)):
        raise AstError(f'Not recognized as an import by the parser: {outcome.added!r}')
    '# Never lose imports we were not asked to remove (e.g. a mis-reported span).'
    kept = present.keys() - removed_keys
    lost = sorted((k for k in kept if k not in after))
    if lost:
        raise AstError(f'The import change would drop other imports: {lost!r}')
    tree.source = source
    return outcome