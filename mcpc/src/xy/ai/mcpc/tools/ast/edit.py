"""``ast_edit`` tool: all modifying AST tools in a single call."""
from __future__ import annotations
from typing import Any
from xy.ai.mcpc.tools.ast.create import CreateFileTool
from xy.ai.mcpc.tools.ast.delete import DeleteTool
from xy.ai.mcpc.tools.ast.edit_block import EditBlockNodeTool
from xy.ai.mcpc.tools.ast.edit_marks import EditMarksNodeTool
from xy.ai.mcpc.tools.ast.insert import InsertNodeTool
from xy.ai.mcpc.tools.ast.replace import ReplaceNodeTool
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
__all__ = ['EditTool', 'register']
'# Execution order: files must exist before edits, deletions come last.'
_STEPS: tuple[tuple[str, ToolDefinition], ...] = (('create', CreateFileTool()), ('insert', InsertNodeTool(
)), ('replace', ReplaceNodeTool()), ('edit_block', EditBlockNodeTool()), ('edit_marks', EditMarksNodeTool()), ('delete', DeleteTool()))

def _error_text(result: ToolResult) -> str:
    return ' '.join((str(c.get('text', '')) for c in result.content)) or 'Unknown error'

class EditTool(ToolDefinition):
    name = 'ast_edit'
    title = 'Batch of AST modifications'
    description = 'Run create, insert, replace, edit_block, edit_marks and delete operations in one call. Each list has the item format of the tool ast_<list name>.'
    input_schema = {
        'type': 'object',
        'properties': {
            key: tool.input_schema['properties']['items'] for key,
            tool in _STEPS},
        'additionalProperties': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        sections: dict[str, Any] = {}
        warnings: list[str] = []
        approve = True
        for key, tool in _STEPS:
            items = ctx.arguments.get(key)
            if not items:
                continue
            sub = tool.handle(ToolContext(session=ctx.session, arguments={'items': items}))
            if sub.is_error:
                sections[key] = {'error': _error_text(sub)}
                approve = False
                continue
            sections[key] = sub.structured_content or {}
            warnings.extend(sub.warnings)
            approve = approve and sub.auto_approve
        if not sections:
            return ToolResult.failure('At least one non-empty operation list is required.')
        return ToolResult(structured_content=sections, warnings=list(dict.fromkeys(warnings)), auto_approve=approve)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditTool())