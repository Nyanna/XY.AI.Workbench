"""Selector machinery shared by the ``ast_*`` tools.

``ast_find`` is the only tool that restricts on diverse node properties, so it
uses the full :data:`SELECTOR_PROPS`. Every mutation tool addresses a node purely
by its unique ``id`` and uses the reduced :data:`PATH_SELECTOR_PROPS`.
"""
from typing import Any
from xy.ai.mcpc.tools.ast import core
__all__ = [
    'SELECTOR_PROPS',
    'PATH_SELECTOR_PROPS',
    'PATH_PROP',
    'IMPORT_PROPS',
    'select_one',
    'select_by_path',
    'select_by_text']
'#: Full node selectors – only ``ast_find`` may restrict on node properties.'
SELECTOR_PROPS = {
    'id': {
        'type': 'string',
        'description': 'Unique node id'},
    'name': {
        'type': 'string',
        'description': 'Simple node name.'},
    'node_type': {
        'type': 'string',
                'description': "Regular expression (case-insensitive) a node type name must match, e.g. 'FunctionDef' or 'pair'."},
    'lineno': {
        'type': 'integer',
        'description': 'Line in the target node.'},
    'end_lineno': {
        'type': 'integer',
        'description': 'End line of a range to get all nodes touching the lines.'},
    'parent_type': {
        'type': 'string',
        'description': 'Node type name of the container.'}}
'#: Path-only selectors used by every mutation tool (replace/insert/delete/edit_*).'
PATH_SELECTOR_PROPS = {'id': SELECTOR_PROPS['id']}
"#: Absolute-path property shared by every batch tool's item schema."
PATH_PROP = {'type': 'string', 'description': 'Absolute path to the file (never directories).'}
'#: Import handling shared by every node-editing tool (replace/insert/delete/edit_*).'
IMPORT_PROPS = {
    'addImport': {
        'type': 'array',
        'items': {
            'type': 'string',
            'minLength': 1},
        'description': "Import/include/use statements this edit needs, one complete statement per entry in the file's own syntax (e.g. 'import os', 'import a.b.C;', 'use std::fmt;')."},
    'removeImport': {
        'type': 'array',
                'items': {
                    'type': 'string',
                    'minLength': 1},
        'description': 'Import/include/use statements that are no longer needed. Removed if present, ignored otherwise.'}}

def select_one(tree, **selectors: Any) -> core.Located:
    """Return the single node in *tree* matching *selectors*.

    Raises:
        core.AstError: If no node matches, or more than one node matches.
    """
    hits = core.find(tree, **selectors)
    if not hits and set(selectors) == {'id'}:
        fallback = core.resolve_by_prefix(core.locate_all(tree), selectors['id'])
        hits = [fallback] if fallback is not None else []
    if not hits:
        raise core.AstError('No node matched the selector. verify the node with `search` or `ast_find`')
    if len(hits) > 1:
        raise core.AstError(f'Selector is ambiguous – {len(hits)} nodes matched.')
    return hits[0]

def select_by_path(tree, *, id: str | None=None) -> core.Located:
    """Return the single node in *tree* addressed by its unique ``id``.

    Raises:
        core.AstError: If ``id`` is missing, or it matches zero/many nodes.
    """
    if id is None:
        raise core.AstError('A node selector (id) is required.')
    return select_one(tree, id=id)

def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:
    """Return the node addressed by ``id``; if ``id`` is omitted, blank or unusable
    (stale, wrong, ambiguous), fall back to the single node whose source contains
    every string in *texts*.

    Raises:
        core.AstError: If no node can be determined. When a non-blank ``id`` was
            given, the original id error is reported.
        core.AstAmbiguous: If the text fallback finds several unrelated candidates;
            carries their ids as ``candidates``.
    """
    if id is not None and id.strip():
        try:
            return select_one(tree, id=id)
        except core.AstError as id_error:
            try:
                return _select_by_contained_text(tree, texts)
            except core.AstError:
                raise id_error from None
    return _select_by_contained_text(tree, texts)

def _select_by_contained_text(tree, texts: list[str]) -> core.Located:
    """Return the single innermost node whose source contains every string in *texts*.

    Nodes fully containing another matching node are dropped in favour of the
    innermost match; nesting alone is not treated as ambiguity.
    """
    candidates = [loc for loc in core.locate_all(tree) if all((text in core.edit_node_source(loc) for text in texts))]
    if not candidates:
        raise core.AstError('No node matched the given text; a node selector (id) is required.')
    candidates = [
        c for c in candidates if not any(
            (other is not c and c.lineno <= other.lineno and (
                c.end_lineno >= other.end_lineno) and (
                    (c.lineno,
                     c.end_lineno) != (
                        other.lineno,
                        other.end_lineno)) for other in candidates))]
    if len(candidates) > 1:
        raise core.AstAmbiguous(
            'Multiple target nodes found; specify a node selector (id).', [
                c.node_id for c in candidates])
    return candidates[0]