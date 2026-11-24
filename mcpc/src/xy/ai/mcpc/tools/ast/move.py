"""``ast_move`` tool: move nodes between/within files, or whole files (insert/create & delete)."""
from __future__ import annotations
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools._tool_helpers import batch_schema, handle_batch_tool
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, select_by_path
from xy.ai.mcpc.tools.ast.outcomes import edit_payload
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.results import ItemError, ItemResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolRegistry, ToolResult
from dataclasses import dataclass, replace
from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, ImportCheckedBatchResult, ImportOutcome
__all__ = ['MoveItem', 'MoveResult', 'MoveError', 'MoveBatchResult', 'ast_move', 'MoveTool', 'register']

@dataclass(frozen=True)
class MoveItem:
    """One move operation.

    Attributes:
        path: Absolute path of the source file.
        target: Absolute path of the target file. May equal ``path`` (move within the
            file); a missing target file is created like ``ast_create`` does.
        ids: Ids of the source nodes to move, inserted in the given order. Empty/``None``
            moves the whole file. A node listed together with one of its ancestors is
            moved along with that ancestor.
        target_id: Id of the target node in the target file. ``None`` appends at the
            end of the target file. An unknown ``target_id`` aborts the item.
        position: ``"before"`` or ``"after"`` the target node. Defaults to ``"after"``.
        add_import: Import statements to add to the target file if missing.
        source_remove_import: Import statements to remove from the source file if present
            (ignored and flagged if the source file is deleted).
    """
    path: str
    target: str
    ids: list[str] | None = None
    target_id: str | None = None
    position: str = 'after'
    add_import: list[str] | None = None
    source_remove_import: list[str] | None = None

@dataclass(frozen=True)
class MoveResult(ItemResult):
    """Result of a single move operation.

    Attributes:
        path: The source path exactly as given in the input, for result association.
        target: The target path exactly as given in the input.
        moved: ``(old id, new id)`` per moved node. Every node is inserted on its own, so
            the new id is what that single insert created: one id, a list if the insert
            created several top-level ids (as ``ast_insert``'s ``ids``), or ``None``.
        imports: Import outcome in the target file (not serialized; only drives the
            batch-wide import notice, like ``source_imports``).
        source_imports: Import outcome in the source file.
        source_deleted: Whether the source file was removed (moved whole or left empty);
            Python API only, not serialized.
    """
    path: str
    target: str
    moved: list[tuple[str, str | list[str] | None]]
    imports: ImportOutcome | None = None
    source_imports: ImportOutcome | None = None
    source_deleted: bool = False

    def key(self) -> dict[str, Any]:
        return {'path': self.path, 'target': self.target}

    def payload(self) -> dict[str, Any]:
        moved = [{'old': old} if new is None else {'old': old, 'new': new} for old, new in self.moved]
        return edit_payload(moved=moved)

@dataclass(frozen=True)
class MoveError(ItemError):
    """Error of a move operation, or of a single source id within it.

    Attributes:
        path: The source path exactly as given in the input.
        target: The target path exactly as given in the input.
        id: The source id that could not be resolved, or ``None`` if the whole item failed.
        error: The error message.
    """
    path: str
    target: str
    id: str | None = None

    def key(self) -> dict[str, Any]:
        out: dict[str, Any] = {'path': self.path, 'target': self.target}
        if self.id is not None:
            out['id'] = self.id
        return out

class MoveBatchResult(ImportCheckedBatchResult[MoveResult, MoveError]):
    """Result of :func:`ast_move`.

    Attributes:
        results: One :class:`MoveResult` per item that moved at least one node or file.
        errors: One :class:`MoveError` per failed item and per unresolvable source id.
    """

    def derived_warnings(self) -> tuple[str, ...]:
        unchecked = any(
            (outcome is not None and outcome.unchecked for r in self.results for outcome in (
                r.imports,
                r.source_imports)))
        return (IMPORT_NOTICE,) if unchecked else ()

def _is_inside(node_id: str, ancestor_id: str) -> bool:
    return node_id.startswith(ancestor_id + '.')

def _same_file(a: Path, b: Path) -> bool:
    return a.resolve() == b.resolve()

def _remove_file(path: Path) -> None:
    """Delete ``path`` like ``ast_delete`` does: drop it from the cache and remove an emptied directory."""
    try:
        path.unlink()
    except OSError as exc:
        raise core.AstError('Deleting the source file failed.') from exc
    core.CACHE.invalidate(path)
    parent = path.parent
    if not any(parent.iterdir()):
        parent.rmdir()

def _save(path: Path, tree: core.Tree) -> None:
    try:
        path.parent.mkdir(parents=True, exist_ok=True)
        core.CACHE.save(path, tree)
    except OSError as exc:
        raise core.AstError('Writing a file failed.') from exc

def _is_empty(tree: core.Tree) -> bool:
    return not tree.engine.serialize(tree).strip()

def _place_each(dst_tree: core.Tree, target_id: str | None, codes: list[str], position: str) -> list[list[str] | None]:
    """Insert every code on its own; return the top-level ids each single insert created.

    Inserted nodes are located by position (:func:`core.place_node`); the target is
    tracked the same way, since an insert may rename it. If one insert changes the
    structure elsewhere, positions are no longer trustworthy and all results become
    ``None``. To keep the given order, nodes placed ``after`` the target are inserted
    last-first (each lands directly behind it).
    """
    located = core.locate_all(dst_tree)
    t = None if target_id is None else next((i for i, loc in enumerate(located) if loc.node_id == target_id))
    ranges: list[tuple[int, int] | None] = [None] * len(codes)
    exact = True
    order = list(range(len(codes)))
    if t is not None and position == 'after':
        order.reverse()
    for i in order:
        anchor = None if t is None else core.locate_all(dst_tree)[t]
        placement = core.place_node(dst_tree, anchor, codes[i], position)
        if not placement.exact:
            exact = False
            if t is not None and position == 'before':
                '# Best effort to keep inserting in front of the target.'
                t = next((j for j, loc in enumerate(core.locate_all(dst_tree))
                         if loc.node_id == target_id), t + max(placement.count, 0))
            continue
        p, k = (placement.start, placement.count)
        ranges = [r if r is None or r[0] < p else (r[0] + k, r[1]) for r in ranges]
        ranges[i] = (p, k)
        if t is not None and position == 'before':
            t += k
    if not exact:
        return [None] * len(codes)
    current = core.locate_all(dst_tree)
    return [core.top_level_ids([loc.node_id for loc in current[r[0]:r[0] + r[1]]]) if r else None for r in ranges]

def _new_id(ids: list[str] | None) -> str | list[str] | None:
    if not ids:
        return None
    return ids[0] if len(ids) == 1 else ids

def _parse_position(position: str) -> str:
    if position not in ('before', 'after'):
        raise core.AstError("'position' must be 'before' or 'after'.")
    return position

def _move_file(item: MoveItem, src_path: Path, dst_path: Path, errors: list[MoveError]) -> MoveResult | None:
    if _same_file(src_path, dst_path):
        raise core.AstError('Moving a whole file onto itself is not possible; give source ids.')
    src_tree = core.CACHE.get_tree(src_path)
    top_ids = core.top_level_ids([loc.node_id for loc in core.locate_all(src_tree)])
    source_imports = ImportOutcome(unchecked=True) if item.source_remove_import else None
    if not dst_path.exists() and src_path.suffix.lower() == dst_path.suffix.lower():
        if item.target_id:
            raise core.AstError('Target file does not exist; omit targetId to create it.')
        '# Plain copy like ast_create: same text and engine give the same outline and ids.'
        dst_tree = core.parse_for(item.target, src_path.read_text(encoding='utf-8'))
        new_top = core.top_level_ids([loc.node_id for loc in core.locate_all(dst_tree)])
        try:
            imports = core.apply_imports(dst_tree, item.add_import, None)
            _save(dst_path, dst_tree)
        except BaseException:
            core.CACHE.invalidate(dst_path)
            raise
        _remove_file(src_path)
        moved = list(zip(top_ids, new_top)) if len(top_ids) == len(new_top) else [(i, None) for i in top_ids]
        return MoveResult(
            path=item.path,
            target=item.target,
            moved=moved,
            imports=imports,
            source_imports=source_imports,
            source_deleted=True)
    if not top_ids:
        _remove_file(src_path)
        return MoveResult(
            path=item.path,
            target=item.target,
            moved=[],
            source_imports=source_imports,
            source_deleted=True)
    '# Otherwise move every top-level node on its own, which keeps the old->new ids exact.'
    result = _move_nodes(replace(item, ids=top_ids), src_path, dst_path, errors)
    if result is not None and (not result.source_deleted):
        result = result.warn('The source file was kept: content outside its top-level nodes remained.')
    return result

