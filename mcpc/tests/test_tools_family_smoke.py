"""Smoke test for the ``tools`` family (tool_search/tool_usage/tool_call).

Exercises one full round-trip through the real registry wiring: discover
``bash`` via keyword search, inspect its usage, then run it through
``tool_call``, checking session-persistent state and STDOUT spilling on the
way.
"""
from __future__ import annotations
from xy.ai.mcpc.server.session import Session
from xy.ai.mcpc.tools import register_tools
from xy.ai.mcpc.tools.registry import ToolRegistry
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_call import STREAM_SPILL_THRESHOLD

def _call(registry: ToolRegistry, session: Session, tool_name: str, **arguments):
    tool = registry.get(tool_name)
    assert tool is not None, f'tool not registered: {tool_name}'
    return tool.handler(ToolContext(session=session, arguments=arguments))

def test_tools_alias_groups_the_three_tools():
    registry = ToolRegistry()
    register_tools(registry)
    assert registry.expand_aliases({'tools'}) == {'tool_search', 'tool_usage', 'tool_call'}

def test_tool_search_to_tool_call_round_trip_with_bash():
    registry = ToolRegistry()
    register_tools(registry)
    session = Session(id='smoke-tools')
    '# 1) discover `bash` by keyword; the same search must not repeat it.'
    search_result = _call(registry, session, 'tool_search', keywords='bash working directory')
    names = [t['name'] for t in search_result.structured_content['tools']]
    assert 'bash' in names
    repeat = _call(registry, session, 'tool_search', keywords='bash working directory')
    assert repeat.structured_content['tools'] == []
    '# 2) inspect its usage; repeating the same request yields a hint, not the info again.'
    usage = _call(registry, session, 'tool_usage', name='bash')
    assert usage.structured_content['signature'].startswith('bash(')
    usage_repeat = _call(registry, session, 'tool_usage', name='bash')
    assert 'already returned' in usage_repeat.content[0]['text']
    '# 3) call `bash` via tool_call; result and a new variable persist in the session.'
    call1 = _call(registry, session, 'tool_call', tool_ids=['bash'], code="r = bash('/tmp', 'echo hi')\nprint(r.stdout.strip())\nkept = r.exit_code")
    assert call1.structured_content['stdout'] == 'hi\n'
    assert not call1.is_error
    '# 4) session persistence: `kept` survives into a fresh call without re-injecting tools.'
    call2 = _call(registry, session, 'tool_call', tool_ids=[], code='print(kept)')
    assert call2.structured_content['stdout'] == '0\n'
    '# 5) STDOUT spilling: oversized output is stored under a variable, not returned inline.'
    call3 = _call(registry, session, 'tool_call', tool_ids=[], code=f"print('x' * {STREAM_SPILL_THRESHOLD + 1})")
    assert 'stdout' not in call3.structured_content
    spill_var = call3.structured_content['stdout_var']
    assert spill_var in call3.content[0]['text']
    '# 6) the spilled content is retrievable by name in a later call (context persists).'
    call4 = _call(registry, session, 'tool_call', tool_ids=[], code=f'print(len({spill_var}))')
    assert call4.structured_content['stdout'] == f'{STREAM_SPILL_THRESHOLD + 2}\n'

def test_tool_call_rejects_unknown_tool_id():
    registry = ToolRegistry()
    register_tools(registry)
    session = Session(id='smoke-tools-unknown')
    result = _call(registry, session, 'tool_call', tool_ids=['does-not-exist'], code='pass')
    assert result.is_error
    assert 'does-not-exist' in result.content[0]['text']