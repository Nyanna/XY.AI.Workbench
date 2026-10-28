from __future__ import annotations
from xy.ai.mcpc.server.session import Session
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolRegistry

def _grep(registry: ToolRegistry, session: Session, tmp_path, **extra):
    tool = registry.get('search')
    assert tool is not None
    arguments = {'items': [{'directory': [str(tmp_path)], 'pattern': 'needle', 'include': ['*.py']}], **extra}
    return tool.handler(ToolContext(session=session, arguments=arguments))

def _write_sample(tmp_path):
    (tmp_path / 'sample.py').write_text('def f():\n    return "needle"\n')

def test_schema_exposes_with_ast_flag(registry: ToolRegistry):
    tool = registry.get('search')
    assert tool.input_schema['properties']['intent']['type'] == 'string'
    assert 'intent' not in tool.input_schema.get('required', [])

def test_default_returns_plain_lines_even_with_ast_tools_enabled(registry: ToolRegistry, tmp_path):
    _write_sample(tmp_path)
    session = Session(id='grep-default')
    session.enabled_tools.update({'tools', 'ast_read'})
    result = _grep(registry, session, tmp_path, intent='survey')
    entry = result.structured_content['results'][0]['matches'][0]
    assert 'nodes' not in entry
    assert entry['matches'] == ['return "needle"']

def test_flag_true_returns_node_ids(registry: ToolRegistry, tmp_path):
    _write_sample(tmp_path)
    session = Session(id='grep-ast')
    session.enabled_tools.add('ast_read')
    result = _grep(registry, session, tmp_path, intent='editing')
    entry = result.structured_content['results'][0]['matches'][0]
    assert entry['nodes'][0]['id']
    assert 'needle' in entry['nodes'][0]['matches'][0]