def _select_sources(item: MoveItem, src_tree: core.Tree, errors: list[MoveError]) -> list[tuple[str, core.Located]]:
    """Resolve every source id; unknown ids become errors without stopping the item."""
    selected: list[tuple[str, core.Located]] = []
    seen: set[str] = set()
    for node_id in dict.fromkeys(item.ids or []):
        try:
            loc = select_by_path(src_tree, id=node_id)
        except core.AstError as exc:
            errors.append(MoveError(path=item.path, target=item.target, id=node_id, error=str(exc)))
            continue
        if loc.node_id not in seen:
            seen.add(loc.node_id)
            selected.append((node_id, loc))
    return selected

def _move_nodes(item: MoveItem, src_path: Path, dst_path: Path, errors: list[MoveError]) -> MoveResult | None:
    src_tree = core.CACHE.get_tree(src_path)
    selected = _select_sources(item, src_tree, errors)
    if not selected:
        return None
    roots = [(given, loc) for given, loc in selected if not any(
        (_is_inside(loc.node_id, other.node_id) for _, other in selected))]
    root_ids = {loc.node_id for _, loc in roots}
    nested = [(given, loc) for given, loc in selected if loc.node_id not in root_ids]
    codes = [core.edit_node_source(loc).rstrip('\n') for _, loc in roots]
    all_src = core.locate_all(src_tree)
    order = {loc.node_id: i for i, loc in enumerate(all_src)}
    removal = sorted((loc for _, loc in roots), key=lambda loc: order.get(loc.node_id, -1), reverse=True)
    same = _same_file(src_path, dst_path)
    try:
        if same:
            dst_tree = src_tree
            target = _move_within(item, src_tree, all_src, roots, removal)
        else:
            if dst_path.exists():
                dst_tree = core.CACHE.get_tree(dst_path)
                target = select_by_path(dst_tree, id=item.target_id) if item.target_id else None
            else:
                if item.target_id:
                    raise core.AstError('Target file does not exist; omit targetId to create it.')
                dst_tree = core.empty_tree(dst_path)
                target = None
            '# Delete from the source in reverse document order so the remaining nodes stay addressed.'
            for loc in removal:
                core.delete_node(loc)
        created = _place_each(dst_tree, target.node_id if target else None, codes, item.position)
        new_all = {loc.node_id for loc in core.locate_all(dst_tree)}
        if same:
            imports = core.apply_imports(dst_tree, item.add_import, item.source_remove_import)
            source_imports = None
            _save(dst_path, dst_tree)
            source_deleted = False
        else:
            imports = core.apply_imports(dst_tree, item.add_import, None)
            source_imports = core.apply_imports(src_tree, None, item.source_remove_import)
            _save(dst_path, dst_tree)
            source_deleted = _is_empty(src_tree)
            if source_deleted:
                _remove_file(src_path)
            else:
                _save(src_path, src_tree)
    except BaseException:
        core.CACHE.invalidate(src_path)
        core.CACHE.invalidate(dst_path)
        raise
    moved: list[tuple[str, str | list[str] | None]] = [(given, _new_id(ids)) for (given, _), ids in zip(roots, created)]
    new_by_root = {loc.node_id: ids[0] for (_, loc), ids in zip(roots, created) if ids and len(ids) == 1}
    for given, loc in nested:
        anc = next((other for _, other in roots if _is_inside(loc.node_id, other.node_id)))
        anc_new = new_by_root.get(anc.node_id)
        new_id = anc_new + loc.node_id[len(anc.node_id):] if anc_new else None
        moved.append((given, new_id if new_id in new_all else None))
    return MoveResult(
        path=item.path,
        target=item.target,
        moved=moved,
        imports=imports,
        source_imports=source_imports,
        source_deleted=source_deleted)

def _move_within(item: MoveItem, tree: core.Tree, all_src: list[core.Located], roots: list[tuple[str, core.Located]], removal: list[core.Located]) -> core.Located | None:
    """Delete ``removal`` from ``tree`` and return the target node re-resolved afterwards."""
    target = select_by_path(tree, id=item.target_id) if item.target_id else None
    root_ids = [loc.node_id for _, loc in roots]
    if target is not None and any((target.node_id == r or _is_inside(target.node_id, r) for r in root_ids)):
        raise core.AstError('The target node is (inside) a moved node.')
    remaining = [loc for loc in all_src if not any((loc.node_id == r or _is_inside(loc.node_id, r) for r in root_ids))]
    position = next((i for i, loc in enumerate(remaining) if target and loc.node_id == target.node_id), None)
    for loc in removal:
        core.delete_node(loc)
    if target is None:
        return None
    after = core.locate_all(tree)
    '# Deleting keeps the document order, so the target keeps its index among the remaining nodes.'
    if position is not None and len(after) == len(remaining):
        return after[position]
    return select_by_path(tree, id=target.node_id)

