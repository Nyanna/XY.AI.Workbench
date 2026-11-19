from __future__ import annotations
from xy.ai.mcpc.server.session import Session
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolRegistry

def _write_sample(tmp_path) -> str:
    path = tmp_path / 'sample.py'
    path.write_text('def f():\n    return "needle"\n')
    return str(path)

def _retrieve(registry: ToolRegistry, arguments: dict):
    tool = registry.get('ast_retrieve')
    assert tool is not None
    session = Session(id='retrieve')
    session.enabled_tools.add('ast_retrieve')
    return tool.handler(ToolContext(session=session, arguments=arguments, registry=registry))

def test_schema_has_all_sections(registry: ToolRegistry):
    tool = registry.get('ast_retrieve')
    assert tool is not None
    "# The registry adds a generic 'reason' property to every tool schema."
    assert set(tool.input_schema['properties']) - {'reason'} == {'search', 'outline', 'find', 'read'}

def test_empty_call_fails(registry: ToolRegistry):
    assert _retrieve(registry, {}).is_error

def test_outline_and_find_in_one_call(registry: ToolRegistry, tmp_path):
    path = _write_sample(tmp_path)
    result = _retrieve(registry, {'outline': [path], 'find': [{'paths': [path], 'text': 'needle'}]})
    assert not result.is_error
    content = result.structured_content
    assert set(content) == {'outline', 'find'}
    assert content['outline']['results']
    assert content['find']['results']

def test_search_returns_node_ids_and_read_uses_them(registry: ToolRegistry, tmp_path):
    path = _write_sample(tmp_path)
    search_item = {'directory': [str(tmp_path)], 'pattern': 'needle', 'include': ['*.py'], 'intent': 'edit'}
    first = _retrieve(registry, {'search': [search_item]})
    assert not first.is_error
    entry = first.structured_content['search']['results'][0]['matches'][0]
    node_id = entry['nodes'][0]['id']
    assert node_id
    second = _retrieve(registry, {'read': [{'path': path, 'ids': [node_id]}]})
    assert not second.is_error
    assert 'needle' in str(second.structured_content['read'])

def test_multiple_find_queries_are_merged(registry: ToolRegistry, tmp_path):
    path = _write_sample(tmp_path)
    result = _retrieve(registry, {'find': [{'paths': [path], 'text': 'needle'}, {'paths': [path], 'name': 'f'}]})
    assert not result.is_error
    assert len(result.structured_content['find']['results']) == 2

def test_failing_section_does_not_abort_others(registry: ToolRegistry, tmp_path):
    path = _write_sample(tmp_path)
    result = _retrieve(registry, {'outline': [path], 'search': [
                       {'directory': [str(tmp_path)], 'pattern': 'needle', 'intent': 'bogus'}]})
    assert 'outline' in result.structured_content
    assert 'search' in result.structured_content