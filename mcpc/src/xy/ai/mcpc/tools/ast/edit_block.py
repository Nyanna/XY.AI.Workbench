"""``ast_edit_block`` tool: exact-block (old_text -> new_text) edits within selected nodes."""
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
from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, TextMatchError
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
from xy.ai.mcpc.tools.tool_registry import ToolFeature
__all__ = [
    'EditBlockItem',
    'EditBlockResult',
    'EditBlockError',
    'EditBlockBatchResult',
    'ast_edit_block',
    'EditBlockNodeTool',
    'register']

@dataclass(frozen=True)
class EditBlockItem:
    """One block edit to apply.

    Attributes:
        path: Absolute path to the file to modify.
        old_text: Unique 10-100 char block to find within the node's source (unless ``replace_all``).
        new_text: Replacement text (may be empty to delete the block).
        exact: If False (default), whitespace in ``old_text`` is matched tolerantly.
        replace_all: If True, replace every occurrence instead of requiring a single match.
        id: Unique id of the target node. If omitted, the node is searched for by ``old_text``.
        add_import: Import statements to add if missing, applied after the edit.
        remove_import: Import statements to remove if present, applied after the edit.
    """
    path: str
    old_text: str
    new_text: str
    exact: bool = False
    replace_all: bool = False
    id: str | None = None
    add_import: list[str] | None = None
    remove_import: list[str] | None = None

@dataclass(frozen=True)
class EditBlockResult(ItemResult):
    """Result of applying a single block edit.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        new_id: The node's new id, only set if the edit changed it.
    """
    path: str
    id: str | None
    new_id: str | None = None
    imports: ImportOutcome | None = None

    def key(self) -> dict[str, Any]:
        return node_key(self.path, self.id)

    def payload(self) -> dict[str, Any]:
        return edit_payload(new_id=self.new_id, imports=self.imports)

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

class EditBlockBatchResult(ImportCheckedBatchResult[EditBlockResult, EditBlockError]):
    """Result of :func:`ast_edit_block`.

    Attributes:
        results: One :class:`EditBlockResult` per successfully applied edit.
        errors: One :class:`EditBlockError` per edit that failed.
    """

def _node_guard(engine, reference):
    """Guard for tolerant node edits.

    Engines that flag malformed edits on replace need no extra check. Others
    (e.g. markup grammars) must not merge lines and must still re-parse cleanly.
    """
    if engine.validates_syntax:
        return None
    keep_lines = line_preserving(reference)
    return lambda span, result: keep_lines(span, result) and engine.validate(result) is None

def _edit_block_one(item: EditBlockItem) -> EditBlockResult:
    file_path = core.require_path(item.path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_text(tree, [item.old_text], id=item.id)
    node_source = core.edit_node_source(target)
    try:
        new_source = replace_in_block(
            node_source,
            item.old_text,
            item.new_text,
            exact=item.exact,
            replace_all=item.replace_all,
            accept=_node_guard(
                tree.engine,
                item.old_text),
            max_level=3 if tree.engine.validates_syntax else 2,
            where='node')
    except TextMatchError as exc:
        raise core.AstTextError(
            str(exc), reason=getattr(
                exc, 'reason', None), position=getattr(
                    exc, 'position', None), corrected_text=getattr(
                        exc, 'corrected_text', None), guess=getattr(
                            exc, 'guess', None), next_step=getattr(
                                exc, 'next_step', None)) from exc
    new_id = core.replace_node(target, new_source)
    imports = core.apply_imports(tree, item.add_import, item.remove_import)
    core.CACHE.save(file_path, tree)
    return EditBlockResult(path=item.path, id=item.id, new_id=new_id, imports=imports)

def ast_edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:
    """Replace occurrence(s) of ``old_text`` with ``new_text`` inside one or more nodes.

    Each addressed node's source is unparsed, its ``old_text`` block replaced
    (as with ``edit_block``), re-parsed, and used to replace the node.

    Args:
        items: Block edits to apply. Must be non-empty.

    Returns:
        EditBlockBatchResult: One result per successful edit, one error per failed edit.

    Raises:
        core.AstError: If ``items`` is empty.
    """
    if not items:
        raise core.AstError("'items' must be a non-empty list.")
    results: list[EditBlockResult] = []
    errors: list[EditBlockError] = []
    for item in items:
        try:
            results.append(_edit_block_one(item))
        except core.AstAmbiguous as exc:
            errors.append(EditBlockError(path=item.path, id=item.id, error=str(exc), candidates=exc.candidates))
        except core.AstError as exc:
            errors.append(
                EditBlockError(
                    path=item.path, id=item.id, error=str(exc), reason=getattr(
                        exc, 'reason', None), corrected_text=getattr(
                            exc, 'corrected_text', None), guess=getattr(
                                exc, 'guess', None), next_step=getattr(
                                    exc, 'next_step', None)))
    return EditBlockBatchResult(results=results, errors=errors)

class EditBlockNodeTool(ToolDefinition):
    name = 'ast_edit_block'
    features = frozenset({ToolFeature.INPUT_ASTID})
    title = 'Replace short text within AST nodes'
    description = "Replace occurrences of short 'old_text' with 'new_text', within nodes addressed by prior known id, for a batch of items. Don't use for large edits, use ast_edit_marks instead."
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
    _ITEM_REQUIRED = ['path', 'old_text', 'new_text', 'id']
    _ITEMS_DESCRIPTION = 'Block edits to apply.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

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

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditBlockNodeTool())
    functions.register(ast_edit_block)