def _move_one(item: MoveItem, errors: list[MoveError]) -> MoveResult | None:
    src_path = core.require_path(item.path)
    dst_path = core.require_path(item.target, must_exist=False)
    _parse_position(item.position)
    if not item.ids:
        return _move_file(item, src_path, dst_path, errors)
    return _move_nodes(item, src_path, dst_path, errors)

def ast_move(items: list[MoveItem]) -> MoveBatchResult:
    """Move nodes between or within files, or whole files.

    Nodes are inserted into the target (relative to ``target_id`` or appended at its
    end) and deleted from the source; without ids the whole source file is moved.
    A missing target file is created, a source file left empty is deleted. An
    unknown source id is reported as an error but the other ids are still moved;
    an unknown ``target_id`` aborts the item. Nothing is written for an aborted item.

    Args:
        items: Move operations to apply in order. Must be non-empty.

    Returns:
        MoveBatchResult: One result per item that moved something, one error per
        failed item or unresolvable source id.

    Raises:
        core.AstError: If ``items`` is empty.
    """
    if not items:
        raise core.AstError("'items' must be a non-empty list.")
    results: list[MoveResult] = []
    errors: list[MoveError] = []
    for item in items:
        item_errors: list[MoveError] = []
        try:
            result = _move_one(item, item_errors)
            if result is not None:
                results.append(result)
        except core.AstError as exc:
            item_errors.append(MoveError(path=item.path, target=item.target, error=str(exc)))
        errors.extend(item_errors)
    return MoveBatchResult(results=results, errors=errors)

def _string_list(it: dict[str, Any], key: str) -> list[str] | None:
    value = it.get(key)
    if value is None:
        return None
    if not isinstance(value, list) or not all((isinstance(v, str) and v.strip() for v in value)):
        raise ValueError(f"'{key}' must be a list of non-empty strings")
    return value or None

class MoveTool(ToolDefinition):
    name = 'ast_move'
    features = frozenset({ToolFeature.INPUT_ASTID})
    title = 'Move AST nodes or files'
    description = 'Move nodes between or within files, or whole files if no ids are given, for a batch of items.'
    _ITEM_PROPERTIES = {
        'path': {
            'type': 'string',
            'description': 'Absolute path of the source file.'},
        'target': {
            'type': 'string',
            'description': 'Absolute path of the target file; may equal path to move within the file. Created if missing.'},
        'ids': {
            'type': 'array',
                    'items': {
                        'type': 'string',
                        'minLength': 1},
            'description': 'Ids of the source nodes to move. Omit or leave empty to move the whole file. To move a container (e.g. an inner class) give its own id; listing only its children leaves the container in place.'},
        'targetId': {
            'type': 'string',
            'description': 'Id of the node in the target file to insert relative to.'},
        'position': {
            'type': 'string',
            'enum': [
                'before',
                'after'],
            'description': 'Placement relative to targetId.',
            'default': 'after'},
        'addImport': {
            **IMPORT_PROPS['addImport'],
            'description': IMPORT_PROPS['addImport']['description'] + ' Applied to the target file.'},
        'sourceRemoveImport': {
            **IMPORT_PROPS['removeImport'],
            'description': 'Import statements the source file no longer needs.'}}
    _ITEM_REQUIRED = ['path', 'target']
    _ITEMS_DESCRIPTION = 'Move operations to apply.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_move`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> MoveItem:
            add_import = _string_list(it, 'addImport')
            target_id = it.get('targetId')
            if target_id is not None and (not isinstance(target_id, str)):
                raise ValueError("'targetId' must be a string")
            return MoveItem(
                path=it['path'], target=it['target'], ids=_string_list(
                    it, 'ids'), target_id=target_id or None, position=it.get(
                        'position', 'after'), add_import=add_import, source_remove_import=_string_list(
                            it, 'sourceRemoveImport'))
        return handle_batch_tool(ctx, item_factory, ast_move, core.AstError, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(MoveTool())
    functions.register(ast_move)